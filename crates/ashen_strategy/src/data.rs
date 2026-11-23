//! Datengetriebene Inhalte (RON unter `assets/data/world/`): Ereignisse mit Bedingungen, Optionen,
//! Effekten und KI-Entscheidungslogik (GDD 15.6). Mods können Einträge mit gleicher `id` ersetzen.

use crate::types::Season;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, OnceLock};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventScope {
    Province,
    Realm,
    Character,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Cond {
    /// Variable kleiner als Wert (Variablennamen siehe `events::var`).
    VarBelow(String, f32),
    VarAbove(String, f32),
    /// Aufeinanderfolgende trockene Monate (Provinz).
    DryMonths(u8),
    Season(Vec<Season>),
    HasState(String),
    NotState(String),
    /// Zufall pro Auswertung (0–1).
    Chance(f32),
    /// Nur für Reiche mit dieser Religionskennung als Staatsreligion.
    Religion(String),
    /// Nur für diese Fraktion (Kanon-ID).
    Faction(String),
    /// Nur ab diesem Jahr n.F.
    YearFrom(i32),
    /// Nur wenn der Herrscher die Eigenschaft hat.
    RulerTrait(String),
    /// Nur wenn der Herrscher die Eigenschaft nicht hat.
    RulerNotTrait(String),
    /// Nur im Krieg / nur im Frieden.
    AtWar(bool),
    /// Nur Provinzen mit diesem Biom (Bezeichnung wie `Biome` in RON, z. B. "Forest").
    Biome(String),
    /// Nur wenn der Besitzer eine Küstenprovinz/Hafen hat (Provinzscope: Provinz ist Küste).
    Coastal,
    /// Nur wenn ein Spielerreich betroffen ist.
    PlayerOnly,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    AddGold(f32),
    AddPrestige(f32),
    AddPiety(f32),
    AddLegitimacy(f32),
    AddTyranny(f32),
    /// Unruhe der Provinz (Realmscope: alle Provinzen).
    AddUnrest(f32),
    /// Zufriedenheit aller Schichten.
    AddSatisfaction(f32),
    /// Ware ins Provinzlager (Realmscope: Hauptstadt).
    AddStock(String, f32),
    /// Preisfaktor auf dem Markt der Provinz für N Monate.
    PriceFactor(String, f32, u32),
    SetState(String, u32),
    ClearState(String),
    AddHollowing(f32),
    AddVeil(f32),
    /// Bevölkerungsänderung in Prozent.
    AddPopulationPct(f32),
    AddFortification(i32),
    HouseLoyalty(f32),
    AddStress(f32),
    AddHealth(f32),
    GrantTrait(String),
    /// Rebellen in Prozent der Provinzbevölkerung entstehen.
    SpawnRebels(f32),
    StartPlague(u8, f32),
    AddWarWeariness(f32),
    /// Meinung aller Nachbarreiche zum Reich ändern.
    NeighborOpinion(f32),
    AddDisease(f32),
    Devastate(f32),
    AddAshglass(f32),
    AddResearch(f32),
    SpawnNeutral(String, f32),
    /// Religionsanteil verschieben (Religions-ID).
    AddFaith(String, f32),
    AddTension(u8, f32),
    AddRefugees(f32),
    /// Zusatz-Chronikvermerk (Vorlage `event_generic`).
    Note(String),
    AddFertility(f32),
    AddRulerHollowing(f32),
    AddTradeBoost(f32),
    RaiseSettlement,
    SpawnArmyPct(f32),
    AddSpyNetwork(f32),
    AddFood(f32),
    AddPopularity(f32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventOption {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub effects: Vec<Effect>,
    /// Goldkosten (für die Verfügbarkeitsprüfung und KI).
    #[serde(default)]
    pub cost: f32,
    /// Grundgewicht der KI-Wahl.
    #[serde(default = "one")]
    pub ai_weight: f32,
    /// Bedingte Zuschläge zum KI-Gewicht (Bedingung, Delta).
    #[serde(default)]
    pub ai_when: Vec<(Cond, f32)>,
    /// Nur wählbar, wenn alle Bedingungen gelten.
    #[serde(default)]
    pub requires: Vec<Cond>,
}

fn one() -> f32 {
    1.0
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventDef {
    pub id: String,
    pub title: String,
    pub text: String,
    pub scope: EventScope,
    #[serde(default)]
    pub category: String,
    pub conditions: Vec<Cond>,
    /// Mittlere Zeit bis zum Auslösen in Monaten (Bedingung erfüllt).
    #[serde(default = "one")]
    pub mtth_months: f32,
    #[serde(default)]
    pub cooldown_months: u32,
    pub options: Vec<EventOption>,
    #[serde(default)]
    pub sets_state: Option<(String, u32)>,
    #[serde(default)]
    pub chronicle_weight: u8,
    /// Wird für den Spieler als Fenster gezeigt, sonst nur Benachrichtigung.
    #[serde(default = "yes")]
    pub window: bool,
}

fn yes() -> bool {
    true
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EventFile {
    pub events: Vec<EventDef>,
}

/// Alle datengetriebenen Inhalte der Strategieebene.
#[derive(Clone, Debug, Default)]
pub struct StrategyData {
    pub events: Vec<EventDef>,
}

const BUILTIN_FILES: &[(&str, &str)] = &[
    ("nature.ron", include_str!("../../../assets/data/world/events/nature.ron")),
    ("economy.ron", include_str!("../../../assets/data/world/events/economy.ron")),
    ("people.ron", include_str!("../../../assets/data/world/events/people.ron")),
    ("politics.ron", include_str!("../../../assets/data/world/events/politics.ron")),
    ("faith_magic.ron", include_str!("../../../assets/data/world/events/faith_magic.ron")),
    ("threats.ron", include_str!("../../../assets/data/world/events/threats.ron")),
];

impl StrategyData {
    /// Eingebaute Standarddaten (zur Compile-Zeit eingebettet, identisch mit `assets/data/world/events`).
    pub fn builtin() -> Arc<StrategyData> {
        static CELL: OnceLock<Arc<StrategyData>> = OnceLock::new();
        CELL.get_or_init(|| {
            let mut d = StrategyData::default();
            for (name, text) in BUILTIN_FILES {
                match ron::from_str::<EventFile>(text) {
                    Ok(f) => d.merge(f.events),
                    Err(e) => log::error!("Ereignisdatei {name} fehlerhaft: {e}"),
                }
            }
            Arc::new(d)
        })
        .clone()
    }

    fn merge(&mut self, events: Vec<EventDef>) {
        for e in events {
            if let Some(slot) = self.events.iter_mut().find(|x| x.id == e.id) {
                *slot = e;
            } else {
                self.events.push(e);
            }
        }
    }

    /// Lädt alle `*.ron` aus `dir/events` über die eingebauten Daten (Mods ersetzen gleiche IDs).
    pub fn load_overlay(dir: &Path) -> Result<StrategyData, String> {
        let mut d = (*StrategyData::builtin()).clone();
        let ev = dir.join("events");
        let Ok(rd) = std::fs::read_dir(&ev) else { return Ok(d) };
        let mut files: Vec<_> = rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "ron")).collect();
        files.sort();
        for p in files {
            let text = std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?;
            let f: EventFile = ron::from_str(&text).map_err(|e| format!("{}: {e}", p.display()))?;
            d.merge(f.events);
        }
        d.validate()?;
        Ok(d)
    }

    /// Prüft Referenzen (Waren, Zustände) der Ereignisse.
    pub fn validate(&self) -> Result<(), String> {
        for e in &self.events {
            if e.options.is_empty() {
                return Err(format!("Ereignis {} hat keine Optionen", e.id));
            }
            for o in &e.options {
                for ef in &o.effects {
                    match ef {
                        Effect::AddStock(g, _) | Effect::PriceFactor(g, _, _) => {
                            if crate::goods::Good::from_id(g).is_none() {
                                return Err(format!("Ereignis {}: unbekannte Ware {g}", e.id));
                            }
                        }
                        Effect::GrantTrait(t)
                            if crate::catalog::Trait::from_id(t).is_none() => {
                                return Err(format!("Ereignis {}: unbekannte Eigenschaft {t}", e.id));
                            }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
