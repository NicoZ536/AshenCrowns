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
    /// Kampagnen-/Missionsflag ist gesetzt (aus früheren Missionen oder per [`Action::SetFlag`]).
    Flag(String),
    /// Mindestens eine lebende Einheit (beliebiger Seite) mit diesem Tag steht im benannten Bereich.
    TagInArea { tag: String, area: String },
    /// Eine lebende Einheit mit diesem Tag hat weniger als `percent` Prozent ihrer Lebenspunkte (Bossphasen).
    TagHealthBelow { tag: String, percent: f32 },
    /// Höchstens so viele Soldaten (ohne Arbeiter und Helden) stehen noch auf Spielerseite.
    ArmyAtMost(u32),
    /// Alle zuvor ausgelösten Dialogzeilen sind gesprochen (zeitliche Abfolge von Szenen).
    DialogueDone,
    /// Mindestens `count` eigene Einheiten dieser Art stehen im Bereich (z. B. gerettete Zivilisten).
    UnitKindInArea { unit: String, area: String, count: u32 },
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
    /// `tag` kennzeichnet die Einheiten (für Bedingungen und [`Action::ChangeOwner`]).
    SpawnUnits { unit: String, owner: u8, area: String, count: u32, #[serde(default)] attack: bool, #[serde(default)] tag: Option<String> },
    /// Setzt ein Flag; Flags bleiben in der Kampagne erhalten.
    SetFlag(String),
    /// Spricht eine Zeilenfolge nacheinander (Erzähler und Figuren); Pausen ergeben sich aus der Textlänge.
    Dialogue(Vec<DialogueLine>),
    /// Öffnet eine Entscheidung; die Wahl setzt das Flag der Option und führt deren Aktionen aus.
    Decision(DecisionDef),
    /// Übergibt alle lebenden Einheiten mit diesem Tag an einen anderen Besitzer (Rettung, Beitritt, Verrat).
    ChangeOwner { tag: String, owner: u8 },
    /// Befiehlt allen Einheiten mit diesem Tag, zum Bereich zu laufen (`attack` = angreifend unterwegs).
    MoveTag { tag: String, area: String, attack: bool },
}

/// Eine gesprochene Zeile (Erzähler oder Figur) mit optionaler Audio-Event-ID.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
    #[serde(default)]
    pub voice: Option<String>,
}

/// Wahlmöglichkeit einer Entscheidung.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionOption {
    /// Flag, das bei Wahl gesetzt wird (Kampagnenvariable).
    pub flag: String,
    pub label: String,
    /// Hinweis auf die Folgen (Tooltip im Entscheidungsfenster).
    #[serde(default)]
    pub detail: String,
    /// Die Option ist nur wählbar, wenn alle diese Flags gesetzt sind.
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub actions: Vec<Action>,
}

/// Entscheidung, die der Spieler per Dialogauswahl trifft.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionDef {
    pub id: String,
    pub speaker: String,
    pub title: String,
    pub text: String,
    #[serde(default)]
    pub voice: Option<String>,
    pub options: Vec<DecisionOption>,
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
    /// Missionstyp zur Anzeige (z. B. „Eskorte“, „Belagerung“).
    #[serde(default)]
    pub kind: String,
    /// Schlusstext im Siegbildschirm; leer = Standardtext.
    #[serde(default)]
    pub victory_text: String,
    /// Schlusstext im Niederlagebildschirm; leer = Standardtext.
    #[serde(default)]
    pub defeat_text: String,
    /// Chronikeintrag beim Missionsstart; leer = Standardtext.
    #[serde(default)]
    pub intro_chronicle: String,
    /// Überschrift der Chronik im Menü; leer = Missionsname.
    #[serde(default)]
    pub chronicle_title: String,
    /// Änderungen je nach Kampagnen-Flags (Entscheidungen früherer Missionen).
    #[serde(default)]
    pub variants: Vec<FlagVariant>,
    /// Zustand aus der Kampagne: gesetzte Flags und übernommene Helden.
    #[serde(default)]
    pub carry: Carry,
    /// Bevorzugte Musikstücke (Track-IDs aus dem Audio-Manifest), z. B. das Boss-Thema der Mission.
    #[serde(default)]
    pub music: Vec<String>,
}

/// Aus früheren Missionen übernommener Kampagnenzustand.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Carry {
    #[serde(default)]
    pub flags: Vec<String>,
    #[serde(default)]
    pub heroes: Vec<HeroRecord>,
}

/// Fortschritt eines Helden (Stufe, Talente, Ausrüstung), der zwischen Missionen übernommen wird.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HeroRecord {
    /// Einheiten-ID des Helden (z. B. `hero_isolde`).
    pub unit: String,
    pub level: u32,
    pub xp: u32,
    pub talents: Vec<u8>,
    /// Ausrüstungsplätze (Waffe, Rüstung, Amulett, zwei Ringe) als Gegenstands-IDs.
    pub equipment: Vec<Option<String>>,
    pub inventory: Vec<String>,
}

/// Änderung der Mission, wenn bestimmte Flags gesetzt (`requires`) beziehungsweise nicht gesetzt (`forbids`) sind.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FlagVariant {
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub forbids: Vec<String>,
    /// Satz für das Briefing („Weil Ihr …“).
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub grant: Cost,
    #[serde(default)]
    pub goods: Vec<(String, f32)>,
    /// Entfernt Platzierungen mit diesen Tags.
    #[serde(default)]
    pub remove_tags: Vec<String>,
    #[serde(default)]
    pub placements: Vec<Placement>,
    #[serde(default)]
    pub objectives: Vec<ObjectiveDef>,
    #[serde(default)]
    pub areas: Vec<AreaDef>,
    #[serde(default)]
    pub triggers: Vec<TriggerDef>,
    /// Faktor auf das Wellenbudget (0 = unverändert).
    #[serde(default)]
    pub wave_mult: f32,
}

impl FlagVariant {
    pub fn applies(&self, flags: &[String]) -> bool {
        self.requires.iter().all(|f| flags.contains(f)) && !self.forbids.iter().any(|f| flags.contains(f))
    }
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

    /// Varianten, die bei diesen Flags wirken.
    pub fn active_variants<'a>(&'a self, flags: &[String]) -> Vec<&'a FlagVariant> {
        self.variants.iter().filter(|v| v.applies(flags)).collect()
    }

    /// Wendet alle zu den Flags passenden Varianten an und liefert die spielbereite Mission.
    pub fn with_flags(&self, flags: &[String]) -> MissionDef {
        let mut m = self.clone();
        for v in self.active_variants(flags) {
            m.start_resources.add(&v.grant);
            m.start_goods.extend(v.goods.iter().cloned());
            m.placements.retain(|p| p.tag.as_ref().is_none_or(|t| !v.remove_tags.contains(t)));
            m.placements.extend(v.placements.iter().cloned());
            m.objectives.extend(v.objectives.iter().cloned());
            m.areas.extend(v.areas.iter().cloned());
            m.triggers.extend(v.triggers.iter().cloned());
            if v.wave_mult > 0.0 {
                m.waves.base_budget *= v.wave_mult;
                m.waves.budget_per_wave *= v.wave_mult;
            }
        }
        m
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
    /// Gesetzte Flags (übernommene und in dieser Mission gesetzte).
    #[serde(default)]
    pub flags: Vec<String>,
    /// Offene Entscheidungen; die erste ist die aktuell angezeigte.
    #[serde(default)]
    pub decisions: Vec<DecisionDef>,
    /// Getroffene Entscheidungen: (Entscheidungs-ID, gewähltes Flag).
    #[serde(default)]
    pub choices: Vec<(String, String)>,
    /// Noch nicht gesprochene Dialogzeilen: (Zeitpunkt in Sekunden, Zeile).
    #[serde(default)]
    pub line_queue: Vec<(f32, DialogueLine)>,
    /// Zeitpunkt, ab dem die nächste Zeile gesprochen werden darf.
    #[serde(default)]
    pub line_ready: f32,
}

impl MissionState {
    pub fn is_completed(&self, id: &str) -> bool {
        self.objectives.iter().any(|(o, s)| o == id && *s == ObjectiveState::Completed)
    }

    pub fn has_objective(&self, id: &str) -> bool {
        self.objectives.iter().any(|(o, _)| o == id)
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }

    /// Die Entscheidung, die dem Spieler gerade zur Wahl steht: erst nachdem alle vorangehenden
    /// Dialogzeilen gesprochen sind.
    pub fn open_decision(&self, time: f32) -> Option<&DecisionDef> {
        if self.line_queue.is_empty() && time >= self.line_ready { self.decisions.first() } else { None }
    }
}
