//! 3D-Modelle der Militärgebäude: Schießstand, Stall, Kapelle, Magierturm, Belagerungswerkstatt, Altar
//! der Flamme sowie die Bauten der Hohlen (Knochengrube, Ghulbau, Aschaltar, Grabschmiede, Knochenlager).
//! Gleiche Pipeline und Palette wie die übrigen Bauwerke (Kachelraum, Ursprung = obere Ecke der Grundfläche).

use super::structures::{
    CRIMSON, DARK_STONE, DARK_WOOD, GOLD, HOLLOW, ROOF, STONE, Structure, WARM, WOOD, block, gable,
    grid, pt, pyramid, window,
};
use ashen_engine::bake::{Mat, Mesh};
use glam::{Mat4, Quat, Vec3};
use std::f32::consts::TAU;

const ROCK: u32 = 0x4c4a43;
const IRON: u32 = 0x2c2f34;
const PLASTER: u32 = 0x8a7e68;
const THATCH: u32 = 0x8a6f36;
const FLAME: u32 = 0xffa030;
const BONE: u32 = 0xc9c6ad;
const BLACK_STONE: u32 = 0x2e2c34;

fn plinth(m: &mut Mesh, w: f32, h: f32) {
    block(
        m,
        &pt(ROCK, Mat::Rock),
        0.04,
        0.04,
        w - 0.04,
        h - 0.04,
        0.0,
        0.07,
    );
}

fn finish(
    mut s: Structure,
    lights: Vec<(Vec3, Vec3, f32)>,
    smoke: Vec<Vec3>,
    fires: Vec<Vec3>,
) -> Structure {
    s.lights = lights;
    s.smoke = smoke;
    s.fires = fires;
    s
}

fn warm() -> Vec3 {
    Vec3::new(1.0, 0.65, 0.3)
}

fn green() -> Vec3 {
    Vec3::new(0.45, 1.0, 0.6)
}

fn orb(m: &mut Mesh, c: Vec3, r: f32, hex: u32) {
    m.ellipsoid(&pt(hex, Mat::Glow).glow(1.0), c, Vec3::splat(r), 6, 4);
}

fn hay(m: &mut Mesh, x: f32, z: f32) {
    block(
        m,
        &pt(0xb89a4a, Mat::Thatch),
        x - 0.13,
        z - 0.1,
        x + 0.13,
        z + 0.1,
        0.07,
        0.25,
    );
    block(
        m,
        &pt(0x6e4a2e, Mat::WoodPlank),
        x - 0.14,
        z - 0.02,
        x + 0.14,
        z + 0.02,
        0.15,
        0.17,
    );
}

fn door(m: &mut Mesh, x: f32, z: f32, w: f32, h: f32) {
    block(
        m,
        &pt(0x0d0b0a, Mat::Plain),
        x - w * 0.5,
        z - 0.01,
        x + w * 0.5,
        z + 0.012,
        0.07,
        0.07 + h,
    );
    block(
        m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        x - w * 0.5 + 0.02,
        z,
        x + w * 0.5 - 0.02,
        z + 0.03,
        0.07,
        0.07 + h - 0.03,
    );
}

/// Strohzielscheibe auf Pfosten (Mittelpunkt, Blick nach +z).
fn target(m: &mut Mesh, x: f32, z: f32) {
    m.frustum(
        &pt(DARK_WOOD, Mat::WoodPlank),
        Vec3::new(x, 0.07, z),
        Vec3::new(x, 0.6, z),
        0.02,
        0.018,
        4,
    );
    let t = Mat4::from_translation(Vec3::new(x, 0.62, z));
    for (r, c) in [
        (0.17, 0xe8dcc0u32),
        (0.12, 0xb3201c),
        (0.07, 0xe8dcc0),
        (0.035, 0xb3201c),
    ] {
        let d = (0.17 - r) * 0.12 + 0.005;
        m.cuboid(
            &pt(c, Mat::Thatch),
            Vec3::new(-r, -r, -0.02 + d),
            Vec3::new(r, r, 0.0 + d),
            t,
        );
    }
}

fn shooting_range() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    // Überdachter Schützenstand hinten (z klein) und Zielscheiben vorn.
    for x in [0.35f32, 1.35] {
        for z in [0.3f32, 0.9] {
            m.frustum(
                &pt(WOOD, Mat::WoodPlank),
                Vec3::new(x, 0.07, z),
                Vec3::new(x, 0.62, z),
                0.03,
                0.028,
                5,
            );
        }
    }
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        0.3,
        0.25,
        1.45,
        0.95,
        0.6,
        0.65,
    );
    gable(
        &mut m,
        &pt(ROOF, Mat::RoofTile),
        0.3,
        0.25,
        1.45,
        0.95,
        0.65,
        0.35,
        true,
        0.1,
    );
    block(
        &mut m,
        &pt(WOOD, Mat::WoodPlank),
        0.3,
        0.9,
        1.45,
        0.95,
        0.3,
        0.42,
    );
    // Waffenständer und Köcher.
    for i in 0..3 {
        let x = 1.65 + i as f32 * 0.12;
        m.frustum(
            &pt(WOOD, Mat::WoodPlank),
            Vec3::new(x, 0.07, 0.4),
            Vec3::new(x, 0.78, 0.38),
            0.016,
            0.014,
            4,
        );
    }
    block(
        &mut m,
        &pt(0x3a261a, Mat::Leather),
        1.62,
        0.3,
        2.0,
        0.5,
        0.07,
        0.2,
    );
    // Bahn mit Zielscheiben und Absperrung.
    for (i, x) in [0.5f32, 1.2, 1.9, 2.55].iter().enumerate() {
        target(&mut m, *x, 1.75 - (i % 2) as f32 * 0.1);
    }
    for i in 0..7 {
        let x = 0.2 + i as f32 * 0.43;
        block(
            &mut m,
            &pt(WOOD, Mat::WoodPlank),
            x - 0.02,
            1.2 - 0.02,
            x + 0.02,
            1.2 + 0.02,
            0.07,
            0.34,
        );
    }
    block(
        &mut m,
        &pt(WOOD, Mat::WoodPlank),
        0.2,
        1.18,
        2.8,
        1.22,
        0.26,
        0.3,
    );
    block(
        &mut m,
        &pt(CRIMSON, Mat::Cloth),
        2.75,
        0.2,
        2.78,
        0.5,
        0.5,
        1.0,
    );
    m.frustum(
        &pt(DARK_WOOD, Mat::WoodPlank),
        Vec3::new(2.74, 0.07, 0.2),
        Vec3::new(2.74, 1.1, 0.2),
        0.02,
        0.018,
        4,
    );
    let mut s = grid("bld_shooting_range", m, 3.0, 2.0, 1.5);
    s.lights = vec![(Vec3::new(0.9, 0.5, 0.6), warm(), 28.0)];
    s
}

fn stable() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    block(
        &mut m,
        &pt(STONE, Mat::StoneBrick),
        0.2,
        0.25,
        2.8,
        1.5,
        0.07,
        0.35,
    );
    block(
        &mut m,
        &pt(0x6e5238, Mat::WoodPlank),
        0.22,
        0.27,
        2.78,
        1.48,
        0.35,
        0.82,
    );
    gable(
        &mut m,
        &pt(THATCH, Mat::Thatch),
        0.2,
        0.25,
        2.8,
        1.5,
        0.82,
        0.6,
        true,
        0.15,
    );
    // Offene Boxen auf der Vorderseite.
    for x in [0.55f32, 1.25, 1.95] {
        block(
            &mut m,
            &pt(0x0d0b0a, Mat::Plain),
            x - 0.22,
            1.49,
            x + 0.22,
            1.51,
            0.07,
            0.62,
        );
        block(
            &mut m,
            &pt(DARK_WOOD, Mat::WoodPlank),
            x - 0.24,
            1.5,
            x - 0.2,
            1.54,
            0.07,
            0.65,
        );
        block(
            &mut m,
            &pt(DARK_WOOD, Mat::WoodPlank),
            x + 0.2,
            1.5,
            x + 0.24,
            1.54,
            0.07,
            0.65,
        );
        block(
            &mut m,
            &pt(0x6e4a2e, Mat::WoodPlank),
            x - 0.22,
            1.5,
            x + 0.22,
            1.53,
            0.35,
            0.39,
        );
    }
    window(&mut m, Vec3::new(2.79, 0.6, 0.9), true, 0.14, 0.2);
    // Heuballen, Futtertrog und Koppelzaun.
    hay(&mut m, 2.55, 1.75);
    hay(&mut m, 2.3, 1.8);
    block(
        &mut m,
        &pt(0x4a3220, Mat::WoodPlank),
        0.5,
        1.75,
        1.3,
        1.92,
        0.07,
        0.2,
    );
    block(
        &mut m,
        &pt(0x3a6a8a, Mat::Plain),
        0.55,
        1.78,
        1.25,
        1.89,
        0.19,
        0.2,
    );
    for i in 0..5 {
        let x = 1.5 + i as f32 * 0.25;
        block(
            &mut m,
            &pt(WOOD, Mat::WoodPlank),
            x - 0.02,
            1.95 - 0.02,
            x + 0.02,
            1.95 + 0.02,
            0.07,
            0.4,
        );
    }
    block(
        &mut m,
        &pt(WOOD, Mat::WoodPlank),
        1.5,
        1.93,
        2.5,
        1.97,
        0.28,
        0.32,
    );
    // Hufeisen über der Tür und Banner.
    m.frustum(
        &pt(GOLD, Mat::Gold),
        Vec3::new(1.25, 0.72, 1.52),
        Vec3::new(1.25, 0.77, 1.52),
        0.07,
        0.07,
        8,
    );
    block(
        &mut m,
        &pt(CRIMSON, Mat::Cloth),
        0.25,
        1.5,
        0.28,
        1.7,
        0.65,
        0.95,
    );
    finish(
        grid("bld_stable", m, 3.0, 2.0, 1.7),
        vec![(Vec3::new(1.25, 0.4, 1.56), warm(), 32.0)],
        vec![Vec3::new(2.4, 1.1, 0.9)],
        vec![],
    )
}

fn chapel() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    block(
        &mut m,
        &pt(0x6a675f, Mat::StoneBrick),
        0.25,
        0.3,
        1.75,
        1.7,
        0.07,
        0.85,
    );
    gable(
        &mut m,
        &pt(0x4a4a52, Mat::RoofTile),
        0.25,
        0.3,
        1.75,
        1.7,
        0.85,
        0.7,
        false,
        0.1,
    );
    // Rundfenster und Tür.
    m.frustum(
        &pt(WARM, Mat::Glow).glow(1.0),
        Vec3::new(1.0, 0.6, 1.71),
        Vec3::new(1.0, 0.6, 1.73),
        0.17,
        0.17,
        10,
    );
    door(&mut m, 1.0, 1.7, 0.3, 0.38);
    for x in [0.5f32, 1.5] {
        window(&mut m, Vec3::new(x, 0.5, 1.71), false, 0.1, 0.28);
    }
    // Glockenturm mit Flammenzeichen.
    block(
        &mut m,
        &pt(0x6a675f, Mat::StoneBrick),
        0.75,
        0.55,
        1.25,
        1.05,
        0.85,
        1.55,
    );
    block(
        &mut m,
        &pt(0x0d0b0a, Mat::Plain),
        0.88,
        0.54,
        1.12,
        0.58,
        1.25,
        1.45,
    );
    pyramid(
        &mut m,
        &pt(ROOF, Mat::RoofTile),
        0.7,
        0.5,
        1.3,
        1.1,
        1.55,
        0.6,
    );
    m.frustum(
        &pt(GOLD, Mat::Gold),
        Vec3::new(1.0, 2.15, 0.8),
        Vec3::new(1.0, 2.35, 0.8),
        0.04,
        0.0,
        4,
    );
    orb(&mut m, Vec3::new(1.0, 2.0, 0.8), 0.04, FLAME);
    // Weihrauchbecken am Eingang.
    for x in [0.55f32, 1.45] {
        m.frustum(
            &pt(IRON, Mat::Metal),
            Vec3::new(x, 0.07, 1.85),
            Vec3::new(x, 0.32, 1.85),
            0.03,
            0.03,
            4,
        );
        m.frustum(
            &pt(IRON, Mat::Metal),
            Vec3::new(x, 0.32, 1.85),
            Vec3::new(x, 0.4, 1.85),
            0.08,
            0.1,
            8,
        );
        m.frustum(
            &pt(FLAME, Mat::Glow).glow(1.0),
            Vec3::new(x, 0.4, 1.85),
            Vec3::new(x, 0.56, 1.85),
            0.06,
            0.0,
            5,
        );
    }
    let s = grid("bld_chapel", m, 2.0, 2.0, 2.6);
    finish(
        s,
        vec![
            (Vec3::new(1.0, 0.6, 1.78), Vec3::new(1.0, 0.75, 0.35), 54.0),
            (Vec3::new(0.55, 0.5, 1.85), Vec3::new(1.0, 0.5, 0.15), 30.0),
        ],
        vec![Vec3::new(1.0, 0.7, 1.85)],
        vec![Vec3::new(0.55, 0.5, 1.85), Vec3::new(1.45, 0.5, 1.85)],
    )
}

fn mage_tower() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    let stone = pt(0x6a6272, Mat::StoneBrick);
    m.frustum(
        &stone,
        Vec3::new(1.0, 0.07, 1.0),
        Vec3::new(1.0, 2.0, 1.0),
        0.72,
        0.52,
        10,
    );
    m.frustum(
        &pt(DARK_STONE, Mat::StoneBrick),
        Vec3::new(1.0, 2.0, 1.0),
        Vec3::new(1.0, 2.12, 1.0),
        0.62,
        0.62,
        10,
    );
    m.frustum(
        &pt(0x7a2a24, Mat::RoofTile).flat(),
        Vec3::new(1.0, 2.12, 1.0),
        Vec3::new(1.0, 3.1, 1.0),
        0.6,
        0.0,
        10,
    );
    // Fenster in Spiralstellung, Eingang, Balkon.
    for i in 0..4 {
        let a = i as f32 * 1.7 + 0.4;
        let y = 0.7 + i as f32 * 0.32;
        let r = 0.72 - (y - 0.07) / 1.93 * 0.2;
        let c = Vec3::new(1.0 + a.cos() * r, y, 1.0 + a.sin() * r);
        m.ellipsoid(
            &pt(WARM, Mat::Glow).glow(1.0),
            c,
            Vec3::new(0.05, 0.09, 0.05),
            5,
            3,
        );
    }
    block(
        &mut m,
        &pt(0x0d0b0a, Mat::Plain),
        0.88,
        1.69,
        1.12,
        1.73,
        0.07,
        0.4,
    );
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        0.9,
        1.7,
        1.1,
        1.74,
        0.07,
        0.37,
    );
    m.frustum(
        &pt(IRON, Mat::Metal),
        Vec3::new(1.0, 1.55, 1.0),
        Vec3::new(1.0, 1.6, 1.0),
        0.7,
        0.7,
        10,
    );
    // Schwebende Feuerkugel mit Ring.
    orb(&mut m, Vec3::new(1.0, 3.3, 1.0), 0.13, FLAME);
    for i in 0..8 {
        let a = i as f32 / 8.0 * TAU;
        m.ellipsoid(
            &pt(0xffd35a, Mat::Glow).glow(1.0),
            Vec3::new(
                1.0 + a.cos() * 0.26,
                3.3 + (a * 2.0).sin() * 0.04,
                1.0 + a.sin() * 0.26,
            ),
            Vec3::splat(0.03),
            4,
            3,
        );
    }
    let s = grid("bld_mage_tower", m, 2.0, 2.0, 3.8);
    finish(
        s,
        vec![
            (Vec3::new(1.0, 3.3, 1.0), Vec3::new(1.0, 0.55, 0.15), 90.0),
            (Vec3::new(1.0, 1.0, 1.7), warm(), 30.0),
        ],
        vec![],
        vec![Vec3::new(1.0, 3.35, 1.0)],
    )
}

fn siege_workshop() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 3.0);
    // Großer offener Schuppen (hinten) mit Dach auf Pfosten.
    for x in [0.25f32, 1.5, 2.75] {
        for z in [0.25f32, 1.25] {
            block(
                &mut m,
                &pt(DARK_WOOD, Mat::WoodPlank),
                x - 0.06,
                z - 0.06,
                x + 0.06,
                z + 0.06,
                0.07,
                1.05,
            );
        }
    }
    block(
        &mut m,
        &pt(0x5a4632, Mat::WoodPlank),
        0.2,
        0.2,
        2.8,
        0.32,
        0.07,
        1.0,
    );
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        0.15,
        0.15,
        2.85,
        1.35,
        1.0,
        1.08,
    );
    gable(
        &mut m,
        &pt(0x4a3a30, Mat::RoofTile),
        0.15,
        0.15,
        2.85,
        1.35,
        1.08,
        0.5,
        true,
        0.12,
    );
    // Halbfertiger Rammbock unter dem Dach.
    block(
        &mut m,
        &pt(WOOD, Mat::WoodPlank),
        0.5,
        0.55,
        1.5,
        0.62,
        0.3,
        0.36,
    );
    block(
        &mut m,
        &pt(WOOD, Mat::WoodPlank),
        0.5,
        1.0,
        1.5,
        1.07,
        0.3,
        0.36,
    );
    m.frustum(
        &pt(0x4a3322, Mat::Bark),
        Vec3::new(0.4, 0.6, 0.8),
        Vec3::new(1.7, 0.6, 0.8),
        0.09,
        0.09,
        8,
    );
    m.frustum(
        &pt(0x3a3a42, Mat::Metal),
        Vec3::new(1.7, 0.6, 0.8),
        Vec3::new(1.85, 0.6, 0.8),
        0.1,
        0.07,
        8,
    );
    for z in [0.55f32, 1.05] {
        m.frustum(
            &pt(DARK_WOOD, Mat::WoodPlank),
            Vec3::new(0.7, 0.07, z),
            Vec3::new(0.7, 0.3, z),
            0.13,
            0.13,
            8,
        );
        m.frustum(
            &pt(DARK_WOOD, Mat::WoodPlank),
            Vec3::new(1.3, 0.07, z),
            Vec3::new(1.3, 0.3, z),
            0.13,
            0.13,
            8,
        );
    }
    // Katapultarm als Gestell mit Wurfarm.
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        2.0,
        0.5,
        2.7,
        0.6,
        0.07,
        0.15,
    );
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        2.0,
        1.0,
        2.7,
        1.1,
        0.07,
        0.15,
    );
    m.cuboid(
        &pt(WOOD, Mat::WoodPlank),
        Vec3::new(-0.04, -0.04, -0.5),
        Vec3::new(0.04, 0.04, 0.5),
        Mat4::from_rotation_translation(Quat::from_rotation_x(-0.8), Vec3::new(2.35, 0.7, 0.8)),
    );
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        2.28,
        0.58,
        2.42,
        0.62,
        0.15,
        0.65,
    );
    block(
        &mut m,
        &pt(DARK_WOOD, Mat::WoodPlank),
        2.28,
        0.98,
        2.42,
        1.02,
        0.15,
        0.65,
    );
    // Vorplatz: Holzstapel, Fässer, Amboss, Fackel.
    for r in 0..3 {
        for i in 0..4 - r {
            m.frustum(
                &pt(0x6e4a2e, Mat::Bark),
                Vec3::new(
                    0.3 + i as f32 * 0.2 + r as f32 * 0.1,
                    0.12 + r as f32 * 0.16,
                    2.45,
                ),
                Vec3::new(
                    0.3 + i as f32 * 0.2 + r as f32 * 0.1,
                    0.12 + r as f32 * 0.16,
                    2.9,
                ),
                0.08,
                0.08,
                6,
            );
        }
    }
    block(
        &mut m,
        &pt(IRON, Mat::Metal),
        1.9,
        2.2,
        2.3,
        2.4,
        0.07,
        0.22,
    );
    block(
        &mut m,
        &pt(IRON, Mat::Metal),
        1.8,
        2.15,
        2.4,
        2.45,
        0.22,
        0.3,
    );
    m.frustum(
        &pt(DARK_WOOD, Mat::WoodPlank),
        Vec3::new(2.7, 0.07, 2.2),
        Vec3::new(2.7, 1.2, 2.2),
        0.03,
        0.025,
        4,
    );
    orb(&mut m, Vec3::new(2.7, 1.25, 2.2), 0.07, FLAME);
    block(
        &mut m,
        &pt(CRIMSON, Mat::Cloth),
        0.2,
        1.36,
        0.8,
        1.38,
        0.6,
        1.0,
    );
    let s = grid("bld_siege_workshop", m, 3.0, 3.0, 2.0);
    finish(
        s,
        vec![(Vec3::new(2.7, 1.25, 2.2), Vec3::new(1.0, 0.55, 0.2), 56.0)],
        vec![Vec3::new(2.3, 1.3, 2.3)],
        vec![Vec3::new(2.7, 1.25, 2.2)],
    )
}

fn altar() -> Structure {
    let mut m = Mesh::default();
    // Rundes Podest mit Stufen, vier Säulen, Flammenschale.
    let stone = pt(0x7a766c, Mat::StoneBrick);
    m.frustum(
        &pt(ROCK, Mat::Rock),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(1.0, 0.1, 1.0),
        0.96,
        0.9,
        12,
    );
    m.frustum(
        &stone,
        Vec3::new(1.0, 0.1, 1.0),
        Vec3::new(1.0, 0.2, 1.0),
        0.84,
        0.8,
        12,
    );
    for i in 0..4 {
        let a = i as f32 / 4.0 * TAU + TAU / 8.0;
        let c = Vec3::new(1.0 + a.cos() * 0.68, 0.2, 1.0 + a.sin() * 0.68);
        m.frustum(&stone, c, c + Vec3::new(0.0, 0.85, 0.0), 0.09, 0.075, 6);
        m.frustum(
            &pt(GOLD, Mat::Gold),
            c + Vec3::new(0.0, 0.85, 0.0),
            c + Vec3::new(0.0, 0.92, 0.0),
            0.11,
            0.11,
            6,
        );
        block(
            &mut m,
            &pt(CRIMSON, Mat::Cloth),
            c.x - 0.05,
            c.z - 0.015,
            c.x + 0.05,
            c.z + 0.015,
            0.45,
            0.82,
        );
    }
    // Zentrale Schale.
    m.frustum(
        &stone,
        Vec3::new(1.0, 0.2, 1.0),
        Vec3::new(1.0, 0.5, 1.0),
        0.14,
        0.1,
        8,
    );
    m.frustum(
        &pt(IRON, Mat::Metal),
        Vec3::new(1.0, 0.5, 1.0),
        Vec3::new(1.0, 0.62, 1.0),
        0.2,
        0.26,
        10,
    );
    m.frustum(
        &pt(FLAME, Mat::Glow).glow(1.0),
        Vec3::new(1.0, 0.62, 1.0),
        Vec3::new(1.0, 1.05, 1.0),
        0.18,
        0.0,
        6,
    );
    m.frustum(
        &pt(0xffe09a, Mat::Glow).glow(1.0),
        Vec3::new(1.0, 0.64, 1.0),
        Vec3::new(1.0, 0.85, 1.0),
        0.1,
        0.0,
        6,
    );
    // Dach aus Gold: Ring über den Säulen.
    for i in 0..4 {
        let a = i as f32 / 4.0 * TAU + TAU / 8.0;
        let c = Vec3::new(1.0 + a.cos() * 0.68, 1.12, 1.0 + a.sin() * 0.68);
        m.ellipsoid(&pt(GOLD, Mat::Gold), c, Vec3::splat(0.05), 5, 3);
    }
    let s = grid("bld_altar", m, 2.0, 2.0, 1.8);
    finish(
        s,
        vec![(Vec3::new(1.0, 0.8, 1.0), Vec3::new(1.0, 0.6, 0.25), 86.0)],
        vec![],
        vec![Vec3::new(1.0, 0.8, 1.0)],
    )
}

// ------------------------------------------------------------------------------------------------
// Die Hohlen
// ------------------------------------------------------------------------------------------------

fn bone_rib(m: &mut Mesh, x: f32, z: f32, along_x: bool, r: f32, h: f32) {
    let bone = pt(BONE, Mat::Bone);
    let n = 7;
    for i in 0..n {
        let a0 = i as f32 / n as f32 * std::f32::consts::PI;
        let a1 = (i + 1) as f32 / n as f32 * std::f32::consts::PI;
        let p = |a: f32| {
            if along_x {
                Vec3::new(x + a.cos() * r, a.sin() * h + 0.07, z)
            } else {
                Vec3::new(x, a.sin() * h + 0.07, z + a.cos() * r)
            }
        };
        m.frustum(&bone, p(a0), p(a1), 0.035, 0.03, 4);
    }
}

fn bone_pit() -> Structure {
    let mut m = Mesh::default();
    block(
        &mut m,
        &pt(BLACK_STONE, Mat::Rock),
        0.05,
        0.05,
        2.95,
        1.95,
        0.0,
        0.1,
    );
    // Grube mit grünem Glühen, umgeben von Rippenbögen und Gebeinhaufen.
    m.frustum(
        &pt(0x0d0b0a, Mat::Plain),
        Vec3::new(1.5, 0.1, 1.0),
        Vec3::new(1.5, 0.12, 1.0),
        0.6,
        0.6,
        12,
    );
    m.frustum(
        &pt(HOLLOW, Mat::Glow).glow(0.9),
        Vec3::new(1.5, 0.11, 1.0),
        Vec3::new(1.5, 0.13, 1.0),
        0.42,
        0.4,
        12,
    );
    for z in [0.55f32, 1.0, 1.45] {
        bone_rib(&mut m, 0.45, z, true, 0.28, 0.9);
        bone_rib(&mut m, 2.55, z, true, 0.28, 0.9);
    }
    for i in 0..10 {
        let a = i as f32 * 2.39;
        let c = Vec3::new(1.5 + a.cos() * 0.75, 0.1, 1.0 + a.sin() * 0.55);
        m.cuboid(
            &pt(BONE, Mat::Bone),
            Vec3::new(-0.15, 0.0, -0.025),
            Vec3::new(0.15, 0.05, 0.025),
            Mat4::from_rotation_translation(Quat::from_rotation_y(a * 1.3), c),
        );
    }
    // Schädelpfosten am Rand.
    for (x, z) in [(0.25f32, 0.3f32), (2.75, 0.3), (1.5, 1.82)] {
        m.frustum(
            &pt(0x3a2c1c, Mat::WoodPlank),
            Vec3::new(x, 0.1, z),
            Vec3::new(x, 0.85, z),
            0.03,
            0.025,
            4,
        );
        m.ellipsoid(
            &pt(BONE, Mat::Bone),
            Vec3::new(x, 0.92, z),
            Vec3::new(0.08, 0.09, 0.08),
            6,
            4,
        );
        orb(&mut m, Vec3::new(x, 0.94, z + 0.06), 0.02, HOLLOW);
    }
    let mut s = grid("bld_bone_pit", m, 3.0, 2.0, 1.6);
    s.lights = vec![(Vec3::new(1.5, 0.3, 1.0), green(), 70.0)];
    s.glow = vec![Vec3::new(1.5, 0.3, 1.0), Vec3::new(0.25, 0.95, 0.3)];
    s
}

fn ghoul_den() -> Structure {
    let mut m = Mesh::default();
    // Felsmound mit dunklem Höhleneingang auf der Vorderseite.
    let rock = pt(0x3e3a40, Mat::Rock);
    m.ellipsoid(
        &rock,
        Vec3::new(1.0, 0.1, 0.9),
        Vec3::new(0.95, 0.85, 0.8),
        9,
        5,
    );
    m.ellipsoid(
        &pt(0x4a464e, Mat::Rock),
        Vec3::new(0.5, 0.1, 0.6),
        Vec3::new(0.5, 0.55, 0.45),
        7,
        4,
    );
    m.ellipsoid(
        &pt(0x4a464e, Mat::Rock),
        Vec3::new(1.55, 0.1, 0.5),
        Vec3::new(0.4, 0.5, 0.4),
        7,
        4,
    );
    m.ellipsoid(
        &pt(0x0a090c, Mat::Plain),
        Vec3::new(1.0, 0.3, 1.6),
        Vec3::new(0.34, 0.34, 0.16),
        8,
        5,
    );
    // Glühende Augen in der Dunkelheit.
    for (x, y) in [(0.9f32, 0.34f32), (1.1, 0.34), (1.0, 0.5)] {
        orb(&mut m, Vec3::new(x, y, 1.74), 0.025, HOLLOW);
    }
    // Knochen und Zähne am Eingang.
    for i in 0..6 {
        let x = 0.72 + i as f32 * 0.11;
        m.frustum(
            &pt(BONE, Mat::Bone),
            Vec3::new(x, 0.07, 1.7),
            Vec3::new(x, if i % 2 == 0 { 0.5 } else { 0.4 }, 1.72),
            0.03,
            0.0,
            4,
        );
    }
    for i in 0..5 {
        let a = i as f32 * 2.1;
        m.cuboid(
            &pt(BONE, Mat::Bone),
            Vec3::new(-0.13, 0.0, -0.02),
            Vec3::new(0.13, 0.05, 0.02),
            Mat4::from_rotation_translation(
                Quat::from_rotation_y(a),
                Vec3::new(0.4 + i as f32 * 0.35, 0.07, 1.85),
            ),
        );
    }
    let mut s = grid("bld_ghoul_den", m, 2.0, 2.0, 1.2);
    s.lights = vec![(Vec3::new(1.0, 0.4, 1.8), green(), 44.0)];
    s.glow = vec![Vec3::new(1.0, 0.42, 1.74)];
    s
}

fn ash_altar() -> Structure {
    let mut m = Mesh::default();
    block(
        &mut m,
        &pt(BLACK_STONE, Mat::Rock),
        0.05,
        0.05,
        2.95,
        1.95,
        0.0,
        0.1,
    );
    let obs = pt(0x1f1d26, Mat::StoneBrick);
    // Stufenpodest und Altarplatte.
    block(&mut m, &obs, 0.4, 0.3, 2.6, 1.7, 0.1, 0.22);
    block(
        &mut m,
        &pt(0x2e2c36, Mat::StoneBrick),
        0.7,
        0.55,
        2.3,
        1.45,
        0.22,
        0.5,
    );
    block(
        &mut m,
        &pt(0x3a3745, Mat::StoneBrick),
        0.62,
        0.48,
        2.38,
        1.52,
        0.5,
        0.58,
    );
    // Aschglas-Kristalle und Pfeiler mit grünen Flammen.
    for (x, z) in [(0.45f32, 0.35f32), (2.55, 0.35), (0.45, 1.65), (2.55, 1.65)] {
        m.frustum(
            &obs,
            Vec3::new(x, 0.1, z),
            Vec3::new(x, 1.3, z),
            0.09,
            0.06,
            5,
        );
        m.ellipsoid(
            &pt(0x3a3745, Mat::StoneBrick),
            Vec3::new(x, 1.32, z),
            Vec3::new(0.11, 0.05, 0.11),
            6,
            3,
        );
        m.frustum(
            &pt(HOLLOW, Mat::Glow).glow(1.0),
            Vec3::new(x, 1.36, z),
            Vec3::new(x, 1.62, z),
            0.06,
            0.0,
            5,
        );
    }
    for k in 0..5 {
        let x = 1.0 + k as f32 * 0.25;
        m.frustum(
            &pt(0x86f2a0, Mat::Glow).glow(1.0),
            Vec3::new(x, 0.58, 1.0),
            Vec3::new(
                x + (k as f32 - 2.0) * 0.03,
                0.58 + 0.25 + (k % 2) as f32 * 0.2,
                1.0,
            ),
            0.05,
            0.0,
            4,
        );
    }
    m.ellipsoid(
        &pt(BONE, Mat::Bone),
        Vec3::new(1.5, 0.66, 0.65),
        Vec3::new(0.08, 0.09, 0.08),
        6,
        4,
    );
    let mut s = grid("bld_ash_altar", m, 3.0, 2.0, 1.9);
    s.lights = vec![(Vec3::new(1.5, 0.9, 1.0), green(), 80.0)];
    s.glow = vec![
        Vec3::new(1.5, 0.9, 1.0),
        Vec3::new(0.45, 1.5, 0.35),
        Vec3::new(2.55, 1.5, 1.65),
    ];
    s
}

fn grave_forge() -> Structure {
    let mut m = Mesh::default();
    block(
        &mut m,
        &pt(BLACK_STONE, Mat::Rock),
        0.05,
        0.05,
        2.95,
        2.95,
        0.0,
        0.1,
    );
    // Esse aus Knochen und Rost mit grünem Feuer.
    block(
        &mut m,
        &pt(0x3a2c24, Mat::StoneBrick),
        0.5,
        0.4,
        2.2,
        1.6,
        0.1,
        0.95,
    );
    block(
        &mut m,
        &pt(0x0d0b0a, Mat::Plain),
        0.9,
        1.59,
        1.8,
        1.64,
        0.2,
        0.75,
    );
    block(
        &mut m,
        &pt(FLAME, Mat::Glow).glow(1.0),
        1.0,
        1.6,
        1.7,
        1.66,
        0.25,
        0.4,
    );
    block(
        &mut m,
        &pt(HOLLOW, Mat::Glow).glow(1.0),
        1.1,
        1.6,
        1.6,
        1.67,
        0.4,
        0.6,
    );
    m.frustum(
        &pt(0x3a2c24, Mat::StoneBrick),
        Vec3::new(1.35, 0.95, 0.9),
        Vec3::new(1.35, 2.5, 0.9),
        0.33,
        0.24,
        8,
    );
    m.frustum(
        &pt(HOLLOW, Mat::Glow).glow(1.0),
        Vec3::new(1.35, 2.5, 0.9),
        Vec3::new(1.35, 2.58, 0.9),
        0.2,
        0.2,
        8,
    );
    for i in 0..5 {
        let x = 0.55 + i as f32 * 0.4;
        m.frustum(
            &pt(BONE, Mat::Bone),
            Vec3::new(x, 0.95, 0.4),
            Vec3::new(x + 0.1, 1.5, 0.4),
            0.035,
            0.02,
            4,
        );
    }
    // Amboss, Ketten mit Gliedern, Werkstücke.
    block(&mut m, &pt(IRON, Mat::Metal), 2.2, 2.0, 2.7, 2.3, 0.1, 0.3);
    block(
        &mut m,
        &pt(IRON, Mat::Metal),
        2.1,
        1.95,
        2.8,
        2.35,
        0.3,
        0.4,
    );
    block(
        &mut m,
        &pt(0x7a4a30, Mat::Metal),
        0.5,
        2.1,
        1.3,
        2.4,
        0.1,
        0.28,
    );
    for x in [0.6f32, 1.1, 1.6] {
        m.frustum(
            &pt(IRON, Mat::Metal),
            Vec3::new(x, 1.0, 2.2),
            Vec3::new(x, 1.7, 2.2),
            0.015,
            0.015,
            4,
        );
        m.ellipsoid(
            &pt(BONE, Mat::Bone),
            Vec3::new(x, 0.9, 2.2),
            Vec3::new(0.07, 0.09, 0.07),
            6,
            4,
        );
    }
    // Koloss in Arbeit: Rumpf aus Knochen neben der Esse.
    m.ellipsoid(
        &pt(0x485644, Mat::Skin),
        Vec3::new(2.4, 0.5, 0.9),
        Vec3::new(0.3, 0.38, 0.25),
        7,
        4,
    );
    m.ellipsoid(
        &pt(BONE, Mat::Bone),
        Vec3::new(2.4, 0.95, 0.9),
        Vec3::new(0.14, 0.14, 0.13),
        6,
        4,
    );
    let mut s = grid("bld_grave_forge", m, 3.0, 3.0, 2.8);
    s.lights = vec![
        (Vec3::new(1.35, 0.4, 1.7), Vec3::new(1.0, 0.5, 0.15), 76.0),
        (Vec3::new(1.35, 2.55, 0.9), green(), 48.0),
    ];
    s.smoke = vec![Vec3::new(1.35, 2.6, 0.9)];
    s.fires = vec![Vec3::new(1.35, 0.35, 1.7)];
    s.glow = vec![Vec3::new(1.35, 2.55, 0.9)];
    s
}

fn bone_cache() -> Structure {
    let mut m = Mesh::default();
    block(
        &mut m,
        &pt(BLACK_STONE, Mat::Rock),
        0.05,
        0.05,
        1.95,
        1.95,
        0.0,
        0.07,
    );
    // Knochenzelt: drei Stangen, Fellbahnen, Vorräte davor.
    for i in 0..4 {
        let a = i as f32 / 4.0 * TAU;
        m.frustum(
            &pt(BONE, Mat::Bone),
            Vec3::new(1.0 + a.cos() * 0.6, 0.07, 0.8 + a.sin() * 0.45),
            Vec3::new(1.0, 1.1, 0.8),
            0.035,
            0.02,
            4,
        );
    }
    for i in 0..4 {
        let a0 = i as f32 / 4.0 * TAU;
        let a1 = (i + 1) as f32 / 4.0 * TAU;
        let col = if i % 2 == 0 { 0x4a3a2c } else { 0x3a2e24 };
        m.tri_flat(
            &pt(col, Mat::Leather),
            Vec3::new(1.0 + a1.cos() * 0.6, 0.07, 0.8 + a1.sin() * 0.45),
            Vec3::new(1.0 + a0.cos() * 0.6, 0.07, 0.8 + a0.sin() * 0.45),
            Vec3::new(1.0, 1.1, 0.8),
        );
    }
    m.ellipsoid(
        &pt(BONE, Mat::Bone),
        Vec3::new(1.0, 1.16, 0.8),
        Vec3::new(0.07, 0.08, 0.07),
        6,
        4,
    );
    m.ellipsoid(
        &pt(0x0d0b0a, Mat::Plain),
        Vec3::new(1.0, 0.25, 1.22),
        Vec3::new(0.16, 0.22, 0.05),
        6,
        4,
    );
    for (x, z, c) in [
        (0.4f32, 1.55f32, 0x6e4a2e),
        (0.65, 1.62, 0x5a3a22),
        (1.5, 1.5, 0x6e4a2e),
    ] {
        block(
            &mut m,
            &pt(c, Mat::WoodPlank),
            x - 0.12,
            z - 0.1,
            x + 0.12,
            z + 0.1,
            0.07,
            0.3,
        );
    }
    m.ellipsoid(
        &pt(0xd2b04a, Mat::Gold),
        Vec3::new(1.5, 0.36, 1.5),
        Vec3::new(0.08, 0.05, 0.08),
        5,
        3,
    );
    for i in 0..4 {
        let a = i as f32 * 2.4;
        m.cuboid(
            &pt(BONE, Mat::Bone),
            Vec3::new(-0.12, 0.0, -0.02),
            Vec3::new(0.12, 0.05, 0.02),
            Mat4::from_rotation_translation(
                Quat::from_rotation_y(a),
                Vec3::new(0.3 + i as f32 * 0.45, 0.07, 1.85),
            ),
        );
    }
    let mut s = grid("bld_bone_cache", m, 2.0, 2.0, 1.4);
    s.lights = vec![(Vec3::new(1.0, 0.3, 1.25), green(), 26.0)];
    s
}

pub fn build_all() -> Vec<Structure> {
    let _ = (PLASTER, DARK_STONE);
    vec![
        shooting_range(),
        stable(),
        chapel(),
        mage_tower(),
        siege_workshop(),
        altar(),
        bone_pit(),
        ghoul_den(),
        ash_altar(),
        grave_forge(),
        bone_cache(),
    ]
}
