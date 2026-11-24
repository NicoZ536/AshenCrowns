//! `ashen_tools rts-balance [--seeds 12] [--minutes 30]`: Gefecht-KI gegen den skriptgesteuerten Testspieler
//! und Konter-Matrix (1v1-Gruppenduelle gleicher Kosten). Siehe docs/design/rts_military.md.

use ashen_sim::balance::{duel, play_match};
use ashen_sim::data::GameData;
use ashen_sim::mission::{Difficulty, MissionDef};
use std::path::Path;
use std::process::ExitCode;

pub fn run(assets: &Path, args: &[String]) -> ExitCode {
    let mut seeds = 12u64;
    let mut minutes = 30.0f32;
    let mut i = 0;
    while i < args.len() {
        match (args[i].as_str(), args.get(i + 1).and_then(|v| v.parse::<f32>().ok())) {
            ("--seeds", Some(v)) => seeds = v as u64,
            ("--minutes", Some(v)) => minutes = v,
            (other, _) => {
                eprintln!("Ungültige Option {other}. Verwendung: rts-balance [--seeds N] [--minutes N]");
                return ExitCode::from(2);
            }
        }
        i += 2;
    }
    let data = match GameData::load_dir(&assets.join("data")) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Fehler: {e}");
            return ExitCode::FAILURE;
        }
    };
    let mission = match MissionDef::load(&assets.join("data/missions/m01_last_settlement.ron")) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Fehler: {e}");
            return ExitCode::FAILURE;
        }
    };
    println!("Testspieler gegen Gefecht-KI: {seeds} Seeds je Stufe, höchstens {minutes:.0} Minuten");
    for d in [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard, Difficulty::Nightmare] {
        let handles: Vec<_> = (1..=seeds)
            .map(|s| {
                let (data, mission) = (data.clone(), mission.clone());
                std::thread::Builder::new().stack_size(32 << 20).spawn(move || play_match(&data, &mission, d, s, minutes * 60.0)).expect("Thread")
            })
            .collect();
        let reports: Vec<_> = handles.into_iter().filter_map(|h| h.join().ok().and_then(Result::ok)).collect();
        let won = reports.iter().filter(|r| r.player_won == Some(true)).count();
        let lost = reports.iter().filter(|r| r.player_won == Some(false)).count();
        let draws = reports.len() - won - lost;
        let mean = reports.iter().map(|r| r.seconds).sum::<f32>() / reports.len().max(1) as f32;
        println!("{:>8}: Spieler siegt {won:>2}, KI siegt {lost:>2}, Zeitlimit {draws:>2} (Punktquote {:.0} %, mittlere Dauer {:.0} s)", format!("{d:?}"), 100.0 * (won as f32 + 0.5 * draws as f32) / reports.len().max(1) as f32, mean);
    }
    println!("\nKonter-Matrix (Gruppen gleichen Gold-/Holz-/Steinwerts, Restlebenspunkte in %):");
    let pairs = [
        ("swordsman", "hollow_thrall"),
        ("archer", "hollow_archer"),
        ("spearman", "hollow_thrall"),
        ("knight", "ghoul"),
        ("halberdier", "grave_brute"),
        ("battle_mage", "ash_witch"),
        ("swordsman", "ghoul"),
        ("archer", "ghoul"),
        ("knight", "hollow_thrall"),
    ];
    for (a, b) in pairs {
        match duel(&data, &mission, a, b, 800.0) {
            Ok(r) => println!("  {a:>12} x{:<2} gegen {b:<14} x{:<2}: {:>3.0} % gegen {:>3.0} %", r.count_a, r.count_b, r.left_a.max(0.0) * 100.0, r.left_b.max(0.0) * 100.0),
            Err(e) => println!("  {a} gegen {b}: {e}"),
        }
    }
    ExitCode::SUCCESS
}
