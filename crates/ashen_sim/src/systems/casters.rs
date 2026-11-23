//! Selbstständig wirkende Fähigkeiten (`UnitDef::autocast`): Priester heilen, Beschwörer rufen Diener,
//! Zauberer entfesseln Flächenzauber, sobald die Lage es verlangt.

use crate::data::{Effect, GameData};
use crate::entity::*;
use crate::systems::abilities;
use crate::systems::heroes::ability_list;
use crate::world::SimWorld;
use glam::Vec2;

/// Alle halbe Sekunde prüfen alle Einheiten mit Autocast-Liste.
pub fn tick(world: &mut SimWorld, data: &GameData) {
    if !world.tick.is_multiple_of(10) {
        return;
    }
    let ids = world.units.ids();
    for id in ids {
        let Some(u) = world.units.get(id) else { continue };
        if u.hp <= 0.0 || u.routing > 0.0 {
            continue;
        }
        let def = data.unit(u.kind);
        if def.autocast.is_empty() {
            continue;
        }
        let (pos, owner) = (u.pos, u.owner);
        let list: Vec<String> = ability_list(data, u).cloned().collect();
        for (i, name) in list.iter().enumerate() {
            if !def.autocast.contains(name) {
                continue;
            }
            let Some(aid) = data.ability_id(name) else { continue };
            let adef = data.ability(aid);
            let Some(u) = world.units.get(id) else { break };
            if u.mana < adef.mana || u.ability_cd.get(i).copied().unwrap_or(1.0) > 0.0 {
                continue;
            }
            if let Some(target) = wants_cast(world, data, id, owner, pos, adef) {
                abilities::cast(world, data, id, i as u8, target);
                break;
            }
        }
    }
}

/// Zielpunkt, wenn die Fähigkeit jetzt sinnvoll ist.
fn wants_cast(world: &mut SimWorld, data: &GameData, id: UnitId, owner: PlayerId, pos: Vec2, adef: &crate::data::AbilityDef) -> Option<Vec2> {
    let mut near = Vec::new();
    for e in &adef.effects {
        match e {
            Effect::Heal { radius, amount } => {
                world.spatial.query(pos, *radius, &mut near);
                let mut deficit = 0.0;
                let mut critical = false;
                for &o in &near {
                    if let Some(a) = world.units.get(o)
                        && a.owner == owner
                        && a.hp > 0.0
                    {
                        deficit += a.max_hp - a.hp;
                        critical |= a.hp < a.max_hp * 0.45;
                    }
                }
                if deficit >= amount * 0.6 || (critical && deficit > 20.0) {
                    return Some(pos);
                }
            }
            Effect::Summon { max, .. } => {
                let alive = world.units.iter().filter(|(_, x)| x.master == Some(id) && x.hp > 0.0).count() as u32;
                if *max != 0 && alive >= *max {
                    continue;
                }
                world.spatial.query(pos, 9.0, &mut near);
                let hostile = near.iter().any(|&o| world.units.get(o).is_some_and(|x| x.hp > 0.0 && SimWorld::hostile(owner, x.owner)));
                if hostile {
                    return Some(pos);
                }
            }
            Effect::Damage { radius, .. } => {
                let reach = if adef.range > 0.0 { adef.range } else { *radius };
                world.spatial.query(pos, reach, &mut near);
                let mut best: Option<(f32, Vec2)> = None;
                let mut count = 0;
                for &o in &near {
                    if let Some(x) = world.units.get(o)
                        && x.hp > 0.0
                        && SimWorld::hostile(owner, x.owner)
                    {
                        count += 1;
                        let d = (x.pos - pos).length();
                        if best.is_none_or(|(bd, _)| d < bd) {
                            best = Some((d, x.pos));
                        }
                    }
                }
                if count >= 2
                    && let Some((_, p)) = best
                {
                    return Some(if adef.range > 0.0 { p } else { pos });
                }
            }
            Effect::Rally { radius, .. } => {
                world.spatial.query(pos, *radius, &mut near);
                let fighting = near.iter().filter(|&&o| world.units.get(o).is_some_and(|x| x.owner == owner && x.combat_timer > 0.0)).count();
                if fighting >= 3 {
                    return Some(pos);
                }
            }
            _ => {}
        }
    }
    let _ = data;
    None
}
