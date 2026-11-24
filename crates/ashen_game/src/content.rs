//! Laden der Spielinhalte inklusive Mods.

use ashen_sim::campaign::CampaignDef;
use ashen_sim::data::GameData;
use ashen_sim::challenge::ChallengeDef;
use ashen_sim::mission::MissionDef;
use ashen_sim::scenario::{ScenarioEntry, ScenarioSource};
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
    /// Mitgelieferte und Mod-Szenarien (eigene Szenarien liegen im Benutzerordner, siehe [`Content::user_scenarios`]).
    pub scenarios: Vec<ScenarioEntry>,
    pub challenges: Vec<ChallengeDef>,
    /// Kampagnenplan „Das Zeitalter der Gefallenen“ und die zugehörigen Missionen.
    pub campaign: Option<CampaignDef>,
    pub campaign_missions: Vec<MissionDef>,
    scenario_dirs: Vec<(PathBuf, ScenarioSource)>,
}

impl Content {
    /// Ordner mit mitgelieferten und Mod-Szenarien.
    pub fn scenario_dirs(&self) -> Vec<(PathBuf, ScenarioSource)> {
        self.scenario_dirs.clone()
    }

    /// Eigene Szenarien aus dem Benutzerordner (bei jedem Aufruf frisch gelesen).
    pub fn user_scenarios(&self, errors: &mut Vec<String>) -> Vec<ScenarioEntry> {
        ashen_sim::scenario::discover(&[(crate::settings::scenarios_dir(), ScenarioSource::User)], errors)
    }

    /// Mission oder Szenario nach ID (Basis von Herausforderungen).
    pub fn find_base(&self, id: &str) -> Option<MissionDef> {
        self.missions.iter().find(|m| m.id == id).cloned().or_else(|| self.scenarios.iter().find(|s| s.def.id == id).map(|s| s.def.clone())).or_else(|| self.campaign_mission(id).cloned())
    }

    /// Kampagnenmission nach ID (Mission 1 liegt bei den Basismissionen).
    pub fn campaign_mission(&self, id: &str) -> Option<&MissionDef> {
        self.campaign_missions.iter().find(|m| m.id == id).or_else(|| self.missions.iter().find(|m| m.id == id))
    }

    pub fn load(assets: &Path, enabled: &[String]) -> Result<Content, String> {
        let mut data = GameData::load_dir(&assets.join("data")).map_err(|e| e.to_string())?;
        let mods = discover_mods(assets);
        let mut errors = Vec::new();
        let mut active = Vec::new();
        let mut mission_dirs = vec![assets.join("data/missions")];
        let mut scenario_dirs = vec![(assets.join("data/scenarios"), ScenarioSource::Bundled)];
        let mut challenge_dirs = vec![assets.join("data/challenges")];
        for m in &mods {
            if !enabled.contains(&m.id) {
                continue;
            }
            match data.apply_overlay(&m.path.join("data")) {
                Ok(changes) => {
                    log::info!("Mod {} aktiv ({} Änderungen)", m.id, changes.len());
                    active.push(m.id.clone());
                    mission_dirs.push(m.path.join("data/missions"));
                    scenario_dirs.push((m.path.join("data/scenarios"), ScenarioSource::Mod));
                    challenge_dirs.push(m.path.join("data/challenges"));
                }
                Err(e) => errors.push(format!("Mod {}: {e}", m.name)),
            }
        }
        for c in data.hotkey_conflicts() {
            log::warn!("Tastenkürzel: {c}");
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
        let scenarios = ashen_sim::scenario::discover(&scenario_dirs, &mut errors);
        let mut challenges: Vec<ChallengeDef> = Vec::new();
        for d in challenge_dirs {
            for c in ChallengeDef::load_dir(&d, &mut errors) {
                // Mods können eine Herausforderung mit gleicher ID ersetzen.
                challenges.retain(|x| x.id != c.id);
                challenges.push(c);
            }
        }
        // Kampagne: Plan und Missionen (fehlt der Plan, bleibt der Menüpunkt leer, ohne das Spiel zu stören).
        let mut campaign_missions: Vec<MissionDef> = Vec::new();
        let mut campaign = None;
        let cdir = assets.join("data/campaign");
        if let Ok(rd) = std::fs::read_dir(&cdir) {
            let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "ron")).collect();
            files.sort();
            for f in files {
                if f.file_name().is_some_and(|n| n == "campaign.ron") {
                    match CampaignDef::load(&f) {
                        Ok(c) => campaign = Some(c),
                        Err(e) => errors.push(e),
                    }
                } else {
                    match MissionDef::load(&f) {
                        Ok(m) => campaign_missions.push(m),
                        Err(e) => errors.push(e),
                    }
                }
            }
        }
        Ok(Content { data, missions, mods, active_mods: active, errors, scenarios, challenges, campaign, campaign_missions, scenario_dirs })
    }
}
