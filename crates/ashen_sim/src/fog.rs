//! Kriegsnebel des Spielers: unerforscht / erforscht / sichtbar.

use glam::Vec2;
use serde::{Deserialize, Serialize};

pub const UNEXPLORED: u8 = 0;
pub const EXPLORED: u8 = 1;
pub const VISIBLE: u8 = 2;

#[derive(Clone, Serialize, Deserialize)]
pub struct FogOfWar {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<u8>,
    /// Erhöht sich bei jeder Aktualisierung (Renderer lädt Textur nur bei Änderung neu).
    pub revision: u32,
    /// Kompletter Nebel deaktiviert (Sandbox / Debug).
    pub disabled: bool,
}

impl FogOfWar {
    pub fn new(width: i32, height: i32) -> Self {
        FogOfWar { width, height, cells: vec![UNEXPLORED; (width * height) as usize], revision: 0, disabled: false }
    }

    pub fn begin_update(&mut self) {
        for c in &mut self.cells {
            if *c == VISIBLE {
                *c = EXPLORED;
            }
        }
    }

    pub fn reveal(&mut self, center: Vec2, radius: f32) {
        let r2 = radius * radius;
        let x0 = ((center.x - radius).floor() as i32).max(0);
        let x1 = ((center.x + radius).ceil() as i32).min(self.width - 1);
        let y0 = ((center.y - radius).floor() as i32).max(0);
        let y1 = ((center.y + radius).ceil() as i32).min(self.height - 1);
        for y in y0..=y1 {
            for x in x0..=x1 {
                let d = Vec2::new(x as f32 + 0.5, y as f32 + 0.5) - center;
                if d.length_squared() <= r2 {
                    self.cells[(y * self.width + x) as usize] = VISIBLE;
                }
            }
        }
    }

    pub fn end_update(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn state(&self, p: Vec2) -> u8 {
        if self.disabled {
            return VISIBLE;
        }
        let x = p.x.floor() as i32;
        let y = p.y.floor() as i32;
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return UNEXPLORED;
        }
        self.cells[(y * self.width + x) as usize]
    }

    pub fn visible(&self, p: Vec2) -> bool {
        self.state(p) == VISIBLE
    }

    pub fn explored(&self, p: Vec2) -> bool {
        self.state(p) != UNEXPLORED
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reveal_and_decay() {
        let mut f = FogOfWar::new(10, 10);
        f.begin_update();
        f.reveal(Vec2::new(5.0, 5.0), 2.0);
        f.end_update();
        assert!(f.visible(Vec2::new(5.2, 5.2)));
        assert!(!f.explored(Vec2::new(0.5, 0.5)));
        f.begin_update();
        f.end_update();
        assert!(f.explored(Vec2::new(5.2, 5.2)) && !f.visible(Vec2::new(5.2, 5.2)));
    }
}
