//! Benutzereinstellungen und plattformgerechte Verzeichnisse (macOS, Windows, Linux).

use ashen_engine::audio::Volumes;
use ashen_engine::render::GraphicsProfile;
use ashen_sim::mission::Difficulty;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct UserSettings {
    pub profile: GraphicsProfile,
    pub fullscreen: bool,
    pub vsync: bool,
    pub volumes: Volumes,
    pub edge_scroll: bool,
    pub scroll_speed: f32,
    pub difficulty: Difficulty,
    pub hero_permadeath: bool,
    pub show_fps: bool,
    pub enabled_mods: Vec<String>,
    pub autosave_minutes: f32,
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            profile: GraphicsProfile::Balanced,
            fullscreen: false,
            vsync: true,
            volumes: Volumes::default(),
            edge_scroll: true,
            scroll_speed: 1.0,
            difficulty: Difficulty::Normal,
            hero_permadeath: false,
            show_fps: false,
            enabled_mods: Vec::new(),
            autosave_minutes: 5.0,
        }
    }
}

/// Basisverzeichnis für Nutzerdaten.
pub fn data_dir() -> PathBuf {
    if let Some(p) = std::env::var_os("ASHEN_USER_DIR") {
        return PathBuf::from(p);
    }
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    if cfg!(target_os = "macos") {
        home.join("Library/Application Support/AshenCrowns")
    } else if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or(home).join("AshenCrowns")
    } else {
        std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).unwrap_or_else(|| home.join(".local/share")).join("ashen_crowns")
    }
}

pub fn saves_dir() -> PathBuf {
    data_dir().join("saves")
}

/// Ordner der selbst erstellten Szenarien.
pub fn scenarios_dir() -> PathBuf {
    data_dir().join("scenarios")
}

/// Lokale Bestenliste der Herausforderungen.
pub fn leaderboard_path() -> PathBuf {
    data_dir().join("leaderboard.ron")
}

fn settings_path() -> PathBuf {
    data_dir().join("settings.ron")
}

impl UserSettings {
    pub fn load() -> UserSettings {
        match std::fs::read_to_string(settings_path()) {
            Ok(t) => ron::from_str(&t).unwrap_or_else(|e| {
                log::warn!("Einstellungen fehlerhaft ({e}), verwende Standardwerte");
                UserSettings::default()
            }),
            Err(_) => UserSettings::default(),
        }
    }

    pub fn save(&self) {
        let path = settings_path();
        if let Some(d) = path.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        match ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()) {
            Ok(t) => {
                if let Err(e) = std::fs::write(&path, t) {
                    log::warn!("Einstellungen nicht gespeichert: {e}");
                }
            }
            Err(e) => log::warn!("Einstellungen nicht serialisierbar: {e}"),
        }
    }
}
