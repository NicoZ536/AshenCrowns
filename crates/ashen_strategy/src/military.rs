//! Heere, Bewegung über den Provinzgraphen, Belagerungen, Schlachten (Auto-Resolve nach GDD 4.6) und
//! die Übergabe an die Echtzeitebene (`BattleRequest` / `BattleResult`).

use crate::catalog::{self, UClass};
use crate::chronicle::{ChronicleKind, Entry};
use crate::model::*;
use crate::types::*;
use crate::world::{BattleMode, StrategyEvent, StrategyWorld};
use serde::{Deserialize, Serialize};

pub const ARMY_SPEED: f32 = 12.0;
pub const SEA_SPEED: f32 = 20.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleSide {
    Attackers,
    Defenders,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Rain,
    Snow,
    Storm,
    AshStorm,
    Fog,
}

impl Weather {
    pub fn label(self) -> &'static str {
        match self {
            Weather::Clear => "klar",
            Weather::Rain => "Regen",
            Weather::Snow => "Schnee",
            Weather::Storm => "Sturm",
            Weather::AshStorm => "Aschesturm",
            Weather::Fog => "Nebel",
        }
    }
}

/// Ein beteiligtes Heer in einer Schlacht (Schnappschuss für die Echtzeitebene).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BattleArmy {
    pub army: ArmyId,
    pub realm: Option<RealmId>,
    pub kind: ArmyKind,
    pub name: String,
    pub commander: Option<CharId>,
    pub commander_name: Option<String>,
    /// Truppen mit Einheiten-IDs (`assets/data/units.ron` bzw. fraktionsspezifische IDs) und Stärke.
    pub regiments: Vec<Regiment>,
    pub morale: f32,
    pub supply: f32,
    /// Helden, die mit dem Heer reisen.
    pub heroes: Vec<CharId>,
    /// Kampfkraft (Auto-Resolve-Maß).
    pub power: f32,
}

/// Übergabe einer Schlacht an die Echtzeitebene: Provinz, Terrain, beteiligte Heere, Belagerung, Wetter.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BattleRequest {
    pub id: BattleId,
    pub province: ProvId,
    pub province_key: String,
    pub province_name: String,
    pub terrain: Biome,
    pub season: Season,
    pub weather: Weather,
    /// Tageszeit in Stunden (0–24).
    pub hour: f32,
    pub siege: bool,
    /// Befestigungsstufe (0–10) und Anteil bereits eingestürzter Mauerabschnitte (0–4).
    pub fortification: u8,
    pub breaches: u8,
    pub attackers: Vec<BattleArmy>,
    pub defenders: Vec<BattleArmy>,
    pub day: u32,
    /// Erwartete Verluste bei Auto-Resolve (Mittelwert des Angreifers/Verteidigers in %).
    pub preview_loss_attacker: f32,
    pub preview_loss_defender: f32,
    /// Hohlung der Provinz (für Spawnpunkte und Schleierstärke).
    pub veil: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeroOutcome {
    Unharmed,
    Wounded,
    Captured,
    Dead,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArmyResult {
    pub army: ArmyId,
    /// Überlebende Truppen (Anzahl, Veteranenstufe, mittlere Lebenspunkte).
    pub regiments: Vec<Regiment>,
    pub retreated: bool,
}

/// Ergebnis einer Schlacht – gespielt oder berechnet. Wird mit `apply_battle_result` übernommen.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BattleResult {
    pub id: BattleId,
    pub winner: BattleSide,
    pub armies: Vec<ArmyResult>,
    pub heroes: Vec<(CharId, HeroOutcome)>,
    pub duration_minutes: f32,
    /// Belagerung wurde gestürmt und die Stadt eingenommen.
    pub city_taken: bool,
}

/// Kampfkraft einer Einheitengruppe (GDD 4.6): KK = (DPS × HP)^0.5 × Veteranenfaktor.
pub fn regiment_power(r: &Regiment) -> f32 {
    let Some(u) = catalog::unit(&r.unit) else { return 0.0 };
    let vet = [1.0, 1.1, 1.2, 1.35][r.vet.min(3) as usize];
    u.power() * vet * r.count as f32 * (0.4 + 0.6 * r.hp)
}

pub fn army_power(a: &Army) -> f32 {
    a.regiments.iter().map(regiment_power).sum()
}

fn side_power(armies: &[BattleArmy]) -> f32 {
    armies.iter().map(|a| a.power).sum()
}

impl StrategyWorld {
    // ------------------------------------------------------------------------------------
    // Armeen anlegen und verwalten
    // ------------------------------------------------------------------------------------
    pub fn new_army(&mut self, kind: ArmyKind, owner: Option<RealmId>, loc: ProvId, regiments: Vec<Regiment>, name: &str) -> ArmyId {
        let id = self.next_army;
        self.next_army += 1;
        let day = self.day;
        self.armies.push(Army {
            id,
            name: name.to_string(),
            kind,
            owner,
            target_realm: None,
            house: None,
            commander: None,
            regiments,
            location: loc,
            path: Vec::new(),
            progress: 0.0,
            order: ArmyOrder::Idle,
            morale: 80.0,
            supply: 100.0,
            in_battle: None,
            locked_until: 0,
            war: None,
            created: day,
            unpaid_months: 0,
            boss: None,
            came_from: None,
            wounded_pop: 0.0,
        });
        id
    }

    pub fn army(&self, id: ArmyId) -> Option<&Army> {
        self.armies.iter().find(|a| a.id == id)
    }

    pub fn armies_at(&self, p: ProvId) -> impl Iterator<Item = &Army> {
        self.armies.iter().filter(move |a| a.location == p)
    }

    pub fn realm_armies(&self, r: RealmId) -> Vec<ArmyId> {
        self.armies.iter().filter(|a| a.owner == Some(r) && matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary)).map(|a| a.id).collect()
    }

    pub fn realm_army_power(&self, r: RealmId) -> f32 {
        self.armies.iter().filter(|a| a.owner == Some(r) && matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary)).map(army_power).sum()
    }

    pub fn disband_army(&mut self, id: ArmyId) {
        if let Some(i) = self.armies.iter().position(|a| a.id == id) {
            let a = self.armies.remove(i);
            if let Some(c) = a.commander {
                self.chars[c as usize].army = None;
            }
            for c in self.chars.iter_mut().filter(|c| c.army == Some(id)) {
                c.army = None;
            }
            // Rückkehr in die Bevölkerung (Heimat = Standort oder Hauptstadt)
            let size: f32 = a.size() as f32;
            let dest = match a.owner {
                Some(o) if self.provinces[a.location as usize].owner == Some(o) => a.location,
                Some(o) => self.realms[o as usize].capital,
                None => a.location,
            };
            if matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary) && self.provinces[dest as usize].owner.is_some() {
                self.restore_soldiers(dest, size);
            }
        }
    }

    /// Entlassene Soldaten werden wieder Bevölkerung (Krieger/Freibauern).
    fn restore_soldiers(&mut self, pid: ProvId, n: f32) {
        let p = &mut self.provinces[pid as usize];
        p.pop[Stratum::Warriors.idx()].adults += n * 0.5;
        p.pop[Stratum::Freeholders.idx()].adults += n * 0.5;
    }

    pub fn merge_armies(&mut self, into: ArmyId, from: ArmyId) -> Result<(), String> {
        let (Some(a), Some(b)) = (self.army(into), self.army(from)) else { return Err("Heer nicht gefunden".into()) };
        if into == from || a.location != b.location || a.owner != b.owner {
            return Err("Heere müssen am selben Ort und vom selben Herrn sein".into());
        }
        let bi = self.armies.iter().position(|x| x.id == from).unwrap();
        let b = self.armies.remove(bi);
        for c in self.chars.iter_mut().filter(|c| c.army == Some(from)) {
            c.army = Some(into);
        }
        let ai = self.armies.iter().position(|x| x.id == into).unwrap();
        let a = &mut self.armies[ai];
        for r in b.regiments {
            if let Some(e) = a.regiments.iter_mut().find(|x| x.unit == r.unit && x.vet == r.vet) {
                let tot = e.count + r.count;
                e.hp = (e.hp * e.count as f32 + r.hp * r.count as f32) / tot.max(1) as f32;
                e.count = tot;
            } else {
                a.regiments.push(r);
            }
        }
        a.morale = (a.morale + b.morale) * 0.5;
        Ok(())
    }

    /// Teilt Truppen ab (`take`: Einheiten-ID → Anzahl) in ein neues Heer.
    pub fn split_army(&mut self, id: ArmyId, take: &[(String, u32)]) -> Result<ArmyId, String> {
        let ai = self.armies.iter().position(|x| x.id == id).ok_or("Heer nicht gefunden")?;
        let mut new_regs = Vec::new();
        for (unit, n) in take {
            let a = &mut self.armies[ai];
            let Some(r) = a.regiments.iter_mut().find(|r| &r.unit == unit && r.count > 0) else { return Err(format!("Keine Einheiten vom Typ {unit}")) };
            let k = (*n).min(r.count);
            r.count -= k;
            new_regs.push(Regiment { unit: unit.clone(), count: k, vet: r.vet, hp: r.hp });
        }
        let a = &mut self.armies[ai];
        a.regiments.retain(|r| r.count > 0);
        if new_regs.is_empty() || new_regs.iter().all(|r| r.count == 0) {
            return Err("Nichts abzuteilen".into());
        }
        let (kind, owner, loc) = (a.kind, a.owner, a.location);
        let morale = a.morale;
        let name = format!("{} (Abteilung)", a.name);
        let nid = self.new_army(kind, owner, loc, new_regs, &name);
        self.armies.iter_mut().find(|x| x.id == nid).unwrap().morale = morale;
        Ok(nid)
    }

    // ------------------------------------------------------------------------------------
    // Rekrutierung
    // ------------------------------------------------------------------------------------
    pub fn recruit_cost(&self, r: RealmId, unit: &str, count: u32) -> f32 {
        let Some(u) = catalog::unit(unit) else { return f32::MAX };
        let rl = &self.realms[r as usize];
        let mut disc = 0.0;
        if let Some(m) = rl.offices[Office::Marshal.idx()] {
            disc += self.chars[m as usize].attr(Attr::Martial) as f32 * 0.01;
        }
        let law = match rl.laws.martial_law {
            1 => 0.0,
            _ => 0.0,
        };
        u.cost * count as f32 * (1.0 - disc - law)
    }

    pub fn recruit(&mut self, r: RealmId, pid: ProvId, unit: &str, count: u32) -> Result<ArmyId, String> {
        let u = catalog::unit(unit).ok_or_else(|| format!("Unbekannte Einheit {unit}"))?;
        if u.cost <= 0.0 {
            return Err("Einheit nicht anwerbbar".into());
        }
        if self.provinces[pid as usize].owner != Some(r) || self.provinces[pid as usize].holder() != Some(r) {
            return Err("Provinz gehört nicht dem Reich".into());
        }
        if self.realms[r as usize].regency.is_some() && false {
            return Err("Unter Regentschaft".into());
        }
        let cost = self.recruit_cost(r, unit, count);
        if self.realms[r as usize].gold < cost {
            return Err("Nicht genug Gold".into());
        }
        // Rekrutierungspool: abhängig von der Einheitenklasse
        let pops = match u.class {
            UClass::Cavalry => vec![Stratum::Nobility, Stratum::Warriors],
            UClass::Worker => vec![Stratum::Serfs, Stratum::Freeholders],
            UClass::Siege => vec![Stratum::Artisans, Stratum::Freeholders],
            UClass::Mage => vec![Stratum::Scholars],
            UClass::Support => vec![Stratum::Clergy, Stratum::Scholars],
            _ => vec![Stratum::Warriors, Stratum::Freeholders, Stratum::Serfs],
        };
        let need = (count * u.pop.max(1) as u32) as f32;
        let p = &mut self.provinces[pid as usize];
        let avail: f32 = pops.iter().map(|s| (p.pop[s.idx()].adults * 0.35).max(0.0)).sum();
        if avail < need {
            return Err("Zu wenig wehrfähige Bevölkerung".into());
        }
        let mut left = need;
        for s in &pops {
            let take = (p.pop[s.idx()].adults * 0.35).min(left);
            p.pop[s.idx()].adults -= take;
            left -= take;
            if left <= 0.0 {
                break;
            }
        }
        self.realms[r as usize].gold -= cost;
        // In bestehendes ruhendes Heer eingliedern
        let existing = self.armies.iter().find(|a| a.owner == Some(r) && a.location == pid && a.kind == ArmyKind::Realm && a.path.is_empty() && a.in_battle.is_none() && a.size() < 1500).map(|a| a.id);
        let reg = Regiment { unit: unit.to_string(), count, vet: 0, hp: 1.0 };
        let id = match existing {
            Some(id) => {
                let a = self.armies.iter_mut().find(|a| a.id == id).unwrap();
                if let Some(e) = a.regiments.iter_mut().find(|x| x.unit == unit && x.vet == 0) {
                    e.count += count;
                } else {
                    a.regiments.push(reg);
                }
                a.morale = a.morale.min(90.0);
                id
            }
            None => {
                let name = format!("Heer von {}", self.pname(pid));
                self.new_army(ArmyKind::Realm, Some(r), pid, vec![reg], &name)
            }
        };
        self.assign_commander(id);
        Ok(id)
    }

    /// Setzt den besten verfügbaren Befehlshaber an die Spitze eines Heeres.
    pub fn assign_commander(&mut self, id: ArmyId) {
        let Some(a) = self.army(id) else { return };
        if a.commander.is_some() || a.size() < 60 {
            return;
        }
        let Some(owner) = a.owner else { return };
        let used: Vec<CharId> = self.armies.iter().filter_map(|x| x.commander).collect();
        let day = self.day;
        let marshal = self.realms[owner as usize].offices[Office::Marshal.idx()];
        let cand = self.realms[owner as usize].court.iter().copied().filter(|c| {
            let ch = &self.chars[*c as usize];
            ch.alive() && ch.adult(day) && ch.age(day) < 62 && ch.captive_of.is_none() && ch.army.is_none() && ch.wounded_until <= day && !used.contains(c) && Some(*c) != marshal && !ch.has(crate::catalog::Trait::Maimed) && *c != self.realms[owner as usize].ruler
        });
        let best = cand.max_by_key(|c| self.chars[*c as usize].attr(Attr::Martial) * 3 + self.chars[*c as usize].abilities.len() as i32);
        // Der Herrscher führt persönlich, wenn er kriegerisch ist und niemand sonst da ist
        let ruler = self.realms[owner as usize].ruler;
        let chosen = best.or_else(|| {
            let r = &self.chars[ruler as usize];
            (r.alive() && r.adult(day) && r.army.is_none() && r.age(day) < 60 && r.attr(Attr::Martial) >= 10 && !r.has(crate::catalog::Trait::Craven)).then_some(ruler)
        });
        if let Some(c) = chosen
            && self.chars[c as usize].attr(Attr::Martial) >= 4 {
                self.armies.iter_mut().find(|x| x.id == id).unwrap().commander = Some(c);
                self.chars[c as usize].army = Some(id);
                self.chars[c as usize].location = self.army(id).unwrap().location;
            }
    }

    // ------------------------------------------------------------------------------------
    // Bewegung
    // ------------------------------------------------------------------------------------
    /// Darf `owner` die Provinz betreten?
    pub fn can_enter(&self, owner: RealmId, pid: ProvId) -> bool {
        let p = &self.provinces[pid as usize];
        match p.holder() {
            None => true,
            Some(h) => {
                h == owner || self.at_war(owner, h) || self.same_lord(owner, h) || self.has_treaty(TreatyKind::Alliance, owner, h) || self.has_treaty(TreatyKind::MilitaryAccess, owner, h) || self.has_treaty(TreatyKind::DefensivePact, owner, h) || self.has_treaty(TreatyKind::JointCampaign, owner, h)
            }
        }
    }

    /// Gemeinsamer Lehnsherr oder Lehnsverhältnis.
    pub fn same_lord(&self, a: RealmId, b: RealmId) -> bool {
        let (ra, rb) = (&self.realms[a as usize], &self.realms[b as usize]);
        ra.vassal_of == Some(b) || rb.vassal_of == Some(a) || (ra.vassal_of.is_some() && ra.vassal_of == rb.vassal_of)
    }

    /// Kantendauer in Tagen.
    pub fn edge_days(&self, from: ProvId, link: &Link) -> f32 {
        let to = &self.provinces[link.to as usize];
        let speed = if link.kind == LinkKind::Sea { SEA_SPEED } else { ARMY_SPEED };
        let mut d = link.dist / speed * if link.kind == LinkKind::Land { to.biome.move_cost() } else { 1.0 };
        if self.season() == Season::Winter && link.kind == LinkKind::Land && matches!(to.biome, Biome::Mountain | Biome::Tundra) {
            d *= 1.3;
        }
        let _ = from;
        d.max(1.0)
    }

    pub fn can_sail(&self, owner: RealmId) -> bool {
        self.realms[owner as usize].fleet > 0
    }

    /// Kürzester Weg (Dijkstra) für ein Reich; `None`, wenn nicht erreichbar.
    pub fn find_path(&self, owner: RealmId, from: ProvId, to: ProvId) -> Option<Vec<ProvId>> {
        let n = self.provinces.len();
        let mut dist = vec![f32::MAX; n];
        let mut prev: Vec<Option<ProvId>> = vec![None; n];
        let mut done = vec![false; n];
        dist[from as usize] = 0.0;
        let sail = self.can_sail(owner);
        loop {
            let mut u = None;
            let mut best = f32::MAX;
            for i in 0..n {
                if !done[i] && dist[i] < best {
                    best = dist[i];
                    u = Some(i);
                }
            }
            let Some(u) = u else { break };
            if u == to as usize {
                break;
            }
            done[u] = true;
            for l in &self.provinces[u].neighbors {
                if l.kind == LinkKind::Sea && !(sail && (self.provinces[u].coastal || self.provinces[u].biome == Biome::Island) && (self.provinces[l.to as usize].coastal || self.provinces[l.to as usize].biome == Biome::Island)) {
                    continue;
                }
                if l.to != to && !self.can_enter(owner, l.to) {
                    continue;
                }
                if l.to == to && !self.can_enter(owner, l.to) {
                    continue;
                }
                let w = self.edge_days(u as ProvId, l);
                let nd = dist[u] + w;
                if nd < dist[l.to as usize] {
                    dist[l.to as usize] = nd;
                    prev[l.to as usize] = Some(u as ProvId);
                }
            }
        }
        if dist[to as usize] == f32::MAX {
            return None;
        }
        let mut path = vec![to];
        let mut cur = to;
        while let Some(p) = prev[cur as usize] {
            if p == from {
                break;
            }
            path.push(p);
            cur = p;
        }
        path.reverse();
        Some(path)
    }

    /// Befehl: Heer bewegt sich in die Zielprovinz.
    pub fn order_move(&mut self, army: ArmyId, to: ProvId) -> Result<(), String> {
        let a = self.army(army).ok_or("Heer nicht gefunden")?;
        if a.in_battle.is_some() {
            return Err("Heer ist in einer Schlacht".into());
        }
        if a.locked_until > self.day {
            return Err("Heer ruht nach der Schlacht".into());
        }
        let owner = a.owner.ok_or("Heer ohne Herrn")?;
        let from = a.location;
        if from == to {
            let a = self.armies.iter_mut().find(|x| x.id == army).unwrap();
            a.path.clear();
            a.order = ArmyOrder::Idle;
            return Ok(());
        }
        let path = self.find_path(owner, from, to).ok_or("Kein Weg zur Zielprovinz")?;
        let a = self.armies.iter_mut().find(|x| x.id == army).unwrap();
        a.path = path;
        a.progress = 0.0;
        a.order = ArmyOrder::Move;
        Ok(())
    }

    fn army_speed_factor(&self, a: &Army) -> f32 {
        let mut slowest = 1.0f32;
        for r in &a.regiments {
            if let Some(u) = catalog::unit(&r.unit) {
                let f = match u.class {
                    UClass::Cavalry => 1.3,
                    UClass::Siege => 0.7,
                    _ => 1.0,
                };
                if f < slowest || slowest == 1.0 && f > 1.0 && a.regiments.len() == 1 {
                    slowest = f;
                }
            }
        }
        let mut f = slowest.min(1.0);
        if a.regiments.iter().all(|r| catalog::unit(&r.unit).is_some_and(|u| u.class == UClass::Cavalry)) {
            f = 1.3;
        }
        if let Some(c) = a.commander
            && self.chars[c as usize].abilities.contains(&Ability::Logistician) {
                f *= 1.1;
            }
        f
    }

    // ------------------------------------------------------------------------------------
    // Tagesverarbeitung
    // ------------------------------------------------------------------------------------
    pub(crate) fn daily_military(&mut self) {
        let day = self.day;
        // Bewegung
        let ids: Vec<ArmyId> = self.armies.iter().filter(|a| !a.path.is_empty() && a.in_battle.is_none() && a.locked_until <= day).map(|a| a.id).collect();
        let mut arrived: Vec<ArmyId> = Vec::new();
        for id in ids {
            let Some(i) = self.armies.iter().position(|a| a.id == id) else { continue };
            let from = self.armies[i].location;
            let next = self.armies[i].path[0];
            let Some(link) = self.provinces[from as usize].neighbors.iter().find(|l| l.to == next).copied() else {
                self.armies[i].path.clear();
                continue;
            };
            let days = self.edge_days(from, &link);
            let sf = self.army_speed_factor(&self.armies[i]);
            self.armies[i].progress += sf / days;
            if self.armies[i].progress >= 1.0 {
                // Betreten verboten? (Frieden ausgebrochen)
                if let Some(o) = self.armies[i].owner
                    && matches!(self.armies[i].kind, ArmyKind::Realm | ArmyKind::Mercenary) && !self.can_enter(o, next) {
                        self.armies[i].path.clear();
                        self.armies[i].progress = 0.0;
                        self.armies[i].order = ArmyOrder::Idle;
                        continue;
                    }
                self.armies[i].progress = 0.0;
                self.armies[i].came_from = Some(from);
                self.armies[i].location = next;
                self.armies[i].path.remove(0);
                if self.armies[i].path.is_empty() {
                    self.armies[i].order = if self.armies[i].order == ArmyOrder::Move { ArmyOrder::Idle } else { self.armies[i].order };
                }
                if let Some(c) = self.armies[i].commander {
                    self.chars[c as usize].location = next;
                }
                arrived.push(id);
            }
        }
        // Schlachten und Besetzung
        let mut provs: Vec<ProvId> = self.armies.iter().filter(|a| a.in_battle.is_none()).map(|a| a.location).collect();
        provs.sort_unstable();
        provs.dedup();
        for p in provs {
            self.resolve_province(p, &arrived);
        }
        self.advance_sieges();
        // Verwundete kehren zurück
        self.pending_battle_timeouts();
    }

    /// Wer steht auf welcher Seite? Freundlich = gleicher Herr, gleiche Kriegspartei, Lehnsverhältnis.
    fn friendly(&self, a: &Army, b: &Army) -> bool {
        match (a.owner, b.owner) {
            (Some(x), Some(y)) if matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary) && matches!(b.kind, ArmyKind::Realm | ArmyKind::Mercenary) => {
                x == y || self.same_lord(x, y) && !self.at_war(x, y) || self.same_side(x, y)
            }
            _ => a.kind == b.kind && a.owner == b.owner,
        }
    }

    pub fn same_side(&self, x: RealmId, y: RealmId) -> bool {
        self.wars.iter().any(|w| w.active && ((w.attackers.contains(&x) && w.attackers.contains(&y)) || (w.defenders.contains(&x) && w.defenders.contains(&y))))
    }

    pub fn armies_hostile(&self, a: &Army, b: &Army) -> bool {
        use ArmyKind::*;
        let realm_like = |k: ArmyKind| matches!(k, Realm | Mercenary);
        if realm_like(a.kind) && realm_like(b.kind) {
            return match (a.owner, b.owner) {
                (Some(x), Some(y)) => x != y && self.at_war(x, y),
                _ => false,
            };
        }
        if realm_like(a.kind) != realm_like(b.kind) {
            let (r, n) = if realm_like(a.kind) { (a, b) } else { (b, a) };
            if n.kind == Boss {
                return self.boss_hostile_to(n, r);
            }
            return match n.kind {
                Rebels => n.target_realm == r.owner || n.target_realm.zip(r.owner).is_some_and(|(t, o)| self.same_lord(t, o)),
                Refugees => false,
                _ => true,
            };
        }
        // Neutral gegen neutral: nur Rebellen gegen Monster/Banditen → nein
        false
    }

    /// Alle Armeen einer Provinz auf Feindschaft prüfen, Schlacht oder Besetzung/Belagerung auslösen.
    fn resolve_province(&mut self, pid: ProvId, arrived: &[ArmyId]) {
        let day = self.day;
        // 1. Feldschlacht
        let here: Vec<ArmyId> = self.armies.iter().filter(|a| a.location == pid && a.in_battle.is_none() && a.locked_until <= day).map(|a| a.id).collect();
        if here.len() >= 2 {
            // Angreifer = zuletzt eingetroffenes Heer
            let atk_first = here.iter().copied().find(|id| arrived.contains(id));
            for &a_id in &here {
                let Some(a) = self.army(a_id).cloned() else { continue };
                if a.in_battle.is_some() {
                    continue;
                }
                let Some(b_id) = here.iter().copied().find(|&b| b != a_id && self.army(b).is_some_and(|b| b.in_battle.is_none() && self.armies_hostile(&a, b))) else { continue };
                let b = self.army(b_id).unwrap().clone();
                let (att, def) = if atk_first == Some(a_id) || (atk_first != Some(b_id) && arrived.contains(&a_id)) { (a, b) } else { (b, a) };
                // Seiten sammeln
                let mut atk_ids = Vec::new();
                let mut def_ids = Vec::new();
                for &x in &here {
                    let Some(xa) = self.army(x) else { continue };
                    if xa.in_battle.is_some() {
                        continue;
                    }
                    if self.friendly(&att, xa) {
                        atk_ids.push(x);
                    } else if self.friendly(&def, xa) {
                        def_ids.push(x);
                    }
                }
                if atk_ids.is_empty() || def_ids.is_empty() {
                    continue;
                }
                self.start_battle(pid, atk_ids, def_ids, false);
                break;
            }
        }
        // 2. Besetzung und Belagerung durch Heere in Feindesland
        let Some(h) = self.provinces[pid as usize].holder() else { return };
        let owner = self.provinces[pid as usize].owner;
        let hostile_here: Vec<ArmyId> = self
            .armies
            .iter()
            .filter(|a| a.location == pid && a.in_battle.is_none() && matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary | ArmyKind::Rebels))
            .filter(|a| match a.kind {
                ArmyKind::Rebels => a.target_realm == owner && a.target_realm.is_some(),
                _ => a.owner.is_some_and(|o| o != h && self.at_war(o, h)),
            })
            .map(|a| a.id)
            .collect();
        // Verteidigende Heere am Ort verhindern die Belagerung
        let defenders_present = self.armies.iter().any(|a| a.location == pid && a.in_battle.is_none() && matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary) && a.owner.is_some_and(|o| o == h || self.same_side_as_holder(o, h)));
        if hostile_here.is_empty() || defenders_present {
            return;
        }
        let lead = hostile_here[0];
        let (kind, a_owner, power) = {
            let a = self.army(lead).unwrap();
            (a.kind, a.owner, hostile_here.iter().filter_map(|i| self.army(*i)).map(army_power).sum::<f32>())
        };
        let p = &self.provinces[pid as usize];
        let garrison_power = p.garrison as f32 * 28.0 * (1.0 + p.fortification as f32 * 0.1);
        if p.siege.is_some() {
            return;
        }
        let fortified = p.fortification > 0 || p.settlement.rank() >= 1;
        if kind == ArmyKind::Rebels {
            // Rebellen gegen Garnison
            if power > garrison_power * 0.9 || p.garrison == 0 {
                self.provinces[pid as usize].rebel_held = true;
                self.provinces[pid as usize].captured_day = day;
                let name = self.pname(pid);
                let _ = name;
            } else {
                // Garnison schlägt Rebellen nieder
                self.crush_rebels(pid, owner);
            }
            return;
        }
        let Some(ao) = a_owner else { return };
        if fortified && garrison_power > power * 0.05 {
            self.provinces[pid as usize].siege = Some(Siege { besieger: ao, army: lead, progress: 0.0, started: day });
            self.stats.sieges += 1;
            let (pn, an) = (self.pname(pid), self.rname(ao));
            let e = Entry::new(ChronicleKind::Siege, "siege_start", 22).prov(pid, &pn).realm(ao).var("a", an);
            self.chron(e);
            self.armies.iter_mut().find(|a| a.id == lead).unwrap().order = ArmyOrder::Siege;
        } else {
            self.occupy_province(pid, ao);
        }
    }

    fn same_side_as_holder(&self, o: RealmId, h: RealmId) -> bool {
        o == h || self.same_lord(o, h) || self.has_treaty(TreatyKind::Alliance, o, h)
    }

    fn crush_rebels(&mut self, pid: ProvId, owner: Option<RealmId>) {
        let ids: Vec<ArmyId> = self.armies.iter().filter(|a| a.location == pid && a.kind == ArmyKind::Rebels).map(|a| a.id).collect();
        for id in ids {
            self.armies.retain(|a| a.id != id);
        }
        self.provinces[pid as usize].rebel_held = false;
        self.provinces[pid as usize].unrest = (self.provinces[pid as usize].unrest * 0.6).max(10.0);
        if let Some(o) = owner {
            let pn = self.pname(pid);
            let e = Entry::new(ChronicleKind::Revolt, "revolt_crushed", 30).prov(pid, &pn).realm(o).var("a", self.rname(o)).cause(self.provinces[pid as usize].chain.revolt);
            self.chron(e);
        }
    }

    /// Provinz wird von einem Heer besetzt (Kontrolle wechselt, Besitz bleibt bis zum Frieden).
    pub fn occupy_province(&mut self, pid: ProvId, by: RealmId) {
        let day = self.day;
        let old_holder = self.provinces[pid as usize].holder();
        if old_holder == Some(by) {
            return;
        }
        let p = &mut self.provinces[pid as usize];
        p.controller = if p.owner == Some(by) { None } else { Some(by) };
        p.siege = None;
        p.captured_day = day;
        // Kriegspunkte: besetzte Provinz
        let val = province_value(&self.provinces[pid as usize], self.provinces[pid as usize].owner.map(|o| self.realms[o as usize].capital == pid).unwrap_or(false));
        self.war_score_for_occupation(by, old_holder, val);
        let (pn, an) = (self.pname(pid), self.rname(by));
        let bn = old_holder.map(|h| self.rname(h)).unwrap_or_default();
        let mag = if self.provinces[pid as usize].settlement.rank() >= 2 { 40 } else { 24 };
        let e = Entry::new(ChronicleKind::Siege, "siege_fall", mag).prov(pid, &pn).realm(by).var("a", an).var("b", bn).tag("occupation");
        self.chron(e);
        self.emit(StrategyEvent::ProvinceChanged { province: pid, old: old_holder, new: Some(by) });
        // Kriegsmüdigkeit des Besetzten
        if let Some(h) = old_holder {
            self.realms[h as usize].war_weariness = (self.realms[h as usize].war_weariness + 2.0).min(100.0);
        }
        // Flüchtlinge
        let pop = self.provinces[pid as usize].population();
        self.add_refugees_from(pid, pop * 0.02);
    }

    /// Setzt Flüchtlinge auf die nächste sichere Nachbarprovinz.
    pub fn add_refugees_from(&mut self, pid: ProvId, amount: f32) {
        if amount <= 0.0 {
            return;
        }
        let owner = self.provinces[pid as usize].owner;
        let mut best: Option<(ProvId, f32)> = None;
        for l in &self.provinces[pid as usize].neighbors {
            let q = &self.provinces[l.to as usize];
            if q.biome == Biome::Hollowland || q.population() < 30.0 {
                continue;
            }
            let safe = q.holder() == owner || q.holder().is_some_and(|h| owner.is_some_and(|o| !self.at_war(o, h)));
            let s = if safe { 2.0 } else { 0.5 } + q.avg_satisfaction() / 100.0;
            if best.is_none_or(|b| s > b.1) {
                best = Some((l.to, s));
            }
        }
        self.add_population(pid as usize, -amount, false);
        if let Some((q, _)) = best {
            self.provinces[q as usize].refugees += amount;
        }
    }

    // ------------------------------------------------------------------------------------
    // Belagerungen
    // ------------------------------------------------------------------------------------
    fn siege_strength(&self, army: &Army) -> f32 {
        let mut s = 0.0;
        for r in &army.regiments {
            if let Some(u) = catalog::unit(&r.unit) {
                if u.class == UClass::Siege {
                    s += 3.0 * r.count as f32;
                } else if matches!(u.class, UClass::Infantry | UClass::Worker) {
                    s += r.count as f32 / 250.0;
                }
            }
        }
        if let Some(c) = army.commander {
            let ch = &self.chars[c as usize];
            if ch.abilities.contains(&Ability::SiegeMaster) {
                s += 3.0;
            }
            if ch.has(crate::catalog::Trait::Patient) {
                s *= 1.1;
            }
        }
        s
    }

    fn advance_sieges(&mut self) {
        let day = self.day;
        let ids: Vec<ProvId> = self.provinces.iter().filter(|p| p.siege.is_some()).map(|p| p.id).collect();
        for pid in ids {
            let siege = self.provinces[pid as usize].siege.clone().unwrap();
            let Some(army) = self.army(siege.army).cloned() else {
                self.provinces[pid as usize].siege = None;
                continue;
            };
            if army.location != pid || army.in_battle.is_some() {
                if army.location != pid {
                    self.provinces[pid as usize].siege = None;
                }
                continue;
            }
            // Entsatzheer?
            let owner = self.provinces[pid as usize].holder();
            let strength = self.siege_strength(&army);
            let fort = self.provinces[pid as usize].fortification as f32;
            let mut gain = (strength - fort * 2.0).max(0.5);
            if self.realm_has_wall_ward(pid) {
                gain *= 0.8;
            }
            let p = &mut self.provinces[pid as usize];
            if let Some(s) = p.siege.as_mut() {
                s.progress += gain;
            }
            // Versorgung: erschöpft → Garnison verliert 2 %/Tag, Zufriedenheit −3
            let food: f32 = [Good2::Grain, Good2::Bread, Good2::Meat, Good2::Fish, Good2::Preserved].iter().map(|g| p.stock[g.idx()]).sum();
            if food < 5.0 {
                for s in p.pop.iter_mut() {
                    s.satisfaction = (s.satisfaction - 3.0 / 30.0).max(0.0);
                }
                p.pop[Stratum::Warriors.idx()].adults *= 0.98;
            }
            p.pop[Stratum::Warriors.idx()].adults = p.pop[Stratum::Warriors.idx()].adults.max(0.0);
            let progress = p.siege.as_ref().map(|s| s.progress).unwrap_or(0.0);
            // Verteidiger kann ausfallen: KI lässt das ausfallen, wenn Stärke vorhanden – hier nicht.
            if progress >= 100.0 {
                let by = siege.besieger;
                if army.kind == ArmyKind::Rebels {
                    self.provinces[pid as usize].siege = None;
                    continue;
                }
                self.occupy_province(pid, by);
                continue;
            }
            // Sturm durch KI-gesteuerte Belagerer (Spieler stürmt per Befehl)
            let besieger_is_player = self.is_player(siege.besieger);
            if !besieger_is_player && progress >= 40.0 && day.is_multiple_of(5) {
                let gp = self.provinces[pid as usize].garrison as f32 * 28.0 * (1.0 + fort * 0.1 * (1.0 - (progress / 100.0).min(1.0)));
                if army_power(&army) > gp * 2.5 {
                    let _ = self.storm_siege(pid);
                }
            }
            let _ = owner;
        }
    }

    fn realm_has_wall_ward(&self, pid: ProvId) -> bool {
        self.provinces[pid as usize].has_state("rune_wall", self.day)
    }

    /// Sturmangriff auf eine belagerte Stadt (RTS-Belagerungsschlacht oder automatisch).
    pub fn storm_siege(&mut self, pid: ProvId) -> Result<(), String> {
        let siege = self.provinces[pid as usize].siege.clone().ok_or("Keine Belagerung")?;
        let atk = self.army(siege.army).ok_or("Belagerer fehlt")?.id;
        let atk_ids = vec![atk];
        let def_ids: Vec<ArmyId> = Vec::new();
        self.start_battle(pid, atk_ids, def_ids, true);
        Ok(())
    }

    // ------------------------------------------------------------------------------------
    // Schlachten
    // ------------------------------------------------------------------------------------
    fn weather_for(&mut self, pid: ProvId) -> Weather {
        let p = &self.provinces[pid as usize];
        let (season, veil, rain) = (self.season(), p.veil, p.rainfall);
        let r = self.rf();
        if veil >= 3.0 && r < 0.2 {
            Weather::AshStorm
        } else if season == Season::Winter && r < 0.4 {
            Weather::Snow
        } else if rain > 1.3 && r < 0.4 {
            Weather::Storm
        } else if rain > 1.0 && r < 0.4 {
            Weather::Rain
        } else if r > 0.9 {
            Weather::Fog
        } else {
            Weather::Clear
        }
    }

    fn make_battle_army(&self, id: ArmyId) -> BattleArmy {
        let a = self.army(id).unwrap();
        let heroes: Vec<CharId> = self.chars.iter().filter(|c| c.army == Some(id) && c.alive() && c.is_hero).map(|c| c.id).collect();
        BattleArmy {
            army: id,
            realm: a.owner,
            kind: a.kind,
            name: a.name.clone(),
            commander: a.commander,
            commander_name: a.commander.map(|c| self.full_name(c)),
            regiments: a.regiments.clone(),
            morale: a.morale,
            supply: a.supply,
            heroes,
            power: army_power(a),
        }
    }

    /// Baut aus Heeren eine Schlacht. Im Modus `PlayerBattlesRequested` mit Spielerbeteiligung wird ein
    /// `BattleRequest` hinterlegt, sonst sofort automatisch aufgelöst.
    pub fn start_battle(&mut self, pid: ProvId, atk_ids: Vec<ArmyId>, def_ids: Vec<ArmyId>, siege: bool) -> Option<BattleId> {
        let id = self.next_battle;
        self.next_battle += 1;
        let weather = self.weather_for(pid);
        let hour = 6.0 + self.rf() * 12.0;
        let mut attackers: Vec<BattleArmy> = atk_ids.iter().map(|i| self.make_battle_army(*i)).collect();
        let mut defenders: Vec<BattleArmy> = def_ids.iter().map(|i| self.make_battle_army(*i)).collect();
        let p = &self.provinces[pid as usize];
        let fort = p.fortification;
        // Garnison verteidigt befestigte Städte
        if siege || (fort > 0 && defenders.is_empty()) {
            let g = p.garrison.max(1);
            if let Some(h) = p.holder() {
                let _ = h;
            }
            let regs = vec![Regiment { unit: "spearman".into(), count: g * 6 / 10 + 1, vet: 0, hp: 1.0 }, Regiment { unit: "archer".into(), count: g * 4 / 10 + 1, vet: 0, hp: 1.0 }];
            let power: f32 = regs.iter().map(regiment_power).sum();
            defenders.push(BattleArmy { army: u32::MAX, realm: p.holder(), kind: ArmyKind::Realm, name: format!("Garnison von {}", p.name), commander: None, commander_name: None, regiments: regs, morale: 70.0, supply: 100.0, heroes: Vec::new(), power });
        }
        if attackers.is_empty() || defenders.is_empty() {
            self.next_battle -= 1;
            return None;
        }
        let breaches = p.siege.as_ref().map(|s| (s.progress / 25.0).floor() as u8).unwrap_or(0).min(4);
        let mut req = BattleRequest {
            id,
            province: pid,
            province_key: p.key.clone(),
            province_name: p.name.clone(),
            terrain: p.biome,
            season: self.season(),
            weather,
            hour,
            siege,
            fortification: if siege || fort > 0 && p.settlement.rank() >= 1 { fort } else { 0 },
            breaches,
            attackers: std::mem::take(&mut attackers),
            defenders: std::mem::take(&mut defenders),
            day: self.day,
            preview_loss_attacker: 0.0,
            preview_loss_defender: 0.0,
            veil: p.veil,
        };
        // Vorschau: Verluste im Mittel aus drei Probeläufen (mit geklontem Zufall → keine Zustandsänderung)
        let (la, ld) = self.battle_preview(&req);
        req.preview_loss_attacker = la;
        req.preview_loss_defender = ld;
        let player_involved = self.player.is_some_and(|pl| req.attackers.iter().chain(req.defenders.iter()).any(|b| b.realm == Some(pl) && b.army != u32::MAX));
        for b in req.attackers.iter().chain(req.defenders.iter()) {
            if let Some(a) = self.armies.iter_mut().find(|a| a.id == b.army) {
                a.in_battle = Some(id);
            }
        }
        if self.options.rules.battle_mode == BattleMode::PlayerBattlesRequested && player_involved {
            self.battles.push(req);
            self.emit(StrategyEvent::BattleRequested { id });
            return Some(id);
        }
        let result = self.auto_resolve(&req);
        let _ = self.apply_battle_result_inner(&req, result);
        Some(id)
    }

    fn battle_preview(&self, req: &BattleRequest) -> (f32, f32) {
        let a = side_power(&req.attackers).max(1.0);
        let d = side_power(&req.defenders).max(1.0) * self.defender_bonus(req);
        let ratio_a = (d / a).clamp(0.1, 4.0);
        let loss_a = (ratio_a * 0.45).clamp(0.05, 1.0) * 100.0;
        let loss_d = (1.0 / ratio_a * 0.45).clamp(0.05, 1.0) * 100.0;
        (loss_a, loss_d)
    }

    fn defender_bonus(&self, req: &BattleRequest) -> f32 {
        let mut f = req.terrain.defense_bonus();
        if req.siege || req.fortification > 0 {
            let wall = req.fortification as f32 * 0.08 * (1.0 - req.breaches as f32 / 4.0);
            f *= 1.0 + wall;
        }
        f
    }

    /// Liefert alle auf ein Ergebnis wartenden Schlachten (zur Übergabe an die Echtzeitebene).
    pub fn pending_battles(&self) -> &[BattleRequest] {
        &self.battles
    }

    /// Löst eine wartende Schlacht automatisch auf (z. B. wenn der Spieler „Automatisch berechnen“ wählt).
    pub fn auto_resolve_pending(&mut self, id: BattleId) -> Result<(), String> {
        let idx = self.battles.iter().position(|b| b.id == id).ok_or("Schlacht nicht gefunden")?;
        let req = self.battles.remove(idx);
        let result = self.auto_resolve(&req);
        self.apply_battle_result_inner(&req, result)
    }

    /// Übernimmt das Ergebnis einer in der Echtzeitebene gespielten Schlacht.
    pub fn apply_battle_result(&mut self, result: BattleResult) -> Result<(), String> {
        let idx = self.battles.iter().position(|b| b.id == result.id).ok_or("Schlacht nicht gefunden")?;
        let req = self.battles.remove(idx);
        self.apply_battle_result_inner(&req, result)
    }

    pub(crate) fn pending_battle_timeouts(&mut self) {
        let day = self.day;
        let stale: Vec<BattleId> = self.battles.iter().filter(|b| day.saturating_sub(b.day) > 30).map(|b| b.id).collect();
        for id in stale {
            let _ = self.auto_resolve_pending(id);
        }
    }

    /// Auto-Resolve nach GDD 4.6 in Runden à 10 s.
    pub fn auto_resolve(&mut self, req: &BattleRequest) -> BattleResult {
        // Zustand der Seiten: Regimenter als Arbeitskopien
        let mut sides: [Vec<(usize, Regiment)>; 2] = [Vec::new(), Vec::new()];
        let mut armies_of: [Vec<&BattleArmy>; 2] = [Vec::new(), Vec::new()];
        for (si, list) in [&req.attackers, &req.defenders].iter().enumerate() {
            for (ai, b) in list.iter().enumerate() {
                armies_of[si].push(b);
                for r in &b.regiments {
                    sides[si].push((ai, r.clone()));
                }
            }
        }
        let init_strength: [f32; 2] = [sides[0].iter().map(|(_, r)| r.count as f32).sum::<f32>().max(1.0), sides[1].iter().map(|(_, r)| r.count as f32).sum::<f32>().max(1.0)];
        let mut morale = [0.0f32; 2];
        let mut supply = [0.0f32; 2];
        let mut cmd = [1.0f32; 2];
        for si in 0..2 {
            let n = armies_of[si].len().max(1) as f32;
            morale[si] = armies_of[si].iter().map(|b| b.morale).sum::<f32>() / n;
            supply[si] = armies_of[si].iter().map(|b| b.supply).sum::<f32>() / n;
            let mut best = 0.0f32;
            for b in &armies_of[si] {
                if let Some(c) = b.commander {
                    let ch = &self.chars[c as usize];
                    let mut v = 0.02 * ch.attr(Attr::Martial) as f32;
                    if ch.abilities.contains(&Ability::Tactician) {
                        v += 0.05;
                    }
                    if ch.has(crate::catalog::Trait::Brave) {
                        v += 0.02;
                    }
                    if ch.has(crate::catalog::Trait::Strategist) {
                        v += 0.03;
                    }
                    best = best.max(v);
                }
                // Helden: pauschaler Bonus
                best += 0.03 * b.heroes.len() as f32;
            }
            cmd[si] = 1.0 + best;
        }
        // Hohle sind nachts stärker (+15 %)
        let night = req.hour < 6.0 || req.hour > 19.0;
        let weather_f = match req.weather {
            Weather::Storm | Weather::AshStorm => 0.95,
            Weather::Fog => 0.97,
            _ => 1.0,
        };
        let def_bonus = self.defender_bonus(req);
        // Rundenschleife
        let mut rounds = 0;
        let mut broke: Option<usize> = None;
        let mut hero_lost: Vec<(CharId, HeroOutcome)> = Vec::new();
        let hero_chars: [Vec<CharId>; 2] = [req.attackers.iter().flat_map(|b| b.heroes.iter().copied().chain(b.commander)).collect(), req.defenders.iter().flat_map(|b| b.heroes.iter().copied().chain(b.commander)).collect()];
        while rounds < 80 {
            rounds += 1;
            // Kampfkraft je Seite
            let mut kk = [0.0f32; 2];
            for si in 0..2 {
                // Zusammensetzungsbonus gegenüber der Gegnerseite
                let enemy = &sides[1 - si];
                let (e_cav, e_rng, e_und, e_tot) = {
                    let mut c = 0.0;
                    let mut r = 0.0;
                    let mut u = 0.0;
                    let mut t = 0.0;
                    for (_, x) in enemy.iter() {
                        if let Some(d) = catalog::unit(&x.unit) {
                            t += x.count as f32;
                            if d.class == UClass::Cavalry {
                                c += x.count as f32;
                            }
                            if d.is_ranged() {
                                r += x.count as f32;
                            }
                            if d.undead {
                                u += x.count as f32;
                            }
                        }
                    }
                    (c, r, u, t.max(1.0))
                };
                let mut sum = 0.0;
                for (_, x) in &sides[si] {
                    if x.count == 0 {
                        continue;
                    }
                    let Some(d) = catalog::unit(&x.unit) else { continue };
                    let mut mult = 1.0;
                    mult += (d.vs_cav - 1.0) * (e_cav / e_tot);
                    mult += (d.vs_ranged - 1.0) * (e_rng / e_tot);
                    mult += (d.vs_undead - 1.0) * (e_und / e_tot);
                    if night && d.undead {
                        mult *= 1.15;
                    }
                    sum += regiment_power(x) * mult;
                }
                let terrain = if si == 1 { def_bonus } else { 1.0 };
                let moral = 0.5 + morale[si].clamp(0.0, 100.0) / 200.0;
                let supply_f = if supply[si] < 30.0 { 0.9 } else { 1.0 };
                // Hohle sind nachts stärker
                kk[si] = sum * terrain * cmd[si] * moral * supply_f * weather_f;
            }
            // Siegbonus der Legion bleibt in den Einheitenwerten
            let rnd = [0.85 + self.rf() * 0.3, 0.85 + self.rf() * 0.3];
            let kk_r = [kk[0] * rnd[0], kk[1] * rnd[1]];
            // Verluste
            let mut loss_hp = [kk_r[1] * 0.8 * 10.0 / 10.0, kk_r[0] * 0.8];
            // Auflösung der Lebenspunkte in Truppenverluste: Frontreihenfolge
            for si in 0..2 {
                let mut lh = loss_hp[si];
                // Reihenfolge: Nahkampf vorn, Fernkampf/Magie hinten
                let mut order: Vec<usize> = (0..sides[si].len()).collect();
                order.sort_by_key(|i| {
                    let c = catalog::unit(&sides[si][*i].1.unit).map(|u| u.class).unwrap_or(UClass::Infantry);
                    match c {
                        UClass::Infantry | UClass::Worker | UClass::Monster | UClass::Boss => 0,
                        UClass::Cavalry => 1,
                        UClass::Siege => 2,
                        _ => 3,
                    }
                });
                // Verluste: Schaden gegen HP ≈ Gesamtleben
                let before: f32 = sides[si].iter().map(|(_, r)| r.count as f32).sum();
                for i in order {
                    if lh <= 0.0 {
                        break;
                    }
                    let (_, reg) = &mut sides[si][i];
                    let Some(d) = catalog::unit(&reg.unit) else { continue };
                    let unit_hp = d.hp * (1.0 + d.armor * 0.06) * reg.hp.max(0.3) * 4.0;
                    let total_hp = unit_hp * reg.count as f32;
                    let dealt = lh.min(total_hp);
                    let killed = (dealt / unit_hp).floor() as u32;
                    let frac = dealt / unit_hp - killed as f32;
                    reg.count = reg.count.saturating_sub(killed);
                    if reg.count > 0 && frac > 0.0 {
                        reg.hp = (reg.hp - frac / reg.count as f32 * 0.5).max(0.2);
                    }
                    lh -= dealt;
                }
                let after: f32 = sides[si].iter().map(|(_, r)| r.count as f32).sum();
                let lost = (before - after).max(0.0);
                let disad = if kk_r[1 - si] > 1.5 * kk_r[si] { 5.0 } else { 0.0 };
                morale[si] -= 10.0 * lost / init_strength[si] + disad;
                // Heldenverletzung
                let lost_share = ((init_strength[si] - after) / init_strength[si] * 100.0).max(0.0);
                let hp = 0.02 + 0.01 * lost_share / 10.0;
                for c in &hero_chars[si] {
                    if hero_lost.iter().any(|(h, _)| h == c) {
                        continue;
                    }
                    if self.chance(hp * 0.5) {
                        let r = self.rf();
                        let out = if r < 0.15 {
                            HeroOutcome::Dead
                        } else if r < 0.35 && si == 1 - broke.unwrap_or(si) {
                            HeroOutcome::Captured
                        } else {
                            HeroOutcome::Wounded
                        };
                        hero_lost.push((*c, out));
                    }
                }
            }
            loss_hp = [0.0, 0.0];
            let _ = loss_hp;
            let alive = [sides[0].iter().map(|(_, r)| r.count).sum::<u32>(), sides[1].iter().map(|(_, r)| r.count).sum::<u32>()];
            if alive[0] == 0 || morale[0] < 20.0 {
                broke = Some(0);
                break;
            }
            if alive[1] == 0 || morale[1] < 20.0 {
                broke = Some(1);
                break;
            }
        }
        let winner_idx = match broke {
            Some(0) => 1,
            Some(_) => 0,
            None => {
                // Unentschieden nach 80 Runden: wer mehr hat, gewinnt knapp
                let a: u32 = sides[0].iter().map(|(_, r)| r.count).sum();
                let d: u32 = sides[1].iter().map(|(_, r)| r.count).sum();
                if a > d { 0 } else { 1 }
            }
        };
        // Ergebnisse je Armee zusammenfassen
        let mut armies: Vec<ArmyResult> = Vec::new();
        for si in 0..2 {
            for (ai, b) in armies_of[si].iter().enumerate() {
                let regs: Vec<Regiment> = sides[si].iter().filter(|(i, _)| *i == ai).map(|(_, r)| r.clone()).filter(|r| r.count > 0).collect();
                armies.push(ArmyResult { army: b.army, regiments: regs, retreated: si != winner_idx });
            }
        }
        let duration = rounds as f32 * 10.0 / 60.0 * 2.0;
        BattleResult {
            id: req.id,
            winner: if winner_idx == 0 { BattleSide::Attackers } else { BattleSide::Defenders },
            armies,
            heroes: hero_lost,
            duration_minutes: duration,
            city_taken: req.siege && winner_idx == 0,
        }
    }

    /// Wendet ein Schlachtergebnis auf die Weltkarte an (GDD 4.7).
    fn apply_battle_result_inner(&mut self, req: &BattleRequest, result: BattleResult) -> Result<(), String> {
        let day = self.day;
        let pid = req.province;
        self.stats.battles += 1;
        let winners_attacker = result.winner == BattleSide::Attackers;
        let (win_list, lose_list) = if winners_attacker { (&req.attackers, &req.defenders) } else { (&req.defenders, &req.attackers) };
        let win_realm = win_list.iter().find_map(|b| b.realm);
        let lose_realm = lose_list.iter().find_map(|b| b.realm);
        let mut total_dead = 0u32;
        let (mut strength_w, mut strength_l) = (0u32, 0u32);
        let (mut lost_w, mut lost_l) = (0u32, 0u32);
        for b in win_list {
            strength_w += b.regiments.iter().map(|r| r.count).sum::<u32>();
        }
        for b in lose_list {
            strength_l += b.regiments.iter().map(|r| r.count).sum::<u32>();
        }
        let lock = if result.duration_minutes < 10.0 { 1 } else if result.duration_minutes <= 25.0 { 2 } else { 3 };
        // Verluste und Überlebende übernehmen
        for ar in &result.armies {
            let Some(b) = req.attackers.iter().chain(req.defenders.iter()).find(|b| b.army == ar.army) else { continue };
            let before: u32 = b.regiments.iter().map(|r| r.count).sum();
            let after: u32 = ar.regiments.iter().map(|r| r.count).sum();
            let lost = before.saturating_sub(after);
            total_dead += lost;
            let is_win = win_list.iter().any(|w| w.army == ar.army);
            if is_win {
                lost_w += lost;
            } else {
                lost_l += lost;
            }
            // 30 % „verwundet heimgekehrt“
            if let Some(o) = b.realm
                && matches!(b.kind, ArmyKind::Realm | ArmyKind::Mercenary) && lost > 0 {
                    let home = if self.provinces[pid as usize].owner == Some(o) { pid } else { self.realms[o as usize].capital };
                    self.returning.push((day + 30, home, lost as f32 * 0.3));
                }
            if ar.army == u32::MAX {
                // Garnison: Verluste von den Kriegern abziehen
                let p = &mut self.provinces[pid as usize];
                let lost_f = lost as f32;
                p.pop[Stratum::Warriors.idx()].adults = (p.pop[Stratum::Warriors.idx()].adults - lost_f * 0.7).max(0.0);
                continue;
            }
            let Some(i) = self.armies.iter().position(|a| a.id == ar.army) else { continue };
            self.armies[i].in_battle = None;
            self.armies[i].regiments = ar.regiments.clone();
            self.armies[i].locked_until = day + lock;
            self.armies[i].morale = if is_win { (self.armies[i].morale + 10.0).min(100.0) } else { (self.armies[i].morale - 20.0).max(5.0) };
            // Veteranen: Überlebende des Siegers steigen mit kleiner Chance auf
            if is_win {
                let n = self.armies[i].regiments.len();
                for k in 0..n {
                    if self.armies[i].regiments[k].vet < 3 && self.chance(0.3) {
                        self.armies[i].regiments[k].vet += 1;
                    }
                }
            }
        }
        // Auch Heere, die nicht im Ergebnis vorkamen, freigeben
        for b in req.attackers.iter().chain(req.defenders.iter()) {
            if let Some(a) = self.armies.iter_mut().find(|a| a.id == b.army) {
                a.in_battle = None;
            }
        }
        // Verlierer: aufgelöst oder Rückzug
        for b in lose_list {
            if b.army == u32::MAX {
                continue;
            }
            let Some(i) = self.armies.iter().position(|a| a.id == b.army) else { continue };
            if self.armies[i].size() == 0 {
                self.remove_army_and_commander(b.army, true);
                continue;
            }
            // Rückzug auf sichere Nachbarprovinz
            let dest = self.retreat_target(i, pid);
            match dest {
                Some(d) => {
                    // Nachhutverluste 10 %
                    for r in self.armies[i].regiments.iter_mut() {
                        r.count = (r.count as f32 * 0.9).ceil() as u32;
                    }
                    self.armies[i].path = vec![d];
                    self.armies[i].progress = 0.0;
                    self.armies[i].order = ArmyOrder::Retreat;
                    self.armies[i].locked_until = day;
                }
                None => {
                    self.remove_army_and_commander(b.army, true);
                }
            }
        }
        // Helden
        for (c, out) in &result.heroes {
            match out {
                HeroOutcome::Dead => self.kill_char(*c, DeathCause::Battle),
                HeroOutcome::Wounded => {
                    self.chars[*c as usize].wounded_until = day + 90;
                    self.chars[*c as usize].health = (self.chars[*c as usize].health - 1.0).max(1.0);
                    if self.chance(0.15) {
                        let t = if self.chance(0.5) { crate::catalog::Trait::OneEyed } else { crate::catalog::Trait::Maimed };
                        if !self.chars[*c as usize].traits.contains(&t) {
                            self.chars[*c as usize].traits.push(t);
                        }
                    }
                }
                HeroOutcome::Captured => {
                    if let Some(wr) = win_realm {
                        self.chars[*c as usize].captive_of = Some(wr);
                        self.chars[*c as usize].army = None;
                    }
                }
                HeroOutcome::Unharmed => {}
            }
        }
        // Siegerarmee-Kommandeur: Ruhm
        for b in win_list {
            if let Some(c) = b.commander
                && self.chars[c as usize].alive() {
                    let defensive = !winners_attacker;
                    let ch = &mut self.chars[c as usize];
                    ch.prestige += 10.0 + (strength_l as f32 / 40.0).min(40.0);
                    ch.deeds.battles_won += 1;
                    if defensive {
                        ch.deeds.defensive_wins += 1;
                    }
                    if strength_w * 2 < strength_l && !ch.has(crate::catalog::Trait::WarHero) {
                        ch.traits.push(crate::catalog::Trait::WarHero);
                        ch.prestige += 100.0;
                    }
                }
        }
        // Kriegspunkte, Statistik, Verwüstung, Hohlung
        let diff = lost_l as f32 - lost_w as f32;
        let total_strength = (strength_w + strength_l).max(1) as f32;
        let score = (diff / total_strength * 50.0).clamp(0.0, 20.0);
        if let (Some(wr), Some(lr)) = (win_realm, lose_realm) {
            self.realms[wr as usize].stats.battles_won += 1;
            self.realms[lr as usize].stats.battles_lost += 1;
            self.war_score_for_battle(wr, lr, score);
            self.realms[lr as usize].war_weariness = (self.realms[lr as usize].war_weariness + (lost_l as f32 / strength_l.max(1) as f32) * 10.0).min(100.0);
            // Beute: 10 % des Unterhaltswerts der besiegten Armee
            let loot: f32 = lose_list.iter().map(|b| b.regiments.iter().map(|r| catalog::unit(&r.unit).map(|u| u.upkeep).unwrap_or(1.0) * r.count as f32).sum::<f32>()).sum::<f32>() * 0.1 * 12.0;
            self.realms[wr as usize].gold += loot;
        }
        let p = &mut self.provinces[pid as usize];
        if total_dead > 100 {
            p.devastation = (p.devastation + 0.2).min(1.0);
        }
        if p.veil >= 3.0 {
            p.hollowing = (p.hollowing + (total_dead / 50) as f32).min(100.0);
        }
        // Rebellen siegen in der Provinz?
        let rebels_won = win_list.iter().any(|b| b.kind == ArmyKind::Rebels);
        if rebels_won {
            let p = &mut self.provinces[pid as usize];
            p.rebel_held = true;
            p.captured_day = day;
        }
        if lose_list.iter().any(|b| b.kind == ArmyKind::Rebels) {
            // Rebellen besiegt
            let remaining = self.armies.iter().any(|a| a.kind == ArmyKind::Rebels && a.location == pid);
            if !remaining {
                let owner = self.provinces[pid as usize].owner;
                self.crush_rebels(pid, owner);
            }
        }
        // Belagerung gestürmt
        if req.siege {
            if result.city_taken {
                if let Some(wr) = win_realm {
                    self.occupy_province(pid, wr);
                }
            } else {
                self.provinces[pid as usize].siege = None;
            }
        }
        // Bossfall
        for b in lose_list {
            if b.kind == ArmyKind::Boss {
                let by = win_realm;
                if let Some(bn) = self.boss_of_army(b.army) {
                    self.boss_defeated(&bn, by);
                }
            }
        }
        // Chronik
        let notable = strength_w + strength_l >= 120 || lose_list.iter().any(|b| matches!(b.kind, ArmyKind::Boss | ArmyKind::Legion)) || req.siege;
        if notable {
            let wn = win_list.iter().find_map(|b| b.commander_name.clone()).or_else(|| win_realm.map(|r| self.rname(r))).unwrap_or_else(|| win_list[0].name.clone());
            let ln = lose_list.iter().find_map(|b| b.commander_name.clone()).or_else(|| lose_realm.map(|r| self.rname(r))).unwrap_or_else(|| lose_list[0].name.clone());
            let boss = lose_list.iter().any(|b| b.kind == ArmyKind::Boss);
            let tpl = if boss { "battle_boss" } else if !winners_attacker { "battle_win_defense" } else { "battle_win" };
            let mag = (20.0 + (total_dead as f32 / 12.0).min(40.0) + if boss { 25.0 } else { 0.0 }) as u8;
            let mut e = Entry::new(ChronicleKind::Battle, tpl, mag).prov(pid, &req.province_name).var("a", wn.clone()).var("b", ln.clone()).var("n", total_dead.to_string()).num("strength_a", strength_w as f32).num("strength_b", strength_l as f32).num("losses_a", lost_w as f32).num("losses_b", lost_l as f32).tag("battle");
            if let Some(w) = win_realm {
                e = e.realm(w);
            }
            for b in win_list {
                if let Some(c) = b.commander {
                    e = e.actor(format!("char_{c}"), self.full_name(c), "Sieger");
                }
            }
            self.chron(e);
        }
        self.emit(StrategyEvent::BattleResolved { province: pid, winner_realm: win_realm });
        Ok(())
    }

    fn boss_of_army(&self, army: ArmyId) -> Option<String> {
        self.bosses.iter().find(|b| b.army == Some(army)).map(|b| b.id.clone())
    }

    fn remove_army_and_commander(&mut self, id: ArmyId, in_battle: bool) {
        let _ = in_battle;
        if let Some(i) = self.armies.iter().position(|a| a.id == id) {
            let a = self.armies.remove(i);
            if let Some(c) = a.commander {
                self.chars[c as usize].army = None;
            }
            for c in self.chars.iter_mut().filter(|c| c.army == Some(id)) {
                c.army = None;
            }
        }
    }

    fn retreat_target(&self, army_idx: usize, from: ProvId) -> Option<ProvId> {
        let a = &self.armies[army_idx];
        let owner = a.owner;
        let mut best: Option<(ProvId, f32)> = None;
        for l in &self.provinces[from as usize].neighbors {
            if l.kind == LinkKind::Sea {
                continue;
            }
            let q = &self.provinces[l.to as usize];
            let ok = match (owner, q.holder()) {
                (Some(o), Some(h)) => h == o || self.same_lord(o, h) || !self.at_war(o, h),
                (Some(_), None) => true,
                (None, _) => a.came_from == Some(l.to) || q.holder().is_none(),
            };
            if !ok {
                continue;
            }
            // Keine feindlichen Heere dort
            if self.armies.iter().any(|x| x.location == l.to && self.armies_hostile(a, x)) {
                continue;
            }
            let mut s = 1.0;
            if a.came_from == Some(l.to) {
                s += 2.0;
            }
            if q.holder() == owner {
                s += 1.5;
            }
            if best.is_none_or(|b| s > b.1) {
                best = Some((l.to, s));
            }
        }
        best.map(|b| b.0)
    }

    // ------------------------------------------------------------------------------------
    // Monat: Versorgung, Sold, Rückkehrer
    // ------------------------------------------------------------------------------------
    pub(crate) fn armies_month(&mut self) {
        let day = self.day;
        let ids: Vec<ArmyId> = self.armies.iter().map(|a| a.id).collect();
        for id in ids {
            let Some(i) = self.armies.iter().position(|a| a.id == id) else { continue };
            let (kind, owner, loc) = (self.armies[i].kind, self.armies[i].owner, self.armies[i].location);
            // Sold: Gold im Plus?
            if matches!(kind, ArmyKind::Realm | ArmyKind::Mercenary) {
                if let Some(o) = owner {
                    if self.realms[o as usize].gold < 0.0 && !matches!(self.realms[o as usize].kind, RealmKind::Hollowed | RealmKind::Legion) {
                        self.armies[i].unpaid_months = self.armies[i].unpaid_months.saturating_add(1);
                    } else {
                        self.armies[i].unpaid_months = 0;
                    }
                }
                if self.armies[i].unpaid_months >= 3 {
                    // Desertion 5 %/Monat
                    for r in self.armies[i].regiments.iter_mut() {
                        r.count = (r.count as f32 * 0.95) as u32;
                    }
                    self.armies[i].morale = (self.armies[i].morale - 8.0).max(0.0);
                }
            }
            // Versorgung
            let p = &self.provinces[loc as usize];
            let own_land = owner.is_some() && (p.holder() == owner || owner.zip(p.holder()).is_some_and(|(o, h)| self.same_lord(o, h) || self.has_treaty(TreatyKind::Alliance, o, h)));
            let a = &mut self.armies[i];
            if matches!(kind, ArmyKind::Realm | ArmyKind::Mercenary) {
                if own_land {
                    a.supply = (a.supply + 30.0).min(100.0);
                } else {
                    let drain = match p.biome {
                        Biome::Desert | Biome::Mountain | Biome::Tundra | Biome::Hollowland => 25.0,
                        _ => 15.0,
                    };
                    a.supply = (a.supply - drain).max(0.0);
                }
                if a.supply < 20.0 {
                    for r in a.regiments.iter_mut() {
                        r.count = (r.count as f32 * 0.985) as u32;
                    }
                    a.morale = (a.morale - 3.0).max(0.0);
                }
                // Heilung
                let heal = if own_land { 0.3 } else { 0.1 };
                for r in a.regiments.iter_mut() {
                    r.hp = (r.hp + heal).min(1.0);
                }
                a.morale = (a.morale + 3.0).min(100.0);
            }
            a.regiments.retain(|r| r.count > 0);
        }
        // leere Armeen aufräumen
        let empty: Vec<ArmyId> = self.armies.iter().filter(|a| a.size() == 0 && a.in_battle.is_none()).map(|a| a.id).collect();
        for id in empty {
            self.remove_army_and_commander(id, false);
        }
        // Rückkehrer
        let mut i = 0;
        while i < self.returning.len() {
            if self.returning[i].0 <= day {
                let (_, pid, n) = self.returning.remove(i);
                self.restore_soldiers(pid, n);
            } else {
                i += 1;
            }
        }
        // Zu viele Rebellenheere ohne Provinzen: auflösen, wenn Provinz beruhigt
        let rebels: Vec<ArmyId> = self.armies.iter().filter(|a| a.kind == ArmyKind::Rebels && self.provinces[a.location as usize].unrest < 15.0 && !self.provinces[a.location as usize].rebel_held).map(|a| a.id).collect();
        for id in rebels {
            self.remove_army_and_commander(id, false);
        }
    }
}

/// Wert einer Provinz für Kriegspunkte und Friedensbedingungen.
pub fn province_value(p: &Province, is_capital: bool) -> f32 {
    let base = 6.0 + p.settlement.rank() as f32 * 6.0 + (p.population() / 1500.0).min(8.0);
    if is_capital {
        base * 3.0
    } else {
        base
    }
}

/// Hilfstyp für Nahrungsgüter-Zugriff im Belagerungscode.
#[derive(Clone, Copy)]
enum Good2 {
    Grain,
    Bread,
    Meat,
    Fish,
    Preserved,
}

impl Good2 {
    fn idx(self) -> usize {
        match self {
            Good2::Grain => crate::goods::Good::Grain.idx(),
            Good2::Bread => crate::goods::Good::Bread.idx(),
            Good2::Meat => crate::goods::Good::Meat.idx(),
            Good2::Fish => crate::goods::Good::Fish.idx(),
            Good2::Preserved => crate::goods::Good::PreservedFood.idx(),
        }
    }
}
