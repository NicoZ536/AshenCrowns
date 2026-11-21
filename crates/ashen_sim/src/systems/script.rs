//! Auswertung des Missionsskripts (Bedingungen → Aktionen) und eingebaute Sieg-/Niederlagenregeln.

use crate::chronicle::ChronicleKind;
use crate::data::{GameData, UnitClass};
use crate::entity::*;
use crate::events::SimEvent;
use crate::mission::{Action, Condition, GameMode, ObjectiveState, Outcome};
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
        Condition::All(v) => v.iter().all(|c| check(world, data, c)),
        Condition::Any(v) => v.iter().any(|c| check(world, data, c)),
    }
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
            world.record(ChronicleKind::Victory, "Brennhollow hielt stand. Die Aschmark erinnert sich an diesen Sieg.");
        }
        Outcome::Defeat => {
            world.events.push(SimEvent::Defeat);
            world.record(ChronicleKind::Defeat, "Brennhollow fiel in Asche und Schweigen.");
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
    let keep_alive = world.building_with_tag(&defeat_tag).is_some();
    let anything_left = world.buildings.iter().any(|(_, b)| b.owner == PLAYER) || world.units.iter().any(|(_, u)| u.owner == PLAYER);
    if mode != GameMode::Sandbox && (!keep_alive || !anything_left) {
        finish(world, Outcome::Defeat);
    }
    if mode == GameMode::Skirmish && world.mission.dead_tags.contains(&world.mission_def.enemy_base_tag) {
        finish(world, Outcome::Victory);
    }
}
