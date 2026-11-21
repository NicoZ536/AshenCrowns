//! Asset-Scanner: findet Audiodateien, ordnet sie dem Manifest zu und prüft sie.
//!
//! Erkennt fehlende Dateien, Lücken in Variantennummern, ungültige Dateinamen (mit
//! Umbenennungsvorschlag), Duplikate (gleicher Name / gleicher Inhalt), unbekannte Dateien,
//! zu konvertierende Formate und beschädigte Dateien.

use super::manifest::AudioManifest;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

pub const PLAYABLE: [&str; 4] = ["ogg", "wav", "flac", "mp3"];
pub const CONVERTIBLE: [&str; 8] = ["m4a", "aac", "aiff", "aif", "opus", "webm", "wma", "caf"];

/// Zuordnung eines Musters zu gefundenen Dateien.
#[derive(Clone, Debug, Default)]
pub struct Resolved {
    pub owner: String,
    pub files: Vec<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct RenameSuggestion {
    pub from: PathBuf,
    pub to: PathBuf,
    pub reason: String,
}

#[derive(Clone, Debug, Default)]
pub struct ScanReport {
    pub root: PathBuf,
    /// Muster → gefundene Dateien (sortiert nach Variantennummer).
    pub resolved: BTreeMap<String, Resolved>,
    pub missing: Vec<(String, String)>,
    pub variant_gaps: Vec<(String, Vec<u32>)>,
    pub invalid_names: Vec<RenameSuggestion>,
    pub ambiguous: Vec<(PathBuf, Vec<String>)>,
    pub duplicates: Vec<Vec<PathBuf>>,
    pub unknown: Vec<PathBuf>,
    pub needs_conversion: Vec<PathBuf>,
    pub corrupt: Vec<(PathBuf, String)>,
    pub total_files: usize,
}

impl ScanReport {
    /// Dateien eines Musters (leer, wenn keine gefunden).
    pub fn files_for(&self, pattern: &str) -> &[PathBuf] {
        self.resolved.get(pattern).map(|r| r.files.as_slice()).unwrap_or(&[])
    }

    pub fn found_count(&self) -> usize {
        self.resolved.values().map(|r| r.files.len()).sum()
    }

    pub fn is_clean(&self) -> bool {
        self.invalid_names.is_empty() && self.duplicates.is_empty() && self.corrupt.is_empty() && self.needs_conversion.is_empty() && self.ambiguous.is_empty()
    }

    /// Bericht als Markdown.
    pub fn to_markdown(&self) -> String {
        let mut s = String::new();
        s.push_str("# Audio-Asset-Bericht\n\n");
        s.push_str(&format!("Wurzel: `{}`  \nDateien gefunden: {}  \nZugeordnet: {}  \nErwartete Muster: {}  \nFehlende Muster: {}\n\n", self.root.display(), self.total_files, self.found_count(), self.resolved.len() + self.missing.len(), self.missing.len()));
        let section = |s: &mut String, title: &str, items: Vec<String>| {
            s.push_str(&format!("## {title} ({})\n\n", items.len()));
            if items.is_empty() {
                s.push_str("Keine.\n\n");
            } else {
                for i in items {
                    s.push_str(&format!("- {i}\n"));
                }
                s.push('\n');
            }
        };
        section(&mut s, "Beschädigte Dateien", self.corrupt.iter().map(|(p, e)| format!("`{}` – {e}", p.display())).collect());
        section(&mut s, "Ungültige Dateinamen (Vorschlag)", self.invalid_names.iter().map(|r| format!("`{}` → `{}` ({})", r.from.display(), r.to.display(), r.reason)).collect());
        section(&mut s, "Mehrdeutige Zuordnung (bitte klären)", self.ambiguous.iter().map(|(p, c)| format!("`{}` passt zu: {}", p.display(), c.join(", "))).collect());
        section(&mut s, "Duplikate", self.duplicates.iter().map(|g| g.iter().map(|p| format!("`{}`", p.display())).collect::<Vec<_>>().join(" = ")).collect());
        section(&mut s, "Konvertierung nötig", self.needs_conversion.iter().map(|p| format!("`{}`", p.display())).collect());
        section(&mut s, "Lücken in Variantennummern", self.variant_gaps.iter().map(|(p, g)| format!("`{p}` fehlt: {g:?}")).collect());
        section(&mut s, "Unbekannte Dateien (keinem Event zugeordnet)", self.unknown.iter().map(|p| format!("`{}`", p.display())).collect());
        section(&mut s, "Fehlende Dateien", self.missing.iter().map(|(p, o)| format!("`{p}` ({o})")).collect());
        s
    }
}

fn ext_of(p: &Path) -> String {
    p.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default()
}

/// Gültiger Dateiname: Kleinbuchstaben, Ziffern, Unterstriche; Endung klein.
pub fn valid_name(p: &Path) -> bool {
    let stem = p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let ext = p.extension().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    !stem.is_empty() && stem.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') && ext.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}

/// Normalisiert einen Dateinamen auf die Konvention (`Archer Move 1.OGG` → `archer_move_01.ogg`).
pub fn normalize_name(file_name: &str) -> String {
    let (stem, ext) = match file_name.rsplit_once('.') {
        Some((s, e)) => (s, e.to_lowercase()),
        None => (file_name, String::new()),
    };
    let mut out = String::new();
    let mut last_us = false;
    for c in stem.chars() {
        let c = match c {
            'ä' | 'Ä' => 'a',
            'ö' | 'Ö' => 'o',
            'ü' | 'Ü' => 'u',
            'ß' => 's',
            c => c,
        };
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            last_us = false;
        } else if !last_us && !out.is_empty() {
            out.push('_');
            last_us = true;
        }
    }
    let mut out = out.trim_end_matches('_').to_string();
    // Einstellige Endnummer auf zwei Stellen auffüllen.
    if let Some((head, tail)) = out.rsplit_once('_')
        && tail.len() == 1
        && tail.chars().all(|c| c.is_ascii_digit())
    {
        out = format!("{head}_0{tail}");
    }
    if ext.is_empty() { out } else { format!("{out}.{ext}") }
}

/// Variantennummer aus `stem` für das Muster `base_*`.
fn variant_of(stem: &str, base: &str) -> Option<u32> {
    let rest = stem.strip_prefix(base)?.strip_prefix('_')?;
    if rest.len() == 2 && rest.chars().all(|c| c.is_ascii_digit()) { rest.parse().ok() } else { None }
}

/// Prüft die Kopfbytes auf ein bekanntes Audioformat.
pub fn check_magic(path: &Path) -> Result<(), String> {
    let mut buf = [0u8; 12];
    let n = {
        use std::io::Read;
        let mut f = std::fs::File::open(path).map_err(|e| e.to_string())?;
        f.read(&mut buf).map_err(|e| e.to_string())?
    };
    if n < 4 {
        return Err("Datei leer oder zu klein".into());
    }
    let ok = match ext_of(path).as_str() {
        "ogg" | "opus" => &buf[..4] == b"OggS",
        "wav" => &buf[..4] == b"RIFF" && n >= 12 && &buf[8..12] == b"WAVE",
        "flac" => &buf[..4] == b"fLaC",
        "mp3" => &buf[..3] == b"ID3" || (buf[0] == 0xFF && buf[1] & 0xE0 == 0xE0),
        _ => true,
    };
    if ok { Ok(()) } else { Err("Dateiinhalt passt nicht zur Endung".into()) }
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else {
            out.push(p);
        }
    }
}

/// Durchsucht `root` (assets/audio) und gleicht mit dem Manifest ab.
pub fn scan(root: &Path, manifest: &AudioManifest) -> ScanReport {
    let mut report = ScanReport { root: root.to_path_buf(), ..Default::default() };
    let mut files = Vec::new();
    walk(root, &mut files);
    files.retain(|p| {
        let e = ext_of(p);
        PLAYABLE.contains(&e.as_str()) || CONVERTIBLE.contains(&e.as_str())
    });
    files.sort();
    report.total_files = files.len();

    // Relativer Pfad ohne Endung → Datei(en).
    let rel_stem = |p: &Path| -> String {
        let rel = p.strip_prefix(root).unwrap_or(p);
        let parent = rel.parent().map(|x| x.to_string_lossy().replace('\\', "/")).unwrap_or_default();
        let stem = rel.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        if parent.is_empty() { stem } else { format!("{parent}/{stem}") }
    };

    let patterns = manifest.patterns();
    let mut matched: HashMap<PathBuf, Vec<String>> = HashMap::new();

    for p in &files {
        let ext = ext_of(p);
        if CONVERTIBLE.contains(&ext.as_str()) {
            report.needs_conversion.push(p.clone());
        }
        if !valid_name(p) {
            let name = p.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
            let to = p.with_file_name(normalize_name(&name));
            report.invalid_names.push(RenameSuggestion { from: p.clone(), to, reason: "Konvention: klein, snake_case, zweistellige Variantennummer".into() });
        } else if PLAYABLE.contains(&ext.as_str())
            && let Err(e) = check_magic(p)
        {
            report.corrupt.push((p.clone(), e));
        }
    }

    for (pattern, owner) in &patterns {
        let mut entry = Resolved { owner: owner.clone(), files: Vec::new() };
        let mut variants = Vec::new();
        if let Some(base) = pattern.strip_suffix("_*") {
            for p in &files {
                if !valid_name(p) || !PLAYABLE.contains(&ext_of(p).as_str()) {
                    continue;
                }
                let rs = rel_stem(p);
                let (dir_part, stem_part) = match rs.rsplit_once('/') {
                    Some((d, s)) => (format!("{d}/"), s.to_string()),
                    None => (String::new(), rs.clone()),
                };
                let (bdir, bstem) = match base.rsplit_once('/') {
                    Some((d, s)) => (format!("{d}/"), s.to_string()),
                    None => (String::new(), base.to_string()),
                };
                if dir_part == bdir
                    && let Some(n) = variant_of(&stem_part, &bstem)
                {
                    entry.files.push(p.clone());
                    variants.push(n);
                    matched.entry(p.clone()).or_default().push(pattern.clone());
                }
            }
            variants.sort_unstable();
            if let Some(&max) = variants.last() {
                let gaps: Vec<u32> = (1..=max).filter(|n| !variants.contains(n)).collect();
                if !gaps.is_empty() {
                    report.variant_gaps.push((pattern.clone(), gaps));
                }
            }
            entry.files.sort();
        } else {
            for p in &files {
                if valid_name(p) && PLAYABLE.contains(&ext_of(p).as_str()) && rel_stem(p) == *pattern {
                    entry.files.push(p.clone());
                    matched.entry(p.clone()).or_default().push(pattern.clone());
                }
            }
        }
        // Gleicher Name mit mehreren Endungen → Duplikat (bevorzugt wird .ogg).
        let mut by_stem: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for f in &entry.files {
            by_stem.entry(rel_stem(f)).or_default().push(f.clone());
        }
        for (_, group) in by_stem {
            if group.len() > 1 {
                report.duplicates.push(group.clone());
                let keep = group.iter().find(|p| ext_of(p) == "ogg").cloned().unwrap_or_else(|| group[0].clone());
                entry.files.retain(|f| !group.contains(f) || *f == keep);
            }
        }
        if entry.files.is_empty() {
            report.missing.push((pattern.clone(), owner.clone()));
        } else {
            report.resolved.insert(pattern.clone(), entry);
        }
    }

    for (p, pats) in &matched {
        if pats.len() > 1 {
            report.ambiguous.push((p.clone(), pats.clone()));
        }
    }
    for p in &files {
        if valid_name(p) && PLAYABLE.contains(&ext_of(p).as_str()) && !matched.contains_key(p) {
            report.unknown.push(p.clone());
        }
    }
    // Inhaltsgleiche Dateien.
    let mut by_hash: HashMap<(u64, u32), Vec<PathBuf>> = HashMap::new();
    for p in &files {
        if let Ok(bytes) = std::fs::read(p) {
            by_hash.entry((bytes.len() as u64, crc32(&bytes))).or_default().push(p.clone());
        }
    }
    for (_, g) in by_hash {
        if g.len() > 1 && !report.duplicates.iter().any(|d| d.iter().all(|x| g.contains(x))) {
            report.duplicates.push(g);
        }
    }
    report.ambiguous.sort();
    report
}

/// Kleine CRC32-Implementierung (vermeidet eine zusätzliche Abhängigkeit in der Engine).
fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// Konvertiert eine Datei mit ffmpeg nach Ogg Vorbis (q6, 48 kHz). Gibt den neuen Pfad zurück.
pub fn convert_to_ogg(path: &Path) -> Result<PathBuf, String> {
    let out = path.with_extension("ogg");
    if out.exists() {
        return Err(format!("Ziel existiert bereits: {}", out.display()));
    }
    let status = std::process::Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-i"])
        .arg(path)
        .args(["-c:a", "libvorbis", "-q:a", "6", "-ar", "48000"])
        .arg(&out)
        .status()
        .map_err(|e| format!("ffmpeg nicht ausführbar ({e}). Bitte ffmpeg installieren."))?;
    if !status.success() {
        return Err(format!("ffmpeg meldet Fehler für {}", path.display()));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::manifest::*;

    #[test]
    fn normalizes_names() {
        assert_eq!(normalize_name("Archer Move 1.OGG"), "archer_move_01.ogg");
        assert_eq!(normalize_name("hero-isolde_select_02.wav"), "hero_isolde_select_02.wav");
        assert!(valid_name(Path::new("x/archer_move_01.ogg")));
        assert!(!valid_name(Path::new("x/Archer.ogg")));
    }

    #[test]
    fn scans_directory() {
        let dir = std::env::temp_dir().join(format!("ashen_scan_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("voices/units")).unwrap();
        std::fs::create_dir_all(dir.join("music")).unwrap();
        let ogg = b"OggS\0\x02rest-of-file";
        std::fs::write(dir.join("voices/units/archer_move_01.ogg"), ogg).unwrap();
        std::fs::write(dir.join("voices/units/archer_move_03.ogg"), b"OggS\0\x02other").unwrap();
        std::fs::write(dir.join("voices/units/Archer Move 2.ogg"), b"OggS\0\x02x2").unwrap();
        std::fs::write(dir.join("voices/units/stray_01.ogg"), b"OggS\0\x02stray").unwrap();
        std::fs::write(dir.join("music/build_calm_loop.ogg"), b"NOPEbad").unwrap();
        let m = AudioManifest {
            version: 1,
            music: vec![MusicTrack { id: "build_calm".into(), intensity: Intensity::Calm, file: "music/build_calm_loop".into(), intro: Some("music/build_calm_intro".into()), looped: true }],
            events: vec![SoundEvent { id: "voice.archer.move".into(), category: Category::Voice, files: vec!["voices/units/archer_move_*".into()], volume_db: 0.0, cooldown_ms: 0, max_instances: 1, playback: Playback::Random }],
            ambience: vec![],
        };
        let r = scan(&dir, &m);
        assert_eq!(r.files_for("voices/units/archer_move_*").len(), 2);
        assert_eq!(r.variant_gaps.len(), 1);
        assert_eq!(r.invalid_names.len(), 1);
        assert!(r.invalid_names[0].to.ends_with("archer_move_02.ogg"));
        assert_eq!(r.unknown.len(), 1);
        assert_eq!(r.corrupt.len(), 1);
        assert!(r.missing.iter().any(|(p, _)| p == "music/build_calm_intro"));
        assert!(r.to_markdown().contains("Fehlende Dateien"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
