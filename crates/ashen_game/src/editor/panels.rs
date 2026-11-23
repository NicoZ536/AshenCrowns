//! Oberfläche des Karteneditors: Werkzeugleiste, Eigenschaften, Dialoge, Kartenüberlagerung.

use super::triggers::{self, Lists};
use super::validate::Level;
use super::{DataTab, Dialog, EditorAction, EditorScene, Sel, Tool};
use crate::art::ArtLibrary;
use crate::content::Content;
use crate::hud_city::building_icon;
use crate::ui_art::{self, PanelKind, UiArt};
use crate::ui_common;
use ashen_core::iso::tile_to_world;
use ashen_engine::Engine;
use ashen_engine::render::ViewParams;
use ashen_engine::ui::theme;
use ashen_sim::data::Faction;
use ashen_sim::entity::NodeKind;
use ashen_sim::genmap::GenPreset;
use ashen_sim::map::{DecorKind, Terrain};
use egui::{
    Align2, Color32, Id, Margin, Order, Pos2, Rect, RichText, Sense, Shape, Stroke, StrokeKind,
    Vec2,
};
use glam::{IVec2, Vec2 as GVec2};

const TOP_H: f32 = 52.0;
const LEFT_W: f32 = 320.0;
const LABEL: Color32 = Color32::from_rgb(214, 190, 146);

fn swatch(t: Terrain) -> Color32 {
    match t {
        Terrain::Grass => Color32::from_rgb(96, 140, 64),
        Terrain::DarkGrass => Color32::from_rgb(58, 100, 56),
        Terrain::Dirt => Color32::from_rgb(130, 100, 66),
        Terrain::Road => Color32::from_rgb(158, 140, 108),
        Terrain::Stone => Color32::from_rgb(122, 122, 130),
        Terrain::Sand => Color32::from_rgb(204, 186, 132),
        Terrain::Ash => Color32::from_rgb(92, 88, 86),
        Terrain::Ford => Color32::from_rgb(112, 156, 164),
        Terrain::Water => Color32::from_rgb(58, 110, 172),
        Terrain::DeepWater => Color32::from_rgb(28, 58, 118),
    }
}

fn to_screen(view: &ViewParams, ppp: f32, t: GVec2) -> Pos2 {
    let s = view.world_to_screen(tile_to_world(t));
    Pos2::new(s.x / ppp, s.y / ppp)
}

fn diamond(view: &ViewParams, ppp: f32, x: f32, y: f32, w: f32, h: f32) -> Vec<Pos2> {
    [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
        .iter()
        .map(|(px, py)| to_screen(view, ppp, GVec2::new(*px, *py)))
        .collect()
}

fn area_color(i: usize) -> Color32 {
    [
        Color32::from_rgb(120, 200, 255),
        Color32::from_rgb(255, 200, 100),
        Color32::from_rgb(180, 255, 150),
        Color32::from_rgb(255, 140, 200),
        Color32::from_rgb(200, 170, 255),
    ][i % 5]
}

fn owner_color(o: u8) -> Color32 {
    if o == 0 {
        Color32::from_rgb(90, 170, 255)
    } else {
        Color32::from_rgb(235, 90, 76)
    }
}

impl EditorScene {
    #[allow(clippy::too_many_arguments)]
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        engine: &mut Engine,
        content: &mut Content,
        art: &UiArt,
        lib: &ArtLibrary,
        tex: egui::TextureId,
    ) {
        let ctx = ui.ctx().clone();
        self.paint_overlay(&ctx, content);
        self.top_bar(&ctx, engine, content, art);
        if self.dialog == Dialog::None {
            self.toolbox(&ctx, engine, content, art, lib, tex);
            self.side_panel(&ctx, engine, content, art);
        }
        self.status_bar(&ctx, content, art);
        match self.dialog {
            Dialog::None => {}
            Dialog::NewMap => self.new_map_dialog(&ctx, engine, content, art),
            Dialog::Load => self.load_dialog(&ctx, engine, content, art),
            Dialog::Data => self.data_dialog(&ctx, engine, content, art),
            Dialog::ConfirmExit => self.exit_dialog(&ctx, engine, content, art),
        }
    }

    // ------------------------------------------------------------------ Überlagerung auf der Karte

    fn paint_overlay(&mut self, ctx: &egui::Context, content: &Content) {
        let Some(view) = self.last_view() else { return };
        let ppp = ctx.pixels_per_point();
        let painter = ctx.layer_painter(egui::LayerId::new(
            Order::Background,
            Id::unique("editor_overlay"),
        ));
        let (w, h) = (self.doc.map.width as f32, self.doc.map.height as f32);
        // Kartenrand.
        painter.add(Shape::closed_line(
            diamond(&view, ppp, 0.0, 0.0, w, h),
            Stroke::new(1.5, theme::GOLD_DIM),
        ));
        if self.show_grid {
            let col = Color32::from_white_alpha(22);
            for i in (0..=self.doc.map.width).step_by(4) {
                painter.line_segment(
                    [
                        to_screen(&view, ppp, GVec2::new(i as f32, 0.0)),
                        to_screen(&view, ppp, GVec2::new(i as f32, h)),
                    ],
                    Stroke::new(1.0, col),
                );
            }
            for j in (0..=self.doc.map.height).step_by(4) {
                painter.line_segment(
                    [
                        to_screen(&view, ppp, GVec2::new(0.0, j as f32)),
                        to_screen(&view, ppp, GVec2::new(w, j as f32)),
                    ],
                    Stroke::new(1.0, col),
                );
            }
        }
        // Bereiche.
        for (i, a) in self.doc.def.areas.iter().enumerate() {
            let c = area_color(i);
            let pts = diamond(&view, ppp, a.x as f32, a.y as f32, a.w as f32, a.h as f32);
            let sel = self.selection == Some(Sel::Area(i));
            painter.add(Shape::convex_polygon(
                pts.clone(),
                c.gamma_multiply(if sel { 0.28 } else { 0.14 }),
                Stroke::new(if sel { 2.5 } else { 1.5 }, c),
            ));
            let label = if a.name.is_empty() {
                a.id.clone()
            } else {
                a.name.clone()
            };
            painter.text(
                to_screen(&view, ppp, a.center()),
                Align2::CENTER_CENTER,
                label,
                theme::heading_font(13.0),
                Color32::WHITE,
            );
        }
        // Platzierungen: Beschriftung getaggter Objekte, Wächter, Auswahl.
        let data = &content.data;
        for (i, p) in self.doc.def.placements.iter().enumerate() {
            let center = self.doc.placement_center(data, i);
            let selected = self.selection == Some(Sel::Placement(i));
            let sp = to_screen(&view, ppp, center);
            if let Some((o, s)) = super::doc::footprint(data, p) {
                if selected {
                    painter.add(Shape::closed_line(
                        diamond(&view, ppp, o.x as f32, o.y as f32, s.x as f32, s.y as f32),
                        Stroke::new(2.5, theme::GOLD_BRIGHT),
                    ));
                }
            } else {
                let r = 12.0 + (p.count as f32).sqrt() * 4.0;
                painter.circle_stroke(
                    sp + Vec2::new(0.0, -6.0),
                    r,
                    Stroke::new(
                        if selected { 2.5 } else { 1.0 },
                        if selected {
                            theme::GOLD_BRIGHT
                        } else {
                            owner_color(p.owner).gamma_multiply(0.7)
                        },
                    ),
                );
                if p.count > 1 {
                    painter.text(
                        sp + Vec2::new(0.0, 12.0),
                        Align2::CENTER_TOP,
                        format!("×{}", p.count),
                        theme::heading_font(12.0),
                        Color32::WHITE,
                    );
                }
                if p.guard {
                    painter.circle_stroke(
                        sp,
                        r + 8.0,
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 200, 80, 120)),
                    );
                }
            }
            if let Some(t) = &p.tag {
                painter.text(
                    sp + Vec2::new(0.0, -28.0),
                    Align2::CENTER_BOTTOM,
                    format!("# {t}"),
                    theme::body_bold_font(12.5),
                    theme::EMBER_HOT,
                );
            }
        }
        // Quellen und Dekoration (Auswahl).
        if let Some(Sel::Node(i)) = self.selection
            && let Some(n) = self.doc.map.nodes.get(i)
        {
            let s = n.kind.size();
            painter.add(Shape::closed_line(
                diamond(&view, ppp, n.x as f32, n.y as f32, s.x as f32, s.y as f32),
                Stroke::new(2.5, theme::GOLD_BRIGHT),
            ));
        }
        if let Some(Sel::Decor(i)) = self.selection
            && let Some(d) = self.doc.map.decor.get(i)
        {
            let sp = to_screen(&view, ppp, GVec2::new(d.x, d.y));
            painter.circle_stroke(
                sp + Vec2::new(0.0, -6.0),
                12.0,
                Stroke::new(2.5, theme::GOLD_BRIGHT),
            );
        }
        // Startpositionen.
        for (i, s) in self.doc.def.starts.iter().enumerate() {
            let sp = to_screen(&view, ppp, GVec2::new(s.x, s.y));
            let col = owner_color(s.owner);
            let sel = self.selection == Some(Sel::Start(i));
            painter.add(Shape::convex_polygon(
                vec![
                    sp + Vec2::new(0.0, -16.0),
                    sp + Vec2::new(11.0, 0.0),
                    sp + Vec2::new(0.0, 8.0),
                    sp + Vec2::new(-11.0, 0.0),
                ],
                col.gamma_multiply(0.85),
                Stroke::new(
                    if sel { 3.0 } else { 1.5 },
                    if sel {
                        theme::GOLD_BRIGHT
                    } else {
                        Color32::BLACK
                    },
                ),
            ));
            painter.text(
                sp + Vec2::new(0.0, -26.0),
                Align2::CENTER_BOTTOM,
                if s.owner == 0 {
                    "START · Glutkrone"
                } else {
                    "START · Die Hohlen"
                },
                theme::heading_font(12.5),
                col,
            );
        }
        // Zeiger und Pinselvorschau.
        let Some(m) = self.mouse_tile() else { return };
        let tile = IVec2::new(m.x.floor() as i32, m.y.floor() as i32);
        let ok = Color32::from_rgb(250, 222, 150);
        match self.tool {
            Tool::Terrain | Tool::Trees | Tool::Erase => {
                for t in self.brush_shape.tiles(tile, self.brush_size) {
                    painter.add(Shape::closed_line(
                        diamond(&view, ppp, t.x as f32, t.y as f32, 1.0, 1.0),
                        Stroke::new(1.0, ok.gamma_multiply(0.8)),
                    ));
                }
            }
            Tool::Fill | Tool::Decor | Tool::Start | Tool::Unit | Tool::Select => {
                painter.add(Shape::closed_line(
                    diamond(&view, ppp, tile.x as f32, tile.y as f32, 1.0, 1.0),
                    Stroke::new(1.5, ok),
                ));
            }
            Tool::Nodes => {
                let s = self.node_kind.size();
                painter.add(Shape::closed_line(
                    diamond(
                        &view,
                        ppp,
                        tile.x as f32,
                        tile.y as f32,
                        s.x as f32,
                        s.y as f32,
                    ),
                    Stroke::new(1.5, ok),
                ));
            }
            Tool::Area => {
                if let Some(a) = self.area_drag() {
                    let (x0, y0) = (a.x.min(tile.x), a.y.min(tile.y));
                    let (x1, y1) = (a.x.max(tile.x) + 1, a.y.max(tile.y) + 1);
                    let pts = diamond(
                        &view,
                        ppp,
                        x0 as f32,
                        y0 as f32,
                        (x1 - x0) as f32,
                        (y1 - y0) as f32,
                    );
                    painter.add(Shape::convex_polygon(
                        pts,
                        Color32::from_rgba_unmultiplied(120, 200, 255, 40),
                        Stroke::new(2.0, Color32::from_rgb(120, 200, 255)),
                    ));
                } else {
                    painter.add(Shape::closed_line(
                        diamond(&view, ppp, tile.x as f32, tile.y as f32, 1.0, 1.0),
                        Stroke::new(1.5, ok),
                    ));
                }
            }
            Tool::Building => {}
        }
    }

    // ------------------------------------------------------------------ Kopfleiste

    fn top_bar(
        &mut self,
        ctx: &egui::Context,
        engine: &mut Engine,
        content: &mut Content,
        art: &UiArt,
    ) {
        let screen = ctx.content_rect();
        egui::Area::new(Id::unique("editor_top"))
            .order(Order::Foreground)
            .fixed_pos(screen.min)
            .show(ctx, |ui| {
                let rect = Rect::from_min_size(screen.min, Vec2::new(screen.width(), TOP_H));
                for s in
                    art.panel_shapes(PanelKind::Plaque, rect.shrink2(Vec2::new(4.0, 2.0)), false)
                {
                    ui.painter().add(s);
                }
                let inner = rect.shrink2(Vec2::new(22.0, 9.0));
                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(inner)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        let title = format!(
                            "Karteneditor – {}{}",
                            self.doc.def.name,
                            if self.modified { " *" } else { "" }
                        );
                        ui.label(
                            RichText::new(title)
                                .font(theme::heading_font(17.0))
                                .color(theme::GOLD),
                        );
                        ui.add_space(14.0);
                        let mut b =
                            |ui: &mut egui::Ui, text: &str, w: f32, enabled: bool| -> bool {
                                let r = ui_art::button(ui, art, text, Vec2::new(w, 32.0), enabled);
                                ui_common::sfx(engine, ui, &r);
                                r.clicked()
                            };
                        if b(ui, "Neu", 70.0, true) {
                            self.dialog = Dialog::NewMap;
                        }
                        if b(ui, "Laden", 80.0, true) {
                            self.refresh_saved_list(content);
                            self.dialog = Dialog::Load;
                        }
                        if b(ui, "Speichern", 110.0, true) {
                            self.save(&content.data);
                        }
                        let (u, r) = (self.can_undo(), self.can_redo());
                        if b(ui, "Zurück", 86.0, u) {
                            self.undo();
                        }
                        if b(ui, "Vor", 64.0, r) {
                            self.redo();
                        }
                        ui.add_space(10.0);
                        if b(ui, "Missionsdaten", 140.0, true) {
                            self.dialog = Dialog::Data;
                        }
                        if b(ui, "Prüfen", 84.0, true) {
                            self.issues_dirty = true;
                            self.refresh_issues(&content.data);
                            self.data_tab = DataTab::Validation;
                            self.dialog = Dialog::Data;
                        }
                        if b(ui, "Testspiel", 110.0, true) {
                            self.refresh_issues(&content.data);
                            if super::validate::has_errors(&self.issues) {
                                self.notify("Testspiel trotz Fehlern: siehe Prüfung.", true);
                            }
                            self.action = EditorAction::Test(Box::new(self.test_mission()));
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if b(ui, "Beenden", 100.0, true) {
                                if self.modified {
                                    self.dialog = Dialog::ConfirmExit;
                                } else {
                                    self.action = EditorAction::Exit;
                                }
                            }
                            ui.checkbox(
                                &mut self.show_grid,
                                RichText::new("Raster (G)")
                                    .font(theme::body_font(14.0))
                                    .color(theme::PARCHMENT_DIM),
                            );
                        });
                    },
                );
            });
    }

    // ------------------------------------------------------------------ Werkzeugleiste

    fn toolbox(
        &mut self,
        ctx: &egui::Context,
        engine: &mut Engine,
        content: &Content,
        art: &UiArt,
        lib: &ArtLibrary,
        tex: egui::TextureId,
    ) {
        egui::Area::new(Id::unique("editor_tools"))
            .order(Order::Foreground)
            .anchor(Align2::LEFT_TOP, Vec2::new(6.0, TOP_H + 6.0))
            .show(ctx, |ui| {
                art.panel(ui, PanelKind::Ornate, Margin::symmetric(38, 34), |ui| {
                    ui.set_width(LEFT_W - 76.0);
                    ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                    ui.horizontal_wrapped(|ui| {
                        for t in Tool::ALL {
                            let r = ui_art::choice(ui, art, t.label(), self.tool == t);
                            ui_common::sfx(engine, ui, &r);
                            if r.clicked() {
                                self.tool = t;
                            }
                        }
                    });
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(self.tool.hint())
                            .font(theme::italic_font(13.0))
                            .color(theme::PARCHMENT_DIM),
                    );
                    ui.add_space(6.0);
                    match self.tool {
                        Tool::Terrain | Tool::Fill => {
                            self.terrain_palette(ui);
                            if self.tool == Tool::Terrain {
                                self.brush_controls(ui);
                            }
                        }
                        Tool::Trees => {
                            self.brush_controls(ui);
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Dichte")
                                        .font(theme::caps_font(13.0))
                                        .color(LABEL),
                                );
                                ui.add(
                                    egui::Slider::new(&mut self.tree_density, 0.05..=1.0)
                                        .show_value(false),
                                );
                            });
                        }
                        Tool::Erase => self.brush_controls(ui),
                        Tool::Decor => {
                            egui::ScrollArea::vertical()
                                .id_salt("decor_list")
                                .min_scrolled_height(300.0)
                                .max_height(320.0)
                                .show(ui, |ui| {
                                    for k in DecorKind::ALL {
                                        let txt = if k.blocks() {
                                            format!("{} (blockiert)", k.label())
                                        } else {
                                            k.label().to_string()
                                        };
                                        if ui
                                            .selectable_label(
                                                self.decor_kind == k,
                                                RichText::new(txt).size(15.0),
                                            )
                                            .clicked()
                                        {
                                            self.decor_kind = k;
                                        }
                                    }
                                });
                        }
                        Tool::Nodes => {
                            for k in [NodeKind::GoldMine, NodeKind::StoneQuarry, NodeKind::Tree] {
                                if ui
                                    .selectable_label(
                                        self.node_kind == k,
                                        RichText::new(k.label()).size(15.5),
                                    )
                                    .clicked()
                                {
                                    self.node_kind = k;
                                }
                            }
                        }
                        Tool::Building | Tool::Unit => {
                            self.owner_toggle(ui, art, engine);
                            if self.tool == Tool::Building {
                                self.building_list(ui, content, lib, tex);
                            } else {
                                self.unit_list(ui, content, lib, tex);
                            }
                        }
                        Tool::Start => self.owner_toggle(ui, art, engine),
                        Tool::Select | Tool::Area => {}
                    }
                });
            });
    }

    fn terrain_palette(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for t in Terrain::ALL {
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(116.0, 26.0), Sense::click());
                let sel = self.terrain == t;
                let p = ui.painter();
                p.rect_filled(rect, 3.0, swatch(t));
                p.rect_stroke(
                    rect,
                    3.0,
                    Stroke::new(
                        if sel { 2.5 } else { 1.0 },
                        if sel {
                            theme::GOLD_BRIGHT
                        } else {
                            Color32::from_black_alpha(200)
                        },
                    ),
                    StrokeKind::Inside,
                );
                let dark = matches!(
                    t,
                    Terrain::Grass | Terrain::Sand | Terrain::Road | Terrain::Ford
                );
                p.text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    t.label(),
                    theme::body_bold_font(14.0),
                    if dark {
                        Color32::from_rgb(30, 22, 14)
                    } else {
                        Color32::WHITE
                    },
                );
                if resp.clicked() {
                    self.terrain = t;
                }
            }
        });
    }

    fn brush_controls(&mut self, ui: &mut egui::Ui) {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Größe")
                    .font(theme::caps_font(13.0))
                    .color(LABEL),
            );
            ui.add(egui::Slider::new(&mut self.brush_size, 1..=12));
        });
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Form")
                    .font(theme::caps_font(13.0))
                    .color(LABEL),
            );
            for s in super::doc::BrushShape::ALL {
                if ui
                    .selectable_label(self.brush_shape == s, s.label())
                    .clicked()
                {
                    self.brush_shape = s;
                }
            }
        });
    }

    fn owner_toggle(&mut self, ui: &mut egui::Ui, art: &UiArt, engine: &mut Engine) {
        ui.horizontal(|ui| {
            for (o, name) in [(0u8, "Glutkrone"), (1u8, "Die Hohlen")] {
                let r = ui_art::choice(ui, art, name, self.owner == o);
                ui_common::sfx(engine, ui, &r);
                if r.clicked() {
                    self.owner = o;
                }
            }
        });
    }

    fn faction_ok(&self, f: Faction) -> bool {
        if self.owner == 0 {
            matches!(f, Faction::EmberCrown | Faction::Neutral)
        } else {
            matches!(f, Faction::Hollowed | Faction::Neutral)
        }
    }

    fn building_list(
        &mut self,
        ui: &mut egui::Ui,
        content: &Content,
        lib: &ArtLibrary,
        tex: egui::TextureId,
    ) {
        let list: Vec<_> = content
            .data
            .buildings
            .iter()
            .filter(|b| self.faction_ok(b.faction))
            .collect();
        if !list.iter().any(|b| b.id == self.building_id)
            && let Some(f) = list.first()
        {
            self.building_id = f.id.clone();
        }
        egui::ScrollArea::vertical()
            .id_salt("bld_list")
            .min_scrolled_height(260.0)
            .max_height(300.0)
            .show(ui, |ui| {
                for b in list {
                    let sel = self.building_id == b.id;
                    let (rect, resp) = ui
                        .allocate_exact_size(Vec2::new(ui.available_width(), 34.0), Sense::click());
                    let p = ui.painter();
                    if sel {
                        p.rect_filled(rect, 3.0, Color32::from_rgba_unmultiplied(236, 112, 40, 60));
                        p.rect_stroke(
                            rect,
                            3.0,
                            Stroke::new(1.5, theme::EMBER),
                            StrokeKind::Inside,
                        );
                    } else if resp.hovered() {
                        p.rect_filled(rect, 3.0, Color32::from_white_alpha(14));
                    }
                    let spr = lib
                        .atlas
                        .try_get(&building_icon(b))
                        .unwrap_or_else(|| lib.get("icon_cancel"));
                    ui_common::paint_sprite(
                        p,
                        tex,
                        spr,
                        Rect::from_min_size(rect.min + Vec2::new(3.0, 1.0), Vec2::splat(32.0)),
                        Color32::WHITE,
                    );
                    p.text(
                        rect.min + Vec2::new(42.0, 17.0),
                        Align2::LEFT_CENTER,
                        format!("{}  {}×{}", b.name, b.size.0, b.size.1),
                        theme::body_font(15.0),
                        theme::PARCHMENT,
                    );
                    if resp.clicked() {
                        self.building_id = b.id.clone();
                    }
                }
            });
    }

    fn unit_list(
        &mut self,
        ui: &mut egui::Ui,
        content: &Content,
        lib: &ArtLibrary,
        tex: egui::TextureId,
    ) {
        let list: Vec<_> = content
            .data
            .units
            .iter()
            .filter(|u| self.faction_ok(u.faction))
            .collect();
        if !list.iter().any(|u| u.id == self.unit_id)
            && let Some(f) = list.first()
        {
            self.unit_id = f.id.clone();
        }
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Anzahl")
                    .font(theme::caps_font(13.0))
                    .color(LABEL),
            );
            ui.add(egui::Slider::new(&mut self.unit_count, 1..=20));
        });
        egui::ScrollArea::vertical()
            .id_salt("unit_list")
            .min_scrolled_height(240.0)
            .max_height(280.0)
            .show(ui, |ui| {
                for u in list {
                    let sel = self.unit_id == u.id;
                    let (rect, resp) = ui
                        .allocate_exact_size(Vec2::new(ui.available_width(), 34.0), Sense::click());
                    let p = ui.painter();
                    if sel {
                        p.rect_filled(rect, 3.0, Color32::from_rgba_unmultiplied(236, 112, 40, 60));
                        p.rect_stroke(
                            rect,
                            3.0,
                            Stroke::new(1.5, theme::EMBER),
                            StrokeKind::Inside,
                        );
                    } else if resp.hovered() {
                        p.rect_filled(rect, 3.0, Color32::from_white_alpha(14));
                    }
                    ui_common::paint_sprite(
                        p,
                        tex,
                        lib.portrait(&u.id),
                        Rect::from_min_size(rect.min + Vec2::new(3.0, 1.0), Vec2::splat(32.0)),
                        Color32::WHITE,
                    );
                    p.text(
                        rect.min + Vec2::new(42.0, 17.0),
                        Align2::LEFT_CENTER,
                        &u.name,
                        theme::body_font(15.0),
                        theme::PARCHMENT,
                    );
                    if resp.clicked() {
                        self.unit_id = u.id.clone();
                    }
                }
            });
    }

    // ------------------------------------------------------------------ Eigenschaften rechts

    fn side_panel(
        &mut self,
        ctx: &egui::Context,
        engine: &mut Engine,
        content: &Content,
        art: &UiArt,
    ) {
        let data = &content.data;
        let (b, u, n) = self.doc.counts(data);
        egui::Area::new(Id::unique("editor_side"))
            .order(Order::Foreground)
            .anchor(Align2::RIGHT_TOP, Vec2::new(-6.0, TOP_H + 6.0))
            .show(ctx, |ui| {
                art.panel(ui, PanelKind::Ornate, Margin::symmetric(38, 34), |ui| {
                    ui.set_width(220.0);
                    ui.label(
                        RichText::new("Eigenschaften")
                            .font(theme::heading_font(15.0))
                            .color(theme::GOLD),
                    );
                    let mut changed = false;
                    let key;
                    match self.selection {
                        None => {
                            key = "";
                            ui.label(
                                RichText::new("Nichts gewählt.")
                                    .font(theme::italic_font(14.0))
                                    .color(theme::PARCHMENT_DIM),
                            );
                        }
                        Some(Sel::Placement(i)) if i < self.doc.def.placements.len() => {
                            key = "text-pl";
                            let is_building =
                                data.building_id(&self.doc.def.placements[i].id).is_some();
                            let name = data
                                .building_id(&self.doc.def.placements[i].id)
                                .map(|b| data.building(b).name.clone())
                                .or_else(|| {
                                    data.unit_id(&self.doc.def.placements[i].id)
                                        .map(|u| data.unit(u).name.clone())
                                })
                                .unwrap_or_default();
                            let p = &mut self.doc.def.placements[i];
                            ui.label(
                                RichText::new(format!("{name} ({})", p.id))
                                    .font(theme::body_bold_font(15.5))
                                    .color(theme::PARCHMENT),
                            );
                            ui.horizontal(|ui| {
                                ui.label("Seite");
                                changed |=
                                    ui.selectable_value(&mut p.owner, 0, "Glutkrone").changed();
                                changed |= ui.selectable_value(&mut p.owner, 1, "Hohle").changed();
                            });
                            let mut tag = p.tag.clone().unwrap_or_default();
                            ui.horizontal(|ui| {
                                ui.label("Tag");
                                if ui
                                    .add(
                                        egui::TextEdit::singleline(&mut tag)
                                            .desired_width(150.0)
                                            .hint_text("für Auslöser"),
                                    )
                                    .changed()
                                {
                                    p.tag = if tag.trim().is_empty() {
                                        None
                                    } else {
                                        Some(tag.trim().to_string())
                                    };
                                    changed = true;
                                }
                            });
                            if !is_building {
                                ui.horizontal(|ui| {
                                    ui.label("Anzahl");
                                    changed |= ui
                                        .add(egui::DragValue::new(&mut p.count).range(1..=60))
                                        .changed();
                                });
                                changed |= ui
                                    .checkbox(&mut p.guard, "Wächter-Haltung (bleibt am Platz)")
                                    .changed();
                            }
                            ui.label(
                                RichText::new(format!("Kachel {:.1} | {:.1}", p.x, p.y))
                                    .font(theme::italic_font(13.0))
                                    .color(theme::PARCHMENT_DIM),
                            );
                        }
                        Some(Sel::Node(i)) if i < self.doc.map.nodes.len() => {
                            key = "text-node";
                            let nd = &mut self.doc.map.nodes[i];
                            ui.label(
                                RichText::new(nd.kind.label())
                                    .font(theme::body_bold_font(15.5))
                                    .color(theme::PARCHMENT),
                            );
                            ui.horizontal(|ui| {
                                ui.label("Vorrat");
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(&mut nd.amount)
                                            .range(0..=100_000)
                                            .speed(10.0),
                                    )
                                    .changed();
                            });
                            ui.label(
                                RichText::new("0 = Standardvorrat der Regeln")
                                    .font(theme::italic_font(13.0))
                                    .color(theme::PARCHMENT_DIM),
                            );
                        }
                        Some(Sel::Decor(i)) if i < self.doc.map.decor.len() => {
                            key = "text-decor";
                            let d = &mut self.doc.map.decor[i];
                            ui.label(
                                RichText::new(d.kind.label())
                                    .font(theme::body_bold_font(15.5))
                                    .color(theme::PARCHMENT),
                            );
                            let mut v = d.variant as u32;
                            ui.horizontal(|ui| {
                                ui.label("Variante");
                                if ui.add(egui::DragValue::new(&mut v).range(0..=7)).changed() {
                                    d.variant = v as u8;
                                    changed = true;
                                }
                            });
                        }
                        Some(Sel::Start(i)) if i < self.doc.def.starts.len() => {
                            key = "text-start";
                            let s = &mut self.doc.def.starts[i];
                            ui.label(
                                RichText::new("Startposition")
                                    .font(theme::body_bold_font(15.5))
                                    .color(theme::PARCHMENT),
                            );
                            ui.horizontal(|ui| {
                                changed |=
                                    ui.selectable_value(&mut s.owner, 0, "Glutkrone").changed();
                                changed |= ui.selectable_value(&mut s.owner, 1, "Hohle").changed();
                            });
                        }
                        Some(Sel::Area(i)) if i < self.doc.def.areas.len() => {
                            key = "text-area";
                            let (mw, mh) = (self.doc.map.width, self.doc.map.height);
                            let a = &mut self.doc.def.areas[i];
                            ui.horizontal(|ui| {
                                ui.label("ID");
                                changed |= ui
                                    .add(egui::TextEdit::singleline(&mut a.id).desired_width(150.0))
                                    .changed();
                            });
                            ui.horizontal(|ui| {
                                ui.label("Name");
                                changed |= ui
                                    .add(
                                        egui::TextEdit::singleline(&mut a.name)
                                            .desired_width(150.0),
                                    )
                                    .changed();
                            });
                            egui::Grid::new("area_geo").num_columns(4).show(ui, |ui| {
                                ui.label("x");
                                changed |= ui
                                    .add(egui::DragValue::new(&mut a.x).range(0..=mw - 1))
                                    .changed();
                                ui.label("y");
                                changed |= ui
                                    .add(egui::DragValue::new(&mut a.y).range(0..=mh - 1))
                                    .changed();
                                ui.end_row();
                                ui.label("Breite");
                                changed |= ui
                                    .add(egui::DragValue::new(&mut a.w).range(1..=mw))
                                    .changed();
                                ui.label("Höhe");
                                changed |= ui
                                    .add(egui::DragValue::new(&mut a.h).range(1..=mh))
                                    .changed();
                                ui.end_row();
                            });
                        }
                        _ => {
                            key = "";
                            self.selection = None;
                        }
                    }
                    if changed {
                        self.commit(key);
                    }
                    if self.selection.is_some() {
                        ui.add_space(6.0);
                        if ui_common::button(ui, engine, art, "Löschen (Entf)", 200.0) {
                            self.delete_selection();
                        }
                    }
                    ui.add_space(8.0);
                    ui.separator();
                    ui.label(
                        RichText::new(format!(
                            "{} Gebäude · {} Einheiten · {} Quellen/Bäume",
                            b, u, n
                        ))
                        .font(theme::italic_font(13.5))
                        .color(theme::PARCHMENT_DIM),
                    );
                    let (e, w) = (
                        self.issues
                            .iter()
                            .filter(|i| i.level == Level::Error)
                            .count(),
                        self.issues
                            .iter()
                            .filter(|i| i.level == Level::Warning)
                            .count(),
                    );
                    if !self.issues_dirty && (e > 0 || w > 0) {
                        ui.label(
                            RichText::new(format!("Prüfung: {e} Fehler, {w} Hinweise"))
                                .font(theme::body_bold_font(14.0))
                                .color(if e > 0 { theme::EMBER } else { theme::GOLD }),
                        );
                    }
                });
            });
    }

    fn status_bar(&mut self, ctx: &egui::Context, content: &Content, art: &UiArt) {
        egui::Area::new(Id::unique("editor_status"))
            .order(Order::Foreground)
            .anchor(Align2::CENTER_BOTTOM, Vec2::new(0.0, -8.0))
            .interactable(false)
            .show(ctx, |ui| {
                art.panel(ui, PanelKind::Thin, Margin::symmetric(18, 8), |ui| {
                    let pos = self
                        .mouse_tile()
                        .map(|p| format!("Kachel {} | {}", p.x.floor() as i32, p.y.floor() as i32))
                        .unwrap_or_else(|| "–".into());
                    let terrain = self
                        .mouse_tile()
                        .and_then(|p| {
                            self.doc
                                .map
                                .terrain_at(IVec2::new(p.x.floor() as i32, p.y.floor() as i32))
                        })
                        .map(|t| t.label())
                        .unwrap_or("");
                    let _ = content;
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "{}×{} · {pos} {terrain}",
                                self.doc.map.width, self.doc.map.height
                            ))
                            .font(theme::body_font(14.5))
                            .color(theme::PARCHMENT_DIM),
                        );
                        if let Some((t, _, err)) = &self.status {
                            ui.label(RichText::new("·").color(theme::PARCHMENT_DIM));
                            ui.label(RichText::new(t).font(theme::body_bold_font(14.5)).color(
                                if *err {
                                    theme::EMBER
                                } else {
                                    Color32::from_rgb(150, 220, 130)
                                },
                            ));
                        }
                    });
                });
            });
    }

    // ------------------------------------------------------------------ Dialoge

    fn dialog_frame(
        &mut self,
        ctx: &egui::Context,
        art: &UiArt,
        name: &'static str,
        title: &str,
        width: f32,
        add: impl FnOnce(&mut Self, &mut egui::Ui),
    ) {
        // Abdunkelung hinter dem Dialog.
        let screen = ctx.content_rect();
        ctx.layer_painter(egui::LayerId::new(
            Order::Middle,
            Id::unique(("editor_dim", name)),
        ))
        .rect_filled(screen, 0.0, Color32::from_black_alpha(120));
        egui::Area::new(Id::unique(name))
            .order(Order::Foreground)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                art.panel(ui, PanelKind::Ornate, Margin::symmetric(32, 26), |ui| {
                    ui.set_width(width);
                    ui_art::heading(ui, art, title, 24.0);
                    add(self, ui);
                });
            });
    }

    fn new_map_dialog(
        &mut self,
        ctx: &egui::Context,
        engine: &mut Engine,
        content: &Content,
        art: &UiArt,
    ) {
        self.dialog_frame(ctx, art, "editor_new", "Neue Karte", 620.0, |s, ui| {
            egui::Grid::new("newmap")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .min_col_width(130.0)
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("Breite")
                            .font(theme::caps_font(13.5))
                            .color(LABEL),
                    );
                    ui.add(egui::Slider::new(&mut s.new_form.width, 32..=128).step_by(8.0));
                    ui.end_row();
                    ui.label(
                        RichText::new("Höhe")
                            .font(theme::caps_font(13.5))
                            .color(LABEL),
                    );
                    ui.add(egui::Slider::new(&mut s.new_form.height, 32..=128).step_by(8.0));
                    ui.end_row();
                    ui.label(
                        RichText::new("Seed")
                            .font(theme::caps_font(13.5))
                            .color(LABEL),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut s.new_form.seed));
                        if ui.button("Würfeln").clicked() {
                            s.new_form.seed =
                                (ui.ctx().input(|i| i.time) * 1000.0) as u64 % 100_000;
                        }
                    });
                    ui.end_row();
                    ui.label(
                        RichText::new("Leere Karte")
                            .font(theme::caps_font(13.5))
                            .color(LABEL),
                    );
                    ui.checkbox(&mut s.new_form.blank, "Nur Gras, ohne Generator");
                    ui.end_row();
                });
            if !s.new_form.blank {
                ui.add_space(6.0);
                ui.label(
                    RichText::new("Generator-Vorlage")
                        .font(theme::caps_font(13.5))
                        .color(LABEL),
                );
                ui.horizontal_wrapped(|ui| {
                    for p in GenPreset::ALL {
                        let r = ui_art::choice(ui, art, p.label(), s.new_form.preset == p);
                        ui_common::sfx(engine, ui, &r);
                        if r.clicked() {
                            s.new_form.preset = p;
                        }
                    }
                });
                ui.label(
                    RichText::new(s.new_form.preset.description())
                        .font(theme::italic_font(14.5))
                        .color(theme::PARCHMENT_DIM),
                );
            }
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 440.0).max(0.0) / 2.0);
                if ui_common::button(ui, engine, art, "Erzeugen", 220.0) {
                    s.new_map(&content.data);
                    s.dialog = Dialog::None;
                }
                if ui_common::button(ui, engine, art, "Abbrechen", 200.0) {
                    s.dialog = Dialog::None;
                }
            });
        });
    }

    fn load_dialog(
        &mut self,
        ctx: &egui::Context,
        engine: &mut Engine,
        content: &Content,
        art: &UiArt,
    ) {
        self.dialog_frame(ctx, art, "editor_load", "Szenario laden", 640.0, |s, ui| {
            let mut pick = None;
            ui.label(
                RichText::new("Szenarien")
                    .font(theme::caps_font(13.5))
                    .color(LABEL),
            );
            egui::ScrollArea::vertical()
                .id_salt("load_list")
                .max_height(220.0)
                .show(ui, |ui| {
                    if s.saved_list.is_empty() {
                        ui.label(
                            RichText::new("Keine Szenarien gefunden.")
                                .font(theme::italic_font(14.5))
                                .color(theme::PARCHMENT_DIM),
                        );
                    }
                    for (path, label, _) in &s.saved_list {
                        if ui
                            .selectable_label(false, RichText::new(label).size(15.5))
                            .clicked()
                        {
                            pick = Some(path.clone());
                        }
                    }
                });
            ui.add_space(6.0);
            ui.label(
                RichText::new("Missionen als Vorlage")
                    .font(theme::caps_font(13.5))
                    .color(LABEL),
            );
            let mut template = None;
            for (i, m) in content.missions.iter().enumerate() {
                if ui
                    .selectable_label(false, RichText::new(&m.name).size(15.5))
                    .clicked()
                {
                    template = Some(i);
                }
            }
            if let Some(p) = pick {
                s.load_path(content, p);
                s.dialog = Dialog::None;
            }
            if let Some(i) = template {
                match super::doc::EditDoc::from_mission(&content.data, &content.missions[i]) {
                    Ok(mut doc) => {
                        doc.def.id = format!("{}_eigene", doc.def.id);
                        doc.def.name = format!("{} (eigene Fassung)", doc.def.name);
                        s.doc = doc;
                        s.after_template_load();
                        s.dialog = Dialog::None;
                    }
                    Err(e) => s.notify(e, true),
                }
            }
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                if ui_common::button(ui, engine, art, "Abbrechen", 200.0) {
                    s.dialog = Dialog::None;
                }
            });
        });
    }

    fn exit_dialog(
        &mut self,
        ctx: &egui::Context,
        engine: &mut Engine,
        content: &Content,
        art: &UiArt,
    ) {
        self.dialog_frame(
            ctx,
            art,
            "editor_exit",
            "Ungespeicherte Änderungen",
            560.0,
            |s, ui| {
                ui.label(
                    RichText::new("Die Karte wurde seit dem letzten Speichern verändert.")
                        .font(theme::body_font(16.0))
                        .color(theme::PARCHMENT),
                );
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui_common::button(ui, engine, art, "Speichern", 160.0)
                        && s.save(&content.data)
                    {
                        s.action = EditorAction::Exit;
                    }
                    if ui_common::button(ui, engine, art, "Verwerfen", 160.0) {
                        s.action = EditorAction::Exit;
                    }
                    if ui_common::button(ui, engine, art, "Abbrechen", 160.0) {
                        s.dialog = Dialog::None;
                    }
                });
            },
        );
    }

    fn data_dialog(
        &mut self,
        ctx: &egui::Context,
        engine: &mut Engine,
        content: &Content,
        art: &UiArt,
    ) {
        self.dialog_frame(ctx, art, "editor_data", "Missionsdaten", 980.0, |s, ui| {
            ui.horizontal(|ui| {
                for (t, label) in [
                    (DataTab::General, "Allgemein"),
                    (DataTab::Objectives, "Ziele"),
                    (DataTab::Triggers, "Auslöser"),
                    (DataTab::Rules, "Regeln"),
                    (DataTab::Areas, "Bereiche"),
                    (DataTab::Validation, "Prüfung"),
                ] {
                    let r = ui_art::choice(ui, art, label, s.data_tab == t);
                    ui_common::sfx(engine, ui, &r);
                    if r.clicked() {
                        s.data_tab = t;
                        if t == DataTab::Validation {
                            s.issues_dirty = true;
                            s.refresh_issues(&content.data);
                        }
                    }
                }
            });
            ui.add_space(8.0);
            let mut changed = false;
            let mut key = "text-data";
            match s.data_tab {
                DataTab::General => changed |= s.general_tab(ui),
                DataTab::Objectives => {
                    changed |= triggers::objectives_tab(ui, &mut s.doc);
                    key = "text-obj";
                }
                DataTab::Triggers => {
                    let lists = Lists::from_doc(&s.doc, &content.data);
                    let mut tui = std::mem::take(&mut s.trigger_ui);
                    changed |= triggers::triggers_tab(ui, &mut s.doc, &lists, &mut tui);
                    s.trigger_ui = tui;
                    key = "text-trg";
                }
                DataTab::Rules => changed |= s.rules_tab(ui, content),
                DataTab::Areas => {
                    changed |= s.areas_tab(ui);
                    key = "text-areas";
                }
                DataTab::Validation => s.validation_tab(ui),
            }
            if changed {
                s.commit(key);
            }
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                if ui_common::button(ui, engine, art, "Schließen", 200.0) {
                    s.dialog = Dialog::None;
                }
            });
        });
    }

    fn general_tab(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        let d = &mut self.doc.def;
        egui::Grid::new("general")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .min_col_width(120.0)
            .show(ui, |ui| {
                ui.label(
                    RichText::new("Name")
                        .font(theme::caps_font(13.5))
                        .color(LABEL),
                );
                changed |= ui
                    .add(egui::TextEdit::singleline(&mut d.name).desired_width(520.0))
                    .changed();
                ui.end_row();
                ui.label(
                    RichText::new("ID (Dateiname)")
                        .font(theme::caps_font(13.5))
                        .color(LABEL),
                );
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(egui::TextEdit::singleline(&mut d.id).desired_width(260.0))
                        .changed();
                    ui.label(
                        RichText::new("klein, ohne Leerzeichen")
                            .font(theme::italic_font(13.0))
                            .color(theme::PARCHMENT_DIM),
                    );
                });
                ui.end_row();
                ui.label(
                    RichText::new("Autor")
                        .font(theme::caps_font(13.5))
                        .color(LABEL),
                );
                changed |= ui
                    .add(egui::TextEdit::singleline(&mut d.author).desired_width(260.0))
                    .changed();
                ui.end_row();
                ui.label(
                    RichText::new("Jahr n.F.")
                        .font(theme::caps_font(13.5))
                        .color(LABEL),
                );
                ui.horizontal(|ui| {
                    let mut has = d.year.is_some();
                    if ui.checkbox(&mut has, "angeben").changed() {
                        d.year = if has { Some(897) } else { None };
                        changed = true;
                    }
                    if let Some(y) = &mut d.year {
                        changed |= ui.add(egui::DragValue::new(y).range(0..=2000)).changed();
                    }
                });
                ui.end_row();
                ui.label(
                    RichText::new("Kurzbeschreibung")
                        .font(theme::caps_font(13.5))
                        .color(LABEL),
                );
                changed |= ui
                    .add(
                        egui::TextEdit::multiline(&mut d.description)
                            .desired_rows(3)
                            .desired_width(520.0),
                    )
                    .changed();
                ui.end_row();
                ui.label(
                    RichText::new("Einsatzbesprechung")
                        .font(theme::caps_font(13.5))
                        .color(LABEL),
                );
                changed |= ui
                    .add(
                        egui::TextEdit::multiline(&mut d.briefing)
                            .desired_rows(7)
                            .desired_width(520.0),
                    )
                    .changed();
                ui.end_row();
            });
        changed
    }

    fn rules_tab(&mut self, ui: &mut egui::Ui, content: &Content) -> bool {
        let mut changed = false;
        let lists = Lists::from_doc(&self.doc, &content.data);
        let d = &mut self.doc.def;
        let lab = |ui: &mut egui::Ui, t: &str| {
            ui.label(RichText::new(t).font(theme::caps_font(13.5)).color(LABEL));
        };
        egui::ScrollArea::vertical()
            .id_salt("rules")
            .max_height(470.0)
            .show(ui, |ui| {
                egui::Grid::new("rules_grid")
                    .num_columns(2)
                    .spacing([20.0, 10.0])
                    .min_col_width(170.0)
                    .show(ui, |ui| {
                        lab(ui, "Startressourcen");
                        ui.horizontal(|ui| {
                            for (name, v) in [
                                ("Gold", &mut d.start_resources.gold),
                                ("Holz", &mut d.start_resources.wood),
                                ("Stein", &mut d.start_resources.stone),
                                ("Nahrung", &mut d.start_resources.food),
                            ] {
                                ui.label(name);
                                changed |= ui
                                    .add(egui::DragValue::new(v).range(0.0..=100_000.0).speed(5.0))
                                    .changed();
                            }
                        });
                        ui.end_row();
                        lab(ui, "Startwaren");
                        ui.vertical(|ui| {
                            let mut remove = None;
                            for (i, (item, amount)) in d.start_goods.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    if ui.small_button("×").clicked() {
                                        remove = Some(i);
                                    }
                                    egui::ComboBox::from_id_salt(("sg", i))
                                        .selected_text(item.as_str())
                                        .show_ui(ui, |ui| {
                                            for (id, name) in &lists.items {
                                                if ui
                                                    .selectable_label(
                                                        item == id,
                                                        format!("{name} ({id})"),
                                                    )
                                                    .clicked()
                                                {
                                                    *item = id.clone();
                                                    changed = true;
                                                }
                                            }
                                        });
                                    changed |= ui
                                        .add(
                                            egui::DragValue::new(amount)
                                                .range(0.0..=10_000.0)
                                                .speed(1.0),
                                        )
                                        .changed();
                                });
                            }
                            if let Some(i) = remove {
                                d.start_goods.remove(i);
                                changed = true;
                            }
                            if ui.small_button("+ Ware").clicked() {
                                d.start_goods.push(("planks".into(), 20.0));
                                changed = true;
                            }
                        });
                        ui.end_row();
                        lab(ui, "Angriffswellen");
                        changed |= ui
                            .checkbox(
                                &mut d.waves.enabled,
                                "aktiv (benötigt das Tag der gegnerischen Basis)",
                            )
                            .changed();
                        ui.end_row();
                        lab(ui, "Erste Welle (s)");
                        changed |= ui
                            .add(egui::DragValue::new(&mut d.waves.first_wave).range(10.0..=3600.0))
                            .changed();
                        ui.end_row();
                        lab(ui, "Abstand Tag / Nacht (s)");
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut d.waves.interval_day)
                                        .range(20.0..=1200.0),
                                )
                                .changed();
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut d.waves.interval_night)
                                        .range(20.0..=1200.0),
                                )
                                .changed();
                        });
                        ui.end_row();
                        lab(ui, "Wellenbudget");
                        ui.horizontal(|ui| {
                            ui.label("Basis");
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut d.waves.base_budget)
                                        .range(0.5..=100.0)
                                        .speed(0.1),
                                )
                                .changed();
                            ui.label("pro Welle");
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut d.waves.budget_per_wave)
                                        .range(0.0..=50.0)
                                        .speed(0.1),
                                )
                                .changed();
                            ui.label("Nacht ×");
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut d.waves.night_mult)
                                        .range(1.0..=5.0)
                                        .speed(0.05),
                                )
                                .changed();
                        });
                        ui.end_row();
                        lab(ui, "Niederlage-Tag");
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut d.defeat_tag)
                                        .desired_width(150.0),
                                )
                                .changed();
                            ui.label(
                                RichText::new(
                                    "fällt das Gebäude, ist die Partie verloren (leer = nie)",
                                )
                                .font(theme::italic_font(13.0))
                                .color(theme::PARCHMENT_DIM),
                            );
                        });
                        ui.end_row();
                        lab(ui, "Tag der gegnerischen Basis");
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut d.enemy_base_tag)
                                        .desired_width(150.0),
                                )
                                .changed();
                            ui.label(
                                RichText::new("Ausgangspunkt der Wellen")
                                    .font(theme::italic_font(13.0))
                                    .color(theme::PARCHMENT_DIM),
                            );
                        });
                        ui.end_row();
                        lab(ui, "Truppenobergrenze");
                        ui.horizontal(|ui| {
                            let mut has = d.unit_cap.is_some();
                            if ui.checkbox(&mut has, "begrenzen").changed() {
                                d.unit_cap = if has { Some(12) } else { None };
                                changed = true;
                            }
                            if let Some(c) = &mut d.unit_cap {
                                changed |= ui.add(egui::DragValue::new(c).range(0..=200)).changed();
                                ui.label(
                                    RichText::new("Soldaten ohne Arbeiter und Heldin")
                                        .font(theme::italic_font(13.0))
                                        .color(theme::PARCHMENT_DIM),
                                );
                            }
                        });
                        ui.end_row();
                        lab(ui, "Tageszeit zu Beginn");
                        ui.horizontal(|ui| {
                            let mut has = d.start_time.is_some();
                            if ui.checkbox(&mut has, "festlegen").changed() {
                                d.start_time = if has { Some(0.3) } else { None };
                                changed = true;
                            }
                            if let Some(t) = &mut d.start_time {
                                changed |= ui.add(egui::Slider::new(t, 0.0..=1.0)).changed();
                            }
                        });
                        ui.end_row();
                    });
            });
        changed
    }

    fn areas_tab(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        let (mw, mh) = (self.doc.map.width, self.doc.map.height);
        ui.label(RichText::new("Bereiche zieht Ihr mit dem Werkzeug „Bereiche“ auf der Karte auf. Hier benennt und ordnet Ihr sie.").font(theme::italic_font(14.0)).color(theme::PARCHMENT_DIM));
        let mut remove = None;
        let mut jump = None;
        egui::ScrollArea::vertical()
            .id_salt("areas")
            .max_height(420.0)
            .show(ui, |ui| {
                for (i, a) in self.doc.def.areas.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.small_button("×").clicked() {
                            remove = Some(i);
                        }
                        if ui
                            .small_button("Hin")
                            .on_hover_text("Kamera zum Bereich")
                            .clicked()
                        {
                            jump = Some(a.center());
                        }
                        changed |= ui
                            .add(egui::TextEdit::singleline(&mut a.id).desired_width(120.0))
                            .changed();
                        changed |= ui
                            .add(egui::TextEdit::singleline(&mut a.name).desired_width(200.0))
                            .changed();
                        for v in [&mut a.x, &mut a.y] {
                            changed |= ui
                                .add(egui::DragValue::new(v).range(0..=mw.max(mh) - 1))
                                .changed();
                        }
                        ui.label("Größe");
                        changed |= ui
                            .add(egui::DragValue::new(&mut a.w).range(1..=mw))
                            .changed();
                        changed |= ui
                            .add(egui::DragValue::new(&mut a.h).range(1..=mh))
                            .changed();
                    });
                }
            });
        if let Some(i) = remove {
            self.doc.def.areas.remove(i);
            changed = true;
        }
        if let Some(p) = jump {
            self.center_on(p);
            self.dialog = Dialog::None;
        }
        changed
    }

    fn validation_tab(&mut self, ui: &mut egui::Ui) {
        let mut jump = None;
        if self.issues.is_empty() {
            ui.label(
                RichText::new("Keine Beanstandungen – das Szenario ist spielbar.")
                    .font(theme::body_bold_font(16.0))
                    .color(Color32::from_rgb(150, 220, 130)),
            );
        }
        egui::ScrollArea::vertical()
            .id_salt("issues")
            .max_height(430.0)
            .show(ui, |ui| {
                for i in &self.issues {
                    ui.horizontal_top(|ui| {
                        let col = match i.level {
                            Level::Error => theme::BLOOD_BRIGHT,
                            Level::Warning => theme::GOLD,
                            Level::Info => theme::PARCHMENT_DIM,
                        };
                        ui.label(
                            RichText::new(i.level.label())
                                .font(theme::caps_font(13.0))
                                .color(col),
                        );
                        ui.vertical(|ui| {
                            ui.set_max_width(780.0);
                            ui.label(RichText::new(&i.text).size(15.0).color(theme::PARCHMENT));
                        });
                        if let Some(p) = i.at
                            && ui.small_button("Hin").clicked()
                        {
                            jump = Some(p);
                        }
                    });
                    ui.separator();
                }
            });
        if let Some(p) = jump {
            self.center_on(p);
            self.dialog = Dialog::None;
        }
    }
}
