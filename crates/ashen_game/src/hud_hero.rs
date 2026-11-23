//! Heldenleiste und Heldenfenster (Taste H oder Klick auf das Porträt): Werte, Talentbaum,
//! Ausrüstung (Waffe, Rüstung, Amulett, zwei Ringe), Inventar und Tränke.

use crate::hud::{HudCtx, TOP_H, shadow_text, sprite};
use crate::mission::MissionScene;
use crate::ui_art::{self, BtnState, PanelKind};
use crate::ui_common;
use ashen_engine::Engine;
use ashen_engine::ui::theme;
use ashen_sim::data::{GameData, UnitClass};
use ashen_sim::entity::{PLAYER, Unit, UnitId};
use ashen_sim::rpg::{EQUIP_LABELS, EQUIP_SLOTS, INVENTORY_SIZE, ItemId, ItemSlot, StatMods};
use ashen_sim::systems::heroes;
use egui::{Align2, Color32, Id, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Ui, Vec2};

const CELL: f32 = 46.0;

pub fn draw(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    hero_bar(scene, ui, engine, cx);
    if scene.show_hero {
        hero_window(scene, ui, engine, cx);
    }
}

fn rarity_color(data: &GameData, id: ItemId) -> Color32 {
    let [r, g, b] = data.item_def(id).rarity.color();
    Color32::from_rgb(r, g, b)
}

/// Zahl mit deutschem Dezimalkomma, ohne überflüssige Nachkommastellen.
fn num(v: f32) -> String {
    if (v - v.round()).abs() < 0.005 {
        format!("{v:.0}")
    } else {
        format!("{v:.1}").replace('.', ",")
    }
}

/// Wertezeilen einer Wertänderung („+10 Schaden“).
pub fn mod_lines(m: &StatMods) -> Vec<String> {
    let mut v = Vec::new();
    let mut add = |x: f32, f: &dyn Fn(f32) -> String| {
        if x.abs() > 1e-4 {
            v.push(f(x));
        }
    };
    add(m.hp, &|x| format!("{:+} Leben", x.round() as i32));
    add(m.mana, &|x| format!("{:+} Mana", x.round() as i32));
    add(m.damage, &|x| format!("{:+} Schaden", x.round() as i32));
    add(m.damage_pct, &|x| {
        format!("{:+} % Schaden", (x * 100.0).round() as i32)
    });
    add(m.armor, &|x| format!("{:+} Rüstung", x.round() as i32));
    add(m.speed_pct, &|x| {
        format!("{:+} % Tempo", (x * 100.0).round() as i32)
    });
    add(m.hp_regen, &|x| {
        format!("{}{} Leben/s", if x > 0.0 { "+" } else { "" }, num(x))
    });
    add(m.mana_regen, &|x| {
        format!("{}{} Mana/s", if x > 0.0 { "+" } else { "" }, num(x))
    });
    add(m.xp_pct, &|x| {
        format!("{:+} % Erfahrung", (x * 100.0).round() as i32)
    });
    add(m.cooldown_pct, &|x| {
        format!("−{} % Abklingzeit", (x * 100.0).round() as i32)
    });
    v
}

fn item_icon_sprite(cx: &HudCtx, id: ItemId) -> ashen_engine::render::SpriteRect {
    let d = cx.data.item_def(id);
    if let Some(r) = cx.art.atlas.try_get(&format!("icon_item_{}", d.id)) {
        return r;
    }
    let slot = match d.slot {
        ItemSlot::Weapon => "weapon",
        ItemSlot::Armor => "armor",
        ItemSlot::Amulet => "amulet",
        ItemSlot::Ring => "ring",
        ItemSlot::Potion => "potion",
    };
    sprite(cx, &format!("icon_itemslot_{slot}"))
}

fn item_tooltip(ui: &mut Ui, cx: &HudCtx, id: ItemId, hint: &str) {
    let d = cx.data.item_def(id);
    ui.set_max_width(300.0);
    ui.label(
        RichText::new(&d.name)
            .font(theme::heading_font(16.0))
            .color(rarity_color(cx.data, id)),
    );
    ui.label(
        RichText::new(format!("{} · {}", d.slot.label(), d.rarity.label()))
            .font(theme::caps_font(11.5))
            .color(theme::GOLD_DIM),
    );
    ui.label(
        RichText::new(&d.description)
            .font(theme::italic_font(13.5))
            .color(theme::PARCHMENT_DIM),
    );
    for l in mod_lines(&d.mods) {
        ui.label(
            RichText::new(l)
                .size(14.0)
                .color(Color32::from_rgb(150, 220, 140)),
        );
    }
    if let Some(u) = d.use_effect {
        if u.heal > 0.0 {
            ui.label(
                RichText::new(format!("Stellt {} Leben her", num(u.heal)))
                    .size(14.0)
                    .color(Color32::from_rgb(150, 220, 140)),
            );
        }
        if u.mana > 0.0 {
            ui.label(
                RichText::new(format!("Stellt {} Mana her", num(u.mana)))
                    .size(14.0)
                    .color(Color32::from_rgb(120, 170, 250)),
            );
        }
        if u.duration > 0.0 {
            let mut parts = Vec::new();
            if u.damage_mult > 1.0 {
                parts.push(format!(
                    "+{} % Schaden",
                    ((u.damage_mult - 1.0) * 100.0).round()
                ));
            }
            if u.armor_bonus > 0.0 {
                parts.push(format!("+{} Rüstung", num(u.armor_bonus)));
            }
            ui.label(
                RichText::new(format!("{} für {} s", parts.join(", "), u.duration))
                    .size(14.0)
                    .color(Color32::from_rgb(150, 220, 140)),
            );
        }
    }
    ui.label(
        RichText::new(format!("Wert {} Gold", num(d.value)))
            .font(theme::italic_font(12.5))
            .color(theme::PARCHMENT_DIM),
    );
    if !hint.is_empty() {
        ui.label(
            RichText::new(hint)
                .font(theme::italic_font(12.5))
                .color(theme::EMBER_HOT),
        );
    }
}

/// Eine Gegenstandskachel: leerer Platz oder Gegenstand mit Seltenheitsrahmen.
fn item_cell(
    ui: &mut Ui,
    engine: &mut Engine,
    cx: &HudCtx,
    item: Option<ItemId>,
    hint: &str,
    empty_hint: &str,
) -> egui::Response {
    let art = cx.ui;
    let (rect, resp) = ui.allocate_exact_size(
        Vec2::splat(CELL),
        if item.is_some() {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let st = BtnState::of(&resp, true);
    let p = ui.painter();
    art.image(
        p,
        art.slot_tex(
            if st == BtnState::Hover && item.is_some() {
                1
            } else {
                0
            },
            false,
        ),
        rect,
        Color32::WHITE,
    );
    if let Some(id) = item {
        ui_common::paint_sprite(
            p,
            cx.tex,
            item_icon_sprite(cx, id),
            Rect::from_center_size(rect.center(), Vec2::splat(CELL - 14.0)),
            Color32::WHITE,
        );
        p.rect_stroke(
            rect.shrink(4.0),
            2.0,
            Stroke::new(1.5, rarity_color(cx.data, id)),
            StrokeKind::Inside,
        );
        ui_common::sfx(engine, ui, &resp);
        return ui_art::tooltip(resp, art, |ui| item_tooltip(ui, cx, id, hint));
    }
    if !empty_hint.is_empty() {
        shadow_text(
            p,
            rect.center(),
            Align2::CENTER_CENTER,
            empty_hint,
            theme::caps_font(9.5),
            Color32::from_rgb(120, 104, 80),
        );
    }
    resp
}

// ---------------------------------------------------------------------------------------------
// Heldenleiste
// ---------------------------------------------------------------------------------------------

fn hero_bar(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &HudCtx) {
    let art = cx.ui;
    let data = cx.data;
    let mut heroes_alive: Vec<(UnitId, Unit)> = scene
        .world
        .units
        .iter()
        .filter(|(_, u)| u.owner == PLAYER && data.unit(u.kind).class == UnitClass::Hero)
        .map(|(id, u)| (id, u.clone()))
        .collect();
    heroes_alive.sort_by_key(|(id, _)| id.index);
    let fallen: Vec<(String, f32, u32)> = scene
        .world
        .fallen
        .iter()
        .map(|f| (f.unit.kind, f.timer, f.unit.level))
        .map(|(k, t, l)| (data.unit(k).id.clone(), t, l))
        .collect();
    if heroes_alive.is_empty() && fallen.is_empty() {
        return;
    }
    let mut pick: Option<UnitId> = None;
    egui::Area::new(Id::unique("hero_bar")).anchor(Align2::LEFT_TOP, Vec2::new(12.0, TOP_H + 50.0)).show(ui.ctx(), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            for (id, u) in &heroes_alive {
                let def = data.unit(u.kind);
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(CELL, CELL + 6.0), Sense::click());
                let selected = scene.selection.contains(id);
                let p = ui.painter();
                let slot = Rect::from_min_size(rect.min, Vec2::splat(CELL));
                if selected || resp.hovered() {
                    art.glow(p, slot.expand(8.0), Color32::from_rgb(90, 50, 10));
                }
                art.image(p, art.slot_tex(if resp.hovered() { 1 } else { 0 }, false), slot, Color32::WHITE);
                ui_common::paint_sprite(p, cx.tex, cx.art.portrait(&def.id), slot.shrink(5.0), Color32::WHITE);
                if selected {
                    p.rect_stroke(slot.shrink(2.0), 2.0, Stroke::new(1.5, Color32::from_rgb(150, 236, 150)), StrokeKind::Inside);
                }
                let frac = (u.hp / u.max_hp).clamp(0.0, 1.0);
                let bar = Rect::from_min_size(Pos2::new(rect.min.x + 4.0, rect.max.y - 5.0), Vec2::new(CELL - 8.0, 4.0));
                p.rect_filled(bar, 0.0, Color32::from_rgb(20, 8, 6));
                p.rect_filled(Rect::from_min_size(bar.min, Vec2::new(bar.width() * frac, 4.0)), 0.0, if frac > 0.5 { theme::LIFE } else { theme::BLOOD_BRIGHT });
                shadow_text(p, slot.right_top() + Vec2::new(-4.0, 9.0), Align2::RIGHT_CENTER, &format!("{}", u.level), theme::body_bold_font(13.0), Color32::from_rgb(240, 200, 90));
                let points = heroes::free_points(data, u);
                if points > 0 {
                    shadow_text(p, slot.left_top() + Vec2::new(8.0, 9.0), Align2::CENTER_CENTER, "+", theme::body_bold_font(16.0), Color32::from_rgb(150, 236, 150));
                }
                ui_common::sfx(engine, ui, &resp);
                let hint = if points > 0 { format!("Klick: auswählen · H: Heldenfenster · {points} Talentpunkt(e) frei") } else { "Klick: auswählen · erneut oder H: Heldenfenster".to_string() };
                if ui_art::tip(resp, art, &format!("{} · Stufe {}", def.name, u.level), &hint).clicked() {
                    pick = Some(*id);
                }
            }
            for (id, timer, level) in &fallen {
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(CELL, CELL + 6.0), Sense::hover());
                let p = ui.painter();
                let slot = Rect::from_min_size(rect.min, Vec2::splat(CELL));
                art.image(p, art.slot_tex(3, false), slot, Color32::WHITE);
                ui_common::paint_sprite(p, cx.tex, cx.art.portrait(id), slot.shrink(5.0), Color32::from_gray(70));
                shadow_text(p, slot.center(), Align2::CENTER_CENTER, &format!("{:.0}", timer.max(0.0)), theme::body_bold_font(15.0), Color32::from_rgb(230, 90, 70));
                let name = cx.data.unit_id(id).map(|u| cx.data.unit(u).name.clone()).unwrap_or_default();
                ui_art::tip(resp, art, &format!("{name} ist gefallen"), &format!("Stufe {level}. Rückkehr im Bergfried in {timer:.0} s, oder sofort am Altar oder Bergfried gegen Gold."));
            }
        });
    });
    if let Some(id) = pick {
        let only = scene.selection.len() == 1 && scene.selection[0] == id;
        scene.selection = vec![id];
        scene.selected_building = None;
        scene.hero_focus = Some(id);
        if only {
            scene.show_hero = !scene.show_hero;
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Heldenfenster
// ---------------------------------------------------------------------------------------------

fn hero_window(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let art = cx.ui;
    let data = cx.data;
    let Some(hid) = scene.active_hero(data) else {
        scene.show_hero = false;
        return;
    };
    let Some(u) = scene.world.units.get(hid).cloned() else {
        return;
    };
    let def = data.unit(u.kind);
    let tree = data.talent_tree(&def.id).cloned();
    let free = heroes::free_points(data, &u);
    let world_armor = scene.world.armor_of(data, &u);
    let dmg_factor = scene.world.damage_factor(data, &u);
    let mut close = false;
    let mut learn: Option<usize> = None;
    let mut equip: Option<usize> = None;
    let mut unequip: Option<usize> = None;
    let mut drop: Option<usize> = None;
    let mut use_it: Option<usize> = None;
    let ability_names: Vec<String> = heroes::ability_list(data, &u).cloned().collect();

    egui::Area::new(Id::unique("hero_window")).anchor(Align2::CENTER_CENTER, Vec2::new(0.0, -26.0)).show(ui.ctx(), |ui| {
        art.panel(ui, PanelKind::Ornate, egui::Margin { left: 30, right: 30, top: 24, bottom: 24 }, |ui| {
            ui.set_width(800.0);
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::splat(54.0), Sense::hover());
                art.image(ui.painter(), art.slot_tex(0, false), r, Color32::WHITE);
                ui_common::paint_sprite(ui.painter(), cx.tex, cx.art.portrait(&def.id), r.shrink(5.0), Color32::WHITE);
                ui.vertical(|ui| {
                    ui.label(RichText::new(&def.name).font(theme::title_bold_font(22.0)).color(theme::GOLD));
                    let levels = &data.rules.xp_levels;
                    let (lo, hi) = (levels.get(u.level as usize - 1).copied().unwrap_or(0), levels.get(u.level as usize).copied().unwrap_or(u.xp.max(1)));
                    let max = u.level as usize >= levels.len();
                    let xp_text = if max { "Höchste Stufe erreicht".to_string() } else { format!("Erfahrung {} / {}", u.xp, hi) };
                    ui.label(RichText::new(format!("Stufe {} · {}", u.level, xp_text)).font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
                    let (br, _) = ui.allocate_exact_size(Vec2::new(250.0, 14.0), Sense::hover());
                    art.bar(ui.painter(), br, if max { 1.0 } else { (u.xp - lo) as f32 / (hi - lo).max(1) as f32 }, Color32::from_rgb(200, 160, 60), "");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    let (r, resp) = ui.allocate_exact_size(Vec2::splat(24.0), Sense::click());
                    art.rune(ui.painter(), r.center(), "x", resp.hovered());
                    ui_common::sfx(engine, ui, &resp);
                    if ui_art::tip(resp, art, "Schließen", "Taste H oder Esc").clicked() {
                        close = true;
                    }
                    ui.add_space(14.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("Talentpunkte: {free}")).font(theme::heading_font(16.0)).color(if free > 0 { Color32::from_rgb(150, 236, 150) } else { theme::PARCHMENT_DIM }));
                        ui.label(RichText::new("Ein Punkt je Stufe · Reihe 2 ab 2, Reihe 3 ab 4 Punkten im Pfad").font(theme::italic_font(12.5)).color(theme::PARCHMENT_DIM));
                    });
                });
            });
            let r = ui.available_rect_before_wrap();
            art.divider(ui.painter(), r.min.x, r.max.x, r.min.y + 6.0);
            ui.add_space(14.0);
            ui.horizontal_top(|ui| {
                // ---- Werte und Fähigkeiten
                ui.vertical(|ui| {
                    ui.set_width(210.0);
                    ui.label(RichText::new("Werte").font(theme::caps_font(12.5)).color(theme::GOLD_DIM));
                    egui::Grid::new("hero_stats").num_columns(2).spacing([14.0, 3.0]).show(ui, |ui| {
                        let level_dmg = u.level.saturating_sub(1) as f32 * def.damage_per_level + u.mods.damage;
                        let rows = [
                            ("Leben", format!("{:.0} / {:.0}", u.hp.max(0.0), u.max_hp)),
                            ("Mana", format!("{:.0} / {:.0}", u.mana, u.max_mana)),
                            ("Schaden", format!("{:.0}–{:.0}", (def.damage_min + level_dmg) * dmg_factor, (def.damage_max + level_dmg) * dmg_factor)),
                            ("Rüstung", format!("{world_armor:.1}")),
                            ("Tempo", format!("{:.1}", def.speed * scene.world.speed_factor(&u))),
                            ("Leben/s", format!("{:.1}", def.hp_regen + u.mods.hp_regen)),
                            ("Mana/s", format!("{:.1}", def.mana_regen + u.mods.mana_regen)),
                        ];
                        for (k, v) in rows {
                            ui.label(RichText::new(k).font(theme::caps_font(11.5)).color(theme::GOLD_DIM));
                            ui.label(RichText::new(v).size(14.5).color(theme::PARCHMENT));
                            ui.end_row();
                        }
                    });
                    ui.add_space(10.0);
                    ui.label(RichText::new("Fähigkeiten").font(theme::caps_font(12.5)).color(theme::GOLD_DIM));
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::splat(4.0);
                        for name in &ability_names {
                            let Some(aid) = data.ability_id(name) else { continue };
                            let ad = data.ability(aid);
                            let (r, resp) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
                            art.image(ui.painter(), art.slot_tex(0, true), r, Color32::WHITE);
                            ui_common::paint_sprite(ui.painter(), cx.tex, sprite(cx, &format!("icon_{}", ad.id)), Rect::from_center_size(r.center(), Vec2::splat(28.0)), Color32::WHITE);
                            ui_art::tooltip(resp, art, |ui| {
                                ui.set_max_width(300.0);
                                ui.label(RichText::new(&ad.name).font(theme::heading_font(16.0)).color(theme::GOLD));
                                ui.label(RichText::new(&ad.description).size(14.0).color(theme::PARCHMENT));
                                ui.label(RichText::new(format!("Mana {:.0} · Abklingzeit {:.0} s", ad.mana, ad.cooldown)).font(theme::italic_font(13.0)).color(theme::PARCHMENT_DIM));
                            });
                        }
                    });
                });
                ui.add_space(16.0);
                // ---- Talentbaum
                ui.vertical(|ui| {
                    ui.set_width(340.0);
                    ui.label(RichText::new("Talente").font(theme::caps_font(12.5)).color(theme::GOLD_DIM));
                    if let Some(tree) = &tree {
                        ui.horizontal_top(|ui| {
                            ui.spacing_mut().item_spacing.x = 10.0;
                            for path in 0..3u8 {
                                ui.vertical(|ui| {
                                    ui.set_width(104.0);
                                    let spent = tree.spent_in_path(&u.talents, path);
                                    let name = tree.paths.get(path as usize).cloned().unwrap_or_default();
                                    ui.label(RichText::new(format!("{name} ({spent})")).font(theme::body_bold_font(13.0)).color(theme::PARCHMENT));
                                    for tier in 0..3u8 {
                                        for (ti, t) in tree.talents.iter().enumerate().filter(|(_, t)| t.path == path && t.tier == tier) {
                                            let rank = u.talents.get(ti).copied().unwrap_or(0);
                                            let need = ashen_sim::rpg::TalentTree::required_points(t);
                                            let unlocked = spent >= need;
                                            let can = free > 0 && rank < t.max_rank && unlocked;
                                            let (r, resp) = ui.allocate_exact_size(Vec2::new(104.0, 54.0), if can { Sense::click() } else { Sense::hover() });
                                            let st = BtnState::of(&resp, can || rank > 0);
                                            let p = ui.painter();
                                            let ic = Rect::from_min_size(r.min + Vec2::new(2.0, 3.0), Vec2::splat(46.0));
                                            if can {
                                                art.glow(p, ic.expand(8.0), Color32::from_rgb(20, 70, 30));
                                            }
                                            art.image(p, art.slot_tex(if st == BtnState::Hover { 1 } else { 0 }, false), ic, Color32::WHITE);
                                            let icon = t.ability.as_ref().map(|a| format!("icon_{a}")).unwrap_or_else(|| "icon_talent".into());
                                            let tint = if rank > 0 { Color32::WHITE } else if unlocked { Color32::from_gray(190) } else { Color32::from_gray(80) };
                                            ui_common::paint_sprite(p, cx.tex, sprite(cx, &icon), Rect::from_center_size(ic.center(), Vec2::splat(30.0)), tint);
                                            if rank > 0 {
                                                p.rect_stroke(ic.shrink(3.0), 2.0, Stroke::new(1.5, Color32::from_rgb(230, 190, 80)), StrokeKind::Inside);
                                            }
                                            let col = if rank >= t.max_rank { Color32::from_rgb(240, 200, 90) } else if unlocked { theme::PARCHMENT } else { theme::ASH };
                                            shadow_text(p, Pos2::new(ic.max.x + 6.0, ic.center().y - 8.0), Align2::LEFT_CENTER, &format!("{rank}/{}", t.max_rank), theme::body_bold_font(15.0), col);
                                            if t.ability.is_some() {
                                                shadow_text(p, Pos2::new(ic.max.x + 6.0, ic.center().y + 9.0), Align2::LEFT_CENTER, "Fähigkeit", theme::caps_font(9.5), Color32::from_rgb(130, 200, 250));
                                            }
                                            ui_common::sfx(engine, ui, &resp);
                                            let resp = ui_art::tooltip(resp, art, |ui| {
                                                ui.set_max_width(300.0);
                                                ui.label(RichText::new(&t.name).font(theme::heading_font(16.0)).color(theme::GOLD));
                                                ui.label(RichText::new(&t.description).size(14.0).color(theme::PARCHMENT));
                                                ui.label(RichText::new(format!("Rang {rank} / {}", t.max_rank)).font(theme::italic_font(13.0)).color(theme::PARCHMENT_DIM));
                                                if !unlocked {
                                                    ui.label(RichText::new(format!("Benötigt {need} Punkte in „{name}“")).font(theme::italic_font(13.0)).color(Color32::from_rgb(230, 100, 80)));
                                                } else if can {
                                                    ui.label(RichText::new("Klick: Talentpunkt einsetzen").font(theme::italic_font(13.0)).color(Color32::from_rgb(150, 236, 150)));
                                                }
                                            });
                                            if resp.clicked() && can {
                                                learn = Some(ti);
                                            }
                                        }
                                    }
                                });
                            }
                        });
                    } else {
                        ui.label(RichText::new("Dieser Held besitzt keinen Talentbaum.").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
                    }
                });
                ui.add_space(16.0);
                // ---- Ausrüstung und Inventar
                ui.vertical(|ui| {
                    ui.set_width(CELL * 4.0 + 24.0);
                    ui.label(RichText::new("Ausrüstung").font(theme::caps_font(12.5)).color(theme::GOLD_DIM));
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::splat(6.0);
                        for (i, label) in EQUIP_LABELS.iter().enumerate().take(EQUIP_SLOTS) {
                            ui.vertical(|ui| {
                                let resp = item_cell(ui, engine, cx, u.equipment[i], "Klick: ablegen", label);
                                if resp.clicked() {
                                    unequip = Some(i);
                                }
                            });
                        }
                    });
                    ui.add_space(10.0);
                    ui.label(RichText::new(format!("Inventar ({}/{})", u.inventory.len(), INVENTORY_SIZE)).font(theme::caps_font(12.5)).color(theme::GOLD_DIM));
                    egui::Grid::new("hero_inventory").spacing([6.0, 6.0]).show(ui, |ui| {
                        for i in 0..INVENTORY_SIZE {
                            let item = u.inventory.get(i).copied();
                            let potion = item.is_some_and(|it| data.item_def(it).slot == ItemSlot::Potion);
                            let hint = if potion { "Klick: trinken · Rechtsklick: ablegen" } else { "Klick: anlegen · Rechtsklick: wegwerfen" };
                            let resp = item_cell(ui, engine, cx, item, hint, "");
                            if resp.clicked() {
                                if potion { use_it = Some(i) } else { equip = Some(i) }
                            }
                            if resp.secondary_clicked() {
                                drop = Some(i);
                            }
                            if i % 4 == 3 {
                                ui.end_row();
                            }
                        }
                    });
                });
            });
        });
    });
    if let Some(t) = learn {
        scene.learn_talent(hid, t);
    }
    if let Some(i) = equip {
        scene.equip_item(hid, i);
    }
    if let Some(i) = unequip {
        scene.unequip_item(hid, i);
    }
    if let Some(i) = drop {
        scene.drop_item(hid, i);
    }
    if let Some(i) = use_it {
        scene.use_item(hid, i);
    }
    if close {
        scene.show_hero = false;
    }
}
