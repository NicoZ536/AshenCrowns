//! Neutrale Akteure und Weltbedrohungen: Monster, Banditen, Kultisten, Söldner, die Hohlen als
//! Ausbreitungssimulation, die Aschenlegion und die Bosse des Kanons (GDD Kapitel 14).

use crate::chronicle::{ChronicleKind, Entry};
use crate::diplomacy::is_antagonist;
use crate::goods::{Good, ServiceKind};
use crate::model::*;
use crate::types::*;
use crate::world::StrategyWorld;

/// Bossdefinition: (ID, Name, Provinzschlüssel, Startzustand, Einheiten-ID, Stärke, Besitzerart).
type BossDef = (&'static str, &'static str, &'static str, BossState, &'static str, u32, Option<RealmKind>);

impl StrategyWorld {
    // ------------------------------------------------------------------------------------
    // Hilfen
    // ------------------------------------------------------------------------------------
    pub fn realm_of_kind(&self, kind: RealmKind) -> Option<RealmId> {
        self.realms.iter().find(|r| r.kind == kind).map(|r| r.id)
    }

    /// Ist der Bossverband für ein Heer des Reiches feindlich?
    pub fn boss_hostile_to(&self, boss_army: &Army, other: &Army) -> bool {
        let Some(b) = self.bosses.iter().find(|b| b.army == Some(boss_army.id)) else { return true };
        if b.state != BossState::Active {
            return false;
        }
        let Some(o) = other.owner else { return true };
        // Dem Herrn der Provinz und seinen Verbündeten gegenüber friedlich (Hüter des eigenen Landes)
        let province_owner = self.provinces[b.province as usize].owner;
        if let Some(po) = province_owner
            && !is_antagonist(self.realms[po as usize].kind) && (po == o || self.same_lord(po, o) || self.has_treaty(TreatyKind::Alliance, po, o)) {
                return false;
            }
        // Isarre Mol und der Drache sind keine Feldgegner außerhalb ihrer Provinz
        true
    }

    pub fn boss(&self, id: &str) -> Option<&Boss> {
        self.bosses.iter().find(|b| b.id == id)
    }

    pub(crate) fn init_bosses(&mut self) {
        self.bosses.clear();
        let hollowed = self.realm_of_kind(RealmKind::Hollowed);
        let legion = self.realm_of_kind(RealmKind::Legion);
        let defs: [BossDef; 6] = [
            ("boss_pale_reeve", "Der Fahle Vogt", "vael_dorn", BossState::Active, "boss_pale_reeve", 6, Some(RealmKind::Hollowed)),
            ("boss_ashen_dragon", "Der Aschdrache Vyrmathax", "der_kessel", BossState::Active, "boss_ashen_dragon", 25, None),
            ("boss_root_mother", "Die Wurzelmutter", "herzhain", BossState::Dormant, "boss_root_mother", 18, None),
            ("boss_hollow_king", "Der Hohle König", "solmere", BossState::Active, "boss_hollow_king", 20, Some(RealmKind::Legion)),
            ("boss_stone_oath", "Gharun, der Steinerne Eid", "waechtertor", BossState::Active, "boss_stone_oath", 14, None),
            ("boss_reef_mistress", "Isarre Mol, Herrin des Schwarzen Riffs", "schwarzes_riff", BossState::Dormant, "boss_reef_mistress", 12, None),
        ];
        for (id, name, key, state, unit, count, owner_kind) in defs {
            let Some(prov) = self.prov_by_key(key) else { continue };
            let owner = owner_kind.and_then(|k| if k == RealmKind::Hollowed { hollowed } else { legion });
            let mut regs = vec![Regiment { unit: unit.to_string(), count, vet: 3, hp: 1.0 }];
            if id == "boss_pale_reeve" {
                regs.push(Regiment { unit: "hollow_thrall".into(), count: 40, vet: 0, hp: 1.0 });
                regs.push(Regiment { unit: "hollow_archer".into(), count: 14, vet: 0, hp: 1.0 });
            }
            if id == "boss_hollow_king" {
                regs.push(Regiment { unit: "ashen_legionnaire".into(), count: 120, vet: 2, hp: 1.0 });
                regs.push(Regiment { unit: "cinder_knight".into(), count: 30, vet: 2, hp: 1.0 });
            }
            let aid = self.new_army(ArmyKind::Boss, None, prov, regs, name);
            if let Some(a) = self.armies.iter_mut().find(|a| a.id == aid) {
                a.boss = Some(id.to_string());
                a.morale = 100.0;
            }
            let _ = owner;
            let next_action = 3 * DAYS_PER_YEAR + (self.rng_u32() % (4 * DAYS_PER_YEAR));
            self.bosses.push(Boss { id: id.to_string(), name: name.to_string(), province: prov, state, army: Some(aid), last_action: 0, next_action, tribute_until: 0, defeated_by: None, awakened: None });
        }
        // Schlafende Legion: Garnisonsheere in den Legionsprovinzen
        if let Some(l) = legion {
            let provs = self.provinces_of(l).to_vec();
            for p in provs {
                if self.provinces[p as usize].key == "solmere" {
                    continue;
                }
                let regs = vec![Regiment { unit: "ashen_legionnaire".into(), count: 80, vet: 1, hp: 1.0 }, Regiment { unit: "cinder_knight".into(), count: 12, vet: 1, hp: 1.0 }];
                let nm = format!("Legion von {}", self.pname(p));
                let aid = self.new_army(ArmyKind::Realm, Some(l), p, regs, &nm);
                if let Some(a) = self.armies.iter_mut().find(|a| a.id == aid) {
                    a.order = ArmyOrder::Garrison;
                    a.morale = 100.0;
                }
            }
        }
        // Hohlenheere an den Schreinen
        if let Some(h) = hollowed {
            let provs = self.provinces_of(h).to_vec();
            for p in provs {
                if self.provinces[p as usize].key == "vael_dorn" {
                    continue;
                }
                let regs = vec![Regiment { unit: "hollow_thrall".into(), count: 60, vet: 0, hp: 1.0 }, Regiment { unit: "hollow_archer".into(), count: 20, vet: 0, hp: 1.0 }, Regiment { unit: "grave_brute".into(), count: 3, vet: 0, hp: 1.0 }];
                let nm = format!("Hohle von {}", self.pname(p));
                let aid = self.new_army(ArmyKind::Realm, Some(h), p, regs, &nm);
                if let Some(a) = self.armies.iter_mut().find(|a| a.id == aid) {
                    a.order = ArmyOrder::Garrison;
                    a.morale = 100.0;
                }
            }
        }
    }

    /// Beendet einen Boss: Chronik, Belohnung, Folgen.
    pub fn boss_defeated(&mut self, id: &str, by: Option<RealmId>) {
        let Some(bi) = self.bosses.iter().position(|b| b.id == id) else { return };
        if self.bosses[bi].state == BossState::Defeated {
            return;
        }
        self.bosses[bi].state = BossState::Defeated;
        self.bosses[bi].defeated_by = by;
        let (name, prov) = (self.bosses[bi].name.clone(), self.bosses[bi].province);
        let mut e = Entry::new(ChronicleKind::BossDefeated, "boss_defeated", 80).prov(prov, &self.pname(prov)).var("b", name.clone()).tag("boss").tag(if id == "boss_hollow_king" { "hollow_king" } else { "boss" });
        if let Some(r) = by {
            e = e.realm(r).var("a", self.rname(r));
            self.realms[r as usize].prestige += 400.0;
            let ruler = self.realms[r as usize].ruler;
            self.chars[ruler as usize].prestige += 300.0;
            self.realms[r as usize].gold += 500.0;
            self.realms[r as usize].ashglass += 20.0;
        } else {
            e = e.var("a", "Unbekannte Helden");
        }
        self.chron(e);
        match id {
            "boss_pale_reeve" => {
                self.provinces[prov as usize].shrine = 0;
                self.provinces[prov as usize].hollow_force = 0.0;
                self.endgame.veil_pressure = (self.endgame.veil_pressure - 5.0).max(0.0);
            }
            "boss_hollow_king" => {
                self.endgame.veil_pressure = (self.endgame.veil_pressure - 15.0).max(0.0);
                self.endgame.hollow_king_awake = None;
            }
            "boss_ashen_dragon" => {
                self.endgame.dragon_wrath = false;
            }
            "boss_root_mother" => {
                self.endgame.withering = false;
            }
            _ => {}
        }
        self.bosses[bi].army = None;
    }

    pub(crate) fn bosses_year(&mut self) {
        let day = self.day;
        let year = self.year();
        let ids: Vec<String> = self.bosses.iter().map(|b| b.id.clone()).collect();
        for id in ids {
            let Some(bi) = self.bosses.iter().position(|b| b.id == id) else { continue };
            if self.bosses[bi].state == BossState::Defeated {
                continue;
            }
            // Armee verloren?
            if let Some(a) = self.bosses[bi].army
                && self.army(a).is_none() {
                    self.boss_defeated(&id, None);
                    continue;
                }
            match id.as_str() {
                "boss_ashen_dragon" => {
                    let interval = if self.endgame.dragon_wrath { 1 } else { 3 + (self.rng_u32() % 5) as i32 };
                    if day >= self.bosses[bi].next_action && self.bosses[bi].tribute_until <= day {
                        self.dragon_flight(bi);
                        self.bosses[bi].next_action = day + interval as u32 * DAYS_PER_YEAR;
                    } else if self.bosses[bi].tribute_until > day {
                        // Tribut hält den Drachen ruhig
                        self.bosses[bi].next_action = self.bosses[bi].next_action.max(self.bosses[bi].tribute_until);
                    }
                }
                "boss_reef_mistress" => {
                    if self.bosses[bi].state == BossState::Dormant && self.chance(0.12) {
                        self.bosses[bi].state = BossState::Active;
                    }
                    if self.bosses[bi].state == BossState::Active && day >= self.bosses[bi].next_action {
                        self.reef_raid();
                        self.bosses[bi].next_action = day + (2 + (self.rng_u32() % 4)) * DAYS_PER_YEAR;
                    }
                }
                "boss_root_mother"
                    // Wut sinkt langsam, steigt durch Abholzung (siehe Waldkette)
                    if self.bosses[bi].state == BossState::Active && !self.endgame.withering && self.chance(0.02) => {
                        self.bosses[bi].state = BossState::Dormant;
                    }
                _ => {}
            }
        }
        let _ = year;
    }

    fn dragon_flight(&mut self, bi: usize) {
        let home = self.bosses[bi].province;
        let mut targets: Vec<ProvId> = self.provinces[home as usize].neighbors.iter().map(|l| l.to).filter(|p| self.provinces[*p as usize].owner.is_some()).collect();
        // Im Zorn: Aschglasminen
        if self.endgame.dragon_wrath {
            let mut mines: Vec<ProvId> = self.provinces.iter().filter(|p| p.owner.is_some_and(|o| !is_antagonist(self.realms[o as usize].kind)) && p.building_count(crate::goods::building_index("ashglass_mine").unwrap_or(0)) > 0).map(|p| p.id).collect();
            mines.truncate(2);
            targets.extend(mines);
        }
        targets.sort_unstable();
        targets.dedup();
        let n = targets.len().min(3);
        if n == 0 {
            return;
        }
        // Tribut: Reiche zahlen, wenn sie es sich leisten können
        let mut hit: Vec<ProvId> = Vec::new();
        for &t in targets.iter().take(n) {
            let Some(o) = self.provinces[t as usize].owner else { continue };
            if !self.is_player(o) && self.realms[o as usize].gold > 600.0 && self.realms[o as usize].ashglass >= 5.0 && self.chance(0.7) {
                self.realms[o as usize].gold -= 400.0;
                self.realms[o as usize].ashglass -= 5.0;
                let e = Entry::new(ChronicleKind::Diplomacy, "dragon_tribute", 40).realm(o).var("a", self.rname(o)).var("n", "5");
                self.chron(e);
                self.bosses[bi].tribute_until = self.day + 2 * DAYS_PER_YEAR;
                continue;
            }
            hit.push(t);
        }
        let mut first_cause = None;
        for t in hit {
            let p = &mut self.provinces[t as usize];
            p.harvest = (p.harvest * 0.2).max(0.1);
            p.devastation = (p.devastation + 0.5).min(1.0);
            let owner = p.owner;
            let name = p.name.clone();
            let e = Entry::new(ChronicleKind::Disaster, "dragon_flight", 60).prov(t, &name).tag("dragon");
            let e = if let Some(o) = owner { e.realm(o) } else { e };
            let id = self.chron(e);
            self.provinces[t as usize].chain.drought = Some(id);
            if first_cause.is_none() {
                first_cause = Some(id);
            }
        }
        self.bosses[bi].last_action = self.day;
    }

    fn reef_raid(&mut self) {
        let day = self.day;
        for r in self.routes.iter_mut().filter(|r| r.sea) {
            r.blocked_until = day + 8 * DAYS_PER_MONTH;
        }
        for mi in 0..self.markets.len() {
            for g in [Good::Spices, Good::Silk] {
                self.markets[mi].event_factor[g.idx()] = (1.6, day + 12 * DAYS_PER_MONTH);
            }
        }
        if let Some(p) = self.prov_by_key("schwarzes_riff") {
            let name = self.pname(p);
            let e = Entry::new(ChronicleKind::Disaster, "event_generic", 40).prov(p, &name).var("a", "die Seewege").var("c", "Geisterschiffe aus dem Nebel des Schwarzen Riffs sperrten die Seewege, die Preise für Gewürze und Seide stiegen");
            self.chron(e);
        }
    }

    /// Waldkette: Abholzung am Dornwaldrand erzürnt den Grünen Kreis, die Wurzelmutter erwacht.
    fn forest_month(&mut self) {
        let mut polluter: Option<RealmId> = None;
        let woodcutter = crate::goods::building_index("woodcutter").unwrap_or(0);
        for pi in 0..self.provinces.len() {
            let (region, biome) = (self.provinces[pi].region, self.provinces[pi].biome);
            if biome != Biome::Forest {
                continue;
            }
            let cutters = self.provinces[pi].building_count(woodcutter) as f32;
            let pop = self.provinces[pi].population();
            let wood_unmet = self.provinces[pi].unmet[Good::Wood.idx()] + self.provinces[pi].unmet[Good::Planks.idx()];
            let forest = self.provinces[pi].service_count(ServiceKind::Granary) as f32 * 0.0;
            let loss = (cutters * 0.0007 + wood_unmet / 4000.0 + forest) * (1.0 + pop / 4000.0);
            let regrow = 0.0015;
            let west_edge = region == 0 && self.provinces[pi].neighbors.iter().any(|l| self.provinces[l.to as usize].region == 3);
            let p = &mut self.provinces[pi];
            p.forest_cover = (p.forest_cover - loss + regrow).clamp(0.1, 1.0);
            // Rand des Dornwalds (Region 3 und Westrand Valemarks)
            let edge = region == 3 || west_edge;
            if edge && self.provinces[pi].forest_cover < 0.7
                && let Some(o) = self.provinces[pi].owner
                    && self.realms[o as usize].faction != "thorn_covenant" && !is_antagonist(self.realms[o as usize].kind) {
                        polluter = Some(o);
                    }
        }
        if let Some(o) = polluter {
            self.root_anger += 8.0;
            if self.root_anger >= 100.0 {
                self.root_anger = 40.0;
                if let Some(bi) = self.bosses.iter().position(|b| b.id == "boss_root_mother")
                    && self.bosses[bi].state == BossState::Dormant {
                        self.bosses[bi].state = BossState::Active;
                        self.bosses[bi].awakened = Some(self.day);
                        let pn = self.pname(self.bosses[bi].province);
                        let e = Entry::new(ChronicleKind::Disaster, "boss_awakes", 60).prov(self.bosses[bi].province, &pn).var("b", "Die Wurzelmutter").tag("boss");
                        self.chron(e);
                    }
                // Der Dornenbund erklärt den Krieg
                if let Some(thorn) = self.realms.iter().find(|r| r.alive && r.faction == "thorn_covenant" && r.kind == RealmKind::Covenant).map(|r| r.id)
                    && !self.at_war(thorn, o) && !self.has_treaty(TreatyKind::Truce, thorn, o) && self.realms[thorn as usize].regency.is_none() {
                        let goal = self.provinces.iter().find(|p| p.owner == Some(o) && p.forest_cover < 0.7).map(|p| WarGoal::TakeProvince(p.id)).unwrap_or(WarGoal::Tribute);
                        let _ = self.declare_war(thorn, o, CasusBelli::Border, goal);
                    }
            }
        } else {
            self.root_anger = (self.root_anger - 1.0).max(0.0);
        }
    }

    // ------------------------------------------------------------------------------------
    // Neutrale Gruppen
    // ------------------------------------------------------------------------------------
    pub fn spawn_neutral(&mut self, kind: NeutralKind, pid: ProvId, strength: f32) {
        let id = self.next_neutral;
        self.next_neutral += 1;
        let aggression = match kind {
            NeutralKind::GlassSeekers | NeutralKind::VeylClans | NeutralKind::Ruins | NeutralKind::Refugees => 0.1,
            NeutralKind::Bandits | NeutralKind::Cultists => 0.6,
            _ => 0.8,
        };
        self.neutrals.push(NeutralGroup { id, kind, province: pid, strength, aggression, founded: self.day, active: true, last_raid: 0 });
    }

    pub(crate) fn init_neutrals(&mut self) {
        self.neutrals.clear();
        for pi in 0..self.provinces.len() {
            let (biome, region, key) = (self.provinces[pi].biome, self.provinces[pi].region, self.provinces[pi].key.clone());
            match (biome, region) {
                (Biome::Ruins, _) => self.spawn_neutral(NeutralKind::GlassSeekers, pi as ProvId, 12.0),
                (Biome::Swamp, _) => self.spawn_neutral(NeutralKind::MoorLights, pi as ProvId, 8.0),
                (Biome::Volcanic, 7) => self.spawn_neutral(NeutralKind::Salamanders, pi as ProvId, 20.0),
                _ => {}
            }
            if matches!(key.as_str(), "kaltherd" | "grauwall" | "merrat") {
                self.spawn_neutral(NeutralKind::AshWolves, pi as ProvId, 14.0);
            }
            if key == "ilvaen" || key == "mottenwacht" {
                self.spawn_neutral(NeutralKind::VeylClans, pi as ProvId, 15.0);
            }
        }
        // Söldnerkompanien an den Saelith-Häfen
        self.mercs.clear();
        let ports = ["selvaris", "varo", "strathhaven"];
        let names = ["Die Silberlanzen", "Schwarzwimpel-Kompanie", "Die Gezeitenklingen"];
        for (i, (port, name)) in ports.iter().zip(names).enumerate() {
            if let Some(p) = self.prov_by_key(port) {
                self.mercs.push(MercCompany { id: i as u32, name: name.to_string(), size: 120 + i as u32 * 40, home: p, hired_by: None, army: None, monthly_pay: 90.0 + i as f32 * 25.0, unpaid_months: 0 });
            }
        }
    }

    pub(crate) fn neutral_month(&mut self) {
        let day = self.day;
        self.forest_month();
        self.mercs_month();
        // Neue Gruppen
        for pi in 0..self.provinces.len() {
            let p = &self.provinces[pi];
            let (owner, unrest, veil, hollowing, biome, region) = (p.owner, p.unrest, p.veil, p.hollowing, p.biome, p.region);
            if self.neutrals.iter().any(|n| n.active && n.province == pi as ProvId && matches!(n.kind, NeutralKind::Bandits | NeutralKind::Cultists)) {
                continue;
            }
            if owner.is_some_and(|o| is_antagonist(self.realms[o as usize].kind)) {
                continue;
            }
            if unrest > 40.0 && self.chance((unrest - 35.0) / 1500.0) {
                self.spawn_neutral(NeutralKind::Bandits, pi as ProvId, 14.0 + unrest / 4.0);
                let name = self.provinces[pi].name.clone();
                let e = Entry::new(ChronicleKind::Disaster, "bandits", 18).prov(pi as ProvId, &name);
                self.chron(e);
            }
            if hollowing > 15.0 && self.chance(0.01 + hollowing / 4000.0) {
                self.spawn_neutral(NeutralKind::Cultists, pi as ProvId, 12.0 + hollowing / 6.0);
            }
            if matches!(biome, Biome::Hills | Biome::Forest | Biome::Desert) && veil >= 0.8 && region != 3 && self.chance(0.004) && !self.neutrals.iter().any(|n| n.active && n.province == pi as ProvId && n.kind == NeutralKind::AshWolves) {
                self.spawn_neutral(NeutralKind::AshWolves, pi as ProvId, 10.0 + veil * 3.0);
            }
        }
        // Gruppen handeln
        let ids: Vec<u32> = self.neutrals.iter().filter(|n| n.active).map(|n| n.id).collect();
        for id in ids {
            let Some(i) = self.neutrals.iter().position(|n| n.id == id) else { continue };
            let n = self.neutrals[i].clone();
            // Wachstum und Schrumpfung
            let growth = match n.kind {
                NeutralKind::Bandits => {
                    if self.provinces[n.province as usize].unrest > 30.0 {
                        1.03
                    } else {
                        0.93
                    }
                }
                NeutralKind::Cultists => {
                    if self.provinces[n.province as usize].hollowing > 15.0 {
                        1.02
                    } else {
                        0.9
                    }
                }
                _ => 1.01,
            };
            self.neutrals[i].strength = (n.strength * growth).min(70.0);
            if self.neutrals[i].strength < 3.0 {
                self.neutrals[i].active = false;
                continue;
            }
            if n.aggression < 0.3 {
                // Aschglas-Sucher: handeln, 10 % Hohlen-Spione (Hohlung steigt leicht)
                if n.kind == NeutralKind::GlassSeekers && self.chance(0.04) {
                    let p = &mut self.provinces[n.province as usize];
                    p.hollowing = (p.hollowing + 0.5).min(100.0);
                    p.stock[Good::AshglassRaw.idx()] += 3.0;
                }
                continue;
            }
            let Some(owner) = self.provinces[n.province as usize].owner else { continue };
            if is_antagonist(self.realms[owner as usize].kind) {
                continue;
            }
            // Raubzug
            let raid_p = 0.05 * n.aggression * (n.strength / 30.0).min(1.5);
            if !self.chance(raid_p) {
                continue;
            }
            let p = &self.provinces[n.province as usize];
            let def = p.garrison as f32 * 2.0 + self.armies.iter().filter(|a| a.location == n.province && a.owner == Some(owner)).map(|a| a.size() as f32 * 1.2).sum::<f32>() + p.fortification as f32 * 6.0 + p.population() * 0.01;
            let atk = n.strength * 3.0;
            let name = p.name.clone();
            if atk > def {
                let loss = (n.strength * 0.35).min(p.population() * 0.01);
                self.add_population(n.province as usize, -loss, false);
                let pr = &mut self.provinces[n.province as usize];
                pr.unrest = (pr.unrest + 4.0).min(100.0);
                for g in [Good::Grain, Good::Bread, Good::Meat, Good::Livestock] {
                    pr.stock[g.idx()] *= 0.9;
                }
                if matches!(n.kind, NeutralKind::Bandits | NeutralKind::MoorLights) {
                    let region = pr.region as usize;
                    for r in self.routes.iter_mut() {
                        if self.regions[region].market == Some(r.a) || self.regions[region].market == Some(r.b) {
                            r.blocked_until = day + 30;
                        }
                    }
                }
                self.neutrals[i].last_raid = day;
                let e = Entry::new(ChronicleKind::Disaster, "raid", 20).prov(n.province, &name).realm(owner).var("c", n.kind.name().to_string()).var("n", format!("{}", loss.round() as i64));
                self.chron(e);
                // Flüchtlinge
                self.add_refugees_from(n.province, loss * 0.5);
            } else {
                self.neutrals[i].strength *= 0.55;
            }
        }
        self.neutrals.retain(|n| n.active || day.saturating_sub(n.founded) < DAYS_PER_YEAR);
        // Seelenasche-Quelle der Hohlen und Schreine
        self.hollow_spread_month();
    }

    // ------------------------------------------------------------------------------------
    // Söldner
    // ------------------------------------------------------------------------------------
    pub fn hire_mercenaries(&mut self, r: RealmId, company: u32) -> Result<ArmyId, String> {
        let ci = self.mercs.iter().position(|m| m.id == company).ok_or("Kompanie nicht gefunden")?;
        if self.mercs[ci].hired_by.is_some() {
            return Err("Bereits angeworben".into());
        }
        let c = self.mercs[ci].clone();
        let cost = c.monthly_pay * 3.0;
        if self.realms[r as usize].gold < cost {
            return Err("Nicht genug Gold (drei Monatssold im Voraus)".into());
        }
        // Meinung zum Saelith-Pakt (Söldner nur bei guter Meinung)
        if let Some(sp) = self.realm_by_key("saelith_compact")
            && sp != r && self.opinion(sp, r) < -20.0 {
                return Err("Der Saelith-Pakt verweigert Söldner".into());
            }
        self.realms[r as usize].gold -= cost;
        // Erscheint in der nächsten eigenen Provinz bzw. Hauptstadt
        let loc = self.realms[r as usize].capital;
        let regs = vec![Regiment { unit: "saelith_marine".into(), count: c.size * 6 / 10, vet: 1, hp: 1.0 }, Regiment { unit: "saelith_arbalest".into(), count: c.size - c.size * 6 / 10, vet: 1, hp: 1.0 }];
        let aid = self.new_army(ArmyKind::Mercenary, Some(r), loc, regs, &c.name);
        self.mercs[ci].hired_by = Some(r);
        self.mercs[ci].army = Some(aid);
        self.mercs[ci].unpaid_months = 0;
        Ok(aid)
    }

    fn mercs_month(&mut self) {
        for i in 0..self.mercs.len() {
            let Some(r) = self.mercs[i].hired_by else {
                // Freie Kompanien: gelegentlich von der KI im Krieg angeworben
                continue;
            };
            let aid = self.mercs[i].army;
            let alive = aid.is_some_and(|a| self.army(a).is_some());
            if !alive || !self.realms[r as usize].alive {
                self.mercs[i].hired_by = None;
                self.mercs[i].army = None;
                continue;
            }
            // Sold
            let pay = self.mercs[i].monthly_pay;
            if self.realms[r as usize].gold >= pay {
                self.realms[r as usize].gold -= pay;
                self.mercs[i].unpaid_months = 0;
            } else {
                self.mercs[i].unpaid_months += 1;
            }
            // Ende des Krieges: Vertrag läuft aus
            if !self.in_any_war(r) && self.chance(0.4) {
                if let Some(a) = aid {
                    self.disband_army(a);
                }
                self.mercs[i].hired_by = None;
                self.mercs[i].army = None;
                continue;
            }
            // Soldverzug ab 3 Monaten: Seitenwechsel oder Banditen
            if self.mercs[i].unpaid_months >= 3 {
                let a = aid.unwrap();
                let enemy = self.wars.iter().filter(|w| w.active).find_map(|w| if w.attackers.contains(&r) { Some(w.leader_d) } else if w.defenders.contains(&r) { Some(w.leader_a) } else { None });
                if let Some(e) = enemy.filter(|e| self.realms[*e as usize].gold > 500.0) {
                    if let Some(arm) = self.armies.iter_mut().find(|x| x.id == a) {
                        arm.owner = Some(e);
                    }
                    self.mercs[i].hired_by = Some(e);
                    self.mercs[i].unpaid_months = 0;
                    let e2 = Entry::new(ChronicleKind::War, "event_generic", 30).realm(r).var("a", self.rname(r)).var("p", self.rname(r)).var("c", format!("Die Söldner {} wechselten wegen ausbleibendem Sold die Seite", self.mercs[i].name));
                    self.chron(e2);
                } else if let Some(arm) = self.armies.iter_mut().find(|x| x.id == a) {
                    arm.kind = ArmyKind::Bandits;
                    arm.owner = None;
                    self.mercs[i].hired_by = None;
                    self.mercs[i].army = None;
                }
            }
        }
        // KI heuert an
        let ids: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind) && !self.is_player(r.id)).map(|r| r.id).collect();
        for r in ids {
            if self.in_any_war(r) && self.realms[r as usize].gold > 1500.0 && self.chance(0.1) && !self.mercs.iter().any(|m| m.hired_by == Some(r))
                && let Some(c) = self.mercs.iter().find(|m| m.hired_by.is_none()).map(|m| m.id) {
                    let _ = self.hire_mercenaries(r, c);
                }
        }
    }

    // ------------------------------------------------------------------------------------
    // Die Hohlen als Weltsystem
    // ------------------------------------------------------------------------------------
    /// Schreine, Hohlenstärke und Raubzüge (GDD 14.2).
    fn hollow_spread_month(&mut self) {
        let hmult = self.options.difficulty.mods().hollow_strength;
        let double = self.endgame.hollow_king_awake.is_some();
        for pi in 0..self.provinces.len() {
            let (veil, shrine) = (self.provinces[pi].veil, self.provinces[pi].shrine);
            let owner = self.provinces[pi].owner;
            // Schreine entstehen bei Schleierstärke ≥ 3 (5 % pro Jahr)
            if shrine == 0 && veil >= 3.0 && self.provinces[pi].biome != Biome::Hollowland {
                let p = 0.05 / 12.0 * if double { 2.0 } else { 1.0 };
                if self.chance(p) {
                    self.provinces[pi].shrine = 1;
                    let name = self.provinces[pi].name.clone();
                    let e = Entry::new(ChronicleKind::Hollowing, "shrine_appears", 32).prov(pi as ProvId, &name).tag("shrine");
                    let e = if let Some(o) = owner { e.realm(o) } else { e };
                    self.chron(e);
                }
            }
            if self.provinces[pi].shrine > 0 {
                // Jeder Schrein erzeugt 5 + Schleierstärke × 3 Hohle pro Monat bis zum Limit 40 × Schleierstärke
                let limit = 40.0 * veil.max(1.0);
                let gain = (5.0 + veil * 3.0) * hmult * if self.provinces[pi].has_state("dead_rest", self.day) { 0.3 } else { 1.0 };
                let p = &mut self.provinces[pi];
                p.hollow_force = (p.hollow_force + gain).min(limit);
                p.hollowing = (p.hollowing + 0.4).min(100.0);
            } else {
                let p = &mut self.provinces[pi];
                p.hollow_force *= 0.9;
            }
        }
        // Raubzüge: Hohlenstärke über 50 % des Limits → Heer entsteht
        let hollowed = self.realm_of_kind(RealmKind::Hollowed);
        let Some(h) = hollowed else { return };
        for pi in 0..self.provinces.len() {
            let (force, veil, shrine, owner) = (self.provinces[pi].hollow_force, self.provinces[pi].veil, self.provinces[pi].shrine, self.provinces[pi].owner);
            if shrine == 0 {
                continue;
            }
            let limit = 40.0 * veil.max(1.0);
            if force < limit * 0.5 || force < 30.0 {
                continue;
            }
            // Bereits ein Hohlenheer vor Ort oder zu viele Heere insgesamt?
            if self.armies.iter().filter(|a| a.owner == Some(h) && a.kind == ArmyKind::Realm).count() >= 10 {
                break;
            }
            let has_army = self.armies.iter().any(|a| a.location == pi as ProvId && a.owner == Some(h) && a.kind == ArmyKind::Realm);
            if has_army {
                continue;
            }
            let n = (force * 0.8) as u32;
            self.provinces[pi].hollow_force -= force * 0.6;
            let regs = vec![Regiment { unit: "hollow_thrall".into(), count: n * 7 / 10, vet: 0, hp: 1.0 }, Regiment { unit: "hollow_archer".into(), count: n * 2 / 10, vet: 0, hp: 1.0 }, Regiment { unit: "grave_brute".into(), count: n / 30 + 1, vet: 0, hp: 1.0 }];
            let nm = format!("Hohle von {}", self.pname(pi as ProvId));
            let aid = self.new_army(ArmyKind::Realm, Some(h), pi as ProvId, regs, &nm);
            let _ = owner;
            if let Some(a) = self.armies.iter_mut().find(|a| a.id == aid) {
                a.order = ArmyOrder::Idle;
                a.morale = 100.0;
            }
        }
    }

    /// KI der Hohlen und der Aschenlegion: Armeen ziehen gegen schwache Nachbarprovinzen lebender Reiche.
    pub(crate) fn hollow_ai_month(&mut self) {
        for kind in [RealmKind::Hollowed, RealmKind::Legion] {
            let Some(r) = self.realm_of_kind(kind) else { continue };
            // Heere am selben Ort vereinen
            let ids = self.realm_armies(r);
            for w in ids.windows(2) {
                let _ = self.merge_armies(w[0], w[1]);
            }
            if kind == RealmKind::Legion && !self.endgame.legion_awake {
                continue;
            }
            let ids = self.realm_armies(r);
            for id in ids {
                let Some(a) = self.army(id).cloned() else { continue };
                if !a.path.is_empty() || a.in_battle.is_some() || a.locked_until > self.day || a.order == ArmyOrder::Siege {
                    continue;
                }
                let power = crate::military::army_power(&a);
                let hmult = self.options.difficulty.mods().hollow_strength;
                if power < 400.0 {
                    continue;
                }
                let mut best: Option<(f32, ProvId)> = None;
                for p in &self.provinces {
                    let Some(o) = p.owner else { continue };
                    if is_antagonist(self.realms[o as usize].kind) {
                        continue;
                    }
                    // Nur angrenzend an Hohlland / Legion oder nahe dem Heer
                    let adjacent = p.neighbors.iter().any(|l| self.provinces[l.to as usize].owner == Some(r)) || self.provinces[a.location as usize].neighbors.iter().any(|l| l.to == p.id);
                    if !adjacent && !(kind == RealmKind::Legion && self.endgame.hollow_king_awake.is_some()) {
                        continue;
                    }
                    let defenders: f32 = self.armies.iter().filter(|x| x.location == p.id && self.armies_hostile(&a, x)).map(crate::military::army_power).sum();
                    let def = defenders + p.garrison as f32 * 25.0 * (1.0 + p.fortification as f32 * 0.1);
                    if power * hmult < def * 1.1 {
                        continue;
                    }
                    let mut s = 10.0 + p.settlement.rank() as f32 * 4.0 - def * 0.002;
                    // Friedhöfe/Schlachtfelder bevorzugt
                    if p.devastation > 0.2 {
                        s += 3.0;
                    }
                    if p.pop[Stratum::Warriors.idx()].adults < 20.0 {
                        s += 2.0;
                    }
                    if self.find_path(r, a.location, p.id).is_none() {
                        continue;
                    }
                    if best.is_none_or(|b| s > b.0) {
                        best = Some((s, p.id));
                    }
                }
                if let Some((_, t)) = best {
                    let _ = self.order_move(id, t);
                }
            }
        }
    }
}
