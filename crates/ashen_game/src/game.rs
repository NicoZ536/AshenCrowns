//! Das Spiel: verbindet Engine, Inhalte, Grafik und Szenen.

use crate::art::ArtLibrary;
use crate::content::Content;
use crate::hud::{self, HudCtx};
use crate::menu::{MenuAction, MenuScene};
use crate::mission::{MissionScene, SceneAction};
use crate::settings::UserSettings;
use crate::ui_art::UiArt;
use crate::worldmap::battle::{self, BattleLink};
use crate::worldmap::battle_ui::{self, BattleCtx, BattleUiAction};
use crate::worldmap::{store, WorldAction, WorldAuto, WorldMapScene};
use ashen_engine::ui::theme;
use ashen_engine::{Engine, Game, WindowRequest};
use ashen_sim::mission::MatchSettings;
use ashen_strategy::{BattleMode, StrategyOptions, StrategyWorld};
use egui::RichText;

pub enum Scene {
    Menu(Box<MenuScene>),
    Mission(Box<MissionScene>),
    /// Weltkarte der Großen Kampagne.
    WorldMap(Box<WorldMapScene>),
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
    /// Fenster für Screenshots öffnen (`storage`, `city`, `build[:Rubrik]`, `select:<id>`; auf der Weltkarte `province`, `diplomacy` …).
    pub ui_open: Vec<String>,
    /// Weltkarte der Großen Kampagne (`--scene worldmap`).
    pub world: Option<WorldAuto>,
    pub world_seed: u64,
    /// Aus einer Weltkarten-Schlacht erzeugte Echtzeitschlacht (`--scene battle`); `true` = Belagerung.
    pub battle_demo: Option<bool>,
}

pub struct AshenGame {
    pub settings: UserSettings,
    content: Option<Content>,
    art: Option<ArtLibrary>,
    atlas_tex: Option<egui::TextureId>,
    ui_art: Option<UiArt>,
    scene: Option<Scene>,
    last_mission: Option<(usize, MatchSettings)>,
    /// Laufende Schlacht aus der Großen Kampagne (die Weltkartenszene ruht darin).
    battle: Option<BattleCtx>,
    battle_action: Option<BattleUiAction>,
    auto: AutoRun,
    fatal: Option<String>,
}

enum Deferred {
    None,
    PlayBattle(u32),
    LoadStrategy(std::path::PathBuf),
    MainMenu,
}

impl AshenGame {
    pub fn new(settings: UserSettings, auto: AutoRun) -> AshenGame {
        AshenGame { settings, content: None, art: None, atlas_tex: None, ui_art: None, scene: None, last_mission: None, battle: None, battle_action: None, auto, fatal: None }
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
                self.battle = None;
                engine.audio.stop_music(1.5);
                self.scene = Some(Scene::Mission(Box::new(s)));
            }
            Err(e) => self.fatal = Some(e),
        }
    }

    fn open_menu(&mut self, engine: &mut Engine) {
        if let Some(c) = &self.content {
            self.battle = None;
            engine.audio.stop_music(1.0);
            engine.audio.set_ambience(&[]);
            engine.renderer.clear_fog_of_war();
            self.scene = Some(Scene::Menu(Box::new(MenuScene::new(c, Self::max_particles(engine)))));
        }
    }

    /// Echtzeit-Szene betreten; gehört der Spielstand zu einer Weltkarten-Schlacht, wird der Rückkehrpunkt wiederhergestellt.
    fn enter_mission(&mut self, engine: &mut Engine, mut scene: MissionScene) {
        self.battle = None;
        if let Some(link) = scene.battle.clone() {
            match store::load_return_point(&link.return_point) {
                Ok(sw) => {
                    let ws = WorldMapScene::new(sw);
                    match ws.world.pending_battles().iter().find(|b| b.id == link.battle_id).cloned() {
                        Some(req) => {
                            let player_attacks = req.attackers.iter().any(|a| a.realm == Some(ws.me()));
                            self.battle = Some(BattleCtx { world: Box::new(ws), req, scale: link.scale, player_attacks, report: None, enemy_timer: 0.0 });
                        }
                        None => scene.status = Some("Die zugehörige Schlacht der Weltkarte ist bereits entschieden.".into()),
                    }
                }
                Err(e) => scene.status = Some(format!("Rückkehrpunkt der Weltkarte fehlt: {e}")),
            }
        }
        engine.audio.stop_music(1.0);
        self.scene = Some(Scene::Mission(Box::new(scene)));
    }

    fn enter_worldmap(&mut self, engine: &mut Engine, world: StrategyWorld) {
        self.battle = None;
        engine.audio.stop_music(1.0);
        engine.audio.set_ambience(&[]);
        engine.renderer.clear_fog_of_war();
        self.scene = Some(Scene::WorldMap(Box::new(WorldMapScene::new(world))));
    }

    /// Echtzeitschlacht aus einer wartenden Weltkarten-Schlacht erzeugen und starten.
    fn start_battle_scene(&mut self, engine: &mut Engine, mut ws: Box<WorldMapScene>, id: u32) {
        let max_particles = Self::max_particles(engine);
        let Some(content) = &self.content else {
            self.scene = Some(Scene::WorldMap(ws));
            return;
        };
        let Some(req) = ws.world.pending_battles().iter().find(|b| b.id == id).cloned() else {
            ws.last_result = Some((false, "Schlacht nicht gefunden".into()));
            self.scene = Some(Scene::WorldMap(ws));
            return;
        };
        let me = ws.me();
        let built = battle::plan_battle(&content.data, me, &req).and_then(|plan| {
            let (scale, attacks, focus) = (plan.scale, plan.player_attacks, plan.player_focus);
            battle::create_world(&content.data, plan, &req).map(|w| (w, scale, attacks, focus))
        });
        let (world, scale, player_attacks, focus) = match built {
            Ok(b) => b,
            Err(e) => {
                ws.last_result = Some((false, format!("Schlacht konnte nicht erzeugt werden: {e}")));
                self.scene = Some(Scene::WorldMap(ws));
                return;
            }
        };
        // Rückkehrpunkt: strategischer Zustand mit der noch offenen Schlachtanfrage.
        let return_point = match store::write_return_point(&ws.world, id) {
            Ok(n) => n,
            Err(e) => {
                log::warn!("Rückkehrpunkt konnte nicht geschrieben werden: {e}");
                String::new()
            }
        };
        let link = BattleLink { return_point, battle_id: id, scale };
        let mut m = MissionScene::from_world(&content.data, world, max_particles);
        m.battle = Some(link);
        m.center_on(focus);
        m.speed = 1.0;
        m.save_name = format!("schlacht_{}", req.province_name.to_lowercase().replace(' ', "_"));
        engine.audio.stop_music(1.0);
        self.battle = Some(BattleCtx { world: ws, req, scale, player_attacks, report: None, enemy_timer: 0.0 });
        self.scene = Some(Scene::Mission(Box::new(m)));
    }

    /// Schlacht beenden und zur Weltkarte zurückkehren (Ergebnis anwenden oder automatisch auflösen).
    fn finish_battle(&mut self, engine: &mut Engine, auto_resolve: bool) {
        let Some(mut bc) = self.battle.take() else { return };
        let id = bc.req.id;
        let note = if auto_resolve {
            match bc.world.world.auto_resolve_pending(id) {
                Ok(()) => "Die Schlacht wurde automatisch aufgelöst.".to_string(),
                Err(e) => format!("Auflösung fehlgeschlagen: {e}"),
            }
        } else if let Some(rep) = bc.report.take() {
            match bc.world.world.apply_battle_result(rep.result) {
                Ok(()) => if rep.player_won { "Sieg! Das Ergebnis der Schlacht wurde übernommen.".to_string() } else { "Niederlage. Das Ergebnis der Schlacht wurde übernommen.".to_string() },
                Err(e) => format!("Ergebnis konnte nicht übernommen werden: {e}"),
            }
        } else {
            "Schlacht abgebrochen.".to_string()
        };
        bc.world.process_events(engine);
        bc.world.status = Some(note.clone());
        bc.world.battle_note = Some(note.clone());
        bc.world.toast(note);
        bc.world.speed = 0;
        let _ = store::autosave(&bc.world.world);
        engine.audio.stop_music(1.0);
        self.scene = Some(Scene::WorldMap(bc.world));
    }

    /// Aufbau für Screenshots: Weltkarte der Großen Kampagne.
    fn start_world_auto(&mut self, engine: &mut Engine, wa: &WorldAuto) {
        let mut opts = StrategyOptions::campaign(self.auto.world_seed);
        opts.rules.battle_mode = BattleMode::PlayerBattlesRequested;
        let mut ws = Box::new(WorldMapScene::new(StrategyWorld::new(opts)));
        ws.apply_auto(wa);
        let _ = engine;
        self.scene = Some(Scene::WorldMap(ws));
    }

    /// Aufbau für Screenshots: Echtzeitschlacht aus einer Weltkarten-Schlacht.
    fn start_battle_demo(&mut self, engine: &mut Engine, siege: bool) {
        let mut opts = StrategyOptions::campaign(self.auto.world_seed);
        opts.rules.battle_mode = BattleMode::PlayerBattlesRequested;
        let mut ws = Box::new(WorldMapScene::new(StrategyWorld::new(opts)));
        let mut wa = self.auto.world.clone().unwrap_or_default();
        wa.battle_dialog = true;
        ws.apply_auto(&wa);
        let Some(id) = ws.world.pending_battles().first().map(|b| b.id) else {
            self.fatal = Some("Keine Schlacht erzeugt".into());
            return;
        };
        if siege && let Some(b) = ws.world.battles.iter_mut().find(|b| b.id == id) {
            b.siege = true;
            b.fortification = 6;
            b.breaches = 1;
        }
        self.start_battle_scene(engine, ws, id);
        let (Some(Scene::Mission(m)), Some(content), Some(bc)) = (&mut self.scene, &self.content, &mut self.battle) else { return };
        let data = &content.data;
        // Spielerseite rückt vor, damit der Schlachtverlauf sichtbar ist.
        let target = glam::Vec2::new(53.0, 53.0);
        let ids: Vec<_> = m.world.units.iter().filter(|(_, u)| u.owner == ashen_sim::entity::PLAYER).map(|(i, _)| i).collect();
        for i in ids {
            if let Some(u) = m.world.units.get_mut(i) {
                u.order = ashen_sim::entity::Order::Move { target, attack_move: true };
                u.request_path(target);
            }
        }
        let steps = (self.auto.sim_seconds * data.rules.tick_rate as f32) as u32;
        for k in 0..steps {
            m.world.step(data);
            if k % 30 == 0 {
                battle::drive_enemy(&mut m.world, data, bc.player_attacks);
            }
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
        if let Some(l) = self.auto.zoom_lines {
            m.camera.target_lines = l;
        }
        if let Some((x, y)) = self.auto.camera_tile {
            m.center_on(glam::Vec2::new(x, y));
        }
    }

    fn apply_auto(&mut self, engine: &mut Engine) {
        if let Some(siege) = self.auto.battle_demo {
            self.start_battle_demo(engine, siege);
            return;
        }
        if let Some(wa) = self.auto.world.clone() {
            self.start_world_auto(engine, &wa);
            return;
        }
        let Some(ms) = self.auto.start_mission.clone() else { return };
        self.start_mission(engine, 0, ms);
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
                "select" => {
                    m.selected_building = m.world.buildings.iter().find(|(_, b)| b.owner == ashen_sim::entity::PLAYER && data.building(b.kind).id == arg).map(|(id, _)| id);
                }
                _ => {}
            }
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
        // Szenenwechsel, die `self` als Ganzes brauchen, werden bis nach dem Szenen-Update aufgeschoben.
        let mut deferred = Deferred::None;
        let mut battle_ui = self.battle_action.take();
        {
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
                            Ok(s) => self.enter_mission(engine, s),
                            Err(e) => m.error = Some(e),
                        },
                        MenuAction::NewCampaign(opts) => {
                            let world = StrategyWorld::new(*opts);
                            self.enter_worldmap(engine, world);
                        }
                        MenuAction::LoadStrategy(p) => match store::load(&p) {
                            Ok(w) => self.enter_worldmap(engine, w),
                            Err(e) => m.error = Some(format!("Spielstand konnte nicht geladen werden: {e}")),
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
                    // Weltkarten-Schlacht: Gegnerführung, Siegbedingung und Ergebnis.
                    if let Some(bc) = &mut self.battle {
                        if bc.report.is_none() {
                            bc.enemy_timer += dt;
                            if bc.enemy_timer > 1.5 {
                                bc.enemy_timer = 0.0;
                                battle::drive_enemy(&mut s.world, &content.data, bc.player_attacks);
                            }
                            if let Some(won) = battle::battle_outcome(&s.world, &bc.req, bc.player_attacks) {
                                let me = bc.world.me();
                                let permadeath = bc.world.world.options.rules.permadeath;
                                bc.report = Some(battle::build_result(&s.world, &bc.req, bc.scale, me, won, permadeath));
                                engine.audio.play_sting(if won { "victory_sting" } else { "defeat_sting" });
                            }
                        }
                        if bc.report.is_some() {
                            s.speed = 0.0;
                            s.menu_open = false;
                        }
                        match battle_ui.take() {
                            Some(BattleUiAction::Retreat) if bc.report.is_none() => {
                                let me = bc.world.me();
                                let permadeath = bc.world.world.options.rules.permadeath;
                                bc.report = Some(battle::build_result(&s.world, &bc.req, bc.scale, me, false, permadeath));
                            }
                            Some(BattleUiAction::AutoResolve) if bc.report.is_none() => {
                                self.finish_battle(engine, true);
                                return;
                            }
                            Some(BattleUiAction::Finish) if bc.report.is_some() => {
                                self.finish_battle(engine, false);
                                return;
                            }
                            _ => {}
                        }
                    }
                    let action = std::mem::replace(&mut s.action, SceneAction::None);
                    match action {
                        SceneAction::None => {}
                        SceneAction::MainMenu => deferred = Deferred::MainMenu,
                        SceneAction::Restart => {
                            if let Some(mut bc) = self.battle.take() {
                                bc.report = None;
                                let id = bc.req.id;
                                self.start_battle_scene(engine, bc.world, id);
                            } else if let Some((i, ms)) = self.last_mission.clone() {
                                self.start_mission(engine, i, ms);
                            }
                        }
                        SceneAction::Load(p) => match MissionScene::load(&p, &content.data, Self::max_particles(engine)) {
                            Ok(ns) => self.enter_mission(engine, ns),
                            Err(e) => s.status = Some(format!("Laden fehlgeschlagen: {e}")),
                        },
                        SceneAction::Quit => engine.request(WindowRequest::Exit),
                    }
                }
                Some(Scene::WorldMap(ws)) => {
                    ws.update(engine, &self.settings, dt);
                    match std::mem::replace(&mut ws.action, WorldAction::None) {
                        WorldAction::None => {}
                        WorldAction::MainMenu => deferred = Deferred::MainMenu,
                        WorldAction::Quit => engine.request(WindowRequest::Exit),
                        WorldAction::Load(p) => deferred = Deferred::LoadStrategy(p),
                        WorldAction::PlayBattle(id) => deferred = Deferred::PlayBattle(id),
                    }
                }
                None => {}
            }
        }
        match deferred {
            Deferred::None => {}
            Deferred::MainMenu => self.open_menu(engine),
            Deferred::LoadStrategy(p) => match store::load(&p) {
                Ok(w) => self.enter_worldmap(engine, w),
                Err(e) => {
                    if let Some(Scene::WorldMap(ws)) = &mut self.scene {
                        ws.last_result = Some((false, format!("Laden fehlgeschlagen: {e}")));
                        ws.status = Some(format!("Laden fehlgeschlagen: {e}"));
                    }
                }
            },
            Deferred::PlayBattle(id) => {
                if let Some(Scene::WorldMap(ws)) = self.scene.take() {
                    self.start_battle_scene(engine, ws, id);
                }
            }
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
                if let Some(bc) = &mut self.battle {
                    let a = battle_ui::draw(&ui.ctx().clone(), engine, ui_art, bc, s);
                    if !matches!(a, BattleUiAction::None) {
                        self.battle_action = Some(a);
                    }
                }
            }
            Some(Scene::WorldMap(ws)) => ws.ui(ui, engine, art, tex, ui_art, &mut self.settings),
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
            Some(Scene::WorldMap(_)) => {
                // Die Karte wird vollständig in der Oberflächenebene gezeichnet.
                engine.fow_enabled = false;
            }
            None => {}
        }
    }

    fn on_exit(&mut self, _engine: &mut Engine) {
        self.settings.save();
    }
}
