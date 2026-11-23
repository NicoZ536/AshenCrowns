//! Missionsszene: Simulation, Kamera, Auswahl, Befehle, Ereignisverarbeitung, Speichern.

use crate::art::ArtLibrary;
use crate::audio_bridge;
use crate::settings::{saves_dir, UserSettings};
use crate::view::{Overlay, WorldView};
use ashen_core::iso::{tile_to_world, world_to_tile};
use ashen_engine::input::{KeyCode, MouseButton};
use ashen_engine::render::{Camera2D, ViewParams};
use ashen_engine::time::FixedStep;
use ashen_engine::Engine;
use ashen_sim::command::{Command, PlayerCommand};
use ashen_sim::data::{BuildCategory, BuildingTypeId, Fort, GameData, Item, UnitClass};
use ashen_sim::entity::{BuildingId, EntityRef, NodeId, UnitId, ENEMY, PLAYER};
use ashen_sim::map::TileMap;
use ashen_sim::mission::{MatchSettings, MissionDef};
use ashen_sim::{SimEvent, SimWorld};
use glam::{IVec2, Vec2};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommandMode {
    Normal,
    Place(BuildingTypeId),
    AttackMove,
    Patrol,
    Rally,
    BuildMenu,
    /// Zielzauber: nächster Linksklick auf den Boden wirkt die Fähigkeit mit diesem Index.
    Cast(u8),
}

pub struct Message {
    pub speaker: String,
    pub text: String,
    pub age: f32,
}

/// Was die Szene vom übergeordneten Spiel möchte.
#[derive(Clone, Debug, PartialEq)]
pub enum SceneAction {
    None,
    MainMenu,
    Load(std::path::PathBuf),
    Restart,
    Quit,
}

#[derive(Serialize, Deserialize, Default)]
struct ClientState {
    camera: (f32, f32),
    lines: f32,
    speed: f32,
}

pub struct MissionScene {
    pub world: SimWorld,
    pub view: WorldView,
    pub step: FixedStep,
    pub camera: Camera2D,
    pub selection: Vec<UnitId>,
    pub selected_building: Option<BuildingId>,
    pub groups: [Vec<UnitId>; 10],
    pub mode: CommandMode,
    pub drag_start: Option<Vec2>,
    pub speed: f32,
    pub messages: VecDeque<Message>,
    pub toast: Option<(String, f32)>,
    pub menu_open: bool,
    pub menu_page: crate::hud::MenuPage,
    pub show_objectives: bool,
    pub autosave_timer: f32,
    /// Eiserner Mann: Speichern und Laden sind gesperrt.
    pub no_save: bool,
    pub last_click: (f32, Option<UnitId>),
    pub hovered: Option<UnitId>,
    pub minimap: Option<egui::TextureHandle>,
    pub minimap_timer: f32,
    pub save_name: String,
    pub status: Option<String>,
    pub action: SceneAction,
    pub fog_enabled: bool,
    pub time_alive: f32,
    /// Lagerfenster (Waren) und Stadtübersicht (Steuern, Bürger, Ausbau) sind aufgeklappt.
    pub show_storage: bool,
    pub show_city: bool,
    /// Gewählte Rubrik im Baumenü.
    pub build_cat: BuildCategory,
    /// Startkachel einer Mauerlinie beim Ziehen im Baumodus.
    pub line_start: Option<IVec2>,
    /// Heldenfenster (Taste H) und der Held, auf den es sich bezieht.
    pub show_hero: bool,
    pub hero_focus: Option<UnitId>,
    last_view: Option<ViewParams>,
    last_select_voice: f32,
    hero_greeted: bool,
}

impl MissionScene {
    pub fn new(data: &GameData, mission: MissionDef, settings: MatchSettings, max_particles: usize) -> Result<MissionScene, String> {
        let world = SimWorld::new(data, mission, settings)?;
        Ok(Self::from_world(data, world, max_particles))
    }

    pub fn from_world(data: &GameData, world: SimWorld, max_particles: usize) -> MissionScene {
        let start = world
            .building_with_tag(&world.mission_def.defeat_tag)
            .and_then(|b| world.buildings.get(b))
            .map(|b| b.center() + Vec2::new(3.0, -1.0))
            .or_else(|| world.mission_def.starts.iter().find(|s| s.owner == PLAYER).map(|s| Vec2::new(s.x, s.y)))
            .or_else(|| world.units.iter().find(|(_, u)| u.owner == PLAYER).map(|(_, u)| u.pos))
            .unwrap_or(Vec2::new(world.map.width as f32 * 0.5, world.map.height as f32 * 0.5));
        let fog = !world.fog.disabled;
        MissionScene {
            step: FixedStep::new(data.rules.tick_rate),
            camera: Camera2D { center: tile_to_world(start), target_lines: 330.0 },
            world,
            view: WorldView::new(max_particles),
            selection: Vec::new(),
            selected_building: None,
            groups: Default::default(),
            mode: CommandMode::Normal,
            drag_start: None,
            speed: 1.0,
            messages: VecDeque::new(),
            toast: None,
            menu_open: false,
            menu_page: crate::hud::MenuPage::Main,
            show_objectives: true,
            autosave_timer: 0.0,
            no_save: false,
            last_click: (-10.0, None),
            hovered: None,
            minimap: None,
            minimap_timer: 0.0,
            save_name: "spielstand".into(),
            status: None,
            action: SceneAction::None,
            fog_enabled: fog,
            time_alive: 0.0,
            show_storage: false,
            show_city: false,
            build_cat: BuildCategory::Housing,
            line_start: None,
            show_hero: false,
            hero_focus: None,
            last_view: None,
            last_select_voice: -10.0,
            hero_greeted: false,
        }
    }

    /// Lädt einen Spielstand.
    pub fn load(path: &std::path::Path, data: &GameData, max_particles: usize) -> Result<MissionScene, String> {
        let (_, payload) = ashen_sim::save::load_from(path, data).map_err(|e| e.to_string())?;
        let mut scene = Self::from_world(data, payload.world, max_particles);
        if let Ok(cs) = ron::from_str::<ClientState>(&payload.client_state) {
            scene.camera.center = Vec2::new(cs.camera.0, cs.camera.1);
            scene.camera.target_lines = cs.lines.clamp(220.0, 480.0);
            scene.speed = cs.speed.clamp(0.0, 3.0);
        }
        scene.status = Some(format!("Geladen: {}", path.file_stem().map(|s| s.to_string_lossy()).unwrap_or_default()));
        Ok(scene)
    }

    fn client_state(&self) -> String {
        ron::to_string(&ClientState { camera: (self.camera.center.x, self.camera.center.y), lines: self.camera.target_lines, speed: self.speed }).unwrap_or_default()
    }

    pub fn save(&mut self, data: &GameData, slot: &str) {
        if self.no_save {
            self.status = Some("Eiserner Mann: Speichern ist gesperrt.".into());
            return;
        }
        match ashen_sim::save::save_to(&saves_dir(), slot, &self.world, data, self.client_state()) {
            Ok(p) => self.status = Some(format!("Gespeichert: {}", p.file_name().map(|s| s.to_string_lossy()).unwrap_or_default())),
            Err(e) => self.status = Some(format!("Speichern fehlgeschlagen: {e}")),
        }
    }

    fn cmd(&mut self, command: Command) {
        self.world.push_command(PlayerCommand { player: PLAYER, command });
    }

    pub fn selected_hero(&self, data: &GameData) -> Option<UnitId> {
        self.selection.iter().copied().find(|id| self.world.units.get(*id).is_some_and(|u| data.unit(u.kind).class == UnitClass::Hero && u.owner == PLAYER))
    }

    pub fn has_workers(&self, data: &GameData) -> bool {
        self.selection.iter().any(|id| self.world.units.get(*id).is_some_and(|u| u.owner == PLAYER && data.unit(u.kind).build_speed > 0.0))
    }

    fn own_selection(&self) -> Vec<UnitId> {
        self.selection.iter().copied().filter(|id| self.world.units.get(*id).is_some_and(|u| u.owner == PLAYER)).collect()
    }

    pub fn select_voice(&mut self, data: &GameData, audio: &mut ashen_engine::audio::AudioSystem, situation: &str) {
        if self.time_alive - self.last_select_voice < 0.6 {
            return;
        }
        if let Some(u) = self.selection.first().and_then(|id| self.world.units.get(*id))
            && u.owner == PLAYER
        {
            self.last_select_voice = self.time_alive;
            let id = &data.unit(u.kind).id;
            // Die Heldin begrüßt beim ersten Anwählen.
            if situation == "select" && data.unit(u.kind).class == UnitClass::Hero && !self.hero_greeted {
                self.hero_greeted = true;
                audio.play(&format!("voice.{id}.greet"));
            } else {
                audio.play(&format!("voice.{id}.{situation}"));
            }
        }
    }

    // ------------------------------------------------------------ Befehle aus UI

    pub fn command_stop(&mut self) {
        let u = self.own_selection();
        self.cmd(Command::Stop { units: u });
    }

    pub fn command_hold(&mut self) {
        let u = self.own_selection();
        self.cmd(Command::Hold { units: u });
    }

    /// Führender Fähigkeitsträger der Auswahl: Held bevorzugt, sonst erste eigene Einheit mit Fähigkeiten.
    pub fn ability_lead(&self, data: &GameData) -> Option<UnitId> {
        self.selected_hero(data).or_else(|| {
            self.selection.iter().copied().find(|id| self.world.units.get(*id).is_some_and(|u| u.owner == PLAYER && ashen_sim::systems::heroes::ability_list(data, u).next().is_some()))
        })
    }

    /// Wirkt die Fähigkeit `index` des führenden Einheitentyps mit allen gleichartigen, bereiten Einheiten.
    /// Zielzauber wechseln in den Zielmodus und wirken beim nächsten Klick.
    pub fn use_ability(&mut self, data: &GameData, index: u8) {
        let Some(lead) = self.ability_lead(data) else { return };
        let Some(name) = self.world.units.get(lead).and_then(|u| ashen_sim::systems::heroes::ability_list(data, u).nth(index as usize).cloned()) else { return };
        let Some(aid) = data.ability_id(&name) else { return };
        if data.ability(aid).targeting == ashen_sim::data::AbilityTargeting::Point {
            self.mode = CommandMode::Cast(index);
            return;
        }
        for (id, pos) in self.ability_casters(data, &name, index) {
            self.cmd(Command::UseAbility { unit: id, ability: index, point: pos });
        }
    }

    /// Eigene ausgewählte Einheiten, die an Index `index` die Fähigkeit `name` haben.
    fn ability_casters(&self, data: &GameData, name: &str, index: u8) -> Vec<(UnitId, Vec2)> {
        self.selection
            .iter()
            .filter_map(|id| self.world.units.get(*id).map(|u| (*id, u)))
            .filter(|(_, u)| u.owner == PLAYER && ashen_sim::systems::heroes::ability_list(data, u).nth(index as usize).is_some_and(|n| n == name))
            .map(|(id, u)| (id, u.pos))
            .collect()
    }

    /// Zielzauber an einem Bodenpunkt wirken (nach Klick im Zielmodus).
    fn cast_at(&mut self, data: &GameData, index: u8, point: Vec2) {
        let Some(lead) = self.ability_lead(data) else { return };
        let Some(name) = self.world.units.get(lead).and_then(|u| ashen_sim::systems::heroes::ability_list(data, u).nth(index as usize).cloned()) else { return };
        for (id, _) in self.ability_casters(data, &name, index) {
            self.cmd(Command::UseAbility { unit: id, ability: index, point });
        }
    }

    pub fn set_formation(&mut self, formation: ashen_sim::rpg::Formation) {
        let units = self.own_selection();
        self.cmd(Command::SetFormation { units, formation });
    }

    /// Wechselt zur nächsten Formation der Reihe Block → Linie → Keil → Kolonne → Karree.
    pub fn cycle_formation(&mut self) {
        use ashen_sim::rpg::Formation;
        let cur = self.selection.iter().find_map(|id| self.world.units.get(*id)).map(|u| u.formation).unwrap_or_default();
        let i = Formation::ALL.iter().position(|f| *f == cur).unwrap_or(0);
        self.set_formation(Formation::ALL[(i + 1) % Formation::ALL.len()]);
    }

    /// Der Held, auf den sich das Heldenfenster bezieht.
    pub fn active_hero(&self, data: &GameData) -> Option<UnitId> {
        let alive = |id: &UnitId| self.world.units.get(*id).is_some_and(|u| u.owner == PLAYER && data.unit(u.kind).class == UnitClass::Hero);
        self.selected_hero(data)
            .or(self.hero_focus.filter(alive))
            .or_else(|| self.world.units.iter().find(|(_, u)| u.owner == PLAYER && data.unit(u.kind).class == UnitClass::Hero).map(|(id, _)| id))
    }

    pub fn learn_talent(&mut self, unit: UnitId, talent: usize) {
        self.cmd(Command::LearnTalent { unit, talent: talent as u16 });
    }

    pub fn equip_item(&mut self, unit: UnitId, index: usize) {
        self.cmd(Command::EquipItem { unit, index: index as u8 });
    }

    pub fn unequip_item(&mut self, unit: UnitId, slot: usize) {
        self.cmd(Command::UnequipItem { unit, slot: slot as u8 });
    }

    pub fn drop_item(&mut self, unit: UnitId, index: usize) {
        self.cmd(Command::DropItem { unit, index: index as u8 });
    }

    pub fn use_item(&mut self, unit: UnitId, index: usize) {
        self.cmd(Command::UseItem { unit, index: index as u8 });
    }

    pub fn research(&mut self, building: BuildingId, upgrade: usize) {
        self.cmd(Command::Research { building, upgrade: upgrade as u8 });
    }

    pub fn cancel_research(&mut self, building: BuildingId) {
        self.cmd(Command::CancelResearch { building });
    }

    pub fn revive_hero(&mut self, building: BuildingId, hero: ashen_sim::data::UnitTypeId) {
        self.cmd(Command::ReviveHero { building, hero });
    }

    pub fn train(&mut self, building: BuildingId, unit: ashen_sim::data::UnitTypeId) {
        self.cmd(Command::Train { building, unit });
    }

    pub fn cancel_train(&mut self, building: BuildingId, index: usize) {
        self.cmd(Command::CancelTrain { building, index });
    }

    pub fn demolish(&mut self, building: BuildingId) {
        self.cmd(Command::Demolish { building });
        self.selected_building = None;
    }

    pub fn set_tax(&mut self, level: u8) {
        self.cmd(Command::SetTax { level });
    }

    pub fn set_workers(&mut self, building: BuildingId, limit: Option<u32>) {
        self.cmd(Command::SetWorkers { building, limit });
    }

    pub fn set_recipe(&mut self, building: BuildingId, recipe: u8) {
        self.cmd(Command::SetRecipe { building, recipe });
    }

    pub fn set_gate(&mut self, building: BuildingId, open: bool) {
        self.cmd(Command::SetGate { building, open });
    }

    pub fn trade(&mut self, item: Item, amount: i32) {
        self.cmd(Command::Trade { item, amount });
    }

    pub fn advance_era(&mut self) {
        self.cmd(Command::AdvanceEra);
    }

    /// Bauteile einer Mauerlinie von `a` nach `b` (Bresenham, diagonale Schritte sind erlaubt).
    pub fn wall_line(a: IVec2, b: IVec2) -> Vec<IVec2> {
        let (mut x, mut y) = (a.x, a.y);
        let (dx, dy) = ((b.x - a.x).abs(), -(b.y - a.y).abs());
        let (sx, sy) = (if a.x < b.x { 1 } else { -1 }, if a.y < b.y { 1 } else { -1 });
        let mut err = dx + dy;
        let mut out = vec![IVec2::new(x, y)];
        while (x, y) != (b.x, b.y) && out.len() < 80 {
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
            out.push(IVec2::new(x, y));
        }
        out
    }

    pub fn center_on(&mut self, tile: Vec2) {
        self.camera.center = tile_to_world(tile);
    }

    // ------------------------------------------------------------ Frame-Logik

    pub fn update(&mut self, engine: &mut Engine, data: &GameData, art: &ArtLibrary, settings: &UserSettings, dt: f32) {
        self.time_alive += dt;
        let view = self.camera.view(engine.size, engine.renderer.settings.min_lines);
        self.last_view = Some(view);
        if !self.menu_open {
            self.controls(engine, data, &view, settings, dt);
        }
        // Simulation.
        let paused = self.menu_open || self.speed <= 0.0 || self.world.mission.outcome.is_some();
        let steps = if paused { 0 } else { self.step.advance(dt * self.speed) };
        for _ in 0..steps {
            self.world.step(data);
            let events = std::mem::take(&mut self.world.events);
            for e in &events {
                self.view.on_event(e, &self.world, data, art);
                audio_bridge::on_event(&mut engine.audio, e, &self.world, data, &view);
                self.on_event(e, data);
            }
        }
        self.view.update(if paused { 0.0 } else { dt * self.speed.max(1.0) });
        // Auswahl bereinigen.
        let world = &self.world;
        self.selection.retain(|id| world.units.contains(*id));
        if self.selected_building.is_some_and(|b| !world.buildings.contains(b)) {
            self.selected_building = None;
        }
        for g in &mut self.groups {
            g.retain(|id| world.units.contains(*id));
        }
        for m in &mut self.messages {
            m.age += dt;
        }
        while self.messages.front().is_some_and(|m| m.age > 14.0) || self.messages.len() > 4 {
            self.messages.pop_front();
        }
        if let Some((_, t)) = &mut self.toast {
            *t -= dt;
        }
        if self.toast.as_ref().is_some_and(|(_, t)| *t <= 0.0) {
            self.toast = None;
        }
        // Musik und Ambience.
        engine.audio.set_music_intensity(audio_bridge::music_intensity(&self.world, data));
        audio_bridge::ambience(&mut engine.audio, &self.world, world_to_tile(self.camera.center));
        // Autosave.
        if settings.autosave_minutes > 0.0 && !self.no_save && self.world.mission.outcome.is_none() && !paused {
            self.autosave_timer += dt;
            if self.autosave_timer > settings.autosave_minutes * 60.0 {
                self.autosave_timer = 0.0;
                match ashen_sim::save::autosave(&saves_dir(), &self.world, data, self.client_state(), 3) {
                    Ok(_) => self.status = Some("Automatisch gespeichert".into()),
                    Err(e) => log::warn!("Autosave fehlgeschlagen: {e}"),
                }
            }
        }
    }

    fn on_event(&mut self, e: &SimEvent, data: &GameData) {
        match e {
            SimEvent::Message { speaker, text, .. } => self.messages.push_back(Message { speaker: speaker.clone(), text: text.clone(), age: 0.0 }),
            SimEvent::Error { owner: PLAYER, text } => self.toast = Some((text.clone(), 2.5)),
            SimEvent::WaveIncoming { size: 0, .. } => self.toast = Some(("Die Hohlen sammeln sich im Nordosten!".into(), 4.0)),
            SimEvent::WaveIncoming { wave, size, .. } => self.toast = Some((format!("Angriffswelle {wave}: {size} Hohle rücken vor!"), 4.0)),
            SimEvent::NightBegins { night } => self.toast = Some((format!("Die {night}. Nacht bricht herein."), 3.5)),
            SimEvent::DayBegins { day } => self.toast = Some((format!("Tag {day} beginnt."), 3.0)),
            SimEvent::Starvation { active: true, .. } => self.toast = Some(("Hungersnot! Baut Gehöfte – die Moral sinkt, Ausbildung stockt.".into(), 5.0)),
            SimEvent::HeroFallen { .. } => {
                let text = if self.world.settings.hero_permadeath { "Isolde Raventhorn ist gefallen – für immer." } else { "Isolde Raventhorn ist gefallen! Sie kehrt im Bergfried zurück." };
                self.toast = Some((text.into(), 5.0));
            }
            SimEvent::HeroRespawned { .. } => self.toast = Some(("Isolde Raventhorn ist zurückgekehrt.".into(), 3.0)),
            SimEvent::LevelUp { level, .. } => self.toast = Some((format!("Isolde erreicht Stufe {level}!"), 3.0)),
            SimEvent::BuildingCompleted { kind, owner: PLAYER, .. } => self.status = Some(format!("{} fertiggestellt", data.building(*kind).name)),
            SimEvent::Unrest { .. } => self.toast = Some(("Unruhe in Brennhollow! Senkt die Steuern oder stillt die Bedürfnisse.".into(), 5.0)),
            SimEvent::EraAdvanced { era, .. } => {
                let name = data.rules.eras.get(*era as usize - 1).map(|e| e.name.as_str()).unwrap_or("?");
                self.toast = Some((format!("Brennhollow ist nun eine {name}!"), 5.0));
            }
            SimEvent::HouseChanged { kind, up, .. } => {
                let n = &data.building(*kind).name;
                self.status = Some(if *up { format!("Ein Haus wurde zum {n} ausgebaut") } else { format!("Ein Haus verfiel zum {n}") });
            }
            _ => {}
        }
    }

    fn pick_unit(&self, data: &GameData, view: &ViewParams, mouse: Vec2) -> Option<UnitId> {
        let mut best: Option<(f32, UnitId)> = None;
        for (id, u) in self.world.units.iter() {
            if u.owner != PLAYER && self.fog_enabled && !self.world.fog.visible(u.pos) {
                continue;
            }
            let def = data.unit(u.kind);
            let h = if def.class == UnitClass::Boss { 30.0 } else if def.radius > 0.42 { 16.0 } else { 10.0 };
            let s = view.world_to_screen(tile_to_world(u.pos) - Vec2::new(0.0, h));
            let r = (h + 4.0) * view.pixel_scale as f32;
            let d = (s - mouse).length();
            if d < r && best.is_none_or(|(bd, _)| d < bd) {
                best = Some((d, id));
            }
        }
        best.map(|b| b.1)
    }

    fn pick_building(&self, tile: Vec2) -> Option<BuildingId> {
        self.world.buildings.iter().find(|(_, b)| b.distance_to(tile) < 0.05 && (!self.fog_enabled || self.world.fog.explored(b.center()))).map(|(id, _)| id)
    }

    fn pick_node(&self, tile: Vec2) -> Option<NodeId> {
        self.world.nodes.iter().find(|(_, n)| n.distance_to(tile) < 0.3).map(|(id, _)| id)
    }

    fn controls(&mut self, engine: &mut Engine, data: &GameData, view: &ViewParams, settings: &UserSettings, dt: f32) {
        let input = &engine.input;
        let scale = view.pixel_scale as f32;
        // Kamera.
        // Kamera: Pfeiltasten, Bildschirmrand, mittlere Maustaste (Buchstaben sind Befehlen vorbehalten).
        let mut pan = Vec2::ZERO;
        if input.key_down(KeyCode::ArrowUp) {
            pan.y -= 1.0;
        }
        if input.key_down(KeyCode::ArrowDown) {
            pan.y += 1.0;
        }
        if input.key_down(KeyCode::ArrowLeft) {
            pan.x -= 1.0;
        }
        if input.key_down(KeyCode::ArrowRight) {
            pan.x += 1.0;
        }
        if settings.edge_scroll && input.cursor_inside && input.focused && !input.ui_wants_pointer {
            let m = input.mouse;
            let edge = 6.0;
            let (w, h) = (engine.size.0 as f32, engine.size.1 as f32);
            if m.x < edge {
                pan.x -= 1.0;
            }
            if m.x > w - edge {
                pan.x += 1.0;
            }
            if m.y < edge {
                pan.y -= 1.0;
            }
            if m.y > h - edge {
                pan.y += 1.0;
            }
        }
        let speed = 420.0 * settings.scroll_speed * (self.camera.target_lines / 330.0);
        self.camera.center += pan.normalize_or_zero() * speed * dt;
        if input.mouse_down(MouseButton::Middle) {
            self.camera.center -= input.mouse_delta / scale;
        }
        if input.wheel != 0.0 && !input.ui_wants_pointer {
            self.camera.target_lines = (self.camera.target_lines * (1.0 - input.wheel * 0.1)).clamp(220.0, 480.0);
        }
        // Kamera auf Kartengrenzen beschränken.
        let max = self.world.map.width as f32;
        let tile = world_to_tile(self.camera.center).clamp(Vec2::splat(2.0), Vec2::splat(max - 2.0));
        self.camera.center = tile_to_world(tile);

        let mouse = input.mouse;
        let mouse_tile = world_to_tile(view.screen_to_world(mouse));
        self.hovered = if input.ui_wants_pointer { None } else { self.pick_unit(data, view, mouse) };

        // Tastenkürzel.
        let shift = input.shift;
        let cmd_mod = input.command_mod();
        if input.key_pressed(KeyCode::Escape) {
            if self.mode != CommandMode::Normal {
                self.mode = CommandMode::Normal;
            } else if self.show_hero {
                self.show_hero = false;
            } else {
                self.menu_open = true;
                self.menu_page = crate::hud::MenuPage::Main;
            }
        }
        if input.key_pressed(KeyCode::F10) {
            self.menu_open = true;
        }
        if input.key_pressed(KeyCode::KeyL) && !cmd_mod {
            self.show_storage = !self.show_storage;
        }
        if input.key_pressed(KeyCode::KeyC) && !cmd_mod {
            self.show_city = !self.show_city;
        }
        if input.key_pressed(KeyCode::Pause) || input.key_pressed(KeyCode::Backquote) {
            self.speed = if self.speed > 0.0 { 0.0 } else { 1.0 };
        }
        if input.key_pressed(KeyCode::Equal) || input.key_pressed(KeyCode::NumpadAdd) {
            self.speed = (self.speed + 1.0).min(3.0);
        }
        if input.key_pressed(KeyCode::Minus) || input.key_pressed(KeyCode::NumpadSubtract) {
            self.speed = (self.speed - 1.0).max(0.0);
        }
        if input.key_pressed(KeyCode::F5) {
            self.save(data, "schnellspeicher");
        }
        if input.key_pressed(KeyCode::F9) {
            self.action = SceneAction::Load(saves_dir().join(format!("schnellspeicher.{}", ashen_sim::save::EXTENSION)));
        }
        if input.key_pressed(KeyCode::Space)
            && let Some(u) = self.selection.first().and_then(|id| self.world.units.get(*id))
        {
            self.camera.center = tile_to_world(u.pos);
        }
        let h_open = input.key_pressed(KeyCode::KeyH) && !cmd_mod && self.selection.is_empty() && self.selected_building.is_none();
        if (h_open || input.key_pressed(KeyCode::Home))
            && let Some(h) = self.world.hero_of(data, PLAYER) {
                self.selection = vec![h];
                self.selected_building = None;
                if h_open {
                    self.show_hero = true;
                }
                if let Some(u) = self.world.units.get(h) {
                    self.camera.center = tile_to_world(u.pos);
                }
            }
        let digits = [KeyCode::Digit0, KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9];
        for (i, k) in digits.iter().enumerate() {
            if input.key_pressed(*k) {
                if cmd_mod {
                    self.groups[i] = self.own_selection();
                    self.status = Some(format!("Gruppe {i} gespeichert ({} Einheiten)", self.groups[i].len()));
                } else if !self.groups[i].is_empty() {
                    if self.selection == self.groups[i]
                        && let Some(u) = self.selection.first().and_then(|id| self.world.units.get(*id))
                    {
                        self.camera.center = tile_to_world(u.pos);
                    }
                    self.selection = self.groups[i].clone();
                    self.selected_building = None;
                }
            }
        }
        let has_units = !self.own_selection().is_empty();
        if has_units && !cmd_mod {
            if self.mode == CommandMode::BuildMenu {
                for bt in player_buildables(data) {
                    if let Some(code) = data.building(bt).hotkey.and_then(letter_key)
                        && input.key_pressed(code)
                    {
                        self.mode = CommandMode::Place(bt);
                    }
                }
            } else {
                if input.key_pressed(KeyCode::KeyA) {
                    self.mode = CommandMode::AttackMove;
                }
                if input.key_pressed(KeyCode::KeyS) {
                    self.command_stop();
                }
                if input.key_pressed(KeyCode::KeyH) {
                    // Nur Helden gewählt: Heldenfenster; sonst Position halten.
                    let own = self.own_selection();
                    let only_heroes = !own.is_empty() && own.iter().all(|id| self.world.units.get(*id).is_some_and(|u| data.unit(u.kind).class == UnitClass::Hero));
                    if only_heroes {
                        self.show_hero = !self.show_hero;
                    } else {
                        self.command_hold();
                    }
                }
                if input.key_pressed(KeyCode::KeyF) {
                    self.cycle_formation();
                }
                if input.key_pressed(KeyCode::KeyP) {
                    self.mode = CommandMode::Patrol;
                }
                if input.key_pressed(KeyCode::KeyB) && self.has_workers(data) {
                    self.mode = CommandMode::BuildMenu;
                }
                for (i, k) in [KeyCode::KeyQ, KeyCode::KeyW, KeyCode::KeyE, KeyCode::KeyR, KeyCode::KeyT, KeyCode::KeyY].iter().enumerate() {
                    if input.key_pressed(*k) && self.ability_lead(data).is_some() {
                        self.use_ability(data, i as u8);
                    }
                }
            }
        }
        if let Some(b) = self.selected_building
            && let Some(bld) = self.world.buildings.get(b)
            && bld.owner == PLAYER
            && bld.complete
            && !cmd_mod
        {
            let def = data.building(bld.kind);
            for t in &def.trains {
                if let Some(ut) = data.unit_id(t)
                    && let Some(hk) = data.unit(ut).hotkey
                    && let Some(code) = letter_key(hk)
                    && input.key_pressed(code)
                {
                    self.train(b, ut);
                }
            }
            if input.key_pressed(KeyCode::KeyR) {
                self.mode = CommandMode::Rally;
            }
        }

        if input.ui_wants_pointer {
            if input.mouse_released(MouseButton::Left) {
                self.drag_start = None;
                self.line_start = None;
            }
            return;
        }

        // Platzierungsmodus.
        if let CommandMode::Place(bt) = self.mode {
            let def = data.building(bt);
            let tile = place_tile(mouse_tile, def.size);
            let builders: Vec<UnitId> = self.own_selection().into_iter().filter(|id| self.world.units.get(*id).is_some_and(|u| data.unit(u.kind).build_speed > 0.0)).collect();
            if def.fort == Some(Fort::Wall) {
                // Mauern werden als Linie gezogen: Maustaste halten, Ziel wählen, loslassen.
                if input.mouse_pressed(MouseButton::Left) {
                    self.line_start = Some(tile);
                }
                if input.mouse_released(MouseButton::Left)
                    && let Some(start) = self.line_start.take()
                {
                    let tiles = Self::wall_line(start, tile);
                    self.cmd(Command::BuildLine { units: builders, kind: bt, tiles });
                    engine.audio.play("voice.worker.work");
                    if !shift {
                        self.mode = CommandMode::Normal;
                    }
                }
            } else if input.mouse_pressed(MouseButton::Left) {
                self.cmd(Command::Build { units: builders, kind: bt, tile });
                engine.audio.play("voice.worker.work");
                if !shift {
                    self.mode = CommandMode::Normal;
                }
            }
            if input.mouse_pressed(MouseButton::Right) {
                self.line_start = None;
                self.mode = CommandMode::Normal;
            }
            return;
        }
        self.line_start = None;

        // Linksklick: Auswahl bzw. Zielbefehle.
        if input.mouse_pressed(MouseButton::Left) {
            match self.mode {
                CommandMode::AttackMove | CommandMode::Patrol => {
                    let units = self.own_selection();
                    if let Some(t) = self.hovered.filter(|t| self.world.units.get(*t).is_some_and(|u| u.owner == ENEMY)) {
                        self.cmd(Command::Attack { units, target: EntityRef::Unit(t) });
                    } else if self.mode == CommandMode::AttackMove {
                        self.cmd(Command::Move { units, target: mouse_tile, attack_move: true, queue: shift });
                    } else {
                        self.cmd(Command::Patrol { units, target: mouse_tile });
                    }
                    self.view.add_marker(mouse_tile, true);
                    self.select_voice(data, &mut engine.audio, "attack");
                    self.mode = CommandMode::Normal;
                    return;
                }
                CommandMode::Rally => {
                    if let Some(b) = self.selected_building {
                        self.cmd(Command::SetRally { building: b, point: mouse_tile });
                    }
                    self.mode = CommandMode::Normal;
                    return;
                }
                CommandMode::Cast(index) => {
                    self.cast_at(data, index, mouse_tile);
                    self.view.add_marker(mouse_tile, true);
                    if !shift {
                        self.mode = CommandMode::Normal;
                    }
                    return;
                }
                _ => {
                    self.drag_start = Some(mouse);
                }
            }
        }
        if input.mouse_released(MouseButton::Left)
            && let Some(start) = self.drag_start.take()
        {
            let rect_sel = (start - mouse).abs().max_element() > 6.0;
            if rect_sel {
                let min = start.min(mouse);
                let max = start.max(mouse);
                let mut picked: Vec<UnitId> = self
                    .world
                    .units
                    .iter()
                    .filter(|(_, u)| u.owner == PLAYER)
                    .filter(|(_, u)| {
                        let s = view.world_to_screen(tile_to_world(u.pos) - Vec2::new(0.0, 6.0));
                        s.x >= min.x && s.y >= min.y && s.x <= max.x && s.y <= max.y
                    })
                    .map(|(id, _)| id)
                    .collect();
                // Kampfeinheiten bevorzugen, wenn gemischt mit Arbeitern.
                let has_military = picked.iter().any(|id| self.world.units.get(*id).is_some_and(|u| data.unit(u.kind).class != UnitClass::Worker));
                if has_military {
                    picked.retain(|id| self.world.units.get(*id).is_some_and(|u| data.unit(u.kind).class != UnitClass::Worker));
                }
                if shift {
                    for p in picked {
                        if !self.selection.contains(&p) {
                            self.selection.push(p);
                        }
                    }
                } else if !picked.is_empty() {
                    self.selection = picked;
                    self.selected_building = None;
                }
                self.select_voice(data, &mut engine.audio, "select");
            } else if let Some(u) = self.hovered {
                let now = self.time_alive;
                let double = self.last_click.1 == Some(u) && now - self.last_click.0 < 0.35;
                self.last_click = (now, Some(u));
                if double {
                    let kind = self.world.units.get(u).map(|x| x.kind);
                    let (vmin, vmax) = view.visible_world(0.0);
                    self.selection = self
                        .world
                        .units
                        .iter()
                        .filter(|(_, x)| Some(x.kind) == kind && x.owner == PLAYER)
                        .filter(|(_, x)| {
                            let w = tile_to_world(x.pos);
                            w.x >= vmin.x && w.y >= vmin.y && w.x <= vmax.x && w.y <= vmax.y
                        })
                        .map(|(id, _)| id)
                        .collect();
                } else if shift {
                    if let Some(i) = self.selection.iter().position(|x| *x == u) {
                        self.selection.remove(i);
                    } else {
                        self.selection.push(u);
                    }
                } else {
                    self.selection = vec![u];
                }
                self.selected_building = None;
                self.select_voice(data, &mut engine.audio, "select");
                engine.audio.play("sfx.ui_click");
            } else if let Some(b) = self.pick_building(mouse_tile) {
                self.selected_building = Some(b);
                self.selection.clear();
                engine.audio.play("sfx.ui_click");
            } else if !shift {
                self.selection.clear();
                self.selected_building = None;
            }
        }

        // Rechtsklick: kontextabhängiger Befehl.
        if input.mouse_pressed(MouseButton::Right) {
            if self.mode != CommandMode::Normal {
                self.mode = CommandMode::Normal;
                return;
            }
            let units = self.own_selection();
            if units.is_empty() {
                if let Some(b) = self.selected_building
                    && self.world.buildings.get(b).is_some_and(|x| x.owner == PLAYER)
                {
                    self.cmd(Command::SetRally { building: b, point: mouse_tile });
                    self.view.add_marker(mouse_tile, false);
                }
                return;
            }
            let enemy_unit = self.hovered.filter(|t| self.world.units.get(*t).is_some_and(|u| u.owner == ENEMY));
            let building = self.pick_building(mouse_tile);
            if let Some(t) = enemy_unit {
                self.cmd(Command::Attack { units, target: EntityRef::Unit(t) });
                self.view.add_marker(mouse_tile, true);
                self.select_voice(data, &mut engine.audio, "attack");
            } else if let Some(b) = building.filter(|b| self.world.buildings.get(*b).is_some_and(|x| x.owner == ENEMY)) {
                self.cmd(Command::Attack { units, target: EntityRef::Building(b) });
                self.view.add_marker(mouse_tile, true);
                self.select_voice(data, &mut engine.audio, "attack");
            } else if let (Some(node), true) = (self.pick_node(mouse_tile), self.has_workers(data)) {
                let workers: Vec<UnitId> = units.iter().copied().filter(|id| self.world.units.get(*id).is_some_and(|u| data.unit(u.kind).carry > 0)).collect();
                let others: Vec<UnitId> = units.iter().copied().filter(|id| !workers.contains(id)).collect();
                self.cmd(Command::Gather { units: workers, node });
                if !others.is_empty() {
                    self.cmd(Command::Move { units: others, target: mouse_tile, attack_move: false, queue: shift });
                }
                self.view.add_marker(mouse_tile, false);
                engine.audio.play("voice.worker.work");
            } else if let Some(b) = building.filter(|b| self.world.buildings.get(*b).is_some_and(|x| x.owner == PLAYER && (!x.complete || x.hp < x.max_hp)))
                && self.has_workers(data)
            {
                self.cmd(Command::Repair { units, building: b });
                engine.audio.play("voice.worker.work");
            } else {
                self.cmd(Command::Move { units, target: mouse_tile, attack_move: false, queue: shift });
                self.view.add_marker(mouse_tile, false);
                self.select_voice(data, &mut engine.audio, "move");
            }
        }
    }

    /// Füllt den Render-Frame.
    pub fn render(&mut self, engine: &mut Engine, data: &GameData, art: &ArtLibrary) {
        engine.frame.camera = self.camera;
        let view = self.camera.view(engine.size, engine.renderer.settings.min_lines);
        let mut overlay = Overlay { selected_units: self.selection.iter().copied().collect(), selected_building: self.selected_building, hovered: self.hovered, ghost: None, ghost_line: Vec::new(), show_all_bars: engine.input.alt };
        if let CommandMode::Place(bt) = self.mode {
            let def = data.building(bt);
            let mouse_tile = world_to_tile(view.screen_to_world(engine.input.mouse));
            let tile = place_tile(mouse_tile, def.size);
            let afford = self.world.afford(PLAYER, &def.cost, &def.goods_cost_r).is_ok();
            match self.line_start {
                Some(start) if def.fort == Some(Fort::Wall) => {
                    // Vorschau der Linie; so viele Teile sind „gültig“, wie bezahlbar sind.
                    let mut left = self.world.players[0].resources;
                    overlay.ghost_line = Self::wall_line(start, tile)
                        .into_iter()
                        .map(|t| {
                            let ok = self.world.can_place(data, bt, t, PLAYER).is_ok() && left.covers(&def.cost);
                            if ok {
                                left.sub(&def.cost);
                            }
                            (t, ok)
                        })
                        .collect();
                }
                _ => {
                    let ok = self.world.can_place(data, bt, tile, PLAYER).is_ok() && afford;
                    overlay.ghost = Some((def.id.clone(), tile, def.size, ok));
                }
            }
        }
        let alpha = self.step.alpha();
        self.view.render(&self.world, data, art, &mut engine.frame, &view, alpha, &overlay, self.fog_enabled);
        engine.fow_enabled = self.fog_enabled;
        engine.fow_size = (self.world.fog.width as u32, self.world.fog.height as u32);
        if self.fog_enabled {
            engine.renderer.set_fog_of_war(&engine.gpu, engine.fow_size, &self.world.fog.cells, self.world.fog.revision);
        }
    }
}

/// Vom Spieler baubare Gebäude (aus den Daten, inklusive Mods).
pub fn player_buildables(data: &GameData) -> Vec<BuildingTypeId> {
    data.buildings
        .iter()
        .filter(|b| b.buildable && b.faction == ashen_sim::data::Faction::EmberCrown)
        .filter_map(|b| data.building_id(&b.id))
        .collect()
}

/// Obere linke Kachel eines Gebäudes, zentriert unter dem Mauszeiger.
pub fn place_tile(mouse_tile: Vec2, size: (u8, u8)) -> IVec2 {
    let p = mouse_tile - Vec2::new(size.0 as f32 * 0.5, size.1 as f32 * 0.5) + Vec2::splat(0.5);
    TileMap::tile_of(p)
}

pub fn letter_key(c: char) -> Option<KeyCode> {
    Some(match c.to_ascii_uppercase() {
        'A' => KeyCode::KeyA,
        'B' => KeyCode::KeyB,
        'C' => KeyCode::KeyC,
        'D' => KeyCode::KeyD,
        'E' => KeyCode::KeyE,
        'F' => KeyCode::KeyF,
        'G' => KeyCode::KeyG,
        'K' => KeyCode::KeyK,
        'M' => KeyCode::KeyM,
        'P' => KeyCode::KeyP,
        'Q' => KeyCode::KeyQ,
        'S' => KeyCode::KeyS,
        'T' => KeyCode::KeyT,
        'V' => KeyCode::KeyV,
        'W' => KeyCode::KeyW,
        'X' => KeyCode::KeyX,
        'Z' => KeyCode::KeyZ,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wall_line_is_contiguous_and_includes_ends() {
        for (a, b) in [(IVec2::new(3, 3), IVec2::new(9, 3)), (IVec2::new(3, 3), IVec2::new(8, 8)), (IVec2::new(10, 4), IVec2::new(4, 9)), (IVec2::new(5, 5), IVec2::new(5, 5))] {
            let line = MissionScene::wall_line(a, b);
            assert_eq!(line.first(), Some(&a));
            assert_eq!(line.last(), Some(&b));
            assert!(line.windows(2).all(|w| (w[0] - w[1]).abs().max_element() == 1), "lückenlos: {line:?}");
        }
        assert_eq!(MissionScene::wall_line(IVec2::new(0, 0), IVec2::new(4, 4)).len(), 5, "Diagonale braucht nur eine Kachel je Schritt");
    }
}
