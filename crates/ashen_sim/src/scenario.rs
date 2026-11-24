//! Szenario-Erweiterungen des Missionsformats: explizite Kartendaten, benannte Bereiche,
//! Startpositionen. Alle Felder sind in [`MissionDef`](crate::mission::MissionDef) optional;
//! Szenarien sind damit gewöhnliche, direkt spielbare Missionen.

use crate::entity::NodeKind;
use crate::map::{Decor, DecorKind, Terrain, TileMap};
use crate::mission::MissionDef;
use glam::{IVec2, Vec2};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

impl Terrain {
    /// Einbuchstabiger Code für das Kartenformat.
    pub fn code(self) -> char {
        match self {
            Terrain::Grass => 'G',
            Terrain::DarkGrass => 'g',
            Terrain::Dirt => 'd',
            Terrain::Road => 'R',
            Terrain::Stone => 'S',
            Terrain::Sand => 's',
            Terrain::Ash => 'A',
            Terrain::Ford => 'F',
            Terrain::Water => 'w',
            Terrain::DeepWater => 'W',
        }
    }

    pub fn from_code(c: char) -> Option<Terrain> {
        Terrain::ALL.into_iter().find(|t| t.code() == c)
    }

    pub fn label(self) -> &'static str {
        match self {
            Terrain::Grass => "Gras",
            Terrain::DarkGrass => "Dunkelgras",
            Terrain::Dirt => "Erde",
            Terrain::Road => "Straße",
            Terrain::Stone => "Fels",
            Terrain::Sand => "Sand",
            Terrain::Ash => "Asche",
            Terrain::Ford => "Furt",
            Terrain::Water => "Wasser",
            Terrain::DeepWater => "Tiefes Wasser",
        }
    }
}

impl DecorKind {
    pub const ALL: [DecorKind; 18] = [
        DecorKind::Rock,
        DecorKind::Boulder,
        DecorKind::Bush,
        DecorKind::GrassTuft,
        DecorKind::Flowers,
        DecorKind::DeadTree,
        DecorKind::Ruin,
        DecorKind::Pillar,
        DecorKind::Grave,
        DecorKind::Bones,
        DecorKind::Brazier,
        DecorKind::Fence,
        DecorKind::Crops,
        DecorKind::Reeds,
        DecorKind::Lantern,
        DecorKind::Pebbles,
        DecorKind::Embers,
        DecorKind::Puddle,
    ];

    pub fn label(self) -> &'static str {
        match self {
            DecorKind::Rock => "Stein",
            DecorKind::Boulder => "Felsbrocken",
            DecorKind::Bush => "Busch",
            DecorKind::GrassTuft => "Grasbüschel",
            DecorKind::Flowers => "Blumen",
            DecorKind::DeadTree => "Toter Baum",
            DecorKind::Ruin => "Ruine",
            DecorKind::Pillar => "Säule",
            DecorKind::Grave => "Grab",
            DecorKind::Bones => "Knochen",
            DecorKind::Brazier => "Kohlenbecken",
            DecorKind::Fence => "Zaun",
            DecorKind::Crops => "Feldfrucht",
            DecorKind::Reeds => "Schilf",
            DecorKind::Lantern => "Laterne",
            DecorKind::Pebbles => "Kiesel",
            DecorKind::Embers => "Ascheglut",
            DecorKind::Puddle => "Pfütze",
        }
    }
}

impl NodeKind {
    pub fn label(self) -> &'static str {
        match self {
            NodeKind::Tree => "Baum",
            NodeKind::GoldMine => "Goldmine",
            NodeKind::StoneQuarry => "Steinbruch",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecorPlace {
    pub kind: DecorKind,
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub variant: u8,
}

/// Ressourcenquelle der Karte (Baum, Goldmine, Steinbruch) an einer Kachel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodePlace {
    pub kind: NodeKind,
    pub x: i32,
    pub y: i32,
    /// Vorrat; 0 = Standardwert der Regeln.
    #[serde(default)]
    pub amount: u32,
    #[serde(default)]
    pub variant: u8,
}

/// Explizite Karte eines Szenarios: Terrain als Zeilen aus Kachelcodes (siehe [`Terrain::code`]).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MapData {
    pub width: i32,
    pub height: i32,
    pub terrain: Vec<String>,
    #[serde(default)]
    pub decor: Vec<DecorPlace>,
    #[serde(default)]
    pub nodes: Vec<NodePlace>,
}

impl MapData {
    pub fn validate(&self) -> Result<(), String> {
        if !(16..=256).contains(&self.width) || !(16..=256).contains(&self.height) {
            return Err(format!("Kartengröße {}×{} außerhalb von 16..256", self.width, self.height));
        }
        if self.terrain.len() != self.height as usize {
            return Err(format!("Terrain hat {} Zeilen, erwartet {}", self.terrain.len(), self.height));
        }
        for (y, row) in self.terrain.iter().enumerate() {
            if row.chars().count() != self.width as usize {
                return Err(format!("Terrainzeile {y} hat {} Zeichen, erwartet {}", row.chars().count(), self.width));
            }
            if let Some(c) = row.chars().find(|c| Terrain::from_code(*c).is_none()) {
                return Err(format!("Terrainzeile {y}: unbekannter Kachelcode '{c}'"));
            }
        }
        Ok(())
    }

    /// Erzeugt die Kachelkarte inklusive blockierender Dekoration.
    pub fn to_tilemap(&self) -> Result<TileMap, String> {
        self.validate()?;
        let mut m = TileMap::new(self.width, self.height, Terrain::Grass);
        for (y, row) in self.terrain.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                m.set_terrain(IVec2::new(x as i32, y as i32), Terrain::from_code(c).unwrap_or(Terrain::Grass));
            }
        }
        for d in &self.decor {
            let pos = Vec2::new(d.x, d.y);
            let t = TileMap::tile_of(pos);
            if !m.in_bounds(t) {
                continue;
            }
            m.decor.push(Decor { kind: d.kind, pos, variant: d.variant });
            if d.kind.blocks() {
                let i = m.idx(t);
                m.static_block[i] = true;
            }
        }
        Ok(m)
    }

    /// Terrain-Zeilen aus einem Kachelvektor.
    pub fn rows_from(width: i32, height: i32, terrain: &[Terrain]) -> Vec<String> {
        (0..height).map(|y| (0..width).map(|x| terrain[(y * width + x) as usize].code()).collect()).collect()
    }

    pub fn terrain_vec(&self) -> Vec<Terrain> {
        self.terrain.iter().flat_map(|r| r.chars().map(|c| Terrain::from_code(c).unwrap_or(Terrain::Grass))).collect()
    }
}

/// Benannter rechteckiger Bereich in Kachelkoordinaten.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AreaDef {
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl AreaDef {
    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.x as f32 && p.y >= self.y as f32 && p.x < (self.x + self.w) as f32 && p.y < (self.y + self.h) as f32
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x as f32 + self.w as f32 * 0.5, self.y as f32 + self.h as f32 * 0.5)
    }
}

/// Startposition eines Spielers (Kachelkoordinaten).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartPos {
    pub owner: u8,
    pub x: f32,
    pub y: f32,
}

/// Herkunft eines Szenarios.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScenarioSource {
    Bundled,
    Mod,
    User,
}

impl ScenarioSource {
    pub fn label(self) -> &'static str {
        match self {
            ScenarioSource::Bundled => "Mitgeliefert",
            ScenarioSource::Mod => "Mod",
            ScenarioSource::User => "Eigenes",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScenarioEntry {
    pub source: ScenarioSource,
    pub path: PathBuf,
    pub def: MissionDef,
}

/// Liest alle Szenarien (`*.ron`) aus den Ordnern; fehlerhafte Dateien landen in `errors`.
pub fn discover(dirs: &[(PathBuf, ScenarioSource)], errors: &mut Vec<String>) -> Vec<ScenarioEntry> {
    let mut out: Vec<ScenarioEntry> = Vec::new();
    for (dir, source) in dirs {
        let Ok(rd) = std::fs::read_dir(dir) else { continue };
        let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "ron")).collect();
        files.sort();
        for f in files {
            match MissionDef::load(&f) {
                Ok(def) => {
                    if let Some(map) = &def.map
                        && let Err(e) = map.validate()
                    {
                        errors.push(format!("{}: {e}", f.display()));
                        continue;
                    }
                    out.push(ScenarioEntry { source: *source, path: f, def });
                }
                Err(e) => errors.push(e),
            }
        }
    }
    out
}

/// Schreibt ein Szenario als RON (kompatibel zum Missionsformat).
pub fn save(def: &MissionDef, path: &Path) -> Result<(), String> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
    }
    let cfg = ron::ser::PrettyConfig::default().indentor("  ").depth_limit(2);
    let text = ron::ser::to_string_pretty(def, cfg).map_err(|e| e.to_string())?;
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Dateiname-tauglicher Bezeichner (klein, ASCII, Unterstriche).
pub fn slug(name: &str) -> String {
    let mut s = String::new();
    for c in name.chars() {
        let c = match c {
            'ä' | 'Ä' => "ae".to_string(),
            'ö' | 'Ö' => "oe".to_string(),
            'ü' | 'Ü' => "ue".to_string(),
            'ß' => "ss".to_string(),
            c if c.is_ascii_alphanumeric() => c.to_ascii_lowercase().to_string(),
            _ => "_".to_string(),
        };
        if c == "_" && (s.ends_with('_') || s.is_empty()) {
            continue;
        }
        s.push_str(&c);
    }
    let s = s.trim_end_matches('_').to_string();
    if s.is_empty() { "szenario".into() } else { s }
}

#[cfg(test)]
mod tests {
    #[test]
    fn slug_is_filename_safe() {
        assert_eq!(super::slug("Der Fall von Grauwall (893)"), "der_fall_von_grauwall_893");
        assert_eq!(super::slug("  Äsche!! "), "aesche");
        assert_eq!(super::slug("???"), "szenario");
    }

    use super::*;

    #[test]
    fn terrain_codes_roundtrip() {
        for t in Terrain::ALL {
            assert_eq!(Terrain::from_code(t.code()), Some(t));
        }
    }

    #[test]
    fn map_data_validates_and_converts() {
        let mut m = TileMap::new(16, 16, Terrain::Dirt);
        m.set_terrain(IVec2::new(3, 4), Terrain::Water);
        let data = MapData { width: 16, height: 16, terrain: MapData::rows_from(16, 16, &m.terrain), decor: vec![DecorPlace { kind: DecorKind::Boulder, x: 5.5, y: 5.5, variant: 0 }], nodes: vec![] };
        let back = data.to_tilemap().unwrap();
        assert_eq!(back.terrain, m.terrain);
        assert!(back.static_block[back.idx(IVec2::new(5, 5))]);
        let mut bad = data.clone();
        bad.terrain[2].push('G');
        assert!(bad.validate().is_err());
    }
}
