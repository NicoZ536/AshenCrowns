//! Ingame-Oberfläche: Ressourcenleiste, Minimap, Auswahl, Befehlskarte, Ziele, Nachrichten,
//! Pausenmenü sowie Sieg-/Niederlagenbildschirm – im Stil von geschmiedetem Eisen, Leder,
//! Pergament und Glut (Grafik aus [`crate::ui_art`]).

use crate::art::ArtLibrary;
use crate::mission::{CommandMode, MissionScene, SceneAction};
use crate::settings::{saves_dir, UserSettings};
use crate::ui_art::{self, BtnState, PanelKind, TextFx, UiArt};
use crate::ui_common;
use ashen_core::iso::{tile_to_world, world_to_tile};
use ashen_engine::render::SpriteRect;
use ashen_engine::ui::theme;
use ashen_engine::Engine;
use ashen_sim::data::{Cost, GameData, ResourceKind, UnitClass, UnitTypeId};
use ashen_sim::entity::{BuildingId, NodeKind, PLAYER};
use ashen_sim::fog::{EXPLORED, UNEXPLORED};
use ashen_sim::map::Terrain;
use ashen_sim::mission::{ObjectiveState, Outcome};
use egui::emath::GuiRounding;
use egui::{Align2, Color32, CornerRadius, Frame, Id, Order, Pos2, Rect, RichText, Sense, Shape, Stroke, StrokeKind, Ui, UiBuilder, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuPage {
    Main,
    Save,
    Load,
    Settings,
    Chronicle,
}

pub struct HudCtx<'a> {
    pub data: &'a GameData,
    pub art: &'a ArtLibrary,
    pub tex: egui::TextureId,
    pub ui: &'a UiArt,
    pub settings: &'a mut UserSettings,
}

/// Höhe der oberen Leiste und der unteren Konsole (Punkte).
pub(crate) const TOP_H: f32 = 50.0;
pub(crate) const CONSOLE_H: f32 = 196.0;
const MINIMAP_W: f32 = 326.0;
const SLOT: f32 = 48.0;
const SLOT_GAP: f32 = 6.0;
const CARD_W: f32 = SLOT * 4.0 + SLOT_GAP * 3.0 + 40.0;

pub fn draw(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    top_bar(scene, ui, engine, cx);
    console(scene, ui, engine, cx);
    crate::hud_hero::draw(scene, ui, engine, cx);
    if !scene.world.mission_def.free_economy {
        crate::hud_city::draw(scene, ui, engine, cx);
    }
    objectives(scene, ui, engine, cx);
    messages(scene, ui, cx);
    if let Some(start) = scene.drag_start {
        let ppp = ui.ctx().pixels_per_point();
        let a = Pos2::new(start.x / ppp, start.y / ppp);
        let b = Pos2::new(engine.input.mouse.x / ppp, engine.input.mouse.y / ppp);
        let r = Rect::from_two_pos(a, b);
        if r.width() > 3.0 || r.height() > 3.0 {
            let painter = ui.ctx().layer_painter(egui::LayerId::new(Order::Foreground, Id::unique("selbox")));
            painter.rect_filled(r, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(120, 220, 120, 22));
            painter.rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.0, Color32::from_rgb(150, 236, 150)), StrokeKind::Inside);
            for c in [r.left_top(), r.right_top(), r.left_bottom(), r.right_bottom()] {
                painter.rect_filled(Rect::from_center_size(c, Vec2::splat(4.0)), 0.0, Color32::from_rgb(190, 255, 190));
            }
        }
    }
    mode_hint(scene, ui, cx);
    if scene.menu_open {
        pause_menu(scene, ui, engine, cx);
    }
    if let Some(o) = scene.world.mission.outcome {
        outcome(scene, ui, engine, cx, o);
    } else if scene.world.mission.open_decision(scene.world.time).is_some() {
        decision(scene, ui, engine, cx);
    }
}

/// Entscheidungsfenster der Kampagne: Frage der Figur und die Antworten; die Simulation ruht währenddessen.
fn decision(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let Some(d) = scene.world.mission.open_decision(scene.world.time).cloned() else { return };
    let ctx = ui.ctx().clone();
    let art = cx.ui;
    let fx_layer = dim(&ctx, art, Color32::from_rgb(30, 14, 6), 150);
    let t = ui.input(|i| i.time) as f32;
    ui_art::embers(art, &fx_layer, ctx.content_rect(), t, 36, 11);
    let mut chosen: Option<usize> = None;
    egui::Area::new(Id::unique("decision")).order(Order::Foreground).anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(&ctx, |ui| {
        art.panel(ui, PanelKind::Ornate, egui::Margin::symmetric(40, 30), |ui| {
            ui.set_width(660.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Entscheidung").font(theme::caps_font(13.0)).color(theme::PARCHMENT_DIM));
                ui.label(RichText::new(&d.title).font(theme::title_bold_font(26.0)).color(theme::GOLD));
                let (r, _) = ui.allocate_exact_size(Vec2::new(620.0, 22.0), Sense::hover());
                art.divider(ui.painter(), r.center().x - 200.0, r.center().x + 200.0, r.center().y);
            });
            art.panel(ui, PanelKind::Parchment, egui::Margin { left: 26, right: 26, top: 18, bottom: 18 }, |ui| {
                ui.set_width(ui.available_width());
                ui.label(RichText::new(&d.speaker).font(theme::caps_font(13.0)).color(theme::INK_RED));
                ui.label(RichText::new(&d.text).font(theme::italic_font(17.0)).color(theme::INK));
            });
            ui.add_space(10.0);
            for (i, o) in d.options.iter().enumerate() {
                let open = o.requires.iter().all(|f| scene.world.mission.has_flag(f));
                ui.vertical_centered(|ui| {
                    let r = ui_art::button(ui, art, &o.label, Vec2::new(600.0, 40.0), open);
                    ui_common::sfx(engine, ui, &r);
                    let detail = if open { o.detail.clone() } else { format!("Verschlossen: {}", o.detail) };
                    let r = ui_art::tip(r, art, &o.label, &detail);
                    if r.clicked() && open {
                        chosen = Some(i);
                    }
                    if !o.detail.is_empty() {
                        ui.label(RichText::new(&o.detail).font(theme::italic_font(13.5)).color(if open { theme::PARCHMENT_DIM } else { theme::EMBER }));
                    }
                });
                ui.add_space(4.0);
            }
        });
    });
    if let Some(i) = chosen {
        scene.decide(&d.id, i);
    }
}

/// Tagesabschnitt für die Anzeige (Morgen, Mittag, Nachmittag, Abend, Nacht).
fn day_part(env: &ashen_sim::env::Environment) -> &'static str {
    use ashen_sim::env::DayPhase;
    let t = env.time_of_day;
    match env.phase() {
        DayPhase::Dawn => "Morgen",
        DayPhase::Day if t < 0.40 => "Morgen",
        DayPhase::Day if t < 0.62 => "Mittag",
        DayPhase::Day => "Nachmittag",
        DayPhase::Dusk => "Abend",
        DayPhase::Night => "Nacht",
    }
}

/// Schlachtleiste statt Ressourcenplakette (Weltkarten-Schlachten ohne Wirtschaft): Stärke und Moral beider Seiten.
fn battle_cells(world: &ashen_sim::SimWorld, data: &GameData) -> Vec<(&'static str, String, String, String, bool)> {
    use ashen_sim::entity::ENEMY;
    let side = |owner| {
        let (mut n, mut morale) = (0u32, 0.0f32);
        for (_, u) in world.units.iter().filter(|(_, u)| u.owner == owner) {
            let def = data.unit(u.kind);
            if def.class == UnitClass::Worker {
                continue;
            }
            n += 1;
            morale += (u.morale / def.morale.max(1.0)).clamp(0.0, 1.0);
        }
        (n, if n > 0 { morale / n as f32 * 100.0 } else { 0.0 })
    };
    let ((mine, mm), (foe, fm)) = (side(PLAYER), side(ENEMY));
    vec![
        ("icon_pop", format!("{mine}"), "Eure Streitmacht".into(), "Einsatzbereite Einheiten Eurer Heere auf dem Schlachtfeld.".into(), mine * 2 < foe),
        ("icon_attack", format!("{foe}"), "Gegner".into(), "Einheiten des Feindes auf dem Schlachtfeld.".into(), false),
        ("icon_happy", format!("{mm:.0}%"), "Eure Moral".into(), "Durchschnittliche Moral Eurer Einheiten. Bricht sie, fliehen die Truppen.".into(), mm < 40.0),
        ("icon_sad", format!("{fm:.0}%"), "Moral des Gegners".into(), "Durchschnittliche Moral der feindlichen Einheiten.".into(), false),
    ]
}

/// Nahrungsverbrauch der Bewohner pro Sekunde.
fn resident_food(world: &ashen_sim::SimWorld, data: &GameData) -> f32 {
    world
        .buildings
        .iter()
        .filter(|(_, b)| b.owner == PLAYER && b.complete)
        .filter_map(|(_, b)| data.building(b.kind).housing.as_ref().map(|h| (b.residents, h)))
        .map(|(r, h)| h.needs.iter().filter(|n| n.item_r == Some(ashen_sim::data::Item::Res(ResourceKind::Food))).map(|n| r * n.rate / 60.0).sum::<f32>())
        .sum()
}

fn res_icon(k: ResourceKind) -> &'static str {
    match k {
        ResourceKind::Gold => "icon_gold",
        ResourceKind::Wood => "icon_wood",
        ResourceKind::Stone => "icon_stone",
        ResourceKind::Food => "icon_food",
    }
}

pub(crate) fn sprite(cx: &HudCtx, name: &str) -> SpriteRect {
    cx.art.atlas.try_get(name).unwrap_or_else(|| cx.art.get("icon_cancel"))
}

/// Text mit dunklem Schlagschatten (Lesbarkeit auf Texturen).
pub(crate) fn shadow_text(p: &egui::Painter, pos: Pos2, anchor: Align2, text: &str, font: egui::FontId, col: Color32) -> Rect {
    p.text(pos + Vec2::new(1.0, 1.5), anchor, text, font.clone(), Color32::from_black_alpha(220));
    p.text(pos, anchor, text, font, col)
}

// ---------------------------------------------------------------------------------------------
// Obere Leiste
// ---------------------------------------------------------------------------------------------

fn top_bar(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let art = cx.ui;
    let p = &scene.world.players[PLAYER as usize];
    let res = p.resources;
    let (pop, cap) = (p.pop, p.pop_cap);
    let starving = p.starving;
    let battle = scene.world.mission_def.free_economy;
    let food_rate: f32 = scene.world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.complete).map(|(_, b)| cx.data.building(b.kind).food_per_sec).sum::<f32>()
        - scene.world.units.iter().filter(|(_, u)| u.owner == PLAYER).map(|(_, u)| cx.data.unit(u.kind).upkeep).sum::<f32>()
        - resident_food(&scene.world, cx.data);
    let food_rate = if food_rate.abs() < 0.05 { 0.0 } else { food_rate };
    egui::Panel::top("topbar").exact_size(TOP_H).frame(Frame::NONE).show(ui, |ui| {
        let rect = ui.max_rect();
        let painter = ui.painter().clone();
        painter.add(art.fill_shape(rect, false, Color32::from_gray(210)));
        // Herabhängendes Eisenband (überlappt die Spielwelt).
        let fg = ui.ctx().layer_painter(egui::LayerId::new(Order::Background, Id::unique("topband")));
        art.band(&fg, rect.min.x, rect.max.x, rect.max.y - 4.0, true);

        // Ressourcenplakette.
        let cells: Vec<(&str, String, String, String, bool)> = if battle { battle_cells(&scene.world, cx.data) } else { vec![
            ("icon_gold", format!("{}", res.gold.floor()), "Gold".into(), "Aus der Mine, Abgaben des Bergfrieds.".into(), false),
            ("icon_wood", format!("{}", res.wood.floor()), "Holz".into(), "Von Holzfällern geschlagen.".into(), false),
            ("icon_stone", format!("{}", res.stone.floor()), "Stein".into(), "Aus dem Steinbruch.".into(), false),
            (
                "icon_food",
                format!("{}", res.food.floor()),
                "Nahrung".into(),
                format!("Gehöfte erzeugen sie, jede Einheit verbraucht sie.\nBilanz: {food_rate:+.1} pro Sekunde{}", if starving { "\nHungersnot! Einheiten verlieren Kraft." } else { "" }),
                starving || food_rate < 0.0 && res.food < 60.0,
            ),
            ("icon_pop", format!("{pop}/{cap}"), "Bevölkerung".into(), "Einheiten / Versorgung. Gehöfte und der Bergfried erhöhen die Versorgung; Bürger wohnen in Hütten und Häusern.".into(), pop >= cap),
        ] };
        let cell_w: Vec<f32> = if battle { vec![104.0; 4] } else { vec![96.0, 96.0, 96.0, 136.0, 104.0] };
        let plaque = Rect::from_min_size(rect.min + Vec2::new(8.0, 4.0), Vec2::new(cell_w.iter().sum::<f32>() + 28.0, TOP_H - 4.0));
        for s in art.panel_shapes(PanelKind::Plaque, plaque, false) {
            painter.add(s);
        }
        let mut x = plaque.min.x + 14.0;
        for (i, (icon, value, title, body, warn)) in cells.iter().enumerate() {
            let cell = Rect::from_min_size(Pos2::new(x, plaque.min.y + 6.0), Vec2::new(cell_w[i], plaque.height() - 12.0));
            let ir = Rect::from_center_size(Pos2::new(cell.min.x + 16.0, cell.center().y), Vec2::splat(32.0));
            art.soft_shadow(&painter, ir.expand(4.0), 0.5);
            ui_common::paint_sprite(&painter, cx.tex, sprite(cx, icon), ir, Color32::WHITE);
            let col = if *warn { theme::EMBER } else { theme::PARCHMENT };
            if *warn {
                let t = ui.input(|i| i.time) as f32;
                let a = 0.5 + 0.5 * (t * 5.0).sin();
                art.glow(&painter, Rect::from_center_size(Pos2::new(cell.min.x + 60.0, cell.center().y), Vec2::new(90.0, 36.0)), Color32::from_rgb((90.0 * a) as u8, (24.0 * a) as u8, 0));
            }
            shadow_text(&painter, Pos2::new(cell.min.x + 37.0, cell.center().y + 1.0), Align2::LEFT_CENTER, value, theme::heading_font(17.0), col);
            if i == 3 && !battle {
                let rate_col = if food_rate < 0.0 { Color32::from_rgb(226, 110, 80) } else { Color32::from_rgb(150, 200, 120) };
                let vw = painter.layout_no_wrap(value.clone(), theme::heading_font(17.0), col).size().x;
                shadow_text(&painter, Pos2::new(cell.min.x + 41.0 + vw, cell.center().y + 2.0), Align2::LEFT_CENTER, &format!("{food_rate:+.1}/s"), theme::body_bold_font(13.0), rate_col);
            }
            let resp = ui.interact(cell, Id::unique("res").with(i), Sense::hover());
            ui_art::tip(resp, art, title, body);
            x += cell_w[i];
            if i + 1 < cells.len() {
                let d = Pos2::new(x - 4.0, cell.center().y);
                painter.add(Shape::convex_polygon(vec![d + Vec2::new(0.0, -4.0), d + Vec2::new(3.0, 0.0), d + Vec2::new(0.0, 4.0), d + Vec2::new(-3.0, 0.0)], theme::GOLD_DIM, Stroke::new(1.0, Color32::from_rgb(20, 12, 6))));
            }
        }

        // Datum, Tageszeit, Wetter (Mitte).
        let env = &scene.world.env;
        let mins = (scene.world.time / 60.0) as u32;
        let center = Pos2::new(rect.center().x.max(plaque.max.x + 150.0), rect.center().y);
        let line1 = format!("Tag {} · {}", env.day, day_part(env));
        let line2 = format!("{} · {:02}:{:02}", env.weather.label(), mins / 60, mins % 60);
        let w1 = painter.layout_no_wrap(line1.clone(), theme::heading_font(16.0), theme::GOLD).size().x;
        let w2 = painter.layout_no_wrap(line2.clone(), theme::italic_font(14.5), theme::GOLD).size().x;
        let x0 = center.x - (w1 + 14.0 + w2) / 2.0;
        let half = (w1 + 14.0 + w2) / 2.0 + 30.0;
        let dp = Rect::from_center_size(center, Vec2::new(half * 2.0, TOP_H - 4.0));
        for sh in art.panel_shapes(PanelKind::Plaque, dp, false) {
            painter.add(sh);
        }
        for sx in [-1.0, 1.0] {
            let d = Pos2::new(center.x + sx * (half + 14.0), center.y);
            painter.add(Shape::convex_polygon(vec![d + Vec2::new(0.0, -6.0), d + Vec2::new(5.0, 0.0), d + Vec2::new(0.0, 6.0), d + Vec2::new(-5.0, 0.0)], theme::GOLD_DIM, Stroke::new(1.0, Color32::from_rgb(20, 12, 6))));
            painter.line_segment([Pos2::new(center.x + sx * (half + 8.0), center.y), Pos2::new(center.x + sx * (half + 1.0), center.y)], Stroke::new(1.5, theme::GOLD_DIM));
        }
        shadow_text(&painter, Pos2::new(x0, center.y + 1.0), Align2::LEFT_CENTER, &line1, theme::heading_font(16.0), theme::GOLD);
        shadow_text(&painter, Pos2::new(x0 + w1 + 14.0, center.y + 2.0), Align2::LEFT_CENTER, &line2, theme::italic_font(14.5), theme::PARCHMENT_DIM);
        let resp = ui.interact(dp, Id::unique("date"), Sense::hover());
        ui_art::tip(resp, art, &format!("Jahr 897 n. F. · Tag {}", env.day), &format!("{} · Wetter: {} · Spielzeit {:02}:{:02}", day_part(env), env.weather.label(), mins / 60, mins % 60));

        // Rechts: Welle, Tempo, Menü.
        let right = UiBuilder::new().max_rect(Rect::from_min_max(Pos2::new(center.x + 140.0, rect.min.y), Pos2::new(rect.max.x - 10.0, rect.max.y))).layout(egui::Layout::right_to_left(egui::Align::Center));
        ui.scope_builder(right, |ui| {
            let r = ui_art::button(ui, art, "Menü", Vec2::new(86.0, 34.0), true);
            ui_common::sfx(engine, ui, &r);
            if ui_art::tip(r, art, "Pausenmenü", "Speichern, Laden, Einstellungen (Esc)").clicked() {
                scene.menu_open = true;
                scene.menu_page = MenuPage::Main;
            }
            ui.add_space(10.0);
            for (label, s) in [("3×", 3.0), ("2×", 2.0), ("1×", 1.0), ("II", 0.0)] {
                let r = ui_art::choice(ui, art, label, (scene.speed - s).abs() < 0.01);
                ui_common::sfx(engine, ui, &r);
                if ui_art::tip(r, art, "Spielgeschwindigkeit", "+ / − ändern, ^ pausiert").clicked() {
                    scene.speed = s;
                }
            }
            if scene.world.director.enabled && scene.world.mission.outcome.is_none() {
                ui.add_space(14.0);
                let t = scene.world.director.next_wave_in.max(0.0);
                let urgent = t < 30.0;
                let (r, _) = ui.allocate_exact_size(Vec2::new(150.0, TOP_H - 8.0), Sense::hover());
                let p = ui.painter();
                if urgent {
                    let a = 0.6 + 0.4 * (ui.input(|i| i.time) as f32 * 6.0).sin();
                    art.glow(p, r.expand2(Vec2::new(10.0, 4.0)), Color32::from_rgb((110.0 * a) as u8, (30.0 * a) as u8, 4));
                }
                let col = if urgent { theme::EMBER_HOT } else { theme::PARCHMENT };
                shadow_text(p, r.center() + Vec2::new(0.0, -8.0), Align2::CENTER_CENTER, "Nächste Welle", theme::caps_font(11.5), theme::PARCHMENT_DIM);
                shadow_text(p, r.center() + Vec2::new(0.0, 9.0), Align2::CENTER_CENTER, &format!("{}:{:02}", t as u32 / 60, t as u32 % 60), theme::heading_font(18.0), col);
                let resp = ui.interact(r, Id::unique("wave"), Sense::hover());
                ui_art::tip(resp, art, "Hohlen-Welle", "Zeit bis zum nächsten Angriff aus dem Nordosten.");
            }
        });
    });
}

// ---------------------------------------------------------------------------------------------
// Untere Konsole
// ---------------------------------------------------------------------------------------------

fn console(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let art = cx.ui;
    egui::Panel::bottom("console").exact_size(CONSOLE_H).frame(Frame::NONE).show(ui, |ui| {
        let rect = ui.max_rect();
        let painter = ui.painter().clone();
        let band_h = art.band_height(false);
        let body = Rect::from_min_max(Pos2::new(rect.min.x, rect.min.y + band_h - 8.0), rect.max);
        painter.add(art.fill_shape(body, false, Color32::WHITE));
        // Steinmulden für Minimap und Befehlskarte.
        let well_l = Rect::from_min_max(Pos2::new(rect.min.x, body.min.y), Pos2::new(rect.min.x + MINIMAP_W, rect.max.y));
        let well_r = Rect::from_min_max(Pos2::new(rect.max.x - CARD_W, body.min.y), rect.max);
        painter.add(art.fill_shape(well_l, true, Color32::from_gray(200)));
        painter.add(art.fill_shape(well_r, true, Color32::from_gray(200)));
        let fg = ui.ctx().layer_painter(egui::LayerId::new(Order::Background, Id::unique("consoleband")));
        art.band(&fg, rect.min.x, rect.max.x, rect.min.y - 4.0, false);
        art.pillar(&painter, well_l.max.x, body.min.y, rect.max.y);
        art.pillar(&painter, well_r.min.x, body.min.y, rect.max.y);

        let inner_top = rect.min.y + band_h;
        // Minimap in Rautenfassung.
        let map_rect = Rect::from_center_size(Pos2::new(well_l.center().x, (inner_top + rect.max.y) / 2.0 + 2.0), Vec2::new(232.0, 116.0)).round_ui();
        minimap(scene, ui, engine, map_rect);
        art.minimap_frame(&painter, map_rect);
        if let Some(s) = &scene.status {
            shadow_text(&painter, Pos2::new(well_l.center().x, rect.max.y - 10.0), Align2::CENTER_BOTTOM, s, theme::italic_font(13.0), theme::PARCHMENT_DIM);
        }

        // Auswahl in eingelassenem Rahmen.
        let sel_frame = Rect::from_min_max(Pos2::new(well_l.max.x + 14.0, inner_top - 2.0), Pos2::new(well_r.min.x - 14.0, rect.max.y - 8.0));
        for s in art.panel_shapes(PanelKind::Thin, sel_frame, false) {
            painter.add(s);
        }
        let sel = sel_frame.shrink2(Vec2::new(18.0, 12.0));
        ui.scope_builder(UiBuilder::new().max_rect(sel), |ui| selection_panel(scene, ui, engine, cx));

        // Befehlskarte.
        let card = Rect::from_min_size(Pos2::new(well_r.min.x + 20.0, inner_top + (rect.max.y - inner_top - (SLOT * 3.0 + SLOT_GAP * 2.0)) / 2.0), Vec2::new(SLOT * 4.0 + SLOT_GAP * 3.0, SLOT * 3.0 + SLOT_GAP * 2.0));
        ui.scope_builder(UiBuilder::new().max_rect(card), |ui| command_card(scene, ui, engine, cx, card));
    });
}

/// Minimap als isometrische Raute.
fn minimap(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &Engine, rect: Rect) {
    let map = &scene.world.map;
    let n = map.width;
    let (w, h) = ((n * 2) as usize, n as usize);
    scene.minimap_timer -= ui.ctx().input(|i| i.stable_dt);
    if scene.minimap.is_none() || scene.minimap_timer <= 0.0 {
        scene.minimap_timer = 0.33;
        let mut img = egui::ColorImage::filled([w, h], Color32::TRANSPARENT);
        for py in 0..h {
            for px in 0..w {
                let world = Vec2::new((px as f32 + 0.5) * 16.0 - n as f32 * 16.0, (py as f32 + 0.5) * 16.0);
                let t = world_to_tile(glam::Vec2::new(world.x, world.y));
                if t.x < 0.0 || t.y < 0.0 || t.x >= n as f32 || t.y >= n as f32 {
                    continue;
                }
                let terr = map.terrain_at(glam::IVec2::new(t.x as i32, t.y as i32));
                let mut c = match terr {
                    Terrain::Grass => [58, 74, 38],
                    Terrain::DarkGrass => [42, 58, 32],
                    Terrain::Dirt => [84, 64, 42],
                    Terrain::Road => [104, 96, 84],
                    Terrain::Stone => [88, 84, 78],
                    Terrain::Sand => [120, 104, 74],
                    Terrain::Ash => [56, 52, 56],
                    Terrain::Ford => [66, 96, 104],
                    Terrain::Water => [32, 60, 78],
                    Terrain::DeepWater => [20, 40, 56],
                };
                let fog = if scene.fog_enabled { scene.world.fog.state(t) } else { 2 };
                if fog == UNEXPLORED {
                    // Unerforschtes Land: dunkles, fleckiges Kartenleder.
                    let n = ashen_core::noise::fbm2(px as f32 * 0.08, py as f32 * 0.16, 17, 3);
                    let v = 22.0 + n * 18.0;
                    img[(px, py)] = Color32::from_rgb((v * 1.25) as u8, v as u8, (v * 0.8) as u8);
                    continue;
                }
                if fog == EXPLORED {
                    c = [c[0] / 2, c[1] / 2, c[2] / 2 + 4];
                }
                img[(px, py)] = Color32::from_rgb(c[0], c[1], c[2]);
            }
        }
        let mut dot = |t: glam::Vec2, col: Color32, r: i32| {
            let wp = tile_to_world(t);
            let px = ((wp.x + n as f32 * 16.0) / 16.0) as i32;
            let py = (wp.y / 16.0) as i32;
            for dy in -r / 2..=r / 2 {
                for dx in -r..=r {
                    let (x, y) = (px + dx, py + dy);
                    if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
                        img[(x as usize, y as usize)] = col;
                    }
                }
            }
        };
        for (_, nd) in scene.world.nodes.iter() {
            if nd.kind == NodeKind::Tree {
                continue;
            }
            if !scene.fog_enabled || scene.world.fog.explored(nd.center()) {
                dot(nd.center(), Color32::from_rgb(230, 196, 80), 1);
            }
        }
        for (_, b) in scene.world.buildings.iter() {
            if !scene.fog_enabled || scene.world.fog.explored(b.center()) {
                dot(b.center(), if b.owner == PLAYER { Color32::from_rgb(214, 64, 50) } else { Color32::from_rgb(120, 240, 140) }, 2);
            }
        }
        for (_, u) in scene.world.units.iter() {
            if u.owner == PLAYER || !scene.fog_enabled || scene.world.fog.visible(u.pos) {
                dot(u.pos, if u.owner == PLAYER { Color32::from_rgb(255, 130, 96) } else { Color32::from_rgb(150, 255, 170) }, 0);
            }
        }
        match &mut scene.minimap {
            Some(t) => t.set(img, egui::TextureOptions::NEAREST),
            None => scene.minimap = Some(ui.ctx().load_texture("minimap", img, egui::TextureOptions::NEAREST)),
        }
    }
    let size = rect.size();
    let resp = ui.interact(rect, Id::unique("minimap"), Sense::click_and_drag());
    let painter = ui.painter_at(rect.expand(2.0));
    if let Some(t) = &scene.minimap {
        painter.image(t.id(), rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
    }
    // Kameraausschnitt.
    let to_mini = |wp: glam::Vec2| Pos2::new(rect.min.x + (wp.x + n as f32 * 16.0) / (n as f32 * 32.0) * size.x, rect.min.y + wp.y / (n as f32 * 16.0) * size.y);
    if let Some(v) = engine.renderer.last_view {
        let (a, b) = v.visible_world(0.0);
        let r = Rect::from_two_pos(to_mini(a), to_mini(b)).intersect(rect);
        painter.rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 236, 200, 220)), StrokeKind::Inside);
        for c in [r.left_top(), r.right_top(), r.left_bottom(), r.right_bottom()] {
            painter.rect_filled(Rect::from_center_size(c, Vec2::splat(3.0)), 0.0, theme::GOLD_BRIGHT);
        }
    }
    if (resp.clicked() || resp.dragged())
        && let Some(p) = resp.interact_pointer_pos()
    {
        let fx = (p.x - rect.min.x) / size.x * n as f32 * 32.0 - n as f32 * 16.0;
        let fy = (p.y - rect.min.y) / size.y * n as f32 * 16.0;
        scene.camera.center = glam::Vec2::new(fx, fy);
    }
}

/// Porträt im Prunkrahmen mit dunklem Hintergrundschein.
fn portrait(ui: &mut Ui, cx: &HudCtx, sprite: SpriteRect, hostile: bool, hero: bool) -> Rect {
    let art = cx.ui;
    let size = 56.0 * ui_art::PX;
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let p = ui.painter();
    let inner = rect.shrink(6.0 * ui_art::PX);
    p.rect_filled(inner, 0.0, Color32::from_rgb(12, 9, 9));
    let bg = if hostile {
        Color32::from_rgb(20, 70, 36)
    } else if hero {
        Color32::from_rgb(110, 60, 14)
    } else {
        Color32::from_rgb(80, 24, 18)
    };
    art.glow(p, inner.expand(10.0).translate(Vec2::new(0.0, 8.0)), bg);
    ui_common::paint_sprite(p, cx.tex, sprite, inner.shrink(4.0), Color32::WHITE);
    art.portrait.paint(p, rect, Color32::WHITE);
    rect
}

fn selection_panel(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &HudCtx) {
    let data = cx.data;
    let art = cx.ui;
    if let Some(bid) = scene.selected_building
        && let Some(b) = scene.world.buildings.get(bid).cloned()
    {
        let def = data.building(b.kind);
        ui.horizontal(|ui| {
            portrait(ui, cx, sprite(cx, &crate::hud_city::building_icon(def)), b.owner != PLAYER, false);
            ui.add_space(14.0);
            ui.vertical(|ui| {
                ui.add_space(4.0);
                title_line(ui, art, &def.name, if b.owner == PLAYER { "Gebäude" } else { "Feindliches Bauwerk" });
                let w = (ui.available_width() - 10.0).clamp(160.0, 300.0);
                bar(ui, art, b.hp / b.max_hp, w, theme::LIFE, &format!("{:.0} / {:.0}", b.hp.max(0.0), b.max_hp));
                if !b.complete {
                    bar(ui, art, b.progress, w, Color32::from_rgb(200, 150, 50), &format!("Bau {:.0} %", b.progress * 100.0));
                }
                if b.owner == PLAYER && b.complete && (def.employs() || def.housing.is_some() || def.fort == Some(ashen_sim::data::Fort::Gate) || def.storage > 0.0 || def.market) {
                    crate::hud_city::building_info(scene, ui, engine, cx, bid, &b);
                } else {
                    ui.label(RichText::new(&def.description).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
                }
                if b.owner == PLAYER && !b.queue.is_empty() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Ausbildung").font(theme::caps_font(11.5)).color(theme::GOLD_DIM));
                        let mut cancel = None;
                        for (i, q) in b.queue.iter().enumerate() {
                            let u = data.unit(q.unit);
                            let (r, resp) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::click());
                            let st = BtnState::of(&resp, true);
                            let p = ui.painter();
                            art.image(p, art.slot_tex(if st == BtnState::Hover { 1 } else { 0 }, true), r, Color32::WHITE);
                            ui_common::paint_sprite(p, cx.tex, cx.art.portrait(&u.id), r.shrink(6.0), Color32::WHITE);
                            if i == 0 {
                                let pr = Rect::from_min_size(Pos2::new(r.min.x + 5.0, r.max.y - 8.0), Vec2::new((r.width() - 10.0) * q.progress, 3.0));
                                p.rect_filled(pr, 0.0, Color32::from_rgb(110, 160, 250));
                            }
                            if st == BtnState::Hover {
                                p.line_segment([r.left_top() + Vec2::splat(9.0), r.right_bottom() - Vec2::splat(9.0)], Stroke::new(2.0, theme::BLOOD_BRIGHT));
                                p.line_segment([r.right_top() + Vec2::new(-9.0, 9.0), r.left_bottom() + Vec2::new(9.0, -9.0)], Stroke::new(2.0, theme::BLOOD_BRIGHT));
                            }
                            ui_common::sfx(engine, ui, &resp);
                            if ui_art::tip(resp, art, &u.name, "Klick: Ausbildung abbrechen").clicked() {
                                cancel = Some(i);
                            }
                        }
                        if let Some(i) = cancel {
                            scene.cancel_train(bid, i);
                        }
                    });
                }
            });
        });
        return;
    }
    let units: Vec<_> = scene.selection.iter().filter_map(|id| scene.world.units.get(*id).map(|u| (*id, u.clone()))).collect();
    if units.is_empty() {
        let r = ui.max_rect();
        let p = ui.painter();
        art.divider(p, r.center().x - 140.0, r.center().x + 140.0, r.center().y - 30.0);
        shadow_text(p, r.center() + Vec2::new(0.0, -4.0), Align2::CENTER_CENTER, "Nichts ausgewählt", theme::heading_font(17.0), theme::PARCHMENT_DIM);
        shadow_text(p, r.center() + Vec2::new(0.0, 20.0), Align2::CENTER_CENTER, "Linksklick: auswählen · Ziehen: Rahmen · Rechtsklick: Befehl", theme::italic_font(14.0), Color32::from_rgb(140, 126, 104));
        return;
    }
    if units.len() == 1 {
        let (_, u) = &units[0];
        let def = data.unit(u.kind);
        let hero = def.class == UnitClass::Hero;
        ui.horizontal(|ui| {
            let prect = portrait(ui, cx, cx.art.portrait(&def.id), u.owner != PLAYER, hero);
            if hero && u.owner == PLAYER {
                // Klick auf das Heldenporträt öffnet das Heldenfenster.
                let resp = ui.interact(prect, Id::unique("hero_portrait"), Sense::click());
                ui_common::sfx(engine, ui, &resp);
                let points = ashen_sim::systems::heroes::free_points(data, u);
                if resp.clicked() {
                    scene.show_hero = !scene.show_hero;
                    scene.hero_focus = Some(units[0].0);
                }
                if points > 0 {
                    shadow_text(ui.painter(), prect.right_bottom() - Vec2::new(8.0, 8.0), Align2::CENTER_CENTER, "+", theme::body_bold_font(20.0), Color32::from_rgb(150, 236, 150));
                }
                ui_art::tip(resp, art, "Heldenfenster [H]", "Talente, Ausrüstung und Inventar.");
            }
            ui.add_space(14.0);
            ui.vertical(|ui| {
                ui.add_space(2.0);
                let hero_word = if def.id == "hero_isolde" { "Heldin" } else { "Held" };
                let sub = if hero { format!("{hero_word} · Stufe {}", u.level) } else if u.owner == PLAYER { format!("{} · Gefolgsmann", def.class.label()) } else { "Hohler Feind".to_string() };
                title_line(ui, art, &def.name, &sub);
                ui.add_space(2.0);
                let w = (ui.available_width() * 0.45).clamp(180.0, 300.0);
                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        bar(ui, art, u.hp / u.max_hp, w, theme::LIFE, &format!("{:.0} / {:.0}", u.hp.max(0.0), u.max_hp));
                        if u.max_mana > 0.0 {
                            bar(ui, art, u.mana / u.max_mana, w, theme::MANA, &format!("Mana {:.0} / {:.0}", u.mana, u.max_mana));
                        }
                        if hero {
                            let levels = &data.rules.xp_levels;
                            let (lo, hi) = (levels.get(u.level as usize - 1).copied().unwrap_or(0), levels.get(u.level as usize).copied().unwrap_or(u.xp.max(1)));
                            bar(ui, art, (u.xp - lo) as f32 / (hi - lo).max(1) as f32, w, Color32::from_rgb(200, 160, 60), &format!("Erfahrung {} / {}", u.xp, hi));
                        }
                    });
                    ui.add_space(18.0);
                    let dmg_bonus = (u.level.saturating_sub(1)) as f32 * def.damage_per_level;
                    let state = if u.routing > 0.0 {
                        "Flieht!".to_string()
                    } else if let Some((k, a)) = u.carry {
                        format!("Trägt {a} {}", k.label())
                    } else {
                        format!("{:?}", u.activity)
                    };
                    egui::Grid::new("unit_stats").num_columns(2).spacing([12.0, 0.0]).show(ui, |ui| {
                        let row = |ui: &mut Ui, k: &str, v: String, col: Color32| {
                            ui.label(RichText::new(k).font(theme::caps_font(11.5)).color(theme::GOLD_DIM));
                            ui.label(RichText::new(v).size(14.5).color(col));
                            ui.end_row();
                        };
                        let c = Color32::from_rgb(222, 206, 176);
                        let factor = scene.world.damage_factor(data, u);
                        let flat = u.mods.damage;
                        row(ui, "Schaden", format!("{:.0}–{:.0} · {:?}", (def.damage_min + dmg_bonus + flat) * factor, (def.damage_max + dmg_bonus + flat) * factor, def.damage_type), c);
                        row(ui, "Rüstung", format!("{:.0} · {:?}", scene.world.armor_of(data, u), def.armor_type), c);
                        let range = if def.min_range > 0.0 { format!("{:.1} (min. {:.1})", def.range, def.min_range) } else { format!("{:.1}", def.range) };
                        row(ui, "Reichweite", format!("{range} · Moral {:.0}", u.morale), c);
                        row(ui, "Zustand", state, if u.routing > 0.0 { theme::EMBER } else { c });
                    });
                });
            });
        });
    } else {
        // Links: Einheitenkarten, rechts: Übersicht nach Typ mit Gesamtleben.
        let full = ui.max_rect();
        let list_w = (full.width() * 0.6).max(260.0);
        let mut groups: Vec<(ashen_sim::data::UnitTypeId, usize, f32, f32)> = Vec::new();
        for (_, u) in &units {
            match groups.iter_mut().find(|g| g.0 == u.kind) {
                Some(g) => {
                    g.1 += 1;
                    g.2 += u.hp.max(0.0);
                    g.3 += u.max_hp;
                }
                None => groups.push((u.kind, 1, u.hp.max(0.0), u.max_hp)),
            }
        }
        let list_rect = Rect::from_min_size(full.min, Vec2::new(list_w, full.height()));
        ui.scope_builder(UiBuilder::new().max_rect(list_rect), |ui| {
            ui.label(RichText::new(format!("{} Einheiten", units.len())).font(theme::heading_font(15.0)).color(theme::GOLD));
            egui::ScrollArea::vertical().max_height(ui.available_height()).show(ui, |ui| {
                ui.spacing_mut().item_spacing = Vec2::splat(3.0);
                ui.horizontal_wrapped(|ui| {
                    let mut pick = None;
                    for (id, u) in &units {
                        let def = data.unit(u.kind);
                        let (r, resp) = ui.allocate_exact_size(Vec2::splat(SLOT), Sense::click());
                        let st = BtnState::of(&resp, true);
                        let p = ui.painter();
                        if st == BtnState::Hover {
                            art.glow(p, r.expand(10.0), Color32::from_rgb(80, 30, 6));
                        }
                        art.image(p, art.slot_tex(if st == BtnState::Hover { 1 } else if st == BtnState::Pressed { 2 } else { 0 }, false), r, Color32::WHITE);
                        ui_common::paint_sprite(p, cx.tex, cx.art.portrait(&def.id), Rect::from_center_size(r.center() + Vec2::new(0.0, -2.0), Vec2::splat(28.0)), Color32::WHITE);
                        let frac = (u.hp / u.max_hp).clamp(0.0, 1.0);
                        let hb = Rect::from_min_size(Pos2::new(r.min.x + 9.0, r.max.y - 12.0), Vec2::new(r.width() - 18.0, 3.0));
                        p.rect_filled(hb, 0.0, Color32::from_rgb(20, 8, 6));
                        p.rect_filled(Rect::from_min_size(hb.min, Vec2::new(hb.width() * frac, 3.0)), 0.0, hp_color(frac));
                        ui_common::sfx(engine, ui, &resp);
                        if ui_art::tip(resp, art, &def.name, &format!("{:.0} / {:.0} Leben · Klick: nur diese Einheit", u.hp.max(0.0), u.max_hp)).clicked() {
                            pick = Some(*id);
                        }
                    }
                    if let Some(p) = pick {
                        scene.selection = vec![p];
                    }
                });
            });
        });
        let sum_rect = Rect::from_min_max(Pos2::new(full.min.x + list_w + 24.0, full.min.y), full.max);
        if sum_rect.width() > 160.0 {
            let p = ui.painter();
            p.line_segment([Pos2::new(sum_rect.min.x - 12.0, sum_rect.min.y + 4.0), Pos2::new(sum_rect.min.x - 12.0, sum_rect.max.y - 4.0)], Stroke::new(1.0, theme::GOLD_DIM.gamma_multiply(0.5)));
            let mut y = sum_rect.min.y + 2.0;
            for (kind, n, hp, max) in groups.iter().take(5) {
                let def = data.unit(*kind);
                let ir = Rect::from_min_size(Pos2::new(sum_rect.min.x, y), Vec2::splat(28.0));
                ui_common::paint_sprite(p, cx.tex, cx.art.portrait(&def.id), ir, Color32::WHITE);
                shadow_text(p, Pos2::new(ir.max.x + 8.0, y + 7.0), Align2::LEFT_CENTER, &format!("{}  ×{n}", def.name), theme::body_bold_font(14.5), theme::PARCHMENT);
                let bw = (sum_rect.width() - 46.0).min(220.0);
                let br = Rect::from_min_size(Pos2::new(ir.max.x + 8.0, y + 17.0), Vec2::new(bw, 6.0));
                p.rect_filled(br, 1.0, Color32::from_rgb(16, 8, 6));
                let f = (hp / max.max(1.0)).clamp(0.0, 1.0);
                p.rect_filled(Rect::from_min_size(br.min, Vec2::new(bw * f, 6.0)), 1.0, hp_color(f));
                p.rect_stroke(br, 1.0, Stroke::new(1.0, Color32::from_rgb(90, 66, 36)), StrokeKind::Outside);
                y += 32.0;
            }
        }
    }
}

fn hp_color(frac: f32) -> Color32 {
    if frac > 0.6 {
        theme::LIFE
    } else if frac > 0.3 {
        Color32::from_rgb(210, 170, 50)
    } else {
        theme::BLOOD_BRIGHT
    }
}

fn title_line(ui: &mut Ui, art: &UiArt, name: &str, sub: &str) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width().min(420.0), 26.0), Sense::hover());
    let p = ui.painter();
    let nr = ui_art::fx_text(art, p, r.left_center(), Align2::LEFT_CENTER, name, theme::heading_font(20.0), &TextFx { glow: None, shadow: 2.0, ..TextFx::gold() });
    p.text(Pos2::new(nr.max.x + 12.0, r.center().y + 2.0), Align2::LEFT_CENTER, sub, theme::italic_font(14.0), theme::PARCHMENT_DIM);
}

fn bar(ui: &mut Ui, art: &UiArt, frac: f32, width: f32, color: Color32, text: &str) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 18.0), Sense::hover());
    art.bar(ui.painter(), rect, frac, color, text);
}

// ---------------------------------------------------------------------------------------------
// Befehlskarte
// ---------------------------------------------------------------------------------------------

enum CmdAction {
    Train(BuildingId, UnitTypeId),
    Rally,
    Demolish(BuildingId),
    BackToNormal,
    Move,
    AttackMove,
    Stop,
    Hold,
    Patrol,
    BuildMenu,
    Ability(u8),
    AdvanceEra,
    Research(BuildingId, usize),
    CancelResearch(BuildingId),
    Revive(BuildingId, UnitTypeId),
    CycleFormation,
    HeroWindow,
}

struct Cmd {
    icon: String,
    hotkey: String,
    enabled: bool,
    title: String,
    body: String,
    cost: Option<Cost>,
    goods: ashen_sim::data::ItemAmounts,
    extra: String,
    cooldown: Option<(f32, f32)>,
    action: CmdAction,
}

impl Cmd {
    fn new(icon: impl Into<String>, hotkey: impl Into<String>, title: impl Into<String>, body: impl Into<String>, action: CmdAction) -> Cmd {
        Cmd { icon: icon.into(), hotkey: hotkey.into(), enabled: true, title: title.into(), body: body.into(), cost: None, goods: Vec::new(), extra: String::new(), cooldown: None, action }
    }
}

fn commands(scene: &MissionScene, cx: &HudCtx) -> Vec<Cmd> {
    let data = cx.data;
    let mut v = Vec::new();
    if let Some(bid) = scene.selected_building
        && let Some(b) = scene.world.buildings.get(bid)
    {
        if b.owner != PLAYER {
            return v;
        }
        let def = data.building(b.kind);
        if b.complete {
            for t in &def.trains {
                let Some(ut) = data.unit_id(t) else { continue };
                let u = data.unit(ut);
                let hk = u.hotkey.map(|c| c.to_string()).unwrap_or_default();
                let portrait = format!("{}_portrait", if cx.art.atlas.try_get(&format!("{}_portrait", u.id)).is_some() { u.id.as_str() } else { "swordsman" });
                let mut c = Cmd::new(portrait, hk, format!("{} ausbilden", u.name), u.description.clone(), CmdAction::Train(bid, ut));
                c.enabled = scene.world.afford(PLAYER, &u.cost, &u.goods_cost_r).is_ok();
                c.cost = Some(u.cost);
                c.goods = u.goods_cost_r.clone();
                c.extra = format!("Ausbildung {:.0} s · Bevölkerung {}", u.train_time, u.pop);
                v.push(c);
            }
            if !def.trains.is_empty() {
                v.push(Cmd::new("icon_rally", "R", "Sammelpunkt", "Neue Einheiten sammeln sich hier (R, oder Rechtsklick).", CmdAction::Rally));
            }
            // Schmiede-Upgrades (Stufen 1–3) des Gebäudes.
            for (ui_idx, up) in data.upgrades.iter().enumerate().filter(|(_, up)| up.building == def.id) {
                let ps = &scene.world.players[PLAYER as usize];
                let lvl = ps.upgrades.get(ui_idx).copied().unwrap_or(0) as usize;
                let icon = format!("icon_up_{}", up.id);
                let running = b.research.filter(|(i, _)| *i as usize == ui_idx);
                if let Some((_, prog)) = running {
                    let need = up.levels.get(lvl).map(|l| l.time).unwrap_or(1.0).max(0.1);
                    let mut c = Cmd::new(icon, "", format!("{} (Stufe {}) wird erforscht", up.name, lvl + 1), "Klick: Forschung abbrechen (volle Erstattung).", CmdAction::CancelResearch(bid));
                    c.cooldown = Some((1.0 - prog / need, need - prog));
                    v.push(c);
                } else if let Some(next) = up.levels.get(lvl) {
                    let mut c = Cmd::new(icon, "", format!("{} (Stufe {})", up.name, lvl + 1), up.description.clone(), CmdAction::Research(bid, ui_idx));
                    c.cost = Some(next.cost);
                    c.goods = next.goods_r.clone();
                    let era_ok = next.era <= ps.era;
                    c.enabled = era_ok && b.research.is_none() && scene.world.afford(PLAYER, &next.cost, &next.goods_r).is_ok();
                    let era_name = data.rules.eras.get((next.era as usize).saturating_sub(1)).map(|e| e.name.as_str()).unwrap_or("");
                    c.extra = format!("Forschung {:.0} s · Stufe {lvl}/{} erreicht{}", next.time, up.levels.len(), if era_ok { String::new() } else { format!(" · benötigt Ausbaustufe {era_name}") });
                    v.push(c);
                } else {
                    let mut c = Cmd::new(icon, "", format!("{} (Stufe {} – Maximum)", up.name, lvl), up.description.clone(), CmdAction::Move);
                    c.enabled = false;
                    v.push(c);
                }
            }
            // Gefallene Helden gegen Gold wiederbeleben (Bergfried oder Altar).
            if b.tag.as_deref() == Some(scene.world.mission_def.defeat_tag.as_str()) || def.id == "altar" {
                for f in &scene.world.fallen {
                    let hd = data.unit(f.unit.kind);
                    let cost = ashen_sim::systems::heroes::revive_cost(data, f.unit.level);
                    let at_altar = def.id == "altar";
                    let mut c = Cmd::new(format!("{}_portrait", hd.id), "", format!("{} wiederbeleben", hd.name), "Der Held kehrt mit Stufe, Talenten und Ausrüstung zurück.", CmdAction::Revive(bid, f.unit.kind));
                    c.cost = Some(Cost { gold: cost, ..Default::default() });
                    c.enabled = scene.world.players[PLAYER as usize].resources.gold >= cost;
                    c.extra = format!("{} % Leben nach der Rückkehr{}", (if at_altar { data.rules.hero_altar_hp } else { data.rules.hero_keep_hp } * 100.0).round(), if at_altar { String::new() } else { " · Altar: mehr Leben".into() });
                    v.push(c);
                }
            }
            if b.tag.as_deref() == Some(scene.world.mission_def.defeat_tag.as_str())
                && let Some(next) = data.rules.eras.get(scene.world.players[0].era as usize)
            {
                let mut c = Cmd::new("icon_castle", "", format!("Zur {} ausbauen", next.name), next.description.clone(), CmdAction::AdvanceEra);
                c.cost = Some(next.cost);
                c.goods = next.goods_r.clone();
                c.enabled = scene.world.players[0].era_progress.is_none();
                c.extra = next.requires.iter().map(|(b, n)| format!("{n}× {}", data.building_id(b).map(|id| data.building(id).name.as_str()).unwrap_or(b))).collect::<Vec<_>>().join(", ");
                v.push(c);
            }
        }
        if b.tag.as_deref() != Some(scene.world.mission_def.defeat_tag.as_str()) {
            v.push(Cmd::new("icon_demolish", "", "Abreißen", "Gebäude abreißen (Teilerstattung der Kosten).", CmdAction::Demolish(bid)));
        }
        return v;
    }
    if scene.selection.iter().all(|id| scene.world.units.get(*id).is_none_or(|u| u.owner != PLAYER)) {
        return v;
    }
    if scene.mode == CommandMode::BuildMenu {
        v.push(Cmd::new("icon_cancel", "Esc", "Zurück", "Baumenü schließen.", CmdAction::BackToNormal));
        return v;
    }
    v.push(Cmd::new("icon_move", "", "Bewegen", "Rechtsklick auf das Ziel.", CmdAction::Move));
    v.push(Cmd::new("icon_attack", "A", "Angriff", "Angriffsbewegung: A, dann Linksklick auf Gegner oder Boden.", CmdAction::AttackMove));
    v.push(Cmd::new("icon_stop", "S", "Halt", "Alle Befehle abbrechen (S).", CmdAction::Stop));
    v.push(Cmd::new("icon_hold", "H", "Position halten", "Stellung halten, nicht verfolgen (H).", CmdAction::Hold));
    v.push(Cmd::new("icon_patrol", "P", "Patrouille", "P, dann Linksklick auf das Patrouillenziel.", CmdAction::Patrol));
    if scene.has_workers(cx.data) {
        v.push(Cmd::new("icon_build", "B", "Bauen", "Öffnet das Baumenü mit Rubriken: Wohnen, Rohstoffe, Handwerk, Nahrung, Stadt, Militär, Mauern.", CmdAction::BuildMenu));
    }
    // Formation: ab zwei Kampfeinheiten.
    let fighters: Vec<&ashen_sim::entity::Unit> = scene
        .selection
        .iter()
        .filter_map(|id| scene.world.units.get(*id))
        .filter(|u| u.owner == PLAYER && !matches!(data.unit(u.kind).class, UnitClass::Worker))
        .collect();
    if fighters.len() >= 2 {
        use ashen_sim::rpg::Formation;
        let cur = fighters[0].formation;
        let next = Formation::ALL[(Formation::ALL.iter().position(|f| *f == cur).unwrap_or(0) + 1) % Formation::ALL.len()];
        let name = match cur {
            Formation::Block => "block",
            Formation::Line => "line",
            Formation::Wedge => "wedge",
            Formation::Column => "column",
            Formation::Square => "square",
        };
        let mut body = String::from("Aufstellung der Gruppe bei Bewegungsbefehlen; wirkt auf Kampfwerte.");
        for f in Formation::ALL.iter().skip(1) {
            body.push_str(&format!("\n{}: {}", f.label(), f.description()));
        }
        let mut c = Cmd::new(format!("icon_formation_{name}"), "F", format!("Formation: {}", cur.label()), body, CmdAction::CycleFormation);
        c.extra = format!("Klick: wechselt zu {}", next.label());
        v.push(c);
    }
    if scene.selected_hero(data).is_some() {
        v.push(Cmd::new("icon_hero", "H", "Heldenfenster", "Talente, Ausrüstung und Inventar (H, oder Klick auf das Porträt).", CmdAction::HeroWindow));
    }
    // Fähigkeiten des führenden Einheitentyps (Held bevorzugt), inklusive Talentfähigkeiten.
    if let Some(lead) = scene.ability_lead(data)
        && let Some(hero) = scene.world.units.get(lead)
    {
        let names: Vec<&String> = ashen_sim::systems::heroes::ability_list(data, hero).collect();
        let start = if names.len() <= 4 { v.len().max(8) } else { v.len() };
        while v.len() < start {
            let mut blank = Cmd::new("", "", "", "", CmdAction::Move);
            blank.enabled = false;
            v.push(blank);
        }
        for (i, a) in names.iter().enumerate().take(12usize.saturating_sub(start)) {
            let Some(aid) = data.ability_id(a) else { continue };
            let ad = data.ability(aid);
            let cd = hero.ability_cd.get(i).copied().unwrap_or(0.0);
            let hk = ["Q", "W", "E", "R", "T", "Y"].get(i).copied().unwrap_or("");
            let mut c = Cmd::new(format!("icon_{}", ad.id), hk, ad.name.clone(), ad.description.clone(), CmdAction::Ability(i as u8));
            c.enabled = cd <= 0.0 && hero.mana >= ad.mana;
            let target = if ad.targeting == ashen_sim::data::AbilityTargeting::Point { format!(" · Zielzauber, Reichweite {:.1}", ad.range) } else { String::new() };
            c.extra = format!("Mana {:.0} · Abklingzeit {:.0} s{target}{}", ad.mana, ad.cooldown, if cd > 0.0 { format!(" · Bereit in {cd:.0} s") } else if hero.mana < ad.mana { " · Zu wenig Mana".into() } else { String::new() });
            if cd > 0.0 {
                c.cooldown = Some((cd / ad.cooldown.max(0.01), cd));
            }
            v.push(c);
        }
    }
    v
}

fn command_card(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &HudCtx, card: Rect) {
    let art = cx.ui;
    let cmds = commands(scene, cx);
    let mut clicked = None;
    for i in 0..12 {
        let (col, row) = (i % 4, i / 4);
        let rect = Rect::from_min_size(card.min + Vec2::new(col as f32 * (SLOT + SLOT_GAP), row as f32 * (SLOT + SLOT_GAP)), Vec2::splat(SLOT)).round_ui();
        let Some(c) = cmds.get(i).filter(|c| !c.icon.is_empty()) else {
            art.image(ui.painter(), art.slot_tex(4, false), rect, Color32::WHITE);
            continue;
        };
        let resp = ui.interact(rect, Id::unique("cmd").with(i), if c.enabled { Sense::click() } else { Sense::hover() });
        let st = BtnState::of(&resp, c.enabled);
        let p = ui.painter();
        let tex_state = match st {
            BtnState::Normal => 0,
            BtnState::Hover => 1,
            BtnState::Pressed => 2,
            BtnState::Disabled => 3,
        };
        if st == BtnState::Hover {
            art.glow(p, rect.expand(14.0), Color32::from_rgb(90, 34, 6));
        }
        art.image(p, art.slot_tex(tex_state, false), rect, Color32::WHITE);
        let spr = sprite(cx, &c.icon);
        let big = spr.size.x > 20.0;
        let icon_size = if big { 36.0 } else if spr.size.x <= 14.5 { 28.0 } else { 32.0 };
        let off = if st == BtnState::Pressed { Vec2::new(1.0, 1.0) } else { Vec2::ZERO };
        let ir = Rect::from_center_size(rect.center() + off, Vec2::splat(icon_size));
        let tint = if c.enabled || c.cooldown.is_some() { Color32::WHITE } else { Color32::from_gray(90) };
        ui_common::paint_sprite(p, cx.tex, spr, ir, tint);
        if let Some((frac, secs)) = c.cooldown {
            ui_art::cooldown_overlay(p, rect.shrink(8.0), frac, secs);
        }
        if !c.hotkey.is_empty() {
            art.rune(p, rect.right_top() + Vec2::new(-5.0, 5.0), &c.hotkey, c.enabled);
        }
        ui_common::sfx(engine, ui, &resp);
        let resp = ui_art::tooltip(resp, art, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(&c.title).font(theme::heading_font(16.0)).color(theme::GOLD));
                if !c.hotkey.is_empty() {
                    ui.label(RichText::new(format!("[{}]", c.hotkey)).font(theme::caps_font(13.0)).color(theme::EMBER_HOT));
                }
            });
            if !c.body.is_empty() {
                ui.label(RichText::new(&c.body).size(14.5).color(theme::PARCHMENT));
            }
            if let Some(cost) = &c.cost {
                crate::hud_city::cost_row_full(ui, cx, &scene.world, cost, &c.goods);
            }
            if !c.extra.is_empty() {
                ui.label(RichText::new(&c.extra).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
            }
        });
        if resp.clicked() && c.enabled {
            clicked = Some(i);
        }
    }
    let Some(i) = clicked else { return };
    match &cmds[i].action {
        CmdAction::Train(b, u) => scene.train(*b, *u),
        CmdAction::Rally => scene.mode = CommandMode::Rally,
        CmdAction::Demolish(b) => scene.demolish(*b),
        CmdAction::BackToNormal => scene.mode = CommandMode::Normal,
        CmdAction::Move => scene.status = Some("Rechtsklick auf das Ziel".into()),
        CmdAction::AttackMove => scene.mode = CommandMode::AttackMove,
        CmdAction::Stop => scene.command_stop(),
        CmdAction::Hold => scene.command_hold(),
        CmdAction::Patrol => scene.mode = CommandMode::Patrol,
        CmdAction::BuildMenu => scene.mode = CommandMode::BuildMenu,
        CmdAction::Ability(a) => scene.use_ability(cx.data, *a),
        CmdAction::AdvanceEra => scene.advance_era(),
        CmdAction::Research(b, i) => scene.research(*b, *i),
        CmdAction::CancelResearch(b) => scene.cancel_research(*b),
        CmdAction::Revive(b, k) => scene.revive_hero(*b, *k),
        CmdAction::CycleFormation => scene.cycle_formation(),
        CmdAction::HeroWindow => scene.show_hero = !scene.show_hero,
    }
}

// ---------------------------------------------------------------------------------------------
// Ziele, Nachrichten, Hinweise
// ---------------------------------------------------------------------------------------------

fn objectives(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &HudCtx) {
    if scene.world.mission.objectives.is_empty() {
        return;
    }
    let art = cx.ui;
    egui::Area::new(Id::unique("objectives")).anchor(Align2::RIGHT_TOP, Vec2::new(-14.0, TOP_H + 26.0)).show(ui.ctx(), |ui| {
        art.scroll(ui, egui::Margin { left: 26, right: 26, top: 30, bottom: 30 }, |ui| {
            ui.set_width(290.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Ziele").font(theme::title_bold_font(19.0)).color(theme::INK_RED));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (r, resp) = ui.allocate_exact_size(Vec2::splat(22.0), Sense::click());
                    art.rune(ui.painter(), r.center(), if scene.show_objectives { "–" } else { "+" }, resp.hovered());
                    ui_common::sfx(engine, ui, &resp);
                    if ui_art::tip(resp, art, if scene.show_objectives { "Einklappen" } else { "Ausklappen" }, "").clicked() {
                        scene.show_objectives = !scene.show_objectives;
                    }
                });
            });
            if !scene.show_objectives {
                return;
            }
            let r = ui.available_rect_before_wrap();
            ui.painter().line_segment([Pos2::new(r.min.x, r.min.y), Pos2::new(r.max.x, r.min.y)], Stroke::new(1.0, theme::INK_FADED.gamma_multiply(0.6)));
            ui.add_space(6.0);
            for (id, state) in &scene.world.mission.objectives {
                let Some(def) = scene.world.mission_def.objective(id) else { continue };
                let done = *state == ObjectiveState::Completed;
                ui.horizontal_top(|ui| {
                    let (mr, _) = ui.allocate_exact_size(Vec2::new(16.0, 18.0), Sense::hover());
                    let p = ui.painter();
                    let c = mr.center();
                    if done {
                        p.line_segment([c + Vec2::new(-5.0, 0.0), c + Vec2::new(-1.5, 4.0)], Stroke::new(2.2, Color32::from_rgb(46, 92, 34)));
                        p.line_segment([c + Vec2::new(-1.5, 4.0), c + Vec2::new(6.0, -6.0)], Stroke::new(2.2, Color32::from_rgb(46, 92, 34)));
                    } else {
                        let col = if def.optional { theme::INK_FADED } else { theme::INK_RED };
                        p.add(Shape::convex_polygon(vec![c + Vec2::new(0.0, -5.0), c + Vec2::new(4.0, 0.0), c + Vec2::new(0.0, 5.0), c + Vec2::new(-4.0, 0.0)], col, Stroke::NONE));
                    }
                    let text = if def.optional { format!("{} (optional)", def.text) } else { def.text.clone() };
                    let t = RichText::new(text).size(15.0);
                    ui.add(egui::Label::new(if done { t.strikethrough().color(theme::INK_FADED) } else if def.optional { t.font(theme::italic_font(15.0)).color(theme::INK_FADED) } else { t.color(theme::INK) }).wrap());
                });
                ui.add_space(2.0);
            }
        });
    });
}

fn messages(scene: &mut MissionScene, ui: &mut egui::Ui, cx: &HudCtx) {
    let art = cx.ui;
    if let Some((t, left)) = &scene.toast {
        let a = (left / 0.4).clamp(0.0, 1.0);
        egui::Area::new(Id::unique("toast")).anchor(Align2::CENTER_TOP, Vec2::new(0.0, TOP_H + 34.0 + if scene.world.mission_def.free_economy { 62.0 } else { 0.0 })).interactable(false).show(ui.ctx(), |ui| {
            ui.multiply_opacity(a);
            let (rect, _) = {
                let g = ui.painter().layout_no_wrap(t.clone(), theme::heading_font(18.0), theme::GOLD);
                ui.allocate_exact_size(Vec2::new(g.size().x + 120.0, 46.0), Sense::hover())
            };
            let p = ui.painter();
            art.glow(p, rect.expand2(Vec2::new(30.0, 22.0)), Color32::from_rgb(110, 38, 6));
            for s in art.panel_shapes(PanelKind::Thin, rect, true) {
                p.add(s);
            }
            let pulse = 0.5 + 0.5 * (ui.input(|i| i.time) as f32 * 4.0).sin();
            art.glow(p, Rect::from_center_size(rect.center(), Vec2::new(rect.width() * 0.8, 30.0)), Color32::from_rgb((40.0 + 30.0 * pulse) as u8, (14.0 + 8.0 * pulse) as u8, 2));
            for sx in [-1.0, 1.0] {
                let d = Pos2::new(rect.center().x + sx * (rect.width() / 2.0 - 30.0), rect.center().y);
                p.add(Shape::convex_polygon(vec![d + Vec2::new(0.0, -5.0), d + Vec2::new(5.0, 0.0), d + Vec2::new(0.0, 5.0), d + Vec2::new(-5.0, 0.0)], theme::EMBER, Stroke::new(1.0, Color32::BLACK)));
            }
            shadow_text(p, rect.center() + Vec2::new(0.0, 1.0), Align2::CENTER_CENTER, t, theme::heading_font(18.0), theme::GOLD_BRIGHT);
        });
    }
    if scene.messages.is_empty() {
        return;
    }
    egui::Area::new(Id::unique("messages")).anchor(Align2::LEFT_BOTTOM, Vec2::new(14.0, -(CONSOLE_H + 14.0))).interactable(false).show(ui.ctx(), |ui| {
        ui.set_max_width(540.0);
        for m in scene.messages.iter() {
            let fade = 1.0 - ((m.age - 11.0) / 3.0).clamp(0.0, 1.0);
            let appear = (m.age / 0.35).clamp(0.0, 1.0);
            ui.scope(|ui| {
                ui.multiply_opacity(fade * appear);
                art.panel(ui, PanelKind::Parchment, egui::Margin { left: 24, right: 24, top: 18, bottom: 18 }, |ui| {
                    ui.set_width(480.0);
                    if !m.speaker.is_empty() {
                        ui.label(RichText::new(&m.speaker).font(theme::title_bold_font(16.0)).color(theme::INK_RED));
                    }
                    ui.label(RichText::new(&m.text).size(16.0).color(theme::INK));
                });
            });
            ui.add_space(4.0);
        }
    });
}

fn mode_hint(scene: &MissionScene, ui: &mut egui::Ui, cx: &HudCtx) {
    let text = match scene.mode {
        CommandMode::Normal => return,
        CommandMode::Place(bt) if cx.data.building(bt).fort == Some(ashen_sim::data::Fort::Wall) => "Mauer ziehen: Maustaste gedrückt halten, Ziel wählen, loslassen · Shift: weiter bauen · Rechtsklick/Esc: abbrechen",
        CommandMode::Place(_) => "Bauplatz wählen: Linksklick platzieren · Shift: mehrere · Rechtsklick/Esc: abbrechen",
        CommandMode::AttackMove => "Angriffsziel: Linksklick auf Gegner oder Boden",
        CommandMode::Patrol => "Patrouillenziel: Linksklick",
        CommandMode::Rally => "Sammelpunkt: Linksklick",
        CommandMode::BuildMenu => "Bauen: Rubrik und Gebäude im Baumenü wählen · Esc zurück",
        CommandMode::Cast(_) => "Zielzauber: Linksklick auf das Ziel · Shift: mehrfach · Rechtsklick/Esc: abbrechen",
    };
    egui::Area::new(Id::unique("modehint")).anchor(Align2::CENTER_BOTTOM, Vec2::new(0.0, -(CONSOLE_H + 16.0))).interactable(false).show(ui.ctx(), |ui| {
        cx.ui.panel(ui, PanelKind::Thin, egui::Margin::symmetric(22, 14), |ui| {
            ui.label(RichText::new(text).size(15.0).color(theme::PARCHMENT));
        });
    });
}

// ---------------------------------------------------------------------------------------------
// Pausenmenü, Chronik, Sieg/Niederlage
// ---------------------------------------------------------------------------------------------

fn dim(ctx: &egui::Context, art: &UiArt, tint: Color32, alpha: u8) -> egui::Painter {
    let painter = ctx.layer_painter(egui::LayerId::new(Order::Middle, Id::unique("dim")));
    let r = ctx.content_rect();
    painter.rect_filled(r, CornerRadius::ZERO, Color32::from_black_alpha(alpha));
    // Vignette aus vier weichen Schatten.
    for (c, s) in [(r.left_center(), Vec2::new(r.width() * 0.5, r.height() * 1.6)), (r.right_center(), Vec2::new(r.width() * 0.5, r.height() * 1.6)), (r.center_top(), Vec2::new(r.width() * 1.6, r.height() * 0.5)), (r.center_bottom(), Vec2::new(r.width() * 1.6, r.height() * 0.5))] {
        art.soft_shadow(&painter, Rect::from_center_size(c, s), 0.55);
    }
    if tint != Color32::TRANSPARENT {
        art.glow(&painter, Rect::from_center_size(r.center(), r.size() * 1.1), tint);
    }
    painter
}

fn pause_menu(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let ctx = ui.ctx().clone();
    let art = cx.ui;
    dim(&ctx, art, Color32::TRANSPARENT, 150);
    egui::Area::new(Id::unique("pause")).order(Order::Foreground).anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(&ctx, |ui| {
        art.panel(ui, PanelKind::Ornate, egui::Margin::symmetric(40, 34), |ui| {
            ui.set_min_width(360.0);
            match scene.menu_page {
                MenuPage::Main => {
                    ui_art::heading(ui, art, "Pause", 30.0);
                    ui.vertical_centered(|ui| {
                        for (label, act) in [("Weiterspielen", 0), ("Speichern", 1), ("Laden", 2), ("Chronik", 3), ("Einstellungen", 4), ("Mission neu starten", 5), ("Hauptmenü", 6), ("Spiel beenden", 7)] {
                            let r = ui_art::menu_item(ui, art, label, 300.0, true, false);
                            ui_common::sfx(engine, ui, &r);
                            if r.clicked() {
                                match act {
                                    0 => scene.menu_open = false,
                                    1 => scene.menu_page = MenuPage::Save,
                                    2 => scene.menu_page = MenuPage::Load,
                                    3 => scene.menu_page = MenuPage::Chronicle,
                                    4 => scene.menu_page = MenuPage::Settings,
                                    5 => scene.action = SceneAction::Restart,
                                    6 => scene.action = SceneAction::MainMenu,
                                    _ => scene.action = SceneAction::Quit,
                                }
                            }
                        }
                    });
                }
                MenuPage::Save => {
                    ui_art::heading(ui, art, "Spiel speichern", 24.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Name").font(theme::caps_font(14.0)).color(theme::GOLD));
                        ui.add(egui::TextEdit::singleline(&mut scene.save_name).desired_width(260.0).font(theme::body_font(16.0)));
                    });
                    if let Some(s) = &scene.status {
                        ui.label(RichText::new(s).font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
                    }
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui_common::button(ui, engine, art, "Speichern", 160.0) {
                            let name = scene.save_name.clone();
                            scene.save(cx.data, &name);
                            scene.menu_page = MenuPage::Main;
                        }
                        if ui_common::button(ui, engine, art, "Zurück", 160.0) {
                            scene.menu_page = MenuPage::Main;
                        }
                    });
                }
                MenuPage::Load => {
                    ui_art::heading(ui, art, "Spiel laden", 24.0);
                    ui.set_width(520.0);
                    if let Some(p) = ui_common::save_list(ui, engine, art, &saves_dir()) {
                        scene.action = SceneAction::Load(p);
                    }
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        if ui_common::button(ui, engine, art, "Zurück", 180.0) {
                            scene.menu_page = MenuPage::Main;
                        }
                    });
                }
                MenuPage::Settings => {
                    ui_art::heading(ui, art, "Einstellungen", 24.0);
                    ui_common::settings_ui(ui, engine, cx.settings, art);
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        if ui_common::button(ui, engine, art, "Zurück", 180.0) {
                            cx.settings.save();
                            scene.menu_page = MenuPage::Main;
                        }
                    });
                }
                MenuPage::Chronicle => {
                    let title = if scene.world.mission_def.chronicle_title.is_empty() { format!("Chronik – {}", scene.world.mission_def.name) } else { scene.world.mission_def.chronicle_title.clone() };
                    ui_art::heading(ui, art, &title, 24.0);
                    chronicle(ui, art, &scene.world, 340.0);
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        if ui_common::button(ui, engine, art, "Zurück", 180.0) {
                            scene.menu_page = MenuPage::Main;
                        }
                    });
                }
            }
        });
    });
}

fn chronicle(ui: &mut egui::Ui, art: &UiArt, world: &ashen_sim::SimWorld, height: f32) {
    art.panel(ui, PanelKind::Parchment, egui::Margin { left: 26, right: 20, top: 20, bottom: 20 }, |ui| {
        ui.set_width(520.0);
        egui::ScrollArea::vertical().max_height(height).auto_shrink([false, true]).show(ui, |ui| {
            if world.chronicle.entries.is_empty() {
                ui.label(RichText::new("Noch ist nichts verzeichnet.").font(theme::italic_font(15.0)).color(theme::INK_FADED));
            }
            for e in &world.chronicle.entries {
                ui.label(RichText::new(format!("Tag {} · {}", e.day, e.phase)).font(theme::caps_font(12.5)).color(theme::INK_RED));
                ui.label(RichText::new(&e.text).size(15.5).color(theme::INK));
                ui.add_space(4.0);
            }
        });
    });
}

fn outcome(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx, o: Outcome) {
    let ctx = ui.ctx().clone();
    let art = cx.ui;
    let victory = o == Outcome::Victory;
    let fx_layer = dim(&ctx, art, if victory { Color32::from_rgb(70, 36, 6) } else { Color32::from_rgb(80, 6, 4) }, if victory { 120 } else { 150 });
    let t = ui.input(|i| i.time) as f32;
    let screen = ctx.content_rect();
    if victory {
        ui_art::embers(art, &fx_layer, screen, t, 70, 7);
    } else {
        // Fallende Asche.
        for i in 0..90 {
            let r = |k: i32| ashen_core::noise::rand2(i, k, 99);
            let speed = 12.0 + r(1) * 30.0;
            let y = screen.min.y + (t * speed + r(2) * screen.height()) % screen.height();
            let x = screen.min.x + r(3) * screen.width() + (t * 0.6 + r(4) * 6.0).sin() * 20.0;
            let s = 1.5 + r(5) * 2.5;
            fx_layer.rect_filled(Rect::from_center_size(Pos2::new(x, y), Vec2::new(s, s * 0.7)), 0.0, Color32::from_rgba_unmultiplied(150, 144, 140, (90.0 + r(6) * 100.0) as u8));
        }
    }
    egui::Area::new(Id::unique("outcome")).order(Order::Foreground).anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(&ctx, |ui| {
        let tint = if victory { Color32::WHITE } else { Color32::from_rgb(196, 150, 140) };
        art.panel_tinted(ui, PanelKind::Ornate, egui::Margin::symmetric(44, 34), tint, |ui| {
            ui.set_width(600.0);
            ui.vertical_centered(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::new(600.0, 86.0), Sense::hover());
                let pulse = 0.8 + 0.2 * (t * 2.0).sin();
                let halo = if victory { Color32::from_rgb((150.0 * pulse) as u8, (80.0 * pulse) as u8, (14.0 * pulse) as u8) } else { Color32::from_rgb((120.0 * pulse) as u8, (10.0 * pulse) as u8, (6.0 * pulse) as u8) };
                art.glow(ui.painter(), Rect::from_center_size(r.center(), Vec2::new(520.0, 150.0)), halo);
                let (title, fx) = if victory { ("Sieg", TextFx::gold()) } else { ("Niederlage", TextFx::ash()) };
                ui_art::fx_text(art, ui.painter(), r.center(), Align2::CENTER_CENTER, title, theme::title_font(64.0), &fx);
                ui.label(RichText::new(&scene.world.mission_def.name).font(theme::title_bold_font(18.0)).color(if victory { theme::GOLD } else { Color32::from_rgb(190, 120, 110) }));
                let (r2, _) = ui.allocate_exact_size(Vec2::new(600.0, 24.0), Sense::hover());
                art.divider(ui.painter(), r2.center().x - 200.0, r2.center().x + 200.0, r2.center().y);
                let def = &scene.world.mission_def;
                let flavour = match (victory, def.victory_text.is_empty(), def.defeat_text.is_empty()) {
                    (true, false, _) => def.victory_text.as_str(),
                    (false, _, false) => def.defeat_text.as_str(),
                    (true, true, _) => "Der Einsatz endete im Sieg. Die Aschmark erinnert sich daran.",
                    (false, _, true) => "Der Einsatz endete in Asche und Schweigen.",
                };
                ui.label(RichText::new(flavour).font(theme::italic_font(16.0)).color(theme::PARCHMENT_DIM));
            });
            ui.add_space(10.0);
            let s = &scene.world.players[0].stats;
            let stats = [
                ("Ausgebildet", s.units_trained.to_string()),
                ("Verluste", s.units_lost.to_string()),
                ("Besiegte Feinde", s.kills.to_string()),
                ("Gebäude errichtet", s.buildings_built.to_string()),
                ("Wellen überstanden", scene.world.mission.waves_survived.to_string()),
                ("Spielzeit", format!("{}:{:02}", scene.world.time as u32 / 60, scene.world.time as u32 % 60)),
            ];
            // Kennzahlen als zentrierte Medaillons (3 × 2).
            let (sr, _) = ui.allocate_exact_size(Vec2::new(600.0, 116.0), Sense::hover());
            let p = ui.painter();
            for (i, (k, v)) in stats.iter().enumerate() {
                let c = Pos2::new(sr.min.x + 100.0 + (i % 3) as f32 * 200.0, sr.min.y + 28.0 + (i / 3) as f32 * 58.0);
                shadow_text(p, c, Align2::CENTER_CENTER, v, theme::heading_font(24.0), if victory { theme::GOLD_BRIGHT } else { Color32::from_rgb(214, 200, 190) });
                p.text(c + Vec2::new(0.0, 21.0), Align2::CENTER_CENTER, *k, theme::caps_font(11.5), theme::PARCHMENT_DIM);
                if i % 3 != 2 {
                    let x = c.x + 100.0;
                    p.line_segment([Pos2::new(x, c.y - 14.0), Pos2::new(x, c.y + 24.0)], Stroke::new(1.0, theme::GOLD_DIM.gamma_multiply(0.5)));
                }
            }
            ui.horizontal(|ui| {
                let w = 470.0;
                ui.add_space((ui.available_width() - w).max(0.0) / 2.0);
                ui.label(RichText::new("Gesammelt").font(theme::caps_font(13.0)).color(theme::PARCHMENT_DIM));
                ui.add_space(6.0);
                for (k, v) in [(ResourceKind::Gold, s.gathered.gold), (ResourceKind::Wood, s.gathered.wood), (ResourceKind::Stone, s.gathered.stone), (ResourceKind::Food, s.gathered.food)] {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(24.0), Sense::hover());
                    ui_common::paint_sprite(ui.painter(), cx.tex, sprite(cx, res_icon(k)), r, Color32::WHITE);
                    ui.label(RichText::new(format!("{v:.0}")).font(theme::heading_font(16.0)));
                    ui.add_space(10.0);
                }
            });
            ui.add_space(8.0);
            chronicle(ui, art, &scene.world, 170.0);
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_space(if scene.campaign { 20.0 } else { 60.0 });
                if scene.campaign && ui_common::button(ui, engine, art, if victory { "Weiter" } else { "Zum Briefing" }, 180.0) {
                    scene.action = SceneAction::CampaignContinue;
                }
                if ui_common::button(ui, engine, art, "Erneut spielen", if scene.campaign { 180.0 } else { 220.0 }) {
                    scene.action = SceneAction::Restart;
                }
                ui.add_space(if scene.campaign { 4.0 } else { 20.0 });
                if ui_common::button(ui, engine, art, "Hauptmenü", if scene.campaign { 180.0 } else { 220.0 }) {
                    scene.action = SceneAction::MainMenu;
                }
            });
        });
    });
}
