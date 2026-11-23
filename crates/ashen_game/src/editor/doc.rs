//! Dokument des Karteneditors: bearbeitbare Karte und Mission, Pinsel und Rückgängig/Wiederholen.
//!
//! Das Dokument ist die einzige Wahrheit; die Vorschau-Welt wird bei Bedarf daraus gebaut.
//! Gespeichert wird als gewöhnliche [`MissionDef`] mit expliziter Karte, die jede Spielinstanz
//! direkt starten kann.

use ashen_sim::SimWorld;
use ashen_sim::data::{Cost, GameData, UnitClass};
use ashen_sim::entity::NodeKind;
use ashen_sim::genmap::{GenPreset, generate_preset};
use ashen_sim::map::{DecorKind, Terrain, TileMap};
use ashen_sim::mission::{
    Action, Condition, GameMode, MatchSettings, MissionDef, ObjectiveDef, Placement, TriggerDef,
    WaveRules,
};
use ashen_sim::scenario::{DecorPlace, MapData, NodePlace, StartPos};
use glam::{IVec2, Vec2};

/// Bearbeitbare Kartendaten.
#[derive(Clone, Debug, PartialEq)]
pub struct EditMap {
    pub width: i32,
    pub height: i32,
    pub terrain: Vec<Terrain>,
    pub decor: Vec<DecorPlace>,
    pub nodes: Vec<NodePlace>,
}

impl EditMap {
    pub fn idx(&self, x: i32, y: i32) -> Option<usize> {
        (x >= 0 && y >= 0 && x < self.width && y < self.height)
            .then(|| (y * self.width + x) as usize)
    }

    pub fn terrain_at(&self, t: IVec2) -> Option<Terrain> {
        self.idx(t.x, t.y).map(|i| self.terrain[i])
    }

    fn from_data(m: &MapData) -> EditMap {
        EditMap {
            width: m.width,
            height: m.height,
            terrain: m.terrain_vec(),
            decor: m.decor.clone(),
            nodes: m.nodes.clone(),
        }
    }

    fn to_data(&self) -> MapData {
        MapData {
            width: self.width,
            height: self.height,
            terrain: MapData::rows_from(self.width, self.height, &self.terrain),
            decor: self.decor.clone(),
            nodes: self.nodes.clone(),
        }
    }
}

/// Form des Pinsels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrushShape {
    Square,
    Circle,
    Diamond,
}

impl BrushShape {
    pub const ALL: [BrushShape; 3] = [BrushShape::Circle, BrushShape::Square, BrushShape::Diamond];

    pub fn label(self) -> &'static str {
        match self {
            BrushShape::Circle => "Rund",
            BrushShape::Square => "Quadrat",
            BrushShape::Diamond => "Raute",
        }
    }

    /// Kacheln, die ein Pinsel der Größe `size` (1..=12) um `center` überdeckt.
    pub fn tiles(self, center: IVec2, size: i32) -> Vec<IVec2> {
        let size = size.clamp(1, 24);
        let half = size as f32 / 2.0;
        let lo = -(size / 2);
        let mut out = Vec::new();
        for dy in lo..lo + size {
            for dx in lo..lo + size {
                // Mittelpunkt der Kachel relativ zur Pinselmitte (gerade Größen liegen zwischen Kacheln).
                let shift = if size % 2 == 0 { 0.5 } else { 0.0 };
                let (ox, oy) = (dx as f32 + shift, dy as f32 + shift);
                let inside = match self {
                    BrushShape::Square => true,
                    BrushShape::Circle => ox * ox + oy * oy <= half * half + 0.01,
                    BrushShape::Diamond => ox.abs() + oy.abs() <= half + 0.01,
                };
                if inside {
                    out.push(center + IVec2::new(dx, dy));
                }
            }
        }
        out
    }
}

/// Das gesamte Editor-Dokument.
#[derive(Clone, Debug)]
pub struct EditDoc {
    /// Missionsdaten; `map` bleibt hier leer und wird beim Speichern aus `map` erzeugt.
    pub def: MissionDef,
    pub map: EditMap,
}

/// Fußabdruck eines Objekts (Gebäude oder Einheiten) für Auswahl und Kollision.
pub fn footprint(data: &GameData, p: &Placement) -> Option<(IVec2, IVec2)> {
    data.building_id(&p.id).map(|b| {
        let d = data.building(b);
        (
            IVec2::new(p.x as i32, p.y as i32),
            IVec2::new(d.size.0 as i32, d.size.1 as i32),
        )
    })
}

impl EditDoc {
    /// Neue, sofort spielbare Karte aus dem Generator: Bergfried und Trupp für die Spielerseite,
    /// Hohlschrein mit Wachen für den Gegner, Siegbedingung „Schrein zerstören“.
    pub fn generated(width: i32, height: i32, seed: u64, preset: GenPreset) -> EditDoc {
        let g = generate_preset(width, height, seed, preset);
        let map = EditMap::from_data(&g.map);
        let mut doc = EditDoc {
            def: MissionDef::default(),
            map,
        };
        let (p, e) = (g.starts[0], g.starts[1]);
        doc.def.id = "eigene_karte".into();
        doc.def.name = "Neue Karte".into();
        doc.def.description = "Eine selbst gestaltete Karte.".into();
        doc.def.map_seed = seed;
        doc.def.author = "Spieler".into();
        doc.def.start_resources = Cost {
            gold: 300.0,
            wood: 200.0,
            stone: 100.0,
            food: 150.0,
        };
        doc.def.start_goods = vec![
            ("weapons".into(), 10.0),
            ("armor".into(), 4.0),
            ("tools".into(), 4.0),
            ("planks".into(), 20.0),
        ];
        doc.def.waves = WaveRules::default();
        doc.def.defeat_tag = "player_keep".into();
        doc.def.enemy_base_tag = "enemy_shrine".into();
        doc.def.starts = vec![
            StartPos {
                owner: 0,
                x: p.x,
                y: p.y,
            },
            StartPos {
                owner: 1,
                x: e.x,
                y: e.y,
            },
        ];
        let tag = |mut pl: Placement, t: &str| {
            pl.tag = Some(t.into());
            pl
        };
        let mk = |id: &str, owner: u8, x: f32, y: f32, count: u32, guard: bool| Placement {
            id: id.into(),
            owner,
            x: x.floor(),
            y: y.floor(),
            count,
            tag: None,
            guard,
            camp: None,
        };
        doc.def.placements = vec![
            tag(mk("keep", 0, p.x - 2.0, p.y - 2.0, 0, false), "player_keep"),
            mk("farmstead", 0, p.x + 2.0, p.y + 2.0, 0, false),
            mk("hut", 0, p.x - 6.0, p.y + 2.0, 0, false),
            Placement {
                id: "worker".into(),
                owner: 0,
                x: p.x + 2.5,
                y: p.y - 1.5,
                count: 5,
                tag: None,
                guard: false,
                camp: None,
            },
            Placement {
                id: "swordsman".into(),
                owner: 0,
                x: p.x + 4.5,
                y: p.y,
                count: 3,
                tag: None,
                guard: false,
                camp: None,
            },
            tag(
                Placement {
                    id: "hero_isolde".into(),
                    owner: 0,
                    x: p.x + 3.5,
                    y: p.y - 2.5,
                    count: 0,
                    tag: None,
                    guard: false,
                    camp: None,
                },
                "hero",
            ),
            tag(
                mk("hollow_shrine", 1, e.x - 2.0, e.y - 2.0, 0, false),
                "enemy_shrine",
            ),
            mk("bone_spire", 1, e.x - 6.0, e.y, 0, false),
            mk("bone_spire", 1, e.x + 2.0, e.y + 4.0, 0, false),
            Placement {
                id: "hollow_thrall".into(),
                owner: 1,
                x: e.x - 3.0,
                y: e.y + 3.0,
                count: 4,
                tag: None,
                guard: true,
                camp: None,
            },
            Placement {
                id: "hollow_archer".into(),
                owner: 1,
                x: e.x + 1.0,
                y: e.y - 4.0,
                count: 3,
                tag: None,
                guard: true,
                camp: None,
            },
        ];
        doc.def.objectives = vec![ObjectiveDef {
            id: "obj_shrine".into(),
            text: "Zerstört den Hohlschrein.".into(),
            optional: false,
        }];
        doc.def.triggers = vec![
            TriggerDef {
                id: "intro".into(),
                condition: Condition::AtTime(1.0),
                actions: vec![Action::AddObjective("obj_shrine".into())],
                modes: vec![],
            },
            TriggerDef {
                id: "sieg".into(),
                condition: Condition::TagDead("enemy_shrine".into()),
                actions: vec![
                    Action::CompleteObjective("obj_shrine".into()),
                    Action::Message {
                        speaker: "Erzähler".into(),
                        text: "Der Schrein ist gefallen.".into(),
                        voice: None,
                    },
                    Action::Victory,
                ],
                modes: vec![],
            },
        ];
        doc.clear_sites();
        doc
    }

    /// Leere Karte aus einem einzigen Gelände.
    pub fn blank(width: i32, height: i32, fill: Terrain) -> EditDoc {
        let (w, h) = (width.clamp(16, 160), height.clamp(16, 160));
        let mut doc = EditDoc {
            def: MissionDef::default(),
            map: EditMap {
                width: w,
                height: h,
                terrain: vec![fill; (w * h) as usize],
                decor: vec![],
                nodes: vec![],
            },
        };
        doc.def.id = "eigene_karte".into();
        doc.def.name = "Leere Karte".into();
        doc.def.description = "Eine leere Karte.".into();
        doc.def.author = "Spieler".into();
        doc.def.start_resources = Cost {
            gold: 300.0,
            wood: 200.0,
            stone: 100.0,
            food: 150.0,
        };
        doc.def.starts = vec![StartPos {
            owner: 0,
            x: w as f32 * 0.25,
            y: h as f32 * 0.75,
        }];
        doc
    }

    /// Bestehende Mission oder Szenario übernehmen. Missionen ohne explizite Karte werden dafür
    /// einmal erzeugt (inklusive Bäumen und Minen) und als feste Karte übernommen.
    pub fn from_mission(data: &GameData, def: &MissionDef) -> Result<EditDoc, String> {
        let mut def = def.clone();
        let map = match def.map.take() {
            Some(m) => {
                m.validate()?;
                EditMap::from_data(&m)
            }
            None => {
                let w = SimWorld::new(
                    data,
                    def.clone(),
                    MatchSettings {
                        mode: GameMode::Sandbox,
                        ..Default::default()
                    },
                )?;
                let nodes = w
                    .nodes
                    .iter()
                    .map(|(_, n)| NodePlace {
                        kind: n.kind,
                        x: n.tile.x,
                        y: n.tile.y,
                        amount: n.max_amount,
                        variant: n.variant,
                    })
                    .collect();
                let decor = w
                    .map
                    .decor
                    .iter()
                    .map(|d| DecorPlace {
                        kind: d.kind,
                        x: d.pos.x,
                        y: d.pos.y,
                        variant: d.variant,
                    })
                    .collect();
                EditMap {
                    width: w.map.width,
                    height: w.map.height,
                    terrain: w.map.terrain.clone(),
                    decor,
                    nodes,
                }
            }
        };
        let mut doc = EditDoc { def, map };
        if doc.def.starts.is_empty() {
            let keep = doc
                .def
                .placements
                .iter()
                .find(|p| p.owner == 0 && p.id == "keep");
            let (x, y) = keep
                .map(|k| (k.x + 1.5, k.y + 1.5))
                .unwrap_or((doc.map.width as f32 * 0.25, doc.map.height as f32 * 0.75));
            doc.def.starts.push(StartPos { owner: 0, x, y });
        }
        Ok(doc)
    }

    /// Fertige Mission mit expliziter Karte.
    pub fn to_mission(&self) -> MissionDef {
        let mut d = self.def.clone();
        d.map = Some(self.map.to_data());
        d
    }

    /// Räumt Bäume und blockierende Dekoration unter allen Gebäuden und Quellen der Platzierungen.
    fn clear_sites(&mut self) {
        let boxes: Vec<(IVec2, IVec2)> = self
            .def
            .placements
            .iter()
            .filter(|p| {
                p.id == "keep"
                    || p.id == "hollow_shrine"
                    || p.id == "farmstead"
                    || p.id == "hut"
                    || p.id == "bone_spire"
            })
            .map(|p| (IVec2::new(p.x as i32 - 1, p.y as i32 - 1), IVec2::new(5, 5)))
            .collect();
        self.map.nodes.retain(|n| {
            !boxes
                .iter()
                .any(|(o, s)| n.x >= o.x && n.y >= o.y && n.x < o.x + s.x && n.y < o.y + s.y)
        });
        self.map.decor.retain(|d| {
            !boxes.iter().any(|(o, s)| {
                (d.x as i32) >= o.x
                    && (d.y as i32) >= o.y
                    && (d.x as i32) < o.x + s.x
                    && (d.y as i32) < o.y + s.y
            })
        });
    }

    // ------------------------------------------------------------------ Gelände

    /// Malt Gelände. Nicht begehbares Gelände entfernt Bäume, Quellen und Dekoration darauf;
    /// Straßen und Furten entfernen Bäume.
    pub fn paint_terrain(&mut self, tiles: &[IVec2], t: Terrain) {
        for &c in tiles {
            let Some(i) = self.map.idx(c.x, c.y) else {
                continue;
            };
            self.map.terrain[i] = t;
            if !t.walkable() || !t.buildable() {
                self.map.nodes.retain(|n| {
                    let size = n.kind.size();
                    !(c.x >= n.x && c.y >= n.y && c.x < n.x + size.x && c.y < n.y + size.y)
                });
            }
            if !t.walkable() {
                self.map
                    .decor
                    .retain(|d| TileMap::tile_of(Vec2::new(d.x, d.y)) != c);
            }
        }
    }

    /// Flutfüllung zusammenhängenden gleichen Geländes (4er-Nachbarschaft), höchstens `limit` Kacheln.
    pub fn flood_fill(&mut self, start: IVec2, t: Terrain, limit: usize) -> usize {
        let Some(src) = self.map.terrain_at(start) else {
            return 0;
        };
        if src == t {
            return 0;
        }
        let mut stack = vec![start];
        let mut seen = std::collections::HashSet::new();
        let mut area = Vec::new();
        while let Some(c) = stack.pop() {
            if area.len() >= limit
                || !seen.insert((c.x, c.y))
                || self.map.terrain_at(c) != Some(src)
            {
                continue;
            }
            area.push(c);
            for d in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y] {
                stack.push(c + d);
            }
        }
        let n = area.len();
        self.paint_terrain(&area, t);
        n
    }

    // ------------------------------------------------------------------ Objekte

    fn node_rect(n: &NodePlace) -> (IVec2, IVec2) {
        (IVec2::new(n.x, n.y), n.kind.size())
    }

    /// Platziert Quelle (Baum, Goldmine, Steinbruch); ersetzt überdeckte Bäume und Dekoration.
    pub fn place_node(&mut self, kind: NodeKind, t: IVec2, variant: u8) -> Result<(), String> {
        let size = kind.size();
        for y in 0..size.y {
            for x in 0..size.x {
                let c = t + IVec2::new(x, y);
                match self.map.terrain_at(c) {
                    None => return Err("Außerhalb der Karte.".into()),
                    Some(tr) if !tr.buildable() => {
                        return Err(format!("{} ist hier nicht möglich.", kind.label()));
                    }
                    _ => {}
                }
            }
        }
        let new = (t, size);
        let overlaps = |a: (IVec2, IVec2), b: (IVec2, IVec2)| {
            a.0.x < b.0.x + b.1.x
                && b.0.x < a.0.x + a.1.x
                && a.0.y < b.0.y + b.1.y
                && b.0.y < a.0.y + a.1.y
        };
        if self
            .map
            .nodes
            .iter()
            .any(|n| n.kind != NodeKind::Tree && overlaps(Self::node_rect(n), new))
        {
            return Err("Dort steht bereits eine Quelle.".into());
        }
        self.map
            .nodes
            .retain(|n| !overlaps(Self::node_rect(n), new));
        self.map.decor.retain(|d| {
            let dt = TileMap::tile_of(Vec2::new(d.x, d.y));
            !overlaps((dt, IVec2::ONE), new)
        });
        self.map.nodes.push(NodePlace {
            kind,
            x: t.x,
            y: t.y,
            amount: 0,
            variant,
        });
        Ok(())
    }

    pub fn place_decor(&mut self, kind: DecorKind, p: Vec2, variant: u8) -> Result<(), String> {
        let t = TileMap::tile_of(p);
        match self.map.terrain_at(t) {
            None => return Err("Außerhalb der Karte.".into()),
            Some(tr) if !tr.walkable() => return Err("Auf Wasser kann nichts stehen.".into()),
            _ => {}
        }
        if self.map.nodes.iter().any(|n| {
            let size = n.kind.size();
            t.x >= n.x && t.y >= n.y && t.x < n.x + size.x && t.y < n.y + size.y
        }) {
            return Err("Dort steht eine Ressourcenquelle.".into());
        }
        // Blockierende Dekoration liegt mittig, freie darf verschoben stehen.
        let pos = if kind.blocks() {
            TileMap::center_of(t)
        } else {
            p
        };
        self.map
            .decor
            .retain(|d| TileMap::tile_of(Vec2::new(d.x, d.y)) != t);
        self.map.decor.push(DecorPlace {
            kind,
            x: pos.x,
            y: pos.y,
            variant,
        });
        Ok(())
    }

    /// Prüft, ob ein Gebäude hier stehen darf (Gelände, Quellen, Dekoration, andere Gebäude).
    pub fn can_place_building(
        &self,
        data: &GameData,
        id: &str,
        tile: IVec2,
        ignore: Option<usize>,
    ) -> Result<(), String> {
        let Some(b) = data.building_id(id) else {
            return Err(format!("Unbekanntes Gebäude {id}."));
        };
        let d = data.building(b);
        let size = IVec2::new(d.size.0 as i32, d.size.1 as i32);
        for y in 0..size.y {
            for x in 0..size.x {
                let c = tile + IVec2::new(x, y);
                match self.map.terrain_at(c) {
                    None => return Err("Außerhalb der Karte.".into()),
                    Some(t) if !t.buildable() => {
                        return Err(format!("Auf {} kann nicht gebaut werden.", t.label()));
                    }
                    _ => {}
                }
                if self.map.nodes.iter().any(|n| {
                    let s = n.kind.size();
                    c.x >= n.x && c.y >= n.y && c.x < n.x + s.x && c.y < n.y + s.y
                }) {
                    return Err("Dort steht eine Ressourcenquelle oder ein Baum.".into());
                }
                if self
                    .map
                    .decor
                    .iter()
                    .any(|dc| dc.kind.blocks() && TileMap::tile_of(Vec2::new(dc.x, dc.y)) == c)
                {
                    return Err("Dekoration blockiert die Stelle.".into());
                }
            }
        }
        for (i, p) in self.def.placements.iter().enumerate() {
            if Some(i) == ignore {
                continue;
            }
            if let Some((o, s)) = footprint(data, p)
                && tile.x < o.x + s.x
                && o.x < tile.x + size.x
                && tile.y < o.y + s.y
                && o.y < tile.y + size.y
            {
                return Err("Dort steht schon ein Gebäude.".into());
            }
        }
        Ok(())
    }

    pub fn can_place_unit(&self, p: Vec2) -> Result<(), String> {
        let t = TileMap::tile_of(p);
        match self.map.terrain_at(t) {
            None => Err("Außerhalb der Karte.".into()),
            Some(tr) if !tr.walkable() => Err("Einheiten können nicht im Wasser stehen.".into()),
            _ => Ok(()),
        }
    }

    /// Platziert ein Gebäude oder eine Einheitengruppe.
    pub fn place_object(
        &mut self,
        data: &GameData,
        id: &str,
        owner: u8,
        p: Vec2,
        count: u32,
    ) -> Result<usize, String> {
        if data.building_id(id).is_some() {
            let tile = IVec2::new(p.x.floor() as i32, p.y.floor() as i32);
            self.can_place_building(data, id, tile, None)?;
            self.def.placements.push(Placement {
                id: id.into(),
                owner,
                x: tile.x as f32,
                y: tile.y as f32,
                count: 0,
                tag: None,
                guard: false,
                camp: None,
            });
        } else if data.unit_id(id).is_some() {
            self.can_place_unit(p)?;
            self.def.placements.push(Placement {
                id: id.into(),
                owner,
                x: (p.x * 2.0).round() / 2.0,
                y: (p.y * 2.0).round() / 2.0,
                count: count.max(1),
                tag: None,
                guard: false,
                camp: None,
            });
        } else {
            return Err(format!("Unbekannte ID {id}."));
        }
        Ok(self.def.placements.len() - 1)
    }

    /// Fußabdruck und Mitte der Platzierung `i`.
    pub fn placement_center(&self, data: &GameData, i: usize) -> Vec2 {
        let p = &self.def.placements[i];
        match footprint(data, p) {
            Some((o, s)) => o.as_vec2() + s.as_vec2() * 0.5,
            None => Vec2::new(p.x, p.y),
        }
    }

    // ------------------------------------------------------------------ Suche

    /// Platzierung (Gebäude/Einheit) unter dem Punkt.
    pub fn placement_at(&self, data: &GameData, p: Vec2) -> Option<usize> {
        // Einheiten zuerst: sie liegen über Gebäuden.
        let unit = self
            .def
            .placements
            .iter()
            .enumerate()
            .filter(|(_, pl)| {
                data.unit_id(&pl.id).is_some()
                    && (Vec2::new(pl.x, pl.y) - p).length() < 0.9 + (pl.count as f32).sqrt() * 0.3
            })
            .min_by(|a, b| {
                (Vec2::new(a.1.x, a.1.y) - p)
                    .length()
                    .total_cmp(&(Vec2::new(b.1.x, b.1.y) - p).length())
            })
            .map(|(i, _)| i);
        unit.or_else(|| {
            self.def.placements.iter().enumerate().find_map(|(i, pl)| {
                let (o, s) = footprint(data, pl)?;
                (p.x >= o.x as f32
                    && p.y >= o.y as f32
                    && p.x < (o.x + s.x) as f32
                    && p.y < (o.y + s.y) as f32)
                    .then_some(i)
            })
        })
    }

    pub fn node_at(&self, p: Vec2) -> Option<usize> {
        self.map.nodes.iter().position(|n| {
            let s = n.kind.size();
            p.x >= n.x as f32
                && p.y >= n.y as f32
                && p.x < (n.x + s.x) as f32
                && p.y < (n.y + s.y) as f32
        })
    }

    pub fn decor_at(&self, p: Vec2) -> Option<usize> {
        self.map
            .decor
            .iter()
            .enumerate()
            .filter(|(_, d)| (Vec2::new(d.x, d.y) - p).length() < 0.8)
            .min_by(|a, b| {
                (Vec2::new(a.1.x, a.1.y) - p)
                    .length()
                    .total_cmp(&(Vec2::new(b.1.x, b.1.y) - p).length())
            })
            .map(|(i, _)| i)
    }

    pub fn start_at(&self, p: Vec2) -> Option<usize> {
        self.def
            .starts
            .iter()
            .position(|s| (Vec2::new(s.x, s.y) - p).length() < 1.4)
    }

    pub fn area_at(&self, p: Vec2) -> Option<usize> {
        self.def.areas.iter().position(|a| a.contains(p))
    }

    /// Entfernt, was unter dem Punkt liegt (Einheit/Gebäude, dann Quelle, dann Dekoration).
    /// Gibt eine Beschreibung des Entfernten zurück.
    pub fn erase_at(&mut self, data: &GameData, p: Vec2) -> Option<String> {
        if let Some(i) = self.placement_at(data, p) {
            let pl = self.def.placements.remove(i);
            return Some(pl.id);
        }
        if let Some(i) = self.node_at(p) {
            let n = self.map.nodes.remove(i);
            return Some(n.kind.label().to_string());
        }
        if let Some(i) = self.decor_at(p) {
            let d = self.map.decor.remove(i);
            return Some(d.kind.label().to_string());
        }
        None
    }

    /// Bäume in einem Pinselbereich entfernen/hinzufügen (Waldpinsel).
    pub fn paint_trees(&mut self, tiles: &[IVec2], add: bool, density: f32, seed: u32) -> usize {
        let mut changed = 0;
        for &c in tiles {
            let Some(t) = self.map.terrain_at(c) else {
                continue;
            };
            let has = self
                .map
                .nodes
                .iter()
                .position(|n| n.kind == NodeKind::Tree && n.x == c.x && n.y == c.y);
            if add {
                let h =
                    (c.x.wrapping_mul(73856093) as u32 ^ c.y.wrapping_mul(19349663) as u32 ^ seed)
                        .wrapping_mul(2654435761)
                        >> 8;
                let r = (h % 1000) as f32 / 1000.0;
                if has.is_none()
                    && r < density
                    && t.buildable()
                    && self.place_node(NodeKind::Tree, c, (h % 6) as u8).is_ok()
                {
                    changed += 1;
                }
            } else if let Some(i) = has {
                self.map.nodes.remove(i);
                changed += 1;
            }
        }
        changed
    }

    pub fn counts(&self, data: &GameData) -> (usize, usize, usize) {
        let mut b = 0;
        let mut u = 0;
        for p in &self.def.placements {
            if data.building_id(&p.id).is_some() {
                b += 1;
            } else {
                u += p.count.max(1) as usize;
            }
        }
        (b, u, self.map.nodes.len())
    }

    /// Hat die Platzierung eine Heldenklasse?
    pub fn is_hero(data: &GameData, id: &str) -> bool {
        data.unit_id(id)
            .is_some_and(|u| data.unit(u).class == UnitClass::Hero)
    }

    /// Neue eindeutige ID mit Präfix für Trigger, Ziele und Bereiche.
    pub fn fresh_id(existing: &[&str], prefix: &str) -> String {
        (1..)
            .map(|n| format!("{prefix}_{n}"))
            .find(|id| !existing.contains(&id.as_str()))
            .unwrap_or_else(|| prefix.to_string())
    }

    /// Gültiger Bezeichner für Dateien und Missions-IDs.
    pub fn sanitize_id(s: &str) -> String {
        ashen_sim::scenario::slug(s)
    }
}

/// Rückgängig/Wiederholen über Schnappschüsse des Dokuments.
pub struct History {
    undo: Vec<EditDoc>,
    redo: Vec<EditDoc>,
}

const HISTORY_LIMIT: usize = 100;

impl History {
    pub fn new() -> History {
        History {
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }

    /// Merkt den Zustand vor einer Änderung.
    pub fn record(&mut self, before: &EditDoc) {
        self.undo.push(before.clone());
        if self.undo.len() > HISTORY_LIMIT {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Macht die letzte Änderung rückgängig und liefert den Zustand davor.
    pub fn undo(&mut self, current: &EditDoc) -> Option<EditDoc> {
        let prev = self.undo.pop()?;
        self.redo.push(current.clone());
        Some(prev)
    }

    pub fn redo(&mut self, current: &EditDoc) -> Option<EditDoc> {
        let next = self.redo.pop()?;
        self.undo.push(current.clone());
        Some(next)
    }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ashen_sim::mission::{Difficulty, Outcome};
    use ashen_sim::scenario::AreaDef;
    use std::path::PathBuf;

    pub fn data() -> GameData {
        GameData::load_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/data"))
            .expect("data")
    }

    #[test]
    fn brush_shapes_cover_expected_tiles() {
        let c = IVec2::new(10, 10);
        assert_eq!(BrushShape::Square.tiles(c, 1), vec![c]);
        assert_eq!(BrushShape::Square.tiles(c, 3).len(), 9);
        assert_eq!(
            BrushShape::Circle.tiles(c, 3).len(),
            9,
            "Kreis Größe 3 deckt 3x3 ab"
        );
        assert_eq!(BrushShape::Diamond.tiles(c, 3).len(), 5);
        assert!(BrushShape::Circle.tiles(c, 6).len() < 36);
        assert!(BrushShape::Circle.tiles(c, 5).contains(&c));
    }

    #[test]
    fn undo_redo_restore_snapshots() {
        let mut doc = EditDoc::blank(32, 32, Terrain::Grass);
        let mut hist = History::new();
        let original = doc.clone();
        hist.record(&doc);
        doc.paint_terrain(
            &BrushShape::Square.tiles(IVec2::new(5, 5), 3),
            Terrain::Water,
        );
        assert_eq!(doc.map.terrain_at(IVec2::new(5, 5)), Some(Terrain::Water));
        let painted = doc.clone();
        hist.record(&doc);
        doc.def.name = "Geändert".into();
        // Zweimal zurück.
        doc = hist.undo(&doc).unwrap();
        assert_eq!(doc.def.name, painted.def.name);
        assert_eq!(doc.map, painted.map);
        doc = hist.undo(&doc).unwrap();
        assert_eq!(doc.map, original.map);
        assert!(!hist.can_undo() && hist.can_redo());
        // Wiederholen.
        doc = hist.redo(&doc).unwrap();
        assert_eq!(doc.map, painted.map);
        doc = hist.redo(&doc).unwrap();
        assert_eq!(doc.def.name, "Geändert");
        assert!(hist.redo(&doc).is_none());
        // Neue Änderung löscht die Wiederholen-Liste.
        doc = hist.undo(&doc).unwrap();
        hist.record(&doc);
        assert!(!hist.can_redo());
    }

    #[test]
    fn placement_rules_are_enforced() {
        let d = data();
        let mut doc = EditDoc::blank(32, 32, Terrain::Grass);
        doc.paint_terrain(
            &BrushShape::Square.tiles(IVec2::new(20, 20), 4),
            Terrain::Water,
        );
        assert!(
            doc.place_object(&d, "keep", 0, Vec2::new(5.5, 5.5), 1)
                .is_ok()
        );
        assert!(
            doc.place_object(&d, "keep", 0, Vec2::new(6.5, 6.5), 1)
                .is_err(),
            "Überlappung"
        );
        assert!(
            doc.place_object(&d, "keep", 0, Vec2::new(20.5, 20.5), 1)
                .is_err(),
            "Wasser"
        );
        assert!(
            doc.place_object(&d, "swordsman", 0, Vec2::new(20.5, 20.5), 3)
                .is_err(),
            "Einheit im Wasser"
        );
        assert!(
            doc.place_object(&d, "swordsman", 0, Vec2::new(12.5, 12.5), 3)
                .is_ok()
        );
        assert!(
            doc.place_object(&d, "gibtsnicht", 0, Vec2::new(12.5, 12.5), 3)
                .is_err()
        );
        assert!(
            doc.place_node(NodeKind::GoldMine, IVec2::new(10, 10), 0)
                .is_ok()
        );
        assert!(
            doc.place_node(NodeKind::GoldMine, IVec2::new(11, 11), 0)
                .is_err()
        );
        let n = doc.def.placements.len();
        assert!(doc.erase_at(&d, Vec2::new(12.5, 12.5)).is_some());
        assert_eq!(doc.def.placements.len(), n - 1);
    }

    #[test]
    fn flood_fill_and_trees() {
        let mut doc = EditDoc::blank(32, 32, Terrain::Grass);
        for y in 0..32 {
            doc.paint_terrain(&[IVec2::new(16, y)], Terrain::Stone);
        }
        let n = doc.flood_fill(IVec2::new(2, 2), Terrain::Sand, 10_000);
        assert_eq!(n, 16 * 32);
        assert_eq!(
            doc.map.terrain_at(IVec2::new(20, 2)),
            Some(Terrain::Grass),
            "Grenze hält"
        );
        let added = doc.paint_trees(
            &BrushShape::Square.tiles(IVec2::new(24, 10), 6),
            true,
            1.0,
            7,
        );
        assert!(added > 0 && added <= 36);
        // Wasser entfernt Bäume darunter.
        doc.paint_terrain(&[IVec2::new(24, 10)], Terrain::Water);
        assert!(!doc.map.nodes.iter().any(|n| n.x == 24 && n.y == 10));
    }

    /// Editor-Zustand → RON → Mission → Simulation läuft.
    #[test]
    fn scenario_roundtrip_through_ron_runs_in_sim() {
        let d = data();
        let mut doc = EditDoc::generated(56, 48, 1234, GenPreset::Riverlands);
        doc.def.name = "Rundreise".into();
        doc.def.id = "rundreise".into();
        doc.paint_terrain(
            &BrushShape::Circle.tiles(IVec2::new(28, 24), 5),
            Terrain::Sand,
        );
        doc.place_decor(DecorKind::Brazier, Vec2::new(20.5, 30.5), 1)
            .unwrap();
        doc.def.areas.push(AreaDef {
            id: "tor".into(),
            name: "Das Tor".into(),
            x: 10,
            y: 10,
            w: 4,
            h: 4,
        });
        doc.def.triggers.push(TriggerDef {
            id: "warnung".into(),
            condition: Condition::All(vec![
                Condition::AtTime(5.0),
                Condition::Not(Box::new(Condition::EnemyUnitsAtMost(0))),
            ]),
            actions: vec![
                Action::Message {
                    speaker: "Wache".into(),
                    text: "Feinde in Sicht!".into(),
                    voice: None,
                },
                Action::SpawnUnits {
                    unit: "hollow_thrall".into(),
                    owner: 1,
                    area: "tor".into(),
                    count: 2,
                    attack: true,
                },
            ],
            modes: vec![],
        });
        let mission = doc.to_mission();
        let dir = std::env::temp_dir().join(format!("ashen_roundtrip_{}", std::process::id()));
        let path = dir.join("rundreise.ron");
        ashen_sim::scenario::save(&mission, &path).expect("speichern");
        let loaded = MissionDef::load(&path).expect("laden");
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(loaded.id, "rundreise");
        assert_eq!(loaded.map.as_ref().unwrap(), mission.map.as_ref().unwrap());
        assert_eq!(loaded.triggers.len(), mission.triggers.len());
        assert_eq!(loaded.areas, mission.areas);
        // Zurück in den Editor und wieder heraus: identisch.
        let again = EditDoc::from_mission(&d, &loaded).unwrap();
        assert_eq!(again.map, doc.map);
        // Die Simulation läuft in jedem Modus.
        for mode in [
            GameMode::Campaign,
            GameMode::Skirmish,
            GameMode::Survival,
            GameMode::Sandbox,
        ] {
            let mut w = SimWorld::new(
                &d,
                loaded.clone(),
                MatchSettings {
                    mode,
                    difficulty: Difficulty::Normal,
                    ..Default::default()
                },
            )
            .expect("Welt");
            assert_eq!((w.map.width, w.map.height), (56, 48));
            assert!(w.building_with_tag("player_keep").is_some());
            for _ in 0..400 {
                w.step(&d);
                w.events.clear();
            }
            assert!(w.mission.outcome != Some(Outcome::Defeat), "{mode:?}");
        }
    }

    #[test]
    fn existing_mission_converts_to_scenario() {
        let d = data();
        let m = MissionDef::load(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../assets/data/missions/m01_last_settlement.ron"),
        )
        .unwrap();
        let doc = EditDoc::from_mission(&d, &m).unwrap();
        assert_eq!((doc.map.width, doc.map.height), (72, 72));
        assert!(
            doc.map
                .nodes
                .iter()
                .filter(|n| n.kind == NodeKind::Tree)
                .count()
                > 200
        );
        let w = SimWorld::new(&d, doc.to_mission(), MatchSettings::default()).unwrap();
        assert!(w.building_with_tag("enemy_shrine").is_some());
    }
}
