//! Das Spiel: verbindet Engine, Inhalte, Grafik und Szenen.

use crate::art::ArtLibrary;
use crate::content::Content;
use crate::editor::{EditorAction, EditorInit, EditorScene};
use crate::hud::{self, HudCtx};
use crate::menu::{MenuAction, MenuScene, Page};
use crate::play_modes::{self, ChallengeReport};
use crate::mission::{MissionScene, SceneAction};
use crate::settings::UserSettings;
use crate::ui_art::UiArt;
use ashen_engine::ui::theme;
use ashen_engine::{Engine, Game, WindowRequest};
use ashen_sim::challenge::ChallengeDef;
use ashen_sim::mission::{MatchSettings, MissionDef};
use egui::RichText;

pub enum Scene {
    Menu(Box<MenuScene>),
    Mission(Box<MissionScene>),
    Editor(Box<EditorScene>),
}

/// Was beim Neustart erneut gestartet wird.
#[derive(Clone)]
struct LastRun {
    mission: MissionDef,
    settings: MatchSettings,
    challenge: Option<(ChallengeDef, bool)>,
}

/// Laufende Herausforderung samt Auswertungsstand.
struct ActiveChallenge {
    def: ChallengeDef,
    iron_man: bool,
    reported: bool,
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
    pub demo_city: bool,
    /// Militär-Vorführung: `army`, `siege` oder `hero`.
    pub demo: Option<String>,
    /// Fenster für Screenshots öffnen (`storage`, `city`, `build[:Rubrik]`, `select:<id>`).
    pub ui_open: Vec<String>,
    /// Szenario (ID) statt der Kampagnenmission starten.
    pub scenario: Option<String>,
    /// Herausforderung (ID) starten.
    pub challenge: Option<String>,
    /// Ergebnis erzwingen (`victory`, `defeat`), um die Auswertung zu zeigen.
    pub outcome: Option<String>,
    /// Karteneditor statt Mission öffnen.
    pub editor: bool,
    /// Werkzeug des Editors (`terrain`, `trees`, `building`, …).
    pub editor_tool: Option<String>,
    /// Dialog des Editors (`data`, `new`, `load`).
    pub editor_dialog: Option<String>,
    /// Registerkarte der Missionsdaten (`general`, `objectives`, `triggers`, `rules`, `areas`, `validation`).
    pub editor_tab: Option<String>,
    /// Editor mit einigen Pinselstrichen füllen (für Screenshots).
    pub editor_demo: bool,
    /// Menüseite öffnen (`scenarios`, `challenges`, `skirmish`).
    pub menu_page: Option<String>,
    /// Szenario in der Liste vorwählen (Index) beziehungsweise Herausforderung.
    pub menu_select: usize,
}

pub struct AshenGame {
    pub settings: UserSettings,
    content: Option<Content>,
    art: Option<ArtLibrary>,
    atlas_tex: Option<egui::TextureId>,
    ui_art: Option<UiArt>,
    scene: Option<Scene>,
    last_run: Option<LastRun>,
    challenge: Option<ActiveChallenge>,
    report: Option<ChallengeReport>,
    /// Editor, der während eines Testspiels im Hintergrund wartet.
    parked_editor: Option<Box<EditorScene>>,
    auto: AutoRun,
    fatal: Option<String>,
}

impl AshenGame {
    pub fn new(settings: UserSettings, auto: AutoRun) -> AshenGame {
        AshenGame { settings, content: None, art: None, atlas_tex: None, ui_art: None, scene: None, last_run: None, challenge: None, report: None, parked_editor: None, auto, fatal: None }
    }

    fn max_particles(engine: &Engine) -> usize {
        engine.renderer.settings.max_particles
    }

    fn start_mission(&mut self, engine: &mut Engine, index: usize, settings: MatchSettings) {
        let Some(content) = &self.content else { return };
        let mission = content.missions[index.min(content.missions.len() - 1)].clone();
        if let Err(e) = self.start_def(engine, mission, settings, None) {
            self.fatal = Some(e);
        }
    }

    /// Startet eine Mission aus Daten. Ein wartender Editor wird für das Testspiel geparkt.
    fn start_def(&mut self, engine: &mut Engine, mission: MissionDef, settings: MatchSettings, challenge: Option<(ChallengeDef, bool)>) -> Result<(), String> {
        let Some(content) = &self.content else { return Err("Keine Spieldaten geladen.".into()) };
        let mut scene = MissionScene::new(&content.data, mission.clone(), settings.clone(), Self::max_particles(engine))?;
        scene.no_save = challenge.as_ref().is_some_and(|c| c.1);
        self.last_run = Some(LastRun { mission, settings, challenge: challenge.clone() });
        self.challenge = challenge.map(|(def, iron_man)| ActiveChallenge { def, iron_man, reported: false });
        self.report = None;
        engine.audio.stop_music(1.5);
        if let Some(Scene::Editor(e)) = self.scene.replace(Scene::Mission(Box::new(scene))) {
            self.parked_editor = Some(e);
        }
        Ok(())
    }

    fn open_menu(&mut self, engine: &mut Engine) {
        if let Some(c) = &self.content {
            engine.audio.stop_music(1.0);
            engine.audio.set_ambience(&[]);
            engine.renderer.clear_fog_of_war();
            self.parked_editor = None;
            self.challenge = None;
            self.report = None;
            self.scene = Some(Scene::Menu(Box::new(MenuScene::new(c, Self::max_particles(engine)))));
        }
    }

    fn apply_auto(&mut self, engine: &mut Engine) {
        if self.auto.editor {
            self.apply_auto_editor(engine);
            return;
        }
        if let Some(page) = self.auto.menu_page.clone() {
            self.apply_auto_page(&page);
            return;
        }
        if let Some(id) = self.auto.scenario.clone() {
            let Some(content) = &self.content else { return };
            let Some(def) = content.find_base(&id) else {
                self.fatal = Some(format!("Szenario '{id}' nicht gefunden."));
                return;
            };
            let settings = MatchSettings { mode: ashen_sim::mission::GameMode::Campaign, seed: def.map_seed, ..Default::default() };
            if let Err(e) = self.start_def(engine, def, settings, None) {
                self.fatal = Some(e);
                return;
            }
        } else if let Some(id) = self.auto.challenge.clone() {
            let Some(content) = &self.content else { return };
            let Some(def) = content.challenges.iter().find(|c| c.id == id).cloned() else {
                self.fatal = Some(format!("Herausforderung '{id}' nicht gefunden."));
                return;
            };
            let Some(base) = content.find_base(&def.base) else { return };
            let p = ashen_sim::challenge::prepare(&def, &base);
            if let Err(e) = self.start_def(engine, p.mission, p.settings, Some((def, p.iron_man))) {
                self.fatal = Some(e);
                return;
            }
        } else {
            let Some(ms) = self.auto.start_mission.clone() else { return };
            self.start_mission(engine, 0, ms);
        }
        let (Some(Scene::Mission(m)), Some(content)) = (&mut self.scene, &self.content) else { return };
        let data = &content.data;
        if let Some(w) = self.auto.weather {
            m.world.env.weather = w;
            m.world.env.forced_weather = Some(w);
            m.world.env.weather_intensity = 1.0;
        }
        if self.auto.demo_city {
            crate::showcase::city(&mut m.world, data);
        }
        match self.auto.demo.as_deref() {
            Some("army") => crate::showcase::army(&mut m.world, data),
            Some("siege") => crate::showcase::siege(&mut m.world, data),
            Some("hero") => crate::showcase::hero(&mut m.world, data),
            // Kriegsnebel aus (z. B. um die Basis der Gefechts-KI zu zeigen).
            Some("reveal") => {
                m.world.fog.disabled = true;
                m.fog_enabled = false;
            }
            _ => {}
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
        for what in &self.auto.ui_open {
            let (key, arg) = what.split_once(':').unwrap_or((what.as_str(), ""));
            match key {
                "storage" => m.show_storage = true,
                "city" => m.show_city = true,
                "build" => {
                    m.mode = crate::mission::CommandMode::BuildMenu;
                    if let Some(c) = ashen_sim::data::BuildCategory::ALL.iter().find(|c| format!("{c:?}").eq_ignore_ascii_case(arg)) {
                        m.build_cat = *c;
                    }
                }
                "hero" => m.show_hero = true,
                "select" => {
                    m.selected_building = m.world.buildings.iter().find(|(_, b)| b.owner == ashen_sim::entity::PLAYER && data.building(b.kind).id == arg).map(|(id, _)| id);
                }
                _ => {}
            }
        }
        match self.auto.outcome.as_deref() {
            Some("victory") => m.world.mission.outcome = Some(ashen_sim::mission::Outcome::Victory),
            Some("defeat") => m.world.mission.outcome = Some(ashen_sim::mission::Outcome::Defeat),
            _ => {}
        }
        if self.auto.select_army {
            m.selection = m.world.units.iter().filter(|(_, u)| u.owner == ashen_sim::entity::PLAYER && data.unit(u.kind).class != ashen_sim::data::UnitClass::Worker).map(|(id, _)| id).collect();
        }
    }
}

impl AshenGame {
    fn apply_auto_page(&mut self, page: &str) {
        let (Some(Scene::Menu(m)), Some(content)) = (&mut self.scene, &self.content) else { return };
        m.modes.refresh_scenarios(content);
        m.modes.refresh_challenges();
        m.modes.scen_sel = self.auto.menu_select;
        m.modes.chal_sel = self.auto.menu_select;
        m.page = match page {
            "scenarios" => Page::Scenarios,
            "challenges" => Page::Challenges,
            _ => Page::Skirmish,
        };
        if page == "scenarios-user" {
            m.page = Page::Scenarios;
        }
    }

    fn apply_auto_editor(&mut self, engine: &mut Engine) {
        let Some(content) = &self.content else { return };
        let init = EditorInit::New { width: 72, height: 72, seed: 897, preset: ashen_sim::genmap::GenPreset::Aschmark, blank: false };
        match EditorScene::new(content, init, Self::max_particles(engine)) {
            Ok(mut e) => {
                e.apply_auto(&content.data, &self.auto.editor_tool, &self.auto.editor_dialog, &self.auto.editor_tab, self.auto.editor_demo);
                if let Some((x, y)) = self.auto.camera_tile {
                    e.center_on(glam::Vec2::new(x, y));
                    e.forced_cursor = Some(glam::Vec2::new(x + 1.5, y + 1.5));
                }
                self.scene = Some(Scene::Editor(Box::new(e)));
            }
            Err(err) => self.fatal = Some(err),
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
                    MenuAction::StartDef { mission, settings, challenge } => {
                        if let Err(e) = self.start_def(engine, *mission, settings, challenge.map(|c| *c))
                            && let Some(Scene::Menu(m)) = &mut self.scene
                        {
                            m.error = Some(e);
                        }
                    }
                    MenuAction::OpenEditor(init) => match EditorScene::new(content, init, Self::max_particles(engine)) {
                        Ok(e) => self.scene = Some(Scene::Editor(Box::new(e))),
                        Err(e) => m.error = Some(e),
                    },
                    MenuAction::Load(p) => match MissionScene::load(&p, &content.data, Self::max_particles(engine)) {
                        Ok(s) => {
                            self.challenge = None;
                            self.scene = Some(Scene::Mission(Box::new(s)));
                        }
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
                if let Some(ch) = &mut self.challenge
                    && !ch.reported
                    && s.world.mission.outcome.is_some()
                {
                    ch.reported = true;
                    self.report = play_modes::finish_challenge(&ch.def, &s.world);
                }
                let action = std::mem::replace(&mut s.action, SceneAction::None);
                match action {
                    SceneAction::None => {}
                    SceneAction::MainMenu => {
                        if let Some(mut e) = self.parked_editor.take() {
                            engine.audio.stop_music(1.0);
                            engine.audio.set_ambience(&[]);
                            engine.renderer.clear_fog_of_war();
                            e.notify("Zurück vom Testspiel.", false);
                            self.challenge = None;
                            self.report = None;
                            self.scene = Some(Scene::Editor(e));
                        } else {
                            self.open_menu(engine);
                        }
                    }
                    SceneAction::Restart => {
                        if let Some(r) = self.last_run.clone()
                            && let Err(e) = self.start_def(engine, r.mission, r.settings, r.challenge)
                        {
                            self.fatal = Some(e);
                        }
                    }
                    SceneAction::Load(_) if self.challenge.as_ref().is_some_and(|c| c.iron_man) => {
                        s.status = Some("Eiserner Mann: Laden ist gesperrt.".into());
                    }
                    SceneAction::Load(p) => match MissionScene::load(&p, &content.data, Self::max_particles(engine)) {
                        Ok(ns) => {
                            self.challenge = None;
                            self.scene = Some(Scene::Mission(Box::new(ns)));
                        }
                        Err(e) => s.status = Some(format!("Laden fehlgeschlagen: {e}")),
                    },
                    SceneAction::Quit => engine.request(WindowRequest::Exit),
                }
            }
            Some(Scene::Editor(e)) => {
                e.update(engine, content, art, &self.settings, dt);
                engine.audio.set_music_intensity(ashen_engine::audio::Intensity::Menu);
                match std::mem::replace(&mut e.action, EditorAction::None) {
                    EditorAction::None => {}
                    EditorAction::Exit => self.open_menu(engine),
                    EditorAction::Test(def) => {
                        let settings = MatchSettings { mode: ashen_sim::mission::GameMode::Campaign, difficulty: self.settings.difficulty, seed: def.map_seed, hero_permadeath: false };
                        if let Err(err) = self.start_def(engine, *def, settings, None)
                            && let Some(Scene::Editor(e)) = &mut self.scene
                        {
                            e.notify(format!("Testspiel nicht möglich: {err}"), true);
                        }
                    }
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
                if let Some(r) = &self.report {
                    draw_report(ui.ctx(), ui_art, r);
                }
            }
            Some(Scene::Editor(e)) => e.ui(ui, engine, content, ui_art, art, tex),
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
            Some(Scene::Editor(e)) => e.render(engine, content, art),
            None => {}
        }
    }

    fn on_exit(&mut self, _engine: &mut Engine) {
        self.settings.save();
    }
}

/// Ergebnisfenster einer Herausforderung (links neben dem Siegbildschirm).
fn draw_report(ctx: &egui::Context, art: &UiArt, r: &ChallengeReport) {
    use crate::ui_art::PanelKind;
    egui::Area::new(egui::Id::unique("challenge_report")).order(egui::Order::Foreground).anchor(egui::Align2::LEFT_CENTER, egui::vec2(18.0, 0.0)).show(ctx, |ui| {
        art.panel(ui, PanelKind::Ornate, egui::Margin::symmetric(34, 32), |ui| {
            ui.set_width(270.0);
            ui.label(RichText::new("Herausforderung").font(theme::caps_font(13.0)).color(theme::PARCHMENT_DIM));
            ui.label(RichText::new(&r.name).font(theme::heading_font(18.0)).color(theme::GOLD));
            ui.add_space(4.0);
            let res = &r.result;
            let verdict = if res.victory { "Bestanden" } else { "Gescheitert" };
            ui.label(RichText::new(verdict).font(theme::heading_font(16.0)).color(if res.victory { egui::Color32::from_rgb(150, 220, 130) } else { theme::EMBER }));
            ui.label(RichText::new(format!("{} Punkte · Zeit {}", res.score, ashen_sim::challenge::format_time(res.time))).font(theme::body_bold_font(16.0)).color(theme::PARCHMENT));
            ui.label(RichText::new(format!("Gefallene Feinde {} · Verluste {} · Nächte {} · Wellen {}", res.kills, res.units_lost, res.nights, res.waves)).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
            ui.add_space(6.0);
            match r.rank {
                Some(0) if r.new_best => {
                    ui.label(RichText::new("Neuer Bestwert!").font(theme::heading_font(16.0)).color(theme::EMBER_HOT));
                }
                Some(i) => {
                    ui.label(RichText::new(format!("Platz {} der Bestenliste", i + 1)).font(theme::body_bold_font(15.0)).color(theme::GOLD));
                }
                None => {
                    ui.label(RichText::new("Nicht in der Bestenliste").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
                }
            }
            ui.add_space(4.0);
            for (i, e) in r.top.iter().enumerate() {
                ui.label(RichText::new(format!("{}. {}", i + 1, play_modes::entry_text(r.rank_by, e))).font(theme::body_font(14.0)).color(theme::PARCHMENT));
            }
        });
    });
}
