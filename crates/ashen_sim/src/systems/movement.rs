//! Budgetierte Wegsuche, Pfadverfolgung, Separation und Kollision.

use crate::data::GameData;
use crate::entity::{Order, UnitId};
use crate::map::TileMap;
use crate::world::SimWorld;
use glam::Vec2;

/// Maximale Wegsuchen pro Tick (verteilt Lastspitzen bei großen Gruppenbefehlen).
const MAX_REQUESTS_PER_TICK: usize = 48;
/// Gesamtbudget expandierter A*-Knoten pro Tick.
const NODE_BUDGET_PER_TICK: usize = 24_000;

pub fn process_paths(world: &mut SimWorld, _data: &GameData) {
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
        let from = u.pos;
        let result = world.pathfinder.find(&world.map, from, goal, 6000.min(budget));
        budget = budget.saturating_sub(world.pathfinder.last_expanded.max(1));
        done += 1;
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

pub fn integrate(world: &mut SimWorld, data: &GameData, dt: f32) {
    let ids = world.units.ids();
    let mut neighbours: Vec<UnitId> = Vec::new();
    for id in ids {
        let Some(u) = world.units.get(id) else { continue };
        let def = data.unit(u.kind);
        let pos = u.pos;
        let radius = def.radius;
        let mut speed = def.speed * if u.routing > 0.0 { 1.15 } else { 1.0 };
        speed /= world.map.terrain_at(TileMap::tile_of(pos)).move_cost();

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
        if !map.walkable(TileMap::tile_of(new_pos)) {
            let tx = Vec2::new(new_pos.x, pos.y);
            let ty = Vec2::new(pos.x, new_pos.y);
            if map.walkable(TileMap::tile_of(tx)) {
                new_pos = tx;
            } else if map.walkable(TileMap::tile_of(ty)) {
                new_pos = ty;
            } else if !map.walkable(TileMap::tile_of(pos)) {
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
            let sx = desired.x - desired.y;
            if sx.abs() > 0.05 {
                u.facing_right = sx > 0.0;
            }
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
