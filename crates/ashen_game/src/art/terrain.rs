//! Bodenkacheln (32×16-Diamanten), Übergangsränder, Brücke, Bauplatz-Fundamente.

use super::palette as pal;
use ashen_core::color::{Ramp, Rgba};
use ashen_core::noise::{bayer, fbm2, rand2};
use ashen_engine::assets::{AtlasBuilder, Canvas};
use ashen_sim::map::Terrain;
use glam::{Vec2, Vec3};

pub const TW: i32 = 32;
pub const TH: i32 = 16;
pub const VARIANTS: u32 = 4;

/// Liegt (x, y) innerhalb des Kachel-Diamanten? (exakt kachelbar)
pub fn in_diamond(x: i32, y: i32) -> bool {
    if !(0..TH).contains(&y) {
        return false;
    }
    let hw = if y < TH / 2 { (y + 1) * 2 } else { (TH - y) * 2 };
    x >= TW / 2 - hw && x < TW / 2 + hw
}

/// Abstand (in Pixeln, grob) zur Kante auf einer Seite: 0 = NE, 1 = SE, 2 = SW, 3 = NW.
fn edge_distance(x: i32, y: i32, side: u8) -> f32 {
    let fx = x as f32 + 0.5 - 16.0;
    let fy = y as f32 + 0.5 - 8.0;
    // Normierte Diamantkoordinaten: |u| + |v| <= 1
    let u = fx / 16.0;
    let v = fy / 8.0;
    let d = match side {
        0 => 1.0 - (u - v), // rechts oben
        1 => 1.0 - (u + v), // rechts unten
        2 => 1.0 - (-u + v),
        _ => 1.0 - (-u - v),
    };
    d * 8.0
}

fn name(t: Terrain) -> &'static str {
    match t {
        Terrain::Grass => "grass",
        Terrain::DarkGrass => "darkgrass",
        Terrain::Dirt => "dirt",
        Terrain::Road => "road",
        Terrain::Stone => "stone",
        Terrain::Sand => "sand",
        Terrain::Ash => "ash",
        Terrain::Ford => "ford",
        Terrain::Water => "water",
        Terrain::DeepWater => "deepwater",
    }
}

pub fn tile_name(t: Terrain, variant: u32) -> String {
    format!("tile_{}_{}", name(t), variant % VARIANTS)
}

pub fn edge_name(t: Terrain, side: u8) -> String {
    format!("edge_{}_{}", name(t), side)
}

/// Ursprung des k-ten Mikrodetails (Kiesel, Rippel, Riss, Büschel) einer Kachelvariante, im Inneren des Diamanten.
fn feat(seed: u32, k: i32) -> (i32, i32) {
    (8 + (rand2(k, 1, seed ^ 0xFE) * 15.0) as i32, 4 + (rand2(k, 2, seed ^ 0xFE) * 7.0) as i32)
}

/// Schaumsaum entlang der Uferkante, die `edge_name(t, side)` in eine Wasserkachel zeichnet.
pub fn foam_name(t: Terrain, side: u8) -> String {
    format!("foam_{}_{}", name(t), side)
}

/// Farbe eines Bodenpixels (Weltkoordinaten-unabhängig, mit Variantensamen).
fn texel(t: Terrain, x: i32, y: i32, seed: u32) -> (Rgba, Vec3, f32) {
    let fx = x as f32;
    let fy = y as f32 * 2.0; // isometrische Stauchung ausgleichen
    let n = 0.28 + fbm2(fx * 0.06, fy * 0.06, seed, 3) * 0.5 + (fbm2(fx * 0.3, fy * 0.3, seed ^ 0x55, 2) - 0.5) * 0.12;
    let r = rand2(x, y, seed ^ 0xA1);
    let b = bayer(x, y);
    let mut normal = Vec3::new(0.0, -0.25, 1.0);
    let mut glow = 0.0;
    let ramp: Ramp;
    let mut tone = n;
    match t {
        Terrain::Grass | Terrain::DarkGrass => {
            ramp = if t == Terrain::Grass { pal::grass() } else { pal::dark_grass() };
            // Halme: helle Pixel mit dunklem Fuß.
            let blade = rand2(x, y / 2, seed ^ 0x51) > 0.95;
            let under = rand2(x, (y + 1) / 2, seed ^ 0x51) > 0.95;
            if blade && y % 2 == 0 {
                tone += 0.22;
                normal = Vec3::new(0.0, -0.6, 0.8);
            } else if under {
                tone -= 0.12;
            }
            // Büschel: kleine Dreiergruppen mit hellen Spitzen, dazu dunkle Moosflecken.
            for k in 0..3 {
                let (ox, oy) = feat(seed, k);
                let (dx, dy) = (x - ox, y - oy);
                if k < 2 {
                    if dy == 0 && (dx == 0 || dx == 2) || dy == -1 && dx == 1 {
                        tone += 0.34;
                        normal = Vec3::new(0.0, -0.6, 0.8);
                    } else if dy == 1 && (0..3).contains(&dx) {
                        tone -= 0.2;
                    }
                } else if dx * dx + dy * dy * 3 <= 12 {
                    tone -= 0.16;
                }
            }
            if r > 0.995 && t == Terrain::Grass {
                return (Rgba::hex(0x6a6a3a), normal, 0.0);
            }
        }
        Terrain::Dirt => {
            ramp = pal::dirt();
            if r > 0.97 {
                tone += 0.3;
                normal = Vec3::new(-0.3, -0.5, 0.8);
            } else if r < 0.03 {
                tone -= 0.25;
            }
            // Mikrodetails: Kiesel mit Schatten und Trockenrisse (Pfützen sind eigene Dekoration).
            for k in 0..4 {
                let (ox, oy) = feat(seed, k);
                let (dx, dy) = (x - ox, y - oy);
                match k {
                    0 | 1 => {
                        if dy == 0 && (dx == 0 || dx == 1) {
                            tone += 0.4;
                            normal = Vec3::new(-0.3, -0.5, 0.8);
                        } else if dy == 1 && dx == 1 {
                            tone -= 0.3;
                        }
                    }
                    // Riss: gezackte Diagonale
                    2 if (0..6).contains(&dx) && dy == (dx + 1) / 2 => tone -= 0.42,
                    _ => {}
                }
            }
        }
        Terrain::Road => {
            ramp = pal::stone();
            // Kopfsteinpflaster: versetzte Steinreihen im isometrischen Raster.
            let row = (fy / 4.0).floor() as i32;
            let off = if row % 2 == 0 { 0.0 } else { 3.0 };
            let col = ((fx + off) / 6.0).floor() as i32;
            let lx = (fx + off) - col as f32 * 6.0;
            let ly = fy - row as f32 * 4.0;
            let mortar = lx < 1.0 || ly < 1.0;
            let stone_tone = rand2(col, row, seed ^ 0x77) * 0.35;
            if mortar {
                return (pal::dirt().dither(0.25 + n * 0.2, x, y), Vec3::new(0.0, 0.0, 1.0), 0.0);
            }
            tone = 0.35 + stone_tone + (if ly < 2.0 { 0.18 } else { -0.05 }) - (lx / 6.0) * 0.12;
            normal = if ly < 2.0 { Vec3::new(-0.2, -0.6, 0.77) } else { Vec3::new(0.15, 0.25, 0.95) };
        }
        Terrain::Stone => {
            ramp = pal::rock();
            let crack = (fbm2(fx * 0.4, fy * 0.4, seed ^ 0x3, 2) - 0.5).abs() < 0.03;
            if crack {
                tone -= 0.4;
            }
        }
        Terrain::Sand => {
            ramp = pal::sand();
            // Weiche Düne plus gebogene Rippelstriche, Kiesel und Feinkorn.
            tone += ((fx * 0.17 + fy * 0.55 + (seed % 7) as f32).sin() * 0.07) + (r - 0.5) * 0.08;
            for k in 0..5 {
                let (ox, oy) = feat(seed, k);
                let dx = x - ox;
                if k < 3 {
                    let len = 5 + (rand2(k, 3, seed) * 5.0) as i32;
                    if (0..len).contains(&dx) {
                        let cy = oy + ((dx as f32 * 0.5).sin() * 0.9).round() as i32;
                        if y == cy {
                            tone += 0.3;
                            normal = Vec3::new(0.0, -0.55, 0.83);
                        } else if y == cy + 1 {
                            tone -= 0.2;
                        }
                    }
                } else if y == oy && (dx == 0 || dx == 1) {
                    tone += 0.35;
                    normal = Vec3::new(-0.3, -0.5, 0.8);
                } else if y == oy + 1 && dx == 1 {
                    tone -= 0.28;
                }
            }
        }
        Terrain::Ash => {
            ramp = pal::ash();
            if r > 0.9993 {
                glow = 0.6;
                return (pal::EMBER, normal, glow);
            }
            let vein = (fbm2(fx * 0.25, fy * 0.25, seed ^ 0x99, 3) - 0.5).abs() < 0.02;
            if vein {
                return (Rgba::hex(0x2a3a2e), normal, 0.0);
            }
            // Aschedriften (helle Streifen), verkohlte Splitter, selten ein glimmender Riss.
            tone += (r - 0.5) * 0.06;
            for k in 0..5 {
                let (ox, oy) = feat(seed, k);
                let dx = x - ox;
                if k < 2 {
                    let len = 6 + (rand2(k, 3, seed) * 5.0) as i32;
                    if (0..len).contains(&dx) {
                        let cy = oy + ((dx as f32 * 0.4).sin() * 0.8).round() as i32;
                        if y == cy {
                            tone += 0.32;
                        } else if y == cy + 1 {
                            tone -= 0.18;
                        }
                    }
                } else if k < 4 {
                    if y == oy && (0..3).contains(&dx) {
                        tone -= 0.4;
                    } else if y == oy - 1 && dx == 1 {
                        tone += 0.25;
                    }
                } else if (0..5).contains(&dx) && y == oy + (dx % 2) && seed % 4 == 1 {
                    return (Rgba::hex(0x7a3418), normal, 0.35);
                }
            }
        }
        Terrain::Ford => {
            // Sandiger Grund mit Kieseln; der Wasser-Shader legt Tönung und Wellenlinien darüber.
            ramp = pal::ford();
            tone = 0.5 + (n - 0.5) * 0.25 + (r - 0.5) * 0.1;
            if r > 0.955 {
                return (pal::rock().at(0.5 + r * 0.1), Vec3::new(-0.3, -0.5, 0.8), 0.0);
            }
        }
        Terrain::Water | Terrain::DeepWater => {
            // Glatte Fläche: Farbe und Bewegung kommen vollständig aus dem Wasser-Shader.
            ramp = pal::water_tex();
            tone = 0.5;
        }
    }
    let _ = b;
    (ramp.dither(tone.clamp(0.0, 1.0), x, y), normal, glow)
}

fn paint_tile(t: Terrain, seed: u32) -> Canvas {
    let mut c = Canvas::new(TW, TH);
    for y in 0..TH {
        for x in 0..TW {
            if in_diamond(x, y) {
                let (col, n, g) = texel(t, x, y, seed);
                c.pxn(x, y, col, n);
                if g > 0.0 {
                    c.glow(x, y, g);
                }
            }
        }
    }
    c
}

fn paint_edge(t: Terrain, side: u8, seed: u32) -> Canvas {
    let mut c = Canvas::new(TW, TH);
    for y in 0..TH {
        for x in 0..TW {
            if !in_diamond(x, y) {
                continue;
            }
            let d = edge_distance(x, y, side);
            let jag = fbm2(x as f32 * 0.3, y as f32 * 0.6, seed ^ side as u32, 2) * 6.5 + 1.0;
            if d < jag {
                let (col, n, g) = texel(t, x, y, seed);
                // Dunkle Randlinie für Tiefe, wenn der Rand höher liegt (Gras über Erde).
                let rim = d > jag - 1.0 && matches!(t, Terrain::Grass | Terrain::DarkGrass | Terrain::Road | Terrain::Stone);
                c.pxn(x, y, if rim { col.scale(0.72) } else { col }, n);
                if g > 0.0 {
                    c.glow(x, y, g);
                }
            }
        }
    }
    c
}

/// Schaumsaum: ein schmaler Streifen direkt außerhalb der Uferkante von `paint_edge` (liegt auf der Wasserkachel).
fn paint_foam_edge(side: u8, seed: u32) -> Canvas {
    let mut c = Canvas::new(TW, TH);
    let foam = Ramp::from_hex(&[0x6f8a86, 0x9bb4ae, 0xc9dad4]);
    for y in 0..TH {
        for x in 0..TW {
            if !in_diamond(x, y) {
                continue;
            }
            let d = edge_distance(x, y, side);
            let jag = fbm2(x as f32 * 0.3, y as f32 * 0.6, seed ^ side as u32, 2) * 6.5 + 1.0;
            let off = d - jag;
            if !(0.0..=2.6).contains(&off) {
                continue;
            }
            let r = rand2(x, y, seed ^ 0xF0A);
            // Kern: durchgehende helle Linie; außen aufgelöste Schaumflecken.
            if off < 1.0 {
                c.pxn(x, y, foam.at(0.85 + r * 0.15), Vec3::Z);
            } else if r > 0.55 {
                c.pxn(x, y, foam.at(0.4 + r * 0.4), Vec3::Z);
            }
        }
    }
    c
}

/// Holzbrücke über Wasser.
fn paint_bridge() -> Canvas {
    let mut c = Canvas::new(TW, TH + 6);
    let wood = pal::wood();
    for y in 0..TH {
        for x in 0..TW {
            if in_diamond(x, y) {
                // Planken quer zur Laufrichtung (entlang der Kachel-y-Achse).
                let plank = ((x + y * 2) / 3) % 2 == 0;
                let gap = (x + y * 2) % 3 == 0;
                let t = if gap { 0.1 } else if plank { 0.6 } else { 0.45 } + rand2(x, y, 9) * 0.15;
                c.pxn(x, y + 2, wood.dither(t, x, y), Vec3::new(0.0, -0.3, 1.0));
            }
        }
    }
    // Seitliche Balken (Unterkante) für Tiefe.
    for x in 0..TW {
        for dy in 0..3 {
            let y = if x < 16 { 8 + x / 2 } else { 8 + (31 - x) / 2 } + 2 + dy;
            if x % 7 != 3 || dy < 2 {
                c.pxn(x, y.min(TH + 5), pal::dark_wood().at(0.3 + dy as f32 * 0.1), Vec3::new(0.0, 0.5, 0.8));
            }
        }
    }
    c
}

/// Fundamentrahmen für Baustellen in Kachelgröße (w × h).
pub fn paint_foundation(w: i32, h: i32) -> Canvas {
    let cw = (w + h) * 16;
    let ch = (w + h) * 8;
    let mut c = Canvas::new(cw, ch);
    let stone = pal::stone();
    let dirt = pal::dirt();
    for y in 0..ch {
        for x in 0..cw {
            // Kachelkoordinate relativ zur Grundfläche.
            let wx = x as f32 + 0.5 - h as f32 * 16.0;
            let wy = y as f32 + 0.5;
            let tx = (wx / 16.0 + wy / 8.0) * 0.5;
            let ty = (wy / 8.0 - wx / 16.0) * 0.5;
            if tx < 0.0 || ty < 0.0 || tx > w as f32 || ty > h as f32 {
                continue;
            }
            let border = tx < 0.18 || ty < 0.18 || tx > w as f32 - 0.18 || ty > h as f32 - 0.18;
            if border {
                c.pxn(x, y, stone.dither(0.45 + rand2(x, y, 3) * 0.3, x, y), Vec3::new(0.0, -0.4, 0.9));
            } else {
                c.pxn(x, y, dirt.dither(0.35 + rand2(x, y, 4) * 0.2, x, y), Vec3::Z);
            }
        }
    }
    c
}

pub fn build(atlas: &mut AtlasBuilder) {
    for (ti, t) in Terrain::ALL.iter().enumerate() {
        for v in 0..VARIANTS {
            let seed = 1000 + ti as u32 * 97 + v * 13;
            atlas.add(tile_name(*t, v), paint_tile(*t, seed), Vec2::new(16.0, 8.0));
        }
        if t.priority() >= Terrain::Sand.priority() || *t == Terrain::Ford {
            for side in 0..4u8 {
                atlas.add(edge_name(*t, side), paint_edge(*t, side, 5000 + ti as u32 * 31), Vec2::new(16.0, 8.0));
                atlas.add(foam_name(*t, side), paint_foam_edge(side, 5000 + ti as u32 * 31), Vec2::new(16.0, 8.0));
            }
        }
    }
    atlas.add("bridge", paint_bridge(), Vec2::new(16.0, 10.0));
    for (w, h) in [(1, 1), (2, 2), (3, 2), (3, 3)] {
        atlas.add(format!("foundation_{w}x{h}"), paint_foundation(w, h), Vec2::new(h as f32 * 16.0, 0.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diamond_tiles_without_gaps() {
        // Benachbarte Kacheln dürfen sich an den Kanten überlappen, aber keine Lücken lassen.
        let mut cov = std::collections::HashMap::new();
        for dx in -3..4 {
            for dy in -3..4 {
                let (ox, oy) = (dx * 16 - dy * 16, dx * 8 + dy * 8);
                for y in 0..TH {
                    for x in 0..TW {
                        if in_diamond(x, y) {
                            *cov.entry((x + ox, y + oy)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        for y in -8..24 {
            for x in -16..48 {
                assert!(cov.get(&(x, y)).copied().unwrap_or(0) >= 1, "Lücke bei {x},{y}");
            }
        }
    }
}
