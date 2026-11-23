//! Fenster der Weltkarte: Provinz, Reich, Dynastie, Adel & Intrigen, Diplomatie, Heere, Wirtschaft, Glaube, Magie,
//! Chronik und Siegziele. Alle Spielerhandlungen laufen über `StrategyCommand`.

use super::mapview::{prosperity, religion_rgb};
use super::widgets::*;
use ashen_engine::ui::theme;
use ashen_strategy::catalog::{self, GRADE_ASHGLASS, GRADE_POINTS, RITUALS};
use ashen_strategy::goods::{BUILDINGS, Good};
use ashen_strategy::*;
use egui::{Align2, Color32, Pos2, Rect, RichText, Sense, Ui, Vec2};

const OK: Color32 = Color32::from_rgb(150, 210, 120);
const WARN: Color32 = Color32::from_rgb(236, 120, 90);

pub fn draw_all(ctx: &egui::Context, cx: &mut Cx, open: &[WinKind]) {
    for &k in open {
        let close = match k {
            WinKind::Province => province(ctx, cx),
            WinKind::Realm => realm(ctx, cx),
            WinKind::Family => family(ctx, cx),
            WinKind::Court => court(ctx, cx),
            WinKind::Diplomacy => diplomacy(ctx, cx),
            WinKind::Military => military(ctx, cx),
            WinKind::Economy => economy(ctx, cx),
            WinKind::Religion => religion(ctx, cx),
            WinKind::Magic => magic(ctx, cx),
            WinKind::Chronicle => chronicle(ctx, cx),
            WinKind::Victory => victory(ctx, cx),
        };
        if close {
            cx.reqs.push(Req::Close(k));
        }
    }
}

fn gold(v: f32) -> String {
    format!("{v:.0} Gold")
}

// -------------------------------------------------------------------------------------------------
// Provinz
// -------------------------------------------------------------------------------------------------

fn province(ctx: &egui::Context, cx: &mut Cx) -> bool {
    let Some(pid) = cx.selected else {
        return window(ctx, cx, WinKind::Province, "Provinz", "", |_, ui| {
            dim(ui, "Wählt eine Provinz auf der Karte.")
        });
    };
    let w = cx.w;
    let p = &w.provinces[pid as usize];
    let region = w
        .regions
        .get(p.region as usize)
        .map(|r| r.name.clone())
        .unwrap_or_default();
    let sub = format!("{region} · {} · {}", p.biome.label(), p.settlement.name());
    window(ctx, cx, WinKind::Province, &p.name, &sub, |cx, ui| {
        let w = cx.w;
        let p = &w.provinces[pid as usize];
        let mine = p.owner == Some(cx.me);
        ui.horizontal(|ui| {
            match p.holder() {
                Some(h) => realm_chip(ui, w, h),
                None => dim(ui, "herrenlos"),
            }
            if let Some(f) = p.fief {
                dim(
                    ui,
                    format!("· Lehen von Haus {}", w.houses[f as usize].name),
                );
            }
            if p.holder() != p.owner && p.owner.is_some() {
                ui.label(
                    RichText::new("besetzt")
                        .font(theme::caps_font(12.0))
                        .color(WARN),
                );
            }
            if p.holder().is_some() && ui.small_button("Diplomatie").clicked() {
                cx.st.dipl_target = p.holder().filter(|h| *h != cx.me);
                cx.reqs.push(Req::Open(WinKind::Diplomacy));
            }
        });
        section(ui, cx.art, "Siedlung");
        kv(ui, "Einwohner", format!("{:.0}", p.population()));
        kv(ui, "Kultur", p.culture.label());
        kv(
            ui,
            "Wohnraum",
            format!("{:.0} von {:.0}", p.population(), p.housing_capacity()),
        );
        kv(
            ui,
            "Befestigung",
            format!("Stufe {} von 10 · Garnison {}", p.fortification, p.garrison),
        );
        bar_row(
            ui,
            cx.art,
            "Zufriedenheit",
            p.avg_satisfaction() / 100.0,
            good_col(p.avg_satisfaction()),
            &format!("{:.0}", p.avg_satisfaction()),
        );
        bar_row(
            ui,
            cx.art,
            "Unruhe",
            p.unrest / 100.0,
            bad_col(p.unrest),
            &format!("{:.0}", p.unrest),
        );
        bar_row(
            ui,
            cx.art,
            "Hohlung",
            p.hollowing / 100.0,
            Color32::from_rgb(90, 220, 140),
            &format!("{:.0}", p.hollowing),
        );
        kv(ui, "Schleierstärke", format!("{:.1} von 5", p.veil));
        bar_row(
            ui,
            cx.art,
            "Ernährung",
            (p.food_ratio / 1.2).min(1.0),
            if p.food_ratio < 0.9 { WARN } else { OK },
            &format!("{:.0} %", p.food_ratio * 100.0),
        );
        if let Some(pl) = &p.plague {
            ui.label(
                RichText::new(format!(
                    "Seuche seit Tag {} (Schwere {:.0})",
                    pl.start_day, pl.severity
                ))
                .font(theme::body_bold_font(14.0))
                .color(Color32::from_rgb(160, 220, 90)),
            );
        }
        if let Some(s) = &p.siege {
            ui.label(
                RichText::new(format!(
                    "Belagert von {} – Fortschritt {:.0} %",
                    w.rname(s.besieger),
                    s.progress
                ))
                .font(theme::body_bold_font(14.0))
                .color(WARN),
            );
        }
        section(ui, cx.art, "Bevölkerungsschichten");
        egui::Grid::new("prov_strata")
            .num_columns(3)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                for s in STRATA {
                    let sp = &p.pop[s.idx()];
                    ui.label(
                        RichText::new(s.name())
                            .font(theme::body_font(14.0))
                            .color(theme::PARCHMENT_DIM),
                    );
                    ui.label(
                        RichText::new(format!("{:.0}", sp.total()))
                            .font(theme::body_bold_font(14.0))
                            .color(theme::PARCHMENT),
                    );
                    let (r, _) = ui.allocate_exact_size(Vec2::new(170.0, 16.0), Sense::hover());
                    cx.art.bar(
                        ui.painter(),
                        r,
                        sp.satisfaction / 100.0,
                        good_col(sp.satisfaction),
                        &format!("{:.0}", sp.satisfaction),
                    );
                    ui.end_row();
                }
            });
        section(ui, cx.art, "Religion");
        for r in RELIGIONS {
            let share = p.faith[r.idx()];
            if share < 0.01 {
                continue;
            }
            let c = religion_rgb(r);
            bar_row(
                ui,
                cx.art,
                r.name(),
                share,
                Color32::from_rgb(c[0] as u8, c[1] as u8, c[2] as u8),
                &format!("{:.0} %", share * 100.0),
            );
        }
        if let Some(h) = p.holy_site {
            dim(ui, format!("Heilige Stätte des Glaubens: {}", h.name()));
        }
        section(ui, cx.art, "Gebäude");
        if p.buildings.is_empty() {
            dim(ui, "Keine Gebäude.");
        } else {
            ui.horizontal_wrapped(|ui| {
                for (i, n) in &p.buildings {
                    ui.label(
                        RichText::new(format!("{}× {}", n, BUILDINGS[*i as usize].name))
                            .font(theme::body_font(14.0))
                            .color(theme::PARCHMENT),
                    );
                    ui.label(RichText::new("·").color(theme::GOLD_DIM));
                }
            });
        }
        if !p.deposits.is_empty() {
            dim(
                ui,
                format!(
                    "Vorkommen: {}",
                    p.deposits
                        .iter()
                        .map(|(g, r)| format!("{} {}", g.name(), r))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            );
        }
        if mine {
            section(ui, cx.art, "Steuern des Reiches");
            let lvl = w.realms[cx.me as usize].tax_level;
            ui.horizontal(|ui| {
                for (i, n) in ["Erlass", "Niedrig", "Normal", "Hoch", "Erdrückend"]
                    .iter()
                    .enumerate()
                {
                    if cx.choice(ui, n, lvl as usize == i) {
                        cx.cmd(StrategyCommand::SetTaxLevel(i as u8));
                    }
                }
            });
            dim(
                ui,
                "Hohe Steuern füllen die Kasse, senken aber die Zufriedenheit aller Provinzen.",
            );
            section(ui, cx.art, "Ausbau");
            let up_cost = 120.0 * (p.fortification as f32 + 1.0);
            ui.horizontal(|ui| {
                if cx.button(
                    ui,
                    &format!("Befestigung ausbauen ({})", gold(up_cost)),
                    290.0,
                    p.fortification < 10,
                ) {
                    cx.cmd(StrategyCommand::UpgradeFortification(pid));
                }
            });
            ui.add_space(4.0);
            egui::CollapsingHeader::new(
                RichText::new("Gebäude bauen")
                    .font(theme::heading_font(15.0))
                    .color(theme::GOLD),
            )
            .default_open(true)
            .show(ui, |ui| {
                for (bi, b) in BUILDINGS.iter().enumerate() {
                    let ok_deposit = b.deposit.is_none_or(|g| p.deposit(g) > 0);
                    let can = ok_deposit && w.can_build(pid, bi).is_ok();
                    if !can && !ok_deposit {
                        continue;
                    }
                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(can, |ui| {
                            ui.set_width(220.0);
                            ui.label(RichText::new(b.name).font(theme::body_font(14.0)).color(
                                if can {
                                    theme::PARCHMENT
                                } else {
                                    theme::PARCHMENT_DIM
                                },
                            ));
                        });
                        dim(ui, gold(b.cost));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if cx.button(
                                ui,
                                "Bauen",
                                80.0,
                                can && w.realms[cx.me as usize].gold >= b.cost,
                            ) {
                                cx.cmd(StrategyCommand::Build {
                                    province: pid,
                                    building: b.id.to_string(),
                                });
                            }
                        });
                    });
                }
            });
            feedback(ui, cx.last_result);
        }
    })
}

// -------------------------------------------------------------------------------------------------
// Reich
// -------------------------------------------------------------------------------------------------

fn realm(ctx: &egui::Context, cx: &mut Cx) -> bool {
    let w = cx.w;
    let r = &w.realms[cx.me as usize];
    let sub = format!(
        "{} · {} · {}",
        r.rank.name(),
        r.gov.name(),
        w.chars[r.ruler as usize].culture.label()
    );
    window(ctx, cx, WinKind::Realm, &r.name, &sub, |cx, ui| {
        let w = cx.w;
        let r = &w.realms[cx.me as usize];
        let ruler = &w.chars[r.ruler as usize];
        section(ui, cx.art, "Herrscher");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                heading(
                    ui,
                    format!(
                        "{}{}",
                        w.full_name(r.ruler),
                        ruler
                            .epithet
                            .as_ref()
                            .map(|e| format!(" {e}"))
                            .unwrap_or_default()
                    ),
                );
                dim(
                    ui,
                    format!(
                        "{} Jahre · Gesundheit {:.0} · Stress {:.0}",
                        ruler.age(w.day),
                        ruler.health,
                        ruler.stress
                    ),
                );
                dim(
                    ui,
                    ruler
                        .traits
                        .iter()
                        .map(|t| t.name())
                        .collect::<Vec<_>>()
                        .join(", "),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if cx.button(ui, "Dynastie", 110.0, true) {
                    cx.st.family_sel = Some(r.ruler);
                    cx.reqs.push(Req::Open(WinKind::Family));
                }
            });
        });
        if let Some(reg) = &r.regency {
            ui.label(
                RichText::new(format!(
                    "Regentschaft: {} (Loyalität {:.0})",
                    w.full_name(reg.regent),
                    reg.loyalty
                ))
                .font(theme::body_bold_font(14.0))
                .color(WARN),
            );
        }
        section(ui, cx.art, "Zustand");
        bar_row(
            ui,
            cx.art,
            "Legitimität",
            r.legitimacy / 100.0,
            good_col(r.legitimacy),
            &format!("{:.0}", r.legitimacy),
        );
        bar_row(
            ui,
            cx.art,
            "Tyrannei",
            r.tyranny / 100.0,
            bad_col(r.tyranny),
            &format!("{:.0}", r.tyranny),
        );
        bar_row(
            ui,
            cx.art,
            "Kriegsmüdigkeit",
            r.war_weariness / 100.0,
            bad_col(r.war_weariness),
            &format!("{:.0}", r.war_weariness),
        );
        kv(ui, "Prestige", format!("{:.0}", r.prestige));
        kv(ui, "Frömmigkeit", format!("{:.0}", r.piety));
        kv(ui, "Provinzen", format!("{}", w.provinces_of(cx.me).len()));
        kv(
            ui,
            "Lehnsherr",
            r.vassal_of
                .map(|v| w.rname(v))
                .unwrap_or_else(|| "unabhängig".into()),
        );
        section(ui, cx.art, "Ämter");
        let candidates: Vec<CharId> = w
            .chars
            .iter()
            .filter(|c| {
                c.realm == Some(cx.me) && c.alive() && c.adult(w.day) && c.captive_of.is_none()
            })
            .map(|c| c.id)
            .collect();
        for o in OFFICES {
            let holder = r.offices[o.idx()];
            ui.horizontal(|ui| {
                ui.set_min_width(100.0);
                ui.label(
                    RichText::new(o.name())
                        .font(theme::body_bold_font(14.5))
                        .color(theme::GOLD),
                );
                ui.add_space(8.0);
                match holder {
                    Some(c) => {
                        ui.label(
                            RichText::new(format!(
                                "{} ({} {})",
                                w.cname(c),
                                o.attr().name(),
                                w.chars[c as usize].attr(o.attr())
                            ))
                            .font(theme::body_font(14.0))
                            .color(theme::PARCHMENT),
                        );
                    }
                    None => dim(ui, "unbesetzt"),
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if holder.is_some() && cx.button(ui, "Entlassen", 90.0, true) {
                        cx.cmd(StrategyCommand::DismissOffice(o));
                    }
                    let mut chosen: Option<CharId> = None;
                    egui::ComboBox::from_id_salt(("office", o.idx()))
                        .width(130.0)
                        .selected_text(RichText::new("Ernennen …").font(theme::body_font(14.0)))
                        .show_ui(ui, |ui| {
                            let mut list = candidates.clone();
                            list.sort_by_key(|c| {
                                std::cmp::Reverse(w.chars[*c as usize].attr(o.attr()))
                            });
                            for c in list.into_iter().take(12) {
                                let ch = &w.chars[c as usize];
                                if ui
                                    .selectable_label(
                                        false,
                                        format!(
                                            "{} · {} {}",
                                            ch.name,
                                            o.attr().name(),
                                            ch.attr(o.attr())
                                        ),
                                    )
                                    .clicked()
                                {
                                    chosen = Some(c);
                                }
                            }
                        });
                    if let Some(c) = chosen {
                        cx.cmd(StrategyCommand::AppointOffice {
                            office: o,
                            character: c,
                        });
                    }
                });
            });
            if holder.is_some() {
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    dim(ui, "Aufgabe:");
                    for (i, t) in o.tasks().iter().enumerate() {
                        if cx.choice(ui, t, r.office_task[o.idx()] as usize == i) {
                            cx.cmd(StrategyCommand::SetOfficeTask {
                                office: o,
                                task: i as u8,
                            });
                        }
                    }
                });
            }
        }
        section(ui, cx.art, "Gesetze und Regierung");
        let laws = &r.laws;
        let row = |cx: &mut Cx,
                   ui: &mut Ui,
                   name: &str,
                   opts: &[&str],
                   cur: usize,
                   mk: &dyn Fn(u8) -> LawChange| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new(name)
                        .font(theme::body_font(14.0))
                        .color(theme::PARCHMENT_DIM),
                );
                for (i, o) in opts.iter().enumerate() {
                    if cx.choice(ui, o, cur == i) && cur != i {
                        cx.cmd(StrategyCommand::SetLaw(mk(i as u8)));
                    }
                }
            });
        };
        row(
            cx,
            ui,
            "Kronautorität",
            &["Schwach", "Begrenzt", "Stark", "Absolut"],
            laws.crown_authority as usize,
            &LawChange::CrownAuthority,
        );
        row(
            cx,
            ui,
            "Leibeigenschaft",
            &["Verboten", "Üblich", "Streng"],
            laws.serfdom as usize,
            &LawChange::Serfdom,
        );
        row(
            cx,
            ui,
            "Stadtrechte",
            &["Keine", "Übliche", "Freie Städte"],
            laws.town_rights as usize,
            &LawChange::TownRights,
        );
        row(
            cx,
            ui,
            "Glaubensgesetz",
            &["Staatskirche", "Duldung", "Freiheit"],
            laws.faith_law as usize,
            &LawChange::FaithLaw,
        );
        row(
            cx,
            ui,
            "Magiegesetz",
            &["Verboten", "Beaufsichtigt", "Frei"],
            laws.magic_law as usize,
            &LawChange::MagicLaw,
        );
        row(
            cx,
            ui,
            "Kriegsrecht",
            &["Aus", "Bereit", "Total"],
            laws.martial_law as usize,
            &LawChange::MartialLaw,
        );
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new("Erbgesetz")
                    .font(theme::body_font(14.0))
                    .color(theme::PARCHMENT_DIM),
            );
            for s in [
                SuccLaw::Primogeniture,
                SuccLaw::Ultimogeniture,
                SuccLaw::Partition,
                SuccLaw::Elective,
                SuccLaw::Seniority,
                SuccLaw::Designation,
            ] {
                if cx.choice(ui, s.name(), laws.succession == s) && laws.succession != s {
                    cx.cmd(StrategyCommand::SetLaw(LawChange::Succession(s)));
                }
            }
        });
        let mut vt = laws.vassal_tax;
        let mut levy = laws.levy;
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Vasallenabgabe")
                    .font(theme::body_font(14.0))
                    .color(theme::PARCHMENT_DIM),
            );
            if ui
                .add(
                    egui::Slider::new(&mut vt, 0.0..=50.0)
                        .suffix(" %")
                        .step_by(5.0),
                )
                .drag_stopped()
            {
                cx.cmd(StrategyCommand::SetLaw(LawChange::VassalTax(vt)));
            }
        });
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Heerfolge")
                    .font(theme::body_font(14.0))
                    .color(theme::PARCHMENT_DIM),
            );
            if ui
                .add(
                    egui::Slider::new(&mut levy, 0.0..=60.0)
                        .suffix(" %")
                        .step_by(5.0),
                )
                .drag_stopped()
            {
                cx.cmd(StrategyCommand::SetLaw(LawChange::Levy(levy)));
            }
        });
        feedback(ui, cx.last_result);
    })
}

// -------------------------------------------------------------------------------------------------
// Dynastie und Charaktere
// -------------------------------------------------------------------------------------------------

fn tree_node(
    cx: &mut Cx,
    ui: &mut Ui,
    painter: &egui::Painter,
    c: CharId,
    center: Pos2,
    tag: &str,
) -> Rect {
    let w = cx.w;
    let ch = &w.chars[c as usize];
    let r = Rect::from_center_size(center, Vec2::new(118.0, 42.0));
    let resp = ui.interact(r, egui::Id::unique(("tree", c)), Sense::click());
    let sel = cx.st.family_sel == Some(c);
    let base = if !ch.alive() {
        Color32::from_rgb(34, 30, 30)
    } else if ch.gender == Gender::Female {
        Color32::from_rgb(58, 34, 44)
    } else {
        Color32::from_rgb(34, 40, 56)
    };
    painter.rect_filled(
        r,
        2.0,
        if resp.hovered() {
            base.gamma_multiply(1.4)
        } else {
            base
        },
    );
    painter.rect_stroke(
        r,
        2.0,
        egui::Stroke::new(
            if sel { 2.0 } else { 1.0 },
            if sel {
                theme::GOLD_BRIGHT
            } else {
                theme::GOLD_DIM
            },
        ),
        egui::StrokeKind::Inside,
    );
    let name: String = ch.name.chars().take(15).collect();
    painter.text(
        r.center() + Vec2::new(0.0, -8.0),
        Align2::CENTER_CENTER,
        name,
        theme::body_bold_font(13.0),
        if ch.alive() {
            theme::PARCHMENT
        } else {
            theme::ASH
        },
    );
    let line = if ch.alive() {
        format!("{} J.{}", ch.age(w.day), tag)
    } else {
        format!(
            "† {}{}",
            ch.died.map(|d| Date::from_day(d.0).year).unwrap_or(0),
            tag
        )
    };
    painter.text(
        r.center() + Vec2::new(0.0, 10.0),
        Align2::CENTER_CENTER,
        line,
        theme::italic_font(12.0),
        theme::PARCHMENT_DIM,
    );
    if w.realms[cx.me as usize].ruler == c {
        painter.text(
            r.left_top() + Vec2::new(4.0, -1.0),
            Align2::LEFT_BOTTOM,
            "Herrscher",
            theme::caps_font(10.0),
            theme::GOLD,
        );
    } else if w.realms[cx.me as usize].heir == Some(c) {
        painter.text(
            r.left_top() + Vec2::new(4.0, -1.0),
            Align2::LEFT_BOTTOM,
            "Erbe",
            theme::caps_font(10.0),
            Color32::from_rgb(150, 200, 250),
        );
    }
    if resp.clicked() {
        cx.st.family_sel = Some(c);
    }
    r
}

fn family(ctx: &egui::Context, cx: &mut Cx) -> bool {
    let w = cx.w;
    let ruler = w.realms[cx.me as usize].ruler;
    let dyn_name = w
        .dynasties
        .get(w.chars[ruler as usize].dynasty as usize)
        .map(|d| d.name.clone())
        .unwrap_or_default();
    let dyn_prestige = w
        .dynasties
        .get(w.chars[ruler as usize].dynasty as usize)
        .map(|d| d.prestige)
        .unwrap_or(0.0);
    window(
        ctx,
        cx,
        WinKind::Family,
        &format!("Haus {dyn_name}"),
        &format!("Dynastieprestige {dyn_prestige:.0}"),
        |cx, ui| {
            let w = cx.w;
            let root = cx
                .st
                .family_sel
                .filter(|c| (*c as usize) < w.chars.len())
                .unwrap_or(ruler);
            let rc = &w.chars[root as usize];
            section(ui, cx.art, "Stammbaum");
            let height = 4.0 * 62.0 + 10.0;
            let (area, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
            let painter = ui.painter_at(area);
            painter.rect_filled(area, 2.0, Color32::from_rgb(20, 16, 14));
            let row_y = |i: i32| area.min.y + 30.0 + i as f32 * 62.0;
            let spread = |n: usize, i: usize| {
                area.min.x + area.width() * (i as f32 + 1.0) / (n as f32 + 1.0)
            };
            // Eltern
            let parents: Vec<CharId> = [rc.father, rc.mother].into_iter().flatten().collect();
            let mut prects = Vec::new();
            for (i, p) in parents.iter().enumerate() {
                let n = parents.len();
                let c = Pos2::new(
                    area.center().x - 70.0 + i as f32 * 140.0 - if n == 1 { 70.0 } else { 0.0 },
                    row_y(0),
                );
                prects.push(tree_node(cx, ui, &painter, *p, c, ""));
            }
            // Zeile 1: Geschwister + Person + Ehepartner
            let mut sibs: Vec<CharId> = Vec::new();
            for p in &parents {
                for c in &w.chars[*p as usize].children {
                    if *c != root && !sibs.contains(c) {
                        sibs.push(*c);
                    }
                }
            }
            sibs.truncate(3);
            let mut row1: Vec<(CharId, &str)> = sibs.iter().map(|s| (*s, "")).collect();
            row1.push((root, ""));
            if let Some(s) = rc.spouse {
                row1.push((s, ""));
            }
            let mut rects1 = Vec::new();
            for (i, (c, t)) in row1.iter().enumerate() {
                rects1.push((
                    *c,
                    tree_node(
                        cx,
                        ui,
                        &painter,
                        *c,
                        Pos2::new(spread(row1.len(), i), row_y(1)),
                        t,
                    ),
                ));
            }
            let line = egui::Stroke::new(1.0, theme::GOLD_DIM);
            if let Some(pr) = prects.first() {
                let mid = Pos2::new(
                    prects.iter().map(|r| r.center().x).sum::<f32>() / prects.len() as f32,
                    pr.max.y,
                );
                for (c, r) in &rects1 {
                    if *c != rc.spouse.unwrap_or(u32::MAX) {
                        painter.line_segment([mid, Pos2::new(r.center().x, r.min.y - 2.0)], line);
                    }
                }
            }
            // Kinder
            let kids: Vec<CharId> = rc.children.iter().copied().take(6).collect();
            let me_rect = rects1.iter().find(|(c, _)| *c == root).map(|(_, r)| *r);
            let sp_rect = rc
                .spouse
                .and_then(|s| rects1.iter().find(|(c, _)| *c == s).map(|(_, r)| *r));
            let mut kid_rects: Vec<(CharId, Rect)> = Vec::new();
            for (i, k) in kids.iter().enumerate() {
                let r = tree_node(
                    cx,
                    ui,
                    &painter,
                    *k,
                    Pos2::new(spread(kids.len(), i), row_y(2)),
                    "",
                );
                kid_rects.push((*k, r));
            }
            if let Some(mr) = me_rect {
                let from = match sp_rect {
                    Some(s) => Pos2::new((mr.center().x + s.center().x) / 2.0, mr.center().y),
                    None => Pos2::new(mr.center().x, mr.max.y),
                };
                if let Some(sr) = sp_rect {
                    painter.line_segment([mr.right_center(), sr.left_center()], line);
                }
                for (_, r) in &kid_rects {
                    painter.line_segment([from, Pos2::new(r.center().x, r.min.y - 2.0)], line);
                }
            }
            // Enkel (erste Kinder des ersten Kindes)
            if let Some((k0, r0)) = kid_rects.first() {
                let gk: Vec<CharId> = w.chars[*k0 as usize]
                    .children
                    .iter()
                    .copied()
                    .take(4)
                    .collect();
                for (i, g) in gk.iter().enumerate() {
                    let r = tree_node(
                        cx,
                        ui,
                        &painter,
                        *g,
                        Pos2::new(r0.center().x - 60.0 + i as f32 * 124.0, row_y(3)),
                        "",
                    );
                    painter.line_segment(
                        [r0.center_bottom(), Pos2::new(r.center().x, r.min.y - 2.0)],
                        line,
                    );
                }
            }
            // Detail der Auswahl
            let ch = &w.chars[root as usize];
            section(ui, cx.art, "Charakter");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    heading(
                        ui,
                        format!(
                            "{}{}",
                            w.full_name(root),
                            ch.epithet
                                .as_ref()
                                .map(|e| format!(" {e}"))
                                .unwrap_or_default()
                        ),
                    );
                    dim(
                        ui,
                        format!(
                            "{} · {} · {} · {}",
                            if ch.gender == Gender::Male {
                                "männlich"
                            } else {
                                "weiblich"
                            },
                            if ch.alive() {
                                format!("{} Jahre", ch.age(w.day))
                            } else {
                                format!("gestorben: {}", ch.died.map(|d| d.1.label()).unwrap_or(""))
                            },
                            ch.culture.label(),
                            ch.religion.name()
                        ),
                    );
                    dim(
                        ui,
                        format!(
                            "Aufenthalt: {} · Hof: {}",
                            w.pname(ch.location),
                            ch.realm
                                .map(|r| w.rname(r))
                                .unwrap_or_else(|| "keiner".into())
                        ),
                    );
                });
            });
            egui::Grid::new("attrs")
                .num_columns(4)
                .spacing([14.0, 4.0])
                .show(ui, |ui| {
                    for (i, a) in ATTRS.iter().enumerate() {
                        ui.label(
                            RichText::new(a.name())
                                .font(theme::body_font(14.0))
                                .color(theme::PARCHMENT_DIM),
                        );
                        let v = ch.attr(*a);
                        let (r, _) = ui.allocate_exact_size(Vec2::new(100.0, 16.0), Sense::hover());
                        cx.art.bar(
                            ui.painter(),
                            r,
                            v as f32 / 20.0,
                            Color32::from_rgb(200, 160, 70),
                            &v.to_string(),
                        );
                        if i % 2 == 1 {
                            ui.end_row();
                        }
                    }
                });
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("Eigenschaften:")
                        .font(theme::body_font(14.0))
                        .color(theme::PARCHMENT_DIM),
                );
                for t in &ch.traits {
                    let r = ui.label(
                        RichText::new(t.name())
                            .font(theme::body_bold_font(14.0))
                            .color(if t.acquired() {
                                Color32::from_rgb(210, 170, 120)
                            } else {
                                theme::PARCHMENT
                            }),
                    );
                    let at = t.attrs();
                    let mods: Vec<String> = ATTRS
                        .iter()
                        .filter(|a| at[a.idx()] != 0)
                        .map(|a| format!("{} {:+}", a.name(), at[a.idx()]))
                        .collect();
                    if !mods.is_empty() {
                        r.on_hover_text(mods.join(", "));
                    }
                }
                if ch.traits.is_empty() {
                    dim(ui, "keine");
                }
            });
            if !ch.abilities.is_empty() {
                dim(
                    ui,
                    format!(
                        "Fähigkeiten: {}",
                        ch.abilities
                            .iter()
                            .map(|a| a.name())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                );
            }
            kv(
                ui,
                "Gesundheit / Stress / Hohlung",
                format!("{:.0} / {:.0} / {:.0}", ch.health, ch.stress, ch.hollowing),
            );
            if !ch.claims.is_empty() {
                dim(
                    ui,
                    format!(
                        "Ansprüche auf: {}",
                        ch.claims
                            .iter()
                            .map(|r| w.rname(*r))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                );
            }
            let mine = ch.realm == Some(cx.me) && ch.alive();
            if mine {
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    if cx.button(ui, "Zum Erben bestimmen", 200.0, ch.adult(w.day) || true) {
                        cx.cmd(StrategyCommand::SetHeir(root));
                    }
                    if !ch.adult(w.day) {
                        ui.label(
                            RichText::new("Erziehung:")
                                .font(theme::body_font(14.0))
                                .color(theme::PARCHMENT_DIM),
                        );
                        for a in ATTRS {
                            if cx.choice(ui, a.name(), ch.education == Some(a)) {
                                cx.cmd(StrategyCommand::SetEducation {
                                    child: root,
                                    focus: a,
                                });
                            }
                        }
                    }
                });
                if ch.adult(w.day) && ch.spouse.is_none() {
                    section(ui, cx.art, "Ehe arrangieren");
                    let mut cands: Vec<CharId> = w
                        .chars
                        .iter()
                        .filter(|o| {
                            o.alive()
                                && o.adult(w.day)
                                && o.spouse.is_none()
                                && o.gender != ch.gender
                                && o.id != root
                                && o.captive_of.is_none()
                                && o.realm.is_some()
                                && !w.close_kin(root, o.id)
                        })
                        .map(|o| o.id)
                        .collect();
                    cands.sort_by(|a, b| {
                        let (ra, rb) = (
                            w.chars[*a as usize].realm.unwrap(),
                            w.chars[*b as usize].realm.unwrap(),
                        );
                        let key = |r: RealmId| {
                            if r == cx.me {
                                1000.0
                            } else {
                                w.opinion(r, cx.me)
                            }
                        };
                        key(rb).total_cmp(&key(ra))
                    });
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Mitgift")
                                .font(theme::body_font(14.0))
                                .color(theme::PARCHMENT_DIM),
                        );
                        ui.add(
                            egui::Slider::new(&mut cx.st.dowry, 0.0..=1000.0)
                                .suffix(" Gold")
                                .step_by(50.0),
                        );
                    });
                    for o in cands.into_iter().take(8) {
                        let oc = &w.chars[o as usize];
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!(
                                    "{} ({} J.) – {}",
                                    oc.name,
                                    oc.age(w.day),
                                    oc.realm.map(|r| w.rname(r)).unwrap_or_default()
                                ))
                                .font(theme::body_font(14.0))
                                .color(theme::PARCHMENT),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if cx.button(ui, "Anbieten", 90.0, true) {
                                        let d = cx.st.dowry;
                                        cx.cmd(StrategyCommand::ArrangeMarriage {
                                            own: root,
                                            other: o,
                                            dowry: d,
                                        });
                                    }
                                },
                            );
                        });
                    }
                }
            }
            feedback(ui, cx.last_result);
        },
    )
}

// -------------------------------------------------------------------------------------------------
// Adel & Intrigen
// -------------------------------------------------------------------------------------------------

fn plot_target(
    w: &StrategyWorld,
    me: RealmId,
    kind: PlotKind,
    prov: Option<ProvId>,
    other: Option<RealmId>,
) -> Option<PlotTarget> {
    match kind {
        PlotKind::Murder
        | PlotKind::Abduct
        | PlotKind::Slander
        | PlotKind::Seduce
        | PlotKind::Blackmail
        | PlotKind::Bribery
        | PlotKind::Treason => other.map(|r| PlotTarget::Char(w.realms[r as usize].ruler)),
        PlotKind::Incite | PlotKind::Espionage | PlotKind::Steal => prov.map(PlotTarget::Prov),
        PlotKind::Fabricate | PlotKind::SecretAlliance => other.map(PlotTarget::Realm),
        PlotKind::Usurp | PlotKind::Conspiracy => other
            .map(PlotTarget::Throne)
            .or(Some(PlotTarget::Throne(me))),
    }
}

fn court(ctx: &egui::Context, cx: &mut Cx) -> bool {
    window(
        ctx,
        cx,
        WinKind::Court,
        "Adel & Intrigen",
        "Häuser, Fraktionen und Komplotte",
        |cx, ui| {
            let w = cx.w;
            let houses: Vec<&NobleHouse> = w
                .houses
                .iter()
                .filter(|h| h.realm == cx.me && h.alive)
                .collect();
            section(ui, cx.art, "Adelshäuser");
            if houses.is_empty() {
                dim(ui, "Euer Reich kennt keine mächtigen Häuser.");
            }
            for h in &houses {
                let sel = cx.st.court_house == Some(h.id);
                ui.horizontal(|ui| {
                    if cx.choice(ui, &h.name, sel) {
                        cx.st.court_house = Some(h.id);
                    }
                    ui.label(
                        RichText::new(h.leaning.name())
                            .font(theme::italic_font(13.5))
                            .color(theme::PARCHMENT_DIM),
                    );
                    let (r, _) = ui.allocate_exact_size(Vec2::new(110.0, 16.0), Sense::hover());
                    cx.art.bar(
                        ui.painter(),
                        r,
                        h.loyalty / 100.0,
                        good_col(h.loyalty),
                        &format!("Loyal. {:.0}", h.loyalty),
                    );
                    let (r, _) = ui.allocate_exact_size(Vec2::new(110.0, 16.0), Sense::hover());
                    cx.art.bar(
                        ui.painter(),
                        r,
                        (h.power / 100.0).min(1.0),
                        Color32::from_rgb(160, 130, 200),
                        &format!("Macht {:.0}", h.power),
                    );
                });
            }
            if let Some(h) = cx
                .st
                .court_house
                .and_then(|id| houses.iter().find(|h| h.id == id))
                .copied()
            {
                ui.add_space(6.0);
                heading(ui, format!("Haus {}", h.name));
                dim(
                    ui,
                    format!(
                        "Oberhaupt {} · Gefolge {} · Vermögen {:.0} · Sitz {}{}",
                        w.cname(h.head),
                        h.retinue,
                        h.wealth,
                        w.pname(h.seat),
                        if h.throne_claim {
                            " · hat Thronanspruch"
                        } else {
                            ""
                        }
                    ),
                );
                if let Some(f) = h
                    .faction
                    .and_then(|f| w.factions.iter().find(|x| x.id == f))
                {
                    ui.label(
                        RichText::new(format!(
                            "Fraktion: Forderung {:?}{}",
                            f.demand,
                            if f.ultimatum { " – Ultimatum!" } else { "" }
                        ))
                        .font(theme::body_bold_font(14.0))
                        .color(WARN),
                    );
                }
                dim(
                    ui,
                    format!(
                        "Rechte: {}",
                        if h.rights.is_empty() {
                            "keine".to_string()
                        } else {
                            h.rights
                                .iter()
                                .map(|r| format!("{r:?}"))
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ),
                );
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Slider::new(&mut cx.st.reward, 50.0..=1000.0)
                            .suffix(" Gold")
                            .step_by(50.0),
                    );
                    if cx.button(ui, "Belohnen", 100.0, true) {
                        let a = cx.st.reward;
                        cx.cmd(StrategyCommand::HouseAction {
                            house: h.id,
                            action: HouseAction::Reward(a),
                        });
                    }
                    if cx.button(ui, "Bestrafen", 100.0, true) {
                        cx.cmd(StrategyCommand::HouseAction {
                            house: h.id,
                            action: HouseAction::Punish,
                        });
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    for (r, n) in [
                        (HouseRight::TaxExempt, "Steuerbefreiung"),
                        (HouseRight::OwnArmy, "Eigene Armee"),
                        (HouseRight::CouncilSeat, "Ratssitz"),
                        (HouseRight::TownCharter, "Stadtrecht"),
                        (HouseRight::Creditor, "Gläubiger"),
                    ] {
                        let has = h.rights.contains(&r);
                        if cx.choice(
                            ui,
                            &format!("{} {}", if has { "Entziehen:" } else { "Gewähren:" }, n),
                            has,
                        ) {
                            cx.cmd(StrategyCommand::HouseAction {
                                house: h.id,
                                action: if has {
                                    HouseAction::Revoke(r)
                                } else {
                                    HouseAction::Grant(r)
                                },
                            });
                        }
                    }
                });
                if let Some(p) = cx
                    .selected
                    .filter(|p| w.provinces[*p as usize].owner == Some(cx.me))
                {
                    ui.horizontal(|ui| {
                        let fief = w.provinces[p as usize].fief;
                        if fief == Some(h.id) {
                            if cx.button(ui, &format!("{} einziehen", w.pname(p)), 220.0, true) {
                                cx.cmd(StrategyCommand::HouseAction {
                                    house: h.id,
                                    action: HouseAction::Dispossess(p),
                                });
                            }
                        } else if cx.button(
                            ui,
                            &format!("{} als Lehen geben", w.pname(p)),
                            260.0,
                            true,
                        ) {
                            cx.cmd(StrategyCommand::HouseAction {
                                house: h.id,
                                action: HouseAction::GrantFief(p),
                            });
                        }
                    });
                }
                ui.horizontal_wrapped(|ui| {
                    if cx.button(ui, "In den Rat berufen", 170.0, true) {
                        cx.cmd(StrategyCommand::HouseAction {
                            house: h.id,
                            action: HouseAction::CouncilSeat,
                        });
                    }
                    for o in [
                        Office::Chancellor,
                        Office::Marshal,
                        Office::Treasurer,
                        Office::Steward,
                    ] {
                        if cx.button(ui, &format!("Als {}", o.name()), 130.0, true) {
                            cx.cmd(StrategyCommand::HouseAction {
                                house: h.id,
                                action: HouseAction::Appoint(o),
                            });
                        }
                    }
                });
            }
            section(ui, cx.art, "Laufende Komplotte");
            let plots: Vec<&Plot> = w
                .plots
                .iter()
                .filter(|p| p.realm == cx.me && !p.done)
                .collect();
            if plots.is_empty() {
                dim(ui, "Keine Komplotte in Eurem Hof.");
            }
            for p in plots {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} – Anführer {}",
                            p.kind.name(),
                            w.cname(p.leader)
                        ))
                        .font(theme::body_bold_font(14.5))
                        .color(theme::PARCHMENT),
                    );
                    dim(ui, format!("{} Mitverschwörer", p.conspirators.len()));
                });
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::new(200.0, 16.0), Sense::hover());
                    cx.art.bar(
                        ui.painter(),
                        r,
                        p.progress / 100.0,
                        Color32::from_rgb(170, 90, 200),
                        &format!("Fortschritt {:.0}", p.progress),
                    );
                    dim(
                        ui,
                        format!("Erfolgschance {:.0} %", w.plot_success_chance(p)),
                    );
                    if p.discovered {
                        ui.label(RichText::new("entdeckt!").color(WARN));
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if cx.button(ui, "Auslösen", 90.0, true) {
                            cx.cmd(StrategyCommand::TriggerPlot(p.id));
                        }
                    });
                });
            }
            section(ui, cx.art, "Neues Komplott");
            let kinds = [
                PlotKind::Murder,
                PlotKind::Slander,
                PlotKind::Fabricate,
                PlotKind::Incite,
                PlotKind::Espionage,
                PlotKind::Bribery,
                PlotKind::Steal,
                PlotKind::SecretAlliance,
                PlotKind::Blackmail,
            ];
            let kind = kinds[cx.st.plot_kind.min(kinds.len() - 1)];
            ui.horizontal_wrapped(|ui| {
                for (i, k) in kinds.iter().enumerate() {
                    if cx.choice(ui, k.name(), cx.st.plot_kind == i) {
                        cx.st.plot_kind = i;
                    }
                }
            });
            let other = cx.st.dipl_target.filter(|r| *r != cx.me);
            let target = plot_target(w, cx.me, kind, cx.selected, other);
            let leader = w.realms[cx.me as usize].offices[Office::Spymaster.idx()]
                .unwrap_or(w.realms[cx.me as usize].ruler);
            let tdesc = match target {
                Some(PlotTarget::Char(c)) => format!(
                    "Ziel: {} ({})",
                    w.cname(c),
                    w.chars[c as usize]
                        .realm
                        .map(|r| w.rname(r))
                        .unwrap_or_default()
                ),
                Some(PlotTarget::Prov(p)) => format!("Ziel: Provinz {}", w.pname(p)),
                Some(PlotTarget::Realm(r)) | Some(PlotTarget::Throne(r)) => {
                    format!("Ziel: {}", w.rname(r))
                }
                None => "Ziel: wählt zuerst ein Reich (Diplomatie) oder eine Provinz".into(),
            };
            dim(
                ui,
                format!(
                    "{tdesc} · Anführer: {} · Schwierigkeit {:.0}",
                    w.cname(leader),
                    kind.difficulty()
                ),
            );
            if cx.button(ui, "Komplott beginnen", 200.0, target.is_some())
                && let Some(t) = target
            {
                cx.cmd(StrategyCommand::StartPlot {
                    leader,
                    kind,
                    target: t,
                });
            }
            feedback(ui, cx.last_result);
        },
    )
}

// -------------------------------------------------------------------------------------------------
// Diplomatie
// -------------------------------------------------------------------------------------------------

fn opinion_col(o: f32) -> Color32 {
    good_col(50.0 + o / 2.0)
}

fn diplomacy(ctx: &egui::Context, cx: &mut Cx) -> bool {
    window(
        ctx,
        cx,
        WinKind::Diplomacy,
        "Diplomatie",
        "Meinungen, Verträge, Krieg und Frieden",
        |cx, ui| {
            let w = cx.w;
            let me = cx.me;
            let mut others: Vec<RealmId> = w
                .realms
                .iter()
                .filter(|r| r.alive && r.id != me)
                .map(|r| r.id)
                .collect();
            others.sort_by(|a, b| w.opinion(*b, me).total_cmp(&w.opinion(*a, me)));
            if cx
                .st
                .dipl_target
                .is_none_or(|t| !w.realms[t as usize].alive)
            {
                cx.st.dipl_target = others.first().copied();
            }
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(250.0);
                    egui::ScrollArea::vertical()
                        .id_salt("dipl_list")
                        .max_height(470.0)
                        .show(ui, |ui| {
                            for r in &others {
                                let rl = &w.realms[*r as usize];
                                let sel = cx.st.dipl_target == Some(*r);
                                let (rect, resp) =
                                    ui.allocate_exact_size(Vec2::new(236.0, 32.0), Sense::click());
                                let p = ui.painter();
                                p.rect_filled(
                                    rect,
                                    2.0,
                                    if sel {
                                        Color32::from_rgb(58, 44, 30)
                                    } else if resp.hovered() {
                                        Color32::from_rgb(40, 32, 26)
                                    } else {
                                        Color32::from_rgb(26, 21, 18)
                                    },
                                );
                                p.rect_filled(
                                    Rect::from_min_size(
                                        rect.min + Vec2::new(4.0, 6.0),
                                        Vec2::new(8.0, 20.0),
                                    ),
                                    0.0,
                                    super::mapview::realm_color(rl),
                                );
                                p.text(
                                    rect.min + Vec2::new(18.0, 4.0),
                                    Align2::LEFT_TOP,
                                    &rl.name,
                                    theme::body_bold_font(13.5),
                                    if sel {
                                        theme::GOLD_BRIGHT
                                    } else {
                                        theme::PARCHMENT
                                    },
                                );
                                let o = w.opinion(*r, me);
                                let mut tags = String::new();
                                if w.at_war(me, *r) {
                                    tags.push_str("Krieg ");
                                }
                                if w.has_treaty(TreatyKind::Alliance, me, *r) {
                                    tags.push_str("Bündnis ");
                                }
                                if rl.vassal_of == Some(me) {
                                    tags.push_str("Vasall ");
                                }
                                p.text(
                                    rect.min + Vec2::new(18.0, 18.0),
                                    Align2::LEFT_TOP,
                                    tags,
                                    theme::italic_font(11.5),
                                    theme::PARCHMENT_DIM,
                                );
                                p.text(
                                    rect.right_center() - Vec2::new(8.0, 0.0),
                                    Align2::RIGHT_CENTER,
                                    format!("{o:+.0}"),
                                    theme::body_bold_font(14.0),
                                    opinion_col(o),
                                );
                                if resp.clicked() {
                                    cx.st.dipl_target = Some(*r);
                                    cx.st.war_confirm = false;
                                    cx.st.peace_cede.clear();
                                }
                            }
                        });
                });
                ui.separator();
                ui.vertical(|ui| {
                    let Some(t) = cx.st.dipl_target else { return };
                    diplomacy_detail(cx, ui, t);
                });
            });
            feedback(ui, cx.last_result);
        },
    )
}

fn diplomacy_detail(cx: &mut Cx, ui: &mut Ui, t: RealmId) {
    let w = cx.w;
    let me = cx.me;
    let rl = &w.realms[t as usize];
    heading(ui, &rl.name);
    dim(
        ui,
        format!(
            "{} · {} · {}",
            rl.rank.name(),
            rl.gov.name(),
            w.full_name(rl.ruler)
        ),
    );
    let o = w.opinion(t, me);
    let o_back = w.opinion(me, t);
    bar_row(
        ui,
        cx.art,
        "Meinung von uns",
        (o + 100.0) / 200.0,
        opinion_col(o),
        &format!("{o:+.0}"),
    );
    kv(ui, "Unsere Meinung", format!("{o_back:+.0}"));
    kv(
        ui,
        "Stärke / Bedrohung",
        format!("{:.0} / {:.0}", rl.strength, rl.threat),
    );
    kv(ui, "Provinzen", format!("{}", w.provinces_of(t).len()));
    let treaties: Vec<&Treaty> = w
        .treaties
        .iter()
        .filter(|x| {
            !x.secret
                && x.end.is_none_or(|e| e > w.day)
                && ((x.a == me && x.b == t) || (x.a == t && x.b == me))
        })
        .collect();
    if !treaties.is_empty() {
        dim(
            ui,
            format!(
                "Verträge: {}",
                treaties
                    .iter()
                    .map(|x| x.kind.name())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
    }
    let antag = rl.kind == RealmKind::Hollowed || rl.kind == RealmKind::Legion;
    if antag {
        ui.label(
            RichText::new("Mit diesem Reich ist keine Verhandlung möglich.")
                .font(theme::italic_font(14.0))
                .color(WARN),
        );
    }
    let at_war = w.at_war(me, t);
    section(ui, cx.art, "Beziehungen");
    ui.horizontal_wrapped(|ui| {
        if cx.button(ui, "Gesandter", 100.0, !antag) {
            cx.cmd(StrategyCommand::Diplomacy {
                target: t,
                action: DiplomaticAction::Envoy,
            });
        }
        let g = cx.st.gift;
        if cx.button(ui, &format!("Geschenk ({g:.0})"), 140.0, !antag) {
            cx.cmd(StrategyCommand::Diplomacy {
                target: t,
                action: DiplomaticAction::Gift(g),
            });
        }
        ui.add(
            egui::Slider::new(&mut cx.st.gift, 50.0..=1000.0)
                .step_by(50.0)
                .show_value(false),
        );
    });
    ui.horizontal_wrapped(|ui| {
        for k in [
            TreatyKind::Alliance,
            TreatyKind::DefensivePact,
            TreatyKind::NonAggression,
            TreatyKind::Trade,
            TreatyKind::MilitaryAccess,
            TreatyKind::Guarantee,
            TreatyKind::JointCampaign,
        ] {
            let has = w.has_treaty(k, me, t);
            if has {
                if cx.button(ui, &format!("{} kündigen", k.name()), 190.0, true) {
                    cx.cmd(StrategyCommand::Diplomacy {
                        target: t,
                        action: DiplomaticAction::BreakTreaty(k),
                    });
                }
            } else if cx.button(
                ui,
                &format!("{} anbieten", k.name()),
                190.0,
                !antag && !at_war,
            ) {
                cx.cmd(StrategyCommand::Diplomacy {
                    target: t,
                    action: DiplomaticAction::Propose(k),
                });
            }
        }
    });
    ui.horizontal_wrapped(|ui| {
        if cx.button(ui, "Tribut fordern", 130.0, !antag) {
            cx.cmd(StrategyCommand::Diplomacy {
                target: t,
                action: DiplomaticAction::DemandTribute,
            });
        }
        if cx.button(ui, "Vasallenschaft anbieten", 190.0, !antag) {
            cx.cmd(StrategyCommand::Diplomacy {
                target: t,
                action: DiplomaticAction::OfferVassalage,
            });
        }
        let embargo = w.realms[me as usize].embargo.contains(&t);
        if cx.button(
            ui,
            if embargo {
                "Embargo aufheben"
            } else {
                "Embargo verhängen"
            },
            170.0,
            true,
        ) {
            cx.cmd(StrategyCommand::Diplomacy {
                target: t,
                action: DiplomaticAction::Embargo(!embargo),
            });
        }
    });
    // Kriege mit diesem Reich
    let wars: Vec<&War> = w
        .wars
        .iter()
        .filter(|x| {
            x.active
                && ((x.attackers.contains(&me) && x.defenders.contains(&t))
                    || (x.defenders.contains(&me) && x.attackers.contains(&t)))
        })
        .collect();
    if let Some(war) = wars.first() {
        section(ui, cx.art, "Frieden aushandeln");
        let i_attack = war.attackers.contains(&me);
        let my_score = if i_attack { war.score } else { -war.score };
        dim(
            ui,
            format!(
                "Krieg seit {} – Kriegsgrund {} · Kriegspunkte {:+.0}",
                Date::from_day(war.start).year,
                war.cb.name(),
                my_score
            ),
        );
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new("Forderung:")
                    .font(theme::body_font(14.0))
                    .color(theme::PARCHMENT_DIM),
            );
            let their: Vec<ProvId> = w.provinces_of(t).to_vec();
            let mine: Vec<ProvId> = w.provinces_of(me).to_vec();
            let list = if my_score >= 0.0 { their } else { mine };
            for p in list.into_iter().take(10) {
                let on = cx.st.peace_cede.contains(&p);
                if cx.choice(ui, &w.pname(p), on) {
                    if on {
                        cx.st.peace_cede.retain(|x| *x != p);
                    } else {
                        cx.st.peace_cede.push(p);
                    }
                }
            }
        });
        ui.horizontal(|ui| {
            ui.checkbox(&mut cx.st.peace_tribute, "Tribut");
            ui.checkbox(&mut cx.st.peace_vassal, "Vasallisierung");
        });
        let terms = PeaceTerms {
            cede: cx.st.peace_cede.clone(),
            tribute: cx.st.peace_tribute,
            vassalize: cx.st.peace_vassal,
            independence: false,
        };
        let cost = w.peace_cost(&terms);
        let col = if cost <= my_score.max(0.0) { OK } else { WARN };
        ui.label(
            RichText::new(format!(
                "Kosten der Bedingungen: {cost:.0} Kriegspunkte (verfügbar {:.0})",
                my_score.max(0.0)
            ))
            .font(theme::body_bold_font(14.0))
            .color(col),
        );
        ui.horizontal(|ui| {
            if cx.button(ui, "Frieden anbieten", 170.0, true) {
                cx.cmd(StrategyCommand::Diplomacy {
                    target: t,
                    action: DiplomaticAction::OfferPeace { war: war.id, terms },
                });
            }
            if cx.button(ui, "Weißer Frieden", 170.0, true) {
                cx.cmd(StrategyCommand::Diplomacy {
                    target: t,
                    action: DiplomaticAction::OfferPeace {
                        war: war.id,
                        terms: PeaceTerms::default(),
                    },
                });
            }
        });
    } else if !antag {
        section(ui, cx.art, "Krieg erklären");
        let cb = w.find_cb(me, t);
        match cb {
            Some(c) => ui.label(
                RichText::new(format!("Kriegsgrund: {}", c.name()))
                    .font(theme::body_bold_font(14.5))
                    .color(OK),
            ),
            None => ui.label(
                RichText::new(
                    "Kein Kriegsgrund – ein Krieg ohne Grund kostet 300 Prestige und 30 Tyrannei.",
                )
                .font(theme::body_bold_font(14.0))
                .color(WARN),
            ),
        };
        let theirs: Vec<ProvId> = w
            .provinces_of(t)
            .iter()
            .copied()
            .filter(|p| {
                w.provinces[*p as usize]
                    .neighbors
                    .iter()
                    .any(|l| w.provinces[l.to as usize].owner == Some(me))
            })
            .collect();
        let theirs = if theirs.is_empty() {
            w.provinces_of(t).to_vec()
        } else {
            theirs
        };
        let goals = [
            "Provinz einnehmen",
            "Tribut",
            "Unterwerfen",
            "Unabhängigkeit",
        ];
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new("Kriegsziel:")
                    .font(theme::body_font(14.0))
                    .color(theme::PARCHMENT_DIM),
            );
            for (i, g) in goals.iter().enumerate() {
                if cx.choice(ui, g, cx.st.war_goal == i) {
                    cx.st.war_goal = i;
                }
            }
        });
        if cx.st.war_goal == 0 {
            ui.horizontal_wrapped(|ui| {
                for p in theirs.iter().take(8) {
                    if cx.choice(ui, &w.pname(*p), cx.st.war_goal_prov == Some(*p)) {
                        cx.st.war_goal_prov = Some(*p);
                    }
                }
            });
        }
        let goal = match cx.st.war_goal {
            0 => WarGoal::TakeProvince(
                cx.st
                    .war_goal_prov
                    .filter(|p| theirs.contains(p))
                    .or(theirs.first().copied())
                    .unwrap_or(0),
            ),
            1 => WarGoal::Tribute,
            2 => WarGoal::Subjugate,
            _ => WarGoal::Independence,
        };
        ui.horizontal(|ui| {
            ui.checkbox(&mut cx.st.war_confirm, "Ich will den Krieg erklären");
            let cbv = cb.unwrap_or(CasusBelli::Unjustified);
            if cx.button(ui, "Krieg erklären", 160.0, cx.st.war_confirm) {
                cx.cmd(StrategyCommand::Diplomacy {
                    target: t,
                    action: DiplomaticAction::DeclareWar { cb: cbv, goal },
                });
                cx.st.war_confirm = false;
            }
        });
    }
}

// -------------------------------------------------------------------------------------------------
// Heere
// -------------------------------------------------------------------------------------------------

fn military(ctx: &egui::Context, cx: &mut Cx) -> bool {
    window(
        ctx,
        cx,
        WinKind::Military,
        "Heere",
        "Aufstellen, bewegen (Rechtsklick auf die Zielprovinz), teilen, belagern",
        |cx, ui| {
            let w = cx.w;
            let me = cx.me;
            let armies: Vec<&Army> = w
                .armies
                .iter()
                .filter(|a| {
                    a.owner == Some(me) && matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary)
                })
                .collect();
            let total: u32 = armies.iter().map(|a| a.size()).sum();
            let upkeep: f32 = armies
                .iter()
                .flat_map(|a| a.regiments.iter())
                .map(|r| catalog::unit(&r.unit).map(|u| u.upkeep).unwrap_or(0.0) * r.count as f32)
                .sum();
            kv(
                ui,
                "Truppen gesamt",
                format!("{total} Mann · Unterhalt {upkeep:.0} Gold/Monat"),
            );
            kv(ui, "Flotte", format!("{}", w.realms[me as usize].fleet));
            section(ui, cx.art, "Heere");
            if armies.is_empty() {
                dim(
                    ui,
                    "Ihr habt keine Heere. Hebt Truppen in einer eigenen Provinz aus.",
                );
            }
            for a in &armies {
                let sel = cx.selected_army == Some(a.id);
                ui.horizontal(|ui| {
                    if cx.choice(ui, &a.name, sel) {
                        cx.reqs.push(Req::SelectArmy(a.id));
                        cx.reqs.push(Req::Center(a.location));
                    }
                    dim(
                        ui,
                        format!(
                            "{} · {} Mann · Moral {:.0} · Nachschub {:.0}",
                            w.pname(a.location),
                            a.size(),
                            a.morale,
                            a.supply
                        ),
                    );
                    if a.in_battle.is_some() {
                        ui.label(RichText::new("in Schlacht").color(WARN));
                    } else if a.order == ArmyOrder::Siege {
                        ui.label(RichText::new("belagert").color(WARN));
                    } else if !a.path.is_empty() {
                        dim(ui, format!("→ {}", w.pname(*a.path.last().unwrap())));
                    }
                });
            }
            if let Some(a) = cx
                .selected_army
                .and_then(|id| w.army(id))
                .filter(|a| a.owner == Some(me))
            {
                section(ui, cx.art, &format!("Ausgewählt: {}", a.name));
                if let Some(c) = a.commander {
                    dim(
                        ui,
                        format!(
                            "Befehlshaber: {} (Kriegskunst {})",
                            w.full_name(c),
                            w.chars[c as usize].attr(Attr::Martial)
                        ),
                    );
                } else {
                    dim(ui, "Kein Befehlshaber.");
                }
                for r in &a.regiments {
                    let name = catalog::unit(&r.unit).map(|u| u.name).unwrap_or("?");
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{}× {name}", r.count))
                                .font(theme::body_font(14.0))
                                .color(theme::PARCHMENT),
                        );
                        dim(
                            ui,
                            format!("Veteran {} · Zustand {:.0} %", r.vet, r.hp * 100.0),
                        );
                    });
                }
                ui.horizontal_wrapped(|ui| {
                    if cx.button(ui, "Zur Provinz", 110.0, true) {
                        cx.reqs.push(Req::Center(a.location));
                        cx.reqs.push(Req::Select(a.location));
                    }
                    let take: Vec<(String, u32)> = a
                        .regiments
                        .iter()
                        .filter(|r| r.count >= 2)
                        .map(|r| (r.unit.clone(), r.count / 2))
                        .collect();
                    if cx.button(ui, "Teilen (Hälfte)", 140.0, !take.is_empty()) {
                        cx.cmd(StrategyCommand::SplitArmy { army: a.id, take });
                    }
                    for o in armies
                        .iter()
                        .filter(|o| o.id != a.id && o.location == a.location)
                    {
                        if cx.button(ui, &format!("{} aufnehmen", o.name), 200.0, true) {
                            cx.cmd(StrategyCommand::MergeArmies {
                                into: a.id,
                                from: o.id,
                            });
                        }
                    }
                    if w.provinces[a.location as usize]
                        .siege
                        .as_ref()
                        .is_some_and(|s| s.besieger == me)
                        && cx.button(ui, "Sturmangriff", 130.0, true)
                    {
                        cx.cmd(StrategyCommand::StormSiege(a.location));
                    }
                    if cx.button(ui, "Auflösen", 100.0, true) {
                        cx.cmd(StrategyCommand::DisbandArmy(a.id));
                    }
                });
                let chars: Vec<CharId> = w
                    .chars
                    .iter()
                    .filter(|c| {
                        c.realm == Some(me)
                            && c.alive()
                            && c.adult(w.day)
                            && c.army.is_none()
                            && c.captive_of.is_none()
                    })
                    .map(|c| c.id)
                    .collect();
                if a.commander.is_none() && !chars.is_empty() {
                    let mut chosen = None;
                    egui::ComboBox::from_id_salt("cmdr")
                        .selected_text(
                            RichText::new("Befehlshaber ernennen …").font(theme::body_font(14.0)),
                        )
                        .show_ui(ui, |ui| {
                            let mut l = chars;
                            l.sort_by_key(|c| {
                                std::cmp::Reverse(w.chars[*c as usize].attr(Attr::Martial))
                            });
                            for c in l.into_iter().take(10) {
                                if ui
                                    .selectable_label(
                                        false,
                                        format!(
                                            "{} · Kriegskunst {}",
                                            w.cname(c),
                                            w.chars[c as usize].attr(Attr::Martial)
                                        ),
                                    )
                                    .clicked()
                                {
                                    chosen = Some(c);
                                }
                            }
                        });
                    if let Some(c) = chosen {
                        cx.cmd(StrategyCommand::AssignCommander {
                            army: a.id,
                            character: c,
                        });
                    }
                }
            }
            section(ui, cx.art, "Truppen ausheben");
            match cx
                .selected
                .filter(|p| w.provinces[*p as usize].owner == Some(me))
            {
                None => dim(ui, "Wählt eine eigene Provinz auf der Karte."),
                Some(p) => {
                    let roster = catalog::roster(w.realms[me as usize].culture);
                    cx.st.recruit_unit = cx.st.recruit_unit.min(roster.len().saturating_sub(1));
                    ui.horizontal_wrapped(|ui| {
                        for (i, id) in roster.iter().enumerate() {
                            let n = catalog::unit(id).map(|u| u.name).unwrap_or(id);
                            if cx.choice(ui, n, cx.st.recruit_unit == i) {
                                cx.st.recruit_unit = i;
                            }
                        }
                    });
                    let unit = roster[cx.st.recruit_unit];
                    let u = catalog::unit(unit);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Anzahl")
                                .font(theme::body_font(14.0))
                                .color(theme::PARCHMENT_DIM),
                        );
                        ui.add(egui::Slider::new(&mut cx.st.recruit_count, 1..=100));
                    });
                    let n = cx.st.recruit_count;
                    let cost = w.recruit_cost(me, unit, n);
                    dim(
                        ui,
                        format!(
                            "In {}: {n}× {} für {} (Unterhalt {:.1}/Monat je Mann · Garnison {})",
                            w.pname(p),
                            u.map(|u| u.name).unwrap_or(unit),
                            gold(cost),
                            u.map(|u| u.upkeep).unwrap_or(0.0),
                            w.provinces[p as usize].garrison
                        ),
                    );
                    ui.horizontal(|ui| {
                        if cx.button(ui, "Ausheben", 130.0, w.realms[me as usize].gold >= cost) {
                            cx.cmd(StrategyCommand::Recruit {
                                province: p,
                                unit: unit.to_string(),
                                count: n,
                            });
                        }
                        if cx.button(ui, "Söldner (20)", 130.0, true) {
                            cx.cmd(StrategyCommand::HireMercenaries(20));
                        }
                    });
                }
            }
            feedback(ui, cx.last_result);
        },
    )
}

// -------------------------------------------------------------------------------------------------
// Wirtschaft
// -------------------------------------------------------------------------------------------------

fn economy(ctx: &egui::Context, cx: &mut Cx) -> bool {
    window(
        ctx,
        cx,
        WinKind::Economy,
        "Wirtschaft",
        "Staatskasse, Steuern, Kredite und Märkte",
        |cx, ui| {
            let w = cx.w;
            let me = cx.me;
            let r = &w.realms[me as usize];
            section(ui, cx.art, "Staatskasse");
            kv(ui, "Gold", format!("{:.0}", r.gold));
            kv_col(
                ui,
                "Einkommen / Ausgaben (Monat)",
                format!(
                    "{:.0} / {:.0}  ({:+.0})",
                    r.income,
                    r.expenses,
                    r.income - r.expenses
                ),
                if r.income >= r.expenses { OK } else { WARN },
            );
            kv(
                ui,
                "Aschglas / Seelenasche",
                format!("{:.0} / {:.0}", r.ashglass, r.soul_ash),
            );
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Steuerstufe")
                        .font(theme::body_font(14.0))
                        .color(theme::PARCHMENT_DIM),
                );
                for (i, n) in ["Erlass", "Niedrig", "Normal", "Hoch", "Erdrückend"]
                    .iter()
                    .enumerate()
                {
                    if cx.choice(ui, n, r.tax_level as usize == i) {
                        cx.cmd(StrategyCommand::SetTaxLevel(i as u8));
                    }
                }
            });
            let mut tt = r.trade_tax;
            let (mut imp, mut tra) = (r.tariff_import, r.tariff_transit);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Handelssteuer")
                        .font(theme::body_font(14.0))
                        .color(theme::PARCHMENT_DIM),
                );
                if ui
                    .add(
                        egui::Slider::new(&mut tt, 0.0..=20.0)
                            .suffix(" %")
                            .step_by(5.0),
                    )
                    .drag_stopped()
                {
                    cx.cmd(StrategyCommand::SetTradeTax(tt));
                }
            });
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Zoll Einfuhr / Durchfuhr")
                        .font(theme::body_font(14.0))
                        .color(theme::PARCHMENT_DIM),
                );
                let a = ui.add(
                    egui::Slider::new(&mut imp, 0.0..=30.0)
                        .suffix(" %")
                        .step_by(5.0),
                );
                let b = ui.add(
                    egui::Slider::new(&mut tra, 0.0..=15.0)
                        .suffix(" %")
                        .step_by(5.0),
                );
                if a.drag_stopped() || b.drag_stopped() {
                    cx.cmd(StrategyCommand::SetTariffs {
                        import: imp,
                        transit: tra,
                    });
                }
            });
            section(ui, cx.art, "Kredite");
            let debt: f32 = r.loans.iter().map(|l| l.principal).sum();
            kv(ui, "Schulden", format!("{debt:.0} Gold"));
            for l in &r.loans {
                dim(
                    ui,
                    format!(
                        "{:?}: {:.0} Gold zu {:.1} % · noch {} Monate",
                        l.lender,
                        l.principal,
                        l.rate * 100.0,
                        l.months_left
                    ),
                );
            }
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut cx.st.loan_amount, 100.0..=3000.0)
                        .suffix(" Gold")
                        .step_by(100.0),
                );
                ui.add(egui::Slider::new(&mut cx.st.loan_years, 1..=15).suffix(" J."));
            });
            ui.horizontal_wrapped(|ui| {
                let (a, y) = (cx.st.loan_amount, cx.st.loan_years);
                if cx.button(ui, "Saelith-Bank", 140.0, true) {
                    cx.cmd(StrategyCommand::TakeLoan {
                        source: LoanSource::SaelithBanks,
                        amount: a,
                        years: y,
                    });
                }
                if cx.button(ui, "Kirche", 100.0, true) {
                    cx.cmd(StrategyCommand::TakeLoan {
                        source: LoanSource::Church,
                        amount: a,
                        years: y,
                    });
                }
                if cx.button(ui, "Kriegsanleihe", 140.0, w.in_any_war(me)) {
                    cx.cmd(StrategyCommand::TakeLoan {
                        source: LoanSource::War,
                        amount: a,
                        years: y,
                    });
                }
            });
            section(ui, cx.art, "Ertragreichste Provinzen");
            let mut mine: Vec<ProvId> = w.provinces_of(me).to_vec();
            mine.sort_by(|a, b| prosperity(w, *b).total_cmp(&prosperity(w, *a)));
            for p in mine.iter().take(6) {
                ui.horizontal(|ui| {
                    if ui
                        .link(
                            RichText::new(w.pname(*p))
                                .font(theme::body_font(14.0))
                                .color(theme::GOLD),
                        )
                        .clicked()
                    {
                        cx.reqs.push(Req::Select(*p));
                        cx.reqs.push(Req::Center(*p));
                    }
                    dim(
                        ui,
                        format!(
                            "Wohlstand {:.0} · Zufriedenheit {:.0} · {:.0} Einwohner",
                            prosperity(w, *p),
                            w.provinces[*p as usize].avg_satisfaction(),
                            w.provinces[*p as usize].population()
                        ),
                    );
                });
            }
            section(ui, cx.art, "Markt");
            let region = w.provinces[w.realms[me as usize].capital as usize].region as usize;
            if let Some(m) = w
                .regions
                .get(region)
                .and_then(|rg| rg.market)
                .and_then(|mi| w.markets.get(mi))
            {
                dim(
                    ui,
                    format!(
                        "{} · Inflation {:.1} % · Handelsvolumen {:.0}",
                        m.name,
                        m.inflation * 100.0,
                        m.volume
                    ),
                );
                let mut rows: Vec<(Good, f32, f32)> = Good::ALL
                    .iter()
                    .map(|g| {
                        (
                            *g,
                            m.prices[g.idx()],
                            m.prices[g.idx()] / g.base_price().max(0.01),
                        )
                    })
                    .collect();
                rows.sort_by(|a, b| b.2.total_cmp(&a.2));
                egui::Grid::new("market_grid")
                    .num_columns(4)
                    .spacing([14.0, 3.0])
                    .show(ui, |ui| {
                        for (g, price, ratio) in
                            rows.iter().take(7).chain(rows.iter().rev().take(5))
                        {
                            ui.label(
                                RichText::new(g.name())
                                    .font(theme::body_font(14.0))
                                    .color(theme::PARCHMENT),
                            );
                            ui.label(
                                RichText::new(format!("{price:.1}"))
                                    .font(theme::body_bold_font(14.0))
                                    .color(theme::PARCHMENT),
                            );
                            let pct = (ratio - 1.0) * 100.0;
                            ui.label(
                                RichText::new(format!("{pct:+.0} %"))
                                    .font(theme::body_font(14.0))
                                    .color(if pct > 15.0 {
                                        WARN
                                    } else if pct < -15.0 {
                                        OK
                                    } else {
                                        theme::PARCHMENT_DIM
                                    }),
                            );
                            ui.label(
                                RichText::new(if m.shortage[g.idx()] > 1.0 {
                                    "Knappheit"
                                } else {
                                    ""
                                })
                                .font(theme::italic_font(13.0))
                                .color(WARN),
                            );
                            ui.end_row();
                        }
                    });
            }
            feedback(ui, cx.last_result);
        },
    )
}

// -------------------------------------------------------------------------------------------------
// Glaube
// -------------------------------------------------------------------------------------------------

fn religion(ctx: &egui::Context, cx: &mut Cx) -> bool {
    window(
        ctx,
        cx,
        WinKind::Religion,
        "Glaube",
        "Bekenntnisse, Eifer und heilige Stätten",
        |cx, ui| {
            let w = cx.w;
            let me = cx.me;
            let r = &w.realms[me as usize];
            kv(ui, "Staatsglaube", r.religion.name());
            kv(ui, "Frömmigkeit", format!("{:.0}", r.piety));
            if r.excommunicated {
                ui.label(
                    RichText::new("Das Reich ist exkommuniziert!")
                        .font(theme::body_bold_font(14.5))
                        .color(WARN),
                );
            }
            if r.heresy_pressure > 1.0 {
                kv_col(
                    ui,
                    "Häresiedruck",
                    format!("{:.0}", r.heresy_pressure),
                    WARN,
                );
            }
            section(ui, cx.art, "Bekenntnisse im Reich");
            let mut share = [0.0f32; 6];
            let mut pop = 0.0;
            for p in w.provinces_of(me) {
                let pr = &w.provinces[*p as usize];
                let n = pr.population();
                pop += n;
                for (sh, f) in share.iter_mut().zip(pr.faith.iter()) {
                    *sh += f * n;
                }
            }
            for rel in RELIGIONS {
                let s = if pop > 0.0 {
                    share[rel.idx()] / pop
                } else {
                    0.0
                };
                let c = religion_rgb(rel);
                bar_row(
                    ui,
                    cx.art,
                    rel.name(),
                    s,
                    Color32::from_rgb(c[0] as u8, c[1] as u8, c[2] as u8),
                    &format!("{:.0} %", s * 100.0),
                );
            }
            section(ui, cx.art, "Eifer der Religionen");
            for rel in RELIGIONS {
                let c = religion_rgb(rel);
                bar_row(
                    ui,
                    cx.art,
                    rel.name(),
                    w.zeal[rel.idx()] / 100.0,
                    Color32::from_rgb(c[0] as u8, c[1] as u8, c[2] as u8),
                    &format!("{:.0}", w.zeal[rel.idx()]),
                );
                let f = rel.feast();
                let _ = f;
            }
            section(ui, cx.art, "Glaubensgesetz");
            ui.horizontal(|ui| {
                for (i, n) in ["Staatskirche", "Duldung", "Freiheit"].iter().enumerate() {
                    if cx.choice(ui, n, r.laws.faith_law as usize == i) {
                        cx.cmd(StrategyCommand::SetLaw(LawChange::FaithLaw(i as u8)));
                    }
                }
            });
            section(ui, cx.art, "Heilige Stätten und Schreine");
            let sites: Vec<ProvId> = w
                .provinces_of(me)
                .iter()
                .copied()
                .filter(|p| {
                    w.provinces[*p as usize].holy_site.is_some()
                        || w.provinces[*p as usize].god_shrine.is_some()
                })
                .collect();
            if sites.is_empty() {
                dim(ui, "Keine heiligen Stätten in Eurem Besitz.");
            }
            for p in sites {
                let pr = &w.provinces[p as usize];
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&pr.name)
                            .font(theme::body_bold_font(14.5))
                            .color(theme::PARCHMENT),
                    );
                    if let Some(h) = pr.holy_site {
                        dim(ui, h.name());
                    }
                    if pr.god_shrine.is_some() {
                        dim(ui, "Götterschrein");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if cx.button(ui, "Wiederherstellen", 150.0, true) {
                                cx.cmd(StrategyCommand::RestoreShrine(p));
                            }
                        });
                    }
                });
            }
            feedback(ui, cx.last_result);
        },
    )
}

// -------------------------------------------------------------------------------------------------
// Magie
// -------------------------------------------------------------------------------------------------

fn magic(ctx: &egui::Context, cx: &mut Cx) -> bool {
    window(
        ctx,
        cx,
        WinKind::Magic,
        "Magie",
        "Forschung, Rituale und der Pfad der Hohlung",
        |cx, ui| {
            let w = cx.w;
            let me = cx.me;
            let r = &w.realms[me as usize];
            kv(ui, "Aschglas", format!("{:.0}", r.ashglass));
            bar_row(
                ui,
                cx.art,
                "Hohlung des Reiches",
                r.hollow_value / 100.0,
                Color32::from_rgb(90, 220, 140),
                &format!("{:.0} · Pfad {}", r.hollow_value, r.hollow_path),
            );
            section(ui, cx.art, "Forschung");
            for s in catalog_schools() {
                let st = &r.research[s.idx()];
                let next = GRADE_POINTS
                    .get(st.grade as usize)
                    .copied()
                    .unwrap_or(GRADE_POINTS[3]);
                let focus = r.research_focus == Some(s);
                ui.horizontal(|ui| {
                    if cx.choice(ui, s.name(), focus) {
                        cx.cmd(StrategyCommand::SetResearchFocus(if focus {
                            None
                        } else {
                            Some(s)
                        }));
                    }
                    let (rc, _) = ui.allocate_exact_size(Vec2::new(150.0, 16.0), Sense::hover());
                    let frac = if st.grade >= 4 { 1.0 } else { st.points / next };
                    cx.art.bar(
                        ui.painter(),
                        rc,
                        frac,
                        Color32::from_rgb(110, 140, 230),
                        &format!("Grad {}", st.grade),
                    );
                    if (st.grade as usize) < 4 {
                        dim(
                            ui,
                            format!(
                                "{:.0}/{next:.0} · {:.0} Aschglas",
                                st.points, GRADE_ASHGLASS[st.grade as usize]
                            ),
                        );
                    }
                });
            }
            section(ui, cx.art, "Rituale");
            let target = cx.selected;
            match target {
                Some(p) => dim(ui, format!("Zielprovinz: {}", w.pname(p))),
                None => dim(ui, "Wählt eine Provinz als Ziel."),
            }
            for rt in RITUALS.iter() {
                let known = r.research[rt.school.idx()].grade >= rt.grade;
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(known, |ui| {
                        ui.set_width(200.0);
                        ui.label(
                            RichText::new(rt.name)
                                .font(theme::body_bold_font(14.0))
                                .color(if known { theme::PARCHMENT } else { theme::ASH }),
                        );
                    });
                    dim(
                        ui,
                        format!(
                            "{} · {} Gold · {} Aschglas{}",
                            rt.school.name(),
                            rt.gold,
                            rt.ashglass,
                            if rt.risk > 0.0 {
                                format!(" · Risiko {:.0} %", rt.risk * 100.0)
                            } else {
                                String::new()
                            }
                        ),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if cx.button(
                            ui,
                            "Wirken",
                            80.0,
                            known
                                && target.is_some()
                                && r.gold >= rt.gold
                                && r.ashglass >= rt.ashglass,
                        ) {
                            cx.cmd(StrategyCommand::CastRitual {
                                ritual: rt.id.to_string(),
                                province: target.unwrap_or(0),
                            });
                        }
                    });
                });
            }
            if w.bosses
                .iter()
                .any(|b| b.id == "boss_ashen_dragon" && b.state != BossState::Defeated)
            {
                section(ui, cx.art, "Der Aschdrache");
                ui.horizontal(|ui| {
                    dim(
                        ui,
                        "Tribut: 400 Gold und 5 Aschglas – der Drache schweigt zwei Jahre.",
                    );
                    if cx.button(ui, "Tribut zahlen", 150.0, true) {
                        cx.cmd(StrategyCommand::PayDragonTribute);
                    }
                });
            }
            if !r.artifacts.is_empty() || !r.mage_orders.is_empty() {
                section(ui, cx.art, "Artefakte und Orden");
                for a in &r.artifacts {
                    if let Some(def) = catalog::ARTIFACTS.get(*a as usize) {
                        text(ui, def.name, 14.0, theme::PARCHMENT);
                    }
                }
            }
            feedback(ui, cx.last_result);
        },
    )
}

fn catalog_schools() -> [School; 9] {
    ashen_strategy::SCHOOLS
}

// -------------------------------------------------------------------------------------------------
// Chronik
// -------------------------------------------------------------------------------------------------

fn chronicle(ctx: &egui::Context, cx: &mut Cx) -> bool {
    window(
        ctx,
        cx,
        WinKind::Chronicle,
        "Chronik von Eldraeth",
        "Die Geschichte, wie sie sich zutrug",
        |cx, ui| {
            let w = cx.w;
            ui.horizontal_wrapped(|ui| {
                if cx.choice(ui, "Alle", cx.st.chron_kind.is_none()) {
                    cx.st.chron_kind = None;
                }
                for k in [
                    ChronicleKind::War,
                    ChronicleKind::Battle,
                    ChronicleKind::Siege,
                    ChronicleKind::Peace,
                    ChronicleKind::Succession,
                    ChronicleKind::Marriage,
                    ChronicleKind::Plot,
                    ChronicleKind::Plague,
                    ChronicleKind::Famine,
                    ChronicleKind::Revolt,
                    ChronicleKind::Religion,
                    ChronicleKind::Hollowing,
                    ChronicleKind::Diplomacy,
                ] {
                    if cx.choice(ui, k.label(), cx.st.chron_kind == Some(k)) {
                        cx.st.chron_kind = Some(k);
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Bedeutung ab")
                        .font(theme::body_font(14.0))
                        .color(theme::PARCHMENT_DIM),
                );
                ui.add(egui::Slider::new(&mut cx.st.chron_min, 0..=90).step_by(5.0));
                ui.checkbox(&mut cx.st.chron_mine, "nur mein Reich");
                ui.add(
                    egui::TextEdit::singleline(&mut cx.st.chron_search)
                        .hint_text("Suche …")
                        .desired_width(130.0),
                );
            });
            let q = cx.st.chron_search.to_lowercase();
            let rows: Vec<&ChronicleEntry> = w
                .chronicle
                .entries
                .iter()
                .rev()
                .filter(|e| e.visibility != ashen_strategy::chronicle::Visibility::Secret)
                .filter(|e| {
                    cx.st.chron_kind.is_none_or(|k| e.kind == k) && e.magnitude >= cx.st.chron_min
                })
                .filter(|e| !cx.st.chron_mine || e.realm == Some(cx.me))
                .filter(|e| q.is_empty() || e.text.to_lowercase().contains(&q))
                .take(60)
                .collect();
            ui.add_space(4.0);
            cx.art.panel(
                ui,
                crate::ui_art::PanelKind::Parchment,
                egui::Margin {
                    left: 22,
                    right: 22,
                    top: 16,
                    bottom: 16,
                },
                |ui| {
                    ui.set_width(ui.available_width());
                    if rows.is_empty() {
                        ui.label(
                            RichText::new("Die Chronik schweigt – noch.")
                                .font(theme::italic_font(15.0))
                                .color(theme::INK_FADED),
                        );
                    }
                    let mut last_year = i32::MIN;
                    for e in rows {
                        if e.date.year != last_year {
                            last_year = e.date.year;
                            ui.label(
                                RichText::new(format!("Im Jahr {} nach dem Fall", e.date.year))
                                    .font(theme::title_bold_font(15.0))
                                    .color(theme::INK_RED),
                            );
                        }
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new(format!(
                                    "{}. {} · {}",
                                    e.date.day + 1,
                                    e.date.month_name(),
                                    e.kind.label()
                                ))
                                .font(theme::caps_font(11.5))
                                .color(theme::INK_FADED),
                            );
                        });
                        let big = e.magnitude >= 60;
                        let resp = ui.label(
                            RichText::new(&e.text)
                                .font(if big {
                                    theme::body_bold_font(15.0)
                                } else {
                                    theme::body_font(15.0)
                                })
                                .color(theme::INK),
                        );
                        if let Some(p) = e.province
                            && resp.clicked()
                        {
                            cx.reqs.push(Req::Center(p));
                        }
                        ui.add_space(4.0);
                    }
                },
            );
        },
    )
}

// -------------------------------------------------------------------------------------------------
// Siegziele
// -------------------------------------------------------------------------------------------------

fn victory(ctx: &egui::Context, cx: &mut Cx) -> bool {
    window(
        ctx,
        cx,
        WinKind::Victory,
        "Siegziele",
        "Acht Wege, Eldraeth zu prägen",
        |cx, ui| {
            let w = cx.w;
            let prog = w.victory_progress(cx.me);
            let active = &w.options.rules.victories;
            for k in VictoryKind::ALL {
                let p = prog.iter().find(|p| p.kind == k);
                let on = active.contains(&k);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(k.name())
                            .font(theme::heading_font(15.0))
                            .color(if on { theme::GOLD } else { theme::ASH }),
                    );
                    if !on {
                        dim(ui, "(in dieser Partie nicht aktiv)");
                    }
                });
                if let Some(p) = p {
                    bar_row(
                        ui,
                        cx.art,
                        "",
                        p.progress,
                        if p.achieved {
                            OK
                        } else {
                            Color32::from_rgb(210, 160, 60)
                        },
                        &format!("{:.0} %", p.progress * 100.0),
                    );
                    dim(ui, &p.detail);
                } else {
                    dim(ui, "Kein Fortschritt erfasst.");
                }
                ui.add_space(4.0);
            }
            if let Some(v) = &w.victory {
                ui.label(
                    RichText::new(format!(
                        "Sieg errungen: {} – {}",
                        w.rname(v.realm),
                        v.kind.name()
                    ))
                    .font(theme::body_bold_font(15.0))
                    .color(OK),
                );
            }
            let _ = (Stratum::Serfs, Align2::CENTER_CENTER, Pos2::ZERO);
        },
    )
}
