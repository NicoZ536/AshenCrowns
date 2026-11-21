//! Gleichförmiges Raster zur schnellen Nachbarschaftssuche von Einheiten.

use crate::entity::UnitId;
use glam::Vec2;

pub const CELL: f32 = 2.0;

#[derive(Default)]
pub struct SpatialGrid {
    w: i32,
    h: i32,
    cells: Vec<Vec<(UnitId, Vec2)>>,
}

impl SpatialGrid {
    pub fn rebuild(&mut self, map_w: i32, map_h: i32, units: impl Iterator<Item = (UnitId, Vec2)>) {
        let w = (map_w as f32 / CELL).ceil() as i32 + 1;
        let h = (map_h as f32 / CELL).ceil() as i32 + 1;
        if w != self.w || h != self.h {
            self.w = w;
            self.h = h;
            self.cells = vec![Vec::new(); (w * h) as usize];
        } else {
            self.cells.iter_mut().for_each(Vec::clear);
        }
        for (id, p) in units {
            let i = self.cell_index(p);
            self.cells[i].push((id, p));
        }
    }

    fn cell_index(&self, p: Vec2) -> usize {
        let cx = ((p.x / CELL) as i32).clamp(0, self.w - 1);
        let cy = ((p.y / CELL) as i32).clamp(0, self.h - 1);
        (cy * self.w + cx) as usize
    }

    /// Alle Einheiten im Radius `r` um `p` (nach Positionen zum Zeitpunkt des Aufbaus).
    pub fn query(&self, p: Vec2, r: f32, out: &mut Vec<UnitId>) {
        out.clear();
        if self.cells.is_empty() {
            return;
        }
        let r2 = r * r;
        let x0 = (((p.x - r) / CELL) as i32).clamp(0, self.w - 1);
        let x1 = (((p.x + r) / CELL) as i32).clamp(0, self.w - 1);
        let y0 = (((p.y - r) / CELL) as i32).clamp(0, self.h - 1);
        let y1 = (((p.y + r) / CELL) as i32).clamp(0, self.h - 1);
        for cy in y0..=y1 {
            for cx in x0..=x1 {
                for &(id, q) in &self.cells[(cy * self.w + cx) as usize] {
                    if (q - p).length_squared() <= r2 {
                        out.push(id);
                    }
                }
            }
        }
    }
}
