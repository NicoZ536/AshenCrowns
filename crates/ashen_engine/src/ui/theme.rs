//! Dunkles Fantasy-Design: Leder, geschwärztes Metall, Pergament, goldene Akzente.

use egui::{Color32, CornerRadius, FontFamily, FontId, Margin, Shadow, Stroke, TextStyle};

pub const GOLD: Color32 = Color32::from_rgb(214, 170, 82);
pub const GOLD_DIM: Color32 = Color32::from_rgb(140, 104, 46);
pub const EMBER: Color32 = Color32::from_rgb(226, 104, 44);
pub const BLOOD: Color32 = Color32::from_rgb(150, 30, 28);
pub const PARCHMENT: Color32 = Color32::from_rgb(232, 218, 188);
pub const PARCHMENT_DIM: Color32 = Color32::from_rgb(170, 156, 128);
pub const LEATHER: Color32 = Color32::from_rgb(28, 21, 18);
pub const LEATHER_LIGHT: Color32 = Color32::from_rgb(46, 34, 28);
pub const IRON: Color32 = Color32::from_rgb(62, 60, 64);
pub const HOLLOW_GREEN: Color32 = Color32::from_rgb(120, 230, 150);
pub const MANA: Color32 = Color32::from_rgb(70, 120, 220);

pub fn apply(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style.text_styles = [
            (TextStyle::Heading, FontId::new(22.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(14.5, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(15.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(11.5, FontFamily::Proportional)),
        ]
        .into();
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.window_margin = Margin::same(12);
        let v = &mut style.visuals;
        v.dark_mode = true;
        v.override_text_color = Some(PARCHMENT);
        v.panel_fill = LEATHER.gamma_multiply(0.96);
        v.window_fill = Color32::from_rgba_unmultiplied(24, 18, 16, 242);
        v.window_stroke = Stroke::new(1.5, GOLD_DIM);
        v.window_corner_radius = CornerRadius::same(3);
        v.window_shadow = Shadow { offset: [0, 6], blur: 18, spread: 2, color: Color32::from_black_alpha(170) };
        v.popup_shadow = v.window_shadow;
        v.extreme_bg_color = Color32::from_rgb(14, 11, 10);
        v.faint_bg_color = LEATHER_LIGHT;
        v.hyperlink_color = GOLD;
        v.selection.bg_fill = GOLD_DIM.gamma_multiply(0.6);
        v.selection.stroke = Stroke::new(1.0, GOLD);
        v.warn_fg_color = EMBER;
        v.error_fg_color = Color32::from_rgb(230, 70, 60);
        let w = &mut v.widgets;
        w.noninteractive.bg_fill = LEATHER;
        w.noninteractive.weak_bg_fill = LEATHER;
        w.noninteractive.bg_stroke = Stroke::new(1.0, GOLD_DIM.gamma_multiply(0.7));
        w.noninteractive.fg_stroke = Stroke::new(1.0, PARCHMENT);
        w.inactive.bg_fill = Color32::from_rgb(44, 34, 28);
        w.inactive.weak_bg_fill = Color32::from_rgb(44, 34, 28);
        w.inactive.bg_stroke = Stroke::new(1.0, GOLD_DIM);
        w.inactive.fg_stroke = Stroke::new(1.0, PARCHMENT);
        w.hovered.bg_fill = Color32::from_rgb(70, 50, 34);
        w.hovered.weak_bg_fill = Color32::from_rgb(70, 50, 34);
        w.hovered.bg_stroke = Stroke::new(1.5, GOLD);
        w.hovered.fg_stroke = Stroke::new(1.5, Color32::from_rgb(255, 236, 200));
        w.active.bg_fill = Color32::from_rgb(96, 62, 32);
        w.active.weak_bg_fill = Color32::from_rgb(96, 62, 32);
        w.active.bg_stroke = Stroke::new(1.5, EMBER);
        w.active.fg_stroke = Stroke::new(1.5, Color32::WHITE);
        w.open = w.hovered;
        for wv in [&mut w.noninteractive, &mut w.inactive, &mut w.hovered, &mut w.active, &mut w.open] {
            wv.corner_radius = CornerRadius::same(2);
        }
    });
}

/// Rahmen für Ingame-Paneele (geschwärztes Leder mit Goldkante).
pub fn panel_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(20, 15, 13, 236))
        .stroke(Stroke::new(1.5, GOLD_DIM))
        .corner_radius(CornerRadius::same(3))
        .inner_margin(Margin::same(8))
        .shadow(Shadow { offset: [0, 4], blur: 12, spread: 0, color: Color32::from_black_alpha(150) })
}

/// Pergament-Rahmen für Nachrichten und Dialoge.
pub fn parchment_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(54, 42, 30, 236))
        .stroke(Stroke::new(1.0, Color32::from_rgb(120, 92, 54)))
        .corner_radius(CornerRadius::same(2))
        .inner_margin(Margin::symmetric(12, 8))
}

/// Überschrift im Gold-Stil.
pub fn heading(text: impl Into<String>, size: f32) -> egui::RichText {
    egui::RichText::new(text.into()).color(GOLD).size(size).strong()
}
