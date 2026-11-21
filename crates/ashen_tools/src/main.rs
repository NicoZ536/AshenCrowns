//! ASHEN CROWNS – Entwicklerwerkzeuge.
//!
//! ```text
//! cargo run -p ashen_tools -- audio-scan [--assets <dir>] [--apply-renames] [--convert] [--report <datei.md>]
//! cargo run -p ashen_tools -- validate-data [--assets <dir>] [--mod <mod-verzeichnis> ...]
//! ```

use ashen_engine::audio::manifest::AudioManifest;
use ashen_engine::audio::scanner;
use ashen_sim::data::GameData;
use ashen_sim::mission::MissionDef;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn usage() -> &'static str {
    "ashen_tools <befehl> [optionen]

Befehle:
  audio-scan       Audiodateien scannen, dem Manifest zuordnen und prüfen
      --assets <dir>      Asset-Verzeichnis (Standard: automatisch gesucht)
      --apply-renames     Ungültige Dateinamen gemäß Vorschlag umbenennen (nur eindeutige Fälle)
      --convert           Nicht abspielbare Formate (m4a, aiff, …) per ffmpeg nach .ogg konvertieren
      --report <datei>    Bericht als Markdown schreiben (Standard: assets/audio/audio_report.md)
  validate-data    Spieldaten, Missionen und Mods laden und auf Konsistenz prüfen
      --assets <dir>
      --mod <dir>         Mod-Verzeichnis zusätzlich anwenden (mehrfach möglich)"
}

fn assets_dir(arg: Option<PathBuf>) -> Result<PathBuf, String> {
    arg.or_else(ashen_engine::assets::find_assets_dir).ok_or_else(|| "Asset-Verzeichnis nicht gefunden (--assets angeben)".to_string())
}

fn audio_scan(assets: &Path, apply: bool, convert: bool, report_path: Option<PathBuf>) -> Result<bool, String> {
    let root = assets.join("audio");
    let manifest = AudioManifest::load(&root.join("manifest.ron"))?;
    let mut report = scanner::scan(&root, &manifest);
    let mut changed = false;
    if apply && !report.invalid_names.is_empty() {
        for r in report.invalid_names.clone() {
            if r.to.exists() {
                eprintln!("übersprungen (Ziel existiert): {} → {}", r.from.display(), r.to.display());
                continue;
            }
            match std::fs::rename(&r.from, &r.to) {
                Ok(_) => {
                    println!("umbenannt: {} → {}", r.from.display(), r.to.display());
                    changed = true;
                }
                Err(e) => eprintln!("Umbenennen fehlgeschlagen ({}): {e}", r.from.display()),
            }
        }
    }
    if changed {
        report = scanner::scan(&root, &manifest);
    }
    if convert && !report.needs_conversion.is_empty() {
        for p in report.needs_conversion.clone() {
            match scanner::convert_to_ogg(&p) {
                Ok(out) => {
                    println!("konvertiert: {} → {}", p.display(), out.display());
                    changed = true;
                }
                Err(e) => eprintln!("Konvertierung fehlgeschlagen: {e}"),
            }
        }
    }
    if changed {
        report = scanner::scan(&root, &manifest);
    }
    let md = report.to_markdown();
    let out = report_path.unwrap_or_else(|| root.join("audio_report.md"));
    std::fs::write(&out, &md).map_err(|e| format!("{}: {e}", out.display()))?;
    println!(
        "Audio: {} Dateien, {} zugeordnet, {} Muster fehlen, {} ungültige Namen, {} Duplikate, {} mehrdeutig, {} beschädigt, {} unbekannt.\nBericht: {}",
        report.total_files,
        report.found_count(),
        report.missing.len(),
        report.invalid_names.len(),
        report.duplicates.len(),
        report.ambiguous.len(),
        report.corrupt.len(),
        report.unknown.len(),
        out.display()
    );
    if !report.ambiguous.is_empty() {
        println!("Mehrdeutige Zuordnungen müssen manuell geklärt werden (siehe Bericht).");
    }
    Ok(report.corrupt.is_empty())
}

fn validate_data(assets: &Path, mods: &[PathBuf]) -> Result<(), String> {
    let mut data = GameData::load_dir(&assets.join("data")).map_err(|e| e.to_string())?;
    println!("Basisdaten: {} Einheiten, {} Gebäude, {} Fähigkeiten", data.units.len(), data.buildings.len(), data.abilities.len());
    for m in mods {
        let changes = data.apply_overlay(&m.join("data")).map_err(|e| format!("Mod {}: {e}", m.display()))?;
        println!("Mod {}: {} Einträge", m.display(), changes.len());
    }
    let dir = assets.join("data/missions");
    let mut count = 0;
    for e in std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?.flatten() {
        let p = e.path();
        if p.extension().is_none_or(|x| x != "ron") {
            continue;
        }
        let m = MissionDef::load(&p)?;
        for pl in &m.placements {
            if data.unit_id(&pl.id).is_none() && data.building_id(&pl.id).is_none() {
                return Err(format!("{}: unbekannte Platzierung {}", p.display(), pl.id));
            }
        }
        // Mission einmal starten, um Platzierungen und Karte zu prüfen.
        ashen_sim::SimWorld::new(&data, m.clone(), Default::default()).map_err(|e| format!("{}: {e}", p.display()))?;
        println!("Mission {} ({}) ok: {} Ziele, {} Auslöser", m.id, m.name, m.objectives.len(), m.triggers.len());
        count += 1;
    }
    println!("{count} Mission(en) geprüft – alles konsistent.");
    Ok(())
}

fn main() -> ExitCode {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().cloned() else {
        println!("{}", usage());
        return ExitCode::from(2);
    };
    let mut assets = None;
    let mut apply = false;
    let mut convert = false;
    let mut report = None;
    let mut mods = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--assets" => {
                i += 1;
                assets = args.get(i).map(PathBuf::from);
            }
            "--apply-renames" => apply = true,
            "--convert" => convert = true,
            "--report" => {
                i += 1;
                report = args.get(i).map(PathBuf::from);
            }
            "--mod" => {
                i += 1;
                if let Some(m) = args.get(i) {
                    mods.push(PathBuf::from(m));
                }
            }
            other => {
                eprintln!("Unbekannte Option {other}\n{}", usage());
                return ExitCode::from(2);
            }
        }
        i += 1;
    }
    let assets = match assets_dir(assets) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let result = match cmd.as_str() {
        "audio-scan" => audio_scan(&assets, apply, convert, report).map(|ok| if ok {  } else { eprintln!("Beschädigte Audiodateien gefunden.") }),
        "validate-data" => validate_data(&assets, &mods),
        _ => {
            println!("{}", usage());
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Fehler: {e}");
            ExitCode::FAILURE
        }
    }
}
