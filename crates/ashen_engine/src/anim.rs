//! Einheitliches Animationssystem: Clips aus Atlas-Sprites, abgespielt über Zeit.

use crate::render::SpriteRect;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct AnimationClip {
    pub frames: Vec<SpriteRect>,
    pub fps: f32,
    pub looped: bool,
}

impl AnimationClip {
    pub fn new(frames: Vec<SpriteRect>, fps: f32, looped: bool) -> AnimationClip {
        AnimationClip { frames, fps, looped }
    }

    pub fn duration(&self) -> f32 {
        self.frames.len() as f32 / self.fps.max(0.01)
    }

    /// Frame zum Zeitpunkt `t` (Sekunden seit Clipstart).
    pub fn frame_at(&self, t: f32) -> &SpriteRect {
        let n = self.frames.len().max(1);
        let i = (t.max(0.0) * self.fps) as usize;
        let i = if self.looped { i % n } else { i.min(n - 1) };
        &self.frames[i.min(self.frames.len().saturating_sub(1))]
    }

    pub fn finished(&self, t: f32) -> bool {
        !self.looped && t >= self.duration()
    }
}

/// Animationen eines Objekts, nach Namen (z. B. "idle", "walk", "attack", "death", "work").
#[derive(Clone, Debug, Default)]
pub struct AnimationSet {
    pub clips: HashMap<String, AnimationClip>,
}

impl AnimationSet {
    pub fn insert(&mut self, name: &str, clip: AnimationClip) {
        self.clips.insert(name.to_string(), clip);
    }

    /// Clip nach Namen mit Rückfall auf "idle".
    pub fn get(&self, name: &str) -> Option<&AnimationClip> {
        self.clips.get(name).or_else(|| self.clips.get("idle"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looping_and_clamping() {
        let frames: Vec<SpriteRect> = (0..4).map(|i| SpriteRect { uv: [i as f32; 4], ..Default::default() }).collect();
        let c = AnimationClip::new(frames.clone(), 10.0, true);
        assert_eq!(c.frame_at(0.45).uv[0], 0.0);
        let d = AnimationClip::new(frames, 10.0, false);
        assert_eq!(d.frame_at(5.0).uv[0], 3.0);
        assert!(d.finished(0.5));
    }
}
