//! Kartendarstellung der Weltkarte Eldraeth: gerasterte Pixel-Art-Karte (Provinzen als gefüllte Polygone mit
//! Gelände-Rauschen, Küsten, Meer), Kartenmodi, Grenzen sowie die Overlays (Siedlungen, Heere, Pfeile, Marker, Nebel).

use crate::art::ArtLibrary;
use crate::ui_art::{self, UiArt};
use crate::ui_common;
use ashen_core::noise::{fbm2, rand2};
use ashen_strategy::{
    ArmyId, ArmyKind, ArmyOrder, Biome, LinkKind, ProvId, RELIGIONS, Realm, RealmId, Religion,
    STRATA, SettlementLevel, StrategyWorld, TreatyKind,
};
use egui::{
    Color32, ColorImage, Painter, Pos2, Rect, Shape, Stroke, TextureHandle, TextureId,
    TextureOptions, Vec2 as EVec2,
};
use glam::Vec2;

/// Kartenkoordinaten der Simulation: 1000 × 700. Ein Texel der Rasterkarte deckt `UNIT` Kartenpunkte ab.
pub const MAP_W: f32 = 1000.0;
pub const MAP_H: f32 = 700.0;
const UNIT: f32 = 2.5;
const RW: usize = (MAP_W / UNIT) as usize;
const RH: usize = (MAP_H / UNIT) as usize;
const NONE: u16 = u16::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapMode {
    Realms,
    Terrain,
    Religion,
    Hollowing,
    Economy,
    Unrest,
    Diplomacy,
}

impl MapMode {
    pub const ALL: [MapMode; 7] = [
        MapMode::Realms,
        MapMode::Terrain,
        MapMode::Religion,
        MapMode::Hollowing,
        MapMode::Economy,
        MapMode::Unrest,
        MapMode::Diplomacy,
    ];
    pub fn label(self) -> &'static str {
        match self {
            MapMode::Realms => "Reiche",
            MapMode::Terrain => "Gelände",
            MapMode::Religion => "Religion",
            MapMode::Hollowing => "Hohlung",
            MapMode::Economy => "Wohlstand",
            MapMode::Unrest => "Unruhe",
            MapMode::Diplomacy => "Diplomatie",
        }
    }
    pub fn id(self) -> &'static str {
        match self {
            MapMode::Realms => "realms",
            MapMode::Terrain => "terrain",
            MapMode::Religion => "religion",
            MapMode::Hollowing => "hollow",
            MapMode::Economy => "economy",
            MapMode::Unrest => "unrest",
            MapMode::Diplomacy => "diplomacy",
        }
    }
    pub fn from_id(s: &str) -> Option<MapMode> {
        MapMode::ALL
            .iter()
            .copied()
            .find(|m| m.id() == s || m.label().eq_ignore_ascii_case(s))
    }
    pub fn next(self) -> MapMode {
        let i = MapMode::ALL.iter().position(|m| *m == self).unwrap_or(0);
        MapMode::ALL[(i + 1) % MapMode::ALL.len()]
    }
    pub fn hint(self) -> &'static str {
        match self {
            MapMode::Realms => "Besitz der Reiche, Grenzen in Reichsfarben",
            MapMode::Terrain => "Gelände und Biome ohne politische Färbung",
            MapMode::Religion => "Mehrheitsglaube je Provinz, Sättigung = Anteil",
            MapMode::Hollowing => "Hohlung der Provinzen und Schleierrisse",
            MapMode::Economy => "Steuerkraft und Wohlstand der Provinzen",
            MapMode::Unrest => "Unruhe, Spannungen und Aufstandsgefahr",
            MapMode::Diplomacy => "Beziehungen der Reiche zum eigenen Reich",
        }
    }
}

fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn to_color(c: [f32; 3]) -> Color32 {
    Color32::from_rgb(
        c[0].clamp(0.0, 255.0) as u8,
        c[1].clamp(0.0, 255.0) as u8,
        c[2].clamp(0.0, 255.0) as u8,
    )
}

fn hsv(h: f32, s: f32, v: f32) -> [f32; 3] {
    let h = (h.fract() + 1.0).fract() * 6.0;
    let (i, f) = (h.floor(), h - h.floor());
    let (p, q, t) = (v * (1.0 - s), v * (1.0 - s * f), v * (1.0 - s * (1.0 - f)));
    let (r, g, b) = match i as i32 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    [r * 255.0, g * 255.0, b * 255.0]
}

/// Reichsfarbe: Kanonreiche haben feste Farben, spätere Gründungen leiten sie aus dem Banner ab.
pub fn realm_rgb(r: &Realm) -> [f32; 3] {
    match r.key.as_str() {
        "ember_crown" => [196.0, 56.0, 40.0],
        "ashmarch_march" => [222.0, 150.0, 52.0],
        "skarn_clans" => [74.0, 116.0, 184.0],
        "wolfspass_clan" => [118.0, 150.0, 206.0],
        "thorn_covenant" => [64.0, 156.0, 76.0],
        "veyl_free" => [138.0, 184.0, 96.0],
        "saelith_compact" => [58.0, 168.0, 178.0],
        "free_port_emberth" => [204.0, 184.0, 74.0],
        "grey_nomads" => [168.0, 156.0, 138.0],
        "slag_brood" => [140.0, 74.0, 52.0],
        "hollowed" => [90.0, 232.0, 136.0],
        "ashen_legion" => [190.0, 190.0, 206.0],
        _ => hsv((r.banner % 360) as f32 / 360.0, 0.55, 0.85),
    }
}

pub fn realm_color(r: &Realm) -> Color32 {
    to_color(realm_rgb(r))
}

fn biome_rgb(b: Biome) -> [f32; 3] {
    match b {
        Biome::Plains => [122.0, 140.0, 76.0],
        Biome::Hills => [134.0, 126.0, 80.0],
        Biome::Forest => [60.0, 92.0, 58.0],
        Biome::Mountain => [122.0, 116.0, 112.0],
        Biome::Swamp => [74.0, 88.0, 66.0],
        Biome::Desert => [198.0, 170.0, 112.0],
        Biome::Coast => [150.0, 152.0, 94.0],
        Biome::Island => [140.0, 150.0, 94.0],
        Biome::Volcanic => [96.0, 62.0, 54.0],
        Biome::Tundra => [172.0, 178.0, 172.0],
        Biome::Ruins => [124.0, 110.0, 98.0],
        Biome::Hollowland => [74.0, 96.0, 82.0],
    }
}

pub fn religion_rgb(r: Religion) -> [f32; 3] {
    match r {
        Religion::EternalFlame => [232.0, 130.0, 46.0],
        Religion::AncestorPath => [150.0, 150.0, 176.0],
        Religion::GreenCircle => [84.0, 176.0, 84.0],
        Religion::TideHouse => [60.0, 150.0, 196.0],
        Religion::HollowLiturgy => [128.0, 232.0, 150.0],
        Religion::SunHeresy => [240.0, 210.0, 70.0],
    }
}

/// Stützstellen einer Farbrampe.
fn ramp(stops: &[(f32, [f32; 3])], v: f32) -> [f32; 3] {
    if v <= stops[0].0 {
        return stops[0].1;
    }
    for w in stops.windows(2) {
        if v <= w[1].0 {
            return mix(w[0].1, w[1].1, (v - w[0].0) / (w[1].0 - w[0].0).max(1e-4));
        }
    }
    stops[stops.len() - 1].1
}

/// Steuerkraft einer Provinz (Kopfsteuer × Zufriedenheit + Gebäude) – Grundlage des Wohlstandsmodus.
pub fn prosperity(world: &StrategyWorld, p: ProvId) -> f32 {
    let pr = &world.provinces[p as usize];
    let head: f32 = STRATA
        .iter()
        .map(|s| pr.pop[s.idx()].total() * s.head_tax())
        .sum();
    head * (0.5 + pr.avg_satisfaction() / 100.0) / 100.0
        + pr.buildings.iter().map(|(_, n)| *n as f32).sum::<f32>() * 0.6
}

/// Legende eines Kartenmodus (Farbfeld, Beschriftung).
pub fn legend(mode: MapMode, world: &StrategyWorld) -> Vec<(Color32, String)> {
    let v = |c: Color32, s: &str| (c, s.to_string());
    match mode {
        MapMode::Realms => {
            let mut held: Vec<(RealmId, usize)> = world
                .realms
                .iter()
                .filter(|r| r.alive)
                .map(|r| {
                    (
                        r.id,
                        world.provinces.iter().filter(|p| p.holder() == Some(r.id)).count(),
                    )
                })
                .filter(|(_, n)| *n > 0)
                .collect();
            held.sort_by_key(|(id, n)| (Some(*id) != world.player, std::cmp::Reverse(*n)));
            let mut out: Vec<(Color32, String)> = held
                .iter()
                .map(|(id, _)| (realm_color(&world.realms[*id as usize]), world.rname(*id)))
                .collect();
            out.push(v(rgb([140, 140, 130]), "Herrenlos"));
            out
        }
        MapMode::Terrain => vec![
            v(to_color(biome_rgb(Biome::Plains)), "Ebene"),
            v(to_color(biome_rgb(Biome::Forest)), "Wald"),
            v(to_color(biome_rgb(Biome::Hills)), "Hügelland"),
            v(to_color(biome_rgb(Biome::Mountain)), "Gebirge"),
            v(to_color(biome_rgb(Biome::Desert)), "Wüste"),
            v(to_color(biome_rgb(Biome::Swamp)), "Sumpf"),
            v(to_color(biome_rgb(Biome::Tundra)), "Tundra"),
            v(to_color(biome_rgb(Biome::Coast)), "Küste"),
            v(to_color(biome_rgb(Biome::Island)), "Insel"),
            v(to_color(biome_rgb(Biome::Volcanic)), "Vulkan"),
            v(to_color(biome_rgb(Biome::Ruins)), "Ruinen"),
            v(to_color(biome_rgb(Biome::Hollowland)), "Hohlland"),
        ],
        MapMode::Religion => RELIGIONS
            .iter()
            .map(|r| (to_color(religion_rgb(*r)), r.name().to_string()))
            .chain(std::iter::once(v(rgb([140, 140, 130]), "Sättigung = Anteil des Glaubens")))
            .collect(),
        MapMode::Hollowing => vec![
            v(rgb([90, 110, 90]), "rein (0)"),
            v(rgb([90, 170, 115]), "gehohlt (50)"),
            v(rgb([90, 230, 140]), "stark gehohlt (100)"),
            v(rgb([176, 96, 230]), "Schleier ≥ 4"),
        ],
        MapMode::Economy => vec![
            v(rgb([70, 52, 40]), "arm"),
            v(rgb([196, 150, 60]), "wohlhabend"),
            v(rgb([250, 228, 130]), "reich"),
            v(rgb([140, 140, 130]), "Zahl = Steuerkraft"),
        ],
        MapMode::Unrest => vec![
            v(rgb([70, 140, 80]), "ruhig"),
            v(rgb([220, 190, 60]), "gespannt"),
            v(rgb([210, 50, 40]), "Aufstand"),
            v(rgb([140, 140, 130]), "Zahl = Unruhe 0–100"),
        ],
        MapMode::Diplomacy => vec![
            v(rgb([240, 200, 70]), "eigenes Reich / Vasallen"),
            v(rgb([80, 130, 230]), "Verbündete"),
            v(rgb([210, 50, 40]), "Krieg"),
            v(rgb([90, 190, 100]), "freundlich"),
            v(rgb([150, 150, 150]), "neutral"),
            v(rgb([190, 110, 60]), "feindlich"),
            v(rgb([70, 70, 70]), "herrenlos"),
        ],
    }
}

/// Färbung einer Provinz im Kartenmodus: (Farbe, Mischanteil mit dem Geländeton).
fn mode_tint(
    mode: MapMode,
    world: &StrategyWorld,
    p: ProvId,
    max_prosperity: f32,
) -> Option<([f32; 3], f32)> {
    let pr = &world.provinces[p as usize];
    match mode {
        MapMode::Terrain => None,
        MapMode::Realms => pr
            .holder()
            .map(|h| (realm_rgb(&world.realms[h as usize]), 0.58)),
        MapMode::Religion => {
            let r = pr.faith_majority();
            let share = pr.faith[r.idx()];
            Some((religion_rgb(r), 0.25 + 0.6 * share.clamp(0.0, 1.0)))
        }
        MapMode::Hollowing => {
            let h = (pr.hollowing / 100.0).clamp(0.0, 1.0);
            let base = ramp(
                &[
                    (0.0, [96.0, 112.0, 96.0]),
                    (0.5, [80.0, 190.0, 120.0]),
                    (1.0, [60.0, 250.0, 150.0]),
                ],
                h,
            );
            if pr.veil >= 4.0 {
                Some((mix(base, [176.0, 96.0, 230.0], 0.55), 0.85))
            } else {
                Some((base, 0.35 + 0.55 * h))
            }
        }
        MapMode::Economy => {
            let v = (prosperity(world, p) / max_prosperity.max(1.0)).clamp(0.0, 1.0);
            Some((
                ramp(
                    &[
                        (0.0, [70.0, 52.0, 40.0]),
                        (0.5, [196.0, 150.0, 60.0]),
                        (1.0, [252.0, 232.0, 130.0]),
                    ],
                    v,
                ),
                0.8,
            ))
        }
        MapMode::Unrest => {
            let v = (pr.unrest / 100.0)
                .clamp(0.0, 1.0)
                .max(pr.tension.iter().cloned().fold(0.0, f32::max) / 120.0);
            Some((
                ramp(
                    &[
                        (0.0, [70.0, 140.0, 80.0]),
                        (0.45, [220.0, 190.0, 60.0]),
                        (1.0, [210.0, 50.0, 40.0]),
                    ],
                    v,
                ),
                0.8,
            ))
        }
        MapMode::Diplomacy => {
            let me = world.player?;
            let Some(h) = pr.holder() else {
                return Some(([70.0, 70.0, 70.0], 0.5));
            };
            if h == me
                || world.realms[h as usize].vassal_of == Some(me)
                || world.realms[me as usize].vassal_of == Some(h)
            {
                return Some(([240.0, 200.0, 70.0], 0.75));
            }
            let c = if world.at_war(me, h) {
                [210.0, 50.0, 40.0]
            } else if world.has_treaty(TreatyKind::Alliance, me, h)
                || world.has_treaty(TreatyKind::DefensivePact, me, h)
            {
                [80.0, 130.0, 230.0]
            } else {
                let o = world.opinion(me, h);
                if o > 15.0 {
                    mix(
                        [150.0, 150.0, 150.0],
                        [90.0, 190.0, 100.0],
                        (o / 60.0).min(1.0),
                    )
                } else if o < -15.0 {
                    mix(
                        [150.0, 150.0, 150.0],
                        [190.0, 110.0, 60.0],
                        (-o / 60.0).min(1.0),
                    )
                } else {
                    [150.0, 150.0, 150.0]
                }
            };
            Some((c, 0.7))
        }
    }
}

struct Geometry {
    ids: Vec<u16>,
    sea: Vec<u8>,
    /// Statisches Gelände-Rauschen (0–1) und feines Körnungsrauschen.
    shade: Vec<f32>,
    speck: Vec<f32>,
    sea_points: Vec<Vec2>,
}

impl Geometry {
    fn build(world: &StrategyWorld, seed: u32) -> Geometry {
        let mut ids = vec![NONE; RW * RH];
        for p in &world.provinces {
            let poly = &p.polygon;
            if poly.len() < 3 {
                continue;
            }
            let (mut y0, mut y1) = (f32::MAX, f32::MIN);
            for v in poly {
                y0 = y0.min(v.y);
                y1 = y1.max(v.y);
            }
            let ty0 = ((y0 / UNIT).floor() as i32).max(0);
            let ty1 = ((y1 / UNIT).ceil() as i32).min(RH as i32 - 1);
            let mut xs: Vec<f32> = Vec::new();
            for ty in ty0..=ty1 {
                let y = (ty as f32 + 0.5) * UNIT;
                xs.clear();
                for i in 0..poly.len() {
                    let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
                    if (a.y <= y && b.y > y) || (b.y <= y && a.y > y) {
                        xs.push(a.x + (y - a.y) / (b.y - a.y) * (b.x - a.x));
                    }
                }
                xs.sort_by(|a, b| a.total_cmp(b));
                for pair in xs.chunks(2) {
                    if pair.len() < 2 {
                        continue;
                    }
                    let tx0 = ((pair[0] / UNIT - 0.5).ceil() as i32).max(0);
                    let tx1 = ((pair[1] / UNIT - 0.5).floor() as i32).min(RW as i32 - 1);
                    for tx in tx0..=tx1 {
                        ids[ty as usize * RW + tx as usize] = p.id;
                    }
                }
            }
        }
        // Abstand zum Land (Texel) per Breitensuche.
        let mut sea = vec![255u8; RW * RH];
        let mut queue = std::collections::VecDeque::new();
        for i in 0..RW * RH {
            if ids[i] != NONE {
                sea[i] = 0;
                queue.push_back(i);
            }
        }
        while let Some(i) = queue.pop_front() {
            let (x, y) = (i % RW, i / RW);
            let d = sea[i];
            if d >= 40 {
                continue;
            }
            let mut push = |nx: i32, ny: i32| {
                if nx < 0 || ny < 0 || nx >= RW as i32 || ny >= RH as i32 {
                    return;
                }
                let j = ny as usize * RW + nx as usize;
                if sea[j] == 255 {
                    sea[j] = d + 1;
                    queue.push_back(j);
                }
            };
            push(x as i32 + 1, y as i32);
            push(x as i32 - 1, y as i32);
            push(x as i32, y as i32 + 1);
            push(x as i32, y as i32 - 1);
        }
        let mut shade = vec![0.0; RW * RH];
        let mut speck = vec![0.0; RW * RH];
        let mut sea_points = Vec::new();
        for y in 0..RH {
            for x in 0..RW {
                let i = y * RW + x;
                shade[i] = fbm2(x as f32 * 0.07, y as f32 * 0.07, seed, 4);
                speck[i] = rand2(x as i32, y as i32, seed ^ 0xA5);
                if ids[i] == NONE && x % 6 == 0 && y % 5 == 0 && sea[i] > 3 && speck[i] > 0.35 {
                    sea_points.push(Vec2::new((x as f32 + 0.5) * UNIT, (y as f32 + 0.5) * UNIT));
                }
            }
        }
        Geometry {
            ids,
            sea,
            shade,
            speck,
            sea_points,
        }
    }
}

/// Zeichenkontext je Frame.
pub struct DrawCtx<'a> {
    pub world: &'a StrategyWorld,
    pub art: &'a ArtLibrary,
    pub ui_art: &'a UiArt,
    pub atlas: TextureId,
    pub t: f32,
    pub selected: Option<ProvId>,
    pub selected_army: Option<ArmyId>,
    pub preview: Option<Vec<ProvId>>,
}

/// Eingaben der Karte in diesem Frame.
#[derive(Default)]
pub struct MapInput {
    pub clicked: Option<ProvId>,
    pub clicked_army: Option<ArmyId>,
    pub right_clicked: Option<ProvId>,
    pub hover: Option<ProvId>,
    pub hover_army: Option<ArmyId>,
    /// Zeiger über dem gebündelten Ereignismarker einer Provinz.
    pub hover_marker: Option<ProvId>,
}

pub struct MapView {
    pub cam: Vec2,
    pub zoom: f32,
    geo: Option<Geometry>,
    tex: Option<TextureHandle>,
    tex_key: Option<(MapMode, u64)>,
    army_hits: Vec<(Rect, ArmyId)>,
    marker_hits: Vec<(Rect, ProvId)>,
    legend_signs: bool,
    drag_moved: f32,
    seed: u32,
}

impl MapView {
    pub fn new(seed: u64) -> MapView {
        MapView {
            cam: Vec2::new(MAP_W * 0.5, MAP_H * 0.5),
            zoom: 1.0,
            geo: None,
            tex: None,
            tex_key: None,
            army_hits: Vec::new(),
            marker_hits: Vec::new(),
            legend_signs: false,
            drag_moved: 0.0,
            seed: (seed as u32) ^ 0x5EED,
        }
    }

    pub fn center_on(&mut self, p: Vec2) {
        self.cam = p;
    }

    fn to_screen(&self, rect: Rect, p: Vec2) -> Pos2 {
        let c = rect.center();
        Pos2::new(
            c.x + (p.x - self.cam.x) * self.zoom,
            c.y + (p.y - self.cam.y) * self.zoom,
        )
    }

    fn to_map(&self, rect: Rect, s: Pos2) -> Vec2 {
        let c = rect.center();
        Vec2::new(
            self.cam.x + (s.x - c.x) / self.zoom,
            self.cam.y + (s.y - c.y) / self.zoom,
        )
    }

    pub fn province_at(&self, world: &StrategyWorld, m: Vec2) -> Option<ProvId> {
        if let Some(g) = &self.geo {
            let (tx, ty) = ((m.x / UNIT) as i32, (m.y / UNIT) as i32);
            if tx >= 0 && ty >= 0 && tx < RW as i32 && ty < RH as i32 {
                let id = g.ids[ty as usize * RW + tx as usize];
                if id != NONE {
                    return Some(id);
                }
            }
        }
        // Küstennähe großzügig: nächster Schwerpunkt innerhalb von 22 Kartenpunkten.
        world
            .provinces
            .iter()
            .filter(|p| (p.centroid - m).length() < 22.0)
            .min_by(|a, b| {
                (a.centroid - m)
                    .length()
                    .total_cmp(&(b.centroid - m).length())
            })
            .map(|p| p.id)
    }

    /// Texturen bei Bedarf neu erzeugen (Kartenmodus, Besitzänderungen, Monatswechsel).
    fn ensure_raster(&mut self, ctx: &egui::Context, world: &StrategyWorld, mode: MapMode) {
        if self.geo.is_none() {
            self.geo = Some(Geometry::build(world, self.seed));
        }
        let mut sig: u64 = (world.day / 30) as u64;
        for p in &world.provinces {
            sig = sig
                .wrapping_mul(1_000_003)
                .wrapping_add(p.holder().map(|h| h as u64 + 1).unwrap_or(0));
        }
        if self.tex.is_some() && self.tex_key == Some((mode, sig)) {
            return;
        }
        let geo = self.geo.as_ref().expect("Geometrie");
        let max_prosperity = (0..world.provinces.len())
            .map(|p| prosperity(world, p as ProvId))
            .fold(1.0, f32::max);
        let tints: Vec<Option<([f32; 3], f32)>> = (0..world.provinces.len())
            .map(|p| mode_tint(mode, world, p as ProvId, max_prosperity))
            .collect();
        let holder_rgb: Vec<Option<[f32; 3]>> = world
            .provinces
            .iter()
            .map(|p| p.holder().map(|h| realm_rgb(&world.realms[h as usize])))
            .collect();
        let holders: Vec<Option<RealmId>> = world.provinces.iter().map(|p| p.holder()).collect();
        let mut img = ColorImage::filled([RW, RH], Color32::BLACK);
        for y in 0..RH {
            for x in 0..RW {
                let i = y * RW + x;
                let id = geo.ids[i];
                let shade = geo.shade[i];
                let speck = geo.speck[i];
                if id == NONE {
                    // Meer: nahe der Küste hell, draußen tief; Wellenkämme als helle Striche.
                    let d = geo.sea[i] as f32;
                    let depth = (d / 16.0).clamp(0.0, 1.0);
                    let mut c = mix([58.0, 104.0, 124.0], [22.0, 40.0, 64.0], depth);
                    let wave = ((x as f32 * 0.55 + y as f32 * 0.9 + shade * 9.0).sin() * 0.5 + 0.5)
                        * (1.0 - depth * 0.6);
                    if wave > 0.93 && speck > 0.4 {
                        c = mix(c, [150.0, 196.0, 210.0], 0.45);
                    }
                    c = mix(c, [255.0, 255.0, 255.0], (speck - 0.5) * 0.05);
                    // Schaumsaum: hell an der Küste, nach außen ausgefranst.
                    let frill = 0.5 + 0.5 * (x as f32 * 0.9 + y as f32 * 0.6 + shade * 14.0).sin();
                    if d <= 1.0 {
                        c = mix(c, [244.0, 250.0, 244.0], 0.82);
                    } else if d <= 2.0 {
                        c = mix(c, [226.0, 240.0, 238.0], 0.6);
                    } else if d <= 3.0 {
                        c = mix(c, [196.0, 226.0, 232.0], 0.3 + 0.25 * frill);
                    } else if d <= 4.0 && frill > 0.62 {
                        c = mix(c, [170.0, 210.0, 222.0], 0.25);
                    }
                    img.pixels[i] = to_color(c);
                    continue;
                }
                let pr = &world.provinces[id as usize];
                let mut c = biome_rgb(pr.biome);
                // Geländetextur je Biom.
                let lum = 0.80 + 0.40 * shade + (speck - 0.5) * 0.10;
                c = [c[0] * lum, c[1] * lum, c[2] * lum];
                match pr.biome {
                    Biome::Forest | Biome::Swamp if speck > 0.78 => {
                        c = mix(c, [24.0, 48.0, 30.0], 0.55)
                    }
                    Biome::Mountain
                        if ((x as f32 * 0.5 - y as f32 * 0.8 + shade * 12.0).sin()) > 0.82 =>
                    {
                        c = mix(c, [60.0, 56.0, 56.0], 0.5)
                    }
                    Biome::Mountain if speck > 0.9 => c = mix(c, [220.0, 216.0, 210.0], 0.5),
                    Biome::Hills if ((y as f32 * 0.7 + shade * 10.0).sin()) > 0.8 => {
                        c = mix(c, [168.0, 156.0, 92.0], 0.35)
                    }
                    Biome::Desert
                        if ((x as f32 * 0.35 + y as f32 * 0.5 + shade * 8.0).sin()) > 0.85 =>
                    {
                        c = mix(c, [232.0, 206.0, 144.0], 0.4)
                    }
                    Biome::Volcanic if speck > 0.93 => c = mix(c, [255.0, 120.0, 40.0], 0.7),
                    Biome::Tundra if speck > 0.85 => c = mix(c, [236.0, 240.0, 240.0], 0.5),
                    Biome::Ruins if speck > 0.9 => c = mix(c, [168.0, 160.0, 150.0], 0.5),
                    Biome::Hollowland if speck > 0.88 => c = mix(c, [110.0, 230.0, 150.0], 0.25),
                    Biome::Plains | Biome::Coast | Biome::Island if speck > 0.9 => {
                        c = mix(c, [150.0, 170.0, 80.0], 0.4)
                    }
                    _ => {}
                }
                // Hohlung färbt in allen Modi die Landschaft leicht grün, Verwüstung dunkelt ab.
                let hollow = (pr.hollowing / 100.0).clamp(0.0, 1.0);
                c = mix(c, [70.0, 150.0, 100.0], hollow * 0.35);
                if let Some((tc, k)) = tints[id as usize] {
                    // Das Gelände scheint durch: Farbe wird mit der Helligkeit der Textur moduliert.
                    let tl = 0.74 + 0.5 * shade + (speck - 0.5) * 0.10;
                    c = mix(c, [tc[0] * tl, tc[1] * tl, tc[2] * tl], k);
                }
                // Reichsgrenze: Abstand (1–2 Texel) zu einem Texel mit anderem Halter.
                let mut realm_border = 0u8;
                'rb: for dist in 1..=2i32 {
                    for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                        let (nx, ny) = (x as i32 + dx * dist, y as i32 + dy * dist);
                        if nx < 0 || ny < 0 || nx >= RW as i32 || ny >= RH as i32 {
                            continue;
                        }
                        let nid = geo.ids[ny as usize * RW + nx as usize];
                        if nid != NONE && nid != id && holders[nid as usize] != holders[id as usize] {
                            realm_border = dist as u8;
                            break 'rb;
                        }
                    }
                }
                // Grenzen: Nachbartexel mit anderer Provinz.
                let mut border_other: Option<u16> = None;
                for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    let nid = if nx < 0 || ny < 0 || nx >= RW as i32 || ny >= RH as i32 {
                        NONE
                    } else {
                        geo.ids[ny as usize * RW + nx as usize]
                    };
                    if nid != id {
                        border_other = Some(nid);
                        if nid == NONE {
                            break;
                        }
                    }
                }
                if let Some(other) = border_other {
                    if other == NONE {
                        c = mix(c, [18.0, 14.0, 12.0], 0.62);
                    } else if holders[id as usize] != holders[other as usize] {
                        let rc = holder_rgb[id as usize].unwrap_or([150.0, 150.0, 140.0]);
                        c = mix(mix(rc, [255.0, 236.0, 190.0], 0.38), [14.0, 10.0, 8.0], 0.12);
                    } else {
                        c = mix(c, [20.0, 14.0, 10.0], 0.22);
                    }
                } else if realm_border == 2 {
                    let rc = holder_rgb[id as usize].unwrap_or([150.0, 150.0, 140.0]);
                    c = mix(c, mix(rc, [255.0, 236.0, 190.0], 0.2), 0.45);
                }
                img.pixels[i] = to_color(c);
            }
        }
        match &mut self.tex {
            Some(t) => t.set(img, TextureOptions::NEAREST),
            None => {
                self.tex = Some(ctx.load_texture("worldmap_raster", img, TextureOptions::NEAREST))
            }
        }
        self.tex_key = Some((mode, sig));
    }

    /// Zeichnet die Karte samt Overlays in `rect` und verarbeitet Kamera und Mauseingaben.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        rect: Rect,
        mode: MapMode,
        cx: &DrawCtx,
        block_input: bool,
    ) -> MapInput {
        let ctx = ui.ctx().clone();
        let world = cx.world;
        self.ensure_raster(&ctx, world, mode);
        let mut out = MapInput::default();
        // ---- Kamera
        let resp = ui.interact(
            rect,
            egui::Id::unique("worldmap_canvas"),
            egui::Sense::click_and_drag(),
        );
        let dt = ctx.input(|i| i.stable_dt).min(0.1);
        if !block_input {
            let (scroll, keys_pan, zoom_keys) = ctx.input(|i| {
                let mut pan = Vec2::ZERO;
                if i.key_down(egui::Key::ArrowLeft) || i.key_down(egui::Key::A) {
                    pan.x -= 1.0;
                }
                if i.key_down(egui::Key::ArrowRight) || i.key_down(egui::Key::D) {
                    pan.x += 1.0;
                }
                if i.key_down(egui::Key::ArrowUp) || i.key_down(egui::Key::W) {
                    pan.y -= 1.0;
                }
                if i.key_down(egui::Key::ArrowDown) || i.key_down(egui::Key::S) {
                    pan.y += 1.0;
                }
                let z = i.key_down(egui::Key::Plus) as i32 - i.key_down(egui::Key::Minus) as i32;
                (i.smooth_scroll_delta.y, pan, z)
            });
            if keys_pan != Vec2::ZERO {
                self.cam += keys_pan * 420.0 * dt / self.zoom;
            }
            // Kantenbildlauf (nur wenn der Zeiger im Fenster ist und nichts anderes ihn beansprucht).
            if let Some(p) = ctx.input(|i| i.pointer.hover_pos())
                && rect.contains(p)
                && !ctx.egui_wants_pointer_input()
                && resp.hovered()
            {
                {
                    let edge = 6.0;
                    let mut pan = Vec2::ZERO;
                    if p.x < rect.min.x + edge {
                        pan.x -= 1.0;
                    }
                    if p.x > rect.max.x - edge {
                        pan.x += 1.0;
                    }
                    if p.y < rect.min.y + edge {
                        pan.y -= 1.0;
                    }
                    if p.y > rect.max.y - edge {
                        pan.y += 1.0;
                    }
                    self.cam += pan * 360.0 * dt / self.zoom;
                }
            }
            let mut dz = zoom_keys as f32 * 1.2 * dt;
            if resp.hovered() && scroll != 0.0 {
                dz += scroll * 0.0025;
            }
            if dz != 0.0 {
                let before = ctx
                    .input(|i| i.pointer.hover_pos())
                    .filter(|p| rect.contains(*p))
                    .map(|p| (p, self.to_map(rect, p)));
                self.zoom = (self.zoom * (1.0 + dz)).clamp(0.55, 4.5);
                if let Some((p, m)) = before {
                    // Zoom auf den Zeiger: Kartenpunkt unter dem Zeiger bleibt stehen.
                    let c = rect.center();
                    self.cam =
                        Vec2::new(m.x - (p.x - c.x) / self.zoom, m.y - (p.y - c.y) / self.zoom);
                }
            }
        }
        if resp.dragged_by(egui::PointerButton::Primary)
            || resp.dragged_by(egui::PointerButton::Middle)
        {
            let d = resp.drag_delta();
            self.cam -= Vec2::new(d.x, d.y) / self.zoom;
            self.drag_moved += d.length();
        }
        let half = Vec2::new(rect.width(), rect.height()) / (2.0 * self.zoom);
        let margin = 60.0;
        let min = Vec2::new(
            (half.x - margin).min(MAP_W * 0.5),
            (half.y - margin).min(MAP_H * 0.5),
        );
        self.cam.x = self
            .cam
            .x
            .clamp(min.x.min(MAP_W - min.x), (MAP_W - min.x).max(min.x));
        self.cam.y = self
            .cam
            .y
            .clamp(min.y.min(MAP_H - min.y), (MAP_H - min.y).max(min.y));

        // ---- Hintergrund und Karte
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, Color32::from_rgb(12, 20, 32));
        let map_rect = Rect::from_min_max(
            self.to_screen(rect, Vec2::ZERO),
            self.to_screen(rect, Vec2::new(MAP_W, MAP_H)),
        );
        if let Some(t) = &self.tex {
            painter.image(
                t.id(),
                map_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        }
        self.draw_sea_shimmer(&painter, rect, cx);
        self.draw_overlays(&painter, rect, mode, cx);
        Self::draw_vignette(&painter, rect);
        Self::draw_frame(&painter, rect);
        Self::draw_compass(&painter, rect);

        // ---- Zeigerinteraktion
        if let Some(p) = ctx
            .input(|i| i.pointer.hover_pos())
            .filter(|p| rect.contains(*p) && resp.hovered())
        {
            out.hover_army = self
                .army_hits
                .iter()
                .rev()
                .find(|(r, _)| r.contains(p))
                .map(|(_, a)| *a);
            out.hover_marker = self
                .marker_hits
                .iter()
                .find(|(r, _)| r.contains(p))
                .map(|(_, id)| *id);
            let m = self.to_map(rect, p);
            out.hover = self.province_at(world, m);
        }
        if resp.clicked_by(egui::PointerButton::Primary) && self.drag_moved < 6.0 {
            out.clicked_army = out.hover_army;
            out.clicked = out.hover;
        }
        if resp.clicked_by(egui::PointerButton::Secondary) {
            out.right_clicked = out.hover;
        }
        if !resp.dragged() {
            self.drag_moved = 0.0;
        }
        out
    }

    /// Leichte Vignette: Ränder und Ecken dunkeln sanft ab.
    fn draw_vignette(painter: &Painter, rect: Rect) {
        let depth = (rect.height() * 0.16).clamp(50.0, 120.0);
        let a_out = 120u8;
        let edge = |p: &Painter, r: Rect, outer: [bool; 4]| {
            // Ecken: links-oben, rechts-oben, rechts-unten, links-unten; true = außen (dunkel).
            let col = |o: bool| {
                if o {
                    Color32::from_black_alpha(a_out)
                } else {
                    Color32::TRANSPARENT
                }
            };
            let mut m = egui::Mesh::default();
            m.colored_vertex(r.left_top(), col(outer[0]));
            m.colored_vertex(r.right_top(), col(outer[1]));
            m.colored_vertex(r.right_bottom(), col(outer[2]));
            m.colored_vertex(r.left_bottom(), col(outer[3]));
            m.add_triangle(0, 1, 2);
            m.add_triangle(0, 2, 3);
            p.add(Shape::mesh(m));
        };
        let (w, h) = (depth * 1.4, depth);
        edge(painter, Rect::from_min_size(rect.min, EVec2::new(w, rect.height())), [true, false, false, true]);
        edge(
            painter,
            Rect::from_min_size(Pos2::new(rect.max.x - w, rect.min.y), EVec2::new(w, rect.height())),
            [false, true, true, false],
        );
        edge(painter, Rect::from_min_size(rect.min, EVec2::new(rect.width(), h)), [true, true, false, false]);
        edge(
            painter,
            Rect::from_min_size(Pos2::new(rect.min.x, rect.max.y - h), EVec2::new(rect.width(), h)),
            [false, false, true, true],
        );
    }

    /// Kartenrahmen: Pergamentrand mit dunklen Fassungslinien und Eckzier.
    fn draw_frame(painter: &Painter, rect: Rect) {
        let band = 9.0;
        let parch = Color32::from_rgb(176, 148, 98);
        let parch_hi = Color32::from_rgb(214, 188, 132);
        let dark = Color32::from_rgb(26, 18, 12);
        for r in [
            Rect::from_min_size(rect.min, EVec2::new(rect.width(), band)),
            Rect::from_min_size(Pos2::new(rect.min.x, rect.max.y - band), EVec2::new(rect.width(), band)),
            Rect::from_min_size(rect.min, EVec2::new(band, rect.height())),
            Rect::from_min_size(Pos2::new(rect.max.x - band, rect.min.y), EVec2::new(band, rect.height())),
        ] {
            painter.rect_filled(r, 0.0, parch);
        }
        painter.rect_stroke(rect.shrink(1.0), 0.0, Stroke::new(2.0, dark), egui::StrokeKind::Inside);
        painter.rect_stroke(rect.shrink(3.5), 0.0, Stroke::new(1.0, parch_hi), egui::StrokeKind::Inside);
        painter.rect_stroke(rect.shrink(band), 0.0, Stroke::new(2.0, dark), egui::StrokeKind::Outside);
        for c in [rect.left_top(), rect.right_top(), rect.right_bottom(), rect.left_bottom()] {
            let d = EVec2::new(
                if c.x <= rect.center().x { 1.0 } else { -1.0 },
                if c.y <= rect.center().y { 1.0 } else { -1.0 },
            );
            let at = c + d * 14.0;
            painter.add(Shape::convex_polygon(
                vec![
                    at + EVec2::new(0.0, -7.0),
                    at + EVec2::new(7.0, 0.0),
                    at + EVec2::new(0.0, 7.0),
                    at + EVec2::new(-7.0, 0.0),
                ],
                Color32::from_rgb(214, 170, 82),
                Stroke::new(1.5, dark),
            ));
        }
    }

    /// Dezente Kompassrose unten rechts.
    fn draw_compass(painter: &Painter, rect: Rect) {
        let c = Pos2::new(rect.max.x - 70.0, rect.max.y - 78.0);
        let r = 34.0;
        let a = 0.55;
        let ink = Color32::from_rgba_unmultiplied(236, 220, 176, (200.0 * a) as u8);
        let shade = Color32::from_rgba_unmultiplied(20, 14, 10, (170.0 * a) as u8);
        painter.circle_filled(c, r + 4.0, Color32::from_rgba_unmultiplied(14, 20, 30, (110.0 * a) as u8));
        painter.circle_stroke(c, r, Stroke::new(1.5, ink));
        painter.circle_stroke(c, r * 0.62, Stroke::new(1.0, ink.gamma_multiply(0.6)));
        for k in 0..4 {
            let ang = k as f32 * std::f32::consts::FRAC_PI_2;
            let (dx, dy) = (ang.sin(), -ang.cos());
            let d = EVec2::new(dx, dy);
            let n = EVec2::new(-dy, dx);
            let len = if k == 0 { r + 8.0 } else { r };
            painter.add(Shape::convex_polygon(
                vec![c + d * len, c + n * 6.0, c - n * 6.0],
                if k == 0 { Color32::from_rgba_unmultiplied(214, 96, 70, (230.0 * a) as u8) } else { ink },
                Stroke::new(1.0, shade),
            ));
            painter.add(Shape::convex_polygon(
                vec![c - d * (r * 0.55), c + n * 4.0, c - n * 4.0],
                ink.gamma_multiply(0.7),
                Stroke::new(1.0, shade),
            ));
        }
        painter.text(
            c + EVec2::new(0.0, -r - 18.0),
            egui::Align2::CENTER_CENTER,
            "N",
            ashen_engine::ui::theme::caps_font(13.0),
            ink,
        );
    }

    fn draw_sea_shimmer(&self, painter: &Painter, rect: Rect, cx: &DrawCtx) {
        let Some(g) = &self.geo else { return };
        let px = (UNIT * self.zoom).max(1.0);
        for (i, p) in g.sea_points.iter().enumerate() {
            let s = self.to_screen(rect, *p);
            if !rect.contains(s) {
                continue;
            }
            let phase = (cx.t * 0.9 + i as f32 * 1.37).sin() * 0.5 + 0.5;
            if phase > 0.55 {
                let a = ((phase - 0.55) / 0.45 * 90.0) as u8;
                painter.rect_filled(
                    Rect::from_min_size(s, EVec2::new(px * 2.0, px)),
                    0.0,
                    Color32::from_rgba_unmultiplied(190, 226, 236, a),
                );
            }
        }
    }

    fn centroid(world: &StrategyWorld, p: ProvId) -> Vec2 {
        world.provinces[p as usize].centroid
    }

    fn draw_overlays(&mut self, painter: &Painter, rect: Rect, mode: MapMode, cx: &DrawCtx) {
        let world = cx.world;
        let z = self.zoom;
        let t = cx.t;
        self.army_hits.clear();
        // Nebel über Hohlungsgebieten und Glut bei Krieg.
        for p in &world.provinces {
            let c = self.to_screen(rect, p.centroid);
            if !rect.expand(120.0).contains(c) {
                continue;
            }
            if p.hollowing >= 25.0 || p.shrine > 0 {
                let k = ((p.hollowing - 20.0) / 80.0).clamp(0.15, 1.0);
                for j in 0..4 {
                    let ph = t * 0.12 + j as f32 * 1.7 + p.id as f32;
                    let off = EVec2::new(ph.sin() * 36.0, (ph * 0.8).cos() * 22.0) * z;
                    let size = EVec2::new(150.0 + j as f32 * 26.0, 70.0 + j as f32 * 12.0) * z;
                    let a = k * (0.5 + 0.5 * (t * 0.3 + j as f32).sin());
                    cx.ui_art.glow(
                        painter,
                        Rect::from_center_size(c + off, size),
                        Color32::from_rgb(
                            (10.0 + 18.0 * a) as u8,
                            (34.0 + 48.0 * a) as u8,
                            (22.0 + 26.0 * a) as u8,
                        ),
                    );
                }
            }
            let war = p.siege.is_some()
                || world.battles.iter().any(|b| b.province == p.id)
                || p.rebel_held;
            if war {
                let a = 0.6 + 0.4 * (t * 4.0 + p.id as f32).sin();
                cx.ui_art.glow(
                    painter,
                    Rect::from_center_size(c, EVec2::new(150.0, 100.0) * z),
                    Color32::from_rgb((120.0 * a) as u8, (44.0 * a) as u8, (8.0 * a) as u8),
                );
                let area = Rect::from_center_size(c, EVec2::new(90.0, 80.0) * z);
                ui_art::embers(
                    cx.ui_art,
                    &painter.with_clip_rect(rect.intersect(area)),
                    area,
                    t,
                    10,
                    p.id as u32 + 3,
                );
            }
        }
        // Auswahlumriss, Provinz-Umrisse bei Zeigerkontakt folgen im Anschluss an die Icons.
        if let Some(sel) = cx.selected {
            let poly: Vec<Pos2> = world.provinces[sel as usize]
                .polygon
                .iter()
                .map(|v| self.to_screen(rect, *v))
                .collect();
            painter.add(Shape::closed_line(
                poly.clone(),
                Stroke::new(3.0, Color32::from_rgb(20, 10, 4)),
            ));
            let pulse = 0.75 + 0.25 * (t * 3.0).sin();
            painter.add(Shape::closed_line(
                poly,
                Stroke::new(
                    1.8,
                    Color32::from_rgb(
                        (250.0 * pulse) as u8,
                        (214.0 * pulse) as u8,
                        (120.0 * pulse) as u8,
                    ),
                ),
            ));
        }
        // Bewegungspfeile.
        let player = world.player;
        for a in &world.armies {
            if a.path.is_empty()
                || !(Some(a.owner.unwrap_or(u16::MAX)) == player || cx.selected_army == Some(a.id))
            {
                continue;
            }
            let selected = cx.selected_army == Some(a.id);
            let col = if selected {
                Color32::from_rgb(255, 224, 120)
            } else {
                Color32::from_rgb(220, 190, 110).gamma_multiply(0.75)
            };
            let mut pts = vec![self.army_pos(world, a)];
            for p in &a.path {
                pts.push(Self::centroid(world, *p));
            }
            self.draw_arrow(
                painter,
                rect,
                &pts,
                col,
                selected,
                a.path.iter().any(|p| {
                    world.provinces[a.location as usize]
                        .neighbors
                        .iter()
                        .any(|l| l.to == *p && l.kind == LinkKind::Sea)
                }),
            );
        }
        if let (Some(path), Some(aid)) = (&cx.preview, cx.selected_army)
            && let Some(a) = world.army(aid)
        {
            let mut pts = vec![self.army_pos(world, a)];
            pts.extend(path.iter().map(|p| Self::centroid(world, *p)));
            self.draw_arrow(
                painter,
                rect,
                &pts,
                Color32::from_rgb(150, 230, 255),
                false,
                false,
            );
        }
        // Siedlungen und Marker an festen Ankern; die belegten Flächen sperren Beschriftungen.
        let mut occ: Vec<Rect> = Vec::new();
        self.marker_hits.clear();
        let mut sites: Vec<(ProvId, Pos2, f32)> = Vec::new();
        for p in &world.provinces {
            let c = self.to_screen(rect, p.centroid);
            if !rect.expand(60.0).contains(c) {
                continue;
            }
            let size = Self::site_size(p, z);
            self.draw_settlement(painter, c, size, p, world, cx);
            occ.push(Rect::from_center_size(
                c + EVec2::new(0.0, -size * 0.05),
                EVec2::new(size * 1.4, size * 1.0),
            ));
            let marks = province_markers(world, p);
            if !marks.is_empty() {
                let br = Self::badge_rect(c, size, z);
                Self::draw_marker_badge(painter, br, &marks, t, z, p.id);
                self.marker_hits.push((br.expand(3.0), p.id));
                occ.push(br.expand(3.0));
            }
            if matches!(mode, MapMode::Economy | MapMode::Unrest | MapMode::Hollowing) {
                let v = match mode {
                    MapMode::Economy => format!("{:.0}", prosperity(world, p.id)),
                    MapMode::Unrest => format!("{:.0}", p.unrest),
                    _ => format!("{:.0}", p.hollowing),
                };
                let pr = Self::plaque(
                    painter,
                    c + EVec2::new(size * 0.62, -size * 0.5),
                    &v,
                    11.0 * z.clamp(0.9, 1.3),
                    Color32::from_rgb(200, 170, 90),
                    Color32::from_rgb(255, 244, 210),
                );
                occ.push(pr.expand(2.0));
            }
            sites.push((p.id, c, size));
        }
        // Regionsnamen (nur bei weitem Zoom, transparent) über den Siedlungen, unter Heeren und Namen.
        self.draw_region_names(painter, rect, world);
        // Heere: fester Standplatz neben dem Siedlungssymbol, marschierende an ihrer Position.
        let mut at: std::collections::BTreeMap<ProvId, Vec<&ashen_strategy::Army>> =
            std::collections::BTreeMap::new();
        for a in &world.armies {
            if a.path.is_empty() || a.progress <= 0.0 {
                at.entry(a.location).or_default().push(a);
            }
        }
        let mut army_draws: Vec<(Pos2, &ashen_strategy::Army)> = Vec::new();
        for (loc, list) in &at {
            let lp = &world.provinces[*loc as usize];
            let c = self.to_screen(rect, lp.centroid);
            let size = Self::site_size(lp, z);
            for (i, a) in list.iter().enumerate() {
                army_draws.push((Self::army_stand(c, size, z, i), a));
            }
        }
        for a in world
            .armies
            .iter()
            .filter(|a| !a.path.is_empty() && a.progress > 0.0)
        {
            army_draws.push((self.to_screen(rect, self.army_pos(world, a)), a));
        }
        for (pos, _) in &army_draws {
            occ.push(Self::army_rect(*pos, z).expand(2.0));
        }
        self.draw_labels(painter, rect, cx, &sites, &mut occ);
        for (pos, a) in army_draws {
            self.draw_army(painter, pos, a, world, cx, rect);
        }
    }

    fn army_pos(&self, world: &StrategyWorld, a: &ashen_strategy::Army) -> Vec2 {
        let from = Self::centroid(world, a.location);
        match a.path.first() {
            Some(n) if a.progress > 0.0 => {
                from.lerp(Self::centroid(world, *n), a.progress.clamp(0.0, 1.0))
            }
            _ => from,
        }
    }

    fn draw_arrow(
        &self,
        painter: &Painter,
        rect: Rect,
        pts: &[Vec2],
        col: Color32,
        bold: bool,
        dashed_sea: bool,
    ) {
        if pts.len() < 2 {
            return;
        }
        let s: Vec<Pos2> = pts.iter().map(|p| self.to_screen(rect, *p)).collect();
        let w = if bold { 3.0 } else { 2.0 };
        painter.add(Shape::line(
            s.clone(),
            Stroke::new(w + 2.0, Color32::from_black_alpha(170)),
        ));
        if dashed_sea {
            painter.extend(Shape::dashed_line(&s, Stroke::new(w, col), 7.0, 4.0));
        } else {
            painter.add(Shape::line(s.clone(), Stroke::new(w, col)));
        }
        let (a, b) = (s[s.len() - 2], s[s.len() - 1]);
        let d = (b - a).normalized();
        if d.x.is_finite() {
            let n = EVec2::new(-d.y, d.x);
            let tip = b;
            let base = b - d * 12.0;
            painter.add(Shape::convex_polygon(
                vec![tip, base + n * 6.0, base - n * 6.0],
                col,
                Stroke::new(1.0, Color32::from_black_alpha(200)),
            ));
        }
    }

    /// Bildschirmgröße des Siedlungssymbols (ohne Zoom-Begrenzung in Karteneinheiten).
    fn site_size(p: &ashen_strategy::Province, z: f32) -> f32 {
        (26.0 + 8.0 * p.settlement.rank() as f32) * z.clamp(0.7, 1.7)
    }

    /// Rechteck des gebündelten Ereignismarkers über der Siedlung.
    fn badge_rect(c: Pos2, size: f32, z: f32) -> Rect {
        let side = if z >= 2.4 { 26.0 } else { 20.0 };
        Rect::from_center_size(c + EVec2::new(0.0, -size * 0.62 - side * 0.55), EVec2::splat(side))
    }

    /// Belegtes Rechteck eines Heeres (Figur, Banner und Stärkeplakette).
    fn army_rect(pos: Pos2, z: f32) -> Rect {
        let size = 34.0 * z.clamp(0.8, 1.7);
        Rect::from_min_max(
            Pos2::new(pos.x - size * 0.55, pos.y - size * 0.95),
            Pos2::new(pos.x + size * 0.6, pos.y + size * 0.22 + 17.0),
        )
    }

    /// Fester Standplatz der Heere einer Provinz: rechts neben dem Siedlungssymbol, nie auf dem Namen.
    fn army_stand(c: Pos2, size: f32, z: f32, idx: usize) -> Pos2 {
        let az = z.clamp(0.8, 1.7);
        c + EVec2::new(size * 0.5 + 20.0 * az + idx as f32 * 34.0 * az, size * 0.42)
    }

    /// Reihenfolge der Beschriftungen: Auswahl > Spielerreich/Hauptstädte > Siedlungsgröße > Provinzfläche.
    fn label_priority(world: &StrategyWorld, id: ProvId, selected: Option<ProvId>) -> f32 {
        let p = &world.provinces[id as usize];
        let mut s = p.settlement.rank() as f32 * 30.0;
        let poly = &p.polygon;
        let mut area = 0.0;
        for i in 0..poly.len() {
            let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
            area += a.x * b.y - b.x * a.y;
        }
        s += (area.abs() * 0.5 / 1500.0).min(40.0);
        if world.realms.iter().any(|r| r.alive && r.capital == id) {
            s += 500.0;
        }
        if world.player.is_some() && p.holder() == world.player {
            s += 300.0;
        }
        if selected == Some(id) {
            s += 1000.0;
        }
        s
    }

    /// Regionsnamen: groß, gesperrt und transparent bei weitem Zoom.
    fn draw_region_names(&self, painter: &Painter, rect: Rect, world: &StrategyWorld) {
        let z = self.zoom;
        let a = ((1.2 - z) / 0.3).clamp(0.0, 1.0);
        if a <= 0.02 {
            return;
        }
        let font = ashen_engine::ui::theme::title_font((24.0 * z).clamp(16.0, 30.0));
        for reg in &world.regions {
            if reg.provinces.is_empty() {
                continue;
            }
            let mut m = Vec2::ZERO;
            for p in &reg.provinces {
                m += world.provinces[*p as usize].centroid;
            }
            m /= reg.provinces.len() as f32;
            let c = self.to_screen(rect, m);
            if !rect.expand(200.0).contains(c) {
                continue;
            }
            let name = reg.name.to_uppercase();
            let galleys: Vec<_> = name
                .chars()
                .map(|ch| painter.layout_no_wrap(ch.to_string(), font.clone(), Color32::WHITE))
                .collect();
            let gap = font.size * 0.16;
            let total: f32 = galleys.iter().map(|g| g.size().x + gap).sum::<f32>() - gap;
            let h = galleys.first().map(|g| g.size().y).unwrap_or(0.0);
            let mut x = c.x - total / 2.0;
            for (ch, g) in name.chars().zip(galleys) {
                let w = g.size().x;
                let pos = Pos2::new(x, c.y - h / 2.0);
                let txt = ch.to_string();
                painter.text(
                    pos + EVec2::new(1.5, 1.5),
                    egui::Align2::LEFT_TOP,
                    &txt,
                    font.clone(),
                    Color32::from_black_alpha((120.0 * a) as u8),
                );
                painter.text(
                    pos,
                    egui::Align2::LEFT_TOP,
                    &txt,
                    font.clone(),
                    Color32::from_rgba_unmultiplied(240, 226, 186, (150.0 * a) as u8),
                );
                x += w + gap;
            }
        }
    }

    /// Provinznamen mit Kollisionsvermeidung: wichtige zuerst, Ausweichplätze, kleinere Schrift, sonst ausgeblendet.
    fn draw_labels(
        &self,
        painter: &Painter,
        rect: Rect,
        cx: &DrawCtx,
        sites: &[(ProvId, Pos2, f32)],
        occ: &mut Vec<Rect>,
    ) {
        let z = self.zoom;
        let world = cx.world;
        if z < 1.0 {
            return;
        }
        let mut order: Vec<(f32, usize)> = sites
            .iter()
            .enumerate()
            .map(|(i, (id, _, _))| (Self::label_priority(world, *id, cx.selected), i))
            .collect();
        order.sort_by(|a, b| b.0.total_cmp(&a.0));
        let base = 11.5 * z.clamp(0.95, 1.5);
        for (prio, i) in order {
            if z < 1.3 && prio < 300.0 {
                continue;
            }
            let (id, c, size) = sites[i];
            let p = &world.provinces[id as usize];
            let col = if prio >= 500.0 {
                Color32::from_rgb(255, 240, 200)
            } else {
                Color32::from_rgb(240, 228, 196)
            };
            let forced = prio >= 500.0;
            let mut chosen: Option<(Rect, std::sync::Arc<egui::Galley>, f32)> = None;
            'search: for scale in [1.0_f32, 0.84] {
                let fsize = (base * scale).round().max(8.0);
                let g = painter.layout_no_wrap(
                    p.name.clone(),
                    ashen_engine::ui::theme::caps_font(fsize),
                    col,
                );
                let sz = g.size();
                let cands = [
                    Rect::from_min_size(Pos2::new(c.x - sz.x / 2.0, c.y + size * 0.5), sz),
                    Rect::from_min_size(
                        Pos2::new(c.x - sz.x / 2.0, c.y - size * 0.62 - 26.0 - sz.y),
                        sz,
                    ),
                    Rect::from_min_size(Pos2::new(c.x - size * 0.78 - sz.x, c.y - sz.y / 2.0), sz),
                    Rect::from_min_size(Pos2::new(c.x + size * 0.78, c.y - sz.y / 2.0), sz),
                ];
                for r in cands.iter() {
                    let probe = r.expand2(EVec2::new(3.0, 0.0));
                    if !rect.intersects(*r) {
                        continue;
                    }
                    if !occ.iter().any(|o| o.intersects(probe)) {
                        chosen = Some((*r, g.clone(), fsize));
                        break 'search;
                    }
                }
            }
            if chosen.is_none() && forced {
                // Wichtige Namen werden trotzdem gesetzt: Platz mit der kleinsten Überdeckung.
                let fsize = (base * 0.84).round().max(8.0);
                let g = painter.layout_no_wrap(
                    p.name.clone(),
                    ashen_engine::ui::theme::caps_font(fsize),
                    col,
                );
                let sz = g.size();
                let cands = [
                    Rect::from_min_size(Pos2::new(c.x - sz.x / 2.0, c.y + size * 0.5), sz),
                    Rect::from_min_size(
                        Pos2::new(c.x - sz.x / 2.0, c.y - size * 0.62 - 26.0 - sz.y),
                        sz,
                    ),
                    Rect::from_min_size(Pos2::new(c.x - size * 0.78 - sz.x, c.y - sz.y / 2.0), sz),
                    Rect::from_min_size(Pos2::new(c.x + size * 0.78, c.y - sz.y / 2.0), sz),
                    Rect::from_min_size(Pos2::new(c.x - sz.x / 2.0, c.y + size * 0.5 + sz.y), sz),
                ];
                let cost = |r: &Rect| -> f32 {
                    occ.iter()
                        .map(|o| {
                            let i = o.intersect(*r);
                            i.width().max(0.0) * i.height().max(0.0)
                        })
                        .sum()
                };
                if let Some(best) = cands.iter().min_by(|a, b| cost(a).total_cmp(&cost(b))) {
                    chosen = Some((*best, g, fsize));
                }
            }
            let Some((r, g, fsize)) = chosen else {
                continue;
            };
            occ.push(r.expand2(EVec2::new(3.0, 0.0)));
            let shadow = painter.layout_no_wrap(
                p.name.clone(),
                ashen_engine::ui::theme::caps_font(fsize),
                Color32::from_black_alpha(230),
            );
            for (dx, dy) in [(1.0, 1.0), (-1.0, 1.0)] {
                painter.galley(r.min + EVec2::new(dx, dy), shadow.clone(), Color32::BLACK);
            }
            painter.galley(r.min, g, col);
        }
    }

    fn draw_settlement(
        &self,
        painter: &Painter,
        c: Pos2,
        size: f32,
        p: &ashen_strategy::Province,
        world: &StrategyWorld,
        cx: &DrawCtx,
    ) {
        let name = match p.settlement {
            SettlementLevel::Village => "bld_hut",
            SettlementLevel::Town => "bld_house",
            SettlementLevel::City => "bld_townhouse",
            SettlementLevel::Capital => "bld_keep",
        };
        let r = Rect::from_center_size(c + EVec2::new(0.0, -size * 0.1), EVec2::splat(size));
        cx.ui_art.soft_shadow(painter, r.expand(size * 0.25), 0.4);
        if let Some(s) = cx.art.atlas.try_get(name) {
            ui_common::paint_sprite(painter, cx.atlas, s, r, Color32::WHITE);
        } else {
            painter.rect_filled(r.shrink(size * 0.2), 0.0, Color32::from_rgb(200, 170, 120));
        }
        if p.fortification >= 3
            && let Some(s) = cx.art.atlas.try_get(if p.fortification >= 6 {
                "bld_wall_tower"
            } else {
                "bld_watchtower"
            })
        {
            ui_common::paint_sprite(
                painter,
                cx.atlas,
                s,
                Rect::from_center_size(
                    c + EVec2::new(size * 0.55, size * 0.05),
                    EVec2::splat(size * 0.62),
                ),
                Color32::WHITE,
            );
        }
        // Reichsfahne an Hauptstädten.
        if let Some(h) = p.owner
            && world.realms[h as usize].capital == p.id
        {
            let col = realm_color(&world.realms[h as usize]);
            let top = c + EVec2::new(-size * 0.45, -size * 0.75);
            painter.line_segment(
                [top, top + EVec2::new(0.0, size * 0.55)],
                Stroke::new(2.0, Color32::from_rgb(40, 28, 20)),
            );
            painter.add(Shape::convex_polygon(
                vec![
                    top,
                    top + EVec2::new(size * 0.4, size * 0.12),
                    top + EVec2::new(0.0, size * 0.26),
                ],
                col,
                Stroke::new(1.0, Color32::BLACK),
            ));
        }
    }

    /// Kleines Plakettchen mit Zahl (Heeresstärke, Kartenwerte).
    fn plaque(painter: &Painter, center: Pos2, text: &str, size: f32, border: Color32, fg: Color32) -> Rect {
        let g = painter.layout_no_wrap(text.to_string(), ashen_engine::ui::theme::body_bold_font(size), fg);
        let r = Rect::from_center_size(center, g.size() + EVec2::new(8.0, 2.0));
        painter.rect_filled(r, 3.0, Color32::from_rgba_unmultiplied(20, 14, 10, 225));
        painter.rect_stroke(r, 3.0, Stroke::new(1.0, border), egui::StrokeKind::Inside);
        painter.galley(r.center() - g.size() / 2.0, g, fg);
        r
    }

    fn draw_army(
        &mut self,
        painter: &Painter,
        pos: Pos2,
        a: &ashen_strategy::Army,
        world: &StrategyWorld,
        cx: &DrawCtx,
        rect: Rect,
    ) {
        let z = self.zoom.clamp(0.8, 1.7);
        let size = 34.0 * z;
        let col = match a.owner {
            Some(o) if matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary) => {
                realm_color(&world.realms[o as usize])
            }
            _ => match a.kind {
                ArmyKind::Hollow | ArmyKind::Boss => Color32::from_rgb(90, 232, 136),
                ArmyKind::Legion => Color32::from_rgb(190, 190, 206),
                ArmyKind::Rebels => Color32::from_rgb(230, 120, 70),
                _ => Color32::from_rgb(200, 80, 70),
            },
        };
        let r = Rect::from_center_size(
            pos + EVec2::new(0.0, -size * 0.3),
            EVec2::new(size * 0.95, size * 1.15),
        );
        if !rect.expand(40.0).intersects(r) {
            return;
        }
        let mine = world.player.is_some() && a.owner == world.player;
        let selected = cx.selected_army == Some(a.id);
        cx.ui_art.soft_shadow(painter, r.expand(size * 0.2), 0.45);
        // Banner: Stange mit Fahne in Reichsfarbe.
        let pole_top = Pos2::new(r.left() + size * 0.1, r.top() - size * 0.05);
        painter.line_segment(
            [pole_top, pole_top + EVec2::new(0.0, size * 0.9)],
            Stroke::new(2.0, Color32::from_rgb(46, 30, 20)),
        );
        let wave = (cx.t * 3.0 + a.id as f32).sin() * size * 0.05;
        painter.add(Shape::convex_polygon(
            vec![
                pole_top,
                pole_top + EVec2::new(size * 0.48, size * 0.08 + wave),
                pole_top + EVec2::new(size * 0.4, size * 0.2),
                pole_top + EVec2::new(size * 0.5, size * 0.32 - wave),
                pole_top + EVec2::new(0.0, size * 0.34),
            ],
            col,
            Stroke::new(1.0, Color32::from_black_alpha(220)),
        ));
        // Figur: Sprite des stärksten Regiments.
        let lead = a
            .regiments
            .iter()
            .max_by_key(|r| r.count)
            .map(|r| r.unit.as_str())
            .unwrap_or("swordsman");
        let sprite_id = crate::worldmap::battle::UNIT_TABLE
            .iter()
            .find(|(s, _, _)| *s == lead)
            .map(|(_, r, _)| *r)
            .unwrap_or("swordsman");
        let frame = cx
            .art
            .atlas
            .try_get(&format!("{lead}_idle_0_0"))
            .or_else(|| cx.art.atlas.try_get(&format!("{sprite_id}_idle_0_0")));
        let figure = Rect::from_center_size(
            pos + EVec2::new(size * 0.12, -size * 0.12),
            EVec2::splat(size * 0.95),
        );
        match frame {
            Some(s) => ui_common::paint_sprite(painter, cx.atlas, s, figure, Color32::WHITE),
            None => {
                painter.circle_filled(figure.center(), size * 0.25, col);
            }
        }
        // Stärke als kleines Plakettchen unter der Figur.
        Self::plaque(
            painter,
            pos + EVec2::new(size * 0.05, size * 0.22 + 8.0),
            &format!("{}", a.size()),
            10.5 * z,
            col.gamma_multiply(0.9),
            if mine {
                Color32::from_rgb(255, 232, 160)
            } else {
                Color32::from_rgb(240, 220, 210)
            },
        );
        if a.in_battle.is_some() {
            Marker::Swords.draw(painter, r.center_top() + EVec2::new(-10.0, -12.0), 2.0, 1.0);
        } else if a.order == ArmyOrder::Siege {
            Marker::Flame.draw(painter, r.center_top() + EVec2::new(-10.0, -12.0), 2.0, 1.0);
        }
        if selected {
            painter.rect_stroke(
                r.expand(3.0),
                0.0,
                Stroke::new(2.0, Color32::from_rgb(255, 224, 120)),
                egui::StrokeKind::Outside,
            );
        } else if mine {
            painter.rect_stroke(
                r.expand(1.0),
                0.0,
                Stroke::new(1.0, Color32::from_rgb(200, 170, 90).gamma_multiply(0.7)),
                egui::StrokeKind::Outside,
            );
        }
        self.army_hits.push((r.expand(4.0), a.id));
    }

    /// Gebündelter Ereignismarker: kleine Plakette mit dem wichtigsten Symbol, bei mehreren ein „+n“; dezentes Pulsieren.
    fn draw_marker_badge(painter: &Painter, r: Rect, marks: &[Marker], t: f32, z: f32, id: ProvId) {
        let m = marks[0];
        let ph = (t * 2.2 + id as f32 * 0.9).sin() * 0.5 + 0.5;
        let tint = m.tint();
        let glow = r.expand(2.0 + ph * 2.5);
        painter.rect_filled(glow, 5.0, tint.gamma_multiply(0.10 + 0.14 * ph));
        painter.rect_filled(r, 4.0, Color32::from_rgba_unmultiplied(20, 14, 12, 225));
        painter.rect_stroke(
            r,
            4.0,
            Stroke::new(1.0, tint.gamma_multiply(0.65 + 0.35 * ph)),
            egui::StrokeKind::Inside,
        );
        let px = if z >= 2.4 { 3.0 } else { 2.0 };
        m.draw(painter, r.center() - EVec2::splat(4.0 * px), px, 0.82 + 0.18 * ph);
        if marks.len() > 1 {
            Self::plaque(
                painter,
                r.right_bottom() + EVec2::new(-1.0, 1.0),
                &format!("+{}", marks.len() - 1),
                9.0,
                tint.gamma_multiply(0.8),
                Color32::from_rgb(240, 228, 196),
            );
        }
    }

    /// Legende des aktuellen Modus unten links: Farbtabelle, optional Zeichen (Grenzen, Ereignissymbole).
    pub fn draw_legend(&mut self, ui: &mut egui::Ui, rect: Rect, mode: MapMode, world: &StrategyWorld, art: &UiArt) {
        use ashen_engine::ui::theme;
        let painter = ui.painter_at(rect);
        let items = legend(mode, world);
        let cols = if items.len() > 7 { 2 } else { 1 };
        let rows = items.len().div_ceil(cols);
        let col_w = if cols == 2 { 210.0 } else { 190.0 };
        let marker_rows = Marker::ALL.len().div_ceil(2);
        let signs_h = if self.legend_signs { 26.0 + 18.0 + marker_rows as f32 * 19.0 } else { 0.0 };
        let w = (cols as f32 * col_w + 20.0).max(if self.legend_signs { 310.0 } else { 0.0 });
        let h = 42.0 + rows as f32 * 18.0 + signs_h;
        let r = Rect::from_min_size(Pos2::new(rect.min.x + 20.0, rect.max.y - h - 20.0), EVec2::new(w, h));
        for s in art.panel_shapes(ui_art::PanelKind::Thin, r, false) {
            painter.add(s);
        }
        painter.text(r.min + EVec2::new(16.0, 10.0), egui::Align2::LEFT_TOP, mode.label(), theme::caps_font(12.5), theme::GOLD);
        for (i, (c, name)) in items.iter().enumerate() {
            let (col, row) = (i / rows, i % rows);
            let x = r.min.x + 16.0 + col as f32 * col_w;
            let y = r.min.y + 32.0 + row as f32 * 18.0;
            let sw = Rect::from_min_size(Pos2::new(x, y), EVec2::new(14.0, 12.0));
            painter.rect_filled(sw, 0.0, *c);
            painter.rect_stroke(sw, 0.0, Stroke::new(1.0, Color32::BLACK), egui::StrokeKind::Outside);
            painter.text(Pos2::new(x + 22.0, y - 1.0), egui::Align2::LEFT_TOP, name, theme::body_font(12.5), theme::PARCHMENT);
        }
        // Umschalter für den Zeichenblock.
        let toggle = Rect::from_min_size(Pos2::new(r.max.x - 140.0, r.min.y + 6.0), EVec2::new(126.0, 20.0));
        let resp = ui.interact(toggle, egui::Id::unique("wm_legend_signs"), egui::Sense::click());
        if resp.clicked() {
            self.legend_signs = !self.legend_signs;
        }
        painter.text(
            toggle.right_center(),
            egui::Align2::RIGHT_CENTER,
            if self.legend_signs { "Zeichen ausblenden" } else { "Zeichen anzeigen" },
            theme::body_font(12.0),
            if resp.hovered() { theme::GOLD_BRIGHT } else { theme::GOLD_DIM },
        );
        if self.legend_signs {
            let y0 = r.min.y + 32.0 + rows as f32 * 18.0 + 6.0;
            painter.line_segment(
                [Pos2::new(r.min.x + 14.0, y0 - 3.0), Pos2::new(r.max.x - 14.0, y0 - 3.0)],
                Stroke::new(1.0, theme::GOLD_DIM.gamma_multiply(0.6)),
            );
            let half = (w - 20.0) / 2.0;
            for (i, (label, wd, col)) in [
                ("Reichsgrenze", 3.5, Color32::from_rgb(240, 200, 120)),
                ("Provinzgrenze", 1.0, Color32::from_rgb(160, 134, 100)),
            ]
            .iter()
            .enumerate()
            {
                let x = r.min.x + 16.0 + i as f32 * half;
                painter.line_segment(
                    [Pos2::new(x, y0 + 8.0), Pos2::new(x + 22.0, y0 + 8.0)],
                    Stroke::new(*wd, *col),
                );
                painter.text(Pos2::new(x + 30.0, y0), egui::Align2::LEFT_TOP, label, theme::body_font(12.5), theme::PARCHMENT);
            }
            for (i, m) in Marker::ALL.iter().enumerate() {
                let (col, row) = (i % 2, i / 2);
                let x = r.min.x + 16.0 + col as f32 * half;
                let y = y0 + 22.0 + row as f32 * 19.0;
                let chip = Rect::from_min_size(Pos2::new(x, y), EVec2::splat(17.0));
                painter.rect_filled(chip, 3.0, Color32::from_rgba_unmultiplied(20, 14, 12, 225));
                painter.rect_stroke(chip, 3.0, Stroke::new(1.0, m.tint().gamma_multiply(0.7)), egui::StrokeKind::Inside);
                m.draw(&painter, chip.center() - EVec2::splat(6.0), 1.5, 1.0);
                painter.text(Pos2::new(x + 24.0, y + 1.0), egui::Align2::LEFT_TOP, m.name(), theme::body_font(12.0), theme::PARCHMENT);
            }
        }
    }
}

/// Aktive Ereignisse einer Provinz in Anzeigepriorität (das erste wird als Marker gezeigt).
pub fn province_markers(world: &StrategyWorld, p: &ashen_strategy::Province) -> Vec<Marker> {
    let mut marks = Vec::new();
    if world.battles.iter().any(|b| b.province == p.id) {
        marks.push(Marker::Swords);
    }
    if p.siege.is_some() {
        marks.push(Marker::Flame);
    }
    if world
        .bosses
        .iter()
        .any(|b| b.province == p.id && b.state == ashen_strategy::BossState::Active)
    {
        marks.push(Marker::Skull);
    }
    if p.unrest >= 60.0 || p.rebel_held {
        marks.push(Marker::Fist);
    }
    if world.neutrals.iter().any(|n| n.active && n.province == p.id) {
        marks.push(Marker::Paw);
    }
    if p.shrine > 0 || p.god_shrine.is_some() && p.veil >= 3.0 {
        marks.push(Marker::Ring);
    }
    if p.plague.is_some() {
        marks.push(Marker::Drop);
    }
    if p.food_ratio < 0.7 && p.population() > 50.0 {
        marks.push(Marker::Bowl);
    }
    marks
}

/// 8×8-Pixelsymbole für Ereignismarker (0 = Loch, 1 = Schatten, 2 = Grundton, 3 = Licht).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    Swords,
    Skull,
    Flame,
    Drop,
    Ring,
    Paw,
    Fist,
    Bowl,
}

impl Marker {
    pub const ALL: [Marker; 8] = [
        Marker::Swords,
        Marker::Flame,
        Marker::Skull,
        Marker::Fist,
        Marker::Paw,
        Marker::Ring,
        Marker::Drop,
        Marker::Bowl,
    ];

    fn rows(self) -> [&'static str; 8] {
        match self {
            Marker::Swords => ["3......3", ".32..23.", "..3223..", "...33...", "..2112..", ".21..12.", "11....11", "........"],
            Marker::Skull => ["..2222..", ".233222.", ".200200.", ".222222.", "..2222..", "..2222..", "..2121..", "........"],
            Marker::Flame => ["...2....", "..22....", "..232.2.", ".23322..", ".233322.", ".233332.", "..2332..", "...22..."],
            Marker::Drop => ["...22...", "...22...", "..2332..", ".233322.", ".233222.", ".222212.", "..2112..", "...11..."],
            Marker::Ring => ["..2222..", ".23..32.", "23....12", "2..22..2", "2..22..2", "21....12", ".21..12.", "..1111.."],
            Marker::Paw => [".2....2.", "222..222", ".2.22.2.", "..2332..", ".233332.", ".233332.", ".22..22.", "........"],
            Marker::Fist => ["...33...", "..2332..", "..2222..", "..2222..", "...22...", "........", "...22...", "..2222.."],
            Marker::Bowl => ["........", "........", "3......3", "23....32", "22222222", ".223322.", "..2222..", "...11..."],
        }
    }

    pub fn tint(self) -> Color32 {
        match self {
            Marker::Swords => Color32::from_rgb(255, 128, 76),
            Marker::Skull => Color32::from_rgb(236, 228, 208),
            Marker::Flame => Color32::from_rgb(255, 164, 64),
            Marker::Drop => Color32::from_rgb(150, 220, 90),
            Marker::Ring => Color32::from_rgb(120, 236, 170),
            Marker::Paw => Color32::from_rgb(226, 96, 76),
            Marker::Fist => Color32::from_rgb(232, 86, 66),
            Marker::Bowl => Color32::from_rgb(222, 184, 100),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Marker::Swords => "Schlacht",
            Marker::Skull => "Boss",
            Marker::Flame => "Belagerung",
            Marker::Drop => "Seuche",
            Marker::Ring => "Schleierriss / Hohlschrein",
            Marker::Paw => "Monster oder Banditen",
            Marker::Fist => "Unruhen / Aufstand",
            Marker::Bowl => "Hungersnot",
        }
    }

    /// Zeichnet das Symbol pixelgenau (jedes Pixel ein Quadrat der Kantenlänge `px`) ab `min`; `k` dimmt (0–1).
    pub fn draw(self, painter: &Painter, min: Pos2, px: f32, k: f32) {
        let tint = self.tint();
        let t = |m: f32| {
            let c = |v: u8| (v as f32 * m).min(255.0) as u8;
            Color32::from_rgb(c(tint.r()), c(tint.g()), c(tint.b())).gamma_multiply(k)
        };
        let light = Color32::from_rgb(
            ((tint.r() as u16 + 255) / 2) as u8,
            ((tint.g() as u16 + 255) / 2) as u8,
            ((tint.b() as u16 + 255) / 2) as u8,
        )
        .gamma_multiply(k);
        for (y, row) in self.rows().iter().enumerate() {
            for (x, ch) in row.bytes().enumerate() {
                let c = match ch {
                    b'1' => t(0.55),
                    b'2' => t(1.0),
                    b'3' => light,
                    _ => continue,
                };
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(min.x + x as f32 * px, min.y + y as f32 * px), EVec2::splat(px)),
                    0.0,
                    c,
                );
            }
        }
    }
}
