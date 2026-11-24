//! Weltdarstellung: übersetzt den Simulationszustand in Sprites, Lichter, Partikel und Atmosphäre.

use crate::art::ArtLibrary;
use ashen_core::iso::{depth_of, tile_to_world, world_to_tile, TILE_H};
use ashen_core::noise::{fbm2, rand2};
use ashen_engine::render::particles::{Particle, ParticleSystem};
use ashen_engine::render::{Atmosphere, FrameData, Light, Material, SpriteInstance, SpriteRect, ViewParams};
use ashen_sim::data::{Fort, GameData, ProjectileKind, ResourceKind, UnitClass};
use ashen_sim::entity::{Activity, Building, BuildingId, NodeKind, ProdStatus, UnitId, ENEMY, PLAYER};
use ashen_sim::env::{DayPhase, Weather};
use ashen_sim::map::{DecorKind, Terrain, TileMap};
use ashen_sim::mapgen::ENEMY_BASE;
use ashen_sim::{SimEvent, SimWorld};
use glam::{IVec2, Vec2, Vec3, Vec4};
use std::collections::{HashMap, HashSet};

/// Weltpixel-y → Tiefenschlüssel (entspricht Kachel x + y).
pub fn depth_from_world_y(y: f32) -> f32 {
    y / (TILE_H * 0.5)
}

struct Corpse {
    pos: Vec2,
    kind: String,
    hollow: bool,
    facing: f32,
    age: f32,
}

struct Flash {
    pos: Vec2,
    color: Vec3,
    radius: f32,
    life: f32,
    max: f32,
}

/// Auswahl- und Platzierungszustand, den die Darstellung kennen muss.
#[derive(Default)]
pub struct Overlay {
    pub selected_units: HashSet<UnitId>,
    pub selected_building: Option<BuildingId>,
    pub hovered: Option<UnitId>,
    /// Platzierungsvorschau: Gebäude-ID, Kachel, gültig.
    pub ghost: Option<(String, IVec2, (u8, u8), bool)>,
    /// Vorschau einer Mauerlinie: Kachel und Gültigkeit je Bauteil.
    pub ghost_line: Vec<(IVec2, bool)>,
    pub show_all_bars: bool,
}

/// Kacheln fertiger Befestigungen (Mauern, Tore, Türme) für Anschlüsse und Torachsen.
fn fort_tiles(world: &SimWorld, data: &GameData) -> HashSet<IVec2> {
    let mut set = HashSet::new();
    for (_, b) in world.buildings.iter() {
        if b.complete && data.building(b.kind).is_fort() {
            for y in 0..b.size.y {
                for x in 0..b.size.x {
                    set.insert(b.tile + IVec2::new(x, y));
                }
            }
        }
    }
    set
}

/// Anschlussmaske einer Mauerkachel: Bit 0 = +x, 1 = −x, 2 = +z (Kachel-y), 3 = −z.
fn wall_mask(forts: &HashSet<IVec2>, t: IVec2) -> u32 {
    let mut m = 0;
    for (i, d) in [IVec2::new(1, 0), IVec2::new(-1, 0), IVec2::new(0, 1), IVec2::new(0, -1)].into_iter().enumerate() {
        if forts.contains(&(t + d)) {
            m |= 1 << i;
        }
    }
    m
}

/// Torachse: `true`, wenn die Mauer in x-Richtung verläuft.
fn gate_along_x(forts: &HashSet<IVec2>, t: IVec2) -> bool {
    let x = forts.contains(&(t + IVec2::new(1, 0))) || forts.contains(&(t + IVec2::new(-1, 0)));
    let z = forts.contains(&(t + IVec2::new(0, 1))) || forts.contains(&(t + IVec2::new(0, -1)));
    x || !z
}

/// Spritename (ohne `bld_`-Präfix) eines Gebäudes: Mauern und Tore hängen von der Nachbarschaft ab,
/// der Bergfried von der Ausbaustufe.
fn sprite_id(world: &SimWorld, data: &GameData, b: &Building, forts: &HashSet<IVec2>) -> String {
    let def = data.building(b.kind);
    match def.fort {
        Some(Fort::Wall) => format!("wall_m{}", wall_mask(forts, b.tile)),
        Some(Fort::Gate) => format!("gate_{}_{}", if gate_along_x(forts, b.tile) { "x" } else { "z" }, if b.gate_open { "open" } else { "closed" }),
        _ if def.id == "keep" && world.players[b.owner as usize].era >= 1 => "keep_castle".into(),
        _ => def.id.clone(),
    }
}

/// Vorab aufgelöste Atlas-Rechtecke für Bodenkacheln, Übergänge, Schaum und Dekoration. Spart je Kachel und Frame
/// den Bau eines Namens (String-Allokation) und die Hash-Suche im Atlas.
struct GroundRects {
    /// [Terrain::priority()][Variante]
    tiles: [[SpriteRect; 4]; 10],
    /// [Terrain::priority()][Seite]
    edges: [[Option<SpriteRect>; 4]; 10],
    foam: [[Option<SpriteRect>; 4]; 10],
    /// [DecorKind als Index][Variante 0..3]
    decor: Vec<[Option<SpriteRect>; 3]>,
    bridge: SpriteRect,
}

fn decor_sprite_name(kind: DecorKind, v: u32) -> String {
    match kind {
        DecorKind::Rock => format!("decor_rock_{v}"),
        DecorKind::Boulder => format!("decor_boulder_{v}"),
        DecorKind::Bush => format!("decor_bush_{v}"),
        DecorKind::GrassTuft => format!("decor_tuft_{v}"),
        DecorKind::Flowers => format!("decor_flowers_{v}"),
        DecorKind::DeadTree => format!("tree_dead_{v}"),
        DecorKind::Ruin => "decor_ruin".into(),
        DecorKind::Pillar => "decor_pillar".into(),
        DecorKind::Grave => format!("decor_grave_{v}"),
        DecorKind::Bones => format!("decor_bones_{v}"),
        DecorKind::Brazier => "decor_brazier".into(),
        DecorKind::Fence => "decor_fence".into(),
        DecorKind::Crops => format!("decor_flowers_{v}"),
        DecorKind::Reeds => format!("decor_reeds_{v}"),
        DecorKind::Lantern => "decor_lantern".into(),
        DecorKind::Pebbles => format!("decor_pebbles_{v}"),
        DecorKind::Embers => format!("decor_embers_{v}"),
        DecorKind::Puddle => format!("decor_puddle_{v}"),
    }
}

impl GroundRects {
    fn new(art: &ArtLibrary) -> GroundRects {
        use crate::art::terrain::{edge_name, foam_name, tile_name};
        let mut g = GroundRects { tiles: [[SpriteRect::default(); 4]; 10], edges: [[None; 4]; 10], foam: [[None; 4]; 10], decor: vec![[None; 3]; DecorKind::ALL.len()], bridge: art.get("bridge") };
        for t in Terrain::ALL {
            let pi = t.priority() as usize;
            for v in 0..4 {
                g.tiles[pi][v] = art.get(&tile_name(t, v as u32));
            }
            for side in 0..4u8 {
                g.edges[pi][side as usize] = art.atlas.try_get(&edge_name(t, side));
                g.foam[pi][side as usize] = art.atlas.try_get(&foam_name(t, side));
            }
        }
        for kind in DecorKind::ALL {
            for v in 0..3u32 {
                g.decor[kind as usize][v as usize] = art.atlas.try_get(&decor_sprite_name(kind, v));
            }
        }
        g
    }
}

pub struct WorldView {
    pub particles: ParticleSystem,
    corpses: Vec<Corpse>,
    stumps: Vec<Vec2>,
    markers: Vec<(Vec2, bool, f32)>,
    flashes: Vec<Flash>,
    hit_flash: HashMap<UnitId, f32>,
    pub time: f32,
    lightning: f32,
    emit_acc: f32,
    tint: Vec<f32>,
    /// Wassertiefe je Kachelecke (4 × 4 Bit, siehe `SpriteInstance::water_corners`).
    water_corners: Vec<u16>,
    bridge: Vec<bool>,
    map_size: (i32, i32),
    ground_rects: Option<GroundRects>,
    /// Wiederverwendete Puffer für Übergänge und Schaum (vermeiden Allokationen je Frame).
    edge_buckets: Vec<Vec<SpriteInstance>>,
    foam_buf: Vec<SpriteInstance>,
}

impl WorldView {
    pub fn new(max_particles: usize) -> WorldView {
        WorldView {
            particles: ParticleSystem::new(max_particles),
            corpses: Vec::new(),
            stumps: Vec::new(),
            markers: Vec::new(),
            flashes: Vec::new(),
            hit_flash: HashMap::new(),
            time: 0.0,
            lightning: 0.0,
            emit_acc: 0.0,
            tint: Vec::new(),
            water_corners: Vec::new(),
            bridge: Vec::new(),
            map_size: (0, 0),
            ground_rects: None,
            edge_buckets: Vec::new(),
            foam_buf: Vec::new(),
        }
    }

    /// Verwirft die vorberechneten Karteneigenschaften (der Karteneditor ändert das Gelände zur Laufzeit).
    pub fn invalidate_map(&mut self) {
        self.map_size = (0, 0);
    }

    /// Statische Karteneigenschaften vorberechnen (Farbvariation, Wassertiefe, Brücken).
    fn prepare(&mut self, map: &TileMap) {
        if self.map_size == (map.width, map.height) && !self.tint.is_empty() {
            return;
        }
        self.map_size = (map.width, map.height);
        let n = (map.width * map.height) as usize;
        self.tint = vec![1.0; n];
        self.water_corners = vec![0; n];
        self.bridge = vec![false; n];
        for y in 0..map.height {
            for x in 0..map.width {
                let i = (y * map.width + x) as usize;
                let f = fbm2(x as f32 * 0.07, y as f32 * 0.07, 77, 3);
                self.tint[i] = 0.82 + f * 0.36;
                if map.terrain[i] == Terrain::Road {
                    let near_water = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y].iter().any(|d| map.terrain_at(IVec2::new(x, y) + *d).is_water());
                    self.bridge[i] = near_water;
                }
            }
        }
        // Distanzfeld zum Ufer auf den Kachelecken (Ecke (vx, vy) grenzt an die Kacheln vx-1..vx, vy-1..vy).
        // Brücken zählen als Wasser, Furten und Land als Ufer. 0 = Ufer, 15 = 5 Kacheln und mehr; Tiefsee zusätzlich abgedunkelt.
        const R: i32 = 5;
        let is_water = |x: i32, y: i32| -> Option<bool> {
            if !map.in_bounds(IVec2::new(x, y)) {
                return None;
            }
            let i = map.idx(IVec2::new(x, y));
            let t = map.terrain[i];
            Some(self.bridge[i] || matches!(t, Terrain::Water | Terrain::DeepWater))
        };
        let (vw, vh) = (map.width + 1, map.height + 1);
        let mut vert = vec![0u8; (vw * vh) as usize];
        for vy in 0..vh {
            for vx in 0..vw {
                let mut dmin = f32::MAX;
                let mut deep = true;
                for ty in vy - R..vy + R {
                    for tx in vx - R..vx + R {
                        // Außerhalb der Karte wie Wasser behandeln (offene See).
                        let wat = is_water(tx, ty).unwrap_or(true);
                        let (dx, dy) = (tx as f32 + 0.5 - vx as f32, ty as f32 + 0.5 - vy as f32);
                        let d2 = dx * dx + dy * dy;
                        if !wat {
                            dmin = dmin.min(d2);
                        }
                        if d2 < 2.1 && map.in_bounds(IVec2::new(tx, ty)) && map.terrain[map.idx(IVec2::new(tx, ty))] != Terrain::DeepWater {
                            deep = false;
                        }
                    }
                }
                let d = if dmin == f32::MAX { 1.0 } else { ((dmin.sqrt() - 0.7) / (R as f32 - 0.7)).clamp(0.0, 1.0) };
                let d = if deep { d.max(0.85) } else { d };
                vert[(vy * vw + vx) as usize] = (d * 15.0).round() as u8;
            }
        }
        for y in 0..map.height {
            for x in 0..map.width {
                let v = |dx: i32, dy: i32| vert[((y + dy) * vw + x + dx) as usize] as u16;
                self.water_corners[(y * map.width + x) as usize] = v(0, 0) | (v(1, 0) << 4) | (v(0, 1) << 8) | (v(1, 1) << 12);
            }
        }
    }

    pub fn add_marker(&mut self, tile_pos: Vec2, attack: bool) {
        self.markers.push((tile_pos, attack, 0.0));
    }

    fn flash(&mut self, tile_pos: Vec2, color: Vec3, radius: f32, life: f32) {
        self.flashes.push(Flash { pos: tile_to_world(tile_pos), color, radius, life, max: life });
    }

    #[allow(clippy::too_many_arguments)]
    fn burst(&mut self, art: &ArtLibrary, tile_pos: Vec2, n: usize, sprite: &str, color: Vec4, speed: f32, z: f32, emissive: f32, seed: u32) {
        let rect = art.get(sprite);
        let base = tile_to_world(tile_pos);
        for i in 0..n {
            let a = rand2(i as i32, seed as i32, 3) * std::f32::consts::TAU;
            let s = speed * (0.4 + rand2(i as i32, seed as i32, 4) * 0.8);
            let mut p = Particle::new(rect, base, z, 0.35 + rand2(i as i32, seed as i32, 5) * 0.5);
            p.vel = Vec2::new(a.cos() * s, a.sin() * s * 0.5);
            p.vz = speed * 0.6 + rand2(i as i32, seed as i32, 6) * speed;
            p.gravity = 60.0;
            p.drag = 2.0;
            p.color = (color, color.with_w(0.0));
            p.emissive = emissive;
            p.material = if emissive > 0.0 { Material::Emissive } else { Material::Lit };
            self.particles.spawn(p);
        }
    }

    /// Verarbeitet Simulationsereignisse für Effekte.
    pub fn on_event(&mut self, e: &SimEvent, world: &SimWorld, data: &GameData, art: &ArtLibrary) {
        let seed = (self.time * 1000.0) as u32;
        match e {
            SimEvent::UnitDied { kind, pos, facing, owner, .. } => {
                let def = data.unit(*kind);
                let hollow = *owner == ENEMY;
                self.corpses.push(Corpse { pos: *pos, kind: def.id.clone(), hollow, facing: *facing, age: 0.0 });
                if hollow {
                    self.burst(art, *pos, 10, "fx_glow", Vec4::new(0.5, 1.0, 0.6, 0.9), 22.0, 6.0, 1.0, seed);
                } else {
                    self.burst(art, *pos, 6, "fx_dust", Vec4::new(0.45, 0.38, 0.3, 0.8), 14.0, 2.0, 0.0, seed);
                }
            }
            SimEvent::Hit { target, pos, damage_type, blocked, .. } => {
                if let ashen_sim::entity::EntityRef::Unit(u) = target {
                    self.hit_flash.insert(*u, 0.12);
                }
                let col = if *blocked {
                    Vec4::new(1.0, 0.7, 0.3, 1.0)
                } else if matches!(damage_type, ashen_sim::data::DamageType::Magic) {
                    Vec4::new(0.6, 1.0, 0.7, 1.0)
                } else if matches!(damage_type, ashen_sim::data::DamageType::Fire) {
                    Vec4::new(1.0, 0.55, 0.2, 1.0)
                } else {
                    Vec4::new(1.0, 0.9, 0.7, 1.0)
                };
                self.burst(art, *pos, 3, "fx_dot", col, 26.0, 8.0, 0.8, seed);
            }
            SimEvent::Splash { pos, radius, kind } => {
                let px = radius * 22.0;
                match kind {
                    ProjectileKind::Boulder => {
                        self.burst(art, *pos, 18, "fx_dust", Vec4::new(0.5, 0.44, 0.36, 0.9), px, 6.0, 0.0, seed);
                        self.burst(art, *pos, 10, "fx_dot", Vec4::new(0.6, 0.56, 0.5, 1.0), 38.0, 16.0, 0.0, seed + 3);
                        self.flash(*pos, Vec3::new(1.0, 0.7, 0.4), 70.0, 0.3);
                    }
                    _ => {
                        self.burst(art, *pos, 22, "fx_glow", Vec4::new(1.0, 0.55, 0.18, 1.0), px * 1.2, 8.0, 1.0, seed);
                        self.burst(art, *pos, 10, "fx_smoke", Vec4::new(0.3, 0.26, 0.24, 0.7), 14.0, 6.0, 0.0, seed + 5);
                        self.flash(*pos, Vec3::new(1.0, 0.55, 0.2), 100.0, 0.5);
                    }
                }
            }
            SimEvent::Healed { pos, amount } => {
                if *amount > 0.0 {
                    self.burst(art, *pos, 18, "fx_glow", Vec4::new(0.55, 1.0, 0.6, 1.0), 26.0, 12.0, 1.0, seed);
                    self.burst(art, *pos, 6, "fx_dot", Vec4::new(1.0, 0.95, 0.6, 1.0), 20.0, 18.0, 1.0, seed + 1);
                    self.flash(*pos, Vec3::new(0.5, 1.0, 0.6), 80.0, 0.5);
                }
            }
            SimEvent::Summoned { pos, .. } => {
                self.burst(art, *pos, 14, "fx_glow", Vec4::new(0.45, 1.0, 0.6, 1.0), 24.0, 8.0, 1.0, seed);
                self.burst(art, *pos, 8, "fx_dust", Vec4::new(0.35, 0.34, 0.3, 0.8), 16.0, 3.0, 0.0, seed + 2);
            }
            SimEvent::TalentLearned { pos, .. } | SimEvent::ItemUsed { pos, .. } | SimEvent::ItemPickedUp { pos, .. } => {
                self.burst(art, *pos, 16, "fx_glow", Vec4::new(1.0, 0.88, 0.45, 1.0), 26.0, 18.0, 1.0, seed);
                self.flash(*pos, Vec3::new(1.0, 0.85, 0.5), 60.0, 0.6);
            }
            SimEvent::ItemDropped { pos, .. } => {
                self.burst(art, *pos, 10, "fx_dot", Vec4::new(1.0, 0.9, 0.5, 1.0), 28.0, 14.0, 1.0, seed);
            }
            SimEvent::ProjectileImpact { kind: ProjectileKind::Bolt, pos, hit: true } => {
                self.burst(art, *pos, 3, "fx_dot", Vec4::new(0.85, 0.85, 0.9, 1.0), 24.0, 8.0, 0.4, seed);
            }
            SimEvent::ProjectileImpact { kind: ProjectileKind::SoulBolt, pos, .. } => {
                self.burst(art, *pos, 8, "fx_glow", Vec4::new(0.5, 1.0, 0.6, 1.0), 20.0, 6.0, 1.0, seed);
                self.flash(*pos, Vec3::new(0.4, 1.0, 0.55), 40.0, 0.3);
            }
            SimEvent::WorkStroke { resource, pos, .. } => {
                let (sprite, col) = match resource {
                    Some(ResourceKind::Wood) => ("fx_dot", Vec4::new(0.55, 0.4, 0.25, 1.0)),
                    Some(ResourceKind::Gold) => ("fx_dot", Vec4::new(1.0, 0.85, 0.35, 1.0)),
                    Some(ResourceKind::Stone) => ("fx_dot", Vec4::new(0.6, 0.6, 0.62, 1.0)),
                    _ => ("fx_dust", Vec4::new(0.5, 0.42, 0.32, 0.6)),
                };
                self.burst(art, *pos + Vec2::new(0.3, 0.0), 3, sprite, col, 16.0, 6.0, if *resource == Some(ResourceKind::Gold) { 0.6 } else { 0.0 }, seed);
            }
            SimEvent::NodeDepleted { pos, tree } => {
                if *tree {
                    self.stumps.push(*pos);
                    self.burst(art, *pos, 12, "fx_dot", Vec4::new(0.2, 0.3, 0.15, 1.0), 20.0, 16.0, 0.0, seed);
                }
            }
            SimEvent::BuildingDestroyed { pos, .. } => {
                self.burst(art, *pos, 24, "fx_smoke_big", Vec4::new(0.3, 0.28, 0.26, 0.8), 18.0, 10.0, 0.0, seed);
                self.burst(art, *pos, 16, "fx_dot", Vec4::new(1.0, 0.6, 0.2, 1.0), 30.0, 12.0, 1.0, seed + 1);
                self.flash(*pos, Vec3::new(1.0, 0.5, 0.2), 120.0, 0.8);
            }
            SimEvent::BuildingCompleted { pos, .. } => {
                self.burst(art, *pos, 14, "fx_dust", Vec4::new(0.5, 0.45, 0.38, 0.7), 22.0, 2.0, 0.0, seed);
            }
            SimEvent::AbilityCast { ability, pos, .. } => match ability.as_str() {
                "ember_cleave" => {
                    for i in 0..36 {
                        let a = i as f32 / 36.0 * std::f32::consts::TAU;
                        let p = *pos + Vec2::new(a.cos(), a.sin()) * 2.2;
                        self.burst(art, p, 1, "fx_glow", Vec4::new(1.0, 0.6, 0.2, 1.0), 10.0, 4.0, 1.0, seed + i);
                    }
                    self.burst(art, *pos, 20, "fx_dot", Vec4::new(1.0, 0.8, 0.4, 1.0), 50.0, 6.0, 1.0, seed);
                    self.flash(*pos, Vec3::new(1.0, 0.55, 0.2), 110.0, 0.6);
                }
                "rallying_banner" | "blessing" | "beacon_of_faith" | "war_cry" | "war_horns" | "clan_roar" | "shield_wall" => {
                    self.burst(art, *pos, 30, "fx_glow", Vec4::new(1.0, 0.85, 0.4, 1.0), 40.0, 10.0, 1.0, seed);
                    self.flash(*pos, Vec3::new(1.0, 0.8, 0.4), 140.0, 1.0);
                }
                "fireball" | "flame_nova" | "sacred_flame" | "phoenix_cut" | "pillar_of_fire" => {
                    for i in 0..24 {
                        let a = i as f32 / 24.0 * std::f32::consts::TAU;
                        let p = *pos + Vec2::new(a.cos(), a.sin()) * 1.6;
                        self.burst(art, p, 1, "fx_glow", Vec4::new(1.0, 0.5, 0.15, 1.0), 12.0, 4.0, 1.0, seed + i);
                    }
                    self.burst(art, *pos, 14, "fx_dot", Vec4::new(1.0, 0.7, 0.25, 1.0), 40.0, 8.0, 1.0, seed);
                    self.flash(*pos, Vec3::new(1.0, 0.5, 0.2), 120.0, 0.6);
                }
                "holy_heal" | "flame_mend" | "resurgence" => {
                    self.burst(art, *pos, 24, "fx_glow", Vec4::new(0.55, 1.0, 0.65, 1.0), 30.0, 14.0, 1.0, seed);
                    self.flash(*pos, Vec3::new(0.5, 1.0, 0.6), 100.0, 0.7);
                }
                "ground_slam" | "ancestral_fury" => {
                    self.burst(art, *pos, 20, "fx_dust", Vec4::new(0.5, 0.44, 0.36, 0.9), 34.0, 4.0, 0.0, seed);
                    self.flash(*pos, Vec3::new(1.0, 0.8, 0.5), 90.0, 0.4);
                }
                "ashen_ward" => {
                    self.burst(art, *pos, 18, "fx_smoke", Vec4::new(0.5, 0.45, 0.42, 0.7), 18.0, 8.0, 0.0, seed);
                    self.flash(*pos, Vec3::new(1.0, 0.5, 0.25), 60.0, 0.5);
                }
                _ => {
                    self.burst(art, *pos, 24, "fx_glow", Vec4::new(0.45, 1.0, 0.6, 1.0), 36.0, 10.0, 1.0, seed);
                    self.flash(*pos, Vec3::new(0.4, 1.0, 0.55), 130.0, 0.8);
                }
            },
            SimEvent::LevelUp { unit, .. } => {
                if let Some(u) = world.units.get(*unit) {
                    self.burst(art, u.pos, 26, "fx_glow", Vec4::new(1.0, 0.9, 0.5, 1.0), 30.0, 20.0, 1.0, seed);
                    self.flash(u.pos, Vec3::new(1.0, 0.85, 0.5), 80.0, 1.0);
                }
            }
            SimEvent::ProductionCycle { kind, pos, .. } => {
                let id = data.building(*kind).id.as_str();
                match id {
                    "smithy" | "smelter" | "armory" | "brickworks" => {
                        self.burst(art, *pos + Vec2::new(0.4, 0.2), 6, "fx_dot", Vec4::new(1.0, 0.7, 0.25, 1.0), 30.0, 10.0, 1.0, seed);
                        self.flash(*pos, Vec3::new(1.0, 0.55, 0.2), 60.0, 0.35);
                    }
                    "sawmill" | "woodcutter" | "windmill" | "bakery" => self.burst(art, *pos, 5, "fx_dust", Vec4::new(0.7, 0.58, 0.4, 0.7), 14.0, 4.0, 0.0, seed),
                    _ => self.burst(art, *pos, 3, "fx_dust", Vec4::new(0.5, 0.45, 0.38, 0.6), 10.0, 3.0, 0.0, seed),
                }
            }
            SimEvent::GateToggled { pos, .. } => self.burst(art, *pos, 6, "fx_dust", Vec4::new(0.5, 0.45, 0.38, 0.7), 12.0, 3.0, 0.0, seed),
            SimEvent::HouseChanged { pos, up, .. } => {
                if *up {
                    self.burst(art, *pos, 14, "fx_glow", Vec4::new(1.0, 0.85, 0.4, 1.0), 26.0, 14.0, 1.0, seed);
                    self.flash(*pos, Vec3::new(1.0, 0.8, 0.4), 70.0, 0.7);
                } else {
                    self.burst(art, *pos, 8, "fx_smoke", Vec4::new(0.35, 0.32, 0.3, 0.7), 12.0, 6.0, 0.0, seed);
                }
            }
            SimEvent::EraAdvanced { .. } => {
                if let Some(k) = world.building_with_tag(&world.mission_def.defeat_tag).and_then(|b| world.buildings.get(b)) {
                    let p = k.center();
                    self.burst(art, p, 40, "fx_glow", Vec4::new(1.0, 0.8, 0.35, 1.0), 40.0, 20.0, 1.0, seed);
                    self.flash(p, Vec3::new(1.0, 0.75, 0.35), 160.0, 1.4);
                }
            }
            SimEvent::Lightning => self.lightning = 1.0,
            SimEvent::HeroRespawned { pos, .. } => self.flash(*pos, Vec3::new(1.0, 0.7, 0.3), 90.0, 1.2),
            _ => {}
        }
    }

    fn spawn_smoke(&mut self, art: &ArtLibrary, world_px: Vec2, dark: bool, i: u32) {
        let rect = art.get(if dark { "fx_smoke_big" } else { "fx_smoke" });
        let mut p = Particle::new(rect, world_px + Vec2::new(0.0, 0.0), 0.0, 3.0 + rand2(i as i32, 1, 9) * 1.5);
        p.vel = Vec2::new(4.0 + rand2(i as i32, 2, 9) * 4.0, -1.0);
        p.vz = 9.0 + rand2(i as i32, 3, 9) * 4.0;
        p.drag = 0.2;
        p.size = (0.6, 1.8);
        let c = if dark { 0.12 } else { 0.35 };
        p.color = (Vec4::new(c, c, c * 1.02, if dark { 0.75 } else { 0.5 }), Vec4::new(c, c, c, 0.0));
        p.depth_bias = 6.0;
        self.particles.spawn(p);
    }

    fn spawn_ember(&mut self, art: &ArtLibrary, world_px: Vec2, color: Vec3, i: u32) {
        let mut p = Particle::new(art.get("fx_dot"), world_px, 0.0, 0.8 + rand2(i as i32, 4, 3) * 0.8);
        p.vel = Vec2::new((rand2(i as i32, 5, 3) - 0.5) * 10.0, 0.0);
        p.vz = 14.0 + rand2(i as i32, 6, 3) * 10.0;
        p.color = (color.extend(1.0), color.extend(0.0));
        p.emissive = 1.0;
        p.material = Material::Emissive;
        p.depth_bias = 4.0;
        self.particles.spawn(p);
    }

    /// Aktualisiert Zeit, Partikel und Leichen.
    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        self.particles.update(dt);
        for c in &mut self.corpses {
            c.age += dt;
        }
        self.corpses.retain(|c| c.age < 22.0);
        for m in &mut self.markers {
            m.2 += dt;
        }
        self.markers.retain(|m| m.2 < 0.7);
        for f in &mut self.flashes {
            f.life -= dt;
        }
        self.flashes.retain(|f| f.life > 0.0);
        for v in self.hit_flash.values_mut() {
            *v -= dt;
        }
        self.hit_flash.retain(|_, v| *v > 0.0);
        self.lightning = (self.lightning - dt * 3.0).max(0.0);
        self.emit_acc += dt;
        if self.stumps.len() > 400 {
            self.stumps.drain(0..100);
        }
    }

    /// Atmosphäre aus Tageszeit, Wetter und Nähe zur Verderbnis.
    pub fn atmosphere(&self, world: &SimWorld, camera_world: Vec2) -> (Atmosphere, f32, f32) {
        let env = &world.env;
        let t = env.time_of_day;
        let e = env.sun_elevation();
        let day = smooth(-0.15, 0.35, e);
        let low_sun = 1.0 - smooth(0.0, 0.5, e.abs());
        let morning = t < 0.5;
        let mut a = Atmosphere::default();
        let night_amb = Vec3::new(0.30, 0.38, 0.76);
        let day_amb = Vec3::new(0.62, 0.62, 0.66);
        let warm = if morning { Vec3::new(0.72, 0.52, 0.55) } else { Vec3::new(0.66, 0.42, 0.52) };
        a.ambient = night_amb.lerp(day_amb, day).lerp(warm, low_sun * day.max(0.3) * 0.6);
        a.ambient_intensity = 1.0 * (1.0 - day) + 0.6 * day;
        let sun_day = Vec3::new(1.0, 0.93, 0.82);
        let sun_low = if morning { Vec3::new(1.0, 0.62, 0.34) } else { Vec3::new(1.0, 0.45, 0.28) };
        let moon = Vec3::new(0.55, 0.66, 0.95);
        a.sun_color = if e > 0.0 { sun_day.lerp(sun_low, low_sun) } else { moon };
        a.sun_intensity = if e > 0.0 { 0.35 + day * 0.65 } else { 0.32 };
        let sx = if e > 0.0 { -((t - 0.25) * std::f32::consts::TAU).cos() * 0.85 } else { 0.55 };
        a.sun_dir = Vec3::new(sx, -0.55, e.abs().max(0.15) * 0.8 + 0.2).normalize();
        a.shadow_strength = if e > 0.0 { 0.55 + day * 0.35 } else { 0.35 };
        // Wetter.
        let wi = env.weather_intensity;
        let mut fog = 0.0;
        if env.phase() == DayPhase::Dawn {
            fog += 0.25;
            a.ground_fog = 0.2;
        }
        match env.weather {
            Weather::Clear => {}
            Weather::Overcast => {
                a.sun_intensity *= 1.0 - 0.45 * wi;
                a.ambient_intensity *= 1.0 + 0.25 * wi;
                a.shadow_strength *= 1.0 - 0.5 * wi;
            }
            Weather::Fog => {
                fog += 0.9 * wi;
                a.sun_intensity *= 1.0 - 0.4 * wi;
                a.shadow_strength *= 0.5;
            }
            Weather::Rain | Weather::Storm => {
                a.rain = wi * if env.weather == Weather::Storm { 1.0 } else { 0.7 };
                a.wetness = wi;
                fog += 0.3 * wi;
                a.sun_intensity *= 1.0 - 0.55 * wi;
                a.shadow_strength *= 0.45;
            }
            Weather::Ashfall => {
                a.ash = wi;
                fog += 0.35 * wi;
                a.fog_color = Vec3::new(0.45, 0.42, 0.40);
                a.sun_intensity *= 1.0 - 0.3 * wi;
            }
        }
        a.fog_density = fog.min(1.0);
        a.fog_color = a.fog_color.lerp(a.ambient, 0.4);
        a.lightning = self.lightning;
        // Verderbnis nahe dem Hohlschrein: entsättigt, kühl-grün getönt.
        let cam_tile = world_to_tile(camera_world);
        let hollow = 1.0 - smooth(8.0, 26.0, (cam_tile - ENEMY_BASE).length());
        a.saturation = 1.02 - hollow * 0.35;
        a.tint = Vec3::new(0.05, 0.02, 0.10).lerp(Vec3::new(-0.02, 0.07, 0.03), hollow);
        a.contrast = 1.08 + hollow * 0.06;
        // Schattenwurf (Scherung, Stauchung) passend zur Lichtrichtung.
        let shear = (-a.sun_dir.x * 1.1 / a.sun_dir.z.max(0.25)).clamp(-2.2, 2.2);
        let squash = 0.55 + 0.2 * (1.0 - low_sun);
        (a, shear, squash)
    }

    /// Schreibt den gesamten Weltzustand in den Frame.
    #[allow(clippy::too_many_arguments)]
    pub fn render(&mut self, world: &SimWorld, data: &GameData, art: &ArtLibrary, frame: &mut FrameData, view: &ViewParams, alpha: f32, overlay: &Overlay, fog: bool) {
        self.prepare(&world.map);
        let map = &world.map;
        let (vmin, vmax) = view.visible_world(48.0);
        let corners = [vmin, Vec2::new(vmax.x, vmin.y), Vec2::new(vmin.x, vmax.y), vmax].map(world_to_tile);
        let tmin = corners.iter().fold(Vec2::splat(f32::MAX), |a, b| a.min(*b)).floor().as_ivec2().max(IVec2::ZERO);
        let tmax = corners.iter().fold(Vec2::splat(f32::MIN), |a, b| a.max(*b)).ceil().as_ivec2().min(IVec2::new(map.width - 1, map.height - 1));
        let in_view = |p: Vec2| p.x >= tmin.x as f32 - 2.0 && p.y >= tmin.y as f32 - 2.0 && p.x <= tmax.x as f32 + 3.0 && p.y <= tmax.y as f32 + 3.0;
        let visible = |p: Vec2| !fog || world.fog.visible(p);
        let explored = |p: Vec2| !fog || world.fog.explored(p);
        let (atmo, shear, squash) = self.atmosphere(world, view.origin + Vec2::new(view.internal.0 as f32, view.internal.1 as f32) * 0.5);
        frame.atmosphere = atmo;
        let night = 1.0 - smooth(-0.1, 0.3, world.env.sun_elevation());
        let wind = (self.time * 1.3).sin() * 0.03;

        // --- Boden
        let gr = self.ground_rects.take().unwrap_or_else(|| GroundRects::new(art));
        let mut edges = std::mem::take(&mut self.edge_buckets);
        edges.resize_with(10, Vec::new);
        let mut foam = std::mem::take(&mut self.foam_buf);
        for ty in tmin.y..=tmax.y {
            for tx in tmin.x..=tmax.x {
                let t = IVec2::new(tx, ty);
                let i = map.idx(t);
                let terr = map.terrain[i];
                let wp = tile_to_world(Vec2::new(tx as f32, ty as f32));
                let tint = self.tint[i];
                let variant = (rand2(tx, ty, 11) * 4.0) as u32;
                let bridge = self.bridge[i];
                let base_terr = if bridge { Terrain::Water } else { terr };
                let rect = gr.tiles[base_terr.priority() as usize][variant as usize % 4];
                let mut inst = SpriteInstance::new(&rect, wp + Vec2::new(0.0, 8.0)).tint(Vec4::new(tint, tint, tint * 0.98, 1.0));
                if base_terr.is_water() && base_terr != Terrain::Ford {
                    inst = inst.material(Material::Water).water_corners(self.water_corners[i]);
                } else if base_terr == Terrain::Ford {
                    inst = inst.material(Material::Water).water_depth(-1.0);
                }
                frame.ground.push(inst);
                if bridge {
                    frame.ground.push(SpriteInstance::new(&gr.bridge, wp + Vec2::new(0.0, 8.0)));
                }
                // Übergänge: höher priorisierte Nachbarn ragen herein.
                for (side, d) in [(0u8, IVec2::new(0, -1)), (1, IVec2::new(1, 0)), (2, IVec2::new(0, 1)), (3, IVec2::new(-1, 0))] {
                    let nt = map.terrain_at(t + d);
                    let n_terr = if map.in_bounds(t + d) && self.bridge[map.idx(t + d)] { Terrain::Water } else { nt };
                    if n_terr.priority() > base_terr.priority() && (n_terr.priority() >= Terrain::Sand.priority() || n_terr == Terrain::Ford) && map.in_bounds(t + d) {
                        let Some(er) = gr.edges[n_terr.priority() as usize][side as usize] else { continue };
                        let ni = map.idx(t + d);
                        if base_terr.is_water() && base_terr != Terrain::Ford && !bridge
                            && let Some(fr) = gr.foam[n_terr.priority() as usize][side as usize]
                        {
                            // Brandungssaum: Schaum atmet mit der Welle (Alpha-Dithering je Kachel versetzt).
                            let a = 0.62 + 0.38 * (self.time * 1.1 + tx as f32 * 0.7 + ty as f32 * 1.3).sin();
                            foam.push(SpriteInstance::new(&fr, wp + Vec2::new(0.0, 8.0)).alpha(a));
                        }
                        let nt_tint = self.tint[ni];
                        let mut ei = SpriteInstance::new(&er, wp + Vec2::new(0.0, 8.0)).tint(Vec4::new(nt_tint, nt_tint, nt_tint * 0.98, 1.0));
                        if n_terr == Terrain::Ford {
                            ei = ei.material(Material::Water).water_depth(-1.0);
                        }
                        edges[n_terr.priority() as usize].push(ei);
                    }
                }
            }
        }
        frame.ground.append(&mut foam);
        for bucket in &mut edges {
            frame.ground.append(bucket);
        }
        self.edge_buckets = edges;
        self.foam_buf = foam;
        for s in &self.stumps {
            if in_view(*s) {
                let r = art.get("decor_stump");
                frame.world.push((depth_of(*s), SpriteInstance::new(&r, tile_to_world(*s))));
            }
        }

        // --- Dekoration
        for d in &map.decor {
            if !in_view(d.pos) {
                continue;
            }
            let wp = tile_to_world(d.pos);
            let v = d.variant as usize % 3;
            let Some(rect) = gr.decor[d.kind as usize][v] else { continue };
            let tall = matches!(d.kind, DecorKind::Boulder | DecorKind::DeadTree | DecorKind::Ruin | DecorKind::Pillar | DecorKind::Brazier | DecorKind::Lantern | DecorKind::Grave);
            let mut inst = SpriteInstance::new(&rect, wp);
            if matches!(d.kind, DecorKind::GrassTuft | DecorKind::Reeds | DecorKind::Bush) {
                inst = inst.shear(wind * if d.kind == DecorKind::Reeds { 3.0 } else { 1.5 });
            }
            if d.kind == DecorKind::Embers {
                // Glut pulsiert träge, je Häufchen phasenverschoben.
                inst = inst.emissive(0.25 + 0.3 * (self.time * 1.7 + d.pos.x * 2.3 + d.pos.y * 1.1).sin().abs());
            }
            frame.world.push((depth_of(d.pos), inst));
            if tall {
                frame.shadows.push(SpriteInstance::new(&rect, wp).material(Material::Shadow).shear(shear).squash(squash));
            }
            if matches!(d.kind, DecorKind::Brazier | DecorKind::Lantern) && explored(d.pos) {
                let (h, col, r) = if d.kind == DecorKind::Brazier { (10.0, Vec3::new(1.0, 0.55, 0.22), 80.0) } else { (15.0, Vec3::new(1.0, 0.72, 0.4), 55.0) };
                frame.lights.push(Light::new(wp - Vec2::new(0.0, h), r, col, 0.9 + night * 0.6).flicker(0.25, d.pos.x * 7.0).height(18.0));
                if d.kind == DecorKind::Brazier && rand2((self.time * 10.0) as i32, d.pos.x as i32, 5) > 0.85 {
                    self.spawn_ember(art, wp - Vec2::new(0.0, 10.0), Vec3::new(1.0, 0.6, 0.2), (self.time * 100.0) as u32);
                }
            }
        }

        self.ground_rects = Some(gr);

        // --- Ressourcenquellen
        for (_, n) in world.nodes.iter() {
            let c = n.center();
            if !in_view(c) || !explored(c) {
                continue;
            }
            match n.kind {
                NodeKind::Tree => {
                    let name = if n.variant >= 200 {
                        format!("tree_dead_{}", n.variant % 3)
                    } else if n.variant % 2 == 0 {
                        format!("tree_pine_{}", (n.variant / 2) % 4)
                    } else {
                        format!("tree_oak_{}", (n.variant / 2) % 4)
                    };
                    let rect = art.get(&name);
                    let wp = tile_to_world(c);
                    let sway = wind + (self.time * 0.9 + c.x * 0.7).sin() * 0.025;
                    let chopped = 1.0 - n.amount as f32 / n.max_amount.max(1) as f32;
                    let tint = 1.0 - chopped * 0.15;
                    frame.world.push((depth_of(c), SpriteInstance::new(&rect, wp).shear(sway).tint(Vec4::new(tint, tint, tint, 1.0))));
                    frame.shadows.push(SpriteInstance::new(&rect, wp).material(Material::Shadow).shear(shear + sway).squash(squash));
                }
                NodeKind::GoldMine | NodeKind::StoneQuarry => {
                    let rect = art.get(if n.kind == NodeKind::GoldMine { "node_gold_mine" } else { "node_quarry" });
                    let wp = tile_to_world(n.tile.as_vec2());
                    frame.world.push((depth_of(c), SpriteInstance::new(&rect, wp)));
                    if n.kind == NodeKind::GoldMine {
                        frame.lights.push(Light::new(wp + Vec2::new(6.0, 4.0), 40.0, Vec3::new(1.0, 0.75, 0.4), 0.6 + night * 0.6).flicker(0.2, 3.0));
                    }
                }
            }
        }

        // --- Gebäude
        let forts = fort_tiles(world, data);
        let mut emit_smoke = Vec::new();
        let mut emit_embers = Vec::new();
        let mut emit_green = Vec::new();
        for (bid, b) in world.buildings.iter() {
            let c = b.center();
            if !in_view(c) || !explored(c) {
                continue;
            }
            let def = data.building(b.kind);
            let wp = tile_to_world(b.tile.as_vec2());
            let sid = sprite_id(world, data, b, &forts);
            let rect = art.get(&format!("bld_{sid}"));
            let depth = depth_of(c);
            let meta = art.buildings.get(&sid).or_else(|| art.buildings.get(&def.id)).cloned().unwrap_or_default();
            // Betriebe leuchten und rauchen nur bei der Arbeit, Wohnhäuser nur mit Bewohnern.
            let active = if !def.recipes.is_empty() {
                b.status == ProdStatus::Running
            } else if def.housing.is_some() {
                b.residents >= 0.5
            } else {
                true
            };
            if b.complete {
                let mut inst = SpriteInstance::new(&rect, wp);
                if b.last_hit_time > world.time - 0.12 {
                    inst = inst.emissive(0.0).tint(Vec4::new(1.25, 1.1, 1.0, 1.0));
                }
                frame.world.push((depth, inst));
                frame.shadows.push(SpriteInstance::new(&rect, wp).material(Material::Shadow).shear(shear).squash(squash));
                if active {
                    for (off, col, r) in &meta.lights {
                        frame.lights.push(Light::new(wp + *off, *r, *col, 0.8 + night * 0.9).flicker(0.2, off.x).height(20.0));
                    }
                    for s in &meta.smoke {
                        emit_smoke.push((wp + *s, false));
                    }
                    for f in &meta.fires {
                        emit_embers.push(wp + *f);
                    }
                }
                for g in &meta.glow_green {
                    emit_green.push(wp + *g);
                }
                // Diagonale Mauerverbindungen zwischen eckig benachbarten Mauerkacheln.
                if def.fort == Some(Fort::Wall) {
                    for (d, name, oy) in [(IVec2::new(1, 1), "wall_d0", 0), (IVec2::new(1, -1), "wall_d1", -1)] {
                        let t = b.tile;
                        if forts.contains(&(t + d)) && !forts.contains(&(t + IVec2::new(d.x, 0))) && !forts.contains(&(t + IVec2::new(0, d.y))) {
                            let origin = t + IVec2::new(0, oy);
                            let r = art.get(&format!("bld_{name}"));
                            let wpos = tile_to_world(origin.as_vec2());
                            frame.world.push((depth_of(origin.as_vec2() + Vec2::new(1.0, 1.0)), SpriteInstance::new(&r, wpos)));
                            frame.shadows.push(SpriteInstance::new(&r, wpos).material(Material::Shadow).shear(shear).squash(squash));
                        }
                    }
                }
                // Statussymbol über stillstehenden Betrieben und unruhigen Häusern.
                if b.owner == PLAYER && (def.employs() || def.housing.is_some()) {
                    let icon = match b.status {
                        ProdStatus::MissingInput => Some("icon_status_input"),
                        ProdStatus::NoWorkers => Some("icon_status_workers"),
                        ProdStatus::StorageFull => Some("icon_status_full"),
                        ProdStatus::NoDepot => Some("icon_status_depot"),
                        ProdStatus::NoSite => Some("icon_status_site"),
                        ProdStatus::Disabled => Some("icon_status_off"),
                        _ if def.housing.is_some() && b.residents >= 1.0 && b.happiness < 20.0 => Some("icon_sad"),
                        _ => None,
                    };
                    if let Some(name) = icon
                        && let Some(r) = art.atlas.try_get(name)
                    {
                        let bob = (self.time * 3.0 + bid.index as f32).sin() * 1.5;
                        let top = tile_to_world(c) - Vec2::new(r.size.x * 0.5, meta.height.max(30.0) * 0.8 + 18.0 + bob);
                        frame.overlay.push(SpriteInstance::new(&r, top).material(Material::Unlit));
                    }
                }
                if b.owner == ENEMY && !meta.glow_green.is_empty() {
                    let pulse = 0.8 + (self.time * 2.0).sin() * 0.2;
                    frame.lights.push(Light::new(wp + meta.glow_green[0], 60.0, Vec3::new(0.4, 1.0, 0.55), pulse).height(30.0));
                }
                // Brände bei Schaden.
                let dmg = 1.0 - b.hp / b.max_hp;
                if dmg > 0.45 {
                    let fires = if dmg > 0.75 { 3 } else { 1 };
                    for k in 0..fires {
                        let fx = (rand2(bid.index as i32, k, 7) - 0.5) * def.size.0 as f32 * 20.0;
                        let fy = rand2(bid.index as i32, k, 8) * meta.height * 0.5;
                        let p = tile_to_world(c) + Vec2::new(fx, -fy);
                        let frame_i = ((self.time * 10.0) as i32 + k) % 4;
                        let fr = art.get(&format!("fx_flame_{frame_i}"));
                        frame.world.push((depth + 0.05, SpriteInstance::new(&fr, p).material(Material::Emissive).emissive(1.0)));
                        frame.lights.push(Light::new(p, 70.0, Vec3::new(1.0, 0.5, 0.2), 1.2).flicker(0.4, k as f32));
                        emit_smoke.push((p - Vec2::new(0.0, 8.0), true));
                        frame.heat.push(SpriteInstance::new(&art.get("fx_heat"), p - Vec2::new(0.0, 10.0)).emissive(0.8));
                    }
                }
            } else {
                let fname = format!("foundation_{}x{}", def.size.0, def.size.1);
                if let Some(fr) = art.atlas.try_get(&fname) {
                    frame.ground.push(SpriteInstance::new(&fr, wp));
                }
                if b.progress > 0.02 {
                    frame.world.push((depth, SpriteInstance::new(&rect, wp).crop_top(b.progress.max(0.15)).tint(Vec4::new(0.85, 0.82, 0.78, 1.0))));
                }
                if let Some(sc) = art.atlas.try_get(&format!("scaffold_{}x{}", def.size.0, def.size.1)) {
                    frame.world.push((depth + 0.02, SpriteInstance::new(&sc, wp)));
                }
            }
            let selected = overlay.selected_building == Some(bid);
            if selected || b.hp < b.max_hp * 0.999 || !b.complete {
                let top = tile_to_world(c) - Vec2::new(0.0, meta.height.max(30.0) * 0.75);
                bar(frame, top, 26.0, b.hp / b.max_hp, b.owner == PLAYER, None);
                if !b.complete {
                    bar(frame, top + Vec2::new(0.0, 4.0), 26.0, b.progress, true, Some(Vec4::new(0.85, 0.7, 0.3, 1.0)));
                }
            }
            if selected {
                let ring = art.get("ring_l");
                frame.ground.push(SpriteInstance::new(&ring, tile_to_world(c)).material(Material::Unlit).tint(Vec4::new(0.5, 1.0, 0.5, 1.0)));
                if let Some(r) = b.rally {
                    let fl = art.get("rally_flag");
                    frame.world.push((depth_of(r), SpriteInstance::new(&fl, tile_to_world(r))));
                }
            }
            if !b.queue.is_empty() && b.owner == PLAYER && b.complete {
                let top = tile_to_world(c) - Vec2::new(0.0, meta.height.max(30.0) * 0.75 - 8.0);
                bar(frame, top, 20.0, b.queue[0].progress, true, Some(Vec4::new(0.4, 0.6, 1.0, 1.0)));
            }
        }

        // Partikel-Emission (gedrosselt).
        if self.emit_acc > 0.12 {
            self.emit_acc = 0.0;
            let k = (self.time * 100.0) as u32;
            for (i, (p, dark)) in emit_smoke.into_iter().enumerate() {
                self.spawn_smoke(art, p, dark, k + i as u32);
            }
            for (i, p) in emit_embers.into_iter().enumerate() {
                if rand2(k as i32, i as i32, 2) > 0.5 {
                    self.spawn_ember(art, p, Vec3::new(1.0, 0.6, 0.2), k + i as u32);
                }
            }
            for (i, p) in emit_green.into_iter().enumerate() {
                if rand2(k as i32, i as i32, 3) > 0.4 {
                    self.spawn_ember(art, p + Vec2::new((rand2(k as i32, i as i32, 4) - 0.5) * 10.0, 0.0), Vec3::new(0.5, 1.0, 0.6), k + i as u32 + 7);
                }
            }
            // Regenspritzer im Sichtbereich.
            if world.env.weather.is_rainy() {
                for i in 0..12 {
                    let px = vmin.x + rand2(k as i32, i, 21) * (vmax.x - vmin.x);
                    let py = vmin.y + rand2(k as i32, i, 22) * (vmax.y - vmin.y);
                    let mut p = Particle::new(art.get("fx_dot"), Vec2::new(px, py), 0.0, 0.25);
                    p.vz = 10.0;
                    p.gravity = 80.0;
                    p.color = (Vec4::new(0.7, 0.75, 0.85, 0.8), Vec4::new(0.7, 0.75, 0.85, 0.0));
                    self.particles.spawn(p);
                }
            }
        }

        // --- Einheiten
        for (id, u) in world.units.iter() {
            let p = u.prev_pos.lerp(u.pos, alpha);
            if !in_view(p) {
                continue;
            }
            if u.owner != PLAYER && !visible(p) {
                continue;
            }
            let def = data.unit(u.kind);
            let (dir, flip) = crate::art3d::dir_for(u.facing);
            let anims = art.unit_anims(&def.id, def.faction == ashen_sim::data::Faction::Hollowed, dir);
            let (clip_name, t) = match u.activity {
                Activity::Walk | Activity::Flee => (if u.carry.is_some() && anims.clips.contains_key("carry") { "carry" } else { "walk" }, self.time + id.index as f32 * 0.13),
                Activity::Attack => ("attack", u.activity_time),
                Activity::Work => ("work", u.activity_time),
                Activity::Cast => (if anims.clips.contains_key("cast") { "cast" } else { "attack" }, u.activity_time),
                Activity::Idle => {
                    if u.is_moving() {
                        ("walk", self.time + id.index as f32 * 0.13)
                    } else {
                        ("idle", self.time * 0.8 + id.index as f32 * 0.37)
                    }
                }
            };
            let clip = anims.get(clip_name).expect("idle exists");
            let clip = if clip.finished(t) && matches!(clip_name, "attack" | "cast") { anims.get("idle").expect("idle") } else { clip };
            let rect = *clip.frame_at(t);
            let wp = tile_to_world(p);
            let mut inst = SpriteInstance::new(&rect, wp).flip(flip);
            if let Some(f) = self.hit_flash.get(&id) {
                inst = inst.tint(Vec4::new(1.6, 1.4, 1.3, 1.0)).emissive(f * 4.0);
            }
            if u.routing > 0.0 {
                inst = inst.tint(Vec4::new(0.8, 0.8, 0.95, 1.0));
            }
            frame.world.push((depth_of(p), inst));
            let shadow = match def.class {
                UnitClass::Cavalry | UnitClass::Boss => "shadow_l",
                UnitClass::Monster if def.radius > 0.5 => "shadow_l",
                _ if def.id == "bat_swarm" => "shadow_l",
                _ => "shadow_s",
            };
            // Fledermausschwarm: schwebt hoch über dem Boden, der Schatten bleibt als weiche Ellipse am Boden zurück.
            let shadow_alpha = if def.id == "bat_swarm" { 0.55 } else { 0.9 };
            frame.shadows.push(SpriteInstance::new(&art.get(shadow), wp).material(Material::Shadow).alpha(shadow_alpha));
            // Auswahl.
            let selected = overlay.selected_units.contains(&id);
            let ring_name = if def.radius > 0.42 { "ring_m" } else { "ring_s" };
            if selected || overlay.hovered == Some(id) {
                let col = if u.owner == PLAYER { Vec4::new(0.45, 1.0, 0.45, 1.0) } else { Vec4::new(1.0, 0.35, 0.3, 1.0) };
                let col = if selected { col } else { col * Vec4::new(0.7, 0.7, 0.7, 1.0) };
                frame.ground.push(SpriteInstance::new(&art.get(ring_name), wp).material(Material::Unlit).tint(col));
            }
            let damaged = u.hp < u.max_hp * 0.999;
            if selected || overlay.show_all_bars || overlay.hovered == Some(id) || (damaged && u.combat_timer > 0.0) {
                let top = wp - Vec2::new(0.0, rect.anchor.y + 3.0);
                bar(frame, top, if def.class == UnitClass::Boss { 26.0 } else { 14.0 }, u.hp / u.max_hp, u.owner == PLAYER, None);
                if u.max_mana > 0.0 && u.owner == PLAYER {
                    bar(frame, top + Vec2::new(0.0, 3.0), 14.0, u.mana / u.max_mana, true, Some(Vec4::new(0.3, 0.5, 1.0, 1.0)));
                }
            }
            // Heldenaura und Effekte.
            if def.class == UnitClass::Hero {
                frame.lights.push(Light::new(wp - Vec2::new(0.0, 10.0), 34.0, Vec3::new(1.0, 0.55, 0.25), 0.55 + night * 0.4).flicker(0.15, 1.0).height(16.0));
            }
            if def.class == UnitClass::Boss {
                frame.lights.push(Light::new(wp - Vec2::new(0.0, 30.0), 60.0, Vec3::new(0.4, 1.0, 0.55), 0.9).flicker(0.2, 2.0).height(24.0));
            }
            if u.shield > 0.0 {
                let ring = art.get("ring_m");
                frame.world.push((depth_of(p) + 0.01, SpriteInstance::new(&ring, wp - Vec2::new(0.0, 8.0)).material(Material::Emissive).emissive(0.8).tint(Vec4::new(1.0, 0.6, 0.3, 1.0))));
            }
            if !u.buffs.is_empty() && rand2((self.time * 30.0) as i32, id.index as i32, 9) > 0.8 {
                self.spawn_ember(art, wp + Vec2::new((rand2(id.index as i32, (self.time * 9.0) as i32, 1) - 0.5) * 10.0, -4.0), Vec3::new(1.0, 0.8, 0.4), id.index + (self.time * 50.0) as u32);
            }
            if u.activity == Activity::Walk && def.class == UnitClass::Cavalry && rand2((self.time * 20.0) as i32, id.index as i32, 2) > 0.7 {
                let mut d = Particle::new(art.get("fx_dust"), wp, 0.0, 0.6);
                d.vz = 4.0;
                d.size = (0.5, 1.2);
                d.color = (Vec4::new(0.45, 0.38, 0.3, 0.5), Vec4::new(0.45, 0.38, 0.3, 0.0));
                self.particles.spawn(d);
            }
        }

        // --- Leichen
        for c in &self.corpses {
            if !in_view(c.pos) || !explored(c.pos) {
                continue;
            }
            let (dir, flip) = crate::art3d::dir_for(c.facing);
            let anims = art.unit_anims(&c.kind, c.hollow, dir);
            let clip = anims.get("death").expect("death");
            let rect = *clip.frame_at(c.age);
            let fade = 1.0 - smooth(16.0, 22.0, c.age);
            frame.world.push((depth_of(c.pos) - 0.3, SpriteInstance::new(&rect, tile_to_world(c.pos)).flip(flip).alpha(fade)));
        }

        // --- Beute am Boden
        for d in &world.drops {
            if !in_view(d.pos) || !visible(d.pos) {
                continue;
            }
            let item = data.item_def(d.item);
            let bob = (self.time * 2.5 + d.pos.x * 1.7).sin() * 1.5;
            let name = format!("icon_item_{}", item.id);
            let rect = art.atlas.try_get(&name).unwrap_or_else(|| art.get("icon_itemslot_ring"));
            let wp = tile_to_world(d.pos);
            let [r, g, b] = item.rarity.color();
            let col = Vec3::new(r as f32, g as f32, b as f32) / 255.0;
            frame.world.push((depth_of(d.pos) + 0.1, SpriteInstance::new(&rect, wp - Vec2::new(0.0, 9.0 + bob)).material(Material::Emissive).emissive(0.5)));
            frame.lights.push(Light::new(wp - Vec2::new(0.0, 6.0), 26.0, col, 0.9).flicker(0.2, d.pos.y));
            if item.rarity >= ashen_sim::rpg::Rarity::Rare && rand2((self.time * 8.0) as i32, (d.pos.x * 10.0) as i32, 4) > 0.82 {
                self.spawn_ember(art, wp - Vec2::new(0.0, 8.0), col, (d.pos.x * 100.0) as u32 + (self.time * 20.0) as u32);
            }
        }

        // --- Projektile
        for p in &world.projectiles {
            let pos = p.prev_pos.lerp(p.pos, alpha);
            if !in_view(pos) || (fog && !world.fog.visible(pos)) {
                continue;
            }
            let t = (p.progress / p.total).clamp(0.0, 1.0);
            let z = p.arc * 4.0 * t * (1.0 - t) * 16.0 + 10.0;
            let wp = tile_to_world(pos);
            let dir = tile_to_world(p.target_pos) - tile_to_world(p.origin);
            let (name, mat) = match p.kind {
                ProjectileKind::Arrow => ("proj_arrow", Material::Lit),
                ProjectileKind::BoneBolt => ("proj_bone", Material::Lit),
                ProjectileKind::SoulBolt => ("proj_soul", Material::Emissive),
                ProjectileKind::Bolt => ("proj_bolt", Material::Lit),
                ProjectileKind::Fireball => ("proj_fireball", Material::Emissive),
                ProjectileKind::Boulder => ("proj_boulder", Material::Lit),
            };
            let rect = art.get(name);
            frame.world.push((depth_of(pos) + 0.5, SpriteInstance::new(&rect, wp - Vec2::new(0.0, z)).flip(dir.x < 0.0).material(mat).emissive(if mat == Material::Emissive { 1.0 } else { 0.0 })));
            if p.kind == ProjectileKind::SoulBolt {
                frame.lights.push(Light::new(wp - Vec2::new(0.0, z), 40.0, Vec3::new(0.4, 1.0, 0.55), 1.0));
            }
            if p.kind == ProjectileKind::Fireball {
                frame.lights.push(Light::new(wp - Vec2::new(0.0, z), 52.0, Vec3::new(1.0, 0.55, 0.2), 1.2).flicker(0.2, 3.0));
                let mut sp = Particle::new(art.get("fx_dot"), wp - Vec2::new(0.0, z), 0.0, 0.3);
                sp.color = (Vec4::new(1.0, 0.6, 0.2, 0.9), Vec4::new(0.6, 0.2, 0.05, 0.0));
                self.particles.spawn(sp);
            }
        }

        // --- Befehlsmarker, Platzierungsvorschau
        for (pos, attack, age) in &self.markers {
            let r = art.get(if *attack { "marker_attack" } else { "marker_move" });
            let a = 1.0 - age / 0.7;
            frame.ground.push(SpriteInstance::new(&r, tile_to_world(*pos)).material(Material::Unlit).alpha(a));
        }
        if let Some((id, tile, size, ok)) = &overlay.ghost {
            let wp = tile_to_world(tile.as_vec2());
            let col = if *ok { Vec4::new(0.6, 1.0, 0.6, 0.7) } else { Vec4::new(1.0, 0.4, 0.35, 0.7) };
            if let Some(fr) = art.atlas.try_get(&format!("foundation_{}x{}", size.0, size.1)) {
                frame.overlay.push(SpriteInstance::new(&fr, wp).material(Material::Unlit).tint(col));
            }
            let name = match id.as_str() {
                "wall" => format!("wall_m{}", wall_mask(&forts, *tile)),
                "gate" => format!("gate_{}_open", if gate_along_x(&forts, *tile) { "x" } else { "z" }),
                other => other.to_string(),
            };
            let rect = art.get(&format!("bld_{name}"));
            frame.overlay.push(SpriteInstance::new(&rect, wp).material(Material::Unlit).tint(col * Vec4::new(1.0, 1.0, 1.0, 0.75)));
        }
        if !overlay.ghost_line.is_empty() {
            let mut set = forts.clone();
            set.extend(overlay.ghost_line.iter().map(|(t, _)| *t));
            for (tile, ok) in &overlay.ghost_line {
                let wp = tile_to_world(tile.as_vec2());
                let col = if *ok { Vec4::new(0.6, 1.0, 0.6, 0.7) } else { Vec4::new(1.0, 0.4, 0.35, 0.7) };
                if let Some(fr) = art.atlas.try_get("foundation_1x1") {
                    frame.overlay.push(SpriteInstance::new(&fr, wp).material(Material::Unlit).tint(col));
                }
                let rect = art.get(&format!("bld_wall_m{}", wall_mask(&set, *tile)));
                frame.overlay.push(SpriteInstance::new(&rect, wp).material(Material::Unlit).tint(col * Vec4::new(1.0, 1.0, 1.0, 0.75)));
            }
        }

        // --- Lichtblitze von Effekten
        for f in &self.flashes {
            let a = f.life / f.max;
            frame.lights.push(Light::new(f.pos - Vec2::new(0.0, 8.0), f.radius * (0.6 + 0.4 * a), f.color, 1.6 * a).height(20.0));
        }
        self.particles.draw(frame, depth_from_world_y);
        let _ = SpriteRect::default();
    }
}

fn smooth(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Lebens-/Fortschrittsbalken als unbeleuchtetes Overlay.
fn bar(frame: &mut FrameData, center_top: Vec2, width: f32, frac: f32, friendly: bool, color: Option<Vec4>) {
    let x = (center_top.x - width * 0.5).round();
    let y = center_top.y.round();
    frame.overlay.push(SpriteInstance::solid(Vec2::new(x - 1.0, y - 1.0), Vec2::new(width + 2.0, 4.0), Vec4::new(0.03, 0.02, 0.02, 1.0)).material(Material::Solid));
    let f = frac.clamp(0.0, 1.0);
    let col = color.unwrap_or_else(|| {
        if friendly {
            Vec4::new(0.25, 0.85, 0.3, 1.0).lerp(Vec4::new(0.95, 0.3, 0.15, 1.0), 1.0 - f)
        } else {
            Vec4::new(0.9, 0.2, 0.15, 1.0)
        }
    });
    if f > 0.0 {
        frame.overlay.push(SpriteInstance::solid(Vec2::new(x, y), Vec2::new((width * f).max(1.0).round(), 2.0), col));
    }
}
