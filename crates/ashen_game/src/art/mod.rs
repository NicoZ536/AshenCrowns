//! Asset-Pipeline des Spiels: erzeugt alle Sprites prozedural, übernimmt optionale
//! PNG-Overrides aus `assets/sprites/` und packt alles in einen Atlas.

pub mod buildings;
pub mod fx;
pub mod icons;
pub mod palette;
pub mod props;
pub mod terrain;
pub mod units;

use ashen_engine::anim::{AnimationClip, AnimationSet};
use ashen_engine::assets::{self, Atlas, AtlasBuilder};
use ashen_engine::render::SpriteRect;
use std::collections::HashMap;
use std::path::Path;

pub use buildings::BuildingMeta;

/// Einheiten mit eigenen Sprites (weitere Einheiten aus Mods fallen auf eine Basisfigur zurück).
pub const UNIT_ART: [&str; 10] = ["worker", "swordsman", "spearman", "archer", "knight", "hero_isolde", "hollow_thrall", "hollow_archer", "grave_brute", "boss_pale_reeve"];

pub struct ArtLibrary {
    pub atlas: Atlas,
    pub buildings: HashMap<String, BuildingMeta>,
    anims: HashMap<String, AnimationSet>,
}

impl ArtLibrary {
    pub fn build(assets_dir: Option<&Path>) -> ArtLibrary {
        let start = std::time::Instant::now();
        let mut b = AtlasBuilder::new();
        terrain::build(&mut b);
        units::build(&mut b);
        let buildings = buildings::build(&mut b);
        props::build(&mut b);
        fx::build(&mut b);
        icons::build(&mut b);
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
        let mut lib = ArtLibrary { atlas, buildings, anims: HashMap::new() };
        for id in UNIT_ART {
            let set = lib.make_anim_set(id);
            lib.anims.insert(id.to_string(), set);
        }
        log::info!("Atlas {}×{} mit {count} Sprites ({overrides} Overrides) in {:.0} ms erzeugt", lib.atlas.size.0, lib.atlas.size.1, start.elapsed().as_secs_f32() * 1000.0);
        lib
    }

    fn frames(&self, prefix: &str, n: usize) -> Vec<SpriteRect> {
        (0..n).filter_map(|i| self.atlas.try_get(&format!("{prefix}_{i}"))).collect()
    }

    fn make_anim_set(&self, id: &str) -> AnimationSet {
        let mut set = AnimationSet::default();
        let clip = |frames: Vec<SpriteRect>, fps: f32, looped: bool| AnimationClip::new(frames, fps, looped);
        set.insert("idle", clip(self.frames(&format!("{id}_idle"), 2), 1.6, true));
        set.insert("walk", clip(self.frames(&format!("{id}_walk"), 4), 8.0, true));
        set.insert("attack", clip(self.frames(&format!("{id}_attack"), 4), 9.0, false));
        set.insert("death", clip(self.frames(&format!("{id}_death"), 5), 9.0, false));
        let work = self.frames(&format!("{id}_work"), 4);
        if !work.is_empty() {
            set.insert("work", clip(work, 6.0, true));
            set.insert("carry", clip(self.frames(&format!("{id}_carry"), 4), 8.0, true));
        }
        let cast = self.frames(&format!("{id}_cast"), 4);
        if !cast.is_empty() {
            set.insert("cast", clip(cast, 8.0, false));
        }
        set
    }

    /// Animationen einer Einheit; unbekannte (Mod-)Einheiten erhalten eine passende Basisfigur.
    pub fn unit_anims(&self, id: &str, hollow: bool) -> &AnimationSet {
        self.anims.get(id).unwrap_or_else(|| &self.anims[if hollow { "hollow_thrall" } else { "swordsman" }])
    }

    pub fn portrait(&self, id: &str) -> SpriteRect {
        self.atlas.try_get(&format!("{id}_portrait")).unwrap_or_else(|| self.atlas.get("swordsman_portrait"))
    }

    pub fn get(&self, name: &str) -> SpriteRect {
        self.atlas.get(name)
    }
}
