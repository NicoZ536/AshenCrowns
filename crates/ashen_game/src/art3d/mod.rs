//! 3D→Pixel-Bake der Spielobjekte: Modelle werden beim Start mit der GPU in Sprites gerastert
//! (5 Blickrichtungen, die übrigen 3 durch Spiegelung), nachbearbeitet und in den Atlas gelegt.

pub mod city;
pub mod military;
pub mod rig;
pub mod structures;
pub mod units3d;

use crate::art::BuildingMeta;
use ashen_core::color::Rgba;
use std::collections::HashMap;
use ashen_engine::assets::{AtlasBuilder, Canvas};
use ashen_engine::bake::{self, BakeCamera, BakeCell, PPU};
use ashen_engine::gpu::GpuContext;
use glam::{Mat4, Vec2};
use rig::Anim;

/// Anzahl gebackener Richtungen (0 = zum Betrachter, 2 = rechts, 4 = weg); 5–7 werden gespiegelt.
pub const DIRS: usize = 5;
/// Maßstab der Figuren relativ zum Kachelraster (Figuren leicht überhöht, aber kleiner als Gebäude).
pub const UNIT_SCALE: f32 = 0.8;

/// Wählt aus einem Blickwinkel (siehe `Unit::facing`) die gebackene Richtung und ob gespiegelt wird.
pub fn dir_for(facing: f32) -> (usize, bool) {
    let d = ((facing / std::f32::consts::FRAC_PI_4).round() as i32).rem_euclid(8) as usize;
    if d <= 4 { (d, false) } else { (8 - d, true) }
}

/// Nachbearbeitung: dunkle, farbige Kontur (selektiv) wie bei handgepixelten Sprites.
fn post(mut c: Canvas) -> Canvas {
    c.outline(0.32);
    c
}

struct Job {
    name: String,
    anchor: Vec2,
}

/// Backt alle Einheiten-Animationen und Bauwerke in den Atlas. Liefert die Effektpunkte der Gebäude.
pub fn bake_all(gpu: &GpuContext, atlas: &mut AtlasBuilder) -> Result<HashMap<String, BuildingMeta>, String> {
    let start = std::time::Instant::now();
    let models = units3d::build_all();
    let mut cells = Vec::new();
    let mut jobs = Vec::new();
    for m in &models {
        let (w, h) = m.cell;
        let anchor = Vec2::new((w / 2) as f32, (h - 8) as f32);
        for anim in &m.anims {
            let frames = anim.frames();
            for dir in 0..DIRS {
                let cam = BakeCamera { yaw: dir as f32 * std::f32::consts::FRAC_PI_4, ppu: PPU * UNIT_SCALE, anchor };
                for f in 0..frames {
                    let t = if anim.looped() { f as f32 / frames as f32 } else { f as f32 / (frames - 1).max(1) as f32 };
                    let pose = units3d::pose(m, *anim, t);
                    let bones = m.skeleton.world(&pose);
                    let verts = bake::project(&m.mesh, &bones, Mat4::IDENTITY, &cam, (w, h));
                    cells.push(BakeCell { w, h, verts });
                    jobs.push(Job { name: format!("{}_{}_{}_{}", m.id, anim.name(), dir, f), anchor });
                }
            }
        }
    }
    let unit_cells = cells.len();
    // Bauwerke, Bäume, Felsen.
    let mut metas = HashMap::new();
    for s in structures::build_all() {
        let cam = BakeCamera { yaw: s.yaw, ppu: PPU, anchor: s.anchor };
        if let Some(id) = s.name.strip_prefix("bld_") {
            let rel = |p: glam::Vec3| cam.project_point(p) - s.anchor;
            metas.insert(
                id.to_string(),
                BuildingMeta {
                    lights: s.lights.iter().map(|(p, c, r)| (rel(*p), *c, *r)).collect(),
                    smoke: s.smoke.iter().map(|p| rel(*p)).collect(),
                    fires: s.fires.iter().map(|p| rel(*p)).collect(),
                    glow_green: s.glow.iter().map(|p| rel(*p)).collect(),
                    height: s.anchor.y,
                },
            );
        }
        let verts = bake::project(&s.mesh, &[Mat4::IDENTITY], Mat4::IDENTITY, &cam, s.cell);
        cells.push(BakeCell { w: s.cell.0, h: s.cell.1, verts });
        jobs.push(Job { name: s.name, anchor: s.anchor });
    }
    let images = bake::bake(gpu, &cells)?;
    for (img, job) in images.into_iter().zip(jobs) {
        let c = post(Canvas::from_baked(img.w as i32, img.h as i32, &img.albedo, &img.normal));
        let (t, off) = c.trimmed();
        atlas.add(job.name, t, job.anchor - off.as_vec2());
    }
    // Porträts aus der Idle-Pose (Richtung 0) – Kopf und Schultern, vergrößert gebacken.
    let mut pcells = Vec::new();
    let mut pnames = Vec::new();
    for m in &models {
        let head = units3d::head_position(m);
        let size = 40u32;
        let scale = PPU * 2.2;
        let cam = BakeCamera { yaw: 0.35, ppu: scale, anchor: Vec2::ZERO };
        let hp = cam.project_point(head);
        let cam = BakeCamera { anchor: Vec2::new(size as f32 / 2.0, size as f32 * 0.58) - hp, ..cam };
        let pose = units3d::pose(m, Anim::Idle, 0.0);
        let bones = m.skeleton.world(&pose);
        pcells.push(BakeCell { w: size, h: size, verts: bake::project(&m.mesh, &bones, Mat4::IDENTITY, &cam, (size, size)) });
        pnames.push(format!("{}_portrait", m.id));
    }
    for (img, name) in bake::bake(gpu, &pcells)?.into_iter().zip(pnames) {
        let mut c = Canvas::from_baked(img.w as i32, img.h as i32, &img.albedo, &img.normal);
        c.outline(0.3);
        // Dunkler Porträthintergrund für Lesbarkeit im UI.
        for px in c.color.iter_mut() {
            if px.3 == 0 {
                *px = Rgba(18, 14, 13, 255);
            }
        }
        atlas.add(name, c, Vec2::ZERO);
    }
    log::info!("3D-Bake: {} Einheiten-Frames, {} Bauwerke in {:.0} ms", unit_cells, cells.len() - unit_cells, start.elapsed().as_secs_f32() * 1000.0);
    Ok(metas)
}

/// Animationen der gebackenen Einheiten (Name, Bildzahl, FPS, Schleife).
pub fn unit_anim_specs() -> Vec<(&'static str, Vec<Anim>)> {
    let mut v: Vec<(&'static str, Vec<Anim>)> = units3d::looks().into_iter().map(|(id, _, _, anims)| (id, anims)).collect();
    v.extend(units3d::special_anims());
    v
}
