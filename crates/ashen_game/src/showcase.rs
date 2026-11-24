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

// ---------------------------------------------------------------------------------------------
// Militär-Vorführungen (`--demo army|siege|hero`)
// ---------------------------------------------------------------------------------------------

use ashen_sim::entity::{EntityRef, Order, UnitId, ENEMY};
use glam::Vec2;

/// Gibt ein Rechteck frei (Bäume fällen, Dekor entfernen), damit Vorführungen nicht an Gelände scheitern.
fn clear_area(world: &mut SimWorld, min: IVec2, max: IVec2) {
    let trees: Vec<_> = world.nodes.iter().filter(|(_, n)| n.kind == ashen_sim::entity::NodeKind::Tree && n.tile.x >= min.x && n.tile.x <= max.x && n.tile.y >= min.y && n.tile.y <= max.y).map(|(id, n)| (id, n.tile)).collect();
    for (id, tile) in trees {
        world.nodes.remove(id);
        world.map.occupy(tile, IVec2::ONE, false);
    }
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let i = world.map.idx(IVec2::new(x, y));
            world.map.static_block[i] = false;
        }
    }
}

fn put_for(world: &mut SimWorld, data: &GameData, id: &str, at: IVec2, owner: u8) {
    let Some(kind) = data.building_id(id) else { return };
    for r in 0..4i32 {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue;
                }
                let t = at + IVec2::new(dx, dy);
                if world.can_place(data, kind, t, owner).is_ok() {
                    world.place_building(data, kind, owner, t, true);
                    return;
                }
            }
        }
    }
}

fn troop(world: &mut SimWorld, data: &GameData, id: &str, owner: u8, p: Vec2) -> Option<UnitId> {
    let kind = data.unit_id(id)?;
    Some(world.spawn_unit(data, kind, owner, p))
}

/// Einheiten je Seite für den Lasttest `--bench stress` (0 = aus). Wird von `main` gesetzt, von `army` gelesen.
static STRESS_UNITS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn set_stress_units(n: usize) {
    STRESS_UNITS.store(n, std::sync::atomic::Ordering::Relaxed);
}

/// Alle neuen Militärgebäude und Einheiten in Reihen (Kontrolle von Modellen und Farben).
/// Ist ein Lasttest gesetzt (`set_stress_units`), entsteht stattdessen eine große Schlacht in der Kartenmitte.
pub fn army(world: &mut SimWorld, data: &GameData) {
    let stress = STRESS_UNITS.load(std::sync::atomic::Ordering::Relaxed);
    if stress > 0 {
        world.map.static_block.iter_mut().for_each(|b| *b = false);
        ashen_sim::bench::populate_battle(world, data, stress);
        return;
    }
    clear_area(world, IVec2::new(12, 24), IVec2::new(40, 56));
    world.players[PLAYER as usize].era = data.rules.eras.len() as u8;
    for (i, id) in ["shooting_range", "stable", "chapel", "mage_tower", "siege_workshop", "altar"].iter().enumerate() {
        put_for(world, data, id, IVec2::new(14 + (i as i32 % 3) * 5, 27 + (i as i32 / 3) * 5), PLAYER);
    }
    for (i, id) in ["bone_pit", "ghoul_den", "ash_altar", "grave_forge", "bone_cache", "bone_spire"].iter().enumerate() {
        put_for(world, data, id, IVec2::new(29 + (i as i32 % 2) * 5, 26 + (i as i32 / 2) * 4), ENEMY);
    }
    let humans = ["swordsman", "spearman", "archer", "crossbowman", "halberdier", "shieldbearer", "scout_rider", "horse_archer", "knight", "battle_mage", "priest", "battering_ram", "catapult"];
    for (i, id) in humans.iter().enumerate() {
        troop(world, data, id, PLAYER, Vec2::new(14.0 + (i % 7) as f32 * 2.4, 40.0 + (i / 7) as f32 * 2.6));
    }
    let hollow = ["hollow_thrall", "hollow_archer", "hollow_digger", "ghoul", "ash_witch", "grave_brute", "bat_swarm"];
    for (i, id) in hollow.iter().enumerate() {
        troop(world, data, id, ENEMY, Vec2::new(14.0 + i as f32 * 2.4, 47.0));
    }
    for (i, id) in ["hero_hal_venn", "hero_torvald"].iter().enumerate() {
        troop(world, data, id, PLAYER, Vec2::new(32.0 + i as f32 * 2.4, 43.0));
    }
    // Feinde halten still: der Kontaktbogen zeigt alle Modelle in Ruhe.
    let ids: Vec<UnitId> = world.units.iter().filter(|(_, u)| u.owner == ENEMY).map(|(id, _)| id).collect();
    for id in ids {
        if let Some(u) = world.units.get_mut(id) {
            u.order = Order::Hold;
        }
    }
}

/// Belagerungsschlacht: Katapulte und Rammböcke gegen eine befestigte Hohlen-Basis.
pub fn siege(world: &mut SimWorld, data: &GameData) {
    clear_area(world, IVec2::new(8, 22), IVec2::new(44, 48));
    world.players[PLAYER as usize].era = data.rules.eras.len() as u8;
    // Gegnerische Basis mit Mauer, Tor und Türmen.
    for (id, x, y) in [("grave_forge", 35, 27), ("bone_pit", 32, 33), ("ghoul_den", 36, 37), ("ash_altar", 31, 38), ("bone_spire", 27, 26), ("bone_spire", 27, 40), ("bone_cache", 39, 33)] {
        put_for(world, data, id, IVec2::new(x, y), ENEMY);
    }
    if let (Some(wall), Some(gate)) = (data.building_id("wall"), data.building_id("gate")) {
        for y in 26..=41 {
            let kind = if y == 33 || y == 34 { gate } else { wall };
            let t = IVec2::new(28, y);
            if world.can_place(data, kind, t, ENEMY).is_ok() {
                world.place_building(data, kind, ENEMY, t, true);
            }
        }
    }
    let mut defenders = Vec::new();
    for (id, n, x, y) in [("hollow_thrall", 6, 30.5, 31.0), ("hollow_archer", 5, 30.0, 35.5), ("ghoul", 3, 32.5, 29.5), ("ash_witch", 2, 33.0, 36.0), ("grave_brute", 1, 31.0, 33.5), ("bat_swarm", 2, 30.5, 38.5)] {
        for i in 0..n {
            if let Some(u) = troop(world, data, id, ENEMY, Vec2::new(x + (i % 3) as f32 * 0.9, y + (i / 3) as f32 * 0.9)) {
                defenders.push(u);
            }
        }
    }
    for u in defenders {
        if let Some(unit) = world.units.get_mut(u) {
            unit.order = Order::Guard { post: unit.pos, radius: 7.0 };
        }
    }
    // Angreifer der Glutkrone.
    let wall_target = world.building_at(IVec2::new(28, 30));
    let gate_target = world.building_at(IVec2::new(28, 33));
    let mut sieges = Vec::new();
    for (id, n, x, y) in [("catapult", 3, 16.0, 29.5), ("battering_ram", 2, 21.0, 32.0)] {
        for i in 0..n {
            if let Some(u) = troop(world, data, id, PLAYER, Vec2::new(x, y + i as f32 * 2.2)) {
                sieges.push((id, u));
            }
        }
    }
    for (id, u) in sieges {
        let target = if id == "catapult" { wall_target } else { gate_target };
        if let (Some(t), Some(unit)) = (target, world.units.get_mut(u)) {
            unit.order = Order::Attack { target: EntityRef::Building(t) };
        }
    }
    let mut escort = Vec::new();
    for (id, n, x, y) in [("shieldbearer", 4, 23.5, 29.5), ("halberdier", 4, 23.0, 33.5), ("crossbowman", 4, 19.0, 28.5), ("battle_mage", 2, 19.5, 36.0), ("priest", 2, 18.0, 33.0), ("knight", 3, 21.0, 38.0), ("archer", 3, 18.5, 31.0)] {
        for i in 0..n {
            if let Some(u) = troop(world, data, id, PLAYER, Vec2::new(x + (i % 2) as f32 * 0.9, y + (i / 2) as f32 * 0.9)) {
                escort.push(u);
            }
        }
    }
    for u in escort {
        if let Some(unit) = world.units.get_mut(u) {
            unit.order = Order::Move { target: Vec2::new(29.5, 33.5), attack_move: true };
            unit.request_path(Vec2::new(29.5, 33.5));
        }
    }
    if let Some(h) = world.hero_of(data, PLAYER)
        && let Some(u) = world.units.get_mut(h)
    {
        u.pos = Vec2::new(20.0, 34.5);
        u.prev_pos = u.pos;
        u.order = Order::Move { target: Vec2::new(29.5, 34.0), attack_move: true };
        u.request_path(Vec2::new(29.5, 34.0));
    }
}

/// Heldenfenster-Demo: Isolde mit Stufe 7, Talenten, Ausrüstung und gefülltem Inventar; zwei weitere Helden.
pub fn hero(world: &mut SimWorld, data: &GameData) {
    let Some(h) = world.hero_of(data, PLAYER) else { return };
    ashen_sim::systems::combat::grant_xp(world, data, h, 1900);
    for id in ["embersteel_blade", "ember_plate", "flamekeeper_pendant", "ember_ring", "iron_band", "wolf_signet", "potion_heal", "potion_heal", "potion_mana", "potion_ember", "chain_hauberk", "edran_blade"] {
        if let Some(item) = data.item_def_id(id) {
            ashen_sim::systems::heroes::give_item(world, h, item);
        }
    }
    for _ in 0..4 {
        world.push_command(ashen_sim::PlayerCommand { player: PLAYER, command: ashen_sim::Command::EquipItem { unit: h, index: 0 } });
    }
    let tree = data.talent_tree("hero_isolde").cloned();
    if let Some(tree) = tree {
        for id in ["edge_of_embers", "edge_of_embers", "searing_strikes", "steadfast_banner", "ashen_plate", "ashen_plate"] {
            if let Some(i) = tree.talents.iter().position(|t| t.id == id) {
                world.push_command(ashen_sim::PlayerCommand { player: PLAYER, command: ashen_sim::Command::LearnTalent { unit: h, talent: i as u16 } });
            }
        }
    }
    let pos = world.units.get(h).map(|u| u.pos).unwrap_or(Vec2::new(18.0, 54.0));
    for (i, id) in ["hero_hal_venn", "hero_torvald"].iter().enumerate() {
        troop(world, data, id, PLAYER, pos + Vec2::new(1.6 + i as f32 * 1.4, 1.2));
    }
    // Beute am Boden neben den Helden.
    for (i, id) in ["varkhan_signet", "potion_heal_greater", "grauwall_aegis"].iter().enumerate() {
        if let Some(item) = data.item_def_id(id) {
            world.drops.push(ashen_sim::entity::GroundItem { pos: pos + Vec2::new(-1.8 + i as f32 * 1.2, 2.4), item });
        }
    }
}
