//! Diplomatie: Meinungsmodell, Verträge, Kriegsgründe, Kriegspunkte, Friedensschlüsse, Bedrohung,
//! Koalitionen, Boykotte und Unterstützung von Aufständen (GDD Kapitel 7).

use crate::catalog::Trait;
use crate::chronicle::{ChronicleKind, Entry};
use crate::model::*;
use crate::types::*;
use crate::world::{StrategyEvent, StrategyWorld};
use serde::{Deserialize, Serialize};

/// Friedensbedingungen.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PeaceTerms {
    /// Provinzen, die der Unterlegene abtritt.
    pub cede: Vec<ProvId>,
    /// Tribut (10 Jahre, 10 % des Einkommens).
    pub tribute: bool,
    /// Der Unterlegene wird Vasall des Siegers.
    pub vassalize: bool,
    /// Der Unterlegene wird unabhängig (Unabhängigkeitskrieg).
    pub independence: bool,
}

/// Diplomatische Aktionen (GDD 7.2).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiplomaticAction {
    Envoy,
    Gift(f32),
    Propose(TreatyKind),
    DemandTribute,
    OfferVassalage,
    Guarantee,
    DeclareWar { cb: CasusBelli, goal: WarGoal },
    OfferPeace { war: WarId, terms: PeaceTerms },
    Ultimatum { goal: WarGoal },
    Embargo(bool),
    SupportRebels(ProvId),
    CallToArms(WarId),
    JoinCoalition(RealmId),
    BreakTreaty(TreatyKind),
    ProposeMarriage { own: CharId, other: CharId, dowry: f32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiplomacyOutcome {
    pub accepted: bool,
    pub message: String,
}

impl DiplomacyOutcome {
    fn yes(m: impl Into<String>) -> DiplomacyOutcome {
        DiplomacyOutcome { accepted: true, message: m.into() }
    }
    fn no(m: impl Into<String>) -> DiplomacyOutcome {
        DiplomacyOutcome { accepted: false, message: m.into() }
    }
}

pub fn is_antagonist(k: RealmKind) -> bool {
    matches!(k, RealmKind::Hollowed | RealmKind::Legion)
}

impl StrategyWorld {
    // ------------------------------------------------------------------------------------
    // Verträge
    // ------------------------------------------------------------------------------------
    pub fn has_treaty(&self, kind: TreatyKind, a: RealmId, b: RealmId) -> bool {
        let day = self.day;
        self.treaties.iter().any(|t| t.kind == kind && t.end.is_none_or(|e| e > day) && ((t.a == a && t.b == b) || (t.a == b && t.b == a)))
    }

    pub fn add_treaty(&mut self, kind: TreatyKind, a: RealmId, b: RealmId, years: Option<u32>, data: i64) -> u32 {
        let id = self.next_treaty;
        self.next_treaty += 1;
        let end = years.map(|y| self.day + y * DAYS_PER_YEAR);
        self.treaties.push(Treaty { id, kind, a, b, start: self.day, end, data, secret: kind == TreatyKind::SecretAlliance });
        id
    }

    pub fn remove_treaties(&mut self, kind: TreatyKind, a: RealmId, b: RealmId) {
        self.treaties.retain(|t| !(t.kind == kind && ((t.a == a && t.b == b) || (t.a == b && t.b == a))));
    }

    pub fn allies_of(&self, r: RealmId) -> Vec<RealmId> {
        let day = self.day;
        let mut v: Vec<RealmId> = self
            .treaties
            .iter()
            .filter(|t| matches!(t.kind, TreatyKind::Alliance | TreatyKind::SecretAlliance) && t.end.is_none_or(|e| e > day))
            .filter_map(|t| if t.a == r { Some(t.b) } else if t.b == r { Some(t.a) } else { None })
            .filter(|x| self.realms[*x as usize].alive)
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    }

    // ------------------------------------------------------------------------------------
    // Meinung
    // ------------------------------------------------------------------------------------
    pub fn add_opinion(&mut self, from: RealmId, to: RealmId, kind: OpKind, value: f32, expires: Option<u32>) {
        if from == to {
            return;
        }
        let day = self.day;
        let v = self.opinion_mods.entry((from, to)).or_default();
        // Gleichartige Modifikatoren ersetzen/stapeln (Gesandter stapelt bis +30)
        if let Some(m) = v.iter_mut().find(|m| m.kind == kind) {
            if matches!(kind, OpKind::Envoy | OpKind::Gift) {
                m.value = (m.value + value).min(40.0);
            } else {
                m.value = if value.abs() > m.value.abs() { value } else { m.value };
            }
            m.start = day;
            m.expires = expires;
            return;
        }
        v.push(OpinionMod { kind, value, start: day, expires });
    }

    fn mods_sum(&self, from: RealmId, to: RealmId) -> f32 {
        let day = self.day;
        let Some(v) = self.opinion_mods.get(&(from, to)) else { return 0.0 };
        v.iter()
            .map(|m| match m.expires {
                None => m.value,
                Some(e) => {
                    if e <= day {
                        0.0
                    } else {
                        let total = (e - m.start).max(1) as f32;
                        m.value * ((e - day) as f32 / total)
                    }
                }
            })
            .sum()
    }

    /// Meinung des Herrschers von `a` über `b` (−100…+100).
    pub fn opinion(&self, a: RealmId, b: RealmId) -> f32 {
        if a == b {
            return 100.0;
        }
        let (ra, rb) = (&self.realms[a as usize], &self.realms[b as usize]);
        if is_antagonist(ra.kind) || is_antagonist(rb.kind) {
            return -100.0;
        }
        let (ca, cb) = (&self.chars[ra.ruler as usize], &self.chars[rb.ruler as usize]);
        let mut v = 0.0;
        // Grundsympathie
        let mut sym: f32 = 0.0;
        for t in &ca.traits {
            if !t.is_personality() {
                continue;
            }
            if cb.traits.contains(t) {
                sym += 10.0;
            }
            if let Some(o) = t.opposite()
                && cb.traits.contains(&o) {
                    sym -= 10.0;
                }
        }
        v += sym.clamp(-30.0, 30.0);
        v += cb.attr(Attr::Diplomacy) as f32 - 7.0;
        for t in &cb.traits {
            let g = t.opinion() as f32;
            v += g * 0.5;
        }
        v += match ra.religion.relation(rb.religion) {
            1 => 15.0,
            0 => 5.0,
            -1 => -25.0,
            _ => 0.0,
        };
        if ra.culture == rb.culture {
            v += 10.0;
        } else if ra.culture.related(rb.culture) {
            v += 5.0;
        }
        if self.has_treaty(TreatyKind::Marriage, a, b) {
            v += 25.0;
        }
        if ra.dynasty == rb.dynasty {
            v += 30.0;
        }
        v += self.mods_sum(a, b);
        // Gemeinsamer Feind
        let common = self.wars.iter().any(|w| w.active && ((w.attackers.contains(&a) && w.attackers.contains(&b)) || (w.defenders.contains(&a) && w.defenders.contains(&b))));
        if common {
            v += 20.0;
        }
        // Grenzspannung
        if self.realm_distance(a, b) == 1 && rb.strength > ra.strength * 1.2 {
            v -= 10.0;
        }
        // Ansprüche von b auf Titel von a
        let claims = self.provinces.iter().filter(|p| p.owner == Some(a) && p.original_owner == Some(b)).count() as f32;
        v -= (claims * 20.0).min(40.0);
        // Bedrohung
        v -= self.threat_view(a, b) * 0.5;
        if rb.peace_reputation_until > self.day {
            v += 10.0;
        }
        if cb.prestige >= 2000.0 {
            v += 10.0;
        }
        v -= rb.tyranny.min(50.0);
        if rb.hollow_value >= 40.0 {
            v -= if ca.has(Trait::Zealous) || ra.religion == Religion::EternalFlame { 60.0 } else { 40.0 };
        }
        // Zölle
        v -= (rb.tariff_import / 5.0).min(10.0) * 0.0 + ra.tariff_import / 5.0 * 0.0;
        // Boykott
        if ra.embargo.contains(&b) {
            v -= 10.0;
        }
        v.clamp(-100.0, 100.0)
    }

    // ------------------------------------------------------------------------------------
    // Stärke und Bedrohung
    // ------------------------------------------------------------------------------------
    pub fn compute_strength(&self, r: RealmId) -> f32 {
        let army = self.realm_army_power(r);
        let mut pop = 0.0;
        let mut provs = 0.0;
        for p in self.provinces_of(r) {
            let pr = &self.provinces[*p as usize];
            pop += pr.population();
            provs += 1.0;
        }
        let rl = &self.realms[r as usize];
        let gold = rl.gold.clamp(0.0, 20000.0);
        let mut s = army + pop * 0.08 + provs * 60.0 + gold * 0.03;
        // Vasallen tragen mit bei
        for v in self.realms.iter().filter(|x| x.alive && x.vassal_of == Some(r)) {
            s += v.strength * 0.6;
        }
        s
    }

    /// Bedrohung, die `by` aus Sicht von `viewer` darstellt (0–100).
    pub fn threat_view(&self, viewer: RealmId, by: RealmId) -> f32 {
        let base = self.realms[by as usize].threat;
        let prox = match self.realm_distance(viewer, by) {
            0 | 1 => 1.0,
            2 => 0.6,
            _ => 0.3,
        };
        let mut t = base * prox;
        if self.realms[by as usize].peace_reputation_until > self.day {
            t *= 0.85;
        }
        t.min(100.0)
    }

    pub(crate) fn threat_month(&mut self) {
        let living: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind)).map(|r| r.id).collect();
        let n = living.len().max(1) as f32;
        let mut strengths = Vec::new();
        for &r in &living {
            let s = self.compute_strength(r);
            self.realms[r as usize].strength = s;
            strengths.push(s);
        }
        for r in self.realms.iter_mut().filter(|r| !r.alive || is_antagonist(r.kind)) {
            r.strength = 0.0;
        }
        let avg = (strengths.iter().sum::<f32>() / n).max(1.0);
        let day = self.day;
        for &r in &living {
            let rl = &self.realms[r as usize];
            let conquests = rl.recent_conquests.iter().filter(|d| day.saturating_sub(**d) < 30 * DAYS_PER_YEAR).count() as f32;
            let mut t = conquests * 5.0 + ((rl.strength / avg) - 1.0).max(0.0) * 20.0 + rl.treaty_breaks as f32 * 10.0 + rl.void_uses.len() as f32 * 15.0;
            if rl.peace_reputation_until > day {
                t *= 0.85;
            }
            self.realms[r as usize].threat = t.clamp(0.0, 100.0);
        }
        // Großmachtstatus: 30 % aller Provinzen oder doppelte Stärke des Zweitstärksten
        let total_provs = self.provinces.len() as f32;
        let mut sorted: Vec<(f32, RealmId)> = living.iter().map(|r| (self.realms[*r as usize].strength, *r)).collect();
        sorted.sort_by(|a, b| b.0.total_cmp(&a.0));
        let mut gp: Option<RealmId> = None;
        if let Some(&(s1, r1)) = sorted.first() {
            let s2 = sorted.get(1).map(|x| x.0).unwrap_or(0.0);
            let direct = self.provinces_of(r1).len() as f32;
            let share = direct + self.realms.iter().filter(|x| x.alive && x.vassal_of == Some(r1)).map(|v| self.provinces_of(v.id).len() as f32).sum::<f32>();
            if direct >= total_provs * 0.30 || (s1 >= s2 * 2.0 && s2 > 0.0 && share >= total_provs * 0.40) {
                gp = Some(r1);
            }
        }
        for r in self.realms.iter_mut() {
            r.great_power = false;
        }
        if let Some(g) = gp {
            self.realms[g as usize].great_power = true;
        }
        if self.endgame.great_power != gp {
            self.endgame.great_power = gp;
            if let Some(g) = gp {
                self.endgame.great_power_since = Some(self.day);
                let e = Entry::new(ChronicleKind::Endgame, "great_power", 65).realm(g).var("a", self.rname(g));
                self.chron(e);
                self.form_great_power_coalition(g);
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Diplomatische Aktionen
    // ------------------------------------------------------------------------------------
    /// Führt eine diplomatische Aktion aus (Spieler oder KI). Prüft Voraussetzungen und Annahme.
    pub fn diplomacy(&mut self, from: RealmId, to: RealmId, action: DiplomaticAction) -> DiplomacyOutcome {
        if from == to || !self.realms[to as usize].alive || !self.realms[from as usize].alive {
            return DiplomacyOutcome::no("Ungültiges Ziel");
        }
        if is_antagonist(self.realms[to as usize].kind) {
            return DiplomacyOutcome::no("Die Hohlen und die Aschenlegion sind nicht verhandelbar");
        }
        let op = self.opinion(to, from);
        let dip_bonus = self.diplomatic_bonus(from);
        let day = self.day;
        match action {
            DiplomaticAction::Envoy => {
                if self.realms[from as usize].gold < 25.0 {
                    return DiplomacyOutcome::no("25 Gold nötig");
                }
                self.realms[from as usize].gold -= 25.0;
                self.add_opinion(to, from, OpKind::Envoy, 10.0, Some(day + 10 * DAYS_PER_YEAR));
                DiplomacyOutcome::yes("Der Gesandte wurde freundlich empfangen")
            }
            DiplomaticAction::Gift(gold) => {
                if gold <= 0.0 || self.realms[from as usize].gold < gold {
                    return DiplomacyOutcome::no("Nicht genug Gold");
                }
                self.realms[from as usize].gold -= gold;
                self.realms[to as usize].gold += gold;
                let monthly = self.realms[to as usize].income.max(50.0);
                let mult = if self.chars[self.realms[from as usize].ruler as usize].has(Trait::Generous) { 1.25 } else { 1.0 };
                self.add_opinion(to, from, OpKind::Gift, (gold / monthly * 10.0 * mult).min(40.0), Some(day + 5 * DAYS_PER_YEAR));
                DiplomacyOutcome::yes("Das Geschenk wurde angenommen")
            }
            DiplomaticAction::Propose(kind) => self.propose_treaty(from, to, kind, op, dip_bonus),
            DiplomaticAction::DemandTribute => {
                let (sa, sb) = (self.realms[from as usize].strength.max(1.0), self.realms[to as usize].strength.max(1.0));
                if sa / sb < 2.0 {
                    return DiplomacyOutcome::no("Stärkeverhältnis unter 2:1");
                }
                if sa / sb * 25.0 + op * 0.3 > 60.0 {
                    self.add_treaty(TreatyKind::Tribute, from, to, Some(10), 0);
                    self.add_opinion(to, from, OpKind::Tribute, -20.0, Some(day + 10 * DAYS_PER_YEAR));
                    let e = Entry::new(ChronicleKind::Diplomacy, "tribute", 28).realm(from).var("a", self.rname(from)).var("b", self.rname(to));
                    self.chron(e);
                    DiplomacyOutcome::yes("Tribut zugesagt")
                } else {
                    self.add_opinion(to, from, OpKind::Insult, -10.0, Some(day + 5 * DAYS_PER_YEAR));
                    DiplomacyOutcome::no("Die Forderung wurde zurückgewiesen")
                }
            }
            DiplomaticAction::OfferVassalage => {
                if self.realms[from as usize].prestige < 200.0 {
                    return DiplomacyOutcome::no("200 Prestige nötig");
                }
                let (sa, sb) = (self.realms[from as usize].strength.max(1.0), self.realms[to as usize].strength.max(1.0));
                if sb > sa || self.realms[to as usize].vassal_of.is_some() || (op < 50.0 && sa / sb < 4.0) {
                    return DiplomacyOutcome::no("Bedingungen nicht erfüllt (Ziel kleiner, Meinung ≥ 50 oder Stärke ≥ 4:1)");
                }
                let accept = op >= 50.0 || sa / sb >= 4.0 && self.chance(0.6);
                if accept {
                    self.realms[from as usize].prestige -= 200.0;
                    self.make_vassal(to, from);
                    DiplomacyOutcome::yes("Der Herrscher beugt das Knie")
                } else {
                    DiplomacyOutcome::no("Abgelehnt")
                }
            }
            DiplomaticAction::Guarantee => {
                if self.realms[from as usize].prestige < 25.0 {
                    return DiplomacyOutcome::no("25 Prestige nötig");
                }
                self.realms[from as usize].prestige -= 25.0;
                self.add_treaty(TreatyKind::Guarantee, from, to, Some(20), 0);
                self.add_opinion(to, from, OpKind::Helped, 10.0, Some(day + 10 * DAYS_PER_YEAR));
                DiplomacyOutcome::yes("Garantie ausgesprochen")
            }
            DiplomaticAction::DeclareWar { cb, goal } => match self.declare_war(from, to, cb, goal) {
                Ok(_) => DiplomacyOutcome::yes("Krieg erklärt"),
                Err(e) => DiplomacyOutcome::no(e),
            },
            DiplomaticAction::Ultimatum { goal } => {
                // Das Ziel gibt nach, wenn es deutlich schwächer ist
                let (sa, sb) = (self.realms[from as usize].strength.max(1.0), self.realms[to as usize].strength.max(1.0));
                let yield_p = ((sa / sb - 1.0) * 0.35 + op * 0.003).clamp(0.0, 0.9);
                if self.chance(yield_p) {
                    match goal {
                        WarGoal::Tribute => {
                            self.add_treaty(TreatyKind::Tribute, from, to, Some(10), 0);
                            DiplomacyOutcome::yes("Das Ziel beugt sich dem Ultimatum")
                        }
                        WarGoal::Subjugate => {
                            self.make_vassal(to, from);
                            DiplomacyOutcome::yes("Das Ziel beugt sich dem Ultimatum")
                        }
                        WarGoal::TakeProvince(p) => {
                            if self.provinces[p as usize].owner == Some(to) {
                                self.transfer_province(p, from, "Ultimatum");
                            }
                            DiplomacyOutcome::yes("Das Ziel tritt die Provinz ab")
                        }
                        _ => DiplomacyOutcome::yes("Das Ziel gibt nach"),
                    }
                } else {
                    DiplomacyOutcome::no("Das Ziel lehnt das Ultimatum ab – Kriegsdrohung")
                }
            }
            DiplomaticAction::OfferPeace { war, terms } => self.offer_peace(from, war, terms),
            DiplomaticAction::Embargo(on) => {
                let rl = &mut self.realms[from as usize];
                if on {
                    if !rl.embargo.contains(&to) {
                        rl.embargo.push(to);
                        self.add_opinion(to, from, OpKind::Boycott, -15.0, Some(day + 8 * DAYS_PER_YEAR));
                        let e = Entry::new(ChronicleKind::Economy, "boycott", 30).realm(from).var("a", self.rname(from)).var("b", self.rname(to));
                        self.chron(e);
                    }
                } else {
                    rl.embargo.retain(|x| *x != to);
                }
                DiplomacyOutcome::yes("Handelsboykott angepasst")
            }
            DiplomaticAction::SupportRebels(p) => {
                if self.provinces[p as usize].owner != Some(to) {
                    return DiplomacyOutcome::no("Provinz gehört nicht dem Ziel");
                }
                if self.realms[from as usize].gold < 150.0 {
                    return DiplomacyOutcome::no("150 Gold nötig");
                }
                self.realms[from as usize].gold -= 150.0;
                self.provinces[p as usize].unrest = (self.provinces[p as usize].unrest + 12.0).min(100.0);
                let discovered = self.chance(0.25);
                if discovered {
                    self.add_opinion(to, from, OpKind::Plot, -40.0, Some(day + 15 * DAYS_PER_YEAR));
                }
                DiplomacyOutcome::yes(if discovered { "Die Unterstützung wurde entdeckt" } else { "Die Unzufriedenen werden unterstützt" })
            }
            DiplomaticAction::CallToArms(w) => {
                let ok = self.wars.iter().any(|x| x.id == w && x.active);
                if !ok {
                    return DiplomacyOutcome::no("Kein laufender Krieg");
                }
                let allied = self.has_treaty(TreatyKind::Alliance, from, to) || self.has_treaty(TreatyKind::DefensivePact, from, to);
                if !allied {
                    return DiplomacyOutcome::no("Kein Bündnis");
                }
                let side_att = self.wars.iter().find(|x| x.id == w).map(|x| x.attackers.contains(&from)).unwrap_or(true);
                self.join_war(to, w, side_att, from)
            }
            DiplomaticAction::JoinCoalition(target) => {
                let ok = self.coalitions.iter().position(|c| c.active && c.target == target);
                match ok {
                    Some(i) => {
                        if !self.coalitions[i].members.contains(&from) {
                            self.coalitions[i].members.push(from);
                        }
                        DiplomacyOutcome::yes("Der Koalition beigetreten")
                    }
                    None => DiplomacyOutcome::no("Keine Koalition gegen dieses Reich"),
                }
            }
            DiplomaticAction::BreakTreaty(kind) => {
                if self.has_treaty(kind, from, to) {
                    self.remove_treaties(kind, from, to);
                    self.realms[from as usize].treaty_breaks += 1;
                    self.add_opinion(to, from, OpKind::BrokenTreaty, -50.0, Some(day + 50 * DAYS_PER_YEAR));
                    self.realms[from as usize].prestige -= 50.0;
                    let e = Entry::new(ChronicleKind::Diplomacy, "treaty_broken", 30).realm(from).var("a", self.rname(from)).var("b", self.rname(to));
                    self.chron(e);
                    DiplomacyOutcome::yes("Vertrag gebrochen")
                } else {
                    DiplomacyOutcome::no("Kein solcher Vertrag")
                }
            }
            DiplomaticAction::ProposeMarriage { own, other, dowry } => self.propose_marriage(from, to, own, other, dowry, op),
        }
    }

    fn diplomatic_bonus(&self, r: RealmId) -> f32 {
        let rl = &self.realms[r as usize];
        let mut b = self.chars[rl.ruler as usize].attr(Attr::Diplomacy) as f32;
        if let Some(d) = rl.offices[Office::Diplomat.idx()] {
            b += self.chars[d as usize].attr(Attr::Diplomacy) as f32 * 0.5;
        }
        b
    }

    fn propose_treaty(&mut self, from: RealmId, to: RealmId, kind: TreatyKind, op: f32, dip: f32) -> DiplomacyOutcome {
        let day = self.day;
        if self.at_war(from, to) {
            return DiplomacyOutcome::no("Die Reiche befinden sich im Krieg");
        }
        if self.has_treaty(kind, from, to) {
            return DiplomacyOutcome::no("Vertrag besteht bereits");
        }
        let (min_op, cost_prestige, base) = match kind {
            TreatyKind::Trade => (0.0, 0.0, 0.55),
            TreatyKind::NonAggression => (10.0, 50.0, 0.45),
            TreatyKind::MilitaryAccess => (20.0, 0.0, 0.4),
            TreatyKind::DefensivePact => (40.0, 0.0, 0.4),
            TreatyKind::Alliance => (60.0, 100.0, 0.35),
            TreatyKind::AgainstHollow => (-20.0, 0.0, 0.6),
            _ => return DiplomacyOutcome::no("Dieser Vertrag wird nicht direkt vorgeschlagen"),
        };
        let family = self.has_treaty(TreatyKind::Marriage, from, to);
        if op < min_op && !(kind == TreatyKind::Alliance && family) {
            return DiplomacyOutcome::no(format!("Meinung zu niedrig (benötigt {min_op:.0})"));
        }
        if self.realms[from as usize].prestige < cost_prestige {
            return DiplomacyOutcome::no(format!("{cost_prestige:.0} Prestige nötig"));
        }
        if kind == TreatyKind::AgainstHollow {
            let borders = |r: RealmId| self.provinces.iter().any(|p| p.owner == Some(r) && p.neighbors.iter().any(|l| self.provinces[l.to as usize].biome == Biome::Hollowland || self.provinces[l.to as usize].owner.is_some_and(|o| self.realms[o as usize].kind == RealmKind::Hollowed)));
            if !borders(from) || !borders(to) {
                return DiplomacyOutcome::no("Beide müssen an Hohlland grenzen");
            }
        }
        let mut p = base + op * 0.005 + dip * 0.01;
        let diplomat = self.realms[from as usize].offices[Office::Diplomat.idx()];
        if let Some(d) = diplomat {
            p += self.chars[d as usize].attr(Attr::Diplomacy) as f32 * 0.01;
            if self.realms[from as usize].office_task[Office::Diplomat.idx()] == 1 && kind == TreatyKind::Alliance {
                p += self.chars[d as usize].attr(Attr::Diplomacy) as f32 * 0.02;
            }
        }
        if self.chars[self.realms[to as usize].ruler as usize].has(Trait::Honest) {
            p += 0.05;
        }
        if self.realms[to as usize].personality == Personality::Conqueror && matches!(kind, TreatyKind::NonAggression | TreatyKind::DefensivePact) {
            p -= 0.15;
        }
        if self.chance(p.clamp(0.02, 0.97)) {
            self.realms[from as usize].prestige -= cost_prestige;
            let years = match kind {
                TreatyKind::NonAggression => Some(10),
                TreatyKind::Trade => Some(20),
                TreatyKind::MilitaryAccess => Some(10),
                _ => None,
            };
            self.add_treaty(kind, from, to, years, 0);
            if kind == TreatyKind::Trade {
                self.add_opinion(to, from, OpKind::TradePact, 20.0, None);
                self.add_opinion(from, to, OpKind::TradePact, 20.0, None);
            }
            if matches!(kind, TreatyKind::Alliance | TreatyKind::AgainstHollow) {
                if let Some(c) = Some(self.realms[from as usize].ruler) {
                    self.chars[c as usize].deeds.alliances += 1;
                }
                let e = Entry::new(ChronicleKind::Diplomacy, "alliance", 40).realm(from).var("a", self.rname(from)).var("b", self.rname(to));
                self.chron(e);
                if kind == TreatyKind::AgainstHollow {
                    self.add_opinion(to, from, OpKind::Helped, 30.0, Some(day + 20 * DAYS_PER_YEAR));
                    self.add_opinion(from, to, OpKind::Helped, 30.0, Some(day + 20 * DAYS_PER_YEAR));
                }
            }
            DiplomacyOutcome::yes(format!("{} geschlossen", kind.name()))
        } else {
            DiplomacyOutcome::no("Abgelehnt")
        }
    }

    /// Annahme eines Heiratsangebots nach GDD 5.7.
    fn propose_marriage(&mut self, from: RealmId, to: RealmId, own: CharId, other: CharId, dowry: f32, op: f32) -> DiplomacyOutcome {
        let (ca, cb) = (&self.chars[own as usize], &self.chars[other as usize]);
        if !ca.alive() || !cb.alive() || ca.spouse.is_some() || cb.spouse.is_some() || ca.gender == cb.gender {
            return DiplomacyOutcome::no("Ungeeignetes Paar");
        }
        if ca.realm != Some(from) || cb.realm != Some(to) {
            return DiplomacyOutcome::no("Figuren gehören nicht zu den Höfen");
        }
        if self.close_kin(own, other) {
            return DiplomacyOutcome::no("Zu nah verwandt");
        }
        if self.realms[from as usize].gold < dowry {
            return DiplomacyOutcome::no("Mitgift fehlt");
        }
        let rank_a = self.realms[from as usize].rank as i32;
        let rank_b = self.realms[to as usize].rank as i32;
        let attr_sum: i32 = ATTRS.iter().map(|a| ca.attr(*a)).sum();
        let income = (self.realms[to as usize].income * 12.0).max(100.0);
        let rel = match ca.religion.relation(cb.religion) {
            1 => 20.0,
            -1 => -30.0,
            _ => 0.0,
        };
        let mut p = 30.0 + (rank_a - rank_b) as f32 * 15.0 + op * 0.5 + (attr_sum as f32 - 40.0) * 0.5 + rel + (dowry / (income * 0.1) * 5.0).min(25.0);
        if ca.hollowing >= 40.0 {
            p -= 50.0;
        }
        if cb.has(Trait::Courtier) {
            p += 10.0;
        }
        if self.chance((p / 100.0).clamp(0.02, 0.97)) {
            self.realms[from as usize].gold -= dowry;
            self.realms[to as usize].gold += dowry;
            self.marry(own, other, true);
            self.add_treaty(TreatyKind::Marriage, from, to, None, own as i64);
            self.add_opinion(to, from, OpKind::Marriage, 25.0, None);
            self.add_opinion(from, to, OpKind::Marriage, 25.0, None);
            DiplomacyOutcome::yes("Die Heirat wurde vereinbart")
        } else {
            DiplomacyOutcome::no("Das Angebot wurde abgelehnt")
        }
    }

    pub fn make_vassal(&mut self, vassal: RealmId, lord: RealmId) {
        self.realms[vassal as usize].vassal_of = Some(lord);
        self.realms[vassal as usize].rank = Rank::Duke.min(self.realms[vassal as usize].rank);
        // Vorherige Lehnsherrschaft des Herrn auflösen, falls Zyklus
        if self.realms[lord as usize].vassal_of == Some(vassal) {
            self.realms[lord as usize].vassal_of = None;
        }
        let e = Entry::new(ChronicleKind::Diplomacy, "vassalized", 36).realm(lord).var("a", self.rname(lord)).var("b", self.rname(vassal));
        self.chron(e);
    }

    // ------------------------------------------------------------------------------------
    // Krieg
    // ------------------------------------------------------------------------------------
    /// Kriegsgrund für `a` gegen `b`, wenn einer besteht.
    pub fn find_cb(&self, a: RealmId, b: RealmId) -> Option<CasusBelli> {
        let (ra, rb) = (&self.realms[a as usize], &self.realms[b as usize]);
        if is_antagonist(rb.kind) {
            return Some(CasusBelli::LiberateHollowland);
        }
        if ra.vassal_of == Some(b) {
            return Some(CasusBelli::Independence);
        }
        // Anspruch: Provinz von b, die früher a gehörte
        if self.provinces.iter().any(|p| p.owner == Some(b) && p.original_owner == Some(a)) {
            return Some(CasusBelli::Claim);
        }
        if self.chars[ra.ruler as usize].claims.contains(&b) {
            return Some(CasusBelli::Throne);
        }
        // Vergeltung
        let day = self.day;
        if ra.memory.iter().any(|m| m.other == b && matches!(m.kind, RealmMemKind::FamilyKilled | RealmMemKind::Betrayed) && day.saturating_sub(m.day) < 40 * DAYS_PER_YEAR) {
            return Some(CasusBelli::Revenge);
        }
        // Heiliger Krieg: Eiferer gegen verfeindete oder häretische Religion
        if ra.religion != rb.religion && ra.religion.relation(rb.religion) <= 0 && (ra.religion == Religion::EternalFlame || self.chars[ra.ruler as usize].has(Trait::Zealous)) && ra.piety >= 100.0 {
            return Some(CasusBelli::HolyWar);
        }
        // Grenzkonflikt: Grenzprovinz mit eigener Kultur
        if self.provinces.iter().any(|p| p.owner == Some(b) && p.culture == ra.culture && p.neighbors.iter().any(|l| self.provinces[l.to as usize].owner == Some(a))) {
            return Some(CasusBelli::Border);
        }
        // Koalition
        if self.coalitions.iter().any(|c| c.active && c.target == b && c.members.contains(&a)) {
            return Some(CasusBelli::Coalition);
        }
        None
    }

    pub fn declare_war(&mut self, a: RealmId, b: RealmId, cb: CasusBelli, goal: WarGoal) -> Result<WarId, String> {
        let day = self.day;
        if a == b || !self.realms[a as usize].alive || !self.realms[b as usize].alive {
            return Err("Ungültige Kriegsparteien".into());
        }
        if self.at_war(a, b) && !is_antagonist(self.realms[b as usize].kind) {
            return Err("Bereits im Krieg".into());
        }
        if is_antagonist(self.realms[b as usize].kind) || is_antagonist(self.realms[a as usize].kind) {
            return Err("Gegen die Hohlen besteht ständiger Kriegszustand".into());
        }
        if self.is_player(a) {
            if self.options.rules.challenge == Some(crate::world::Challenge::NoBlood) {
                return Err("Herausforderung „Kein Blut“: keine Kriegserklärung".into());
            }
            if self.realms[a as usize].regency.is_some() {
                return Err("Unter Regentschaft ist Krieg erklären gesperrt".into());
            }
        }
        if self.has_treaty(TreatyKind::Truce, a, b) && cb != CasusBelli::Independence {
            return Err("Waffenstillstand besteht".into());
        }
        // Vasall gegen Lehnsherrn nur mit Unabhängigkeits-/Thronkriegsgrund
        if self.realms[a as usize].vassal_of == Some(b) && !matches!(cb, CasusBelli::Independence | CasusBelli::Throne) {
            return Err("Vasallen dürfen den Lehnsherrn nicht angreifen".into());
        }
        if self.realms[b as usize].vassal_of == Some(a) && matches!(cb, CasusBelli::Claim) {
            // Lehnsherr gegen Vasall: Zwangsunterwerfung erlaubt
        }
        let mut broke_treaty = false;
        if self.has_treaty(TreatyKind::NonAggression, a, b) {
            broke_treaty = true;
            self.remove_treaties(TreatyKind::NonAggression, a, b);
        }
        if self.has_treaty(TreatyKind::Alliance, a, b) {
            broke_treaty = true;
            self.remove_treaties(TreatyKind::Alliance, a, b);
        }
        let unjustified = cb == CasusBelli::Unjustified;
        if unjustified {
            if self.realms[a as usize].prestige < 300.0 && !self.realms_ai(a) {
                return Err("300 Prestige für einen Krieg ohne Grund nötig".into());
            }
            self.realms[a as usize].prestige -= 300.0;
            self.realms[a as usize].tyranny += 30.0;
        }
        if broke_treaty {
            self.realms[a as usize].treaty_breaks += 1;
            self.add_opinion_all(b, a, OpKind::BrokenTreaty, -50.0, Some(day + 50 * DAYS_PER_YEAR));
            self.realms[a as usize].prestige -= 50.0;
        }
        // Stress des Herrschers
        self.ruler_act(a, crate::catalog::StressAct::DeclareWar);
        if self.last_war_with(a, b).is_some_and(|d| day.saturating_sub(d) < 2 * DAYS_PER_YEAR) {
            self.ruler_act(a, crate::catalog::StressAct::WarSoonAfterPeace);
        }
        let id = self.next_war;
        self.next_war += 1;
        let mut attackers = vec![a];
        let mut defenders = vec![b];
        let civil = self.realms[a as usize].vassal_of == Some(b) || self.realms[b as usize].vassal_of == Some(a);
        // Vasallen folgen ihrem Lehnsherrn
        for r in self.realms.iter().filter(|r| r.alive && r.vassal_of == Some(a) && r.id != b).map(|r| r.id).collect::<Vec<_>>() {
            if !attackers.contains(&r) && !defenders.contains(&r) {
                attackers.push(r);
            }
        }
        for r in self.realms.iter().filter(|r| r.alive && r.vassal_of == Some(b) && r.id != a).map(|r| r.id).collect::<Vec<_>>() {
            if !attackers.contains(&r) && !defenders.contains(&r) {
                defenders.push(r);
            }
        }
        let claimant = if matches!(goal, WarGoal::Throne) { Some(self.chars[self.realms[a as usize].ruler as usize].id) } else { None };
        let (an, bn) = (self.rname(a), self.rname(b));
        let e = Entry::new(ChronicleKind::War, "war_declared", 45).realm(a).var("a", an).var("b", bn).var("c", cb.name()).var("n", cb.name()).tag("war");
        let chron = self.chron(e);
        self.wars.push(War { id, attackers, defenders, leader_a: a, leader_d: b, cb, goal, score: 0.0, start: day, active: true, battles: 0, casualties_a: 0, casualties_d: 0, civil, claimant, chronicle: Some(chron) });
        self.stats.wars_started += 1;
        self.realms[a as usize].stats.wars_declared += 1;
        self.add_opinion(b, a, OpKind::WarAgainst, -40.0, Some(day + 40 * DAYS_PER_YEAR));
        self.add_opinion(a, b, OpKind::WarAgainst, -20.0, Some(day + 40 * DAYS_PER_YEAR));
        self.emit(StrategyEvent::WarDeclared { war: id });
        // Verbündete rufen
        for ally in self.allies_of(a) {
            if ally != b {
                let _ = self.join_war(ally, id, true, a);
            }
        }
        let mut def_allies: Vec<RealmId> = self.allies_of(b);
        for t in self.treaties.iter().filter(|t| t.end.is_none_or(|e| e > day)) {
            if matches!(t.kind, TreatyKind::DefensivePact) {
                if t.a == b {
                    def_allies.push(t.b);
                } else if t.b == b {
                    def_allies.push(t.a);
                }
            }
            if t.kind == TreatyKind::Guarantee && t.b == b {
                def_allies.push(t.a);
            }
        }
        def_allies.sort_unstable();
        def_allies.dedup();
        for ally in def_allies {
            if ally != a {
                let _ = self.join_war(ally, id, false, b);
            }
        }
        // Koalitionen gegen den Angreifer: Mitglieder treten auf der Verteidigerseite bei
        let coal: Vec<RealmId> = self.coalitions.iter().filter(|c| c.active && c.target == a && c.members.contains(&b)).flat_map(|c| c.members.clone()).collect();
        for m in coal {
            if m != b && m != a {
                let _ = self.join_war(m, id, false, b);
            }
        }
        // Heilige Kriege kosten Frömmigkeit
        if cb == CasusBelli::HolyWar {
            self.realms[a as usize].piety -= 100.0;
        }
        Ok(id)
    }

    fn realms_ai(&self, r: RealmId) -> bool {
        !self.is_player(r)
    }

    fn last_war_with(&self, a: RealmId, b: RealmId) -> Option<u32> {
        self.wars.iter().filter(|w| !w.active && ((w.attackers.contains(&a) && w.defenders.contains(&b)) || (w.attackers.contains(&b) && w.defenders.contains(&a)))).map(|w| w.start).max()
    }

    pub fn add_opinion_all(&mut self, from_view_of: RealmId, about: RealmId, kind: OpKind, value: f32, expires: Option<u32>) {
        self.add_opinion(from_view_of, about, kind, value, expires);
    }

    /// Ein Reich tritt einem Krieg bei (Bündnisruf). Ablehnung kostet Meinung und Prestige.
    pub fn join_war(&mut self, ally: RealmId, war: WarId, attacker_side: bool, caller: RealmId) -> DiplomacyOutcome {
        let day = self.day;
        let Some(w) = self.wars.iter().find(|w| w.id == war && w.active) else { return DiplomacyOutcome::no("Krieg nicht aktiv") };
        if w.attackers.contains(&ally) || w.defenders.contains(&ally) {
            return DiplomacyOutcome::no("Bereits beteiligt");
        }
        if is_antagonist(self.realms[ally as usize].kind) {
            return DiplomacyOutcome::no("Nicht verhandelbar");
        }
        // Sind Alliierte gegeneinander? Dann kein Beitritt
        let enemy_leader = if attacker_side { w.leader_d } else { w.leader_a };
        if self.has_treaty(TreatyKind::Alliance, ally, enemy_leader) {
            return DiplomacyOutcome::no("Bündnis mit der Gegenseite");
        }
        let op = self.opinion(ally, caller);
        let honest = self.chars[self.realms[ally as usize].ruler as usize].has(Trait::Honest);
        let in_other_war = self.in_any_war(ally);
        let p = 0.55 + op * 0.004 + if honest { 0.15 } else { 0.0 } - if in_other_war { 0.2 } else { 0.0 } - if self.realms[ally as usize].regency.is_some() { 0.2 } else { 0.0 };
        if self.chance(p.clamp(0.05, 0.95)) {
            let w = self.wars.iter_mut().find(|w| w.id == war).unwrap();
            if attacker_side {
                w.attackers.push(ally);
            } else {
                w.defenders.push(ally);
            }
            // Gemeinsamer Feind
            DiplomacyOutcome::yes("Beigetreten")
        } else {
            self.add_opinion(caller, ally, OpKind::AllianceCallRefused, -25.0, Some(day + 10 * DAYS_PER_YEAR));
            self.realms[ally as usize].prestige -= 100.0;
            DiplomacyOutcome::no("Bündnisruf abgelehnt")
        }
    }

    /// Kriegspunkte nach einer Schlacht (aus Sicht der Angreifer gerechnet).
    pub(crate) fn war_score_for_battle(&mut self, winner: RealmId, loser: RealmId, score: f32) {
        for w in self.wars.iter_mut().filter(|w| w.active) {
            let win_att = w.attackers.contains(&winner) && w.defenders.contains(&loser);
            let win_def = w.defenders.contains(&winner) && w.attackers.contains(&loser);
            if win_att {
                w.score = (w.score + score).clamp(-100.0, 100.0);
                w.battles += 1;
            } else if win_def {
                w.score = (w.score - score).clamp(-100.0, 100.0);
                w.battles += 1;
            }
        }
    }

    pub(crate) fn war_score_for_occupation(&mut self, by: RealmId, victim: Option<RealmId>, value: f32) {
        let Some(v) = victim else { return };
        for w in self.wars.iter_mut().filter(|w| w.active) {
            if w.attackers.contains(&by) && w.defenders.contains(&v) {
                w.score = (w.score + value).clamp(-100.0, 100.0);
            } else if w.defenders.contains(&by) && w.attackers.contains(&v) {
                w.score = (w.score - value).clamp(-100.0, 100.0);
            }
        }
    }

    /// Provinz wechselt endgültig den Besitzer.
    pub fn transfer_province(&mut self, pid: ProvId, new_owner: RealmId, reason: &str) {
        let day = self.day;
        let old = self.provinces[pid as usize].owner;
        if old == Some(new_owner) {
            self.provinces[pid as usize].controller = None;
            return;
        }
        let name = self.provinces[pid as usize].name.clone();
        {
            let p = &mut self.provinces[pid as usize];
            if p.original_owner.is_none() {
                p.original_owner = old;
            }
            // Rückgabe an den ursprünglichen Besitzer löscht den Anspruch
            if p.original_owner == Some(new_owner) {
                p.original_owner = None;
            }
            p.owner = Some(new_owner);
            p.controller = None;
            p.fief = None;
            p.siege = None;
            p.rebel_held = false;
            p.captured_day = day;
            p.unrest = (p.unrest + 15.0).min(100.0);
        }
        // Heere des alten Herrn verlassen die Provinz nicht automatisch; Rebellen verschwinden
        self.rebuild_caches();
        self.realms[new_owner as usize].recent_conquests.push(day);
        self.realms[new_owner as usize].stats.provinces_conquered += 1;
        self.realms[new_owner as usize].stats.peak_provinces = self.realms[new_owner as usize].stats.peak_provinces.max(self.provinces_of(new_owner).len() as u32);
        let hollow_gain = is_antagonist(self.realms[new_owner as usize].kind);
        if hollow_gain {
            let p = &mut self.provinces[pid as usize];
            p.veil = (p.veil + 0.6).min(5.0);
            p.hollowing = (p.hollowing + 20.0).min(100.0);
        } else if old.is_some_and(|o| is_antagonist(self.realms[o as usize].kind)) {
            let p = &mut self.provinces[pid as usize];
            p.hollowing = (p.hollowing * 0.6).max(0.0);
            p.shrine = 0;
            p.hollow_force = 0.0;
            if let Some(c) = self.realms[new_owner as usize].offices[Office::HighPriest.idx()] {
                let _ = c;
            }
        }
        if let Some(o) = old {
            self.realms[o as usize].stats.provinces_lost += 1;
            // Hauptstadtverlust
            if self.realms[o as usize].capital == pid {
                let provs = self.provinces_of(o).to_vec();
                if let Some(best) = provs.iter().copied().max_by(|a, b| self.provinces[*a as usize].population().total_cmp(&self.provinces[*b as usize].population())) {
                    self.realms[o as usize].capital = best;
                }
            }
            self.check_realm_death(o);
        }
        let (an, bn) = (self.rname(new_owner), old.map(|o| self.rname(o)).unwrap_or_default());
        let is_hollow = is_antagonist(self.realms[new_owner as usize].kind);
        let tpl = if is_hollow { "hollow_conquest" } else if old.is_some_and(|o| is_antagonist(self.realms[o as usize].kind)) { "hollow_liberated" } else { "province_conquered" };
        let mag = if self.provinces[pid as usize].settlement.rank() >= 2 { 45 } else { 30 };
        let e = Entry::new(ChronicleKind::War, tpl, mag).prov(pid, &name).realm(new_owner).var("a", an).var("b", bn).var("c", reason.to_string()).tag("conquest");
        self.chron(e);
        self.emit(StrategyEvent::ProvinceChanged { province: pid, old, new: Some(new_owner) });
    }

    /// Prüft, ob ein Reich ausgelöscht ist.
    pub fn check_realm_death(&mut self, r: RealmId) {
        if !self.realms[r as usize].alive || self.provinces_of(r).iter().any(|p| self.provinces[*p as usize].owner == Some(r)) {
            return;
        }
        if self.provinces.iter().any(|p| p.owner == Some(r)) {
            return;
        }
        if is_antagonist(self.realms[r as usize].kind) {
            // Hohle und Legion können nicht auslöschen
            return;
        }
        let day = self.day;
        self.realms[r as usize].alive = false;
        self.realms[r as usize].fallen = Some(day);
        self.stats.realms_fallen += 1;
        let name = self.realms[r as usize].name.clone();
        let e = Entry::new(ChronicleKind::Realm, "realm_fell", 55).realm(r).var("a", name);
        self.chron(e);
        self.emit(StrategyEvent::RealmFell { realm: r });
        // Heere auflösen
        let ids = self.realm_armies(r);
        for a in ids {
            self.disband_army(a);
        }
        // Vasallen werden unabhängig oder gehen an den Herrn
        let lord = self.realms[r as usize].vassal_of;
        for v in self.realms.iter_mut().filter(|x| x.vassal_of == Some(r)) {
            v.vassal_of = lord;
        }
        // Verträge und Kriege bereinigen
        self.treaties.retain(|t| t.a != r && t.b != r);
        for w in self.wars.iter_mut().filter(|w| w.active) {
            w.attackers.retain(|x| *x != r);
            w.defenders.retain(|x| *x != r);
        }
        let ended: Vec<WarId> = self.wars.iter().filter(|w| w.active && (w.attackers.is_empty() || w.defenders.is_empty() || w.leader_a == r || w.leader_d == r)).map(|w| w.id).collect();
        for id in ended {
            self.end_war(id, true);
        }
        for c in self.coalitions.iter_mut() {
            c.members.retain(|x| *x != r);
            if c.target == r || c.members.len() < 2 {
                c.active = false;
            }
        }
        // Gebundene Aufgaben
        for h in self.houses.iter_mut().filter(|h| h.realm == r) {
            h.alive = false;
        }
        for c in self.chars.iter_mut().filter(|c| c.realm == Some(r)) {
            c.realm = None;
            c.office = None;
        }
        self.realms[r as usize].court.clear();
        self.realms[r as usize].offices = [None; 7];
        if self.player == Some(r) {
            // Der Spieler verliert sein Reich: Fortsetzung beim Lehnsherrn, wenn vorhanden
            self.player = lord.filter(|l| self.realms[*l as usize].alive);
        }
    }

    // ------------------------------------------------------------------------------------
    // Friede
    // ------------------------------------------------------------------------------------
    /// Kosten der Friedensbedingungen in Kriegspunkten (GDD 7.6).
    pub fn peace_cost(&self, terms: &PeaceTerms) -> f32 {
        let mut c = 0.0;
        for p in &terms.cede {
            let pr = &self.provinces[*p as usize];
            let cap = pr.owner.is_some_and(|o| self.realms[o as usize].capital == *p);
            c += crate::military::province_value(pr, cap);
        }
        if terms.tribute {
            c += 20.0;
        }
        if terms.vassalize {
            c += 45.0;
        }
        if terms.independence {
            c += 30.0;
        }
        c
    }

    /// Bietet Frieden an; `from` ist eine Kriegspartei. Bedingungen belasten den Unterlegenen (der Gegner von `from`).
    pub fn offer_peace(&mut self, from: RealmId, war: WarId, terms: PeaceTerms) -> DiplomacyOutcome {
        let Some(w) = self.wars.iter().find(|w| w.id == war && w.active).cloned() else { return DiplomacyOutcome::no("Kein laufender Krieg") };
        let from_att = w.attackers.contains(&from);
        if !from_att && !w.defenders.contains(&from) {
            return DiplomacyOutcome::no("Nicht beteiligt");
        }
        // Punktestand aus Sicht von `from`
        let my_score = if from_att { w.score } else { -w.score };
        let cost = self.peace_cost(&terms);
        let white = cost <= 0.0;
        let opp_leader = if from_att { w.leader_d } else { w.leader_a };
        // Annahme durch Gegner
        let weariness = self.realms[opp_leader as usize].war_weariness;
        let accept = if white {
            // Weißer Friede: Gegner akzeptiert, wenn er nicht klar gewinnt oder kriegsmüde ist
            -my_score <= 10.0 || weariness > 50.0
        } else {
            my_score >= cost || (weariness > 70.0 && cost <= my_score + 20.0)
        };
        if !accept {
            return DiplomacyOutcome::no("Der Gegner lehnt die Bedingungen ab");
        }
        // Spielerreiche bestätigen nicht automatisch – hier: Annahme gilt direkt (Spieler als Gegner entscheidet über Befehl)
        self.make_peace(war, from, terms);
        DiplomacyOutcome::yes("Frieden geschlossen")
    }

    /// Setzt Friedensbedingungen um und beendet den Krieg.
    pub fn make_peace(&mut self, war: WarId, winner_side_member: RealmId, terms: PeaceTerms) {
        let Some(w) = self.wars.iter().find(|w| w.id == war).cloned() else { return };
        let winner_att = w.attackers.contains(&winner_side_member);
        let (winners, losers) = if winner_att { (w.attackers.clone(), w.defenders.clone()) } else { (w.defenders.clone(), w.attackers.clone()) };
        let winner_lead = if winner_att { w.leader_a } else { w.leader_d };
        let loser_lead = if winner_att { w.leader_d } else { w.leader_a };
        for p in &terms.cede {
            if let Some(o) = self.provinces[*p as usize].owner
                && losers.contains(&o) {
                    self.transfer_province(*p, winner_lead, "Friedensschluss");
                }
        }
        if terms.tribute {
            self.add_treaty(TreatyKind::Tribute, winner_lead, loser_lead, Some(10), 0);
            let e = Entry::new(ChronicleKind::Diplomacy, "tribute", 26).realm(winner_lead).var("a", self.rname(winner_lead)).var("b", self.rname(loser_lead));
            self.chron(e);
        }
        if terms.vassalize {
            self.make_vassal(loser_lead, winner_lead);
        }
        if terms.independence {
            // Separatisten (Verlierer ist der Lehnsherr) entlassen den Gewinner aus der Lehnspflicht
            if self.realms[winner_lead as usize].vassal_of == Some(loser_lead) {
                self.realms[winner_lead as usize].vassal_of = None;
                let e = Entry::new(ChronicleKind::Diplomacy, "independence", 48).realm(winner_lead).var("a", self.rname(winner_lead)).var("b", self.rname(loser_lead));
                self.chron(e);
            }
        }
        // Thronkrieg: Anspruch durchsetzen
        if w.goal == WarGoal::Throne && winner_att {
            self.enforce_throne_claim(winner_lead, loser_lead);
        }
        let white = terms == PeaceTerms::default();
        let (an, bn) = (self.rname(winner_lead), self.rname(loser_lead));
        let e = Entry::new(ChronicleKind::Peace, if white { "peace_white" } else { "peace_win" }, 40).realm(winner_lead).var("a", an).var("b", bn).cause(w.chronicle).tag("peace");
        self.chron(e);
        // Meinungen
        let day = self.day;
        for &l in &losers {
            for &win in &winners {
                self.add_opinion(l, win, OpKind::WarAgainst, -40.0, Some(day + 20 * DAYS_PER_YEAR));
            }
        }
        self.add_treaty(TreatyKind::Truce, winner_lead, loser_lead, Some(5), 0);
        self.realms[winner_lead as usize].peace_reputation_until = day + 5 * DAYS_PER_YEAR;
        if !white {
            self.realms[winner_lead as usize].stats.wars_won += 1;
            self.realms[loser_lead as usize].stats.wars_lost += 1;
        }
        self.end_war(war, false);
    }

    /// Beendet einen Krieg: Besatzung aufheben, Kriegsmüdigkeit abbauen.
    pub fn end_war(&mut self, war: WarId, collapsed: bool) {
        let Some(wi) = self.wars.iter().position(|w| w.id == war) else { return };
        if !self.wars[wi].active {
            return;
        }
        self.wars[wi].active = false;
        self.stats.wars_ended += 1;
        let parts: Vec<RealmId> = self.wars[wi].attackers.iter().chain(self.wars[wi].defenders.iter()).copied().collect();
        let day = self.day;
        // Besetzte Provinzen werden an ihre Besitzer zurückgegeben, sofern nicht (mehr) im Krieg
        for pi in 0..self.provinces.len() {
            let (Some(c), Some(o)) = (self.provinces[pi].controller, self.provinces[pi].owner) else { continue };
            if !self.at_war(c, o) {
                self.provinces[pi].controller = None;
                self.provinces[pi].siege = None;
            }
        }
        for r in parts {
            if !self.in_any_war(r) {
                self.realms[r as usize].war_weariness *= 0.4;
                self.realms[r as usize].last_peace_day = day;
            }
        }
        if !collapsed {
            self.emit(StrategyEvent::PeaceMade { war });
        }
        // Heere kehren heim, wenn kein Krieg mehr
        let ids: Vec<ArmyId> = self.armies.iter().filter(|a| matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary)).map(|a| a.id).collect();
        for id in ids {
            let Some(a) = self.army(id) else { continue };
            let Some(o) = a.owner else { continue };
            let p = &self.provinces[a.location as usize];
            let foreign = p.holder().is_some_and(|h| h != o && !self.same_lord(o, h) && !self.has_treaty(TreatyKind::Alliance, o, h) && !self.has_treaty(TreatyKind::MilitaryAccess, o, h) && !self.at_war(o, h));
            if foreign {
                let cap = self.realms[o as usize].capital;
                if let Ok(()) = self.order_move(id, cap) {}
            }
        }
    }

    fn enforce_throne_claim(&mut self, winner: RealmId, loser: RealmId) {
        // Der Anspruchsteller (Herrscher von `winner`) wird Herrscher des Verlierers: Reiche vereinen sich.
        let provs: Vec<ProvId> = self.provinces_of(loser).to_vec();
        for p in provs {
            self.transfer_province(p, winner, "Thronanspruch durchgesetzt");
        }
        // Vasallen des Verlierers werden Vasallen des Gewinners
        for v in self.realms.iter_mut().filter(|x| x.vassal_of == Some(loser)) {
            v.vassal_of = Some(winner);
        }
    }

    // ------------------------------------------------------------------------------------
    // Koalitionen
    // ------------------------------------------------------------------------------------
    fn form_great_power_coalition(&mut self, target: RealmId) {
        // Pakt der Kronen: alle Reiche mit Bedrohung ≥ 40 gegenüber der Großmacht
        let members: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && r.id != target && !is_antagonist(r.kind) && r.vassal_of != Some(target) && self.threat_view(r.id, target) >= 40.0).map(|r| r.id).collect();
        if members.len() >= 2 {
            self.form_coalition(target, members, true);
        }
    }

    pub fn form_coalition(&mut self, target: RealmId, members: Vec<RealmId>, forced: bool) {
        if let Some(c) = self.coalitions.iter_mut().find(|c| c.active && c.target == target) {
            for m in members {
                if !c.members.contains(&m) {
                    c.members.push(m);
                }
            }
            c.forced |= forced;
            return;
        }
        let id = self.next_coalition;
        self.next_coalition += 1;
        let names: Vec<String> = members.iter().map(|m| self.rname(*m)).collect();
        let e = Entry::new(ChronicleKind::Diplomacy, "coalition", 48).realm(target).var("b", self.rname(target)).var("c", names.join(", ")).tag("coalition");
        self.chron(e);
        self.coalitions.push(Coalition { id, target, members, founded: self.day, forced, active: true });
        self.stats.coalitions += 1;
    }

    fn coalitions_month(&mut self) {
        let day = self.day;
        let living: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind)).map(|r| r.id).collect();
        for &t in &living {
            if self.realms[t as usize].threat < 45.0 {
                continue;
            }
            let mut joiners = Vec::new();
            for &c in &living {
                if c == t || self.realms[c as usize].vassal_of == Some(t) || self.realms[t as usize].vassal_of == Some(c) || self.has_treaty(TreatyKind::Alliance, c, t) {
                    continue;
                }
                let th = self.threat_view(c, t);
                let op = self.opinion(c, t);
                if th >= 75.0 && (op < 0.0 || th > 90.0) || (th >= 50.0 && op < -20.0 && self.chance(0.25)) {
                    joiners.push(c);
                }
            }
            if joiners.len() >= 2 || (self.coalitions.iter().any(|c| c.active && c.target == t) && !joiners.is_empty()) {
                self.form_coalition(t, joiners, false);
            }
        }
        // Auflösen, wenn alle Mitglieder Bedrohung unter 40 sehen
        for i in 0..self.coalitions.len() {
            if !self.coalitions[i].active {
                continue;
            }
            let t = self.coalitions[i].target;
            let all_low = self.coalitions[i].members.iter().all(|m| self.threat_view(*m, t) < 40.0);
            if all_low && !self.coalitions[i].forced || !self.realms[t as usize].alive {
                self.coalitions[i].active = false;
            }
            if self.coalitions[i].forced && self.endgame.great_power != Some(t) {
                self.coalitions[i].active = false;
            }
        }
        // Präventivkrieg: Koalitionsstärke ≥ 1,5 × Stärke des Ziels
        let cs: Vec<(usize, RealmId, Vec<RealmId>)> = self.coalitions.iter().enumerate().filter(|(_, c)| c.active).map(|(i, c)| (i, c.target, c.members.clone())).collect();
        for (_, t, members) in cs {
            if self.in_any_war(t) && members.iter().any(|m| self.at_war(*m, t)) {
                continue;
            }
            let total: f32 = members.iter().map(|m| self.realms[*m as usize].strength).sum();
            if total >= 1.5 * self.realms[t as usize].strength && self.chance(0.1) {
                // Stärkstes Mitglied erklärt, Rest tritt bei
                let Some(&lead) = members.iter().filter(|m| self.realms[**m as usize].alive && !self.in_any_war(**m)).max_by(|a, b| self.realms[**a as usize].strength.total_cmp(&self.realms[**b as usize].strength)) else { continue };
                if let Ok(w) = self.declare_war(lead, t, CasusBelli::Coalition, WarGoal::RevertConquests) {
                    for m in members {
                        if m != lead && !self.wars.iter().find(|x| x.id == w).is_some_and(|x| x.attackers.contains(&m))
                            && let Some(x) = self.wars.iter_mut().find(|x| x.id == w)
                                && !x.defenders.contains(&m) {
                                    x.attackers.push(m);
                                }
                    }
                }
            }
        }
        let _ = day;
    }

    // ------------------------------------------------------------------------------------
    // Monat
    // ------------------------------------------------------------------------------------
    pub(crate) fn diplomacy_month(&mut self) {
        self.threat_month();
        self.coalitions_month();
        // Gemeinsamer Kampf gegen die Hohlen: Meinungsgewinn
        let day = self.day;
        let fighters: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind)).map(|r| r.id).collect();
        for &r in &fighters {
            let at_hollow = self.armies.iter().any(|a| a.owner == Some(r) && a.kind == ArmyKind::Realm && self.provinces[a.location as usize].owner.is_some_and(|o| is_antagonist(self.realms[o as usize].kind)));
            if at_hollow && self.endgame.hollow_king_awake.is_some() {
                for &o in &fighters {
                    if o != r && self.armies.iter().any(|a| a.owner == Some(o) && self.provinces[a.location as usize].owner.is_some_and(|x| is_antagonist(self.realms[x as usize].kind))) {
                        self.add_opinion(r, o, OpKind::HelpedAgainstHollow, 2.0, Some(day + 30 * DAYS_PER_YEAR));
                    }
                }
            }
        }
        // Handelsabkommen: +5 Meinung/Jahr bis +20 wird über TradePact-Modifikator abgebildet
        // Verträge ohne Reich bereinigen
        let alive: Vec<bool> = self.realms.iter().map(|r| r.alive).collect();
        self.treaties.retain(|t| alive[t.a as usize] && alive[t.b as usize]);
    }

    /// Kriegsmonat: Punkte halten, Kriegsmüdigkeit, Armeeversorgung.
    pub(crate) fn war_month(&mut self) {
        self.armies_month();
        let ids: Vec<WarId> = self.wars.iter().filter(|w| w.active).map(|w| w.id).collect();
        for id in ids {
            let Some(wi) = self.wars.iter().position(|w| w.id == id) else { continue };
            // Kriegsziel gehalten: +1/Monat
            let goal = self.wars[wi].goal;
            if let WarGoal::TakeProvince(p) = goal {
                let pr = &self.provinces[p as usize];
                if let Some(c) = pr.controller {
                    if self.wars[wi].attackers.contains(&c) {
                        self.wars[wi].score = (self.wars[wi].score + 1.0).min(100.0);
                    }
                } else if pr.owner.is_some_and(|o| self.wars[wi].attackers.contains(&o)) {
                    self.wars[wi].score = (self.wars[wi].score + 1.0).min(100.0);
                }
            }
            // Kriegsmüdigkeit: +0,5/Monat je Partei
            let parts: Vec<RealmId> = self.wars[wi].attackers.iter().chain(self.wars[wi].defenders.iter()).copied().collect();
            for r in parts {
                let rl = &mut self.realms[r as usize];
                rl.war_weariness = (rl.war_weariness + 0.5).min(100.0);
            }
            // Kriegsende: Angreifer oder Verteidiger ohne Land / lange Kriege ohne Fortschritt
            let w = &self.wars[wi];
            let years = self.day.saturating_sub(w.start) / DAYS_PER_YEAR;
            let alive_att = w.attackers.iter().any(|r| self.realms[*r as usize].alive);
            let alive_def = w.defenders.iter().any(|r| self.realms[*r as usize].alive);
            if !alive_att || !alive_def {
                self.end_war(id, true);
                continue;
            }
            let _ = years;
        }
        // Zusätzlich: vergessene Kriege mit Hohlen existieren nicht (ständiger Zustand)
    }

    /// Stress des Herrschers, wenn er gegen seine Persönlichkeit handelt (GDD 5.3).
    pub fn ruler_act(&mut self, r: RealmId, act: crate::catalog::StressAct) {
        let c = self.realms[r as usize].ruler;
        let mut add = 0.0;
        for t in &self.chars[c as usize].traits {
            if let Some((a, s)) = t.stress_act()
                && a == act {
                    add += s as f32;
                }
        }
        if add > 0.0 {
            let ch = &mut self.chars[c as usize];
            ch.stress = (ch.stress + add).min(100.0);
        }
    }
}
