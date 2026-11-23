//! 3D-Modelle für Gebäude, Ressourcenquellen, Bäume und Dekoration.
//!
//! Gebäude liegen im Kachelraum: Modell-X = Kachel-x (Bildschirm rechts unten), Modell-Z = Kachel-y
//! (links unten), Ursprung = obere Ecke der Grundfläche. Die Bake-Kamera dreht dafür um −45°.

use ashen_engine::bake::{Mat, Mesh, Paint};
use glam::{Mat4, Quat, Vec2, Vec3};
use std::f32::consts::{FRAC_PI_4, TAU};

pub struct Structure {
    pub name: String,
    pub mesh: Mesh,
    pub yaw: f32,
    pub cell: (u32, u32),
    pub anchor: Vec2,
    /// Effektpunkte im Modellraum: Lichter (Position, Farbe, Radius in Pixeln), Rauch, Feuer, Hohlglühen.
    pub lights: Vec<(Vec3, Vec3, f32)>,
    pub smoke: Vec<Vec3>,
    pub fires: Vec<Vec3>,
    pub glow: Vec<Vec3>,
}

pub(super) fn pt(hex: u32, mat: Mat) -> Paint {
    Paint::new(hex, mat)
}

pub(super) const STONE: u32 = 0x5a5a60;
pub(super) const DARK_STONE: u32 = 0x3a3a40;
pub(super) const ROOF: u32 = 0x7a2a24;
pub(super) const WOOD: u32 = 0x5f3a22;
pub(super) const DARK_WOOD: u32 = 0x2c1c13;
pub(super) const WARM: u32 = 0xf2b25a;
pub(super) const CRIMSON: u32 = 0x8f2220;
pub(super) const GOLD: u32 = 0xcfa23f;
pub(super) const HOLLOW: u32 = 0x86f2a0;

/// Gitterausgerichtetes Gebäude: Zelle und Anker aus Grundfläche und Höhe.
pub(super) fn grid(name: &str, mesh: Mesh, w: f32, h: f32, height: f32) -> Structure {
    let margin = 6.0;
    let cw = ((w + h) * 16.0 + margin * 2.0).ceil() as u32;
    let top = height * 0.866 * ashen_engine::bake::PPU + margin;
    let ch = (top + (w + h) * 8.0 + margin).ceil() as u32;
    Structure {
        name: name.into(),
        mesh,
        yaw: -FRAC_PI_4,
        cell: (cw, ch),
        anchor: Vec2::new(h * 16.0 + margin, top),
        lights: Vec::new(),
        smoke: Vec::new(),
        fires: Vec::new(),
        glow: Vec::new(),
    }
}

/// Freistehendes Objekt (Baum, Fels) mit Fußpunkt unten mittig.
pub(super) fn free(name: &str, mesh: Mesh, yaw: f32, w: u32, h: u32) -> Structure {
    Structure {
        name: name.into(),
        mesh,
        yaw,
        cell: (w, h),
        anchor: Vec2::new((w / 2) as f32, (h - 6) as f32),
        lights: Vec::new(),
        smoke: Vec::new(),
        fires: Vec::new(),
        glow: Vec::new(),
    }
}

/// Quader in Kachelkoordinaten (x0,z0)-(x1,z1), Höhe y0..y1.
#[allow(clippy::too_many_arguments)]
pub(super) fn block(m: &mut Mesh, p: &Paint, x0: f32, z0: f32, x1: f32, z1: f32, y0: f32, y1: f32) {
    m.cuboid(
        p,
        Vec3::new(x0, y0, z0),
        Vec3::new(x1, y1, z1),
        Mat4::IDENTITY,
    );
}

/// Zinnen entlang einer Kante.
pub(super) fn merlons(m: &mut Mesh, p: &Paint, a: Vec2, b: Vec2, y: f32, size: f32) {
    let n = ((b - a).length() / (size * 2.0)).floor().max(1.0) as i32;
    for i in 0..=n {
        let c = a.lerp(b, i as f32 / n as f32);
        block(
            m,
            p,
            c.x - size * 0.5,
            c.y - size * 0.5,
            c.x + size * 0.5,
            c.y + size * 0.5,
            y,
            y + size * 1.1,
        );
    }
}

/// Satteldach über einem Rechteck, First entlang X (along_x) oder Z.
#[allow(clippy::too_many_arguments)]
pub(super) fn gable(
    m: &mut Mesh,
    p: &Paint,
    x0: f32,
    z0: f32,
    x1: f32,
    z1: f32,
    y: f32,
    rise: f32,
    along_x: bool,
    over: f32,
) {
    let (x0, z0, x1, z1) = (x0 - over, z0 - over, x1 + over, z1 + over);
    if along_x {
        let zm = (z0 + z1) * 0.5;
        let ridge_a = Vec3::new(x0, y + rise, zm);
        let ridge_b = Vec3::new(x1, y + rise, zm);
        m.quad(
            p,
            Vec3::new(x0, y, z1),
            Vec3::new(x1, y, z1),
            ridge_b,
            ridge_a,
        );
        m.quad(
            p,
            Vec3::new(x1, y, z0),
            Vec3::new(x0, y, z0),
            ridge_a,
            ridge_b,
        );
        let gp = pt(0x3a261a, Mat::WoodPlank);
        m.tri_flat(&gp, Vec3::new(x0, y, z0), Vec3::new(x0, y, z1), ridge_a);
        m.tri_flat(&gp, Vec3::new(x1, y, z1), Vec3::new(x1, y, z0), ridge_b);
    } else {
        let xm = (x0 + x1) * 0.5;
        let ra = Vec3::new(xm, y + rise, z0);
        let rb = Vec3::new(xm, y + rise, z1);
        m.quad(p, Vec3::new(x1, y, z0), Vec3::new(x1, y, z1), rb, ra);
        m.quad(p, Vec3::new(x0, y, z1), Vec3::new(x0, y, z0), ra, rb);
        let gp = pt(0x3a261a, Mat::WoodPlank);
        m.tri_flat(&gp, Vec3::new(x1, y, z1), Vec3::new(x0, y, z1), rb);
        m.tri_flat(&gp, Vec3::new(x0, y, z0), Vec3::new(x1, y, z0), ra);
    }
}

/// Pyramidendach (quadratisch) über einem Rechteck.
#[allow(clippy::too_many_arguments)]
pub(super) fn pyramid(
    m: &mut Mesh,
    p: &Paint,
    x0: f32,
    z0: f32,
    x1: f32,
    z1: f32,
    y: f32,
    rise: f32,
) {
    let apex = Vec3::new((x0 + x1) * 0.5, y + rise, (z0 + z1) * 0.5);
    let c = [
        Vec3::new(x0, y, z0),
        Vec3::new(x1, y, z0),
        Vec3::new(x1, y, z1),
        Vec3::new(x0, y, z1),
    ];
    for i in 0..4 {
        m.tri_flat(p, c[(i + 1) % 4], c[i], apex);
    }
}

pub(super) fn tower(m: &mut Mesh, x: f32, z: f32, r: f32, h: f32) {
    m.frustum(
        &pt(STONE, Mat::StoneBrick),
        Vec3::new(x, 0.0, z),
        Vec3::new(x, h, z),
        r * 1.05,
        r,
        10,
    );
    m.frustum(
        &pt(STONE, Mat::StoneBrick),
        Vec3::new(x, h, z),
        Vec3::new(x, h + 0.08, z),
        r * 1.15,
        r * 1.15,
        10,
    );
    m.frustum(
        &pt(ROOF, Mat::RoofTile).flat(),
        Vec3::new(x, h + 0.08, z),
        Vec3::new(x, h + r * 2.4, z),
        r * 1.3,
        0.0,
        10,
    );
    m.frustum(
        &pt(GOLD, Mat::Gold),
        Vec3::new(x, h + r * 2.4, z),
        Vec3::new(x, h + r * 2.4 + 0.18, z),
        0.02,
        0.0,
        4,
    );
}

pub(super) fn window(m: &mut Mesh, center: Vec3, normal_axis_x: bool, w: f32, h: f32) {
    let glow = pt(WARM, Mat::Glow).glow(1.0);
    let (hw, hh) = (w * 0.5, h * 0.5);
    if normal_axis_x {
        block(
            m,
            &glow,
            center.x - 0.01,
            center.z - hw,
            center.x + 0.02,
            center.z + hw,
            center.y - hh,
            center.y + hh,
        );
    } else {
        block(
            m,
            &glow,
            center.x - hw,
            center.z - 0.01,
            center.x + hw,
            center.z + 0.02,
            center.y - hh,
            center.y + hh,
        );
    }
}

pub(super) fn banner(m: &mut Mesh, top: Vec3, along_x: bool, len: f32) {
    let cloth = pt(CRIMSON, Mat::Cloth);
    let w = 0.18;
    if along_x {
        block(
            m,
            &cloth,
            top.x - 0.01,
            top.z - w,
            top.x + 0.03,
            top.z + w,
            top.y - len,
            top.y,
        );
        block(
            m,
            &pt(GOLD, Mat::Gold),
            top.x + 0.02,
            top.z - 0.05,
            top.x + 0.04,
            top.z + 0.05,
            top.y - len * 0.55,
            top.y - len * 0.3,
        );
    } else {
        block(
            m,
            &cloth,
            top.x - w,
            top.z - 0.01,
            top.x + w,
            top.z + 0.03,
            top.y - len,
            top.y,
        );
        block(
            m,
            &pt(GOLD, Mat::Gold),
            top.x - 0.05,
            top.z + 0.02,
            top.x + 0.05,
            top.z + 0.04,
            top.y - len * 0.55,
            top.y - len * 0.3,
        );
    }
}

pub(super) fn keep_variant(castle: bool) -> Structure {
    // Ausbaustufe Burg: höhere Mauern, Türme und Bergfried.
    let k = if castle { 1.0 } else { 0.0 };
    let dh = 2.35 + 0.6 * k;
    let mut m = Mesh::default();
    let brick = pt(STONE, Mat::StoneBrick);
    block(
        &mut m,
        &pt(0x4c4a43, Mat::Rock),
        0.05,
        0.05,
        2.95,
        2.95,
        0.0,
        0.12,
    );
    let (lo, hi, t, wh) = (0.22, 2.78, 0.2, 0.85 + 0.2 * k);
    // Ringmauer.
    block(&mut m, &brick, lo, lo, hi, lo + t, 0.12, wh);
    block(&mut m, &brick, lo, lo, lo + t, hi, 0.12, wh);
    block(&mut m, &brick, lo, hi - t, hi, hi, 0.12, wh);
    block(&mut m, &brick, hi - t, lo, hi, hi, 0.12, wh);
    for (a, b) in [
        ((lo, lo + 0.1), (hi, lo + 0.1)),
        ((lo + 0.1, lo), (lo + 0.1, hi)),
        ((lo, hi - 0.1), (hi, hi - 0.1)),
        ((hi - 0.1, lo), (hi - 0.1, hi)),
    ] {
        merlons(
            &mut m,
            &brick,
            Vec2::new(a.0, a.1),
            Vec2::new(b.0, b.1),
            wh,
            0.13,
        );
    }
    for (x, z) in [(lo, lo), (hi, lo), (lo, hi), (hi, hi)] {
        tower(&mut m, x, z, 0.34 + 0.04 * k, 1.45 + 0.5 * k);
    }
    // Donjon.
    block(&mut m, &brick, 0.85, 0.85, 2.15, 2.15, 0.12, dh);
    for (a, b) in [
        ((0.85, 2.12), (2.15, 2.12)),
        ((2.12, 0.85), (2.12, 2.15)),
        ((0.85, 0.88), (2.15, 0.88)),
        ((0.88, 0.85), (0.88, 2.15)),
    ] {
        merlons(
            &mut m,
            &brick,
            Vec2::new(a.0, a.1),
            Vec2::new(b.0, b.1),
            dh,
            0.14,
        );
    }
    pyramid(
        &mut m,
        &pt(ROOF, Mat::RoofTile),
        1.0,
        1.0,
        2.0,
        2.0,
        dh,
        1.1,
    );
    m.frustum(
        &pt(GOLD, Mat::Gold),
        Vec3::new(1.5, dh + 1.1, 1.5),
        Vec3::new(1.5, dh + 1.4, 1.5),
        0.025,
        0.0,
        4,
    );
    for y in [1.0, 1.7] {
        window(&mut m, Vec3::new(2.15, y, 1.25), true, 0.12, 0.22);
        window(&mut m, Vec3::new(2.15, y + 0.1, 1.75), true, 0.12, 0.22);
        window(&mut m, Vec3::new(1.3, y + 0.05, 2.15), false, 0.12, 0.22);
        window(&mut m, Vec3::new(1.75, y, 2.15), false, 0.12, 0.22);
    }
    banner(&mut m, Vec3::new(2.16, dh - 0.1, 1.5), true, 0.75);
    banner(&mut m, Vec3::new(1.5, dh - 0.1, 2.16), false, 0.75);
    // Torhaus in der rechten Frontmauer.
    block(
        &mut m,
        &pt(0x0d0b0a, Mat::Plain),
        hi - 0.02,
        1.25,
        hi + 0.01,
        1.75,
        0.12,
        0.62,
    );
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        hi - 0.04,
        1.3,
        hi - 0.01,
        1.7,
        0.12,
        0.55,
    );
    for z in [1.18, 1.82] {
        block(
            &mut m,
            &pt(DARK_WOOD, Mat::WoodPlank),
            hi + 0.01,
            z - 0.02,
            hi + 0.05,
            z + 0.02,
            0.5,
            0.68,
        );
        m.ellipsoid(
            &pt(0xffc24a, Mat::Glow).glow(1.0),
            Vec3::new(hi + 0.04, 0.73, z),
            Vec3::splat(0.045),
            5,
            3,
        );
    }
    let mut s = grid(
        if castle {
            "bld_keep_castle"
        } else {
            "bld_keep"
        },
        m,
        3.0,
        3.0,
        4.0 + 0.6 * k,
    );
    s.lights = vec![
        (
            Vec3::new(hi + 0.06, 0.73, 1.18),
            Vec3::new(1.0, 0.6, 0.25),
            70.0,
        ),
        (
            Vec3::new(hi + 0.06, 0.73, 1.82),
            Vec3::new(1.0, 0.6, 0.25),
            70.0,
        ),
        (Vec3::new(2.2, 1.4, 1.5), Vec3::new(1.0, 0.7, 0.35), 46.0),
        (Vec3::new(1.5, 1.4, 2.2), Vec3::new(1.0, 0.7, 0.35), 46.0),
    ];
    s.fires = vec![
        Vec3::new(hi + 0.04, 0.76, 1.18),
        Vec3::new(hi + 0.04, 0.76, 1.82),
    ];
    s.smoke = vec![Vec3::new(0.6, 1.9, 0.6)];
    s
}

fn keep() -> Structure {
    keep_variant(false)
}

pub(super) fn timber_frame(m: &mut Mesh, x0: f32, z0: f32, x1: f32, z1: f32, y0: f32, y1: f32) {
    let beam = pt(DARK_WOOD, Mat::WoodPlank);
    let s = 0.035;
    for (x, z) in [(x0, z0), (x1, z0), (x0, z1), (x1, z1)] {
        block(m, &beam, x - s, z - s, x + s, z + s, y0, y1);
    }
    for y in [y0 + 0.02, (y0 + y1) * 0.5, y1 - 0.03] {
        block(m, &beam, x0 - s, z1 - s, x1 + s, z1 + s, y - s, y + s);
        block(m, &beam, x1 - s, z0 - s, x1 + s, z1 + s, y - s, y + s);
    }
    // Diagonale Streben an den Sichtseiten.
    let mid_x = (x0 + x1) * 0.5;
    let rot = Quat::from_rotation_z(0.75);
    m.cuboid(
        &beam,
        Vec3::new(-0.03, -0.25, -0.03),
        Vec3::new(0.03, 0.25, 0.03),
        Mat4::from_rotation_translation(rot, Vec3::new(mid_x, (y0 + y1) * 0.5, z1 + 0.01)),
    );
}

fn farmstead() -> Structure {
    let mut m = Mesh::default();
    // Felder.
    let crops = pt(0x9a8040, Mat::Crops);
    block(
        &mut m,
        &pt(0x3a2b20, Mat::Plain),
        0.04,
        0.04,
        1.96,
        1.96,
        0.0,
        0.02,
    );
    block(&mut m, &crops, 1.15, 0.08, 1.92, 1.92, 0.02, 0.1);
    block(&mut m, &crops, 0.08, 1.1, 1.15, 1.92, 0.02, 0.1);
    // Haus.
    block(
        &mut m,
        &pt(0x8a7e68, Mat::Plaster),
        0.15,
        0.12,
        1.05,
        0.98,
        0.0,
        0.62,
    );
    timber_frame(&mut m, 0.15, 0.12, 1.05, 0.98, 0.0, 0.62);
    gable(
        &mut m,
        &pt(0x8a6f36, Mat::Thatch),
        0.15,
        0.12,
        1.05,
        0.98,
        0.62,
        0.55,
        true,
        0.1,
    );
    block(
        &mut m,
        &pt(DARK_STONE, Mat::StoneBrick),
        0.3,
        0.3,
        0.45,
        0.45,
        0.6,
        1.35,
    );
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        1.04,
        0.45,
        1.07,
        0.68,
        0.0,
        0.38,
    );
    window(&mut m, Vec3::new(0.6, 0.35, 0.99), false, 0.14, 0.12);
    window(&mut m, Vec3::new(1.06, 0.38, 0.25), true, 0.12, 0.12);
    // Zaun.
    let fence = pt(WOOD, Mat::WoodPlank);
    for i in 0..10 {
        let z = 0.1 + i as f32 * 0.2;
        block(&mut m, &fence, 1.92, z - 0.025, 1.97, z + 0.025, 0.0, 0.26);
    }
    block(&mut m, &fence, 1.93, 0.08, 1.96, 1.92, 0.16, 0.2);
    m.ellipsoid(
        &pt(0x8a6f36, Mat::Thatch),
        Vec3::new(1.5, 0.14, 1.5),
        Vec3::new(0.16, 0.14, 0.16),
        7,
        4,
    );
    let mut s = grid("bld_farmstead", m, 2.0, 2.0, 1.6);
    s.lights = vec![(Vec3::new(0.6, 0.35, 1.05), Vec3::new(1.0, 0.7, 0.35), 34.0)];
    s.smoke = vec![Vec3::new(0.38, 1.4, 0.38)];
    s
}

fn barracks() -> Structure {
    let mut m = Mesh::default();
    block(
        &mut m,
        &pt(0x4c4a43, Mat::Rock),
        0.08,
        0.08,
        2.92,
        1.92,
        0.0,
        0.1,
    );
    block(
        &mut m,
        &pt(STONE, Mat::StoneBrick),
        0.25,
        0.3,
        2.7,
        1.6,
        0.1,
        0.5,
    );
    block(
        &mut m,
        &pt(0x8a7e68, Mat::Plaster),
        0.27,
        0.32,
        2.68,
        1.58,
        0.5,
        1.0,
    );
    timber_frame(&mut m, 0.27, 0.32, 2.68, 1.58, 0.5, 1.0);
    gable(
        &mut m,
        &pt(ROOF, Mat::RoofTile),
        0.27,
        0.32,
        2.68,
        1.58,
        1.0,
        0.75,
        true,
        0.12,
    );
    // Tor auf der Längsseite.
    block(
        &mut m,
        &pt(0x0d0b0a, Mat::Plain),
        1.15,
        1.59,
        1.6,
        1.61,
        0.1,
        0.72,
    );
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        1.18,
        1.6,
        1.57,
        1.63,
        0.1,
        0.66,
    );
    for x in [0.6, 2.1] {
        window(&mut m, Vec3::new(x, 0.78, 1.59), false, 0.14, 0.16);
    }
    window(&mut m, Vec3::new(2.69, 0.8, 0.95), true, 0.14, 0.16);
    // Waffenständer.
    for i in 0..4 {
        let x = 0.45 + i as f32 * 0.1;
        m.frustum(
            &pt(WOOD, Mat::WoodPlank),
            Vec3::new(x, 0.1, 1.8),
            Vec3::new(x, 0.85, 1.75),
            0.018,
            0.015,
            4,
        );
        m.frustum(
            &pt(0x8b969e, Mat::Metal),
            Vec3::new(x, 0.85, 1.75),
            Vec3::new(x, 0.97, 1.75),
            0.03,
            0.0,
            4,
        );
    }
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        0.4,
        1.74,
        0.82,
        1.82,
        0.4,
        0.45,
    );
    // Übungspuppe.
    m.frustum(
        &pt(WOOD, Mat::WoodPlank),
        Vec3::new(2.5, 0.1, 1.8),
        Vec3::new(2.5, 0.75, 1.8),
        0.03,
        0.03,
        5,
    );
    m.ellipsoid(
        &pt(0x8a6f36, Mat::Thatch),
        Vec3::new(2.5, 0.65, 1.8),
        Vec3::new(0.11, 0.15, 0.09),
        6,
        4,
    );
    block(
        &mut m,
        &pt(WOOD, Mat::WoodPlank),
        2.3,
        1.78,
        2.7,
        1.82,
        0.6,
        0.64,
    );
    m.frustum(
        &pt(DARK_WOOD, Mat::WoodPlank),
        Vec3::new(2.78, 0.1, 1.68),
        Vec3::new(2.78, 1.6, 1.68),
        0.025,
        0.02,
        4,
    );
    banner(&mut m, Vec3::new(2.8, 1.55, 1.68), true, 0.6);
    m.ellipsoid(
        &pt(0xffc24a, Mat::Glow).glow(1.0),
        Vec3::new(1.08, 0.68, 1.66),
        Vec3::splat(0.045),
        5,
        3,
    );
    let mut s = grid("bld_barracks", m, 3.0, 2.0, 2.2);
    s.lights = vec![(Vec3::new(1.08, 0.68, 1.72), Vec3::new(1.0, 0.6, 0.25), 56.0)];
    s.fires = vec![Vec3::new(1.08, 0.7, 1.68)];
    s.smoke = vec![Vec3::new(2.2, 1.7, 0.9)];
    s
}

fn watchtower() -> Structure {
    let mut m = Mesh::default();
    block(
        &mut m,
        &pt(STONE, Mat::StoneBrick),
        0.1,
        0.1,
        0.9,
        0.9,
        0.0,
        0.3,
    );
    let post = pt(WOOD, Mat::WoodPlank);
    for (x, z) in [(0.2, 0.2), (0.8, 0.2), (0.2, 0.8), (0.8, 0.8)] {
        m.frustum(
            &post,
            Vec3::new(x, 0.3, z),
            Vec3::new(x, 2.0, z),
            0.045,
            0.04,
            5,
        );
    }
    for y in [0.7, 1.3] {
        let rot = Quat::from_rotation_x(0.8);
        m.cuboid(
            &post,
            Vec3::new(-0.025, -0.4, -0.025),
            Vec3::new(0.025, 0.4, 0.025),
            Mat4::from_rotation_translation(rot, Vec3::new(0.8, y, 0.5)),
        );
        let rot = Quat::from_rotation_z(-0.8);
        m.cuboid(
            &post,
            Vec3::new(-0.025, -0.4, -0.025),
            Vec3::new(0.025, 0.4, 0.025),
            Mat4::from_rotation_translation(rot, Vec3::new(0.5, y, 0.8)),
        );
    }
    block(&mut m, &post, 0.0, 0.0, 1.0, 1.0, 2.0, 2.1);
    for (a, b) in [
        ((0.0, 0.97), (1.0, 1.0)),
        ((0.97, 0.0), (1.0, 1.0)),
        ((0.0, 0.0), (1.0, 0.03)),
        ((0.0, 0.0), (0.03, 1.0)),
    ] {
        block(&mut m, &post, a.0, a.1, b.0, b.1, 2.1, 2.35);
    }
    m.frustum(
        &pt(0x2c2f34, Mat::Metal),
        Vec3::new(0.5, 2.1, 0.5),
        Vec3::new(0.5, 2.28, 0.5),
        0.12,
        0.2,
        8,
    );
    for i in 0..5 {
        let a = i as f32 / 5.0 * TAU;
        let base = Vec3::new(0.5 + a.cos() * 0.08, 2.28, 0.5 + a.sin() * 0.08);
        m.frustum(
            &pt(0xffa030, Mat::Glow).glow(1.0),
            base,
            base + Vec3::new(0.0, 0.22 + (i % 2) as f32 * 0.1, 0.0),
            0.06,
            0.0,
            4,
        );
    }
    let mut s = grid("bld_watchtower", m, 1.0, 1.0, 3.0);
    s.lights = vec![(Vec3::new(0.5, 2.45, 0.5), Vec3::new(1.0, 0.55, 0.2), 100.0)];
    s.fires = vec![Vec3::new(0.5, 2.4, 0.5)];
    s
}

fn hollow_shrine() -> Structure {
    let mut m = Mesh::default();
    let dark = pt(0x56505a, Mat::StoneBrick);
    block(
        &mut m,
        &pt(0x3e3a40, Mat::Rock),
        0.05,
        0.05,
        2.95,
        2.95,
        0.0,
        0.1,
    );
    let heights = [1.6, 0.9, 1.9, 0.5, 1.3, 0.8];
    for (i, h) in heights.iter().enumerate() {
        let f = 0.2 + i as f32 * 0.43;
        block(&mut m, &dark, f, 0.15, f + 0.4, 0.38, 0.1, *h);
        let h2 = heights[(i + 3) % 6] * 0.9;
        block(&mut m, &dark, 0.15, f, 0.38, f + 0.4, 0.1, h2);
    }
    // Spitzbogen-Rest.
    for i in 0..8 {
        let a = i as f32 / 7.0 * std::f32::consts::PI;
        let c = Vec3::new(0.27, 1.55 + a.sin() * 0.45, 1.2 + a.cos() * 0.45);
        block(
            &mut m,
            &dark,
            c.x - 0.1,
            c.z - 0.07,
            c.x + 0.1,
            c.z + 0.07,
            c.y - 0.07,
            c.y + 0.07,
        );
    }
    // Obelisk mit Riss.
    m.frustum(
        &pt(0x3a3540, Mat::StoneBrick).flat(),
        Vec3::new(1.5, 0.1, 1.5),
        Vec3::new(1.5, 2.6, 1.5),
        0.42,
        0.25,
        4,
    );
    m.frustum(
        &pt(0x3a3540, Mat::StoneBrick).flat(),
        Vec3::new(1.5, 2.6, 1.5),
        Vec3::new(1.5, 3.1, 1.5),
        0.25,
        0.0,
        4,
    );
    let rift = pt(HOLLOW, Mat::Glow).glow(1.0);
    for k in 0..10 {
        let y = 0.4 + k as f32 * 0.22;
        let wob = ((k as f32) * 1.7).sin() * 0.05;
        let w = if (2..8).contains(&k) { 0.05 } else { 0.025 };
        block(
            &mut m,
            &rift,
            1.5 + wob - w,
            1.79,
            1.5 + wob + w,
            1.86,
            y,
            y + 0.23,
        );
        block(
            &mut m,
            &rift,
            1.79,
            1.5 - wob - w,
            1.86,
            1.5 - wob + w,
            y + 0.1,
            y + 0.3,
        );
    }
    // Knochen und Kerzen.
    for i in 0..16 {
        let a = i as f32 * 2.39;
        let r = 0.7 + (i % 4) as f32 * 0.18;
        let c = Vec3::new(1.5 + a.cos() * r, 0.1, 1.5 + a.sin() * r);
        if i % 3 == 0 {
            m.frustum(
                &pt(0xe8d8a0, Mat::Plain),
                c,
                c + Vec3::new(0.0, 0.12, 0.0),
                0.025,
                0.025,
                4,
            );
            m.ellipsoid(
                &pt(0xffc24a, Mat::Glow).glow(1.0),
                c + Vec3::new(0.0, 0.15, 0.0),
                Vec3::splat(0.025),
                4,
                3,
            );
        } else {
            m.cuboid(
                &pt(0xc9c6ad, Mat::Bone),
                Vec3::new(-0.12, 0.0, -0.02),
                Vec3::new(0.12, 0.04, 0.02),
                Mat4::from_rotation_translation(Quat::from_rotation_y(a), c),
            );
        }
    }
    let mut s = grid("bld_hollow_shrine", m, 3.0, 3.0, 3.4);
    s.lights = vec![(Vec3::new(1.85, 1.3, 1.85), Vec3::new(0.45, 1.0, 0.6), 140.0)];
    s.glow = (0..6)
        .map(|k| Vec3::new(1.85, 0.5 + k as f32 * 0.4, 1.6))
        .collect();
    s
}

fn bone_spire() -> Structure {
    let mut m = Mesh::default();
    let bone = pt(0x837b60, Mat::Bone);
    for i in 0..7 {
        let y = i as f32 * 0.32;
        let r = 0.34 - i as f32 * 0.04;
        m.ellipsoid(
            &bone,
            Vec3::new(0.5, y + 0.16, 0.5),
            Vec3::new(r, 0.2, r),
            7,
            4,
        );
        if i % 2 == 0 && i < 6 {
            for k in 0..4 {
                let a = k as f32 / 4.0 * TAU + i as f32;
                let base = Vec3::new(0.5 + a.cos() * r, y + 0.2, 0.5 + a.sin() * r);
                m.frustum(
                    &bone,
                    base,
                    base + Vec3::new(a.cos() * 0.3, 0.25, a.sin() * 0.3),
                    0.04,
                    0.0,
                    4,
                );
            }
        }
    }
    m.ellipsoid(
        &pt(HOLLOW, Mat::Glow).glow(1.0),
        Vec3::new(0.5, 2.5, 0.5),
        Vec3::splat(0.13),
        7,
        4,
    );
    let mut s = grid("bld_bone_spire", m, 1.0, 1.0, 3.0);
    s.lights = vec![(Vec3::new(0.5, 2.5, 0.5), Vec3::new(0.45, 1.0, 0.6), 72.0)];
    s.glow = vec![Vec3::new(0.5, 2.5, 0.5)];
    s
}

fn pine(seed: u32) -> Mesh {
    let mut m = Mesh::default();
    let v = seed as f32;
    m.frustum(
        &pt(0x3f2e21, Mat::Bark),
        Vec3::ZERO,
        Vec3::new(0.0, 0.6, 0.0),
        0.09,
        0.06,
        6,
    );
    let tiers = 4 + seed % 2;
    for i in 0..tiers {
        let y = 0.35 + i as f32 * 0.42;
        let r = 0.62 - i as f32 * 0.11 + (v * 1.3 + i as f32).sin() * 0.04;
        let col = if i % 2 == 0 { 0x22382a } else { 0x2b4430 };
        m.frustum(
            &pt(col, Mat::Leaves),
            Vec3::new(0.0, y, 0.0),
            Vec3::new((v + i as f32).sin() * 0.03, y + 0.62, 0.0),
            r,
            0.0,
            8,
        );
    }
    m
}

fn oak(seed: u32) -> Mesh {
    let mut m = Mesh::default();
    let v = seed as f32;
    m.frustum(
        &pt(0x3f2e21, Mat::Bark),
        Vec3::ZERO,
        Vec3::new(0.0, 0.9, 0.0),
        0.12,
        0.08,
        6,
    );
    m.frustum(
        &pt(0x3f2e21, Mat::Bark),
        Vec3::new(0.0, 0.7, 0.0),
        Vec3::new(0.35, 1.1, 0.1),
        0.06,
        0.03,
        5,
    );
    m.frustum(
        &pt(0x3f2e21, Mat::Bark),
        Vec3::new(0.0, 0.75, 0.0),
        Vec3::new(-0.3, 1.15, -0.1),
        0.06,
        0.03,
        5,
    );
    let blobs = [
        (0.0, 1.35, 0.0, 0.55),
        (0.35, 1.15, 0.15, 0.42),
        (-0.35, 1.2, -0.1, 0.42),
        (0.05, 1.05, 0.38, 0.38),
        (-0.1, 1.1, -0.35, 0.38),
        (0.1, 1.7, 0.05, 0.35),
    ];
    for (i, (x, y, z, r)) in blobs.iter().enumerate() {
        let col = [0x3c5023, 0x2e3f1b, 0x4d622d][(i + seed as usize) % 3];
        m.ellipsoid(
            &pt(col, Mat::Leaves),
            Vec3::new(*x, *y + (v + i as f32).sin() * 0.04, *z),
            Vec3::splat(*r),
            8,
            5,
        );
    }
    m
}

fn dead_tree(seed: u32) -> Mesh {
    let mut m = Mesh::default();
    let bark = pt(0x2a2628, Mat::Bark);
    m.frustum(&bark, Vec3::ZERO, Vec3::new(0.05, 1.1, 0.0), 0.11, 0.06, 6);
    let v = seed as f32;
    for i in 0..5 {
        let a = i as f32 * 2.1 + v;
        let y = 0.6 + i as f32 * 0.12;
        let tip = Vec3::new(a.cos() * 0.45, y + 0.45, a.sin() * 0.45);
        m.frustum(&bark, Vec3::new(0.03, y, 0.0), tip, 0.045, 0.015, 4);
        m.frustum(
            &bark,
            tip,
            tip + Vec3::new(a.cos() * 0.15, 0.2, a.sin() * -0.1),
            0.015,
            0.0,
            3,
        );
    }
    m
}

pub(super) fn rock_mound(m: &mut Mesh, center: Vec3, r: Vec3, seed: u32) {
    let rock = pt(0x625f56, Mat::Rock).flat();
    m.ellipsoid(&rock, center, r, 7 + seed % 3, 4);
}

fn gold_mine() -> Structure {
    let mut m = Mesh::default();
    rock_mound(
        &mut m,
        Vec3::new(1.0, 0.0, 0.95),
        Vec3::new(0.95, 0.7, 0.9),
        1,
    );
    rock_mound(
        &mut m,
        Vec3::new(0.55, 0.15, 0.6),
        Vec3::new(0.5, 0.55, 0.5),
        2,
    );
    rock_mound(
        &mut m,
        Vec3::new(1.4, 0.1, 0.55),
        Vec3::new(0.45, 0.5, 0.45),
        3,
    );
    for i in 0..14 {
        let a = i as f32 * 2.4;
        let c = Vec3::new(
            1.0 + a.cos() * 0.6,
            0.35 + (i % 4) as f32 * 0.1,
            0.95 + a.sin() * 0.55,
        );
        m.ellipsoid(
            &pt(0xeccb6c, Mat::Gold).glow(0.25),
            c,
            Vec3::splat(0.05),
            4,
            3,
        );
    }
    // Stolleneingang auf der rechten Seite.
    let w = pt(WOOD, Mat::WoodPlank);
    block(
        &mut m,
        &pt(0x080706, Mat::Plain),
        1.62,
        0.7,
        1.66,
        1.2,
        0.0,
        0.5,
    );
    block(&mut m, &w, 1.64, 0.62, 1.72, 0.7, 0.0, 0.58);
    block(&mut m, &w, 1.64, 1.2, 1.72, 1.28, 0.0, 0.58);
    block(&mut m, &w, 1.64, 0.6, 1.72, 1.3, 0.52, 0.6);
    m.ellipsoid(
        &pt(WARM, Mat::Glow).glow(1.0),
        Vec3::new(1.72, 0.62, 0.55),
        Vec3::splat(0.05),
        5,
        3,
    );
    // Lore.
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        1.75,
        1.4,
        1.95,
        1.75,
        0.08,
        0.28,
    );
    m.ellipsoid(
        &pt(0xeccb6c, Mat::Gold),
        Vec3::new(1.85, 0.28, 1.57),
        Vec3::new(0.09, 0.05, 0.14),
        5,
        3,
    );
    grid("node_gold_mine", m, 2.0, 2.0, 1.6)
}

fn quarry() -> Structure {
    let mut m = Mesh::default();
    for (i, (x, z, r)) in [
        (0.6, 0.6, 0.45),
        (1.25, 0.7, 0.55),
        (0.8, 1.3, 0.4),
        (1.45, 1.4, 0.35),
    ]
    .iter()
    .enumerate()
    {
        rock_mound(
            &mut m,
            Vec3::new(*x, 0.0, *z),
            Vec3::new(*r, *r * 0.9, *r),
            i as u32,
        );
    }
    let stone = pt(0x75757a, Mat::StoneBrick);
    block(&mut m, &stone, 1.6, 0.2, 1.85, 0.45, 0.0, 0.22);
    block(&mut m, &stone, 1.7, 0.5, 1.95, 0.72, 0.0, 0.2);
    block(&mut m, &stone, 1.65, 0.3, 1.88, 0.52, 0.22, 0.4);
    grid("node_quarry", m, 2.0, 2.0, 1.2)
}

fn decor() -> Vec<Structure> {
    let mut out = Vec::new();
    for v in 0..3u32 {
        let mut m = Mesh::default();
        rock_mound(
            &mut m,
            Vec3::ZERO,
            Vec3::new(0.16 + v as f32 * 0.03, 0.12, 0.13),
            v,
        );
        out.push(free(&format!("decor_rock_{v}"), m, v as f32, 20, 16));
        let mut m = Mesh::default();
        rock_mound(&mut m, Vec3::ZERO, Vec3::new(0.48, 0.42, 0.42), v + 5);
        rock_mound(
            &mut m,
            Vec3::new(0.25, 0.0, -0.2),
            Vec3::new(0.3, 0.3, 0.28),
            v + 7,
        );
        for k in 0..5 {
            m.ellipsoid(
                &pt(0x3f5428, Mat::Leaves),
                Vec3::new(-0.2 + k as f32 * 0.1, 0.36, 0.1),
                Vec3::new(0.08, 0.04, 0.08),
                5,
                3,
            );
        }
        out.push(free(
            &format!("decor_boulder_{v}"),
            m,
            v as f32 * 1.3,
            40,
            34,
        ));
        let mut m = Mesh::default();
        for k in 0..4 {
            let a = k as f32 * 1.6 + v as f32;
            m.ellipsoid(
                &pt([0x3c5023, 0x2e3f1b][k % 2], Mat::Leaves),
                Vec3::new(a.cos() * 0.12, 0.14, a.sin() * 0.1),
                Vec3::splat(0.17),
                6,
                4,
            );
        }
        out.push(free(&format!("decor_bush_{v}"), m, 0.0, 30, 24));
        let mut m = Mesh::default();
        if v % 2 == 0 {
            m.cuboid(
                &pt(0x5a5a60, Mat::Rock),
                Vec3::new(-0.12, 0.0, -0.04),
                Vec3::new(0.12, 0.42, 0.04),
                Mat4::from_rotation_z(0.08),
            );
        } else {
            m.cuboid(
                &pt(0x5a5a60, Mat::Rock),
                Vec3::new(-0.03, 0.0, -0.03),
                Vec3::new(0.03, 0.5, 0.03),
                Mat4::IDENTITY,
            );
            m.cuboid(
                &pt(0x5a5a60, Mat::Rock),
                Vec3::new(-0.15, 0.32, -0.03),
                Vec3::new(0.15, 0.38, 0.03),
                Mat4::IDENTITY,
            );
        }
        out.push(free(&format!("decor_grave_{v}"), m, 0.3 * v as f32, 22, 28));
        let mut m = Mesh::default();
        for k in 0..3 {
            m.cuboid(
                &pt(0xc9c6ad, Mat::Bone),
                Vec3::new(-0.13, 0.0, -0.02),
                Vec3::new(0.13, 0.035, 0.02),
                Mat4::from_rotation_translation(
                    Quat::from_rotation_y(k as f32 * 1.1 + v as f32),
                    Vec3::new(k as f32 * 0.05, 0.0, 0.0),
                ),
            );
        }
        if v != 1 {
            m.ellipsoid(
                &pt(0xc9c6ad, Mat::Bone),
                Vec3::new(0.1, 0.06, 0.06),
                Vec3::splat(0.07),
                6,
                4,
            );
        }
        out.push(free(&format!("decor_bones_{v}"), m, 0.0, 22, 14));
        out.push(free(
            &format!("tree_dead_{v}"),
            dead_tree(v),
            v as f32,
            50,
            70,
        ));
    }
    for v in 0..4u32 {
        out.push(free(
            &format!("tree_pine_{v}"),
            pine(v),
            v as f32 * 0.7,
            40,
            84,
        ));
        out.push(free(
            &format!("tree_oak_{v}"),
            oak(v),
            v as f32 * 0.9,
            56,
            78,
        ));
    }
    // Ruine, Säule, Kohlenbecken, Laterne, Zaun, Baumstumpf.
    let mut m = Mesh::default();
    for i in 0..6 {
        let h = [0.9, 0.6, 1.1, 0.4, 0.75, 0.3][i];
        block(
            &mut m,
            &pt(0x3a373a, Mat::StoneBrick),
            -0.5 + i as f32 * 0.17,
            -0.08,
            -0.33 + i as f32 * 0.17,
            0.08,
            0.0,
            h,
        );
    }
    out.push(free("decor_ruin", m, -FRAC_PI_4, 52, 52));
    let mut m = Mesh::default();
    m.frustum(
        &pt(0x4c4749, Mat::StoneBrick),
        Vec3::ZERO,
        Vec3::new(0.0, 0.1, 0.0),
        0.2,
        0.2,
        4,
    );
    m.frustum(
        &pt(0x5f595b, Mat::Rock),
        Vec3::new(0.0, 0.1, 0.0),
        Vec3::new(0.0, 1.15, 0.0),
        0.13,
        0.12,
        8,
    );
    out.push(free("decor_pillar", m, 0.4, 28, 50));
    let mut m = Mesh::default();
    m.frustum(
        &pt(0x2c2f34, Mat::Metal),
        Vec3::ZERO,
        Vec3::new(0.0, 0.45, 0.0),
        0.03,
        0.03,
        4,
    );
    m.frustum(
        &pt(0x2c2f34, Mat::Metal),
        Vec3::new(0.0, 0.45, 0.0),
        Vec3::new(0.0, 0.55, 0.0),
        0.07,
        0.13,
        7,
    );
    for i in 0..4 {
        let a = i as f32 / 4.0 * TAU;
        let b = Vec3::new(a.cos() * 0.05, 0.55, a.sin() * 0.05);
        m.frustum(
            &pt(0xffa030, Mat::Glow).glow(1.0),
            b,
            b + Vec3::new(0.0, 0.16, 0.0),
            0.05,
            0.0,
            4,
        );
    }
    out.push(free("decor_brazier", m, 0.0, 22, 34));
    let mut m = Mesh::default();
    m.frustum(
        &pt(DARK_WOOD, Mat::WoodPlank),
        Vec3::ZERO,
        Vec3::new(0.0, 0.9, 0.0),
        0.03,
        0.025,
        4,
    );
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        -0.02,
        -0.02,
        0.18,
        0.02,
        0.86,
        0.9,
    );
    m.ellipsoid(
        &pt(WARM, Mat::Glow).glow(1.0),
        Vec3::new(0.17, 0.78, 0.0),
        Vec3::new(0.05, 0.07, 0.05),
        5,
        3,
    );
    out.push(free("decor_lantern", m, 0.0, 22, 38));
    let mut m = Mesh::default();
    for i in 0..3 {
        block(
            &mut m,
            &pt(WOOD, Mat::WoodPlank),
            -0.03,
            i as f32 * 0.4 - 0.03,
            0.03,
            i as f32 * 0.4 + 0.03,
            0.0,
            0.32,
        );
    }
    block(
        &mut m,
        &pt(WOOD, Mat::WoodPlank),
        -0.015,
        0.0,
        0.015,
        0.8,
        0.18,
        0.23,
    );
    out.push(free("decor_fence", m, -FRAC_PI_4, 32, 24));
    let mut m = Mesh::default();
    m.frustum(
        &pt(0x3f2e21, Mat::Bark),
        Vec3::ZERO,
        Vec3::new(0.0, 0.12, 0.0),
        0.12,
        0.1,
        7,
    );
    m.frustum(
        &pt(0x8a6a48, Mat::WoodPlank),
        Vec3::new(0.0, 0.12, 0.0),
        Vec3::new(0.0, 0.125, 0.0),
        0.1,
        0.1,
        7,
    );
    out.push(free("decor_stump", m, 0.0, 20, 16));
    out
}

pub fn build_all() -> Vec<Structure> {
    let mut v = vec![
        keep(),
        farmstead(),
        barracks(),
        watchtower(),
        hollow_shrine(),
        bone_spire(),
        gold_mine(),
        quarry(),
    ];
    v.push(keep_variant(true));
    v.extend(super::city::build_all());
    v.extend(super::military::build_all());
    v.extend(decor());
    v
}
