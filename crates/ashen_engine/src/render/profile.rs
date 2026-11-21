//! Grafikprofile, abgestimmt auf Apple Silicon (A18 Pro, 5-Kern-GPU, 8 GB Unified Memory).

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GraphicsProfile {
    /// Maximale Flüssigkeit: kein Bloom, halbe Lichtauflösung, wenige Partikel.
    Performance,
    /// Ausgewogen (Standard).
    Balanced,
    /// Mehr Licht, Partikel, Nebel und Bloom-Stufen.
    Quality,
}

impl GraphicsProfile {
    pub const ALL: [GraphicsProfile; 3] = [GraphicsProfile::Performance, GraphicsProfile::Balanced, GraphicsProfile::Quality];

    pub fn label(self) -> &'static str {
        match self {
            GraphicsProfile::Performance => "Leistung",
            GraphicsProfile::Balanced => "Ausgewogen",
            GraphicsProfile::Quality => "Qualität",
        }
    }

    pub fn settings(self) -> ProfileSettings {
        match self {
            GraphicsProfile::Performance => ProfileSettings { bloom_levels: 0, max_lights: 48, max_particles: 600, normal_mapping: false, fog: true, grain: 0.0, min_lines: 300.0 },
            GraphicsProfile::Balanced => ProfileSettings { bloom_levels: 3, max_lights: 128, max_particles: 1600, normal_mapping: true, fog: true, grain: 0.006, min_lines: 260.0 },
            GraphicsProfile::Quality => ProfileSettings { bloom_levels: 4, max_lights: 256, max_particles: 3200, normal_mapping: true, fog: true, grain: 0.009, min_lines: 220.0 },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ProfileSettings {
    pub bloom_levels: u32,
    pub max_lights: usize,
    pub max_particles: usize,
    pub normal_mapping: bool,
    pub fog: bool,
    pub grain: f32,
    /// Mindestanzahl sichtbarer Zeilen (begrenzt Zoom und damit Pixelkosten nicht nach unten).
    pub min_lines: f32,
}
