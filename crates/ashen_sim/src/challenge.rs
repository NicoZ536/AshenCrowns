//! Herausforderungen: datengetriebene Regelvarianten auf Missionen und Szenarien, mit Wertung
//! und lokal gespeicherter Bestenliste.
//!
//! Eine Herausforderung (`assets/data/challenges/*.ron`) verweist auf eine Mission oder ein
//! Szenario (`base`), setzt Modus, Schwierigkeit und Modifikatoren und definiert, wie das
//! Ergebnis bewertet wird. Die Modifikatoren werden vor dem Start in die [`MissionDef`] eingearbeitet
//! und wirken deshalb ohne Sonderlogik in der Simulation.

use crate::mission::{Action, Condition, Difficulty, GameMode, MatchSettings, MissionDef, Outcome, TriggerDef};
use crate::world::SimWorld;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Regeländerung einer Herausforderung.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Modifier {
    /// Höchstens so viele Streitkräfte (ohne Arbeiter und Heldin).
    UnitCap(u32),
    /// Faktor auf die Stärke der Angriffswellen.
    WaveStrength(f32),
    /// Faktor auf die Wellenabstände (kleiner als 1: häufigere Angriffe).
    WaveInterval(f32),
    /// Faktor auf Startressourcen und Startwaren.
    StartResources(f32),
    /// Niederlage, sobald die Zeit (Sekunden Spielzeit) abgelaufen ist.
    TimeLimit(f32),
    /// Niederlage, sobald das Hauptgebäude Schaden nimmt.
    KeepIntact,
    /// Die Heldin kehrt nach dem Tod nicht zurück.
    HeroPermadeath,
    /// Kein Speichern und Laden während der Partie.
    IronMan,
    /// Ohne Heldin starten.
    NoHero,
}

impl Modifier {
    /// Kurzbeschreibung für die Anzeige.
    pub fn text(&self) -> String {
        match self {
            Modifier::UnitCap(n) => format!("Höchstens {n} Soldaten"),
            Modifier::WaveStrength(f) => format!("Wellen {:+.0} % stärker", (f - 1.0) * 100.0),
            Modifier::WaveInterval(f) => format!("Wellenabstand ×{f:.2}"),
            Modifier::StartResources(f) => format!("Startvorräte ×{f:.2}"),
            Modifier::TimeLimit(s) => format!("Zeitlimit {}:{:02}", *s as u32 / 60, *s as u32 % 60),
            Modifier::KeepIntact => "Der Bergfried darf keinen Schaden nehmen".into(),
            Modifier::HeroPermadeath => "Dauerhafter Heldentod".into(),
            Modifier::IronMan => "Eiserner Mann: kein Speichern".into(),
            Modifier::NoHero => "Ohne Heldin".into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RankBy {
    /// Höchste Punktzahl gewinnt.
    #[default]
    Score,
    /// Kürzeste Zeit eines Siegs gewinnt.
    Time,
}

/// Bewertungsregel: Punkte = Siegprämie + Summe der gewichteten Kennzahlen + Zeitbonus.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Scoring {
    pub victory_bonus: f32,
    pub per_kill: f32,
    pub per_unit_lost: f32,
    pub per_building: f32,
    pub per_resident: f32,
    pub per_wave: f32,
    pub per_night: f32,
    /// Zeitbonus: (`par_time` − Spielzeit) × `time_bonus_per_sec`, nie negativ.
    pub par_time: f32,
    pub time_bonus_per_sec: f32,
    pub rank_by: RankBy,
    /// Auch Niederlagen kommen in die Bestenliste (Überlebensmodi).
    pub counts_when_defeated: bool,
}

impl Default for Scoring {
    fn default() -> Self {
        Scoring { victory_bonus: 1000.0, per_kill: 5.0, per_unit_lost: -8.0, per_building: 3.0, per_resident: 0.0, per_wave: 0.0, per_night: 0.0, par_time: 0.0, time_bonus_per_sec: 0.0, rank_by: RankBy::Score, counts_when_defeated: false }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChallengeDef {
    pub id: String,
    pub name: String,
    pub description: String,
    /// ID einer Mission oder eines Szenarios.
    pub base: String,
    #[serde(default = "default_mode")]
    pub mode: GameMode,
    #[serde(default = "default_difficulty")]
    pub difficulty: Difficulty,
    #[serde(default)]
    pub seed: u64,
    #[serde(default)]
    pub modifiers: Vec<Modifier>,
    /// Zusätzliche Siegbedingung der Herausforderung (z. B. `DayStarted(21)`).
    #[serde(default)]
    pub victory: Option<Condition>,
    /// Ziel als Satz für die Einsatzliste.
    #[serde(default)]
    pub goal_text: String,
    #[serde(default)]
    pub scoring: Scoring,
}

fn default_mode() -> GameMode {
    GameMode::Campaign
}

fn default_difficulty() -> Difficulty {
    Difficulty::Normal
}

impl ChallengeDef {
    pub fn load(path: &Path) -> Result<ChallengeDef, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        ron::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// Lädt alle `*.ron` eines Ordners (sortiert); Fehler werden gesammelt.
    pub fn load_dir(dir: &Path, errors: &mut Vec<String>) -> Vec<ChallengeDef> {
        let Ok(rd) = std::fs::read_dir(dir) else { return Vec::new() };
        let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "ron")).collect();
        files.sort();
        let mut out = Vec::new();
        for f in files {
            match ChallengeDef::load(&f) {
                Ok(c) => out.push(c),
                Err(e) => errors.push(e),
            }
        }
        out
    }

    pub fn iron_man(&self) -> bool {
        self.modifiers.contains(&Modifier::IronMan)
    }
}

/// Eine für den Start vorbereitete Herausforderung.
#[derive(Clone, Debug)]
pub struct PreparedChallenge {
    pub mission: MissionDef,
    pub settings: MatchSettings,
    pub iron_man: bool,
}

/// Arbeitet die Modifikatoren in die Mission ein.
pub fn prepare(def: &ChallengeDef, base: &MissionDef) -> PreparedChallenge {
    let mut m = base.clone();
    let mut settings = MatchSettings { mode: def.mode, difficulty: def.difficulty, seed: if def.seed == 0 { base.map_seed } else { def.seed }, hero_permadeath: false };
    for md in &def.modifiers {
        match md {
            Modifier::UnitCap(n) => m.unit_cap = Some(*n),
            Modifier::WaveStrength(f) => {
                m.waves.base_budget *= f;
                m.waves.budget_per_wave *= f;
            }
            Modifier::WaveInterval(f) => {
                m.waves.first_wave *= f;
                m.waves.interval_day *= f;
                m.waves.interval_night *= f;
            }
            Modifier::StartResources(f) => {
                m.start_resources = m.start_resources.scaled(*f);
                for g in &mut m.start_goods {
                    g.1 *= f;
                }
            }
            Modifier::TimeLimit(s) => m.triggers.push(TriggerDef {
                id: "challenge_time_limit".into(),
                condition: Condition::AtTime(*s),
                actions: vec![Action::Message { speaker: "Herausforderung".into(), text: "Die Zeit ist abgelaufen.".into(), voice: None }, Action::Defeat],
                modes: vec![],
            }),
            Modifier::KeepIntact => {
                if !m.defeat_tag.is_empty() {
                    m.triggers.push(TriggerDef {
                        id: "challenge_keep_intact".into(),
                        condition: Condition::TagDamaged(m.defeat_tag.clone()),
                        actions: vec![Action::Message { speaker: "Herausforderung".into(), text: "Der Bergfried wurde beschädigt – die Herausforderung ist verloren.".into(), voice: None }, Action::Defeat],
                        modes: vec![],
                    });
                }
            }
            Modifier::HeroPermadeath => settings.hero_permadeath = true,
            Modifier::IronMan => {}
            Modifier::NoHero => m.placements.retain(|p| !(p.owner == 0 && p.id.starts_with("hero_"))),
        }
    }
    if let Some(c) = &def.victory {
        m.triggers.push(TriggerDef {
            id: "challenge_victory".into(),
            condition: c.clone(),
            actions: vec![Action::Message { speaker: "Herausforderung".into(), text: format!("Geschafft: {}", def.name), voice: None }, Action::Victory],
            modes: vec![],
        });
    }
    if !def.goal_text.is_empty() {
        m.description = format!("{}\n\nHerausforderung: {}", base.description, def.goal_text);
    }
    PreparedChallenge { mission: m, settings, iron_man: def.iron_man() }
}

/// Ergebnis einer beendeten Partie.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub victory: bool,
    pub time: f32,
    pub kills: u32,
    pub units_lost: u32,
    pub buildings: u32,
    pub residents: u32,
    pub waves: u32,
    pub nights: u32,
    pub score: i64,
}

/// Wertet eine beendete Partie aus (`None`, solange kein Ergebnis feststeht).
pub fn evaluate(def: &ChallengeDef, world: &SimWorld) -> Option<ChallengeResult> {
    let outcome = world.mission.outcome?;
    let st = &world.players[0].stats;
    let victory = outcome == Outcome::Victory;
    let sc = &def.scoring;
    let time = world.time;
    let residents = world.players[0].residents.max(0.0) as u32;
    let waves = world.mission.waves_survived;
    let nights = world.env.night;
    let mut score = st.kills as f32 * sc.per_kill + st.units_lost as f32 * sc.per_unit_lost + st.buildings_built as f32 * sc.per_building + residents as f32 * sc.per_resident + waves as f32 * sc.per_wave + nights as f32 * sc.per_night;
    if victory {
        score += sc.victory_bonus;
        if sc.par_time > 0.0 {
            score += (sc.par_time - time).max(0.0) * sc.time_bonus_per_sec;
        }
    }
    Some(ChallengeResult { victory, time, kills: st.kills, units_lost: st.units_lost, buildings: st.buildings_built, residents, waves, nights, score: score.round().max(0.0) as i64 })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub challenge: String,
    pub name: String,
    pub score: i64,
    pub time: f32,
    pub victory: bool,
    /// Sekunden seit der Unix-Epoche.
    pub date: u64,
}

/// Lokale Bestenliste (RON-Datei im Benutzerordner).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Leaderboard {
    pub entries: Vec<LeaderboardEntry>,
}

pub const LEADERBOARD_KEEP: usize = 10;

impl Leaderboard {
    pub fn load(path: &Path) -> Leaderboard {
        std::fs::read_to_string(path).ok().and_then(|t| ron::from_str(&t).ok()).unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        let text = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).map_err(|e| e.to_string())?;
        std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
    }

    fn better(rank: RankBy, a: &LeaderboardEntry, b: &LeaderboardEntry) -> std::cmp::Ordering {
        match rank {
            RankBy::Score => b.score.cmp(&a.score).then(a.time.total_cmp(&b.time)),
            RankBy::Time => a.time.total_cmp(&b.time).then(b.score.cmp(&a.score)),
        }
    }

    /// Bestenliste einer Herausforderung, beste zuerst.
    pub fn top(&self, def: &ChallengeDef, n: usize) -> Vec<&LeaderboardEntry> {
        let mut v: Vec<&LeaderboardEntry> = self.entries.iter().filter(|e| e.challenge == def.id).collect();
        v.sort_by(|a, b| Self::better(def.scoring.rank_by, a, b));
        v.truncate(n);
        v
    }

    /// Trägt ein Ergebnis ein; liefert den Rang (0 = Bestwert) oder `None`, wenn es nicht zählt.
    pub fn submit(&mut self, def: &ChallengeDef, name: &str, r: &ChallengeResult, date: u64) -> Option<usize> {
        if !r.victory && !def.scoring.counts_when_defeated {
            return None;
        }
        let entry = LeaderboardEntry { challenge: def.id.clone(), name: name.to_string(), score: r.score, time: r.time, victory: r.victory, date };
        let rank = 1 + self.top(def, usize::MAX).iter().filter(|e| Self::better(def.scoring.rank_by, e, &entry) == std::cmp::Ordering::Less).count();
        self.entries.push(entry);
        // Pro Herausforderung nur die besten Einträge behalten.
        let keep: Vec<LeaderboardEntry> = self.top(def, LEADERBOARD_KEEP).into_iter().cloned().collect();
        self.entries.retain(|e| e.challenge != def.id);
        self.entries.extend(keep);
        if rank > LEADERBOARD_KEEP { None } else { Some(rank - 1) }
    }

    pub fn best(&self, def: &ChallengeDef) -> Option<&LeaderboardEntry> {
        self.top(def, 1).into_iter().next()
    }
}

/// Zeit als `m:ss`.
pub fn format_time(secs: f32) -> String {
    let s = secs.max(0.0) as u32;
    format!("{}:{:02}", s / 60, s % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn def(rank: RankBy, defeated: bool) -> ChallengeDef {
        ChallengeDef {
            id: "t".into(),
            name: "T".into(),
            description: String::new(),
            base: "x".into(),
            mode: GameMode::Campaign,
            difficulty: Difficulty::Normal,
            seed: 0,
            modifiers: vec![],
            victory: None,
            goal_text: String::new(),
            scoring: Scoring { rank_by: rank, counts_when_defeated: defeated, ..Default::default() },
        }
    }

    fn res(victory: bool, time: f32, score: i64) -> ChallengeResult {
        ChallengeResult { victory, time, kills: 0, units_lost: 0, buildings: 0, residents: 0, waves: 0, nights: 0, score }
    }

    #[test]
    fn leaderboard_ranks_by_score_and_time() {
        let mut lb = Leaderboard::default();
        let d = def(RankBy::Score, false);
        assert_eq!(lb.submit(&d, "A", &res(true, 100.0, 500), 1), Some(0));
        assert_eq!(lb.submit(&d, "B", &res(true, 100.0, 900), 2), Some(0));
        assert_eq!(lb.submit(&d, "C", &res(true, 100.0, 700), 3), Some(1));
        assert_eq!(lb.submit(&d, "D", &res(false, 100.0, 9999), 4), None, "Niederlage zählt nicht");
        assert_eq!(lb.top(&d, 5).iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["B", "C", "A"]);
        let t = def(RankBy::Time, false);
        let mut lb = Leaderboard::default();
        lb.submit(&t, "A", &res(true, 300.0, 1), 1);
        lb.submit(&t, "B", &res(true, 200.0, 1), 2);
        assert_eq!(lb.best(&t).unwrap().name, "B");
    }

    #[test]
    fn leaderboard_keeps_top_ten() {
        let mut lb = Leaderboard::default();
        let d = def(RankBy::Score, true);
        for i in 0..25 {
            lb.submit(&d, "X", &res(i % 2 == 0, 10.0, i), i as u64);
        }
        assert_eq!(lb.entries.len(), LEADERBOARD_KEEP);
        assert_eq!(lb.best(&d).unwrap().score, 24);
    }
}
