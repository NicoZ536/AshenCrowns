//! Weltgenerierung: die acht Kanon-Regionen sind handgefertigt, Provinzzahl (50–64), Lage,
//! Zuschnitt (Voronoi-Polygone), Ressourcenverteilung und Schleierverteilung variieren je Seed.

use crate::goods::Good;
use crate::model::*;
use crate::types::*;
use glam::Vec2;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub const MAP_W: f32 = 1000.0;
pub const MAP_H: f32 = 700.0;

/// (id, Name, Marktgebiet)
pub const REGION_DEFS: [(&str, &str, Option<usize>); 8] = [
    ("valemark", "Valemark", Some(0)),
    ("ashmarch", "Die Aschmark", Some(1)),
    ("skarnheim", "Skarnheim", Some(2)),
    ("thornwild", "Der Dornwald", Some(3)),
    ("saelith", "Die Saelith-Inseln", Some(4)),
    ("aureth_wastes", "Die Aurethwüste", Some(5)),
    ("veilscar", "Die Schleiernarbe", None),
    ("cinderreach", "Glutweite", Some(6)),
];

pub const MARKET_COUNT: usize = 7;

struct Pt {
    key: &'static str,
    name: &'static str,
    region: u8,
    x: f32,
    y: f32,
    biome: Biome,
    /// 0 Dorf, 1 Stadt, 2 Großstadt, 3 Hauptstadt
    settle: u8,
    dep: &'static [(&'static str, u8)],
    optional: bool,
    coastal: bool,
}

#[allow(clippy::too_many_arguments)]
const fn p(key: &'static str, name: &'static str, region: u8, x: f32, y: f32, biome: Biome, settle: u8, dep: &'static [(&'static str, u8)], optional: bool, coastal: bool) -> Pt {
    Pt { key, name, region, x, y, biome, settle, dep, optional, coastal }
}

use Biome::*;

const PROVINCES: &[Pt] = &[
    // Valemark (0)
    p("arnvale", "Arnvale", 0, 480.0, 210.0, Plains, 3, &[("grain", 4), ("wood", 2), ("clay", 3), ("stone", 2), ("livestock", 3), ("flax", 2)], false, false),
    p("kaldris", "Kaldris", 0, 570.0, 200.0, Hills, 2, &[("grain", 3), ("grapes", 3), ("herbs", 2), ("wax", 3), ("stone", 2)], false, false),
    p("vellmund", "Vellmund", 0, 520.0, 272.0, Plains, 2, &[("grain", 5), ("fish", 4), ("clay", 2), ("hops", 2)], false, false),
    p("koenigsfurt", "Königsfurt", 0, 470.0, 305.0, Plains, 1, &[("grain", 4), ("fish", 2), ("livestock", 3)], false, false),
    p("corvanshold", "Corvanshold", 0, 420.0, 150.0, Hills, 1, &[("iron_ore", 2), ("stone", 3), ("wood", 2), ("game", 2), ("grain", 1)], false, false),
    p("rabental", "Rabental", 0, 360.0, 175.0, Forest, 0, &[("wood", 4), ("game", 3), ("herbs", 2), ("wool", 3)], false, false),
    p("ilverwacht", "Ilverwacht", 0, 330.0, 235.0, Forest, 1, &[("wood", 5), ("game", 3), ("herbs", 2), ("wax", 3)], false, false),
    p("strathhaven", "Strathhaven", 0, 640.0, 285.0, Coast, 2, &[("fish", 5), ("salt", 3), ("grain", 2), ("hops", 1)], false, true),
    p("sonnfall", "Sonnfall", 0, 410.0, 310.0, Hills, 1, &[("grapes", 5), ("grain", 3), ("livestock", 2), ("stone", 1)], false, false),
    p("melbrechts_auen", "Melbrechts Auen", 0, 550.0, 245.0, Plains, 1, &[("grain", 5), ("hops", 3), ("flax", 3), ("livestock", 2)], false, false),
    p("eschenhain", "Eschenhain", 0, 380.0, 255.0, Forest, 0, &[("wood", 4), ("hops", 3), ("wool", 2), ("game", 2)], false, false),
    p("weissfurt", "Weißfurt", 0, 605.0, 238.0, Plains, 0, &[("grain", 3), ("flax", 3), ("fish", 2), ("wool", 2)], false, false),
    p("harkenmoor", "Harkenmoor", 0, 440.0, 262.0, Swamp, 0, &[("clay", 3), ("herbs", 3), ("game", 2), ("fish", 2)], false, false),
    p("rotsandfeste", "Rotsandfeste", 0, 520.0, 150.0, Hills, 0, &[("stone", 4), ("iron_ore", 1), ("wool", 3)], true, false),
    p("falkenmark", "Falkenmark", 0, 610.0, 160.0, Plains, 0, &[("grain", 3), ("livestock", 4), ("wool", 2)], true, false),
    // Aschmark (1)
    p("brennhollow", "Brennhollow", 1, 470.0, 375.0, Hills, 0, &[("wood", 4), ("stone", 3), ("game", 2), ("fish", 1), ("grain", 1)], false, false),
    p("grauwall", "Grauwall", 1, 380.0, 385.0, Hills, 0, &[("stone", 4), ("iron_ore", 1), ("wood", 2)], false, false),
    p("vael_dorn", "Ruinen von Vael Dorn", 1, 545.0, 395.0, Ruins, 0, &[("ashglass_raw", 2), ("stone", 3), ("wood", 2)], false, false),
    p("kaltherd", "Kaltherd", 1, 480.0, 340.0, Plains, 0, &[("wood", 2), ("grain", 2), ("livestock", 2)], false, false),
    p("kraehenmoore", "Die Krähenmoore", 1, 420.0, 415.0, Swamp, 0, &[("clay", 3), ("herbs", 3), ("game", 2)], false, false),
    p("letzter_meilenstein", "Der Letzte Meilenstein", 1, 520.0, 428.0, Ruins, 0, &[("stone", 2), ("salt", 1)], true, false),
    // Skarnheim (2)
    p("kharnfels", "Kharnfels", 2, 480.0, 65.0, Mountain, 2, &[("iron_ore", 5), ("stone", 5), ("coal", 3), ("game", 2)], false, false),
    p("eisenkehle", "Die Eisenkehle", 2, 390.0, 55.0, Mountain, 1, &[("iron_ore", 5), ("coal", 4), ("silver_ore", 2), ("stone", 3)], false, false),
    p("grauhelm", "Ahnenhöhen von Grauhelm", 2, 570.0, 50.0, Mountain, 0, &[("stone", 4), ("game", 3), ("wool", 3), ("ashglass_raw", 1)], false, false),
    p("waechtertor", "Das Wächtertor", 2, 450.0, 115.0, Mountain, 0, &[("stone", 3), ("iron_ore", 2), ("game", 2)], false, false),
    p("wolfspass", "Der Wolfspass", 2, 370.0, 108.0, Hills, 0, &[("game", 4), ("wood", 3), ("iron_ore", 2)], false, false),
    p("vhalmirsee", "Der Vhalmirsee", 2, 540.0, 105.0, Tundra, 0, &[("fish", 4), ("game", 3), ("salt", 1), ("wool", 2)], false, false),
    p("frostgrat", "Frostgrat", 2, 315.0, 45.0, Tundra, 0, &[("game", 3), ("wool", 2), ("coal", 2), ("stone", 2)], true, false),
    p("nordhafen", "Nordhafen", 2, 640.0, 95.0, Tundra, 0, &[("fish", 5), ("salt", 2), ("game", 2)], true, false),
    // Dornwald (3)
    p("herzhain", "Der Herzhain", 3, 150.0, 285.0, Forest, 1, &[("wood", 5), ("herbs", 5), ("game", 3), ("wax", 2)], false, false),
    p("ilvaen", "Mottenhain von Ilvaen", 3, 110.0, 200.0, Forest, 2, &[("wood", 4), ("herbs", 4), ("wax", 4), ("flax", 2)], false, false),
    p("calyxruh", "Kreis von Calyxruh", 3, 200.0, 225.0, Forest, 0, &[("herbs", 4), ("wood", 4), ("game", 3)], false, false),
    p("bernsteinsuempfe", "Die Bernsteinsümpfe", 3, 170.0, 390.0, Swamp, 0, &[("herbs", 4), ("clay", 3), ("wax", 3), ("fish", 2)], false, false),
    p("brandsaum", "Der Brandsaum", 3, 255.0, 310.0, Ruins, 0, &[("wood", 2), ("clay", 2), ("coal", 1), ("game", 1)], false, false),
    p("dornpforte", "Die Dornpforte", 3, 250.0, 262.0, Forest, 0, &[("wood", 4), ("game", 3), ("herbs", 3)], false, false),
    p("mottenwacht", "Mottenwacht", 3, 80.0, 330.0, Forest, 0, &[("wood", 4), ("wax", 3), ("herbs", 3)], true, false),
    p("wurzelmark", "Wurzelmark", 3, 65.0, 130.0, Forest, 0, &[("wood", 5), ("game", 3), ("herbs", 2)], true, false),
    // Saelith (4)
    p("selvaris", "Selvaris", 4, 880.0, 200.0, Island, 3, &[("fish", 4), ("salt", 4), ("spices", 1), ("silk", 1)], false, true),
    p("thalmoor", "Thalmoor", 4, 810.0, 135.0, Island, 1, &[("fish", 4), ("salt", 3), ("herbs", 1)], false, true),
    p("isk", "Gewürzterrassen von Isk", 4, 930.0, 262.0, Island, 1, &[("spices", 5), ("grapes", 3), ("silk", 1), ("fish", 2)], false, true),
    p("varo", "Silberwerft von Varo", 4, 860.0, 322.0, Island, 1, &[("fish", 3), ("wood", 2), ("silver_ore", 2), ("salt", 1)], false, true),
    p("schwarzes_riff", "Das Schwarze Riff", 4, 960.0, 365.0, Island, 0, &[("fish", 3), ("salt", 2)], false, true),
    p("thals_auge", "Thals Auge", 4, 790.0, 232.0, Island, 0, &[("fish", 3), ("salt", 2), ("grapes", 1)], false, true),
    p("merenhal", "Merenhal-Salzinseln", 4, 940.0, 140.0, Island, 0, &[("salt", 5), ("silk", 3), ("fish", 2)], true, true),
    p("corallo", "Corallo-Perleninseln", 4, 790.0, 340.0, Island, 0, &[("fish", 4), ("spices", 2), ("salt", 1)], true, true),
    // Aurethwüste (5)
    p("solmere", "Solmere", 5, 460.0, 525.0, Ruins, 0, &[("ashglass_raw", 4), ("stone", 4)], false, false),
    p("merrat", "Oase Merrat", 5, 380.0, 480.0, Desert, 0, &[("grain", 2), ("livestock", 2), ("salt", 2), ("herbs", 1)], false, false),
    p("glasfelder_von_ilum", "Die Glasfelder von Ilum", 5, 545.0, 478.0, Ruins, 0, &[("ashglass_raw", 5), ("stone", 2)], false, false),
    p("gebrochene_saeule", "Die Gebrochene Säule", 5, 430.0, 455.0, Desert, 0, &[("stone", 3), ("ashglass_raw", 1)], false, false),
    p("kesh_auran", "Ruinen von Kesh-Auran", 5, 330.0, 548.0, Desert, 0, &[("stone", 3), ("clay", 2), ("ashglass_raw", 2)], false, false),
    p("sonnenstrasse_posten", "Posten an der Sonnenstraße", 5, 480.0, 450.0, Desert, 0, &[("stone", 2), ("salt", 1), ("livestock", 1)], false, false),
    p("nekropole_der_sonnenkoenige", "Nekropole der Sonnenkönige", 5, 590.0, 545.0, Ruins, 0, &[("stone", 3), ("ashglass_raw", 2)], true, false),
    p("duenenmeer", "Das Dünenmeer", 5, 605.0, 490.0, Desert, 0, &[("salt", 3), ("stone", 2)], true, false),
    // Schleiernarbe (6)
    p("hohles_auge", "Das Hohle Auge", 6, 460.0, 640.0, Hollowland, 0, &[("ashglass_raw", 5)], false, false),
    p("ner_moraun", "Ner Moraun", 6, 360.0, 625.0, Hollowland, 0, &[("ashglass_raw", 3), ("stone", 2)], false, false),
    p("vael_ithar", "Vael Ithar", 6, 555.0, 630.0, Hollowland, 0, &[("ashglass_raw", 4)], false, false),
    p("schwebende_steine", "Felder der Schwebenden Steine", 6, 305.0, 668.0, Hollowland, 0, &[("stone", 4), ("ashglass_raw", 2)], true, false),
    p("stillwasser", "Stillwasser", 6, 610.0, 672.0, Hollowland, 0, &[("fish", 1), ("ashglass_raw", 2)], true, false),
    // Glutweite (7)
    p("der_kessel", "Der Kessel", 7, 790.0, 570.0, Volcanic, 0, &[("sulfur", 5), ("coal", 3), ("stone", 3)], false, false),
    p("khar_ossun", "Khar Ossun", 7, 725.0, 520.0, Volcanic, 1, &[("sulfur", 4), ("iron_ore", 3), ("coal", 3)], false, false),
    p("emberth", "Ruinen von Emberth", 7, 880.0, 470.0, Coast, 0, &[("fish", 3), ("salt", 3), ("sulfur", 2)], false, true),
    p("obsidiankueste", "Die Obsidianküste", 7, 880.0, 605.0, Coast, 0, &[("stone", 4), ("sulfur", 3), ("fish", 2)], false, true),
    p("brennender_pass", "Der Brennende Pass", 7, 655.0, 500.0, Volcanic, 0, &[("sulfur", 2), ("stone", 3)], false, false),
    p("feuerquellen", "Die Feuerquellen", 7, 770.0, 455.0, Hills, 0, &[("herbs", 3), ("sulfur", 1), ("game", 1)], true, false),
];

/// Landverbindungen zwischen Regionen (nur über diese „Tore“).
const GATES: &[(&str, &str)] = &[
    ("koenigsfurt", "kaltherd"),
    ("sonnfall", "grauwall"),
    ("vellmund", "kaltherd"),
    ("corvanshold", "wolfspass"),
    ("corvanshold", "waechtertor"),
    ("rabental", "wolfspass"),
    ("rotsandfeste", "waechtertor"),
    ("ilverwacht", "dornpforte"),
    ("eschenhain", "dornpforte"),
    ("letzter_meilenstein", "sonnenstrasse_posten"),
    ("vael_dorn", "glasfelder_von_ilum"),
    ("brennhollow", "sonnenstrasse_posten"),
    ("kraehenmoore", "gebrochene_saeule"),
    ("solmere", "hohles_auge"),
    ("kesh_auran", "ner_moraun"),
    ("solmere", "ner_moraun"),
    ("duenenmeer", "brennender_pass"),
    ("nekropole_der_sonnenkoenige", "brennender_pass"),
    ("glasfelder_von_ilum", "brennender_pass"),
];

const SEA_LINKS: &[(&str, &str)] = &[
    ("strathhaven", "thals_auge"),
    ("strathhaven", "selvaris"),
    ("emberth", "varo"),
    ("emberth", "schwarzes_riff"),
    ("obsidiankueste", "schwarzes_riff"),
    ("strathhaven", "emberth"),
    ("nordhafen", "thalmoor"),
];

pub struct MapData {
    pub provinces: Vec<Province>,
    pub regions: Vec<Region>,
}

fn clip(poly: &[Vec2], n: Vec2, c: f32) -> Vec<Vec2> {
    let mut out = Vec::with_capacity(poly.len() + 2);
    for i in 0..poly.len() {
        let a = poly[i];
        let b = poly[(i + 1) % poly.len()];
        let da = a.dot(n) - c;
        let db = b.dot(n) - c;
        if da <= 0.0 {
            out.push(a);
        }
        if (da < 0.0 && db > 0.0) || (da > 0.0 && db < 0.0) {
            let t = da / (da - db);
            out.push(a + (b - a) * t);
        }
    }
    out
}

fn centroid(poly: &[Vec2], fallback: Vec2) -> Vec2 {
    let mut a = 0.0;
    let mut c = Vec2::ZERO;
    for i in 0..poly.len() {
        let p0 = poly[i];
        let p1 = poly[(i + 1) % poly.len()];
        let cr = p0.x * p1.y - p1.x * p0.y;
        a += cr;
        c += (p0 + p1) * cr;
    }
    if a.abs() < 1e-3 {
        return fallback;
    }
    c / (3.0 * a)
}

fn cell_radius(b: Biome) -> f32 {
    match b {
        Biome::Island => 46.0,
        Biome::Hollowland => 78.0,
        _ => 86.0,
    }
}

fn culture_of(region: u8, key: &str) -> Culture {
    match region {
        0 => Culture::Valemarker,
        1 => Culture::Aschmaerker,
        2 => Culture::Skarn,
        3 => {
            if matches!(key, "ilvaen" | "mottenwacht" | "calyxruh") {
                Culture::Veyl
            } else {
                Culture::Dornwaldler
            }
        }
        4 => Culture::Saelith,
        5 => {
            if key == "solmere" {
                Culture::Aurethisch
            } else {
                Culture::GraueNomaden
            }
        }
        6 => Culture::Hohl,
        _ => {
            if key == "emberth" {
                Culture::Saelith
            } else {
                Culture::Schlackenbrut
            }
        }
    }
}

fn base_veil(region: u8) -> f32 {
    match region {
        0 => 0.1,
        1 => 1.6,
        2 => 0.7,
        3 => 0.5,
        4 => 0.2,
        5 => 3.0,
        6 => 5.0,
        _ => 2.2,
    }
}

fn base_hollowing(region: u8) -> f32 {
    match region {
        1 => 6.0,
        5 => 35.0,
        6 => 92.0,
        7 => 8.0,
        _ => 1.0,
    }
}

fn fertility(b: Biome) -> f32 {
    match b {
        Biome::Plains => 0.95,
        Biome::Hills => 0.7,
        Biome::Forest => 0.6,
        Biome::Mountain => 0.3,
        Biome::Swamp => 0.5,
        Biome::Desert => 0.2,
        Biome::Coast => 0.75,
        Biome::Island => 0.55,
        Biome::Volcanic => 0.3,
        Biome::Tundra => 0.3,
        Biome::Ruins => 0.4,
        Biome::Hollowland => 0.1,
    }
}

/// Erzeugt die Welt für einen Seed. Provinzen: 50 Kernprovinzen plus 0–14 optionale.
pub fn generate(seed: u64) -> MapData {
    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ 0xA5A5_1234_9876_0F0F);
    // Auswahl
    let mut chosen: Vec<(&Pt, Vec2)> = Vec::new();
    for t in PROVINCES {
        if t.optional && rng.random::<f32>() > 0.55 {
            continue;
        }
        let j = Vec2::new(rng.random::<f32>() - 0.5, rng.random::<f32>() - 0.5) * 22.0;
        let pos = Vec2::new(t.x, t.y) + j;
        chosen.push((t, pos.clamp(Vec2::new(20.0, 20.0), Vec2::new(MAP_W - 20.0, MAP_H - 20.0))));
    }
    // Sortierung nach Region, stabil
    chosen.sort_by_key(|(t, _)| t.region);
    let n = chosen.len();
    let mut provinces: Vec<Province> = Vec::with_capacity(n);
    let mut regions: Vec<Region> = REGION_DEFS.iter().map(|(id, name, m)| Region { id: id.to_string(), name: name.to_string(), provinces: Vec::new(), market: *m }).collect();

    for (i, (t, pos)) in chosen.iter().enumerate() {
        let r = cell_radius(t.biome);
        let mut poly: Vec<Vec2> = (0..16)
            .map(|k| {
                let a = k as f32 / 16.0 * std::f32::consts::TAU;
                *pos + Vec2::new(a.cos(), a.sin()) * r
            })
            .collect();
        poly = clip(&poly, Vec2::new(1.0, 0.0), MAP_W - 6.0);
        poly = clip(&poly, Vec2::new(-1.0, 0.0), -6.0);
        poly = clip(&poly, Vec2::new(0.0, 1.0), MAP_H - 6.0);
        poly = clip(&poly, Vec2::new(0.0, -1.0), -6.0);
        for (j, (o, opos)) in chosen.iter().enumerate() {
            if i == j {
                continue;
            }
            let d = (*opos - *pos).length();
            if d > r + cell_radius(o.biome) {
                continue;
            }
            let dir = *opos - *pos;
            let mid = (*pos + *opos) * 0.5;
            // Inseln werden nur durch ihren eigenen Radius begrenzt, Küstenland nicht durch Inseln beschnitten.
            if (t.biome == Biome::Island) != (o.biome == Biome::Island) {
                continue;
            }
            poly = clip(&poly, dir, mid.dot(dir));
        }
        let mut deposits = Vec::new();
        for (g, rich) in t.dep {
            if let Some(good) = Good::from_id(g) {
                let jit: i32 = rng.random_range(-1..=1);
                deposits.push((good, (*rich as i32 + jit).clamp(1, 5) as u8));
            }
        }
        // Gelegentliche Zusatzvorkommen (Seed-Variation)
        if rng.random::<f32>() < 0.3 {
            let extra = match t.biome {
                Biome::Hills | Biome::Mountain => Good::IronOre,
                Biome::Forest => Good::Herbs,
                Biome::Plains => Good::Flax,
                Biome::Coast | Biome::Island => Good::Salt,
                Biome::Swamp => Good::Clay,
                _ => Good::Stone,
            };
            if !deposits.iter().any(|(g, _)| *g == extra) {
                deposits.push((extra, 1 + rng.random_range(0..2) as u8));
            }
        }
        // Jede bewohnbare Provinz hat zumindest etwas Ackerland oder Weide (Selbstversorgung mit Handel).
        if !matches!(t.biome, Biome::Hollowland) && !deposits.iter().any(|(g, _)| *g == Good::Grain) {
            let base = (fertility(t.biome) * 3.0).round().max(2.0) as u8;
            deposits.push((Good::Grain, base));
        }
        let c = centroid(&poly, *pos);
        let region = t.region;
        let culture = culture_of(region, t.key);
        let mut faith = [0.0f32; 6];
        let main = crate::catalog::default_religion(culture);
        faith[main.idx()] = 0.9;
        faith[Religion::SunHeresy.idx()] += if region == 5 { 0.0 } else { 0.03 };
        faith[Religion::HollowLiturgy.idx()] += if region >= 5 { 0.05 } else { 0.0 };
        let total: f32 = faith.iter().sum();
        for f in &mut faith {
            *f /= total;
        }
        let veil = (base_veil(region) + (rng.random::<f32>() - 0.5) * 0.6).clamp(0.0, 5.0);
        let holy = match t.key {
            "kaldris" | "brennhollow" => Some(Religion::EternalFlame),
            "solmere" => Some(Religion::SunHeresy),
            "grauhelm" | "kharnfels" => Some(Religion::AncestorPath),
            "herzhain" => Some(Religion::GreenCircle),
            "thalmoor" => Some(Religion::TideHouse),
            "hohles_auge" => Some(Religion::HollowLiturgy),
            _ => None,
        };
        let god = match t.key {
            "kesh_auran" => Some(0),
            "ner_moraun" => Some(2),
            "herzhain" => Some(3),
            "thalmoor" => Some(4),
            "vhalmirsee" => Some(5),
            _ => None,
        };
        let settlement = match t.settle {
            0 => SettlementLevel::Village,
            1 => SettlementLevel::Town,
            2 => SettlementLevel::City,
            _ => SettlementLevel::Capital,
        };
        provinces.push(Province {
            id: i as ProvId,
            key: t.key.to_string(),
            name: t.name.to_string(),
            region,
            centroid: c,
            polygon: poly,
            neighbors: Vec::new(),
            biome: t.biome,
            coastal: t.coastal,
            deposits,
            fertility: fertility(t.biome),
            rainfall: 1.0,
            dry_months: 0,
            settlement,
            buildings: Vec::new(),
            fortification: 0,
            garrison: 0,
            pop: [StratumPop::default(); 8],
            housing: settlement.housing(),
            culture,
            faith,
            owner: None,
            controller: None,
            fief: None,
            rebel_held: false,
            unrest: 5.0,
            hollowing: base_hollowing(region) + rng.random::<f32>() * 3.0,
            veil,
            devastation: 0.0,
            disease: 5.0,
            hygiene: 60.0,
            plague: None,
            tension: [0.0; 5],
            stock: vec![0.0; Good::COUNT],
            unmet: vec![0.0; Good::COUNT],
            produced: vec![0.0; Good::COUNT],
            food_ratio: 1.0,
            variety: 0.0,
            harvest: 1.0,
            shrine: 0,
            hollow_force: 0.0,
            holy_site: holy,
            god_shrine: god,
            siege: None,
            states: Default::default(),
            chain: Default::default(),
            conversion: 0.0,
            refugees: 0.0,
            forest_cover: if t.biome == Biome::Forest { 0.85 } else { 0.4 },
            original_owner: None,
            captured_day: 0,
            artifact: None,
            turnover: 0.0,
            trade_post: false,
        });
        regions[region as usize].provinces.push(i as ProvId);
    }

    // Nachbarschaft
    let key_idx = |k: &str| provinces.iter().position(|p| p.key == k);
    let mut links: Vec<(usize, usize, LinkKind)> = Vec::new();
    let add = |links: &mut Vec<(usize, usize, LinkKind)>, a: usize, b: usize, k: LinkKind| {
        if a != b && !links.iter().any(|(x, y, _)| (*x == a && *y == b) || (*x == b && *y == a)) {
            links.push((a, b, k));
        }
    };
    for i in 0..n {
        for j in (i + 1)..n {
            let (pi, pj) = (&provinces[i], &provinces[j]);
            if pi.region != pj.region {
                continue;
            }
            let (ci, cj) = (chosen[i].1, chosen[j].1);
            let d = (ci - cj).length();
            if d > 150.0 {
                continue;
            }
            if pi.biome == Biome::Island {
                continue; // Inselverbindungen entstehen als Seewege unten
            }
            // gemeinsame Kante?
            let dir = (cj - ci).normalize_or_zero();
            let mid = (ci + cj) * 0.5;
            let on: Vec<Vec2> = pi.polygon.iter().copied().filter(|v| (*v - mid).dot(dir).abs() < 0.05).collect();
            let mut shared = false;
            for a in 0..on.len() {
                for b in (a + 1)..on.len() {
                    if (on[a] - on[b]).length() > 3.0 {
                        shared = true;
                    }
                }
            }
            if shared {
                add(&mut links, i, j, LinkKind::Land);
            }
        }
    }
    for (a, b) in GATES {
        if let (Some(i), Some(j)) = (key_idx(a), key_idx(b)) {
            add(&mut links, i, j, LinkKind::Land);
        }
    }
    for (a, b) in SEA_LINKS {
        if let (Some(i), Some(j)) = (key_idx(a), key_idx(b)) {
            add(&mut links, i, j, LinkKind::Sea);
        }
    }
    // Inselketten: jede Insel mit ihren zwei nächsten Nachbarn (Seeweg)
    let islands: Vec<usize> = (0..n).filter(|&i| provinces[i].biome == Biome::Island).collect();
    for &i in &islands {
        let mut near: Vec<(f32, usize)> = islands.iter().filter(|&&j| j != i).map(|&j| ((chosen[i].1 - chosen[j].1).length(), j)).collect();
        near.sort_by(|a, b| a.0.total_cmp(&b.0));
        for (d, j) in near.into_iter().take(2) {
            if d < 170.0 {
                add(&mut links, i, j, LinkKind::Sea);
            }
        }
    }
    // Konnektivität sicherstellen
    loop {
        let mut comp = vec![usize::MAX; n];
        let mut ncomp = 0;
        for s in 0..n {
            if comp[s] != usize::MAX {
                continue;
            }
            let mut stack = vec![s];
            comp[s] = ncomp;
            while let Some(x) = stack.pop() {
                for (a, b, _) in &links {
                    let o = if *a == x {
                        *b
                    } else if *b == x {
                        *a
                    } else {
                        continue;
                    };
                    if comp[o] == usize::MAX {
                        comp[o] = ncomp;
                        stack.push(o);
                    }
                }
            }
            ncomp += 1;
        }
        if ncomp <= 1 {
            break;
        }
        // kürzeste Verbindung zwischen Komponente 0 und dem Rest
        let mut best: Option<(f32, usize, usize)> = None;
        for i in 0..n {
            for j in 0..n {
                if comp[i] == 0 && comp[j] != 0 {
                    let d = (chosen[i].1 - chosen[j].1).length();
                    if best.is_none_or(|b| d < b.0) {
                        best = Some((d, i, j));
                    }
                }
            }
        }
        let (_, i, j) = best.expect("Komponenten vorhanden");
        let kind = if provinces[i].biome == Biome::Island || provinces[j].biome == Biome::Island || provinces[i].coastal && provinces[j].coastal { LinkKind::Sea } else { LinkKind::Land };
        add(&mut links, i, j, kind);
    }
    for (a, b, k) in links {
        let d = (chosen[a].1 - chosen[b].1).length();
        provinces[a].neighbors.push(Link { to: b as ProvId, kind: k, dist: d });
        provinces[b].neighbors.push(Link { to: a as ProvId, kind: k, dist: d });
    }
    for p in &mut provinces {
        p.neighbors.sort_by_key(|l| l.to);
    }
    MapData { provinces, regions }
}
