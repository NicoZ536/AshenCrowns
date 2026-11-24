//! `audio-list`: vollständige Liste aller vom Audio-Manifest erwarteten Dateien.
//!
//! Das Manifest kennt Varianten nur als Muster (`dir/stem_*`). Die tatsächliche Variantenzahl steht in den
//! Prompt-Dokumenten (`docs/audio/elevenlabs_*.md`, `suno_*.md`): jede Zeile mit Ziel-Datei dort ist ein
//! produzierbarer Prompt. Die Liste verbindet beides: Muster → exakte Dateinamen (`stem_01.ogg`, …) und prüft
//! gleichzeitig, dass jedes Muster einen Prompt besitzt.

use ashen_engine::audio::manifest::{AudioManifest, Category, Playback};
use ashen_engine::audio::scanner;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Anzahl Varianten, wenn ein Muster keinen Prompt hat (nur zur Anzeige; der Eintrag wird als „ohne Prompt“ markiert).
const DEFAULT_VARIANTS: u32 = 3;

/// Dateiendung der Ziel-Dateien in der Liste.
const EXT: &str = "ogg";

const ROOTS: [&str; 4] = ["music/", "sfx/", "voices/", "ambience/"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Markdown,
    Csv,
}

#[derive(Clone, Debug)]
pub struct Entry {
    /// Relativer Pfad zu `assets/audio` ohne Endung, z. B. `sfx/combat/sword_hit_01`.
    pub rel: String,
    pub category: &'static str,
    /// Event-, Track- oder Ambience-ID.
    pub id: String,
    /// Dauer-, Loop- oder Sequenzhinweis (leer, wenn nicht ableitbar).
    pub hint: String,
    /// Ob die Prompt-Dokumente für diese Datei (bzw. dieses Muster) einen Prompt enthalten.
    pub has_prompt: bool,
}

impl Entry {
    pub fn folder(&self) -> &str {
        self.rel.rsplit_once('/').map(|(d, _)| d).unwrap_or("")
    }

    pub fn file_name(&self) -> String {
        let stem = self.rel.rsplit_once('/').map(|(_, s)| s).unwrap_or(&self.rel);
        format!("{stem}.{EXT}")
    }

    #[cfg(test)]
    pub fn path(&self) -> String {
        format!("{}.{EXT}", self.rel)
    }
}

#[derive(Clone, Debug, Default)]
struct DocEntry {
    duration: Option<String>,
}

/// Aus den Prompt-Dokumenten gelesene Ziel-Dateien (relativer Pfad ohne Endung → Angaben).
#[derive(Clone, Debug, Default)]
pub struct DocPrompts {
    files: BTreeMap<String, DocEntry>,
    pub docs_found: bool,
}

/// Ziel-Dateipfad (ohne Endung) aus einem Backtick-Ausdruck wie `` `voices/units/x_01.ogg` `` oder
/// `` `assets/audio/music/x.ogg` ``. Bereichsschreibweisen mit „…“ zählen nicht.
fn path_of(code: &str) -> Option<String> {
    let c = code.strip_prefix("assets/audio/").unwrap_or(code);
    let stem = c.strip_suffix(".ogg").or_else(|| c.strip_suffix(".wav"))?;
    if !ROOTS.iter().any(|r| stem.starts_with(r)) || !stem.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '/') {
        return None;
    }
    Some(stem.to_string())
}

/// Alle Backtick-Ausdrücke einer Zeile.
fn code_spans(line: &str) -> Vec<&str> {
    line.split('`').skip(1).step_by(2).collect()
}

/// `0,6 s`, `8,0 s (Loop)`, `30 s` → wahr.
fn is_duration(cell: &str) -> bool {
    let digits: String = cell.chars().take_while(|c| c.is_ascii_digit() || *c == ',' || *c == '.').collect();
    !digits.is_empty() && digits.chars().next().is_some_and(|c| c.is_ascii_digit()) && cell[digits.len()..].trim_start().starts_with('s')
}

impl DocPrompts {
    /// Liest `elevenlabs_*.md` und `suno_*.md` aus `docs_dir` (Leitfaden mit Beispielzeilen und das Master-Dokument zählen nicht).
    pub fn load(docs_dir: &Path) -> DocPrompts {
        let mut out = DocPrompts::default();
        let Ok(rd) = std::fs::read_dir(docs_dir) else { return out };
        let mut names: Vec<_> = rd.flatten().map(|e| e.path()).filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(".md") && (n.starts_with("elevenlabs_") || n.starts_with("suno_")) && !n.contains("production_guide"))).collect();
        names.sort();
        for p in names {
            if let Ok(text) = std::fs::read_to_string(&p) {
                out.docs_found = true;
                out.add_text(&text);
            }
        }
        out
    }

    pub fn add_text(&mut self, text: &str) {
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let t = line.trim_start();
            if t.starts_with('|') {
                let cells: Vec<&str> = t.split('|').map(str::trim).collect();
                // Tabellenzeile: Ziel-Datei steht in einer der ersten Zellen (Spalte „#“ davor möglich).
                let key_cells = cells.iter().skip(1).take(3);
                let is_id_row = cells.get(1).is_some_and(|c| c.starts_with("ID / Ziel-Datei"));
                if is_id_row {
                    for span in code_spans(line) {
                        if let Some(p) = path_of(span) {
                            self.files.entry(p).or_default();
                        }
                    }
                    continue;
                }
                for cell in key_cells {
                    if cell.len() > 2 && cell.starts_with('`') && cell.ends_with('`') && let Some(p) = path_of(&cell[1..cell.len() - 1]) {
                        let duration = cells.iter().find(|c| is_duration(c)).map(|c| c.to_string());
                        let e = self.files.entry(p).or_default();
                        if e.duration.is_none() {
                            e.duration = duration;
                        }
                        break;
                    }
                }
            } else if t.starts_with('#') {
                for span in code_spans(line) {
                    if let Some(p) = path_of(span) {
                        // Ambience-Abschnitt: „**Länge:** 90 s.“ in den folgenden Zeilen.
                        let dur = lines.iter().skip(i + 1).take(4).find_map(|l| l.split_once("**Länge:**").map(|(_, r)| r.trim().trim_end_matches('.').to_string()));
                        let e = self.files.entry(p).or_default();
                        if e.duration.is_none() {
                            e.duration = dur;
                        }
                    }
                }
            }
        }
    }

    fn contains(&self, rel: &str) -> bool {
        self.files.contains_key(rel)
    }

    fn duration(&self, rel: &str) -> Option<&str> {
        self.files.get(rel).and_then(|e| e.duration.as_deref())
    }

    /// Variantennummern, für die ein Prompt existiert (`base_NN`), aufsteigend.
    fn variants(&self, base: &str) -> Vec<u32> {
        let prefix = format!("{base}_");
        self.files
            .range(prefix.clone()..)
            .take_while(|(k, _)| k.starts_with(&prefix))
            .filter_map(|(k, _)| {
                let rest = &k[prefix.len()..];
                if rest.len() == 2 && rest.chars().all(|c| c.is_ascii_digit()) { rest.parse().ok() } else { None }
            })
            .collect()
    }
}

fn join_hint(parts: &[String]) -> String {
    parts.iter().filter(|p| !p.is_empty()).cloned().collect::<Vec<_>>().join("; ")
}

/// Baut die sortierte Liste aller erwarteten Dateien.
pub fn build(manifest: &AudioManifest, prompts: &DocPrompts) -> Vec<Entry> {
    let mut map: BTreeMap<String, Entry> = BTreeMap::new();
    let mut put = |e: Entry| {
        map.entry(e.rel.clone()).or_insert(e);
    };
    for m in &manifest.music {
        let loop_hint = if m.looped { "komplette Suno-Ausgabe, wird als Ganzes geloopt (Naht und Pegel: Integration)" } else { "komplette Suno-Ausgabe, einmalig, kein Loop" };
        put(Entry { rel: m.file.clone(), category: "music", id: m.id.clone(), hint: loop_hint.into(), has_prompt: prompts.contains(&m.file) });
        if let Some(i) = &m.intro {
            put(Entry { rel: i.clone(), category: "music_intro", id: m.id.clone(), hint: "Intro, einmalig (derzeit von keinem Manifest-Track genutzt)".into(), has_prompt: prompts.contains(i) });
        }
    }
    for a in &manifest.ambience {
        let hint = join_hint(&["eine komplette Generierung, Loop-Option an".into(), prompts.duration(&a.file).map(|d| format!("Länge {d}")).unwrap_or_default()]);
        put(Entry { rel: a.file.clone(), category: "ambience", id: a.id.clone(), hint, has_prompt: prompts.contains(&a.file) });
    }
    for e in &manifest.events {
        let category = match e.category {
            Category::Voice => "voice",
            Category::Narration => "narration",
            Category::Dialogue => "dialogue",
            Category::Sfx => "sfx",
            Category::Ui => "ui",
        };
        for pat in &e.files {
            let (bases, nums, found): (String, Vec<u32>, bool) = match pat.strip_suffix("_*") {
                Some(base) => {
                    let v = prompts.variants(base);
                    if v.is_empty() { (base.into(), (1..=DEFAULT_VARIANTS).collect(), false) } else { (base.into(), v, true) }
                }
                None => (pat.clone(), vec![0], prompts.contains(pat)),
            };
            let single = !pat.ends_with("_*");
            let total = nums.len();
            for (idx, n) in nums.iter().enumerate() {
                let rel = if single { bases.clone() } else { format!("{bases}_{n:02}") };
                let seq = if e.playback == Playback::Sequential { format!("Sequenz Teil {}/{}", idx + 1, total) } else { String::new() };
                let dur = prompts.duration(&rel).map(|d| format!("Dauer {d}")).unwrap_or_default();
                put(Entry { rel, category, id: e.id.clone(), hint: join_hint(&[dur, seq]), has_prompt: found });
            }
        }
    }
    map.into_values().collect()
}

/// Zählt Einträge je Kategorie (sortiert nach Kategoriename).
pub fn counts(entries: &[Entry]) -> BTreeMap<&'static str, usize> {
    let mut c = BTreeMap::new();
    for e in entries {
        *c.entry(e.category).or_insert(0) += 1;
    }
    c
}

fn csv_cell(s: &str) -> String {
    if s.contains([',', '"', '\n']) { format!("\"{}\"", s.replace('"', "\"\"")) } else { s.to_string() }
}

pub fn render(entries: &[Entry], format: Format) -> String {
    let mut s = String::new();
    match format {
        Format::Csv => {
            s.push_str("folder,file,category,id,hint,prompt\n");
            for e in entries {
                s.push_str(&format!("{},{},{},{},{},{}\n", csv_cell(e.folder()), csv_cell(&e.file_name()), e.category, csv_cell(&e.id), csv_cell(&e.hint), if e.has_prompt { "ja" } else { "FEHLT" }));
            }
        }
        Format::Markdown => {
            s.push_str("# Erwartete Audiodateien (audio-list)\n\n");
            s.push_str(&format!("Dateien gesamt: **{}**  \n", entries.len()));
            let c = counts(entries);
            s.push_str(&format!("Je Kategorie: {}\n\n", c.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(" · ")));
            s.push_str("| Ordner | Datei | Kategorie | ID | Hinweis |\n|---|---|---|---|---|\n");
            for e in entries {
                let hint = if e.has_prompt { e.hint.clone() } else { join_hint(&[e.hint.clone(), "PROMPT FEHLT".into()]) };
                s.push_str(&format!("| `{}` | `{}` | {} | `{}` | {} |\n", e.folder(), e.file_name(), e.category, e.id, hint));
            }
        }
    }
    s
}

/// Ergebnis von `audio-scan --import`.
#[derive(Debug, Default)]
pub struct ImportResult {
    /// (Quelle, Ziel)
    pub copied: Vec<(PathBuf, PathBuf)>,
    /// Quellen, deren Ziel (oder eine gleichnamige Datei mit anderer Endung) schon existiert.
    pub existing: Vec<PathBuf>,
    /// Audiodateien, deren Name zu keiner erwarteten Datei passt.
    pub unmatched: Vec<PathBuf>,
}

fn walk_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() { walk_files(&p, out) } else { out.push(p) }
    }
}

/// Kopiert Audiodateien aus einem beliebig strukturierten Lieferordner anhand des (normalisierten) Dateinamens in den
/// richtigen Zielordner unter `assets/audio`. Bestehende Dateien werden nie überschrieben, Originale bleiben liegen.
pub fn import_delivery(assets: &Path, from: &Path) -> Result<ImportResult, String> {
    let manifest = AudioManifest::load(&assets.join("audio/manifest.ron"))?;
    let prompts = assets.parent().map(|p| DocPrompts::load(&p.join("docs/audio"))).unwrap_or_default();
    let entries = build(&manifest, &prompts);
    let mut by_name: BTreeMap<String, Option<String>> = BTreeMap::new();
    for e in &entries {
        let stem = e.rel.rsplit_once('/').map(|(_, s)| s).unwrap_or(&e.rel).to_string();
        // Doppelte Dateinamen in verschiedenen Ordnern sind nicht eindeutig zuordenbar.
        by_name.entry(stem).and_modify(|v| *v = None).or_insert_with(|| Some(e.rel.clone()));
    }
    let mut files = Vec::new();
    walk_files(from, &mut files);
    files.sort();
    let root = assets.join("audio");
    let mut res = ImportResult::default();
    for f in files {
        let ext = f.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
        if !scanner::PLAYABLE.contains(&ext.as_str()) && !scanner::CONVERTIBLE.contains(&ext.as_str()) {
            continue;
        }
        let name = f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let norm = scanner::normalize_name(&name);
        let stem = norm.rsplit_once('.').map(|(s, _)| s).unwrap_or(&norm).to_string();
        let Some(Some(rel)) = by_name.get(&stem) else {
            res.unmatched.push(f);
            continue;
        };
        let dest = root.join(format!("{rel}.{ext}"));
        let sibling_exists = dest
            .parent()
            .and_then(|d| std::fs::read_dir(d).ok())
            .is_some_and(|rd| rd.flatten().any(|e| e.path().file_stem().is_some_and(|s| s.to_string_lossy() == stem.as_str())));
        if sibling_exists {
            res.existing.push(f);
            continue;
        }
        if let Some(d) = dest.parent() {
            std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
        }
        std::fs::copy(&f, &dest).map_err(|e| format!("{} → {}: {e}", f.display(), dest.display()))?;
        res.copied.push((f, dest));
    }
    Ok(res)
}

/// Befehl `audio-list`: schreibt die Liste (stdout oder `out`) und meldet Muster ohne Prompt als Fehler.
pub fn run(assets: &Path, format: Format, out: Option<&Path>) -> Result<(), String> {
    let manifest = AudioManifest::load(&assets.join("audio/manifest.ron"))?;
    let docs_dir = assets.parent().map(|p| p.join("docs/audio"));
    let prompts = docs_dir.as_deref().map(DocPrompts::load).unwrap_or_default();
    let entries = build(&manifest, &prompts);
    let text = render(&entries, format);
    match out {
        Some(p) => {
            std::fs::write(p, &text).map_err(|e| format!("{}: {e}", p.display()))?;
            let c = counts(&entries);
            eprintln!("{} Dateien ({}) → {}", entries.len(), c.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "), p.display());
        }
        None => print!("{text}"),
    }
    if !prompts.docs_found {
        eprintln!("Hinweis: Prompt-Dokumente nicht gefunden ({}); Variantenzahl {DEFAULT_VARIANTS} angenommen.", docs_dir.map(|d| d.display().to_string()).unwrap_or_default());
        return Ok(());
    }
    let missing: Vec<&Entry> = entries.iter().filter(|e| !e.has_prompt).collect();
    if missing.is_empty() {
        Ok(())
    } else {
        let mut ids: Vec<String> = missing.iter().map(|e| e.id.clone()).collect();
        ids.dedup();
        Err(format!("{} Datei(en) ohne Prompt in docs/audio: {}", missing.len(), ids.join(", ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ashen_engine::audio::manifest::{AmbienceLayer, Intensity, MusicTrack, SoundEvent};
    use ashen_engine::audio::scanner;

    fn repo_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    fn mini_manifest() -> AudioManifest {
        AudioManifest {
            version: 1,
            music: vec![MusicTrack { id: "t".into(), intensity: Intensity::Calm, file: "music/t_loop".into(), intro: Some("music/t_intro".into()), looped: true }],
            events: vec![
                SoundEvent { id: "voice.a.move".into(), category: Category::Voice, files: vec!["voices/units/a_move_*".into()], volume_db: 0.0, cooldown_ms: 0, max_instances: 1, playback: Playback::Sequential },
                SoundEvent { id: "sfx.b".into(), category: Category::Sfx, files: vec!["sfx/combat/b_*".into()], volume_db: 0.0, cooldown_ms: 0, max_instances: 1, playback: Playback::Random },
            ],
            ambience: vec![AmbienceLayer { id: "amb".into(), file: "ambience/amb_loop".into(), volume_db: 0.0 }],
        }
    }

    #[test]
    fn expands_variants_from_prompt_docs() {
        let mut p = DocPrompts::default();
        p.add_text(
            "| Ziel-Datei | Text |\n|---|---|\n| `voices/units/a_move_01.ogg` | x |\n| `voices/units/a_move_02.ogg` | y |\n\
             | `sfx/combat/b_01.ogg` | prompt | 0,6 s | 0,5 |\n| `sfx/combat/b_03.ogg` | prompt | 0,7 s | 0,5 |\n\
             | `sfx/combat/c_01…03.ogg` | Bereich zählt nicht |\n\
             ### 1.1 `ambience/amb_loop.ogg` – Test\n*Charakter:* x\n**Länge:** 90 s.\n\
             | ID / Ziel-Dateien | `t` → `assets/audio/music/t_intro.ogg`, `assets/audio/music/t_loop.ogg` |\n",
        );
        let list = build(&mini_manifest(), &p);
        let names: Vec<String> = list.iter().map(Entry::path).collect();
        assert_eq!(names, ["ambience/amb_loop.ogg", "music/t_intro.ogg", "music/t_loop.ogg", "sfx/combat/b_01.ogg", "sfx/combat/b_03.ogg", "voices/units/a_move_01.ogg", "voices/units/a_move_02.ogg"]);
        assert!(list.iter().all(|e| e.has_prompt), "{list:?}");
        let amb = &list[0];
        assert!(amb.hint.contains("90 s"));
        assert_eq!(list[3].hint, "Dauer 0,6 s");
        assert_eq!(list[6].hint, "Sequenz Teil 2/2");
        assert_eq!(list[0].folder(), "ambience");
        assert_eq!(list[5].file_name(), "a_move_01.ogg");
    }

    #[test]
    fn missing_prompt_is_flagged() {
        let list = build(&mini_manifest(), &DocPrompts::default());
        assert_eq!(list.iter().filter(|e| e.id == "sfx.b").count(), 3);
        assert!(list.iter().all(|e| !e.has_prompt));
        let md = render(&list, Format::Markdown);
        assert!(md.contains("PROMPT FEHLT"));
        let csv = render(&list, Format::Csv);
        assert!(csv.starts_with("folder,file,category,id,hint,prompt\n"));
        assert!(csv.contains("sfx/combat,b_01.ogg,sfx,sfx.b,,FEHLT"));
    }

    #[test]
    fn csv_quotes_special_characters() {
        assert_eq!(csv_cell("a,b"), "\"a,b\"");
        assert_eq!(csv_cell("a\"b"), "\"a\"\"b\"");
        assert_eq!(csv_cell("plain"), "plain");
    }

    /// Echtes Manifest + echte Prompt-Dokumente: jede Datei hat einen Prompt, die Liste ist sortiert und eindeutig,
    /// und der Scanner ordnet nach Anlegen aller Dateien jedes Muster ohne Lücke, Fremddatei oder Mehrdeutigkeit zu.
    #[test]
    fn real_manifest_is_fully_listed_and_scannable() {
        let root = repo_root();
        let manifest = AudioManifest::load(&root.join("assets/audio/manifest.ron")).unwrap();
        let prompts = DocPrompts::load(&root.join("docs/audio"));
        assert!(prompts.docs_found);
        let list = build(&manifest, &prompts);
        let missing: Vec<String> = list.iter().filter(|e| !e.has_prompt).map(|e| e.id.clone()).collect();
        assert!(missing.is_empty(), "Muster ohne Prompt: {missing:?}");
        let mut sorted: Vec<String> = list.iter().map(|e| format!("{}/{}", e.folder(), e.file_name())).collect();
        let before = sorted.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(before, sorted, "Liste muss sortiert und eindeutig sein");
        let mut stems: Vec<&str> = list.iter().map(|e| e.rel.rsplit('/').next().unwrap()).collect();
        stems.sort_unstable();
        let n = stems.len();
        stems.dedup();
        assert_eq!(stems.len(), n, "Dateinamen müssen ordnerübergreifend eindeutig sein (Lieferordner-Import)");
        for e in &list {
            assert!(scanner::valid_name(Path::new(&e.path())), "ungültiger Name {}", e.path());
        }

        let dir = std::env::temp_dir().join(format!("ashen_audio_list_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for e in &list {
            let p = dir.join(e.path());
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            // Eindeutiger Inhalt je Datei (sonst meldet der Scanner inhaltsgleiche Duplikate).
            std::fs::write(&p, format!("OggS\0\x02{}", e.rel).as_bytes()).unwrap();
        }
        let r = scanner::scan(&dir, &manifest);
        let _ = std::fs::remove_dir_all(&dir);
        assert!(r.missing.is_empty(), "fehlend: {:?}", r.missing);
        assert!(r.unknown.is_empty(), "unbekannt: {:?}", r.unknown);
        assert!(r.variant_gaps.is_empty(), "Lücken: {:?}", r.variant_gaps);
        assert!(r.ambiguous.is_empty() && r.duplicates.is_empty() && r.invalid_names.is_empty() && r.corrupt.is_empty());
        assert_eq!(r.found_count(), list.len());
    }

    #[test]
    fn import_sorts_flat_delivery_into_folders() {
        let root = repo_root();
        let tmp = std::env::temp_dir().join(format!("ashen_import_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        let assets = tmp.join("assets");
        std::fs::create_dir_all(assets.join("audio")).unwrap();
        std::fs::copy(root.join("assets/audio/manifest.ron"), assets.join("audio/manifest.ron")).unwrap();
        std::fs::create_dir_all(tmp.join("docs")).unwrap();
        // Prompt-Dokumente für die Variantenzahl bereitstellen.
        let docs_dst = tmp.join("docs/audio");
        std::fs::create_dir_all(&docs_dst).unwrap();
        for e in std::fs::read_dir(root.join("docs/audio")).unwrap().flatten() {
            if e.path().extension().is_some_and(|x| x == "md") {
                std::fs::copy(e.path(), docs_dst.join(e.file_name())).unwrap();
            }
        }
        let from = tmp.join("lieferung/unterordner");
        std::fs::create_dir_all(&from).unwrap();
        std::fs::write(from.join("Sword Hit 1.WAV"), b"RIFF\0\0\0\0WAVEdata").unwrap();
        std::fs::write(from.join("main_menu.m4a"), b"m4a").unwrap();
        std::fs::write(from.join("irgendwas_01.ogg"), b"OggS\0\x02").unwrap();
        std::fs::write(from.join("notes.txt"), b"x").unwrap();
        let r = import_delivery(&assets, &tmp.join("lieferung")).unwrap();
        assert_eq!(r.copied.len(), 2, "{r:?}");
        assert!(assets.join("audio/sfx/combat/sword_hit_01.wav").exists());
        assert!(assets.join("audio/music/main_menu.m4a").exists());
        assert_eq!(r.unmatched.len(), 1);
        // Zweiter Lauf überschreibt nichts.
        let r2 = import_delivery(&assets, &tmp.join("lieferung")).unwrap();
        assert!(r2.copied.is_empty());
        assert_eq!(r2.existing.len(), 2);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// Jede erwartete Datei steht im Master-Dokument `docs/audio/PRODUCTION_CHECKLIST.md`.
    #[test]
    fn every_expected_file_is_in_production_checklist() {
        let root = repo_root();
        let manifest = AudioManifest::load(&root.join("assets/audio/manifest.ron")).unwrap();
        let prompts = DocPrompts::load(&root.join("docs/audio"));
        let list = build(&manifest, &prompts);
        let doc = std::fs::read_to_string(root.join("docs/audio/PRODUCTION_CHECKLIST.md")).expect("PRODUCTION_CHECKLIST.md fehlt");
        let missing: Vec<String> = list.iter().map(Entry::path).filter(|p| !doc.contains(&format!("`{p}`"))).collect();
        assert!(missing.is_empty(), "{} Datei(en) fehlen in der Checkliste, z. B. {:?}", missing.len(), &missing[..missing.len().min(10)]);
    }

    /// `[solemn]`, `[short pause]`: mindestens ein Audio-Tag (englisch, Kleinbuchstaben) im Text.
    fn has_audio_tag(s: &str) -> bool {
        let mut rest = s;
        while let Some(i) = rest.find('[') {
            let after = &rest[i + 1..];
            let Some(j) = after.find(']') else { return false };
            let tag = &after[..j];
            if !tag.is_empty() && tag.len() <= 40 && tag.chars().all(|c| c.is_ascii_lowercase() || c == ' ' || c == '-') && tag.chars().any(|c| c.is_ascii_lowercase()) {
                return true;
            }
            rest = &after[j + 1..];
        }
        false
    }

    #[test]
    fn audio_tag_detection() {
        assert!(has_audio_tag("[solemn] Jahr 897."));
        assert!(has_audio_tag("Nein … [exhales]"));
        assert!(has_audio_tag("[short pause] x"));
        assert!(!has_audio_tag("Jahr 897 nach dem Fall."));
        assert!(!has_audio_tag("[] leer"));
        assert!(!has_audio_tag("[Option A] Großschreibung ist kein Tag"));
        assert!(!has_audio_tag("offen [solemn"));
    }

    /// Jede Sprechzeile (Erzähler, Figuren, Bosse, Helden, Einheiten) steht in der Checkliste als einfügefertiger
    /// ```text-Block, und dieser enthält mindestens ein Eleven-v3-Audio-Tag in eckigen Klammern.
    #[test]
    fn every_spoken_line_in_checklist_has_audio_tag() {
        let root = repo_root();
        let manifest = AudioManifest::load(&root.join("assets/audio/manifest.ron")).unwrap();
        let prompts = DocPrompts::load(&root.join("docs/audio"));
        let list = build(&manifest, &prompts);
        let doc = std::fs::read_to_string(root.join("docs/audio/PRODUCTION_CHECKLIST.md")).expect("PRODUCTION_CHECKLIST.md fehlt");
        let lines: Vec<&str> = doc.lines().collect();
        let mut problems = Vec::new();
        let mut checked = 0;
        for e in list.iter().filter(|e| matches!(e.category, "voice" | "narration" | "dialogue") && !e.id.starts_with("voice.bat_swarm")) {
            let needle = format!("`{}`", e.path());
            let Some(i) = lines.iter().position(|l| l.starts_with("- [") && l.contains(&needle)) else {
                problems.push(format!("{}: kein Eintrag", e.path()));
                continue;
            };
            let Some(start) = lines.iter().skip(i + 1).take(3).position(|l| l.trim() == "```text").map(|p| i + 1 + p) else {
                problems.push(format!("{}: kein ```text-Block", e.path()));
                continue;
            };
            let body: Vec<&str> = lines.iter().skip(start + 1).take_while(|l| l.trim() != "```").copied().collect();
            let text = body.join(" ");
            checked += 1;
            if text.trim().is_empty() {
                problems.push(format!("{}: leerer Sprechtext", e.path()));
            } else if !has_audio_tag(&text) {
                problems.push(format!("{}: kein [Audio-Tag]", e.path()));
            }
        }
        assert!(checked > 0, "keine Sprechzeilen geprüft");
        assert!(problems.is_empty(), "{} Problem(e), z. B. {:?}", problems.len(), &problems[..problems.len().min(10)]);
    }

    /// Musik: pro Track genau eine Datei `music/<id>` (komplette Suno-Ausgabe), kein Intro; Erzähler-/Dialogzeilen: je Event eine Datei.
    #[test]
    fn music_and_spoken_lines_are_single_files() {
        let root = repo_root();
        let manifest = AudioManifest::load(&root.join("assets/audio/manifest.ron")).unwrap();
        let prompts = DocPrompts::load(&root.join("docs/audio"));
        let list = build(&manifest, &prompts);
        assert_eq!(manifest.music.len(), 34);
        for m in &manifest.music {
            assert!(m.intro.is_none(), "{} hat ein Intro", m.id);
            assert_eq!(m.file, format!("music/{}", m.id));
        }
        assert_eq!(list.iter().filter(|e| e.category == "music").count(), 34);
        assert_eq!(list.iter().filter(|e| e.category == "music_intro").count(), 0);
        let mut per_event: BTreeMap<&str, usize> = BTreeMap::new();
        for e in list.iter().filter(|e| matches!(e.category, "narration" | "dialogue")) {
            *per_event.entry(&e.id).or_default() += 1;
        }
        let multi: Vec<_> = per_event.iter().filter(|(_, n)| **n != 1).collect();
        assert!(multi.is_empty(), "Erzähler-/Dialog-Events mit mehreren Dateien: {multi:?}");
        assert!(manifest.events.iter().all(|e| e.playback == Playback::Random), "Sequential wird nicht mehr genutzt");
    }
}
