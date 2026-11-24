//! Schmiede der Echtzeit-Kampagne „Das Zeitalter der Gefallenen“.
//!
//! `cargo run -p ashen_tools -- forge-campaign` erzeugt reproduzierbar
//! - die Missionen unter `assets/data/campaign/` (jede mit eigener, handgeformter Karte),
//! - den Kampagnenplan `assets/data/campaign/campaign.ron`,
//! - die Sprach-Events der Dialoge und Erzählertexte im Audio-Manifest,
//! - die ElevenLabs-Zeilen in `docs/audio/elevenlabs_dialogues.md`.
//!
//! Alle Dialogzeilen entstehen an einer Stelle ([`Script`]); dieselbe Zeile steht damit garantiert
//! in der Mission, im Manifest und im Sprechskript.

use crate::forge::{base_mission, trig};
use ashen_sim::campaign::{CampaignDef, CampaignMission, Chapter, Ending};
use ashen_sim::data::GameData;
use ashen_sim::mission::{Action, Condition, DecisionDef, DecisionOption, DialogueLine, FlagVariant, MissionDef, ObjectiveDef, TriggerDef};
use std::path::Path;

/// Sprecher der Kampagne.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Sp {
    Narrator,
    Isolde,
    Hal,
    Aldric,
    Mireille,
    Torvald,
    Sylaen,
    Ophira,
    Kassian,
    Deep,
    Wendel,
    Nel,
    Gharun,
    Isarre,
    RootMother,
    HollowKing,
}

impl Sp {
    /// Anzeigename im Spiel.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Sp::Narrator => "Erzähler",
            Sp::Isolde => "Isolde Raventhorn",
            Sp::Hal => "Bruder Hal Venn",
            Sp::Aldric => "König Aldric III.",
            Sp::Mireille => "Herzogin Mireille Corvane",
            Sp::Torvald => "Torvald Eisenhand",
            Sp::Sylaen => "Sylaen der Veyl",
            Sp::Ophira => "Ophira Saelith-Varo",
            Sp::Kassian => "Kassian Vale",
            Sp::Deep => "Die Stimme aus dem Tief",
            Sp::Wendel => "Ser Wendel der Graue",
            Sp::Nel => "Nel",
            Sp::Gharun => "Gharun, der Steinerne Eid",
            Sp::Isarre => "Isarre Mol",
            Sp::RootMother => "Die Wurzelmutter",
            Sp::HollowKing => "Der Hohle König",
        }
    }

    /// Kanon-ID (Präfix der Audio-Events und Dateien).
    pub(crate) fn id(self) -> &'static str {
        match self {
            Sp::Narrator => "narrator",
            Sp::Isolde => "hero_isolde",
            Sp::Hal => "char_hal_venn",
            Sp::Aldric => "char_king_aldric",
            Sp::Mireille => "char_mireille",
            Sp::Torvald => "char_torvald",
            Sp::Sylaen => "char_sylaen",
            Sp::Ophira => "char_ophira",
            Sp::Kassian => "char_kassian",
            Sp::Deep => "char_deep_voice",
            Sp::Wendel => "char_wendel",
            Sp::Nel => "char_nel",
            Sp::Gharun => "boss_stone_oath",
            Sp::Isarre => "boss_reef_mistress",
            Sp::RootMother => "boss_root_mother",
            Sp::HollowKing => "boss_hollow_king",
        }
    }

    pub(crate) fn is_narrator(self) -> bool {
        self == Sp::Narrator
    }
}

/// Eine registrierte Sprechzeile (Spiel, Manifest und Sprechskript).
pub(crate) struct VoiceLine {
    pub event: String,
    pub sp: Sp,
    pub text: String,
    pub regie: String,
    pub mission: String,
    pub scene: String,
    pub file: String,
}

/// Sammelt alle Dialogzeilen und liefert dem Spiel die Zeilen mit Audio-Event-ID.
#[derive(Default)]
pub(crate) struct Script {
    pub lines: Vec<VoiceLine>,
    /// (Missions-ID, Titel)
    pub titles: Vec<(String, String)>,
    /// (Missions-ID, Szene, Kontext)
    pub notes: Vec<(String, String, String)>,
}

impl Script {
    pub(crate) fn title(&mut self, mission: &str, title: &str) {
        self.titles.push((mission.into(), title.into()));
    }

    pub(crate) fn note(&mut self, mission: &str, scene: &str, text: &str) {
        self.notes.push((mission.into(), scene.into(), text.into()));
    }

    /// Eine einzelne Zeile (Buchstabe = laufende Nummer in der Szene).
    pub(crate) fn ln(&mut self, mission: &str, scene: &str, sp: Sp, text: &str, regie: &str) -> DialogueLine {
        let n = self.lines.iter().filter(|l| l.mission == mission && l.scene == scene).count();
        assert!(n < 26, "zu viele Zeilen in {mission}/{scene}");
        let letter = (b'a' + n as u8) as char;
        let base = format!("{}_{mission}_{scene}_{letter}", sp.id());
        let dir = if sp.is_narrator() { "narrator" } else { "characters" };
        let event = format!("{}.{mission}_{scene}_{letter}", sp.id());
        self.lines.push(VoiceLine { event: event.clone(), sp, text: text.into(), regie: regie.into(), mission: mission.into(), scene: scene.into(), file: format!("voices/{dir}/{base}") });
        DialogueLine { speaker: sp.name().into(), text: text.into(), voice: Some(event) }
    }

    /// Dialogfolge als Aktion: (Sprecher, Text, Regie).
    pub(crate) fn dlg(&mut self, mission: &str, scene: &str, lines: &[(Sp, &str, &str)]) -> Action {
        Action::Dialogue(lines.iter().map(|(sp, t, r)| self.ln(mission, scene, *sp, t, r)).collect())
    }

    /// Entscheidung mit eingesprochener Frage.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn decision(&mut self, mission: &str, scene: &str, id: &str, title: &str, ask: (Sp, &str, &str), options: Vec<DecisionOption>) -> Action {
        let l = self.ln(mission, scene, ask.0, ask.1, ask.2);
        Action::Decision(DecisionDef { id: id.into(), speaker: l.speaker, title: title.into(), text: l.text, voice: l.voice, options })
    }

}

pub(crate) fn opt(flag: &str, label: &str, detail: &str, requires: &[&str], actions: Vec<Action>) -> DecisionOption {
    DecisionOption { flag: flag.into(), label: label.into(), detail: detail.into(), requires: requires.iter().map(|s| s.to_string()).collect(), actions }
}

/// Variante, die bei gesetzten (`requires`) beziehungsweise fehlenden (`forbids`) Flags wirkt.
pub(crate) fn variant(requires: &[&str], forbids: &[&str], note: &str) -> FlagVariant {
    FlagVariant { requires: requires.iter().map(|s| s.to_string()).collect(), forbids: forbids.iter().map(|s| s.to_string()).collect(), note: note.into(), ..Default::default() }
}

/// Auslöser, der erst nach `t` Sekunden gilt (Kurzform).
pub(crate) fn at(id: &str, t: f32, actions: Vec<Action>) -> TriggerDef {
    trig(id, Condition::AtTime(t), actions)
}

/// Einheiten für eine Seite im Bereich erscheinen lassen.
pub(crate) fn sp_units(unit: &str, owner: u8, area: &str, count: u32, attack: bool, tag: Option<&str>) -> Action {
    Action::SpawnUnits { unit: unit.into(), owner, area: area.into(), count, attack, tag: tag.map(|t| t.into()) }
}

/// Gegnerwelle: mehrere Einheitentypen am selben Ort.
pub(crate) fn wave(area: &str, units: &[(&str, u32)]) -> Vec<Action> {
    units.iter().map(|(u, n)| sp_units(u, 1, area, *n, true, None)).collect()
}

pub(crate) fn all_of(v: Vec<Condition>) -> Condition {
    Condition::All(v)
}

pub(crate) fn flag(f: &str) -> Condition {
    Condition::Flag(f.into())
}

pub(crate) fn any_flag(fs: &[&str]) -> Condition {
    Condition::Any(fs.iter().map(|f| Condition::Flag(f.to_string())).collect())
}

pub(crate) fn not(c: Condition) -> Condition {
    Condition::Not(Box::new(c))
}

pub(crate) fn tag_dead(t: &str) -> Condition {
    Condition::TagDead(t.into())
}

pub(crate) fn in_area(area: &str, count: u32) -> Condition {
    Condition::UnitsInArea { area: area.into(), count }
}

/// Mindestens `count` eigene Einheiten dieser Art im Bereich.
pub(crate) fn unit_in(unit: &str, area: &str, count: u32) -> Condition {
    Condition::UnitKindInArea { unit: unit.into(), area: area.into(), count }
}

pub(crate) fn objective(id: &str, text: &str, optional: bool) -> ObjectiveDef {
    ObjectiveDef { id: id.into(), text: text.into(), optional }
}

pub(crate) fn add_obj(id: &str) -> Action {
    Action::AddObjective(id.into())
}

pub(crate) fn done_obj(id: &str) -> Action {
    Action::CompleteObjective(id.into())
}

pub(crate) fn chron(t: &str) -> Action {
    Action::Chronicle(t.into())
}

/// Missionsgerüst mit Typ, Texten und Musik.
#[allow(clippy::too_many_arguments)]
pub(crate) fn campaign_mission(id: &str, name: &str, year: u32, kind: &str, description: &str, briefing: &str, victory: &str, defeat: &str, music: &[&str]) -> MissionDef {
    let mut m = base_mission(id, name, year, description, briefing);
    m.kind = kind.into();
    m.victory_text = victory.into();
    m.defeat_text = defeat.into();
    m.chronicle_title = name.into();
    m.music = music.iter().map(|s| s.to_string()).collect();
    m
}

/// Heldentod ist in Heldenmissionen eine Niederlage.
pub(crate) fn hero_lost(text: &str) -> TriggerDef {
    trig("hero_lost", Condition::TagDead("hero".into()), vec![Action::Message { speaker: "Erzähler".into(), text: text.into(), voice: None }, Action::Defeat])
}

// ---------------------------------------------------------------------------------------------
// Ausgabe
// ---------------------------------------------------------------------------------------------

const BEGIN_MANIFEST: &str = "        // BEGIN KAMPAGNE (erzeugt von `ashen_tools forge-campaign`, nicht von Hand ändern)";
const END_MANIFEST: &str = "        // END KAMPAGNE";
const BEGIN_DOC: &str = "<!-- BEGIN KAMPAGNE (erzeugt von `ashen_tools forge-campaign`, nicht von Hand ändern) -->";
const END_DOC: &str = "<!-- END KAMPAGNE -->";

fn manifest_block(s: &Script) -> String {
    let mut out = String::new();
    out.push_str(BEGIN_MANIFEST);
    out.push('\n');
    out.push_str("        // Kampagnendialoge: je Zeile ein Event `<sprecher>.<mission>_<szene>_<buchstabe>`, eine Datei `_01`.\n");
    for l in &s.lines {
        let (cat, vol) = if l.sp.is_narrator() { ("Narration", 2.0) } else { ("Dialogue", 1.0) };
        out.push_str(&format!("        SoundEvent(id: \"{}\", category: {cat}, files: [\"{}_*\"], volume_db: {vol:.1}, cooldown_ms: 0, max_instances: 1),\n", l.event, l.file));
    }
    out.push_str(END_MANIFEST);
    out.push('\n');
    out
}

fn voice_profile(sp: Sp) -> String {
    format!("AC_{}", sp.id())
}

/// Entfernt Eleven-v3-Audio-Tags (`[solemn]`, `[short pause]`) und normalisiert Leerraum.
fn strip_audio_tags(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find('[') {
        let after = &rest[i + 1..];
        match after.find(']') {
            Some(j) if !after[..j].is_empty() && after[..j].chars().all(|c| c.is_ascii_lowercase() || c == ' ' || c == '-' || c == ',' || c == '\'') => {
                out.push_str(&rest[..i]);
                out.push(' ');
                rest = &after[j + 1..];
            }
            _ => {
                out.push_str(&rest[..=i]);
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Liest aus dem bestehenden Kampagnenblock die von Hand ergänzten Sprechtexte mit Audio-Tags (Dateiname → Text).
/// Sie bleiben beim Neuerzeugen erhalten, solange sie ohne Tags dem Spieltext entsprechen.
fn existing_tagged_texts(doc: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let (Some(b), Some(e)) = (doc.find(BEGIN_DOC), doc.find(END_DOC)) else { return map };
    if e < b {
        return map;
    }
    for line in doc[b..e].lines().filter(|l| l.starts_with("| ")) {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        if cells.len() >= 6 && cells[2].starts_with('`') && cells[2].ends_with(".ogg`") {
            map.insert(cells[2].trim_matches('`').to_string(), cells[4].to_string());
        }
    }
    map
}

fn doc_block(s: &Script, tagged: &std::collections::HashMap<String, String>) -> String {
    let mut out = String::new();
    out.push_str(BEGIN_DOC);
    out.push('\n');
    out.push_str("# Teil 3 – Echtzeit-Kampagne „Das Zeitalter der Gefallenen“ (Missionsdialoge, im Manifest)\n\n");
    out.push_str("Dieser Teil wird aus den Missionsdaten erzeugt (`cargo run -p ashen_tools -- forge-campaign`): Jede Zeile steht\n");
    out.push_str("identisch in der Mission (`assets/data/campaign/*.ron`), als Event im Manifest und hier. **Sprechtext = exakt der deutsche Spieltext**,\n");
    out.push_str("Die Audio-Tags im Sprechtext sind von Hand ergänzt und bleiben beim Neuerzeugen erhalten, solange der Text ohne Tags dem Spieltext entspricht (sonst steht der Spieltext ohne Tags da und muss neu getaggt werden).\n\n");
    out.push_str("**Konventionen**\n");
    out.push_str("- Event: `<sprecher-id>.<mission>_<szene>_<buchstabe>`; Datei: `voices/narrator/narrator_<mission>_<szene>_<buchstabe>_01.ogg` bzw. `voices/characters/<sprecher-id>_<mission>_<szene>_<buchstabe>_01.ogg`.\n");
    out.push_str("- Jede Zeile ein eigenes Event mit genau einer Datei (`_01`); die Reihenfolge der Zeilen steht in der Spalte **#**. Das Spiel spielt die Zeilen einer Szene nacheinander; die Pausen ergeben sich aus der Textlänge.\n");
    out.push_str("- Stimme: Profil `AC_<sprecher-id>` aus `elevenlabs_characters.md` (Sprache **Deutsch**, alle Stimmen inklusive Erzähler mit Eleven v3, Stability-Modus *Natural*; jede Zeile enthält Eleven-v3-Audio-Tags in eckigen Klammern, die nicht gesprochen werden und nicht in den Spieltext gehören). Akzentangaben der Profile gelten als leichte Färbung.\n");
    out.push_str("- Zielformat und Loudness wie in Teil 1 (Ogg Vorbis q6, 48 kHz, mono, −16 LUFS, −1 dBTP). Manifest: Erzähler +2 dB, Figuren +1 dB.\n");
    out.push_str("- Neue Sprecher dieses Teils: `char_wendel`, `char_nel`, `boss_stone_oath`, `boss_reef_mistress` (Profile in `elevenlabs_characters.md`, Teil F).\n\n");
    let mut missions: Vec<&str> = Vec::new();
    for l in &s.lines {
        if !missions.contains(&l.mission.as_str()) {
            missions.push(&l.mission);
        }
    }
    for m in missions {
        let title = s.titles.iter().find(|t| t.0 == m).map(|t| t.1.as_str()).unwrap_or(m);
        out.push_str(&format!("## {title} (`{m}`)\n\n"));
        let mut scenes: Vec<&str> = Vec::new();
        for l in s.lines.iter().filter(|l| l.mission == m) {
            if !scenes.contains(&l.scene.as_str()) {
                scenes.push(&l.scene);
            }
        }
        for sc in scenes {
            out.push_str(&format!("### `{m}_{sc}`\n"));
            if let Some(n) = s.notes.iter().find(|n| n.0 == m && n.1 == sc) {
                out.push_str(&format!("*Kontext:* {}\n", n.2));
            }
            out.push('\n');
            out.push_str("| # | Ziel-Datei | Sprecher (Stimme) | Sprechtext (DE, exakt wie im Spiel) | Regie |\n|---|---|---|---|---|\n");
            for (i, l) in s.lines.iter().filter(|l| l.mission == m && l.scene == sc).enumerate() {
                let plain = l.text.replace('|', "/");
                let key = format!("{}_01.ogg", l.file);
                let spoken = tagged.get(&key).filter(|t| strip_audio_tags(t) == strip_audio_tags(&plain)).cloned().unwrap_or(plain);
                out.push_str(&format!("| {} | `{}` | {} (`{}`) | {} | {} |\n", i + 1, key, l.sp.name(), voice_profile(l.sp), spoken, l.regie));
            }
            out.push('\n');
        }
    }
    out.push_str(END_DOC);
    out.push('\n');
    out
}

/// Ersetzt den Bereich zwischen den Markern oder fügt ihn vor `anchor` ein.
fn splice(text: &str, begin: &str, end: &str, block: &str, anchor: &str) -> Result<String, String> {
    if let (Some(b), Some(e)) = (text.find(begin), text.find(end)) {
        let e_end = e + end.len();
        let e_end = text[e_end..].find('\n').map(|n| e_end + n + 1).unwrap_or(text.len());
        let b_start = text[..b].rfind('\n').map(|n| n + 1).unwrap_or(0);
        return Ok(format!("{}{}{}", &text[..b_start], block, &text[e_end..]));
    }
    let i = text.find(anchor).ok_or_else(|| format!("Anker '{anchor}' nicht gefunden"))?;
    let i = text[..i].rfind('\n').map(|n| n + 1).unwrap_or(i);
    Ok(format!("{}{}{}", &text[..i], block, &text[i..]))
}

fn write_if_changed(path: &Path, text: &str) -> Result<(), String> {
    if std::fs::read_to_string(path).ok().as_deref() == Some(text) {
        return Ok(());
    }
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Erzeugt Missionen, Kampagnenplan, Manifest-Events und Sprechskript.
pub fn forge_campaign(assets: &Path, repo: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let data = GameData::load_dir(&assets.join("data")).map_err(|e| e.to_string())?;
    let mut script = Script::default();
    let missions = crate::campaign_missions::all(&data, &mut script);
    let dir = assets.join("data/campaign");
    let mut out = Vec::new();
    for m in &missions {
        let path = dir.join(format!("{}.ron", m.id));
        let tmp = ashen_sim::scenario::save(m, &path);
        tmp?;
        out.push(path);
    }
    let plan = crate::campaign_missions::plan(&mut script);
    let plan_text = plan.to_ron()?;
    let plan_path = dir.join("campaign.ron");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    write_if_changed(&plan_path, &format!("// ASHEN CROWNS – Kampagnenplan „Das Zeitalter der Gefallenen“ (erzeugt von `ashen_tools forge-campaign`).\n{plan_text}\n"))?;
    out.push(plan_path);
    let mut errs = plan.validate(&missions);
    // Plan und Missionen müssen zusammenpassen; Mission 1 liegt unter missions/.
    errs.retain(|e| !e.contains("m01_last_settlement hat keine Datei"));
    if !errs.is_empty() {
        return Err(errs.join("; "));
    }

    let manifest_path = assets.join("audio/manifest.ron");
    let manifest = std::fs::read_to_string(&manifest_path).map_err(|e| format!("{}: {e}", manifest_path.display()))?;
    let anchor = "    ],\n    ambience: [";
    let manifest = splice(&manifest, BEGIN_MANIFEST, END_MANIFEST, &manifest_block(&script), anchor)?;
    write_if_changed(&manifest_path, &manifest)?;

    let doc_path = repo.join("docs/audio/elevenlabs_dialogues.md");
    let doc = std::fs::read_to_string(&doc_path).map_err(|e| format!("{}: {e}", doc_path.display()))?;
    let tagged = existing_tagged_texts(&doc);
    let doc = splice(&doc, BEGIN_DOC, END_DOC, &format!("{}\n---\n\n", doc_block(&script, &tagged)), "## Anhang – Produktionshinweise Dialog")?;
    write_if_changed(&doc_path, &doc)?;
    println!("Kampagne: {} Missionen, {} Sprechzeilen", missions.len(), script.lines.len());
    Ok(out)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn chapter(id: &str, numeral: &str, title: &str, subtitle: &str, years: &str, blurb: &str, voice: Option<String>, missions: Vec<CampaignMission>) -> Chapter {
    Chapter { id: id.into(), numeral: numeral.into(), title: title.into(), subtitle: subtitle.into(), years: years.into(), blurb: blurb.into(), voice, missions }
}

pub(crate) fn cm(id: &str, needs: &[&str], pos: (f32, f32)) -> CampaignMission {
    CampaignMission { id: id.into(), needs: needs.iter().map(|s| s.to_string()).collect(), if_flags: vec![], unless_flags: vec![], finale: false, pos }
}

pub(crate) fn ending(id: &str, title: &str, text: &str, requires: &[&str], voice: Option<String>) -> Ending {
    Ending { id: id.into(), title: title.into(), text: text.into(), requires: requires.iter().map(|s| s.to_string()).collect(), voice }
}

pub(crate) type Plan = CampaignDef;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_audio_tags_and_normalizes_whitespace() {
        assert_eq!(strip_audio_tags("[solemn] Jahr 897. [pause] Am Rand – [short pause] der Mark."), "Jahr 897. Am Rand – der Mark.");
        assert_eq!(strip_audio_tags("Nein … nicht so – [exhales]"), "Nein … nicht so –");
        assert_eq!(strip_audio_tags("Ohne Tags  bleibt gleich."), "Ohne Tags bleibt gleich.");
        assert_eq!(strip_audio_tags("[Option A] bleibt Text, [offen"), "[Option A] bleibt Text, [offen");
    }

    #[test]
    fn keeps_hand_written_tags_only_when_text_matches() {
        let doc = format!("{BEGIN_DOC}\n| 1 | `voices/narrator/narrator_m02_x_a_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Hallo Welt. | ruhig |\n| 2 | `voices/narrator/narrator_m02_x_b_01.ogg` | Erzähler (`AC_narrator`) | [solemn] Alter Text. | ruhig |\n{END_DOC}\n");
        let map = existing_tagged_texts(&doc);
        assert_eq!(map.len(), 2);
        assert_eq!(map["voices/narrator/narrator_m02_x_a_01.ogg"], "[solemn] Hallo Welt.");
        let plain = "Hallo Welt.";
        let kept = map.get("voices/narrator/narrator_m02_x_a_01.ogg").filter(|t| strip_audio_tags(t) == strip_audio_tags(plain));
        assert!(kept.is_some());
        let changed = map.get("voices/narrator/narrator_m02_x_b_01.ogg").filter(|t| strip_audio_tags(t) == strip_audio_tags("Neuer Text."));
        assert!(changed.is_none());
    }
}
