//! Sim-Benchmark: Schlacht mit n Einheiten je Seite, Zeit je Tick und je Systemphase.
//!
//! `cargo run --release -p ashen_sim --example sim_bench -- [einheiten_je_seite=200] [sekunden=60]`

use ashen_sim::bench::{populate_battle, run_battle};
use ashen_sim::data::GameData;
use ashen_sim::mission::{Difficulty, GameMode, MatchSettings, MissionDef};
use ashen_sim::world::{SimWorld, STEP_PHASES};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let per_side: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(200);
    let seconds: f32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(60.0);
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/data");
    let data = GameData::load_dir(&assets).expect("Daten");
    let mission = MissionDef::load(&assets.join("missions/m01_last_settlement.ron")).expect("Mission");
    let mut world = SimWorld::new(&data, mission, MatchSettings { mode: GameMode::Sandbox, difficulty: Difficulty::Normal, ..Default::default() }).expect("Welt");
    let n = populate_battle(&mut world, &data, per_side);
    println!("Sim-Benchmark: {n} Einheiten ({per_side} je Seite), {seconds} s Spielzeit, Tickrate {}", data.rules.tick_rate);
    let r = run_battle(&mut world, &data, seconds);
    let budget = 1000.0 / data.rules.tick_rate as f64;
    println!("Einheiten am Ende: {} (Start {})", r.units_end, r.units_start);
    println!("Tick: mittel {:.3} ms, p50 {:.3}, p95 {:.3}, p99 {:.3}, max {:.3} (Budget bei {} Hz: {:.1} ms)", r.mean_ms(), r.quantile_ms(0.5), r.quantile_ms(0.95), r.quantile_ms(0.99), r.quantile_ms(1.0), data.rules.tick_rate, budget);
    let total: f64 = r.profile.secs.iter().sum();
    for (name, s) in STEP_PHASES.iter().zip(r.profile.secs) {
        println!("  {:<42} {:>8.3} ms/Tick  {:>5.1} %", name, s * 1000.0 / r.ticks.max(1) as f64, 100.0 * s / total.max(1e-12));
    }
}
