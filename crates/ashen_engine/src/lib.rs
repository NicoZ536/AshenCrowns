//! # ashen_engine – eigene modulare 2D-Engine von ASHEN CROWNS
//!
//! Unabhängig von konkreten Spielsystemen. Module:
//! - [`app`] – Fenster, Hauptschleife, Headless-Rendering, `Game`-Schnittstelle
//! - [`gpu`] – wgpu-Kontext (Metal auf macOS, Vulkan/DX12 später)
//! - [`render`] – Deferred-2D-Renderer (G-Buffer, Lichter, Nebel, Wetter, Bloom, Farbkorrektur), Partikel
//! - [`assets`] – prozedurale Pixel-Art, Atlas, PNG-Overrides
//! - [`anim`] – Animationsclips
//! - [`audio`] – Manifest, Scanner, Event-Registry, Musik-Crossfades
//! - [`ui`] – egui-Integration und Theme
//! - [`input`], [`time`] – Eingabe und Zeitsteuerung

pub mod anim;
pub mod app;
pub mod assets;
pub mod audio;
pub mod bake;
pub mod gpu;
pub mod input;
pub mod render;
pub mod time;
pub mod ui;

pub use app::{run, run_headless, AppConfig, Engine, Game, WindowRequest};
pub use egui;
pub use glam;
pub use winit::keyboard::KeyCode;
