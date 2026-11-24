//! Übergang Weltkarte ↔ Echtzeitschlacht: erzeugt aus einem `BattleRequest` eine RTS-Mission samt Karte,
//! Befestigungen und Heeren und wertet die gespielte Schlacht als `BattleResult` aus.
//!
//! Zuordnung der Einheiten: [`UNIT_TABLE`] bildet jede strategische Einheiten-ID auf eine RTS-Einheit ab.
//! Gibt es die strategische ID selbst in den RTS-Daten (`units.ron`), wird sie direkt verwendet
//! ([`resolve_unit`]); die Tabelle ist die Rückfallebene für Einheiten ohne eigene RTS-Entsprechung.

use ashen_core::noise::{fbm2, rand2};
use ashen_sim::SimWorld;
use ashen_sim::data::{Cost, GameData};
use ashen_sim::entity::{ENEMY, NodeKind, Order, PLAYER};
use ashen_sim::env::Weather as SimWeather;
use ashen_sim::map::{Decor, DecorKind, Terrain, TileMap};
use ashen_sim::mission::{
    Action, Condition, Difficulty, GameMode, MatchSettings, MissionDef, ObjectiveDef, Placement,
    TriggerDef, WaveRules,
};
use ashen_sim::scenario::{DecorPlace, MapData, NodePlace};
use ashen_sim::systems::tactics;
use ashen_strategy::catalog::{self, UClass};
use ashen_strategy::{
    ArmyResult, BattleArmy, BattleRequest, BattleResult, BattleSide, Biome, HeroOutcome, Regiment,
    Season, Weather,
};
use glam::{IVec2, Vec2};
use serde::{Deserialize, Serialize};

/// Kartenkantenlänge der generierten Schlachtfelder (Kacheln).
pub const BATTLE_MAP: i32 = 72;
/// Höchstzahl gleichzeitiger RTS-Einheiten je Seite; größere Heere werden skaliert (1 RTS-Einheit = `scale` Soldaten).
pub const MAX_UNITS_PER_SIDE: u32 = 48;
/// Zeitlimit einer Schlacht in Sekunden; danach gewinnt der Verteidiger (Belagerung) bzw. die stärkere Seite.
pub const TIME_LIMIT: f32 = 30.0 * 60.0;

/// Strategische Einheiten-ID → RTS-Einheit (Rückfallebene) mit Begründung.
pub const UNIT_TABLE: &[(&str, &str, &str)] = &[
    ("worker", "worker", "gleiche Einheit"),
    ("swordsman", "swordsman", "gleiche Einheit"),
    ("spearman", "spearman", "gleiche Einheit"),
    ("archer", "archer", "gleiche Einheit"),
    ("crossbowman", "crossbowman", "gleiche Einheit"),
    ("halberdier", "halberdier", "gleiche Einheit"),
    ("shieldbearer", "shieldbearer", "gleiche Einheit"),
    ("scout_rider", "scout_rider", "gleiche Einheit"),
    ("horse_archer", "horse_archer", "gleiche Einheit"),
    ("knight", "knight", "gleiche Einheit"),
    (
        "ember_adept",
        "swordsman",
        "Elite-Nahkämpfer ohne eigene RTS-Einheit",
    ),
    (
        "flame_priest",
        "priest",
        "Unterstützung aus der zweiten Reihe",
    ),
    ("priest", "priest", "gleiche Einheit"),
    ("ember_mage", "battle_mage", "Magier"),
    ("battle_mage", "battle_mage", "gleiche Einheit"),
    ("battering_ram", "battering_ram", "gleiche Einheit"),
    ("mangonel", "catapult", "Wurfgerät"),
    ("trebuchet", "catapult", "Wurfgerät"),
    ("catapult", "catapult", "gleiche Einheit"),
    ("hero_isolde", "hero_isolde", "gleiche Einheit"),
    ("hero_hal_venn", "hero_hal_venn", "gleiche Einheit"),
    ("hero_torvald", "hero_torvald", "gleiche Einheit"),
    ("skarn_ironguard", "shieldbearer", "schwere Infanterie"),
    ("skarn_rune_thrower", "archer", "Fernkämpfer"),
    ("veyl_stalker", "archer", "Fernkämpfer"),
    ("thorn_warden", "spearman", "Infanterie mit Stangenwaffe"),
    ("saelith_marine", "swordsman", "Infanterie"),
    ("saelith_arbalest", "crossbowman", "Fernkämpfer"),
    ("hollow_thrall", "hollow_thrall", "gleiche Einheit"),
    ("hollow_archer", "hollow_archer", "gleiche Einheit"),
    ("grave_brute", "grave_brute", "gleiche Einheit"),
    ("hollow_digger", "hollow_digger", "gleiche Einheit"),
    ("ghoul", "ghoul", "gleiche Einheit"),
    ("ash_witch", "ash_witch", "gleiche Einheit"),
    ("bat_swarm", "bat_swarm", "gleiche Einheit"),
    ("veil_wraith", "ghoul", "leichter Hohler"),
    ("hollow_priest", "ash_witch", "Hohler Zauberer"),
    ("ashen_legionnaire", "hollow_thrall", "untote Infanterie"),
    ("cinder_knight", "grave_brute", "untote Schwerkavallerie"),
    ("ash_wolf", "bat_swarm", "Raubtier (kleine Kreatur)"),
    ("grave_worm", "grave_brute", "große Kreatur"),
    ("glow_salamander", "grave_brute", "große Kreatur"),
    ("moor_light", "hollow_archer", "Irrlicht als Fernkämpfer"),
    ("boss_pale_reeve", "boss_pale_reeve", "gleiche Einheit"),
    ("boss_ashen_dragon", "boss_pale_reeve", "Boss"),
    ("boss_root_mother", "boss_pale_reeve", "Boss"),
    ("boss_hollow_king", "boss_pale_reeve", "Boss"),
    ("boss_stone_oath", "boss_pale_reeve", "Boss"),
    ("boss_reef_mistress", "boss_pale_reeve", "Boss"),
];

/// Letzte Rückfallebene nach Einheitenklasse (für unbekannte, z. B. per Mod ergänzte strategische IDs).
fn class_fallback(id: &str) -> &'static str {
    match catalog::unit(id).map(|u| u.class) {
        Some(UClass::Ranged | UClass::Mage | UClass::Support | UClass::Siege) => "archer",
        Some(UClass::Cavalry) => "knight",
        Some(UClass::Monster) => "grave_brute",
        Some(UClass::Boss) => "boss_pale_reeve",
        Some(UClass::Worker) => "worker",
        Some(UClass::Hero) => "hero_isolde",
        _ => "swordsman",
    }
}

/// RTS-Einheiten-ID zu einer strategischen ID: direkt, wenn die RTS-Daten sie kennen, sonst über die Tabelle.
pub fn resolve_unit(data: &GameData, strategic: &str) -> String {
    if data.unit_id(strategic).is_some() {
        return strategic.to_string();
    }
    let via_table = UNIT_TABLE
        .iter()
        .find(|(s, _, _)| *s == strategic)
        .map(|(_, r, _)| *r)
        .unwrap_or_else(|| class_fallback(strategic));
    if data.unit_id(via_table).is_some() {
        via_table.to_string()
    } else {
        "swordsman".to_string()
    }
}

/// Verknüpfung einer laufenden Schlacht mit dem strategischen Rückkehrpunkt (wird im RTS-Spielstand mitgespeichert).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BattleLink {
    /// Dateiname des strategischen Rückkehrpunkts im Ordner `battle_return`.
    pub return_point: String,
    pub battle_id: u32,
    pub scale: u32,
}

/// Fertig geplante Schlacht: Mission, Karte und Randbedingungen.
pub struct BattlePlan {
    pub mission: MissionDef,
    pub settings: MatchSettings,
    pub scale: u32,
    pub player_attacks: bool,
    pub weather: SimWeather,
    pub time_of_day: f32,
    /// Wo der Spieler beginnt (Kamerastart).
    pub player_focus: Vec2,
    /// Wie viele Einheiten je Seite erzeugt werden (Spieler, Gegner).
    pub spawned: (u32, u32),
}

fn tag_of(army: u32, reg: usize) -> String {
    format!("r{army}:{reg}")
}

fn hero_tag(c: u32) -> String {
    format!("h{c}")
}

fn rts_count(count: u32, scale: u32) -> u32 {
    if count == 0 {
        0
    } else {
        count.div_ceil(scale.max(1)).max(1)
    }
}

fn side_total(list: &[BattleArmy]) -> u32 {
    list.iter()
        .flat_map(|b| b.regiments.iter())
        .map(|r| r.count)
        .sum()
}

/// Skalierung: so, dass die größere Seite höchstens `MAX_UNITS_PER_SIDE` Einheiten stellt.
pub fn scale_for(req: &BattleRequest) -> u32 {
    side_total(&req.attackers)
        .max(side_total(&req.defenders))
        .div_ceil(MAX_UNITS_PER_SIDE)
        .max(1)
}

/// Wetter der Weltkarte → Echtzeitwetter (Schnee: bedeckter Himmel, das Echtzeitspiel kennt keinen Schnee).
pub fn map_weather(w: Weather) -> SimWeather {
    match w {
        Weather::Clear => SimWeather::Clear,
        Weather::Rain => SimWeather::Rain,
        Weather::Snow => SimWeather::Overcast,
        Weather::Storm => SimWeather::Storm,
        Weather::AshStorm => SimWeather::Ashfall,
        Weather::Fog => SimWeather::Fog,
    }
}

fn terrain_for(biome: Biome, season: Season, n: f32, n2: f32, n3: f32) -> Terrain {
    let mut t = match biome {
        Biome::Plains | Biome::Coast | Biome::Island => {
            if n > 0.62 {
                Terrain::DarkGrass
            } else if n < 0.24 {
                Terrain::Dirt
            } else {
                Terrain::Grass
            }
        }
        Biome::Hills => {
            if n2 > 0.62 {
                Terrain::Stone
            } else if n < 0.34 {
                Terrain::Dirt
            } else {
                Terrain::Grass
            }
        }
        Biome::Forest => {
            if n < 0.18 {
                Terrain::Dirt
            } else {
                Terrain::DarkGrass
            }
        }
        Biome::Mountain => {
            if n < 0.28 {
                Terrain::Dirt
            } else if n > 0.78 {
                Terrain::Grass
            } else {
                Terrain::Stone
            }
        }
        Biome::Swamp => {
            if n2 > 0.74 {
                Terrain::Water
            } else if n2 > 0.58 {
                Terrain::Ford
            } else if n < 0.3 {
                Terrain::Dirt
            } else {
                Terrain::DarkGrass
            }
        }
        Biome::Desert => {
            if n < 0.25 {
                Terrain::Dirt
            } else if n2 > 0.8 {
                Terrain::Stone
            } else {
                Terrain::Sand
            }
        }
        Biome::Volcanic => {
            if n2 > 0.58 {
                Terrain::Stone
            } else if n < 0.3 {
                Terrain::Dirt
            } else {
                Terrain::Ash
            }
        }
        Biome::Tundra => {
            if n2 > 0.55 {
                Terrain::Stone
            } else if n < 0.45 {
                Terrain::Dirt
            } else {
                Terrain::Sand
            }
        }
        Biome::Ruins => {
            if n2 > 0.6 {
                Terrain::Stone
            } else if n < 0.4 {
                Terrain::Dirt
            } else {
                Terrain::Grass
            }
        }
        Biome::Hollowland => {
            if n < 0.28 {
                Terrain::Dirt
            } else {
                Terrain::Ash
            }
        }
    };
    // Jahreszeit: im Winter und Herbst weicht das Gras dem kahlen Boden.
    let bare = match season {
        Season::Winter => 0.45,
        Season::Autumn => 0.2,
        _ => 0.0,
    };
    if matches!(t, Terrain::Grass | Terrain::DarkGrass) && n3 < bare {
        t = Terrain::Dirt;
    }
    t
}

fn dist_to_line(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    (p - (a + ab * t)).length()
}

/// Terrain und Dekoration des Schlachtfelds. Bäume (Wald) werden erst in der Welt als Ressourcenknoten gesetzt.
pub fn battle_terrain(req: &BattleRequest, seed: u64) -> TileMap {
    let s = (seed as u32) ^ (seed >> 32) as u32;
    let mut m = TileMap::new(BATTLE_MAP, BATTLE_MAP, Terrain::Grass);
    let river = matches!(
        req.terrain,
        Biome::Plains | Biome::Hills | Biome::Forest | Biome::Coast
    ) && !req.siege
        && s.is_multiple_of(3);
    // Fluss quer zur Schlachtachse (Diagonale von links unten nach rechts oben) mit zwei Furten.
    let (ra, rb) = (Vec2::new(8.0, 64.0), Vec2::new(64.0, 8.0));
    let fords = [Vec2::new(28.0, 44.0), Vec2::new(46.0, 26.0)];
    for y in 0..BATTLE_MAP {
        for x in 0..BATTLE_MAP {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let n = fbm2(p.x * 0.09, p.y * 0.09, s, 4);
            let n2 = fbm2(p.x * 0.17 + 40.0, p.y * 0.17, s.wrapping_add(7), 3);
            let n3 = fbm2(p.x * 0.23 + 9.0, p.y * 0.23, s.wrapping_add(13), 2);
            let mut t = terrain_for(req.terrain, req.season, n, n2, n3);
            match req.terrain {
                Biome::Coast => {
                    // Meer im Südosten (hohes x+y), Strand davor; die Mitte bleibt Land.
                    let c = (p.x + p.y) - (BATTLE_MAP as f32 * 1.52) + (n2 - 0.5) * 5.0;
                    if c > 3.0 {
                        t = Terrain::DeepWater;
                    } else if c > 0.5 {
                        t = Terrain::Water;
                    } else if c > -2.0 {
                        t = Terrain::Sand;
                    }
                }
                Biome::Island => {
                    let c = (p - Vec2::splat(BATTLE_MAP as f32 / 2.0)).length() + (n2 - 0.5) * 6.0;
                    if c > 40.0 {
                        t = Terrain::DeepWater;
                    } else if c > 36.0 {
                        t = Terrain::Water;
                    } else if c > 33.0 {
                        t = Terrain::Sand;
                    }
                }
                _ => {}
            }
            if river {
                let d = dist_to_line(p, ra, rb) + (n2 - 0.5) * 1.4;
                let ford = fords.iter().any(|f| (p - *f).length() < 3.0);
                if d < 1.6 {
                    t = if ford {
                        Terrain::Ford
                    } else {
                        Terrain::DeepWater
                    };
                } else if d < 2.6 {
                    t = if ford { Terrain::Ford } else { Terrain::Water };
                } else if d < 3.6 && t.buildable() {
                    t = Terrain::Sand;
                }
            }
            // Hohlung: Aschestreifen in verderbten Gebieten.
            if req.veil >= 3.0 && n3 > 0.6 && t.buildable() && n2 > 0.35 {
                t = Terrain::Ash;
            }
            m.set_terrain(IVec2::new(x, y), t);
        }
    }
    // Dekoration.
    for y in 0..BATTLE_MAP {
        for x in 0..BATTLE_MAP {
            let t = IVec2::new(x, y);
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let terr = m.terrain_at(t);
            let r = rand2(x, y, s ^ 0x51);
            let jitter = Vec2::new(rand2(x, y, s ^ 0x77) - 0.5, rand2(x, y, s ^ 0x78) - 0.5) * 0.6;
            let variant = (rand2(x, y, s ^ 0x99) * 8.0) as u8;
            let kind = match terr {
                Terrain::Grass | Terrain::DarkGrass => {
                    if r < 0.10 {
                        Some(DecorKind::GrassTuft)
                    } else if r < 0.125 && req.season != Season::Winter {
                        Some(DecorKind::Flowers)
                    } else if r < 0.15 {
                        Some(DecorKind::Bush)
                    } else if r < 0.16 {
                        Some(DecorKind::Rock)
                    } else {
                        None
                    }
                }
                Terrain::Dirt if r < 0.05 => Some(DecorKind::Rock),
                Terrain::Stone if r < 0.1 => Some(DecorKind::Rock),
                Terrain::Stone
                    if r < 0.14 && matches!(req.terrain, Biome::Mountain | Biome::Hills) =>
                {
                    Some(DecorKind::Boulder)
                }
                Terrain::Sand if r < 0.14 && !matches!(req.terrain, Biome::Desert) => {
                    Some(DecorKind::Reeds)
                }
                Terrain::Ash => {
                    if r < 0.05 {
                        Some(DecorKind::Bones)
                    } else if r < 0.08 && req.terrain == Biome::Hollowland {
                        Some(DecorKind::Grave)
                    } else if r < 0.095 {
                        Some(DecorKind::DeadTree)
                    } else if r < 0.105 && req.terrain == Biome::Ruins {
                        Some(DecorKind::Pillar)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            let kind = match (req.terrain, kind) {
                (Biome::Ruins, None) if terr.buildable() && r > 0.985 => Some(DecorKind::Ruin),
                (Biome::Ruins, Some(DecorKind::Bush)) => Some(DecorKind::Pillar),
                (Biome::Swamp, Some(DecorKind::Flowers)) => Some(DecorKind::Reeds),
                (_, k) => k,
            };
            if let Some(k) = kind {
                let pos = if k.blocks() { p } else { p + jitter };
                m.decor.push(Decor {
                    kind: k,
                    pos,
                    variant,
                });
                if k.blocks() {
                    let i = m.idx(t);
                    m.static_block[i] = true;
                }
            }
        }
    }
    m
}

/// Bäume für Waldgebiete als Ressourcenknoten der Karte (abseits der Aufstellungszonen der Heere).
fn tree_nodes(
    map: &TileMap,
    req: &BattleRequest,
    seed: u64,
    keep_free: &[(Vec2, f32)],
) -> Vec<NodePlace> {
    let s = (seed as u32) ^ (seed >> 32) as u32;
    let density = match req.terrain {
        Biome::Forest => 0.55,
        Biome::Swamp | Biome::Hills => 0.12,
        Biome::Plains | Biome::Coast | Biome::Ruins => 0.03,
        Biome::Hollowland => 0.04,
        _ => 0.0,
    };
    let mut out = Vec::new();
    if density <= 0.0 {
        return out;
    }
    for y in 0..BATTLE_MAP {
        for x in 0..BATTLE_MAP {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let t = IVec2::new(x, y);
            if keep_free.iter().any(|(c, r)| (p - *c).length() < *r) || !map.walkable(t) {
                continue;
            }
            let terr = map.terrain_at(t);
            if !matches!(
                terr,
                Terrain::Grass | Terrain::DarkGrass | Terrain::Dirt | Terrain::Ash
            ) {
                continue;
            }
            let f = fbm2(p.x * 0.11 + 13.0, p.y * 0.11, s ^ 0xF0, 4);
            let r = rand2(x, y, s ^ 0x7EE);
            let place = if f > 0.55 {
                r < density * 1.4
            } else {
                r < density * 0.08
            };
            if place {
                let variant = if terr == Terrain::Ash {
                    200 + (r * 1000.0) as u8 % 3
                } else {
                    (rand2(x, y, s ^ 0x3) * 6.0) as u8
                };
                out.push(NodePlace {
                    kind: NodeKind::Tree,
                    x,
                    y,
                    amount: 0,
                    variant,
                });
            }
        }
    }
    out
}

/// Karte der Schlacht als Szenario-Kartendaten (`MissionDef::map`): Gelände, Dekoration und Bäume.
pub fn battle_map(req: &BattleRequest, seed: u64) -> MapData {
    let tile = battle_terrain(req, seed);
    let zones = [
        (Vec2::new(19.0, 19.0), 8.0),
        (Vec2::new(18.0, 18.0), 8.0),
        (Vec2::new(53.0, 53.0), 8.0),
        (Vec2::new(50.0, 50.0), if req.siege { 15.0 } else { 8.0 }),
    ];
    let nodes = tree_nodes(&tile, req, seed, &zones);
    MapData {
        width: tile.width,
        height: tile.height,
        terrain: MapData::rows_from(tile.width, tile.height, &tile.terrain),
        decor: tile
            .decor
            .iter()
            .map(|d| DecorPlace {
                kind: d.kind,
                x: d.pos.x,
                y: d.pos.y,
                variant: d.variant,
            })
            .collect(),
        nodes,
    }
}

struct RegPlan {
    army: u32,
    idx: usize,
    rts: String,
    class: UClass,
    count: u32,
}

fn clamp_pos(p: Vec2) -> Vec2 {
    p.clamp(Vec2::splat(4.0), Vec2::splat(BATTLE_MAP as f32 - 4.0))
}

/// Ordnet die Regimenter einer Seite in Linien an: Infanterie vorn, Fernkämpfer dahinter, Reiter an den Flanken.
fn layout(regs: &[RegPlan], origin: Vec2, forward: Vec2, scale: u32) -> Vec<Vec2> {
    let perp = Vec2::new(-forward.y, forward.x);
    let mut out = vec![origin; regs.len()];
    let width = |r: &RegPlan| 2.0 * 0.55 * (rts_count(r.count, scale) as f32).sqrt() + 2.2;
    let pick = |f: &dyn Fn(UClass) -> bool| {
        regs.iter()
            .enumerate()
            .filter(|(_, r)| f(r.class))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    };
    let front = pick(&|c| {
        matches!(
            c,
            UClass::Infantry | UClass::Monster | UClass::Boss | UClass::Hero | UClass::Worker
        )
    });
    let back = pick(&|c| {
        matches!(
            c,
            UClass::Ranged | UClass::Mage | UClass::Support | UClass::Siege
        )
    });
    let flank = pick(&|c| c == UClass::Cavalry);
    let front_w: f32 = front.iter().map(|&i| width(&regs[i])).sum();
    for (idxs, depth, is_flank) in [
        (&front, 0.0, false),
        (&back, -4.5, false),
        (&flank, -1.0, true),
    ] {
        if is_flank {
            // Reiter stehen außen: die Breite der Frontlinie bleibt frei, abwechselnd links und rechts.
            let mut cursor = [front_w / 2.0 + 2.0; 2];
            for (k, &i) in idxs.iter().enumerate() {
                let side = k % 2;
                let w = width(&regs[i]);
                let off = cursor[side] + w / 2.0;
                cursor[side] += w;
                out[i] =
                    clamp_pos(origin + forward * depth + perp * if side == 1 { -off } else { off });
            }
        } else {
            let total: f32 = idxs.iter().map(|&i| width(&regs[i])).sum();
            let mut cursor = -total / 2.0;
            for &i in idxs.iter() {
                let w = width(&regs[i]);
                out[i] = clamp_pos(origin + forward * depth + perp * (cursor + w / 2.0));
                cursor += w;
            }
        }
    }
    out
}

fn placement(
    id: &str,
    owner: u8,
    p: Vec2,
    count: u32,
    tag: Option<String>,
    guard: bool,
) -> Placement {
    Placement {
        id: id.to_string(),
        owner,
        x: p.x,
        y: p.y,
        count,
        tag,
        guard,
        camp: None,
    }
}

/// Wehranlage um `center`: Mauerring, Tore, Türme, Bresche(n) zur Angreiferseite hin, Bergfried und Häuser.
fn fortress(
    data: &GameData,
    owner: u8,
    center: IVec2,
    fort: u8,
    breaches: u8,
    rank: u8,
    keep_tag: &str,
) -> Vec<Placement> {
    let mut out = Vec::new();
    let has = |id: &str| data.building_id(id).is_some();
    let half = 6 + (fort as i32) / 2;
    let at = |x: i32, y: i32| Vec2::new(x as f32, y as f32);
    if has("keep") {
        out.push(placement(
            "keep",
            owner,
            at(center.x - 1, center.y - 1),
            1,
            Some(keep_tag.to_string()),
            false,
        ));
    }
    if fort > 0 && has("wall") {
        let (x0, y0, x1, y1) = (
            center.x - half,
            center.y - half,
            center.x + half,
            center.y + half,
        );
        // Angreifer kommen von niedrigem x/y: Tore und Breschen liegen an den Seiten x0 und y0.
        let gate_a = IVec2::new(x0, center.y);
        let gate_b = IVec2::new(center.x, y0);
        let mut gaps: Vec<IVec2> = Vec::new();
        for b in 0..breaches.min(4) as i32 {
            let off = 3 + (b / 2) * 3;
            let sign = if b % 2 == 0 { 1 } else { -1 };
            if b < 2 {
                gaps.extend((0..3).map(|k| IVec2::new(x0, center.y + sign * (off + k))));
            } else {
                gaps.extend((0..3).map(|k| IVec2::new(center.x + sign * (off + k), y0)));
            }
        }
        let mut ring: Vec<IVec2> = Vec::new();
        for i in 0..=2 * half {
            ring.push(IVec2::new(x0 + i, y0));
            ring.push(IVec2::new(x0 + i, y1));
            ring.push(IVec2::new(x0, y0 + i));
            ring.push(IVec2::new(x1, y0 + i));
        }
        ring.sort_by_key(|t| (t.x, t.y));
        ring.dedup();
        let tower_tiles: Vec<IVec2> = {
            let mut v = vec![
                IVec2::new(x0, y0),
                IVec2::new(x1 - 1, y0),
                IVec2::new(x0, y1 - 1),
                IVec2::new(x1 - 1, y1 - 1),
            ];
            if fort >= 6 {
                v.extend([
                    IVec2::new(x0, center.y - 6),
                    IVec2::new(x0, center.y + 6),
                    IVec2::new(center.x - 6, y0),
                    IVec2::new(center.x + 6, y0),
                ]);
            }
            v
        };
        let in_tower = |t: IVec2| {
            tower_tiles
                .iter()
                .any(|o| t.x >= o.x && t.x < o.x + 2 && t.y >= o.y && t.y < o.y + 2)
        };
        for t in ring {
            if in_tower(t) {
                continue;
            }
            let is_gate = t == gate_a || (fort >= 5 && t == gate_b);
            if is_gate && has("gate") {
                out.push(placement("gate", owner, at(t.x, t.y), 1, None, false));
            } else if !gaps.contains(&t) {
                out.push(placement("wall", owner, at(t.x, t.y), 1, None, false));
            }
        }
        if has("wall_tower") {
            for o in tower_tiles {
                out.push(placement("wall_tower", owner, at(o.x, o.y), 1, None, false));
            }
        }
    }
    if has("watchtower") && fort >= 3 {
        out.push(placement(
            "watchtower",
            owner,
            at(center.x + 3, center.y - 2),
            1,
            None,
            false,
        ));
        out.push(placement(
            "watchtower",
            owner,
            at(center.x - 4, center.y + 3),
            1,
            None,
            false,
        ));
    }
    if has("hut") {
        let huts = 2 + rank as i32 * 2;
        for i in 0..huts.min(10) {
            let a = i as f32 * 0.9 + 0.5;
            let r = 3.6 + (i % 3) as f32 * 0.7;
            out.push(placement(
                "hut",
                owner,
                at(
                    center.x + (a.cos() * r) as i32,
                    center.y + (a.sin() * r) as i32,
                ),
                1,
                None,
                false,
            ));
        }
    }
    out
}

/// Plant die RTS-Schlacht zu einer wartenden Weltkarten-Schlacht (Spielerreich `player`).
pub fn plan_battle(
    data: &GameData,
    player: u16,
    req: &BattleRequest,
) -> Result<BattlePlan, String> {
    let player_attacks = req.attackers.iter().any(|b| b.realm == Some(player));
    if !player_attacks && !req.defenders.iter().any(|b| b.realm == Some(player)) {
        return Err("Das Spielerreich ist an dieser Schlacht nicht beteiligt".into());
    }
    let (mine, theirs) = if player_attacks {
        (&req.attackers, &req.defenders)
    } else {
        (&req.defenders, &req.attackers)
    };
    let scale = scale_for(req);
    let seed = ((req.province as u64) * 0x9E37_79B1) ^ ((req.day as u64) << 20) ^ req.id as u64;
    let siege = req.siege;
    // Frontrichtung: Angreifer starten bei niedrigem x/y, Verteidiger gegenüber.
    let fwd_attack = Vec2::new(1.0, 1.0).normalize();
    let (attack_origin, defend_origin) = if siege {
        (Vec2::new(18.0, 18.0), Vec2::new(50.0, 50.0))
    } else {
        (Vec2::new(19.0, 19.0), Vec2::new(53.0, 53.0))
    };
    let mut placements: Vec<Placement> = Vec::new();
    let mut spawned = (0u32, 0u32);
    for (side_idx, (list, owner)) in [(mine, PLAYER), (theirs, ENEMY)].into_iter().enumerate() {
        let is_attacker = (side_idx == 0) == player_attacks;
        let (origin, forward) = if is_attacker {
            (attack_origin, fwd_attack)
        } else {
            (defend_origin, -fwd_attack)
        };
        let mut regs: Vec<RegPlan> = Vec::new();
        for b in list.iter() {
            for (i, r) in b.regiments.iter().enumerate() {
                if r.count == 0 {
                    continue;
                }
                regs.push(RegPlan {
                    army: b.army,
                    idx: i,
                    rts: resolve_unit(data, &r.unit),
                    class: catalog::unit(&r.unit)
                        .map(|u| u.class)
                        .unwrap_or(UClass::Infantry),
                    count: r.count,
                });
            }
        }
        let positions = layout(&regs, origin, forward, scale);
        for (r, p) in regs.iter().zip(positions) {
            let n = rts_count(r.count, scale);
            if side_idx == 0 {
                spawned.0 += n
            } else {
                spawned.1 += n
            }
            placements.push(placement(
                &r.rts,
                owner,
                p,
                n,
                Some(tag_of(r.army, r.idx)),
                !is_attacker,
            ));
        }
        // Helden: nur die Heldin Isolde hat ein RTS-Gegenstück; weitere Helden treten als Ritter an.
        let mut isolde_used = false;
        for b in list.iter() {
            for (hi, c) in b.heroes.iter().enumerate() {
                let id = if owner == PLAYER && !isolde_used && data.unit_id("hero_isolde").is_some()
                {
                    isolde_used = true;
                    "hero_isolde"
                } else {
                    "knight"
                };
                let p = clamp_pos(
                    origin
                        + forward * 3.0
                        + Vec2::new(-forward.y, forward.x) * (hi as f32 * 2.0 - 1.0),
                );
                placements.push(placement(id, owner, p, 1, Some(hero_tag(*c)), !is_attacker));
                if side_idx == 0 {
                    spawned.0 += 1
                } else {
                    spawned.1 += 1
                }
            }
        }
    }
    // Wehranlage des Verteidigers.
    let (defender_owner, keep_tag) = if player_attacks {
        (ENEMY, "enemy_keep")
    } else {
        (PLAYER, "player_keep")
    };
    let rank = 1 + (req.fortification / 4).min(2);
    let objective = if siege {
        placements.extend(fortress(
            data,
            defender_owner,
            IVec2::new(defend_origin.x as i32, defend_origin.y as i32),
            req.fortification,
            req.breaches,
            rank,
            keep_tag,
        ));
        if player_attacks {
            format!(
                "Brecht die Mauern von {} und zerstört den Bergfried oder vernichtet die Besatzung.",
                req.province_name
            )
        } else {
            format!(
                "Haltet {} gegen die Belagerer – fällt der Bergfried, ist die Stadt verloren.",
                req.province_name
            )
        }
    } else if player_attacks {
        "Vernichtet das feindliche Heer.".to_string()
    } else {
        "Schlagt den Angriff zurück und haltet das Feld.".to_string()
    };
    let weather = map_weather(req.weather);
    let intro = format!(
        "{} bei {} ({}, {}, {}). Eure Heere stehen bereit.",
        if siege { "Belagerung" } else { "Feldschlacht" },
        req.province_name,
        req.terrain.label(),
        req.season.label(),
        req.weather.label()
    );
    let mission = MissionDef {
        id: format!("gen_battle_{}", req.id),
        name: format!("Schlacht bei {}", req.province_name),
        description: intro.clone(),
        map_seed: seed,
        start_resources: Cost::default(),
        start_goods: Vec::new(),
        waves: WaveRules {
            enabled: false,
            ..Default::default()
        },
        placements,
        objectives: vec![ObjectiveDef {
            id: "obj_battle".into(),
            text: objective,
            optional: false,
        }],
        triggers: vec![TriggerDef {
            id: "battle_intro".into(),
            condition: Condition::AtTime(1.0),
            actions: vec![
                Action::Message {
                    speaker: "Heerführer".into(),
                    text: intro,
                    voice: None,
                },
                Action::AddObjective("obj_battle".into()),
            ],
            modes: Vec::new(),
        }],
        defeat_tag: "player_keep".into(),
        enemy_base_tag: "enemy_keep".into(),
        map: Some(battle_map(req, seed)),
        start_time: Some((req.hour / 24.0).clamp(0.2, 0.9)),
        // Keine Wirtschaftsstrafen in Weltkarten-Schlachten (kein Nahrungsunterhalt, kein Bevölkerungslimit).
        free_economy: true,
        ..Default::default()
    };
    // Sandbox-Modus: keine Standard-Niederlage (kein Bergfried nötig) und keine Wellen; Sieg/Niederlage entscheidet `battle_outcome`.
    let settings = MatchSettings {
        mode: GameMode::Sandbox,
        difficulty: Difficulty::Normal,
        seed,
        hero_permadeath: false,
    };
    let focus = if player_attacks {
        attack_origin
    } else {
        defend_origin
    };
    Ok(BattlePlan {
        mission,
        settings,
        scale,
        player_attacks,
        weather,
        time_of_day: (req.hour / 24.0).clamp(0.2, 0.9),
        player_focus: focus,
        spawned,
    })
}

/// Erzeugt die Simulationswelt der Schlacht aus dem Plan.
pub fn create_world(
    data: &GameData,
    plan: BattlePlan,
    req: &BattleRequest,
) -> Result<SimWorld, String> {
    let (player_attacks, focus, weather, tod) = (
        plan.player_attacks,
        plan.player_focus,
        plan.weather,
        plan.time_of_day,
    );
    log::info!(
        "Weltkarten-Schlacht: {} gegen {} Einheiten (1 Einheit = {} Soldaten)",
        plan.spawned.0,
        plan.spawned.1,
        plan.scale
    );
    let mut world = SimWorld::new(data, plan.mission, plan.settings)?;
    // Sandbox-Zugaben entfernen: es gibt weder Wirtschaft noch Nachschub.
    world.players[PLAYER as usize].resources = Cost::default();
    for g in world.players[PLAYER as usize].goods.iter_mut() {
        *g = 0.0;
    }
    world.env.weather = weather;
    world.env.forced_weather = Some(weather);
    world.env.weather_intensity = 1.0;
    world.env.time_of_day = tod;
    world.fog.disabled = true;
    // Zustand der Regimenter: Lebenspunkte und Moral aus der Weltkarte.
    let state: Vec<(String, f32, f32)> = req
        .attackers
        .iter()
        .chain(req.defenders.iter())
        .flat_map(|b| {
            b.regiments
                .iter()
                .enumerate()
                .map(move |(i, r)| (tag_of(b.army, i), r.hp, b.morale))
        })
        .collect();
    for (_, u) in world.units.iter_mut() {
        if let Some(t) = &u.tag
            && let Some((_, hp, morale)) = state.iter().find(|(x, _, _)| x == t)
        {
            u.hp = (u.max_hp * hp.clamp(0.3, 1.0)).max(1.0);
            u.morale *= 0.6 + 0.4 * (morale / 100.0).clamp(0.0, 1.0);
        }
    }
    // Angreifer des Gegners marschieren auf den Spieler zu; Verteidiger halten ihren Posten (Guard aus der Platzierung).
    if !player_attacks {
        for (_, u) in world.units.iter_mut() {
            if u.owner == ENEMY {
                u.order = Order::Move {
                    target: focus,
                    attack_move: true,
                };
                u.request_path(focus);
            }
        }
    }
    Ok(world)
}

/// Führt die gegnerische Seite taktisch (alle paar Sekunden aufrufen): Fokusfeuer, Fernkämpfer hinter dem Nahkampf,
/// Rückzug Angeschlagener, Belagerungswaffen gegen Tore und Mauern, bemannte Türme beim Verteidiger
/// (siehe `ashen_sim::systems::tactics`).
pub fn drive_enemy(world: &mut SimWorld, data: &GameData, player_attacks: bool) {
    let role = if player_attacks {
        tactics::Role::Defender
    } else {
        tactics::Role::Attacker
    };
    tactics::drive(world, data, ENEMY, role);
}

/// Wertet den Stand der Schlacht aus: `Some(true)` = Spieler hat gesiegt.
pub fn battle_outcome(world: &SimWorld, req: &BattleRequest, player_attacks: bool) -> Option<bool> {
    let alive = |owner: u8| {
        world
            .units
            .iter()
            .any(|(_, u)| u.owner == owner && u.hp > 0.0)
    };
    let (mine, theirs) = (alive(PLAYER), alive(ENEMY));
    if !theirs {
        return Some(true);
    }
    if !mine {
        return Some(false);
    }
    if req.siege {
        let enemy_keep_dead = world.mission.dead_tags.iter().any(|t| t == "enemy_keep");
        let own_keep_dead = world.mission.dead_tags.iter().any(|t| t == "player_keep");
        if enemy_keep_dead || own_keep_dead {
            // Wer den Bergfried verlor, unterliegt (der Verteidiger).
            return Some(if enemy_keep_dead {
                player_attacks
            } else {
                !player_attacks
            });
        }
    }
    if world.time >= TIME_LIMIT {
        if req.siege {
            return Some(!player_attacks);
        }
        let power = |o: u8| {
            world
                .units
                .iter()
                .filter(|(_, u)| u.owner == o)
                .map(|(_, u)| u.hp)
                .sum::<f32>()
        };
        return Some(power(PLAYER) >= power(ENEMY));
    }
    None
}

/// Verluste je strategischer Einheit (Name, vorher, nachher) – für den Ergebnisdialog.
#[derive(Clone, Debug, Default)]
pub struct LossRow {
    pub unit: String,
    pub before: u32,
    pub after: u32,
}

/// Auswertung einer beendeten Schlacht.
pub struct BattleReport {
    pub result: BattleResult,
    pub player_won: bool,
    pub my_losses: Vec<LossRow>,
    pub enemy_losses: Vec<LossRow>,
    pub hero_notes: Vec<(u32, HeroOutcome)>,
}

fn add_row(rows: &mut Vec<LossRow>, unit: &str, before: u32, after: u32) {
    let name = catalog::unit(unit)
        .map(|u| u.name.to_string())
        .unwrap_or_else(|| unit.to_string());
    if let Some(r) = rows.iter_mut().find(|r| r.unit == name) {
        r.before += before;
        r.after += after;
    } else {
        rows.push(LossRow {
            unit: name,
            before,
            after,
        });
    }
}

/// Übersetzt den Endzustand der Echtzeitschlacht in ein `BattleResult` (Verluste je Regiment, Sieger, Heldenschicksal).
pub fn build_result(
    world: &SimWorld,
    req: &BattleRequest,
    scale: u32,
    player: u16,
    player_won: bool,
    permadeath: bool,
) -> BattleReport {
    let player_attacks = req.attackers.iter().any(|b| b.realm == Some(player));
    let winner = if player_won == player_attacks {
        BattleSide::Attackers
    } else {
        BattleSide::Defenders
    };
    let mut armies = Vec::new();
    let mut heroes = Vec::new();
    let (mut my_losses, mut enemy_losses, mut hero_notes) = (Vec::new(), Vec::new(), Vec::new());
    for (is_att, list) in [(true, &req.attackers), (false, &req.defenders)] {
        let mine = is_att == player_attacks;
        let side_won = is_att == (winner == BattleSide::Attackers);
        for b in list.iter() {
            let mut regs: Vec<Regiment> = Vec::new();
            for (i, r) in b.regiments.iter().enumerate() {
                let tag = tag_of(b.army, i);
                let spawned = rts_count(r.count, scale);
                let alive: Vec<&ashen_sim::entity::Unit> = world
                    .units
                    .iter()
                    .filter(|(_, u)| u.tag.as_deref() == Some(tag.as_str()))
                    .map(|(_, u)| u)
                    .collect();
                let n = alive.len() as u32;
                let survivors = if r.count == 0 {
                    0
                } else if n >= spawned {
                    r.count
                } else {
                    ((r.count as f32 * n as f32 / spawned.max(1) as f32).round() as u32)
                        .min(r.count)
                };
                let hp = if alive.is_empty() {
                    r.hp
                } else {
                    (alive.iter().map(|u| u.hp / u.max_hp.max(1.0)).sum::<f32>()
                        / alive.len() as f32)
                        .clamp(0.2, 1.0)
                };
                add_row(
                    if mine {
                        &mut my_losses
                    } else {
                        &mut enemy_losses
                    },
                    &r.unit,
                    r.count,
                    survivors,
                );
                if survivors > 0 {
                    regs.push(Regiment {
                        unit: r.unit.clone(),
                        count: survivors,
                        vet: r.vet,
                        hp,
                    });
                }
            }
            let any_left = !regs.is_empty();
            armies.push(ArmyResult {
                army: b.army,
                regiments: regs,
                retreated: !side_won && any_left,
            });
            for c in &b.heroes {
                let t = hero_tag(*c);
                let unit = world
                    .units
                    .iter()
                    .find(|(_, u)| u.tag.as_deref() == Some(t.as_str()))
                    .map(|(_, u)| u);
                let outcome = match unit {
                    None if permadeath => HeroOutcome::Dead,
                    None => HeroOutcome::Wounded,
                    Some(u) if !side_won && u.hp < u.max_hp * 0.35 => HeroOutcome::Captured,
                    Some(u) if u.hp < u.max_hp * 0.5 || !side_won => HeroOutcome::Wounded,
                    Some(_) => HeroOutcome::Unharmed,
                };
                if mine {
                    hero_notes.push((*c, outcome));
                }
                heroes.push((*c, outcome));
            }
        }
    }
    let keep_dead = world
        .mission
        .dead_tags
        .iter()
        .any(|t| t == "enemy_keep" || t == "player_keep");
    let city_taken = req.siege && winner == BattleSide::Attackers && keep_dead;
    let result = BattleResult {
        id: req.id,
        winner,
        armies,
        heroes,
        duration_minutes: (world.time / 60.0).max(0.5),
        city_taken,
    };
    BattleReport {
        result,
        player_won,
        my_losses,
        enemy_losses,
        hero_notes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ashen_strategy::{ArmyKind, BattleMode, StrategyOptions, StrategyWorld};

    fn data() -> GameData {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/data");
        GameData::load_dir(&dir).expect("Spieldaten")
    }

    fn pending(seed: u64) -> (StrategyWorld, BattleRequest, u32) {
        let mut opts = StrategyOptions::campaign(seed);
        opts.rules.battle_mode = BattleMode::PlayerBattlesRequested;
        let mut w = StrategyWorld::new(opts);
        let me = w.player.unwrap();
        let hollow = w.realm_by_key("hollowed").unwrap();
        let prov = w.prov_by_key("kaltherd").unwrap();
        let mine = w.new_army(
            ArmyKind::Realm,
            Some(me),
            prov,
            vec![
                Regiment {
                    unit: "swordsman".into(),
                    count: 60,
                    vet: 1,
                    hp: 1.0,
                },
                Regiment {
                    unit: "archer".into(),
                    count: 40,
                    vet: 0,
                    hp: 1.0,
                },
                Regiment {
                    unit: "knight".into(),
                    count: 12,
                    vet: 0,
                    hp: 1.0,
                },
            ],
            "Test-Heer",
        );
        let _enemy = w.new_army(
            ArmyKind::Realm,
            Some(hollow),
            prov,
            vec![
                Regiment {
                    unit: "hollow_thrall".into(),
                    count: 70,
                    vet: 0,
                    hp: 1.0,
                },
                Regiment {
                    unit: "grave_brute".into(),
                    count: 4,
                    vet: 0,
                    hp: 1.0,
                },
            ],
            "Hohle",
        );
        w.tick_day();
        let req = w
            .pending_battles()
            .first()
            .cloned()
            .expect("Schlachtanfrage");
        (w, req, mine)
    }

    #[test]
    fn mapping_table_covers_all_strategic_units() {
        let data = data();
        for u in catalog::UNITS {
            let row = UNIT_TABLE
                .iter()
                .find(|(s, _, _)| *s == u.id)
                .unwrap_or_else(|| panic!("strategische Einheit {} fehlt in UNIT_TABLE", u.id));
            assert!(
                data.unit_id(row.1).is_some(),
                "Ziel {} von {} existiert nicht in units.ron",
                row.1,
                u.id
            );
            assert!(data.unit_id(&resolve_unit(&data, u.id)).is_some());
        }
        for (s, _, _) in UNIT_TABLE {
            assert!(
                catalog::unit(s).is_some(),
                "Tabelle verweist auf unbekannte strategische ID {s}"
            );
        }
        // Unbekannte (Mod-)IDs fallen auf eine Standardeinheit zurück.
        assert!(
            data.unit_id(&resolve_unit(&data, "mod_unbekannt"))
                .is_some()
        );
    }

    #[test]
    fn battle_request_becomes_valid_mission() {
        let data = data();
        let (w, req, _) = pending(606);
        let me = w.player.unwrap();
        let plan = plan_battle(&data, me, &req).expect("Plan");
        assert!(
            plan.spawned.0 > 0 && plan.spawned.1 > 0,
            "beide Seiten haben Truppen"
        );
        assert!(
            plan.mission.objectives.iter().any(|o| !o.text.is_empty()),
            "Ziele vorhanden"
        );
        assert!(
            plan.mission.placements.iter().any(|p| p.owner == PLAYER)
                && plan.mission.placements.iter().any(|p| p.owner == ENEMY)
        );
        let world = create_world(&data, plan, &req).expect("Welt");
        let mine = world
            .units
            .iter()
            .filter(|(_, u)| u.owner == PLAYER)
            .count();
        let theirs = world.units.iter().filter(|(_, u)| u.owner == ENEMY).count();
        assert!(mine > 0 && theirs > 0);
        assert!(
            world
                .units
                .iter()
                .all(|(_, u)| world.map.walkable(TileMap::tile_of(u.pos))),
            "alle Einheiten stehen auf begehbaren Kacheln"
        );
        assert!(mine as u32 <= MAX_UNITS_PER_SIDE + 10 && theirs as u32 <= MAX_UNITS_PER_SIDE + 10);
    }

    #[test]
    fn siege_battle_builds_fortress_with_breaches() {
        let data = data();
        let (w, mut req, _) = pending(607);
        let me = w.player.unwrap();
        req.siege = true;
        req.fortification = 6;
        req.breaches = 2;
        let mut intact = req.clone();
        intact.breaches = 0;
        let plain = plan_battle(&data, me, &intact).unwrap();
        let plan = plan_battle(&data, me, &req).unwrap();
        let count =
            |p: &BattlePlan, id: &str| p.mission.placements.iter().filter(|x| x.id == id).count();
        assert!(count(&plan, "wall") > 20, "Mauerring vorhanden");
        assert!(
            count(&plan, "gate") >= 1
                && count(&plan, "wall_tower") >= 4
                && count(&plan, "keep") == 1
        );
        assert!(
            count(&plain, "wall") > count(&plan, "wall"),
            "Breschen entfernen Mauerstücke"
        );
        let world = create_world(&data, plan, &req).expect("Belagerungswelt");
        assert!(
            world.building_with_tag("enemy_keep").is_some()
                || world.building_with_tag("player_keep").is_some()
        );
    }

    #[test]
    fn played_battle_roundtrips_into_strategy() {
        let data = data();
        let (mut w, req, mine_army) = pending(606);
        let me = w.player.unwrap();
        let before = w.army(mine_army).unwrap().size();
        let plan = plan_battle(&data, me, &req).unwrap();
        let (scale, attacks) = (plan.scale, plan.player_attacks);
        let mut world = create_world(&data, plan, &req).unwrap();
        // Spieler-Einheiten greifen an (stellvertretend für die Spielereingaben).
        let foe = Vec2::new(53.0, 53.0);
        let ids: Vec<_> = world
            .units
            .iter()
            .filter(|(_, u)| u.owner == PLAYER)
            .map(|(i, _)| i)
            .collect();
        for id in ids {
            if let Some(u) = world.units.get_mut(id) {
                u.order = Order::Move {
                    target: foe,
                    attack_move: true,
                };
                u.request_path(foe);
            }
        }
        let mut verdict = None;
        for tick in 0..40_000u32 {
            world.step(&data);
            if tick % 40 == 0 {
                drive_enemy(&mut world, &data, attacks);
            }
            if let Some(v) = battle_outcome(&world, &req, attacks) {
                verdict = Some(v);
                break;
            }
        }
        let player_won = verdict.expect("Schlacht endet");
        let report = build_result(&world, &req, scale, me, player_won, true);
        assert_eq!(report.result.id, req.id);
        assert!(
            report.my_losses.iter().any(|r| r.after < r.before)
                || report.enemy_losses.iter().any(|r| r.after < r.before),
            "es gab Verluste"
        );
        let expected: u32 = report
            .result
            .armies
            .iter()
            .filter(|a| a.army == mine_army)
            .flat_map(|a| a.regiments.iter())
            .map(|r| r.count)
            .sum();
        w.apply_battle_result(report.result.clone())
            .expect("Ergebnis wird übernommen");
        assert!(w.pending_battles().is_empty());
        if let Some(a) = w.army(mine_army) {
            assert!(a.size() <= before, "Verluste werden angewendet");
            assert!(a.in_battle.is_none());
            assert!(
                a.size() <= expected.max(1),
                "Überlebende entsprechen dem Ergebnis"
            );
        }
    }

    #[test]
    fn terrain_follows_biome() {
        let (_, req, _) = pending(608);
        for biome in [
            Biome::Plains,
            Biome::Forest,
            Biome::Mountain,
            Biome::Swamp,
            Biome::Desert,
            Biome::Coast,
            Biome::Island,
            Biome::Hollowland,
            Biome::Volcanic,
            Biome::Tundra,
            Biome::Ruins,
            Biome::Hills,
        ] {
            let mut r = req.clone();
            r.terrain = biome;
            let m = battle_terrain(&r, 7);
            let walk = (0..BATTLE_MAP)
                .flat_map(|y| (0..BATTLE_MAP).map(move |x| IVec2::new(x, y)))
                .filter(|t| m.walkable(*t))
                .count();
            assert!(
                walk as f32 > (BATTLE_MAP * BATTLE_MAP) as f32 * 0.5,
                "{biome:?}: genug begehbare Fläche ({walk})"
            );
        }
        let mut desert = req.clone();
        desert.terrain = Biome::Desert;
        assert!(
            battle_terrain(&desert, 3)
                .terrain
                .iter()
                .filter(|t| **t == Terrain::Sand)
                .count()
                > 500
        );
    }
}
