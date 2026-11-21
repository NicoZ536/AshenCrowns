//! Deterministisches, plattformunabhängiges Hash-Rauschen (Value Noise + fBm).
//!
//! Keine Zufallsquelle mit globalem Zustand: gleiche Eingabe + Seed ⇒ gleiches Ergebnis.

/// 32-Bit-Integer-Hash (lowbias32 von Chris Wellons).
#[inline]
pub fn hash_u32(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x
}

/// Hash zweier Koordinaten plus Seed.
#[inline]
pub fn hash2(x: i32, y: i32, seed: u32) -> u32 {
    hash_u32((x as u32).wrapping_mul(0x9E37_79B1) ^ hash_u32((y as u32) ^ seed.wrapping_mul(0x85EB_CA77)))
}

/// Gleichverteilte Zahl in `[0, 1)` aus Koordinaten.
#[inline]
pub fn rand2(x: i32, y: i32, seed: u32) -> f32 {
    (hash2(x, y, seed) >> 8) as f32 / (1u32 << 24) as f32
}

#[inline]
fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Bilinear interpoliertes Value Noise in `[0, 1)`.
pub fn value2(x: f32, y: f32, seed: u32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let fx = smooth(x - xi as f32);
    let fy = smooth(y - yi as f32);
    let a = rand2(xi, yi, seed);
    let b = rand2(xi + 1, yi, seed);
    let c = rand2(xi, yi + 1, seed);
    let d = rand2(xi + 1, yi + 1, seed);
    let ab = a + (b - a) * fx;
    let cd = c + (d - c) * fx;
    ab + (cd - ab) * fy
}

/// Fraktales Rauschen (fractional Brownian motion), Ergebnis in `[0, 1)`.
pub fn fbm2(x: f32, y: f32, seed: u32, octaves: u32) -> f32 {
    let mut amp = 0.5;
    let mut freq = 1.0;
    let mut sum = 0.0;
    let mut norm = 0.0;
    for o in 0..octaves {
        sum += value2(x * freq, y * freq, seed.wrapping_add(o * 1013)) * amp;
        norm += amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    sum / norm
}

/// 4×4-Bayer-Matrix für geordnetes Dithering, Werte in `[0, 1)`.
pub const BAYER4: [[f32; 4]; 4] = [
    [0.0 / 16.0, 8.0 / 16.0, 2.0 / 16.0, 10.0 / 16.0],
    [12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0, 6.0 / 16.0],
    [3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0],
    [15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0],
];

/// Schwellwert des Bayer-Ditherings an einer Pixelposition.
#[inline]
pub fn bayer(x: i32, y: i32) -> f32 {
    BAYER4[(y & 3) as usize][(x & 3) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_and_bounded() {
        for i in 0..200 {
            let x = i as f32 * 0.37;
            let y = i as f32 * 1.13;
            let a = fbm2(x, y, 42, 4);
            assert_eq!(a, fbm2(x, y, 42, 4));
            assert!((0.0..1.0).contains(&a));
        }
        assert_ne!(rand2(1, 2, 3), rand2(2, 1, 3));
    }
}
