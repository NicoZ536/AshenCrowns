//! Asset-Pipeline des Spiels: erzeugt alle Sprites prozedural, übernimmt optionale
//! PNG-Overrides aus `assets/sprites/` und packt alles in einen Atlas.

pub mod buildings;
pub mod fx;
pub mod icons;
pub mod palette;
pub mod props;
pub mod terrain;

use crate::art3d::{self, rig::Anim, DIRS};
use ashen_engine::anim::{AnimationClip, AnimationSet};
use ashen_engine::assets::{self, Atlas, AtlasBuilder};
use ashen_engine::gpu::GpuContext;
use ashen_engine::render::SpriteRect;
use std::collections::HashMap;
use std::path::Path;

pub use buildings::BuildingMeta;

pub struct ArtLibrary {
    pub atlas: Atlas,
    pub buildings: HashMap<String, BuildingMeta>,
    /// Animationen je Einheit und gebackener Blickrichtung.
    anims: HashMap<String, Vec<AnimationSet>>,
}

impl ArtLibrary {
    pub fn build(assets_dir: Option<&Path>, gpu: &GpuContext) -> Result<ArtLibrary, String> {
        let start = std::time::Instant::now();
        let mut b = AtlasBuilder::new();
        terrain::build(&mut b);
        buildings::build(&mut b);
        props::build(&mut b);
        fx::build(&mut b);
        icons::build(&mut b);
        let metas = art3d::bake_all(gpu, &mut b)?;
        let mut overrides = 0;
        if let Some(dir) = assets_dir {
            let sprites = dir.join("sprites");
            if let Ok(rd) = std::fs::read_dir(&sprites) {
                for e in rd.flatten() {
                    let p = e.path();
                    let Some(stem) = p.file_stem().map(|s| s.to_string_lossy().to_string()) else { continue };
                    if p.extension().is_none_or(|x| x != "png") || stem.ends_with("_n") {
                        continue;
                    }
                    if let Some(c) = assets::load_override(dir, &stem) {
                        let anchor = glam::Vec2::new(c.w as f32 * 0.5, c.h as f32 - 2.0);
                        b.replace(&stem, c, anchor);
                        overrides += 1;
                    }
                }
            }
        }
        let count = b.len();
        let atlas = b.build();
        let mut lib = ArtLibrary { atlas, buildings: metas, anims: HashMap::new() };
        for (id, anims) in art3d::unit_anim_specs() {
            let sets = (0..DIRS).map(|d| lib.make_anim_set(id, &anims, d)).collect();
            lib.anims.insert(id.to_string(), sets);
        }
        log::info!("Atlas {}×{} mit {count} Sprites ({overrides} Overrides) in {:.0} ms erzeugt", lib.atlas.size.0, lib.atlas.size.1, start.elapsed().as_secs_f32() * 1000.0);
        Ok(lib)
    }

    fn make_anim_set(&self, id: &str, anims: &[Anim], dir: usize) -> AnimationSet {
        let mut set = AnimationSet::default();
        for a in anims {
            let frames: Vec<SpriteRect> = (0..a.frames()).filter_map(|i| self.atlas.try_get(&format!("{id}_{}_{dir}_{i}", a.name()))).collect();
            if !frames.is_empty() {
                set.insert(a.name(), AnimationClip::new(frames, a.fps(), a.looped()));
            }
        }
        set
    }

    /// Animationen einer Einheit in einer gebackenen Richtung; unbekannte (Mod-)Einheiten
    /// erhalten eine passende Basisfigur.
    pub fn unit_anims(&self, id: &str, hollow: bool, dir: usize) -> &AnimationSet {
        let sets = self.anims.get(id).unwrap_or_else(|| &self.anims[if hollow { "hollow_thrall" } else { "swordsman" }]);
        &sets[dir.min(DIRS - 1)]
    }

    pub fn portrait(&self, id: &str) -> SpriteRect {
        self.atlas.try_get(&format!("{id}_portrait")).unwrap_or_else(|| self.atlas.get("swordsman_portrait"))
    }

    /// Kontaktbogen der Einheiten-Frames (Debug/Art-Review): je Einheit Zeilen mit allen Richtungen
    /// für Idle, Lauf, Angriff und Tod, vierfach vergrößert.
    pub fn contact_sheet(&self, path: &Path) -> Result<(), String> {
        let ids: Vec<&'static str> = art3d::unit_anim_specs().into_iter().map(|(id, _)| id).collect();
        let picks = [("idle", 0), ("walk", 2), ("attack", 2), ("death", 5)];
        let (cw, ch, z) = (100u32, 112u32, 3u32);
        let cols = (DIRS * picks.len()) as u32;
        let (w, h) = (cols * cw, ids.len() as u32 * ch);
        let mut img = image::RgbaImage::from_pixel(w, h, image::Rgba([46, 52, 40, 255]));
        let (aw, _) = self.atlas.size;
        for (row, id) in ids.iter().enumerate() {
            for (pi, (anim, f)) in picks.iter().enumerate() {
                for d in 0..DIRS {
                    let Some((x, y, sw, sh)) = self.pixel_rect(&format!("{id}_{anim}_{d}_{f}")) else { continue };
                    let ox = ((pi * DIRS + d) as u32) * cw + 2;
                    let oy = row as u32 * ch + 2;
                    for py in 0..sh.min((ch - 4) / z) {
                        for px in 0..sw.min((cw - 4) / z) {
                            let i = (((y + py) * aw + x + px) * 4) as usize;
                            let c = &self.atlas.albedo[i..i + 4];
                            if c[3] == 0 {
                                continue;
                            }
                            for dy in 0..z {
                                for dx in 0..z {
                                    img.put_pixel(ox + px * z + dx, oy + py * z + dy, image::Rgba([c[0], c[1], c[2], 255]));
                                }
                            }
                        }
                    }
                }
            }
        }
        img.save(path).map_err(|e| e.to_string())
    }

    /// Pixelrechteck (x, y, w, h) eines Sprites im Atlas (Debug-Werkzeuge).
    pub fn pixel_rect(&self, name: &str) -> Option<(u32, u32, u32, u32)> {
        let r = self.atlas.try_get(name)?;
        let (w, h) = (self.atlas.size.0 as f32, self.atlas.size.1 as f32);
        Some(((r.uv[0] * w) as u32, (r.uv[1] * h) as u32, r.size.x as u32, r.size.y as u32))
    }

    pub fn get(&self, name: &str) -> SpriteRect {
        self.atlas.get(name)
    }
}
