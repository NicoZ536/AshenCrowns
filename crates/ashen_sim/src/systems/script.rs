//! Auswertung des Missionsskripts (Bedingungen → Aktionen) und eingebaute Sieg-/Niederlagenregeln.

use crate::chronicle::ChronicleKind;
use crate::data::{GameData, UnitClass};
use crate::entity::*;
use crate::events::SimEvent;
use crate::mission::{Action, Condition, DialogueLine, GameMode, ObjectiveState, Outcome};
use crate::systems::ai;
use crate::world::SimWorld;

fn check(world: &SimWorld, data: &GameData, c: &Condition) -> bool {
    match c {
        Condition::AtTime(t) => world.time >= *t,
        Condition::BuildingCompleted(id) => {
            let Some(bt) = data.building_id(id) else { return false };
            world.buildings.iter().any(|(_, b)| b.owner == PLAYER && b.kind == bt && b.complete)
        }
        Condition::BuildingCount { building, count } => {
            let Some(bt) = data.building_id(building) else { return false };
            world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.kind == bt && b.complete).count() as u32 >= *count
        }
        Condition::UnitCount { unit, count } => {
            let Some(ut) = data.unit_id(unit) else { return false };
            world.units.iter().filter(|(_, u)| u.owner == PLAYER && u.kind == ut).count() as u32 >= *count
        }
        Condition::MilitaryCount(n) => {
            world
                .units
                .iter()
                .filter(|(_, u)| u.owner == PLAYER && !matches!(data.unit(u.kind).class, UnitClass::Worker | UnitClass::Hero))
                .count() as u32
                >= *n
        }
        Condition::ObjectiveCompleted(id) => world.mission.is_completed(id),
        Condition::NightStarted(n) => world.env.night >= *n,
        Condition::DayStarted(n) => world.env.day >= *n,
        Condition::TagDead(t) => world.mission.dead_tags.iter().any(|d| d == t),
        Condition::TagDiscovered(t) => world.mission.discovered_tags.iter().any(|d| d == t),
        Condition::HeroLevel(l) => world.hero_of(data, PLAYER).and_then(|h| world.units.get(h)).is_some_and(|h| h.level >= *l),
        Condition::WavesSurvived(n) => world.mission.waves_survived >= *n,
        Condition::EraReached(e) => world.players[PLAYER as usize].era >= *e,
        Condition::Residents(n) => world.players[PLAYER as usize].residents >= *n as f32,
        Condition::Stock { item, amount } => data.item(item).is_some_and(|it| world.players[PLAYER as usize].stock(it) >= *amount),
        Condition::UnitsInArea { area, count } => world
            .mission_def
            .area(area)
            .is_some_and(|a| world.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && a.contains(u.pos)).count() as u32 >= *count),
        Condition::EnemyUnitsAtMost(n) => world.units.iter().filter(|(_, u)| u.owner == ENEMY && u.hp > 0.0).count() as u32 <= *n,
        Condition::TagDamaged(t) => world.building_with_tag(t).and_then(|b| world.buildings.get(b)).is_some_and(|b| b.hp < b.max_hp * 0.999),
        Condition::Flag(f) => world.mission.has_flag(f),
        Condition::TagInArea { tag, area } => world.mission_def.area(area).is_some_and(|a| world.units.iter().any(|(_, u)| u.hp > 0.0 && u.tag.as_deref() == Some(tag.as_str()) && a.contains(u.pos))),
        Condition::TagHealthBelow { tag, percent } => world.units.iter().any(|(_, u)| u.hp > 0.0 && u.tag.as_deref() == Some(tag.as_str()) && u.hp < u.max_hp * percent / 100.0),
        Condition::DialogueDone => world.mission.line_queue.is_empty() && world.time >= world.mission.line_ready,
        Condition::UnitKindInArea { unit, area, count } => {
            let (Some(ut), Some(a)) = (data.unit_id(unit), world.mission_def.area(area)) else { return false };
            world.units.iter().filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && u.kind == ut && a.contains(u.pos)).count() as u32 >= *count
        }
        Condition::ArmyAtMost(n) => {
            world
                .units
                .iter()
                .filter(|(_, u)| u.owner == PLAYER && u.hp > 0.0 && !matches!(data.unit(u.kind).class, UnitClass::Worker | UnitClass::Hero))
                .count() as u32
                <= *n
        }
        Condition::Not(c) => !check(world, data, c),
        Condition::All(v) => v.iter().all(|c| check(world, data, c)),
        Condition::Any(v) => v.iter().any(|c| check(world, data, c)),
    }
}

/// Führt Aktionen aus (Auslöser und Entscheidungsoptionen).
pub fn run_actions(world: &mut SimWorld, data: &GameData, actions: &[Action]) {
    for a in actions {
        run(world, data, a);
    }
}

/// Geschätzte Sprechdauer einer Zeile in Sekunden.
pub fn line_duration(text: &str) -> f32 {
    (1.6 + text.chars().count() as f32 * 0.058).clamp(2.4, 14.0)
}

/// Gibt fällige Dialogzeilen aus.
fn pump_lines(world: &mut SimWorld) {
    while let Some((_, line)) = world.mission.line_queue.first().cloned() {
        if world.time < world.mission.line_ready {
            break;
        }
        world.mission.line_queue.remove(0);
        world.mission.line_ready = world.time + line_duration(&line.text);
        world.events.push(SimEvent::Message { speaker: line.speaker, text: line.text, voice: line.voice });
    }
}

/// Wählt eine Option der ersten offenen Entscheidung. Ungültige Wahlen werden ignoriert.
pub fn decide(world: &mut SimWorld, data: &GameData, decision: &str, option: usize) {
    let Some(d) = world.mission.open_decision(world.time).cloned() else { return };
    if d.id != decision {
        return;
    }
    let Some(o) = d.options.get(option).cloned() else { return };
    if !o.requires.iter().all(|f| world.mission.has_flag(f)) {
        world.error(PLAYER, "Diese Möglichkeit steht Euch nicht offen.");
        return;
    }
    world.mission.decisions.remove(0);
    world.mission.choices.push((d.id.clone(), o.flag.clone()));
    if !world.mission.has_flag(&o.flag) {
        world.mission.flags.push(o.flag.clone());
    }
    world.record(ChronicleKind::Omen, format!("{}: {}", d.title, o.label));
    run_actions(world, data, &o.actions);
}

fn run(world: &mut SimWorld, data: &GameData, a: &Action) {
    match a {
        Action::Message { speaker, text, voice } => {
            world.events.push(SimEvent::Message { speaker: speaker.clone(), text: text.clone(), voice: voice.clone() });
        }
        Action::AddObjective(id) => {
            if !world.mission.has_objective(id) {
                world.mission.objectives.push((id.clone(), ObjectiveState::Active));
                world.events.push(SimEvent::ObjectiveAdded { id: id.clone() });
            }
        }
        Action::CompleteObjective(id) => {
            if !world.mission.has_objective(id) {
                world.mission.objectives.push((id.clone(), ObjectiveState::Active));
            }
            if let Some(o) = world.mission.objectives.iter_mut().find(|(o, _)| o == id)
                && o.1 != ObjectiveState::Completed
            {
                o.1 = ObjectiveState::Completed;
                world.events.push(SimEvent::ObjectiveCompleted { id: id.clone() });
            }
        }
        Action::Victory => finish(world, Outcome::Victory),
        Action::Defeat => finish(world, Outcome::Defeat),
        Action::SpawnWave(b) => ai::spawn_wave(world, data, *b * world.settings.difficulty.wave_mult()),
        Action::Grant(c) => world.players[PLAYER as usize].resources.add(c),
        Action::EnableWaves(on) => {
            if world.settings.mode != GameMode::Sandbox {
                world.director.enabled = *on;
            }
        }
        Action::SetWeather(w) => {
            world.env.weather = *w;
            world.env.weather_intensity = 0.0;
            world.env.weather_timer = 150.0;
            world.events.push(SimEvent::WeatherChanged { weather: *w });
        }
        Action::Chronicle(t) => world.record(ChronicleKind::Omen, t.clone()),
        Action::SpawnUnits { unit, owner, area, count, attack, tag } => {
            let (Some(ut), Some(a)) = (data.unit_id(unit), world.mission_def.area(area).cloned()) else { return };
            let home = world
                .building_with_tag(&world.mission_def.defeat_tag.clone())
                .and_then(|b| world.buildings.get(b))
                .map(|b| b.center())
                .or_else(|| world.buildings.iter().find(|(_, b)| b.owner == PLAYER).map(|(_, b)| b.center()))
                .or_else(|| world.units.iter().find(|(_, u)| u.owner == PLAYER).map(|(_, u)| u.pos));
            // Gegnerische Verstärkungen skalieren mit der Schwierigkeit (Bosse und Markierte nicht).
            let count = if *owner == ENEMY && tag.is_none() { ((*count as f32) * world.settings.difficulty.wave_mult()).round().max(1.0) as u32 } else { *count };
            for i in 0..count {
                let ang = i as f32 * 2.399;
                let off = glam::Vec2::new(ang.cos(), ang.sin()) * (0.6 * (i as f32).sqrt());
                let id = world.spawn_unit(data, ut, *owner, a.center() + off);
                if let Some(u) = world.units.get_mut(id) {
                    u.tag = tag.clone();
                }
                if *owner == PLAYER {
                    world.apply_hero_record(data, id);
                }
                if *attack
                    && let (Some(h), Some(u)) = (home, world.units.get_mut(id))
                {
                    u.order = Order::Move { target: h, attack_move: true };
                    u.request_path(h);
                }
            }
        }
        Action::SetFlag(f) => {
            if !world.mission.has_flag(f) {
                world.mission.flags.push(f.clone());
            }
        }
        Action::Dialogue(lines) => {
            let base = world.time;
            for l in lines {
                world.mission.line_queue.push((base, DialogueLine { speaker: l.speaker.clone(), text: l.text.clone(), voice: l.voice.clone() }));
            }
        }
        Action::Decision(d) => {
            if !world.mission.decisions.iter().any(|x| x.id == d.id) && !world.mission.choices.iter().any(|(x, _)| *x == d.id) {
                world.mission.decisions.push(d.clone());
                world.events.push(SimEvent::Message { speaker: d.speaker.clone(), text: d.text.clone(), voice: d.voice.clone() });
            }
        }
        Action::MoveTag { tag, area, attack } => {
            let Some(a) = world.mission_def.area(area).cloned() else { return };
            let target = a.center();
            for (_, u) in world.units.iter_mut() {
                if u.hp > 0.0 && u.tag.as_deref() == Some(tag.as_str()) {
                    u.order = Order::Move { target, attack_move: *attack };
                    u.target = None;
                    u.request_path(target);
                }
            }
        }
        Action::ChangeOwner { tag, owner } => {
            for (_, u) in world.units.iter_mut() {
                if u.hp > 0.0 && u.tag.as_deref() == Some(tag.as_str()) {
                    u.owner = *owner;
                    u.order = Order::Idle;
                    u.target = None;
                    u.clear_path();
                }
            }
        }
    }
}

fn finish(world: &mut SimWorld, o: Outcome) {
    if world.mission.outcome.is_some() {
        return;
    }
    world.mission.outcome = Some(o);
    match o {
        Outcome::Victory => {
            world.events.push(SimEvent::Victory);
            let text = if !world.mission_def.victory_text.is_empty() {
                world.mission_def.victory_text.clone()
            } else if world.mission_def.map.is_some() {
                "Der Einsatz endete im Sieg. Die Aschmark erinnert sich daran.".to_string()
            } else {
                "Brennhollow hielt stand. Die Aschmark erinnert sich an diesen Sieg.".to_string()
            };
            world.record(ChronicleKind::Victory, text);
        }
        Outcome::Defeat => {
            world.events.push(SimEvent::Defeat);
            let text = if !world.mission_def.defeat_text.is_empty() {
                world.mission_def.defeat_text.clone()
            } else if world.mission_def.map.is_some() {
                "Der Einsatz endete in Asche und Schweigen.".to_string()
            } else {
                "Brennhollow fiel in Asche und Schweigen.".to_string()
            };
            world.record(ChronicleKind::Defeat, text);
        }
    }
}

fn update_discovery(world: &mut SimWorld) {
    let mut found = Vec::new();
    for (_, b) in world.buildings.iter() {
        if let Some(t) = &b.tag
            && b.owner != PLAYER
            && world.fog.visible(b.center())
            && !world.mission.discovered_tags.contains(t)
        {
            found.push(t.clone());
        }
    }
    for (_, u) in world.units.iter() {
        if let Some(t) = &u.tag
            && u.owner != PLAYER
            && world.fog.visible(u.pos)
            && !world.mission.discovered_tags.contains(t)
            && !found.contains(t)
        {
            found.push(t.clone());
        }
    }
    world.mission.discovered_tags.extend(found);
}

pub fn tick(world: &mut SimWorld, data: &GameData) {
    if !world.tick.is_multiple_of(10) || world.mission.outcome.is_some() {
        return;
    }
    update_discovery(world);
    pump_lines(world);
    let mode = world.settings.mode;
    let triggers = world.mission_def.triggers.clone();
    for t in &triggers {
        if world.mission.fired.contains(&t.id) {
            continue;
        }
        if !t.modes.is_empty() && !t.modes.contains(&mode) {
            continue;
        }
        if check(world, data, &t.condition) {
            world.mission.fired.push(t.id.clone());
            for a in &t.actions {
                run(world, data, a);
            }
        }
    }

    // Eingebaute Regeln.
    let defeat_tag = world.mission_def.defeat_tag.clone();
    let keep_alive = defeat_tag.is_empty() || world.building_with_tag(&defeat_tag).is_some();
    let anything_left = world.buildings.iter().any(|(_, b)| b.owner == PLAYER) || world.units.iter().any(|(_, u)| u.owner == PLAYER);
    if mode != GameMode::Sandbox && (!keep_alive || !anything_left) {
        finish(world, Outcome::Defeat);
    }
    if mode == GameMode::Skirmish && world.mission.dead_tags.contains(&world.mission_def.enemy_base_tag) {
        finish(world, Outcome::Victory);
    }
}
