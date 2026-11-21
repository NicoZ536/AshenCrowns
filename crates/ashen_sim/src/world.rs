//! Der vollständige Simulationszustand und der feste Simulationstakt.

use crate::arena::Arena;
use crate::chronicle::{Chronicle, ChronicleKind};
use crate::command::PlayerCommand;
use crate::data::{BuildingTypeId, Cost, DamageType, GameData, ResourceKind, UnitClass, UnitTypeId};
use crate::entity::*;
use crate::env::Environment;
use crate::events::SimEvent;
use crate::fog::FogOfWar;
use crate::map::TileMap;
use crate::mission::{GameMode, MatchSettings, MissionDef, MissionState, ObjectiveState};
use crate::path::Pathfinder;
use crate::spatial::SpatialGrid;
use crate::systems;
use glam::{IVec2, Vec2};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

/// Aktuelles Jahr der Spielwelt (Kanon: 897 n.F.).
pub const START_YEAR: u32 = 897;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerStats {
    pub units_trained: u32,
    pub units_lost: u32,
    pub kills: u32,
    pub buildings_built: u32,
    pub gathered: Cost,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub resources: Cost,
    pub pop: u32,
    pub pop_cap: u32,
    pub starving: bool,
    pub stats: PlayerStats,
}

/// Steuerung der gegnerischen Angriffswellen.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Director {
    pub enabled: bool,
    pub next_wave_in: f32,
    pub wave: u32,
    pub warned: bool,
    pub boss_summons_done: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SimWorld {
    pub tick: u64,
    pub time: f32,
    pub settings: MatchSettings,
    pub mission_def: MissionDef,
    pub mission: MissionState,
    pub map: TileMap,
    pub units: Arena<Unit>,
    pub buildings: Arena<Building>,
    pub nodes: Arena<ResourceNode>,
    pub projectiles: Vec<Projectile>,
    pub players: Vec<PlayerState>,
    pub env: Environment,
    pub fog: FogOfWar,
    pub director: Director,
    pub chronicle: Chronicle,
    pub rng: ChaCha8Rng,
    pub hero_respawn: Option<(UnitTypeId, f32)>,
    #[serde(skip)]
    pub events: Vec<SimEvent>,
    #[serde(skip)]
    pub(crate) pending: Vec<PlayerCommand>,
    #[serde(skip)]
    pub(crate) pathfinder: Pathfinder,
    #[serde(skip)]
    pub(crate) spatial: SpatialGrid,
    #[serde(skip)]
    pub(crate) scratch: Vec<UnitId>,
}

impl SimWorld {
    /// Erzeugt eine neue Partie aus Missionsdefinition und Startoptionen.
    pub fn new(data: &GameData, mission: MissionDef, settings: MatchSettings) -> Result<SimWorld, String> {
        let map = crate::mapgen::generate(mission.map_seed, settings.seed);
        let (w, h) = (map.width, map.height);
        let mut world = SimWorld {
            tick: 0,
            time: 0.0,
            rng: ChaCha8Rng::seed_from_u64(settings.seed ^ 0xA5_4E_C0_0D),
            settings,
            mission: MissionState::default(),
            map,
            units: Arena::default(),
            buildings: Arena::default(),
            nodes: Arena::default(),
            projectiles: Vec::new(),
            players: vec![
                PlayerState { name: "Glutkrone".into(), resources: mission.start_resources, pop: 0, pop_cap: 0, starving: false, stats: PlayerStats::default() },
                PlayerState { name: "Die Hohlen".into(), resources: Cost::default(), pop: 0, pop_cap: 999, starving: false, stats: PlayerStats::default() },
                PlayerState { name: "Neutral".into(), resources: Cost::default(), pop: 0, pop_cap: 999, starving: false, stats: PlayerStats::default() },
            ],
            env: Environment::default(),
            fog: FogOfWar::new(w, h),
            director: Director::default(),
            chronicle: Chronicle::default(),
            hero_respawn: None,
            events: Vec::new(),
            pending: Vec::new(),
            pathfinder: Pathfinder::default(),
            spatial: SpatialGrid::default(),
            scratch: Vec::new(),
            mission_def: mission,
        };
        world.apply_mode_rules();
        crate::mapgen::populate_resources(&mut world, data);
        world.apply_placements(data)?;
        systems::economy::recompute_population(&mut world, data);
        systems::fog::update(&mut world, data);
        let name = world.mission_def.name.clone();
        world.record(ChronicleKind::Founding, format!("Die Wächterin der Aschmark übernimmt den Befehl über Brennhollow. ({name})"));
        Ok(world)
    }

    fn apply_mode_rules(&mut self) {
        let waves = self.mission_def.waves.clone();
        self.director.enabled = waves.enabled;
        self.director.next_wave_in = waves.first_wave;
        match self.settings.mode {
            GameMode::Campaign => {}
            GameMode::Skirmish => {
                self.director.next_wave_in = waves.first_wave * 0.8;
            }
            GameMode::Survival => {
                self.director.next_wave_in = 90.0;
            }
            GameMode::Sandbox => {
                self.director.enabled = false;
                self.players[PLAYER as usize].resources.add(&Cost { gold: 5000.0, wood: 5000.0, stone: 3000.0, food: 3000.0 });
                self.fog.disabled = true;
            }
        }
    }

    fn apply_placements(&mut self, data: &GameData) -> Result<(), String> {
        let placements = self.mission_def.placements.clone();
        for p in placements {
            let count = p.count.max(1);
            if let Some(bt) = data.building_id(&p.id) {
                let tile = IVec2::new(p.x as i32, p.y as i32);
                let id = self.place_building(data, bt, p.owner, tile, true);
                if let Some(b) = self.buildings.get_mut(id) {
                    b.tag = p.tag.clone();
                    if self.settings.mode == GameMode::Survival && p.tag.as_deref() == Some(self.mission_def.enemy_base_tag.as_str()) {
                        b.invulnerable = true;
                    }
                }
            } else if let Some(ut) = data.unit_id(&p.id) {
                for i in 0..count {
                    // Kleine Spirale, damit Gruppen nicht exakt übereinander stehen.
                    let a = i as f32 * 2.399;
                    let r = 0.55 * (i as f32).sqrt();
                    let pos = Vec2::new(p.x + a.cos() * r, p.y + a.sin() * r);
                    let id = self.spawn_unit(data, ut, p.owner, pos);
                    if let Some(u) = self.units.get_mut(id) {
                        u.tag = p.tag.clone();
                        if p.guard {
                            u.order = Order::Guard { post: pos, radius: 6.5 };
                        }
                    }
                }
            } else {
                return Err(format!("Missionsplatzierung verweist auf unbekannte ID '{}'", p.id));
            }
        }
        Ok(())
    }

    // ------------------------------------------------------------------ Erzeugung

    pub fn spawn_unit(&mut self, data: &GameData, kind: UnitTypeId, owner: PlayerId, pos: Vec2) -> UnitId {
        let def = data.unit(kind);
        let tile = TileMap::tile_of(pos);
        let pos = if self.map.walkable(tile) {
            pos
        } else {
            self.map.nearest_walkable(tile, 8).map(TileMap::center_of).unwrap_or(pos)
        };
        let unit = Unit::new(kind, owner, pos, def.hp, def.mana, def.morale, def.abilities.len());
        let id = self.units.insert(unit);
        self.events.push(SimEvent::UnitSpawned { id, kind, owner, pos });
        id
    }

    /// Platziert ein Gebäude. `complete = false` erzeugt eine Baustelle.
    pub fn place_building(&mut self, data: &GameData, kind: BuildingTypeId, owner: PlayerId, tile: IVec2, complete: bool) -> BuildingId {
        let def = data.building(kind);
        let size = IVec2::new(def.size.0 as i32, def.size.1 as i32);
        self.map.occupy(tile, size, true);
        let b = Building {
            kind,
            owner,
            tile,
            size,
            hp: if complete { def.hp } else { def.hp * 0.1 },
            max_hp: def.hp,
            progress: if complete { 1.0 } else { 0.0 },
            complete,
            queue: Vec::new(),
            rally: None,
            attack_cd: 0.0,
            income_acc: 0.0,
            tag: None,
            invulnerable: false,
            last_hit_time: -100.0,
        };
        let center = b.center();
        let id = self.buildings.insert(b);
        self.push_units_out_of(tile, size);
        self.events.push(SimEvent::BuildingPlaced { id, kind, owner, pos: center });
        id
    }

    pub fn spawn_node(&mut self, kind: NodeKind, tile: IVec2, amount: u32, variant: u8) -> Option<NodeId> {
        let size = kind.size();
        for y in 0..size.y {
            for x in 0..size.x {
                if !self.map.walkable(tile + IVec2::new(x, y)) {
                    return None;
                }
            }
        }
        self.map.occupy(tile, size, true);
        Some(self.nodes.insert(ResourceNode { kind, tile, amount, max_amount: amount, variant }))
    }

    /// Schiebt Einheiten aus einer neu belegten Fläche heraus.
    fn push_units_out_of(&mut self, tile: IVec2, size: IVec2) {
        let min = tile.as_vec2();
        let max = min + size.as_vec2();
        let ids = self.units.ids();
        for id in ids {
            let Some(u) = self.units.get(id) else { continue };
            let p = u.pos;
            if p.x >= min.x - 0.2 && p.x <= max.x + 0.2 && p.y >= min.y - 0.2 && p.y <= max.y + 0.2 {
                let target = self.map.nearest_walkable(TileMap::tile_of(p), 6).map(TileMap::center_of);
                if let (Some(t), Some(u)) = (target, self.units.get_mut(id)) {
                    u.pos = t;
                    u.prev_pos = t;
                }
            }
        }
    }

    // ------------------------------------------------------------------ Abfragen

    pub fn entity_pos(&self, e: EntityRef) -> Option<Vec2> {
        match e {
            EntityRef::Unit(id) => self.units.get(id).map(|u| u.pos),
            EntityRef::Building(id) => self.buildings.get(id).map(|b| b.center()),
        }
    }

    pub fn entity_owner(&self, e: EntityRef) -> Option<PlayerId> {
        match e {
            EntityRef::Unit(id) => self.units.get(id).map(|u| u.owner),
            EntityRef::Building(id) => self.buildings.get(id).map(|b| b.owner),
        }
    }

    pub fn entity_alive(&self, e: EntityRef) -> bool {
        match e {
            EntityRef::Unit(id) => self.units.get(id).is_some_and(|u| u.hp > 0.0),
            EntityRef::Building(id) => self.buildings.get(id).is_some_and(|b| b.hp > 0.0),
        }
    }

    /// Abstand von `p` zur Oberfläche des Ziels.
    pub fn entity_distance(&self, data: &GameData, e: EntityRef, p: Vec2) -> Option<f32> {
        match e {
            EntityRef::Unit(id) => self.units.get(id).map(|u| ((u.pos - p).length() - data.unit(u.kind).radius).max(0.0)),
            EntityRef::Building(id) => self.buildings.get(id).map(|b| b.distance_to(p)),
        }
    }

    pub fn hostile(a: PlayerId, b: PlayerId) -> bool {
        a != b && a != NEUTRAL && b != NEUTRAL
    }

    /// Effektive Sichtweite einer Einheit unter Berücksichtigung von Nacht und Nebel.
    pub fn sight_of(&self, data: &GameData, u: &Unit) -> f32 {
        let def = data.unit(u.kind);
        let mut s = def.sight;
        if u.owner == PLAYER {
            if self.env.is_night() {
                s *= data.rules.night_sight_mult;
            }
            if self.env.weather == crate::env::Weather::Fog {
                s *= data.rules.fog_sight_mult;
            }
        }
        s
    }

    pub fn hero_of(&self, data: &GameData, owner: PlayerId) -> Option<UnitId> {
        self.units.iter().find(|(_, u)| u.owner == owner && data.unit(u.kind).class == UnitClass::Hero).map(|(id, _)| id)
    }

    pub fn building_with_tag(&self, tag: &str) -> Option<BuildingId> {
        self.buildings.iter().find(|(_, b)| b.tag.as_deref() == Some(tag)).map(|(id, _)| id)
    }

    pub fn objective_state(&self, id: &str) -> Option<ObjectiveState> {
        self.mission.objectives.iter().find(|(o, _)| o == id).map(|(_, s)| *s)
    }

    /// Nächstes eigenes, fertiges Abgabegebäude.
    pub fn nearest_dropoff(&self, data: &GameData, owner: PlayerId, p: Vec2) -> Option<BuildingId> {
        self.buildings
            .iter()
            .filter(|(_, b)| b.owner == owner && b.complete && data.building(b.kind).dropoff)
            .min_by(|a, b| a.1.distance_to(p).total_cmp(&b.1.distance_to(p)))
            .map(|(id, _)| id)
    }

    /// Prüft, ob ein Gebäude an `tile` platziert werden kann.
    pub fn can_place(&self, data: &GameData, kind: BuildingTypeId, tile: IVec2, owner: PlayerId) -> Result<(), String> {
        let def = data.building(kind);
        for y in 0..def.size.1 as i32 {
            for x in 0..def.size.0 as i32 {
                let t = tile + IVec2::new(x, y);
                if !self.map.buildable(t) {
                    return Err("Hier kann nicht gebaut werden.".into());
                }
                if owner == PLAYER && !self.fog.explored(TileMap::center_of(t)) {
                    return Err("Gelände unerforscht.".into());
                }
            }
        }
        for r in &def.requires {
            let Some(req) = data.building_id(r) else { continue };
            if !self.buildings.iter().any(|(_, b)| b.owner == owner && b.kind == req && b.complete) {
                return Err(format!("Benötigt: {}", data.building(req).name));
            }
        }
        Ok(())
    }

    pub fn current_year(&self) -> u32 {
        START_YEAR
    }

    pub fn record(&mut self, kind: ChronicleKind, text: impl Into<String>) {
        let phase = self.env.phase().label();
        let (year, day) = (self.current_year(), self.env.day);
        self.chronicle.record(year, day, phase, kind, text);
    }

    pub fn error(&mut self, owner: PlayerId, text: impl Into<String>) {
        self.events.push(SimEvent::Error { owner, text: text.into() });
    }

    // ------------------------------------------------------------------ Schaden

    /// Fügt einem Ziel Schaden zu (Rüstung, Typen, Schild, Moral). Tote werden später entfernt.
    pub fn deal_damage(&mut self, data: &GameData, target: EntityRef, raw: f32, dtype: DamageType, bonus: &[(UnitClass, f32)], source: Option<EntityRef>) {
        match target {
            EntityRef::Unit(id) => {
                let Some(u) = self.units.get(id) else { return };
                let def = data.unit(u.kind);
                let class_mult = bonus.iter().find(|(c, _)| *c == def.class).map(|(_, m)| *m).unwrap_or(1.0);
                let armor = def.armor + u.armor_bonus() + (u.level.saturating_sub(1)) as f32 * 0.5;
                let reduction = if armor >= 0.0 { armor * 0.06 / (1.0 + 0.06 * armor) } else { -(1.0 - 0.94f32.powf(-armor)) };
                let mut amount = raw * dtype.multiplier(def.armor_type) * class_mult * (1.0 - reduction);
                let pos = u.pos;
                let fearless = def.fearless;
                let max_hp = u.max_hp;
                let u = self.units.get_mut(id).expect("checked above");
                let mut blocked = false;
                if u.shield > 0.0 {
                    let absorbed = u.shield.min(amount);
                    u.shield -= absorbed;
                    amount -= absorbed;
                    blocked = absorbed > 0.0;
                }
                u.hp -= amount;
                u.combat_timer = 4.0;
                u.last_attacker = source;
                if !fearless {
                    u.morale -= amount / max_hp * 38.0;
                }
                self.events.push(SimEvent::Hit { target, pos, amount, damage_type: dtype, blocked });
            }
            EntityRef::Building(id) => {
                let time = self.time;
                let Some(b) = self.buildings.get_mut(id) else { return };
                if b.invulnerable {
                    return;
                }
                let def = data.building(b.kind);
                let reduction = def.armor * 0.06 / (1.0 + 0.06 * def.armor);
                let amount = raw * dtype.multiplier(def.armor_type) * (1.0 - reduction);
                b.hp -= amount;
                b.last_hit_time = time;
                let pos = b.center();
                self.events.push(SimEvent::Hit { target, pos, amount, damage_type: dtype, blocked: false });
            }
        }
    }

    // ------------------------------------------------------------------ Takt

    pub fn push_command(&mut self, cmd: PlayerCommand) {
        self.pending.push(cmd);
    }

    pub fn dt(data: &GameData) -> f32 {
        1.0 / data.rules.tick_rate as f32
    }

    /// Ein fester Simulationsschritt.
    pub fn step(&mut self, data: &GameData) {
        if self.mission.outcome.is_some() {
            return;
        }
        let dt = Self::dt(data);
        self.tick += 1;
        self.time += dt;
        for (_, u) in self.units.iter_mut() {
            u.prev_pos = u.pos;
        }
        for p in &mut self.projectiles {
            p.prev_pos = p.pos;
        }

        let cmds = std::mem::take(&mut self.pending);
        for c in cmds {
            systems::commands::apply(self, data, c);
        }
        systems::environment::tick(self, data, dt);
        systems::economy::tick(self, data, dt);
        systems::ai::tick(self, data, dt);
        let ids: Vec<(UnitId, Vec2)> = self.units.iter().map(|(id, u)| (id, u.pos)).collect();
        self.spatial.rebuild(self.map.width, self.map.height, ids.into_iter());
        systems::units::tick(self, data, dt);
        systems::movement::process_paths(self, data);
        systems::movement::integrate(self, data, dt);
        systems::combat::tick(self, data, dt);
        systems::units::status_tick(self, data, dt);
        systems::combat::cleanup_dead(self, data);
        if self.tick.is_multiple_of(4) {
            systems::fog::update(self, data);
        }
        systems::script::tick(self, data);
        systems::economy::hero_respawn(self, data, dt);
    }

    /// Stabiler Hash des Zustands (für Determinismus-Tests und Desync-Erkennung).
    pub fn state_hash(&self) -> u32 {
        let mut h = crc32fast::Hasher::new();
        for (id, u) in self.units.iter() {
            h.update(&id.index.to_le_bytes());
            h.update(&u.pos.x.to_bits().to_le_bytes());
            h.update(&u.pos.y.to_bits().to_le_bytes());
            h.update(&u.hp.to_bits().to_le_bytes());
        }
        for (_, b) in self.buildings.iter() {
            h.update(&b.hp.to_bits().to_le_bytes());
            h.update(&b.progress.to_bits().to_le_bytes());
        }
        for p in &self.players {
            for k in ResourceKind::ALL {
                h.update(&p.resources.get(k).to_bits().to_le_bytes());
            }
        }
        h.update(&self.tick.to_le_bytes());
        h.finalize()
    }
}
