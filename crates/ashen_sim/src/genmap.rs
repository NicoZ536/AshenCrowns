//! Parametrischer Kartengenerator für Gefechte und den Karteneditor.
//!
//! Erzeugt aus Größe, Seed und Vorlage eine vollständige [`MapData`] (Terrain, Dekoration,
//! Ressourcenquellen) und zwei Startpositionen, die garantiert durch begehbares Gelände verbunden sind.

use crate::data::GameData;
use crate::entity::NodeKind;
use crate::map::{Decor, DecorKind, Terrain, TileMap};
use crate::mapgen::{dist_to_polyline, RIVER, ROAD};
use crate::scenario::{DecorPlace, MapData, NodePlace};
use crate::world::SimWorld;
use ashen_core::noise::{fbm2, rand2};
use glam::{IVec2, Vec2};
use serde::{Deserialize, Serialize};

/// Kartenvorlagen des Generators.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GenPreset {
    /// Wie Mission 1: Fluss mit Brücke, Aschland um den gegnerischen Schrein.
    Aschmark,
    /// Offenes Grasland mit Wäldern und kleinen Teichen.
    Heartland,
    /// Zwei Ufer, ein breiter Fluss mit Furten und Brücke.
    Riverlands,
    /// Felsiges Hochland mit Felsbrocken und Engpässen.
    Highlands,
    /// Verbranntes Land: Asche, tote Bäume, Ruinen.
    Ashlands,
    /// Großer See in der Mitte; Pässe an den Ufern.
    Lakes,
}

impl GenPreset {
    pub const ALL: [GenPreset; 6] = [GenPreset::Aschmark, GenPreset::Heartland, GenPreset::Riverlands, GenPreset::Highlands, GenPreset::Ashlands, GenPreset::Lakes];

    pub fn label(self) -> &'static str {
        match self {
            GenPreset::Aschmark => "Aschmark",
            GenPreset::Heartland => "Grasland",
            GenPreset::Riverlands => "Flusstal",
            GenPreset::Highlands => "Hochland",
            GenPreset::Ashlands => "Aschland",
            GenPreset::Lakes => "Seenplatte",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            GenPreset::Aschmark => "Fluss mit Brücke und Furten, verderbte Asche um den Gegner – wie in Mission 1.",
            GenPreset::Heartland => "Offenes Land, dichte Wälder, einige Teiche. Viel Platz zum Bauen.",
            GenPreset::Riverlands => "Ein breiter Fluss trennt die Parteien; nur Furten und eine Brücke führen hinüber.",
            GenPreset::Highlands => "Fels, Felsbrocken und Engpässe. Verteidiger haben es leicht.",
            GenPreset::Ashlands => "Asche, tote Bäume und Ruinen. Holz ist knapp.",
            GenPreset::Lakes => "Ein See in der Mitte lenkt Heere an die Ufer und auf schmale Pässe.",
        }
    }
}

/// Ergebnis des Generators: Karte plus Startpositionen (Spieler, Gegner).
#[derive(Clone, Debug)]
pub struct Generated {
    pub map: MapData,
    pub starts: [Vec2; 2],
}

struct PresetParams {
    river: bool,
    lakes: bool,
    stone: f32,
    ash_radius: f32,
    ash_everywhere: bool,
    forest: f32,
    boulders: f32,
    ruins: bool,
    dead_trees: bool,
}

fn params(p: GenPreset) -> PresetParams {
    let base = PresetParams { river: false, lakes: false, stone: 0.0, ash_radius: 0.0, ash_everywhere: false, forest: 0.60, boulders: 0.0, ruins: false, dead_trees: false };
    match p {
        GenPreset::Aschmark => PresetParams { river: true, stone: 0.52, ash_radius: 13.0, ruins: true, ..base },
        GenPreset::Heartland => PresetParams { lakes: true, forest: 0.56, ..base },
        GenPreset::Riverlands => PresetParams { river: true, forest: 0.58, ..base },
        GenPreset::Highlands => PresetParams { stone: 0.40, boulders: 0.045, forest: 0.68, ..base },
        GenPreset::Ashlands => PresetParams { ash_everywhere: true, ash_radius: 16.0, forest: 0.80, ruins: true, dead_trees: true, ..base },
        GenPreset::Lakes => PresetParams { lakes: true, forest: 0.60, ..base },
    }
}

fn scaled(points: &[(f32, f32)], w: i32, h: i32) -> Vec<(f32, f32)> {
    points.iter().map(|(x, y)| (x * w as f32 / 72.0, y * h as f32 / 72.0)).collect()
}

fn dist_to_segment(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_squared().max(1e-6)).clamp(0.0, 1.0);
    (p - (a + ab * t)).length()
}

/// Deterministischer Seed der Zusatzdekoration aus Größe und Gelände (unabhängig von Spielstand und Zufall).
fn flavor_seed(map: &TileMap) -> u32 {
    let mut h: u32 = 2166136261 ^ (map.width as u32).wrapping_mul(31) ^ (map.height as u32).wrapping_mul(1009);
    for t in &map.terrain {
        h = (h ^ t.code() as u32).wrapping_mul(16777619);
    }
    h
}

/// Streut rein kosmetische Bodenbewohner nach Biom aus: Kiesel, Felsbrocken, Büschel an Gras-Übergängen, Schilf am Ufer,
/// Knochen und Ascheglut im Aschland. Die Platzierung hängt nur vom Gelände ab (deterministisch, idempotent: Kacheln mit
/// vorhandener Dekoration bleiben unberührt) und verwendet ausschließlich nicht blockierende Arten; Wegfindung und
/// Bauplätze bleiben unverändert. Dichte schwankt mit einem Flächenrauschen in Flecken, damit Gruppen statt Gleichverteilung entstehen.
pub fn add_flavor_decor(map: &mut TileMap) {
    let s = flavor_seed(map);
    let (w, h) = (map.width, map.height);
    let mut taken = vec![false; (w * h) as usize];
    for d in &map.decor {
        let t = TileMap::tile_of(d.pos);
        if map.in_bounds(t) {
            taken[map.idx(t)] = true;
        }
    }
    let is_water = |t: Terrain| matches!(t, Terrain::Water | Terrain::DeepWater);
    let mut out: Vec<Decor> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let t = IVec2::new(x, y);
            let i = map.idx(t);
            if taken[i] || map.static_block[i] {
                continue;
            }
            let terr = map.terrain[i];
            let around = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y].map(|d| map.terrain_at(t + d));
            let near_water = around.iter().any(|a| is_water(*a));
            let near_dry = around.iter().any(|a| matches!(a, Terrain::Dirt | Terrain::Sand | Terrain::Ash | Terrain::Stone));
            // Flecken: 0.35 .. 1.75-fache Dichte.
            let patch = 0.35 + fbm2(x as f32 * 0.13 + 9.0, y as f32 * 0.13, s ^ 0xB10E, 3) * 1.4;
            let r = rand2(x, y, s ^ 0xF1A0);
            let kind = match terr {
                Terrain::Sand => {
                    if near_water && r < 0.34 {
                        Some(DecorKind::Reeds)
                    } else if r < 0.07 * patch {
                        Some(DecorKind::Pebbles)
                    } else if r < 0.07 * patch + 0.012 {
                        Some(DecorKind::Rock)
                    } else if r > 1.0 - 0.004 * patch {
                        Some(DecorKind::Bones)
                    } else {
                        None
                    }
                }
                Terrain::Dirt => {
                    if r < 0.08 * patch {
                        Some(DecorKind::Pebbles)
                    } else if r < 0.08 * patch + 0.025 * patch {
                        Some(DecorKind::Rock)
                    } else if r > 0.93 - 0.03 * patch && r <= 0.93 {
                        Some(DecorKind::Puddle)
                    } else if r > 1.0 - 0.05 * patch {
                        Some(DecorKind::GrassTuft)
                    } else if near_water && r > 0.82 {
                        Some(DecorKind::Reeds)
                    } else {
                        None
                    }
                }
                Terrain::Ash => {
                    if r < 0.05 * patch {
                        Some(DecorKind::Embers)
                    } else if r < 0.05 * patch + 0.014 * patch {
                        Some(DecorKind::Bones)
                    } else if r < 0.05 * patch + 0.014 * patch + 0.035 * patch {
                        Some(DecorKind::Pebbles)
                    } else if r > 1.0 - 0.02 * patch {
                        Some(DecorKind::Rock)
                    } else {
                        None
                    }
                }
                Terrain::Stone => {
                    if r < 0.06 * patch {
                        Some(DecorKind::Pebbles)
                    } else {
                        None
                    }
                }
                Terrain::Grass | Terrain::DarkGrass => {
                    if near_water && r < 0.22 {
                        Some(DecorKind::Reeds)
                    } else if near_dry && r < 0.2 {
                        // Übergang Gras/Erde: Büschel und einzelne Kiesel am Rand.
                        Some(if r < 0.17 { DecorKind::GrassTuft } else { DecorKind::Pebbles })
                    } else if terr == Terrain::DarkGrass && r > 1.0 - 0.012 * patch {
                        Some(DecorKind::Bush)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(kind) = kind {
                debug_assert!(!kind.blocks());
                let jitter = Vec2::new(rand2(x, y, s ^ 0x31) - 0.5, rand2(x, y, s ^ 0x32) - 0.5) * 0.6;
                out.push(Decor { kind, pos: TileMap::center_of(t) + jitter, variant: (rand2(x, y, s ^ 0x33) * 8.0) as u8 });
            }
        }
    }
    map.decor.extend(out);
}

/// Erzeugt eine vollständige Karte für Größen von 24 bis 160, Seed und Vorlage.
pub fn generate_preset(width: i32, height: i32, seed: u64, preset: GenPreset) -> Generated {
    let (w, h) = (width.clamp(24, 160), height.clamp(24, 160));
    let pr = params(preset);
    let s = (seed as u32) ^ (seed >> 32) as u32;
    let player = Vec2::new((w as f32 * 0.20).round() + 0.5, (h as f32 * 0.78).round() + 0.5);
    let enemy = Vec2::new((w as f32 * 0.85).round() + 0.5, (h as f32 * 0.19).round() + 0.5);
    let river = scaled(&RIVER, w, h);
    let road = {
        let mut r = scaled(&ROAD, w, h);
        r[0] = (player.x + 1.0, player.y);
        let last = r.len() - 1;
        r[last] = (enemy.x - 6.0, enemy.y + 5.0);
        r
    };
    let mut m = TileMap::new(w, h, Terrain::Grass);
    let corridor = |p: Vec2| dist_to_segment(p, player, enemy);
    let (fw, fh) = (w as f32 / 72.0, h as f32 / 72.0);

    for y in 0..h {
        for x in 0..w {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let n = fbm2(p.x * 0.09, p.y * 0.09, s, 4);
            let n2 = fbm2(p.x * 0.17 + 40.0, p.y * 0.17, s.wrapping_add(7), 3);
            let mut terr = if n > 0.60 {
                Terrain::DarkGrass
            } else if n < 0.27 {
                Terrain::Dirt
            } else {
                Terrain::Grass
            };
            if pr.stone > 0.0 {
                let corner = if preset == GenPreset::Aschmark { (p.x + p.y) < 26.0 * (fw + fh) * 0.5 } else { true };
                if corner && n2 > pr.stone {
                    terr = Terrain::Stone;
                }
            }
            if pr.ash_everywhere && n > 0.42 {
                terr = Terrain::Ash;
            }
            if pr.ash_radius > 0.0 {
                let ash = (p - enemy).length() + (n2 - 0.5) * 8.0;
                if ash < pr.ash_radius {
                    terr = Terrain::Ash;
                } else if ash < pr.ash_radius + 3.0 && n > 0.45 && terr != Terrain::Ash {
                    terr = Terrain::Dirt;
                }
            }
            if pr.lakes {
                let l = fbm2(p.x * 0.075 + 90.0, p.y * 0.075 + 11.0, s ^ 0x1A4E, 3);
                let centre = 1.0 - ((p - Vec2::new(w as f32, h as f32) * 0.5).length() / (w.max(h) as f32 * 0.5)).min(1.0);
                let (score, limit) = if preset == GenPreset::Lakes { (l + centre * 0.35, 0.80) } else { (l, 0.70) };
                if score > limit + 0.07 {
                    terr = Terrain::DeepWater;
                } else if score > limit {
                    terr = Terrain::Water;
                } else if score > limit - 0.04 {
                    terr = Terrain::Sand;
                }
            }
            if pr.river {
                let (d, along) = dist_to_polyline(p, &river);
                let width_r = 1.9 + (along * 9.0 + n2 * 2.0).sin().abs() * 0.9;
                if d < width_r * 0.45 {
                    terr = Terrain::DeepWater;
                } else if d < width_r {
                    terr = Terrain::Water;
                } else if d < width_r + 1.1 {
                    terr = Terrain::Sand;
                }
                let ford_n = (p - Vec2::new(21.0 * fw, 12.0 * fh)).length() < 2.6;
                let ford_s = (p - Vec2::new(53.0 * fw, 63.0 * fh)).length() < 2.6;
                if (ford_n || ford_s) && d < width_r {
                    terr = Terrain::Ford;
                }
                if dist_to_polyline(p, &road).0 < 0.85 {
                    terr = Terrain::Road;
                }
            }
            // Gesicherter Korridor zwischen den Startpositionen: Furt statt Wasser.
            if corridor(p) < 1.4 && !terr.walkable() {
                terr = Terrain::Ford;
            }
            // Bauflächen an den Startpositionen.
            if (p - player).length() < 8.0 && (!terr.buildable() || terr == Terrain::Stone) {
                terr = Terrain::Grass;
            }
            if (p - enemy).length() < 7.0 && !terr.buildable() {
                terr = if pr.ash_radius > 0.0 { Terrain::Ash } else { Terrain::Dirt };
            }
            m.set_terrain(IVec2::new(x, y), terr);
        }
    }

    let reserved = |p: Vec2| (p - player).length() < 9.0 || (p - enemy).length() < 8.0 || corridor(p) < 2.4;
    let mut decor: Vec<DecorPlace> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let t = IVec2::new(x, y);
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let terr = m.terrain_at(t);
            let r = rand2(x, y, s ^ 0x51);
            let jitter = Vec2::new(rand2(x, y, s ^ 0x77) - 0.5, rand2(x, y, s ^ 0x78) - 0.5) * 0.6;
            let variant = (rand2(x, y, s ^ 0x99) * 8.0) as u8;
            let near_base = (p - player).length() < 7.0;
            let res = reserved(p);
            let kind = match terr {
                Terrain::Grass | Terrain::DarkGrass if !near_base => {
                    if r < 0.10 {
                        Some(DecorKind::GrassTuft)
                    } else if r < 0.13 {
                        Some(DecorKind::Flowers)
                    } else if r < 0.155 {
                        Some(DecorKind::Bush)
                    } else if r < 0.165 {
                        Some(DecorKind::Rock)
                    } else if pr.boulders > 0.0 && !res && r > 1.0 - pr.boulders {
                        Some(DecorKind::Boulder)
                    } else {
                        None
                    }
                }
                Terrain::Dirt if r < 0.05 => Some(DecorKind::Rock),
                Terrain::Stone if r < 0.12 => Some(DecorKind::Rock),
                Terrain::Stone if r < 0.12 + pr.boulders.max(0.03) && !res => Some(DecorKind::Boulder),
                Terrain::Sand if r < 0.25 => Some(DecorKind::Reeds),
                Terrain::Ash if !res => {
                    if r < 0.06 {
                        Some(DecorKind::Bones)
                    } else if r < 0.10 {
                        Some(DecorKind::Grave)
                    } else if r < 0.115 || (pr.dead_trees && r < 0.15) {
                        Some(DecorKind::DeadTree)
                    } else if r < 0.13 + if pr.ruins { 0.02 } else { 0.0 } {
                        Some(DecorKind::Pillar)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(k) = kind {
                let pos = if k.blocks() { p } else { p + jitter };
                decor.push(DecorPlace { kind: k, x: pos.x, y: pos.y, variant });
            }
        }
    }
    if pr.ruins {
        let c = (player + enemy) * 0.5;
        for (k, dx, dy) in [(DecorKind::Ruin, -2.0, 1.0), (DecorKind::Pillar, 0.0, -2.0), (DecorKind::Pillar, -4.0, 3.0), (DecorKind::Ruin, 3.0, 3.0)] {
            let p = Vec2::new((c.x + dx).floor() + 0.5, (c.y + dy).floor() + 0.5);
            let t = IVec2::new(p.x as i32, p.y as i32);
            if m.terrain_at(t).walkable() && corridor(p) > 2.0 {
                decor.retain(|d| TileMap::tile_of(Vec2::new(d.x, d.y)) != t);
                decor.push(DecorPlace { kind: k, x: p.x, y: p.y, variant: 0 });
            }
        }
    }

    // Ressourcen: feste Minen und Steinbrüche (Spielbalance), danach Wälder.
    let mut occ = vec![false; (w * h) as usize];
    let mut nodes: Vec<NodePlace> = Vec::new();
    let free2 = |m: &TileMap, occ: &[bool], t: IVec2| -> bool {
        (0..2).all(|dy| {
            (0..2).all(|dx| {
                let c = t + IVec2::new(dx, dy);
                m.in_bounds(c) && !matches!(m.terrain_at(c), Terrain::Ford | Terrain::Road | Terrain::Water | Terrain::DeepWater) && !occ[(c.y * w + c.x) as usize]
            })
        })
    };
    let sites: [(NodeKind, Vec2); 6] = [
        (NodeKind::GoldMine, player + Vec2::new(-7.0, -9.0)),
        (NodeKind::StoneQuarry, player + Vec2::new(8.0, -9.0)),
        (NodeKind::GoldMine, Vec2::new(w as f32 * 0.69, h as f32 * 0.62)),
        (NodeKind::StoneQuarry, Vec2::new(w as f32 * 0.42, h as f32 * 0.28)),
        (NodeKind::GoldMine, enemy + Vec2::new(1.5, 17.0 * fh)),
        (NodeKind::GoldMine, Vec2::new(w as f32 * 0.34, h as f32 * 0.58)),
    ];
    for (k, c) in sites {
        let base = IVec2::new(c.x.round() as i32, c.y.round() as i32).clamp(IVec2::splat(1), IVec2::new(w - 3, h - 3));
        'search: for r in 0..10_i32 {
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx.abs().max(dy.abs()) != r {
                        continue;
                    }
                    let t = base + IVec2::new(dx, dy);
                    if t.x < 1 || t.y < 1 || t.x > w - 3 || t.y > h - 3 {
                        continue;
                    }
                    if free2(&m, &occ, t) && corridor(TileMap::center_of(t) + Vec2::splat(0.5)) > 2.2 {
                        for yy in 0..2 {
                            for xx in 0..2 {
                                occ[((t.y + yy) * w + t.x + xx) as usize] = true;
                            }
                        }
                        nodes.push(NodePlace { kind: k, x: t.x, y: t.y, amount: 0, variant: 0 });
                        break 'search;
                    }
                }
            }
        }
    }
    decor.retain(|d| {
        let t = TileMap::tile_of(Vec2::new(d.x, d.y));
        !nodes.iter().any(|n| {
            let dt = t - IVec2::new(n.x, n.y);
            dt.x >= -1 && dt.y >= -1 && dt.x < 3 && dt.y < 3
        })
    });
    for y in 0..h {
        for x in 0..w {
            let t = IVec2::new(x, y);
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            if reserved(p) || occ[(y * w + x) as usize] {
                continue;
            }
            let terr = m.terrain_at(t);
            if !matches!(terr, Terrain::Grass | Terrain::DarkGrass | Terrain::Dirt | Terrain::Ash | Terrain::Stone) {
                continue;
            }
            let forest = fbm2(p.x * 0.11 + 13.0, p.y * 0.11, s ^ 0xF0, 4);
            let r = rand2(x, y, s ^ 0x7EE);
            let edge = ((6.0 - x.min(y) as f32).max(0.0) / 6.0) * 0.35;
            let dense = forest + edge > pr.forest;
            let place = if dense { r < 0.78 } else { r < 0.025 };
            if !place {
                continue;
            }
            if nodes.iter().any(|n| n.kind != NodeKind::Tree && (Vec2::new(n.x as f32 + 1.0, n.y as f32 + 1.0) - p).length() < 2.4) {
                continue;
            }
            // Blockierende Dekoration auf der Kachel behält den Vorrang.
            if decor.iter().any(|d| TileMap::tile_of(Vec2::new(d.x, d.y)) == t && d.kind.blocks()) {
                continue;
            }
            decor.retain(|d| TileMap::tile_of(Vec2::new(d.x, d.y)) != t);
            let variant = if terr == Terrain::Ash { 200 + (r * 1000.0) as u8 % 3 } else { (rand2(x, y, s ^ 0x3) * 6.0) as u8 };
            occ[(y * w + x) as usize] = true;
            nodes.push(NodePlace { kind: NodeKind::Tree, x, y, amount: 0, variant });
        }
    }
    let map = MapData { width: w, height: h, terrain: MapData::rows_from(w, h, &m.terrain), decor, nodes };
    Generated { map, starts: [player, enemy] }
}

/// Platziert die Ressourcenquellen einer expliziten Karte in der Welt.
pub fn populate_from_data(world: &mut SimWorld, data: &GameData) {
    let Some(md) = world.mission_def.map.clone() else { return };
    let rules = &data.rules;
    for n in &md.nodes {
        let amount = if n.amount > 0 {
            n.amount
        } else {
            match n.kind {
                NodeKind::GoldMine => rules.gold_mine_gold,
                NodeKind::StoneQuarry => rules.quarry_stone,
                NodeKind::Tree => rules.tree_wood,
            }
        };
        let t = IVec2::new(n.x, n.y);
        let size = n.kind.size();
        // Blockierende Dekoration am Standort weicht der Quelle.
        world.map.decor.retain(|d| {
            let dt = TileMap::tile_of(d.pos) - t;
            !(dt.x >= 0 && dt.y >= 0 && dt.x < size.x && dt.y < size.y)
        });
        for dy in 0..size.y {
            for dx in 0..size.x {
                let c = t + IVec2::new(dx, dy);
                if world.map.in_bounds(c) {
                    let i = world.map.idx(c);
                    world.map.static_block[i] = false;
                }
            }
        }
        if world.spawn_node(n.kind, t, amount, n.variant).is_none() {
            log::warn!("Ressourcenquelle {:?} bei {t} konnte nicht platziert werden", n.kind);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_is_deterministic_and_connected() {
        for preset in GenPreset::ALL {
            for (w, h) in [(48, 48), (72, 72), (96, 64)] {
                let a = generate_preset(w, h, 4242, preset);
                let b = generate_preset(w, h, 4242, preset);
                assert_eq!(a.map, b.map, "{preset:?} deterministisch");
                assert_eq!(a.map.terrain.len(), h as usize);
                assert!(a.map.nodes.iter().any(|n| n.kind == NodeKind::GoldMine), "{preset:?} hat Gold");
                let m = a.map.to_tilemap().unwrap();
                let mut pf = crate::path::Pathfinder::default();
                let from = a.starts[0] + Vec2::new(2.0, 0.0);
                let to = a.starts[1] + Vec2::new(-3.0, 2.0);
                let p = pf.find(&m, from, to, 60_000).unwrap_or_else(|| panic!("{preset:?} {w}x{h}: kein Weg"));
                assert!((*p.last().unwrap() - to).length() < 1.5, "{preset:?} {w}x{h}: Ziel nicht erreicht");
            }
        }
    }

    #[test]
    fn flavor_decor_is_deterministic_idempotent_and_never_blocks() {
        for preset in GenPreset::ALL {
            let g = generate_preset(72, 72, 99, preset);
            let mut a = g.map.to_tilemap().unwrap();
            let blocked: Vec<bool> = a.static_block.clone();
            let before = a.decor.len();
            add_flavor_decor(&mut a);
            let added = a.decor.len() - before;
            assert!(added > 20, "{preset:?}: kaum Zusatzdekoration ({added})");
            assert_eq!(a.static_block, blocked, "{preset:?}: Dekoration darf nichts blockieren");
            assert!(a.decor[before..].iter().all(|d| !d.kind.blocks()), "{preset:?}: blockierende Art");
            // Gleiche Karte, gleiches Ergebnis.
            let mut b = g.map.to_tilemap().unwrap();
            add_flavor_decor(&mut b);
            assert_eq!(a.decor.len(), b.decor.len());
            assert!(a.decor.iter().zip(&b.decor).all(|(x, y)| x.kind == y.kind && x.pos == y.pos && x.variant == y.variant));
            // Zweiter Durchlauf fügt nichts mehr hinzu.
            let n = a.decor.len();
            add_flavor_decor(&mut a);
            assert_eq!(a.decor.len(), n, "{preset:?}: nicht idempotent");
        }
    }
}
