//! Ingame-Oberfläche: Ressourcenleiste, Minimap, Auswahl, Befehlskarte, Ziele, Nachrichten,
//! Pausenmenü sowie Sieg-/Niederlagenbildschirm.

use crate::art::ArtLibrary;
use crate::mission::{CommandMode, MissionScene, SceneAction};
use crate::settings::{saves_dir, UserSettings};
use crate::ui_common::{self, atlas_image};
use ashen_core::iso::{tile_to_world, world_to_tile};
use ashen_engine::ui::theme;
use ashen_engine::Engine;
use ashen_sim::data::{GameData, ResourceKind, UnitClass};
use ashen_sim::entity::{NodeKind, PLAYER};
use ashen_sim::fog::{EXPLORED, UNEXPLORED};
use ashen_sim::map::Terrain;
use ashen_sim::mission::{ObjectiveState, Outcome};
use egui::{Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Vec2};

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
    pub settings: &'a mut UserSettings,
}

pub fn draw(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    top_bar(scene, ui, cx);
    bottom_panel(scene, ui, engine, cx);
    objectives(scene, ui, cx);
    messages(scene, ui);
    if let Some(start) = scene.drag_start {
        let ppp = ui.ctx().pixels_per_point();
        let a = Pos2::new(start.x / ppp, start.y / ppp);
        let b = Pos2::new(engine.input.mouse.x / ppp, engine.input.mouse.y / ppp);
        let r = Rect::from_two_pos(a, b);
        if r.width() > 3.0 || r.height() > 3.0 {
            let painter = ui.ctx().layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::unique("selbox")));
            painter.rect_filled(r, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(120, 220, 120, 24));
            painter.rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.0, Color32::from_rgb(140, 230, 140)), StrokeKind::Inside);
        }
    }
    mode_hint(scene, ui);
    if scene.menu_open {
        pause_menu(scene, ui, engine, cx);
    }
    if let Some(o) = scene.world.mission.outcome {
        outcome(scene, ui, cx, o);
    }
}

fn resource(ui: &mut egui::Ui, cx: &HudCtx, icon: &str, value: String, tooltip: &str, warn: bool) {
    ui.add(atlas_image(cx.art, cx.tex, icon, 22.0));
    let col = if warn { theme::EMBER } else { theme::PARCHMENT };
    ui.label(RichText::new(value).color(col).strong().size(15.0)).on_hover_text(tooltip);
    ui.add_space(10.0);
}

fn top_bar(scene: &mut MissionScene, ui: &mut egui::Ui, cx: &mut HudCtx) {
    let p = &scene.world.players[PLAYER as usize];
    let res = p.resources;
    let (pop, cap) = (p.pop, p.pop_cap);
    let starving = p.starving;
    let food_rate: f32 = scene.world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.complete).map(|(_, b)| cx.data.building(b.kind).food_per_sec).sum::<f32>()
        - scene.world.units.iter().filter(|(_, u)| u.owner == PLAYER).map(|(_, u)| cx.data.unit(u.kind).upkeep).sum::<f32>();
    egui::Panel::top("topbar").frame(theme::panel_frame().corner_radius(CornerRadius::ZERO)).show(ui, |ui| {
        ui.horizontal(|ui| {
            resource(ui, cx, "icon_gold", format!("{}", res.gold.floor()), "Gold – aus der Mine, Abgaben des Bergfrieds", false);
            resource(ui, cx, "icon_wood", format!("{}", res.wood.floor()), "Holz – von Holzfällern", false);
            resource(ui, cx, "icon_stone", format!("{}", res.stone.floor()), "Stein – aus dem Steinbruch", false);
            resource(ui, cx, "icon_food", format!("{} ({:+.1}/s)", res.food.floor(), food_rate), "Nahrung – Gehöfte erzeugen sie, jede Einheit verbraucht sie", starving || food_rate < 0.0 && res.food < 60.0);
            resource(ui, cx, "icon_pop", format!("{pop}/{cap}"), "Bevölkerung / Wohnraum (Gehöfte erhöhen den Wohnraum)", pop >= cap);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Menü").on_hover_text("Pausenmenü (Esc)").clicked() {
                    scene.menu_open = true;
                    scene.menu_page = MenuPage::Main;
                }
                for (label, s) in [("3×", 3.0), ("2×", 2.0), ("1×", 1.0), ("Pause", 0.0)] {
                    if ui.selectable_label((scene.speed - s).abs() < 0.01, label).on_hover_text("Spielgeschwindigkeit (+/−, Pause: ^)").clicked() {
                        scene.speed = s;
                    }
                }
                ui.add_space(12.0);
                let env = &scene.world.env;
                let mins = (scene.world.time / 60.0) as u32;
                ui.label(RichText::new(format!("Jahr 897 n.F. · Tag {} · {} · {} · {:02}:{:02}", env.day, env.phase().label(), env.weather.label(), mins / 60, mins % 60)).color(theme::PARCHMENT_DIM));
                if scene.world.director.enabled && scene.world.mission.outcome.is_none() {
                    let t = scene.world.director.next_wave_in.max(0.0);
                    let col = if t < 30.0 { theme::EMBER } else { theme::PARCHMENT_DIM };
                    ui.label(RichText::new(format!("Nächste Welle: {}:{:02}", t as u32 / 60, t as u32 % 60)).color(col));
                }
            });
        });
    });
}

/// Minimap als isometrische Raute.
fn minimap(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &Engine, cx: &HudCtx) {
    let map = &scene.world.map;
    let n = map.width;
    let (w, h) = ((n * 2) as usize, n as usize);
    scene.minimap_timer -= ui.ctx().input(|i| i.stable_dt);
    if scene.minimap.is_none() || scene.minimap_timer <= 0.0 {
        scene.minimap_timer = 0.33;
        let mut img = egui::ColorImage::filled([w, h], Color32::from_rgb(6, 5, 6));
        for py in 0..h {
            for px in 0..w {
                let world = Vec2::new((px as f32 + 0.5) * 16.0 - n as f32 * 16.0, (py as f32 + 0.5) * 16.0);
                let t = world_to_tile(glam::Vec2::new(world.x, world.y));
                if t.x < 0.0 || t.y < 0.0 || t.x >= n as f32 || t.y >= n as f32 {
                    continue;
                }
                let terr = map.terrain_at(glam::IVec2::new(t.x as i32, t.y as i32));
                let mut c = match terr {
                    Terrain::Grass => [52, 70, 36],
                    Terrain::DarkGrass => [38, 54, 30],
                    Terrain::Dirt => [72, 56, 40],
                    Terrain::Road => [90, 88, 86],
                    Terrain::Stone => [80, 78, 72],
                    Terrain::Sand => [110, 96, 70],
                    Terrain::Ash => [50, 48, 52],
                    Terrain::Ford => [60, 90, 100],
                    Terrain::Water => [30, 58, 74],
                    Terrain::DeepWater => [18, 38, 52],
                };
                let fog = if scene.fog_enabled { scene.world.fog.state(t) } else { 2 };
                if fog == UNEXPLORED {
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
                dot(nd.center(), Color32::from_rgb(220, 190, 80), 1);
            }
        }
        for (_, b) in scene.world.buildings.iter() {
            if !scene.fog_enabled || scene.world.fog.explored(b.center()) {
                dot(b.center(), if b.owner == PLAYER { Color32::from_rgb(200, 60, 50) } else { Color32::from_rgb(120, 240, 140) }, 2);
            }
        }
        for (_, u) in scene.world.units.iter() {
            if u.owner == PLAYER || !scene.fog_enabled || scene.world.fog.visible(u.pos) {
                dot(u.pos, if u.owner == PLAYER { Color32::from_rgb(255, 120, 90) } else { Color32::from_rgb(150, 255, 170) }, 0);
            }
        }
        match &mut scene.minimap {
            Some(t) => t.set(img, egui::TextureOptions::NEAREST),
            None => scene.minimap = Some(ui.ctx().load_texture("minimap", img, egui::TextureOptions::NEAREST)),
        }
    }
    let size = Vec2::new(232.0, 116.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click_and_drag());
    let painter = ui.painter_at(rect.expand(2.0));
    painter.rect_filled(rect.expand(2.0), CornerRadius::same(2), Color32::from_rgb(8, 6, 6));
    if let Some(t) = &scene.minimap {
        painter.image(t.id(), rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
    }
    painter.rect_stroke(rect.expand(2.0), CornerRadius::same(2), Stroke::new(1.5, theme::GOLD_DIM), StrokeKind::Outside);
    // Kameraausschnitt.
    let to_mini = |wp: glam::Vec2| Pos2::new(rect.min.x + (wp.x + n as f32 * 16.0) / (n as f32 * 32.0) * size.x, rect.min.y + wp.y / (n as f32 * 16.0) * size.y);
    if let Some(v) = engine.renderer.last_view {
        let (a, b) = v.visible_world(0.0);
        let r = Rect::from_two_pos(to_mini(a), to_mini(b));
        painter.rect_stroke(r.intersect(rect), CornerRadius::ZERO, Stroke::new(1.0, Color32::WHITE), StrokeKind::Inside);
    }
    if (resp.clicked() || resp.dragged())
        && let Some(p) = resp.interact_pointer_pos()
    {
        let fx = (p.x - rect.min.x) / size.x * n as f32 * 32.0 - n as f32 * 16.0;
        let fy = (p.y - rect.min.y) / size.y * n as f32 * 16.0;
        scene.camera.center = glam::Vec2::new(fx, fy);
    }
    let _ = cx;
}

fn bar(ui: &mut egui::Ui, frac: f32, width: f32, color: Color32, text: String) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 14.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(20, 14, 12));
    let mut fill = rect;
    fill.set_width(rect.width() * frac.clamp(0.0, 1.0));
    p.rect_filled(fill, CornerRadius::same(2), color);
    p.rect_stroke(rect, CornerRadius::same(2), Stroke::new(1.0, theme::GOLD_DIM), StrokeKind::Inside);
    p.text(rect.center(), Align2::CENTER_CENTER, text, FontId::proportional(11.0), Color32::WHITE);
}

fn selection_panel(scene: &mut MissionScene, ui: &mut egui::Ui, cx: &HudCtx) {
    let data = cx.data;
    if let Some(bid) = scene.selected_building
        && let Some(b) = scene.world.buildings.get(bid).cloned()
    {
        let def = data.building(b.kind);
        ui.horizontal(|ui| {
            ui.add(atlas_image(cx.art, cx.tex, &format!("bld_{}", def.id), 64.0));
            ui.vertical(|ui| {
                ui.label(theme::heading(&def.name, 18.0));
                bar(ui, b.hp / b.max_hp, 180.0, Color32::from_rgb(70, 160, 70), format!("{:.0} / {:.0}", b.hp.max(0.0), b.max_hp));
                if !b.complete {
                    bar(ui, b.progress, 180.0, Color32::from_rgb(190, 150, 60), format!("Bau {:.0} %", b.progress * 100.0));
                }
                ui.label(RichText::new(&def.description).small().color(theme::PARCHMENT_DIM));
                if b.owner == PLAYER && !b.queue.is_empty() {
                    ui.horizontal(|ui| {
                        let mut cancel = None;
                        for (i, q) in b.queue.iter().enumerate() {
                            let u = data.unit(q.unit);
                            let r = ui.add(egui::Button::image(atlas_image_rect(cx, cx.art.portrait(&u.id), 28.0))).on_hover_text(format!("{} – Klick: abbrechen", u.name));
                            if i == 0 {
                                let rr = r.rect;
                                let mut f = rr;
                                f.set_height(3.0);
                                f.set_width(rr.width() * q.progress);
                                f = f.translate(Vec2::new(0.0, rr.height() - 3.0));
                                ui.painter().rect_filled(f, CornerRadius::ZERO, Color32::from_rgb(90, 140, 240));
                            }
                            if r.clicked() {
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
        ui.vertical_centered(|ui| {
            ui.add_space(30.0);
            ui.label(RichText::new("Nichts ausgewählt").color(theme::PARCHMENT_DIM));
            ui.label(RichText::new("Linksklick: auswählen · Ziehen: Rahmen · Rechtsklick: Befehl").small().color(theme::PARCHMENT_DIM));
        });
        return;
    }
    if units.len() == 1 {
        let (_, u) = &units[0];
        let def = data.unit(u.kind);
        ui.horizontal(|ui| {
            ui.add(atlas_image_rect(cx, cx.art.portrait(&def.id), 72.0));
            ui.vertical(|ui| {
                let title = if def.class == UnitClass::Hero { format!("{} · Stufe {}", def.name, u.level) } else { def.name.clone() };
                ui.label(theme::heading(title, 18.0));
                bar(ui, u.hp / u.max_hp, 190.0, Color32::from_rgb(70, 160, 70), format!("{:.0} / {:.0}", u.hp.max(0.0), u.max_hp));
                if u.max_mana > 0.0 {
                    bar(ui, u.mana / u.max_mana, 190.0, Color32::from_rgb(60, 100, 210), format!("Mana {:.0} / {:.0}", u.mana, u.max_mana));
                }
                if def.class == UnitClass::Hero {
                    let levels = &data.rules.xp_levels;
                    let (lo, hi) = (levels.get(u.level as usize - 1).copied().unwrap_or(0), levels.get(u.level as usize).copied().unwrap_or(u.xp.max(1)));
                    bar(ui, (u.xp - lo) as f32 / (hi - lo).max(1) as f32, 190.0, Color32::from_rgb(170, 140, 60), format!("Erfahrung {} / {}", u.xp, hi));
                }
                let armor = def.armor + u.armor_bonus();
                let dmg_bonus = (u.level.saturating_sub(1)) as f32 * def.damage_per_level;
                ui.label(RichText::new(format!(
                    "Schaden {:.0}–{:.0} ({:?}) · Rüstung {:.0} ({:?}) · Reichweite {:.1} · Moral {:.0}",
                    def.damage_min + dmg_bonus,
                    def.damage_max + dmg_bonus,
                    def.damage_type,
                    armor,
                    def.armor_type,
                    def.range,
                    u.morale
                )).small().color(theme::PARCHMENT_DIM));
                let state = if u.routing > 0.0 {
                    "Flieht!".to_string()
                } else if let Some((k, a)) = u.carry {
                    format!("Trägt {a} {}", k.label())
                } else {
                    format!("{:?}", u.activity)
                };
                ui.label(RichText::new(state).small());
            });
        });
    } else {
        egui::ScrollArea::vertical().max_height(118.0).show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let mut pick = None;
                for (id, u) in &units {
                    let def = data.unit(u.kind);
                    let r = ui.add(egui::Button::image(atlas_image_rect(cx, cx.art.portrait(&def.id), 30.0)).min_size(Vec2::splat(36.0))).on_hover_text(&def.name);
                    let rr = r.rect;
                    let mut f = Rect::from_min_size(Pos2::new(rr.min.x + 2.0, rr.max.y - 4.0), Vec2::new((rr.width() - 4.0) * (u.hp / u.max_hp), 2.0));
                    f.set_height(2.0);
                    ui.painter().rect_filled(f, CornerRadius::ZERO, Color32::from_rgb(80, 200, 80));
                    if r.clicked() {
                        pick = Some(*id);
                    }
                }
                if let Some(p) = pick {
                    scene.selection = vec![p];
                }
            });
        });
    }
}

fn atlas_image_rect<'a>(cx: &HudCtx, rect: ashen_engine::render::SpriteRect, size: f32) -> egui::Image<'a> {
    ui_common::image_from_rect(cx.tex, rect, size)
}

/// Befehlsschaltfläche mit Icon und Tastenkürzel.
fn cmd_button(ui: &mut egui::Ui, cx: &HudCtx, icon: &str, hotkey: &str, enabled: bool, tooltip: &str) -> bool {
    let size = Vec2::splat(46.0);
    let (rect, resp) = ui.allocate_exact_size(size, if enabled { Sense::click() } else { Sense::hover() });
    let p = ui.painter();
    let hovered = resp.hovered() && enabled;
    p.rect_filled(rect, CornerRadius::same(3), if hovered { Color32::from_rgb(70, 50, 34) } else { Color32::from_rgb(34, 26, 22) });
    let r = cx.art.get(icon);
    let uv = Rect::from_min_max(Pos2::new(r.uv[0], r.uv[1]), Pos2::new(r.uv[2], r.uv[3]));
    let tint = if enabled { Color32::WHITE } else { Color32::from_gray(80) };
    let img_rect = Rect::from_center_size(rect.center(), Vec2::splat(if r.size.x > 20.0 { 40.0 } else { 32.0 }));
    p.image(cx.tex, img_rect, uv, tint);
    p.rect_stroke(rect, CornerRadius::same(3), Stroke::new(if hovered { 1.5 } else { 1.0 }, if hovered { theme::GOLD } else { theme::GOLD_DIM }), StrokeKind::Inside);
    if !hotkey.is_empty() {
        p.text(rect.right_top() + Vec2::new(-4.0, 3.0), Align2::RIGHT_TOP, hotkey, FontId::proportional(11.0), theme::GOLD);
    }
    let clicked = resp.clicked();
    resp.on_hover_text(tooltip);
    clicked && enabled
}

fn cost_text(c: &ashen_sim::data::Cost) -> String {
    let mut parts = Vec::new();
    for k in ResourceKind::ALL {
        let v = c.get(k);
        if v > 0.0 {
            parts.push(format!("{v:.0} {}", k.label()));
        }
    }
    if parts.is_empty() { "kostenlos".into() } else { parts.join(", ") }
}

fn command_card(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &HudCtx) {
    let data = cx.data;
    let res = scene.world.players[0].resources;
    egui::Grid::new("cmdcard").spacing(Vec2::splat(4.0)).show(ui, |ui| {
        let mut col = 0;
        let mut next = |ui: &mut egui::Ui| {
            col += 1;
            if col % 4 == 0 {
                ui.end_row();
            }
        };
        if let Some(bid) = scene.selected_building
            && let Some(b) = scene.world.buildings.get(bid).cloned()
        {
            if b.owner != PLAYER {
                return;
            }
            let def = data.building(b.kind);
            if b.complete {
                for t in &def.trains {
                    let Some(ut) = data.unit_id(t) else { continue };
                    let u = data.unit(ut);
                    let hk = u.hotkey.map(|c| c.to_string()).unwrap_or_default();
                    let ok = res.covers(&u.cost);
                    let tip = format!("{} ausbilden ({hk})\n{}\nKosten: {} · Zeit {:.0} s · Bevölkerung {}", u.name, u.description, cost_text(&u.cost), u.train_time, u.pop);
                    let portrait = format!("{}_portrait", if cx.art.atlas.try_get(&format!("{}_portrait", u.id)).is_some() { u.id.as_str() } else { "swordsman" });
                    if cmd_button(ui, cx, &portrait, &hk, ok, &tip) {
                        scene.train(bid, ut);
                        engine.audio.play("sfx.ui_click");
                    }
                    next(ui);
                }
                if !def.trains.is_empty() && cmd_button(ui, cx, "icon_rally", "R", true, "Sammelpunkt setzen (R, oder Rechtsklick)") {
                    scene.mode = CommandMode::Rally;
                }
                next(ui);
            }
            if b.tag.as_deref() != Some(scene.world.mission_def.defeat_tag.as_str()) && cmd_button(ui, cx, "icon_demolish", "", true, "Abreißen (Teilerstattung)") {
                scene.demolish(bid);
            }
            return;
        }
        if scene.selection.iter().all(|id| scene.world.units.get(*id).is_none_or(|u| u.owner != PLAYER)) {
            return;
        }
        if scene.mode == CommandMode::BuildMenu {
            for bt in crate::mission::player_buildables(data) {
                let id = data.building(bt).id.as_str();
                let d = data.building(bt);
                let hk = d.hotkey.map(|c| c.to_string()).unwrap_or_default();
                let req_ok = d.requires.iter().all(|r| data.building_id(r).is_some_and(|rb| scene.world.buildings.iter().any(|(_, x)| x.owner == PLAYER && x.kind == rb && x.complete)));
                let ok = res.covers(&d.cost) && req_ok;
                let tip = format!("{} bauen ({hk})\n{}\nKosten: {} · Bauzeit {:.0} s", d.name, d.description, cost_text(&d.cost), d.build_time);
                if cmd_button(ui, cx, &format!("bld_{id}"), &hk, ok, &tip) {
                    scene.mode = CommandMode::Place(bt);
                }
                next(ui);
            }
            if cmd_button(ui, cx, "icon_cancel", "Esc", true, "Zurück") {
                scene.mode = CommandMode::Normal;
            }
            return;
        }
        if cmd_button(ui, cx, "icon_move", "", true, "Bewegen (Rechtsklick)") {
            scene.status = Some("Rechtsklick auf das Ziel".into());
        }
        next(ui);
        if cmd_button(ui, cx, "icon_attack", "A", true, "Angriff / Angriffsbewegung (A, dann Linksklick)") {
            scene.mode = CommandMode::AttackMove;
        }
        next(ui);
        if cmd_button(ui, cx, "icon_stop", "S", true, "Halt (S)") {
            scene.command_stop();
        }
        next(ui);
        if cmd_button(ui, cx, "icon_hold", "H", true, "Position halten (H)") {
            scene.command_hold();
        }
        next(ui);
        if cmd_button(ui, cx, "icon_patrol", "P", true, "Patrouillieren (P, dann Linksklick)") {
            scene.mode = CommandMode::Patrol;
        }
        next(ui);
        if scene.has_workers(data) && cmd_button(ui, cx, "icon_build", "B", true, "Bauen (B) – Gehöft F, Kaserne B, Wachturm T") {
            scene.mode = CommandMode::BuildMenu;
        }
        if scene.has_workers(data) {
            next(ui);
        }
        if let Some(h) = scene.selected_hero(data)
            && let Some(hero) = scene.world.units.get(h).cloned()
        {
            let def = data.unit(hero.kind);
            for (i, a) in def.abilities.iter().enumerate() {
                let Some(aid) = data.ability_id(a) else { continue };
                let ad = data.ability(aid);
                let cd = hero.ability_cd.get(i).copied().unwrap_or(0.0);
                let ok = cd <= 0.0 && hero.mana >= ad.mana;
                let hk = ad.hotkey.map(|c| c.to_string()).unwrap_or_default();
                let tip = format!("{} ({hk})\n{}\nMana {:.0} · Abklingzeit {:.0} s{}", ad.name, ad.description, ad.mana, ad.cooldown, if cd > 0.0 { format!("\nBereit in {cd:.0} s") } else { String::new() });
                let icon = format!("icon_{}", ad.id);
                let clicked = cmd_button(ui, cx, &icon, &hk, ok, &tip);
                if cd > 0.0 {
                    let r = ui.min_rect();
                    let _ = r;
                }
                if clicked {
                    scene.use_ability(data, i as u8);
                }
                next(ui);
            }
        }
    });
}

fn bottom_panel(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    egui::Panel::bottom("bottom").exact_size(150.0).frame(theme::panel_frame().corner_radius(CornerRadius::ZERO)).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                minimap(scene, ui, engine, cx);
                if let Some(s) = &scene.status {
                    ui.label(RichText::new(s).small().color(theme::PARCHMENT_DIM));
                }
            });
            ui.separator();
            let avail = ui.available_width();
            ui.allocate_ui(Vec2::new((avail - 220.0).max(200.0), 140.0), |ui| {
                ui.set_min_width((avail - 220.0).max(200.0));
                selection_panel(scene, ui, cx);
            });
            ui.separator();
            ui.vertical(|ui| command_card(scene, ui, engine, cx));
        });
    });
}

fn objectives(scene: &mut MissionScene, ui: &mut egui::Ui, cx: &HudCtx) {
    if scene.world.mission.objectives.is_empty() {
        return;
    }
    egui::Area::new(egui::Id::unique("objectives")).anchor(Align2::RIGHT_TOP, Vec2::new(-10.0, 48.0)).show(ui.ctx(), |ui| {
        theme::panel_frame().show(ui, |ui| {
            ui.set_max_width(300.0);
            ui.horizontal(|ui| {
                ui.label(theme::heading("Ziele", 16.0));
                if ui.small_button(if scene.show_objectives { "–" } else { "+" }).clicked() {
                    scene.show_objectives = !scene.show_objectives;
                }
            });
            if !scene.show_objectives {
                return;
            }
            for (id, state) in &scene.world.mission.objectives {
                let Some(def) = scene.world.mission_def.objective(id) else { continue };
                let (mark, col) = match state {
                    ObjectiveState::Completed => ("√", Color32::from_rgb(120, 200, 110)),
                    ObjectiveState::Active => ("•", if def.optional { theme::PARCHMENT_DIM } else { theme::GOLD }),
                };
                let text = if def.optional { format!("{} (optional)", def.text) } else { def.text.clone() };
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(mark).color(col));
                    let t = RichText::new(text).size(13.0);
                    ui.label(if *state == ObjectiveState::Completed { t.strikethrough().color(theme::PARCHMENT_DIM) } else { t });
                });
            }
        });
    });
    let _ = cx;
}

fn messages(scene: &mut MissionScene, ui: &mut egui::Ui) {
    if let Some((t, _)) = &scene.toast {
        egui::Area::new(egui::Id::unique("toast")).anchor(Align2::CENTER_TOP, Vec2::new(0.0, 56.0)).show(ui.ctx(), |ui| {
            theme::panel_frame().show(ui, |ui| {
                ui.label(RichText::new(t).color(theme::GOLD).size(16.0).strong());
            });
        });
    }
    if scene.messages.is_empty() {
        return;
    }
    egui::Area::new(egui::Id::unique("messages")).anchor(Align2::LEFT_BOTTOM, Vec2::new(12.0, -162.0)).show(ui.ctx(), |ui| {
        ui.set_max_width(560.0);
        for m in scene.messages.iter() {
            let fade = (1.0 - ((m.age - 11.0) / 3.0).clamp(0.0, 1.0)) * 255.0;
            theme::parchment_frame().multiply_with_opacity(fade / 255.0).show(ui, |ui| {
                if !m.speaker.is_empty() {
                    ui.label(RichText::new(&m.speaker).color(theme::GOLD).strong());
                }
                ui.label(RichText::new(&m.text).color(theme::PARCHMENT.gamma_multiply(fade / 255.0)));
            });
            ui.add_space(4.0);
        }
    });
}

fn mode_hint(scene: &MissionScene, ui: &mut egui::Ui) {
    let text = match scene.mode {
        CommandMode::Normal => return,
        CommandMode::Place(_) => "Bauplatz wählen: Linksklick platzieren · Shift: mehrere · Rechtsklick/Esc: abbrechen",
        CommandMode::AttackMove => "Angriffsziel: Linksklick auf Gegner oder Boden",
        CommandMode::Patrol => "Patrouillenziel: Linksklick",
        CommandMode::Rally => "Sammelpunkt: Linksklick",
        CommandMode::BuildMenu => "Bauen: F Gehöft · B Kaserne · T Wachturm · Esc zurück",
    };
    egui::Area::new(egui::Id::unique("modehint")).anchor(Align2::CENTER_BOTTOM, Vec2::new(0.0, -160.0)).show(ui.ctx(), |ui| {
        theme::panel_frame().show(ui, |ui| {
            ui.label(RichText::new(text).color(theme::PARCHMENT));
        });
    });
}

fn pause_menu(scene: &mut MissionScene, ui: &mut egui::Ui, engine: &mut Engine, cx: &mut HudCtx) {
    let ctx = ui.ctx().clone();
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::unique("dim")));
    painter.rect_filled(ctx.content_rect(), CornerRadius::ZERO, Color32::from_black_alpha(140));
    egui::Window::new("pause").title_bar(false).resizable(false).anchor(Align2::CENTER_CENTER, Vec2::ZERO).frame(theme::panel_frame().inner_margin(18)).show(&ctx, |ui| {
        ui.set_min_width(340.0);
        match scene.menu_page {
            MenuPage::Main => {
                ui.vertical_centered(|ui| {
                    ui.label(theme::heading("Pause", 26.0));
                    ui.add_space(8.0);
                    let b = |ui: &mut egui::Ui, t: &str| ui.add_sized([260.0, 34.0], egui::Button::new(t)).clicked();
                    if b(ui, "Weiterspielen") {
                        scene.menu_open = false;
                    }
                    if b(ui, "Speichern") {
                        scene.menu_page = MenuPage::Save;
                    }
                    if b(ui, "Laden") {
                        scene.menu_page = MenuPage::Load;
                    }
                    if b(ui, "Chronik") {
                        scene.menu_page = MenuPage::Chronicle;
                    }
                    if b(ui, "Einstellungen") {
                        scene.menu_page = MenuPage::Settings;
                    }
                    if b(ui, "Mission neu starten") {
                        scene.action = SceneAction::Restart;
                    }
                    if b(ui, "Hauptmenü") {
                        scene.action = SceneAction::MainMenu;
                    }
                    if b(ui, "Spiel beenden") {
                        scene.action = SceneAction::Quit;
                    }
                });
            }
            MenuPage::Save => {
                ui.label(theme::heading("Spiel speichern", 22.0));
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut scene.save_name);
                });
                if ui.button("Speichern").clicked() {
                    let name = scene.save_name.clone();
                    scene.save(cx.data, &name);
                    scene.menu_page = MenuPage::Main;
                }
                if let Some(s) = &scene.status {
                    ui.label(RichText::new(s).small());
                }
                if ui.button("Zurück").clicked() {
                    scene.menu_page = MenuPage::Main;
                }
            }
            MenuPage::Load => {
                ui.label(theme::heading("Spiel laden", 22.0));
                if let Some(p) = ui_common::save_list(ui, &saves_dir()) {
                    scene.action = SceneAction::Load(p);
                }
                if ui.button("Zurück").clicked() {
                    scene.menu_page = MenuPage::Main;
                }
            }
            MenuPage::Settings => {
                ui.label(theme::heading("Einstellungen", 22.0));
                ui_common::settings_ui(ui, engine, cx.settings);
                if ui.button("Zurück").clicked() {
                    cx.settings.save();
                    scene.menu_page = MenuPage::Main;
                }
            }
            MenuPage::Chronicle => {
                ui.label(theme::heading("Chronik von Brennhollow", 22.0));
                chronicle(ui, &scene.world);
                if ui.button("Zurück").clicked() {
                    scene.menu_page = MenuPage::Main;
                }
            }
        }
    });
}

fn chronicle(ui: &mut egui::Ui, world: &ashen_sim::SimWorld) {
    egui::ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
        ui.set_max_width(520.0);
        for e in &world.chronicle.entries {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("Tag {} · {}", e.day, e.phase)).color(theme::GOLD).small());
                ui.label(RichText::new(&e.text).size(13.5));
            });
        }
    });
}

fn outcome(scene: &mut MissionScene, ui: &mut egui::Ui, cx: &mut HudCtx, o: Outcome) {
    let ctx = ui.ctx().clone();
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::unique("dim2")));
    painter.rect_filled(ctx.content_rect(), CornerRadius::ZERO, Color32::from_black_alpha(120));
    egui::Window::new("outcome").title_bar(false).resizable(false).anchor(Align2::CENTER_CENTER, Vec2::ZERO).frame(theme::panel_frame().inner_margin(22)).show(&ctx, |ui| {
        ui.set_min_width(520.0);
        ui.vertical_centered(|ui| {
            let (title, col) = match o {
                Outcome::Victory => ("SIEG", theme::GOLD),
                Outcome::Defeat => ("NIEDERLAGE", theme::BLOOD),
            };
            ui.label(RichText::new(title).size(40.0).color(col).strong());
            ui.label(RichText::new(&scene.world.mission_def.name).color(theme::PARCHMENT_DIM));
        });
        ui.add_space(8.0);
        let s = &scene.world.players[0].stats;
        ui.label(format!(
            "Ausgebildet: {} · Verluste: {} · Besiegte Feinde: {} · Gebäude: {} · Wellen überstanden: {} · Spielzeit: {}:{:02}",
            s.units_trained,
            s.units_lost,
            s.kills,
            s.buildings_built,
            scene.world.mission.waves_survived,
            scene.world.time as u32 / 60,
            scene.world.time as u32 % 60
        ));
        ui.label(format!("Gesammelt: {:.0} Gold, {:.0} Holz, {:.0} Stein, {:.0} Nahrung", s.gathered.gold, s.gathered.wood, s.gathered.stone, s.gathered.food));
        ui.add_space(6.0);
        ui.label(theme::heading("Chronik", 18.0));
        chronicle(ui, &scene.world);
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button("Erneut spielen").clicked() {
                scene.action = SceneAction::Restart;
            }
            if ui.button("Hauptmenü").clicked() {
                scene.action = SceneAction::MainMenu;
            }
        });
    });
    let _ = cx;
}
