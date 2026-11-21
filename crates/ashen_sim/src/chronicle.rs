//! Chronik: hält bemerkenswerte Ereignisse als lesbare Geschichte fest.
//!
//! In der Vertical Slice protokolliert sie die Mission; das Datenmodell ist für die
//! Weltchronik der Grand Campaign ausgelegt (Jahr, Akteure, Kategorie).

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChronicleKind {
    Founding,
    Construction,
    Battle,
    Death,
    Hero,
    Omen,
    Victory,
    Defeat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChronicleEntry {
    pub year: u32,
    pub day: u32,
    pub phase: String,
    pub kind: ChronicleKind,
    pub text: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Chronicle {
    pub entries: Vec<ChronicleEntry>,
}

impl Chronicle {
    pub fn record(&mut self, year: u32, day: u32, phase: &str, kind: ChronicleKind, text: impl Into<String>) {
        let text = text.into();
        // Doppelte aufeinanderfolgende Einträge vermeiden.
        if self.entries.last().is_some_and(|e| e.text == text) {
            return;
        }
        self.entries.push(ChronicleEntry { year, day, phase: phase.to_string(), kind, text });
    }

    /// Fließtext für Abschlussbildschirme.
    pub fn narrative(&self) -> String {
        self.entries
            .iter()
            .map(|e| format!("Jahr {} n.F., Tag {} ({}): {}", e.year, e.day, e.phase, e.text))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
