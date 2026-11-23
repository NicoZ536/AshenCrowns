//! Gemeinsame UI-Bausteine für Hauptmenü und Ingame-Menüs.

use crate::settings::UserSettings;
use crate::ui_art::{self, BtnState, UiArt};
use ashen_engine::render::{GraphicsProfile, SpriteRect};
use ashen_engine::ui::theme;
use ashen_engine::{Engine, WindowRequest};
use egui::emath::GuiRounding;
use egui::{Align2, Color32, Pos2, Rect, RichText, Sense, Vec2};
use std::path::{Path, PathBuf};

pub fn sprite_uv(r: SpriteRect) -> Rect {
    Rect::from_min_max(Pos2::new(r.uv[0], r.uv[1]), Pos2::new(r.uv[2], r.uv[3]))
}

/// Zeichnet einen Atlas-Ausschnitt eingepasst (Seitenverhältnis erhalten) in `rect`.
pub fn paint_sprite(painter: &egui::Painter, tex: egui::TextureId, r: SpriteRect, rect: Rect, tint: Color32) {
    let aspect = r.size.x / r.size.y.max(1.0);
    let s = if aspect >= rect.width() / rect.height() { Vec2::new(rect.width(), rect.width() / aspect) } else { Vec2::new(rect.height() * aspect, rect.height()) };
    painter.image(tex, Rect::from_center_size(rect.center(), s).round_ui(), sprite_uv(r), tint);
}

/// Spielt UI-Klänge: Hover-Beginn und Klick.
pub fn sfx(engine: &mut Engine, ui: &egui::Ui, resp: &egui::Response) {
    if ui_art::hover_started(ui, resp) && resp.sense.senses_click() {
        engine.audio.play("sfx.ui_hover");
    }
    if resp.clicked() {
        engine.audio.play("sfx.ui_click");
    }
}

/// Schaltfläche mit Klang.
pub fn button(ui: &mut egui::Ui, engine: &mut Engine, art: &UiArt, text: &str, width: f32) -> bool {
    let r = ui_art::button(ui, art, text, Vec2::new(width, 36.0), true);
    sfx(engine, ui, &r);
    r.clicked()
}

/// Liste der Spielstände; liefert den gewählten Pfad.
pub fn save_list(ui: &mut egui::Ui, engine: &mut Engine, art: &UiArt, dir: &Path) -> Option<PathBuf> {
    let saves = ashen_sim::save::list(dir);
    let mut chosen = None;
    if saves.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(12.0);
            ui.label(RichText::new("Keine Spielstände vorhanden.").font(theme::italic_font(16.0)).color(theme::PARCHMENT_DIM));
            ui.add_space(12.0);
        });
        return None;
    }
    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
        for (path, header) in saves {
            match header {
                Ok(h) => {
                    let mins = h.play_time_secs as u32 / 60;
                    let detail = format!("{} ({}) · Tag {} · Spielzeit {}:{:02}", h.mission_name, h.mode, h.day, mins / 60, mins % 60);
                    let w = ui.available_width().max(360.0);
                    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 50.0), Sense::click());
                    let st = BtnState::of(&resp, true);
                    art.button_nine(st).paint(ui.painter(), rect.round_ui(), Color32::WHITE);
                    let p = ui.painter();
                    let name_col = if st == BtnState::Hover { theme::GOLD_BRIGHT } else { theme::GOLD };
                    p.text(rect.left_top() + Vec2::new(22.0, 9.0), Align2::LEFT_TOP, &h.name, theme::heading_font(15.0), name_col);
                    p.text(rect.left_bottom() + Vec2::new(22.0, -9.0), Align2::LEFT_BOTTOM, detail, theme::italic_font(13.5), theme::PARCHMENT_DIM);
                    sfx(engine, ui, &resp);
                    if resp.clicked() {
                        chosen = Some(path.clone());
                    }
                    ui.add_space(2.0);
                }
                Err(e) => {
                    ui.label(RichText::new(format!("{}: {e}", path.display())).color(theme::EMBER).small());
                }
            }
        }
    });
    chosen
}

fn row_label(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).font(theme::caps_font(13.5)).color(Color32::from_rgb(214, 190, 146)));
}

/// Einstellungsseite (wirkt sofort).
pub fn settings_ui(ui: &mut egui::Ui, engine: &mut Engine, s: &mut UserSettings, art: &UiArt) {
    let section = |ui: &mut egui::Ui, title: &str| {
        ui.add_space(4.0);
        ui.label(RichText::new(title).font(theme::heading_font(15.0)).color(theme::GOLD));
        let r = ui.available_rect_before_wrap();
        let y = ui.cursor().min.y + 1.0;
        ui.painter().line_segment([Pos2::new(r.min.x, y), Pos2::new(r.min.x + 420.0, y)], egui::Stroke::new(1.0, theme::GOLD_DIM.gamma_multiply(0.6)));
        ui.add_space(6.0);
    };
    section(ui, "Grafik");
    egui::Grid::new("settings_gfx").num_columns(2).spacing([20.0, 10.0]).min_col_width(170.0).show(ui, |ui| {
        row_label(ui, "Grafikprofil");
        ui.horizontal(|ui| {
            for p in GraphicsProfile::ALL {
                let r = ui_art::choice(ui, art, p.label(), s.profile == p);
                sfx(engine, ui, &r);
                if r.clicked() {
                    s.profile = p;
                    engine.renderer.set_profile(p);
                }
            }
        });
        ui.end_row();
        row_label(ui, "Vollbild");
        if ui.checkbox(&mut s.fullscreen, "").changed() {
            engine.request(WindowRequest::Fullscreen(s.fullscreen));
        }
        ui.end_row();
        row_label(ui, "VSync");
        if ui.checkbox(&mut s.vsync, "").changed() {
            engine.request(WindowRequest::Vsync(s.vsync));
        }
        ui.end_row();
        row_label(ui, "FPS anzeigen");
        ui.checkbox(&mut s.show_fps, "");
        ui.end_row();
    });
    section(ui, "Klang");
    egui::Grid::new("settings_audio").num_columns(2).spacing([20.0, 10.0]).min_col_width(170.0).show(ui, |ui| {
        let mut v = s.volumes;
        let mut changed = false;
        for (label, val) in [("Gesamtlautstärke", &mut v.master), ("Musik", &mut v.music), ("Effekte", &mut v.sfx), ("Stimmen", &mut v.voice), ("Umgebung", &mut v.ambience), ("Oberfläche", &mut v.ui)] {
            row_label(ui, label);
            changed |= ui.add(egui::Slider::new(val, 0.0..=1.0).show_value(false)).changed();
            ui.end_row();
        }
        if changed {
            s.volumes = v;
            engine.audio.set_volumes(v);
        }
    });
    section(ui, "Steuerung");
    egui::Grid::new("settings_ctrl").num_columns(2).spacing([20.0, 10.0]).min_col_width(170.0).show(ui, |ui| {
        row_label(ui, "Bildlauf am Rand");
        ui.checkbox(&mut s.edge_scroll, "");
        ui.end_row();
        row_label(ui, "Bildlauf-Tempo");
        ui.add(egui::Slider::new(&mut s.scroll_speed, 0.4..=2.5));
        ui.end_row();
        row_label(ui, "Autosave (Minuten)");
        ui.add(egui::Slider::new(&mut s.autosave_minutes, 0.0..=15.0).step_by(1.0).custom_formatter(|v, _| if v < 0.5 { "aus".into() } else { format!("{v:.0}") }));
        ui.end_row();
    });
    ui.add_space(6.0);
    ui.label(RichText::new(format!("Audio: {} · Zugeordnete Audiodateien: {} · Fehlend: {}", engine.audio.backend_name(), engine.audio.report.found_count(), engine.audio.report.missing.len())).font(theme::italic_font(13.0)).color(theme::PARCHMENT_DIM));
}
