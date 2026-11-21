//! Farbhilfen für prozedurale Pixel-Art und UI.

use serde::{Deserialize, Serialize};

/// 8-Bit-RGBA-Farbe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub const TRANSPARENT: Rgba = Rgba(0, 0, 0, 0);

    /// Aus `0xRRGGBB` (voll deckend).
    pub const fn hex(v: u32) -> Rgba {
        Rgba((v >> 16) as u8, (v >> 8) as u8, v as u8, 255)
    }

    pub fn with_alpha(self, a: u8) -> Rgba {
        Rgba(self.0, self.1, self.2, a)
    }

    /// Lineare Interpolation im sRGB-Raum (für Pixel-Art bewusst so gewählt).
    pub fn lerp(self, o: Rgba, t: f32) -> Rgba {
        let t = t.clamp(0.0, 1.0);
        let l = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round() as u8;
        Rgba(l(self.0, o.0), l(self.1, o.1), l(self.2, o.2), l(self.3, o.3))
    }

    /// Helligkeit skalieren (`f < 1` dunkler).
    pub fn scale(self, f: f32) -> Rgba {
        let s = |a: u8| (a as f32 * f).round().clamp(0.0, 255.0) as u8;
        Rgba(s(self.0), s(self.1), s(self.2), self.3)
    }

    pub fn luma(self) -> f32 {
        (0.299 * self.0 as f32 + 0.587 * self.1 as f32 + 0.114 * self.2 as f32) / 255.0
    }

    pub fn to_f32(self) -> [f32; 4] {
        [self.0 as f32 / 255.0, self.1 as f32 / 255.0, self.2 as f32 / 255.0, self.3 as f32 / 255.0]
    }

    pub fn to_array(self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

/// Farbrampe von dunkel nach hell. Grundlage für Schattierung mit Dithering.
#[derive(Clone, Debug)]
pub struct Ramp(pub Vec<Rgba>);

impl Ramp {
    pub fn from_hex(v: &[u32]) -> Ramp {
        Ramp(v.iter().map(|&h| Rgba::hex(h)).collect())
    }

    /// Farbe für Helligkeit `t ∈ [0,1]` ohne Dithering.
    pub fn at(&self, t: f32) -> Rgba {
        let n = self.0.len();
        let i = ((t.clamp(0.0, 0.999)) * n as f32) as usize;
        self.0[i.min(n - 1)]
    }

    /// Farbe für Helligkeit `t` mit geordnetem Dithering zwischen zwei Stufen.
    pub fn dither(&self, t: f32, x: i32, y: i32) -> Rgba {
        let n = self.0.len() as f32;
        let v = t.clamp(0.0, 0.999) * (n - 1.0);
        let base = v.floor();
        let frac = v - base;
        let i = base as usize;
        if frac > crate::noise::bayer(x, y) && i + 1 < self.0.len() {
            self.0[i + 1]
        } else {
            self.0[i]
        }
    }

    pub fn darkest(&self) -> Rgba {
        self.0[0]
    }

    pub fn lightest(&self) -> Rgba {
        *self.0.last().expect("ramp must not be empty")
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_and_ramp() {
        assert_eq!(Rgba::hex(0x102030), Rgba(16, 32, 48, 255));
        let r = Ramp::from_hex(&[0x000000, 0x808080, 0xffffff]);
        assert_eq!(r.at(0.0), Rgba::hex(0x000000));
        assert_eq!(r.at(1.0), Rgba::hex(0xffffff));
        assert_eq!(r.dither(0.0, 0, 0), Rgba::hex(0x000000));
    }
}
