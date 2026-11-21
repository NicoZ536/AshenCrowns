//! Datengetriebene Spielinhalte (Einheiten, Gebäude, Fähigkeiten, Regeln).
//!
//! Alle Inhalte werden aus RON-Dateien unter `assets/data/` geladen. Mods können Einträge
//! gleicher `id` überschreiben oder neue hinzufügen (siehe [`GameData::apply_overlay`]).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Index eines Einheitentyps in [`GameData::units`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct UnitTypeId(pub u16);

/// Index eines Gebäudetyps in [`GameData::buildings`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct BuildingTypeId(pub u16);

/// Index einer Fähigkeit in [`GameData::abilities`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AbilityId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Faction {
    EmberCrown,
    Hollowed,
    Neutral,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitClass {
    Worker,
    Infantry,
    Ranged,
    Cavalry,
    Hero,
    Monster,
    Boss,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorType {
    Unarmored,
    Light,
    Medium,
    Heavy,
    Fortified,
    Hero,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Normal,
    Pierce,
    Siege,
    Magic,
    Hero,
}

impl DamageType {
    /// Schadensmultiplikator gegen einen Rüstungstyp.
    pub fn multiplier(self, armor: ArmorType) -> f32 {
        use ArmorType as A;
        use DamageType as D;
        match (self, armor) {
            (D::Normal, A::Unarmored) => 1.0,
            (D::Normal, A::Light) => 1.0,
            (D::Normal, A::Medium) => 1.5,
            (D::Normal, A::Heavy) => 1.0,
            (D::Normal, A::Fortified) => 0.7,
            (D::Normal, A::Hero) => 1.0,
            (D::Pierce, A::Unarmored) => 1.5,
            (D::Pierce, A::Light) => 2.0,
            (D::Pierce, A::Medium) => 0.75,
            (D::Pierce, A::Heavy) => 0.8,
            (D::Pierce, A::Fortified) => 0.35,
            (D::Pierce, A::Hero) => 0.5,
            (D::Siege, A::Unarmored) => 1.5,
            (D::Siege, A::Light) => 1.0,
            (D::Siege, A::Medium) => 0.5,
            (D::Siege, A::Heavy) => 1.0,
            (D::Siege, A::Fortified) => 1.5,
            (D::Siege, A::Hero) => 0.5,
            (D::Magic, A::Unarmored) => 1.0,
            (D::Magic, A::Light) => 1.25,
            (D::Magic, A::Medium) => 0.75,
            (D::Magic, A::Heavy) => 2.0,
            (D::Magic, A::Fortified) => 0.35,
            (D::Magic, A::Hero) => 0.5,
            (D::Hero, A::Fortified) => 0.5,
            (D::Hero, _) => 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceKind {
    Gold,
    Wood,
    Stone,
    Food,
}

impl ResourceKind {
    pub const ALL: [ResourceKind; 4] = [ResourceKind::Gold, ResourceKind::Wood, ResourceKind::Stone, ResourceKind::Food];

    pub fn label(self) -> &'static str {
        match self {
            ResourceKind::Gold => "Gold",
            ResourceKind::Wood => "Holz",
            ResourceKind::Stone => "Stein",
            ResourceKind::Food => "Nahrung",
        }
    }
}

/// Kosten bzw. Ressourcenmengen.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Cost {
    pub gold: f32,
    pub wood: f32,
    pub stone: f32,
    pub food: f32,
}

impl Cost {
    pub fn get(&self, k: ResourceKind) -> f32 {
        match k {
            ResourceKind::Gold => self.gold,
            ResourceKind::Wood => self.wood,
            ResourceKind::Stone => self.stone,
            ResourceKind::Food => self.food,
        }
    }

    pub fn get_mut(&mut self, k: ResourceKind) -> &mut f32 {
        match k {
            ResourceKind::Gold => &mut self.gold,
            ResourceKind::Wood => &mut self.wood,
            ResourceKind::Stone => &mut self.stone,
            ResourceKind::Food => &mut self.food,
        }
    }

    pub fn scaled(&self, f: f32) -> Cost {
        Cost { gold: self.gold * f, wood: self.wood * f, stone: self.stone * f, food: self.food * f }
    }

    /// `true`, wenn `self` alle Kosten `c` decken kann.
    pub fn covers(&self, c: &Cost) -> bool {
        ResourceKind::ALL.iter().all(|&k| self.get(k) + 1e-3 >= c.get(k))
    }

    pub fn sub(&mut self, c: &Cost) {
        for k in ResourceKind::ALL {
            *self.get_mut(k) -= c.get(k);
        }
    }

    pub fn add(&mut self, c: &Cost) {
        for k in ResourceKind::ALL {
            *self.get_mut(k) += c.get(k);
        }
    }

    /// Erste fehlende Ressource (für Fehlermeldungen).
    pub fn first_missing(&self, c: &Cost) -> Option<ResourceKind> {
        ResourceKind::ALL.into_iter().find(|&k| self.get(k) + 1e-3 < c.get(k))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectileKind {
    Arrow,
    BoneBolt,
    SoulBolt,
}

/// Definition eines Einheitentyps.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct UnitDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub faction: Faction,
    pub class: UnitClass,
    pub hp: f32,
    pub mana: f32,
    pub mana_regen: f32,
    pub hp_regen: f32,
    pub armor: f32,
    pub armor_type: ArmorType,
    pub damage_min: f32,
    pub damage_max: f32,
    pub damage_type: DamageType,
    /// Angriffsreichweite in Kacheln (zusätzlich zu den Radien).
    pub range: f32,
    pub cooldown: f32,
    pub windup: f32,
    /// Bewegung in Kacheln pro Sekunde.
    pub speed: f32,
    pub sight: f32,
    pub radius: f32,
    pub cost: Cost,
    pub train_time: f32,
    pub pop: u32,
    /// Nahrungsverbrauch pro Sekunde.
    pub upkeep: f32,
    pub morale: f32,
    pub fearless: bool,
    pub projectile: Option<ProjectileKind>,
    pub bonus_vs: Vec<(UnitClass, f32)>,
    pub abilities: Vec<String>,
    pub carry: u32,
    pub gather_speed: f32,
    pub build_speed: f32,
    pub xp_value: u32,
    pub hp_per_level: f32,
    pub damage_per_level: f32,
    pub hotkey: Option<char>,
}

impl Default for UnitDef {
    fn default() -> Self {
        UnitDef {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            faction: Faction::Neutral,
            class: UnitClass::Infantry,
            hp: 100.0,
            mana: 0.0,
            mana_regen: 0.0,
            hp_regen: 0.0,
            armor: 0.0,
            armor_type: ArmorType::Medium,
            damage_min: 5.0,
            damage_max: 7.0,
            damage_type: DamageType::Normal,
            range: 0.6,
            cooldown: 1.5,
            windup: 0.3,
            speed: 2.5,
            sight: 7.0,
            radius: 0.3,
            cost: Cost::default(),
            train_time: 15.0,
            pop: 1,
            upkeep: 0.05,
            morale: 100.0,
            fearless: false,
            projectile: None,
            bonus_vs: Vec::new(),
            abilities: Vec::new(),
            carry: 0,
            gather_speed: 0.0,
            build_speed: 0.0,
            xp_value: 10,
            hp_per_level: 0.0,
            damage_per_level: 0.0,
            hotkey: None,
        }
    }
}

impl UnitDef {
    pub fn is_ranged(&self) -> bool {
        self.projectile.is_some()
    }

    pub fn bonus_against(&self, class: UnitClass) -> f32 {
        self.bonus_vs.iter().find(|(c, _)| *c == class).map(|(_, m)| *m).unwrap_or(1.0)
    }
}

/// Turmangriff eines Gebäudes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TowerAttack {
    pub damage: f32,
    pub damage_type: DamageType,
    pub range: f32,
    pub cooldown: f32,
    pub projectile: ProjectileKind,
}

/// Definition eines Gebäudetyps.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildingDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub faction: Faction,
    /// Grundfläche in Kacheln (x, y).
    pub size: (u8, u8),
    pub hp: f32,
    pub armor: f32,
    pub armor_type: ArmorType,
    pub cost: Cost,
    pub build_time: f32,
    pub sight: f32,
    pub pop_provided: u32,
    pub trains: Vec<String>,
    pub dropoff: bool,
    pub food_per_sec: f32,
    pub gold_per_sec: f32,
    pub attack: Option<TowerAttack>,
    pub requires: Vec<String>,
    pub buildable: bool,
    pub hotkey: Option<char>,
}

impl Default for BuildingDef {
    fn default() -> Self {
        BuildingDef {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            faction: Faction::EmberCrown,
            size: (2, 2),
            hp: 800.0,
            armor: 5.0,
            armor_type: ArmorType::Fortified,
            cost: Cost::default(),
            build_time: 30.0,
            sight: 6.0,
            pop_provided: 0,
            trains: Vec::new(),
            dropoff: false,
            food_per_sec: 0.0,
            gold_per_sec: 0.0,
            attack: None,
            requires: Vec::new(),
            buildable: true,
            hotkey: None,
        }
    }
}

/// Zielart einer Fähigkeit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbilityTargeting {
    /// Wirkt sofort um den Wirker.
    SelfCentered,
    /// Benötigt einen Bodenpunkt.
    Point,
}

/// Elementare Effekte, aus denen Fähigkeiten zusammengesetzt werden.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Effect {
    /// Flächenschaden um den Zielpunkt.
    Damage { radius: f32, amount: f32, damage_type: DamageType },
    /// Schild auf dem Wirker.
    Shield { amount: f32, duration: f32 },
    /// Verbündete im Radius stärken.
    Rally { radius: f32, damage_mult: f32, armor_bonus: f32, morale: f32, duration: f32 },
    /// Lebensentzug von Gegnern im Radius, heilt den Wirker.
    Drain { radius: f32, amount: f32 },
    /// Einheiten beschwören.
    Summon { unit: String, count: u32 },
    /// Moralschaden bei Gegnern (Furcht).
    Terror { radius: f32, morale: f32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub mana: f32,
    pub cooldown: f32,
    pub range: f32,
    pub targeting: AbilityTargeting,
    pub effects: Vec<Effect>,
    pub hotkey: Option<char>,
}

/// Globale Regelwerte.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Rules {
    pub tick_rate: u32,
    pub day_length: f32,
    pub tree_wood: u32,
    pub gold_mine_gold: u32,
    pub quarry_stone: u32,
    pub gather_time: f32,
    pub max_population: u32,
    pub hero_respawn_time: f32,
    pub xp_levels: Vec<u32>,
    pub night_sight_mult: f32,
    pub fog_sight_mult: f32,
    pub rain_ranged_mult: f32,
    pub demolish_refund: f32,
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            tick_rate: 20,
            day_length: 360.0,
            tree_wood: 120,
            gold_mine_gold: 6000,
            quarry_stone: 2000,
            gather_time: 3.5,
            max_population: 80,
            hero_respawn_time: 40.0,
            xp_levels: vec![0, 100, 250, 450, 700, 1000, 1400, 1900, 2500, 3200],
            night_sight_mult: 0.7,
            fog_sight_mult: 0.75,
            rain_ranged_mult: 0.9,
            demolish_refund: 0.5,
        }
    }
}

/// Datei-Hüllen für das Laden.
#[derive(Deserialize)]
struct UnitsFile {
    units: Vec<UnitDef>,
}
#[derive(Deserialize)]
struct BuildingsFile {
    buildings: Vec<BuildingDef>,
}
#[derive(Deserialize)]
struct AbilitiesFile {
    abilities: Vec<AbilityDef>,
}

/// Fehler beim Laden der Spieldaten.
#[derive(Debug)]
pub enum DataError {
    Io(String, std::io::Error),
    Parse(String, String),
    Invalid(String),
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::Io(p, e) => write!(f, "Datei {p} nicht lesbar: {e}"),
            DataError::Parse(p, e) => write!(f, "Datei {p} fehlerhaft: {e}"),
            DataError::Invalid(m) => write!(f, "Ungültige Spieldaten: {m}"),
        }
    }
}

impl std::error::Error for DataError {}

/// Alle Spielinhalte mit schnellen Index-Tabellen.
#[derive(Clone, Debug, Default)]
pub struct GameData {
    pub units: Vec<UnitDef>,
    pub buildings: Vec<BuildingDef>,
    pub abilities: Vec<AbilityDef>,
    pub rules: Rules,
    unit_index: HashMap<String, UnitTypeId>,
    building_index: HashMap<String, BuildingTypeId>,
    ability_index: HashMap<String, AbilityId>,
}

fn read_ron<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, DataError> {
    let text = std::fs::read_to_string(path).map_err(|e| DataError::Io(path.display().to_string(), e))?;
    ron::from_str(&text).map_err(|e| DataError::Parse(path.display().to_string(), e.to_string()))
}

impl GameData {
    /// Lädt die Basisdaten aus einem `data`-Verzeichnis.
    pub fn load_dir(dir: &Path) -> Result<GameData, DataError> {
        let units: UnitsFile = read_ron(&dir.join("units.ron"))?;
        let buildings: BuildingsFile = read_ron(&dir.join("buildings.ron"))?;
        let abilities: AbilitiesFile = read_ron(&dir.join("abilities.ron"))?;
        let rules: Rules = read_ron(&dir.join("rules.ron"))?;
        let mut d = GameData { units: units.units, buildings: buildings.buildings, abilities: abilities.abilities, rules, ..Default::default() };
        d.reindex();
        d.validate()?;
        Ok(d)
    }

    /// Wendet ein Mod-Datenverzeichnis an. Fehlende Dateien werden ignoriert; Einträge mit
    /// vorhandener `id` werden ersetzt, neue werden angehängt.
    pub fn apply_overlay(&mut self, dir: &Path) -> Result<Vec<String>, DataError> {
        let mut changes = Vec::new();
        let p = dir.join("units.ron");
        if p.exists() {
            let f: UnitsFile = read_ron(&p)?;
            for u in f.units {
                changes.push(format!("unit:{}", u.id));
                match self.units.iter_mut().find(|x| x.id == u.id) {
                    Some(slot) => *slot = u,
                    None => self.units.push(u),
                }
            }
        }
        let p = dir.join("buildings.ron");
        if p.exists() {
            let f: BuildingsFile = read_ron(&p)?;
            for b in f.buildings {
                changes.push(format!("building:{}", b.id));
                match self.buildings.iter_mut().find(|x| x.id == b.id) {
                    Some(slot) => *slot = b,
                    None => self.buildings.push(b),
                }
            }
        }
        let p = dir.join("abilities.ron");
        if p.exists() {
            let f: AbilitiesFile = read_ron(&p)?;
            for a in f.abilities {
                changes.push(format!("ability:{}", a.id));
                match self.abilities.iter_mut().find(|x| x.id == a.id) {
                    Some(slot) => *slot = a,
                    None => self.abilities.push(a),
                }
            }
        }
        self.reindex();
        self.validate()?;
        Ok(changes)
    }

    fn reindex(&mut self) {
        self.unit_index = self.units.iter().enumerate().map(|(i, u)| (u.id.clone(), UnitTypeId(i as u16))).collect();
        self.building_index = self.buildings.iter().enumerate().map(|(i, b)| (b.id.clone(), BuildingTypeId(i as u16))).collect();
        self.ability_index = self.abilities.iter().enumerate().map(|(i, a)| (a.id.clone(), AbilityId(i as u16))).collect();
    }

    /// Prüft Querverweise (Trainingslisten, Fähigkeiten, Beschwörungen).
    pub fn validate(&self) -> Result<(), DataError> {
        for u in &self.units {
            if u.hp <= 0.0 || u.speed < 0.0 || u.damage_max < u.damage_min {
                return Err(DataError::Invalid(format!("Einheit {} hat ungültige Werte", u.id)));
            }
            for a in &u.abilities {
                if !self.ability_index.contains_key(a) {
                    return Err(DataError::Invalid(format!("Einheit {} verweist auf unbekannte Fähigkeit {a}", u.id)));
                }
            }
        }
        for b in &self.buildings {
            for t in &b.trains {
                if !self.unit_index.contains_key(t) {
                    return Err(DataError::Invalid(format!("Gebäude {} trainiert unbekannte Einheit {t}", b.id)));
                }
            }
            for r in &b.requires {
                if !self.building_index.contains_key(r) {
                    return Err(DataError::Invalid(format!("Gebäude {} benötigt unbekanntes Gebäude {r}", b.id)));
                }
            }
            if b.size.0 == 0 || b.size.1 == 0 {
                return Err(DataError::Invalid(format!("Gebäude {} hat Größe 0", b.id)));
            }
        }
        for a in &self.abilities {
            for e in &a.effects {
                if let Effect::Summon { unit, .. } = e
                    && !self.unit_index.contains_key(unit)
                {
                    return Err(DataError::Invalid(format!("Fähigkeit {} beschwört unbekannte Einheit {unit}", a.id)));
                }
            }
        }
        Ok(())
    }

    pub fn unit_id(&self, id: &str) -> Option<UnitTypeId> {
        self.unit_index.get(id).copied()
    }

    pub fn building_id(&self, id: &str) -> Option<BuildingTypeId> {
        self.building_index.get(id).copied()
    }

    pub fn ability_id(&self, id: &str) -> Option<AbilityId> {
        self.ability_index.get(id).copied()
    }

    pub fn unit(&self, id: UnitTypeId) -> &UnitDef {
        &self.units[id.0 as usize]
    }

    pub fn building(&self, id: BuildingTypeId) -> &BuildingDef {
        &self.buildings[id.0 as usize]
    }

    pub fn ability(&self, id: AbilityId) -> &AbilityDef {
        &self.abilities[id.0 as usize]
    }

    /// Typtabelle für Spielstände (Reihenfolge der IDs).
    pub fn type_table(&self) -> Vec<String> {
        self.units
            .iter()
            .map(|u| format!("u:{}", u.id))
            .chain(self.buildings.iter().map(|b| format!("b:{}", b.id)))
            .chain(self.abilities.iter().map(|a| format!("a:{}", a.id)))
            .collect()
    }
}
