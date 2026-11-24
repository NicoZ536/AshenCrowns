//! Rauchtest der Benchmark-Last: 400 Einheiten im Kampf (siehe `ashen_sim::bench`, Beispiel `sim_bench`).

use ashen_sim::bench::{populate_battle, run_battle};
use ashen_sim::data::GameData;
use ashen_sim::mission::{Difficulty, GameMode, MatchSettings, MissionDef};
use ashen_sim::world::SimWorld;
use std::path::PathBuf;

fn world() -> (GameData, SimWorld) {
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/data");
    let data = GameData::load_dir(&assets).expect("data");
    let mission = MissionDef::load(&assets.join("missions/m01_last_settlement.ron")).expect("mission");
    let world = SimWorld::new(&data, mission, MatchSettings { mode: GameMode::Sandbox, difficulty: Difficulty::Normal, ..Default::default() }).expect("world");
    (data, world)
}

#[test]
fn four_hundred_units_fight_within_tick_budget() {
    let (data, mut w) = world();
    let n = populate_battle(&mut w, &data, 200);
    assert_eq!(n, 400);
    let r = run_battle(&mut w, &data, 30.0);
    assert_eq!(r.ticks, 30 * data.rules.tick_rate);
    assert!(r.units_end < r.units_start, "es muss gekämpft werden: {} -> {}", r.units_start, r.units_end);
    // Budget bei 20 Hz: 50 ms je Tick; großzügige Grenze für das Debug-Profil und langsame CI-Rechner.
    assert!(r.mean_ms() < 50.0, "mittlere Tickzeit {:.2} ms", r.mean_ms());
    assert_eq!(r.profile.steps, r.ticks);
}

#[test]
fn benchmark_battle_is_deterministic() {
    let hash = || {
        let (data, mut w) = world();
        populate_battle(&mut w, &data, 60);
        run_battle(&mut w, &data, 5.0);
        w.state_hash()
    };
    assert_eq!(hash(), hash());
}
