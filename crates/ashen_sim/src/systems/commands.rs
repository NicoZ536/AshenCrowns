//! Anwendung von Spielerbefehlen auf den Simulationszustand.

use crate::command::{Command, PlayerCommand};
use crate::data::{AbilityTargeting, GameData, UnitClass};
use crate::rpg::Formation;
use crate::entity::*;
use crate::events::SimEvent;
use crate::map::TileMap;
use crate::systems::{abilities, city, economy, formation, heroes};
use crate::world::SimWorld;
use glam::Vec2;

/// Filtert Einheiten auf lebende Einheiten des Absenders.
fn owned(world: &SimWorld, player: PlayerId, units: &[UnitId]) -> Vec<UnitId> {
    units.iter().copied().filter(|&id| world.units.get(id).is_some_and(|u| u.owner == player && u.hp > 0.0)).collect()
}

/// Formationsplätze nach der Aufstellung der Gruppe (häufigste Formation der Einheiten):
/// Nahkämpfer vorne, Fernkämpfer, Unterstützer und Arbeiter hinten.
pub fn formation_slots(world: &SimWorld, data: &GameData, units: &[UnitId], target: Vec2) -> Vec<(UnitId, Vec2)> {
    if units.len() == 1 {
        return vec![(units[0], target)];
    }
    let mut sorted: Vec<(UnitId, u8, Vec2)> = units
        .iter()
        .filter_map(|&id| {
            let u = world.units.get(id)?;
            let def = data.unit(u.kind);
            let rank = match def.class {
                UnitClass::Cavalry => 0,
                UnitClass::Infantry | UnitClass::Monster | UnitClass::Boss | UnitClass::Flyer => 1,
                UnitClass::Hero => 2,
                UnitClass::Ranged | UnitClass::Siege => 3,
                UnitClass::Support => 4,
                UnitClass::Worker => 5,
            };
            Some((id, rank, u.pos))
        })
        .collect();
    sorted.sort_by_key(|(id, r, _)| (*r, id.index));
    let mut counts = [0usize; 5];
    for (id, _, _) in &sorted {
        if let Some(u) = world.units.get(*id) {
            counts[Formation::ALL.iter().position(|f| *f == u.formation).unwrap_or(0)] += 1;
        }
    }
    let best = (0..5).max_by_key(|&i| (counts[i], std::cmp::Reverse(i))).unwrap_or(0);
    let formation = Formation::ALL[best];
    let centroid = sorted.iter().map(|s| s.2).sum::<Vec2>() / sorted.len() as f32;
    let fwd = (target - centroid).try_normalize().unwrap_or(Vec2::new(1.0, 0.0));
    let right = Vec2::new(-fwd.y, fwd.x);
    let offs = formation::offsets(formation, sorted.len());
    sorted
        .iter()
        .enumerate()
        .map(|(i, (id, _, _))| {
            let mut p = target + right * offs[i].x - fwd * offs[i].y;
            let t = TileMap::tile_of(p);
            if !world.map.walkable(t)
                && let Some(w) = world.map.nearest_walkable(t, 4)
            {
                p = TileMap::center_of(w);
            }
            (*id, p)
        })
        .collect()
}

fn set_order(u: &mut Unit, order: Order, queue: bool) {
    if queue && u.order != Order::Idle {
        u.queue.push(order);
        return;
    }
    u.queue.clear();
    u.order = order;
    u.target = None;
    u.windup = None;
    match order {
        Order::Move { target, .. } => u.request_path(target),
        Order::Patrol { b, .. } => u.request_path(b),
        _ => u.clear_path(),
    }
}

pub fn apply(world: &mut SimWorld, data: &GameData, pc: PlayerCommand) {
    let player = pc.player;
    match pc.command {
        Command::Move { units, target, attack_move, queue } => {
            let units = owned(world, player, &units);
            for (id, slot) in formation_slots(world, data, &units, target) {
                if let Some(u) = world.units.get_mut(id) {
                    set_order(u, Order::Move { target: slot, attack_move }, queue);
                }
            }
        }
        Command::Attack { units, target } => {
            let Some(target_owner) = world.entity_owner(target) else { return };
            if !SimWorld::hostile(player, target_owner) {
                // Angriff auf eigene/neutrale Ziele wird als Bewegung interpretiert.
                if let Some(p) = world.entity_pos(target) {
                    apply(world, data, PlayerCommand { player, command: Command::Move { units, target: p, attack_move: false, queue: false } });
                }
                return;
            }
            let tgt_kind = match target {
                EntityRef::Unit(t) => world.units.get(t).map(|x| (t, x.kind)),
                _ => None,
            };
            for id in owned(world, player, &units) {
                if let Some(u) = world.units.get_mut(id) {
                    if data.unit(u.kind).damage_max <= 0.0 {
                        continue;
                    }
                    if let EntityRef::Unit(t) = target
                        && let Some(tu) = tgt_kind
                        && t == tu.0
                        && !crate::systems::units::can_hit(data.unit(u.kind), data.unit(tu.1))
                    {
                        continue;
                    }
                    set_order(u, Order::Attack { target }, false);
                }
            }
        }
        Command::Stop { units } => {
            for id in owned(world, player, &units) {
                if let Some(u) = world.units.get_mut(id) {
                    set_order(u, Order::Idle, false);
                    u.vel = Vec2::ZERO;
                }
            }
        }
        Command::Hold { units } => {
            for id in owned(world, player, &units) {
                if let Some(u) = world.units.get_mut(id) {
                    set_order(u, Order::Hold, false);
                }
            }
        }
        Command::Patrol { units, target } => {
            for id in owned(world, player, &units) {
                if let Some(u) = world.units.get_mut(id) {
                    let a = u.pos;
                    set_order(u, Order::Patrol { a, b: target, to_b: true }, false);
                }
            }
        }
        Command::Gather { units, node } => {
            if !world.nodes.contains(node) {
                return;
            }
            for id in owned(world, player, &units) {
                if let Some(u) = world.units.get_mut(id)
                    && data.unit(u.kind).carry > 0
                {
                    set_order(u, Order::Gather { node }, false);
                }
            }
        }
        Command::Build { units, kind, tile } => {
            let builders: Vec<UnitId> = owned(world, player, &units).into_iter().filter(|&id| world.units.get(id).is_some_and(|u| data.unit(u.kind).build_speed > 0.0)).collect();
            if builders.is_empty() {
                return;
            }
            let def = data.building(kind);
            if !def.buildable {
                return;
            }
            if let Err(e) = world.can_place(data, kind, tile, player) {
                world.error(player, e);
                return;
            }
            if let Err(it) = world.afford(player, &def.cost, &def.goods_cost_r) {
                world.error(player, city::missing_text(data, it));
                return;
            }
            world.pay(player, &def.cost, &def.goods_cost_r);
            let bid = world.place_building(data, kind, player, tile, false);
            for id in builders {
                if let Some(u) = world.units.get_mut(id) {
                    set_order(u, Order::Build { building: bid }, false);
                }
            }
        }
        Command::Repair { units, building } => {
            let ok = world.buildings.get(building).is_some_and(|b| b.owner == player && (!b.complete || b.hp < b.max_hp));
            if !ok {
                return;
            }
            for id in owned(world, player, &units) {
                if let Some(u) = world.units.get_mut(id)
                    && data.unit(u.kind).build_speed > 0.0
                {
                    set_order(u, Order::Build { building }, false);
                }
            }
        }
        Command::Train { building, unit } => {
            let Some(b) = world.buildings.get(building) else { return };
            if b.owner != player || !b.complete {
                return;
            }
            let bdef = data.building(b.kind);
            let udef = data.unit(unit);
            if !bdef.trains.contains(&udef.id) {
                return;
            }
            if player == PLAYER && udef.era > world.players[PLAYER as usize].era {
                let name = data.rules.eras.get(udef.era as usize - 1).map(|e| e.name.as_str()).unwrap_or("?");
                world.error(player, format!("{} erfordert die Ausbaustufe {name}.", udef.name));
                return;
            }
            if udef.class == UnitClass::Hero {
                let exists = world.units.iter().any(|(_, x)| x.kind == unit && x.owner == player)
                    || world.fallen.iter().any(|f| f.unit.kind == unit && f.unit.owner == player)
                    || world.buildings.iter().any(|(_, x)| x.owner == player && x.queue.iter().any(|q| q.unit == unit));
                if exists {
                    world.error(player, format!("{} steht Euch bereits zur Seite.", udef.name));
                    return;
                }
            }
            if b.queue.len() >= 5 {
                world.error(player, "Ausbildungswarteschlange voll.");
                return;
            }
            let ps = &world.players[player as usize];
            let queued: u32 = world.buildings.iter().filter(|(_, x)| x.owner == player).flat_map(|(_, x)| x.queue.iter()).map(|q| data.unit(q.unit).pop).sum();
            if ps.pop + queued + udef.pop > ps.pop_cap.min(data.rules.max_population) {
                world.error(player, "Nicht genug Wohnraum – baut weitere Gehöfte.");
                return;
            }
            if let Err(it) = world.afford(player, &udef.cost, &udef.goods_cost_r) {
                world.error(player, city::missing_text(data, it));
                return;
            }
            world.pay(player, &udef.cost, &udef.goods_cost_r);
            if let Some(b) = world.buildings.get_mut(building) {
                b.queue.push(TrainItem { unit, progress: 0.0 });
            }
        }
        Command::CancelTrain { building, index } => {
            let Some(b) = world.buildings.get_mut(building) else { return };
            if b.owner != player || index >= b.queue.len() {
                return;
            }
            let item = b.queue.remove(index);
            let u = data.unit(item.unit);
            world.refund(player, &u.cost, &u.goods_cost_r, 1.0);
        }
        Command::SetRally { building, point } => {
            if let Some(b) = world.buildings.get_mut(building)
                && b.owner == player
            {
                b.rally = Some(point);
            }
        }
        Command::UseAbility { unit, ability, point } => {
            let Some(u) = world.units.get(unit) else { return };
            if u.owner != player {
                return;
            }
            let Some(adef) = abilities::ability_at(data, u, ability) else { return };
            if u.ability_cd.get(ability as usize).copied().unwrap_or(0.0) > 0.0 {
                world.error(player, format!("{} ist noch nicht bereit.", adef.name));
                return;
            }
            if u.mana < adef.mana {
                world.error(player, "Nicht genug Mana.");
                return;
            }
            match adef.targeting {
                AbilityTargeting::SelfCentered => {
                    let p = u.pos;
                    abilities::cast(world, data, unit, ability, p);
                }
                AbilityTargeting::Point => {
                    if (u.pos - point).length() <= adef.range {
                        abilities::cast(world, data, unit, ability, point);
                    } else if let Some(u) = world.units.get_mut(unit) {
                        set_order(u, Order::Cast { ability, point }, false);
                        u.request_path(point);
                    }
                }
            }
        }
        Command::BuildLine { units, kind, tiles } => build_line(world, data, player, &units, kind, &tiles),
        Command::SetWorkers { building, limit } => {
            if let Some(b) = world.buildings.get_mut(building)
                && b.owner == player
            {
                b.worker_limit = limit;
            }
        }
        Command::SetRecipe { building, recipe } => {
            if let Some(b) = world.buildings.get_mut(building)
                && b.owner == player
                && (recipe as usize) < data.building(b.kind).recipes.len()
                && b.recipe != recipe
            {
                // Ein laufender Zyklus ist verloren; die Eingaben werden zurückerstattet.
                let old = &data.building(b.kind).recipes[b.recipe as usize];
                let refund = if b.cycle_active { old.inputs_r.clone() } else { Vec::new() };
                b.recipe = recipe;
                b.cycle_active = false;
                b.cycle = 0.0;
                for (it, n) in refund {
                    *world.players[player as usize].stock_mut(it) += n;
                }
            }
        }
        Command::SetGate { building, open } => {
            if world.buildings.get(building).is_some_and(|b| b.owner == player) {
                city::set_gate(world, data, building, open);
            }
        }
        Command::SetTax { level } => {
            if (level as usize) < data.rules.city.tax_levels.len() {
                world.players[player as usize].tax_level = level;
            }
        }
        Command::Trade { item, amount } => city::trade(world, data, player, item, amount),
        Command::AdvanceEra => city::advance_era(world, data, player),
        Command::SetFormation { units, formation } => {
            for id in owned(world, player, &units) {
                if let Some(u) = world.units.get_mut(id)
                    && !matches!(data.unit(u.kind).class, UnitClass::Worker)
                {
                    u.formation = formation;
                }
            }
        }
        Command::LearnTalent { unit, talent } => heroes::learn_talent(world, data, player, unit, talent as usize),
        Command::EquipItem { unit, index } => heroes::equip(world, data, player, unit, index as usize),
        Command::UnequipItem { unit, slot } => heroes::unequip(world, data, player, unit, slot as usize),
        Command::DropItem { unit, index } => heroes::drop_item(world, data, player, unit, index as usize),
        Command::UseItem { unit, index } => heroes::use_item(world, data, player, unit, index as usize),
        Command::Research { building, upgrade } => economy::start_research(world, data, player, building, upgrade as usize),
        Command::CancelResearch { building } => economy::cancel_research(world, data, player, building),
        Command::ReviveHero { building, hero } => heroes::revive(world, data, player, building, hero),
        Command::Demolish { building } => {
            let Some(b) = world.buildings.get_mut(building) else { return };
            if b.owner != player || b.tag.as_deref() == Some(world.mission_def.defeat_tag.as_str()) {
                return;
            }
            let def = data.building(b.kind);
            let refund = if b.complete { data.rules.demolish_refund } else { 0.75 };
            let queue = std::mem::take(&mut b.queue);
            b.hp = 0.0;
            b.invulnerable = false;
            for item in queue {
                let u = data.unit(item.unit);
                world.refund(player, &u.cost, &u.goods_cost_r, 1.0);
            }
            world.refund(player, &def.cost, &def.goods_cost_r, refund);
            world.events.push(SimEvent::Message { speaker: String::new(), text: format!("{} abgerissen.", def.name), voice: None });
        }
    }
}

/// Platziert mehrere Bauteile (Mauerlinie) und verteilt die Bauarbeiter reihum darauf.
fn build_line(world: &mut SimWorld, data: &GameData, player: PlayerId, units: &[UnitId], kind: crate::data::BuildingTypeId, tiles: &[glam::IVec2]) {
    let builders: Vec<UnitId> = owned(world, player, units).into_iter().filter(|&id| world.units.get(id).is_some_and(|u| data.unit(u.kind).build_speed > 0.0)).collect();
    if builders.is_empty() {
        return;
    }
    let def = data.building(kind);
    if !def.buildable {
        return;
    }
    let mut placed = 0usize;
    let mut first_error: Option<String> = None;
    for &tile in tiles.iter().take(80) {
        if let Err(e) = world.can_place(data, kind, tile, player) {
            first_error.get_or_insert(e);
            continue;
        }
        if let Err(it) = world.afford(player, &def.cost, &def.goods_cost_r) {
            first_error.get_or_insert(city::missing_text(data, it));
            break;
        }
        world.pay(player, &def.cost, &def.goods_cost_r);
        let bid = world.place_building(data, kind, player, tile, false);
        let uid = builders[placed % builders.len()];
        if let Some(u) = world.units.get_mut(uid) {
            // Der erste Auftrag ersetzt den bisherigen, weitere reihen sich in die Warteschlange.
            set_order(u, Order::Build { building: bid }, placed >= builders.len());
        }
        placed += 1;
    }
    if placed == 0
        && let Some(e) = first_error
    {
        world.error(player, e);
    }
}
