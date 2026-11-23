//! Schmiede der mitgelieferten Szenarien: legt die handentworfenen Karten und Missionen der
//! historischen Szenarien als RON unter `assets/data/scenarios/` ab.
//!
//! Jede Karte beginnt mit der Gelände-Vorlage des Generators und wird anschließend von Hand
//! umgestaltet (Wälle, Flüsse, Lichtungen, Lager). `cargo run -p ashen_tools -- forge-scenarios`
//! erzeugt die Dateien reproduzierbar; sie können danach im Karteneditor weiter bearbeitet werden.

use ashen_sim::entity::NodeKind;
use ashen_sim::genmap::{generate_preset, GenPreset};
use ashen_sim::map::{DecorKind, Terrain};
use ashen_sim::mission::{Action, Condition, ObjectiveDef, Placement, TriggerDef, WaveRules};
use ashen_sim::data::{Cost, GameData};
use ashen_sim::mission::MissionDef;
use ashen_sim::scenario::{AreaDef, DecorPlace, MapData, NodePlace, StartPos};
use glam::Vec2;
use std::path::Path;

/// Bearbeitbare Karte beim Entwurf.
struct Canvas {
    w: i32,
    h: i32,
    terrain: Vec<Terrain>,
    decor: Vec<DecorPlace>,
    nodes: Vec<NodePlace>,
}

impl Canvas {
    fn new(w: i32, h: i32, seed: u64, preset: GenPreset) -> Canvas {
        let g = generate_preset(w, h, seed, preset);
        let mut c = Canvas { w, h, terrain: g.map.terrain_vec(), decor: g.map.decor, nodes: g.map.nodes };
        // Feste Minen und Steinbrüche des Generators entfallen; sie werden von Hand gesetzt.
        c.nodes.retain(|n| n.kind == NodeKind::Tree);
        c
    }

    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        (x >= 0 && y >= 0 && x < self.w && y < self.h).then(|| (y * self.w + x) as usize)
    }

    fn set(&mut self, x: i32, y: i32, t: Terrain) {
        if let Some(i) = self.idx(x, y) {
            self.terrain[i] = t;
        }
    }

    /// Fläche (inklusive) leeren: Bäume, Dekoration und Terrain.
    fn flatten(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, t: Terrain) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                self.set(x, y, t);
            }
        }
        self.clear(x0, y0, x1, y1);
    }

    fn clear(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        self.nodes.retain(|n| !(n.x >= x0 && n.x <= x1 && n.y >= y0 && n.y <= y1));
        self.decor.retain(|d| {
            let (tx, ty) = (d.x.floor() as i32, d.y.floor() as i32);
            !(tx >= x0 && tx <= x1 && ty >= y0 && ty <= y1)
        });
    }

    fn disc(&mut self, cx: f32, cy: f32, r: f32, t: Terrain) {
        for y in 0..self.h {
            for x in 0..self.w {
                if (Vec2::new(x as f32 + 0.5, y as f32 + 0.5) - Vec2::new(cx, cy)).length() <= r {
                    self.set(x, y, t);
                }
            }
        }
        self.clear((cx - r) as i32, (cy - r) as i32, (cx + r) as i32 + 1, (cy + r) as i32 + 1);
    }

    /// Breiter Streifen entlang einer Linie.
    fn path(&mut self, pts: &[(f32, f32)], half: f32, t: Terrain) {
        for win in pts.windows(2) {
            let (a, b) = (Vec2::new(win[0].0, win[0].1), Vec2::new(win[1].0, win[1].1));
            let steps = ((b - a).length() * 2.0) as i32 + 1;
            for i in 0..=steps {
                let p = a.lerp(b, i as f32 / steps as f32);
                for y in (p.y - half - 1.0) as i32..=(p.y + half + 1.0) as i32 {
                    for x in (p.x - half - 1.0) as i32..=(p.x + half + 1.0) as i32 {
                        if (Vec2::new(x as f32 + 0.5, y as f32 + 0.5) - p).length() <= half + 0.5 {
                            self.set(x, y, t);
                            self.clear(x, y, x, y);
                        }
                    }
                }
            }
        }
    }

    fn free(&self, x: i32, y: i32, size: i32) -> bool {
        (0..size).all(|dy| (0..size).all(|dx| self.idx(x + dx, y + dy).is_some_and(|i| matches!(self.terrain[i], Terrain::Grass | Terrain::DarkGrass | Terrain::Dirt | Terrain::Stone | Terrain::Ash))))
    }

    /// Setzt eine Mine oder einen Steinbruch am nächsten freien Platz (Spirale um den Wunschort).
    fn node(&mut self, kind: NodeKind, x: i32, y: i32) {
        let size = kind.size();
        for r in 0..16_i32 {
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx.abs().max(dy.abs()) != r || !self.free(x + dx, y + dy, size.x) {
                        continue;
                    }
                    let (px, py) = (x + dx, y + dy);
                    self.flatten_keep_terrain(px, py, px + size.x - 1, py + size.y - 1);
                    self.nodes.push(NodePlace { kind, x: px, y: py, amount: 0, variant: 0 });
                    return;
                }
            }
        }
        panic!("kein Platz für {kind:?} bei {x},{y}");
    }

    fn flatten_keep_terrain(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        self.clear(x0 - 1, y0 - 1, x1 + 1, y1 + 1);
    }

    fn deco(&mut self, kind: DecorKind, x: f32, y: f32) {
        self.decor.push(DecorPlace { kind, x, y, variant: ((x * 7.0 + y * 3.0) as u32 % 8) as u8 });
    }

    /// Lichter Wald in einem Rechteck (deterministisch, ohne Zufallsquelle).
    fn forest(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, step: i32) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                let h = (x.wrapping_mul(73856093) ^ y.wrapping_mul(19349663)).rem_euclid(97);
                if (x + y * 2) % step == 0 && h < 70 && self.idx(x, y).is_some_and(|i| self.terrain[i].buildable()) && !self.nodes.iter().any(|n| n.x == x && n.y == y) {
                    self.nodes.push(NodePlace { kind: NodeKind::Tree, x, y, amount: 0, variant: (h % 6) as u8 });
                }
            }
        }
    }

    /// Räumt Bäume unter allen Gebäuden der Aufstellung (mit einer Kachel Rand) und schließt die Karte ab.
    fn finish_with(mut self, placements: &[Placement], data: &GameData) -> MapData {
        for p in placements {
            if let Some(b) = data.building_id(&p.id) {
                let d = data.building(b);
                let (x, y) = (p.x as i32, p.y as i32);
                self.clear(x - 1, y - 1, x + d.size.0 as i32, y + d.size.1 as i32);
            }
        }
        self.finish()
    }

    fn finish(self) -> MapData {
        let (w, h) = (self.w, self.h);
        let terrain = self.terrain;
        let t_at = |x: i32, y: i32| terrain.get((y * w + x) as usize).copied().filter(|_| x >= 0 && y >= 0 && x < w && y < h);
        // Quellen nur auf bebaubarem Gelände (keine Bäume im Wasser oder auf Straßen).
        let nodes: Vec<NodePlace> = self
            .nodes
            .into_iter()
            .filter(|n| {
                let s = n.kind.size();
                (0..s.y).all(|dy| (0..s.x).all(|dx| t_at(n.x + dx, n.y + dy).is_some_and(|t| t.buildable() && t != Terrain::Road)))
            })
            .collect();
        let mut decor: Vec<DecorPlace> = self.decor.into_iter().filter(|d| t_at(d.x.floor() as i32, d.y.floor() as i32).is_some_and(|t| t.walkable())).collect();
        // Dekoration unter Quellen entfernen.
        decor.retain(|d| !nodes.iter().any(|n| (d.x.floor() as i32) == n.x && (d.y.floor() as i32) == n.y));
        MapData { width: w, height: h, terrain: MapData::rows_from(w, h, &terrain), decor, nodes }
    }
}

fn pl(id: &str, owner: u8, x: f32, y: f32) -> Placement {
    Placement { id: id.into(), owner, x, y, count: 0, tag: None, guard: false }
}

fn group(id: &str, owner: u8, x: f32, y: f32, count: u32) -> Placement {
    Placement { id: id.into(), owner, x, y, count, tag: None, guard: false }
}

fn guard(id: &str, owner: u8, x: f32, y: f32, count: u32) -> Placement {
    Placement { id: id.into(), owner, x, y, count, tag: None, guard: true }
}

fn tagged(mut p: Placement, tag: &str) -> Placement {
    p.tag = Some(tag.into());
    p
}

fn area(id: &str, name: &str, x: i32, y: i32, w: i32, h: i32) -> AreaDef {
    AreaDef { id: id.into(), name: name.into(), x, y, w, h }
}

fn obj(id: &str, text: &str, optional: bool) -> ObjectiveDef {
    ObjectiveDef { id: id.into(), text: text.into(), optional }
}

fn msg(speaker: &str, text: &str) -> Action {
    Action::Message { speaker: speaker.into(), text: text.into(), voice: None }
}

fn trig(id: &str, cond: Condition, actions: Vec<Action>) -> TriggerDef {
    TriggerDef { id: id.into(), condition: cond, actions, modes: vec![] }
}

fn spawn(unit: &str, area_id: &str, count: u32) -> Action {
    Action::SpawnUnits { unit: unit.into(), owner: 1, area: area_id.into(), count, attack: true }
}

/// Eine Mauerlinie als Platzierungen (inklusive beider Endkacheln).
fn wall_ring(out: &mut Vec<Placement>, x0: i32, y0: i32, x1: i32, y1: i32, gates: &[(i32, i32)], towers: &[(i32, i32)]) {
    let in_tower = |x: i32, y: i32| towers.iter().any(|(tx, ty)| x >= *tx && x < tx + 2 && y >= *ty && y < ty + 2);
    for y in y0..=y1 {
        for x in x0..=x1 {
            if x != x0 && x != x1 && y != y0 && y != y1 {
                continue;
            }
            if in_tower(x, y) {
                continue;
            }
            if gates.contains(&(x, y)) {
                out.push(pl("gate", 0, x as f32, y as f32));
            } else {
                out.push(pl("wall", 0, x as f32, y as f32));
            }
        }
    }
    for (tx, ty) in towers {
        out.push(pl("wall_tower", 0, *tx as f32, *ty as f32));
    }
}

fn base_mission(id: &str, name: &str, year: u32, description: &str, briefing: &str) -> MissionDef {
    MissionDef {
        id: id.into(),
        name: name.into(),
        description: description.into(),
        briefing: briefing.into(),
        author: "Ashen Crowns".into(),
        year: Some(year),
        waves: WaveRules { enabled: false, ..Default::default() },
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------------------------
// 893 n.F. – Der Fall von Grauwall
// ---------------------------------------------------------------------------------------------

fn grauwall(data: &GameData) -> MissionDef {
    let mut c = Canvas::new(64, 64, 893, GenPreset::Highlands);
    // Der Bergfried von Grauwall steht auf einem Felsplateau; draußen Felder und Gräberhügel.
    c.flatten(14, 14, 50, 50, Terrain::Dirt);
    c.flatten(20, 20, 42, 42, Terrain::Stone);
    c.path(&[(42.0, 31.0), (58.0, 31.0), (63.0, 34.0)], 1.2, Terrain::Road);
    // Schneisen durch den Waldgürtel: dort strömen die Hohlen heran.
    c.path(&[(3.0, 32.0), (20.0, 32.0)], 2.5, Terrain::Dirt);
    c.path(&[(32.0, 3.0), (32.0, 20.0)], 2.5, Terrain::Dirt);
    c.path(&[(32.0, 60.0), (32.0, 42.0)], 2.5, Terrain::Dirt);
    c.node(NodeKind::GoldMine, 46, 12);
    c.node(NodeKind::StoneQuarry, 8, 44);
    c.forest(2, 2, 12, 60, 3);
    c.forest(52, 44, 62, 62, 3);
    for (x, y) in [(15.5, 17.5), (47.5, 18.5), (16.5, 46.5), (48.5, 47.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    for (x, y) in [(12.5, 30.5), (13.5, 33.5), (50.5, 26.5), (51.5, 36.5)] {
        c.deco(DecorKind::Grave, x, y);
    }
    let mut m = base_mission(
        "sc_fall_of_grauwall",
        "Der Fall von Grauwall",
        893,
        "Eine einzige Nacht, eine Flut von Hohlen und eine Festung, die zweieinhalb Jahrhunderte hielt. Haltet die Mauern, bis die Flüchtlinge den Hügel hinter sich haben.",
        "Grauwall, Stammfeste von Haus Raventhorn, Jahr 893 n.F. Seit dem Ergrauen des Südens kommen die Toten aus der Aschmark. Heute Nacht sind es Hunderte. Markgraf Edran Raventhorn hat befohlen, die Tore zu schließen und die Mauern zu halten, bis der Zug der Flüchtlinge das Hügelland erreicht hat. Ihr führt die Garnison unter Isolde Raventhorn. Tauscht Schaden gegen Zeit: Haltet Mauern und Bergfried bis zum Morgengrauen. Es ist die Nacht, in der Edran auf den Mauern fiel – wie sie endet, hängt von Euch ab.",
    );
    m.map_seed = 893;
    m.start_resources = Cost { gold: 500.0, wood: 300.0, stone: 250.0, food: 400.0 };
    m.start_goods = vec![("weapons".into(), 16.0), ("armor".into(), 10.0), ("planks".into(), 20.0), ("tools".into(), 6.0)];
    m.defeat_tag = "player_keep".into();
    m.start_time = Some(0.66);
    let mut p = vec![
        tagged(pl("keep", 0, 30.0, 30.0), "player_keep"),
        pl("barracks", 0, 24.0, 24.0),
        pl("farmstead", 0, 35.0, 24.0),
        pl("farmstead", 0, 35.0, 36.0),
        pl("hut", 0, 24.0, 36.0),
        pl("hut", 0, 28.0, 37.0),
        pl("watchtower", 0, 38.0, 31.0),
        group("worker", 0, 33.0, 34.5, 6),
        group("swordsman", 0, 27.0, 34.0, 8),
        group("spearman", 0, 25.0, 31.0, 4),
        group("archer", 0, 36.5, 29.0, 6),
        tagged(pl("hero_isolde", 0, 33.0, 31.5), "hero"),
    ];
    wall_ring(&mut p, 20, 20, 42, 42, &[(42, 31)], &[(20, 20), (41, 20), (20, 41), (41, 41)]);
    m.placements = p;
    m.areas = vec![
        area("flut_west", "Westflut", 2, 28, 5, 8),
        area("flut_nord", "Nordflut", 28, 2, 8, 5),
        area("flut_sued", "Südflut", 28, 57, 8, 5),
        area("hoehe", "Die Mauerkrone", 20, 20, 23, 23),
    ];
    m.starts = vec![StartPos { owner: 0, x: 31.0, y: 33.0 }, StartPos { owner: 1, x: 4.0, y: 32.0 }];
    m.objectives = vec![
        obj("obj_hold", "Haltet Grauwall bis zum Morgengrauen.", false),
        obj("obj_keep", "Der Bergfried darf nicht fallen.", false),
        obj("obj_brutes", "Zerschlagt die Grabkolosse, bevor sie die Mauer einreißen.", true),
    ];
    m.triggers = vec![
        trig("intro", Condition::AtTime(1.0), vec![
            Action::AddObjective("obj_hold".into()),
            Action::AddObjective("obj_keep".into()),
            msg("Markgraf Edran Raventhorn", "Die Tore sind verriegelt, Isolde. Haltet die Mauer, bis die Letzten den Hügel erreicht haben – dann folgt Ihr ihnen. Das ist ein Befehl Eures Vaters, nicht des Markgrafen."),
            Action::Chronicle("In der Nacht des Falls hielt die Garnison von Grauwall die Mauerkrone.".into()),
        ]),
        trig("fog", Condition::AtTime(20.0), vec![Action::SetWeather(ashen_sim::env::Weather::Ashfall)]),
        trig("w1", Condition::AtTime(30.0), vec![msg("Isolde Raventhorn", "Sie kommen von Westen! Bogenschützen auf die Mauer!"), spawn("hollow_thrall", "flut_west", 10), spawn("hollow_archer", "flut_west", 2)]),
        trig("w2", Condition::AtTime(75.0), vec![spawn("hollow_thrall", "flut_nord", 12), spawn("hollow_archer", "flut_nord", 4)]),
        trig("w3", Condition::AtTime(115.0), vec![msg("Erzähler", "Aus dem Süden drang das Knirschen schwerer Schritte. Etwas Großes trug die Toten wie ein Hirte seine Herde."), spawn("grave_brute", "flut_sued", 2), spawn("hollow_thrall", "flut_sued", 10)]),
        trig("w4", Condition::AtTime(150.0), vec![spawn("hollow_thrall", "flut_west", 14), spawn("hollow_archer", "flut_west", 6), spawn("grave_brute", "flut_nord", 2)]),
        trig("w5", Condition::AtTime(185.0), vec![msg("Markgraf Edran Raventhorn", "Zum letzten Mal, Isolde: Haltet den Bergfried!"), spawn("hollow_thrall", "flut_sued", 10), spawn("hollow_archer", "flut_nord", 4), spawn("hollow_thrall", "flut_west", 10), spawn("hollow_archer", "flut_west", 2)]),
        trig("w6", Condition::AtTime(170.0), vec![spawn("grave_brute", "flut_sued", 2), spawn("hollow_thrall", "flut_nord", 12), spawn("hollow_archer", "flut_sued", 5)]),
        trig("dawn", Condition::DayStarted(2), vec![
            Action::CompleteObjective("obj_hold".into()),
            Action::CompleteObjective("obj_keep".into()),
            msg("Erzähler", "Als das Licht kam, standen noch Männer auf der Mauer. Der Zug der Flüchtlinge war über den Hügel. Grauwall aber war verloren – und mit ihm Edran Raventhorn, der nicht mehr vom Wall herabstieg."),
            Action::Chronicle("Grauwall fiel in einer einzigen Nacht; seine Garnison verschaffte den Flüchtlingen den Vorsprung.".into()),
            Action::Victory,
        ]),
    ];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// 894 n.F. – Die Flucht nach Vael Dorn
// ---------------------------------------------------------------------------------------------

fn vael_dorn_flight(data: &GameData) -> MissionDef {
    let mut c = Canvas::new(112, 112, 894, GenPreset::Heartland);
    // Zwei Flüsse sperren das Land; nur Furten führen hinüber. Die Straße windet sich von der
    // Dorfruine im Südwesten über beide Furten zum Kloster im Nordosten.
    let river1 = [(0.0, 74.0), (30.0, 80.0), (56.0, 96.0), (70.0, 112.0)];
    let river2 = [(40.0, 0.0), (52.0, 26.0), (60.0, 50.0), (96.0, 62.0), (112.0, 60.0)];
    for r in [&river1[..], &river2[..]] {
        c.path(r, 3.6, Terrain::Sand);
        c.path(r, 2.4, Terrain::Water);
        c.path(r, 1.4, Terrain::DeepWater);
    }
    c.disc(30.0, 80.0, 3.2, Terrain::Ford);
    c.disc(58.0, 40.0, 3.2, Terrain::Ford);
    let road = [(12.0, 98.0), (20.0, 90.0), (30.0, 80.0), (36.0, 66.0), (44.0, 56.0), (50.0, 50.0), (58.0, 40.0), (68.0, 32.0), (78.0, 34.0), (88.0, 28.0), (94.0, 18.0), (98.0, 14.0)];
    c.path(&road, 1.0, Terrain::Road);
    c.flatten(4, 86, 22, 108, Terrain::Dirt);
    c.flatten(88, 6, 108, 26, Terrain::Stone);
    c.flatten(26, 56, 33, 63, Terrain::Grass);
    c.node(NodeKind::GoldMine, 70, 70);
    c.node(NodeKind::StoneQuarry, 46, 86);
    // Klosterruine von Vael Dorn.
    for (x, y) in [(90.5, 8.5), (104.5, 8.5), (90.5, 24.5), (104.5, 24.5), (97.5, 6.5)] {
        c.deco(DecorKind::Pillar, x, y);
    }
    for (x, y) in [(88.5, 14.5), (106.5, 16.5), (94.5, 26.5), (100.5, 10.5)] {
        c.deco(DecorKind::Ruin, x, y);
    }
    // Brennende Dorfreste und Spuren der Hohlen.
    for (x, y) in [(6.5, 90.5), (9.5, 100.5), (14.5, 88.5), (17.5, 104.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    for (x, y) in [(36.5, 64.5), (62.5, 36.5), (80.5, 36.5)] {
        c.deco(DecorKind::Bones, x, y);
    }
    let mut m = base_mission(
        "sc_flight_to_vael_dorn",
        "Die Flucht nach Vael Dorn",
        894,
        "Das Dorf brennt, die Hohlen sind dicht hinter Euch. Führt die Flüchtlinge durch die Aschmark über zwei Furten zu den alten Klostermauern von Vael Dorn.",
        "Aschmark, Jahr 894 n.F. Ein Jahr nach dem Fall von Grauwall überrennen die Hohlen die Dörfer der Mark. Vogt Aldous Meringer, ein pedantischer, ehrlicher Beamter, hat sich zum Anführer der Überlebenden gemacht: Nur die dicken Mauern des aurethischen Klosters Vael Dorn versprechen Schutz. Ihr führt die Flüchtlinge und ihre kleine Eskorte unter Isolde Raventhorn. Gebäude gibt es nicht zu verteidigen – nur Menschen. Folgt der Straße, überquert beide Furten, brecht die Wachposten der Hohlen und bringt mindestens zwölf Seelen hinter die Klostermauern. Verliert Ihr mehr als acht Flüchtlinge, ist die Flucht gescheitert. Die Hohlen setzen nach – bleibt in Bewegung.",
    );
    m.map_seed = 894;
    m.start_resources = Cost { gold: 0.0, wood: 0.0, stone: 0.0, food: 300.0 };
    m.defeat_tag = String::new();
    m.start_time = Some(0.62);
    m.placements = vec![
        group("worker", 0, 10.0, 98.0, 16),
        group("swordsman", 0, 15.5, 94.0, 4),
        group("spearman", 0, 13.0, 92.0, 3),
        group("archer", 0, 9.5, 94.0, 3),
        tagged(pl("hero_isolde", 0, 16.5, 95.5), "hero"),
        // Wachposten und Lager der Hohlen entlang der Straße.
        guard("hollow_thrall", 1, 31.5, 79.0, 4),
        guard("hollow_archer", 1, 29.0, 81.5, 2),
        guard("hollow_thrall", 1, 44.0, 57.0, 3),
        guard("hollow_archer", 1, 46.0, 59.5, 1),
        guard("hollow_thrall", 1, 58.5, 41.0, 5),
        guard("hollow_archer", 1, 56.0, 43.0, 2),
        guard("grave_brute", 1, 60.5, 38.5, 1),
        guard("hollow_thrall", 1, 78.5, 34.0, 4),
        guard("hollow_archer", 1, 80.0, 31.0, 2),
        guard("grave_brute", 1, 86.0, 28.0, 1),
        guard("hollow_thrall", 1, 93.0, 20.5, 5),
        guard("hollow_archer", 1, 95.5, 18.0, 3),
        guard("hollow_thrall", 1, 40.0, 20.0, 4),
    ];
    m.areas = vec![
        area("dorf", "Das brennende Dorf", 4, 86, 18, 22),
        area("vael_dorn", "Klostermauern von Vael Dorn", 90, 8, 16, 16),
        area("furt1", "Die untere Furt", 27, 77, 7, 7),
        area("furt2", "Die obere Furt", 55, 37, 7, 7),
        area("falle1", "Hinterhalt im Wald", 38, 62, 6, 6),
        area("falle1_spawn", "Hinterhalt im Wald (Flanke)", 28, 58, 4, 4),
        area("falle2", "Hinterhalt am Hohlweg", 72, 28, 6, 6),
        area("falle2_spawn", "Hinterhalt am Hohlweg (Flanke)", 76, 20, 4, 4),
    ];
    m.starts = vec![StartPos { owner: 0, x: 12.0, y: 95.0 }];
    m.objectives = vec![
        obj("obj_reach", "Führt mindestens zwölf Seelen nach Vael Dorn.", false),
        obj("obj_keep_refugees", "Verliert nicht mehr als acht Flüchtlinge.", false),
        obj("obj_ford1", "Überquert den Fluss an der unteren Furt.", true),
        obj("obj_ford2", "Überquert den zweiten Fluss an der oberen Furt.", true),
    ];
    m.triggers = vec![
        trig("intro", Condition::AtTime(1.0), vec![
            Action::AddObjective("obj_reach".into()),
            Action::AddObjective("obj_keep_refugees".into()),
            Action::AddObjective("obj_ford1".into()),
            msg("Vogt Aldous Meringer", "Weiter, weiter! Die Klostermauern von Vael Dorn halten, hat man mir gesagt. Nach Nordosten, der alten Straße nach – und nicht stehenbleiben!"),
            Action::Chronicle("Vogt Meringer führte die Überlebenden der Dörfer in Richtung Vael Dorn.".into()),
        ]),
        trig("pursuit1", Condition::AtTime(40.0), vec![msg("Isolde Raventhorn", "Hinter uns! Sie holen auf – nicht umdrehen!"), spawn("hollow_thrall", "dorf", 8)]),
        trig("pursuit2", Condition::AtTime(100.0), vec![spawn("hollow_thrall", "dorf", 6), spawn("hollow_archer", "dorf", 2)]),
        trig("pursuit3", Condition::AtTime(170.0), vec![spawn("hollow_thrall", "dorf", 8), spawn("grave_brute", "dorf", 1)]),
        trig("ford1", Condition::UnitsInArea { area: "furt1".into(), count: 3 }, vec![
            Action::CompleteObjective("obj_ford1".into()),
            Action::AddObjective("obj_ford2".into()),
            msg("Vogt Aldous Meringer", "Das Wasser ist kalt, aber es hält sie nicht auf. Weiter!"),
        ]),
        trig("ambush1", Condition::UnitsInArea { area: "falle1".into(), count: 1 }, vec![msg("Isolde Raventhorn", "Hinterhalt im Unterholz! Schließt auf!"), spawn("hollow_thrall", "falle1_spawn", 3)]),
        trig("ford2", Condition::UnitsInArea { area: "furt2".into(), count: 3 }, vec![Action::CompleteObjective("obj_ford2".into()), msg("Erzähler", "Hinter dem zweiten Fluss lag die Aschmark still. Zu still.")]),
        trig("ambush2", Condition::UnitsInArea { area: "falle2".into(), count: 1 }, vec![spawn("hollow_thrall", "falle2_spawn", 3), spawn("hollow_archer", "falle2_spawn", 1)]),
        trig("lost", Condition::Not(Box::new(Condition::UnitCount { unit: "worker".into(), count: 8 })), vec![
            msg("Erzähler", "Zu wenige erreichten das Kloster. Die Stille der Aschmark schloss sich über ihnen."),
            Action::Defeat,
        ]),
        trig("arrived", Condition::UnitsInArea { area: "vael_dorn".into(), count: 12 }, vec![
            Action::CompleteObjective("obj_reach".into()),
            Action::CompleteObjective("obj_keep_refugees".into()),
            msg("Vogt Aldous Meringer", "Die Mauern halten! Schließt das Tor, bringt die Verwundeten ins Innere. Hier sind wir sicher."),
            msg("Erzähler", "Er irrte sich. Aber das wusste an diesem Abend noch niemand."),
            Action::Chronicle("Die Flüchtlinge erreichten die Ruinen von Vael Dorn. Im Frühjahr kehrte keiner von ihnen zurück.".into()),
            Action::Victory,
        ]),
    ];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// 896 n.F. – Die Belagerung von Kaltherd
// ---------------------------------------------------------------------------------------------

fn kaltherd(data: &GameData) -> MissionDef {
    let mut c = Canvas::new(72, 72, 896, GenPreset::Aschmark);
    c.flatten(22, 22, 50, 50, Terrain::Grass);
    c.flatten(26, 26, 46, 46, Terrain::Dirt);
    c.path(&[(36.0, 50.0), (36.0, 70.0)], 1.2, Terrain::Road);
    c.path(&[(2.0, 36.0), (26.0, 36.0)], 1.2, Terrain::Road);
    c.path(&[(36.0, 2.0), (36.0, 22.0)], 1.2, Terrain::Road);
    c.path(&[(50.0, 36.0), (70.0, 36.0)], 1.2, Terrain::Road);
    c.node(NodeKind::GoldMine, 52, 24);
    c.node(NodeKind::StoneQuarry, 20, 52);
    c.node(NodeKind::StoneQuarry, 54, 52);
    c.forest(14, 14, 24, 30, 2);
    c.forest(48, 30, 58, 42, 3);
    for (x, y) in [(28.5, 25.5), (44.5, 25.5), (28.5, 47.5), (44.5, 47.5)] {
        c.deco(DecorKind::Lantern, x, y);
    }
    for (x, y) in [(33.5, 33.5), (39.5, 33.5)] {
        c.deco(DecorKind::Crops, x, y);
    }
    let mut m = base_mission(
        "sc_siege_of_kaltherd",
        "Die Belagerung von Kaltherd",
        896,
        "Kaltherd ist eines der letzten bewohnten Dörfer der Aschmark. Zwei Nächte lang werden die Hohlen es berennen. Ein Tag bleibt, die Palisade zu errichten.",
        "Kaltherd, Aschmark, Jahr 896 n.F. Hunger, Fieber und die Wellen aus Vael Dorn haben die Dörfer der Mark entvölkert. Kaltherd steht noch – dreißig Familien, ein Dorfhaus, ein Gehöft und ein Waisenjunge, der Zeichnungen in die Asche malt. Isolde Raventhorn hat eine Handvoll Soldaten gebracht und etwas Stein. Bis zur ersten Nacht bleibt Zeit, eine Mauer mit Tor zu ziehen und Türme zu setzen. Haltet das Dorf durch zwei Nächte; am dritten Morgen soll ein Boten-Trupp aus Brennhollow eintreffen.",
    );
    m.map_seed = 896;
    m.start_resources = Cost { gold: 350.0, wood: 400.0, stone: 420.0, food: 350.0 };
    m.start_goods = vec![("weapons".into(), 12.0), ("armor".into(), 6.0), ("planks".into(), 30.0), ("tools".into(), 6.0)];
    m.defeat_tag = "player_keep".into();
    m.placements = vec![
        tagged(pl("keep", 0, 34.0, 33.0), "player_keep"),
        pl("barracks", 0, 28.0, 28.0),
        pl("farmstead", 0, 40.0, 30.0),
        pl("farmstead", 0, 40.0, 40.0),
        pl("hut", 0, 29.0, 40.0),
        pl("hut", 0, 33.0, 41.0),
        pl("hut", 0, 37.0, 41.0),
        pl("woodcutter", 0, 25.0, 33.0),
        group("worker", 0, 33.5, 37.5, 8),
        group("swordsman", 0, 31.0, 36.0, 5),
        group("spearman", 0, 36.0, 37.0, 4),
        group("archer", 0, 38.0, 35.0, 5),
        tagged(pl("hero_isolde", 0, 35.5, 36.5), "hero"),
    ];
    m.areas = vec![
        area("ost", "Östlicher Weg", 62, 32, 6, 8),
        area("west", "Westlicher Weg", 2, 32, 6, 8),
        area("nord", "Nördlicher Weg", 32, 2, 8, 6),
        area("sued", "Südlicher Weg", 32, 62, 8, 6),
        area("dorf", "Kaltherd", 24, 24, 24, 24),
    ];
    m.starts = vec![StartPos { owner: 0, x: 35.0, y: 36.0 }, StartPos { owner: 1, x: 66.0, y: 36.0 }];
    m.objectives = vec![
        obj("obj_wall", "Errichtet eine Mauer mit Tor um Kaltherd.", true),
        obj("obj_night1", "Übersteht die erste Nacht.", false),
        obj("obj_night2", "Übersteht die zweite Nacht.", false),
        obj("obj_hall", "Der Bergfried darf nicht fallen.", false),
    ];
    m.triggers = vec![
        trig("intro", Condition::AtTime(1.0), vec![
            Action::AddObjective("obj_night1".into()),
            Action::AddObjective("obj_hall".into()),
            Action::AddObjective("obj_wall".into()),
            msg("Isolde Raventhorn", "Wir haben bis zum Abend Zeit. Stein und Holz liegen im Lager – zieht die Mauer, setzt ein Tor nach Süden, und stellt Türme an die Ecken."),
        ]),
        trig("wall_done", Condition::BuildingCompleted("gate".into()), vec![Action::CompleteObjective("obj_wall".into())]),
        trig("n1a", Condition::AtTime(190.0), vec![msg("Erzähler", "Der erste Nebel stieg aus der Asche. Man hörte sie, bevor man sie sah."), spawn("hollow_thrall", "ost", 8), spawn("hollow_archer", "ost", 2)]),
        trig("n1b", Condition::AtTime(240.0), vec![spawn("hollow_thrall", "west", 8), spawn("hollow_archer", "west", 3)]),
        trig("n1c", Condition::AtTime(290.0), vec![spawn("hollow_thrall", "nord", 8), spawn("grave_brute", "nord", 1)]),
        trig("n1_done", Condition::DayStarted(2), vec![
            Action::CompleteObjective("obj_night1".into()),
            Action::AddObjective("obj_night2".into()),
            Action::Grant(Cost { gold: 120.0, wood: 120.0, stone: 160.0, food: 0.0 }),
            msg("Isolde Raventhorn", "Eine Nacht überstanden. Flickt die Mauer. Die nächste wird schlimmer."),
        ]),
        trig("n2a", Condition::AtTime(550.0), vec![spawn("hollow_thrall", "ost", 10), spawn("hollow_archer", "ost", 3), spawn("grave_brute", "ost", 1)]),
        trig("n2b", Condition::AtTime(600.0), vec![spawn("hollow_thrall", "sued", 10), spawn("hollow_archer", "sued", 4)]),
        trig("n2c", Condition::AtTime(650.0), vec![msg("Erzähler", "Mit den Toten kam etwas Größeres: Gestalten, die Mauern nicht fürchteten."), spawn("grave_brute", "west", 2), spawn("hollow_thrall", "west", 10), spawn("hollow_archer", "nord", 4)]),
        trig("victory", Condition::DayStarted(3), vec![
            Action::CompleteObjective("obj_night2".into()),
            Action::CompleteObjective("obj_hall".into()),
            msg("Erzähler", "Am dritten Morgen standen die Mauern von Kaltherd noch. Die Hohlen aber hatten sich in die Asche zurückgezogen – für diesmal."),
            Action::Chronicle("Kaltherd überstand zwei Nächte der Belagerung.".into()),
            Action::Victory,
        ]),
    ];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// 898 n.F. – Das Banner von Grauwall
// ---------------------------------------------------------------------------------------------

fn banner(data: &GameData) -> MissionDef {
    let mut c = Canvas::new(80, 80, 898, GenPreset::Ashlands);
    c.flatten(6, 52, 26, 74, Terrain::Grass);
    c.flatten(50, 10, 74, 34, Terrain::Ash);
    c.path(&[(24.0, 60.0), (34.0, 52.0), (44.0, 42.0), (54.0, 30.0), (60.0, 24.0)], 1.0, Terrain::Road);
    c.node(NodeKind::GoldMine, 8, 46);
    c.node(NodeKind::StoneQuarry, 28, 48);
    c.node(NodeKind::GoldMine, 40, 14);
    c.forest(4, 60, 10, 78, 2);
    // Ruine von Grauwall: eingestürzte Mauern aus Pfeilern und Ruinen.
    for i in 0..9 {
        c.deco(DecorKind::Pillar, 52.5 + i as f32 * 2.0, 12.5);
        c.deco(DecorKind::Pillar, 52.5 + i as f32 * 2.0, 32.5);
    }
    for (x, y) in [(51.5, 22.5), (71.5, 20.5), (58.5, 16.5)] {
        c.deco(DecorKind::Ruin, x, y);
    }
    for (x, y) in [(60.5, 20.5), (62.5, 22.5), (64.5, 18.5), (66.5, 24.5)] {
        c.deco(DecorKind::Grave, x, y);
    }
    let mut m = base_mission(
        "sc_banner_of_grauwall",
        "Das Banner von Grauwall",
        898,
        "Fünf Jahre nach dem Fall kehrt Isolde Raventhorn zurück. In den Ruinen von Grauwall wandelt ihr Vater unter den Hohlen – und ein Schrein hält die Festung in Stille.",
        "Grauwall, Jahr 898 n.F. Der Fahle Vogt ist gefallen, Brennhollow atmet auf. Doch Grauwall, die Stammfeste von Haus Raventhorn, ist eine stille, hohle Ruine, deren Banner noch an den Mauern hängt. Späher berichten von einem neuen Hohlschrein im Hof und von einer Gestalt in Markgrafenrüstung, die der Schrein lenkt. Isolde führt eine Kolonne aus Brennhollow: ein Lager im Südwesten, eine Kaserne, eine Hand voll Soldaten. Baut Euer Lager aus, bildet Truppen aus, zerstört den Schrein und bringt den Grabkoloss in Edrans Rüstung zur Ruhe. Die Hohlen schicken Wellen, solange der Schrein steht.",
    );
    m.map_seed = 898;
    m.start_resources = Cost { gold: 500.0, wood: 350.0, stone: 200.0, food: 350.0 };
    m.start_goods = vec![("weapons".into(), 24.0), ("armor".into(), 14.0), ("planks".into(), 20.0), ("tools".into(), 6.0)];
    m.defeat_tag = "player_keep".into();
    m.enemy_base_tag = "grauwall_shrine".into();
    m.waves = WaveRules { enabled: true, first_wave: 330.0, interval_day: 190.0, interval_night: 140.0, base_budget: 2.5, budget_per_wave: 1.2, night_mult: 1.3 };
    m.placements = vec![
        tagged(pl("keep", 0, 12.0, 62.0), "player_keep"),
        pl("barracks", 0, 18.0, 58.0),
        pl("farmstead", 0, 16.0, 68.0),
        pl("hut", 0, 8.0, 58.0),
        pl("hut", 0, 8.0, 66.0),
        pl("woodcutter", 0, 6.0, 72.0),
        group("worker", 0, 17.0, 63.0, 6),
        group("swordsman", 0, 20.5, 64.0, 6),
        group("spearman", 0, 21.5, 61.0, 4),
        group("archer", 0, 19.0, 66.0, 5),
        group("knight", 0, 22.5, 66.5, 2),
        tagged(pl("hero_isolde", 0, 18.5, 62.5), "hero"),
        // Grauwall in Hohler Hand.
        tagged(pl("hollow_shrine", 1, 62.0, 18.0), "grauwall_shrine"),
        pl("bone_spire", 1, 56.0, 22.0),
        pl("bone_spire", 1, 68.0, 22.0),
        pl("bone_spire", 1, 56.0, 14.0),
        tagged(guard("grave_brute", 1, 62.5, 22.5, 1), "edran"),
        guard("hollow_thrall", 1, 58.5, 20.5, 5),
        guard("hollow_thrall", 1, 66.5, 18.5, 4),
        guard("hollow_archer", 1, 60.0, 14.5, 4),
        // Vorposten auf dem Weg.
        guard("hollow_thrall", 1, 40.0, 44.0, 3),
        guard("hollow_archer", 1, 42.0, 46.0, 2),
        guard("hollow_thrall", 1, 46.5, 28.5, 4),
    ];
    m.areas = vec![area("lager", "Lager der Kolonne", 6, 54, 20, 20), area("grauwall", "Die Ruinen von Grauwall", 50, 10, 24, 24)];
    m.starts = vec![StartPos { owner: 0, x: 17.0, y: 62.0 }, StartPos { owner: 1, x: 62.0, y: 18.0 }];
    m.objectives = vec![
        obj("obj_shrine", "Zerstört den Hohlschrein von Grauwall.", false),
        obj("obj_edran", "Bringt den Grabkoloss in Edrans Rüstung zur Ruhe.", false),
        obj("obj_army", "Bildet eine Streitmacht von mindestens zwölf Soldaten aus.", true),
    ];
    m.triggers = vec![
        trig("intro", Condition::AtTime(1.0), vec![
            Action::AddObjective("obj_shrine".into()),
            Action::AddObjective("obj_edran".into()),
            Action::AddObjective("obj_army".into()),
            msg("Isolde Raventhorn", "Fünf Jahre. Das Banner hängt noch. Vater, ich bringe Euch heim – ob Ihr wollt oder nicht."),
        ]),
        trig("army", Condition::MilitaryCount(12), vec![Action::CompleteObjective("obj_army".into())]),
        trig("edran_dead", Condition::TagDead("edran".into()), vec![Action::CompleteObjective("obj_edran".into()), msg("Isolde Raventhorn", "Ruht, Vater. Die Wache ist abgelöst.")]),
        trig("shrine_dead", Condition::TagDead("grauwall_shrine".into()), vec![Action::CompleteObjective("obj_shrine".into())]),
        trig("victory", Condition::All(vec![Condition::TagDead("edran".into()), Condition::TagDead("grauwall_shrine".into())]), vec![
            msg("Erzähler", "Auf den Mauern von Grauwall wehte wieder das Banner der Raventhorns – schwarzer Rabe auf rotem Dornenzweig."),
            Action::Chronicle("Isolde Raventhorn eroberte Grauwall zurück und bestattete ihren Vater.".into()),
            Action::Victory,
        ]),
    ];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

/// Alle mitgelieferten Szenarien (Dateiname, Definition).
pub fn scenarios(data: &GameData) -> Vec<(&'static str, MissionDef)> {
    vec![("s1_fall_of_grauwall.ron", grauwall(data)), ("s2_flight_to_vael_dorn.ron", vael_dorn_flight(data)), ("s3_siege_of_kaltherd.ron", kaltherd(data)), ("s4_banner_of_grauwall.ron", banner(data))]
}

/// Schreibt alle Szenarien nach `<assets>/data/scenarios/`.
pub fn forge(assets: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let dir = assets.join("data/scenarios");
    let data = GameData::load_dir(&assets.join("data")).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for (file, def) in scenarios(&data) {
        let path = dir.join(file);
        ashen_sim::scenario::save(&def, &path)?;
        out.push(path);
    }
    Ok(out)
}
