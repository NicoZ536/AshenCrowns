//! Speichern und Laden der Weltkarte (Format nach dem Muster von `ashen_sim::save`):
//! versionierter Header, CRC32-Prüfsumme, atomares Schreiben, Backup und rotierende Autosaves.
//!
//! Dateiformat (`.acstrat`):
//! ```text
//! "ASTR" | u32 Formatversion | u32 Header-Länge | Header (RON) | u32 CRC32 | u64 Länge | Payload (bincode)
//! ```

use crate::data::StrategyData;
use crate::world::StrategyWorld;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub const MAGIC: &[u8; 4] = b"ASTR";
pub const FORMAT_VERSION: u32 = 1;
pub const EXTENSION: &str = "acstrat";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrategySaveHeader {
    pub format_version: u32,
    pub game_version: String,
    pub id: uuid::Uuid,
    pub name: String,
    pub seed: u64,
    pub year: i32,
    pub day: u32,
    pub player_realm: Option<String>,
    pub difficulty: String,
    pub mode: String,
    pub created_unix: u64,
    /// Prüfsumme der Ereignisliste (Kompatibilität mit Mods).
    pub data_hash: u32,
    pub state_hash: u64,
}

#[derive(Debug)]
pub enum SaveError {
    Io(std::io::Error),
    Format(String),
    Checksum,
    Version(u32),
    Incompatible(String),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Io(e) => write!(f, "Ein-/Ausgabefehler: {e}"),
            SaveError::Format(e) => write!(f, "Spielstand beschädigt: {e}"),
            SaveError::Checksum => write!(f, "Prüfsumme stimmt nicht – Spielstand beschädigt"),
            SaveError::Version(v) => write!(f, "Spielstandformat {v} wird nicht unterstützt (erwartet {FORMAT_VERSION})"),
            SaveError::Incompatible(e) => write!(f, "Spielstand passt nicht zu den geladenen Ereignisdaten/Mods: {e}"),
        }
    }
}

impl std::error::Error for SaveError {}

impl From<std::io::Error> for SaveError {
    fn from(e: std::io::Error) -> Self {
        SaveError::Io(e)
    }
}

fn data_hash(d: &StrategyData) -> u32 {
    let mut h = crc32fast::Hasher::new();
    for e in &d.events {
        h.update(e.id.as_bytes());
        h.update(&[0]);
    }
    h.finalize()
}

fn now_unix() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Serialisiert die Welt in Bytes.
pub fn encode(world: &StrategyWorld, name: &str) -> Result<Vec<u8>, SaveError> {
    let header = StrategySaveHeader {
        format_version: FORMAT_VERSION,
        game_version: env!("CARGO_PKG_VERSION").to_string(),
        id: uuid::Uuid::new_v4(),
        name: name.to_string(),
        seed: world.options.seed,
        year: world.year(),
        day: world.day,
        player_realm: world.player.map(|p| world.rname(p)),
        difficulty: world.options.difficulty.name().to_string(),
        mode: format!("{:?}", world.options.rules.mode),
        created_unix: now_unix(),
        data_hash: data_hash(&world.data),
        state_hash: world.state_hash(),
    };
    let payload = bincode::serde::encode_to_vec(world, bincode::config::standard()).map_err(|e| SaveError::Format(e.to_string()))?;
    let header_text = ron::ser::to_string(&header).map_err(|e| SaveError::Format(e.to_string()))?.into_bytes();
    let mut out = Vec::with_capacity(payload.len() + header_text.len() + 32);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    out.extend_from_slice(&(header_text.len() as u32).to_le_bytes());
    out.extend_from_slice(&header_text);
    out.extend_from_slice(&crc32fast::hash(&payload).to_le_bytes());
    out.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    out.extend_from_slice(&payload);
    Ok(out)
}

fn read_u32(r: &mut impl Read) -> Result<u32, SaveError> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

fn read_header_from(r: &mut impl Read) -> Result<StrategySaveHeader, SaveError> {
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(SaveError::Format("keine Weltkarten-Spielstanddatei".into()));
    }
    let version = read_u32(r)?;
    if version != FORMAT_VERSION {
        return Err(SaveError::Version(version));
    }
    let hlen = read_u32(r)? as usize;
    if hlen > 1 << 20 {
        return Err(SaveError::Format("Header zu groß".into()));
    }
    let mut hb = vec![0u8; hlen];
    r.read_exact(&mut hb)?;
    let text = String::from_utf8(hb).map_err(|e| SaveError::Format(e.to_string()))?;
    ron::from_str(&text).map_err(|e| SaveError::Format(e.to_string()))
}

/// Dekodiert einen Spielstand, prüft CRC und Datenkompatibilität und stellt Caches wieder her.
pub fn decode(bytes: &[u8], data: Arc<StrategyData>) -> Result<(StrategySaveHeader, StrategyWorld), SaveError> {
    let mut r = std::io::Cursor::new(bytes);
    let header = read_header_from(&mut r)?;
    let crc = read_u32(&mut r)?;
    let mut lb = [0u8; 8];
    r.read_exact(&mut lb)?;
    let len = u64::from_le_bytes(lb) as usize;
    let start = r.position() as usize;
    let payload = bytes.get(start..start + len).ok_or_else(|| SaveError::Format("Datei abgeschnitten".into()))?;
    if crc32fast::hash(payload) != crc {
        return Err(SaveError::Checksum);
    }
    if header.data_hash != data_hash(&data) {
        return Err(SaveError::Incompatible("abweichende Ereignisliste".into()));
    }
    let (mut world, _): (StrategyWorld, usize) = bincode::serde::decode_from_slice(payload, bincode::config::standard()).map_err(|e| SaveError::Format(e.to_string()))?;
    world.data = data;
    world.rebuild_caches();
    Ok((header, world))
}

/// Liest nur den Header (schnell, für den Ladebildschirm).
pub fn read_header(path: &Path) -> Result<StrategySaveHeader, SaveError> {
    let mut f = std::fs::File::open(path)?;
    read_header_from(&mut f)
}

/// Schreibt atomar: temporäre Datei → fsync → vorherige Version als `.bak` sichern → umbenennen.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), SaveError> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("tmp");
    {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
    }
    if path.exists() {
        let bak = path.with_extension(format!("{EXTENSION}.bak"));
        let _ = std::fs::rename(path, bak);
    }
    std::fs::rename(&tmp, path)?;
    Ok(())
}

/// Speichert unter `dir/<slot>.acstrat`.
pub fn save_to(dir: &Path, slot: &str, world: &StrategyWorld) -> Result<PathBuf, SaveError> {
    let bytes = encode(world, slot)?;
    let path = dir.join(format!("{}.{EXTENSION}", sanitize(slot)));
    write_atomic(&path, &bytes)?;
    Ok(path)
}

/// Lädt einen Spielstand; bei beschädigter Datei wird automatisch das Backup versucht.
pub fn load_from(path: &Path, data: Arc<StrategyData>) -> Result<(StrategySaveHeader, StrategyWorld), SaveError> {
    let primary = std::fs::read(path).map_err(SaveError::from).and_then(|b| decode(&b, data.clone()));
    match primary {
        Ok(v) => Ok(v),
        Err(e @ SaveError::Incompatible(_)) | Err(e @ SaveError::Version(_)) => Err(e),
        Err(e) => {
            let bak = path.with_extension(format!("{EXTENSION}.bak"));
            if bak.exists() {
                log::warn!("Spielstand {} fehlerhaft ({e}), versuche Backup", path.display());
                let b = std::fs::read(&bak)?;
                decode(&b, data)
            } else {
                Err(e)
            }
        }
    }
}

/// Rotierende Autosaves (`autosave_0` … `autosave_{n-1}`), überschreibt den ältesten.
pub fn autosave(dir: &Path, world: &StrategyWorld, slots: usize) -> Result<PathBuf, SaveError> {
    let mut oldest: Option<(u64, String)> = None;
    for i in 0..slots.max(1) {
        let name = format!("autosave_{i}");
        let p = dir.join(format!("{name}.{EXTENSION}"));
        let t = read_header(&p).map(|h| h.created_unix).unwrap_or(0);
        if oldest.as_ref().is_none_or(|(ot, _)| t < *ot) {
            oldest = Some((t, name));
        }
    }
    let slot = oldest.map(|o| o.1).unwrap_or_else(|| "autosave_0".into());
    save_to(dir, &slot, world)
}

/// Alle Spielstände eines Verzeichnisses, neueste zuerst.
pub fn list(dir: &Path) -> Vec<(PathBuf, Result<StrategySaveHeader, String>)> {
    let Ok(rd) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut v: Vec<(PathBuf, Result<StrategySaveHeader, String>)> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == EXTENSION))
        .map(|p| {
            let h = read_header(&p).map_err(|e| e.to_string());
            (p, h)
        })
        .collect();
    v.sort_by_key(|(_, h)| std::cmp::Reverse(h.as_ref().map(|h| h.created_unix).unwrap_or(0)));
    v
}

fn sanitize(s: &str) -> String {
    let t: String = s.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' || c == '-' { c } else { '_' }).collect();
    if t.is_empty() { "spielstand".into() } else { t }
}
