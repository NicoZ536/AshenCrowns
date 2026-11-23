//! Formationslayouts. Reine Geometrie: liefert Versätze relativ zum Zielpunkt der Gruppe.
//!
//! Konvention: `x` = seitlich (nach rechts, vom Marschziel aus gesehen), `y` = Reihen nach hinten
//! (positiv = hinter dem Ziel). Die Einheiten sind nach Rang sortiert (Index 0 = vorderster Platz).

use crate::rpg::Formation;
use glam::Vec2;

/// Kleinster Abstand zwischen zwei Plätzen in Kacheln.
pub const MIN_SPACING: f32 = 0.85;

/// Versätze für `n` Einheiten in der gewählten Aufstellung.
pub fn offsets(f: Formation, n: usize) -> Vec<Vec2> {
    if n == 0 {
        return Vec::new();
    }
    match f {
        Formation::Block => {
            let cols = ((n as f32 * 1.6).sqrt().ceil() as usize).max(1);
            rows_of(n, cols, 0.85, 0.85)
        }
        Formation::Line => {
            let per_row = n.min(12);
            let rows = n.div_ceil(per_row);
            // Gleichmäßig auf die Reihen verteilen, damit die hinteren nicht nur eine Einheit enthalten.
            let cols = n.div_ceil(rows);
            rows_of(n, cols, 0.95, 0.9)
        }
        Formation::Column => {
            let cols = if n > 12 { 3 } else { 2 };
            rows_of(n, cols.min(n), 0.9, 0.9)
        }
        Formation::Wedge => {
            let mut v = Vec::with_capacity(n);
            let mut row = 0usize;
            while v.len() < n {
                let in_row = row + 1;
                for c in 0..in_row {
                    if v.len() == n {
                        break;
                    }
                    v.push(Vec2::new((c as f32 - (in_row as f32 - 1.0) * 0.5) * 1.0, row as f32 * 0.87));
                }
                row += 1;
            }
            v
        }
        Formation::Square => {
            let side = (n as f32).sqrt().ceil() as i32;
            let half = (side as f32 - 1.0) * 0.5;
            let mut cells: Vec<(i32, f32, f32, f32)> = Vec::new();
            for gy in 0..side {
                for gx in 0..side {
                    let (dx, dy) = (gx as f32 - half, gy as f32 - half);
                    let ring = dx.abs().max(dy.abs());
                    cells.push(((ring * 2.0) as i32, dy, dx.abs(), dx));
                }
            }
            // Äußerer Ring zuerst, dort die Frontseite (kleines dy) vor den Flanken und der Rückseite.
            cells.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.total_cmp(&b.1)).then(a.2.total_cmp(&b.2)).then(a.3.total_cmp(&b.3)));
            cells.truncate(n);
            cells.into_iter().map(|(_, dy, _, dx)| Vec2::new(dx * 0.95, (dy + half) * 0.95)).collect()
        }
    }
}

fn rows_of(n: usize, cols: usize, dx: f32, dy: f32) -> Vec<Vec2> {
    let cols = cols.max(1);
    (0..n)
        .map(|i| {
            let (row, col) = (i / cols, i % cols);
            let in_row = if row == n / cols { n % cols } else { cols }.max(1);
            Vec2::new((col as f32 - (in_row as f32 - 1.0) * 0.5) * dx, row as f32 * dy)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_keep_spacing() {
        for f in Formation::ALL {
            for n in [1usize, 2, 3, 5, 8, 12, 17, 24, 40] {
                let o = offsets(f, n);
                assert_eq!(o.len(), n);
                for i in 0..n {
                    for j in i + 1..n {
                        let d = (o[i] - o[j]).length();
                        assert!(d >= MIN_SPACING - 1e-3, "{f:?} n={n}: Plätze {i}/{j} nur {d:.2} auseinander");
                    }
                }
            }
        }
    }

    #[test]
    fn square_is_hollow_and_column_is_narrow() {
        let sq = offsets(Formation::Square, 12);
        let xs: Vec<f32> = sq.iter().map(|p| p.x).collect();
        let width = xs.iter().cloned().fold(f32::MIN, f32::max) - xs.iter().cloned().fold(f32::MAX, f32::min);
        let col = offsets(Formation::Column, 12);
        let cw = col.iter().map(|p| p.x).fold(f32::MIN, f32::max) - col.iter().map(|p| p.x).fold(f32::MAX, f32::min);
        assert!(cw < width, "Kolonne schmaler als Karree");
        // Zwölf Einheiten füllen exakt den Außenring eines 4×4-Karrees; die Mitte bleibt frei.
        let c = sq.iter().sum::<glam::Vec2>() / sq.len() as f32;
        assert!(sq.iter().all(|p| (*p - c).length() > 1.0), "Karree ist innen hohl");
    }
}
