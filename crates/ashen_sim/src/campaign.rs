//! Die Echtzeit-Kampagne „Das Zeitalter der Gefallenen“: Aufbau aus Zeitaltern und Missionen,
//! Freischaltung, Entscheidungen (Flags), Enden und der gespeicherte Fortschritt je Profil.
//!
//! Die Struktur liegt als RON unter `assets/data/campaign/campaign.ron`; die Missionen selbst sind
//! gewöhnliche [`MissionDef`]s mit Varianten, die von den Flags früherer Entscheidungen abhängen.

use crate::data::GameData;
use crate::mission::{Action, Carry, Difficulty, GameMode, HeroRecord, MatchSettings, MissionDef, Outcome};
use crate::world::SimWorld;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Eine Mission im Kampagnenplan.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignMission {
    /// Mission-ID (Datei `<id>.ron`).
    pub id: String,
    /// Missionen, die vorher abgeschlossen (oder übersprungen) sein müssen.
    #[serde(default)]
    pub needs: Vec<String>,
    /// Die Mission erscheint nur, wenn alle diese Flags gesetzt sind.
    #[serde(default)]
    pub if_flags: Vec<String>,
    /// Die Mission entfällt, wenn eines dieser Flags gesetzt ist.
    #[serde(default)]
    pub unless_flags: Vec<String>,
    /// Letzte Mission der Kampagne; ihr Abschluss beendet sie und zeigt das Ende.
    #[serde(default)]
    pub finale: bool,
    /// Lage auf der Pergamentkarte (0..1, von links oben).
    #[serde(default)]
    pub pos: (f32, f32),
}

/// Ein Zeitalter mit seinen Missionen.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub numeral: String,
    pub title: String,
    pub subtitle: String,
    pub years: String,
    pub blurb: String,
    /// Audio-Event der Kapitel-Erzählung.
    #[serde(default)]
    pub voice: Option<String>,
    pub missions: Vec<CampaignMission>,
}

/// Ende der Kampagne; das erste Ende, dessen Flags alle gesetzt sind, gilt.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ending {
    pub id: String,
    pub title: String,
    pub text: String,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub voice: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CampaignDef {
    pub id: String,
    pub name: String,
    pub intro: String,
    pub chapters: Vec<Chapter>,
    pub endings: Vec<Ending>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MissionStatus {
    Completed,
    Available,
    /// Vorgänger fehlen noch.
    Locked,
    /// Entfällt wegen früherer Entscheidungen.
    Skipped,
}

impl CampaignDef {
    pub fn load(path: &Path) -> Result<CampaignDef, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        ron::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// Schreibt den Plan als RON (für den Szenario-Generator).
    pub fn to_ron(&self) -> Result<String, String> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default().indentor("  ").depth_limit(4)).map_err(|e| e.to_string())
    }

    pub fn missions(&self) -> impl Iterator<Item = (&Chapter, &CampaignMission)> {
        self.chapters.iter().flat_map(|c| c.missions.iter().map(move |m| (c, m)))
    }

    pub fn entry(&self, id: &str) -> Option<(&Chapter, &CampaignMission)> {
        self.missions().find(|(_, m)| m.id == id)
    }

    pub fn mission_count(&self) -> usize {
        self.missions().count()
    }

    fn flags_ok(m: &CampaignMission, p: &CampaignProgress) -> bool {
        m.if_flags.iter().all(|f| p.has_flag(f)) && !m.unless_flags.iter().any(|f| p.has_flag(f))
    }

    /// Ob die Mission wegen der Flags entfällt (ohne Rücksicht auf Vorgänger).
    pub fn is_skipped(&self, p: &CampaignProgress, id: &str) -> bool {
        self.entry(id).is_some_and(|(_, m)| !Self::flags_ok(m, p))
    }

    pub fn status(&self, p: &CampaignProgress, id: &str) -> MissionStatus {
        let Some((_, m)) = self.entry(id) else { return MissionStatus::Locked };
        if p.is_completed(id) {
            return MissionStatus::Completed;
        }
        let needs_ok = m.needs.iter().all(|n| p.is_completed(n) || self.is_skipped(p, n));
        if !needs_ok {
            return MissionStatus::Locked;
        }
        if !Self::flags_ok(m, p) {
            return MissionStatus::Skipped;
        }
        MissionStatus::Available
    }

    /// Alle derzeit spielbaren, noch nicht abgeschlossenen Missionen in Kampagnenreihenfolge.
    pub fn available(&self, p: &CampaignProgress) -> Vec<&str> {
        self.missions().filter(|(_, m)| self.status(p, &m.id) == MissionStatus::Available).map(|(_, m)| m.id.as_str()).collect()
    }

    /// Empfohlene nächste Mission (erste offene).
    pub fn next(&self, p: &CampaignProgress) -> Option<&str> {
        self.available(p).into_iter().next()
    }

    /// Ob die Kampagne beendet ist (Schlussmission gewonnen).
    pub fn is_finished(&self, p: &CampaignProgress) -> bool {
        self.missions().any(|(_, m)| m.finale && p.is_completed(&m.id))
    }

    /// Das zum Spielstand passende Ende.
    pub fn ending(&self, p: &CampaignProgress) -> Option<&Ending> {
        self.endings.iter().find(|e| e.requires.iter().all(|f| p.has_flag(f)))
    }

    /// Anzahl der abgeschlossenen Missionen des Plans.
    pub fn completed_count(&self, p: &CampaignProgress) -> usize {
        self.missions().filter(|(_, m)| p.is_completed(&m.id)).count()
    }

    /// Prüft den Plan gegen die geladenen Missionen; liefert Fehlertexte.
    pub fn validate(&self, missions: &[MissionDef]) -> Vec<String> {
        let mut errs = Vec::new();
        let mut seen: Vec<&str> = Vec::new();
        for (_, m) in self.missions() {
            if seen.contains(&m.id.as_str()) {
                errs.push(format!("Mission {} kommt mehrfach vor", m.id));
            }
            seen.push(&m.id);
            if !missions.iter().any(|d| d.id == m.id) {
                errs.push(format!("Mission {} hat keine Datei", m.id));
            }
            for n in &m.needs {
                if self.entry(n).is_none() {
                    errs.push(format!("{}: Vorgänger {n} unbekannt", m.id));
                }
            }
        }
        if !self.missions().any(|(_, m)| m.finale) {
            errs.push("Keine Schlussmission (finale)".into());
        }
        if self.endings.len() < 2 {
            errs.push("Mindestens zwei Enden erforderlich".into());
        }
        if self.endings.last().is_none_or(|e| !e.requires.is_empty()) {
            errs.push("Das letzte Ende muss ohne Bedingung gelten (Standardende)".into());
        }
        errs
    }
}

/// Ergebnis einer abgeschlossenen Mission.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MissionResult {
    pub id: String,
    pub time: f32,
    pub kills: u32,
    pub lost: u32,
    pub trained: u32,
    /// Getroffene Entscheidungen (ID, gewähltes Flag).
    pub choices: Vec<(String, String)>,
}

pub const PROGRESS_VERSION: u32 = 1;
/// Anzahl der Profile (Spielstandplätze).
pub const SLOT_COUNT: usize = 5;

/// Gespeicherter Kampagnenfortschritt eines Profils.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CampaignProgress {
    pub version: u32,
    pub name: String,
    pub difficulty: Difficulty,
    pub hero_permadeath: bool,
    pub completed: Vec<MissionResult>,
    pub flags: Vec<String>,
    pub heroes: Vec<HeroRecord>,
    /// Zuletzt gewählte Mission.
    pub selected: Option<String>,
}

impl Default for CampaignProgress {
    fn default() -> Self {
        CampaignProgress { version: PROGRESS_VERSION, name: "Wächterin".into(), difficulty: Difficulty::Normal, hero_permadeath: false, completed: Vec::new(), flags: Vec::new(), heroes: Vec::new(), selected: None }
    }
}

impl CampaignProgress {
    pub fn new(name: &str) -> CampaignProgress {
        CampaignProgress { name: if name.trim().is_empty() { "Wächterin".into() } else { name.trim().to_string() }, ..Default::default() }
    }

    pub fn is_completed(&self, id: &str) -> bool {
        self.completed.iter().any(|r| r.id == id)
    }

    pub fn has_flag(&self, f: &str) -> bool {
        self.flags.iter().any(|x| x == f)
    }

    pub fn hero(&self, unit: &str) -> Option<&HeroRecord> {
        self.heroes.iter().find(|h| h.unit == unit)
    }

    /// Kampagnenzustand für die nächste Mission.
    pub fn carry(&self) -> Carry {
        Carry { flags: self.flags.clone(), heroes: self.heroes.clone() }
    }

    /// Spielbereite Mission und Startoptionen für diesen Fortschritt.
    pub fn start(&self, def: &MissionDef) -> (MissionDef, MatchSettings) {
        let mut m = def.clone();
        m.carry = self.carry();
        let settings = MatchSettings { mode: GameMode::Campaign, difficulty: self.difficulty, seed: def.map_seed, hero_permadeath: self.hero_permadeath };
        (m, settings)
    }

    /// Übernimmt das Ergebnis einer Mission; nur Siege schalten weiter. Gibt zurück, ob etwas gespeichert wurde.
    pub fn record(&mut self, def: &MissionDef, world: &SimWorld, data: &GameData) -> bool {
        if world.mission.outcome != Some(Outcome::Victory) {
            return false;
        }
        // Bei Wiederholung gelten die alten Entscheidungen dieser Mission nicht mehr.
        for f in decision_flags(def) {
            self.flags.retain(|x| *x != f);
        }
        for f in &world.mission.flags {
            if !self.has_flag(f) {
                self.flags.push(f.clone());
            }
        }
        for h in world.capture_heroes(data) {
            self.heroes.retain(|x| x.unit != h.unit);
            self.heroes.push(h);
        }
        let s = &world.players[0].stats;
        let res = MissionResult { id: def.id.clone(), time: world.time, kills: s.kills, lost: s.units_lost, trained: s.units_trained, choices: world.mission.choices.clone() };
        self.completed.retain(|r| r.id != def.id);
        self.completed.push(res);
        true
    }

    /// Entfernt Verweise auf Missionen, die der Plan nicht (mehr) kennt, und repariert Werte.
    pub fn sanitize(&mut self, campaign: &CampaignDef) {
        self.completed.retain(|r| campaign.entry(&r.id).is_some());
        let mut seen: Vec<String> = Vec::new();
        self.completed.retain(|r| {
            let new = !seen.contains(&r.id);
            seen.push(r.id.clone());
            new
        });
        self.flags.sort();
        self.flags.dedup();
        if self.selected.as_ref().is_some_and(|s| campaign.entry(s).is_none()) {
            self.selected = None;
        }
        self.version = PROGRESS_VERSION;
    }

    /// Schreibt atomar (erst temporäre Datei, dann umbenennen).
    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
        }
        let text = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default().indentor("  ")).map_err(|e| e.to_string())?;
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, text).map_err(|e| format!("{}: {e}", tmp.display()))?;
        std::fs::rename(&tmp, path).map_err(|e| format!("{}: {e}", path.display()))
    }

    pub fn load(path: &Path) -> Result<CampaignProgress, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        ron::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }
}

/// Alle Flags, die Entscheidungen der Mission setzen können.
pub fn decision_flags(def: &MissionDef) -> Vec<String> {
    fn walk(actions: &[Action], out: &mut Vec<String>) {
        for a in actions {
            if let Action::Decision(d) = a {
                for o in &d.options {
                    out.push(o.flag.clone());
                    walk(&o.actions, out);
                }
            }
        }
    }
    let mut out = Vec::new();
    for t in &def.triggers {
        walk(&t.actions, &mut out);
    }
    out
}

/// Zustand eines Profils für die Anzeige.
#[derive(Clone, Debug)]
pub enum SlotState {
    Empty,
    Ready(Box<CampaignProgress>),
    /// Datei vorhanden, aber nicht lesbar (wurde nicht überschrieben).
    Damaged(String),
}

pub fn slot_path(dir: &Path, index: usize) -> PathBuf {
    dir.join(format!("profil_{}.ron", index + 1))
}

/// Liest ein Profil. Fehlende Dateien gelten als leer, beschädigte werden gemeldet, ohne sie zu löschen.
pub fn read_slot(dir: &Path, index: usize, campaign: &CampaignDef) -> SlotState {
    let p = slot_path(dir, index);
    if !p.exists() {
        return SlotState::Empty;
    }
    match CampaignProgress::load(&p) {
        Ok(mut prog) => {
            prog.sanitize(campaign);
            SlotState::Ready(Box::new(prog))
        }
        Err(e) => SlotState::Damaged(e),
    }
}

/// Sichert eine beschädigte Datei als `.defekt` und gibt den Platz frei.
pub fn quarantine_slot(dir: &Path, index: usize) -> Result<(), String> {
    let p = slot_path(dir, index);
    if p.exists() {
        std::fs::rename(&p, p.with_extension("defekt")).map_err(|e| format!("{}: {e}", p.display()))?;
    }
    Ok(())
}

pub fn delete_slot(dir: &Path, index: usize) -> Result<(), String> {
    let p = slot_path(dir, index);
    if p.exists() {
        std::fs::remove_file(&p).map_err(|e| format!("{}: {e}", p.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan() -> CampaignDef {
        let m = |id: &str, needs: &[&str]| CampaignMission { id: id.into(), needs: needs.iter().map(|s| s.to_string()).collect(), if_flags: vec![], unless_flags: vec![], finale: false, pos: (0.0, 0.0) };
        let mut b = m("b", &["a"]);
        b.if_flags = vec!["x".into()];
        let mut c = m("c", &["a"]);
        c.unless_flags = vec!["x".into()];
        let mut d = m("d", &["b", "c"]);
        d.finale = true;
        CampaignDef {
            id: "t".into(),
            name: "T".into(),
            intro: String::new(),
            chapters: vec![Chapter { id: "k".into(), numeral: "I".into(), title: "K".into(), subtitle: String::new(), years: String::new(), blurb: String::new(), voice: None, missions: vec![m("a", &[]), b, c, d] }],
            endings: vec![Ending { id: "e1".into(), title: "E1".into(), text: String::new(), requires: vec!["x".into()], voice: None }, Ending { id: "e2".into(), title: "E2".into(), text: String::new(), requires: vec![], voice: None }],
        }
    }

    fn done(p: &mut CampaignProgress, id: &str) {
        p.completed.push(MissionResult { id: id.into(), ..Default::default() });
    }

    #[test]
    fn unlocking_follows_needs_and_flags() {
        let c = plan();
        let mut p = CampaignProgress::new("Test");
        assert_eq!(c.available(&p), vec!["a"]);
        assert_eq!(c.status(&p, "d"), MissionStatus::Locked);
        done(&mut p, "a");
        // Ohne Flag entfällt b, c ist offen.
        assert_eq!(c.available(&p), vec!["c"]);
        assert_eq!(c.status(&p, "b"), MissionStatus::Skipped);
        p.flags.push("x".into());
        assert_eq!(c.available(&p), vec!["b"]);
        done(&mut p, "b");
        assert!(!c.is_finished(&p));
        // c entfällt wegen des Flags und blockiert d nicht.
        assert_eq!(c.available(&p), vec!["d"]);
        done(&mut p, "d");
        assert!(c.is_finished(&p));
        assert_eq!(c.ending(&p).unwrap().id, "e1");
        p.flags.clear();
        assert_eq!(c.ending(&p).unwrap().id, "e2");
    }

    #[test]
    fn progress_roundtrip_and_damaged_files() {
        let dir = std::env::temp_dir().join(format!("ashen_campaign_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let c = plan();
        assert!(matches!(read_slot(&dir, 0, &c), SlotState::Empty));
        let mut p = CampaignProgress::new("Isolde");
        p.flags.push("x".into());
        done(&mut p, "a");
        done(&mut p, "ghost");
        p.save(&slot_path(&dir, 0)).unwrap();
        match read_slot(&dir, 0, &c) {
            SlotState::Ready(q) => {
                assert_eq!(q.name, "Isolde");
                assert!(q.is_completed("a") && !q.is_completed("ghost"), "unbekannte Missionen werden bereinigt");
            }
            other => panic!("{other:?}"),
        }
        // Beschädigte und alte (unvollständige) Dateien.
        std::fs::write(slot_path(&dir, 1), "(((kaputt").unwrap();
        assert!(matches!(read_slot(&dir, 1, &c), SlotState::Damaged(_)));
        quarantine_slot(&dir, 1).unwrap();
        assert!(matches!(read_slot(&dir, 1, &c), SlotState::Empty));
        std::fs::write(slot_path(&dir, 2), "(name: \"Alt\", completed: [(id: \"a\")])").unwrap();
        match read_slot(&dir, 2, &c) {
            SlotState::Ready(q) => assert!(q.is_completed("a") && q.difficulty == Difficulty::Normal),
            other => panic!("{other:?}"),
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
