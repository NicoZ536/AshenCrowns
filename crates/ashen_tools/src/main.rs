//! ASHEN CROWNS – Entwicklerwerkzeuge.
//!
//! ```text
//! cargo run -p ashen_tools -- audio-scan [--assets <dir>] [--apply-renames] [--convert] [--report <datei.md>]
//! cargo run -p ashen_tools -- validate-data [--assets <dir>] [--mod <mod-verzeichnis> ...]
//! cargo run -p ashen_tools -- forge-scenarios [--assets <dir>]
//! cargo run -p ashen_tools -- forge-campaign [--assets <dir>]
//! cargo run -p ashen_tools -- rts-balance [--seeds 12] [--minutes 30]
//! cargo run -p ashen_tools -- strategy-stats [--seeds 12] [--years 200] [--first-seed 1]
//! ```

mod campaign_forge;
mod campaign_missions;
mod campaign_missions_b;
mod campaign_missions_c;
mod forge;
mod rts_balance;
mod strategy_stats;

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
      --mod <dir>         Mod-Verzeichnis zusätzlich anwenden (mehrfach möglich)
  forge-scenarios  Die mitgelieferten historischen Szenarien neu erzeugen (assets/data/scenarios/)
      --assets <dir>
  forge-campaign   Die Echtzeit-Kampagne neu erzeugen (assets/data/campaign/, Audio-Manifest, Sprechskript)
      --assets <dir>
  rts-balance      Gefecht-KI gegen skriptgesteuerten Testspieler und Konter-Matrix messen
      --seeds <n> --minutes <n>   (Standard: 12 Seeds, 30 Minuten)
  strategy-stats   Balancemessung der Strategieebene: KI-Beobachterläufe, Siegquote und -zeitpunkt
      --seeds <n> --years <n> --first-seed <n>   (Standard: 12 Seeds, 200 Jahre, ab Seed 1)"
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
    if let Some(e) = data.hotkey_conflicts().first() {
        return Err(format!("Tastenkürzel-Konflikt: {e}"));
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
    validate_campaign(&data, assets)?;

    // Szenarien: laden, Karte prüfen, Welt aufbauen.
    let mut errors = Vec::new();
    let scenarios = ashen_sim::scenario::discover(&[(assets.join("data/scenarios"), ashen_sim::scenario::ScenarioSource::Bundled)], &mut errors);
    if let Some(e) = errors.first() {
        return Err(e.clone());
    }
    for s in &scenarios {
        ashen_sim::SimWorld::new(&data, s.def.clone(), Default::default()).map_err(|e| format!("{}: {e}", s.path.display()))?;
        println!("Szenario {} ({}) ok: {} Ziele, {} Auslöser, Karte {}", s.def.id, s.def.name, s.def.objectives.len(), s.def.triggers.len(), s.def.map.as_ref().map(|m| format!("{}×{}", m.width, m.height)).unwrap_or_else(|| "generiert".into()));
    }
    // Herausforderungen: Basis vorhanden, Modifikatoren anwendbar, Welt startet.
    let challenges = ashen_sim::challenge::ChallengeDef::load_dir(&assets.join("data/challenges"), &mut errors);
    if let Some(e) = errors.first() {
        return Err(e.clone());
    }
    let mut missions: Vec<MissionDef> = scenarios.iter().map(|s| s.def.clone()).collect();
    for e in std::fs::read_dir(&dir).map_err(|e| e.to_string())?.flatten() {
        if let Ok(m) = MissionDef::load(&e.path()) {
            missions.push(m);
        }
    }
    for c in &challenges {
        let base = missions.iter().find(|m| m.id == c.base).ok_or_else(|| format!("Herausforderung {}: Basis '{}' nicht gefunden", c.id, c.base))?;
        let p = ashen_sim::challenge::prepare(c, base);
        ashen_sim::SimWorld::new(&data, p.mission, p.settings).map_err(|e| format!("Herausforderung {}: {e}", c.id))?;
        println!("Herausforderung {} ({}) ok", c.id, c.name);
    }
    println!("{} Szenario(en), {} Herausforderung(en) geprüft.", scenarios.len(), challenges.len());
    Ok(())
}

/// Kampagnenplan und -missionen: Dateien laden, Platzierungen und Karten prüfen, Varianten und Flags anwenden.
fn validate_campaign(data: &GameData, assets: &Path) -> Result<(), String> {
    let dir = assets.join("data/campaign");
    let plan = ashen_sim::campaign::CampaignDef::load(&dir.join("campaign.ron"))?;
    let mut missions: Vec<MissionDef> = Vec::new();
    for e in std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?.flatten() {
        let p = e.path();
        if p.extension().is_none_or(|x| x != "ron") || p.file_name().is_some_and(|n| n == "campaign.ron") {
            continue;
        }
        missions.push(MissionDef::load(&p)?);
    }
    let first = MissionDef::load(&assets.join("data/missions/m01_last_settlement.ron"))?;
    missions.push(first);
    let errs = plan.validate(&missions);
    if !errs.is_empty() {
        return Err(format!("Kampagnenplan: {}", errs.join("; ")));
    }
    let mut decisions = 0;
    for m in &missions {
        for pl in m.placements.iter().chain(m.variants.iter().flat_map(|v| v.placements.iter())) {
            if data.unit_id(&pl.id).is_none() && data.building_id(&pl.id).is_none() {
                return Err(format!("{}: unbekannte Platzierung {}", m.id, pl.id));
            }
        }
        let flags: Vec<String> = m.variants.iter().flat_map(|v| v.requires.iter().cloned()).collect();
        for variant in [m.clone(), m.with_flags(&flags)] {
            ashen_sim::SimWorld::new(data, variant, Default::default()).map_err(|e| format!("{}: {e}", m.id))?;
        }
        decisions += ashen_sim::campaign::decision_flags(m).len();
    }
    println!("Kampagne {}: {} Zeitalter, {} Missionen, {} Entscheidungsoptionen, {} Enden – ok.", plan.name, plan.chapters.len(), plan.mission_count(), decisions, plan.endings.len());
    Ok(())
}

fn main() -> ExitCode {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().cloned() else {
        println!("{}", usage());
        return ExitCode::from(2);
    };
    if cmd == "strategy-stats" {
        return strategy_stats::run(&args[1..]);
    }
    if cmd == "rts-balance" {
        return match assets_dir(None) {
            Ok(a) => rts_balance::run(&a, &args[1..]),
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        };
    }
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
        "forge-scenarios" => forge::forge(&assets).map(|files| {
            for f in files {
                println!("geschrieben: {}", f.display());
            }
        }),
        "forge-campaign" => {
            let repo = assets.parent().map(Path::to_path_buf).unwrap_or_else(|| PathBuf::from("."));
            campaign_forge::forge_campaign(&assets, &repo).map(|files| {
                for f in files {
                    println!("geschrieben: {}", f.display());
                }
            })
        }
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
