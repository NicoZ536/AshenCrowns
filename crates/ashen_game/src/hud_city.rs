//! Oberfläche der Echtzeit-Wirtschaft: Lagerfenster, Stadtübersicht (Steuern, Bürger, Ausbau),
//! Baumenü mit Rubriken, Marktfenster und die Betriebsansicht eines gewählten Gebäudes.

use crate::hud::{shadow_text, sprite, HudCtx, CONSOLE_H, TOP_H};
use crate::mission::{CommandMode, MissionScene};
use crate::ui_art::{self, BtnState, PanelKind};
use crate::ui_common;
use ashen_engine::ui::theme;
use ashen_engine::Engine;
use ashen_sim::data::{BuildCategory, BuildingDef, BuildingTypeId, Cost, Item, ItemAmounts, Recipe, ResourceKind};
use ashen_sim::entity::{Building, BuildingId, ProdStatus, PLAYER};
use ashen_sim::world::SimWorld;
use ashen_sim::GameData;
use egui::{Align2, Color32, Id, Pos2, Rect, RichText, Sense, Ui, Vec2};

const RED: Color32 = Color32::from_rgb(236, 90, 70);

/// Atlas-Name des Gebäudebildes für Kacheln und Porträts (Mauer und Tor als Einzelteil).
pub fn building_icon(d: &BuildingDef) -> String {
    match d.fort {
        Some(ashen_sim::data::Fort::Wall) => "bld_wall_m0".into(),
        Some(ashen_sim::data::Fort::Gate) => "bld_gate_x_open".into(),
        _ => format!("bld_{}", d.id),
    }
}

/// Icon-Name eines Items im Atlas.
pub fn item_icon(data: &GameData, it: Item) -> String {
    match it {
        Item::Res(ResourceKind::Gold) => "icon_gold".into(),
        Item::Res(ResourceKind::Wood) => "icon_wood".into(),
        Item::Res(ResourceKind::Stone) => "icon_stone".into(),
        Item::Res(ResourceKind::Food) => "icon_food".into(),
        Item::Good(g) => format!("icon_{}", data.good(g).id),
    }
}

/// Kosten (Grundressourcen und Waren) als Icon-Reihe; fehlende Mengen werden rot.
pub fn cost_row_full(ui: &mut Ui, cx: &HudCtx, world: &SimWorld, cost: &Cost, goods: &ItemAmounts) {
    let have = &world.players[PLAYER as usize];
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        let mut entries: Vec<(Item, f32)> = ResourceKind::ALL.iter().filter(|k| cost.get(**k) > 0.0).map(|k| (Item::Res(*k), cost.get(*k))).collect();
        entries.extend(goods.iter().copied());
        for (it, v) in entries {
            let (r, _) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::hover());
            ui_common::paint_sprite(ui.painter(), cx.tex, sprite(cx, &item_icon(cx.data, it)), r, Color32::WHITE);
            let ok = have.stock(it) + 1e-3 >= v;
            ui.label(RichText::new(format!("{v:.0}")).font(theme::body_bold_font(15.0)).color(if ok { theme::PARCHMENT } else { RED }));
            ui.add_space(5.0);
        }
    });
}

/// Rezept als lesbarer Text: „2× Holz → 3× Bretter · 30 s“.
pub fn recipe_text(data: &GameData, r: &Recipe) -> String {
    let list = |l: &ItemAmounts| l.iter().map(|(it, n)| format!("{n:.0}× {}", data.item_name(*it))).collect::<Vec<_>>().join(" + ");
    match (r.inputs_r.is_empty(), r.outputs_r.is_empty()) {
        (true, false) => format!("{} · {:.0} s", list(&r.outputs_r), r.cycle),
        (false, true) => format!("verbraucht {} · alle {:.0} s", list(&r.inputs_r), r.cycle),
        (false, false) => format!("{} → {} · {:.0} s", list(&r.inputs_r), list(&r.outputs_r), r.cycle),
        (true, true) => String::new(),
    }
}

fn status_text(data: &GameData, b: &Building) -> (String, Color32) {
    let radius = data.rules.city.logistics_radius;
    let item = b.stall_item.map(|it| data.item_name(it).to_string()).unwrap_or_default();
    match b.status {
        ProdStatus::None => (String::new(), theme::PARCHMENT),
        ProdStatus::Running => ("Arbeitet".into(), Color32::from_rgb(150, 210, 120)),
        ProdStatus::MissingInput => (format!("Stillstand: es fehlt {item}"), RED),
        ProdStatus::NoWorkers => ("Stillstand: keine Arbeiter frei – baut Wohnhäuser".into(), RED),
        ProdStatus::StorageFull => (format!("Stillstand: Lager für {item} ist voll"), RED),
        ProdStatus::NoDepot => (format!("Stillstand: kein Lager in {radius:.0} Kacheln Reichweite"), RED),
        ProdStatus::NoSite => ("Stillstand: Vorkommen erschöpft oder nicht in Reichweite".into(), RED),
        ProdStatus::Disabled => ("Stillgelegt".into(), theme::PARCHMENT_DIM),
    }
}

/// Zusammenfassung dessen, was ein Gebäude tut (für Tooltips des Baumenüs).
fn building_summary(data: &GameData, d: &BuildingDef) -> Vec<String> {
    let mut v = Vec::new();
    for r in &d.recipes {
        let t = recipe_text(data, r);
        if !t.is_empty() {
            v.push(if d.recipes.len() > 1 { format!("{}: {t}", r.name) } else { t });
        }
    }
    if d.workers > 0 {
        v.push(format!("Arbeiter: {}", d.workers));
    }
    if let Some(h) = &d.housing {
        v.push(format!("Wohnraum: {} Bewohner · Steuer {:.1} Gold/min je Bewohner", h.capacity, h.tax * 60.0));
    }
    if d.storage > 0.0 {
        v.push(format!("Lager: {:.0} je Ware", d.storage));
    }
    if d.market {
        v.push("Handel: Waren gegen Gold".into());
    }
    v
}

// ---------------------------------------------------------------------------------------------
// Einstieg
// ---------------------------------------------------------------------------------------------

pub fn draw(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    shortcut_bar(scene, ui, engine, cx);
    if scene.show_storage {
        storage_window(scene, ui, engine, cx);
    }
    if scene.show_city {
        city_window(scene, ui, engine, cx);
    }
    if scene.mode == CommandMode::BuildMenu {
        build_panel(scene, ui, engine, cx);
    }
    if scene.selected_building.is_some_and(|b| scene.world.buildings.get(b).is_some_and(|x| x.owner == PLAYER && x.complete && cx.data.building(x.kind).market)) {
        market_window(scene, ui, engine, cx);
    }
}

/// Kleine Leiste links unter der Hauptleiste: Lager, Stadt, Bürgerzahl.
fn shortcut_bar(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &HudCtx) {
    let art = cx.ui;
    let p = &scene.world.players[PLAYER as usize];
    let (res, work, used, hap, unrest) = (p.residents.max(0.0), p.workforce, p.workers_used, p.avg_happiness, p.unrest);
    egui::Area::new(Id::unique("city_shortcuts")).anchor(Align2::LEFT_TOP, Vec2::new(12.0, TOP_H + 14.0)).show(ui.ctx(), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let r = ui_art::button(ui, art, "Lager", Vec2::new(84.0, 30.0), true);
            ui_common::sfx(engine, ui, &r);
            if ui_art::tip(r, art, "Lagerfenster [L]", "Alle Waren mit Vorrat und Lagerkapazität.").clicked() {
                scene.show_storage = !scene.show_storage;
            }
            let r = ui_art::button(ui, art, "Stadt", Vec2::new(84.0, 30.0), true);
            ui_common::sfx(engine, ui, &r);
            if ui_art::tip(r, art, "Stadtübersicht [C]", "Bürger, Steuern, Zufriedenheit und Ausbau der Siedlung.").clicked() {
                scene.show_city = !scene.show_city;
            }
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(236.0, 30.0), Sense::hover());
            let painter = ui.painter();
            painter.rect_filled(rect, 3.0, Color32::from_rgba_unmultiplied(14, 10, 8, 215));
            painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, theme::GOLD_DIM), egui::StrokeKind::Inside);
            let icon = if unrest || hap < 25.0 && res > 1.0 { "icon_sad" } else { "icon_happy" };
            ui_common::paint_sprite(painter, cx.tex, sprite(cx, "icon_workers"), Rect::from_center_size(Pos2::new(rect.min.x + 18.0, rect.center().y), Vec2::splat(22.0)), Color32::WHITE);
            shadow_text(painter, Pos2::new(rect.min.x + 34.0, rect.center().y + 1.0), Align2::LEFT_CENTER, &format!("{res:.0} Bürger · Arbeit {used}/{work}"), theme::body_bold_font(14.0), if unrest { theme::EMBER } else { theme::PARCHMENT });
            ui_common::paint_sprite(painter, cx.tex, sprite(cx, icon), Rect::from_center_size(Pos2::new(rect.max.x - 16.0, rect.center().y), Vec2::splat(22.0)), Color32::WHITE);
            ui_art::tip(resp, art, "Bürger", &format!("{res:.0} Bewohner, davon {work} Arbeitskräfte ({used} im Einsatz).\nDurchschnittliche Zufriedenheit: {hap:.0} %."));
        });
    });
}

// ---------------------------------------------------------------------------------------------
// Lagerfenster
// ---------------------------------------------------------------------------------------------

fn producers_text(data: &GameData, it: Item) -> (String, String) {
    let names = |outputs: bool| {
        let mut v: Vec<&str> = data
            .buildings
            .iter()
            .filter(|b| b.recipes.iter().any(|r| if outputs { r.outputs_r.iter().any(|(i, _)| *i == it) } else { r.inputs_r.iter().any(|(i, _)| *i == it) }))
            .map(|b| b.name.as_str())
            .collect();
        v.dedup();
        v.join(", ")
    };
    (names(true), names(false))
}

fn storage_window(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let art = cx.ui;
    let data = cx.data;
    let p = &scene.world.players[PLAYER as usize];
    let cap = p.capacity;
    let mut close = false;
    egui::Area::new(Id::unique("storage_window")).anchor(Align2::LEFT_TOP, Vec2::new(12.0, TOP_H + 54.0)).show(ui.ctx(), |ui| {
        art.panel(ui, PanelKind::Thin, egui::Margin { left: 22, right: 22, top: 18, bottom: 18 }, |ui| {
            ui.set_width(330.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Lager").font(theme::title_bold_font(20.0)).color(theme::GOLD));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (r, resp) = ui.allocate_exact_size(Vec2::splat(22.0), Sense::click());
                    art.rune(ui.painter(), r.center(), "x", resp.hovered());
                    ui_common::sfx(engine, ui, &resp);
                    if resp.clicked() {
                        close = true;
                    }
                    ui.label(RichText::new(format!("Kapazität {cap:.0} je Ware")).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
                });
            });
            ui.add_space(4.0);
            let tiers = [(1u8, "Rohstoffe"), (2, "Zwischengüter"), (3, "Fertiggüter")];
            egui::ScrollArea::vertical().max_height((ui.ctx().content_rect().height() - TOP_H - 110.0).max(300.0)).show(ui, |ui| {
                for (tier, title) in tiers {
                    ui.label(RichText::new(title).font(theme::caps_font(12.5)).color(theme::GOLD_DIM));
                    let r = ui.available_rect_before_wrap();
                    art.divider(ui.painter(), r.min.x, r.max.x, r.min.y + 3.0);
                    ui.add_space(8.0);
                    egui::Grid::new(("goods_grid", tier)).num_columns(2).spacing([14.0, 5.0]).show(ui, |ui| {
                        let mut n = 0;
                        for (i, g) in data.goods.iter().enumerate().filter(|(_, g)| g.tier == tier) {
                            let it = Item::from_index(ashen_sim::data::BASE_ITEMS + i);
                            let amount = scene.world.players[PLAYER as usize].stock(it);
                            let full = amount >= cap - 0.5;
                            ui.horizontal(|ui| {
                                let (r, resp) = ui.allocate_exact_size(Vec2::new(150.0, 26.0), Sense::hover());
                                ui_common::paint_sprite(ui.painter(), cx.tex, sprite(cx, &item_icon(data, it)), Rect::from_center_size(Pos2::new(r.min.x + 13.0, r.center().y), Vec2::splat(24.0)), Color32::WHITE);
                                let col = if full { Color32::from_rgb(120, 170, 240) } else if amount < 0.5 { theme::ASH } else { theme::PARCHMENT };
                                shadow_text(ui.painter(), Pos2::new(r.min.x + 32.0, r.center().y + 1.0), Align2::LEFT_CENTER, &g.name, theme::body_font(14.5), col);
                                shadow_text(ui.painter(), Pos2::new(r.max.x, r.center().y + 1.0), Align2::RIGHT_CENTER, &format!("{amount:.0}"), theme::body_bold_font(15.0), col);
                                let (made, used) = producers_text(data, it);
                                let price = scene.world.market_price(data, PLAYER, it);
                                ui_art::tooltip(resp, art, |ui| {
                                    ui.label(RichText::new(&g.name).font(theme::heading_font(16.0)).color(theme::GOLD));
                                    ui.label(RichText::new(&g.description).size(14.0).color(theme::PARCHMENT));
                                    if !made.is_empty() {
                                        ui.label(RichText::new(format!("Hergestellt in: {made}")).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
                                    }
                                    if !used.is_empty() {
                                        ui.label(RichText::new(format!("Verbraucht von: {used}")).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
                                    }
                                    ui.label(RichText::new(format!("Marktpreis {price:.1} Gold · Vorrat {amount:.0} / {cap:.0}")).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
                                });
                            });
                            n += 1;
                            if n % 2 == 0 {
                                ui.end_row();
                            }
                        }
                    });
                    ui.add_space(6.0);
                }
            });
        });
    });
    if close {
        scene.show_storage = false;
    }
}

// ---------------------------------------------------------------------------------------------
// Stadtübersicht
// ---------------------------------------------------------------------------------------------

fn city_window(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let art = cx.ui;
    let data = cx.data;
    let p = scene.world.players[PLAYER as usize].clone();
    let rules = &data.rules.city;
    // Bewohner je Stufe und häufigste unerfüllte Bedürfnisse.
    let mut tiers: Vec<(String, u32, f32)> = Vec::new();
    let mut unmet: Vec<(String, u32)> = Vec::new();
    for (_, b) in scene.world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.complete) {
        let d = data.building(b.kind);
        let Some(h) = &d.housing else { continue };
        match tiers.iter_mut().find(|t| t.0 == d.name) {
            Some(t) => {
                t.1 += 1;
                t.2 += b.residents;
            }
            None => tiers.push((d.name.clone(), 1, b.residents)),
        }
        for (i, n) in h.needs.iter().enumerate() {
            if b.unmet & (1 << i) != 0 {
                let name = match &n.kind {
                    ashen_sim::data::NeedKind::Item(_) => n.item_r.map(|it| data.item_name(it).to_string()).unwrap_or_default(),
                    ashen_sim::data::NeedKind::Faith => "Tempel in der Nähe".into(),
                    ashen_sim::data::NeedKind::Safety => "Schutz (Turm/Kaserne)".into(),
                };
                match unmet.iter_mut().find(|u| u.0 == name) {
                    Some(u) => u.1 += 1,
                    None => unmet.push((name, 1)),
                }
            }
        }
    }
    unmet.sort_by_key(|u| std::cmp::Reverse(u.1));
    let mut close = false;
    let mut set_tax = None;
    let mut advance = false;
    egui::Area::new(Id::unique("city_window")).anchor(Align2::LEFT_TOP, Vec2::new(12.0, TOP_H + 54.0)).show(ui.ctx(), |ui| {
        art.panel(ui, PanelKind::Thin, egui::Margin { left: 22, right: 22, top: 18, bottom: 18 }, |ui| {
            ui.set_width(360.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Stadt").font(theme::title_bold_font(20.0)).color(theme::GOLD));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (r, resp) = ui.allocate_exact_size(Vec2::splat(22.0), Sense::click());
                    art.rune(ui.painter(), r.center(), "x", resp.hovered());
                    ui_common::sfx(engine, ui, &resp);
                    if resp.clicked() {
                        close = true;
                    }
                    let era_name = if p.era == 0 { "Siedlung".to_string() } else { data.rules.eras.get(p.era as usize - 1).map(|e| e.name.clone()).unwrap_or_default() };
                    ui.label(RichText::new(format!("Ausbaustufe: {era_name}")).font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
                });
            });
            ui.add_space(6.0);
            let row = |ui: &mut Ui, k: &str, v: String, col: Color32| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(k).font(theme::caps_font(12.0)).color(theme::GOLD_DIM));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.label(RichText::new(v).font(theme::body_bold_font(15.0)).color(col)));
                });
            };
            row(ui, "Bewohner", format!("{:.0}", p.residents), theme::PARCHMENT);
            row(ui, "Arbeitskräfte", format!("{} von {} im Einsatz", p.workers_used, p.workforce), if p.workers_used < p.workforce { Color32::from_rgb(230, 200, 100) } else { theme::PARCHMENT });
            row(ui, "Zufriedenheit", format!("{:.0} %", p.avg_happiness), if p.unrest { RED } else if p.avg_happiness >= 60.0 { Color32::from_rgb(150, 210, 120) } else { theme::PARCHMENT });
            row(ui, "Steuereinnahmen", format!("{:+.2} Gold/s", p.tax_rate), theme::GOLD);
            for (name, n, r) in &tiers {
                row(ui, name, format!("{n}× · {r:.0} Bewohner"), theme::PARCHMENT_DIM);
            }
            if !unmet.is_empty() {
                ui.add_space(4.0);
                ui.label(RichText::new("Es mangelt an").font(theme::caps_font(12.0)).color(RED));
                for (name, n) in unmet.iter().take(5) {
                    ui.label(RichText::new(format!("{name}  ({n} Häuser)")).size(14.0).color(theme::PARCHMENT));
                }
            }
            ui.add_space(8.0);
            let r = ui.available_rect_before_wrap();
            art.divider(ui.painter(), r.min.x, r.max.x, r.min.y);
            ui.add_space(12.0);
            ui.label(RichText::new("Steuersatz").font(theme::caps_font(12.5)).color(theme::GOLD_DIM));
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                for (i, lvl) in rules.tax_levels.iter().enumerate() {
                    let r = ui_art::choice(ui, art, &lvl.name, p.tax_level as usize == i);
                    ui_common::sfx(engine, ui, &r);
                    let body = format!("Einnahmen ×{:.1}\nZufriedenheit {:+.0}\nZu hohe Steuern lassen die Bürger murren; bei Unruhe wandern sie ab und zahlen nichts.", lvl.income, lvl.happiness);
                    if ui_art::tip(r, art, &format!("Steuersatz: {}", lvl.name), &body).clicked() {
                        set_tax = Some(i as u8);
                    }
                }
            });
            ui.add_space(10.0);
            let r = ui.available_rect_before_wrap();
            art.divider(ui.painter(), r.min.x, r.max.x, r.min.y);
            ui.add_space(12.0);
            // Ausbau zur nächsten Stufe.
            match data.rules.eras.get(p.era as usize) {
                Some(next) => {
                    ui.label(RichText::new(format!("Ausbau zur {}", next.name)).font(theme::heading_font(16.0)).color(theme::GOLD));
                    ui.label(RichText::new(&next.description).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
                    ui.add_space(4.0);
                    for (bid, n) in &next.requires {
                        let Some(bt) = data.building_id(bid) else { continue };
                        let have = scene.world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.kind == bt && b.complete).count() as u32;
                        let ok = have >= *n;
                        ui.label(RichText::new(format!("{}× {} ({have}/{n}){}", n, data.building(bt).name, if ok { "" } else { " – fehlt" })).size(14.0).color(if ok { Color32::from_rgb(150, 210, 120) } else { RED }));
                    }
                    cost_row_full(ui, cx, &scene.world, &next.cost, &next.goods_r);
                    ui.add_space(4.0);
                    if let Some(prog) = p.era_progress {
                        let (r, _) = ui.allocate_exact_size(Vec2::new(300.0, 18.0), Sense::hover());
                        art.bar(ui.painter(), r, prog, Color32::from_rgb(200, 150, 50), &format!("Ausbau {:.0} %", prog * 100.0));
                    } else {
                        let r = ui_art::button(ui, art, &format!("Zur {} ausbauen", next.name), Vec2::new(300.0, 34.0), true);
                        ui_common::sfx(engine, ui, &r);
                        if ui_art::tip(r, art, &format!("Zur {} ausbauen", next.name), "Schaltet neue Gebäude frei und vergrößert die Lager.").clicked() {
                            advance = true;
                        }
                    }
                }
                None => {
                    ui.label(RichText::new("Die höchste Ausbaustufe ist erreicht.").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
                }
            }
        });
    });
    if let Some(l) = set_tax {
        scene.set_tax(l);
    }
    if advance {
        scene.advance_era();
    }
    if close {
        scene.show_city = false;
    }
}

// ---------------------------------------------------------------------------------------------
// Baumenü
// ---------------------------------------------------------------------------------------------

fn build_panel(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let art = cx.ui;
    let data = cx.data;
    let era = scene.world.players[PLAYER as usize].era;
    let mut pick: Option<BuildingTypeId> = None;
    let mut cat = scene.build_cat;
    let mut back = false;
    egui::Area::new(Id::unique("build_panel")).anchor(Align2::RIGHT_BOTTOM, Vec2::new(-12.0, -(CONSOLE_H + 10.0))).show(ui.ctx(), |ui| {
        art.panel(ui, PanelKind::Thin, egui::Margin { left: 20, right: 20, top: 16, bottom: 16 }, |ui| {
            ui.set_width(540.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Bauen").font(theme::title_bold_font(19.0)).color(theme::GOLD));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let r = ui_art::button(ui, art, "Zurück", Vec2::new(80.0, 28.0), true);
                    ui_common::sfx(engine, ui, &r);
                    if r.clicked() {
                        back = true;
                    }
                });
            });
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                for c in BuildCategory::ALL {
                    let r = ui_art::choice(ui, art, c.label(), cat == c);
                    ui_common::sfx(engine, ui, &r);
                    if r.clicked() {
                        cat = c;
                    }
                }
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(6.0, 6.0);
                for bt in crate::mission::player_buildables(data) {
                    let d = data.building(bt);
                    if d.category != cat {
                        continue;
                    }
                    let req_ok = d.requires.iter().all(|r| data.building_id(r).is_some_and(|rb| scene.world.buildings.iter().any(|(_, x)| x.owner == PLAYER && x.kind == rb && x.complete)));
                    let era_ok = d.era <= era;
                    let afford = scene.world.afford(PLAYER, &d.cost, &d.goods_cost_r).is_ok();
                    let enabled = era_ok && req_ok;
                    let (rect, resp) = ui.allocate_exact_size(Vec2::new(122.0, 104.0), if enabled { Sense::click() } else { Sense::hover() });
                    let st = BtnState::of(&resp, enabled);
                    let p = ui.painter();
                    art.image(p, art.slot_tex(match st { BtnState::Normal => 0, BtnState::Hover => 1, BtnState::Pressed => 2, BtnState::Disabled => 3 }, false), rect, Color32::WHITE);
                    let tint = if enabled { Color32::WHITE } else { Color32::from_gray(90) };
                    let spr = sprite(cx, &building_icon(d));
                    let scale = (60.0 / spr.size.x.max(spr.size.y)).min(1.0);
                    ui_common::paint_sprite(p, cx.tex, spr, Rect::from_center_size(rect.center() + Vec2::new(0.0, -14.0), Vec2::new(spr.size.x * scale, spr.size.y * scale)), tint);
                    shadow_text(p, Pos2::new(rect.center().x, rect.max.y - 24.0), Align2::CENTER_CENTER, &d.name, theme::body_bold_font(13.5), if enabled { theme::PARCHMENT } else { theme::ASH });
                    // Kosten als kleine Icon-Reihe am unteren Rand.
                    let mut entries: Vec<(Item, f32)> = ResourceKind::ALL.iter().filter(|k| d.cost.get(**k) > 0.0).map(|k| (Item::Res(*k), d.cost.get(*k))).collect();
                    entries.extend(d.goods_cost_r.iter().copied());
                    let w = entries.len() as f32 * 34.0;
                    let mut x = rect.center().x - w / 2.0;
                    for (it, n) in &entries {
                        ui_common::paint_sprite(p, cx.tex, sprite(cx, &item_icon(data, *it)), Rect::from_min_size(Pos2::new(x, rect.max.y - 14.0), Vec2::splat(11.0)), Color32::WHITE);
                        let have = scene.world.players[PLAYER as usize].stock(*it) >= *n;
                        p.text(Pos2::new(x + 12.0, rect.max.y - 8.5), Align2::LEFT_CENTER, format!("{n:.0}"), theme::body_font(11.5), if have { theme::PARCHMENT_DIM } else { RED });
                        x += 34.0;
                    }
                    if !era_ok {
                        shadow_text(p, rect.center() + Vec2::new(0.0, 6.0), Align2::CENTER_CENTER, "gesperrt", theme::caps_font(12.0), RED);
                    } else if !afford {
                        p.rect_stroke(rect.shrink(3.0), 0.0, egui::Stroke::new(1.0, RED.gamma_multiply(0.6)), egui::StrokeKind::Inside);
                    }
                    ui_common::sfx(engine, ui, &resp);
                    let resp = ui_art::tooltip(resp, art, |ui| {
                        ui.label(RichText::new(&d.name).font(theme::heading_font(16.0)).color(theme::GOLD));
                        ui.set_max_width(320.0);
                        ui.label(RichText::new(&d.description).size(14.0).color(theme::PARCHMENT));
                        for line in building_summary(data, d) {
                            ui.label(RichText::new(line).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
                        }
                        cost_row_full(ui, cx, &scene.world, &d.cost, &d.goods_cost_r);
                        ui.label(RichText::new(format!("Bauzeit {:.0} s", d.build_time)).font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
                        if !era_ok {
                            let name = data.rules.eras.get(d.era as usize - 1).map(|e| e.name.as_str()).unwrap_or("?");
                            ui.label(RichText::new(format!("Benötigt die Ausbaustufe {name}")).color(RED));
                        }
                        if !req_ok {
                            ui.label(RichText::new(format!("Benötigt: {}", d.requires.join(", "))).color(RED));
                        }
                        if let Some(s) = &d.site {
                            ui.label(RichText::new(&s.hint).font(theme::italic_font(13.5)).color(theme::EMBER));
                        }
                    });
                    if resp.clicked() && enabled {
                        pick = Some(bt);
                    }
                }
            });
        });
    });
    scene.build_cat = cat;
    if let Some(bt) = pick {
        scene.mode = CommandMode::Place(bt);
    }
    if back {
        scene.mode = CommandMode::Normal;
    }
}

// ---------------------------------------------------------------------------------------------
// Markt
// ---------------------------------------------------------------------------------------------

fn market_window(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let art = cx.ui;
    let data = cx.data;
    let fee = data.rules.city.market_fee;
    let mut trade: Option<(Item, i32)> = None;
    egui::Area::new(Id::unique("market_window")).anchor(Align2::RIGHT_BOTTOM, Vec2::new(-12.0, -(CONSOLE_H + 10.0))).show(ui.ctx(), |ui| {
        art.panel(ui, PanelKind::Thin, egui::Margin { left: 20, right: 20, top: 16, bottom: 16 }, |ui| {
            ui.set_width(500.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Markt").font(theme::title_bold_font(19.0)).color(theme::GOLD));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("Kauf +{:.0} %, Verkauf -{:.0} % auf den Marktpreis", fee * 100.0, fee * 100.0)).font(theme::italic_font(13.0)).color(theme::PARCHMENT_DIM));
                });
            });
            ui.add_space(4.0);
            egui::ScrollArea::vertical().max_height(340.0).show(ui, |ui| {
                let items: Vec<Item> = [ResourceKind::Wood, ResourceKind::Stone, ResourceKind::Food].into_iter().map(Item::Res).chain((0..data.goods.len()).map(|i| Item::from_index(ashen_sim::data::BASE_ITEMS + i))).collect();
                for it in items {
                    let stock = scene.world.players[PLAYER as usize].stock(it);
                    let price = scene.world.market_price(data, PLAYER, it);
                    let base = data.base_price(it);
                    let trend = price / base;
                    let col = if trend > 1.05 { Color32::from_rgb(230, 200, 100) } else if trend < 0.95 { Color32::from_rgb(130, 190, 240) } else { theme::PARCHMENT };
                    ui.horizontal(|ui| {
                        let (r, resp) = ui.allocate_exact_size(Vec2::new(220.0, 26.0), Sense::hover());
                        ui_common::paint_sprite(ui.painter(), cx.tex, sprite(cx, &item_icon(data, it)), Rect::from_center_size(Pos2::new(r.min.x + 13.0, r.center().y), Vec2::splat(22.0)), Color32::WHITE);
                        shadow_text(ui.painter(), Pos2::new(r.min.x + 30.0, r.center().y), Align2::LEFT_CENTER, data.item_name(it), theme::body_font(14.0), theme::PARCHMENT);
                        shadow_text(ui.painter(), Pos2::new(r.min.x + 128.0, r.center().y), Align2::LEFT_CENTER, &format!("{stock:.0}"), theme::body_bold_font(14.0), theme::PARCHMENT_DIM);
                        shadow_text(ui.painter(), Pos2::new(r.max.x, r.center().y), Align2::RIGHT_CENTER, &format!("{:.1}{}", price, if trend > 1.05 { " (teuer)" } else if trend < 0.95 { " (billig)" } else { "" }), theme::body_bold_font(14.0), col);
                        ui_art::tip(resp, art, data.item_name(it), &format!("Marktpreis {price:.1} Gold (Grundpreis {base:.1}).\nKaufen: {:.1} · Verkaufen: {:.1} Gold je Stück.", price * (1.0 + fee), price * (1.0 - fee)));
                        for (label, amt) in [("-10", -10), ("-1", -1), ("+1", 1), ("+10", 10)] {
                            let ok = if amt < 0 { stock >= 1.0 } else { scene.world.players[PLAYER as usize].resources.gold >= price * (1.0 + fee) };
                            let r = ui_art::button(ui, art, label, Vec2::new(48.0, 24.0), ok);
                            ui_common::sfx(engine, ui, &r);
                            if r.clicked() {
                                trade = Some((it, amt));
                            }
                        }
                    });
                }
            });
        });
    });
    if let Some((it, n)) = trade {
        scene.trade(it, n);
    }
}

// ---------------------------------------------------------------------------------------------
// Betriebsansicht im Auswahlfeld
// ---------------------------------------------------------------------------------------------

/// Zeilen zu Produktion, Arbeitern, Bewohnern und Tor eines gewählten eigenen Gebäudes.
pub fn building_info(scene: &mut MissionScene, ui: &mut Ui, engine: &mut Engine, cx: &HudCtx, bid: BuildingId, b: &Building) {
    let art = cx.ui;
    let data = cx.data;
    let def = data.building(b.kind);
    if b.owner != PLAYER || !b.complete {
        return;
    }
    if let Some(h) = &def.housing {
        let tier = ["Hütte", "Haus", "Bürgerhaus"].get(h.tier as usize).copied().unwrap_or("Haus");
        let col = if b.happiness < 20.0 { RED } else if b.happiness >= 70.0 { Color32::from_rgb(150, 210, 120) } else { theme::PARCHMENT };
        ui.label(RichText::new(format!("{tier} · Bewohner {:.1}/{} · Zufriedenheit {:.0} %", b.residents, h.capacity, b.happiness)).size(14.5).color(col));
        let mut missing = Vec::new();
        for (i, n) in h.needs.iter().enumerate() {
            if b.unmet & (1 << i) != 0 {
                missing.push(match &n.kind {
                    ashen_sim::data::NeedKind::Item(_) => n.item_r.map(|it| data.item_name(it).to_string()).unwrap_or_default(),
                    ashen_sim::data::NeedKind::Faith => "Tempel".into(),
                    ashen_sim::data::NeedKind::Safety => "Schutz".into(),
                });
            }
        }
        if !missing.is_empty() {
            ui.label(RichText::new(format!("Es mangelt an: {}", missing.join(", "))).size(14.0).color(RED));
        } else if h.upgrade_to.is_some() {
            ui.label(RichText::new("Alle Bedürfnisse erfüllt – das Haus baut sich bei Zufriedenheit aus.").font(theme::italic_font(13.5)).color(theme::PARCHMENT_DIM));
        }
        return;
    }
    if def.employs() {
        let recipe = &def.recipes[(b.recipe as usize).min(def.recipes.len() - 1)];
        ui.label(RichText::new(recipe_text(data, recipe)).size(14.0).color(theme::PARCHMENT));
        let (text, col) = status_text(data, b);
        ui.label(RichText::new(text).size(14.5).color(col));
        ui.horizontal(|ui| {
            ui.label(RichText::new("Arbeiter").font(theme::caps_font(11.5)).color(theme::GOLD_DIM));
            let limit = b.worker_limit.unwrap_or(def.workers).min(def.workers);
            let minus = ui_art::button(ui, art, "-", Vec2::new(28.0, 24.0), limit > 0);
            ui_common::sfx(engine, ui, &minus);
            if minus.clicked() {
                scene.set_workers(bid, Some(limit - 1));
            }
            ui.label(RichText::new(format!("{} / {}", b.workers, def.workers)).font(theme::body_bold_font(15.0)).color(if b.workers < limit { RED } else { theme::PARCHMENT }));
            let plus = ui_art::button(ui, art, "+", Vec2::new(28.0, 24.0), limit < def.workers);
            ui_common::sfx(engine, ui, &plus);
            if plus.clicked() {
                scene.set_workers(bid, if limit + 1 >= def.workers { None } else { Some(limit + 1) });
            }
            if def.recipes.len() > 1 {
                ui.add_space(10.0);
                for (i, r) in def.recipes.iter().enumerate() {
                    let resp = ui_art::choice(ui, art, &r.name, b.recipe as usize == i);
                    ui_common::sfx(engine, ui, &resp);
                    if ui_art::tip(resp, art, &r.name, &recipe_text(data, r)).clicked() {
                        scene.set_recipe(bid, i as u8);
                    }
                }
            }
        });
        return;
    }
    if def.fort == Some(ashen_sim::data::Fort::Gate) {
        ui.horizontal(|ui| {
            let label = if b.gate_open { "Tor verschließen" } else { "Tor öffnen" };
            let r = ui_art::button(ui, art, label, Vec2::new(160.0, 28.0), true);
            ui_common::sfx(engine, ui, &r);
            if ui_art::tip(r, art, label, "Offen: eigene Einheiten passieren, Feinde nie. Verschlossen: niemand kommt durch.").clicked() {
                scene.set_gate(bid, !b.gate_open);
            }
            ui.label(RichText::new(if b.gate_open { "Offen für eigene Einheiten" } else { "Verschlossen" }).size(14.0).color(theme::PARCHMENT));
        });
        return;
    }
    if def.storage > 0.0 {
        ui.label(RichText::new(format!("Lagert {:.0} je Ware. Betriebe im Umkreis von {:.0} Kacheln liefern hierher.", def.storage, data.rules.city.logistics_radius)).size(14.0).color(theme::PARCHMENT));
    }
    if def.market {
        ui.label(RichText::new("Handel im Marktfenster rechts: Waren kaufen und verkaufen.").size(14.0).color(theme::PARCHMENT));
    }
}
