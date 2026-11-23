//! Politik: Adelshäuser (Macht, Loyalität, Fraktionen), Ämter mit Wirkungen, Gesetze, Adelsrat,
//! Intrigen mit nachvollziehbaren Erfolgsformeln (GDD Kapitel 6).

use crate::catalog::Trait;
use crate::chronicle::{ChronicleKind, Entry};
use crate::model::*;
use crate::types::*;
use crate::world::StrategyWorld;
use serde::{Deserialize, Serialize};

/// Aktionen gegenüber Adelshäusern.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HouseAction {
    /// Gold schenken (Loyalität steigt mit dem Betrag).
    Reward(f32),
    /// Recht gewähren (Steuerbefreiung, eigene Armee, Ratssitz, Stadtrecht, Gläubiger).
    Grant(HouseRight),
    /// Recht entziehen.
    Revoke(HouseRight),
    /// Ein Lehen (Provinz) verleihen: das Haus wird Statthalter dort.
    GrantFief(ProvId),
    /// Ein Lehen einziehen (Enteignung).
    Dispossess(ProvId),
    /// Bestrafen: Hausoberhaupt einkerkern (Loyalität sinkt stark, Macht des Hauses sinkt).
    Punish,
    /// Ein Mitglied der Herrscherfamilie mit einem Mitglied des Hauses verheiraten.
    Marry { own: CharId, other: CharId },
    /// Ins Amt berufen.
    Appoint(Office),
    /// In den Adelsrat berufen (Ratssitz).
    CouncilSeat,
}

impl StrategyWorld {
    // ------------------------------------------------------------------------------------
    // Ämter
    // ------------------------------------------------------------------------------------
    pub fn appoint_office(&mut self, r: RealmId, office: Office, c: CharId) -> Result<(), String> {
        let day = self.day;
        let ch = &self.chars[c as usize];
        if !ch.alive() || !ch.adult(day) {
            return Err("Kandidat nicht amtsfähig".into());
        }
        if ch.captive_of.is_some() {
            return Err("Kandidat ist gefangen".into());
        }
        if self.realms[r as usize].ruler == c {
            return Err("Der Herrscher kann kein Amt halten".into());
        }
        if ch.realm != Some(r) {
            return Err("Kandidat gehört nicht zum Hof".into());
        }
        if let Some((rr, o)) = ch.office {
            self.realms[rr as usize].offices[o.idx()] = None;
        }
        if let Some(old) = self.realms[r as usize].offices[office.idx()] {
            self.chars[old as usize].office = None;
        }
        self.realms[r as usize].offices[office.idx()] = Some(c);
        self.chars[c as usize].office = Some((r, office));
        // Ämterbesetzung durch Häuser: +10 Loyalität über Formel, Meinung
        if let Some(h) = self.chars[c as usize].house {
            self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty + 5.0).min(100.0);
        }
        Ok(())
    }

    pub fn dismiss_office(&mut self, r: RealmId, office: Office) -> Result<(), String> {
        let Some(c) = self.realms[r as usize].offices[office.idx()] else { return Err("Amt ist unbesetzt".into()) };
        self.realms[r as usize].offices[office.idx()] = None;
        self.chars[c as usize].office = None;
        self.remember(c, MemoryKind::Insulted, self.realms[r as usize].ruler, -20.0, 20);
        if let Some(h) = self.chars[c as usize].house {
            self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty - 5.0).max(0.0);
        }
        if office == Office::Spymaster {
            self.ruler_act(r, crate::catalog::StressAct::DismissSpymaster);
        }
        Ok(())
    }

    /// Wirkung der Ämter (monatlich), skaliert mit dem Attribut A des Inhabers.
    fn offices_month(&mut self, r: RealmId) {
        let day = self.day;
        let rl = &self.realms[r as usize];
        let offices = rl.offices;
        let tasks = rl.office_task;
        for (i, o) in OFFICES.iter().enumerate() {
            let Some(c) = offices[i] else { continue };
            if !self.chars[c as usize].alive() {
                self.realms[r as usize].offices[i] = None;
                continue;
            }
            let a = self.chars[c as usize].attr(o.attr()) as f32;
            let task = tasks[i];
            match o {
                Office::Chancellor => {
                    if task == 0 {
                        // Beziehungen pflegen: +A Meinung/Jahr bei einem gewählten Reich (Nachbar mit der besten Aussicht)
                        if let Some(t) = self.best_relations_target(r) {
                            self.add_opinion(t, r, OpKind::Envoy, a / 12.0, Some(day + 5 * DAYS_PER_YEAR));
                        }
                    } else if task == 1 {
                        // Ansprüche erarbeiten: Anspruch auf eine Grenzprovinz in max(6, 36−A) Monaten
                        let p = 1.0 / (36.0 - a).max(6.0);
                        if self.chance(p) {
                            self.fabricate_claim(r);
                        }
                    }
                }
                Office::Marshal => {
                    if task == 0 {
                        // Truppen drillen: Erfahrung
                        let ids = self.realm_armies(r);
                        for id in ids {
                            if self.chance(a * 0.006)
                                && let Some(arm) = self.armies.iter_mut().find(|x| x.id == id)
                                    && let Some(reg) = arm.regiments.iter_mut().find(|g| g.vet < 3) {
                                        reg.vet += 1;
                                    }
                        }
                    } else if task == 2 {
                        let months = (24.0 - a / 2.0).max(6.0);
                        if self.chance(1.0 / months) {
                            let provs = self.provinces_of(r).to_vec();
                            let frontier: Vec<ProvId> = provs.iter().copied().filter(|p| self.provinces[*p as usize].neighbors.iter().any(|l| self.provinces[l.to as usize].owner != Some(r)) && self.provinces[*p as usize].fortification < 10).collect();
                            if let Some(p) = frontier.first() {
                                self.provinces[*p as usize].fortification += 1;
                            }
                        }
                    }
                }
                Office::Diplomat => {
                    if task == 0 {
                        if let Some(t) = self.best_relations_target(r) {
                            self.add_opinion(t, r, OpKind::Envoy, a / 12.0, Some(day + 5 * DAYS_PER_YEAR));
                        }
                    } else if task == 2 {
                        // Frieden vermitteln: Kriegsmüdigkeit beider Seiten in einem Fremdkrieg
                        if let Some(w) = self.wars.iter().find(|w| w.active && !w.attackers.contains(&r) && !w.defenders.contains(&r)).cloned() {
                            for p in [w.leader_a, w.leader_d] {
                                self.realms[p as usize].war_weariness = (self.realms[p as usize].war_weariness + a * 0.5).min(100.0);
                            }
                            self.realms[r as usize].prestige += 2.0;
                        }
                    }
                }
                Office::Spymaster => {
                    if task == 1 {
                        // Spionagenetz im gefährlichsten Nachbarn
                        if let Some(t) = self.spy_target(r) {
                            let e = self.realms[r as usize].spy_net.entry(t).or_insert(0.0);
                            *e = (*e + a).min(100.0);
                        }
                    } else if task == 2 {
                        // Sabotage der Produktion einer feindlichen Provinz
                        if let Some(t) = self.war_enemy(r) {
                            let provs = self.provinces_of(t).to_vec();
                            if let Some(p) = provs.first() {
                                self.provinces[*p as usize].states.insert("strike".into(), day + 30);
                                if self.chance(((30.0 - a) / 100.0).max(0.0)) {
                                    self.add_opinion(t, r, OpKind::Plot, -30.0, Some(day + 10 * DAYS_PER_YEAR));
                                }
                            }
                        }
                    }
                }
                Office::HighPriest => {
                    self.realms[r as usize].piety += a * 0.1;
                    if task == 1 {
                        // Hohlung reinigen in der am stärksten betroffenen Provinz
                        let provs = self.provinces_of(r).to_vec();
                        if let Some(p) = provs.iter().copied().max_by(|a, b| self.provinces[*a as usize].hollowing.total_cmp(&self.provinces[*b as usize].hollowing)) {
                            let pr = &mut self.provinces[p as usize];
                            pr.hollowing = (pr.hollowing - a * 0.2).max(0.0);
                        }
                    }
                }
                Office::Treasurer | Office::Steward => {}
            }
        }
    }

    fn best_relations_target(&self, r: RealmId) -> Option<RealmId> {
        self.realms.iter().filter(|x| x.alive && x.id != r && !crate::diplomacy::is_antagonist(x.kind) && self.realm_distance(r, x.id) <= 2 && !self.at_war(r, x.id)).max_by(|a, b| {
            let (oa, ob) = (self.opinion(a.id, r), self.opinion(b.id, r));
            (60.0 - oa).max(0.0).total_cmp(&(60.0 - ob).max(0.0))
        }).map(|x| x.id)
    }

    fn spy_target(&self, r: RealmId) -> Option<RealmId> {
        self.realms.iter().filter(|x| x.alive && x.id != r && !crate::diplomacy::is_antagonist(x.kind) && self.realm_distance(r, x.id) == 1).max_by(|a, b| self.realms[a.id as usize].strength.total_cmp(&self.realms[b.id as usize].strength)).map(|x| x.id)
    }

    fn war_enemy(&self, r: RealmId) -> Option<RealmId> {
        self.wars.iter().filter(|w| w.active).find_map(|w| if w.attackers.contains(&r) { Some(w.leader_d) } else if w.defenders.contains(&r) { Some(w.leader_a) } else { None })
    }

    /// Anspruch auf eine Grenzprovinz erarbeiten (Anspruch = `original_owner`).
    pub fn fabricate_claim(&mut self, r: RealmId) {
        let mut cands: Vec<ProvId> = Vec::new();
        for p in self.provinces_of(r).to_vec() {
            for l in &self.provinces[p as usize].neighbors {
                let q = &self.provinces[l.to as usize];
                if let Some(o) = q.owner
                    && o != r && !crate::diplomacy::is_antagonist(self.realms[o as usize].kind) && q.original_owner.is_none() && self.realms[o as usize].vassal_of != Some(r) {
                        cands.push(l.to);
                    }
            }
        }
        cands.sort_unstable();
        cands.dedup();
        if cands.is_empty() {
            return;
        }
        let i = self.ri(0, cands.len() as i32 - 1) as usize;
        let p = cands[i];
        self.provinces[p as usize].original_owner = Some(r);
    }

    // ------------------------------------------------------------------------------------
    // Monat: Reiche, Häuser, Fraktionen
    // ------------------------------------------------------------------------------------
    pub(crate) fn realm_month(&mut self) {
        let day = self.day;
        for r in 0..self.realms.len() as RealmId {
            if !self.realms[r as usize].alive || crate::diplomacy::is_antagonist(self.realms[r as usize].kind) {
                continue;
            }
            self.offices_month(r);
            let rl = &mut self.realms[r as usize];
            rl.piety += 1.0 + if rl.religion == Religion::EternalFlame { 0.5 } else { 0.0 };
            rl.prestige += 0.3 + rl.income.max(0.0).sqrt() * 0.05;
            rl.tyranny = (rl.tyranny - 1.0 / 12.0).max(0.0);
            // Zeitweilige Ausnahme: Kriegsmüdigkeit baut im Frieden ab
            if !self.wars.iter().any(|w| w.active && (w.attackers.contains(&r) || w.defenders.contains(&r))) {
                let rl = &mut self.realms[r as usize];
                rl.war_weariness = (rl.war_weariness - 2.0).max(0.0);
            }
            // Flotte: aus Häfen
            let ports: u32 = self.provinces_of(r).iter().map(|p| self.provinces[*p as usize].service_count(crate::goods::ServiceKind::Harbor) as u32).sum();
            self.realms[r as usize].fleet = ports + if self.realms[r as usize].faction == "saelith_compact" { 4 } else { 0 };
            // Kredite Kirche: Verzug etc. entfällt (siehe Wirtschaft)
            let _ = day;
        }
    }

    /// Jahreswechsel der Reiche: Vertragsbrüche verblassen, Ämter werden geprüft.
    pub(crate) fn realm_year(&mut self) {
        let day = self.day;
        for r in 0..self.realms.len() {
            if !self.realms[r].alive {
                continue;
            }
            if self.realms[r].treaty_breaks > 0 && day.is_multiple_of(10 * DAYS_PER_YEAR) {
                self.realms[r].treaty_breaks -= 1;
            }
            // Abwesende Ämter bereinigen
            for i in 0..7 {
                if let Some(c) = self.realms[r].offices[i]
                    && !self.chars[c as usize].alive() {
                        self.realms[r].offices[i] = None;
                    }
            }
        }
    }

    /// Berechnet Macht und Loyalität der Häuser, bildet Fraktionen, löst Ultimaten und Rebellionen aus.
    pub(crate) fn politics_month(&mut self) {
        let nr = self.realms.len();
        for r in 0..nr as RealmId {
            if !self.realms[r as usize].alive || crate::diplomacy::is_antagonist(self.realms[r as usize].kind) {
                continue;
            }
            self.houses_month(r);
        }
    }

    fn houses_month(&mut self, r: RealmId) {
        let day = self.day;
        let houses: Vec<HouseId> = self.houses.iter().filter(|h| h.alive && h.realm == r).map(|h| h.id).collect();
        if houses.is_empty() {
            return;
        }
        let realm_pop: f32 = self.provinces_of(r).iter().map(|p| self.provinces[*p as usize].population()).sum::<f32>().max(1.0);
        let ruler = self.realms[r as usize].ruler;
        let legit = self.realms[r as usize].legitimacy;
        let (tax, levy, ca, serf, tyr) = {
            let l = &self.realms[r as usize].laws;
            (l.vassal_tax, l.levy, l.crown_authority, l.serfdom, self.realms[r as usize].tyranny)
        };
        let holders: Vec<CharId> = self.realms[r as usize].offices.iter().flatten().copied().collect();
        let mut sum_power = 0.0;
        for &h in &houses {
            let fief_pop: f32 = self.provinces.iter().filter(|p| p.fief == Some(h) && p.owner == Some(r)).map(|p| p.population()).sum();
            let head = self.houses[h as usize].head;
            let retinue = self.houses[h as usize].retinue as f32;
            let wealth = self.houses[h as usize].wealth;
            let power = (fief_pop / realm_pop * 100.0 * 0.8 + retinue * 0.02 + wealth * 0.0008).clamp(0.0, 100.0);
            let opinion = self.char_opinion(head, ruler);
            let hh = &self.houses[h as usize];
            let rights = hh.rights.len() as f32;
            let office_bonus = holders.iter().any(|c| self.chars[*c as usize].house == Some(h));
            let target = 50.0 + opinion * 0.4 + 5.0 * rights - (tax - 20.0) - (levy - 25.0) * 0.8 + legit * 0.1 - 5.0 - if hh.throne_claim { 15.0 } else { 0.0 } + if office_bonus { 10.0 } else { 0.0 } - [0.0, 5.0, 10.0, 20.0][ca.min(3) as usize] + match serf {
                0 => 10.0,
                1 => 0.0,
                _ => -10.0,
            } - tyr * 0.2 - if self.endgame.great_power == Some(r) { 10.0 } else { 0.0 } - if self.realms[r as usize].regency.is_some() { 6.0 } else { 0.0 } - (self.realms[r as usize].war_weariness - 40.0).max(0.0) * 0.3 + if self.in_any_war(r) && hh.leaning == HouseLeaning::War { 6.0 } else { 0.0 } - if self.in_any_war(r) && hh.leaning == HouseLeaning::Trade { 4.0 } else { 0.0 };
            let hh = &mut self.houses[h as usize];
            hh.power = power;
            sum_power += power;
            hh.loyalty = (hh.loyalty + (target - hh.loyalty) * 0.25).clamp(0.0, 100.0);
            // Aufrüstung des Gefolges
            let want = (fief_pop * 0.006) as u32 + 10;
            if hh.retinue < want {
                hh.retinue += 2;
            } else if hh.retinue > want {
                hh.retinue -= 1;
            }
            hh.wealth = (hh.wealth * 0.995).max(0.0);
            // Aufstieg: Haus mit Macht über Loyalität etc. bleibt
            hh.grudge = (hh.grudge - 0.5).max(0.0);
        }
        // Fraktionen
        let crown_power: f32 = (100.0 - sum_power).max(10.0) + houses.iter().filter(|h| self.houses[**h as usize].loyalty >= 50.0).map(|h| self.houses[*h as usize].power).sum::<f32>();
        let unhappy: Vec<HouseId> = houses.iter().copied().filter(|h| self.houses[*h as usize].loyalty < 30.0 && self.houses[*h as usize].faction.is_none()).collect();
        for h in unhappy {
            if self.chance(0.35) {
                self.join_or_form_faction(r, h);
            }
        }
        // Mitglieder mit wieder hoher Loyalität verlassen die Fraktion
        let fids: Vec<u32> = self.factions.iter().filter(|f| f.realm == r).map(|f| f.id).collect();
        for fid in fids {
            let Some(fi) = self.factions.iter().position(|f| f.id == fid) else { continue };
            let members = self.factions[fi].members.clone();
            let mut keep = Vec::new();
            for m in members {
                if self.houses[m as usize].alive && self.houses[m as usize].loyalty < 45.0 {
                    keep.push(m);
                } else {
                    self.houses[m as usize].faction = None;
                }
            }
            if keep.is_empty() {
                self.factions.remove(fi);
                continue;
            }
            self.factions[fi].members = keep.clone();
            if !keep.contains(&self.factions[fi].leader) {
                self.factions[fi].leader = keep[0];
            }
            let leader_head = self.houses[self.factions[fi].leader as usize].head;
            let int = self.chars[leader_head as usize].attr(Attr::Intrigue) as f32;
            let strength: f32 = keep.iter().map(|m| self.houses[*m as usize].power).sum::<f32>() * (1.0 + 0.02 * int);
            if !self.factions[fi].ultimatum && strength >= 1.2 * crown_power && strength > 15.0 {
                self.factions[fi].ultimatum = true;
                self.faction_ultimatum(r, fid);
            }
        }
        // Offene Rebellion: Loyalität < 15 und Macht > 20
        for &h in &houses {
            let hh = &self.houses[h as usize];
            if hh.alive && hh.loyalty < 15.0 && hh.power > 10.0 && self.chance(0.15) {
                self.house_rebellion(h);
                break;
            }
        }
        // Throne claim bei geringer Legitimität
        if legit < 30.0 {
            for &h in &houses {
                if !self.houses[h as usize].throne_claim && self.houses[h as usize].power > 10.0 && self.chance(0.04) {
                    self.houses[h as usize].throne_claim = true;
                    let head = self.houses[h as usize].head;
                    self.chars[head as usize].claims.push(r);
                }
            }
        }
        let _ = day;
    }

    fn join_or_form_faction(&mut self, r: RealmId, h: HouseId) {
        let demand = {
            let hh = &self.houses[h as usize];
            if hh.throne_claim {
                FactionDemand::OtherHeir
            } else {
                match hh.leaning {
                    HouseLeaning::Separatist => FactionDemand::Independence,
                    HouseLeaning::Faith => FactionDemand::ReligiousReform,
                    HouseLeaning::War => {
                        let t = self.realms.iter().filter(|x| x.alive && x.id != r && self.realm_distance(r, x.id) == 1 && !crate::diplomacy::is_antagonist(x.kind)).map(|x| x.id).next();
                        match t {
                            Some(t) => FactionDemand::WarOn(t),
                            None => FactionDemand::LessCrownAuthority,
                        }
                    }
                    _ => FactionDemand::LessCrownAuthority,
                }
            }
        };
        if let Some(fi) = self.factions.iter().position(|f| f.realm == r && f.demand == demand) {
            self.factions[fi].members.push(h);
            self.houses[h as usize].faction = Some(self.factions[fi].id);
            return;
        }
        let id = self.next_faction;
        self.next_faction += 1;
        let head = self.houses[h as usize].head;
        self.factions.push(PoliticalFaction { id, realm: r, demand, members: vec![h], leader: h, ultimatum: false, started: self.day, candidate: if demand == FactionDemand::OtherHeir { Some(head) } else { None } });
        self.houses[h as usize].faction = Some(id);
    }

    fn faction_ultimatum(&mut self, r: RealmId, fid: u32) {
        let Some(f) = self.factions.iter().find(|f| f.id == fid).cloned() else { return };
        let name = self.houses[f.leader as usize].name.clone();
        let demand = match f.demand {
            FactionDemand::LessCrownAuthority => "Weniger Kronautorität",
            FactionDemand::Independence => "Unabhängigkeit",
            FactionDemand::OtherHeir => "Ein anderer Thronerbe",
            FactionDemand::WarOn(_) => "Krieg gegen einen Nachbarn",
            FactionDemand::ReligiousReform => "Religiöse Reform",
        };
        let e = Entry::new(ChronicleKind::Plot, "faction_ultimatum", 50).realm(r).var("a", self.rname(r)).var("c", format!("{demand} (angeführt von Haus {name})"));
        self.chron(e);
        if self.is_player(r) {
            // Spielerentscheidung: Ereignisfenster
            let id = self.events.next_id;
            self.events.next_id += 1;
            self.events.pending.push(crate::world::PendingEvent {
                id,
                event_id: format!("faction_ultimatum:{fid}"),
                realm: r,
                province: None,
                title: format!("Ultimatum der Häuser: {demand}"),
                text: format!("Haus {name} und seine Verbündeten fordern: {demand}. Gebt Ihr nach, kostet es 100 Prestige. Lehnt Ihr ab, droht der Bürgerkrieg."),
                options: vec![("yield".into(), "Nachgeben".into(), true), ("refuse".into(), "Ablehnen".into(), true)],
                day: self.day,
            });
            self.emit(crate::world::StrategyEvent::EventWindow { pending: id });
            return;
        }
        // KI-Herrscher: Nachgeben bei schwacher Legitimität, sonst eher ablehnen
        let ruler = &self.chars[self.realms[r as usize].ruler as usize];
        let mut p_yield = 0.35 + if ruler.has(Trait::Merciful) { 0.2 } else { 0.0 } + if ruler.has(Trait::Cruel) || ruler.has(Trait::Wrathful) { -0.25 } else { 0.0 };
        if self.realms[r as usize].legitimacy < 40.0 {
            p_yield += 0.2;
        }
        if self.chance(p_yield) {
            self.resolve_faction_ultimatum(r, fid, true);
        } else {
            self.resolve_faction_ultimatum(r, fid, false);
        }
    }

    /// Löst ein Ultimatum auf: Nachgeben (Forderung erfüllt) oder Ablehnen (Bürgerkrieg).
    pub fn resolve_faction_ultimatum(&mut self, r: RealmId, fid: u32, yield_: bool) {
        let Some(fi) = self.factions.iter().position(|f| f.id == fid) else { return };
        let f = self.factions.remove(fi);
        if yield_ {
            self.realms[r as usize].prestige -= 100.0;
            match f.demand {
                FactionDemand::LessCrownAuthority => {
                    let l = &mut self.realms[r as usize].laws;
                    l.crown_authority = l.crown_authority.saturating_sub(1);
                    l.vassal_tax = (l.vassal_tax - 5.0).max(5.0);
                }
                FactionDemand::Independence => {
                    let provs: Vec<ProvId> = self.provinces.iter().filter(|p| p.owner == Some(r) && p.fief.is_some_and(|h| f.members.contains(&h))).map(|p| p.id).collect();
                    if !provs.is_empty() && self.provinces_of(r).len() > provs.len() {
                        let head = self.houses[f.leader as usize].head;
                        let name = format!("Herzogtum {}", self.houses[f.leader as usize].name);
                        let nr = self.found_realm(&name, r, head, provs, None);
                        let e = Entry::new(ChronicleKind::Diplomacy, "independence", 50).realm(nr).var("a", name).var("b", self.rname(r));
                        self.chron(e);
                    }
                }
                FactionDemand::OtherHeir => {
                    if let Some(c) = f.candidate
                        && self.chars[c as usize].alive() {
                            self.realms[r as usize].heir = Some(c);
                        }
                }
                FactionDemand::WarOn(t) => {
                    if let Some(cb) = self.find_cb(r, t).or(Some(CasusBelli::Unjustified)) {
                        let _ = self.declare_war(r, t, cb, WarGoal::Tribute);
                    }
                }
                FactionDemand::ReligiousReform => {
                    self.realms[r as usize].laws.faith_law = 1;
                }
            }
            for m in &f.members {
                if self.houses[*m as usize].alive {
                    self.houses[*m as usize].loyalty = (self.houses[*m as usize].loyalty + 25.0).min(100.0);
                    self.houses[*m as usize].faction = None;
                }
            }
        } else {
            self.house_rebellion(f.leader);
            for m in &f.members {
                if *m != f.leader && self.houses[*m as usize].alive && self.houses[*m as usize].loyalty < 35.0 {
                    // Mitglieder schließen sich dem Bürgerkrieg an
                    self.houses[*m as usize].faction = None;
                }
            }
        }
    }

    /// Ein Adelshaus erhebt sich: aus seinen Lehen entsteht ein eigenes Reich im Krieg gegen den Lehnsherrn.
    pub fn house_rebellion(&mut self, h: HouseId) {
        let r = self.houses[h as usize].realm;
        let provs: Vec<ProvId> = self.provinces.iter().filter(|p| p.fief == Some(h) && p.owner == Some(r)).map(|p| p.id).collect();
        if provs.is_empty() || self.provinces_of(r).len() <= provs.len() {
            // Ohne Land: Putschplan statt offener Rebellion
            let head = self.houses[h as usize].head;
            let _ = self.start_plot(head, PlotKind::Usurp, PlotTarget::Throne(r), None);
            self.houses[h as usize].loyalty = 25.0;
            return;
        }
        let head = self.houses[h as usize].head;
        let name = format!("Haus {}", self.houses[h as usize].name);
        let claim = self.houses[h as usize].throne_claim;
        let nr = self.found_realm(&name, r, head, provs, None);
        self.houses[h as usize].alive = false;
        // Gefolge als Heer
        let retinue = self.houses[h as usize].retinue.max(20);
        let roster = crate::catalog::roster(self.realms[nr as usize].culture);
        let unit = roster.first().copied().unwrap_or("spearman").to_string();
        let seat = self.realms[nr as usize].capital;
        let reg = vec![Regiment { unit, count: retinue * 3, vet: 1, hp: 1.0 }];
        let aid = self.new_army(ArmyKind::Realm, Some(nr), seat, reg, &format!("Heer des Hauses {}", self.houses[h as usize].name));
        self.assign_commander(aid);
        self.realms[nr as usize].gold += self.houses[h as usize].wealth + 150.0;
        let e = Entry::new(ChronicleKind::War, "house_rebels", 55).realm(nr).var("a", self.houses[h as usize].name.clone()).var("b", self.rname(r));
        self.chron(e);
        let (cb, goal) = if claim { (CasusBelli::Throne, WarGoal::Throne) } else { (CasusBelli::Independence, WarGoal::Independence) };
        self.realms[nr as usize].vassal_of = Some(r);
        self.realms[nr as usize].vassal_of = Some(r); // Lehnsstatus, damit der Unabhängigkeitskrieg zulässig ist
        let res = self.declare_war(nr, r, cb, goal);
        if res.is_err() {
            self.realms[nr as usize].vassal_of = None;
        }
        // Gleichgesinnte Häuser schließen sich an
        let others: Vec<HouseId> = self.houses.iter().filter(|x| x.alive && x.realm == r && x.loyalty < 30.0 && x.id != h).map(|x| x.id).collect();
        for o in others {
            if let Ok(w) = res {
                let _ = w;
            }
            self.houses[o as usize].loyalty -= 5.0;
        }
        self.stats.revolts += 1;
    }

    /// Gründet ein neues Reich aus Provinzen eines bestehenden (Kulturen, Religion, Gesetze erbt es).
    pub fn found_realm(&mut self, name: &str, template: RealmId, ruler: CharId, provs: Vec<ProvId>, vassal_of: Option<RealmId>) -> RealmId {
        let id = self.realms.len() as RealmId;
        let t = self.realms[template as usize].clone();
        let day = self.day;
        let capital = provs.iter().copied().max_by(|a, b| self.provinces[*a as usize].population().total_cmp(&self.provinces[*b as usize].population())).unwrap_or(provs[0]);
        let ch = &self.chars[ruler as usize];
        let (culture, religion, dynasty) = (ch.culture, ch.religion, ch.dynasty);
        let mut laws = t.laws.clone();
        laws.last_succession_change = day;
        let personality = self.random_personality(ruler);
        let kind = if provs.len() > 3 { RealmKind::Duchy } else { RealmKind::Free };
        let banner = self.rng_u32();
        self.realms.push(Realm {
            id,
            key: format!("realm_{id}"),
            name: name.to_string(),
            faction: t.faction.clone(),
            kind,
            alive: true,
            culture,
            religion,
            gov: if t.gov == Government::Legion || t.gov == Government::HollowCourt { Government::Feudal } else { t.gov },
            rank: Rank::Count,
            ruler,
            dynasty,
            capital,
            vassal_of,
            gold: 200.0,
            tax_level: 2,
            trade_tax: 5.0,
            tariff_import: 5.0,
            tariff_transit: 2.0,
            loans: Vec::new(),
            credit_blocked_until: 0,
            laws,
            offices: [None; 7],
            office_task: [0; 7],
            prestige: 50.0,
            piety: 50.0,
            tyranny: 0.0,
            war_weariness: 0.0,
            legitimacy: 55.0,
            strength: 0.0,
            threat: 0.0,
            recent_conquests: Vec::new(),
            treaty_breaks: 0,
            goals: Vec::new(),
            personality,
            regency: None,
            heir: None,
            research: [ResearchState::default(); 9],
            research_focus: None,
            ashglass: 0.0,
            soul_ash: 0.0,
            hollow_path: 0,
            hollow_value: 0.0,
            void_uses: Vec::new(),
            spy_net: Default::default(),
            court: Vec::new(),
            fleet: 0,
            is_player: false,
            embargo: Vec::new(),
            income: 0.0,
            expenses: 0.0,
            great_power: false,
            memory: Vec::new(),
            stats: RealmStats { rulers: 1, peak_provinces: provs.len() as u32, ..Default::default() },
            founded: day,
            fallen: None,
            peace_reputation_until: 0,
            last_peace_day: day,
            forbidden_discovered: false,
            ritual_cooldown: 0,
            last_ruler_change: day,
            tribute_paid: 0.0,
            banner,
            last_event: Default::default(),
            heresy_pressure: 0.0,
            recruit_budget: 0.0,
            excommunicated: false,
            shrines_restored: 0,
            mage_orders: Vec::new(),
            artifacts: Vec::new(),
            pending_regime: false,
        });
        // Provinzen übertragen
        for p in &provs {
            let old = self.provinces[*p as usize].owner;
            let pr = &mut self.provinces[*p as usize];
            pr.owner = Some(id);
            pr.controller = None;
            pr.fief = None;
            pr.siege = None;
            if pr.original_owner.is_none() {
                pr.original_owner = old;
            }
            pr.captured_day = day;
        }
        // Herrscher und Familie wechseln
        self.court_add(id, ruler);
        if let Some(o) = self.chars[ruler as usize].office {
            self.realms[o.0 as usize].offices[o.1.idx()] = None;
            self.chars[ruler as usize].office = None;
        }
        let fam: Vec<CharId> = {
            let c = &self.chars[ruler as usize];
            c.children.iter().copied().chain(c.spouse).filter(|x| self.chars[*x as usize].alive()).collect()
        };
        for f in fam {
            if let Some(old) = self.chars[f as usize].realm {
                self.court_remove(old, f);
            }
            self.court_add(id, f);
        }
        // Haus des Herrschers bleibt im alten Reich ausgetragen
        self.chars[ruler as usize].reign_start = Some(day);
        self.chars[ruler as usize].location = capital;
        self.rebuild_caches();
        // Neue Häuser aus ehemaligen Lehnshäusern entstehen bei Bedarf nicht – Reich startet schlank
        self.stats.realms_founded += 1;
        self.emit(crate::world::StrategyEvent::RealmFounded { realm: id });
        if self.realms[template as usize].alive && self.provinces_of(template).is_empty() {
            self.check_realm_death(template);
        }
        let e = Entry::new(ChronicleKind::Realm, "realm_founded", 45).realm(id).var("a", name.to_string());
        self.chron(e);
        // Meinung zum Ursprungsreich
        self.add_opinion(id, template, OpKind::Betrayal, -30.0, Some(day + 20 * DAYS_PER_YEAR));
        self.opinion_mods.entry((template, id)).or_default();
        // Das Reich braucht Ämter und Hof
        self.assign_basic_court(id);
        id
    }

    /// Erzeugt eine KI-Persönlichkeit aus den Eigenschaften des Herrschers.
    pub fn random_personality(&self, ruler: CharId) -> Personality {
        let c = &self.chars[ruler as usize];
        if c.has(Trait::Ambitious) && c.has(Trait::Brave) || c.has(Trait::Wrathful) {
            Personality::Conqueror
        } else if c.has(Trait::Greedy) || c.has(Trait::Patient) {
            Personality::Merchant
        } else if c.has(Trait::Zealous) {
            Personality::Zealot
        } else if c.has(Trait::Deceitful) || c.has(Trait::Paranoid) {
            Personality::Schemer
        } else if c.has(Trait::Content) || c.has(Trait::Just) {
            Personality::Keeper
        } else if c.has(Trait::Ambitious) {
            Personality::Conqueror
        } else {
            Personality::Keeper
        }
    }

    /// Einfache Hofgründung für neu entstandene Reiche (Familie bleibt, Ämter durch Kandidaten).
    pub fn assign_basic_court(&mut self, r: RealmId) {
        let ruler = self.realms[r as usize].ruler;
        let culture = self.realms[r as usize].culture;
        let dynasty = self.realms[r as usize].dynasty;
        let cap = self.realms[r as usize].capital;
        let n = 2 + self.ri(0, 2);
        for _ in 0..n {
            let g = if self.chance(0.5) { Gender::Male } else { Gender::Female };
            let age = 24 + self.ri(0, 25);
            let fam = if self.chance(0.4) { self.dynasties[dynasty as usize].name.clone() } else { self.random_family_name(culture) };
            let d = if fam == self.dynasties[dynasty as usize].name { dynasty } else { self.create_dynasty(&fam, culture) };
            let c = self.spawn_person(None, culture, d, g, age, Some(r), None, cap);
            let _ = c;
        }
        self.fill_offices(r);
        let _ = ruler;
    }

    /// Besetzt leere Ämter mit den besten Kandidaten (KI und Autoverwaltung).
    pub fn fill_offices(&mut self, r: RealmId) {
        let day = self.day;
        for o in OFFICES {
            if self.realms[r as usize].offices[o.idx()].is_some_and(|c| self.chars[c as usize].alive()) {
                continue;
            }
            if o == Office::Steward && self.realms[r as usize].gov == Government::MerchantRepublic {
                // Republiken: Statthalter ebenfalls
            }
            let ruler = self.realms[r as usize].ruler;
            let cand = self.realms[r as usize].court.iter().copied().filter(|c| {
                let ch = &self.chars[*c as usize];
                ch.alive() && ch.adult(day) && ch.office.is_none() && ch.captive_of.is_none() && *c != ruler && ch.army.is_none() && !ch.has(Trait::Lunatic)
            });
            let best = cand.max_by_key(|c| {
                let ch = &self.chars[*c as usize];
                ch.attr(o.attr()) * 10 + ch.attr(Attr::Charisma) + if ch.has(Trait::Greedy) { -4 } else { 0 } + if ch.has(Trait::Ambitious) { -2 } else { 0 }
            });
            if let Some(c) = best {
                let _ = self.appoint_office(r, o, c);
                // Aufgabe nach Lage wählen
                let task = match o {
                    Office::Marshal => 0,
                    Office::Treasurer => 0,
                    Office::Spymaster => 1,
                    Office::Diplomat => 0,
                    Office::Chancellor => 0,
                    Office::HighPriest => 1,
                    Office::Steward => 1,
                };
                self.realms[r as usize].office_task[o.idx()] = task;
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Aktionen gegenüber Häusern
    // ------------------------------------------------------------------------------------
    pub fn house_action(&mut self, r: RealmId, h: HouseId, action: HouseAction) -> Result<String, String> {
        if h as usize >= self.houses.len() || !self.houses[h as usize].alive || self.houses[h as usize].realm != r {
            return Err("Haus gehört nicht zum Reich".into());
        }
        let ruler = self.realms[r as usize].ruler;
        let head = self.houses[h as usize].head;
        match action {
            HouseAction::Reward(gold) => {
                if gold <= 0.0 || self.realms[r as usize].gold < gold {
                    return Err("Nicht genug Gold".into());
                }
                self.realms[r as usize].gold -= gold;
                let hh = &mut self.houses[h as usize];
                hh.wealth += gold;
                hh.loyalty = (hh.loyalty + (gold / 40.0).min(25.0)).min(100.0);
                self.remember(head, MemoryKind::Gifted, ruler, 10.0, 10);
                Ok("Das Haus ist erfreut".into())
            }
            HouseAction::Grant(right) => {
                if self.houses[h as usize].rights.contains(&right) {
                    return Err("Recht bereits gewährt".into());
                }
                self.houses[h as usize].rights.push(right);
                self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty + 8.0).min(100.0);
                Ok("Recht gewährt".into())
            }
            HouseAction::Revoke(right) => {
                if !self.houses[h as usize].rights.contains(&right) {
                    return Err("Haus besitzt dieses Recht nicht".into());
                }
                if !self.council_approves(r, Some(h)) {
                    self.realms[r as usize].prestige -= 200.0;
                    for o in self.houses.iter_mut().filter(|x| x.alive && x.realm == r && x.id != h) {
                        o.loyalty = (o.loyalty - 20.0).max(0.0);
                    }
                }
                self.houses[h as usize].rights.retain(|x| *x != right);
                self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty - 15.0).max(0.0);
                Ok("Recht entzogen".into())
            }
            HouseAction::GrantFief(p) => {
                if self.provinces[p as usize].owner != Some(r) || self.provinces[p as usize].fief.is_some() || self.realms[r as usize].capital == p {
                    return Err("Provinz nicht verleihbar".into());
                }
                self.provinces[p as usize].fief = Some(h);
                self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty + 12.0).min(100.0);
                self.houses[h as usize].seat = p;
                Ok("Lehen verliehen".into())
            }
            HouseAction::Dispossess(p) => {
                if self.provinces[p as usize].fief != Some(h) {
                    return Err("Haus hält dieses Lehen nicht".into());
                }
                self.provinces[p as usize].fief = None;
                let hh = &mut self.houses[h as usize];
                hh.loyalty = (hh.loyalty - 30.0).max(0.0);
                hh.grudge += 30.0;
                self.remember(head, MemoryKind::Insulted, ruler, -40.0, 30);
                self.ruler_act(r, crate::catalog::StressAct::FabricateClaim);
                Ok("Lehen eingezogen".into())
            }
            HouseAction::Punish => {
                let cruel = self.chars[ruler as usize].has(Trait::Cruel);
                let hh = &mut self.houses[h as usize];
                hh.loyalty = (hh.loyalty - 25.0).max(0.0);
                hh.power *= 0.9;
                hh.wealth *= 0.5;
                self.chars[head as usize].prestige -= 80.0;
                if !cruel {
                    self.realms[r as usize].tyranny += 5.0;
                }
                self.remember(head, MemoryKind::Insulted, ruler, -30.0, 25);
                Ok("Das Haus wurde bestraft".into())
            }
            HouseAction::Marry { own, other } => {
                let (a, b) = (&self.chars[own as usize], &self.chars[other as usize]);
                if a.realm != Some(r) || b.house != Some(h) || a.spouse.is_some() || b.spouse.is_some() || a.gender == b.gender || !a.alive() || !b.alive() {
                    return Err("Ungeeignetes Paar".into());
                }
                if self.close_kin(own, other) {
                    return Err("Zu nah verwandt".into());
                }
                self.marry(own, other, false);
                self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty + 15.0).min(100.0);
                Ok("Die Häuser sind verbunden".into())
            }
            HouseAction::Appoint(office) => {
                self.appoint_office(r, office, head).map(|_| "Amt vergeben".to_string())
            }
            HouseAction::CouncilSeat => {
                if !self.houses[h as usize].rights.contains(&HouseRight::CouncilSeat) {
                    self.houses[h as usize].rights.push(HouseRight::CouncilSeat);
                }
                self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty + 8.0).min(100.0);
                Ok("In den Rat berufen".into())
            }
        }
    }

    /// Adelsrat: Mehrheit nach Macht gewichtet (loyale Häuser stimmen zu). Reiche mit weniger als drei Häusern haben keinen Rat.
    pub fn council_approves(&self, r: RealmId, ignore: Option<HouseId>) -> bool {
        let houses: Vec<&NobleHouse> = self.houses.iter().filter(|h| h.alive && h.realm == r && Some(h.id) != ignore).collect();
        if houses.len() < 3 {
            return true;
        }
        let total: f32 = houses.iter().map(|h| h.power.max(1.0)).sum();
        let yes: f32 = houses.iter().filter(|h| h.loyalty >= 45.0).map(|h| h.power.max(1.0)).sum();
        yes * 2.0 > total
    }

    // ------------------------------------------------------------------------------------
    // Intrigen
    // ------------------------------------------------------------------------------------
    /// Komplottstärke K (GDD 6.6).
    pub fn plot_strength(&self, p: &Plot) -> f32 {
        let leader = &self.chars[p.leader as usize];
        let mut k = leader.attr(Attr::Intrigue) as f32 * 3.0;
        for c in &p.conspirators {
            let ch = &self.chars[*c as usize];
            let power = ch.house.map(|h| self.houses[h as usize].power).unwrap_or(2.0);
            k += ch.attr(Attr::Intrigue) as f32 + power * 0.5;
        }
        // Netzwerk im Zielreich
        let net = p.sponsor.and_then(|s| self.realms[s as usize].spy_net.get(&p.realm).copied()).unwrap_or(0.0);
        k += net * 0.3;
        if leader.has(Trait::Deceitful) {
            k *= 1.15;
        }
        if leader.has(Trait::Honest) {
            k *= 0.85;
        }
        k
    }

    /// Abwehr A (GDD 6.6).
    pub fn plot_defense(&self, p: &Plot) -> f32 {
        let realm = &self.realms[p.realm as usize];
        let spy = realm.offices[Office::Spymaster.idx()].map(|c| self.chars[c as usize].attr(Attr::Intrigue) as f32).unwrap_or(0.0);
        let target_char = match p.target {
            PlotTarget::Char(c) => Some(c),
            PlotTarget::Throne(_) | PlotTarget::Realm(_) => Some(realm.ruler),
            PlotTarget::Prov(_) => None,
        };
        let mut a = spy * 3.0;
        if let Some(t) = target_char {
            let tc = &self.chars[t as usize];
            a += tc.attr(Attr::Intrigue) as f32 * 1.5;
            let heroes = self.chars.iter().filter(|c| c.alive() && c.is_hero && c.location == tc.location && c.realm == tc.realm && c.id != t).count() as f32;
            a += (heroes * 10.0).min(30.0);
            if tc.has(Trait::Paranoid) {
                a += 20.0;
            }
        }
        if realm.office_task[Office::Spymaster.idx()] == 0 {
            a += spy * 3.0 * 0.0 + spy;
        }
        a
    }

    /// Erfolgschance in Prozent: clamp(K − A − Schwierigkeit + 50, 5, 95).
    pub fn plot_success_chance(&self, p: &Plot) -> f32 {
        (self.plot_strength(p) - self.plot_defense(p) - p.kind.difficulty() + 50.0).clamp(5.0, 95.0)
    }

    pub fn start_plot(&mut self, leader: CharId, kind: PlotKind, target: PlotTarget, sponsor: Option<RealmId>) -> Result<PlotId, String> {
        let day = self.day;
        let ch = &self.chars[leader as usize];
        if !ch.alive() || !ch.adult(day) || ch.captive_of.is_some() {
            return Err("Anführer ungeeignet".into());
        }
        let realm = match target {
            PlotTarget::Char(c) => self.chars[c as usize].realm.ok_or("Ziel ohne Reich")?,
            PlotTarget::Prov(p) => self.provinces[p as usize].owner.ok_or("Provinz ohne Besitzer")?,
            PlotTarget::Realm(r) | PlotTarget::Throne(r) => r,
        };
        if self.plots.iter().any(|p| !p.done && p.leader == leader && p.kind == kind && p.target == target) {
            return Err("Komplott läuft bereits".into());
        }
        if matches!(target, PlotTarget::Char(c) if c == leader) {
            return Err("Kein Komplott gegen sich selbst".into());
        }
        let id = self.next_plot;
        self.next_plot += 1;
        self.plots.push(Plot { id, kind, leader, conspirators: Vec::new(), target, realm, sponsor, progress: 0.0, started: day, candidate: if kind == PlotKind::Usurp { Some(leader) } else { None }, discovered: false, done: false });
        self.stats.plots_started += 1;
        // Eigenschaften: Ehrlicher Anführer → Stress
        self.chars[leader as usize].stress = (self.chars[leader as usize].stress + if self.chars[leader as usize].has(Trait::Honest) { 15.0 } else { 0.0 }).min(100.0);
        Ok(id)
    }

    /// Mitverschwörer anwerben (max. 6).
    pub fn recruit_conspirator(&mut self, plot: PlotId, c: CharId) -> Result<(), String> {
        let idx = self.plots.iter().position(|p| p.id == plot).ok_or("Komplott nicht gefunden")?;
        let p = &self.plots[idx];
        if p.conspirators.len() >= 6 || p.conspirators.contains(&c) || p.leader == c {
            return Err("Nicht möglich".into());
        }
        let op = self.char_opinion(c, p.leader);
        let hooked = self.chars[c as usize].secrets.iter().any(|s| s.known_by.contains(&p.leader));
        if op > 10.0 || hooked || self.chance(((op + 40.0) / 120.0).clamp(0.05, 0.8)) {
            self.plots[idx].conspirators.push(c);
            Ok(())
        } else {
            Err("Abgelehnt".into())
        }
    }

    pub(crate) fn plots_month(&mut self) {
        let ids: Vec<PlotId> = self.plots.iter().filter(|p| !p.done).map(|p| p.id).collect();
        for id in ids {
            let Some(i) = self.plots.iter().position(|p| p.id == id) else { continue };
            // Anführer oder Ziel ungültig?
            let (leader, realm) = (self.plots[i].leader, self.plots[i].realm);
            if !self.chars[leader as usize].alive() || !self.realms[realm as usize].alive {
                self.plots[i].done = true;
                continue;
            }
            if let PlotTarget::Char(c) = self.plots[i].target
                && !self.chars[c as usize].alive() {
                    self.plots[i].done = true;
                    continue;
                }
            // Anwerbung (KI): Mitverschwörer aus unzufriedenen Mitgliedern
            if self.plots[i].conspirators.len() < 4 && self.chance(0.25) {
                let cands: Vec<CharId> = self.chars.iter().filter(|c| c.alive() && c.adult(self.day) && c.realm == self.chars[leader as usize].realm && c.id != leader && !self.plots[i].conspirators.contains(&c.id)).map(|c| c.id).take(60).collect();
                if !cands.is_empty() {
                    let j = self.ri(0, cands.len() as i32 - 1) as usize;
                    let _ = self.recruit_conspirator(id, cands[j]);
                }
            }
            let snapshot = self.plots[i].clone();
            let k = self.plot_strength(&snapshot);
            let a = self.plot_defense(&snapshot);
            self.plots[i].progress = (self.plots[i].progress + k / 10.0).min(100.0);
            // Entdeckung: (A/(K+A)) × 8 % × (1 + 0,1 × Mitverschwörer) [× 1,5 bei ehrlichem Mitverschwörer]
            let honest = snapshot.conspirators.iter().any(|c| self.chars[*c as usize].has(Trait::Honest));
            let mut pd = a / (k + a).max(1.0) * 0.08 * (1.0 + 0.1 * snapshot.conspirators.len() as f32) * if honest { 1.5 } else { 1.0 };
            // Kenntnis Verteidiger: Hüter der Komplotte
            if let Some(spy) = self.realms[realm as usize].offices[Office::Spymaster.idx()]
                && self.realms[realm as usize].office_task[Office::Spymaster.idx()] == 0 {
                    pd += self.chars[spy as usize].attr(Attr::Intrigue) as f32 * 0.003;
                }
            if self.chars[self.realms[realm as usize].ruler as usize].has(Trait::Paranoid) {
                pd *= 1.25;
            }
            if self.chance(pd) {
                self.plot_discovered(id);
                continue;
            }
            // Auslösen: KI automatisch bei 100 %, Spieler manuell
            let sponsor_player = snapshot.sponsor.is_some_and(|s| self.is_player(s)) || self.chars[leader as usize].realm.is_some_and(|r| self.is_player(r));
            if self.plots[i].progress >= 100.0 && !sponsor_player {
                self.trigger_plot(id);
            }
        }
        // Neue KI-Intrigen
        self.ai_plots_month();
    }

    fn plot_discovered(&mut self, id: PlotId) {
        let day = self.day;
        let Some(i) = self.plots.iter().position(|p| p.id == id) else { return };
        let p = self.plots[i].clone();
        self.plots[i].discovered = true;
        self.plots[i].done = true;
        let victim_realm = p.realm;
        let spy_i = self.realms[victim_realm as usize].offices[Office::Spymaster.idx()].map(|c| self.chars[c as usize].attr(Attr::Intrigue)).unwrap_or(0);
        let leader_exposed = spy_i >= self.chars[p.leader as usize].attr(Attr::Intrigue) || self.chance(0.5);
        let who = if leader_exposed { p.leader } else { p.conspirators.first().copied().unwrap_or(p.leader) };
        let wn = self.full_name(who);
        let victim = self.realms[victim_realm as usize].ruler;
        let vn = self.full_name(victim);
        let e = Entry::new(ChronicleKind::Plot, "plot_discovered", 40).realm(victim_realm).actor(format!("char_{who}"), wn.clone(), "Verschwörer").var("a", wn).var("b", vn).var("c", p.kind.name()).tag("plot");
        self.chron(e);
        // Folgen: Gefangennahme, Meinungsverlust (−50, Erinnerung 30 Jahre), Kriegsgrund
        if self.chars[who as usize].realm == Some(victim_realm) {
            // Inlandsverschwörer: Haft oder Hinrichtung
            let executed = self.chars[victim as usize].has(Trait::Cruel) && self.chance(0.5);
            if executed {
                self.kill_char(who, DeathCause::Executed);
            } else {
                self.chars[who as usize].captive_of = Some(victim_realm);
                self.chars[who as usize].prestige -= 100.0;
            }
            if let Some(h) = self.chars[who as usize].house {
                self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty - 20.0).max(0.0);
            }
        } else if let Some(s) = p.sponsor {
            self.add_opinion(victim_realm, s, OpKind::Plot, -50.0, Some(day + 30 * DAYS_PER_YEAR));
            self.realms[victim_realm as usize].memory.push(RealmMemory { kind: RealmMemKind::Betrayed, other: s, day });
            if self.chance(0.5) {
                self.chars[who as usize].captive_of = Some(victim_realm);
            }
        }
        self.remember(victim, MemoryKind::Plotted, who, -50.0, 30);
    }

    /// Löst ein Komplott aus: Erfolg nach Formel, Folgen für Beteiligte.
    pub fn trigger_plot(&mut self, id: PlotId) -> bool {
        let Some(i) = self.plots.iter().position(|p| p.id == id && !p.done) else { return false };
        let p = self.plots[i].clone();
        let chance = self.plot_success_chance(&p) / 100.0;
        self.plots[i].done = true;
        let success = self.chance(chance);
        let leader_name = self.full_name(p.leader);
        let victim_realm = p.realm;
        let rn = self.rname(victim_realm);
        let day = self.day;
        if success {
            self.stats.plots_succeeded += 1;
            self.chars[p.leader as usize].deeds.plots += 1;
            self.chars[p.leader as usize].prestige += 20.0;
            if let Some(s) = p.sponsor {
                self.realms[s as usize].stats.plots_run += 1;
            }
            let target_name = match p.target {
                PlotTarget::Char(c) => self.full_name(c),
                PlotTarget::Prov(pv) => self.pname(pv),
                PlotTarget::Realm(r) | PlotTarget::Throne(r) => self.rname(r),
            };
            self.apply_plot_effect(&p);
            let e = Entry::new(ChronicleKind::Plot, "plot_success", 45).realm(victim_realm).actor(format!("char_{}", p.leader), leader_name.clone(), "Anführer").var("a", leader_name).var("b", target_name).var("c", p.kind.name()).tag("plot").secret();
            // Geheime Taten erscheinen erst bei Aufdeckung: wir veröffentlichen die Wirkung, nicht den Urheber
            let public = Entry::new(ChronicleKind::Plot, "event_generic", 38).realm(victim_realm).var("a", rn.clone()).var("p", rn).var("c", format!("Ein Komplott ({}) traf sein Ziel; der Urheber blieb im Dunkeln", p.kind.name()));
            let _ = self.chron(e);
            self.chron(public);
        } else {
            // Scheitern: Ausführende werden mit 50 % gefasst; Anführer mit 50 % × (1 − Intrige/25) enttarnt
            let caught = self.chance(0.5);
            let exposed = caught && self.chance(0.5 * (1.0 - self.chars[p.leader as usize].attr(Attr::Intrigue) as f32 / 25.0).max(0.0) * 2.0);
            let e = Entry::new(ChronicleKind::Plot, "plot_failed", 30).realm(victim_realm).var("a", leader_name.clone()).var("b", rn).var("c", p.kind.name()).tag("plot");
            if exposed {
                self.chron(e);
                if let Some(s) = p.sponsor {
                    self.add_opinion(victim_realm, s, OpKind::Plot, -50.0, Some(day + 30 * DAYS_PER_YEAR));
                }
                if self.chars[p.leader as usize].realm == Some(victim_realm) {
                    self.chars[p.leader as usize].captive_of = Some(victim_realm);
                    if let Some(h) = self.chars[p.leader as usize].house {
                        self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty - 15.0).max(0.0);
                    }
                }
            }
            self.chars[p.leader as usize].stress = (self.chars[p.leader as usize].stress + 10.0).min(100.0);
        }
        success
    }

    fn apply_plot_effect(&mut self, p: &Plot) {
        let day = self.day;
        match (p.kind, p.target) {
            (PlotKind::Murder, PlotTarget::Char(t)) => {
                self.chars[p.leader as usize].secrets.push(Secret { kind: SecretKind::Murder, public: false, known_by: Vec::new() });
                // Verwandte erinnern sich
                let kin: Vec<CharId> = {
                    let c = &self.chars[t as usize];
                    c.children.iter().copied().chain(c.spouse).chain(c.father).chain(c.mother).collect()
                };
                self.kill_char(t, DeathCause::Murder);
                for k in kin {
                    if self.chars[k as usize].alive() {
                        self.remember(k, MemoryKind::FamilyKilled, p.leader, -80.0, 40);
                    }
                }
                if self.chars[t as usize].dynasty == self.chars[p.leader as usize].dynasty && !self.chars[p.leader as usize].has(Trait::Kinslayer) {
                    self.chars[p.leader as usize].traits.push(Trait::Kinslayer);
                }
            }
            (PlotKind::Abduct, PlotTarget::Char(t)) => {
                if let Some(r) = self.chars[p.leader as usize].realm.or(p.sponsor) {
                    self.chars[t as usize].captive_of = Some(r);
                }
            }
            (PlotKind::Slander, PlotTarget::Char(t)) => {
                self.chars[t as usize].prestige -= 150.0;
                if let Some(r) = self.chars[t as usize].realm {
                    if let Some(h) = self.chars[t as usize].house {
                        self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty - 5.0).max(0.0);
                    }
                    if self.realms[r as usize].ruler == t {
                        self.realms[r as usize].legitimacy = (self.realms[r as usize].legitimacy - 10.0).max(0.0);
                    }
                }
            }
            (PlotKind::Fabricate, _) => {
                if let Some(r) = self.chars[p.leader as usize].realm {
                    self.fabricate_claim(r);
                    self.chars[p.leader as usize].claims.push(p.realm);
                }
            }
            (PlotKind::Incite, PlotTarget::Prov(pv)) => {
                let pr = &mut self.provinces[pv as usize];
                pr.unrest = (pr.unrest + 30.0).min(100.0);
                pr.tension[0] = (pr.tension[0] + 25.0).min(100.0);
            }
            (PlotKind::Usurp, _) | (PlotKind::Conspiracy, PlotTarget::Throne(_)) => {
                let old = self.realms[p.realm as usize].ruler;
                if old == p.leader {
                    return;
                }
                // Der alte Herrscher stirbt oder wird abgesetzt
                let fate = if self.chance(0.4) { DeathCause::Murder } else { DeathCause::Deposed };
                let cand = p.candidate.unwrap_or(p.leader);
                if self.chars[cand as usize].realm != Some(p.realm) {
                    self.court_add(p.realm, cand);
                }
                self.chars[cand as usize].claims.retain(|r| *r != p.realm);
                // Kinder des alten Herrschers können überleben; der neue Herrscher erbt kein Amt
                let old_dyn = self.chars[old as usize].dynasty;
                self.kill_char_for_coup(old, fate);
                let new_dyn = self.chars[cand as usize].dynasty != old_dyn;
                self.install_ruler(p.realm, cand, Some(old), fate, new_dyn);
                self.realms[p.realm as usize].legitimacy = (self.realms[p.realm as usize].legitimacy - 20.0).max(10.0);
            }
            (PlotKind::Steal, _) => {
                if let Some(s) = p.sponsor.or(self.chars[p.leader as usize].realm) {
                    let (mut best, mut bp) = (0usize, 0.0f32);
                    for (i, rs) in self.realms[p.realm as usize].research.iter().enumerate() {
                        if rs.points > bp {
                            bp = rs.points;
                            best = i;
                        }
                    }
                    let take = bp * 0.25;
                    self.realms[p.realm as usize].research[best].points -= take;
                    self.realms[s as usize].research[best].points += take;
                }
            }
            (PlotKind::Seduce, PlotTarget::Char(t)) => {
                self.chars[t as usize].secrets.push(Secret { kind: SecretKind::Affair, public: false, known_by: vec![p.leader] });
            }
            (PlotKind::Espionage, _) => {
                if let Some(s) = p.sponsor.or(self.chars[p.leader as usize].realm) {
                    let e = self.realms[s as usize].spy_net.entry(p.realm).or_insert(0.0);
                    *e = (*e + 25.0).min(100.0);
                }
            }
            (PlotKind::Bribery, PlotTarget::Char(t)) => {
                if let Some(h) = self.chars[t as usize].house {
                    let own = self.chars[p.leader as usize].realm == self.chars[t as usize].realm;
                    self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty + if own { 15.0 } else { -15.0 }).clamp(0.0, 100.0);
                    self.houses[h as usize].wealth += 200.0;
                }
                self.chars[t as usize].memories.push(Memory { kind: MemoryKind::Gifted, subject: p.leader, value: 30.0, expires: day + 10 * DAYS_PER_YEAR });
            }
            (PlotKind::Treason, PlotTarget::Prov(pv)) => {
                // Ein Verräter öffnet die Tore: die Provinz geht an den Auftraggeber, wenn Krieg herrscht
                if let Some(s) = p.sponsor {
                    if self.at_war(s, p.realm) {
                        self.occupy_province(pv, s);
                    } else {
                        self.provinces[pv as usize].unrest = (self.provinces[pv as usize].unrest + 20.0).min(100.0);
                    }
                }
            }
            (PlotKind::SecretAlliance, PlotTarget::Realm(t)) => {
                if let Some(s) = p.sponsor.or(self.chars[p.leader as usize].realm)
                    && !self.has_treaty(TreatyKind::SecretAlliance, s, t) {
                        self.add_treaty(TreatyKind::SecretAlliance, s, t, None, 0);
                        self.add_opinion(s, t, OpKind::Helped, 25.0, None);
                        self.add_opinion(t, s, OpKind::Helped, 25.0, None);
                    }
            }
            (PlotKind::Blackmail, PlotTarget::Char(t)) => {
                let gold = 150.0 + self.chars[t as usize].secrets.len() as f32 * 150.0;
                if let Some(s) = p.sponsor.or(self.chars[p.leader as usize].realm) {
                    self.realms[s as usize].gold += gold;
                }
                if let Some(h) = self.chars[t as usize].house {
                    self.houses[h as usize].wealth = (self.houses[h as usize].wealth - gold).max(0.0);
                }
                if let Some(sec) = self.chars[t as usize].secrets.first_mut() {
                    sec.known_by.push(p.leader);
                }
            }
            (PlotKind::Conspiracy, _) => {
                for h in self.houses.iter_mut().filter(|h| h.alive && h.realm == p.realm) {
                    h.loyalty = (h.loyalty - 15.0).max(0.0);
                }
            }
            _ => {}
        }
    }

    fn kill_char_for_coup(&mut self, old: CharId, fate: DeathCause) {
        // Der Herrscher wird getötet oder abgesetzt – in beiden Fällen scheidet er aus dem Spiel
        self.chars[old as usize].epithet = self.compute_epithet(old);
        // kill_char würde die Nachfolge auslösen; wir setzen die Figur direkt auf tot und überlassen install_ruler die Nachfolge
        let realm = self.chars[old as usize].realm;
        self.chars[old as usize].died = Some((self.day, fate));
        self.stats.deaths += 1;
        if let Some(s) = self.chars[old as usize].spouse {
            self.chars[s as usize].spouse = None;
        }
        if let Some(r) = realm {
            self.court_remove(r, old);
        }
        if let Some((rr, o)) = self.chars[old as usize].office {
            self.realms[rr as usize].offices[o.idx()] = None;
        }
        let name = self.full_name(old);
        let tpl = if fate == DeathCause::Murder { "death_murder" } else { "deposed" };
        let mut e = Entry::new(ChronicleKind::Death, tpl, 55).actor(format!("char_{old}"), name.clone(), "Herrscher").var("a", name.clone()).var("b", name).var("n", "0");
        if let Some(r) = realm {
            e = e.realm(r);
        }
        self.chron(e);
    }
}
