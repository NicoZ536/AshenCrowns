//! `ashen_tools strategy-stats --seeds 12 --years 200`: Balancemessung der Strategieebene.
//!
//! Lässt reine KI-Beobachterläufe über mehrere Seeds laufen und zählt, wie oft und wann eine KI gewinnt.

use ashen_strategy::observe::{observe, summarize};
use std::process::ExitCode;

pub fn run(args: &[String]) -> ExitCode {
    let mut seeds = 12u64;
    let mut years = 200u32;
    let mut first = 1u64;
    let mut i = 0;
    while i < args.len() {
        let value = args.get(i + 1).and_then(|v| v.parse::<u64>().ok());
        match (args[i].as_str(), value) {
            ("--seeds", Some(v)) => seeds = v,
            ("--years", Some(v)) => years = v as u32,
            ("--first-seed", Some(v)) => first = v,
            (other, _) => {
                eprintln!("Ungültige Option {other}. Verwendung: strategy-stats [--seeds N] [--years N] [--first-seed N]");
                return ExitCode::from(2);
            }
        }
        i += 2;
    }
    let threads: Vec<_> = (first..first + seeds).map(|s| std::thread::Builder::new().stack_size(64 << 20).spawn(move || observe(s, years)).expect("Thread")).collect();
    let runs: Vec<_> = threads.into_iter().map(|t| t.join().expect("Lauf")).collect();
    println!("Beobachterläufe: {seeds} Seeds ab {first}, höchstens {years} Jahre");
    for r in &runs {
        match &r.victory {
            Some((k, realm, year)) => println!("  Seed {:>3}: SIEG {} durch {realm} im Jahr {year} n.F. (nach {} Jahren)", r.seed, k.name(), r.victory_after_years().unwrap_or(0)),
            None => println!("  Seed {:>3}: kein Sieg in {} Jahren, {} Reiche am Leben", r.seed, r.years, r.realms_alive),
        }
    }
    let s = summarize(&runs);
    println!("Siege: {} von {} Läufen ({:.0} %), früheste Entscheidung nach {} Jahren", s.victories, s.runs, 100.0 * s.victories as f32 / s.runs.max(1) as f32, s.earliest.map_or("–".to_string(), |e| e.to_string()));
    for (k, n) in s.by_kind.iter().filter(|(_, n)| *n > 0) {
        println!("  {}: {n}", k.name());
    }
    ExitCode::SUCCESS
}
