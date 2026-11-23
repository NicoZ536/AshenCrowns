//! Kachelkarte: Terrain, statische Blockaden, dynamische Belegung durch Gebäude/Ressourcen.

use glam::{IVec2, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Terrain {
    Grass,
    DarkGrass,
    Dirt,
    Road,
    Stone,
    Sand,
    Ash,
    Ford,
    Water,
    DeepWater,
}

impl Terrain {
    pub const ALL: [Terrain; 10] = [
        Terrain::Grass,
        Terrain::DarkGrass,
        Terrain::Dirt,
        Terrain::Road,
        Terrain::Stone,
        Terrain::Sand,
        Terrain::Ash,
        Terrain::Ford,
        Terrain::Water,
        Terrain::DeepWater,
    ];

    pub fn walkable(self) -> bool {
        !matches!(self, Terrain::Water | Terrain::DeepWater)
    }

    pub fn buildable(self) -> bool {
        matches!(self, Terrain::Grass | Terrain::DarkGrass | Terrain::Dirt | Terrain::Road | Terrain::Stone | Terrain::Ash)
    }

    pub fn is_water(self) -> bool {
        matches!(self, Terrain::Water | Terrain::DeepWater | Terrain::Ford)
    }

    /// Zeichenpriorität für Kachelübergänge (höher überlappt niedriger).
    pub fn priority(self) -> u8 {
        match self {
            Terrain::DeepWater => 0,
            Terrain::Water => 1,
            Terrain::Ford => 2,
            Terrain::Sand => 3,
            Terrain::Ash => 4,
            Terrain::Dirt => 5,
            Terrain::Stone => 6,
            Terrain::Grass => 7,
            Terrain::DarkGrass => 8,
            Terrain::Road => 9,
        }
    }

    /// Bewegungskostenfaktor.
    pub fn move_cost(self) -> f32 {
        match self {
            Terrain::Road => 0.8,
            Terrain::Ford => 1.6,
            Terrain::Sand => 1.15,
            _ => 1.0,
        }
    }
}

/// Dekorationsobjekte der Karte (rein visuell oder blockierend).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecorKind {
    Rock,
    Boulder,
    Bush,
    GrassTuft,
    Flowers,
    DeadTree,
    Ruin,
    Pillar,
    Grave,
    Bones,
    Brazier,
    Fence,
    Crops,
    Reeds,
    Lantern,
}

impl DecorKind {
    pub fn blocks(self) -> bool {
        matches!(self, DecorKind::Boulder | DecorKind::Ruin | DecorKind::Pillar | DecorKind::DeadTree)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Decor {
    pub kind: DecorKind,
    pub pos: Vec2,
    pub variant: u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TileMap {
    pub width: i32,
    pub height: i32,
    pub terrain: Vec<Terrain>,
    /// Statisch blockiert (Felsen, Ruinen).
    pub static_block: Vec<bool>,
    /// Dynamische Belegung (Gebäude, Bäume, Minen). Zähler für überlappungsfreie Verwaltung.
    pub occupied: Vec<u8>,
    pub decor: Vec<Decor>,
    /// Torkacheln: Bitmaske der Spieler, deren Einheiten das (offene) Tor passieren dürfen.
    pub gate_pass: Vec<u8>,
    /// Version der Belegung; ändert sich bei jedem Blockadewechsel (für Pfad-Caches).
    pub revision: u32,
}

impl TileMap {
    pub fn new(width: i32, height: i32, fill: Terrain) -> Self {
        let n = (width * height) as usize;
        TileMap {
            width,
            height,
            terrain: vec![fill; n],
            static_block: vec![false; n],
            occupied: vec![0; n],
            decor: Vec::new(),
            gate_pass: vec![0; n],
            revision: 0,
        }
    }

    #[inline]
    pub fn in_bounds(&self, t: IVec2) -> bool {
        t.x >= 0 && t.y >= 0 && t.x < self.width && t.y < self.height
    }

    #[inline]
    pub fn idx(&self, t: IVec2) -> usize {
        (t.y * self.width + t.x) as usize
    }

    pub fn terrain_at(&self, t: IVec2) -> Terrain {
        if self.in_bounds(t) { self.terrain[self.idx(t)] } else { Terrain::DeepWater }
    }

    pub fn set_terrain(&mut self, t: IVec2, v: Terrain) {
        if self.in_bounds(t) {
            let i = self.idx(t);
            self.terrain[i] = v;
        }
    }

    #[inline]
    pub fn walkable(&self, t: IVec2) -> bool {
        if !self.in_bounds(t) {
            return false;
        }
        let i = self.idx(t);
        self.terrain[i].walkable() && !self.static_block[i] && self.occupied[i] == 0
    }

    /// Begehbar für Einheiten des Spielers `owner`: wie [`walkable`](Self::walkable), zusätzlich
    /// offene eigene Tore (nur von einem einzigen Gebäude belegte Kachel).
    #[inline]
    pub fn walkable_for(&self, t: IVec2, owner: u8) -> bool {
        if !self.in_bounds(t) {
            return false;
        }
        let i = self.idx(t);
        if !self.terrain[i].walkable() || self.static_block[i] {
            return false;
        }
        self.occupied[i] == 0 || (self.occupied[i] == 1 && self.gate_pass[i] & (1u8 << owner.min(7)) != 0)
    }

    /// Öffnet (`pass = true`) oder sperrt ein Tor für die Einheiten von `owner`.
    pub fn set_gate(&mut self, t: IVec2, owner: u8, pass: bool) {
        if self.in_bounds(t) {
            let i = self.idx(t);
            let bit = 1u8 << owner.min(7);
            if pass {
                self.gate_pass[i] |= bit;
            } else {
                self.gate_pass[i] &= !bit;
            }
            self.revision = self.revision.wrapping_add(1);
        }
    }

    /// Entfernt alle Torrechte einer Kachel (Tor zerstört oder abgerissen).
    pub fn clear_gate(&mut self, t: IVec2) {
        if self.in_bounds(t) {
            let i = self.idx(t);
            self.gate_pass[i] = 0;
        }
    }

    /// Bebaubar: begehbar, bebaubares Terrain, nicht belegt.
    pub fn buildable(&self, t: IVec2) -> bool {
        self.walkable(t) && self.terrain[self.idx(t)].buildable()
    }

    pub fn occupy(&mut self, origin: IVec2, size: IVec2, on: bool) {
        for y in origin.y..origin.y + size.y {
            for x in origin.x..origin.x + size.x {
                let t = IVec2::new(x, y);
                if self.in_bounds(t) {
                    let i = self.idx(t);
                    if on {
                        self.occupied[i] = self.occupied[i].saturating_add(1);
                    } else {
                        self.occupied[i] = self.occupied[i].saturating_sub(1);
                    }
                }
            }
        }
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn tile_of(p: Vec2) -> IVec2 {
        IVec2::new(p.x.floor() as i32, p.y.floor() as i32)
    }

    pub fn center_of(t: IVec2) -> Vec2 {
        Vec2::new(t.x as f32 + 0.5, t.y as f32 + 0.5)
    }

    /// Nächste begehbare Kachel (spiralförmige Suche bis `max_r`).
    pub fn nearest_walkable(&self, t: IVec2, max_r: i32) -> Option<IVec2> {
        if self.walkable(t) {
            return Some(t);
        }
        for r in 1..=max_r {
            let mut best: Option<(i32, IVec2)> = None;
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx.abs() != r && dy.abs() != r {
                        continue;
                    }
                    let c = t + IVec2::new(dx, dy);
                    if self.walkable(c) {
                        let d = dx * dx + dy * dy;
                        if best.is_none_or(|(bd, _)| d < bd) {
                            best = Some((d, c));
                        }
                    }
                }
            }
            if let Some((_, c)) = best {
                return Some(c);
            }
        }
        None
    }

    /// Sichtlinie für Pfadglättung: alle überstrichenen Kacheln begehbar (Supercover).
    pub fn line_walkable(&self, a: Vec2, b: Vec2) -> bool {
        self.line_walkable_for(a, b, 255)
    }

    /// Wie [`line_walkable`](Self::line_walkable) aus Sicht eines Spielers (offene eigene Tore zählen).
    /// `owner >= 8` ignoriert Tore.
    pub fn line_walkable_for(&self, a: Vec2, b: Vec2, owner: u8) -> bool {
        let ok = |t: IVec2| if owner >= 8 { self.walkable(t) } else { self.walkable_for(t, owner) };
        let d = b - a;
        let len = d.length();
        if len < 1e-4 {
            return ok(Self::tile_of(a));
        }
        let steps = (len * 3.0).ceil() as i32;
        // Seitlicher Versatz, damit Einheiten mit Radius nicht an Ecken hängen.
        let n = Vec2::new(-d.y, d.x) / len * 0.28;
        for i in 0..=steps {
            let p = a + d * (i as f32 / steps as f32);
            for off in [Vec2::ZERO, n, -n] {
                if !ok(Self::tile_of(p + off)) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupancy_and_walkability() {
        let mut m = TileMap::new(8, 8, Terrain::Grass);
        assert!(m.walkable(IVec2::new(2, 2)));
        m.occupy(IVec2::new(1, 1), IVec2::new(2, 2), true);
        assert!(!m.walkable(IVec2::new(2, 2)));
        assert_eq!(m.nearest_walkable(IVec2::new(1, 1), 3).map(|t| (t - IVec2::new(1, 1)).abs().max_element()), Some(1));
        m.occupy(IVec2::new(1, 1), IVec2::new(2, 2), false);
        assert!(m.walkable(IVec2::new(2, 2)));
        m.set_terrain(IVec2::new(4, 4), Terrain::Water);
        assert!(!m.walkable(IVec2::new(4, 4)));
        assert!(!m.line_walkable(Vec2::new(4.5, 0.5), Vec2::new(4.5, 7.5)));
        assert!(m.line_walkable(Vec2::new(0.5, 0.5), Vec2::new(0.5, 7.5)));
    }
}
