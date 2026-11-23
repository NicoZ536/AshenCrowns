//! Datengetriebene Spielinhalte (Einheiten, Gebäude, Fähigkeiten, Regeln).
//!
//! Alle Inhalte werden aus RON-Dateien unter `assets/data/` geladen. Mods können Einträge
//! gleicher `id` überschreiben oder neue hinzufügen (siehe [`GameData::apply_overlay`]).

use crate::rpg::{ItemDef, ItemId, TalentTree, UpgradeDef};
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
    /// Belagerungsmaschine (Ramme, Katapult): schwach im Nahkampf, verheerend gegen Gebäude.
    Siege,
    /// Heiler und Unterstützer.
    Support,
    /// Fliegende Gegner (nur von Fernkämpfern und langen Stangenwaffen zu treffen).
    Flyer,
}

impl UnitClass {
    pub fn label(self) -> &'static str {
        match self {
            UnitClass::Worker => "Arbeiter",
            UnitClass::Infantry => "Infanterie",
            UnitClass::Ranged => "Fernkämpfer",
            UnitClass::Cavalry => "Kavallerie",
            UnitClass::Hero => "Held",
            UnitClass::Monster => "Monster",
            UnitClass::Boss => "Boss",
            UnitClass::Siege => "Belagerung",
            UnitClass::Support => "Unterstützung",
            UnitClass::Flyer => "Flieger",
        }
    }
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
    /// Feuerschaden (Feuerschule): stark gegen leichte Ziele und Untote in Stofffetzen.
    Fire,
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
            (D::Fire, A::Unarmored) => 1.5,
            (D::Fire, A::Light) => 1.25,
            (D::Fire, A::Medium) => 1.0,
            (D::Fire, A::Heavy) => 0.75,
            (D::Fire, A::Fortified) => 0.6,
            (D::Fire, A::Hero) => 0.75,
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

/// Index einer Ware in [`GameData::goods`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct GoodId(pub u16);

/// Alles, was gelagert, gehandelt oder verbraucht werden kann: Grundressource oder Ware.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Item {
    Res(ResourceKind),
    Good(GoodId),
}

/// Anzahl der Grundressourcen (Vorderteil des einheitlichen Item-Index).
pub const BASE_ITEMS: usize = 4;

impl Item {
    /// Fortlaufender Index: Grundressourcen zuerst, danach die Waren.
    pub fn index(self) -> usize {
        match self {
            Item::Res(ResourceKind::Gold) => 0,
            Item::Res(ResourceKind::Wood) => 1,
            Item::Res(ResourceKind::Stone) => 2,
            Item::Res(ResourceKind::Food) => 3,
            Item::Good(g) => BASE_ITEMS + g.0 as usize,
        }
    }

    pub fn from_index(i: usize) -> Item {
        match i {
            0 => Item::Res(ResourceKind::Gold),
            1 => Item::Res(ResourceKind::Wood),
            2 => Item::Res(ResourceKind::Stone),
            3 => Item::Res(ResourceKind::Food),
            _ => Item::Good(GoodId((i - BASE_ITEMS) as u16)),
        }
    }
}

/// Definition einer Ware (`goods.ron`).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GoodDef {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Grundpreis in Gold beim Handel.
    pub price: f32,
    /// Güterstufe: 1 Rohstoff, 2 Zwischengut, 3 Fertiggut (Gruppierung im Lagerfenster).
    pub tier: u8,
}

impl Default for GoodDef {
    fn default() -> Self {
        GoodDef { id: String::new(), name: String::new(), description: String::new(), price: 5.0, tier: 1 }
    }
}

/// Mengenliste in Rohform (`("planks", 2.0)`); wird beim Laden zu [`ItemAmounts`] aufgelöst.
pub type ItemList = Vec<(String, f32)>;
/// Aufgelöste Mengenliste.
pub type ItemAmounts = Vec<(Item, f32)>;

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
    /// Armbrustbolzen.
    Bolt,
    /// Feuerball der Kampfmagier.
    Fireball,
    /// Felsbrocken des Katapults (Flugbogen, Flächenschaden).
    Boulder,
}

impl ProjectileKind {
    /// Fluggeschwindigkeit in Kacheln pro Sekunde.
    pub fn speed(self) -> f32 {
        match self {
            ProjectileKind::Arrow => 11.0,
            ProjectileKind::BoneBolt => 8.0,
            ProjectileKind::SoulBolt => 7.0,
            ProjectileKind::Bolt => 14.0,
            ProjectileKind::Fireball => 8.5,
            ProjectileKind::Boulder => 6.0,
        }
    }
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
    /// Zusätzlich benötigte Waren (Ausrüstung) je Ausbildung.
    pub goods_cost: ItemList,
    #[serde(skip)]
    pub goods_cost_r: ItemAmounts,
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
    /// Benötigte Ausbaustufe der Siedlung für die Ausbildung (0 = Siedlung, 1 = Burg).
    pub era: u8,
    /// Mindestreichweite in Kacheln: näher liegende Ziele können nicht beschossen werden (Katapult).
    pub min_range: f32,
    /// Flächenschaden-Radius am Einschlagpunkt (0 = Einzelziel).
    pub splash: f32,
    /// Schadensfaktor gegen Gebäude (Ramme, Katapult, Grabkoloss).
    pub building_mult: f32,
    /// Fliegt über Hindernisse und kann nur von Fernkämpfern und langen Stangenwaffen getroffen werden.
    pub flying: bool,
    /// Anteil des Pfeil-/Bolzenschadens, den Schild oder Haltung abfangen (0..1).
    pub ranged_resist: f32,
    /// Fähigkeiten, die die Einheit selbstständig wirkt (Heilen, Beschwören).
    pub autocast: Vec<String>,
    /// Beutetabelle (Gegenstands-ID, Wahrscheinlichkeit) bei Tod.
    pub loot: Vec<(String, f32)>,
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
            goods_cost: Vec::new(),
            goods_cost_r: Vec::new(),
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
            era: 0,
            min_range: 0.0,
            splash: 0.0,
            building_mult: 1.0,
            flying: false,
            ranged_resist: 0.0,
            autocast: Vec::new(),
            loot: Vec::new(),
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

/// Rubrik im Baumenü.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BuildCategory {
    Housing,
    Raw,
    Craft,
    Food,
    #[default]
    Civic,
    Military,
    Wall,
}

impl BuildCategory {
    pub const ALL: [BuildCategory; 7] = [BuildCategory::Housing, BuildCategory::Raw, BuildCategory::Craft, BuildCategory::Food, BuildCategory::Civic, BuildCategory::Military, BuildCategory::Wall];

    pub fn label(self) -> &'static str {
        match self {
            BuildCategory::Housing => "Wohnen",
            BuildCategory::Raw => "Rohstoffe",
            BuildCategory::Craft => "Handwerk",
            BuildCategory::Food => "Nahrung",
            BuildCategory::Civic => "Stadt",
            BuildCategory::Military => "Militär",
            BuildCategory::Wall => "Mauern",
        }
    }
}

/// Befestigungsart eines Gebäudes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fort {
    /// Mauersegment: blockiert alle.
    Wall,
    /// Tor: eigene Einheiten passieren, solange es offen ist; Feinde nie.
    Gate,
    /// Mauerturm: blockiert wie eine Mauer und beschießt Feinde.
    Tower,
}

/// Rezept eines Produktionsgebäudes. Ohne Ausgaben ist es ein Dienst (z. B. Räucherwerk im Tempel).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub inputs: ItemList,
    pub outputs: ItemList,
    /// Dauer eines Zyklus bei voller Besetzung in Sekunden.
    pub cycle: f32,
    #[serde(skip)]
    pub inputs_r: ItemAmounts,
    #[serde(skip)]
    pub outputs_r: ItemAmounts,
}

/// Standortbedingung eines Gewinnungsgebäudes.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Site {
    /// Benötigte Ressourcenquelle in der Nähe.
    pub node: Option<crate::entity::NodeKind>,
    pub node_radius: f32,
    pub node_min: u32,
    /// Jeder Zyklus verbraucht Vorrat der nächsten Quelle (Bäume werden gefällt).
    pub consume_node: bool,
    /// Benötigtes Gelände in der Nähe.
    pub terrain: Vec<crate::map::Terrain>,
    pub terrain_radius: i32,
    pub terrain_min: u32,
    /// Hinweistext bei verletzter Bedingung.
    pub hint: String,
}

/// Art eines Bedürfnisses der Bewohner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NeedKind {
    /// Verbrauch einer Ware oder Grundressource (ID).
    Item(String),
    /// Ein bedienter Tempel in Reichweite.
    Faith,
    /// Eine Wehranlage (Turm, Kaserne, Bergfried) in Reichweite.
    Safety,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Need {
    pub kind: NeedKind,
    /// Verbrauch je Bewohner und Minute (nur Waren).
    pub rate: f32,
    /// Gewicht für die Zufriedenheit.
    pub weight: f32,
    #[serde(skip)]
    pub item_r: Option<Item>,
}

impl Default for Need {
    fn default() -> Self {
        Need { kind: NeedKind::Faith, rate: 0.0, weight: 1.0, item_r: None }
    }
}

/// Wohnhaus einer Stufe.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Housing {
    pub tier: u8,
    pub capacity: u32,
    pub needs: Vec<Need>,
    /// Steuer je Bewohner und Sekunde bei Normalsatz und voller Zufriedenheit.
    pub tax: f32,
    pub upgrade_to: Option<String>,
    /// Mindest-Ausbaustufe der Siedlung für den Aufstieg.
    pub upgrade_era: u8,
    pub downgrade_to: Option<String>,
}

/// Dienstart eines Gebäudes mit Wirkradius.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceKind {
    Faith,
    Safety,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Service {
    pub kind: ServiceKind,
    pub radius: f32,
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
    /// Zusätzlich benötigte Waren (Bretter, Ziegel, Werkzeug …).
    pub goods_cost: ItemList,
    #[serde(skip)]
    pub goods_cost_r: ItemAmounts,
    pub build_time: f32,
    pub sight: f32,
    pub pop_provided: u32,
    pub trains: Vec<String>,
    pub dropoff: bool,
    pub food_per_sec: f32,
    pub gold_per_sec: f32,
    pub attack: Option<TowerAttack>,
    pub requires: Vec<String>,
    /// Mindest-Ausbaustufe der Siedlung (Index in `Rules::eras`).
    pub era: u8,
    pub category: BuildCategory,
    /// Arbeitsplätze (Bewohner), die das Gebäude für volle Leistung braucht.
    pub workers: u32,
    pub recipes: Vec<Recipe>,
    pub site: Option<Site>,
    pub housing: Option<Housing>,
    /// Lagerkapazität je Ware und Grundressource (außer Gold); zugleich Lieferziel der Produktion.
    pub storage: f32,
    pub market: bool,
    pub fort: Option<Fort>,
    pub service: Option<Service>,
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
            goods_cost: Vec::new(),
            goods_cost_r: Vec::new(),
            build_time: 30.0,
            sight: 6.0,
            pop_provided: 0,
            trains: Vec::new(),
            dropoff: false,
            food_per_sec: 0.0,
            gold_per_sec: 0.0,
            attack: None,
            requires: Vec::new(),
            era: 0,
            category: BuildCategory::Civic,
            workers: 0,
            recipes: Vec::new(),
            site: None,
            housing: None,
            storage: 0.0,
            market: false,
            fort: None,
            service: None,
            buildable: true,
            hotkey: None,
        }
    }
}

impl BuildingDef {
    /// Hat das Gebäude Arbeitsplätze mit Rezepten?
    pub fn employs(&self) -> bool {
        self.workers > 0 && !self.recipes.is_empty()
    }

    pub fn is_fort(&self) -> bool {
        self.fort.is_some()
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
    /// Einheiten beschwören. `max` begrenzt die gleichzeitig lebenden Diener des Wirkers (0 = unbegrenzt).
    Summon {
        unit: String,
        count: u32,
        #[serde(default)]
        max: u32,
    },
    /// Heilt Verbündete im Radius um den Zielpunkt.
    Heal { radius: f32, amount: f32 },
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

/// Steuersatz der Bürger.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxLevel {
    pub name: String,
    /// Faktor auf die Steuereinnahmen.
    pub income: f32,
    /// Aufschlag/Abzug auf die Zufriedenheit.
    pub happiness: f32,
}

/// Ausbaustufe der Siedlung (Bergfried → Burg …). Stufe 0 ist der Anfangszustand.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EraDef {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Kosten des Ausbaus *zu* dieser Stufe.
    pub cost: Cost,
    pub goods: ItemList,
    #[serde(skip)]
    pub goods_r: ItemAmounts,
    /// Voraussetzungen: (Gebäude-ID, Anzahl fertiger Gebäude).
    pub requires: Vec<(String, u32)>,
    pub build_time: f32,
    /// Zusätzliche Lagerkapazität je Ware.
    pub storage_bonus: f32,
    /// Zusätzlicher Wohnraum für Einheiten.
    pub pop_bonus: u32,
}

/// Regelwerte der Stadtsimulation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CityRules {
    /// Maximale Entfernung zwischen Produktionsgebäude und Lager (Kacheln).
    pub logistics_radius: f32,
    /// Anteil der Bewohner, der als Arbeitskraft zur Verfügung steht.
    pub workforce_ratio: f32,
    /// Zufriedenheitsänderung pro Sekunde in Richtung Zielwert.
    pub happiness_rate: f32,
    /// Ab dieser Zufriedenheit ziehen neue Bewohner zu.
    pub growth_min_happiness: f32,
    pub growth_rate: f32,
    pub emigration_rate: f32,
    /// Unter diesem Wert herrscht Unruhe (keine Steuern, Abwanderung).
    pub unrest_below: f32,
    pub upgrade_above: f32,
    pub upgrade_time: f32,
    pub downgrade_time: f32,
    pub default_tax: u8,
    pub tax_levels: Vec<TaxLevel>,
    pub price_wood: f32,
    pub price_stone: f32,
    pub price_food: f32,
    /// Handelsspanne (Kauf teurer, Verkauf billiger als der Marktpreis).
    pub market_fee: f32,
    /// Preisänderung je gehandelter Einheit.
    pub market_impact: f32,
    /// Anteil des Handelsdrucks, der pro Sekunde abklingt.
    pub market_recovery: f32,
    pub market_min: f32,
    pub market_max: f32,
}

impl Default for CityRules {
    fn default() -> Self {
        CityRules {
            logistics_radius: 14.0,
            workforce_ratio: 0.8,
            happiness_rate: 3.0,
            growth_min_happiness: 35.0,
            growth_rate: 0.08,
            emigration_rate: 0.12,
            unrest_below: 20.0,
            upgrade_above: 70.0,
            upgrade_time: 25.0,
            downgrade_time: 40.0,
            default_tax: 2,
            tax_levels: vec![
                TaxLevel { name: "Keine".into(), income: 0.0, happiness: 15.0 },
                TaxLevel { name: "Niedrig".into(), income: 0.5, happiness: 6.0 },
                TaxLevel { name: "Normal".into(), income: 1.0, happiness: 0.0 },
                TaxLevel { name: "Hoch".into(), income: 1.6, happiness: -14.0 },
                TaxLevel { name: "Drückend".into(), income: 2.4, happiness: -32.0 },
            ],
            price_wood: 2.0,
            price_stone: 3.0,
            price_food: 1.5,
            market_fee: 0.12,
            market_impact: 0.02,
            market_recovery: 0.012,
            market_min: 0.4,
            market_max: 2.6,
        }
    }
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
    pub city: CityRules,
    pub eras: Vec<EraDef>,
    /// Grundkosten der Wiederbelebung eines gefallenen Helden (Gold) plus Zuschlag je Stufe.
    pub hero_revive_base: f32,
    pub hero_revive_per_level: f32,
    /// Lebensanteil nach der Wiederbelebung am Altar bzw. im Bergfried.
    pub hero_altar_hp: f32,
    pub hero_keep_hp: f32,
    /// Aufnahmeradius für Gegenstände am Boden.
    pub pickup_radius: f32,
    /// Beute eines vollständig besiegten Lagers (Gegenstands-ID, Gewicht).
    pub camp_loot: Vec<(String, f32)>,
    /// Einstellungen der Gegner-KI je Schwierigkeitsgrad (Leicht, Normal, Schwer, Albtraum).
    pub ai_levels: Vec<AiLevel>,
}

/// Parameter der Gegner-KI für einen Schwierigkeitsgrad.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AiLevel {
    pub name: String,
    /// Sekunden zwischen zwei KI-Entscheidungen.
    pub think: f32,
    /// Ziel-Arbeiterzahl.
    pub workers: u32,
    /// Faktor auf die Einnahmen (Ausgleich bzw. Bonus).
    pub income_mult: f32,
    /// Faktor auf die Angriffsgröße.
    pub army_mult: f32,
    /// Sekunden bis zum ersten Angriff.
    pub first_attack: f32,
    /// Mindestabstand zwischen Angriffen in Sekunden.
    pub attack_interval: f32,
    /// Sekunden zwischen den Schmiede-Stufen der KI.
    pub tech_interval: f32,
    /// Wahrscheinlichkeit, die Armee nach Konterlogik statt zufällig zu wählen (0..1).
    pub counter_skill: f32,
    /// Verhältnis eigener zu gegnerischer Stärke, unter dem die KI den Angriff abbricht.
    pub retreat_ratio: f32,
    /// Obergrenze der Armeegröße (Einheitenzahl).
    pub max_army: u32,
    /// Greift mit mehreren Gruppen an und belästigt Arbeiter (nur Albtraum).
    pub harass: bool,
    /// Anzahl der Verteidigungstürme.
    pub towers: u32,
}

impl Default for AiLevel {
    fn default() -> Self {
        AiLevel {
            name: "Normal".into(),
            think: 2.0,
            workers: 8,
            income_mult: 1.0,
            army_mult: 1.0,
            first_attack: 420.0,
            attack_interval: 150.0,
            tech_interval: 360.0,
            counter_skill: 0.6,
            retreat_ratio: 0.5,
            max_army: 40,
            harass: false,
            towers: 2,
        }
    }
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
            city: CityRules::default(),
            eras: Vec::new(),
            hero_revive_base: 150.0,
            hero_revive_per_level: 60.0,
            hero_altar_hp: 0.8,
            hero_keep_hp: 0.5,
            pickup_radius: 1.3,
            camp_loot: Vec::new(),
            ai_levels: vec![AiLevel::default()],
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
struct GoodsFile {
    goods: Vec<GoodDef>,
}
#[derive(Deserialize)]
struct AbilitiesFile {
    abilities: Vec<AbilityDef>,
}
#[derive(Deserialize)]
struct ItemsFile {
    items: Vec<ItemDef>,
}
#[derive(Deserialize)]
struct TalentsFile {
    heroes: Vec<TalentTree>,
}
#[derive(Deserialize)]
struct UpgradesFile {
    upgrades: Vec<UpgradeDef>,
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
    pub goods: Vec<GoodDef>,
    pub items: Vec<ItemDef>,
    pub talents: Vec<TalentTree>,
    pub upgrades: Vec<UpgradeDef>,
    pub rules: Rules,
    unit_index: HashMap<String, UnitTypeId>,
    building_index: HashMap<String, BuildingTypeId>,
    ability_index: HashMap<String, AbilityId>,
    good_index: HashMap<String, GoodId>,
    item_index: HashMap<String, ItemId>,
    resolve_errors: Vec<String>,
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
        let goods: GoodsFile = read_ron(&dir.join("goods.ron"))?;
        let items: ItemsFile = read_ron(&dir.join("items.ron"))?;
        let talents: TalentsFile = read_ron(&dir.join("talents.ron"))?;
        let upgrades: UpgradesFile = read_ron(&dir.join("upgrades.ron"))?;
        let mut d = GameData {
            units: units.units,
            buildings: buildings.buildings,
            abilities: abilities.abilities,
            goods: goods.goods,
            items: items.items,
            talents: talents.heroes,
            upgrades: upgrades.upgrades,
            rules,
            ..Default::default()
        };
        d.reindex();
        d.validate()?;
        Ok(d)
    }

    /// Wendet ein Mod-Datenverzeichnis an. Fehlende Dateien werden ignoriert; Einträge mit
    /// vorhandener `id` werden ersetzt, neue werden angehängt.
    pub fn apply_overlay(&mut self, dir: &Path) -> Result<Vec<String>, DataError> {
        // Atomar: erst auf einer Kopie anwenden und prüfen, dann übernehmen.
        let mut next = self.clone();
        let changes = next.apply_overlay_inner(dir)?;
        *self = next;
        Ok(changes)
    }

    fn apply_overlay_inner(&mut self, dir: &Path) -> Result<Vec<String>, DataError> {
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
        let p = dir.join("goods.ron");
        if p.exists() {
            let f: GoodsFile = read_ron(&p)?;
            for g in f.goods {
                changes.push(format!("good:{}", g.id));
                match self.goods.iter_mut().find(|x| x.id == g.id) {
                    Some(slot) => *slot = g,
                    None => self.goods.push(g),
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
        let p = dir.join("items.ron");
        if p.exists() {
            let f: ItemsFile = read_ron(&p)?;
            for i in f.items {
                changes.push(format!("item:{}", i.id));
                match self.items.iter_mut().find(|x| x.id == i.id) {
                    Some(slot) => *slot = i,
                    None => self.items.push(i),
                }
            }
        }
        let p = dir.join("talents.ron");
        if p.exists() {
            let f: TalentsFile = read_ron(&p)?;
            for t in f.heroes {
                changes.push(format!("talents:{}", t.hero));
                match self.talents.iter_mut().find(|x| x.hero == t.hero) {
                    Some(slot) => *slot = t,
                    None => self.talents.push(t),
                }
            }
        }
        let p = dir.join("upgrades.ron");
        if p.exists() {
            let f: UpgradesFile = read_ron(&p)?;
            for u in f.upgrades {
                changes.push(format!("upgrade:{}", u.id));
                match self.upgrades.iter_mut().find(|x| x.id == u.id) {
                    Some(slot) => *slot = u,
                    None => self.upgrades.push(u),
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
        self.good_index = self.goods.iter().enumerate().map(|(i, g)| (g.id.clone(), GoodId(i as u16))).collect();
        self.item_index = self.items.iter().enumerate().map(|(i, it)| (it.id.clone(), ItemId(i as u16))).collect();
        // Mengenlisten auflösen; unbekannte IDs werden in `validate` gemeldet.
        let gi = self.good_index.clone();
        let mut errors = Vec::new();
        let resolve = |s: &str| -> Option<Item> {
            match s {
                "gold" => Some(Item::Res(ResourceKind::Gold)),
                "wood" => Some(Item::Res(ResourceKind::Wood)),
                "stone" => Some(Item::Res(ResourceKind::Stone)),
                "food" => Some(Item::Res(ResourceKind::Food)),
                other => gi.get(other).map(|g| Item::Good(*g)),
            }
        };
        let list = |owner: &str, l: &ItemList, errors: &mut Vec<String>| -> ItemAmounts {
            l.iter()
                .filter_map(|(id, n)| match resolve(id) {
                    Some(it) => Some((it, *n)),
                    None => {
                        errors.push(format!("{owner} verweist auf unbekannte Ware {id}"));
                        None
                    }
                })
                .collect()
        };
        for u in &mut self.units {
            u.goods_cost_r = list(&format!("Einheit {}", u.id), &u.goods_cost, &mut errors);
        }
        for b in &mut self.buildings {
            b.goods_cost_r = list(&format!("Gebäude {}", b.id), &b.goods_cost, &mut errors);
            for r in &mut b.recipes {
                r.inputs_r = list(&format!("Rezept {}/{}", b.id, r.id), &r.inputs, &mut errors);
                r.outputs_r = list(&format!("Rezept {}/{}", b.id, r.id), &r.outputs, &mut errors);
            }
            if let Some(h) = &mut b.housing {
                for n in &mut h.needs {
                    n.item_r = match &n.kind {
                        NeedKind::Item(id) => {
                            let r = resolve(id);
                            if r.is_none() {
                                errors.push(format!("Gebäude {} hat Bedürfnis nach unbekannter Ware {id}", b.id));
                            }
                            r
                        }
                        _ => None,
                    };
                }
            }
        }
        for e in &mut self.rules.eras {
            e.goods_r = list(&format!("Ausbaustufe {}", e.id), &e.goods, &mut errors);
        }
        for up in &mut self.upgrades {
            for (i, l) in up.levels.iter_mut().enumerate() {
                l.goods_r = list(&format!("Upgrade {} Stufe {}", up.id, i + 1), &l.goods, &mut errors);
            }
        }
        self.resolve_errors = errors;
    }

    /// Prüft Querverweise (Trainingslisten, Fähigkeiten, Beschwörungen).
    pub fn validate(&self) -> Result<(), DataError> {
        if let Some(e) = self.resolve_errors.first() {
            return Err(DataError::Invalid(e.clone()));
        }
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
            if let Some(h) = &b.housing {
                for t in [&h.upgrade_to, &h.downgrade_to].into_iter().flatten() {
                    if !self.building_index.contains_key(t) {
                        return Err(DataError::Invalid(format!("Wohnhaus {} verweist auf unbekannte Stufe {t}", b.id)));
                    }
                }
            }
            for r in &b.recipes {
                if r.cycle <= 0.0 {
                    return Err(DataError::Invalid(format!("Rezept {}/{} hat keine Zykluszeit", b.id, r.id)));
                }
            }
        }
        if self.rules.city.tax_levels.is_empty() || self.rules.city.default_tax as usize >= self.rules.city.tax_levels.len() {
            return Err(DataError::Invalid("Steuerstufen fehlen oder Standardsatz ungültig".into()));
        }
        for e in &self.rules.eras {
            for (b, _) in &e.requires {
                if !self.building_index.contains_key(b) {
                    return Err(DataError::Invalid(format!("Ausbaustufe {} verlangt unbekanntes Gebäude {b}", e.id)));
                }
            }
        }
        for b in &self.buildings {
            if b.era as usize > self.rules.eras.len() {
                return Err(DataError::Invalid(format!("Gebäude {} verlangt unbekannte Ausbaustufe {}", b.id, b.era)));
            }
        }
        for u in &self.units {
            for a in &u.autocast {
                if !self.ability_index.contains_key(a) {
                    return Err(DataError::Invalid(format!("Einheit {} wirkt unbekannte Fähigkeit {a} selbstständig", u.id)));
                }
            }
            for (it, _) in &u.loot {
                if !self.item_index.contains_key(it) {
                    return Err(DataError::Invalid(format!("Einheit {} wirft unbekannten Gegenstand {it} ab", u.id)));
                }
            }
        }
        for (it, _) in &self.rules.camp_loot {
            if !self.item_index.contains_key(it) {
                return Err(DataError::Invalid(format!("Lagerbeute verweist auf unbekannten Gegenstand {it}")));
            }
        }
        for tree in &self.talents {
            let Some(hero) = self.unit_index.get(&tree.hero) else {
                return Err(DataError::Invalid(format!("Talentbaum für unbekannten Helden {}", tree.hero)));
            };
            if self.units[hero.0 as usize].class != UnitClass::Hero {
                return Err(DataError::Invalid(format!("{} ist kein Held, hat aber einen Talentbaum", tree.hero)));
            }
            for t in &tree.talents {
                if t.path >= 3 || t.max_rank == 0 {
                    return Err(DataError::Invalid(format!("Talent {} hat ungültigen Pfad oder Rang", t.id)));
                }
                if let Some(a) = &t.ability
                    && !self.ability_index.contains_key(a)
                {
                    return Err(DataError::Invalid(format!("Talent {} schaltet unbekannte Fähigkeit {a} frei", t.id)));
                }
            }
        }
        for up in &self.upgrades {
            if !self.building_index.contains_key(&up.building) {
                return Err(DataError::Invalid(format!("Upgrade {} verweist auf unbekanntes Gebäude {}", up.id, up.building)));
            }
            if up.levels.is_empty() {
                return Err(DataError::Invalid(format!("Upgrade {} hat keine Stufen", up.id)));
            }
        }
        if self.rules.ai_levels.is_empty() {
            return Err(DataError::Invalid("Keine KI-Stufen definiert".into()));
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

    pub fn item_def_id(&self, id: &str) -> Option<ItemId> {
        self.item_index.get(id).copied()
    }

    pub fn item_def(&self, id: ItemId) -> &ItemDef {
        &self.items[id.0 as usize]
    }

    /// Talentbaum eines Helden (nach Einheitenkennung).
    pub fn talent_tree(&self, hero: &str) -> Option<&TalentTree> {
        self.talents.iter().find(|t| t.hero == hero)
    }

    pub fn upgrade_id(&self, id: &str) -> Option<usize> {
        self.upgrades.iter().position(|u| u.id == id)
    }

    pub fn good_id(&self, id: &str) -> Option<GoodId> {
        self.good_index.get(id).copied()
    }

    pub fn good(&self, id: GoodId) -> &GoodDef {
        &self.goods[id.0 as usize]
    }

    /// Löst eine Item-ID (Grundressource oder Ware) auf.
    pub fn item(&self, id: &str) -> Option<Item> {
        match id {
            "gold" => Some(Item::Res(ResourceKind::Gold)),
            "wood" => Some(Item::Res(ResourceKind::Wood)),
            "stone" => Some(Item::Res(ResourceKind::Stone)),
            "food" => Some(Item::Res(ResourceKind::Food)),
            other => self.good_id(other).map(Item::Good),
        }
    }

    /// Gesamtzahl der Item-Typen (Grundressourcen + Waren).
    pub fn item_count(&self) -> usize {
        BASE_ITEMS + self.goods.len()
    }

    pub fn item_name(&self, it: Item) -> &str {
        match it {
            Item::Res(k) => k.label(),
            Item::Good(g) => &self.good(g).name,
        }
    }

    pub fn item_id(&self, it: Item) -> &str {
        match it {
            Item::Res(ResourceKind::Gold) => "gold",
            Item::Res(ResourceKind::Wood) => "wood",
            Item::Res(ResourceKind::Stone) => "stone",
            Item::Res(ResourceKind::Food) => "food",
            Item::Good(g) => &self.good(g).id,
        }
    }

    /// Grundpreis in Gold beim Handel.
    pub fn base_price(&self, it: Item) -> f32 {
        match it {
            Item::Res(ResourceKind::Gold) => 1.0,
            Item::Res(ResourceKind::Wood) => self.rules.city.price_wood,
            Item::Res(ResourceKind::Stone) => self.rules.city.price_stone,
            Item::Res(ResourceKind::Food) => self.rules.city.price_food,
            Item::Good(g) => self.good(g).price,
        }
    }

    /// Typtabelle für Spielstände (Reihenfolge der IDs).
    pub fn type_table(&self) -> Vec<String> {
        self.units
            .iter()
            .map(|u| format!("u:{}", u.id))
            .chain(self.buildings.iter().map(|b| format!("b:{}", b.id)))
            .chain(self.abilities.iter().map(|a| format!("a:{}", a.id)))
            .chain(self.goods.iter().map(|g| format!("g:{}", g.id)))
            .chain(self.items.iter().map(|i| format!("i:{}", i.id)))
            .chain(self.upgrades.iter().map(|u| format!("p:{}", u.id)))
            .collect()
    }
}
