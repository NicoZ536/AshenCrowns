//! Hauptmenü mit lebendiger Hintergrundszene (Brennhollow in der Dämmerung).

use crate::art::ArtLibrary;
use crate::content::Content;
use crate::settings::{saves_dir, UserSettings};
use crate::ui_common;
use crate::view::{Overlay, WorldView};
use ashen_core::iso::tile_to_world;
use ashen_engine::render::Camera2D;
use ashen_engine::ui::theme;
use ashen_engine::Engine;
use ashen_sim::env::Weather;
use ashen_sim::mission::{Difficulty, GameMode, MatchSettings};
use ashen_sim::SimWorld;
use egui::{Align2, Color32, RichText, Vec2};
use glam::Vec2 as GVec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Main,
    NewGame,
    Load,
    Settings,
    Mods,
    Credits,
}

/// Wunsch des Menüs an das Spiel.
#[derive(Clone, Debug)]
pub enum MenuAction {
    None,
    Start { mission: usize, settings: MatchSettings },
    Load(std::path::PathBuf),
    ReloadContent,
    Quit,
}

pub struct MenuScene {
    pub page: Page,
    background: Option<SimWorld>,
    view: WorldView,
    camera: Camera2D,
    t: f32,
    pub new_mode: GameMode,
    pub new_difficulty: Difficulty,
    pub new_seed: u64,
    pub new_mission: usize,
    pub action: MenuAction,
    pub error: Option<String>,
}

impl MenuScene {
    pub fn new(content: &Content, max_particles: usize) -> MenuScene {
        let mission = content.missions[0].clone();
        let bg = SimWorld::new(&content.data, mission, MatchSettings { mode: GameMode::Sandbox, ..Default::default() }).ok().map(|mut w| {
            w.env.time_of_day = 0.735;
            w.env.weather = Weather::Clear;
            w.env.forced_weather = Some(Weather::Clear);
            w.fog.disabled = true;
            w
        });
        MenuScene {
            page: Page::Main,
            background: bg,
            view: WorldView::new(max_particles),
            camera: Camera2D { center: tile_to_world(GVec2::new(18.0, 50.0)), target_lines: 300.0 },
            t: 0.0,
            new_mode: GameMode::Campaign,
            new_difficulty: Difficulty::Normal,
            new_seed: 897,
            new_mission: 0,
            action: MenuAction::None,
            error: None,
        }
    }

    pub fn update(&mut self, content: &Content, dt: f32) {
        self.t += dt;
        // Langsame Kamerafahrt von Brennhollow zum Fluss.
        let a = (self.t * 0.03).sin() * 0.5 + 0.5;
        let tile = GVec2::new(17.0, 52.0).lerp(GVec2::new(30.0, 41.0), a);
        self.camera.center = tile_to_world(tile);
        if let Some(w) = &mut self.background {
            // Die Szene lebt (Animationen, Rauch), Tageszeit bleibt in der Dämmerung.
            w.env.time_of_day = 0.735 + (self.t * 0.002).sin() * 0.01;
            w.time += dt;
            for (_, u) in w.units.iter_mut() {
                u.activity_time += dt;
            }
            let _ = content;
        }
        self.view.update(dt);
    }

    pub fn render(&mut self, engine: &mut Engine, content: &Content, art: &ArtLibrary) {
        engine.frame.camera = self.camera;
        engine.fow_enabled = false;
        if let Some(w) = &self.background {
            let view = self.camera.view(engine.size, engine.renderer.settings.min_lines);
            self.view.render(w, &content.data, art, &mut engine.frame, &view, 1.0, &Overlay::default(), false);
            engine.frame.atmosphere.vignette = 0.75;
            engine.frame.atmosphere.fog_density = 0.3;
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, engine: &mut Engine, content: &mut Content, settings: &mut UserSettings) {
        let ctx = ui.ctx().clone();
        // Titel.
        egui::Area::new(egui::Id::unique("title")).anchor(Align2::CENTER_TOP, Vec2::new(0.0, 40.0)).show(&ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("ASHEN CROWNS").size(64.0).color(theme::GOLD).strong());
                ui.label(RichText::new("THE AGE OF THE FALLEN").size(20.0).color(theme::PARCHMENT_DIM));
            });
        });
        egui::Area::new(egui::Id::unique("version")).anchor(Align2::RIGHT_BOTTOM, Vec2::new(-10.0, -8.0)).show(&ctx, |ui| {
            ui.label(RichText::new(format!("Vertical Slice {} · eigene Rust-Engine (wgpu)", env!("CARGO_PKG_VERSION"))).small().color(theme::PARCHMENT_DIM));
        });
        let frame = theme::panel_frame().inner_margin(18);
        egui::Window::new("menu").title_bar(false).resizable(false).anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 60.0)).frame(frame).show(&ctx, |ui| {
            match self.page {
                Page::Main => self.main_page(ui),
                Page::NewGame => self.new_game_page(ui, content, settings),
                Page::Load => {
                    ui.label(theme::heading("Spiel laden", 24.0));
                    if let Some(p) = ui_common::save_list(ui, &saves_dir()) {
                        self.action = MenuAction::Load(p);
                    }
                    if ui.button("Zurück").clicked() {
                        self.page = Page::Main;
                    }
                }
                Page::Settings => {
                    ui.label(theme::heading("Einstellungen", 24.0));
                    ui_common::settings_ui(ui, engine, settings);
                    if ui.button("Zurück").clicked() {
                        settings.save();
                        self.page = Page::Main;
                    }
                }
                Page::Mods => self.mods_page(ui, content, settings),
                Page::Credits => {
                    ui.set_max_width(480.0);
                    ui.label(theme::heading("Credits", 24.0));
                    ui.label("ASHEN CROWNS – The Age of the Fallen");
                    ui.label("Eigenständiges Dark-Fantasy-Strategiespiel. Eigene Engine in Rust (wgpu, winit, egui, kira).");
                    ui.label("Alle Grafiken der Vertical Slice werden zur Laufzeit prozedural als Pixel-Art erzeugt; handgezeichnete Sprites können über assets/sprites/ eingespielt werden.");
                    ui.label("Stimmen: ElevenLabs · Musik: Suno (Produktionsleitfäden unter docs/audio/).");
                    ui.label(RichText::new("Welt, Figuren und Geschichte von Eldraeth sind Originalinhalte.").small().color(theme::PARCHMENT_DIM));
                    if ui.button("Zurück").clicked() {
                        self.page = Page::Main;
                    }
                }
            }
            if let Some(e) = &self.error {
                ui.label(RichText::new(e).color(theme::EMBER));
            }
        });
    }

    fn main_page(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.set_width(300.0);
            let b = |ui: &mut egui::Ui, t: &str, enabled: bool, tip: &str| ui.add_enabled(enabled, egui::Button::new(RichText::new(t).size(17.0)).min_size(egui::vec2(280.0, 36.0))).on_hover_text(tip).on_disabled_hover_text(tip).clicked();
            if b(ui, "Kampagne", true, "Zeitalter I – Mission 1: Die letzte Siedlung") {
                self.action = MenuAction::Start { mission: 0, settings: MatchSettings { mode: GameMode::Campaign, difficulty: self.new_difficulty, seed: 897, hero_permadeath: false } };
            }
            if b(ui, "Neues Spiel", true, "Modus, Schwierigkeit und Seed wählen") {
                self.page = Page::NewGame;
            }
            if b(ui, "Spiel laden", true, "Gespeicherte Partien") {
                self.page = Page::Load;
            }
            if b(ui, "Gefecht (Skirmish)", true, "Schneller Kampf: Zerstört den Hohlschrein") {
                self.action = MenuAction::Start { mission: 0, settings: MatchSettings { mode: GameMode::Skirmish, difficulty: self.new_difficulty, seed: self.new_seed, hero_permadeath: false } };
            }
            b(ui, "Grand Campaign", false, "Geplant: Weltkarte, Dynastien und Diplomatie (Phasen 3, 8, 10 der Roadmap)");
            if b(ui, "Sandbox", true, "Freies Bauen ohne Angriffe, reichlich Rohstoffe, kein Kriegsnebel") {
                self.action = MenuAction::Start { mission: 0, settings: MatchSettings { mode: GameMode::Sandbox, difficulty: Difficulty::Normal, seed: self.new_seed, hero_permadeath: false } };
            }
            b(ui, "Karteneditor", false, "Geplant: Phase 14 der Roadmap");
            if b(ui, "Modifikationen", true, "Daten-Mods aktivieren oder deaktivieren") {
                self.page = Page::Mods;
            }
            if b(ui, "Einstellungen", true, "Grafik, Audio, Steuerung") {
                self.page = Page::Settings;
            }
            if b(ui, "Credits", true, "") {
                self.page = Page::Credits;
            }
            if b(ui, "Beenden", true, "") {
                self.action = MenuAction::Quit;
            }
        });
    }

    fn new_game_page(&mut self, ui: &mut egui::Ui, content: &Content, settings: &mut UserSettings) {
        ui.set_width(420.0);
        ui.label(theme::heading("Neues Spiel", 24.0));
        egui::Grid::new("newgame").num_columns(2).spacing([16.0, 8.0]).show(ui, |ui| {
            ui.label("Mission");
            egui::ComboBox::from_id_salt("mission").selected_text(&content.missions[self.new_mission.min(content.missions.len() - 1)].name).show_ui(ui, |ui| {
                for (i, m) in content.missions.iter().enumerate() {
                    ui.selectable_value(&mut self.new_mission, i, &m.name);
                }
            });
            ui.end_row();
            ui.label("Modus");
            ui.horizontal(|ui| {
                for m in [GameMode::Campaign, GameMode::Skirmish, GameMode::Survival, GameMode::Sandbox] {
                    ui.selectable_value(&mut self.new_mode, m, m.label());
                }
            });
            ui.end_row();
            ui.label("Schwierigkeit");
            ui.horizontal(|ui| {
                for d in [Difficulty::Story, Difficulty::Normal, Difficulty::Hard] {
                    ui.selectable_value(&mut self.new_difficulty, d, d.label());
                }
            });
            ui.end_row();
            ui.label("Dauerhafter Heldentod");
            ui.checkbox(&mut settings.hero_permadeath, "");
            ui.end_row();
            ui.label("Seed (Wetter/Variation)");
            ui.add(egui::DragValue::new(&mut self.new_seed));
            ui.end_row();
        });
        let m = &content.missions[self.new_mission.min(content.missions.len() - 1)];
        ui.label(RichText::new(&m.description).small().color(theme::PARCHMENT_DIM));
        ui.horizontal(|ui| {
            if ui.button(RichText::new("Starten").size(17.0)).clicked() {
                self.action = MenuAction::Start {
                    mission: self.new_mission,
                    settings: MatchSettings { mode: self.new_mode, difficulty: self.new_difficulty, seed: self.new_seed, hero_permadeath: settings.hero_permadeath },
                };
            }
            if ui.button("Zurück").clicked() {
                self.page = Page::Main;
            }
        });
    }

    fn mods_page(&mut self, ui: &mut egui::Ui, content: &Content, settings: &mut UserSettings) {
        ui.set_width(460.0);
        ui.label(theme::heading("Modifikationen", 24.0));
        if content.mods.is_empty() {
            ui.label("Keine Mods gefunden (Ordner mods/ oder Nutzerverzeichnis/mods).");
        }
        let mut changed = false;
        for m in &content.mods {
            let mut on = settings.enabled_mods.contains(&m.id);
            ui.horizontal(|ui| {
                if ui.checkbox(&mut on, RichText::new(format!("{} {}", m.name, m.version)).strong()).changed() {
                    changed = true;
                    if on {
                        settings.enabled_mods.push(m.id.clone());
                    } else {
                        settings.enabled_mods.retain(|x| x != &m.id);
                    }
                }
                if content.active_mods.contains(&m.id) {
                    ui.label(RichText::new("aktiv").color(Color32::from_rgb(120, 200, 110)));
                }
            });
            ui.label(RichText::new(format!("{} – {}", m.author, m.description)).small().color(theme::PARCHMENT_DIM));
        }
        for e in &content.errors {
            ui.label(RichText::new(e).color(theme::EMBER).small());
        }
        if changed {
            settings.save();
            self.action = MenuAction::ReloadContent;
        }
        if ui.button("Zurück").clicked() {
            self.page = Page::Main;
        }
    }
}
