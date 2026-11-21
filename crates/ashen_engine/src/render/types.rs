//! GPU-Datentypen der Engine (Sprite-Instanzen, Lichter, Uniforms).

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3, Vec4};

/// Materialkennungen im Sprite-Shader.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Material {
    Lit = 0,
    Water = 1,
    Unlit = 2,
    Emissive = 3,
    Shadow = 4,
    Heat = 5,
    Solid = 6,
}

/// Rechteck im Textur-Atlas (UV 0..1) plus Pixelgröße und Ankerpunkt.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SpriteRect {
    pub uv: [f32; 4],
    pub size: Vec2,
    /// Ankerpunkt in Pixeln relativ zur linken oberen Ecke (meist Fußpunkt).
    pub anchor: Vec2,
}

/// Eine Sprite-Instanz, wie sie an die GPU geht (80 Byte).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SpriteInstance {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub uv: [f32; 4],
    pub color: [f32; 4],
    /// Material, Emission, Spiegelung, Reserve
    pub p0: [f32; 4],
    /// Scherung, Stauchung, Wassertiefe, Reserve
    pub p1: [f32; 4],
}

impl SpriteInstance {
    /// Platziert ein Sprite so, dass sein Anker auf `world` liegt.
    pub fn new(rect: &SpriteRect, world: Vec2) -> SpriteInstance {
        let pos = world - rect.anchor;
        SpriteInstance {
            pos: [pos.x.round(), pos.y.round()],
            size: rect.size.to_array(),
            uv: rect.uv,
            color: [1.0; 4],
            p0: [0.0; 4],
            p1: [0.0; 4],
        }
    }

    /// Einfarbiges Rechteck (Balken, Rahmen).
    pub fn solid(pos: Vec2, size: Vec2, color: Vec4) -> SpriteInstance {
        SpriteInstance {
            pos: pos.to_array(),
            size: size.to_array(),
            uv: [0.0; 4],
            color: color.to_array(),
            p0: [Material::Solid as u8 as f32, 0.0, 0.0, 0.0],
            p1: [0.0; 4],
        }
    }

    pub fn material(mut self, m: Material) -> Self {
        self.p0[0] = m as u8 as f32;
        self
    }

    pub fn emissive(mut self, e: f32) -> Self {
        self.p0[1] = e;
        self
    }

    pub fn flip(mut self, f: bool) -> Self {
        self.p0[2] = if f { 1.0 } else { 0.0 };
        self
    }

    pub fn tint(mut self, c: Vec4) -> Self {
        self.color = c.to_array();
        self
    }

    pub fn alpha(mut self, a: f32) -> Self {
        self.color[3] = a;
        self
    }

    pub fn shear(mut self, s: f32) -> Self {
        self.p1[0] = s;
        self
    }

    pub fn squash(mut self, s: f32) -> Self {
        self.p1[1] = s;
        self
    }

    pub fn water_depth(mut self, d: f32) -> Self {
        self.p1[2] = d;
        self
    }

    /// Ausschnitt von oben abschneiden (z. B. Baufortschritt): nur der untere Anteil `frac` bleibt sichtbar.
    pub fn crop_top(mut self, frac: f32) -> Self {
        let frac = frac.clamp(0.0, 1.0);
        let cut = self.size[1] * (1.0 - frac);
        self.pos[1] += cut.round();
        self.size[1] -= cut.round();
        let v0 = self.uv[1] + (self.uv[3] - self.uv[1]) * (1.0 - frac);
        self.uv[1] = v0;
        self
    }
}

/// Punktlicht (48 Byte).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Light {
    /// Weltpixel x, y, Radius (Pixel), Höhe über Boden (Pixel)
    pub pos_radius: [f32; 4],
    /// rgb, Intensität
    pub color: [f32; 4],
    /// Flackern 0..1, Phase, Reserve, Reserve
    pub params: [f32; 4],
}

impl Light {
    pub fn new(pos: Vec2, radius: f32, color: Vec3, intensity: f32) -> Light {
        Light { pos_radius: [pos.x, pos.y, radius, 24.0], color: [color.x, color.y, color.z, intensity], params: [0.0; 4] }
    }

    pub fn flicker(mut self, amount: f32, phase: f32) -> Light {
        self.params[0] = amount;
        self.params[1] = phase;
        self
    }

    pub fn height(mut self, h: f32) -> Light {
        self.pos_radius[3] = h;
        self
    }
}

/// Uniform-Block aller Shader (siehe `shaders/common.wgsl`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Globals {
    pub view_proj: [[f32; 4]; 4],
    pub screen: [f32; 4],
    pub time: [f32; 4],
    pub camera: [f32; 4],
    pub ambient: [f32; 4],
    pub sun_color: [f32; 4],
    pub sun_dir: [f32; 4],
    pub fog: [f32; 4],
    pub fog_color: [f32; 4],
    pub weather: [f32; 4],
    pub fow: [f32; 4],
    pub grade: [f32; 4],
    pub grade_tint: [f32; 4],
    pub bloom: [f32; 4],
    pub output: [f32; 4],
    pub tile: [f32; 4],
}

/// Atmosphäre und Farbgebung eines Frames (vom Spiel gesetzt).
#[derive(Clone, Copy, Debug)]
pub struct Atmosphere {
    pub ambient: Vec3,
    pub ambient_intensity: f32,
    pub sun_color: Vec3,
    pub sun_intensity: f32,
    /// Lichtrichtung im Bildschirmraum (x rechts, y unten, z zum Betrachter).
    pub sun_dir: Vec3,
    pub shadow_strength: f32,
    pub fog_density: f32,
    pub ground_fog: f32,
    pub wind: Vec2,
    pub fog_color: Vec3,
    pub fog_light_scatter: f32,
    pub rain: f32,
    pub ash: f32,
    pub lightning: f32,
    pub wetness: f32,
    pub exposure: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub vignette: f32,
    pub tint: Vec3,
    pub bloom_intensity: f32,
    pub bloom_threshold: f32,
}

impl Default for Atmosphere {
    fn default() -> Self {
        Atmosphere {
            ambient: Vec3::new(0.55, 0.58, 0.68),
            ambient_intensity: 0.45,
            sun_color: Vec3::new(1.0, 0.92, 0.8),
            sun_intensity: 0.8,
            sun_dir: Vec3::new(-0.5, -0.6, 0.62).normalize(),
            shadow_strength: 0.8,
            fog_density: 0.0,
            ground_fog: 0.0,
            wind: Vec2::new(1.0, 0.3),
            fog_color: Vec3::new(0.5, 0.52, 0.58),
            fog_light_scatter: 0.4,
            rain: 0.0,
            ash: 0.0,
            lightning: 0.0,
            wetness: 0.0,
            exposure: 1.3,
            contrast: 1.08,
            saturation: 0.92,
            vignette: 0.35,
            tint: Vec3::new(0.05, 0.02, 0.10),
            bloom_intensity: 0.65,
            bloom_threshold: 0.85,
        }
    }
}
