//! Dunkles Fantasy-Design: angelaufenes Eisen, Leder, Pergament, Goldfiligran und Glut.
//!
//! Enthält Farbpalette, Schriftfamilien (Cinzel/Cinzel Decorative für Titel und Überschriften,
//! Alegreya für Fließtext, mit Rückfall auf die eingebauten egui-Schriften) sowie die
//! Widget-Visuals für Standard-Widgets (Regler, Häkchen, Bildlaufleisten, Eingabefelder).

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Margin, Shadow, Stroke, TextStyle};
use std::path::Path;
use std::sync::Arc;

pub const GOLD: Color32 = Color32::from_rgb(214, 170, 82);
pub const GOLD_BRIGHT: Color32 = Color32::from_rgb(250, 222, 150);
pub const GOLD_DIM: Color32 = Color32::from_rgb(140, 104, 46);
pub const BRONZE: Color32 = Color32::from_rgb(120, 82, 40);
pub const EMBER: Color32 = Color32::from_rgb(236, 112, 40);
pub const EMBER_HOT: Color32 = Color32::from_rgb(255, 200, 104);
pub const BLOOD: Color32 = Color32::from_rgb(150, 30, 28);
pub const BLOOD_BRIGHT: Color32 = Color32::from_rgb(206, 52, 40);
pub const ASH: Color32 = Color32::from_rgb(128, 122, 118);
pub const PARCHMENT: Color32 = Color32::from_rgb(234, 220, 190);
pub const PARCHMENT_DIM: Color32 = Color32::from_rgb(170, 156, 128);
/// Tinte auf Pergament.
pub const INK: Color32 = Color32::from_rgb(52, 32, 18);
pub const INK_FADED: Color32 = Color32::from_rgb(96, 68, 42);
pub const INK_RED: Color32 = Color32::from_rgb(128, 26, 18);
pub const LEATHER: Color32 = Color32::from_rgb(28, 21, 18);
pub const LEATHER_LIGHT: Color32 = Color32::from_rgb(46, 34, 28);
pub const IRON: Color32 = Color32::from_rgb(62, 60, 64);
pub const IRON_DARK: Color32 = Color32::from_rgb(22, 21, 24);
pub const HOLLOW_GREEN: Color32 = Color32::from_rgb(120, 230, 150);
pub const LIFE: Color32 = Color32::from_rgb(70, 168, 64);
pub const MANA: Color32 = Color32::from_rgb(70, 120, 220);

/// Schriftfamilien des Designs (immer registriert, notfalls mit den eingebauten Schriften).
pub const FAMILY_TITLE: &str = "ac_title";
pub const FAMILY_TITLE_BOLD: &str = "ac_title_bold";
pub const FAMILY_HEADING: &str = "ac_heading";
pub const FAMILY_CAPS: &str = "ac_caps";
pub const FAMILY_BODY_BOLD: &str = "ac_body_bold";
pub const FAMILY_ITALIC: &str = "ac_italic";

fn family(name: &str) -> FontFamily {
    FontFamily::Name(name.into())
}

/// Zierschrift für große Titel (Cinzel Decorative Black).
pub fn title_font(size: f32) -> FontId {
    FontId::new(size, family(FAMILY_TITLE))
}
/// Zierschrift, etwas leichter (Cinzel Decorative Bold) – Untertitel, Sprechernamen.
pub fn title_bold_font(size: f32) -> FontId {
    FontId::new(size, family(FAMILY_TITLE_BOLD))
}
/// Überschriften (Cinzel, fett).
pub fn heading_font(size: f32) -> FontId {
    FontId::new(size, family(FAMILY_HEADING))
}
/// Versalien für Schaltflächen und Beschriftungen (Cinzel, halbfett).
pub fn caps_font(size: f32) -> FontId {
    FontId::new(size, family(FAMILY_CAPS))
}
/// Fließtext (Alegreya).
pub fn body_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}
pub fn body_bold_font(size: f32) -> FontId {
    FontId::new(size, family(FAMILY_BODY_BOLD))
}
pub fn italic_font(size: f32) -> FontId {
    FontId::new(size, family(FAMILY_ITALIC))
}

/// Lädt die Schriften aus `dir` (Unterordner `cinzel/`, `cinzel_decorative/`, `alegreya/`).
/// Fehlende Dateien fallen auf die eingebauten egui-Schriften zurück. Liefert die Anzahl
/// geladener Schriftdateien.
pub fn install_fonts(ctx: &egui::Context, dir: Option<&Path>) -> usize {
    let mut defs = FontDefinitions::default();
    let fallback: Vec<String> = defs.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let mut loaded = 0;
    let mut load = |file: &str| -> Option<Arc<Vec<u8>>> {
        let p = dir?.join(file);
        match std::fs::read(&p) {
            Ok(b) => {
                loaded += 1;
                Some(Arc::new(b))
            }
            Err(e) => {
                log::warn!("Schrift {} nicht geladen ({e}) – eingebaute Schrift wird verwendet", p.display());
                None
            }
        }
    };
    let cinzel = load("cinzel/Cinzel-Variable.ttf");
    let deco_black = load("cinzel_decorative/CinzelDecorative-Black.ttf");
    let deco_bold = load("cinzel_decorative/CinzelDecorative-Bold.ttf");
    let alegreya = load("alegreya/Alegreya-Variable.ttf");
    let alegreya_it = load("alegreya/Alegreya-Italic-Variable.ttf");

    let add = |defs: &mut FontDefinitions, name: &str, bytes: &Option<Arc<Vec<u8>>>, wght: Option<f32>, tweak_y: f32| -> Option<String> {
        let b = bytes.as_ref()?;
        let mut tweak = egui::FontTweak { y_offset_factor: tweak_y, ..Default::default() };
        if let Some(w) = wght {
            tweak.coords = egui::epaint::text::VariationCoords::new([("wght", w)]);
        }
        let data = FontData::from_blob(b.clone(), 0).tweak(tweak);
        defs.font_data.insert(name.to_string(), Arc::new(data));
        Some(name.to_string())
    };
    let body = add(&mut defs, "Alegreya-Medium", &alegreya, Some(500.0), 0.0);
    let body_bold = add(&mut defs, "Alegreya-Bold", &alegreya, Some(760.0), 0.0);
    let italic = add(&mut defs, "Alegreya-Italic", &alegreya_it, Some(500.0), 0.0);
    let heading = add(&mut defs, "Cinzel-Bold", &cinzel, Some(700.0), 0.04);
    let caps = add(&mut defs, "Cinzel-SemiBold", &cinzel, Some(600.0), 0.04);
    let title = add(&mut defs, "CinzelDecorative-Black", &deco_black, None, 0.02);
    let title_bold = add(&mut defs, "CinzelDecorative-Bold", &deco_bold, None, 0.02);

    let chain = |primary: &[&Option<String>]| -> Vec<String> {
        let mut v: Vec<String> = primary.iter().filter_map(|p| (*p).clone()).collect();
        v.extend(fallback.iter().cloned());
        v
    };
    defs.families.insert(FontFamily::Proportional, chain(&[&body]));
    defs.families.insert(family(FAMILY_BODY_BOLD), chain(&[&body_bold, &body]));
    defs.families.insert(family(FAMILY_ITALIC), chain(&[&italic, &body]));
    defs.families.insert(family(FAMILY_HEADING), chain(&[&heading, &body_bold]));
    defs.families.insert(family(FAMILY_CAPS), chain(&[&caps, &heading]));
    defs.families.insert(family(FAMILY_TITLE), chain(&[&title, &heading]));
    defs.families.insert(family(FAMILY_TITLE_BOLD), chain(&[&title_bold, &title, &heading]));
    ctx.set_fonts(defs);
    loaded
}

pub fn apply(ctx: &egui::Context) {
    // Familien sofort binden (eingebaute Schriften), damit benannte Familien nie fehlen.
    install_fonts(ctx, None);
    ctx.all_styles_mut(|style| {
        style.text_styles = [
            (TextStyle::Heading, heading_font(22.0)),
            (TextStyle::Body, body_font(16.0)),
            (TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace)),
            (TextStyle::Button, caps_font(14.5)),
            (TextStyle::Small, body_font(13.5)),
        ]
        .into();
        let s = &mut style.spacing;
        s.item_spacing = egui::vec2(8.0, 6.0);
        s.button_padding = egui::vec2(12.0, 5.0);
        s.window_margin = Margin::same(14);
        s.menu_margin = Margin::same(8);
        s.interact_size = egui::vec2(36.0, 24.0);
        s.slider_width = 200.0;
        s.slider_rail_height = 6.0;
        s.icon_width = 18.0;
        s.icon_width_inner = 10.0;
        s.icon_spacing = 6.0;
        s.combo_width = 200.0;
        s.tooltip_width = 340.0;
        let mut scroll = egui::style::ScrollStyle::solid();
        scroll.bar_width = 8.0;
        scroll.handle_min_length = 24.0;
        scroll.bar_inner_margin = 4.0;
        scroll.bar_outer_margin = 2.0;
        s.scroll = scroll;

        let v = &mut style.visuals;
        v.dark_mode = true;
        v.override_text_color = Some(PARCHMENT);
        v.weak_text_color = Some(PARCHMENT_DIM);
        v.panel_fill = Color32::from_rgb(20, 15, 13);
        v.window_fill = Color32::from_rgba_unmultiplied(22, 17, 15, 246);
        v.window_stroke = Stroke::new(1.5, GOLD_DIM);
        v.window_corner_radius = CornerRadius::same(2);
        v.window_shadow = Shadow { offset: [0, 8], blur: 22, spread: 2, color: Color32::from_black_alpha(190) };
        v.popup_shadow = Shadow { offset: [0, 6], blur: 16, spread: 0, color: Color32::from_black_alpha(200) };
        v.menu_corner_radius = CornerRadius::same(2);
        v.extreme_bg_color = Color32::from_rgb(12, 9, 8);
        v.text_edit_bg_color = Some(Color32::from_rgb(14, 10, 9));
        v.faint_bg_color = LEATHER_LIGHT;
        v.code_bg_color = Color32::from_rgb(30, 24, 20);
        v.hyperlink_color = GOLD;
        v.selection.bg_fill = Color32::from_rgb(110, 62, 24);
        v.selection.stroke = Stroke::new(1.0, EMBER_HOT);
        v.warn_fg_color = EMBER;
        v.error_fg_color = Color32::from_rgb(230, 70, 60);
        v.slider_trailing_fill = true;
        v.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 0.55 };
        v.striped = false;
        v.button_frame = true;
        v.collapsing_header_frame = true;
        v.indent_has_left_vline = false;
        v.window_highlight_topmost = false;
        v.interact_cursor = Some(egui::CursorIcon::PointingHand);

        let w = &mut v.widgets;
        // Nicht interaktiv: Trennlinien, Rahmen, Beschriftungen.
        w.noninteractive.bg_fill = Color32::from_rgb(22, 17, 15);
        w.noninteractive.weak_bg_fill = Color32::from_rgb(22, 17, 15);
        w.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(92, 68, 36));
        w.noninteractive.fg_stroke = Stroke::new(1.0, PARCHMENT);
        // Ruhend: dunkles Eisen mit Bronzekante.
        w.inactive.bg_fill = Color32::from_rgb(40, 31, 26);
        w.inactive.weak_bg_fill = Color32::from_rgb(36, 28, 24);
        w.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(104, 76, 40));
        w.inactive.fg_stroke = Stroke::new(1.5, Color32::from_rgb(222, 204, 168));
        w.inactive.expansion = 0.0;
        // Hover: Glutkante.
        w.hovered.bg_fill = Color32::from_rgb(74, 46, 26);
        w.hovered.weak_bg_fill = Color32::from_rgb(64, 40, 24);
        w.hovered.bg_stroke = Stroke::new(1.5, EMBER);
        w.hovered.fg_stroke = Stroke::new(1.5, GOLD_BRIGHT);
        w.hovered.expansion = 1.0;
        // Gedrückt: glühend.
        w.active.bg_fill = Color32::from_rgb(120, 64, 26);
        w.active.weak_bg_fill = Color32::from_rgb(104, 56, 24);
        w.active.bg_stroke = Stroke::new(1.5, EMBER_HOT);
        w.active.fg_stroke = Stroke::new(2.0, Color32::WHITE);
        w.active.expansion = 1.0;
        w.open = w.hovered;
        for wv in [&mut w.noninteractive, &mut w.inactive, &mut w.hovered, &mut w.active, &mut w.open] {
            wv.corner_radius = CornerRadius::same(2);
        }
    });
}

/// Rahmen für einfache Ingame-Paneele (geschwärztes Leder mit Goldkante).
pub fn panel_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(20, 15, 13, 236))
        .stroke(Stroke::new(1.5, GOLD_DIM))
        .corner_radius(CornerRadius::same(2))
        .inner_margin(Margin::same(8))
        .shadow(Shadow { offset: [0, 4], blur: 12, spread: 0, color: Color32::from_black_alpha(150) })
}

/// Einfacher Pergament-Rahmen (Rückfall, wenn keine Texturen vorhanden sind).
pub fn parchment_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(54, 42, 30, 236))
        .stroke(Stroke::new(1.0, Color32::from_rgb(120, 92, 54)))
        .corner_radius(CornerRadius::same(2))
        .inner_margin(Margin::symmetric(12, 8))
}

/// Überschrift im Gold-Stil (Cinzel).
pub fn heading(text: impl Into<String>, size: f32) -> egui::RichText {
    egui::RichText::new(text.into()).color(GOLD).font(heading_font(size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_families_always_bound() {
        // Ohne Schriftdateien müssen alle Familien auf die eingebauten Schriften zurückfallen.
        let ctx = egui::Context::default();
        apply(&ctx);
        install_fonts(&ctx, Some(Path::new("/nicht/vorhanden")));
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            for f in [title_font(30.0), title_bold_font(20.0), heading_font(18.0), caps_font(14.0), body_font(14.0), body_bold_font(14.0), italic_font(14.0)] {
                ui.label(egui::RichText::new("Glut & Asche – ÄÖÜ ×").font(f));
            }
        });
        out.textures_delta.clear();
    }
}
