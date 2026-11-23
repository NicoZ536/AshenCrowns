//! Robustes Speichersystem: versionierter Header, CRC32-Prüfsumme, atomares Schreiben,
//! Backups und rotierende Autosaves.
//!
//! Dateiformat (`.acsave`):
//! ```text
//! "ASHC" | u32 Formatversion | u32 Header-Länge | Header (JSON) | u32 CRC32 | u64 Länge | Payload (bincode)
//! ```
//! Der Header ist JSON, damit die Spielstandliste ohne Dekodieren des Payloads gelesen werden kann.

use crate::data::GameData;
use crate::world::SimWorld;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub const MAGIC: &[u8; 4] = b"ASHC";
pub const FORMAT_VERSION: u32 = 4;
pub const EXTENSION: &str = "acsave";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveHeader {
    pub format_version: u32,
    pub game_version: String,
    pub id: uuid::Uuid,
    pub name: String,
    pub mission_id: String,
    pub mission_name: String,
    pub mode: String,
    pub created_unix: u64,
    pub play_time_secs: f32,
    pub day: u32,
    pub type_table_hash: u32,
}

/// Inhalt eines Spielstands: Simulation plus optionale Zusatzdaten des Spiels (z. B. Kamera).
#[derive(Serialize, Deserialize)]
pub struct SavePayload {
    pub world: SimWorld,
    pub type_table: Vec<String>,
    pub client_state: String,
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
            SaveError::Incompatible(e) => write!(f, "Spielstand passt nicht zu den geladenen Spieldaten/Mods: {e}"),
        }
    }
}

impl std::error::Error for SaveError {}

impl From<std::io::Error> for SaveError {
    fn from(e: std::io::Error) -> Self {
        SaveError::Io(e)
    }
}

fn table_hash(t: &[String]) -> u32 {
    let mut h = crc32fast::Hasher::new();
    for s in t {
        h.update(s.as_bytes());
        h.update(&[0]);
    }
    h.finalize()
}

fn now_unix() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Serialisiert einen Spielstand in Bytes.
pub fn encode(world: &SimWorld, data: &GameData, name: &str, client_state: String) -> Result<Vec<u8>, SaveError> {
    let type_table = data.type_table();
    let header = SaveHeader {
        format_version: FORMAT_VERSION,
        game_version: env!("CARGO_PKG_VERSION").to_string(),
        id: uuid::Uuid::new_v4(),
        name: name.to_string(),
        mission_id: world.mission_def.id.clone(),
        mission_name: world.mission_def.name.clone(),
        mode: world.settings.mode.label().to_string(),
        created_unix: now_unix(),
        play_time_secs: world.time,
        day: world.env.day,
        type_table_hash: table_hash(&type_table),
    };
    // SavePayload besitzt die Welt nicht; wir serialisieren über eine Referenz-Hülle.
    #[derive(Serialize)]
    struct PayloadRef<'a> {
        world: &'a SimWorld,
        type_table: &'a [String],
        client_state: &'a str,
    }
    let payload = bincode::serde::encode_to_vec(PayloadRef { world, type_table: &type_table, client_state: &client_state }, bincode::config::standard())
        .map_err(|e| SaveError::Format(e.to_string()))?;
    let header_json = serde_json::to_vec(&header).map_err(|e| SaveError::Format(e.to_string()))?;
    let mut out = Vec::with_capacity(payload.len() + header_json.len() + 32);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    out.extend_from_slice(&(header_json.len() as u32).to_le_bytes());
    out.extend_from_slice(&header_json);
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

fn read_header_from(r: &mut impl Read) -> Result<SaveHeader, SaveError> {
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(SaveError::Format("keine Ashen-Crowns-Datei".into()));
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
    serde_json::from_slice(&hb).map_err(|e| SaveError::Format(e.to_string()))
}

/// Dekodiert einen Spielstand und prüft Prüfsumme und Datenkompatibilität.
pub fn decode(bytes: &[u8], data: &GameData) -> Result<(SaveHeader, SavePayload), SaveError> {
    let mut r = std::io::Cursor::new(bytes);
    let header = read_header_from(&mut r)?;
    let crc = read_u32(&mut r)?;
    let mut lb = [0u8; 8];
    r.read_exact(&mut lb)?;
    let len = u64::from_le_bytes(lb) as usize;
    let start = r.position() as usize;
    let payload_bytes = bytes.get(start..start + len).ok_or_else(|| SaveError::Format("Datei abgeschnitten".into()))?;
    if crc32fast::hash(payload_bytes) != crc {
        return Err(SaveError::Checksum);
    }
    let (payload, _): (SavePayload, usize) =
        bincode::serde::decode_from_slice(payload_bytes, bincode::config::standard()).map_err(|e| SaveError::Format(e.to_string()))?;
    let current = data.type_table();
    if payload.type_table != current {
        let missing: Vec<&String> = payload.type_table.iter().filter(|t| !current.contains(t)).take(5).collect();
        return Err(SaveError::Incompatible(format!("abweichende Inhaltsliste (z. B. fehlend: {missing:?})")));
    }
    Ok((header, payload))
}

/// Liest nur den Header (schnell, für Ladebildschirm).
pub fn read_header(path: &Path) -> Result<SaveHeader, SaveError> {
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

/// Speichert einen Spielstand unter `dir/<slot>.acsave`.
pub fn save_to(dir: &Path, slot: &str, world: &SimWorld, data: &GameData, client_state: String) -> Result<PathBuf, SaveError> {
    let bytes = encode(world, data, slot, client_state)?;
    let path = dir.join(format!("{}.{EXTENSION}", sanitize(slot)));
    write_atomic(&path, &bytes)?;
    Ok(path)
}

/// Lädt einen Spielstand; bei beschädigter Datei wird automatisch das Backup versucht.
pub fn load_from(path: &Path, data: &GameData) -> Result<(SaveHeader, SavePayload), SaveError> {
    let primary = std::fs::read(path).map_err(SaveError::from).and_then(|b| decode(&b, data));
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
pub fn autosave(dir: &Path, world: &SimWorld, data: &GameData, client_state: String, slots: usize) -> Result<PathBuf, SaveError> {
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
    save_to(dir, &slot, world, data, client_state)
}

/// Alle Spielstände eines Verzeichnisses, neueste zuerst.
pub fn list(dir: &Path) -> Vec<(PathBuf, Result<SaveHeader, String>)> {
    let Ok(rd) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut v: Vec<(PathBuf, Result<SaveHeader, String>)> = rd
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
