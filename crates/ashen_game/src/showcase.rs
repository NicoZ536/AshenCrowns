//! Vorführstadt für Screenshots und Art-Reviews (`--demo-city`): setzt in der Sandbox eine
//! belebte Stadt mit Produktionsgebäuden, Wohnhäusern, Markt und einem Mauerring mit Toren.

use ashen_sim::data::{Fort, GameData};
use ashen_sim::entity::PLAYER;
use ashen_sim::SimWorld;
use glam::IVec2;

/// Platziert ein fertiges Gebäude möglichst nahe an `want`.
fn put(world: &mut SimWorld, data: &GameData, id: &str, want: IVec2) -> bool {
    let Some(kind) = data.building_id(id) else { return false };
    for r in 0..8i32 {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue;
                }
                let t = want + IVec2::new(dx, dy);
                if world.can_place(data, kind, t, PLAYER).is_ok() {
                    world.place_building(data, kind, PLAYER, t, true);
                    return true;
                }
            }
        }
    }
    false
}

/// Baut die Vorführstadt in den freien Raum östlich des Bergfrieds.
pub fn city(world: &mut SimWorld, data: &GameData) {
    // Platz schaffen: Bäume im Baugebiet fällen.
    let (min, max) = (IVec2::new(15, 40), IVec2::new(50, 69));
    let trees: Vec<_> = world.nodes.iter().filter(|(_, n)| n.kind == ashen_sim::entity::NodeKind::Tree && n.tile.x >= min.x && n.tile.x <= max.x && n.tile.y >= min.y && n.tile.y <= max.y).map(|(id, n)| (id, n.tile)).collect();
    for (id, tile) in trees {
        world.nodes.remove(id);
        world.map.occupy(tile, IVec2::ONE, false);
    }
    // Weg und Gras glätten, damit Bauplätze nicht an Dekor scheitern.
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let t = IVec2::new(x, y);
            let i = world.map.idx(t);
            world.map.static_block[i] = false;
        }
    }
    let layout: [(&str, i32, i32); 30] = [
        ("storehouse", 19, 46),
        ("sawmill", 24, 52),
        ("brickworks", 29, 46),
        ("smelter", 33, 46),
        ("smithy", 36, 51),
        ("armory", 40, 46),
        ("market", 20, 56),
        ("temple", 25, 60),
        ("bakery", 30, 56),
        ("windmill", 33, 56),
        ("brewery", 37, 57),
        ("weaver", 41, 52),
        ("tailor", 41, 57),
        ("farm", 22, 62),
        ("sheep_farm", 29, 63),
        ("herb_garden", 34, 62),
        ("charcoal_burner", 38, 62),
        ("clay_pit", 44, 60),
        ("hut", 17, 61),
        ("hut", 17, 64),
        ("house", 20, 61),
        ("house", 20, 64),
        ("townhouse", 22, 65),
        ("townhouse", 27, 65),
        ("hut", 31, 65),
        ("woodcutter", 44, 46),
        ("iron_mine", 25, 45),
        ("storehouse", 34, 64),
        ("storehouse", 36, 49),
        ("house", 39, 64),
    ];
    for (id, x, y) in layout {
        put(world, data, id, IVec2::new(x, y));
    }
    // Diagonale Mauerlinie (Bresenham, wie im Baumodus gezogen) auf dem ersten freien Streifen.
    if let Some(wall) = data.building_id("wall") {
        'search: for (sx, sy) in [(40, 59), (16, 46), (27, 49), (44, 53), (18, 58), (36, 44), (28, 58)] {
            let line: Vec<IVec2> = (0..7).map(|i| IVec2::new(sx + i, sy + i)).collect();
            if line.iter().all(|t| world.can_place(data, wall, *t, PLAYER).is_ok()) {
                for t in line {
                    world.place_building(data, wall, PLAYER, t, true);
                }
                break 'search;
            }
        }
    }
    // Mauerring mit Toren und Ecktürmen.
    let (a, b) = (IVec2::new(15, 43), IVec2::new(48, 68));
    let (Some(wall), Some(gate), Some(tower)) = (data.building_id("wall"), data.building_id("gate"), data.building_id("wall_tower")) else { return };
    debug_assert_eq!(data.building(wall).fort, Some(Fort::Wall));
    for t in [IVec2::new(b.x - 1, a.y - 1), IVec2::new(b.x - 1, b.y - 1), IVec2::new(a.x - 1, b.y - 1), IVec2::new(a.x - 1, a.y - 1)] {
        if world.can_place(data, tower, t, PLAYER).is_ok() {
            world.place_building(data, tower, PLAYER, t, true);
        }
    }
    let gates = [IVec2::new(31, b.y), IVec2::new(a.x, 55), IVec2::new(b.x, 57)];
    for x in a.x..=b.x {
        for y in [a.y, b.y] {
            let t = IVec2::new(x, y);
            let kind = if gates.contains(&t) { gate } else { wall };
            if world.can_place(data, kind, t, PLAYER).is_ok() {
                world.place_building(data, kind, PLAYER, t, true);
            }
        }
    }
    for y in a.y + 1..b.y {
        for x in [a.x, b.x] {
            let t = IVec2::new(x, y);
            let kind = if gates.contains(&t) { gate } else { wall };
            if world.can_place(data, kind, t, PLAYER).is_ok() {
                world.place_building(data, kind, PLAYER, t, true);
            }
        }
    }
}
