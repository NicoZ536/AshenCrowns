//! Das Spiel: verbindet Engine, Inhalte, Grafik und Szenen.

use crate::art::ArtLibrary;
use crate::content::Content;
use crate::hud::{self, HudCtx};
use crate::menu::{MenuAction, MenuScene};
use crate::mission::{MissionScene, SceneAction};
use crate::settings::UserSettings;
use crate::ui_art::UiArt;
use ashen_engine::ui::theme;
use ashen_engine::{Engine, Game, WindowRequest};
use ashen_sim::mission::MatchSettings;
use egui::RichText;

pub enum Scene {
    Menu(Box<MenuScene>),
    Mission(Box<MissionScene>),
}

/// Optionen für automatisierte Läufe (Screenshots, Demos).
#[derive(Clone, Debug, Default)]
pub struct AutoRun {
    pub start_mission: Option<MatchSettings>,
    pub sim_seconds: f32,
    pub time_of_day: Option<f32>,
    pub weather: Option<ashen_sim::env::Weather>,
    pub battle: bool,
    pub select_army: bool,
    pub camera_tile: Option<(f32, f32)>,
    pub zoom_lines: Option<f32>,
    pub hide_ui: bool,
}

pub struct AshenGame {
    pub settings: UserSettings,
    content: Option<Content>,
    art: Option<ArtLibrary>,
    atlas_tex: Option<egui::TextureId>,
    ui_art: Option<UiArt>,
    scene: Option<Scene>,
    last_mission: Option<(usize, MatchSettings)>,
    auto: AutoRun,
    fatal: Option<String>,
}

impl AshenGame {
    pub fn new(settings: UserSettings, auto: AutoRun) -> AshenGame {
        AshenGame { settings, content: None, art: None, atlas_tex: None, ui_art: None, scene: None, last_mission: None, auto, fatal: None }
    }

    fn max_particles(engine: &Engine) -> usize {
        engine.renderer.settings.max_particles
    }

    fn start_mission(&mut self, engine: &mut Engine, index: usize, settings: MatchSettings) {
        let Some(content) = &self.content else { return };
        let mission = content.missions[index.min(content.missions.len() - 1)].clone();
        match MissionScene::new(&content.data, mission, settings.clone(), Self::max_particles(engine)) {
            Ok(s) => {
                self.last_mission = Some((index, settings));
                engine.audio.stop_music(1.5);
                self.scene = Some(Scene::Mission(Box::new(s)));
            }
            Err(e) => self.fatal = Some(e),
        }
    }

    fn open_menu(&mut self, engine: &mut Engine) {
        if let Some(c) = &self.content {
            engine.audio.stop_music(1.0);
            engine.audio.set_ambience(&[]);
            engine.renderer.clear_fog_of_war();
            self.scene = Some(Scene::Menu(Box::new(MenuScene::new(c, Self::max_particles(engine)))));
        }
    }

    fn apply_auto(&mut self, engine: &mut Engine) {
        let Some(ms) = self.auto.start_mission.clone() else { return };
        self.start_mission(engine, 0, ms);
        let (Some(Scene::Mission(m)), Some(content)) = (&mut self.scene, &self.content) else { return };
        let data = &content.data;
        if let Some(w) = self.auto.weather {
            m.world.env.weather = w;
            m.world.env.forced_weather = Some(w);
            m.world.env.weather_intensity = 1.0;
        }
        let steps = (self.auto.sim_seconds * data.rules.tick_rate as f32) as u32;
        if self.auto.battle {
            // Eine Welle direkt vor die Basis setzen, damit Kampfgeschehen sichtbar ist.
            let thrall = data.unit_id("hollow_thrall");
            let brute = data.unit_id("grave_brute");
            for i in 0..8 {
                let kind = if i == 7 { brute } else { thrall };
                if let Some(k) = kind {
                    let id = m.world.spawn_unit(data, k, ashen_sim::entity::ENEMY, glam::Vec2::new(24.0 + (i % 3) as f32, 47.0 + (i / 3) as f32));
                    if let Some(u) = m.world.units.get_mut(id) {
                        u.order = ashen_sim::entity::Order::Move { target: glam::Vec2::new(15.0, 55.0), attack_move: true };
                        u.request_path(glam::Vec2::new(15.0, 55.0));
                    }
                }
            }
        }
        for _ in 0..steps {
            m.world.step(data);
            let events = std::mem::take(&mut m.world.events);
            if let Some(art) = &self.art {
                for e in &events {
                    m.view.on_event(e, &m.world, data, art);
                }
            }
            m.view.update(1.0 / data.rules.tick_rate as f32);
        }
        if let Some(t) = self.auto.time_of_day {
            m.world.env.time_of_day = t;
        }
        if let Some((x, y)) = self.auto.camera_tile {
            m.center_on(glam::Vec2::new(x, y));
        }
        if let Some(l) = self.auto.zoom_lines {
            m.camera.target_lines = l;
        }
        if self.auto.select_army {
            m.selection = m.world.units.iter().filter(|(_, u)| u.owner == ashen_sim::entity::PLAYER && data.unit(u.kind).class != ashen_sim::data::UnitClass::Worker).map(|(id, _)| id).collect();
        }
    }
}

impl Game for AshenGame {
    fn init(&mut self, engine: &mut Engine) {
        engine.audio.set_volumes(self.settings.volumes);
        // Schriften und UI-Texturen (fehlende Schriften fallen auf die eingebauten zurück).
        let fonts = engine.assets_dir.as_ref().map(|a| a.join("fonts"));
        let n = theme::install_fonts(&engine.ui.ctx, fonts.as_deref());
        log::info!("{n} UI-Schriftdateien geladen");
        self.ui_art = Some(UiArt::new(&engine.ui.ctx));
        let Some(assets) = engine.assets_dir.clone() else {
            self.fatal = Some("Asset-Verzeichnis nicht gefunden. Setze ASHEN_ASSETS oder starte aus dem Projektordner.".into());
            return;
        };
        match Content::load(&assets, &self.settings.enabled_mods) {
            Ok(c) => self.content = Some(c),
            Err(e) => {
                self.fatal = Some(format!("Spieldaten konnten nicht geladen werden: {e}"));
                return;
            }
        }
        let art = match ArtLibrary::build(Some(&assets), &engine.gpu) {
            Ok(a) => a,
            Err(e) => {
                self.fatal = Some(format!("Grafik konnte nicht erzeugt werden: {e}"));
                return;
            }
        };
        if let Some(path) = std::env::var_os("ASHEN_DUMP_SHEET")
            && let Err(e) = art.contact_sheet(std::path::Path::new(&path))
        {
            log::warn!("Kontaktbogen fehlgeschlagen: {e}");
        }
        if let Some(path) = std::env::var_os("ASHEN_DUMP_ATLAS") {
            match art.atlas.save_png(std::path::Path::new(&path)) {
                Ok(()) => log::info!("Atlas gespeichert: {}", std::path::Path::new(&path).display()),
                Err(e) => log::warn!("Atlas-Export fehlgeschlagen: {e}"),
            }
        }
        engine.renderer.set_atlas(&engine.gpu, art.atlas.size, &art.atlas.albedo, &art.atlas.normal);
        let tex = engine.ui.register_texture(&engine.gpu, engine.renderer.atlas_view());
        self.atlas_tex = Some(tex);
        self.art = Some(art);
        self.open_menu(engine);
        self.apply_auto(engine);
    }

    fn update(&mut self, engine: &mut Engine, dt: f32) {
        let (Some(content), Some(art)) = (&mut self.content, &self.art) else { return };
        match &mut self.scene {
            Some(Scene::Menu(m)) => {
                m.update(content, dt);
                engine.audio.set_music_intensity(ashen_engine::audio::Intensity::Menu);
                let action = std::mem::replace(&mut m.action, MenuAction::None);
                match action {
                    MenuAction::None => {}
                    MenuAction::Start { mission, settings } => self.start_mission(engine, mission, settings),
                    MenuAction::Load(p) => match MissionScene::load(&p, &content.data, Self::max_particles(engine)) {
                        Ok(s) => self.scene = Some(Scene::Mission(Box::new(s))),
                        Err(e) => m.error = Some(e),
                    },
                    MenuAction::ReloadContent => {
                        if let Some(assets) = engine.assets_dir.clone() {
                            match Content::load(&assets, &self.settings.enabled_mods) {
                                Ok(c) => {
                                    *content = c;
                                    let page = m.page;
                                    let mut nm = MenuScene::new(content, Self::max_particles(engine));
                                    nm.page = page;
                                    self.scene = Some(Scene::Menu(Box::new(nm)));
                                }
                                Err(e) => m.error = Some(e),
                            }
                        }
                    }
                    MenuAction::Quit => engine.request(WindowRequest::Exit),
                }
            }
            Some(Scene::Mission(s)) => {
                s.update(engine, &content.data, art, &self.settings, dt);
                let action = std::mem::replace(&mut s.action, SceneAction::None);
                match action {
                    SceneAction::None => {}
                    SceneAction::MainMenu => self.open_menu(engine),
                    SceneAction::Restart => {
                        if let Some((i, ms)) = self.last_mission.clone() {
                            self.start_mission(engine, i, ms);
                        }
                    }
                    SceneAction::Load(p) => match MissionScene::load(&p, &content.data, Self::max_particles(engine)) {
                        Ok(ns) => self.scene = Some(Scene::Mission(Box::new(ns))),
                        Err(e) => s.status = Some(format!("Laden fehlgeschlagen: {e}")),
                    },
                    SceneAction::Quit => engine.request(WindowRequest::Exit),
                }
            }
            None => {}
        }
    }

    fn ui(&mut self, engine: &mut Engine, ui: &mut egui::Ui) {
        if let Some(e) = &self.fatal {
            egui::Window::new("Fehler").show(ui.ctx(), |ui| {
                ui.label(RichText::new(e).color(theme::EMBER));
                if ui.button("Beenden").clicked() {
                    engine.request(WindowRequest::Exit);
                }
            });
            return;
        }
        if self.auto.hide_ui {
            return;
        }
        let (Some(content), Some(art), Some(tex), Some(ui_art)) = (&mut self.content, &self.art, self.atlas_tex, &self.ui_art) else { return };
        match &mut self.scene {
            Some(Scene::Menu(m)) => m.ui(ui, engine, content, &mut self.settings, ui_art),
            Some(Scene::Mission(s)) => {
                let mut cx = HudCtx { data: &content.data, art, tex, ui: ui_art, settings: &mut self.settings };
                hud::draw(s, ui, engine, &mut cx);
            }
            None => {}
        }
        if self.settings.show_fps {
            let st = engine.renderer.stats;
            egui::Area::new(egui::Id::unique("fps")).anchor(egui::Align2::LEFT_TOP, egui::vec2(8.0, 44.0)).show(ui.ctx(), |ui| {
                ui.label(RichText::new(format!(
                    "{:.0} FPS · {:.1} ms (max {:.1}) · {} Sprites · {} Lichter · {} Draws · intern {}×{} ×{}",
                    engine.timer.fps(),
                    engine.timer.avg_ms(),
                    engine.timer.max_ms(),
                    st.sprites,
                    st.lights,
                    st.draw_calls,
                    st.internal.0,
                    st.internal.1,
                    st.pixel_scale
                )).small().color(theme::PARCHMENT_DIM));
            });
        }
    }

    fn render(&mut self, engine: &mut Engine) {
        let (Some(content), Some(art)) = (&self.content, &self.art) else { return };
        match &mut self.scene {
            Some(Scene::Menu(m)) => m.render(engine, content, art),
            Some(Scene::Mission(s)) => s.render(engine, &content.data, art),
            None => {}
        }
    }

    fn on_exit(&mut self, _engine: &mut Engine) {
        self.settings.save();
    }
}
