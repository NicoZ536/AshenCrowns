//! 3D-Modelle der Echtzeit-Wirtschaft: Wohnhäuser, Produktionsgebäude, Lager, Markt, Tempel sowie
//! Mauerteile, Tore und Mauertürme. Gleiche Pipeline und Palette wie die übrigen Bauwerke.
//!
//! Mauern werden als 16 Varianten (4-Bit-Anschlussmaske der Nachbarn) und zwei Diagonalverbindern
//! gebacken; Tore als vier Varianten (Achse × offen/geschlossen).

use super::structures::{block, gable, grid, merlons, pt, pyramid, timber_frame, window, Structure, CRIMSON, DARK_STONE, DARK_WOOD, GOLD, ROOF, STONE, WARM, WOOD};
use ashen_engine::bake::{Mat, Mesh};
use glam::{Mat4, Quat, Vec2, Vec3};
use std::f32::consts::TAU;

const PLASTER: u32 = 0x8a7e68;
const THATCH: u32 = 0x8a6f36;
const ROCK: u32 = 0x4c4a43;
const BRICK: u32 = 0x8a3a2a;
const IRON: u32 = 0x2c2f34;
const COPPER: u32 = 0xb4693a;
const FLAME: u32 = 0xffa030;

/// Sockelplatte unter dem Gebäude.
fn plinth(m: &mut Mesh, w: f32, h: f32) {
    block(m, &pt(ROCK, Mat::Rock), 0.04, 0.04, w - 0.04, h - 0.04, 0.0, 0.07);
}

fn glow_orb(m: &mut Mesh, c: Vec3, r: f32) {
    m.ellipsoid(&pt(0xffc24a, Mat::Glow).glow(1.0), c, Vec3::splat(r), 5, 3);
}

fn logs(m: &mut Mesh, x: f32, z: f32, len: f32, rows: i32) {
    let bark = pt(0x4a3524, Mat::Bark);
    for row in 0..rows {
        for k in 0..(rows - row) {
            let zz = z + k as f32 * 0.085 + row as f32 * 0.042;
            let y = 0.05 + row as f32 * 0.075;
            m.frustum(&bark, Vec3::new(x, y, zz), Vec3::new(x + len, y, zz), 0.04, 0.04, 6);
        }
    }
}

fn barrel(m: &mut Mesh, x: f32, z: f32, h: f32) {
    m.frustum(&pt(0x5a3a22, Mat::WoodPlank), Vec3::new(x, 0.0, z), Vec3::new(x, h, z), 0.09, 0.09, 8);
    for y in [h * 0.25, h * 0.75] {
        m.frustum(&pt(IRON, Mat::Metal), Vec3::new(x, y - 0.012, z), Vec3::new(x, y + 0.012, z), 0.095, 0.095, 8);
    }
}

fn crate_box(m: &mut Mesh, x: f32, z: f32, s: f32) {
    block(m, &pt(0x6e4a2e, Mat::WoodPlank), x, z, x + s, z + s, 0.0, s * 0.8);
    block(m, &pt(DARK_WOOD, Mat::WoodPlank), x - 0.005, z + s * 0.45, x + s + 0.005, z + s * 0.55, s * 0.2, s * 0.6);
}

fn sack(m: &mut Mesh, x: f32, z: f32, col: u32) {
    m.ellipsoid(&pt(col, Mat::Cloth), Vec3::new(x, 0.08, z), Vec3::new(0.09, 0.1, 0.08), 6, 4);
}

fn chimney(m: &mut Mesh, x: f32, z: f32, y0: f32, y1: f32) {
    block(m, &pt(0x4a4a50, Mat::StoneBrick), x - 0.07, z - 0.07, x + 0.07, z + 0.07, y0, y1);
    block(m, &pt(DARK_STONE, Mat::StoneBrick), x - 0.09, z - 0.09, x + 0.09, z + 0.09, y1, y1 + 0.04);
}

/// Dunkle Tür auf der +z-Seite (Vorderseite links unten) bzw. +x-Seite.
fn door_z(m: &mut Mesh, x: f32, z: f32, w: f32, h: f32) {
    block(m, &pt(0x0d0b0a, Mat::Plain), x - w * 0.5, z - 0.01, x + w * 0.5, z + 0.012, 0.07, 0.07 + h);
    block(m, &pt(DARK_WOOD, Mat::WoodPlank), x - w * 0.5 + 0.02, z, x + w * 0.5 - 0.02, z + 0.03, 0.07, 0.07 + h - 0.03);
}

fn finish(mut s: Structure, lights: Vec<(Vec3, Vec3, f32)>, smoke: Vec<Vec3>, fires: Vec<Vec3>) -> Structure {
    s.lights = lights;
    s.smoke = smoke;
    s.fires = fires;
    s
}

fn warm() -> Vec3 {
    Vec3::new(1.0, 0.65, 0.3)
}

// ------------------------------------------------------------------------------------------------
// Wohnen
// ------------------------------------------------------------------------------------------------

fn hut() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    block(&mut m, &pt(0x7a6a52, Mat::Plaster), 0.4, 0.4, 1.6, 1.5, 0.07, 0.5);
    timber_frame(&mut m, 0.4, 0.4, 1.6, 1.5, 0.07, 0.5);
    gable(&mut m, &pt(THATCH, Mat::Thatch), 0.4, 0.4, 1.6, 1.5, 0.5, 0.5, true, 0.12);
    door_z(&mut m, 0.75, 1.5, 0.22, 0.3);
    window(&mut m, Vec3::new(1.25, 0.32, 1.51), false, 0.14, 0.12);
    chimney(&mut m, 1.35, 0.6, 0.5, 1.05);
    sack(&mut m, 1.75, 1.35, 0x8a7a50);
    barrel(&mut m, 0.25, 1.65, 0.2);
    finish(grid("bld_hut", m, 2.0, 2.0, 1.5), vec![(Vec3::new(1.25, 0.32, 1.55), warm(), 30.0)], vec![Vec3::new(1.35, 1.1, 0.6)], vec![])
}

fn house() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    block(&mut m, &pt(STONE, Mat::StoneBrick), 0.3, 0.3, 1.7, 1.6, 0.07, 0.5);
    block(&mut m, &pt(PLASTER, Mat::Plaster), 0.26, 0.26, 1.74, 1.64, 0.5, 1.0);
    timber_frame(&mut m, 0.26, 0.26, 1.74, 1.64, 0.5, 1.0);
    gable(&mut m, &pt(ROOF, Mat::RoofTile), 0.26, 0.26, 1.74, 1.64, 1.0, 0.62, true, 0.12);
    door_z(&mut m, 0.7, 1.6, 0.24, 0.34);
    for x in [1.1, 1.45] {
        window(&mut m, Vec3::new(x, 0.3, 1.61), false, 0.13, 0.14);
        window(&mut m, Vec3::new(x - 0.3, 0.78, 1.65), false, 0.13, 0.15);
    }
    window(&mut m, Vec3::new(1.745, 0.8, 0.95), true, 0.13, 0.15);
    chimney(&mut m, 1.4, 0.55, 1.0, 1.55);
    finish(grid("bld_house", m, 2.0, 2.0, 1.9), vec![(Vec3::new(1.1, 0.5, 1.7), warm(), 40.0)], vec![Vec3::new(1.4, 1.6, 0.55)], vec![])
}

fn townhouse() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    block(&mut m, &pt(STONE, Mat::StoneBrick), 0.2, 0.2, 1.8, 1.7, 0.07, 0.7);
    block(&mut m, &pt(PLASTER, Mat::Plaster), 0.16, 0.16, 1.84, 1.74, 0.7, 1.3);
    timber_frame(&mut m, 0.16, 0.16, 1.84, 1.74, 0.7, 1.3);
    gable(&mut m, &pt(ROOF, Mat::RoofTile), 0.16, 0.16, 1.84, 1.74, 1.3, 0.7, false, 0.12);
    // Erker mit eigenem Giebel zur Straße.
    block(&mut m, &pt(PLASTER, Mat::Plaster), 0.7, 1.6, 1.3, 1.95, 0.7, 1.2);
    gable(&mut m, &pt(ROOF, Mat::RoofTile), 0.7, 1.6, 1.3, 1.95, 1.2, 0.4, true, 0.06);
    door_z(&mut m, 0.5, 1.7, 0.26, 0.42);
    block(&mut m, &pt(GOLD, Mat::Gold), 0.36, 1.72, 0.64, 1.74, 0.55, 0.6);
    for x in [1.1, 1.5] {
        window(&mut m, Vec3::new(x, 0.42, 1.71), false, 0.14, 0.18);
    }
    window(&mut m, Vec3::new(1.0, 0.95, 1.96), false, 0.16, 0.18);
    window(&mut m, Vec3::new(1.845, 0.95, 0.7), true, 0.14, 0.18);
    window(&mut m, Vec3::new(1.845, 0.95, 1.2), true, 0.14, 0.18);
    chimney(&mut m, 1.55, 0.45, 1.3, 2.05);
    // Wetterfahne.
    m.frustum(&pt(GOLD, Mat::Gold), Vec3::new(1.0, 1.95, 0.9), Vec3::new(1.0, 2.25, 0.9), 0.02, 0.0, 4);
    banner_small(&mut m, Vec3::new(1.82, 1.2, 1.6));
    finish(grid("bld_townhouse", m, 2.0, 2.0, 2.4), vec![(Vec3::new(1.2, 0.5, 1.8), warm(), 50.0), (Vec3::new(1.85, 0.95, 0.95), warm(), 36.0)], vec![Vec3::new(1.55, 2.1, 0.45)], vec![])
}

fn banner_small(m: &mut Mesh, top: Vec3) {
    block(m, &pt(CRIMSON, Mat::Cloth), top.x - 0.01, top.z - 0.1, top.x + 0.02, top.z + 0.1, top.y - 0.3, top.y);
}

// ------------------------------------------------------------------------------------------------
// Rohstoffe
// ------------------------------------------------------------------------------------------------

fn woodcutter() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    block(&mut m, &pt(0x5a3d26, Mat::WoodPlank), 0.25, 0.2, 1.15, 1.0, 0.07, 0.5);
    gable(&mut m, &pt(0x5a4a2e, Mat::Thatch), 0.25, 0.2, 1.15, 1.0, 0.5, 0.42, true, 0.1);
    door_z(&mut m, 0.55, 1.0, 0.2, 0.3);
    logs(&mut m, 1.0, 1.25, 0.8, 3);
    logs(&mut m, 0.3, 1.5, 0.6, 2);
    // Hackklotz mit Axt.
    m.frustum(&pt(0x6a4a30, Mat::Bark), Vec3::new(1.55, 0.07, 0.5), Vec3::new(1.55, 0.3, 0.5), 0.13, 0.12, 8);
    m.cuboid(&pt(0x5a3a22, Mat::WoodPlank), Vec3::new(-0.01, 0.0, -0.01), Vec3::new(0.01, 0.34, 0.01), Mat4::from_rotation_translation(Quat::from_rotation_z(0.9), Vec3::new(1.55, 0.3, 0.5)));
    block(&mut m, &pt(0x8b969e, Mat::Metal), 1.52, 0.47, 1.64, 0.53, 0.3, 0.4);
    // Gestapelte Bretter.
    block(&mut m, &pt(0x8a6a48, Mat::WoodPlank), 1.5, 1.0, 1.9, 1.3, 0.07, 0.3);
    finish(grid("bld_woodcutter", m, 2.0, 2.0, 1.4), vec![], vec![], vec![])
}

fn clay_pit() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    // Grube und Halden.
    block(&mut m, &pt(0x6a4630, Mat::Plain), 0.3, 0.3, 1.5, 1.4, 0.07, 0.1);
    block(&mut m, &pt(0x3a281c, Mat::Plain), 0.45, 0.45, 1.35, 1.25, 0.1, 0.11);
    for (x, z, r) in [(1.55, 0.5, 0.3), (1.6, 1.3, 0.24), (0.4, 1.6, 0.26)] {
        m.ellipsoid(&pt(0x8a5a3a, Mat::Rock).flat(), Vec3::new(x, 0.08, z), Vec3::new(r, r * 0.7, r), 7, 4);
    }
    // Schubkarre und Rampe aus Brettern.
    block(&mut m, &pt(0x6e4a2e, Mat::WoodPlank), 0.2, 0.9, 0.5, 1.0, 0.07, 0.12);
    for k in 0..4 {
        block(&mut m, &pt(0x6e4a2e, Mat::WoodPlank), 0.5 + k as f32 * 0.1, 0.92, 0.57 + k as f32 * 0.1, 0.98, 0.1 + k as f32 * 0.02, 0.12 + k as f32 * 0.02);
    }
    block(&mut m, &pt(DARK_WOOD, Mat::WoodPlank), 1.2, 1.6, 1.8, 1.9, 0.07, 0.5);
    gable(&mut m, &pt(0x5a4a2e, Mat::Thatch), 1.2, 1.6, 1.8, 1.9, 0.5, 0.3, true, 0.05);
    finish(grid("bld_clay_pit", m, 2.0, 2.0, 1.0), vec![], vec![], vec![])
}

fn charcoal_burner() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    m.ellipsoid(&pt(0x2a2624, Mat::Rock).flat(), Vec3::new(1.0, 0.1, 0.95), Vec3::new(0.75, 0.62, 0.75), 10, 5);
    // Glimmende Ritzen und Rauchabzug.
    for i in 0..7 {
        let a = i as f32 / 7.0 * TAU;
        let p = Vec3::new(1.0 + a.cos() * 0.58, 0.2 + (i % 3) as f32 * 0.14, 0.95 + a.sin() * 0.58);
        glow_orb(&mut m, p, 0.035);
    }
    m.frustum(&pt(0x3a3a3e, Mat::StoneBrick), Vec3::new(1.0, 0.62, 0.95), Vec3::new(1.0, 0.82, 0.95), 0.09, 0.07, 6);
    // Holzstapel und Hütte.
    logs(&mut m, 0.2, 1.65, 0.9, 3);
    block(&mut m, &pt(0x5a3d26, Mat::WoodPlank), 1.45, 1.4, 1.9, 1.85, 0.07, 0.4);
    gable(&mut m, &pt(0x5a4a2e, Mat::Thatch), 1.45, 1.4, 1.9, 1.85, 0.4, 0.3, false, 0.05);
    let mut s = grid("bld_charcoal_burner", m, 2.0, 2.0, 1.2);
    s.glow = vec![];
    finish(s, vec![(Vec3::new(1.0, 0.4, 1.3), Vec3::new(1.0, 0.45, 0.15), 44.0)], vec![Vec3::new(1.0, 0.85, 0.95)], vec![Vec3::new(1.0, 0.5, 1.5)])
}

fn iron_mine() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    crate::art3d::structures::rock_mound(&mut m, Vec3::new(0.9, 0.0, 0.8), Vec3::new(0.85, 0.75, 0.7), 4);
    crate::art3d::structures::rock_mound(&mut m, Vec3::new(0.4, 0.1, 0.5), Vec3::new(0.4, 0.45, 0.4), 5);
    // Stolleneingang mit Stützbalken auf der +x-Seite.
    let w = pt(WOOD, Mat::WoodPlank);
    block(&mut m, &pt(0x080706, Mat::Plain), 1.5, 0.55, 1.56, 1.05, 0.07, 0.5);
    block(&mut m, &w, 1.52, 0.48, 1.62, 0.56, 0.07, 0.6);
    block(&mut m, &w, 1.52, 1.04, 1.62, 1.12, 0.07, 0.6);
    block(&mut m, &w, 1.52, 0.46, 1.62, 1.14, 0.55, 0.64);
    glow_orb(&mut m, Vec3::new(1.64, 0.62, 0.5), 0.04);
    // Erzlore auf Schienen.
    block(&mut m, &pt(IRON, Mat::Metal), 1.7, 0.55, 1.74, 1.9, 0.07, 0.09);
    block(&mut m, &pt(IRON, Mat::Metal), 1.82, 0.55, 1.86, 1.9, 0.07, 0.09);
    block(&mut m, &pt(0x3a3028, Mat::Metal), 1.66, 1.2, 1.95, 1.6, 0.12, 0.3);
    for i in 0..4 {
        m.ellipsoid(&pt(0x7a3a28, Mat::Rock).flat(), Vec3::new(1.72 + (i % 2) as f32 * 0.12, 0.33, 1.3 + (i / 2) as f32 * 0.14), Vec3::splat(0.07), 5, 3);
    }
    finish(grid("bld_iron_mine", m, 2.0, 2.0, 1.4), vec![(Vec3::new(1.64, 0.62, 0.55), warm(), 30.0)], vec![], vec![])
}

fn herb_garden() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    let soil = pt(0x3a2b20, Mat::Plain);
    for (x0, z0) in [(0.2, 0.2), (1.1, 0.2), (0.2, 1.1), (1.1, 1.1)] {
        block(&mut m, &soil, x0, z0, x0 + 0.7, z0 + 0.7, 0.07, 0.12);
        for i in 0..5 {
            let c = [0x4a7a38, 0x8a5a9a, 0x6a8a3a, 0xc8b04a, 0x3c6a30][i % 5];
            m.ellipsoid(&pt(c, Mat::Leaves), Vec3::new(x0 + 0.15 + (i % 3) as f32 * 0.2, 0.17, z0 + 0.15 + (i / 3) as f32 * 0.3), Vec3::new(0.09, 0.08, 0.09), 5, 3);
        }
    }
    // Zaun und Trockengestell.
    for i in 0..8 {
        block(&mut m, &pt(WOOD, Mat::WoodPlank), 0.1 + i as f32 * 0.25, 1.92, 0.14 + i as f32 * 0.25, 1.96, 0.07, 0.3);
    }
    block(&mut m, &pt(WOOD, Mat::WoodPlank), 0.1, 1.93, 1.95, 1.95, 0.2, 0.23);
    finish(grid("bld_herb_garden", m, 2.0, 2.0, 0.9), vec![], vec![], vec![])
}

// ------------------------------------------------------------------------------------------------
// Nahrung
// ------------------------------------------------------------------------------------------------

fn farm() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 3.0);
    let soil = pt(0x3a2b20, Mat::Plain);
    let crops = pt(0x9a8040, Mat::Crops);
    block(&mut m, &soil, 0.1, 0.1, 2.9, 2.9, 0.07, 0.09);
    for (x0, z0, x1, z1) in [(1.5, 0.15, 2.85, 1.2), (1.5, 1.35, 2.85, 2.85), (0.15, 1.9, 1.35, 2.85)] {
        block(&mut m, &crops, x0, z0, x1, z1, 0.09, 0.2);
        for k in 0..6 {
            let x = x0 + (x1 - x0) * (k as f32 + 0.5) / 6.0;
            block(&mut m, &pt(0x6a5a2c, Mat::Crops), x - 0.015, z0, x + 0.015, z1, 0.2, 0.24);
        }
    }
    // Scheune.
    block(&mut m, &pt(0x6a4a30, Mat::WoodPlank), 0.2, 0.2, 1.3, 1.6, 0.09, 0.75);
    gable(&mut m, &pt(0x7a5a38, Mat::Thatch), 0.2, 0.2, 1.3, 1.6, 0.75, 0.6, false, 0.12);
    block(&mut m, &pt(0x0d0b0a, Mat::Plain), 0.5, 1.59, 1.0, 1.62, 0.09, 0.6);
    block(&mut m, &pt(DARK_WOOD, Mat::WoodPlank), 0.52, 1.6, 0.98, 1.64, 0.09, 0.58);
    // Heuhaufen und Karren.
    m.ellipsoid(&pt(0x9a8040, Mat::Thatch), Vec3::new(1.0, 0.2, 2.2), Vec3::new(0.28, 0.24, 0.28), 7, 4);
    block(&mut m, &pt(0x6e4a2e, Mat::WoodPlank), 0.3, 2.3, 0.8, 2.6, 0.12, 0.25);
    finish(grid("bld_farm", m, 3.0, 3.0, 1.7), vec![], vec![], vec![])
}

fn windmill() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    m.frustum(&pt(0x7a7064, Mat::StoneBrick), Vec3::new(1.0, 0.07, 1.0), Vec3::new(1.0, 1.35, 1.0), 0.52, 0.34, 10);
    m.frustum(&pt(ROOF, Mat::RoofTile).flat(), Vec3::new(1.0, 1.35, 1.0), Vec3::new(1.0, 1.75, 1.0), 0.4, 0.0, 10);
    door_z(&mut m, 1.0, 1.5, 0.24, 0.4);
    window(&mut m, Vec3::new(1.3, 0.8, 1.2), false, 0.1, 0.14);
    // Flügelkreuz zur Sichtseite (+x).
    let hub = Vec3::new(1.38, 1.38, 1.0);
    m.frustum(&pt(DARK_WOOD, Mat::WoodPlank), Vec3::new(1.0, 1.38, 1.0), hub + Vec3::new(0.08, 0.0, 0.0), 0.05, 0.05, 6);
    for k in 0..4 {
        let a = 0.5 + k as f32 * TAU / 4.0;
        let t = Mat4::from_rotation_translation(Quat::from_rotation_x(a), hub + Vec3::new(0.04, 0.0, 0.0));
        m.cuboid(&pt(WOOD, Mat::WoodPlank), Vec3::new(-0.015, 0.05, -0.02), Vec3::new(0.015, 0.95, 0.02), t);
        m.cuboid(&pt(0xb8a888, Mat::Cloth), Vec3::new(-0.01, 0.2, 0.02), Vec3::new(0.012, 0.9, 0.17), t);
    }
    sack(&mut m, 0.5, 1.7, 0xc8b898);
    sack(&mut m, 0.65, 1.78, 0xc8b898);
    finish(grid("bld_windmill", m, 2.0, 2.0, 2.2), vec![], vec![], vec![])
}

fn bakery() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    block(&mut m, &pt(0x8a7058, Mat::Plaster), 0.25, 0.3, 1.5, 1.6, 0.07, 0.62);
    timber_frame(&mut m, 0.25, 0.3, 1.5, 1.6, 0.07, 0.62);
    gable(&mut m, &pt(ROOF, Mat::RoofTile), 0.25, 0.3, 1.5, 1.6, 0.62, 0.5, false, 0.1);
    // Backofen mit Kamin.
    m.ellipsoid(&pt(BRICK, Mat::StoneBrick).flat(), Vec3::new(1.65, 0.25, 0.85), Vec3::new(0.3, 0.28, 0.3), 8, 4);
    block(&mut m, &pt(0x0d0b0a, Mat::Plain), 1.88, 0.72, 1.92, 0.98, 0.14, 0.34);
    glow_orb(&mut m, Vec3::new(1.93, 0.22, 0.85), 0.07);
    chimney(&mut m, 1.65, 0.8, 0.5, 1.2);
    door_z(&mut m, 0.6, 1.6, 0.22, 0.34);
    window(&mut m, Vec3::new(1.05, 0.34, 1.61), false, 0.14, 0.12);
    // Brote auf dem Tisch.
    block(&mut m, &pt(0x6e4a2e, Mat::WoodPlank), 1.0, 1.72, 1.6, 1.94, 0.12, 0.2);
    for i in 0..4 {
        m.ellipsoid(&pt(0xb8803a, Mat::Thatch), Vec3::new(1.1 + i as f32 * 0.13, 0.24, 1.83), Vec3::new(0.06, 0.04, 0.045), 5, 3);
    }
    finish(grid("bld_bakery", m, 2.0, 2.0, 1.7), vec![(Vec3::new(1.95, 0.23, 0.85), Vec3::new(1.0, 0.55, 0.2), 38.0)], vec![Vec3::new(1.65, 1.25, 0.8)], vec![Vec3::new(1.95, 0.25, 0.85)])
}

fn brewery() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    block(&mut m, &pt(0x7a6a52, Mat::Plaster), 0.25, 0.25, 2.0, 1.55, 0.07, 0.6);
    timber_frame(&mut m, 0.25, 0.25, 2.0, 1.55, 0.07, 0.6);
    gable(&mut m, &pt(0x5a4a3a, Mat::RoofTile), 0.25, 0.25, 2.0, 1.55, 0.6, 0.55, true, 0.1);
    door_z(&mut m, 0.7, 1.55, 0.24, 0.36);
    // Kupferkessel mit Dampfrohr.
    m.ellipsoid(&pt(COPPER, Mat::Metal), Vec3::new(2.45, 0.38, 0.8), Vec3::new(0.3, 0.32, 0.3), 9, 5);
    m.frustum(&pt(COPPER, Mat::Metal), Vec3::new(2.45, 0.62, 0.8), Vec3::new(2.45, 0.95, 0.8), 0.08, 0.05, 7);
    chimney(&mut m, 1.65, 0.5, 0.6, 1.25);
    for (x, z) in [(2.35, 1.5), (2.6, 1.55), (2.48, 1.7), (1.5, 1.85), (1.75, 1.85)] {
        barrel(&mut m, x, z, 0.26);
    }
    window(&mut m, Vec3::new(1.5, 0.35, 1.56), false, 0.14, 0.13);
    finish(grid("bld_brewery", m, 3.0, 2.0, 1.8), vec![(Vec3::new(1.5, 0.35, 1.6), warm(), 34.0)], vec![Vec3::new(1.65, 1.3, 0.5), Vec3::new(2.45, 1.0, 0.8)], vec![])
}

fn sheep_farm() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 3.0);
    block(&mut m, &pt(0x3a5226, Mat::Leaves), 0.1, 0.1, 2.9, 2.9, 0.07, 0.1);
    // Zaun.
    let post = pt(WOOD, Mat::WoodPlank);
    for i in 0..12 {
        let t = 0.15 + i as f32 * 0.24;
        for (x, z) in [(t, 0.12), (t, 2.88), (0.12, t), (2.88, t)] {
            block(&mut m, &post, x - 0.025, z - 0.025, x + 0.025, z + 0.025, 0.1, 0.34);
        }
    }
    for (a, b) in [((0.1, 0.1), (2.9, 0.14)), ((0.1, 2.86), (2.9, 2.9)), ((0.1, 0.1), (0.14, 2.9)), ((2.86, 0.1), (2.9, 2.9))] {
        block(&mut m, &post, a.0, a.1, b.0, b.1, 0.22, 0.26);
    }
    // Schafe.
    for (i, (x, z)) in [(1.2, 1.6), (1.7, 1.2), (2.1, 1.9), (1.4, 2.3), (2.4, 0.8), (0.9, 1.1)].iter().enumerate() {
        let a = i as f32 * 1.3;
        m.ellipsoid(&pt(0xe6e0d0, Mat::Cloth), Vec3::new(*x, 0.24, *z), Vec3::new(0.17, 0.12, 0.12), 6, 4);
        m.ellipsoid(&pt(0x2a2420, Mat::Plain), Vec3::new(x + a.cos() * 0.17, 0.27, z + a.sin() * 0.17), Vec3::splat(0.05), 5, 3);
        for (dx, dz) in [(-0.08, -0.05), (0.08, -0.05), (-0.08, 0.05), (0.08, 0.05)] {
            block(&mut m, &pt(0x2a2420, Mat::Plain), x + dx - 0.01, z + dz - 0.01, x + dx + 0.01, z + dz + 0.01, 0.1, 0.17);
        }
    }
    // Stall.
    block(&mut m, &pt(0x5a3d26, Mat::WoodPlank), 0.25, 0.25, 1.0, 0.85, 0.1, 0.5);
    gable(&mut m, &pt(THATCH, Mat::Thatch), 0.25, 0.25, 1.0, 0.85, 0.5, 0.3, true, 0.06);
    finish(grid("bld_sheep_farm", m, 3.0, 3.0, 1.1), vec![], vec![], vec![])
}

// ------------------------------------------------------------------------------------------------
// Handwerk
// ------------------------------------------------------------------------------------------------

fn sawmill() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    // Offene Halle auf Pfosten.
    let post = pt(WOOD, Mat::WoodPlank);
    for (x, z) in [(0.25, 0.3), (1.55, 0.3), (0.25, 1.3), (1.55, 1.3)] {
        m.frustum(&post, Vec3::new(x, 0.07, z), Vec3::new(x, 0.75, z), 0.05, 0.045, 6);
    }
    gable(&mut m, &pt(0x5a4a2e, Mat::Thatch), 0.15, 0.2, 1.65, 1.4, 0.75, 0.4, true, 0.1);
    // Sägetisch mit Blatt.
    block(&mut m, &pt(0x6e4a2e, Mat::WoodPlank), 0.5, 0.5, 1.3, 1.0, 0.25, 0.32);
    m.frustum(&pt(0x8b969e, Mat::Metal), Vec3::new(0.9, 0.35, 0.74), Vec3::new(0.9, 0.35, 0.78), 0.2, 0.2, 14);
    m.frustum(&pt(IRON, Mat::Metal), Vec3::new(0.9, 0.35, 0.74), Vec3::new(0.9, 0.35, 0.82), 0.04, 0.04, 6);
    // Stämme und Bretterstapel.
    logs(&mut m, 1.85, 0.3, 0.9, 4);
    logs(&mut m, 1.9, 1.3, 0.8, 3);
    for k in 0..6 {
        block(&mut m, &pt(0x9a7a52, Mat::WoodPlank), 0.4, 1.55 + k as f32 * 0.0, 1.5, 1.8, 0.07 + k as f32 * 0.035, 0.1 + k as f32 * 0.035);
    }
    finish(grid("bld_sawmill", m, 3.0, 2.0, 1.4), vec![], vec![Vec3::new(0.9, 0.7, 0.6)], vec![])
}

fn brickworks() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    // Brennofen.
    m.ellipsoid(&pt(BRICK, Mat::StoneBrick).flat(), Vec3::new(0.95, 0.12, 0.85), Vec3::new(0.62, 0.65, 0.58), 10, 5);
    block(&mut m, &pt(0x0d0b0a, Mat::Plain), 0.75, 1.4, 1.15, 1.46, 0.07, 0.42);
    glow_orb(&mut m, Vec3::new(0.95, 0.22, 1.44), 0.08);
    chimney(&mut m, 1.0, 0.75, 0.6, 1.5);
    // Ziegelstapel.
    for (x, z, n) in [(1.9, 0.3, 4), (2.3, 0.3, 3), (1.9, 0.75, 3)] {
        for r in 0..n {
            block(&mut m, &pt(0xa4472f, Mat::StoneBrick), x, z + r as f32 * 0.0, x + 0.34, z + 0.38, 0.07 + r as f32 * 0.07, 0.14 + r as f32 * 0.07);
        }
    }
    // Lehmhaufen und Hütte.
    m.ellipsoid(&pt(0x8a5a3a, Mat::Rock).flat(), Vec3::new(2.4, 0.12, 1.5), Vec3::new(0.3, 0.2, 0.26), 7, 4);
    block(&mut m, &pt(0x5a3d26, Mat::WoodPlank), 1.55, 1.3, 2.05, 1.8, 0.07, 0.4);
    gable(&mut m, &pt(0x5a4a2e, Mat::Thatch), 1.55, 1.3, 2.05, 1.8, 0.4, 0.28, false, 0.05);
    finish(grid("bld_brickworks", m, 3.0, 2.0, 1.9), vec![(Vec3::new(0.95, 0.22, 1.5), Vec3::new(1.0, 0.45, 0.15), 40.0)], vec![Vec3::new(1.0, 1.55, 0.75)], vec![Vec3::new(0.95, 0.25, 1.5)])
}

fn smelter() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    // Schachtofen.
    m.frustum(&pt(0x5a5a60, Mat::StoneBrick), Vec3::new(0.95, 0.07, 0.8), Vec3::new(0.95, 1.2, 0.8), 0.62, 0.4, 4);
    m.frustum(&pt(0x3a3a40, Mat::StoneBrick), Vec3::new(0.95, 1.2, 0.8), Vec3::new(0.95, 1.7, 0.8), 0.2, 0.15, 6);
    block(&mut m, &pt(0x0d0b0a, Mat::Plain), 0.7, 1.28, 1.2, 1.34, 0.12, 0.5);
    glow_orb(&mut m, Vec3::new(0.95, 0.3, 1.31), 0.1);
    // Barren und Schlackehalde.
    for k in 0..4 {
        block(&mut m, &pt(0x8b969e, Mat::Metal), 1.5 + (k % 2) as f32 * 0.22, 1.35 + (k / 2) as f32 * 0.13, 1.68 + (k % 2) as f32 * 0.22, 1.45 + (k / 2) as f32 * 0.13, 0.07, 0.14);
    }
    m.ellipsoid(&pt(0x2a2a2e, Mat::Rock).flat(), Vec3::new(0.35, 0.1, 1.55), Vec3::new(0.3, 0.18, 0.26), 7, 4);
    finish(grid("bld_smelter", m, 2.0, 2.0, 2.1), vec![(Vec3::new(0.95, 0.3, 1.36), Vec3::new(1.0, 0.45, 0.12), 56.0)], vec![Vec3::new(0.95, 1.75, 0.8)], vec![Vec3::new(0.95, 0.32, 1.36)])
}

fn smithy() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    block(&mut m, &pt(STONE, Mat::StoneBrick), 0.25, 0.25, 1.8, 1.55, 0.07, 0.55);
    block(&mut m, &pt(0x6a5a46, Mat::Plaster), 0.27, 0.27, 1.78, 1.53, 0.55, 0.85);
    gable(&mut m, &pt(0x4a3a30, Mat::RoofTile), 0.25, 0.25, 1.8, 1.55, 0.85, 0.5, true, 0.1);
    // Esse mit Kamin und offene Werkbank.
    block(&mut m, &pt(0x4a4a50, Mat::StoneBrick), 1.9, 0.5, 2.5, 1.05, 0.07, 0.4);
    block(&mut m, &pt(0x0d0b0a, Mat::Plain), 2.0, 1.04, 2.4, 1.08, 0.2, 0.34);
    glow_orb(&mut m, Vec3::new(2.2, 0.28, 1.08), 0.09);
    chimney(&mut m, 2.2, 0.75, 0.4, 1.5);
    block(&mut m, &pt(IRON, Mat::Metal), 2.35, 1.4, 2.65, 1.55, 0.07, 0.26);
    block(&mut m, &pt(IRON, Mat::Metal), 2.3, 1.38, 2.7, 1.58, 0.26, 0.31);
    door_z(&mut m, 0.8, 1.55, 0.3, 0.4);
    window(&mut m, Vec3::new(1.4, 0.4, 1.56), false, 0.14, 0.13);
    // Werkzeuge an der Wand.
    for i in 0..4 {
        block(&mut m, &pt(0x8b969e, Mat::Metal), 1.82, 0.5 + i as f32 * 0.2, 1.85, 0.54 + i as f32 * 0.2, 0.55, 0.8);
    }
    barrel(&mut m, 2.6, 1.75, 0.22);
    finish(grid("bld_smithy", m, 3.0, 2.0, 2.0), vec![(Vec3::new(2.2, 0.3, 1.14), Vec3::new(1.0, 0.45, 0.12), 58.0), (Vec3::new(1.4, 0.4, 1.6), warm(), 30.0)], vec![Vec3::new(2.2, 1.55, 0.75)], vec![Vec3::new(2.2, 0.32, 1.12)])
}

fn armory() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    block(&mut m, &pt(STONE, Mat::StoneBrick), 0.25, 0.25, 2.2, 1.55, 0.07, 0.75);
    gable(&mut m, &pt(0x4a4a50, Mat::RoofTile), 0.25, 0.25, 2.2, 1.55, 0.75, 0.5, true, 0.1);
    merlons(&mut m, &pt(STONE, Mat::StoneBrick), Vec2::new(0.35, 1.45), Vec2::new(2.1, 1.45), 0.75, 0.1);
    door_z(&mut m, 1.2, 1.55, 0.34, 0.5);
    window(&mut m, Vec3::new(2.21, 0.45, 0.75), true, 0.14, 0.2);
    // Rüstungsständer vor dem Haus.
    for (i, x) in [0.6, 1.9, 2.55].iter().enumerate() {
        let z = if i == 2 { 1.0 } else { 1.8 };
        block(&mut m, &pt(DARK_WOOD, Mat::WoodPlank), x - 0.02, z - 0.02, x + 0.02, z + 0.02, 0.07, 0.5);
        m.ellipsoid(&pt(0x8b969e, Mat::Metal), Vec3::new(*x, 0.42, z), Vec3::new(0.11, 0.14, 0.08), 6, 4);
        m.ellipsoid(&pt(0xb4bec4, Mat::Metal), Vec3::new(*x, 0.62, z), Vec3::new(0.07, 0.08, 0.07), 6, 4);
    }
    // Schild und Banner.
    block(&mut m, &pt(CRIMSON, Mat::Cloth), 1.0, 1.56, 1.4, 1.58, 0.55, 0.72);
    m.frustum(&pt(DARK_WOOD, Mat::WoodPlank), Vec3::new(2.4, 0.07, 0.4), Vec3::new(2.4, 1.5, 0.4), 0.025, 0.02, 4);
    block(&mut m, &pt(CRIMSON, Mat::Cloth), 2.4, 0.3, 2.43, 0.62, 0.95, 1.45);
    finish(grid("bld_armory", m, 3.0, 2.0, 1.7), vec![(Vec3::new(1.2, 0.5, 1.6), warm(), 30.0)], vec![Vec3::new(2.0, 1.2, 0.5)], vec![])
}

fn weaver() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    block(&mut m, &pt(0x7a6a52, Mat::Plaster), 0.25, 0.25, 1.85, 1.5, 0.07, 0.58);
    timber_frame(&mut m, 0.25, 0.25, 1.85, 1.5, 0.07, 0.58);
    gable(&mut m, &pt(THATCH, Mat::Thatch), 0.25, 0.25, 1.85, 1.5, 0.58, 0.5, true, 0.1);
    door_z(&mut m, 0.7, 1.5, 0.22, 0.34);
    window(&mut m, Vec3::new(1.3, 0.34, 1.51), false, 0.16, 0.12);
    // Webstuhl im Freien.
    let wood = pt(WOOD, Mat::WoodPlank);
    for z in [0.3, 0.9] {
        block(&mut m, &wood, 2.2, z, 2.26, z + 0.06, 0.07, 0.85);
        block(&mut m, &wood, 2.7, z, 2.76, z + 0.06, 0.07, 0.85);
    }
    block(&mut m, &wood, 2.2, 0.3, 2.76, 0.36, 0.78, 0.84);
    block(&mut m, &wood, 2.2, 0.9, 2.76, 0.96, 0.78, 0.84);
    for k in 0..8 {
        block(&mut m, &pt(0xd6d0c0, Mat::Cloth), 2.24 + k as f32 * 0.065, 0.36, 2.255 + k as f32 * 0.065, 0.9, 0.2, 0.78);
    }
    block(&mut m, &pt(0x3a5a8a, Mat::Cloth), 2.24, 0.36, 2.72, 0.9, 0.2, 0.42);
    // Wollballen und Stoffbahn.
    for (x, z) in [(1.0, 1.75), (1.25, 1.8), (1.12, 1.9)] {
        m.ellipsoid(&pt(0xe6e0d0, Mat::Cloth), Vec3::new(x, 0.15, z), Vec3::splat(0.12), 6, 4);
    }
    block(&mut m, &pt(0x8a2a2a, Mat::Cloth), 1.95, 1.2, 2.5, 1.24, 0.3, 0.72);
    finish(grid("bld_weaver", m, 3.0, 2.0, 1.6), vec![(Vec3::new(1.3, 0.34, 1.56), warm(), 28.0)], vec![], vec![])
}

fn tailor() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 2.0, 2.0);
    block(&mut m, &pt(0x7a6a52, Mat::Plaster), 0.25, 0.3, 1.45, 1.55, 0.07, 0.6);
    timber_frame(&mut m, 0.25, 0.3, 1.45, 1.55, 0.07, 0.6);
    gable(&mut m, &pt(ROOF, Mat::RoofTile), 0.25, 0.3, 1.45, 1.55, 0.6, 0.5, false, 0.1);
    door_z(&mut m, 0.55, 1.55, 0.22, 0.34);
    window(&mut m, Vec3::new(1.0, 0.34, 1.56), false, 0.18, 0.14);
    // Schneiderpuppe und Stoffballen.
    m.frustum(&pt(DARK_WOOD, Mat::WoodPlank), Vec3::new(1.7, 0.07, 1.0), Vec3::new(1.7, 0.5, 1.0), 0.025, 0.025, 4);
    m.ellipsoid(&pt(0xb8a888, Mat::Cloth), Vec3::new(1.7, 0.62, 1.0), Vec3::new(0.1, 0.17, 0.08), 6, 4);
    m.ellipsoid(&pt(0xb8a888, Mat::Cloth), Vec3::new(1.7, 0.84, 1.0), Vec3::splat(0.045), 5, 3);
    for (i, c) in [0x3a5a8a, 0x8a2a2a, 0x4a7a38].iter().enumerate() {
        m.frustum(&pt(*c, Mat::Cloth), Vec3::new(0.3 + i as f32 * 0.2, 0.1, 1.8), Vec3::new(0.3 + i as f32 * 0.2, 0.34, 1.8), 0.07, 0.07, 6);
    }
    finish(grid("bld_tailor", m, 2.0, 2.0, 1.5), vec![(Vec3::new(1.0, 0.34, 1.6), warm(), 26.0)], vec![Vec3::new(1.2, 1.1, 0.5)], vec![])
}

// ------------------------------------------------------------------------------------------------
// Stadt
// ------------------------------------------------------------------------------------------------

fn storehouse() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    block(&mut m, &pt(0x6a4a30, Mat::WoodPlank), 0.2, 0.2, 2.3, 1.5, 0.07, 0.75);
    timber_frame(&mut m, 0.2, 0.2, 2.3, 1.5, 0.07, 0.75);
    gable(&mut m, &pt(0x4a3a30, Mat::RoofTile), 0.2, 0.2, 2.3, 1.5, 0.75, 0.62, true, 0.12);
    // Großes Doppeltor.
    block(&mut m, &pt(0x0d0b0a, Mat::Plain), 0.95, 1.49, 1.65, 1.51, 0.07, 0.62);
    block(&mut m, &pt(DARK_WOOD, Mat::WoodPlank), 0.97, 1.5, 1.3, 1.54, 0.07, 0.6);
    block(&mut m, &pt(DARK_WOOD, Mat::WoodPlank), 1.3, 1.5, 1.63, 1.54, 0.07, 0.6);
    // Lastenkran-Ausleger.
    block(&mut m, &pt(WOOD, Mat::WoodPlank), 2.28, 0.6, 2.32, 0.64, 0.5, 0.95);
    block(&mut m, &pt(WOOD, Mat::WoodPlank), 2.28, 0.4, 2.6, 0.44, 0.9, 0.95);
    // Waren davor.
    for (x, z) in [(0.4, 1.7), (0.7, 1.75), (2.0, 1.7), (2.3, 1.7)] {
        crate_box(&mut m, x, z, 0.22);
    }
    for (x, z) in [(2.6, 1.0), (2.7, 1.25), (2.62, 1.45)] {
        barrel(&mut m, x, z, 0.26);
    }
    sack(&mut m, 1.9, 1.8, 0xb8a070);
    sack(&mut m, 2.05, 1.88, 0xb8a070);
    finish(grid("bld_storehouse", m, 3.0, 2.0, 1.7), vec![(Vec3::new(1.3, 0.55, 1.58), warm(), 30.0)], vec![], vec![])
}

fn market() -> Structure {
    let mut m = Mesh::default();
    // Gepflasterter Platz.
    block(&mut m, &pt(0x55524c, Mat::StoneBrick), 0.05, 0.05, 2.95, 2.95, 0.0, 0.06);
    // Vier Stände mit farbigen Markisen.
    let stands = [(0.25, 0.25, 0xa83a2a), (1.65, 0.25, 0x3a5a8a), (0.25, 2.0, 0x4a7a38), (1.9, 2.05, 0xc8a03a)];
    for (x, z, c) in stands {
        let (w, d) = (0.95, 0.7);
        block(&mut m, &pt(0x6e4a2e, Mat::WoodPlank), x, z + 0.25, x + w, z + d, 0.06, 0.28);
        for (px, pz) in [(x, z + 0.08), (x + w - 0.04, z + 0.08), (x, z + d - 0.04), (x + w - 0.04, z + d - 0.04)] {
            block(&mut m, &pt(WOOD, Mat::WoodPlank), px, pz, px + 0.04, pz + 0.04, 0.06, 0.62);
        }
        m.quad(&pt(c, Mat::Cloth), Vec3::new(x - 0.05, 0.58, z + d + 0.05), Vec3::new(x + w + 0.05, 0.58, z + d + 0.05), Vec3::new(x + w + 0.05, 0.74, z + 0.02), Vec3::new(x - 0.05, 0.74, z + 0.02));
        m.quad(&pt(c, Mat::Cloth), Vec3::new(x - 0.05, 0.74, z + 0.02), Vec3::new(x + w + 0.05, 0.74, z + 0.02), Vec3::new(x + w + 0.05, 0.58, z + d + 0.05), Vec3::new(x - 0.05, 0.58, z + d + 0.05));
        for k in 0..4 {
            m.ellipsoid(&pt([0xc87a2a, 0xa83a2a, 0x8a9a3a, 0xd8c8a8][k], Mat::Leaves), Vec3::new(x + 0.15 + k as f32 * 0.2, 0.34, z + 0.5), Vec3::splat(0.06), 5, 3);
        }
    }
    // Marktbrunnen / Fahnenmast in der Mitte.
    m.frustum(&pt(0x6a675f, Mat::StoneBrick), Vec3::new(1.5, 0.06, 1.5), Vec3::new(1.5, 0.3, 1.5), 0.26, 0.26, 10);
    m.frustum(&pt(0x2a4a5a, Mat::Plain), Vec3::new(1.5, 0.29, 1.5), Vec3::new(1.5, 0.31, 1.5), 0.2, 0.2, 10);
    m.frustum(&pt(DARK_WOOD, Mat::WoodPlank), Vec3::new(1.5, 0.3, 1.5), Vec3::new(1.5, 1.7, 1.5), 0.03, 0.025, 5);
    block(&mut m, &pt(CRIMSON, Mat::Cloth), 1.5, 1.35, 1.52, 1.8, 1.25, 1.65);
    m.ellipsoid(&pt(GOLD, Mat::Gold), Vec3::new(1.5, 1.72, 1.5), Vec3::splat(0.05), 5, 3);
    for (x, z) in [(1.0, 1.2), (2.2, 1.6), (2.1, 1.1)] {
        crate_box(&mut m, x, z, 0.2);
    }
    barrel(&mut m, 2.4, 1.4, 0.24);
    finish(grid("bld_market", m, 3.0, 3.0, 1.9), vec![(Vec3::new(1.5, 1.2, 1.5), Vec3::new(1.0, 0.7, 0.35), 60.0)], vec![], vec![])
}

fn temple() -> Structure {
    let mut m = Mesh::default();
    plinth(&mut m, 3.0, 2.0);
    block(&mut m, &pt(0x6a675f, Mat::StoneBrick), 0.3, 0.25, 2.1, 1.65, 0.07, 0.75);
    gable(&mut m, &pt(0x4a4a50, Mat::RoofTile), 0.3, 0.25, 2.1, 1.65, 0.75, 0.55, true, 0.1);
    // Glühende Spitzbogenfenster.
    for x in [0.7, 1.2, 1.7] {
        window(&mut m, Vec3::new(x, 0.45, 1.66), false, 0.14, 0.3);
        m.frustum(&pt(WARM, Mat::Glow).glow(1.0), Vec3::new(x, 0.6, 1.67), Vec3::new(x, 0.66, 1.67), 0.07, 0.0, 4);
    }
    door_z(&mut m, 1.2, 1.65, 0.3, 0.38);
    // Glockenturm.
    block(&mut m, &pt(0x6a675f, Mat::StoneBrick), 2.1, 0.3, 2.75, 0.95, 0.07, 1.4);
    block(&mut m, &pt(DARK_STONE, Mat::StoneBrick), 2.2, 0.4, 2.65, 0.85, 1.4, 1.62);
    pyramid(&mut m, &pt(ROOF, Mat::RoofTile), 2.08, 0.28, 2.77, 0.97, 1.62, 0.55);
    m.frustum(&pt(GOLD, Mat::Gold), Vec3::new(2.425, 2.17, 0.625), Vec3::new(2.425, 2.4, 0.625), 0.025, 0.0, 4);
    block(&mut m, &pt(0x0d0b0a, Mat::Plain), 2.32, 0.94, 2.52, 0.97, 1.42, 1.58);
    // Weihrauchbecken.
    m.frustum(&pt(IRON, Mat::Metal), Vec3::new(0.65, 0.07, 1.95), Vec3::new(0.65, 0.3, 1.95), 0.03, 0.03, 4);
    m.frustum(&pt(IRON, Mat::Metal), Vec3::new(0.65, 0.3, 1.95), Vec3::new(0.65, 0.38, 1.95), 0.1, 0.12, 8);
    m.frustum(&pt(FLAME, Mat::Glow).glow(1.0), Vec3::new(0.65, 0.38, 1.95), Vec3::new(0.65, 0.55, 1.95), 0.07, 0.0, 5);
    banner_small(&mut m, Vec3::new(2.14, 1.2, 1.3));
    let mut s = grid("bld_temple", m, 3.0, 2.0, 2.8);
    s.glow = vec![];
    finish(s, vec![(Vec3::new(1.2, 0.5, 1.75), Vec3::new(1.0, 0.7, 0.35), 60.0), (Vec3::new(0.65, 0.55, 1.95), Vec3::new(1.0, 0.5, 0.15), 36.0)], vec![Vec3::new(0.65, 0.6, 1.95)], vec![Vec3::new(0.65, 0.5, 1.95)])
}

// ------------------------------------------------------------------------------------------------
// Befestigungen
// ------------------------------------------------------------------------------------------------

/// Mauerteil mit Anschlüssen: Bit 0 = +x, 1 = −x, 2 = +z, 3 = −z.
fn wall_piece(mask: u32) -> Structure {
    let mut m = Mesh::default();
    let brick = pt(0x5a5a60, Mat::StoneBrick);
    let cap = pt(0x45454b, Mat::StoneBrick);
    let (lo, hi) = (0.28, 0.72);
    let h = 0.95;
    // Pfosten in der Mitte (leicht höher, mit Zinnenkranz).
    block(&mut m, &brick, lo - 0.02, lo - 0.02, hi + 0.02, hi + 0.02, 0.0, h + 0.12);
    block(&mut m, &cap, lo - 0.05, lo - 0.05, hi + 0.05, hi + 0.05, h + 0.12, h + 0.2);
    let arms = [(mask & 1 != 0, hi, lo, 1.0, hi, true), (mask & 2 != 0, 0.0, lo, lo, hi, true), (mask & 4 != 0, lo, hi, hi, 1.0, false), (mask & 8 != 0, lo, 0.0, hi, lo, false)];
    for (on, x0, z0, x1, z1, along_x) in arms {
        if !on {
            continue;
        }
        block(&mut m, &brick, x0, z0 + 0.04, x1, z1 - 0.04, 0.0, h);
        // Wehrgang-Brüstung mit Zinnen.
        block(&mut m, &cap, x0, z0 + 0.02, x1, z1 - 0.02, h, h + 0.06);
        let (a, b) = if along_x { (Vec2::new(x0 + 0.05, (z0 + z1) * 0.5), Vec2::new(x1 - 0.05, (z0 + z1) * 0.5)) } else { (Vec2::new((x0 + x1) * 0.5, z0 + 0.05), Vec2::new((x0 + x1) * 0.5, z1 - 0.05)) };
        merlons(&mut m, &cap, a, b, h + 0.06, 0.1);
    }
    if mask == 0 {
        // Einzelner Mauerstumpf wird zum kleinen Rundpfeiler.
        m.frustum(&pt(0x6a6a70, Mat::StoneBrick), Vec3::new(0.5, h + 0.2, 0.5), Vec3::new(0.5, h + 0.28, 0.5), 0.12, 0.1, 8);
    }
    let mut s = grid(&format!("bld_wall_m{mask}"), m, 1.0, 1.0, 1.2);
    s.cell.1 += 4;
    s.anchor.y += 4.0;
    s
}

/// Diagonale Mauerverbindung in einem 2×2-Block: `dir` 0 = von (0,0) nach (1,1), 1 = von (0,1) nach (1,0).
fn wall_diagonal(dir: u32) -> Structure {
    let mut m = Mesh::default();
    let brick = pt(0x5a5a60, Mat::StoneBrick);
    let cap = pt(0x45454b, Mat::StoneBrick);
    let (a, b) = if dir == 0 { (Vec2::new(0.5, 0.5), Vec2::new(1.5, 1.5)) } else { (Vec2::new(0.5, 1.5), Vec2::new(1.5, 0.5)) };
    let c = (a + b) * 0.5;
    let len = (b - a).length();
    let ang = (-(b - a).y).atan2((b - a).x);
    let t = Mat4::from_rotation_translation(Quat::from_rotation_y(ang), Vec3::new(c.x, 0.0, c.y));
    m.cuboid(&brick, Vec3::new(-len * 0.5, 0.0, -0.18), Vec3::new(len * 0.5, 0.95, 0.18), t);
    m.cuboid(&cap, Vec3::new(-len * 0.5, 0.95, -0.2), Vec3::new(len * 0.5, 1.01, 0.2), t);
    for k in 0..5 {
        let f = (k as f32 + 0.5) / 5.0 - 0.5;
        m.cuboid(&cap, Vec3::new(f * len - 0.05, 1.01, -0.05), Vec3::new(f * len + 0.05, 1.11, 0.05), t);
    }
    let mut s = grid(&format!("bld_wall_d{dir}"), m, 2.0, 2.0, 1.2);
    s.cell.1 += 4;
    s.anchor.y += 4.0;
    s
}

/// Tor: Wandachse `along_x` (Mauer verläuft in x-Richtung), offen oder geschlossen.
fn gate(along_x: bool, open: bool) -> Structure {
    let mut m = Mesh::default();
    let brick = pt(0x5a5a60, Mat::StoneBrick);
    let cap = pt(0x45454b, Mat::StoneBrick);
    let door = pt(0x6a4a2e, Mat::WoodPlank);
    let strap = pt(IRON, Mat::Metal);
    let h = 1.4;
    let red = pt(CRIMSON, Mat::Cloth);
    // Modelliert für Achse x (Pfosten bei x∈[0,0.26] und [0.74,1]); für z-Achse gespiegelt über Vertauschen.
    let mut b = |x0: f32, z0: f32, x1: f32, z1: f32, y0: f32, y1: f32, p: &ashen_engine::bake::Paint| {
        if along_x {
            block(&mut m, p, x0, z0, x1, z1, y0, y1);
        } else {
            block(&mut m, p, z0, x0, z1, x1, y0, y1);
        }
    };
    b(0.0, 0.2, 0.26, 0.8, 0.0, h, &brick);
    b(0.74, 0.2, 1.0, 0.8, 0.0, h, &brick);
    b(-0.01, 0.17, 0.3, 0.83, h, h + 0.08, &cap);
    b(0.7, 0.17, 1.01, 0.83, h, h + 0.08, &cap);
    // Torbogen-Sturz mit Zinnen.
    b(0.0, 0.22, 1.0, 0.78, 1.0, h, &brick);
    b(0.28, 0.3, 0.72, 0.7, 0.96, 1.02, &pt(GOLD, Mat::Gold));
    b(0.05, 0.8, 0.21, 0.83, 0.55, 1.3, &red);
    b(0.79, 0.8, 0.95, 0.83, 0.55, 1.3, &red);
    b(-0.01, 0.2, 1.01, 0.8, h, h + 0.06, &cap);
    for i in 0..5 {
        let x = 0.1 + i as f32 * 0.2;
        b(x - 0.05, 0.4, x + 0.05, 0.6, h + 0.06, h + 0.17, &cap);
    }
    if open {
        // Torflügel aufgeschwenkt an den Pfosten.
        b(0.26, 0.55, 0.3, 0.9, 0.0, 0.95, &door);
        b(0.7, 0.55, 0.74, 0.9, 0.0, 0.95, &door);
        b(0.26, 0.6, 0.3, 0.64, 0.3, 0.34, &strap);
        b(0.7, 0.6, 0.74, 0.64, 0.3, 0.34, &strap);
        // Dunkler Durchgang.
        b(0.3, 0.3, 0.7, 0.7, 0.0, 0.02, &pt(0x1a1612, Mat::Plain));
    } else {
        b(0.26, 0.42, 0.74, 0.58, 0.0, 0.98, &door);
        b(0.26, 0.405, 0.74, 0.43, 0.2, 0.25, &strap);
        b(0.26, 0.405, 0.74, 0.43, 0.55, 0.6, &strap);
        b(0.49, 0.4, 0.51, 0.6, 0.0, 0.98, &strap);
    }
    // Laternen an den Pfosten.
    let lamp = |m: &mut Mesh, x: f32, z: f32| {
        glow_orb(m, if along_x { Vec3::new(x, 0.95, z) } else { Vec3::new(z, 0.95, x) }, 0.04);
    };
    lamp(&mut m, 0.13, 0.85);
    lamp(&mut m, 0.87, 0.85);
    let name = format!("bld_gate_{}_{}", if along_x { "x" } else { "z" }, if open { "open" } else { "closed" });
    let mut s = grid(&name, m, 1.0, 1.0, 1.7);
    s.cell.1 += 4;
    s.anchor.y += 4.0;
    let (l1, l2) = if along_x { (Vec3::new(0.13, 0.95, 0.85), Vec3::new(0.87, 0.95, 0.85)) } else { (Vec3::new(0.85, 0.95, 0.13), Vec3::new(0.85, 0.95, 0.87)) };
    s.lights = vec![(l1, Vec3::new(1.0, 0.6, 0.25), 36.0), (l2, Vec3::new(1.0, 0.6, 0.25), 36.0)];
    s
}

fn wall_tower() -> Structure {
    let mut m = Mesh::default();
    block(&mut m, &pt(ROCK, Mat::Rock), 0.1, 0.1, 1.9, 1.9, 0.0, 0.1);
    let brick = pt(0x5a5a60, Mat::StoneBrick);
    m.frustum(&brick, Vec3::new(1.0, 0.1, 1.0), Vec3::new(1.0, 1.85, 1.0), 0.8, 0.66, 12);
    m.frustum(&pt(0x45454b, Mat::StoneBrick), Vec3::new(1.0, 1.85, 1.0), Vec3::new(1.0, 2.0, 1.0), 0.78, 0.78, 12);
    // Zinnenkranz.
    for i in 0..10 {
        let a = i as f32 / 10.0 * TAU;
        let c = Vec3::new(1.0 + a.cos() * 0.72, 2.0, 1.0 + a.sin() * 0.72);
        block(&mut m, &pt(0x45454b, Mat::StoneBrick), c.x - 0.07, c.z - 0.07, c.x + 0.07, c.z + 0.07, 2.0, 2.18);
    }
    // Schießscharten.
    for (x, z, ax) in [(1.66, 1.0, true), (1.0, 1.66, false), (1.62, 1.35, true), (1.35, 1.62, false)] {
        window(&mut m, Vec3::new(x, 1.3, z), ax, 0.08, 0.26);
    }
    // Wachfeuer und Banner.
    m.frustum(&pt(IRON, Mat::Metal), Vec3::new(1.0, 2.0, 1.0), Vec3::new(1.0, 2.12, 1.0), 0.14, 0.2, 8);
    for i in 0..4 {
        let a = i as f32 / 4.0 * TAU;
        let base = Vec3::new(1.0 + a.cos() * 0.07, 2.12, 1.0 + a.sin() * 0.07);
        m.frustum(&pt(FLAME, Mat::Glow).glow(1.0), base, base + Vec3::new(0.0, 0.2 + (i % 2) as f32 * 0.08, 0.0), 0.05, 0.0, 4);
    }
    m.frustum(&pt(DARK_WOOD, Mat::WoodPlank), Vec3::new(1.55, 1.9, 0.62), Vec3::new(1.55, 2.7, 0.62), 0.02, 0.015, 4);
    block(&mut m, &pt(CRIMSON, Mat::Cloth), 1.55, 0.45, 1.57, 0.8, 2.25, 2.68);
    // Mauer-Anschlüsse als Stumpf an den Seiten sind Sache der Nachbarn; Tür am Fuß.
    block(&mut m, &pt(0x0d0b0a, Mat::Plain), 0.9, 1.72, 1.2, 1.78, 0.1, 0.5);
    let mut s = grid("bld_wall_tower", m, 2.0, 2.0, 3.0);
    s.lights = vec![(Vec3::new(1.0, 2.25, 1.0), Vec3::new(1.0, 0.55, 0.2), 90.0)];
    s.fires = vec![Vec3::new(1.0, 2.3, 1.0)];
    s
}

pub fn build_all() -> Vec<Structure> {
    let mut v = vec![
        hut(),
        house(),
        townhouse(),
        woodcutter(),
        clay_pit(),
        charcoal_burner(),
        iron_mine(),
        herb_garden(),
        farm(),
        windmill(),
        bakery(),
        brewery(),
        sheep_farm(),
        sawmill(),
        brickworks(),
        smelter(),
        smithy(),
        armory(),
        weaver(),
        tailor(),
        storehouse(),
        market(),
        temple(),
        wall_tower(),
    ];
    for mask in 0..16 {
        v.push(wall_piece(mask));
    }
    v.push(wall_diagonal(0));
    v.push(wall_diagonal(1));
    for along_x in [true, false] {
        for open in [false, true] {
            v.push(gate(along_x, open));
        }
    }
    v
}
