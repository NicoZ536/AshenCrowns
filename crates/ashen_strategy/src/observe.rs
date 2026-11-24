//! Beobachterläufe: reine KI-Simulation ohne Spielerreich über viele Jahre und Seeds.
//!
//! Grundlage für die Balancemessung der Siegbedingungen (Werkzeug `ashen_tools strategy-stats`,
//! Test `observer_victories_are_rare_and_late`).

use crate::endgame::VictoryKind;
use crate::types::START_YEAR;
use crate::world::{StrategyOptions, StrategyWorld};

/// Ergebnis eines Beobachterlaufs.
#[derive(Clone, Debug)]
pub struct ObserverRun {
    pub seed: u64,
    /// Simulierte Jahre (kürzer als gefordert, wenn früher ein Sieg eintrat).
    pub years: u32,
    /// Sieg der KI: Art, Name des Reiches und Jahr n.F.
    pub victory: Option<(VictoryKind, String, i32)>,
    /// Lebende Reiche am Ende.
    pub realms_alive: usize,
}

impl ObserverRun {
    /// Jahre seit Spielbeginn bis zum Sieg.
    pub fn victory_after_years(&self) -> Option<i32> {
        self.victory.as_ref().map(|(_, _, y)| y - START_YEAR)
    }
}

/// Simuliert einen Beobachterlauf bis zum Sieg oder bis `years` Jahre vergangen sind.
pub fn observe(seed: u64, years: u32) -> ObserverRun {
    let mut w = StrategyWorld::new(StrategyOptions::observer(seed));
    let mut done = 0;
    while done < years && w.victory.is_none() {
        w.advance_days(crate::types::DAYS_PER_YEAR);
        done += 1;
    }
    let victory = w.victory.map(|v| (v.kind, w.rname(v.realm), START_YEAR + (v.day / crate::types::DAYS_PER_YEAR) as i32));
    ObserverRun { seed, years: done, victory, realms_alive: w.realms.iter().filter(|r| r.alive).count() }
}

/// Zusammenfassung mehrerer Läufe.
#[derive(Clone, Debug, Default)]
pub struct ObserverSummary {
    pub runs: usize,
    pub victories: usize,
    /// Früheste Siegdauer in Jahren seit Spielbeginn.
    pub earliest: Option<i32>,
    /// Siege je Art, nach der Reihenfolge von [`VictoryKind::ALL`] plus Schleierbruch.
    pub by_kind: Vec<(VictoryKind, usize)>,
}

pub fn summarize(runs: &[ObserverRun]) -> ObserverSummary {
    let mut by_kind: Vec<(VictoryKind, usize)> = VictoryKind::ALL.iter().map(|k| (*k, 0)).collect();
    by_kind.push((VictoryKind::WorldFalls, 0));
    let mut s = ObserverSummary { runs: runs.len(), by_kind, ..Default::default() };
    for r in runs {
        if let Some((k, _, _)) = &r.victory {
            s.victories += 1;
            if let Some(e) = by_kind_mut(&mut s.by_kind, *k) {
                *e += 1;
            }
            let after = r.victory_after_years().unwrap_or(0);
            s.earliest = Some(s.earliest.map_or(after, |e| e.min(after)));
        }
    }
    s
}

fn by_kind_mut(v: &mut [(VictoryKind, usize)], k: VictoryKind) -> Option<&mut usize> {
    v.iter_mut().find(|(x, _)| *x == k).map(|(_, n)| n)
}
