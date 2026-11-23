//! Audio-Manifest: verbindliche Zuordnung von Spiel-Events zu Audiodateien.
//!
//! Pfade sind relativ zu `assets/audio` und ohne Endung. Das Suffix `_*` steht für
//! nummerierte Varianten (`_01`, `_02`, …).

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Intensity {
    Menu,
    /// Weltkarte: Friedenszeit, Krieg und Hohlung (aufsteigend, Reihenfolge ist für das Überblenden wichtig).
    WorldPeace,
    WorldWar,
    WorldHollow,
    Calm,
    Tension,
    Combat,
    Battle,
    Boss,
    Sting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Voice,
    Narration,
    Dialogue,
    Sfx,
    Ui,
}

/// Wiedergabe mehrerer Dateien eines Events.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Playback {
    /// Zufällige Variante (ohne direkte Wiederholung).
    #[default]
    Random,
    /// Alle Teile nacheinander in Nummernreihenfolge (Erzählerpassagen, Dialoge).
    Sequential,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MusicTrack {
    pub id: String,
    pub intensity: Intensity,
    pub file: String,
    pub intro: Option<String>,
    pub looped: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoundEvent {
    pub id: String,
    pub category: Category,
    pub files: Vec<String>,
    pub volume_db: f32,
    pub cooldown_ms: u32,
    pub max_instances: u32,
    #[serde(default)]
    pub playback: Playback,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmbienceLayer {
    pub id: String,
    pub file: String,
    pub volume_db: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AudioManifest {
    pub version: u32,
    pub music: Vec<MusicTrack>,
    pub events: Vec<SoundEvent>,
    pub ambience: Vec<AmbienceLayer>,
}

impl AudioManifest {
    pub fn load(path: &Path) -> Result<AudioManifest, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        ron::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    pub fn event(&self, id: &str) -> Option<&SoundEvent> {
        self.events.iter().find(|e| e.id == id)
    }

    pub fn track(&self, id: &str) -> Option<&MusicTrack> {
        self.music.iter().find(|m| m.id == id)
    }

    /// Alle erwarteten Dateimuster mit ihrem Besitzer (Event-, Track- oder Ambience-ID).
    pub fn patterns(&self) -> Vec<(String, String)> {
        let mut v = Vec::new();
        for m in &self.music {
            v.push((m.file.clone(), format!("music:{}", m.id)));
            if let Some(i) = &m.intro {
                v.push((i.clone(), format!("music:{}:intro", m.id)));
            }
        }
        for e in &self.events {
            for f in &e.files {
                v.push((f.clone(), format!("event:{}", e.id)));
            }
        }
        for a in &self.ambience {
            v.push((a.file.clone(), format!("ambience:{}", a.id)));
        }
        v
    }
}
