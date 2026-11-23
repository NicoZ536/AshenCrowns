//! Wirtschaft: Einkommen, Unterhalt (Nahrung), Ausbildung, Bevölkerung, Heldenrückkehr.

use crate::chronicle::ChronicleKind;
use crate::data::{GameData, ResourceKind, UnitClass};
use crate::entity::*;
use crate::events::SimEvent;
use crate::map::TileMap;
use crate::world::SimWorld;
use glam::{IVec2, Vec2};

/// Bevölkerung (lebende Einheiten) und Wohnraum (fertige Gebäude) neu berechnen.
pub fn recompute_population(world: &mut SimWorld, data: &GameData) {
    for p in &mut world.players {
        p.pop = 0;
    }
    world.players[PLAYER as usize].pop_cap = 0;
    for (_, u) in world.units.iter() {
        world.players[u.owner as usize].pop += data.unit(u.kind).pop;
    }
    for (_, b) in world.buildings.iter() {
        if b.complete && b.owner == PLAYER {
            world.players[PLAYER as usize].pop_cap += data.building(b.kind).pop_provided;
        }
    }
    let cap = data.rules.max_population;
    let era = world.players[PLAYER as usize].era as usize;
    let bonus: u32 = data.rules.eras.iter().take(era).map(|e| e.pop_bonus).sum();
    let p = &mut world.players[PLAYER as usize];
    p.pop_cap = (p.pop_cap + bonus).min(cap);
}

/// Austrittspunkt eines Gebäudes (vordere, dem Betrachter zugewandte Ecke).
pub fn exit_point(world: &SimWorld, b: &Building, toward: Option<Vec2>) -> Vec2 {
    let front = b.tile + b.size;
    let candidates = [front, IVec2::new(b.tile.x + b.size.x / 2, front.y), IVec2::new(front.x, b.tile.y + b.size.y / 2)];
    let mut best = None;
    for c in candidates {
        if let Some(t) = world.map.nearest_walkable(c, 4) {
            let p = TileMap::center_of(t);
            let score = toward.map(|tw| (tw - p).length()).unwrap_or(0.0);
            if best.is_none_or(|(s, _)| score < s) {
                best = Some((score, p));
            }
        }
    }
    best.map(|b| b.1).unwrap_or(b.center())
}

pub fn tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    research_tick(world, data, dt);
    // Passives Einkommen (Nahrung unterliegt der Lagerkapazität).
    let mut food_income = [0.0f32; 3];
    for (_, b) in world.buildings.iter() {
        if !b.complete {
            continue;
        }
        let def = data.building(b.kind);
        food_income[b.owner as usize] += def.food_per_sec * dt;
        if def.gold_per_sec > 0.0 {
            let p = &mut world.players[b.owner as usize];
            p.resources.gold += def.gold_per_sec * dt;
            p.stats.gathered.gold += def.gold_per_sec * dt;
        }
    }
    for (owner, f) in food_income.iter().enumerate() {
        if *f > 0.0 {
            let got = world.credit(owner as PlayerId, crate::data::Item::Res(ResourceKind::Food), *f);
            world.players[owner].stats.gathered.food += got;
        }
    }

    // Unterhalt: Nahrung (entfällt in Missionen mit `free_economy`).
    let upkeep: f32 = if world.mission_def.free_economy { 0.0 } else { world.units.iter().filter(|(_, u)| u.owner == PLAYER).map(|(_, u)| data.unit(u.kind).upkeep).sum() };
    let free = world.mission_def.free_economy;
    let p = &mut world.players[PLAYER as usize];
    p.resources.food -= upkeep * dt;
    if free {
        if p.starving {
            p.starving = false;
            world.events.push(SimEvent::Starvation { owner: PLAYER, active: false });
        }
    } else if p.resources.food <= 0.0 {
        p.resources.food = 0.0;
        if !p.starving {
            p.starving = true;
            world.events.push(SimEvent::Starvation { owner: PLAYER, active: true });
            world.record(ChronicleKind::Omen, "Die Kornspeicher von Brennhollow waren leer. Hunger ging um.");
        }
    } else if p.starving && p.resources.food > 10.0 {
        p.starving = false;
        world.events.push(SimEvent::Starvation { owner: PLAYER, active: false });
    }

    // Ausbildung.
    let starving = world.players[PLAYER as usize].starving;
    let bids = world.buildings.ids();
    for bid in bids {
        let Some(b) = world.buildings.get_mut(bid) else { continue };
        if !b.complete || b.queue.is_empty() || (b.owner == PLAYER && starving) {
            continue;
        }
        let item = &mut b.queue[0];
        let def = data.unit(item.unit);
        item.progress += dt / def.train_time;
        if item.progress < 1.0 {
            continue;
        }
        let kind = item.unit;
        b.queue.remove(0);
        let owner = b.owner;
        let rally = b.rally;
        let b = world.buildings.get(bid).expect("exists").clone();
        let exit = exit_point(world, &b, rally);
        let uid = world.spawn_unit(data, kind, owner, exit);
        world.players[owner as usize].stats.units_trained += 1;
        world.events.push(SimEvent::TrainCompleted { unit: uid, kind, owner, pos: exit });
        if let Some(r) = rally {
            let is_worker = data.unit(kind).class == UnitClass::Worker;
            let node = if is_worker {
                world.nodes.iter().filter(|(_, n)| n.distance_to(r) < 1.5).map(|(id, _)| id).next()
            } else {
                None
            };
            if let Some(u) = world.units.get_mut(uid) {
                match node {
                    Some(n) => u.order = Order::Gather { node: n },
                    None => {
                        u.order = Order::Move { target: r, attack_move: false };
                        u.request_path(r);
                    }
                }
            }
        }
        recompute_population(world, data);
    }
}

/// Startet ein Schmiede-Upgrade im Gebäude `building`.
pub fn start_research(world: &mut SimWorld, data: &GameData, player: PlayerId, building: BuildingId, upgrade: usize) {
    let Some(up) = data.upgrades.get(upgrade) else { return };
    let Some(b) = world.buildings.get(building) else { return };
    if b.owner != player || !b.complete || data.building(b.kind).id != up.building {
        return;
    }
    if b.research.is_some() {
        world.error(player, "Hier läuft bereits eine Forschung.");
        return;
    }
    let level = world.players[player as usize].upgrades.get(upgrade).copied().unwrap_or(0) as usize;
    let Some(next) = up.levels.get(level) else {
        world.error(player, "Diese Verbesserung ist bereits auf der höchsten Stufe.");
        return;
    };
    if next.era > world.players[player as usize].era {
        let name = data.rules.eras.get(next.era as usize - 1).map(|e| e.name.as_str()).unwrap_or("?");
        world.error(player, format!("Benötigt Ausbaustufe: {name}"));
        return;
    }
    if let Err(it) = world.afford(player, &next.cost, &next.goods_r) {
        world.error(player, crate::systems::city::missing_text(data, it));
        return;
    }
    world.pay(player, &next.cost, &next.goods_r);
    if let Some(b) = world.buildings.get_mut(building) {
        b.research = Some((upgrade as u8, 0.0));
    }
}

/// Bricht eine Forschung ab und erstattet die Kosten.
pub fn cancel_research(world: &mut SimWorld, data: &GameData, player: PlayerId, building: BuildingId) {
    let Some(b) = world.buildings.get_mut(building) else { return };
    if b.owner != player {
        return;
    }
    let Some((up, _)) = b.research.take() else { return };
    let level = world.players[player as usize].upgrades.get(up as usize).copied().unwrap_or(0) as usize;
    if let Some(l) = data.upgrades.get(up as usize).and_then(|u| u.levels.get(level)) {
        world.refund(player, &l.cost, &l.goods_r, 1.0);
    }
}

/// Fortschritt laufender Forschungen.
pub fn research_tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    let bids = world.buildings.ids();
    for bid in bids {
        let Some(b) = world.buildings.get_mut(bid) else { continue };
        let Some((up, prog)) = b.research else { continue };
        if !b.complete {
            continue;
        }
        let owner = b.owner;
        let level = world.players[owner as usize].upgrades.get(up as usize).copied().unwrap_or(0) as usize;
        let need = data.upgrades.get(up as usize).and_then(|u| u.levels.get(level)).map(|l| l.time).unwrap_or(0.0);
        let prog = prog + dt;
        let b = world.buildings.get_mut(bid).expect("exists");
        if prog < need {
            b.research = Some((up, prog));
            continue;
        }
        b.research = None;
        if let Some(l) = world.players[owner as usize].upgrades.get_mut(up as usize) {
            *l += 1;
        }
        let lv = world.players[owner as usize].upgrades[up as usize];
        let name = data.upgrades[up as usize].name.clone();
        world.events.push(SimEvent::UpgradeResearched { owner, upgrade: data.upgrades[up as usize].id.clone(), level: lv });
        if owner == PLAYER {
            world.record(ChronicleKind::Construction, format!("{name} (Stufe {lv}) wurde erforscht."));
            world.events.push(SimEvent::Message { speaker: String::new(), text: format!("{name}: Stufe {lv} erforscht."), voice: None });
        }
        // Bereits lebende Einheiten profitieren von zusätzlichen Lebenspunkten (Anteil bleibt gleich).
        let _ = data;
    }
}
