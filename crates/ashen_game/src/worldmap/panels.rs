//! Rahmen der Weltkarten-Oberfläche: Oberleiste, Bedienleiste, Benachrichtigungen, Ereignis-, Friedens- und
//! Schlacht-Dialoge, Pausenmenü und Tooltips.

use super::mapview::{DrawCtx, MapMode, province_markers, realm_color};
use super::store;
use super::widgets::*;
use super::windows;
use super::{DAYS_PER_SECOND, MenuPage, WorldAction, WorldMapScene};
use crate::art::ArtLibrary;
use crate::hud::shadow_text;
use crate::settings::{UserSettings, saves_dir};
use crate::ui_art::{self, PanelKind, TextFx, UiArt};
use crate::ui_common;
use ashen_engine::Engine;
use ashen_engine::ui::theme;
use ashen_strategy::catalog;
use ashen_strategy::*;
use egui::{
    Align2, Color32, Id, Order, Pos2, Rect, RichText, Sense, Shape, Stroke, TextureId, Vec2,
};

pub const TOP_H: f32 = 56.0;
pub const BOTTOM_H: f32 = 50.0;

pub(super) fn draw(
    s: &mut WorldMapScene,
    ui: &mut egui::Ui,
    engine: &mut Engine,
    lib: &ArtLibrary,
    tex: TextureId,
    art: &UiArt,
    settings: &mut UserSettings,
) {
    let ctx = ui.ctx().clone();
    let screen = ctx.content_rect();
    let t = ctx.input(|i| i.time) as f32;
    let me = s.me();
    let mut cmds: Vec<StrategyCommand> = Vec::new();
    let mut reqs: Vec<Req> = Vec::new();
    hotkeys(s, &ctx);
    let blocked = s.modal_open();

    // ---- Karte
    let rect = Rect::from_min_max(
        Pos2::new(screen.min.x, screen.min.y + TOP_H),
        Pos2::new(screen.max.x, screen.max.y - BOTTOM_H),
    );
    let preview = match (s.selected_army, s.hover) {
        (Some(a), Some(h)) => s
            .world
            .army(a)
            .filter(|a| a.owner == Some(me) && a.location != h)
            .and_then(|a| s.world.find_path(me, a.location, h)),
        _ => None,
    };
    let input = {
        let cx = DrawCtx {
            world: &s.world,
            art: lib,
            ui_art: art,
            atlas: tex,
            t,
            selected: s.selected,
            selected_army: s.selected_army,
            preview,
        };
        s.view.show(ui, rect, s.mode, &cx, blocked)
    };
    s.hover = if blocked { None } else { input.hover };
    if !blocked {
        if let Some(a) = input.clicked_army {
            s.selected_army = Some(a);
            if let Some(army) = s.world.army(a) {
                s.selected = Some(army.location);
            }
            s.open_window(WinKind::Military);
            s.sounds.push("sfx.map_select");
        } else if let Some(p) = input.clicked {
            s.selected = Some(p);
            if !s.open.contains(&WinKind::Province) {
                s.open_window(WinKind::Province);
            }
            s.sounds.push("sfx.map_select");
        }
        if let (Some(p), Some(a)) = (input.right_clicked, s.selected_army)
            && s.world.army(a).is_some_and(|x| x.owner == Some(me))
        {
            cmds.push(StrategyCommand::MoveArmy { army: a, to: p });
        }
    }
    s.view.draw_legend(ui, rect, s.mode, &s.world, art);
    if !blocked {
        map_tooltip(s, &ctx, art, &input);
    }

    // ---- Leisten (nach der Karte, damit sie darüber liegen)
    top_bar(s, ui, engine, lib, tex, art, t);
    bottom_bar(s, ui, engine, art);

    // ---- Kartenmodi (schwebende Leiste)
    egui::Area::new(Id::unique("wm_modes"))
        .order(Order::Middle)
        .anchor(Align2::LEFT_TOP, Vec2::new(12.0, TOP_H + 10.0))
        .show(&ctx, |ui| {
            art.panel(ui, PanelKind::Thin, egui::Margin::symmetric(10, 6), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 3.0;
                    for m in MapMode::ALL {
                        let r = ui_art::choice(ui, art, m.label(), s.mode == m);
                        ui_common::sfx(engine, ui, &r);
                        if ui_art::tip(r, art, m.label(), &format!("{} · Tab wechselt", m.hint()))
                            .clicked()
                        {
                            s.mode = m;
                        }
                    }
                });
            });
        });

    // ---- Fenster
    {
        let open = s.open.clone();
        let mut cx = Cx {
            w: &s.world,
            me,
            art,
            engine,
            st: &mut s.st,
            cmds: &mut cmds,
            reqs: &mut reqs,
            selected: s.selected,
            selected_army: s.selected_army,
            last_result: &s.last_result,
        };
        windows::draw_all(&ctx, &mut cx, &open);
    }
    notices(s, &ctx, art, &mut reqs);

    // ---- Dialoge
    if s.game_over() {
        game_over(s, &ctx, engine, art);
    } else if s.pending_battle().is_some() {
        battle_dialog(s, &ctx, engine, art, &mut reqs);
    } else if s.pending_event().is_some() {
        event_popup(s, &ctx, engine, art, &mut cmds);
    } else if s.pending_peace().is_some() {
        peace_popup(s, &ctx, engine, art, &mut cmds);
    }
    if s.menu_open {
        pause_menu(s, &ctx, engine, art, settings);
    }
    toast(s, &ctx, art);

    s.run_commands(engine, cmds);
    s.handle_reqs(reqs);
}

fn hotkeys(s: &mut WorldMapScene, ctx: &egui::Context) {
    if ctx.egui_wants_keyboard_input() {
        return;
    }
    let (space, esc, tab, digits, wins) = ctx.input(|i| {
        let mut d = None;
        for (n, k) in [
            egui::Key::Num1,
            egui::Key::Num2,
            egui::Key::Num3,
            egui::Key::Num4,
            egui::Key::Num5,
        ]
        .iter()
        .enumerate()
        {
            if i.key_pressed(*k) {
                d = Some(n as u8 + 1);
            }
        }
        let w: Vec<WinKind> = WinKind::ALL
            .iter()
            .copied()
            .filter(|k| i.key_pressed(k.key()) && !i.modifiers.any())
            .collect();
        (
            i.key_pressed(egui::Key::Space),
            i.key_pressed(egui::Key::Escape),
            i.key_pressed(egui::Key::Tab),
            d,
            w,
        )
    });
    if s.modal_open() && !s.menu_open {
        return;
    }
    if esc {
        if s.menu_open {
            s.menu_open = false;
        } else if let Some(k) = s.open.pop() {
            let _ = k;
        } else {
            s.menu_open = true;
            s.menu_page = MenuPage::Main;
        }
        return;
    }
    if s.menu_open {
        return;
    }
    if space {
        if s.speed == 0 {
            s.speed = s.last_speed.max(1);
        } else {
            s.last_speed = s.speed;
            s.speed = 0;
        }
    }
    if let Some(d) = digits {
        s.speed = d;
        s.last_speed = d;
    }
    if tab {
        s.mode = s.mode.next();
    }
    for k in wins {
        s.toggle_window(k);
    }
}

fn top_bar(
    s: &mut WorldMapScene,
    ui: &mut egui::Ui,
    engine: &mut Engine,
    lib: &ArtLibrary,
    tex: TextureId,
    art: &UiArt,
    t: f32,
) {
    let w = &s.world;
    let me = s.me();
    let realm = &w.realms[me as usize];
    let date = w.date();
    let season = w.season();
    let ruler = &w.chars[realm.ruler as usize];
    let net = realm.income - realm.expenses;
    let cur_speed = s.speed;
    let mut new_speed: Option<u8> = None;
    let mut to_open: Vec<WinKind> = Vec::new();
    let mut open_menu = false;
    egui::Panel::top("wm_top")
        .exact_size(TOP_H)
        .frame(egui::Frame::NONE)
        .show(ui, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter().clone();
            painter.add(art.fill_shape(rect, false, Color32::from_gray(210)));
            let fg = ui.ctx().layer_painter(egui::LayerId::new(
                Order::Background,
                Id::unique("wm_topband"),
            ));
            art.band(&fg, rect.min.x, rect.max.x, rect.max.y - 4.0, true);
            // Links: Reich und Herrscher
            let col = realm_color(realm);
            let crest = Rect::from_min_size(rect.min + Vec2::new(10.0, 8.0), Vec2::new(26.0, 34.0));
            painter.add(Shape::convex_polygon(
                vec![
                    crest.left_top(),
                    crest.right_top(),
                    crest.right_center() + Vec2::new(0.0, 6.0),
                    crest.center_bottom(),
                    crest.left_center() + Vec2::new(0.0, 6.0),
                ],
                col,
                Stroke::new(2.0, theme::GOLD_DIM),
            ));
            if let Some(sp) = lib
                .atlas
                .try_get("icon_crown")
                .or_else(|| lib.atlas.try_get("icon_pop"))
            {
                ui_common::paint_sprite(
                    &painter,
                    tex,
                    sp,
                    crest.shrink(5.0),
                    Color32::from_white_alpha(200),
                );
            }
            shadow_text(
                &painter,
                rect.min + Vec2::new(46.0, 16.0),
                Align2::LEFT_CENTER,
                &realm.name,
                theme::heading_font(17.0),
                theme::GOLD,
            );
            let age = ruler.age(w.day);
            shadow_text(
                &painter,
                rect.min + Vec2::new(46.0, 36.0),
                Align2::LEFT_CENTER,
                &format!(
                    "{} · {} Jahre{}",
                    w.full_name(realm.ruler),
                    age,
                    if realm.regency.is_some() {
                        " · Regentschaft"
                    } else {
                        ""
                    }
                ),
                theme::italic_font(13.5),
                theme::PARCHMENT_DIM,
            );
            let resp = ui.interact(
                Rect::from_min_size(rect.min, Vec2::new(300.0, TOP_H)),
                Id::unique("wm_top_realm"),
                Sense::click(),
            );
            if resp.clicked() {
                to_open.push(WinKind::Realm);
            }
            // Mitte: Datum + Geschwindigkeit
            let line = format!(
                "{}. {} {} n. F.",
                date.day + 1,
                date.month_name(),
                date.year
            );
            let lw = painter
                .layout_no_wrap(line.clone(), theme::heading_font(17.0), theme::GOLD)
                .size()
                .x;
            let dp = Rect::from_min_size(
                Pos2::new(rect.min.x + 320.0, rect.min.y + 4.0),
                Vec2::new(lw + 70.0, TOP_H - 8.0),
            );
            for sh in art.panel_shapes(PanelKind::Plaque, dp, false) {
                painter.add(sh);
            }
            shadow_text(
                &painter,
                dp.center() + Vec2::new(0.0, -7.0),
                Align2::CENTER_CENTER,
                &line,
                theme::heading_font(17.0),
                theme::GOLD,
            );
            shadow_text(
                &painter,
                dp.center() + Vec2::new(0.0, 12.0),
                Align2::CENTER_CENTER,
                &format!("{} · Tag {}", season.label(), w.day),
                theme::italic_font(13.0),
                theme::PARCHMENT_DIM,
            );
            let speed_rect = Rect::from_min_size(
                Pos2::new(dp.max.x + 16.0, rect.min.y + 8.0),
                Vec2::new(300.0, 34.0),
            );
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(speed_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
                |ui| {
                    let r = ui_art::choice(ui, art, "II", cur_speed == 0);
                    ui_common::sfx(engine, ui, &r);
                    if ui_art::tip(r, art, "Pause", "Leertaste").clicked() {
                        new_speed = Some(0);
                    }
                    for n in 1..=5u8 {
                        let r = ui_art::choice(ui, art, &n.to_string(), cur_speed == n);
                        ui_common::sfx(engine, ui, &r);
                        if ui_art::tip(
                            r,
                            art,
                            &format!("Geschwindigkeit {n}"),
                            &format!(
                                "{:.1} Tage pro Sekunde · Taste {n}",
                                DAYS_PER_SECOND[n as usize - 1]
                            ),
                        )
                        .clicked()
                        {
                            new_speed = Some(n);
                        }
                    }
                },
            );
            // Rechts: Kasse, Legitimität, Hohlung, Menü
            let mut x = rect.max.x - 10.0;
            let menu =
                Rect::from_min_size(Pos2::new(x - 84.0, rect.min.y + 9.0), Vec2::new(84.0, 34.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(menu), |ui| {
                let r = ui_art::button(ui, art, "Menü", menu.size(), true);
                ui_common::sfx(engine, ui, &r);
                if ui_art::tip(
                    r,
                    art,
                    "Pausenmenü",
                    "Speichern, Laden, Einstellungen (Esc)",
                )
                .clicked()
                {
                    open_menu = true;
                }
            });
            x -= 96.0;
            let cells: [(String, String, f32, Color32, String, String); 4] = [
                (
                    format!("{:.0}", realm.gold),
                    format!("{net:+.0}/Mon."),
                    0.0,
                    if net >= 0.0 {
                        Color32::from_rgb(150, 210, 120)
                    } else {
                        Color32::from_rgb(236, 110, 80)
                    },
                    "Staatskasse".into(),
                    format!(
                        "Einkommen {:.0} · Ausgaben {:.0} pro Monat. Klick öffnet die Wirtschaft.",
                        realm.income, realm.expenses
                    ),
                ),
                (
                    format!("{:.0}", realm.legitimacy),
                    "Legitimität".into(),
                    realm.legitimacy / 100.0,
                    Color32::from_rgb(120, 170, 240),
                    "Legitimität".into(),
                    "Wie fest Euer Anspruch auf den Thron steht.".into(),
                ),
                (
                    format!("{:.0}", realm.hollow_value),
                    format!("Pfad {}", realm.hollow_path),
                    realm.hollow_value / 100.0,
                    Color32::from_rgb(90, 220, 140),
                    "Hohlung des Reiches".into(),
                    format!(
                        "Schleierdruck der Welt: {:.0} von 100.",
                        w.endgame.veil_pressure
                    ),
                ),
                (
                    format!("{:.0}", realm.prestige),
                    "Prestige".into(),
                    0.0,
                    theme::GOLD,
                    "Prestige".into(),
                    "Ansehen Eures Hauses; für Gesetze und Kriegsgründe.".into(),
                ),
            ];
            for (i, (value, sub, frac, col, title, body)) in cells.iter().enumerate().rev() {
                let cw = if i == 0 { 150.0 } else { 118.0 };
                let cell = Rect::from_min_size(
                    Pos2::new(x - cw, rect.min.y + 6.0),
                    Vec2::new(cw, TOP_H - 14.0),
                );
                for sh in art.panel_shapes(PanelKind::Plaque, cell, false) {
                    painter.add(sh);
                }
                shadow_text(
                    &painter,
                    cell.min + Vec2::new(12.0, 13.0),
                    Align2::LEFT_CENTER,
                    value,
                    theme::heading_font(16.0),
                    theme::PARCHMENT,
                );
                if i == 0 {
                    shadow_text(
                        &painter,
                        cell.min + Vec2::new(cw - 10.0, 15.0),
                        Align2::RIGHT_CENTER,
                        sub,
                        theme::body_bold_font(12.5),
                        *col,
                    );
                    if let Some(sp) = lib.atlas.try_get("icon_gold") {
                        ui_common::paint_sprite(
                            &painter,
                            tex,
                            sp,
                            Rect::from_center_size(
                                cell.min + Vec2::new(cw * 0.5 - 6.0, 15.0),
                                Vec2::splat(18.0),
                            ),
                            Color32::WHITE,
                        );
                    }
                } else {
                    shadow_text(
                        &painter,
                        cell.min + Vec2::new(cw - 10.0, 14.0),
                        Align2::RIGHT_CENTER,
                        sub,
                        theme::italic_font(12.0),
                        theme::PARCHMENT_DIM,
                    );
                }
                if *frac > 0.0 || i == 1 || i == 2 {
                    let br = Rect::from_min_size(
                        cell.min + Vec2::new(10.0, cell.height() - 14.0),
                        Vec2::new(cw - 20.0, 9.0),
                    );
                    painter.rect_filled(br, 0.0, Color32::from_rgb(10, 7, 6));
                    painter.rect_filled(
                        Rect::from_min_size(
                            br.min,
                            Vec2::new(br.width() * frac.clamp(0.0, 1.0), br.height()),
                        ),
                        0.0,
                        *col,
                    );
                    painter.rect_stroke(
                        br,
                        0.0,
                        Stroke::new(1.0, theme::GOLD_DIM),
                        egui::StrokeKind::Outside,
                    );
                }
                let resp = ui.interact(cell, Id::unique(("wm_cell", i)), Sense::click());
                if i == 0 && resp.clicked() {
                    to_open.push(WinKind::Economy);
                }
                if i == 1 && resp.clicked() {
                    to_open.push(WinKind::Realm);
                }
                if i == 2 && resp.clicked() {
                    to_open.push(WinKind::Magic);
                }
                ui_art::tip(resp, art, title, body);
                x -= cw + 8.0;
            }
            let _ = t;
        });
    if let Some(n) = new_speed {
        if s.speed > 0 {
            s.last_speed = s.speed;
        }
        s.speed = n;
        if n > 0 {
            s.last_speed = n;
        }
    }
    for k in to_open {
        s.open_window(k);
    }
    if open_menu {
        s.menu_open = true;
        s.menu_page = MenuPage::Main;
    }
}
fn bottom_bar(s: &mut WorldMapScene, ui: &mut egui::Ui, engine: &mut Engine, art: &UiArt) {
    egui::Panel::bottom("wm_bottom")
        .exact_size(BOTTOM_H)
        .frame(egui::Frame::NONE)
        .show(ui, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter().clone();
            painter.add(art.fill_shape(rect, false, Color32::from_gray(210)));
            let fg = ui.ctx().layer_painter(egui::LayerId::new(
                Order::Background,
                Id::unique("wm_botband"),
            ));
            art.band(&fg, rect.min.x, rect.max.x, rect.min.y - 4.0, false);
            let inner = Rect::from_min_max(
                rect.min + Vec2::new(10.0, 10.0),
                rect.max - Vec2::new(10.0, 4.0),
            );
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(inner)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    for k in WinKind::ALL {
                        let r = ui_art::choice(ui, art, k.short(), s.open.contains(&k));
                        ui_common::sfx(engine, ui, &r);
                        if ui_art::tip(r, art, k.title(), &format!("Taste {}", k.key_label()))
                            .clicked()
                        {
                            s.toggle_window(k);
                        }
                    }
                    if let Some(st) = &s.status {
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new(st)
                                .font(theme::italic_font(13.0))
                                .color(theme::PARCHMENT_DIM),
                        );
                    }
                },
            );
        });
}

fn notices(s: &mut WorldMapScene, ctx: &egui::Context, art: &UiArt, reqs: &mut Vec<Req>) {
    if s.notices.is_empty() {
        return;
    }
    egui::Area::new(Id::unique("wm_notices"))
        .order(Order::Foreground)
        .anchor(Align2::RIGHT_TOP, Vec2::new(-12.0, TOP_H + 10.0))
        .show(ctx, |ui| {
            for n in s.notices.iter().rev() {
                let fade = (1.0 - ((n.age - 10.0) / 4.0).clamp(0.0, 1.0))
                    .min((n.age / 0.25).clamp(0.0, 1.0));
                let col = match n.kind {
                    ChronicleKind::War
                    | ChronicleKind::Battle
                    | ChronicleKind::Siege
                    | ChronicleKind::Revolt => Color32::from_rgb(236, 120, 90),
                    ChronicleKind::Peace
                    | ChronicleKind::Victory
                    | ChronicleKind::Marriage
                    | ChronicleKind::Coronation => Color32::from_rgb(150, 210, 120),
                    ChronicleKind::Plague
                    | ChronicleKind::Famine
                    | ChronicleKind::Disaster
                    | ChronicleKind::Hollowing => Color32::from_rgb(190, 220, 90),
                    _ => theme::GOLD,
                };
                let resp = art.panel_tinted(
                    ui,
                    PanelKind::Thin,
                    egui::Margin::symmetric(14, 8),
                    Color32::from_white_alpha((fade * 255.0) as u8),
                    |ui| {
                        ui.set_width(300.0);
                        ui.label(
                            RichText::new(n.kind.label())
                                .font(theme::caps_font(11.0))
                                .color(col.gamma_multiply(fade)),
                        );
                        ui.label(
                            RichText::new(&n.text)
                                .font(theme::body_font(14.0))
                                .color(theme::PARCHMENT.gamma_multiply(fade)),
                        );
                    },
                );
                if let Some(p) = n.province
                    && ui
                        .interact(
                            resp.response.rect,
                            Id::unique(("wm_notice", n.age.to_bits())),
                            Sense::click(),
                        )
                        .clicked()
                {
                    reqs.push(Req::Center(p));
                    reqs.push(Req::Select(p));
                }
                ui.add_space(4.0);
            }
        });
}

fn toast(s: &WorldMapScene, ctx: &egui::Context, art: &UiArt) {
    let Some((text, _)) = &s.toast else { return };
    egui::Area::new(Id::unique("wm_toast"))
        .order(Order::Tooltip)
        .anchor(Align2::CENTER_BOTTOM, Vec2::new(0.0, -BOTTOM_H - 24.0))
        .show(ctx, |ui| {
            art.panel(ui, PanelKind::Thin, egui::Margin::symmetric(18, 9), |ui| {
                ui.label(
                    RichText::new(text)
                        .font(theme::body_font(15.0))
                        .color(theme::PARCHMENT),
                );
            });
        });
}

fn map_tooltip(
    s: &WorldMapScene,
    ctx: &egui::Context,
    art: &UiArt,
    input: &super::mapview::MapInput,
) {
    let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) else {
        return;
    };
    if ctx.egui_is_using_pointer() || ctx.input(|i| i.pointer.any_down()) {
        return;
    }
    // Nur über der Karte (nicht über Fenstern): Fenster-Areas melden `is_pointer_over_egui`.
    if ctx
        .layer_id_at(pos)
        .is_some_and(|l| l.order != Order::Background)
    {
        return;
    }
    let w = &s.world;
    let lines: Vec<(String, Color32)> = if let Some(a) = input.hover_army.and_then(|a| w.army(a)) {
        let owner = a
            .owner
            .map(|o| w.rname(o))
            .unwrap_or_else(|| "Niemandsland".into());
        let mut v = vec![
            (a.name.clone(), theme::GOLD),
            (
                format!("{owner} · {} Mann · Moral {:.0}", a.size(), a.morale),
                theme::PARCHMENT,
            ),
        ];
        for r in a.regiments.iter().take(5) {
            v.push((
                format!(
                    "{}× {}",
                    r.count,
                    catalog::unit(&r.unit).map(|u| u.name).unwrap_or("?")
                ),
                theme::PARCHMENT_DIM,
            ));
        }
        v
    } else if let Some(p) = input.hover_marker.map(|p| &w.provinces[p as usize]) {
        let mut v = vec![(format!("Ereignisse in {}", p.name), theme::GOLD)];
        for m in province_markers(w, p) {
            v.push((m.name().into(), m.tint()));
        }
        v
    } else if let Some(p) = input.hover.map(|p| &w.provinces[p as usize]) {
        let mut v = vec![
            (p.name.clone(), theme::GOLD),
            (
                format!("{} · {}", p.biome.label(), p.settlement.name()),
                theme::PARCHMENT_DIM,
            ),
        ];
        v.push((
            p.holder()
                .map(|h| w.rname(h))
                .unwrap_or_else(|| "herrenlos".into()),
            theme::PARCHMENT,
        ));
        v.push((
            format!(
                "{:.0} Einwohner · Zufriedenheit {:.0} · Unruhe {:.0}",
                p.population(),
                p.avg_satisfaction(),
                p.unrest
            ),
            theme::PARCHMENT_DIM,
        ));
        if p.hollowing >= 10.0 {
            v.push((
                format!("Hohlung {:.0}", p.hollowing),
                Color32::from_rgb(110, 230, 150),
            ));
        }
        for m in province_markers(w, p) {
            v.push((m.name().into(), m.tint()));
        }
        if s.selected_army.is_some() && p.owner.is_some() {
            v.push((
                "Rechtsklick: Heer hierher schicken".into(),
                theme::EMBER_HOT,
            ));
        }
        v
    } else {
        return;
    };
    egui::Area::new(Id::unique("wm_maptip"))
        .order(Order::Tooltip)
        .fixed_pos(pos + Vec2::new(18.0, 14.0))
        .interactable(false)
        .show(ctx, |ui| {
            art.panel(ui, PanelKind::Thin, egui::Margin::symmetric(14, 10), |ui| {
                for (t, c) in &lines {
                    ui.label(RichText::new(t).font(theme::body_font(14.0)).color(*c));
                }
            });
        });
}

fn dim_screen(ctx: &egui::Context, alpha: u8) {
    let p = ctx.layer_painter(egui::LayerId::new(Order::Middle, Id::unique("wm_dim")));
    p.rect_filled(ctx.content_rect(), 0.0, Color32::from_black_alpha(alpha));
}

fn fx_title(art: &UiArt, ui: &mut egui::Ui, text: &str, size: f32) {
    let (r, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), size * 1.4), Sense::hover());
    ui_art::fx_text(
        art,
        ui.painter(),
        r.center(),
        Align2::CENTER_CENTER,
        text,
        theme::title_font(size),
        &TextFx::gold(),
    );
}

// -------------------------------------------------------------------------------------------------
// Ereignis-Popup (Pergament)
// -------------------------------------------------------------------------------------------------

fn event_popup(
    s: &WorldMapScene,
    ctx: &egui::Context,
    engine: &mut Engine,
    art: &UiArt,
    cmds: &mut Vec<StrategyCommand>,
) {
    let Some(ev) = s.pending_event() else { return };
    dim_screen(ctx, 150);
    egui::Area::new(Id::unique("wm_event"))
        .order(Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            art.panel(
                ui,
                PanelKind::Ornate,
                egui::Margin::symmetric(40, 30),
                |ui| {
                    ui.set_width(600.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new(match ev.province {
                                Some(p) => format!(
                                    "{} · {}",
                                    s.world.pname(p),
                                    Date::from_day(ev.day).month_name()
                                ),
                                None => "Ein Ereignis".to_string(),
                            })
                            .font(theme::caps_font(13.0))
                            .color(theme::GOLD_DIM),
                        );
                        fx_title(art, ui, &ev.title, 30.0);
                    });
                    art.panel(
                        ui,
                        PanelKind::Parchment,
                        egui::Margin {
                            left: 30,
                            right: 30,
                            top: 22,
                            bottom: 22,
                        },
                        |ui| {
                            ui.set_width(ui.available_width());
                            ui.label(
                                RichText::new(&ev.text)
                                    .font(theme::body_font(16.5))
                                    .color(theme::INK),
                            );
                        },
                    );
                    ui.add_space(12.0);
                    for (id, text, affordable) in &ev.options {
                        let r = ui_art::button(
                            ui,
                            art,
                            text,
                            Vec2::new(ui.available_width(), 40.0),
                            *affordable,
                        );
                        if *affordable {
                            ui_common::sfx(engine, ui, &r);
                        }
                        if r.clicked() && *affordable {
                            cmds.push(StrategyCommand::ChooseEvent {
                                pending: ev.id,
                                option: id.clone(),
                            });
                        }
                        ui.add_space(2.0);
                    }
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(
                            "Unbeantwortete Fenster entscheiden Eure Räte nach 60 Tagen.",
                        )
                        .font(theme::italic_font(13.0))
                        .color(theme::PARCHMENT_DIM),
                    );
                },
            );
        });
}

fn peace_popup(
    s: &WorldMapScene,
    ctx: &egui::Context,
    engine: &mut Engine,
    art: &UiArt,
    cmds: &mut Vec<StrategyCommand>,
) {
    let Some(o) = s.pending_peace() else { return };
    dim_screen(ctx, 140);
    let w = &s.world;
    egui::Area::new(Id::unique("wm_peace"))
        .order(Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            art.panel(
                ui,
                PanelKind::Ornate,
                egui::Margin::symmetric(40, 30),
                |ui| {
                    ui.set_width(520.0);
                    ui.vertical_centered(|ui| fx_title(art, ui, "Friedensangebot", 30.0));
                    art.panel(
                        ui,
                        PanelKind::Parchment,
                        egui::Margin {
                            left: 28,
                            right: 28,
                            top: 20,
                            bottom: 20,
                        },
                        |ui| {
                            ui.set_width(ui.available_width());
                            ui.label(
                                RichText::new(format!("{} bietet Frieden an.", w.rname(o.from)))
                                    .font(theme::heading_font(17.0))
                                    .color(theme::INK_RED),
                            );
                            let t = &o.terms;
                            let mut lines: Vec<String> = Vec::new();
                            if !t.cede.is_empty() {
                                lines.push(format!(
                                    "Abtretung: {}",
                                    t.cede
                                        .iter()
                                        .map(|p| w.pname(*p))
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                ));
                            }
                            if t.tribute {
                                lines.push("Tribut über zehn Jahre".into());
                            }
                            if t.vassalize {
                                lines.push("Unterwerfung als Vasall".into());
                            }
                            if t.independence {
                                lines.push("Unabhängigkeit".into());
                            }
                            if lines.is_empty() {
                                lines.push("Weißer Frieden – keine Bedingungen.".into());
                            }
                            for l in lines {
                                ui.label(
                                    RichText::new(l)
                                        .font(theme::body_font(16.0))
                                        .color(theme::INK),
                                );
                            }
                        },
                    );
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(40.0);
                        if ui_common::button(ui, engine, art, "Annehmen", 200.0) {
                            cmds.push(StrategyCommand::AnswerPeaceOffer {
                                offer: o.id,
                                accept: true,
                            });
                        }
                        ui.add_space(20.0);
                        if ui_common::button(ui, engine, art, "Ablehnen", 200.0) {
                            cmds.push(StrategyCommand::AnswerPeaceOffer {
                                offer: o.id,
                                accept: false,
                            });
                        }
                    });
                },
            );
        });
}

// -------------------------------------------------------------------------------------------------
// Schlacht-Dialog
// -------------------------------------------------------------------------------------------------

fn side_column(
    ui: &mut egui::Ui,
    w: &StrategyWorld,
    title: &str,
    list: &[BattleArmy],
    me: RealmId,
) {
    ui.vertical(|ui| {
        ui.set_width(250.0);
        ui.label(
            RichText::new(title)
                .font(theme::caps_font(13.0))
                .color(theme::INK_RED),
        );
        for b in list {
            let mine = b.realm == Some(me);
            let realm = b
                .realm
                .map(|r| w.rname(r))
                .unwrap_or_else(|| "Niemandsland".into());
            ui.label(
                RichText::new(format!("{} ({realm})", b.name))
                    .font(theme::body_bold_font(15.0))
                    .color(if mine { theme::INK_RED } else { theme::INK }),
            );
            if let Some(c) = &b.commander_name {
                ui.label(
                    RichText::new(format!("Befehlshaber: {c}"))
                        .font(theme::italic_font(13.5))
                        .color(theme::INK_FADED),
                );
            }
            for r in &b.regiments {
                ui.label(
                    RichText::new(format!(
                        "{}× {}{}",
                        r.count,
                        catalog::unit(&r.unit).map(|u| u.name).unwrap_or("?"),
                        if r.vet > 0 {
                            format!(" (Veteran {})", r.vet)
                        } else {
                            String::new()
                        }
                    ))
                    .font(theme::body_font(14.0))
                    .color(theme::INK),
                );
            }
            if !b.heroes.is_empty() {
                ui.label(
                    RichText::new(format!(
                        "Helden: {}",
                        b.heroes
                            .iter()
                            .map(|c| w.cname(*c))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                    .font(theme::body_font(14.0))
                    .color(theme::INK_RED),
                );
            }
            ui.label(
                RichText::new(format!("Moral {:.0} · Kampfkraft {:.0}", b.morale, b.power))
                    .font(theme::italic_font(13.5))
                    .color(theme::INK_FADED),
            );
            ui.add_space(4.0);
        }
    });
}

fn battle_dialog(
    s: &WorldMapScene,
    ctx: &egui::Context,
    engine: &mut Engine,
    art: &UiArt,
    reqs: &mut Vec<Req>,
) {
    let Some(b) = s.pending_battle() else { return };
    dim_screen(ctx, 160);
    let w = &s.world;
    let me = s.me();
    let player_attacks = b.attackers.iter().any(|a| a.realm == Some(me));
    egui::Area::new(Id::unique("wm_battle")).order(Order::Foreground).anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(ctx, |ui| {
        art.panel(ui, PanelKind::Ornate, egui::Margin::symmetric(40, 30), |ui| {
            ui.set_width(640.0);
            let t = ui.input(|i| i.time) as f32;
            ui.vertical_centered(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::new(600.0, 56.0), Sense::hover());
                let pulse = 0.8 + 0.2 * (t * 2.5).sin();
                art.glow(ui.painter(), Rect::from_center_size(r.center(), Vec2::new(520.0, 100.0)), Color32::from_rgb((110.0 * pulse) as u8, (30.0 * pulse) as u8, (8.0 * pulse) as u8));
                ui_art::fx_text(art, ui.painter(), r.center(), Align2::CENTER_CENTER, &if b.siege { format!("Belagerung von {}", b.province_name) } else { format!("Schlacht bei {}", b.province_name) }, theme::title_font(32.0), &TextFx::gold());
                ui.label(RichText::new(format!("{} · {} · {} · {}", b.terrain.label(), b.season.label(), b.weather.label(), if player_attacks { "Ihr greift an" } else { "Ihr verteidigt" })).font(theme::italic_font(15.0)).color(theme::PARCHMENT_DIM));
                if b.siege || b.fortification > 0 {
                    ui.label(RichText::new(format!("Befestigung Stufe {} · {} Mauerabschnitte eingestürzt", b.fortification, b.breaches)).font(theme::body_font(14.5)).color(theme::PARCHMENT));
                }
            });
            ui.add_space(8.0);
            art.panel(ui, PanelKind::Parchment, egui::Margin { left: 26, right: 26, top: 18, bottom: 18 }, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal_top(|ui| {
                    side_column(ui, w, "Angreifer", &b.attackers, me);
                    ui.add_space(24.0);
                    side_column(ui, w, "Verteidiger", &b.defenders, me);
                });
                ui.add_space(6.0);
                ui.label(RichText::new(format!("Erwartete Verluste bei automatischer Auflösung: Angreifer {:.0} %, Verteidiger {:.0} %", b.preview_loss_attacker, b.preview_loss_defender)).font(theme::italic_font(14.0)).color(theme::INK_FADED));
            });
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.add_space(24.0);
                if ui_common::button(ui, engine, art, "Selbst kommandieren", 270.0) {
                    reqs.push(Req::PlayBattle(b.id));
                }
                ui.add_space(20.0);
                if ui_common::button(ui, engine, art, "Automatisch auflösen", 270.0) {
                    reqs.push(Req::AutoBattle(b.id));
                }
            });
        });
    });
}

// -------------------------------------------------------------------------------------------------
// Sieg / Untergang
// -------------------------------------------------------------------------------------------------

fn game_over(s: &mut WorldMapScene, ctx: &egui::Context, engine: &mut Engine, art: &UiArt) {
    dim_screen(ctx, 170);
    let me = s.me();
    let fallen = !s.world.realms[me as usize].alive;
    let victory = s.world.victory;
    let (title, text, ok) = match (victory, fallen) {
        (Some(v), _) if v.realm == me => (format!("Sieg: {}", v.kind.name()), "Eure Dynastie hat die Geschichte Eldraeths geschrieben. Ihr dürft weiterspielen oder die Chronik lesen.".to_string(), true),
        (Some(v), _) => (format!("{} errang den Sieg", s.world.rname(v.realm)), format!("{} erfüllte das Ziel „{}“. Eldraeth gehört nun anderen Händen.", s.world.rname(v.realm), v.kind.name()), false),
        (None, true) => ("Euer Reich ist gefallen".to_string(), "Die letzte Krone zerbrach. Nur die Chronik erinnert noch an Euer Haus.".to_string(), false),
        _ => return,
    };
    egui::Area::new(Id::unique("wm_over"))
        .order(Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            art.panel(
                ui,
                PanelKind::Ornate,
                egui::Margin::symmetric(44, 34),
                |ui| {
                    ui.set_width(560.0);
                    ui.vertical_centered(|ui| {
                        let (r, _) = ui.allocate_exact_size(Vec2::new(560.0, 70.0), Sense::hover());
                        ui_art::fx_text(
                            art,
                            ui.painter(),
                            r.center(),
                            Align2::CENTER_CENTER,
                            &title,
                            theme::title_font(34.0),
                            &if ok { TextFx::gold() } else { TextFx::ash() },
                        );
                        ui.label(
                            RichText::new(text)
                                .font(theme::italic_font(16.0))
                                .color(theme::PARCHMENT_DIM),
                        );
                    });
                    ui.add_space(14.0);
                    ui.horizontal(|ui| {
                        ui.add_space(30.0);
                        if !fallen && ui_common::button(ui, engine, art, "Weiterspielen", 220.0) {
                            s.victory_ack = true;
                        }
                        if fallen && ui_common::button(ui, engine, art, "Chronik lesen", 220.0) {
                            s.defeat_ack = true;
                            s.open_window(WinKind::Chronicle);
                        }
                        ui.add_space(14.0);
                        if ui_common::button(ui, engine, art, "Hauptmenü", 220.0) {
                            s.action = WorldAction::MainMenu;
                        }
                    });
                },
            );
        });
}

// -------------------------------------------------------------------------------------------------
// Pausenmenü
// -------------------------------------------------------------------------------------------------

fn pause_menu(
    s: &mut WorldMapScene,
    ctx: &egui::Context,
    engine: &mut Engine,
    art: &UiArt,
    settings: &mut UserSettings,
) {
    dim_screen(ctx, 150);
    egui::Area::new(Id::unique("wm_pause"))
        .order(Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            art.panel(
                ui,
                PanelKind::Ornate,
                egui::Margin::symmetric(40, 34),
                |ui| {
                    ui.set_min_width(380.0);
                    match s.menu_page {
                        MenuPage::Main => {
                            ui_art::heading(ui, art, "Pause", 30.0);
                            ui.vertical_centered(|ui| {
                                for (label, act) in [
                                    ("Weiterspielen", 0),
                                    ("Speichern", 1),
                                    ("Laden", 2),
                                    ("Einstellungen", 3),
                                    ("Hauptmenü", 4),
                                    ("Spiel beenden", 5),
                                ] {
                                    let r = ui_art::menu_item(ui, art, label, 300.0, true, false);
                                    ui_common::sfx(engine, ui, &r);
                                    if r.clicked() {
                                        match act {
                                            0 => s.menu_open = false,
                                            1 => s.menu_page = MenuPage::Save,
                                            2 => s.menu_page = MenuPage::Load,
                                            3 => s.menu_page = MenuPage::Settings,
                                            4 => {
                                                let _ = store::autosave(&s.world);
                                                s.action = WorldAction::MainMenu;
                                            }
                                            _ => s.action = WorldAction::Quit,
                                        }
                                    }
                                }
                            });
                        }
                        MenuPage::Save => {
                            ui_art::heading(ui, art, "Spiel speichern", 24.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Name")
                                        .font(theme::caps_font(14.0))
                                        .color(theme::GOLD),
                                );
                                ui.add(
                                    egui::TextEdit::singleline(&mut s.save_name)
                                        .desired_width(260.0)
                                        .font(theme::body_font(16.0)),
                                );
                            });
                            if let Some(st) = &s.status {
                                ui.label(
                                    RichText::new(st)
                                        .font(theme::italic_font(14.0))
                                        .color(theme::PARCHMENT_DIM),
                                );
                            }
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                if ui_common::button(ui, engine, art, "Speichern", 160.0) {
                                    match store::save_slot(&s.world, &s.save_name) {
                                        Ok(p) => {
                                            s.status = Some(format!(
                                                "Gespeichert: {}",
                                                p.file_name()
                                                    .map(|f| f.to_string_lossy().to_string())
                                                    .unwrap_or_default()
                                            ));
                                            s.menu_page = MenuPage::Main;
                                        }
                                        Err(e) => {
                                            s.status =
                                                Some(format!("Speichern fehlgeschlagen: {e}"))
                                        }
                                    }
                                }
                                if ui_common::button(ui, engine, art, "Zurück", 160.0) {
                                    s.menu_page = MenuPage::Main;
                                }
                            });
                        }
                        MenuPage::Load => {
                            ui_art::heading(ui, art, "Spiel laden", 24.0);
                            ui.set_width(540.0);
                            match store::save_list(ui, engine, art, &saves_dir()) {
                                Some(p) => s.action = WorldAction::Load(p),
                                None => {
                                    if ashen_strategy::save::list(&saves_dir()).is_empty() {
                                        ui.label(
                                            RichText::new(
                                                "Keine Spielstände der Großen Kampagne vorhanden.",
                                            )
                                            .font(theme::italic_font(15.0))
                                            .color(theme::PARCHMENT_DIM),
                                        );
                                    }
                                }
                            }
                            ui.add_space(8.0);
                            ui.vertical_centered(|ui| {
                                if ui_common::button(ui, engine, art, "Zurück", 200.0) {
                                    s.menu_page = MenuPage::Main;
                                }
                            });
                        }
                        MenuPage::Settings => {
                            ui.set_width(600.0);
                            ui_art::heading(ui, art, "Einstellungen", 24.0);
                            ui_common::settings_ui(ui, engine, settings, art);
                            ui.add_space(8.0);
                            ui.vertical_centered(|ui| {
                                if ui_common::button(ui, engine, art, "Zurück", 200.0) {
                                    settings.save();
                                    s.menu_page = MenuPage::Main;
                                }
                            });
                        }
                    }
                },
            );
        });
}
