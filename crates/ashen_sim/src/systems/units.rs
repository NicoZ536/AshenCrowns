//! Befehlslogik der Einheiten (Zustandsautomat je Befehl) und Statuswerte.

use crate::data::{GameData, UnitClass};
use crate::entity::*;
use crate::events::SimEvent;
use crate::map::TileMap;
use crate::systems::abilities;
use crate::world::SimWorld;
use glam::Vec2;
use std::collections::HashMap;

/// Ergebnis eines Kampfkontakts.
#[derive(PartialEq, Eq, Debug)]
pub enum Engage {
    InRange,
    Chasing,
    Lost,
}

/// Sucht das nächste feindliche Ziel. Einheiten werden Gebäuden vorgezogen.
pub fn acquire(world: &mut SimWorld, data: &GameData, id: UnitId, radius: f32, include_buildings: bool) -> Option<EntityRef> {
    let u = world.units.get(id)?;
    let (pos, owner) = (u.pos, u.owner);
    let mut found = std::mem::take(&mut world.scratch);
    world.spatial.query(pos, radius, &mut found);
    let mut best: Option<(f32, UnitId)> = None;
    for &o in &found {
        let Some(other) = world.units.get(o) else { continue };
        if other.hp <= 0.0 || !SimWorld::hostile(owner, other.owner) {
            continue;
        }
        // Der Spieler sieht nur Ziele, die nicht im Kriegsnebel liegen.
        if owner == PLAYER && !world.fog.visible(other.pos) {
            continue;
        }
        let mut d = (other.pos - pos).length();
        // Arbeiter sind weniger attraktive Ziele als Kämpfer.
        if data.unit(other.kind).class == UnitClass::Worker {
            d += 2.5;
        }
        if best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, o));
        }
    }
    world.scratch = found;
    if let Some((_, o)) = best {
        return Some(EntityRef::Unit(o));
    }
    if include_buildings {
        let mut bb: Option<(f32, BuildingId)> = None;
        for (bid, b) in world.buildings.iter() {
            if !SimWorld::hostile(owner, b.owner) || b.hp <= 0.0 || b.invulnerable || data.building(b.kind).is_fort() {
                continue;
            }
            let d = b.distance_to(pos);
            if d <= radius && bb.is_none_or(|(bd, _)| d < bd) {
                bb = Some((d, bid));
            }
        }
        if let Some((_, b)) = bb {
            return Some(EntityRef::Building(b));
        }
    }
    None
}

/// Bewegt sich zum Ziel oder beginnt den Angriff, wenn es in Reichweite ist.
pub fn engage(world: &mut SimWorld, data: &GameData, id: UnitId, target: EntityRef, can_move: bool) -> Engage {
    if !world.entity_alive(target) {
        return Engage::Lost;
    }
    let Some(tpos) = world.entity_pos(target) else { return Engage::Lost };
    let Some(u) = world.units.get(id) else { return Engage::Lost };
    let def = data.unit(u.kind);
    let dist = world.entity_distance(data, target, u.pos).unwrap_or(f32::MAX) - def.radius;
    let slack = if matches!(target, EntityRef::Building(_)) { 0.35 } else { 0.1 };
    let u = world.units.get_mut(id).expect("unit exists");
    u.target = Some(target);
    if dist <= def.range + slack {
        u.clear_path();
        u.vel = Vec2::ZERO;
        let to = tpos - u.pos;
        u.face(to);
        if u.windup.is_none() && u.attack_cd <= 0.0 {
            u.windup = Some((def.windup, target));
            u.set_activity(Activity::Attack);
        } else if u.windup.is_none() && u.activity != Activity::Attack {
            u.set_activity(Activity::Idle);
        }
        Engage::InRange
    } else {
        if !can_move {
            return Engage::Lost;
        }
        u.windup = None;
        let goal_moved = u.path_goal.is_none_or(|g| (g - tpos).length() > 1.2);
        if (u.path.is_empty() && !u.path_pending) || (u.repath_timer <= 0.0 && goal_moved) {
            u.request_path(tpos);
            u.repath_timer = 0.7;
        }
        u.set_activity(Activity::Walk);
        Engage::Chasing
    }
}

fn next_order(u: &mut Unit) {
    u.target = None;
    u.windup = None;
    if u.queue.is_empty() {
        u.order = Order::Idle;
        u.clear_path();
        u.set_activity(Activity::Idle);
    } else {
        u.order = u.queue.remove(0);
        if let Order::Move { target, .. } = u.order {
            u.request_path(target);
        }
    }
}

fn move_toward(u: &mut Unit, goal: Vec2) {
    let need = u.path_goal.is_none_or(|g| (g - goal).length() > 0.5);
    if need || (u.path.is_empty() && !u.path_pending) {
        u.request_path(goal);
    }
    u.set_activity(Activity::Walk);
}

/// Pro-Tick-Logik aller Einheiten.
pub fn tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    // Anzahl der Bauarbeiter je Baustelle (für abnehmenden Grenznutzen).
    let mut builders: HashMap<BuildingId, u32> = HashMap::new();
    for (_, u) in world.units.iter() {
        if let Order::Build { building } = u.order
            && u.activity == Activity::Work
        {
            *builders.entry(building).or_default() += 1;
        }
    }

    let ids = world.units.ids();
    for id in ids {
        let Some(u) = world.units.get_mut(id) else { continue };
        if u.hp <= 0.0 {
            continue;
        }
        u.repath_timer -= dt;
        // Fliehende Einheiten ignorieren Befehle.
        if u.routing > 0.0 {
            flee(world, data, id);
            continue;
        }
        let order = u.order;
        let def = data.unit(u.kind);
        let can_fight = def.damage_max > 0.0;
        let cur_target = u.target;
        match order {
            Order::Idle => {
                if can_fight && def.class != UnitClass::Worker {
                    let radius = if u.owner == PLAYER { def.sight * 0.75 } else { def.sight };
                    let current = cur_target.filter(|t| world.entity_alive(*t));
                    let t = current.or_else(|| acquire(world, data, id, radius, true));
                    match t {
                        Some(t) => {
                            if engage(world, data, id, t, true) == Engage::Lost
                                && let Some(u) = world.units.get_mut(id)
                            {
                                u.target = None;
                            }
                        }
                        None => {
                            if let Some(u) = world.units.get_mut(id) {
                                u.target = None;
                                if !u.is_moving() {
                                    u.set_activity(Activity::Idle);
                                }
                            }
                        }
                    }
                } else if !u.is_moving() {
                    u.set_activity(Activity::Idle);
                }
            }
            Order::Move { target, attack_move } => {
                if attack_move && can_fight {
                    let current = cur_target.filter(|t| world.entity_alive(*t));
                    let sight = def.sight;
                    // Eine Belagerungsmauer als Ziel weicht Feinden in Sichtweite.
                    let siege = matches!(current, Some(EntityRef::Building(b)) if world.buildings.get(b).is_some_and(|x| data.building(x.kind).is_fort()));
                    let chosen = if siege { acquire(world, data, id, sight, false).or(current) } else { current.or_else(|| acquire(world, data, id, sight, true)) };
                    if let Some(t) = chosen {
                        if engage(world, data, id, t, true) != Engage::Lost {
                            continue;
                        }
                        if let Some(u) = world.units.get_mut(id) {
                            u.target = None;
                            u.request_path(target);
                        }
                    }
                }
                let Some(u) = world.units.get_mut(id) else { continue };
                if u.target.is_some() {
                    u.target = None;
                    u.request_path(target);
                }
                if !u.is_moving() {
                    next_order(u);
                } else {
                    u.set_activity(Activity::Walk);
                }
            }
            Order::Attack { target } => {
                if engage(world, data, id, target, true) == Engage::Lost
                    && let Some(u) = world.units.get_mut(id)
                {
                    next_order(u);
                }
            }
            Order::Hold => {
                let range = def.range + def.radius + 0.6;
                let current = cur_target.filter(|t| world.entity_alive(*t));
                match current.or_else(|| acquire(world, data, id, range, true)) {
                    Some(t) => {
                        if engage(world, data, id, t, false) == Engage::Lost
                            && let Some(u) = world.units.get_mut(id)
                        {
                            u.target = None;
                        }
                    }
                    None => {
                        if let Some(u) = world.units.get_mut(id) {
                            u.set_activity(Activity::Idle);
                        }
                    }
                }
            }
            Order::Patrol { a, b, to_b } => {
                let sight = def.sight;
                let current = cur_target.filter(|t| world.entity_alive(*t));
                if let Some(t) = current.or_else(|| acquire(world, data, id, sight, true))
                    && engage(world, data, id, t, true) != Engage::Lost
                {
                    continue;
                }
                let Some(u) = world.units.get_mut(id) else { continue };
                let goal = if to_b { b } else { a };
                if u.target.take().is_some() {
                    u.request_path(goal);
                }
                if !u.is_moving() {
                    if (u.pos - goal).length() < 1.5 {
                        u.order = Order::Patrol { a, b, to_b: !to_b };
                        u.request_path(if to_b { a } else { b });
                    } else {
                        u.request_path(goal);
                    }
                }
                u.set_activity(Activity::Walk);
            }
            Order::Guard { post, radius } => {
                if (u.pos - post).length() > radius + 5.0 {
                    u.target = None;
                    move_toward(u, post);
                    continue;
                }
                let current = cur_target.filter(|t| world.entity_alive(*t));
                let t = current.or_else(|| acquire(world, data, id, radius + 1.0, true));
                if let Some(t) = t
                    && world.entity_pos(t).is_some_and(|p| (p - post).length() <= radius + 4.0)
                    && engage(world, data, id, t, true) != Engage::Lost
                {
                    continue;
                }
                let Some(u) = world.units.get_mut(id) else { continue };
                u.target = None;
                if (u.pos - post).length() > 1.0 {
                    move_toward(u, post);
                } else if !u.is_moving() {
                    u.set_activity(Activity::Idle);
                }
            }
            Order::Cast { ability, point } => {
                let range = def.abilities.get(ability as usize).and_then(|a| data.ability_id(a)).map(|a| data.ability(a).range).unwrap_or(0.0);
                if (u.pos - point).length() <= range {
                    u.clear_path();
                    let cd = u.ability_cd.get(ability as usize).copied().unwrap_or(0.0);
                    if cd <= 0.0 {
                        abilities::cast(world, data, id, ability, point);
                    }
                    if let Some(u) = world.units.get_mut(id) {
                        next_order(u);
                    }
                } else {
                    move_toward(u, point);
                }
            }
            Order::Gather { node } => gather(world, data, id, node, dt),
            Order::ReturnCargo { node } => return_cargo(world, data, id, node),
            Order::Build { building } => build(world, data, id, building, dt, &builders),
        }
    }
}

fn flee(world: &mut SimWorld, data: &GameData, id: UnitId) {
    let Some(u) = world.units.get(id) else { return };
    let pos = u.pos;
    let owner = u.owner;
    let threat = u.last_attacker.and_then(|a| world.entity_pos(a));
    let home = if owner == PLAYER {
        world.building_with_tag(&world.mission_def.defeat_tag).and_then(|b| world.buildings.get(b)).map(|b| b.center())
    } else {
        world.building_with_tag(&world.mission_def.enemy_base_tag).and_then(|b| world.buildings.get(b)).map(|b| b.center())
    };
    let mut dir = Vec2::ZERO;
    if let Some(t) = threat {
        dir += (pos - t).try_normalize().unwrap_or(Vec2::ZERO);
    }
    if let Some(h) = home {
        dir += (h - pos).try_normalize().unwrap_or(Vec2::ZERO) * 0.8;
    }
    let goal = pos + dir.try_normalize().unwrap_or(Vec2::X) * 4.0;
    let goal = world.map.nearest_walkable(TileMap::tile_of(goal), 4).map(TileMap::center_of).unwrap_or(pos);
    let u = world.units.get_mut(id).expect("exists");
    u.windup = None;
    u.target = None;
    if u.path.is_empty() && !u.path_pending {
        u.request_path(goal);
    }
    u.set_activity(Activity::Flee);
    let _ = data;
}

fn gather(world: &mut SimWorld, data: &GameData, id: UnitId, node: NodeId, dt: f32) {
    let Some(u) = world.units.get(id) else { return };
    let def = data.unit(u.kind);
    let carry_cap = def.carry;
    let pos = u.pos;
    let carrying = u.carry;
    let Some(n) = world.nodes.get(node) else {
        // Quelle erschöpft: nächste gleichartige Quelle in der Nähe suchen.
        let kind = carrying.map(|c| c.0);
        let next = find_node_near(world, pos, kind, 12.0);
        let u = world.units.get_mut(id).expect("exists");
        match (next, carrying) {
            (Some(nn), _) => u.order = Order::Gather { node: nn },
            (None, Some(_)) => u.order = Order::ReturnCargo { node: None },
            (None, None) => next_order(u),
        }
        return;
    };
    let res = n.kind.resource();
    let dist = n.distance_to(pos);
    let center = n.center();
    if let Some((k, amt)) = carrying
        && (k != res || amt >= carry_cap)
    {
        let u = world.units.get_mut(id).expect("exists");
        u.order = Order::ReturnCargo { node: Some(node) };
        return;
    }
    let u = world.units.get_mut(id).expect("exists");
    if dist > 0.85 {
        move_toward(u, center);
        return;
    }
    u.clear_path();
    u.vel = Vec2::ZERO;
    u.face(center - pos);
    u.set_activity(Activity::Work);
    let before = u.work_timer;
    u.work_timer += dt * def.gather_speed.max(0.1);
    // Ein hörbarer/sichtbarer Arbeitsschlag pro Sekunde.
    if (before / 0.95).floor() != (u.work_timer / 0.95).floor() {
        world.events.push(SimEvent::WorkStroke { unit: id, resource: Some(res), pos });
    }
    let gather_time = data.rules.gather_time;
    let u = world.units.get_mut(id).expect("exists");
    if u.work_timer >= gather_time {
        u.work_timer = 0.0;
        let n = world.nodes.get_mut(node).expect("checked");
        let take = carry_cap.min(n.amount);
        n.amount -= take;
        let depleted = n.amount == 0;
        let (tile, kind) = (n.tile, n.kind);
        let u = world.units.get_mut(id).expect("exists");
        u.carry = Some((res, take));
        u.order = Order::ReturnCargo { node: Some(node) };
        if depleted {
            world.nodes.remove(node);
            world.map.occupy(tile, kind.size(), false);
            world.events.push(SimEvent::NodeDepleted { pos: center, tree: kind == NodeKind::Tree });
        }
    }
}

fn find_node_near(world: &SimWorld, p: Vec2, kind: Option<crate::data::ResourceKind>, radius: f32) -> Option<NodeId> {
    world
        .nodes
        .iter()
        .filter(|(_, n)| kind.is_none_or(|k| n.kind.resource() == k))
        .map(|(id, n)| (id, n.distance_to(p)))
        .filter(|(_, d)| *d <= radius)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(id, _)| id)
}

fn return_cargo(world: &mut SimWorld, data: &GameData, id: UnitId, node: Option<NodeId>) {
    let Some(u) = world.units.get(id) else { return };
    let (pos, owner, carry) = (u.pos, u.owner, u.carry);
    let Some((kind, amount)) = carry else {
        let u = world.units.get_mut(id).expect("exists");
        match node {
            Some(n) => u.order = Order::Gather { node: n },
            None => next_order(u),
        }
        return;
    };
    let Some(drop) = world.nearest_dropoff(data, owner, pos) else {
        let u = world.units.get_mut(id).expect("exists");
        next_order(u);
        return;
    };
    let b = world.buildings.get(drop).expect("exists");
    let (dist, center) = (b.distance_to(pos), b.center());
    let u = world.units.get_mut(id).expect("exists");
    if dist > 0.9 {
        move_toward(u, center);
        return;
    }
    u.clear_path();
    u.carry = None;
    let accepted = world.credit(owner, crate::data::Item::Res(kind), amount as f32);
    *world.players[owner as usize].stats.gathered.get_mut(kind) += accepted;
    if accepted < amount as f32 && owner == PLAYER && world.players[owner as usize].full_cooldown <= 0.0 {
        world.players[owner as usize].full_cooldown = 10.0;
        world.error(owner, format!("Lager voll ({}) – baut ein Lagerhaus.", kind.label()));
    }
    world.events.push(SimEvent::ResourceDelivered { owner, kind, amount, pos: center });
    let next = node.filter(|n| world.nodes.contains(*n)).or_else(|| find_node_near(world, pos, Some(kind), 14.0));
    let u = world.units.get_mut(id).expect("exists");
    match next {
        Some(n) => u.order = Order::Gather { node: n },
        None => next_order(u),
    }
}

fn build(world: &mut SimWorld, data: &GameData, id: UnitId, building: BuildingId, dt: f32, builders: &HashMap<BuildingId, u32>) {
    let Some(u) = world.units.get(id) else { return };
    let pos = u.pos;
    let speed = data.unit(u.kind).build_speed;
    let Some(b) = world.buildings.get(building) else {
        let u = world.units.get_mut(id).expect("exists");
        next_order(u);
        return;
    };
    if b.complete && b.hp >= b.max_hp {
        let u = world.units.get_mut(id).expect("exists");
        next_order(u);
        return;
    }
    let (dist, center, kind, owner) = (b.distance_to(pos), b.center(), b.kind, b.owner);
    let u = world.units.get_mut(id).expect("exists");
    if dist > 0.9 {
        move_toward(u, center);
        return;
    }
    u.clear_path();
    u.vel = Vec2::ZERO;
    u.face(center - pos);
    u.set_activity(Activity::Work);
    let before = u.work_timer;
    u.work_timer += dt;
    if (before / 0.8).floor() != (u.work_timer / 0.8).floor() {
        world.events.push(SimEvent::WorkStroke { unit: id, resource: None, pos });
    }
    let n = builders.get(&building).copied().unwrap_or(1).max(1) as f32;
    let def = data.building(kind);
    let b = world.buildings.get_mut(building).expect("exists");
    if !b.complete {
        // Jeder weitere Arbeiter beschleunigt mit abnehmendem Grenznutzen (n^0.75 gesamt).
        let rate = speed * n.powf(0.75) / n / def.build_time;
        let dp = (rate * dt).min(1.0 - b.progress);
        b.progress += dp;
        b.hp = (b.hp + b.max_hp * 0.9 * dp).min(b.max_hp);
        if b.progress >= 0.999 {
            b.progress = 1.0;
            b.complete = true;
            world.players[owner as usize].stats.buildings_built += 1;
            world.finish_building(data, building);
            world.events.push(SimEvent::BuildingCompleted { id: building, kind, owner, pos: center });
            let name = def.name.clone();
            if owner == PLAYER {
                world.record(crate::chronicle::ChronicleKind::Construction, format!("{name} wurde fertiggestellt."));
            }
            crate::systems::economy::recompute_population(world, data);
        }
    } else {
        b.hp = (b.hp + b.max_hp / (def.build_time * 1.5) * speed * dt).min(b.max_hp);
    }
}

/// Abklingzeiten, Effekte, Regeneration und Moral.
pub fn status_tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    let starving = world.players[PLAYER as usize].starving;
    let ids = world.units.ids();
    for id in ids {
        let Some(u) = world.units.get_mut(id) else { continue };
        let def = data.unit(u.kind);
        u.alive_time += dt;
        u.activity_time += dt;
        u.attack_cd = (u.attack_cd - dt).max(0.0);
        for c in &mut u.ability_cd {
            *c = (*c - dt).max(0.0);
        }
        for b in &mut u.buffs {
            b.remaining -= dt;
        }
        u.buffs.retain(|b| b.remaining > 0.0);
        if u.shield_time > 0.0 {
            u.shield_time -= dt;
            if u.shield_time <= 0.0 {
                u.shield = 0.0;
            }
        }
        u.combat_timer = (u.combat_timer - dt).max(0.0);
        let regen = def.hp_regen * if u.combat_timer <= 0.0 { 2.0 } else { 1.0 };
        u.hp = (u.hp + regen * dt).min(u.max_hp);
        u.mana = (u.mana + def.mana_regen * dt).min(u.max_mana);
        if def.fearless {
            u.morale = def.morale;
            continue;
        }
        if u.routing > 0.0 {
            u.routing -= dt;
            if u.routing <= 0.0 {
                u.morale = def.morale * 0.4;
                u.clear_path();
            }
            continue;
        }
        if u.combat_timer <= 0.0 {
            u.morale = (u.morale + 5.0 * dt).min(def.morale);
        }
        if starving && u.owner == PLAYER {
            u.morale -= 1.2 * dt;
        }
        if u.morale < def.morale * 0.15 && def.class != UnitClass::Hero {
            u.routing = 5.0;
            u.clear_path();
            let pos = u.pos;
            world.events.push(SimEvent::MoraleBroken { unit: id, pos });
        }
    }
}
