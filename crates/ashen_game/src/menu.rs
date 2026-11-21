//! Hauptmenü mit lebendiger Hintergrundszene (Brennhollow in der Dämmerung).

use crate::art::ArtLibrary;
use crate::content::Content;
use crate::settings::{saves_dir, UserSettings};
use crate::ui_art::{self, PanelKind, TextFx, UiArt};
use crate::ui_common;
use crate::view::{Overlay, WorldView};
use ashen_core::iso::tile_to_world;
use ashen_engine::render::Camera2D;
use ashen_engine::ui::theme;
use ashen_engine::Engine;
use ashen_sim::env::Weather;
use ashen_sim::mission::{Difficulty, GameMode, MatchSettings};
use ashen_sim::SimWorld;
use egui::{Align2, Color32, Id, Order, Pos2, Rect, RichText, Vec2};
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

    pub fn ui(&mut self, ui: &mut egui::Ui, engine: &mut Engine, content: &mut Content, settings: &mut UserSettings, art: &UiArt) {
        let ctx = ui.ctx().clone();
        let screen = ctx.content_rect();
        let main = self.page == Page::Main;
        let k = ctx.animate_bool_with_time(Id::unique("menu_title_main"), main, 0.35);
        let t = self.t;
        // Hintergrund-Effekte: Vignette, Glutpartikel, Titel.
        let fx = ctx.layer_painter(egui::LayerId::new(Order::Background, Id::unique("menu_fx")));
        for (c, s) in [(screen.left_center(), Vec2::new(screen.width() * 0.55, screen.height() * 1.8)), (screen.right_center(), Vec2::new(screen.width() * 0.55, screen.height() * 1.8)), (screen.center_bottom(), Vec2::new(screen.width() * 1.8, screen.height() * 0.45))] {
            art.soft_shadow(&fx, Rect::from_center_size(c, s), 0.6);
        }
        art.glow(&fx, Rect::from_center_size(screen.center_bottom(), Vec2::new(screen.width() * 1.2, screen.height() * 0.5)), Color32::from_rgb(46, 16, 4));
        ui_art::embers(art, &fx, screen, t, 80, 3);
        let title_size = 54.0 + 34.0 * k;
        let ty = screen.min.y + 74.0 + 44.0 * k;
        art.soft_shadow(&fx, Rect::from_center_size(Pos2::new(screen.center().x, ty), Vec2::new(title_size * 13.0, title_size * 3.2)), 0.75);
        art.glow(&fx, Rect::from_center_size(Pos2::new(screen.center().x, ty - title_size * 0.85), Vec2::new(140.0, 90.0) * (0.6 + 0.4 * k)), Color32::from_rgb(70, 30, 6));
        art.crown(&fx, Pos2::new(screen.center().x, ty - title_size * 0.8 - 4.0), 1.5 + 1.5 * k);
        let pulse = 0.85 + 0.15 * (t * 1.7).sin();
        let mut gold = TextFx::gold();
        gold.glow = Some(Color32::from_rgb((130.0 * pulse) as u8, (54.0 * pulse) as u8, (10.0 * pulse) as u8));
        gold.spacing = title_size * 0.04;
        gold.shadow = 2.0 + 3.0 * k;
        let tr = ui_art::fx_text(art, &fx, Pos2::new(screen.center().x, ty), Align2::CENTER_CENTER, "ASHEN CROWNS", theme::title_font(title_size), &gold);
        let sub = TextFx { top: Color32::from_rgb(226, 210, 178), bottom: Color32::from_rgb(150, 128, 100), outline: Color32::from_rgb(12, 8, 6), glow: None, shadow: 2.0, spacing: 4.0 };
        let sr = ui_art::fx_text(art, &fx, Pos2::new(screen.center().x, tr.max.y + 6.0 + 4.0 * k), Align2::CENTER_TOP, "The Age of the Fallen", theme::title_bold_font(16.0 + 7.0 * k), &sub);
        let dw = (tr.width() * 0.62).min(520.0);
        art.divider(&fx, screen.center().x - dw / 2.0, screen.center().x + dw / 2.0, sr.max.y + 14.0);
        fx.text(screen.right_bottom() + Vec2::new(-14.0, -10.0), Align2::RIGHT_BOTTOM, format!("Vertical Slice {} · eigene Rust-Engine (wgpu)", env!("CARGO_PKG_VERSION")), theme::italic_font(13.0), Color32::from_rgb(150, 136, 114));

        let top = sr.max.y + 34.0;
        egui::Area::new(Id::unique("menu_panel")).anchor(Align2::CENTER_TOP, Vec2::new(0.0, top - screen.min.y)).show(&ctx, |ui| {
            art.panel(ui, PanelKind::Ornate, egui::Margin::symmetric(36, 30), |ui| {
                match self.page {
                    Page::Main => self.main_page(ui, engine, art),
                    Page::NewGame => self.new_game_page(ui, engine, content, settings, art),
                    Page::Load => {
                        ui.set_width(560.0);
                        ui_art::heading(ui, art, "Spiel laden", 26.0);
                        if let Some(p) = ui_common::save_list(ui, engine, art, &saves_dir()) {
                            self.action = MenuAction::Load(p);
                        }
                        self.back(ui, engine, art, None);
                    }
                    Page::Settings => {
                        ui.set_width(600.0);
                        ui_art::heading(ui, art, "Einstellungen", 26.0);
                        ui_common::settings_ui(ui, engine, settings, art);
                        self.back(ui, engine, art, Some(settings));
                    }
                    Page::Mods => self.mods_page(ui, engine, content, settings, art),
                    Page::Credits => {
                        ui.set_width(520.0);
                        ui_art::heading(ui, art, "Credits", 26.0);
                        art.panel(ui, PanelKind::Parchment, egui::Margin { left: 28, right: 28, top: 22, bottom: 22 }, |ui| {
                            ui.set_width(460.0);
                            let h = |ui: &mut egui::Ui, t: &str| {
                                ui.label(RichText::new(t).font(theme::caps_font(14.0)).color(theme::INK_RED));
                            };
                            let p = |ui: &mut egui::Ui, t: &str| {
                                ui.label(RichText::new(t).size(15.5).color(theme::INK));
                                ui.add_space(4.0);
                            };
                            h(ui, "Ashen Crowns – The Age of the Fallen");
                            p(ui, "Eigenständiges Dark-Fantasy-Strategiespiel. Eigene Engine in Rust (wgpu, winit, egui, kira).");
                            h(ui, "Grafik");
                            p(ui, "Alle Grafiken der Vertical Slice – auch Rahmen, Schaltflächen und Pergament der Oberfläche – werden zur Laufzeit prozedural als Pixel-Art erzeugt; handgezeichnete Sprites können über assets/sprites/ eingespielt werden.");
                            h(ui, "Schriften");
                            p(ui, "Cinzel und Cinzel Decorative (Natanael Gama), Alegreya (Huerta Tipográfica) – SIL Open Font License 1.1.");
                            h(ui, "Klang");
                            p(ui, "Stimmen: ElevenLabs · Musik: Suno (Produktionsleitfäden unter docs/audio/).");
                            ui.label(RichText::new("Welt, Figuren und Geschichte von Eldraeth sind Originalinhalte.").font(theme::italic_font(14.0)).color(theme::INK_FADED));
                        });
                        self.back(ui, engine, art, None);
                    }
                }
                if let Some(e) = &self.error {
                    ui.label(RichText::new(e).color(theme::EMBER));
                }
            });
        });
    }

    fn back(&mut self, ui: &mut egui::Ui, engine: &mut Engine, art: &UiArt, settings: Option<&mut UserSettings>) {
        ui.add_space(10.0);
        ui.vertical_centered(|ui| {
            if ui_common::button(ui, engine, art, "Zurück", 200.0) {
                if let Some(s) = settings {
                    s.save();
                }
                self.page = Page::Main;
            }
        });
    }

    fn main_page(&mut self, ui: &mut egui::Ui, engine: &mut Engine, art: &UiArt) {
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            let mut b = |ui: &mut egui::Ui, t: &str, enabled: bool, tip: &str| {
                let r = ui_art::menu_item(ui, art, t, 340.0, enabled, !enabled);
                ui_common::sfx(engine, ui, &r);
                let r = if tip.is_empty() {
                    r
                } else {
                    ui_art::tooltip_at(r, art, Some(egui::RectAlign::RIGHT_START), |ui| {
                        ui.label(RichText::new(t).font(theme::heading_font(15.0)).color(theme::GOLD));
                        ui.label(RichText::new(tip).size(14.5).color(theme::PARCHMENT));
                    })
                };
                r.clicked()
            };
            if b(ui, "Kampagne", true, "Zeitalter I – Mission 1: Die letzte Siedlung") {
                self.action = MenuAction::Start { mission: 0, settings: MatchSettings { mode: GameMode::Campaign, difficulty: self.new_difficulty, seed: 897, hero_permadeath: false } };
            }
            if b(ui, "Neues Spiel", true, "Modus, Schwierigkeit und Seed wählen") {
                self.page = Page::NewGame;
            }
            if b(ui, "Spiel laden", true, "Gespeicherte Partien") {
                self.page = Page::Load;
            }
            if b(ui, "Gefecht", true, "Schneller Kampf (Skirmish): Zerstört den Hohlschrein") {
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

    fn new_game_page(&mut self, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, settings: &mut UserSettings, art: &UiArt) {
        ui.set_width(600.0);
        ui_art::heading(ui, art, "Neues Spiel", 26.0);
        let label = |ui: &mut egui::Ui, t: &str| {
            ui.label(RichText::new(t).font(theme::caps_font(13.5)).color(Color32::from_rgb(214, 190, 146)));
        };
        egui::Grid::new("newgame").num_columns(2).spacing([20.0, 12.0]).min_col_width(150.0).show(ui, |ui| {
            label(ui, "Mission");
            egui::ComboBox::from_id_salt("mission").width(300.0).selected_text(RichText::new(&content.missions[self.new_mission.min(content.missions.len() - 1)].name).font(theme::body_font(16.0))).show_ui(ui, |ui| {
                for (i, m) in content.missions.iter().enumerate() {
                    ui.selectable_value(&mut self.new_mission, i, &m.name);
                }
            });
            ui.end_row();
            label(ui, "Modus");
            ui.horizontal(|ui| {
                for m in [GameMode::Campaign, GameMode::Skirmish, GameMode::Survival, GameMode::Sandbox] {
                    let r = ui_art::choice(ui, art, m.label(), self.new_mode == m);
                    ui_common::sfx(engine, ui, &r);
                    if r.clicked() {
                        self.new_mode = m;
                    }
                }
            });
            ui.end_row();
            label(ui, "Schwierigkeit");
            ui.horizontal(|ui| {
                for d in [Difficulty::Story, Difficulty::Normal, Difficulty::Hard] {
                    let r = ui_art::choice(ui, art, d.label(), self.new_difficulty == d);
                    ui_common::sfx(engine, ui, &r);
                    if r.clicked() {
                        self.new_difficulty = d;
                    }
                }
            });
            ui.end_row();
            label(ui, "Dauerhafter Heldentod");
            ui.checkbox(&mut settings.hero_permadeath, RichText::new("Fällt die Heldin, bleibt sie tot").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
            ui.end_row();
            label(ui, "Seed");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut self.new_seed));
                ui.label(RichText::new("Wetter und Variation").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
            });
            ui.end_row();
        });
        ui.add_space(10.0);
        let m = &content.missions[self.new_mission.min(content.missions.len() - 1)];
        art.panel(ui, PanelKind::Parchment, egui::Margin { left: 26, right: 26, top: 20, bottom: 20 }, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(&m.name).font(theme::title_bold_font(17.0)).color(theme::INK_RED));
            ui.label(RichText::new(&m.description).size(15.5).color(theme::INK));
        });
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 440.0).max(0.0) / 2.0);
            if ui_common::button(ui, engine, art, "Starten", 220.0) {
                self.action = MenuAction::Start {
                    mission: self.new_mission,
                    settings: MatchSettings { mode: self.new_mode, difficulty: self.new_difficulty, seed: self.new_seed, hero_permadeath: settings.hero_permadeath },
                };
            }
            if ui_common::button(ui, engine, art, "Zurück", 200.0) {
                self.page = Page::Main;
            }
        });
    }

    fn mods_page(&mut self, ui: &mut egui::Ui, engine: &mut Engine, content: &Content, settings: &mut UserSettings, art: &UiArt) {
        ui.set_width(560.0);
        ui_art::heading(ui, art, "Modifikationen", 26.0);
        if content.mods.is_empty() {
            ui.label(RichText::new("Keine Mods gefunden (Ordner mods/ oder Nutzerverzeichnis/mods).").font(theme::italic_font(15.0)).color(theme::PARCHMENT_DIM));
        }
        let mut changed = false;
        for m in &content.mods {
            let mut on = settings.enabled_mods.contains(&m.id);
            art.panel(ui, PanelKind::Thin, egui::Margin::symmetric(16, 12), |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut on, RichText::new(format!("{}  {}", m.name, m.version)).font(theme::heading_font(15.0)).color(theme::GOLD)).changed() {
                        changed = true;
                        if on {
                            settings.enabled_mods.push(m.id.clone());
                        } else {
                            settings.enabled_mods.retain(|x| x != &m.id);
                        }
                    }
                    if content.active_mods.contains(&m.id) {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new("aktiv").font(theme::caps_font(12.5)).color(Color32::from_rgb(130, 210, 120)));
                        });
                    }
                });
                ui.label(RichText::new(format!("{} – {}", m.author, m.description)).font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
            });
            ui.add_space(4.0);
        }
        for e in &content.errors {
            ui.label(RichText::new(e).color(theme::EMBER).small());
        }
        if changed {
            settings.save();
            self.action = MenuAction::ReloadContent;
        }
        self.back(ui, engine, art, None);
    }
}
