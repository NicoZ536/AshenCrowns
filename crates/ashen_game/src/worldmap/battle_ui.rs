//! Oberfläche über der Echtzeitschlacht, die aus der Weltkarte gestartet wurde: Schlachtleiste (Rückzug,
//! automatisch beenden) und Ergebnisdialog mit Verlusten und Heldenschicksal.

use super::WorldMapScene;
use super::battle::{BattleReport, LossRow};
use crate::mission::MissionScene;
use crate::ui_art::{self, PanelKind, TextFx, UiArt};
use crate::ui_common;
use ashen_engine::Engine;
use ashen_engine::ui::theme;
use ashen_strategy::{BattleRequest, HeroOutcome};
use egui::{Align2, Color32, Id, Order, RichText, Sense, Vec2};

/// Zustand einer laufenden Weltkarten-Schlacht; die Weltkartenszene ruht währenddessen.
pub struct BattleCtx {
    pub world: Box<WorldMapScene>,
    pub req: BattleRequest,
    pub scale: u32,
    pub player_attacks: bool,
    pub report: Option<BattleReport>,
    pub enemy_timer: f32,
}

pub enum BattleUiAction {
    None,
    Retreat,
    AutoResolve,
    Finish,
}

fn loss_table(ui: &mut egui::Ui, title: &str, rows: &[LossRow], mine: bool) {
    ui.label(
        RichText::new(title)
            .font(theme::caps_font(12.5))
            .color(theme::INK_RED),
    );
    if rows.is_empty() {
        ui.label(RichText::new("–").color(theme::INK));
    }
    for r in rows {
        let lost = r.before.saturating_sub(r.after);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(&r.unit)
                    .font(theme::body_font(14.5))
                    .color(theme::INK),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let col = if lost == 0 {
                    Color32::from_rgb(50, 110, 40)
                } else if mine {
                    theme::INK_RED
                } else {
                    theme::INK
                };
                ui.label(
                    RichText::new(format!("{} → {} (−{lost})", r.before, r.after))
                        .font(theme::body_bold_font(14.5))
                        .color(col),
                );
            });
        });
    }
}

pub fn draw(
    ctx: &egui::Context,
    engine: &mut Engine,
    art: &UiArt,
    bc: &mut BattleCtx,
    mission: &MissionScene,
) -> BattleUiAction {
    let mut action = BattleUiAction::None;
    if bc.report.is_none() {
        egui::Area::new(Id::unique("battle_bar"))
            .order(Order::Foreground)
            .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 58.0))
            .show(ctx, |ui| {
                art.panel(ui, PanelKind::Thin, egui::Margin::symmetric(18, 8), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(&mission.world.mission_def.name)
                                .font(theme::heading_font(15.0))
                                .color(theme::GOLD),
                        );
                        ui.add_space(8.0);
                        let secs = mission.world.time as u32;
                        ui.label(
                            RichText::new(format!("{}:{:02}", secs / 60, secs % 60))
                                .font(theme::body_bold_font(14.0))
                                .color(theme::PARCHMENT),
                        );
                        ui.add_space(8.0);
                        if ui_common::button(ui, engine, art, "Rückzug", 110.0) {
                            action = BattleUiAction::Retreat;
                        }
                        if ui_common::button(ui, engine, art, "Automatisch beenden", 190.0) {
                            action = BattleUiAction::AutoResolve;
                        }
                    });
                });
            });
        return action;
    }
    let report = bc.report.as_ref().expect("Bericht");
    let victory = report.player_won;
    let p = ctx.layer_painter(egui::LayerId::new(Order::Middle, Id::unique("battle_dim")));
    p.rect_filled(ctx.content_rect(), 0.0, Color32::from_black_alpha(150));
    egui::Area::new(Id::unique("battle_result"))
        .order(Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            art.panel_tinted(
                ui,
                PanelKind::Ornate,
                egui::Margin::symmetric(40, 30),
                if victory {
                    Color32::WHITE
                } else {
                    Color32::from_rgb(196, 150, 140)
                },
                |ui| {
                    ui.set_width(600.0);
                    ui.vertical_centered(|ui| {
                        let (r, _) = ui.allocate_exact_size(Vec2::new(600.0, 70.0), Sense::hover());
                        ui_art::fx_text(
                            art,
                            ui.painter(),
                            r.center(),
                            Align2::CENTER_CENTER,
                            if victory { "Sieg" } else { "Niederlage" },
                            theme::title_font(52.0),
                            &if victory {
                                TextFx::gold()
                            } else {
                                TextFx::ash()
                            },
                        );
                        ui.label(
                            RichText::new(&mission.world.mission_def.name)
                                .font(theme::title_bold_font(17.0))
                                .color(if victory {
                                    theme::GOLD
                                } else {
                                    Color32::from_rgb(190, 120, 110)
                                }),
                        );
                        let secs = mission.world.time as u32;
                        ui.label(
                            RichText::new(format!(
                                "Dauer {}:{:02} · {}",
                                secs / 60,
                                secs % 60,
                                if report.result.city_taken {
                                    "Die Stadt wurde erstürmt."
                                } else {
                                    "Das Feld ist entschieden."
                                }
                            ))
                            .font(theme::italic_font(15.0))
                            .color(theme::PARCHMENT_DIM),
                        );
                    });
                    ui.add_space(8.0);
                    art.panel(
                        ui,
                        PanelKind::Parchment,
                        egui::Margin {
                            left: 26,
                            right: 26,
                            top: 16,
                            bottom: 16,
                        },
                        |ui| {
                            ui.set_width(ui.available_width());
                            loss_table(ui, "Eure Truppen", &report.my_losses, true);
                            ui.add_space(6.0);
                            loss_table(ui, "Gegner", &report.enemy_losses, false);
                            if !report.hero_notes.is_empty() {
                                ui.add_space(6.0);
                                ui.label(
                                    RichText::new("Helden")
                                        .font(theme::caps_font(12.5))
                                        .color(theme::INK_RED),
                                );
                                for (c, o) in &report.hero_notes {
                                    let name = bc.world.world.cname(*c);
                                    let t = match o {
                                        HeroOutcome::Unharmed => "unversehrt",
                                        HeroOutcome::Wounded => "verwundet",
                                        HeroOutcome::Captured => "gefangen",
                                        HeroOutcome::Dead => "gefallen",
                                    };
                                    ui.label(
                                        RichText::new(format!("{name}: {t}"))
                                            .font(theme::body_bold_font(14.5))
                                            .color(theme::INK),
                                    );
                                }
                            }
                        },
                    );
                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        if ui_common::button(ui, engine, art, "Zurück zur Weltkarte", 300.0) {
                            action = BattleUiAction::Finish;
                        }
                    });
                },
            );
        });
    action
}
