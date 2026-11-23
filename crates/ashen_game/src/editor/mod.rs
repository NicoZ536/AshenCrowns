//! Karteneditor als eigene Szene: Gelände, Objekte, Startpositionen, Bereiche, Ziele und Trigger
//! bearbeiten, prüfen, als Szenario speichern und direkt im Testspiel erproben.

pub mod doc;
mod panels;
pub mod triggers;
pub mod validate;

use crate::art::ArtLibrary;
use crate::content::Content;
use crate::settings::{scenarios_dir, UserSettings};
use crate::view::{Overlay, WorldView};
use ashen_core::iso::{tile_to_world, world_to_tile};
use ashen_engine::input::{KeyCode, MouseButton};
use ashen_engine::render::{Camera2D, ViewParams};
use ashen_engine::Engine;
use ashen_sim::entity::NodeKind;
use ashen_sim::genmap::GenPreset;
use ashen_sim::map::{DecorKind, Terrain};
use ashen_sim::mission::{GameMode, MatchSettings, MissionDef};
use ashen_sim::scenario::{AreaDef, StartPos};
use ashen_sim::SimWorld;
use doc::{BrushShape, EditDoc, History};
use glam::{IVec2, Vec2};
use std::path::PathBuf;
use triggers::TriggerUi;
use validate::Issue;

/// Womit der Editor startet.
#[derive(Clone, Debug)]
pub enum EditorInit {
    /// Neue Karte aus dem Generator.
    New { width: i32, height: i32, seed: u64, preset: GenPreset, blank: bool },
    /// Gespeichertes Szenario bearbeiten.
    Load(PathBuf),
}

/// Was der Editor vom übergeordneten Spiel möchte.
#[derive(Clone, Debug)]
pub enum EditorAction {
    None,
    Exit,
    /// Testspiel der aktuellen Karte.
    Test(Box<MissionDef>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Select,
    Terrain,
    Fill,
    Trees,
    Decor,
    Nodes,
    Building,
    Unit,
    Start,
    Area,
    Erase,
}

impl Tool {
    pub const ALL: [Tool; 11] = [Tool::Select, Tool::Terrain, Tool::Fill, Tool::Trees, Tool::Decor, Tool::Nodes, Tool::Building, Tool::Unit, Tool::Start, Tool::Area, Tool::Erase];

    pub fn label(self) -> &'static str {
        match self {
            Tool::Select => "Auswahl",
            Tool::Terrain => "Gelände",
            Tool::Fill => "Füllen",
            Tool::Trees => "Wald",
            Tool::Decor => "Deko",
            Tool::Nodes => "Quellen",
            Tool::Building => "Gebäude",
            Tool::Unit => "Einheiten",
            Tool::Start => "Start",
            Tool::Area => "Bereiche",
            Tool::Erase => "Radierer",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Tool::Select => "Klicken wählt Objekte, Startpunkte und Bereiche. Ziehen verschiebt. Entf löscht.",
            Tool::Terrain => "Malt Gelände mit Pinsel. Wasser entfernt Bäume und Dekoration darunter.",
            Tool::Fill => "Füllt zusammenhängendes gleiches Gelände.",
            Tool::Trees => "Linke Maustaste pflanzt Bäume (Dichte), Umschalt entfernt sie.",
            Tool::Decor => "Setzt Dekoration. Blockierende Objekte (Felsbrocken, Ruinen …) sperren die Kachel.",
            Tool::Nodes => "Setzt Goldminen, Steinbrüche und einzelne Bäume.",
            Tool::Building => "Setzt fertige Gebäude beider Seiten. Entfernen mit dem Radierer oder über die Auswahl.",
            Tool::Unit => "Setzt Einheitengruppen. Wächter-Haltung stellt Ihr in den Eigenschaften ein.",
            Tool::Start => "Setzt die Startposition der gewählten Seite.",
            Tool::Area => "Zieht einen benannten Bereich auf (für Auslöser: Einheiten im Bereich, Erscheinen).",
            Tool::Erase => "Entfernt Objekte unter dem Zeiger (Einheiten, Gebäude, Quellen, Deko).",
        }
    }
}

/// Auswahl im Editor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    Placement(usize),
    Decor(usize),
    Node(usize),
    Start(usize),
    Area(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dialog {
    None,
    NewMap,
    Load,
    Data,
    ConfirmExit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTab {
    General,
    Objectives,
    Triggers,
    Rules,
    Areas,
    Validation,
}

/// Parameter des „Neue Karte“-Dialogs.
#[derive(Clone, Debug)]
pub struct NewMapForm {
    pub width: i32,
    pub height: i32,
    pub seed: u64,
    pub preset: GenPreset,
    pub blank: bool,
}

impl Default for NewMapForm {
    fn default() -> Self {
        NewMapForm { width: 72, height: 72, seed: 897, preset: GenPreset::Aschmark, blank: false }
    }
}

pub struct EditorScene {
    pub doc: EditDoc,
    last_stable: EditDoc,
    history: History,
    last_commit: (String, f32),
    world: Option<SimWorld>,
    dirty_world: bool,
    view: WorldView,
    pub camera: Camera2D,
    last_view: Option<ViewParams>,
    // Werkzeuge
    pub tool: Tool,
    pub terrain: Terrain,
    pub brush_size: i32,
    pub brush_shape: BrushShape,
    pub tree_density: f32,
    pub decor_kind: DecorKind,
    pub node_kind: NodeKind,
    pub building_id: String,
    pub unit_id: String,
    pub owner: u8,
    pub unit_count: u32,
    pub show_grid: bool,
    // Interaktion
    pub selection: Option<Sel>,
    stroke_id: u32,
    stroking: bool,
    last_decor_tile: Option<IVec2>,
    area_drag: Option<IVec2>,
    move_origin: Option<(Vec2, Vec2)>,
    mouse_tile: Option<Vec2>,
    /// Fester Zeigerpunkt für automatisierte Screenshots.
    pub forced_cursor: Option<Vec2>,
    // Oberfläche
    pub dialog: Dialog,
    pub data_tab: DataTab,
    pub trigger_ui: TriggerUi,
    pub new_form: NewMapForm,
    pub file_path: Option<PathBuf>,
    pub modified: bool,
    pub status: Option<(String, f32, bool)>,
    pub issues: Vec<Issue>,
    pub issues_dirty: bool,
    pub saved_list: Vec<(PathBuf, String, String)>,
    pub action: EditorAction,
    time: f32,
}

impl EditorScene {
    pub fn new(content: &Content, init: EditorInit, max_particles: usize) -> Result<EditorScene, String> {
        let (doc, path) = match init {
            EditorInit::New { width, height, seed, preset, blank } => {
                let doc = if blank { EditDoc::blank(width, height, Terrain::Grass) } else { EditDoc::generated(width, height, seed, preset) };
                (doc, None)
            }
            EditorInit::Load(p) => {
                let def = MissionDef::load(&p)?;
                let mut doc = EditDoc::from_mission(&content.data, &def)?;
                let own = p.starts_with(scenarios_dir());
                if !own {
                    doc.def.id = format!("{}_kopie", doc.def.id);
                    doc.def.name = format!("{} (Kopie)", doc.def.name);
                }
                (doc, own.then_some(p))
            }
        };
        let (cx, cy) = (doc.map.width as f32 * 0.5, doc.map.height as f32 * 0.5);
        let start = doc.def.starts.first().map(|s| Vec2::new(s.x, s.y)).unwrap_or(Vec2::new(cx, cy));
        let building_id = content.data.buildings.iter().find(|b| b.faction == ashen_sim::data::Faction::EmberCrown && b.buildable).map(|b| b.id.clone()).unwrap_or_else(|| "keep".into());
        let unit_id = content.data.units.iter().find(|u| u.faction == ashen_sim::data::Faction::EmberCrown).map(|u| u.id.clone()).unwrap_or_else(|| "swordsman".into());
        let mut s = EditorScene {
            last_stable: doc.clone(),
            doc,
            history: History::new(),
            last_commit: (String::new(), -10.0),
            world: None,
            dirty_world: true,
            view: WorldView::new(max_particles),
            camera: Camera2D { center: tile_to_world(start), target_lines: 420.0 },
            last_view: None,
            tool: Tool::Terrain,
            terrain: Terrain::Grass,
            brush_size: 3,
            brush_shape: BrushShape::Circle,
            tree_density: 0.6,
            decor_kind: DecorKind::Rock,
            node_kind: NodeKind::GoldMine,
            building_id,
            unit_id,
            owner: 0,
            unit_count: 3,
            show_grid: false,
            selection: None,
            stroke_id: 0,
            stroking: false,
            last_decor_tile: None,
            area_drag: None,
            move_origin: None,
            mouse_tile: None,
            forced_cursor: None,
            dialog: Dialog::None,
            data_tab: DataTab::General,
            trigger_ui: TriggerUi::default(),
            new_form: NewMapForm::default(),
            file_path: path,
            modified: false,
            status: None,
            issues: Vec::new(),
            issues_dirty: true,
            saved_list: Vec::new(),
            action: EditorAction::None,
            time: 0.0,
        };
        s.rebuild_world(&content.data);
        Ok(s)
    }

    // ------------------------------------------------------------------ Dokument

    /// Übernimmt Änderungen des Frames in die Undo-Liste (mit Zusammenfassung gleicher Eingaben).
    pub fn commit(&mut self, key: &str) {
        let merge_text = key.starts_with("text") && self.last_commit.0 == key && self.time - self.last_commit.1 < 1.5;
        let merge_stroke = key.starts_with("stroke") && self.last_commit.0 == key;
        if !(merge_text || merge_stroke) {
            self.history.record(&self.last_stable);
        }
        self.last_commit = (key.to_string(), self.time);
        self.last_stable = self.doc.clone();
        self.modified = true;
        self.dirty_world = true;
        self.issues_dirty = true;
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.history.undo(&self.doc) {
            self.doc = prev;
            self.after_restore("Rückgängig");
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.history.redo(&self.doc) {
            self.doc = next;
            self.after_restore("Wiederholt");
        }
    }

    fn after_restore(&mut self, text: &str) {
        self.last_stable = self.doc.clone();
        self.last_commit = (String::new(), -10.0);
        self.modified = true;
        self.dirty_world = true;
        self.issues_dirty = true;
        self.selection = None;
        self.notify(text, false);
    }

    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    pub fn notify(&mut self, text: impl Into<String>, error: bool) {
        self.status = Some((text.into(), 5.0, error));
    }

    fn rebuild_world(&mut self, data: &ashen_sim::GameData) {
        match SimWorld::new(data, self.doc.to_mission(), MatchSettings { mode: GameMode::Sandbox, ..Default::default() }) {
            Ok(mut w) => {
                w.env.time_of_day = 0.46;
                w.env.weather = ashen_sim::env::Weather::Clear;
                w.env.forced_weather = Some(ashen_sim::env::Weather::Clear);
                w.fog.disabled = true;
                self.world = Some(w);
                self.view.invalidate_map();
            }
            Err(e) => self.notify(format!("Vorschau nicht möglich: {e}"), true),
        }
        self.dirty_world = false;
    }

    /// Aktualisiert die Prüfliste bei Bedarf.
    pub fn refresh_issues(&mut self, data: &ashen_sim::GameData) {
        if self.issues_dirty {
            self.issues = validate::validate(&self.doc, data);
            self.issues_dirty = false;
        }
    }

    pub fn file_name(&self) -> PathBuf {
        scenarios_dir().join(format!("{}.ron", EditDoc::sanitize_id(&self.doc.def.id)))
    }

    /// Speichert unter `<Benutzerordner>/scenarios/<id>.ron`.
    pub fn save(&mut self, data: &ashen_sim::GameData) -> bool {
        self.doc.def.id = EditDoc::sanitize_id(&self.doc.def.id);
        self.refresh_issues(data);
        let path = self.file_name();
        match ashen_sim::scenario::save(&self.doc.to_mission(), &path) {
            Ok(()) => {
                let errors = self.issues.iter().filter(|i| i.level == validate::Level::Error).count();
                self.file_path = Some(path.clone());
                self.modified = false;
                let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                if errors > 0 {
                    self.notify(format!("Gespeichert als {name} – aber {errors} Fehler in der Prüfung, die Karte ist so nicht fair spielbar."), true);
                } else {
                    self.notify(format!("Gespeichert: {name}"), false);
                }
                true
            }
            Err(e) => {
                self.notify(format!("Speichern fehlgeschlagen: {e}"), true);
                false
            }
        }
    }

    pub fn load_path(&mut self, content: &Content, path: PathBuf) {
        match MissionDef::load(&path).and_then(|d| EditDoc::from_mission(&content.data, &d)) {
            Ok(mut doc) => {
                let own = path.starts_with(scenarios_dir());
                if !own {
                    doc.def.id = format!("{}_kopie", doc.def.id);
                    doc.def.name = format!("{} (Kopie)", doc.def.name);
                }
                self.doc = doc;
                self.last_stable = self.doc.clone();
                self.history.clear();
                self.file_path = own.then_some(path);
                self.modified = false;
                self.selection = None;
                self.dirty_world = true;
                self.issues_dirty = true;
                if let Some(s) = self.doc.def.starts.first() {
                    self.camera.center = tile_to_world(Vec2::new(s.x, s.y));
                }
                self.notify("Szenario geladen", false);
            }
            Err(e) => self.notify(format!("Laden fehlgeschlagen: {e}"), true),
        }
    }

    /// Zustand nach dem Ersetzen des Dokuments durch eine Vorlage.
    pub fn after_template_load(&mut self) {
        self.last_stable = self.doc.clone();
        self.history.clear();
        self.file_path = None;
        self.modified = true;
        self.selection = None;
        self.dirty_world = true;
        self.issues_dirty = true;
        if let Some(s) = self.doc.def.starts.first() {
            self.camera.center = tile_to_world(Vec2::new(s.x, s.y));
        }
        self.notify("Vorlage geladen – Speichern legt eine eigene Fassung an.", false);
    }

    pub fn new_map(&mut self, data: &ashen_sim::GameData) {
        let f = self.new_form.clone();
        let doc = if f.blank { EditDoc::blank(f.width, f.height, Terrain::Grass) } else { EditDoc::generated(f.width, f.height, f.seed, f.preset) };
        self.doc = doc;
        self.last_stable = self.doc.clone();
        self.history.clear();
        self.file_path = None;
        self.modified = true;
        self.selection = None;
        self.dirty_world = true;
        self.issues_dirty = true;
        if let Some(s) = self.doc.def.starts.first() {
            self.camera.center = tile_to_world(Vec2::new(s.x, s.y));
        }
        self.rebuild_world(data);
        self.notify("Neue Karte erzeugt", false);
    }

    /// Alle Szenarien des Benutzers und Vorlagen zum Laden.
    pub fn refresh_saved_list(&mut self, content: &Content) {
        let mut list = Vec::new();
        let mut errors = Vec::new();
        let mut dirs = vec![(scenarios_dir(), ashen_sim::scenario::ScenarioSource::User)];
        dirs.extend(content.scenario_dirs());
        for e in ashen_sim::scenario::discover(&dirs, &mut errors) {
            list.push((e.path, format!("{} · {}", e.def.name, e.source.label()), e.def.id));
        }
        self.saved_list = list;
    }

    /// Zustand für automatisierte Läufe (Screenshots): Werkzeug, Dialog, Reiter, Beispielstriche.
    pub fn apply_auto(&mut self, data: &ashen_sim::GameData, tool: &Option<String>, dialog: &Option<String>, tab: &Option<String>, demo: bool) {
        if demo {
            // Ein Teich und ein Wäldchen nahe der Spielersiedlung, Auswahl des Bergfrieds.
            let c = self.doc.def.starts.first().map(|s| IVec2::new(s.x as i32, s.y as i32)).unwrap_or(IVec2::new(20, 50));
            let pond = c + IVec2::new(10, -6);
            self.doc.paint_terrain(&BrushShape::Circle.tiles(pond, 6), Terrain::Water);
            self.doc.paint_terrain(&BrushShape::Circle.tiles(pond, 8).into_iter().filter(|t| self.doc.map.terrain_at(*t).is_some_and(|x| x.walkable() && x != Terrain::Road)).collect::<Vec<_>>(), Terrain::Sand);
            self.doc.paint_terrain(&BrushShape::Circle.tiles(pond, 4), Terrain::DeepWater);
            self.doc.paint_terrain(&BrushShape::Diamond.tiles(c + IVec2::new(-4, 10), 7), Terrain::DarkGrass);
            self.doc.paint_trees(&BrushShape::Circle.tiles(c + IVec2::new(-4, 10), 7), true, 0.7, 11);
            let _ = self.doc.place_decor(DecorKind::Ruin, glam::Vec2::new(c.x as f32 + 6.5, c.y as f32 + 8.5), 0);
            let _ = self.doc.place_decor(DecorKind::Brazier, glam::Vec2::new(c.x as f32 + 3.5, c.y as f32 + 10.5), 0);
            let _ = self.doc.place_object(data, "swordsman", 0, glam::Vec2::new(c.x as f32 + 5.5, c.y as f32 + 5.5), 4);
            self.doc.def.areas.push(AreaDef { id: "lager".into(), name: "Lager".into(), x: c.x - 8, y: c.y - 8, w: 8, h: 6 });
            self.selection = self.doc.def.placements.iter().position(|p| p.id == "keep").map(Sel::Placement);
            self.commit("demo");
            self.rebuild_world(data);
        }
        if let Some(t) = tool {
            self.tool = match t.as_str() {
                "select" => Tool::Select,
                "fill" => Tool::Fill,
                "trees" => Tool::Trees,
                "decor" => Tool::Decor,
                "nodes" => Tool::Nodes,
                "building" => Tool::Building,
                "unit" => Tool::Unit,
                "start" => Tool::Start,
                "area" => Tool::Area,
                "erase" => Tool::Erase,
                _ => Tool::Terrain,
            };
        }
        if let Some(d) = dialog {
            self.dialog = match d.as_str() {
                "new" => Dialog::NewMap,
                "load" => Dialog::Load,
                _ => Dialog::Data,
            };
        }
        if let Some(t) = tab {
            self.data_tab = match t.as_str() {
                "objectives" => DataTab::Objectives,
                "triggers" => DataTab::Triggers,
                "rules" => DataTab::Rules,
                "areas" => DataTab::Areas,
                "validation" => DataTab::Validation,
                _ => DataTab::General,
            };
            self.issues_dirty = true;
            self.refresh_issues(data);
        }
    }

    // ------------------------------------------------------------------ Eingabe

    fn brush_tiles(&self, center: IVec2) -> Vec<IVec2> {
        self.brush_shape.tiles(center, self.brush_size)
    }

    fn clamp_camera(&mut self) {
        let (w, h) = (self.doc.map.width as f32, self.doc.map.height as f32);
        let tile = world_to_tile(self.camera.center).clamp(Vec2::ZERO, Vec2::new(w, h));
        self.camera.center = tile_to_world(tile);
    }

    pub fn center_on(&mut self, p: Vec2) {
        self.camera.center = tile_to_world(p);
    }

    pub fn update(&mut self, engine: &mut Engine, content: &Content, _art: &ArtLibrary, _settings: &UserSettings, dt: f32) {
        self.time += dt;
        if let Some((_, t, _)) = &mut self.status {
            *t -= dt;
        }
        if self.status.as_ref().is_some_and(|(_, t, _)| *t <= 0.0) {
            self.status = None;
        }
        let view = self.camera.view(engine.size, engine.renderer.settings.min_lines);
        self.last_view = Some(view);
        self.view.update(dt);
        self.controls(engine, content, &view, dt);
        if self.dirty_world {
            self.rebuild_world(&content.data);
        }
        if let Some(w) = &mut self.world {
            w.time += dt;
            for (_, u) in w.units.iter_mut() {
                u.activity_time += dt;
            }
        }
        if self.issues_dirty && self.dialog == Dialog::Data && self.data_tab == DataTab::Validation {
            self.refresh_issues(&content.data);
        }
    }

    fn controls(&mut self, engine: &mut Engine, content: &Content, view: &ViewParams, dt: f32) {
        let input = &engine.input;
        let scale = view.pixel_scale as f32;
        // Kamera: Pfeiltasten, WASD ist für Befehle frei; mittlere Maustaste zieht.
        let mut pan = Vec2::ZERO;
        for (k, d) in [(KeyCode::ArrowUp, Vec2::NEG_Y), (KeyCode::ArrowDown, Vec2::Y), (KeyCode::ArrowLeft, Vec2::NEG_X), (KeyCode::ArrowRight, Vec2::X)] {
            if input.key_down(k) {
                pan += d;
            }
        }
        self.camera.center += pan.normalize_or_zero() * 420.0 * (self.camera.target_lines / 330.0) * dt;
        if input.mouse_down(MouseButton::Middle) {
            self.camera.center -= input.mouse_delta / scale;
        }
        if input.wheel != 0.0 && !input.ui_wants_pointer {
            self.camera.target_lines = (self.camera.target_lines * (1.0 - input.wheel * 0.1)).clamp(200.0, 720.0);
        }
        self.clamp_camera();

        let cmd = input.command_mod();
        if input.key_pressed(KeyCode::KeyZ) && cmd {
            if input.shift { self.redo() } else { self.undo() }
        }
        if input.key_pressed(KeyCode::KeyY) && cmd {
            self.redo();
        }
        if input.key_pressed(KeyCode::KeyS) && cmd {
            self.save(&content.data);
        }
        if input.key_pressed(KeyCode::KeyG) && !cmd {
            self.show_grid = !self.show_grid;
        }
        if input.key_pressed(KeyCode::Escape) {
            if self.dialog != Dialog::None {
                self.dialog = Dialog::None;
            } else if self.selection.is_some() {
                self.selection = None;
            } else if self.tool != Tool::Select {
                self.tool = Tool::Select;
            }
        }
        if input.key_pressed(KeyCode::Delete) || input.key_pressed(KeyCode::Backspace) {
            self.delete_selection();
        }

        let mouse = input.mouse;
        let world_pos = world_to_tile(view.screen_to_world(mouse));
        let inside = world_pos.x >= 0.0 && world_pos.y >= 0.0 && world_pos.x < self.doc.map.width as f32 && world_pos.y < self.doc.map.height as f32;
        self.mouse_tile = (inside && input.cursor_inside).then_some(world_pos).or(self.forced_cursor);
        if input.ui_wants_pointer || self.dialog != Dialog::None {
            if !input.mouse_down(MouseButton::Left) {
                self.stroking = false;
            }
            return;
        }
        let Some(p) = self.mouse_tile else {
            if !input.mouse_down(MouseButton::Left) {
                self.stroking = false;
            }
            return;
        };
        let tile = IVec2::new(p.x.floor() as i32, p.y.floor() as i32);
        let pressed = input.mouse_pressed(MouseButton::Left);
        let down = input.mouse_down(MouseButton::Left);
        let released = input.mouse_released(MouseButton::Left);
        let shift = input.shift;
        if input.mouse_pressed(MouseButton::Right) {
            self.selection = None;
            self.tool = Tool::Select;
        }
        if pressed {
            self.stroke_id += 1;
            self.stroking = true;
        }
        let key = format!("stroke{}", self.stroke_id);
        let data = &content.data;
        let mut changed = false;
        match self.tool {
            Tool::Terrain if self.stroking && down => {
                let tiles = self.brush_tiles(tile);
                let before = self.doc.map.terrain.clone();
                self.doc.paint_terrain(&tiles, self.terrain);
                changed = before != self.doc.map.terrain || pressed;
            }
            Tool::Fill if pressed => {
                changed = self.doc.flood_fill(tile, self.terrain, 40_000) > 0;
            }
            Tool::Trees if self.stroking && down => {
                let tiles = self.brush_tiles(tile);
                let seed = self.stroke_id.wrapping_mul(2654435761);
                changed = self.doc.paint_trees(&tiles, !shift, self.tree_density, seed) > 0;
            }
            Tool::Decor if self.stroking && down => {
                let center = if self.decor_kind.blocks() { TileCenter::center(tile) } else { p };
                let moved = self.last_decor_tile != Some(tile);
                if pressed || moved {
                    self.last_decor_tile = Some(tile);
                    if let Err(e) = self.doc.place_decor(self.decor_kind, center, (tile.x + tile.y).rem_euclid(8) as u8) {
                        if pressed {
                            self.notify(e, true);
                        }
                    } else {
                        changed = true;
                    }
                }
            }
            Tool::Nodes if pressed => match self.doc.place_node(self.node_kind, tile, (tile.x + tile.y).rem_euclid(6) as u8) {
                Ok(()) => changed = true,
                Err(e) => self.notify(e, true),
            },
            Tool::Building if pressed => {
                let id = self.building_id.clone();
                match self.doc.place_object(data, &id, self.owner, p, 1) {
                    Ok(i) => {
                        self.selection = Some(Sel::Placement(i));
                        changed = true;
                    }
                    Err(e) => self.notify(e, true),
                }
            }
            Tool::Unit if pressed => {
                let id = self.unit_id.clone();
                let count = if EditDoc::is_hero(data, &id) { 1 } else { self.unit_count };
                match self.doc.place_object(data, &id, self.owner, p, count) {
                    Ok(i) => {
                        self.selection = Some(Sel::Placement(i));
                        changed = true;
                    }
                    Err(e) => self.notify(e, true),
                }
            }
            Tool::Start if pressed => {
                if self.doc.can_place_unit(p).is_err() {
                    self.notify("Startpositionen brauchen begehbares Gelände.", true);
                } else {
                    let owner = self.owner;
                    if let Some(s) = self.doc.def.starts.iter_mut().find(|s| s.owner == owner) {
                        s.x = p.x;
                        s.y = p.y;
                    } else {
                        self.doc.def.starts.push(StartPos { owner, x: p.x, y: p.y });
                    }
                    changed = true;
                }
            }
            Tool::Area => {
                if pressed {
                    self.area_drag = Some(tile);
                }
                if released && let Some(a) = self.area_drag.take() {
                    let (x0, y0) = (a.x.min(tile.x), a.y.min(tile.y));
                    let (x1, y1) = (a.x.max(tile.x), a.y.max(tile.y));
                    let ids: Vec<&str> = self.doc.def.areas.iter().map(|a| a.id.as_str()).collect();
                    let id = EditDoc::fresh_id(&ids, "bereich");
                    self.doc.def.areas.push(AreaDef { name: id.clone(), id, x: x0, y: y0, w: x1 - x0 + 1, h: y1 - y0 + 1 });
                    self.selection = Some(Sel::Area(self.doc.def.areas.len() - 1));
                    changed = true;
                }
            }
            Tool::Erase if self.stroking && down => {
                for c in self.brush_tiles(tile) {
                    if self.doc.erase_at(data, TileCenter::center(c)).is_some() {
                        changed = true;
                    }
                }
            }
            Tool::Select => {
                if pressed {
                    self.selection = self.pick(data, p);
                    self.move_origin = self.selection.map(|s| (p, self.sel_pos(s)));
                }
                if down && self.stroking && let (Some(sel), Some((grab, origin))) = (self.selection, self.move_origin) {
                    let target = origin + (p - grab);
                    if self.move_selected(data, sel, target) {
                        changed = true;
                    }
                }
            }
            _ => {}
        }
        if !down {
            self.stroking = false;
            self.last_decor_tile = None;
            if self.tool != Tool::Select {
                self.move_origin = None;
            }
        }
        if released {
            self.move_origin = None;
        }
        if changed {
            self.commit(&key);
        }
    }

    fn pick(&self, data: &ashen_sim::GameData, p: Vec2) -> Option<Sel> {
        if let Some(i) = self.doc.placement_at(data, p) {
            return Some(Sel::Placement(i));
        }
        if let Some(i) = self.doc.start_at(p) {
            return Some(Sel::Start(i));
        }
        if let Some(i) = self.doc.node_at(p) {
            return Some(Sel::Node(i));
        }
        if let Some(i) = self.doc.decor_at(p) {
            return Some(Sel::Decor(i));
        }
        self.doc.area_at(p).map(Sel::Area)
    }

    /// Bezugspunkt einer Auswahl (für Verschieben).
    fn sel_pos(&self, s: Sel) -> Vec2 {
        match s {
            Sel::Placement(i) => Vec2::new(self.doc.def.placements[i].x, self.doc.def.placements[i].y),
            Sel::Decor(i) => Vec2::new(self.doc.map.decor[i].x, self.doc.map.decor[i].y),
            Sel::Node(i) => Vec2::new(self.doc.map.nodes[i].x as f32, self.doc.map.nodes[i].y as f32),
            Sel::Start(i) => Vec2::new(self.doc.def.starts[i].x, self.doc.def.starts[i].y),
            Sel::Area(i) => Vec2::new(self.doc.def.areas[i].x as f32, self.doc.def.areas[i].y as f32),
        }
    }

    /// Verschiebt die Auswahl; liefert `true`, wenn sich etwas geändert hat.
    fn move_selected(&mut self, data: &ashen_sim::GameData, sel: Sel, target: Vec2) -> bool {
        match sel {
            Sel::Placement(i) => {
                let id = self.doc.def.placements[i].id.clone();
                let (old_x, old_y) = (self.doc.def.placements[i].x, self.doc.def.placements[i].y);
                if data.building_id(&id).is_some() {
                    let tile = IVec2::new(target.x.round() as i32, target.y.round() as i32);
                    if tile.x as f32 == old_x && tile.y as f32 == old_y {
                        return false;
                    }
                    if self.doc.can_place_building(data, &id, tile, Some(i)).is_ok() {
                        self.doc.def.placements[i].x = tile.x as f32;
                        self.doc.def.placements[i].y = tile.y as f32;
                        return true;
                    }
                    false
                } else {
                    let (nx, ny) = ((target.x * 2.0).round() / 2.0, (target.y * 2.0).round() / 2.0);
                    if nx == old_x && ny == old_y || self.doc.can_place_unit(Vec2::new(nx, ny)).is_err() {
                        return false;
                    }
                    self.doc.def.placements[i].x = nx;
                    self.doc.def.placements[i].y = ny;
                    true
                }
            }
            Sel::Decor(i) => {
                let d = self.doc.map.decor[i].clone();
                let np = if d.kind.blocks() { TileCenter::center(IVec2::new(target.x.floor() as i32, target.y.floor() as i32)) } else { target };
                if (np - Vec2::new(d.x, d.y)).length() < 0.05 {
                    return false;
                }
                self.doc.map.decor.remove(i);
                match self.doc.place_decor(d.kind, np, d.variant) {
                    Ok(()) => {
                        self.selection = Some(Sel::Decor(self.doc.map.decor.len() - 1));
                        true
                    }
                    Err(_) => {
                        self.doc.map.decor.insert(i, d);
                        false
                    }
                }
            }
            Sel::Node(i) => {
                let n = self.doc.map.nodes[i].clone();
                let t = IVec2::new(target.x.round() as i32, target.y.round() as i32);
                if t.x == n.x && t.y == n.y {
                    return false;
                }
                self.doc.map.nodes.remove(i);
                match self.doc.place_node(n.kind, t, n.variant) {
                    Ok(()) => {
                        let last = self.doc.map.nodes.len() - 1;
                        self.doc.map.nodes[last].amount = n.amount;
                        self.selection = Some(Sel::Node(last));
                        true
                    }
                    Err(_) => {
                        self.doc.map.nodes.insert(i, n);
                        false
                    }
                }
            }
            Sel::Start(i) => {
                if self.doc.can_place_unit(target).is_err() {
                    return false;
                }
                self.doc.def.starts[i].x = target.x;
                self.doc.def.starts[i].y = target.y;
                true
            }
            Sel::Area(i) => {
                let (nx, ny) = (target.x.round() as i32, target.y.round() as i32);
                let a = &mut self.doc.def.areas[i];
                if a.x == nx && a.y == ny {
                    return false;
                }
                a.x = nx.clamp(0, self.doc.map.width - a.w);
                a.y = ny.clamp(0, self.doc.map.height - a.h);
                true
            }
        }
    }

    pub fn delete_selection(&mut self) {
        let Some(sel) = self.selection.take() else { return };
        match sel {
            Sel::Placement(i) if i < self.doc.def.placements.len() => {
                self.doc.def.placements.remove(i);
            }
            Sel::Decor(i) if i < self.doc.map.decor.len() => {
                self.doc.map.decor.remove(i);
            }
            Sel::Node(i) if i < self.doc.map.nodes.len() => {
                self.doc.map.nodes.remove(i);
            }
            Sel::Start(i) if i < self.doc.def.starts.len() => {
                self.doc.def.starts.remove(i);
            }
            Sel::Area(i) if i < self.doc.def.areas.len() => {
                self.doc.def.areas.remove(i);
            }
            _ => return,
        }
        self.commit("delete");
    }

    // ------------------------------------------------------------------ Darstellung

    pub fn render(&mut self, engine: &mut Engine, content: &Content, art: &ArtLibrary) {
        engine.frame.camera = self.camera;
        engine.fow_enabled = false;
        let view = self.camera.view(engine.size, engine.renderer.settings.min_lines);
        let mut overlay = Overlay::default();
        if self.tool == Tool::Building
            && let Some(p) = self.mouse_tile
            && let Some(b) = content.data.building_id(&self.building_id)
        {
            let def = content.data.building(b);
            let tile = IVec2::new(p.x.floor() as i32, p.y.floor() as i32);
            let ok = self.doc.can_place_building(&content.data, &self.building_id, tile, None).is_ok();
            overlay.ghost = Some((def.id.clone(), tile, def.size, ok));
        }
        if let Some(w) = &self.world {
            self.view.render(w, &content.data, art, &mut engine.frame, &view, 1.0, &overlay, false);
            engine.frame.atmosphere.vignette = 0.35;
        }
    }

    pub fn last_view(&self) -> Option<ViewParams> {
        self.last_view
    }

    pub fn mouse_tile(&self) -> Option<Vec2> {
        self.mouse_tile
    }

    pub fn area_drag(&self) -> Option<IVec2> {
        self.area_drag
    }

    pub fn test_mission(&self) -> MissionDef {
        self.doc.to_mission()
    }
}

/// Mittelpunkt einer Kachel als Hilfsfunktion.
struct TileCenter;

impl TileCenter {
    fn center(t: IVec2) -> Vec2 {
        Vec2::new(t.x as f32 + 0.5, t.y as f32 + 0.5)
    }
}
