//! Low-Poly-Meshes für den 3D→Pixel-Bake. Jeder Vertex gehört starr zu genau einem Knochen
//! (Rigid Skinning) und trägt Farbe, Leuchtkraft und eine Materialkennung für prozedurale Muster.

use glam::{Mat4, Quat, Vec3};

/// Materialkennungen (müssen zu `bake.wgsl` passen).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Mat {
    Plain = 0,
    StoneBrick = 1,
    WoodPlank = 2,
    RoofTile = 3,
    Thatch = 4,
    Cloth = 5,
    Metal = 6,
    Skin = 7,
    Leaves = 8,
    Bark = 9,
    Bone = 10,
    Glow = 11,
    Rock = 12,
    Plaster = 13,
    Hair = 14,
    Crops = 15,
    Leather = 16,
    Gold = 17,
}

#[derive(Clone, Copy, Debug)]
pub struct MVertex {
    /// Position im Knochenraum.
    pub pos: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
    pub glow: f32,
    pub mat: Mat,
    pub bone: usize,
}

/// Dreiecksnetz (je drei Vertices ein Dreieck, gegen den Uhrzeigersinn von außen).
#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub verts: Vec<MVertex>,
}

/// Material- und Knotenzustand beim Modellieren.
#[derive(Clone, Copy, Debug)]
pub struct Paint {
    pub color: Vec3,
    pub glow: f32,
    pub mat: Mat,
    pub bone: usize,
    /// Flache (facettierte) statt geglätteter Normalen.
    pub flat: bool,
}

impl Paint {
    pub fn new(hex: u32, mat: Mat) -> Paint {
        Paint { color: hex_rgb(hex), glow: 0.0, mat, bone: 0, flat: false }
    }
    pub fn bone(mut self, b: usize) -> Paint {
        self.bone = b;
        self
    }
    pub fn glow(mut self, g: f32) -> Paint {
        self.glow = g;
        self
    }
    pub fn flat(mut self) -> Paint {
        self.flat = true;
        self
    }
}

pub fn hex_rgb(h: u32) -> Vec3 {
    // sRGB-Hex → linear (der Bake rechnet linear, Ausgabe wieder sRGB).
    let c = |v: u32| {
        let s = ((h >> v) & 0xff) as f32 / 255.0;
        if s <= 0.04045 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
    };
    Vec3::new(c(16), c(8), c(0))
}

impl Mesh {
#[allow(clippy::too_many_arguments)]
    pub fn tri(&mut self, p: &Paint, a: Vec3, b: Vec3, c: Vec3, na: Vec3, nb: Vec3, nc: Vec3) {
        let flat = (b - a).cross(c - a).normalize_or_zero();
        for (pos, n) in [(a, na), (b, nb), (c, nc)] {
            let normal = if p.flat { flat } else { n.normalize_or(flat) };
            self.verts.push(MVertex { pos, normal, color: p.color, glow: p.glow, mat: p.mat, bone: p.bone });
        }
    }

    pub fn tri_flat(&mut self, p: &Paint, a: Vec3, b: Vec3, c: Vec3) {
        let n = (b - a).cross(c - a).normalize_or_zero();
        self.tri(p, a, b, c, n, n, n);
    }

    pub fn quad(&mut self, p: &Paint, a: Vec3, b: Vec3, c: Vec3, d: Vec3) {
        self.tri_flat(p, a, b, c);
        self.tri_flat(p, a, c, d);
    }

    /// Quader zwischen `min` und `max`, transformiert mit `t`.
    pub fn cuboid(&mut self, p: &Paint, min: Vec3, max: Vec3, t: Mat4) {
        let c = |x: f32, y: f32, z: f32| t.transform_point3(Vec3::new(x, y, z));
        let (a, b) = (min, max);
        let v = [c(a.x, a.y, a.z), c(b.x, a.y, a.z), c(b.x, b.y, a.z), c(a.x, b.y, a.z), c(a.x, a.y, b.z), c(b.x, a.y, b.z), c(b.x, b.y, b.z), c(a.x, b.y, b.z)];
        let faces = [[4, 5, 6, 7], [1, 0, 3, 2], [5, 1, 2, 6], [0, 4, 7, 3], [7, 6, 2, 3], [0, 1, 5, 4]];
        let fp = Paint { flat: true, ..*p };
        for f in faces {
            self.quad(&fp, v[f[0]], v[f[1]], v[f[2]], v[f[3]]);
        }
    }

    /// Achsenparalleler Quader in Knochenkoordinaten (Mitte, halbe Ausdehnung).
    pub fn bx(&mut self, p: &Paint, center: Vec3, half: Vec3) {
        self.cuboid(p, center - half, center + half, Mat4::IDENTITY);
    }

    /// Kegelstumpf/Zylinder entlang einer Achse von `a` nach `b`.
    pub fn frustum(&mut self, p: &Paint, a: Vec3, b: Vec3, ra: f32, rb: f32, seg: u32) {
        self.frustum_caps(p, a, b, ra, rb, seg, true);
    }

    /// Kegelstumpf ohne Deckel (Röcke, Ärmel, Manschetten): von der offenen Seite sieht man in den Hohlkörper
    /// statt auf eine flache Scheibe.
    pub fn frustum_open(&mut self, p: &Paint, a: Vec3, b: Vec3, ra: f32, rb: f32, seg: u32) {
        self.frustum_caps(p, a, b, ra, rb, seg, false);
    }

    #[allow(clippy::too_many_arguments)]
    fn frustum_caps(&mut self, p: &Paint, a: Vec3, b: Vec3, ra: f32, rb: f32, seg: u32, caps: bool) {
        let axis = (b - a).normalize_or(Vec3::Y);
        let rot = Quat::from_rotation_arc(Vec3::Y, axis);
        let ring = |r: f32, base: Vec3, i: u32| {
            let ang = i as f32 / seg as f32 * std::f32::consts::TAU;
            base + rot * Vec3::new(ang.cos() * r, 0.0, ang.sin() * r)
        };
        let slope = (ra - rb) / (b - a).length().max(1e-4);
        for i in 0..seg {
            let (a0, a1) = (ring(ra, a, i), ring(ra, a, i + 1));
            let (b0, b1) = (ring(rb, b, i), ring(rb, b, i + 1));
            let n = |j: u32| {
                let ang = j as f32 / seg as f32 * std::f32::consts::TAU;
                (rot * Vec3::new(ang.cos(), slope, ang.sin())).normalize()
            };
            let (n0, n1) = (n(i), n(i + 1));
            if rb > 1e-4 {
                self.tri(p, a0, b1, b0, n0, n1, n0);
                self.tri(p, a0, a1, b1, n0, n1, n1);
            } else {
                self.tri(p, a0, a1, b, n0, n1, (n0 + n1) * 0.5);
            }
            // Deckel.
            if caps {
                self.tri_flat(p, a, a1, a0);
                if rb > 1e-4 {
                    self.tri_flat(p, b, b0, b1);
                }
            }
        }
    }

    /// Ellipsoid (UV-Kugel) mit Mittelpunkt und Radien.
    pub fn ellipsoid(&mut self, p: &Paint, c: Vec3, r: Vec3, seg: u32, rings: u32) {
        let pt = |i: u32, j: u32| {
            let th = i as f32 / seg as f32 * std::f32::consts::TAU;
            let ph = j as f32 / rings as f32 * std::f32::consts::PI;
            let n = Vec3::new(ph.sin() * th.cos(), ph.cos(), ph.sin() * th.sin());
            (c + n * r, (n / r).normalize())
        };
        for j in 0..rings {
            for i in 0..seg {
                let (p00, n00) = pt(i, j);
                let (p10, n10) = pt(i + 1, j);
                let (p01, n01) = pt(i, j + 1);
                let (p11, n11) = pt(i + 1, j + 1);
                if j > 0 {
                    self.tri(p, p00, p10, p11, n00, n10, n11);
                }
                if j + 1 < rings {
                    self.tri(p, p00, p11, p01, n00, n11, n01);
                }
            }
        }
    }

    /// Prisma eines konvexen Polygons (in XZ, gegen den Uhrzeigersinn von oben) von y0 bis y1.
    pub fn prism(&mut self, p: &Paint, poly: &[(f32, f32)], y0: f32, y1: f32, t: Mat4) {
        let fp = Paint { flat: true, ..*p };
        let n = poly.len();
        let lo: Vec<Vec3> = poly.iter().map(|&(x, z)| t.transform_point3(Vec3::new(x, y0, z))).collect();
        let hi: Vec<Vec3> = poly.iter().map(|&(x, z)| t.transform_point3(Vec3::new(x, y1, z))).collect();
        for i in 0..n {
            let j = (i + 1) % n;
            self.quad(&fp, lo[j], lo[i], hi[i], hi[j]);
        }
        for i in 1..n - 1 {
            self.tri_flat(&fp, hi[0], hi[i + 1], hi[i]);
            self.tri_flat(&fp, lo[0], lo[i], lo[i + 1]);
        }
    }

    pub fn append(&mut self, other: &Mesh) {
        self.verts.extend_from_slice(&other.verts);
    }

    pub fn triangles(&self) -> usize {
        self.verts.len() / 3
    }
}
