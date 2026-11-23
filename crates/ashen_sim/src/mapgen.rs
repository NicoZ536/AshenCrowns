//! Kartengenerator der Aschmark: handentworfenes Layout (Basis, Fluss, Brücke, Ruinen)
//! kombiniert mit seed-basierter Variation (Rauschen für Terrain, Wälder, Dekoration).
//!
//! Achsen: Kachel-x verläuft auf dem Bildschirm nach rechts unten, Kachel-y nach links unten.
//! Die Spielerbasis liegt damit links (hohes y), der Hohlschrein rechts (hohes x).

use crate::data::GameData;
use crate::entity::NodeKind;
use crate::map::{Decor, DecorKind, Terrain, TileMap};
use crate::world::SimWorld;
use ashen_core::noise::{fbm2, rand2};
use glam::{IVec2, Vec2};

pub const MAP_SIZE: i32 = 72;

/// Mittellinie des Flusses (Kachelkoordinaten).
pub(crate) const RIVER: [(f32, f32); 6] = [(12.0, -2.0), (24.0, 16.0), (33.0, 30.0), (40.0, 42.0), (50.0, 58.0), (58.0, 74.0)];
/// Straße vom Bergfried über die Brücke Richtung Ruinen.
pub(crate) const ROAD: [(f32, f32); 6] = [(15.0, 56.0), (22.0, 50.0), (30.0, 42.0), (38.0, 36.5), (46.0, 28.0), (55.0, 18.0)];

pub const PLAYER_BASE: Vec2 = Vec2::new(14.0, 56.0);
pub const ENEMY_BASE: Vec2 = Vec2::new(61.5, 13.5);

pub(crate) fn dist_to_polyline(p: Vec2, line: &[(f32, f32)]) -> (f32, f32) {
    let mut best = (f32::MAX, 0.0);
    let n = line.len() - 1;
    for i in 0..n {
        let a = Vec2::new(line[i].0, line[i].1);
        let b = Vec2::new(line[i + 1].0, line[i + 1].1);
        let ab = b - a;
        let t = ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
        let d = (p - (a + ab * t)).length();
        if d < best.0 {
            best = (d, (i as f32 + t) / n as f32);
        }
    }
    best
}

/// Liegt die Kachel in einer Zone, die frei von Wald bleiben muss?
pub fn reserved(p: Vec2) -> bool {
    (p - PLAYER_BASE).length() < 9.5 || (p - ENEMY_BASE).length() < 8.5 || dist_to_polyline(p, &ROAD).0 < 2.2
}

/// Erzeugt das Terrain und die Dekoration der Karte.
pub fn generate(map_seed: u64, variation_seed: u64) -> TileMap {
    let s = (map_seed as u32) ^ (map_seed >> 32) as u32;
    let v = (variation_seed as u32).wrapping_mul(0x9E37_79B9);
    let mut m = TileMap::new(MAP_SIZE, MAP_SIZE, Terrain::Grass);

    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let t = IVec2::new(x, y);
            let n = fbm2(p.x * 0.09, p.y * 0.09, s ^ v, 4);
            let n2 = fbm2(p.x * 0.17 + 40.0, p.y * 0.17, s.wrapping_add(7), 3);
            let mut terr = if n > 0.60 {
                Terrain::DarkGrass
            } else if n < 0.27 {
                Terrain::Dirt
            } else {
                Terrain::Grass
            };
            // Felsiges Hochland im Norden (niedriges x und y).
            if (x + y) < 26 && n2 > 0.52 {
                terr = Terrain::Stone;
            }
            // Verderbte Asche um den Schrein und entlang der Ruinen.
            let ash = (p - ENEMY_BASE).length() + (n2 - 0.5) * 8.0;
            if ash < 13.0 {
                terr = Terrain::Ash;
            } else if ash < 16.0 && n > 0.45 {
                terr = Terrain::Dirt;
            }
            // Fluss mit Wellenverlauf.
            let (d, along) = dist_to_polyline(p, &RIVER);
            let width = 1.9 + (along * 9.0 + n2 * 2.0).sin().abs() * 0.9;
            if d < width * 0.45 {
                terr = Terrain::DeepWater;
            } else if d < width {
                terr = Terrain::Water;
            } else if d < width + 1.1 {
                terr = Terrain::Sand;
            }
            // Furten im Norden und Süden.
            let ford_n = (p - Vec2::new(21.0, 12.0)).length() < 2.6;
            let ford_s = (p - Vec2::new(53.0, 63.0)).length() < 2.6;
            if (ford_n || ford_s) && d < width {
                terr = Terrain::Ford;
            }
            // Straße (inkl. Brücke über den Fluss).
            let (rd, _) = dist_to_polyline(p, &ROAD);
            if rd < 0.85 {
                terr = Terrain::Road;
            }
            m.set_terrain(t, terr);
        }
    }

    // Dekoration.
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let t = IVec2::new(x, y);
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let terr = m.terrain_at(t);
            let r = rand2(x, y, s ^ 0x51);
            let jitter = Vec2::new(rand2(x, y, s ^ 0x77) - 0.5, rand2(x, y, s ^ 0x78) - 0.5) * 0.6;
            let variant = (rand2(x, y, s ^ 0x99) * 8.0) as u8;
            let near_base = (p - PLAYER_BASE).length() < 7.0;
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
                    } else {
                        None
                    }
                }
                Terrain::Dirt if r < 0.05 => Some(DecorKind::Rock),
                Terrain::Stone if r < 0.12 => Some(DecorKind::Rock),
                Terrain::Stone if r < 0.15 && !reserved(p) => Some(DecorKind::Boulder),
                Terrain::Sand if r < 0.25 => Some(DecorKind::Reeds),
                Terrain::Ash if !reserved(p) => {
                    if r < 0.06 {
                        Some(DecorKind::Bones)
                    } else if r < 0.10 {
                        Some(DecorKind::Grave)
                    } else if r < 0.115 {
                        Some(DecorKind::DeadTree)
                    } else if r < 0.13 {
                        Some(DecorKind::Pillar)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(k) = kind {
                let pos = if k.blocks() { p } else { p + jitter };
                m.decor.push(Decor { kind: k, pos, variant });
                if k.blocks() {
                    let i = m.idx(t);
                    m.static_block[i] = true;
                }
            }
        }
    }

    // Handplatzierte Wahrzeichen: Ruinen um die Flussmitte, Kohlenbecken an der Straße.
    let landmarks: [(DecorKind, f32, f32); 14] = [
        (DecorKind::Ruin, 44.5, 37.5),
        (DecorKind::Pillar, 46.5, 35.5),
        (DecorKind::Pillar, 42.5, 39.5),
        (DecorKind::Ruin, 47.5, 40.5),
        (DecorKind::Brazier, 19.5, 52.5),
        (DecorKind::Brazier, 27.5, 44.5),
        (DecorKind::Lantern, 17.5, 59.5),
        (DecorKind::Lantern, 10.5, 52.5),
        (DecorKind::Fence, 20.5, 60.5),
        (DecorKind::Fence, 21.5, 60.5),
        (DecorKind::Grave, 9.5, 61.5),
        (DecorKind::Grave, 8.5, 62.5),
        (DecorKind::Ruin, 33.5, 22.5),
        (DecorKind::Pillar, 52.5, 46.5),
    ];
    for (k, x, y) in landmarks {
        let t = IVec2::new(x as i32, y as i32);
        if !m.terrain_at(t).walkable() {
            continue;
        }
        m.decor.retain(|d| TileMap::tile_of(d.pos) != t);
        m.decor.push(Decor { kind: k, pos: Vec2::new(x, y), variant: 0 });
        if k.blocks() {
            let i = m.idx(t);
            m.static_block[i] = true;
        }
    }
    m
}

/// Platziert Bäume, Goldminen und Steinbrüche.
pub fn populate_resources(world: &mut SimWorld, data: &GameData) {
    let seed = world.mission_def.map_seed as u32;
    let rules = data.rules.clone();
    // Feste Minen und Steinbrüche (Spielbalance).
    let fixed = [
        (NodeKind::GoldMine, 7, 47),
        (NodeKind::GoldMine, 50, 45),
        (NodeKind::GoldMine, 63, 30),
        (NodeKind::StoneQuarry, 22, 47),
        (NodeKind::StoneQuarry, 30, 20),
    ];
    for (k, x, y) in fixed {
        let amount = match k {
            NodeKind::GoldMine => rules.gold_mine_gold,
            NodeKind::StoneQuarry => rules.quarry_stone,
            NodeKind::Tree => rules.tree_wood,
        };
        let t = IVec2::new(x, y);
        // Dekoration am Standort entfernen.
        world.map.decor.retain(|d| {
            let dt = TileMap::tile_of(d.pos) - t;
            !(dt.x >= 0 && dt.y >= 0 && dt.x < 2 && dt.y < 2)
        });
        for dy in 0..2 {
            for dx in 0..2 {
                let i = world.map.idx(t + IVec2::new(dx, dy));
                world.map.static_block[i] = false;
            }
        }
        if world.spawn_node(k, t, amount, 0).is_none() {
            log::warn!("Ressourcenquelle {k:?} bei {t} konnte nicht platziert werden");
        }
    }
    // Wälder: dichte Cluster über Rauschen, verstreute Einzelbäume.
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let t = IVec2::new(x, y);
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            if reserved(p) || !world.map.walkable(t) {
                continue;
            }
            let terr = world.map.terrain_at(t);
            if !matches!(terr, Terrain::Grass | Terrain::DarkGrass | Terrain::Dirt | Terrain::Ash | Terrain::Stone) {
                continue;
            }
            let forest = fbm2(p.x * 0.11 + 13.0, p.y * 0.11, seed ^ 0xF0, 4);
            let r = rand2(x, y, seed ^ 0x7EE);
            // Waldgürtel am westlichen und südlichen Kartenrand verstärken.
            let edge = ((6.0 - x.min(y) as f32).max(0.0) / 6.0) * 0.35;
            let dense = forest + edge > 0.60;
            let place = if dense { r < 0.78 } else { r < 0.025 };
            if !place {
                continue;
            }
            // Kein Baum direkt neben Minen, damit Arbeiter Platz haben.
            let blocked_near = world.nodes.iter().any(|(_, n)| n.kind != NodeKind::Tree && n.distance_to(p) < 1.6);
            if blocked_near {
                continue;
            }
            world.map.decor.retain(|d| TileMap::tile_of(d.pos) != t);
            let variant = if terr == Terrain::Ash { 200 + (r * 1000.0) as u8 % 3 } else { (rand2(x, y, seed ^ 0x3) * 6.0) as u8 };
            let _ = world.spawn_node(NodeKind::Tree, t, rules.tree_wood, variant);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_is_deterministic_and_has_crossings() {
        let a = generate(897, 1);
        let b = generate(897, 1);
        assert_eq!(a.terrain, b.terrain);
        assert!(a.terrain.contains(&Terrain::Ford));
        assert!(a.terrain.contains(&Terrain::Road));
        // Die Straße muss den Fluss überqueren (Brücke).
        let mut pf = crate::path::Pathfinder::default();
        let p = pf.find(&a, PLAYER_BASE + Vec2::new(3.0, 3.0), ENEMY_BASE + Vec2::new(-4.0, 4.0), 20_000).expect("path");
        let end = *p.last().unwrap();
        assert!((end - (ENEMY_BASE + Vec2::new(-4.0, 4.0))).length() < 1.5, "enemy base reachable");
    }
}
