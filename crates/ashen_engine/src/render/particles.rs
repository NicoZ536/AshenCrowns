//! CPU-Partikelsystem (Rauch, Glut, Funken, Magie, Staub, Regenspritzer).
//!
//! Partikel bewegen sich über dem Boden (Bodenposition + Höhe), werden nach Bodentiefe
//! sortiert in die Welt gezeichnet und können eigene Lichter erzeugen. Die maximale Anzahl
//! begrenzt das Grafikprofil.

use super::{FrameData, Light, Material, SpriteInstance, SpriteRect};
use glam::{Vec2, Vec3, Vec4};

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    /// Bodenposition in Weltpixeln.
    pub pos: Vec2,
    /// Höhe über dem Boden in Pixeln.
    pub z: f32,
    pub vel: Vec2,
    pub vz: f32,
    pub gravity: f32,
    pub drag: f32,
    pub life: f32,
    pub max_life: f32,
    pub size: (f32, f32),
    pub color: (Vec4, Vec4),
    pub sprite: SpriteRect,
    pub material: Material,
    pub emissive: f32,
    /// Lichtradius (0 = kein Licht) und Lichtfarbe.
    pub light: f32,
    pub light_color: Vec3,
    /// Tiefenumrechnung: Weltpixel-y je Tiefeneinheit.
    pub depth_bias: f32,
}

impl Particle {
    pub fn new(sprite: SpriteRect, pos: Vec2, z: f32, life: f32) -> Particle {
        Particle {
            pos,
            z,
            vel: Vec2::ZERO,
            vz: 0.0,
            gravity: 0.0,
            drag: 0.0,
            life,
            max_life: life,
            size: (1.0, 1.0),
            color: (Vec4::ONE, Vec4::ONE),
            sprite,
            material: Material::Lit,
            emissive: 0.0,
            light: 0.0,
            light_color: Vec3::ONE,
            depth_bias: 0.0,
        }
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub max: usize,
}

impl ParticleSystem {
    pub fn new(max: usize) -> ParticleSystem {
        ParticleSystem { particles: Vec::with_capacity(max), max }
    }

    /// Fügt einen Partikel hinzu; bei vollem Puffer wird der älteste ersetzt.
    pub fn spawn(&mut self, p: Particle) {
        if self.particles.len() < self.max {
            self.particles.push(p);
        } else if let Some((i, _)) = self.particles.iter().enumerate().min_by(|a, b| (a.1.life / a.1.max_life).total_cmp(&(b.1.life / b.1.max_life))) {
            self.particles[i] = p;
        }
    }

    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.life -= dt;
            let damp = (1.0 - p.drag * dt).max(0.0);
            p.vel *= damp;
            p.vz = p.vz * damp - p.gravity * dt;
            p.pos += p.vel * dt;
            p.z += p.vz * dt;
            if p.z < 0.0 {
                p.z = 0.0;
                p.vz = -p.vz * 0.3;
                p.vel *= 0.5;
            }
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    /// Schreibt Partikel als Sprites (und ggf. Lichter) in den Frame.
    /// `depth_of_world_y` rechnet Weltpixel-y in den Tiefenschlüssel der Weltobjekte um.
    pub fn draw(&self, frame: &mut FrameData, depth_of_world_y: impl Fn(f32) -> f32) {
        for p in &self.particles {
            let t = 1.0 - (p.life / p.max_life).clamp(0.0, 1.0);
            let scale = p.size.0 + (p.size.1 - p.size.0) * t;
            let color = p.color.0.lerp(p.color.1, t);
            let size = p.sprite.size * scale;
            let screen = Vec2::new(p.pos.x, p.pos.y - p.z);
            let inst = SpriteInstance {
                pos: [(screen.x - size.x * 0.5).round(), (screen.y - size.y * 0.5).round()],
                size: [size.x.max(1.0), size.y.max(1.0)],
                uv: p.sprite.uv,
                color: color.to_array(),
                p0: [p.material as u8 as f32, p.emissive * color.w, 0.0, 0.0],
                p1: [0.0; 4],
            };
            if p.material == Material::Heat {
                frame.heat.push(inst);
            } else {
                frame.world.push((depth_of_world_y(p.pos.y + p.depth_bias), inst));
            }
            if p.light > 0.0 {
                frame.lights.push(Light::new(screen, p.light * (0.6 + 0.4 * (1.0 - t)), p.light_color, color.w).height(10.0));
            }
        }
    }

    pub fn len(&self) -> usize {
        self.particles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.particles.is_empty()
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn particles_age_and_respect_cap() {
        let mut ps = ParticleSystem::new(3);
        for i in 0..5 {
            let mut p = Particle::new(SpriteRect::default(), Vec2::ZERO, 5.0, 1.0 + i as f32);
            p.vz = 10.0;
            p.gravity = 20.0;
            ps.spawn(p);
        }
        assert_eq!(ps.len(), 3);
        ps.update(2.5);
        assert!(ps.len() < 3);
        assert!(ps.particles.iter().all(|p| p.z >= 0.0));
    }
}
