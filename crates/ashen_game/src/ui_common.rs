//! Gemeinsame UI-Bausteine für Hauptmenü und Ingame-Menüs.

use crate::art::ArtLibrary;
use crate::settings::UserSettings;
use ashen_engine::render::{GraphicsProfile, SpriteRect};
use ashen_engine::ui::theme;
use ashen_engine::{Engine, WindowRequest};
use egui::{Pos2, Rect, RichText, Vec2};
use std::path::{Path, PathBuf};

/// Bild aus einem Atlas-Ausschnitt, proportional auf `size` (längste Kante) skaliert.
pub fn image_from_rect<'a>(tex: egui::TextureId, r: SpriteRect, size: f32) -> egui::Image<'a> {
    let aspect = r.size.x / r.size.y.max(1.0);
    let s = if aspect >= 1.0 { Vec2::new(size, size / aspect) } else { Vec2::new(size * aspect, size) };
    egui::Image::new((tex, s)).uv(Rect::from_min_max(Pos2::new(r.uv[0], r.uv[1]), Pos2::new(r.uv[2], r.uv[3])))
}

pub fn atlas_image<'a>(art: &ArtLibrary, tex: egui::TextureId, name: &str, size: f32) -> egui::Image<'a> {
    image_from_rect(tex, art.get(name), size)
}

/// Liste der Spielstände; liefert den gewählten Pfad.
pub fn save_list(ui: &mut egui::Ui, dir: &Path) -> Option<PathBuf> {
    let saves = ashen_sim::save::list(dir);
    let mut chosen = None;
    if saves.is_empty() {
        ui.label(RichText::new("Keine Spielstände vorhanden.").color(theme::PARCHMENT_DIM));
        return None;
    }
    egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
        for (path, header) in saves {
            match header {
                Ok(h) => {
                    let mins = h.play_time_secs as u32 / 60;
                    let label = format!("{}  ·  {} ({})  ·  Tag {}  ·  {}:{:02}", h.name, h.mission_name, h.mode, h.day, mins / 60, mins % 60);
                    if ui.add_sized([ui.available_width(), 28.0], egui::Button::new(label)).clicked() {
                        chosen = Some(path.clone());
                    }
                }
                Err(e) => {
                    ui.label(RichText::new(format!("{}: {e}", path.display())).color(theme::EMBER).small());
                }
            }
        }
    });
    chosen
}

/// Einstellungsseite (wirkt sofort).
pub fn settings_ui(ui: &mut egui::Ui, engine: &mut Engine, s: &mut UserSettings) {
    egui::Grid::new("settings").num_columns(2).spacing([16.0, 8.0]).show(ui, |ui| {
        ui.label("Grafikprofil");
        ui.horizontal(|ui| {
            for p in GraphicsProfile::ALL {
                if ui.selectable_label(s.profile == p, p.label()).clicked() {
                    s.profile = p;
                    engine.renderer.set_profile(p);
                }
            }
        });
        ui.end_row();
        ui.label("Vollbild");
        if ui.checkbox(&mut s.fullscreen, "").changed() {
            engine.request(WindowRequest::Fullscreen(s.fullscreen));
        }
        ui.end_row();
        ui.label("VSync");
        if ui.checkbox(&mut s.vsync, "").changed() {
            engine.request(WindowRequest::Vsync(s.vsync));
        }
        ui.end_row();
        let mut v = s.volumes;
        let mut changed = false;
        for (label, val) in [("Gesamtlautstärke", &mut v.master), ("Musik", &mut v.music), ("Effekte", &mut v.sfx), ("Stimmen", &mut v.voice), ("Umgebung", &mut v.ambience), ("Oberfläche", &mut v.ui)] {
            ui.label(label);
            changed |= ui.add(egui::Slider::new(val, 0.0..=1.0).show_value(false)).changed();
            ui.end_row();
        }
        if changed {
            s.volumes = v;
            engine.audio.set_volumes(v);
        }
        ui.label("Bildlauf am Rand");
        ui.checkbox(&mut s.edge_scroll, "");
        ui.end_row();
        ui.label("Bildlauf-Tempo");
        ui.add(egui::Slider::new(&mut s.scroll_speed, 0.4..=2.5));
        ui.end_row();
        ui.label("Autosave (Minuten, 0 = aus)");
        ui.add(egui::Slider::new(&mut s.autosave_minutes, 0.0..=15.0).step_by(1.0));
        ui.end_row();
        ui.label("FPS anzeigen");
        ui.checkbox(&mut s.show_fps, "");
        ui.end_row();
    });
    ui.label(RichText::new(format!("Audio: {} · Zugeordnete Audiodateien: {} · Fehlend: {}", engine.audio.backend_name(), engine.audio.report.found_count(), engine.audio.report.missing.len())).small().color(theme::PARCHMENT_DIM));
}
