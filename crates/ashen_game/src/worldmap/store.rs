//! Speichern und Laden der Grand Campaign (`.acstrat`) sowie Rückkehrpunkte laufender Schlachten.
//!
//! Während einer gespielten Schlacht ruht die Weltzeit. Beim Start der Schlacht wird der strategische Zustand
//! (mit der noch offenen Schlachtanfrage) als Rückkehrpunkt unter `saves/battle_return/` abgelegt; jeder
//! Echtzeit-Spielstand der Schlacht verweist darauf (`BattleLink`). So bleibt der Gesamtzustand konsistent.

use crate::settings::saves_dir;
use crate::ui_art::{self, BtnState, UiArt};
use crate::ui_common;
use ashen_engine::Engine;
use ashen_engine::ui::theme;
use ashen_strategy::save::{self, SaveError, StrategySaveHeader};
use ashen_strategy::{StrategyData, StrategyWorld};
use egui::emath::GuiRounding;
use egui::{Align2, Color32, RichText, Sense, Vec2};
use std::path::{Path, PathBuf};

pub fn return_dir() -> PathBuf {
    saves_dir().join("battle_return")
}

/// Speichert die Welt im Standardordner unter dem Namen `slot`.
pub fn save_slot(world: &StrategyWorld, slot: &str) -> Result<PathBuf, SaveError> {
    save::save_to(&saves_dir(), slot, world)
}

pub fn autosave(world: &StrategyWorld) -> Result<PathBuf, SaveError> {
    save::autosave(&saves_dir(), world, 3)
}

pub fn load(path: &Path) -> Result<StrategyWorld, SaveError> {
    save::load_from(path, StrategyData::builtin()).map(|(_, w)| w)
}

/// Rückkehrpunkt einer Schlacht schreiben; liefert den Dateinamen.
pub fn write_return_point(world: &StrategyWorld, battle_id: u32) -> Result<String, SaveError> {
    let slot = format!(
        "rueckkehr_{}_{}_{}",
        world.options.seed, world.day, battle_id
    );
    let p = save::save_to(&return_dir(), &slot, world)?;
    Ok(p.file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or(slot))
}

pub fn load_return_point(name: &str) -> Result<StrategyWorld, SaveError> {
    load(&return_dir().join(name))
}

/// Spielstandliste der Grand Campaign mit eigener Kennzeichnung; liefert den gewählten Pfad.
pub fn save_list(
    ui: &mut egui::Ui,
    engine: &mut Engine,
    art: &UiArt,
    dir: &Path,
) -> Option<PathBuf> {
    let saves = save::list(dir);
    let mut chosen = None;
    if saves.is_empty() {
        return None;
    }
    egui::ScrollArea::vertical()
        .id_salt("strat_saves")
        .max_height(220.0)
        .show(ui, |ui| {
            for (path, header) in saves {
                match header {
                    Ok(h) => {
                        let w = ui.available_width().max(360.0);
                        let (rect, resp) =
                            ui.allocate_exact_size(Vec2::new(w, 50.0), Sense::click());
                        let st = BtnState::of(&resp, true);
                        art.button_nine(st)
                            .paint(ui.painter(), rect.round_ui(), Color32::WHITE);
                        let p = ui.painter();
                        let name_col = if st == BtnState::Hover {
                            theme::GOLD_BRIGHT
                        } else {
                            theme::GOLD
                        };
                        p.text(
                            rect.left_top() + Vec2::new(22.0, 9.0),
                            Align2::LEFT_TOP,
                            &h.name,
                            theme::heading_font(15.0),
                            name_col,
                        );
                        p.text(
                            rect.left_bottom() + Vec2::new(22.0, -9.0),
                            Align2::LEFT_BOTTOM,
                            describe(&h),
                            theme::italic_font(13.5),
                            theme::PARCHMENT_DIM,
                        );
                        // Kennzeichnung als Strategie-Spielstand
                        let badge = "GROSSE KAMPAGNE";
                        let bw = p
                            .layout_no_wrap(badge.into(), theme::caps_font(11.0), Color32::WHITE)
                            .size()
                            .x
                            + 16.0;
                        let br = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.max.x - bw - 16.0, rect.min.y + 8.0),
                            Vec2::new(bw, 17.0),
                        );
                        p.rect_filled(br, 2.0, Color32::from_rgb(86, 28, 20));
                        p.rect_stroke(
                            br,
                            2.0,
                            egui::Stroke::new(1.0, theme::GOLD_DIM),
                            egui::StrokeKind::Inside,
                        );
                        p.text(
                            br.center(),
                            Align2::CENTER_CENTER,
                            badge,
                            theme::caps_font(11.0),
                            theme::GOLD_BRIGHT,
                        );
                        ui_common::sfx(engine, ui, &resp);
                        if resp.clicked() {
                            chosen = Some(path.clone());
                        }
                        ui.add_space(2.0);
                    }
                    Err(e) => {
                        ui.label(
                            RichText::new(format!("{}: {e}", path.display()))
                                .color(theme::EMBER)
                                .small(),
                        );
                    }
                }
            }
        });
    let _ = ui_art::PanelKind::Thin;
    chosen
}

fn describe(h: &StrategySaveHeader) -> String {
    format!(
        "{} · Jahr {} · {} · {}",
        h.player_realm
            .clone()
            .unwrap_or_else(|| "ohne Spielerreich".into()),
        h.year,
        h.difficulty,
        h.mode
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ashen_strategy::StrategyOptions;

    #[test]
    fn campaign_save_roundtrip_through_game_layer() {
        let dir = std::env::temp_dir().join(format!("ashen_wm_test_{}", std::process::id()));
        let mut w = StrategyWorld::new(StrategyOptions::campaign(4242));
        w.advance_days(40);
        let path = save::save_to(&dir, "Test Spielstand", &w).expect("speichern");
        assert_eq!(
            path.extension().and_then(|e| e.to_str()),
            Some(save::EXTENSION)
        );
        let listed = save::list(&dir);
        assert_eq!(listed.len(), 1);
        let header = listed[0].1.as_ref().expect("Kopfdaten");
        assert_eq!(header.day, w.day);
        let loaded = load(&path).expect("laden");
        assert_eq!(
            loaded.state_hash(),
            w.state_hash(),
            "geladener Zustand ist identisch"
        );
        // Rückkehrpunkt-Datei im Unterordner wird nicht in der Hauptliste geführt.
        let rp =
            save::save_to(&dir.join("battle_return"), "rueckkehr_1", &w).expect("Rückkehrpunkt");
        assert!(rp.exists());
        assert_eq!(save::list(&dir).len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
