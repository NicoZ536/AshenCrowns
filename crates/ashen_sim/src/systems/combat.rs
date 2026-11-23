//! Angriffsauflösung, Projektile, Türme sowie das Entfernen gefallener Entitäten.

use crate::chronicle::ChronicleKind;
use crate::data::{GameData, ProjectileKind, UnitClass};
use crate::entity::*;
use crate::events::SimEvent;
use crate::world::SimWorld;
use glam::Vec2;
use rand::RngExt;

/// Geschossparameter, die nicht aus dem Ziel folgen.
#[derive(Clone, Copy, Default)]
struct Shot {
    splash: f32,
    building_mult: f32,
}

#[allow(clippy::too_many_arguments)]
fn spawn_projectile(world: &mut SimWorld, kind: ProjectileKind, owner: PlayerId, from: Vec2, target: EntityRef, damage: f32, dtype: crate::data::DamageType, bonus: Vec<(UnitClass, f32)>, source: Option<EntityRef>, shot: Shot) {
    let Some(tp) = world.entity_pos(target) else { return };
    let dist = (tp - from).length().max(0.5);
    let speed = kind.speed();
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
        arc: match kind {
            ProjectileKind::Arrow => (dist * 0.18).min(1.6),
            ProjectileKind::Boulder => (dist * 0.45).clamp(1.5, 4.5),
            ProjectileKind::Fireball => (dist * 0.1).min(0.8),
            _ => 0.15,
        },
        progress: 0.0,
        total: dist / speed,
        splash: shot.splash,
        building_mult: if shot.building_mult > 0.0 { shot.building_mult } else { 1.0 },
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
        let level_bonus = (u.level.saturating_sub(1)) as f32 * def.damage_per_level + u.mods.damage;
        let mult = world.units.get(id).map(|u| world.damage_factor(data, u)).unwrap_or(1.0);
        if !world.entity_alive(target) {
            continue;
        }
        let reach = world.entity_distance(data, target, pos).map(|d| d - def.radius);
        let in_reach = reach.is_some_and(|d| d <= def.range + 0.8 && d + 0.3 >= def.min_range);
        if !in_reach {
            continue;
        }
        let roll: f32 = world.rng.random();
        let mut dmg = (def.damage_min + (def.damage_max - def.damage_min) * roll + level_bonus) * mult;
        if matches!(target, EntityRef::Building(_)) {
            dmg *= def.building_mult;
        }
        match def.projectile {
            Some(pk) => {
                if rain {
                    dmg *= data.rules.rain_ranged_mult;
                }
                let shot = Shot { splash: def.splash, building_mult: def.building_mult };
                spawn_projectile(world, pk, owner, pos, target, dmg, def.damage_type, def.bonus_vs.clone(), Some(EntityRef::Unit(id)), shot);
            }
            None => {
                world.events.push(SimEvent::MeleeSwing { attacker: id, kind, pos });
                let bonus = def.bonus_vs.clone();
                world.deal_damage(data, target, dmg, def.damage_type, &bonus, Some(EntityRef::Unit(id)));
                if def.splash > 0.0 {
                    splash_damage(world, data, owner, world.entity_pos(target).unwrap_or(pos), def.splash, dmg * 0.5, def.damage_type, &def.bonus_vs, Some(EntityRef::Unit(id)), Some(target), def.building_mult);
                }
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
            if p.splash > 0.0 {
                // Flächengeschoss: schlägt immer am Zielpunkt ein und trifft alle Feinde im Radius.
                world.events.push(SimEvent::ProjectileImpact { kind: p.kind, pos: p.target_pos, hit: true });
                world.events.push(SimEvent::Splash { pos: p.target_pos, radius: p.splash, kind: p.kind });
                splash_damage(world, data, p.owner, p.target_pos, p.splash, p.damage, p.damage_type, &p.bonus_class_mult, p.source, None, p.building_mult);
                continue;
            }
            let hit = world.entity_alive(p.target) && world.entity_distance(data, p.target, p.pos).is_some_and(|d| d < 0.9);
            world.events.push(SimEvent::ProjectileImpact { kind: p.kind, pos: p.pos, hit });
            if hit {
                let mult = if matches!(p.target, EntityRef::Building(_)) { p.building_mult } else { 1.0 };
                world.deal_damage(data, p.target, p.damage * mult, p.damage_type, &p.bonus_class_mult, p.source);
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
            spawn_projectile(world, att.projectile, owner, from, EntityRef::Unit(t), att.damage, att.damage_type, Vec::new(), Some(EntityRef::Building(bid)), Shot::default());
            if let Some(b) = world.buildings.get_mut(bid) {
                b.attack_cd = att.cooldown;
            }
        }
    }
}

/// Flächenschaden um `center`: Feinde von `owner` im Radius erhalten bis zu `damage` (zum Rand hin
/// abnehmend bis 50 %), Gebäude zusätzlich den Gebäudefaktor. `skip` wird ausgelassen (bereits getroffen).
#[allow(clippy::too_many_arguments)]
pub fn splash_damage(world: &mut SimWorld, data: &GameData, owner: PlayerId, center: Vec2, radius: f32, damage: f32, dtype: crate::data::DamageType, bonus: &[(UnitClass, f32)], source: Option<EntityRef>, skip: Option<EntityRef>, building_mult: f32) {
    let mut near = Vec::new();
    world.spatial.query(center, radius + 0.5, &mut near);
    let targets: Vec<(UnitId, f32)> = near
        .iter()
        .filter_map(|&o| world.units.get(o).map(|u| (o, u)))
        .filter(|(o, u)| u.hp > 0.0 && SimWorld::hostile(owner, u.owner) && skip != Some(EntityRef::Unit(*o)))
        .map(|(o, u)| (o, (u.pos - center).length()))
        .filter(|(_, d)| *d <= radius + 0.3)
        .collect();
    for (o, d) in targets {
        let falloff = 1.0 - 0.5 * (d / radius.max(0.1)).clamp(0.0, 1.0);
        world.deal_damage(data, EntityRef::Unit(o), damage * falloff, dtype, bonus, source);
    }
    let bids: Vec<(BuildingId, f32)> = world
        .buildings
        .iter()
        .filter(|(id, b)| b.hp > 0.0 && SimWorld::hostile(owner, b.owner) && skip != Some(EntityRef::Building(*id)) && b.distance_to(center) <= radius)
        .map(|(id, b)| (id, b.distance_to(center)))
        .collect();
    for (b, d) in bids {
        let falloff = 1.0 - 0.5 * (d / radius.max(0.1)).clamp(0.0, 1.0);
        world.deal_damage(data, EntityRef::Building(b), damage * falloff * building_mult.max(0.0), dtype, bonus, source);
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
        crate::systems::heroes::on_unit_death(world, data, &u);
        // Beschworene Diener vergehen mit ihrem Herrn.
        let servants: Vec<UnitId> = world.units.iter().filter(|(_, x)| x.master == Some(id) && x.hp > 0.0).map(|(sid, _)| sid).collect();
        for sid in servants {
            if let Some(x) = world.units.get_mut(sid) {
                x.hp = 0.0;
            }
        }
        // Erfahrung für Helden des Gegners in der Nähe.
        let killer_owner = u.last_attacker.and_then(|a| world.entity_owner(a));
        if let Some(ko) = killer_owner {
            world.players[ko as usize].stats.kills += 1;
            if let Some(hid) = world.hero_of(data, ko) {
                let near = world.units.get(hid).is_some_and(|h| (h.pos - u.pos).length() < 14.0);
                if near {
                    let pct = world.units.get(hid).map(|h| h.mods.xp_pct).unwrap_or(0.0);
                    grant_xp(world, data, hid, (def.xp_value as f32 * (1.0 + pct)).round() as u32);
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
                world.fallen.push(FallenHero { unit: u.clone(), timer: data.rules.hero_respawn_time });
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
        world.map.clear_gate(b.tile);
        let def = data.building(b.kind);
        world.events.push(SimEvent::BuildingDestroyed { id, kind: b.kind, owner: b.owner, pos: b.center() });
        if let Some(tag) = &b.tag {
            world.mission.dead_tags.push(tag.clone());
        }
        let text = if b.owner == PLAYER { format!("{} wurde zerstört.", def.name) } else { format!("{} der Hohlen fiel.", def.name) };
        world.record(ChronicleKind::Battle, text);
        crate::systems::economy::recompute_population(world, data);
        crate::systems::city::recompute_capacity(world, data);
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
        gained.push(u.level);
    }
    if !gained.is_empty() {
        crate::systems::heroes::recompute(world, data, id);
        if let Some(u) = world.units.get_mut(id) {
            u.hp = (u.hp + def.hp_per_level * gained.len() as f32).min(u.max_hp);
            u.mana = u.max_mana;
        }
    }
    for lvl in gained {
        world.events.push(SimEvent::LevelUp { unit: id, level: lvl });
        world.record(ChronicleKind::Hero, format!("{} erreichte Stufe {lvl}.", def.name));
    }
}
