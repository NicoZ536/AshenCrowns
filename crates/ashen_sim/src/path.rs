//! A*-Wegfindung auf dem Kachelgitter (8 Nachbarn, Oktil-Heuristik, keine Eckenschnitte)
//! mit anschließender Glättung per Sichtlinie.
//!
//! Der Pathfinder hält seine Puffer dauerhaft (Stempel-Technik), damit pro Anfrage
//! keine Allokationen für das Gitter nötig sind.

use crate::map::TileMap;
use glam::{IVec2, Vec2};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq)]
struct Node {
    f: f32,
    idx: u32,
}

impl Eq for Node {}
impl Ord for Node {
    fn cmp(&self, o: &Self) -> Ordering {
        // Min-Heap über f, bei Gleichstand stabil nach Index.
        o.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal).then_with(|| o.idx.cmp(&self.idx))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

/// Wiederverwendbarer A*-Löser.
#[derive(Default)]
pub struct Pathfinder {
    g: Vec<f32>,
    came: Vec<u32>,
    stamp: Vec<u32>,
    closed: Vec<u32>,
    current: u32,
    heap: BinaryHeap<Node>,
    /// Anzahl expandierter Knoten der letzten Suche (für Budgetierung/Profiling).
    pub last_expanded: usize,
    /// `true`, wenn die letzte Suche das Ziel tatsächlich erreicht hat (sonst Teilpfad).
    pub last_found: bool,
}

const DIRS: [(i32, i32, f32); 8] = [
    (1, 0, 1.0),
    (-1, 0, 1.0),
    (0, 1, 1.0),
    (0, -1, 1.0),
    (1, 1, std::f32::consts::SQRT_2),
    (1, -1, std::f32::consts::SQRT_2),
    (-1, 1, std::f32::consts::SQRT_2),
    (-1, -1, std::f32::consts::SQRT_2),
];

fn octile(a: IVec2, b: IVec2) -> f32 {
    let d = (a - b).abs();
    let (mn, mx) = (d.x.min(d.y) as f32, d.x.max(d.y) as f32);
    mx + (std::f32::consts::SQRT_2 - 1.0) * mn
}

impl Pathfinder {
    fn prepare(&mut self, n: usize) {
        if self.g.len() != n {
            self.g = vec![0.0; n];
            self.came = vec![u32::MAX; n];
            self.stamp = vec![0; n];
            self.closed = vec![0; n];
            self.current = 0;
        }
        self.current = self.current.wrapping_add(1);
        if self.current == 0 {
            self.stamp.iter_mut().for_each(|s| *s = 0);
            self.closed.iter_mut().for_each(|s| *s = 0);
            self.current = 1;
        }
        self.heap.clear();
    }

    /// Sucht einen Pfad von `from` nach `to` (Weltkoordinaten in Kacheln), ohne Torrechte.
    ///
    /// Ist das Ziel blockiert, wird die nächstgelegene begehbare Kachel angesteuert.
    /// Gibt geglättete Wegpunkte zurück (ohne Startpunkt). `None`, wenn unerreichbar
    /// oder das Knotenbudget erschöpft ist – dann wird der Knoten mit der geringsten
    /// Heuristik als Teilziel verwendet.
    pub fn find(&mut self, map: &TileMap, from: Vec2, to: Vec2, max_nodes: usize) -> Option<Vec<Vec2>> {
        self.find_for(map, from, to, max_nodes, 255)
    }

    /// Wie [`find`](Self::find) aus Sicht von Spieler `owner`: dessen offene Tore sind passierbar.
    /// Nach dem Aufruf sagt [`last_found`](Self::last_found), ob das Ziel wirklich erreicht wird
    /// (`false` = nur Teilpfad bis zum nächstgelegenen Punkt).
    pub fn find_for(&mut self, map: &TileMap, from: Vec2, to: Vec2, max_nodes: usize, owner: u8) -> Option<Vec<Vec2>> {
        let ok = |t: IVec2| if owner >= 8 { map.walkable(t) } else { map.walkable_for(t, owner) };
        let start = map.nearest_walkable(TileMap::tile_of(from), 3)?;
        let goal_tile = TileMap::tile_of(to);
        let goal_exact_ok = ok(goal_tile);
        let goal = if goal_exact_ok { goal_tile } else { map.nearest_walkable(goal_tile, 6)? };
        self.last_found = true;

        if start == goal {
            let end = if goal_exact_ok { to } else { TileMap::center_of(goal) };
            return Some(vec![end]);
        }
        let (found, end_idx, si) = self.search(map, start, goal, max_nodes, ok, |_| 0.0);
        self.last_found = found;
        if end_idx == si {
            return None;
        }
        let w = map.width;
        let tiles = self.trace(end_idx, si, w);
        let mut pts: Vec<Vec2> = tiles.into_iter().map(TileMap::center_of).collect();
        if found && goal_exact_ok
            && let Some(last) = pts.last_mut()
        {
            *last = to;
        }
        Some(smooth_for(map, from, pts, owner))
    }

    /// Sucht den Weg zu `to`, wobei feindliche Befestigungen (`breakable[idx]`) mit Zusatzkosten
    /// passierbar sind, und liefert die erste Befestigungskachel auf diesem Weg: das Ziel einer
    /// Belagerung, wenn der normale Weg versperrt ist.
    pub fn find_breach(&mut self, map: &TileMap, from: Vec2, to: Vec2, max_nodes: usize, owner: u8, breakable: &[bool]) -> Option<IVec2> {
        let ok = |t: IVec2| map.walkable_for(t, owner) || (map.in_bounds(t) && breakable[map.idx(t)]);
        let start = map.nearest_walkable(TileMap::tile_of(from), 3)?;
        let goal_tile = TileMap::tile_of(to);
        let goal = if ok(goal_tile) { goal_tile } else { map.nearest_walkable(goal_tile, 6)? };
        if start == goal {
            return None;
        }
        let extra = |t: IVec2| if breakable[map.idx(t)] { 14.0 } else { 0.0 };
        let (found, end_idx, si) = self.search(map, start, goal, max_nodes, ok, extra);
        if !found || end_idx == si {
            return None;
        }
        let tiles = self.trace(end_idx, si, map.width);
        tiles.into_iter().find(|t| breakable[map.idx(*t)])
    }

    /// A*-Kern. Liefert (Ziel erreicht, Endknoten, Startknoten).
    fn search<W: Fn(IVec2) -> bool, C: Fn(IVec2) -> f32>(&mut self, map: &TileMap, start: IVec2, goal: IVec2, max_nodes: usize, ok: W, extra: C) -> (bool, usize, usize) {
        let n = (map.width * map.height) as usize;
        self.prepare(n);
        let w = map.width;
        let si = map.idx(start);
        let gi = map.idx(goal);
        self.g[si] = 0.0;
        self.came[si] = u32::MAX;
        self.stamp[si] = self.current;
        self.heap.push(Node { f: octile(start, goal), idx: si as u32 });

        let mut best = (f32::MAX, si);
        let mut expanded = 0usize;
        let mut found = false;
        while let Some(Node { idx, .. }) = self.heap.pop() {
            let i = idx as usize;
            if self.closed[i] == self.current {
                continue;
            }
            self.closed[i] = self.current;
            if i == gi {
                found = true;
                break;
            }
            expanded += 1;
            if expanded > max_nodes {
                break;
            }
            let p = IVec2::new(i as i32 % w, i as i32 / w);
            let h = octile(p, goal);
            if h < best.0 {
                best = (h, i);
            }
            for &(dx, dy, cost) in &DIRS {
                let q = IVec2::new(p.x + dx, p.y + dy);
                if !ok(q) {
                    continue;
                }
                // Keine Diagonale an blockierten Ecken vorbei.
                if dx != 0 && dy != 0 && (!ok(IVec2::new(p.x + dx, p.y)) || !ok(IVec2::new(p.x, p.y + dy))) {
                    continue;
                }
                let qi = map.idx(q);
                if self.closed[qi] == self.current {
                    continue;
                }
                let ng = self.g[i] + cost * map.terrain[qi].move_cost() + extra(q);
                if self.stamp[qi] != self.current || ng < self.g[qi] {
                    self.stamp[qi] = self.current;
                    self.g[qi] = ng;
                    self.came[qi] = i as u32;
                    self.heap.push(Node { f: ng + octile(q, goal), idx: qi as u32 });
                }
            }
        }
        self.last_expanded = expanded;
        (found, if found { gi } else { best.1 }, si)
    }

    /// Kachelfolge vom Start (ausgeschlossen) bis `end_idx`.
    fn trace(&self, end_idx: usize, si: usize, w: i32) -> Vec<IVec2> {
        let mut tiles = Vec::new();
        let mut c = end_idx;
        while c != u32::MAX as usize {
            tiles.push(IVec2::new(c as i32 % w, c as i32 / w));
            c = self.came[c] as usize;
            if c == si {
                break;
            }
        }
        tiles.reverse();
        tiles
    }
}

/// Entfernt überflüssige Wegpunkte, solange die direkte Linie begehbar ist.
pub fn smooth(map: &TileMap, from: Vec2, pts: Vec<Vec2>) -> Vec<Vec2> {
    smooth_for(map, from, pts, 255)
}

/// Wie [`smooth`], aber mit Torrechten des Spielers `owner` (`>= 8` = keine).
pub fn smooth_for(map: &TileMap, from: Vec2, pts: Vec<Vec2>, owner: u8) -> Vec<Vec2> {
    if pts.len() <= 1 {
        return pts;
    }
    let mut out = Vec::with_capacity(pts.len());
    let mut anchor = from;
    let mut i = 0;
    while i < pts.len() {
        let mut j = pts.len() - 1;
        while j > i && !map.line_walkable_for(anchor, pts[j], owner) {
            j -= 1;
        }
        out.push(pts[j]);
        anchor = pts[j];
        i = j + 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Terrain;

    #[test]
    fn finds_path_around_wall() {
        let mut m = TileMap::new(20, 20, Terrain::Grass);
        for y in 0..18 {
            m.set_terrain(IVec2::new(10, y), Terrain::Water);
        }
        let mut pf = Pathfinder::default();
        let p = pf.find(&m, Vec2::new(2.5, 2.5), Vec2::new(17.5, 2.5), 10_000).expect("path");
        assert!((p.last().copied().unwrap() - Vec2::new(17.5, 2.5)).length() < 0.01);
        // Muss unterhalb der Mauer (y >= 18) vorbeiführen.
        assert!(p.iter().any(|w| w.y >= 18.0));
        // Jeder Abschnitt begehbar.
        let mut a = Vec2::new(2.5, 2.5);
        for &w in &p {
            assert!(m.line_walkable(a, w));
            a = w;
        }
    }

    #[test]
    fn unreachable_goal_returns_partial_or_none() {
        let mut m = TileMap::new(10, 10, Terrain::Grass);
        for y in 0..10 {
            m.set_terrain(IVec2::new(5, y), Terrain::DeepWater);
        }
        let mut pf = Pathfinder::default();
        let p = pf.find(&m, Vec2::new(1.5, 5.5), Vec2::new(8.5, 5.5), 10_000);
        // Ziel unerreichbar: Teilpfad endet diesseits des Wassers.
        if let Some(p) = p {
            assert!(p.last().unwrap().x < 5.0);
        }
    }

    #[test]
    fn blocked_goal_redirects() {
        let mut m = TileMap::new(10, 10, Terrain::Grass);
        m.occupy(IVec2::new(6, 6), IVec2::new(2, 2), true);
        let mut pf = Pathfinder::default();
        let p = pf.find(&m, Vec2::new(1.5, 1.5), Vec2::new(6.9, 6.9), 10_000).expect("path");
        let end = TileMap::tile_of(*p.last().unwrap());
        assert!(m.walkable(end));
    }
}
