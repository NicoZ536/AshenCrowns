//! Balance des Echtzeit-Gefechts: Schwierigkeitsgrade gegen den skriptgesteuerten Testspieler und Konter-Matrix.
//! Messwerte siehe docs/design/rts_military.md; Werkzeug: `ashen_tools rts-balance`.

use ashen_sim::balance::{duel, play_match};
use ashen_sim::data::GameData;
use ashen_sim::mission::{Difficulty, MissionDef};
use std::path::PathBuf;

fn assets() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/data")
}

fn load() -> (GameData, MissionDef) {
    let data = GameData::load_dir(&assets()).expect("data");
    let mission = MissionDef::load(&assets().join("missions/m01_last_settlement.ron")).expect("mission");
    (data, mission)
}

/// Punktquote des Testspielers über mehrere Seeds (Sieg 1, Zeitlimit 0,5, Niederlage 0): (Siege, Niederlagen, Quote).
fn win_rate(data: &GameData, mission: &MissionDef, d: Difficulty, seeds: u64) -> (u32, u32, f32) {
    let handles: Vec<_> = (1..=seeds)
        .map(|s| {
            let (data, mission) = (data.clone(), mission.clone());
            std::thread::Builder::new().stack_size(32 << 20).spawn(move || play_match(&data, &mission, d, s, 60.0 * 30.0).expect("Gefecht")).unwrap()
        })
        .collect();
    let reports: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let (mut won, mut lost) = (0, 0);
    for r in &reports {
        println!("{:?} Seed {}: {:?} nach {:.0} s, Verluste Spieler {} / KI {}, Angriffe der KI {}", d, r.seed, r.player_won, r.seconds, r.player_units_lost, r.enemy_units_lost, r.attacks);
        match r.player_won {
            Some(true) => won += 1,
            Some(false) => lost += 1,
            None => {}
        }
    }
    (won, lost, (won as f32 + 0.5 * (seeds as u32 - won - lost) as f32) / seeds as f32)
}

#[test]
fn difficulty_levels_scale_against_an_active_player() {
    let (data, mission) = load();
    let seeds = 12;
    let easy = win_rate(&data, &mission, Difficulty::Easy, seeds);
    let normal = win_rate(&data, &mission, Difficulty::Normal, seeds);
    let hard = win_rate(&data, &mission, Difficulty::Hard, seeds);
    let night = win_rate(&data, &mission, Difficulty::Nightmare, seeds);
    println!("Siegquote des Testspielers: Leicht {:.0} %, Normal {:.0} %, Schwer {:.0} %, Albtraum {:.0} %", easy.2 * 100.0, normal.2 * 100.0, hard.2 * 100.0, night.2 * 100.0);
    // Leicht: der aktive Spieler gewinnt meist; Normal ausgeglichen; Schwer und Albtraum gewinnt meist die KI.
    assert!(easy.2 >= 0.66, "Leicht: Spieler gewinnt meist ({:.2})", easy.2);
    assert!((0.25..=0.75).contains(&normal.2), "Normal ausgeglichen ({:.2})", normal.2);
    assert!(hard.2 <= 0.4, "Schwer: Spieler gewinnt selten ({:.2})", hard.2);
    assert!(night.2 <= 0.25, "Albtraum: Spieler gewinnt selten ({:.2})", night.2);
    assert!(easy.2 >= normal.2 && normal.2 >= hard.2 && hard.2 >= night.2, "Schwierigkeit steigt monoton");
}

#[test]
fn counter_matrix_has_no_extreme_outliers_at_equal_cost() {
    let (data, mission) = load();
    // Gleich teure Gruppen: kein Typ vernichtet einen anderen nahezu verlustfrei (Restanteil > 0,9 bei völliger Vernichtung).
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
        let r = duel(&data, &mission, a, b, 800.0).expect("Duell");
        println!("{a} ×{} gegen {b} ×{}: Rest {:.2} gegen {:.2}", r.count_a, r.count_b, r.left_a, r.left_b);
        assert!(r.margin().abs() < 0.85, "Ausreißer {a} gegen {b}: {r:?}");
    }
}
