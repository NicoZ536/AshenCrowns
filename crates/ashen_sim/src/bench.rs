//! Reproduzierbare Last für Leistungsmessungen: Schlacht mit n Einheiten je Seite im offenen Gelände.
//!
//! Wird vom Beispiel `sim_bench` (`cargo run --release -p ashen_sim --example sim_bench`), vom Rauchtest
//! `tests/bench_smoke.rs` und vom Spiel-Benchmark (`ashen_crowns --bench stress`) verwendet.

use crate::data::GameData;
use crate::entity::{Order, ENEMY, PLAYER};
use crate::world::{SimWorld, StepProfile};
use glam::Vec2;

const PLAYER_MIX: [&str; 9] = ["swordsman", "spearman", "archer", "crossbowman", "halberdier", "shieldbearer", "knight", "battle_mage", "priest"];
const ENEMY_MIX: [&str; 6] = ["hollow_thrall", "hollow_archer", "ghoul", "ash_witch", "grave_brute", "bat_swarm"];

/// Setzt `per_side` Einheiten je Seite in zwei Blöcke links und rechts der Kartenmitte und befiehlt Angriffsmarsch
/// aufeinander. Gibt die Anzahl erzeugter Einheiten zurück. Deterministisch (feste Mischung, Gitteranordnung).
pub fn populate_battle(world: &mut SimWorld, data: &GameData, per_side: usize) -> usize {
    let mid = Vec2::new(world.map.width as f32 * 0.5, world.map.height as f32 * 0.5);
    let cols = ((per_side as f32).sqrt().ceil() as usize).max(1);
    let mut spawned = 0;
    for (owner, mix, x0, target) in [(PLAYER, &PLAYER_MIX[..], mid.x - 12.0, mid.x + 12.0), (ENEMY, &ENEMY_MIX[..], mid.x + 12.0, mid.x - 12.0)] {
        for i in 0..per_side {
            let Some(kind) = data.unit_id(mix[i % mix.len()]) else { continue };
            let (cx, cy) = ((i % cols) as f32, (i / cols) as f32);
            let dir = if owner == PLAYER { -1.0 } else { 1.0 };
            let pos = Vec2::new(x0 + dir * cx * 0.9, mid.y - cols as f32 * 0.45 + cy * 0.9);
            let id = world.spawn_unit(data, kind, owner, pos);
            let tgt = Vec2::new(target, pos.y);
            if let Some(u) = world.units.get_mut(id) {
                u.order = Order::Move { target: tgt, attack_move: true };
                u.request_path(tgt);
            }
            spawned += 1;
        }
    }
    spawned
}

/// Ergebnis eines Sim-Benchmarks.
#[derive(Clone, Debug)]
pub struct SimBenchResult {
    pub ticks: u32,
    pub units_start: usize,
    pub units_end: usize,
    /// Dauer je Tick in Millisekunden, nach Tick-Nummer.
    pub tick_ms: Vec<f64>,
    pub profile: StepProfile,
}

impl SimBenchResult {
    pub fn mean_ms(&self) -> f64 {
        self.tick_ms.iter().sum::<f64>() / self.tick_ms.len().max(1) as f64
    }

    /// p-Quantil (0..1) der Tickdauer in Millisekunden.
    pub fn quantile_ms(&self, q: f64) -> f64 {
        let mut v = self.tick_ms.clone();
        v.sort_by(|a, b| a.total_cmp(b));
        v.get(((v.len() as f64 - 1.0) * q).round() as usize).copied().unwrap_or(0.0)
    }
}

/// Lässt die Schlacht `seconds` Spielsekunden laufen und misst jeden Tick.
pub fn run_battle(world: &mut SimWorld, data: &GameData, seconds: f32) -> SimBenchResult {
    let ticks = (seconds * data.rules.tick_rate as f32) as u32;
    let units_start = world.units.iter().count();
    let mut prof = StepProfile::default();
    let mut tick_ms = Vec::with_capacity(ticks as usize);
    for _ in 0..ticks {
        let t0 = std::time::Instant::now();
        world.step_profiled(data, &mut prof);
        tick_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
        world.events.clear();
    }
    SimBenchResult { ticks, units_start, units_end: world.units.iter().count(), tick_ms, profile: prof }
}
