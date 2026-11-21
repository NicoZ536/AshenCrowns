//! Gemeinsame Grundbausteine ohne Grafik- oder Plattformabhängigkeiten.
//!
//! Wird sowohl von der Simulation (`ashen_sim`) als auch von Engine und Spiel genutzt.

pub mod color;
pub mod iso;
pub mod noise;

pub use glam::{IVec2, UVec2, Vec2, Vec3, Vec4};
