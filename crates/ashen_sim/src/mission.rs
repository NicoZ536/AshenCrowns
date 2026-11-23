//! Datengetriebene Missionen: Regeln, Startaufstellung, Ziele und Auslöser.
//!
//! Missionen liegen als RON unter `assets/data/missions/` und können von Mods ergänzt werden.

use crate::data::Cost;
use crate::env::Weather;
use crate::scenario::{AreaDef, MapData, StartPos};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Spielmodus, der die Regeln einer Mission variiert.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameMode {
    Campaign,
    Skirmish,
    Survival,
    Sandbox,
}

impl GameMode {
    pub fn label(self) -> &'static str {
        match self {
            GameMode::Campaign => "Kampagne",
            GameMode::Skirmish => "Gefecht",
            GameMode::Survival => "Überleben",
            GameMode::Sandbox => "Sandbox",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(alias = "Story")]
    Easy,
    Normal,
    Hard,
    Nightmare,
}

impl Difficulty {
    pub const ALL: [Difficulty; 4] = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard, Difficulty::Nightmare];

    pub fn wave_mult(self) -> f32 {
        match self {
            Difficulty::Easy => 0.65,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.4,
            Difficulty::Nightmare => 1.8,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Difficulty::Easy => "Leicht",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Schwer",
            Difficulty::Nightmare => "Albtraum",
        }
    }

    /// Index in `Rules::ai_levels`.
    pub fn index(self) -> usize {
        match self {
            Difficulty::Easy => 0,
            Difficulty::Normal => 1,
            Difficulty::Hard => 2,
            Difficulty::Nightmare => 3,
        }
    }
}

/// Startoptionen einer Partie.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchSettings {
    pub mode: GameMode,
    pub difficulty: Difficulty,
    pub seed: u64,
    /// Dauerhafter Heldentod (kein Wiederbeleben im Bergfried).
    pub hero_permadeath: bool,
}

impl Default for MatchSettings {
    fn default() -> Self {
        MatchSettings { mode: GameMode::Campaign, difficulty: Difficulty::Normal, seed: 897, hero_permadeath: false }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct WaveRules {
    pub enabled: bool,
    pub first_wave: f32,
    pub interval_day: f32,
    pub interval_night: f32,
    pub base_budget: f32,
    pub budget_per_wave: f32,
    pub night_mult: f32,
}

impl Default for WaveRules {
    fn default() -> Self {
        WaveRules {
            enabled: true,
            first_wave: 200.0,
            interval_day: 150.0,
            interval_night: 95.0,
            base_budget: 3.0,
            budget_per_wave: 1.7,
            night_mult: 1.5,
        }
    }
}

/// Einheit oder Gebäude der Startaufstellung, platziert in Kachelkoordinaten.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Placement {
    pub id: String,
    pub owner: u8,
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub count: u32,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub guard: bool,
    /// Lagerkennung: fällt die letzte Einheit des Lagers, lässt es Beute fallen.
    #[serde(default)]
    pub camp: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectiveDef {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Condition {
    AtTime(f32),
    BuildingCompleted(String),
    BuildingCount { building: String, count: u32 },
    UnitCount { unit: String, count: u32 },
    MilitaryCount(u32),
    ObjectiveCompleted(String),
    NightStarted(u32),
    DayStarted(u32),
    TagDead(String),
    TagDiscovered(String),
    HeroLevel(u32),
    WavesSurvived(u32),
    /// Ausbaustufe der Siedlung erreicht.
    EraReached(u8),
    /// Mindestzahl an Bewohnern.
    Residents(u32),
    /// Mindestvorrat einer Ware oder Ressource (Item-ID).
    Stock { item: String, amount: f32 },
    /// Mindestens `count` eigene Einheiten stehen im benannten Bereich.
    UnitsInArea { area: String, count: u32 },
    /// Höchstens so viele gegnerische Einheiten leben noch.
    EnemyUnitsAtMost(u32),
    /// Das Gebäude mit diesem Tag hat Schaden genommen.
    TagDamaged(String),
    Not(Box<Condition>),
    All(Vec<Condition>),
    Any(Vec<Condition>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    Message { speaker: String, text: String, #[serde(default)] voice: Option<String> },
    AddObjective(String),
    CompleteObjective(String),
    Victory,
    Defeat,
    SpawnWave(f32),
    Grant(Cost),
    EnableWaves(bool),
    SetWeather(Weather),
    Chronicle(String),
    /// Erzeugt Einheiten im benannten Bereich; `attack` schickt sie gegen die Spielersiedlung.
    SpawnUnits { unit: String, owner: u8, area: String, count: u32, #[serde(default)] attack: bool },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TriggerDef {
    pub id: String,
    pub condition: Condition,
    pub actions: Vec<Action>,
    /// Modi, in denen der Auslöser aktiv ist (leer = alle).
    #[serde(default)]
    pub modes: Vec<GameMode>,
}

/// Definition einer Mission.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MissionDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub map_seed: u64,
    pub start_resources: Cost,
    /// Startvorrat an Waren (Item-ID, Menge).
    #[serde(default)]
    pub start_goods: Vec<(String, f32)>,
    pub waves: WaveRules,
    pub placements: Vec<Placement>,
    pub objectives: Vec<ObjectiveDef>,
    pub triggers: Vec<TriggerDef>,
    /// Tag des Gebäudes, dessen Zerstörung zur Niederlage führt.
    pub defeat_tag: String,
    /// Tag des gegnerischen Hauptgebäudes (Ziel in Gefecht/Kampagne, Spawnpunkt der Wellen).
    pub enemy_base_tag: String,
    /// Explizite Karte (Szenarien); ohne Angabe wird die Karte aus `map_seed` erzeugt.
    #[serde(default)]
    pub map: Option<MapData>,
    /// Benannte Bereiche für Auslöser und Aktionen.
    #[serde(default)]
    pub areas: Vec<AreaDef>,
    /// Startpositionen der Spieler (Editor, Gefechtseinrichtung, Validierung).
    #[serde(default)]
    pub starts: Vec<StartPos>,
    /// Ausführliche Einsatzbesprechung (Szenarienliste).
    #[serde(default)]
    pub briefing: String,
    #[serde(default)]
    pub author: String,
    /// Jahr n.F. des historischen Ereignisses (Anzeige).
    #[serde(default)]
    pub year: Option<u32>,
    /// Obergrenze für Streitkräfte (alle Einheiten außer Arbeitern) des Spielers.
    #[serde(default)]
    pub unit_cap: Option<u32>,
    /// Tageszeit (0..1) bei Spielbeginn; ohne Angabe der Standard der Umgebung.
    #[serde(default)]
    pub start_time: Option<f32>,
    /// Keine Wirtschaftsstrafen: kein Nahrungsunterhalt (keine Hungersnot) und kein Bevölkerungslimit.
    /// Gedacht für Schlachten aus der Großen Kampagne, die ohne Stadt und Nachschub beginnen.
    #[serde(default)]
    pub free_economy: bool,
}

impl MissionDef {
    pub fn load(path: &Path) -> Result<MissionDef, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        ron::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    pub fn area(&self, id: &str) -> Option<&AreaDef> {
        self.areas.iter().find(|a| a.id == id)
    }

    pub fn objective(&self, id: &str) -> Option<&ObjectiveDef> {
        self.objectives.iter().find(|o| o.id == id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectiveState {
    Active,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Victory,
    Defeat,
}

/// Laufzeitzustand der Mission (wird gespeichert).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MissionState {
    pub objectives: Vec<(String, ObjectiveState)>,
    pub fired: Vec<String>,
    pub outcome: Option<Outcome>,
    pub discovered_tags: Vec<String>,
    pub dead_tags: Vec<String>,
    pub waves_survived: u32,
}

impl MissionState {
    pub fn is_completed(&self, id: &str) -> bool {
        self.objectives.iter().any(|(o, s)| o == id && *s == ObjectiveState::Completed)
    }

    pub fn has_objective(&self, id: &str) -> bool {
        self.objectives.iter().any(|(o, _)| o == id)
    }
}
