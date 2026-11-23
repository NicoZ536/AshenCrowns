//! Budgetierte Wegsuche, Pfadverfolgung, Separation und Kollision.

use crate::data::GameData;
use crate::entity::{EntityRef, Order, UnitId};
use crate::map::TileMap;
use crate::world::SimWorld;
use glam::{IVec2, Vec2};

/// Maximale Wegsuchen pro Tick (verteilt Lastspitzen bei großen Gruppenbefehlen).
const MAX_REQUESTS_PER_TICK: usize = 48;
/// Gesamtbudget expandierter A*-Knoten pro Tick.
const NODE_BUDGET_PER_TICK: usize = 24_000;

pub fn process_paths(world: &mut SimWorld, data: &GameData) {
    let mut done = 0;
    let mut budget = NODE_BUDGET_PER_TICK;
    let ids = world.units.ids();
    for id in ids {
        if done >= MAX_REQUESTS_PER_TICK || budget == 0 {
            break;
        }
        let Some(u) = world.units.get(id) else { continue };
        if !u.path_pending {
            continue;
        }
        let Some(goal) = u.path_goal else {
            if let Some(u) = world.units.get_mut(id) {
                u.path_pending = false;
            }
            continue;
        };
        let (from, owner, order) = (u.pos, u.owner, u.order);
        // Flieger gehen geradeaus über alle Hindernisse.
        if data.unit(u.kind).flying {
            let max = Vec2::new(world.map.width as f32 - 0.5, world.map.height as f32 - 0.5);
            if let Some(u) = world.units.get_mut(id) {
                u.path = vec![goal.clamp(Vec2::splat(0.5), max)];
                u.path_pending = false;
            }
            continue;
        }
        let result = world.pathfinder.find_for(&world.map, from, goal, 6000.min(budget), owner);
        let found = world.pathfinder.last_found;
        budget = budget.saturating_sub(world.pathfinder.last_expanded.max(1));
        done += 1;
        // Kein Weg (Mauern/Tore versperren ihn): die Befestigung angreifen, statt festzustecken.
        if !found && matches!(order, Order::Move { attack_move: true, .. }) {
            let spent = plan_breach(world, data, id, goal);
            budget = budget.saturating_sub(spent);
        }
        let u = world.units.get_mut(id).expect("exists");
        u.path_pending = false;
        match result {
            Some(p) => u.path = p,
            None => {
                u.path.clear();
                u.path_goal = None;
                if matches!(u.order, Order::Move { .. }) {
                    u.order = Order::Idle;
                }
            }
        }
    }
}

/// Sucht für eine Einheit ohne freien Weg die erste feindliche Befestigung auf dem kürzesten Weg
/// (Befestigungen sind mit Zusatzkosten passierbar) und macht sie zum Angriffsziel.
/// Gibt die verbrauchten A*-Knoten zurück.
fn plan_breach(world: &mut SimWorld, data: &GameData, id: UnitId, goal: Vec2) -> usize {
    let Some(u) = world.units.get(id) else { return 0 };
    if data.unit(u.kind).damage_max <= 0.0 {
        return 0;
    }
    // Hat die Einheit schon eine lebende Befestigung als Ziel, bleibt sie dabei.
    if let Some(EntityRef::Building(b)) = u.target
        && world.buildings.get(b).is_some_and(|x| x.hp > 0.0 && data.building(x.kind).is_fort())
    {
        return 0;
    }
    let (from, owner) = (u.pos, u.owner);
    let mut breakable = vec![false; (world.map.width * world.map.height) as usize];
    let mut any = false;
    for (_, b) in world.buildings.iter() {
        if b.hp > 0.0 && !b.invulnerable && SimWorld::hostile(owner, b.owner) && data.building(b.kind).is_fort() {
            any = true;
            for y in b.tile.y..b.tile.y + b.size.y {
                for x in b.tile.x..b.tile.x + b.size.x {
                    let t = IVec2::new(x, y);
                    if world.map.in_bounds(t) {
                        breakable[world.map.idx(t)] = true;
                    }
                }
            }
        }
    }
    if !any {
        return 0;
    }
    let tile = world.pathfinder.find_breach(&world.map, from, goal, 6000, owner, &breakable);
    let spent = world.pathfinder.last_expanded.max(1);
    if let Some(t) = tile
        && let Some(bid) = world.building_at(t)
        && let Some(u) = world.units.get_mut(id)
    {
        u.target = Some(EntityRef::Building(bid));
    }
    spent
}

pub fn integrate(world: &mut SimWorld, data: &GameData, dt: f32) {
    let ids = world.units.ids();
    let mut neighbours: Vec<UnitId> = Vec::new();
    for id in ids {
        let Some(u) = world.units.get(id) else { continue };
        let def = data.unit(u.kind);
        let pos = u.pos;
        let owner = u.owner;
        let radius = def.radius;
        let flying = def.flying;
        let mut speed = def.speed * if u.routing > 0.0 { 1.15 } else { 1.0 } * world.speed_factor(u);
        if !flying {
            speed /= world.map.terrain_at(TileMap::tile_of(pos)).move_cost();
        }

        // Gewünschte Geschwindigkeit entlang des Pfades.
        let mut desired = Vec2::ZERO;
        let mut pop_wp = false;
        let moving = !u.path.is_empty();
        if let Some(&wp) = u.path.first() {
            let to = wp - pos;
            let d = to.length();
            let last = u.path.len() == 1;
            let eps = if last { 0.12 } else { 0.35 };
            if d < eps {
                pop_wp = true;
            } else {
                desired = to / d * speed.min(d / dt);
            }
        }

        // Separation von Nachbarn.
        world.spatial.query(pos, radius + 0.6, &mut neighbours);
        let mut push = Vec2::ZERO;
        let mut crowded_at_goal = false;
        for &o in &neighbours {
            if o == id {
                continue;
            }
            let Some(other) = world.units.get(o) else { continue };
            let odef = data.unit(other.kind);
            if odef.flying != flying {
                continue;
            }
            let d = pos - other.pos;
            let len = d.length();
            let min = radius + odef.radius;
            if len < min {
                let dir = if len > 1e-4 { d / len } else { Vec2::new(((id.index * 7 + o.index) % 5) as f32 - 2.0, 1.0).normalize() };
                let other_moving = !other.path.is_empty();
                let weight = if moving && !other_moving { 0.35 } else { 1.0 };
                push += dir * (min - len) * weight;
                if moving && !other_moving && u.path.len() == 1 && (other.pos - u.path[0]).length() < 0.7 {
                    crowded_at_goal = true;
                }
            }
        }

        let u = world.units.get_mut(id).expect("exists");
        if pop_wp {
            u.path.remove(0);
        }
        if crowded_at_goal && u.path.len() == 1 && (u.path[0] - pos).length() < 1.2 {
            u.path.clear();
        }
        let step = desired * dt + push * 0.45;
        let mut new_pos = pos + step;
        let map = &world.map;
        let (ot, nt) = (TileMap::tile_of(pos), TileMap::tile_of(new_pos));
        // Diagonale Schritte über die Ecke zweier Mauerkacheln sind verboten.
        let corner_cut = nt.x != ot.x && nt.y != ot.y && (!map.walkable_for(IVec2::new(nt.x, ot.y), owner) || !map.walkable_for(IVec2::new(ot.x, nt.y), owner));
        if !flying && (corner_cut || !map.walkable_for(nt, owner)) {
            let tx = Vec2::new(new_pos.x, pos.y);
            let ty = Vec2::new(pos.x, new_pos.y);
            if map.walkable_for(TileMap::tile_of(tx), owner) {
                new_pos = tx;
            } else if map.walkable_for(TileMap::tile_of(ty), owner) {
                new_pos = ty;
            } else if !map.walkable_for(ot, owner) {
                // In einer Blockade gefangen (z. B. neues Gebäude): herausschieben.
                if let Some(t) = map.nearest_walkable(TileMap::tile_of(pos), 5) {
                    new_pos = TileMap::center_of(t);
                }
            } else {
                new_pos = pos;
            }
        }
        new_pos = new_pos.clamp(Vec2::splat(0.3), Vec2::new(map.width as f32 - 0.3, map.height as f32 - 0.3));
        let moved = (new_pos - pos).length();
        u.vel = (new_pos - pos) / dt;
        u.pos = new_pos;
        if desired.length_squared() > 1e-4 {
            u.face(desired);
            // Steckenbleiben erkennen und neu planen.
            if moved < speed * dt * 0.15 {
                u.stuck_timer += dt;
                if u.stuck_timer > 1.0 {
                    if let Some(g) = u.path_goal {
                        if u.stuck_timer > 3.5 || (g - pos).length() < 1.2 {
                            u.path.clear();
                            u.stuck_timer = 0.0;
                        } else if u.stuck_timer > 1.0 && u.repath_timer <= 0.0 {
                            u.path_pending = true;
                            u.repath_timer = 1.0;
                        }
                    } else {
                        u.path.clear();
                    }
                }
            } else {
                u.stuck_timer = 0.0;
            }
        }
    }
}
