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

/// Legende eines Kartenmodus.
pub fn legend(mode: MapMode) -> Vec<(Color32, &'static str)> {
    match mode {
        MapMode::Realms => vec![
            (rgb([196, 56, 40]), "Reichsfarbe = Besitzer"),
            (rgb([140, 140, 130]), "Herrenlos"),
        ],
        MapMode::Terrain => vec![
            (to_color(biome_rgb(Biome::Plains)), "Ebene"),
            (to_color(biome_rgb(Biome::Forest)), "Wald"),
            (to_color(biome_rgb(Biome::Mountain)), "Gebirge"),
            (to_color(biome_rgb(Biome::Desert)), "Wüste"),
            (to_color(biome_rgb(Biome::Swamp)), "Sumpf"),
        ],
        MapMode::Religion => RELIGIONS
            .iter()
            .map(|r| (to_color(religion_rgb(*r)), r.name()))
            .collect(),
        MapMode::Hollowing => vec![
            (rgb([90, 110, 90]), "rein"),
            (rgb([90, 230, 140]), "stark gehohlt"),
            (rgb([176, 96, 230]), "Schleier ≥ 4"),
        ],
        MapMode::Economy => vec![
            (rgb([70, 52, 40]), "arm"),
            (rgb([196, 150, 60]), "wohlhabend"),
            (rgb([250, 228, 130]), "reich"),
        ],
        MapMode::Unrest => vec![
            (rgb([70, 140, 80]), "ruhig"),
            (rgb([220, 190, 60]), "gespannt"),
            (rgb([210, 50, 40]), "Aufstand"),
        ],
        MapMode::Diplomacy => vec![
            (rgb([240, 200, 70]), "eigenes Reich/Vasallen"),
            (rgb([80, 130, 230]), "Verbündete"),
            (rgb([210, 50, 40]), "Krieg"),
            (rgb([90, 190, 100]), "freundlich"),
            (rgb([150, 150, 150]), "neutral"),
            (rgb([190, 110, 60]), "feindlich"),
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
}

pub struct MapView {
    pub cam: Vec2,
    pub zoom: f32,
    geo: Option<Geometry>,
    tex: Option<TextureHandle>,
    tex_key: Option<(MapMode, u64)>,
    army_hits: Vec<(Rect, ArmyId)>,
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
                    if d <= 1.0 {
                        c = mix(c, [210.0, 226.0, 220.0], 0.55);
                    } else if d <= 2.0 {
                        c = mix(c, [170.0, 206.0, 214.0], 0.25);
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
                        let rc = holder_rgb[id as usize].unwrap_or([120.0, 120.0, 112.0]);
                        c = mix(mix(c, [14.0, 10.0, 8.0], 0.45), rc, 0.75);
                    } else {
                        c = mix(c, [20.0, 14.0, 10.0], 0.30);
                    }
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
        // Randverdunkelung
        for (c, s) in [
            (
                rect.left_center(),
                EVec2::new(rect.width() * 0.35, rect.height() * 1.5),
            ),
            (
                rect.right_center(),
                EVec2::new(rect.width() * 0.35, rect.height() * 1.5),
            ),
            (
                rect.center_top(),
                EVec2::new(rect.width() * 1.5, rect.height() * 0.3),
            ),
            (
                rect.center_bottom(),
                EVec2::new(rect.width() * 1.5, rect.height() * 0.3),
            ),
        ] {
            cx.ui_art
                .soft_shadow(&painter, Rect::from_center_size(c, s), 0.45);
        }

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
        // Siedlungen, Namen, Marker.
        for p in &world.provinces {
            let c = self.to_screen(rect, p.centroid);
            if !rect.expand(60.0).contains(c) {
                continue;
            }
            self.draw_settlement(painter, c, p, world, cx, mode);
        }
        // Heere.
        let mut at: std::collections::BTreeMap<ProvId, Vec<&ashen_strategy::Army>> =
            std::collections::BTreeMap::new();
        for a in &world.armies {
            if a.path.is_empty() || a.progress <= 0.0 {
                at.entry(a.location).or_default().push(a);
            }
        }
        for (loc, list) in &at {
            let base = self.to_screen(rect, Self::centroid(world, *loc))
                + EVec2::new(0.0, 22.0 * z.clamp(0.8, 2.0));
            let n = list.len() as f32;
            for (i, a) in list.iter().enumerate() {
                let off = EVec2::new((i as f32 - (n - 1.0) / 2.0) * 26.0 * z.clamp(0.8, 1.6), 0.0);
                self.draw_army(painter, base + off, a, world, cx, rect);
            }
        }
        for a in world
            .armies
            .iter()
            .filter(|a| !a.path.is_empty() && a.progress > 0.0)
        {
            let pos = self.to_screen(rect, self.army_pos(world, a));
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

    fn draw_settlement(
        &self,
        painter: &Painter,
        c: Pos2,
        p: &ashen_strategy::Province,
        world: &StrategyWorld,
        cx: &DrawCtx,
        mode: MapMode,
    ) {
        let z = self.zoom;
        let size = (26.0 + 8.0 * p.settlement.rank() as f32) * z.clamp(0.7, 1.7);
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
        // Name (ab mittlerer Zoomstufe).
        if z >= 0.85 {
            let f = ashen_engine::ui::theme::caps_font((11.0 * z.clamp(0.9, 1.5)).round());
            let pos = c + EVec2::new(0.0, size * 0.52);
            painter.text(
                pos + EVec2::new(1.0, 1.0),
                egui::Align2::CENTER_TOP,
                &p.name,
                f.clone(),
                Color32::from_black_alpha(230),
            );
            painter.text(
                pos,
                egui::Align2::CENTER_TOP,
                &p.name,
                f,
                Color32::from_rgb(240, 228, 196),
            );
        }
        // Marker über der Siedlung.
        let mut marks: Vec<(Marker, Color32)> = Vec::new();
        if world.battles.iter().any(|b| b.province == p.id) {
            marks.push((Marker::Swords, Color32::from_rgb(255, 120, 70)));
        }
        if p.siege.is_some() {
            marks.push((Marker::Flame, Color32::from_rgb(255, 160, 60)));
        }
        if p.plague.is_some() {
            marks.push((Marker::Drop, Color32::from_rgb(150, 220, 90)));
        }
        if p.shrine > 0 || p.god_shrine.is_some() && p.veil >= 3.0 {
            marks.push((Marker::Ring, Color32::from_rgb(140, 250, 170)));
        }
        if world
            .neutrals
            .iter()
            .any(|n| n.active && n.province == p.id)
        {
            marks.push((Marker::Paw, Color32::from_rgb(230, 90, 70)));
        }
        if world
            .bosses
            .iter()
            .any(|b| b.province == p.id && b.state == ashen_strategy::BossState::Active)
        {
            marks.push((Marker::Skull, Color32::from_rgb(250, 240, 220)));
        }
        if p.unrest >= 60.0 || p.rebel_held {
            marks.push((Marker::Fist, Color32::from_rgb(235, 80, 60)));
        }
        if p.food_ratio < 0.7 && p.population() > 50.0 {
            marks.push((Marker::Bowl, Color32::from_rgb(220, 180, 90)));
        }
        let ms = (3.0 * z.clamp(0.9, 1.6)).max(2.0);
        let total = marks.len() as f32 * (8.0 * ms + 3.0);
        for (i, (m, col)) in marks.iter().enumerate() {
            let x = c.x - total / 2.0 + i as f32 * (8.0 * ms + 3.0);
            let y = c.y - size * 0.62 - 8.0 * ms;
            let pulse = if matches!(m, Marker::Swords | Marker::Flame) {
                0.75 + 0.25 * (cx.t * 6.0).sin()
            } else {
                1.0
            };
            m.draw(painter, Pos2::new(x, y), ms, col.gamma_multiply(pulse));
        }
        if mode == MapMode::Economy || mode == MapMode::Unrest || mode == MapMode::Hollowing {
            let v = match mode {
                MapMode::Economy => format!("{:.0}", prosperity(world, p.id)),
                MapMode::Unrest => format!("{:.0}", p.unrest),
                _ => format!("{:.0}", p.hollowing),
            };
            let f = ashen_engine::ui::theme::body_bold_font(12.0 * z.clamp(0.9, 1.4));
            let pos = c + EVec2::new(0.0, size * 0.52 + 14.0 * z.clamp(0.9, 1.4));
            painter.text(
                pos + EVec2::new(1.0, 1.0),
                egui::Align2::CENTER_TOP,
                &v,
                f.clone(),
                Color32::BLACK,
            );
            painter.text(
                pos,
                egui::Align2::CENTER_TOP,
                &v,
                f,
                Color32::from_rgb(255, 244, 210),
            );
        }
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
        // Stärke
        let txt = format!("{}", a.size());
        let f = ashen_engine::ui::theme::body_bold_font(11.5 * z);
        let tp = pos + EVec2::new(0.0, size * 0.22);
        painter.text(
            tp + EVec2::new(1.0, 1.0),
            egui::Align2::CENTER_TOP,
            &txt,
            f.clone(),
            Color32::BLACK,
        );
        painter.text(
            tp,
            egui::Align2::CENTER_TOP,
            &txt,
            f,
            if mine {
                Color32::from_rgb(255, 232, 160)
            } else {
                Color32::from_rgb(240, 220, 210)
            },
        );
        if a.in_battle.is_some() {
            Marker::Swords.draw(
                painter,
                r.center_top() + EVec2::new(-12.0, -22.0),
                3.0,
                Color32::from_rgb(255, 130, 80),
            );
        } else if a.order == ArmyOrder::Siege {
            Marker::Flame.draw(
                painter,
                r.center_top() + EVec2::new(-12.0, -22.0),
                3.0,
                Color32::from_rgb(255, 170, 70),
            );
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

    /// Legende des aktuellen Modus unten links.
    pub fn draw_legend(painter: &Painter, rect: Rect, mode: MapMode, art: &UiArt) {
        let items = legend(mode);
        let h = 22.0 + items.len() as f32 * 18.0;
        let r = Rect::from_min_size(
            Pos2::new(rect.min.x + 14.0, rect.max.y - h - 14.0),
            EVec2::new(210.0, h),
        );
        for s in art.panel_shapes(ui_art::PanelKind::Thin, r, false) {
            painter.add(s);
        }
        painter.text(
            r.min + EVec2::new(16.0, 10.0),
            egui::Align2::LEFT_TOP,
            mode.label(),
            ashen_engine::ui::theme::caps_font(12.5),
            ashen_engine::ui::theme::GOLD,
        );
        for (i, (c, name)) in items.iter().enumerate() {
            let y = r.min.y + 30.0 + i as f32 * 18.0;
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(r.min.x + 16.0, y), EVec2::new(14.0, 12.0)),
                0.0,
                *c,
            );
            painter.rect_stroke(
                Rect::from_min_size(Pos2::new(r.min.x + 16.0, y), EVec2::new(14.0, 12.0)),
                0.0,
                Stroke::new(1.0, Color32::BLACK),
                egui::StrokeKind::Outside,
            );
            painter.text(
                Pos2::new(r.min.x + 38.0, y - 1.0),
                egui::Align2::LEFT_TOP,
                *name,
                ashen_engine::ui::theme::body_font(13.0),
                ashen_engine::ui::theme::PARCHMENT,
            );
        }
    }
}

/// 8×8-Pixelsymbole für Ereignismarker.
#[derive(Clone, Copy)]
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
    fn bits(self) -> [u8; 8] {
        match self {
            Marker::Swords => [
                0b1000_0001,
                0b1100_0011,
                0b0110_0110,
                0b0011_1100,
                0b0011_1100,
                0b0110_0110,
                0b1100_0011,
                0b1000_0001,
            ],
            Marker::Skull => [
                0b0011_1100,
                0b0111_1110,
                0b1101_1011,
                0b1101_1011,
                0b1111_1111,
                0b0011_1100,
                0b0010_0100,
                0b0010_0100,
            ],
            Marker::Flame => [
                0b0001_0000,
                0b0001_1000,
                0b0011_1000,
                0b0111_1100,
                0b0111_1110,
                0b1111_1110,
                0b0111_1100,
                0b0011_1000,
            ],
            Marker::Drop => [
                0b0001_1000,
                0b0001_1000,
                0b0011_1100,
                0b0111_1110,
                0b1111_1111,
                0b1111_1111,
                0b0111_1110,
                0b0011_1100,
            ],
            Marker::Ring => [
                0b0011_1100,
                0b0100_0010,
                0b1001_1001,
                0b1010_0101,
                0b1010_0101,
                0b1001_1001,
                0b0100_0010,
                0b0011_1100,
            ],
            Marker::Paw => [
                0b0100_0010,
                0b1110_0111,
                0b0100_0010,
                0b0001_1000,
                0b0011_1100,
                0b0111_1110,
                0b0111_1110,
                0b0011_1100,
            ],
            Marker::Fist => [
                0b0110_1100,
                0b1111_1110,
                0b1111_1111,
                0b1111_1111,
                0b1111_1111,
                0b0111_1110,
                0b0011_1100,
                0b0001_1000,
            ],
            Marker::Bowl => [
                0,
                0,
                0,
                0b1111_1111,
                0b1111_1111,
                0b0111_1110,
                0b0011_1100,
                0,
            ],
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

    /// Zeichnet das Symbol pixelgenau (jedes Pixel ein Quadrat der Kantenlänge `px`), mit dunklem Schlagschatten.
    pub fn draw(self, painter: &Painter, min: Pos2, px: f32, col: Color32) {
        let bits = self.bits();
        let px = px.round().max(2.0);
        for pass in 0..2 {
            let (off, c) = if pass == 0 {
                (px * 0.6, Color32::from_black_alpha(210))
            } else {
                (0.0, col)
            };
            for (y, row) in bits.iter().enumerate() {
                for x in 0..8 {
                    if row & (0x80 >> x) != 0 {
                        painter.rect_filled(
                            Rect::from_min_size(
                                Pos2::new(min.x + x as f32 * px + off, min.y + y as f32 * px + off),
                                EVec2::splat(px),
                            ),
                            0.0,
                            c,
                        );
                    }
                }
            }
        }
    }
}
