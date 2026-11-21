//! Gegner-KI der Vertical Slice: Wellen-Direktor („Hohler Ruf“), Wellenziele und Boss-Verhalten.

use crate::chronicle::ChronicleKind;
use crate::data::{GameData, UnitClass};
use crate::entity::*;
use crate::events::SimEvent;
use crate::mission::GameMode;
use crate::systems::{abilities, economy};
use crate::world::SimWorld;
use glam::Vec2;
use rand::RngExt;

pub fn tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    director(world, data, dt);
    if world.tick.is_multiple_of(20) {
        retarget_waves(world, data);
        boss_and_casters(world, data);
        count_survived_waves(world);
    }
}

fn enemy_base(world: &SimWorld) -> Option<Building> {
    world.building_with_tag(&world.mission_def.enemy_base_tag).and_then(|b| world.buildings.get(b)).cloned()
}

fn player_home(world: &SimWorld) -> Option<Vec2> {
    world
        .building_with_tag(&world.mission_def.defeat_tag)
        .and_then(|b| world.buildings.get(b))
        .map(|b| b.center())
        .or_else(|| world.buildings.iter().find(|(_, b)| b.owner == PLAYER).map(|(_, b)| b.center()))
}

fn director(world: &mut SimWorld, data: &GameData, dt: f32) {
    if !world.director.enabled {
        return;
    }
    let Some(base) = enemy_base(world) else { return };
    world.director.next_wave_in -= dt;
    if !world.director.warned && world.director.next_wave_in <= 20.0 {
        world.director.warned = true;
        let from = base.center();
        let size = 0;
        world.events.push(SimEvent::WaveIncoming { wave: world.director.wave + 1, from, size });
    }
    if world.director.next_wave_in > 0.0 {
        return;
    }
    let rules = world.mission_def.waves.clone();
    let wave = world.director.wave + 1;
    let night = world.env.is_night();
    let mut budget = rules.base_budget + rules.budget_per_wave * (wave - 1) as f32;
    if night {
        budget *= rules.night_mult;
    }
    if world.settings.mode == GameMode::Survival {
        budget *= 1.0 + wave as f32 * 0.08;
    }
    budget *= world.settings.difficulty.wave_mult();
    spawn_wave(world, data, budget);
    world.director.next_wave_in = if night { rules.interval_night } else { rules.interval_day };
    world.director.warned = false;
}

/// Erzeugt eine Angriffswelle mit gegebenem Punktebudget am gegnerischen Hauptgebäude.
pub fn spawn_wave(world: &mut SimWorld, data: &GameData, budget: f32) {
    let Some(base) = enemy_base(world) else { return };
    let Some(home) = player_home(world) else { return };
    world.director.wave += 1;
    let wave = world.director.wave;
    let exit = economy::exit_point(world, &base, Some(home));
    let thrall = data.unit_id("hollow_thrall");
    let archer = data.unit_id("hollow_archer");
    let brute = data.unit_id("grave_brute");
    let mut left = budget;
    let mut spawned = 0u32;
    let mut guard = 0;
    while left >= 1.0 && guard < 64 {
        guard += 1;
        let r: f32 = world.rng.random();
        let (kind, cost) = if left >= 4.0 && r < 0.22 && wave >= 2 {
            (brute, 4.0)
        } else if left >= 1.5 && r < 0.55 {
            (archer, 1.5)
        } else {
            (thrall, 1.0)
        };
        let Some(kind) = kind else { break };
        left -= cost;
        let a = spawned as f32 * 2.399;
        let off = Vec2::new(a.cos(), a.sin()) * (0.6 * (spawned as f32).sqrt());
        let id = world.spawn_unit(data, kind, ENEMY, exit + off);
        if let Some(u) = world.units.get_mut(id) {
            u.wave = Some(wave);
            u.order = Order::Move { target: home, attack_move: true };
            u.request_path(home);
        }
        spawned += 1;
    }
    world.events.push(SimEvent::WaveIncoming { wave, from: exit, size: spawned });
    let text = if world.env.is_night() {
        format!("In der {}. Nacht brach eine Welle von {spawned} Hohlen aus Vael Dorn hervor.", world.env.night.max(1))
    } else {
        format!("Am {}. Tag marschierten {spawned} Hohle gegen Brennhollow.", world.env.day)
    };
    world.record(ChronicleKind::Battle, text);
}

/// Wellen, die ihr Ziel erreicht haben, suchen sich das nächste Ziel.
fn retarget_waves(world: &mut SimWorld, data: &GameData) {
    let targets: Vec<Vec2> = world
        .buildings
        .iter()
        .filter(|(_, b)| b.owner == PLAYER)
        .map(|(_, b)| b.center())
        .chain(world.units.iter().filter(|(_, u)| u.owner == PLAYER).map(|(_, u)| u.pos))
        .collect();
    if targets.is_empty() {
        return;
    }
    let ids = world.units.ids();
    for id in ids {
        let Some(u) = world.units.get_mut(id) else { continue };
        if u.wave.is_none() || u.owner != ENEMY || u.order != Order::Idle || u.target.is_some() {
            continue;
        }
        let p = u.pos;
        if let Some(t) = targets.iter().min_by(|a, b| (**a - p).length().total_cmp(&(**b - p).length())) {
            u.order = Order::Move { target: *t, attack_move: true };
            u.request_path(*t);
        }
    }
    let _ = data;
}

/// Fähigkeitseinsatz gegnerischer Zauberer und Bosse.
fn boss_and_casters(world: &mut SimWorld, data: &GameData) {
    let ids = world.units.ids();
    for id in ids {
        let Some(u) = world.units.get(id) else { continue };
        if u.owner != ENEMY {
            continue;
        }
        let def = data.unit(u.kind);
        if def.abilities.is_empty() {
            continue;
        }
        let (pos, hp_frac, mana) = (u.pos, u.hp / u.max_hp, u.mana);
        let in_combat = u.combat_timer > 0.0 || u.target.is_some();
        if !in_combat {
            continue;
        }
        let mut near = Vec::new();
        world.spatial.query(pos, 4.0, &mut near);
        let hostile_near = near.iter().filter(|&&o| world.units.get(o).is_some_and(|x| x.owner == PLAYER)).count();
        for (i, a) in def.abilities.iter().enumerate() {
            let Some(aid) = data.ability_id(a) else { continue };
            let adef = data.ability(aid);
            let ready = world.units.get(id).is_some_and(|x| x.ability_cd.get(i).copied().unwrap_or(1.0) <= 0.0) && mana >= adef.mana;
            if !ready {
                continue;
            }
            let wants = adef.effects.iter().any(|e| match e {
                crate::data::Effect::Summon { .. } => {
                    let done = world.director.boss_summons_done;
                    (done == 0 && hp_frac < 0.75) || (done == 1 && hp_frac < 0.4)
                }
                crate::data::Effect::Drain { .. } => hostile_near >= 1 && hp_frac < 0.9,
                crate::data::Effect::Terror { .. } => hostile_near >= 3,
                _ => hostile_near >= 2,
            });
            if wants {
                if adef.effects.iter().any(|e| matches!(e, crate::data::Effect::Summon { .. })) {
                    world.director.boss_summons_done += 1;
                }
                abilities::cast(world, data, id, i as u8, pos);
                break;
            }
        }
        if def.class == UnitClass::Boss && hp_frac < 0.5 {
            // Unterhalb der Hälfte greift der Boss rücksichtslos an.
            if let Some(u) = world.units.get_mut(id)
                && let Order::Guard { post, .. } = u.order
            {
                u.order = Order::Guard { post, radius: 11.0 };
            }
        }
    }
}

fn count_survived_waves(world: &mut SimWorld) {
    let mut survived = 0;
    for w in 1..=world.director.wave {
        if !world.units.iter().any(|(_, u)| u.wave == Some(w)) {
            survived += 1;
        }
    }
    world.mission.waves_survived = survived;
}
