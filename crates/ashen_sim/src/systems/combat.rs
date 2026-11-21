//! Angriffsauflösung, Projektile, Türme sowie das Entfernen gefallener Entitäten.

use crate::chronicle::ChronicleKind;
use crate::data::{GameData, ProjectileKind, UnitClass};
use crate::entity::*;
use crate::events::SimEvent;
use crate::world::SimWorld;
use glam::Vec2;
use rand::RngExt;

fn projectile_speed(kind: ProjectileKind) -> f32 {
    match kind {
        ProjectileKind::Arrow => 11.0,
        ProjectileKind::BoneBolt => 8.0,
        ProjectileKind::SoulBolt => 7.0,
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_projectile(world: &mut SimWorld, kind: ProjectileKind, owner: PlayerId, from: Vec2, target: EntityRef, damage: f32, dtype: crate::data::DamageType, bonus: Vec<(UnitClass, f32)>, source: Option<EntityRef>) {
    let Some(tp) = world.entity_pos(target) else { return };
    let dist = (tp - from).length().max(0.5);
    let speed = projectile_speed(kind);
    world.projectiles.push(Projectile {
        kind,
        owner,
        origin: from,
        pos: from,
        prev_pos: from,
        target,
        target_pos: tp,
        speed,
        damage,
        damage_type: dtype,
        bonus_class_mult: bonus,
        source,
        arc: if kind == ProjectileKind::Arrow { (dist * 0.18).min(1.6) } else { 0.15 },
        progress: 0.0,
        total: dist / speed,
    });
    world.events.push(SimEvent::ProjectileFired { kind, from, owner });
}

pub fn tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    let rain = world.env.weather.is_rainy();
    // 1) Ausholbewegungen abschließen.
    let ids = world.units.ids();
    for id in ids {
        let Some(u) = world.units.get_mut(id) else { continue };
        let Some((t, target)) = u.windup else { continue };
        let t = t - dt;
        if t > 0.0 {
            u.windup = Some((t, target));
            continue;
        }
        u.windup = None;
        let def = data.unit(u.kind);
        u.attack_cd = def.cooldown;
        let (pos, owner, kind) = (u.pos, u.owner, u.kind);
        let level_bonus = (u.level.saturating_sub(1)) as f32 * def.damage_per_level;
        let mult = u.damage_mult();
        if !world.entity_alive(target) {
            continue;
        }
        let in_reach = world.entity_distance(data, target, pos).is_some_and(|d| d - def.radius <= def.range + 0.8);
        if !in_reach {
            continue;
        }
        let roll: f32 = world.rng.random();
        let mut dmg = (def.damage_min + (def.damage_max - def.damage_min) * roll + level_bonus) * mult;
        match def.projectile {
            Some(pk) => {
                if rain {
                    dmg *= data.rules.rain_ranged_mult;
                }
                spawn_projectile(world, pk, owner, pos, target, dmg, def.damage_type, def.bonus_vs.clone(), Some(EntityRef::Unit(id)));
            }
            None => {
                world.events.push(SimEvent::MeleeSwing { attacker: id, kind, pos });
                let bonus = def.bonus_vs.clone();
                world.deal_damage(data, target, dmg, def.damage_type, &bonus, Some(EntityRef::Unit(id)));
            }
        }
    }

    // 2) Projektile bewegen.
    let mut i = 0;
    while i < world.projectiles.len() {
        let target = world.projectiles[i].target;
        if let Some(tp) = world.entity_pos(target) {
            world.projectiles[i].target_pos = tp;
        }
        let p = &mut world.projectiles[i];
        p.progress += dt;
        let t = (p.progress / p.total).min(1.0);
        p.pos = p.origin.lerp(p.target_pos, t);
        if t >= 1.0 {
            let p = world.projectiles.swap_remove(i);
            let hit = world.entity_alive(p.target) && world.entity_distance(data, p.target, p.pos).is_some_and(|d| d < 0.9);
            world.events.push(SimEvent::ProjectileImpact { kind: p.kind, pos: p.pos, hit });
            if hit {
                world.deal_damage(data, p.target, p.damage, p.damage_type, &p.bonus_class_mult, p.source);
            }
            continue;
        }
        i += 1;
    }

    // 3) Türme.
    let bids = world.buildings.ids();
    for bid in bids {
        let Some(b) = world.buildings.get_mut(bid) else { continue };
        if !b.complete {
            continue;
        }
        let Some(att) = data.building(b.kind).attack.clone() else { continue };
        b.attack_cd -= dt;
        if b.attack_cd > 0.0 {
            continue;
        }
        let (center, owner) = (b.center(), b.owner);
        let mut found = std::mem::take(&mut world.scratch);
        world.spatial.query(center, att.range + 1.0, &mut found);
        let target = found
            .iter()
            .filter_map(|&u| world.units.get(u).map(|x| (u, x)))
            .filter(|(_, x)| x.hp > 0.0 && SimWorld::hostile(owner, x.owner))
            .min_by(|a, b| (a.1.pos - center).length().total_cmp(&(b.1.pos - center).length()))
            .map(|(u, _)| u);
        world.scratch = found;
        if let Some(t) = target {
            let from = center + Vec2::new(-0.6, -0.6);
            spawn_projectile(world, att.projectile, owner, from, EntityRef::Unit(t), att.damage, att.damage_type, Vec::new(), Some(EntityRef::Building(bid)));
            if let Some(b) = world.buildings.get_mut(bid) {
                b.attack_cd = att.cooldown;
            }
        }
    }
}

/// Entfernt Einheiten und Gebäude ohne Lebenspunkte und verteilt Folgen (XP, Moral, Chronik).
pub fn cleanup_dead(world: &mut SimWorld, data: &GameData) {
    let dead: Vec<UnitId> = world.units.iter().filter(|(_, u)| u.hp <= 0.0).map(|(id, _)| id).collect();
    for id in dead {
        let Some(u) = world.units.remove(id) else { continue };
        let def = data.unit(u.kind);
        world.events.push(SimEvent::UnitDied { id, kind: u.kind, owner: u.owner, pos: u.pos, facing_right: u.facing_right, facing: u.facing });
        world.players[u.owner as usize].stats.units_lost += 1;
        if let Some(tag) = &u.tag {
            world.mission.dead_tags.push(tag.clone());
        }
        // Erfahrung für Helden des Gegners in der Nähe.
        let killer_owner = u.last_attacker.and_then(|a| world.entity_owner(a));
        if let Some(ko) = killer_owner {
            world.players[ko as usize].stats.kills += 1;
            if let Some(hid) = world.hero_of(data, ko) {
                let near = world.units.get(hid).is_some_and(|h| (h.pos - u.pos).length() < 14.0);
                if near {
                    grant_xp(world, data, hid, def.xp_value);
                }
            }
        }
        // Moralfolgen für Verbündete.
        let is_hero = def.class == UnitClass::Hero;
        let radius = if is_hero { 12.0 } else { 5.0 };
        let loss = if is_hero { 35.0 } else { 6.0 };
        let mut near = std::mem::take(&mut world.scratch);
        world.spatial.query(u.pos, radius, &mut near);
        for &o in &near {
            if let Some(a) = world.units.get_mut(o)
                && a.owner == u.owner
                && !data.unit(a.kind).fearless
            {
                a.morale -= loss;
            }
        }
        world.scratch = near;
        if is_hero {
            world.events.push(SimEvent::HeroFallen { kind: u.kind, pos: u.pos });
            world.record(ChronicleKind::Death, format!("{} fiel im Kampf.", def.name));
            if u.owner == PLAYER && !world.settings.hero_permadeath {
                world.hero_respawn = Some((u.kind, data.rules.hero_respawn_time));
            }
        }
        if def.class == UnitClass::Boss {
            world.record(ChronicleKind::Battle, format!("{} wurde vernichtet.", def.name));
        }
    }

    let dead_b: Vec<BuildingId> = world.buildings.iter().filter(|(_, b)| b.hp <= 0.0).map(|(id, _)| id).collect();
    for id in dead_b {
        let Some(b) = world.buildings.remove(id) else { continue };
        world.map.occupy(b.tile, b.size, false);
        let def = data.building(b.kind);
        world.events.push(SimEvent::BuildingDestroyed { id, kind: b.kind, owner: b.owner, pos: b.center() });
        if let Some(tag) = &b.tag {
            world.mission.dead_tags.push(tag.clone());
        }
        let text = if b.owner == PLAYER { format!("{} wurde zerstört.", def.name) } else { format!("{} der Hohlen fiel.", def.name) };
        world.record(ChronicleKind::Battle, text);
        crate::systems::economy::recompute_population(world, data);
    }
}

/// Erfahrung vergeben und ggf. (mehrfachen) Stufenaufstieg auslösen.
pub fn grant_xp(world: &mut SimWorld, data: &GameData, id: UnitId, amount: u32) {
    let levels = &data.rules.xp_levels;
    let Some(u) = world.units.get_mut(id) else { return };
    let def = data.unit(u.kind);
    u.xp += amount;
    let mut gained = Vec::new();
    while (u.level as usize) < levels.len() && u.xp >= levels[u.level as usize] {
        u.level += 1;
        u.max_hp += def.hp_per_level;
        u.hp += def.hp_per_level;
        u.max_mana += 15.0;
        u.mana = u.max_mana;
        gained.push(u.level);
    }
    for lvl in gained {
        world.events.push(SimEvent::LevelUp { unit: id, level: lvl });
        world.record(ChronicleKind::Hero, format!("{} erreichte Stufe {lvl}.", def.name));
    }
}
