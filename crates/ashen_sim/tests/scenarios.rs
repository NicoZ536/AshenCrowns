//! Tests der mitgelieferten Szenarien, Herausforderungen und Missionsmodifikatoren.

use ashen_sim::challenge::{self, ChallengeDef, Modifier};
use ashen_sim::command::{Command, PlayerCommand};
use ashen_sim::data::{GameData, UnitClass};
use ashen_sim::entity::{EntityRef, UnitId, ENEMY, PLAYER};
use ashen_sim::mission::{Condition, GameMode, MatchSettings, MissionDef, Outcome};
use ashen_sim::scenario::{self, ScenarioSource};
use ashen_sim::world::SimWorld;
use glam::Vec2;
use std::path::PathBuf;

fn assets() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/data")
}

fn data() -> GameData {
    GameData::load_dir(&assets()).expect("data")
}

fn scenarios() -> Vec<scenario::ScenarioEntry> {
    let mut errors = Vec::new();
    let list = scenario::discover(&[(assets().join("scenarios"), ScenarioSource::Bundled)], &mut errors);
    assert!(errors.is_empty(), "{errors:?}");
    list
}

fn challenges() -> Vec<ChallengeDef> {
    let mut errors = Vec::new();
    let list = ChallengeDef::load_dir(&assets().join("challenges"), &mut errors);
    assert!(errors.is_empty(), "{errors:?}");
    list
}

fn load_mission(id: &str) -> MissionDef {
    if let Some(s) = scenarios().into_iter().find(|s| s.def.id == id) {
        return s.def;
    }
    MissionDef::load(&assets().join(format!("missions/{id}.ron"))).expect("mission")
}

fn world(def: &MissionDef) -> SimWorld {
    SimWorld::new(&data(), def.clone(), MatchSettings { mode: GameMode::Campaign, ..Default::default() }).expect("world")
}

fn military(w: &SimWorld, d: &GameData) -> Vec<UnitId> {
    w.units.iter().filter(|(_, u)| u.owner == PLAYER && d.unit(u.kind).class != UnitClass::Worker).map(|(id, _)| id).collect()
}

fn run_ticks(w: &mut SimWorld, d: &GameData, ticks: u32, mut each: impl FnMut(&mut SimWorld, u32)) {
    for t in 0..ticks {
        each(w, t);
        w.step(d);
        w.events.clear();
        if w.mission.outcome.is_some() {
            break;
        }
    }
}

#[derive(Clone, Copy)]
enum Plan {
    /// Streitmacht hält die Mitte der Siedlung und bekämpft Angreifer in der Nähe.
    Defend,
    /// Alle Einheiten ziehen zu einem Bereich; Soldaten kämpfen sich voran.
    Escort(&'static str),
    /// Truppen ausbilden und gegen das Ziel (Bereich) marschieren.
    Assault(&'static str),
}

/// Einfacher Testspieler: bildet Soldaten aus und führt sie gemäß Plan.
fn bot(w: &mut SimWorld, d: &GameData, t: u32, plan: Plan) {
    if !t.is_multiple_of(40) {
        return;
    }
    // Ausbildung.
    let kinds = ["swordsman", "archer", "spearman"];
    let barracks: Vec<_> = w.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.complete && d.building(b.kind).id == "barracks" && b.queue.len() < 2).map(|(id, _)| id).collect();
    for (i, b) in barracks.into_iter().enumerate() {
        let unit = d.unit_id(kinds[(t as usize / 40 + i) % 3]).unwrap();
        w.push_command(PlayerCommand { player: PLAYER, command: Command::Train { building: b, unit } });
    }
    let army = military(w, d);
    if army.is_empty() {
        return;
    }
    let center = |ids: &[UnitId], w: &SimWorld| ids.iter().filter_map(|i| w.units.get(*i)).map(|u| u.pos).sum::<Vec2>() / ids.len() as f32;
    match plan {
        Plan::Defend => {
            let home = w.building_with_tag("player_keep").and_then(|b| w.buildings.get(b)).map(|b| b.center()).unwrap_or(center(&army, w));
            let threat = w.units.iter().filter(|(_, u)| u.owner == ENEMY && (u.pos - home).length() < 24.0).map(|(_, u)| u.pos).min_by(|a, b| (*a - home).length().total_cmp(&(*b - home).length()));
            let target = threat.map(|p| home + (p - home).clamp_length_max(9.0)).unwrap_or(home + Vec2::new(0.0, 3.0));
            w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: army, target, attack_move: true, queue: false } });
        }
        Plan::Escort(area) => {
            let a = w.mission_def.area(area).expect("Bereich").center();
            let workers: Vec<UnitId> = w.units.iter().filter(|(_, u)| u.owner == PLAYER && d.unit(u.kind).class == UnitClass::Worker).map(|(id, _)| id).collect();
            // Die Soldaten brechen den Weg (Angriffsbewegung), die Flüchtlinge folgen dem Heer.
            let front = center(&army, w);
            w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: army, target: a, attack_move: true, queue: false } });
            w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: workers, target: front, attack_move: false, queue: false } });
        }
        Plan::Assault(area) => {
            let a = w.mission_def.area(area).expect("Bereich").center();
            // Erst sammeln, dann in einem Zug vorrücken.
            let need = 20;
            if army.len() >= need || w.time > 500.0 {
                w.push_command(PlayerCommand { player: PLAYER, command: Command::Move { units: army, target: a, attack_move: true, queue: false } });
            }
        }
    }
}

fn play(def: &MissionDef, plan: Plan, seconds: u32) -> SimWorld {
    let d = data();
    let mut w = world(def);
    run_ticks(&mut w, &d, seconds * 20, |w, t| bot(w, &d, t, plan));
    let enemies = w.units.iter().filter(|(_, u)| u.owner == ENEMY).count();
    eprintln!("{}: Ergebnis {:?} nach {:.0}s, Verluste {}, Gefallene Gegner {}, Gegner übrig {}, Soldaten {}", def.id, w.mission.outcome, w.time, w.players[0].stats.units_lost, w.players[0].stats.kills, enemies, military(&w, &d).len());
    w
}

#[test]
fn bundled_scenarios_are_well_formed() {
    let d = data();
    let list = scenarios();
    assert!(list.len() >= 3, "mindestens drei historische Szenarien, gefunden {}", list.len());
    for s in &list {
        let def = &s.def;
        assert!(!def.briefing.is_empty() && !def.description.is_empty(), "{}: Briefing", def.id);
        assert!(def.year.is_some(), "{}: Jahr", def.id);
        assert!(!def.objectives.is_empty() && !def.triggers.is_empty(), "{}: Ziele/Trigger", def.id);
        let w = world(def);
        assert!(w.map.width >= 48);
        // Es gibt einen Siegauslöser.
        assert!(def.triggers.iter().any(|t| t.actions.iter().any(|a| matches!(a, ashen_sim::mission::Action::Victory))), "{}: kein Sieg", def.id);
        // Alle Bereiche und Tags der Auslöser existieren.
        fn check(def: &MissionDef, c: &Condition) {
            match c {
                Condition::UnitsInArea { area, .. } => assert!(def.area(area).is_some(), "{}: Bereich {area}", def.id),
                Condition::TagDead(t) | Condition::TagDiscovered(t) | Condition::TagDamaged(t) => {
                    assert!(def.placements.iter().any(|p| p.tag.as_deref() == Some(t.as_str())), "{}: Tag {t}", def.id)
                }
                Condition::All(v) | Condition::Any(v) => v.iter().for_each(|c| check(def, c)),
                Condition::Not(c) => check(def, c),
                _ => {}
            }
        }
        for t in &def.triggers {
            check(def, &t.condition);
            for a in &t.actions {
                if let ashen_sim::mission::Action::SpawnUnits { area, unit, .. } = a {
                    assert!(def.area(area).is_some(), "{}: Spawnbereich {area}", def.id);
                    assert!(d.unit_id(unit).is_some());
                }
            }
        }
        // Startposition ist begehbar und mit allen Gegnerstarts verbunden.
        let mut pf = ashen_sim::path::Pathfinder::default();
        for st in def.starts.iter().filter(|s| s.owner == 0) {
            for en in def.starts.iter().filter(|s| s.owner != 0) {
                let from = w.map.nearest_walkable(glam::IVec2::new(st.x as i32, st.y as i32), 6).expect("Start begehbar");
                let to = w.map.nearest_walkable(glam::IVec2::new(en.x as i32, en.y as i32), 6).expect("Gegnerstart begehbar");
                let goal = ashen_sim::map::TileMap::center_of(to);
                let p = pf.find_for(&w.map, ashen_sim::map::TileMap::center_of(from), goal, 200_000, PLAYER);
                assert!(p.is_some_and(|p| p.last().is_some_and(|e| (*e - goal).length() < 1.6)), "{}: Start nicht mit Gegnerstart verbunden", def.id);
            }
        }
    }
}

#[test]
fn scenarios_are_not_lost_when_idle() {
    let d = data();
    for s in scenarios() {
        let mut w = world(&s.def);
        run_ticks(&mut w, &d, 20 * 45, |_, _| {});
        assert_ne!(w.mission.outcome, Some(Outcome::Defeat), "{}: nach 45 s untätig nicht verloren", s.def.id);
        assert!(w.units.iter().any(|(_, u)| u.owner == PLAYER), "{}: Spielereinheiten", s.def.id);
    }
}

#[test]
fn grauwall_is_winnable_by_defending() {
    let def = load_mission("sc_fall_of_grauwall");
    let w = play(&def, Plan::Defend, 330);
    assert_eq!(w.mission.outcome, Some(Outcome::Victory), "Grauwall: Zeit {:.0}s, Spielerverluste {}", w.time, w.players[0].stats.units_lost);
    assert!(w.players[0].stats.kills >= 30, "echte Schlacht: {} Gefallene Hohle", w.players[0].stats.kills);
}

#[test]
fn grauwall_is_not_trivial() {
    // Ohne Eingreifen (nur Eigenverteidigung der Einheiten auf ihren Plätzen) darf die Nacht nicht folgenlos bleiben.
    let def = load_mission("sc_fall_of_grauwall");
    let d = data();
    let mut w = world(&def);
    run_ticks(&mut w, &d, 20 * 330, |_, _| {});
    let lost = w.players[0].stats.units_lost;
    let keep_hp = w.building_with_tag("player_keep").and_then(|b| w.buildings.get(b)).map(|b| b.hp / b.max_hp).unwrap_or(0.0);
    assert!(lost >= 3 || keep_hp < 0.95 || w.mission.outcome == Some(Outcome::Defeat), "Angriffe haben Wirkung: Verluste {lost}, Bergfried {keep_hp:.2}");
}

#[test]
fn flight_to_vael_dorn_is_winnable() {
    let def = load_mission("sc_flight_to_vael_dorn");
    let w = play(&def, Plan::Escort("vael_dorn"), 900);
    assert_eq!(w.mission.outcome, Some(Outcome::Victory), "Flucht: Zeit {:.0}s, Verluste {}", w.time, w.players[0].stats.units_lost);
}

#[test]
fn siege_of_kaltherd_is_winnable() {
    let def = load_mission("sc_siege_of_kaltherd");
    let w = play(&def, Plan::Defend, 760);
    assert_eq!(w.mission.outcome, Some(Outcome::Victory), "Kaltherd: Zeit {:.0}s, Verluste {}", w.time, w.players[0].stats.units_lost);
}

#[test]
fn banner_of_grauwall_is_winnable() {
    let def = load_mission("sc_banner_of_grauwall");
    let w = play(&def, Plan::Assault("grauwall"), 1500);
    assert_eq!(w.mission.outcome, Some(Outcome::Victory), "Grauwall-Rückeroberung: Zeit {:.0}s, Verluste {}", w.time, w.players[0].stats.units_lost);
}

// ------------------------------------------------------------------------------ Herausforderungen

#[test]
fn at_least_six_challenges_with_valid_bases() {
    let list = challenges();
    assert!(list.len() >= 6, "mindestens sechs Herausforderungen, gefunden {}", list.len());
    let mut ids: Vec<_> = list.iter().map(|c| c.id.clone()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), list.len(), "eindeutige IDs");
    let d = data();
    let missions: Vec<MissionDef> = std::fs::read_dir(assets().join("missions")).unwrap().flatten().filter_map(|e| MissionDef::load(&e.path()).ok()).chain(scenarios().into_iter().map(|s| s.def)).collect();
    for c in &list {
        let base = missions.iter().find(|m| m.id == c.base).unwrap_or_else(|| panic!("{}: Basis {} fehlt", c.id, c.base));
        let p = challenge::prepare(c, base);
        let w = SimWorld::new(&d, p.mission, p.settings).unwrap_or_else(|e| panic!("{}: {e}", c.id));
        assert!(w.mission.outcome.is_none(), "{}: startet nicht sofort beendet", c.id);
    }
}

fn base_def(id: &str, base: &MissionDef, mods: Vec<Modifier>, victory: Option<Condition>) -> (ChallengeDef, challenge::PreparedChallenge) {
    let def = ChallengeDef {
        id: id.into(),
        name: id.into(),
        description: String::new(),
        base: base.id.clone(),
        mode: GameMode::Campaign,
        difficulty: ashen_sim::mission::Difficulty::Normal,
        seed: 0,
        modifiers: mods,
        victory,
        goal_text: String::new(),
        scoring: Default::default(),
    };
    let p = challenge::prepare(&def, base);
    (def, p)
}

#[test]
fn modifier_unit_cap_blocks_training() {
    let d = data();
    let base = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).unwrap();
    let (_, p) = base_def("cap", &base, vec![Modifier::UnitCap(4)], None);
    let mut w = SimWorld::new(&d, p.mission, p.settings).unwrap();
    // Eine Kaserne und viel Gold, damit nur die Obergrenze entscheidet.
    let barracks = d.building_id("barracks").unwrap();
    let b = w.place_building(&d, barracks, PLAYER, glam::IVec2::new(20, 58), true);
    w.players[0].resources.gold = 5000.0;
    w.players[0].resources.wood = 5000.0;
    w.players[0].resources.stone = 5000.0;
    for g in w.players[0].goods.iter_mut() {
        *g = 100.0;
    }
    let before = military(&w, &d).len();
    assert_eq!(before, 4, "3 Schwertkämpfer + Heldin");
    let before_mil = before - 1;
    let sword = d.unit_id("swordsman").unwrap();
    for _ in 0..4 {
        w.push_command(PlayerCommand { player: PLAYER, command: Command::Train { building: b, unit: sword } });
    }
    w.step(&d);
    let queued = w.buildings.get(b).unwrap().queue.len();
    assert_eq!(queued, 4 - before_mil, "nur {} weitere Soldaten ausbildbar", 4 - before_mil);
    assert!(w.events.iter().any(|e| matches!(e, ashen_sim::SimEvent::Error { owner: PLAYER, .. })));
}

#[test]
fn free_economy_disables_upkeep_and_population_cap() {
    let d = data();
    let base = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).unwrap();
    let barracks = d.building_id("barracks").unwrap();
    let sword = d.unit_id("swordsman").unwrap();
    for free in [false, true] {
        let mut def = base.clone();
        def.free_economy = free;
        let mut w = world(&def);
        w.players[0].resources.food = 0.0;
        w.players[0].resources.gold = 5000.0;
        w.players[0].resources.wood = 5000.0;
        w.players[0].resources.stone = 5000.0;
        for g in w.players[0].goods.iter_mut() {
            *g = 100.0;
        }
        let b = w.place_building(&d, barracks, PLAYER, glam::IVec2::new(20, 58), true);
        // Bevölkerung am Limit: Ausbildung scheitert nur mit Bevölkerungslimit.
        w.players[0].pop = w.players[0].pop_cap;
        w.push_command(PlayerCommand { player: PLAYER, command: Command::Train { building: b, unit: sword } });
        for _ in 0..40 {
            w.step(&d);
        }
        w.players[0].pop = w.players[0].pop_cap;
        let queued = w.buildings.get(b).map(|b| b.queue.len()).unwrap_or(0);
        let upkeep: f32 = w.units.iter().filter(|(_, u)| u.owner == PLAYER).map(|(_, u)| d.unit(u.kind).upkeep).sum();
        assert!(upkeep > 0.0, "Testwelt braucht Einheiten mit Unterhalt");
        if free {
            assert!(!w.players[0].starving, "keine Hungersnot bei free_economy");
            assert!(queued > 0 || w.players[0].stats.units_trained > 0, "Ausbildung trotz Bevölkerungslimit");
        } else {
            assert!(w.players[0].starving, "ohne free_economy Hungersnot bei 0 Nahrung");
            assert_eq!(queued, 0, "Bevölkerungslimit blockiert die Ausbildung");
        }
    }
}

#[test]
fn modifiers_scale_waves_resources_and_remove_hero() {
    let d = data();
    let base = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).unwrap();
    let (_, p) = base_def("m", &base, vec![Modifier::WaveStrength(2.0), Modifier::WaveInterval(0.5), Modifier::StartResources(0.5), Modifier::NoHero, Modifier::HeroPermadeath], None);
    assert!((p.mission.waves.base_budget - base.waves.base_budget * 2.0).abs() < 1e-4);
    assert!((p.mission.waves.interval_night - base.waves.interval_night * 0.5).abs() < 1e-4);
    assert!((p.mission.start_resources.gold - base.start_resources.gold * 0.5).abs() < 1e-4);
    assert!(p.settings.hero_permadeath);
    let w = SimWorld::new(&d, p.mission, p.settings).unwrap();
    assert!(w.hero_of(&d, PLAYER).is_none(), "ohne Heldin");
    assert!((w.players[0].resources.gold - 150.0).abs() < 1e-3);
}

#[test]
fn modifier_time_limit_and_keep_intact_cause_defeat() {
    let d = data();
    let base = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).unwrap();
    let (_, p) = base_def("t", &base, vec![Modifier::TimeLimit(30.0)], None);
    let mut w = SimWorld::new(&d, p.mission, p.settings).unwrap();
    run_ticks(&mut w, &d, 20 * 40, |_, _| {});
    assert_eq!(w.mission.outcome, Some(Outcome::Defeat), "Zeitlimit");

    let (_, p) = base_def("k", &base, vec![Modifier::KeepIntact], None);
    let mut w = SimWorld::new(&d, p.mission, p.settings).unwrap();
    run_ticks(&mut w, &d, 20 * 5, |_, _| {});
    assert!(w.mission.outcome.is_none());
    let keep = w.building_with_tag("player_keep").unwrap();
    w.buildings.get_mut(keep).unwrap().hp -= 5.0;
    run_ticks(&mut w, &d, 20 * 2, |_, _| {});
    assert_eq!(w.mission.outcome, Some(Outcome::Defeat), "beschädigter Bergfried");
}

#[test]
fn challenge_victory_condition_scores_and_ranks() {
    let d = data();
    let base = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).unwrap();
    let (def, p) = base_def("v", &base, vec![], Some(Condition::AtTime(10.0)));
    let mut w = SimWorld::new(&d, p.mission, p.settings).unwrap();
    assert!(challenge::evaluate(&def, &w).is_none());
    run_ticks(&mut w, &d, 20 * 15, |_, _| {});
    let r = challenge::evaluate(&def, &w).expect("Ergebnis");
    assert!(r.victory && r.score >= 1000, "{r:?}");
}

#[test]
fn skirmish_hands_difficulty_through() {
    let d = data();
    let base = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).unwrap();
    for diff in [ashen_sim::mission::Difficulty::Easy, ashen_sim::mission::Difficulty::Hard] {
        let w = SimWorld::new(&d, base.clone(), MatchSettings { mode: GameMode::Skirmish, difficulty: diff, ..Default::default() }).unwrap();
        assert_eq!(w.settings.difficulty, diff);
    }
    let _ = EntityRef::Unit;
}
