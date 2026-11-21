//! Ausführung von Fähigkeiten aus elementaren Effekten.

use crate::data::{Effect, GameData};
use crate::entity::*;
use crate::events::SimEvent;
use crate::world::SimWorld;
use glam::Vec2;

pub fn cast(world: &mut SimWorld, data: &GameData, caster: UnitId, ability: u8, point: Vec2) {
    let Some(u) = world.units.get(caster) else { return };
    let def = data.unit(u.kind);
    let Some(aid) = def.abilities.get(ability as usize).and_then(|a| data.ability_id(a)) else { return };
    let adef = data.ability(aid).clone();
    if u.mana < adef.mana || u.ability_cd.get(ability as usize).copied().unwrap_or(0.0) > 0.0 {
        return;
    }
    let (owner, origin, level) = (u.owner, u.pos, u.level);
    let level_mult = 1.0 + (level.saturating_sub(1)) as f32 * 0.08;
    {
        let u = world.units.get_mut(caster).expect("exists");
        u.mana -= adef.mana;
        if let Some(cd) = u.ability_cd.get_mut(ability as usize) {
            *cd = adef.cooldown;
        }
        u.set_activity(Activity::Cast);
        u.face(point - origin);
    }
    world.events.push(SimEvent::AbilityCast { unit: caster, ability: adef.id.clone(), pos: origin, target: point });

    let mut near = Vec::new();
    for e in &adef.effects {
        match e {
            Effect::Damage { radius, amount, damage_type } => {
                world.spatial.query(point, *radius + 0.4, &mut near);
                for &o in &near {
                    let hostile = world.units.get(o).is_some_and(|x| x.hp > 0.0 && SimWorld::hostile(owner, x.owner));
                    if hostile {
                        world.deal_damage(data, EntityRef::Unit(o), amount * level_mult, *damage_type, &[], Some(EntityRef::Unit(caster)));
                    }
                }
                let bids: Vec<BuildingId> = world.buildings.iter().filter(|(_, b)| SimWorld::hostile(owner, b.owner) && b.distance_to(point) <= *radius).map(|(id, _)| id).collect();
                for b in bids {
                    world.deal_damage(data, EntityRef::Building(b), amount * 0.5 * level_mult, *damage_type, &[], Some(EntityRef::Unit(caster)));
                }
            }
            Effect::Shield { amount, duration } => {
                if let Some(u) = world.units.get_mut(caster) {
                    u.shield = amount * level_mult;
                    u.shield_time = *duration;
                }
            }
            Effect::Rally { radius, damage_mult, armor_bonus, morale, duration } => {
                world.spatial.query(origin, *radius, &mut near);
                if !near.contains(&caster) {
                    near.push(caster);
                }
                for &o in &near {
                    if let Some(a) = world.units.get_mut(o)
                        && a.owner == owner
                    {
                        a.buffs.push(Buff { damage_mult: *damage_mult, armor_bonus: *armor_bonus, remaining: *duration });
                        let max = data.unit(a.kind).morale;
                        a.morale = (a.morale + morale).min(max);
                        if a.routing > 0.0 {
                            a.routing = 0.0;
                            a.morale = a.morale.max(max * 0.5);
                        }
                    }
                }
            }
            Effect::Drain { radius, amount } => {
                world.spatial.query(origin, *radius, &mut near);
                let mut total = 0.0;
                for &o in &near {
                    let hostile = world.units.get(o).is_some_and(|x| x.hp > 0.0 && SimWorld::hostile(owner, x.owner));
                    if hostile {
                        let before = world.units.get(o).map(|x| x.hp).unwrap_or(0.0);
                        world.deal_damage(data, EntityRef::Unit(o), *amount, crate::data::DamageType::Magic, &[], Some(EntityRef::Unit(caster)));
                        let after = world.units.get(o).map(|x| x.hp).unwrap_or(0.0);
                        total += (before - after).max(0.0);
                    }
                }
                if let Some(u) = world.units.get_mut(caster) {
                    u.hp = (u.hp + total * 0.6).min(u.max_hp);
                }
            }
            Effect::Summon { unit, count } => {
                if let Some(ut) = data.unit_id(unit) {
                    for i in 0..*count {
                        let a = i as f32 / *count as f32 * std::f32::consts::TAU;
                        let p = origin + Vec2::new(a.cos(), a.sin()) * 1.4;
                        let id = world.spawn_unit(data, ut, owner, p);
                        if let Some(s) = world.units.get_mut(id) {
                            s.order = Order::Guard { post: origin, radius: 8.0 };
                        }
                    }
                }
            }
            Effect::Terror { radius, morale } => {
                world.spatial.query(origin, *radius, &mut near);
                for &o in &near {
                    if let Some(a) = world.units.get_mut(o)
                        && SimWorld::hostile(owner, a.owner)
                        && !data.unit(a.kind).fearless
                    {
                        a.morale -= morale;
                    }
                }
            }
        }
    }
}
