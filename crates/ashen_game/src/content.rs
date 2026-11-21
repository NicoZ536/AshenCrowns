//! Laden der Spielinhalte inklusive Mods.

use ashen_sim::data::GameData;
use ashen_sim::mission::MissionDef;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    #[serde(default)]
    pub load_order: i32,
    #[serde(skip)]
    pub path: PathBuf,
}

/// Findet Mods in `<assets>/../mods` und im Nutzerverzeichnis.
pub fn discover_mods(assets: &Path) -> Vec<ModInfo> {
    let mut dirs = vec![assets.parent().map(|p| p.join("mods")).unwrap_or_else(|| PathBuf::from("mods")), crate::settings::data_dir().join("mods")];
    dirs.dedup();
    let mut out = Vec::new();
    for d in dirs {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            let info = p.join("mod.ron");
            if !info.exists() {
                continue;
            }
            match std::fs::read_to_string(&info).map_err(|e| e.to_string()).and_then(|t| ron::from_str::<ModInfo>(&t).map_err(|e| e.to_string())) {
                Ok(mut m) => {
                    m.path = p;
                    if !out.iter().any(|x: &ModInfo| x.id == m.id) {
                        out.push(m);
                    }
                }
                Err(e) => log::warn!("Mod {} ungültig: {e}", p.display()),
            }
        }
    }
    out.sort_by_key(|m| (m.load_order, m.id.clone()));
    out
}

pub struct Content {
    pub data: GameData,
    pub missions: Vec<MissionDef>,
    pub mods: Vec<ModInfo>,
    pub active_mods: Vec<String>,
    pub errors: Vec<String>,
}

impl Content {
    pub fn load(assets: &Path, enabled: &[String]) -> Result<Content, String> {
        let mut data = GameData::load_dir(&assets.join("data")).map_err(|e| e.to_string())?;
        let mods = discover_mods(assets);
        let mut errors = Vec::new();
        let mut active = Vec::new();
        let mut mission_dirs = vec![assets.join("data/missions")];
        for m in &mods {
            if !enabled.contains(&m.id) {
                continue;
            }
            match data.apply_overlay(&m.path.join("data")) {
                Ok(changes) => {
                    log::info!("Mod {} aktiv ({} Änderungen)", m.id, changes.len());
                    active.push(m.id.clone());
                    mission_dirs.push(m.path.join("data/missions"));
                }
                Err(e) => errors.push(format!("Mod {}: {e}", m.name)),
            }
        }
        let mut missions = Vec::new();
        for d in mission_dirs {
            let Ok(rd) = std::fs::read_dir(&d) else { continue };
            let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "ron")).collect();
            files.sort();
            for f in files {
                match MissionDef::load(&f) {
                    Ok(m) => missions.push(m),
                    Err(e) => errors.push(e),
                }
            }
        }
        if missions.is_empty() {
            return Err("Keine Missionen gefunden".into());
        }
        Ok(Content { data, missions, mods, active_mods: active, errors })
    }
}
