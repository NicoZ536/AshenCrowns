//! KI der Reiche: langfristige Ziele, Wirtschaft, Rekrutierung, Kriegswille, Armeeführung,
//! Diplomatie, Friedensverhandlungen und Intrigen (GDD 7.4).

use crate::catalog::{self, Trait, UClass};
use crate::diplomacy::{is_antagonist, DiplomaticAction, PeaceTerms};
use crate::model::*;
use crate::types::*;
use crate::world::StrategyWorld;

/// Friedensangebot der KI an das Spielerreich.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PeaceOffer {
    pub id: u32,
    pub war: WarId,
    pub from: RealmId,
    pub to: RealmId,
    pub terms: PeaceTerms,
    pub day: u32,
}

impl StrategyWorld {
    fn is_ai(&self, r: RealmId) -> bool {
        let rl = &self.realms[r as usize];
        rl.alive && !is_antagonist(rl.kind) && !self.is_player(r)
    }

    pub(crate) fn ai_month(&mut self) {
        let ids: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind)).map(|r| r.id).collect();
        let month = self.day / DAYS_PER_MONTH;
        for r in ids {
            // Spielerreich: nur Kriegsregeln der Gegner, keine Eigenaktionen
            if self.is_player(r) {
                self.ai_player_support(r);
                continue;
            }
            self.ai_finance(r);
            if (month + r as u32).is_multiple_of(2) {
                self.ai_recruit(r);
            }
            if (month + r as u32).is_multiple_of(3) {
                self.ai_diplomacy(r);
            }
            self.ai_wars(r);
            self.ai_peace(r);
            self.ai_armies(r);
            if (month + r as u32).is_multiple_of(6) {
                self.fill_offices(r);
            }
        }
        self.hollow_ai_month();
        self.expire_peace_offers();
    }

    /// Dem Spieler wird nicht in die Entscheidungen hineingegriffen; fehlende Ämter füllt nur die Autoverwaltung.
    fn ai_player_support(&mut self, r: RealmId) {
        if self.options.rules.auto_govern_player {
            let month = self.day / DAYS_PER_MONTH;
            if month.is_multiple_of(12) {
                self.fill_offices(r);
            }
            // Gefahr durch finanzielle Not abfedern: Kredit bei Notlage
            if self.realms[r as usize].gold < 0.0 {
                let amt = -self.realms[r as usize].gold + 100.0;
                let _ = self.take_loan(r, LoanSource::SaelithBanks, amt, 5);
            }
        }
    }

    pub(crate) fn ai_year(&mut self) {
        let ids: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind)).map(|r| r.id).collect();
        for r in ids {
            self.choose_goals(r);
            if !self.is_player(r) {
                self.fill_offices(r);
                self.ai_marry_dynasty(r);
                self.ai_succession_law(r);
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Ziele
    // ------------------------------------------------------------------------------------
    pub fn choose_goals(&mut self, r: RealmId) {
        let day = self.day;
        let rl = self.realms[r as usize].clone();
        let n_provs = self.provinces_of(r).len() as f32;
        let aggression = self.options.difficulty.mods().ai_aggression;
        let mut cands: Vec<AiGoal> = Vec::new();
        let g = |kind: GoalKind, w: f32, t: Option<RealmId>, p: Option<ProvId>, rel: Option<Religion>| AiGoal { kind, target_realm: t, target_prov: p, religion: rel, started: day, weight: w };
        // Persönlichkeit
        let pm = |p: Personality, kind: GoalKind| -> f32 {
            match (p, kind) {
                (Personality::Conqueror, GoalKind::Expand) => 1.5,
                (Personality::Keeper, GoalKind::Expand) => 0.5,
                (Personality::Merchant, GoalKind::Wealth | GoalKind::TradeRoute) => 1.5,
                (Personality::Zealot, GoalKind::FaithPurity | GoalKind::SpreadFaith) => 1.5,
                (Personality::Schemer, GoalKind::WeakenRival) => 1.5,
                (Personality::Keeper, GoalKind::Survive | GoalKind::Balance | GoalKind::SecureDynasty) => 1.3,
                _ => 1.0,
            }
        };
        let neighbors: Vec<RealmId> = self.realms.iter().filter(|x| x.alive && x.id != r && !is_antagonist(x.kind) && self.realm_distance(r, x.id) == 1).map(|x| x.id).collect();
        let hollow_border = self.provinces.iter().any(|p| p.owner == Some(r) && p.neighbors.iter().any(|l| self.provinces[l.to as usize].owner.is_some_and(|o| is_antagonist(self.realms[o as usize].kind))));
        // Überleben
        let weakest = rl.strength < 500.0 || hollow_border;
        if weakest {
            cands.push(g(GoalKind::Survive, 45.0 * pm(rl.personality, GoalKind::Survive) + if hollow_border { 15.0 } else { 0.0 }, None, None, None));
        }
        if n_provs >= 1.0 {
            cands.push(g(GoalKind::Expand, 40.0 * pm(rl.personality, GoalKind::Expand) * aggression, None, None, None));
        }
        cands.push(g(GoalKind::Wealth, 30.0 * pm(rl.personality, GoalKind::Wealth), None, None, None));
        if rl.faction == "saelith_compact" {
            cands.push(g(GoalKind::Wealth, 60.0, None, None, None));
            cands.push(g(GoalKind::TradeRoute, 55.0, None, None, None));
        }
        if rl.personality == Personality::Merchant {
            cands.push(g(GoalKind::TradeRoute, 45.0, neighbors.first().copied(), None, None));
        }
        if rl.religion == Religion::EternalFlame || rl.personality == Personality::Zealot {
            cands.push(g(GoalKind::FaithPurity, 40.0 * pm(rl.personality, GoalKind::FaithPurity), None, None, Some(rl.religion)));
            cands.push(g(GoalKind::SpreadFaith, 35.0 * pm(rl.personality, GoalKind::SpreadFaith), None, None, Some(rl.religion)));
        }
        // Unabhängigkeit der Vasallen
        if let Some(lord) = rl.vassal_of {
            let op = self.opinion(r, lord);
            cands.push(g(GoalKind::Independence, 45.0 + (-op).max(0.0) * 0.4, Some(lord), None, None));
        }
        // Rache
        if let Some(m) = rl.memory.iter().rev().find(|m| matches!(m.kind, RealmMemKind::FamilyKilled | RealmMemKind::Betrayed) && self.realms[m.other as usize].alive) {
            cands.push(g(GoalKind::Revenge, 55.0, Some(m.other), None, None));
        }
        // Rückeroberung
        let lost = self.provinces.iter().find(|p| p.original_owner == Some(r) && p.owner.is_some_and(|o| o != r && !is_antagonist(self.realms[o as usize].kind)));
        if let Some(p) = lost {
            cands.push(g(GoalKind::Reconquer, 60.0, p.owner, Some(p.id), None));
        }
        // Thronanspruch (Herzogshäuser und Vasallen mit Thronanspruch)
        if let Some(lord) = rl.vassal_of
            && self.chars[rl.ruler as usize].claims.contains(&lord) {
                cands.push(g(GoalKind::ThroneOfValemark, 70.0, Some(lord), None, None));
            }
        // Schleier schließen
        if rl.faction == "thorn_covenant" || (hollow_border && rl.religion != Religion::HollowLiturgy) {
            cands.push(g(GoalKind::SealVeil, 40.0 + self.endgame.veil_pressure * 0.4, None, None, None));
        }
        // Gleichgewicht: Mittelmächte gegen den Stärksten
        if let Some(top) = self.realms.iter().filter(|x| x.alive && !is_antagonist(x.kind) && x.id != r).max_by(|a, b| a.strength.total_cmp(&b.strength)).map(|x| x.id)
            && self.realms[top as usize].strength > rl.strength * 1.4 && self.realms[top as usize].threat > 30.0 {
                cands.push(g(GoalKind::Balance, 40.0 * pm(rl.personality, GoalKind::Balance), Some(top), None, None));
                cands.push(g(GoalKind::WeakenRival, 35.0 * pm(rl.personality, GoalKind::WeakenRival), Some(top), None, None));
            }
        // Dynastie sichern
        let heirs = self.chars[rl.ruler as usize].children.iter().filter(|c| self.chars[**c as usize].alive()).count();
        if heirs == 0 {
            cands.push(g(GoalKind::SecureDynasty, 75.0 * pm(rl.personality, GoalKind::SecureDynasty), None, None, None));
        }
        // Erzfeind vernichten: Reich, das lange im Krieg lag und sehr schlecht bewertet wird
        if let Some(nem) = neighbors.iter().copied().filter(|n| self.opinion(r, *n) < -55.0 && self.realms[*n as usize].alive).min_by(|a, b| self.opinion(r, *a).total_cmp(&self.opinion(r, *b))) {
            cands.push(g(GoalKind::DestroyNemesis, 50.0 + (-self.opinion(r, nem) - 55.0) * 0.5, Some(nem), None, None));
        }
        // Bisherige Ziele behalten leichten Bonus (Trägheit über Jahre)
        for c in cands.iter_mut() {
            if rl.goals.iter().any(|o| o.kind == c.kind && o.target_realm == c.target_realm) {
                c.weight *= 1.25;
            }
        }
        cands.sort_by(|a, b| b.weight.total_cmp(&a.weight));
        let mut chosen: Vec<AiGoal> = Vec::new();
        for c in cands {
            if chosen.len() >= 3 {
                break;
            }
            if chosen.iter().any(|x| x.kind == c.kind) {
                continue;
            }
            let mut c = c;
            // Beibehalten, wenn schon aktiv: ursprünglichen Beginn übernehmen
            if let Some(old) = rl.goals.iter().find(|o| o.kind == c.kind && o.target_realm == c.target_realm) {
                c.started = old.started;
            }
            chosen.push(c);
        }
        self.realms[r as usize].goals = chosen;
        // Aufgaben passend setzen
        if self.is_ai(r) {
            let spread = self.realms[r as usize].goals.iter().any(|g| g.kind == GoalKind::SpreadFaith);
            let hollow = hollow_border;
            self.realms[r as usize].office_task[Office::HighPriest.idx()] = if hollow { 1 } else if spread { 0 } else { 2 };
            let wars = self.in_any_war(r);
            self.realms[r as usize].office_task[Office::Marshal.idx()] = if wars { 1 } else { 0 };
            let schemer = self.realms[r as usize].personality == Personality::Schemer;
            self.realms[r as usize].office_task[Office::Spymaster.idx()] = if schemer { 1 } else { 0 };
        }
    }

    fn goal_weight(&self, r: RealmId, kind: GoalKind, target: Option<RealmId>) -> f32 {
        self.realms[r as usize].goals.iter().filter(|g| g.kind == kind && (target.is_none() || g.target_realm.is_none() || g.target_realm == target)).map(|g| g.weight).fold(0.0, f32::max)
    }

    // ------------------------------------------------------------------------------------
    // Finanzen
    // ------------------------------------------------------------------------------------
    fn ai_finance(&mut self, r: RealmId) {
        let day = self.day;
        let provs = self.provinces_of(r).to_vec();
        if provs.is_empty() {
            return;
        }
        let avg_unrest = provs.iter().map(|p| self.provinces[*p as usize].unrest).sum::<f32>() / provs.len() as f32;
        let (gold, exp) = (self.realms[r as usize].gold, self.realms[r as usize].expenses);
        let reserve = (exp * 6.0).max(600.0);
        let regency = self.realms[r as usize].regency.is_some();
        let tax = self.realms[r as usize].tax_level;
        let day_ok = !regency || self.day.is_multiple_of(DAYS_PER_YEAR);
        if day_ok {
            if gold < reserve * 0.4 && tax < 3 && avg_unrest < 28.0 {
                self.realms[r as usize].tax_level += 1;
                if self.chars[self.realms[r as usize].ruler as usize].has(Trait::Generous) {
                    self.ruler_act(r, catalog::StressAct::RaiseTaxes);
                }
            } else if (avg_unrest > 42.0 && tax > 1) || (gold > reserve * 4.0 && tax > 2) {
                self.realms[r as usize].tax_level -= 1;
            }
        }
        // Kredite
        if gold < reserve * 0.15 && self.realms[r as usize].credit_blocked_until <= day {
            let want = reserve;
            let src = if self.in_any_war(r) { LoanSource::War } else { LoanSource::SaelithBanks };
            if self.take_loan(r, src, want, 5).is_err() {
                let _ = self.take_loan(r, LoanSource::SaelithBanks, want, 5);
            }
        }
        // Ausgaben: überschüssiges Gold in Befestigungen und Forschung
        if gold > reserve * 3.0 {
            let frontier: Vec<ProvId> = provs.iter().copied().filter(|p| self.provinces[*p as usize].neighbors.iter().any(|l| self.provinces[l.to as usize].owner.is_some_and(|o| o != r && (self.at_war(r, o) || self.opinion(r, o) < 0.0))) && self.provinces[*p as usize].fortification < 6).collect();
            if let Some(&p) = frontier.first() {
                self.provinces[p as usize].fortification += 1;
                self.realms[r as usize].gold -= 120.0;
            }
        }
        // Hofhaltung: Überschüsse fließen in Prachtbauten, Stiftungen und Hofstaat (Prestigegewinn)
        let excess = gold - exp * 24.0 - 2000.0;
        if excess > 0.0 {
            self.realms[r as usize].gold -= excess * 0.04;
            self.realms[r as usize].prestige += excess * 0.04 * 0.02;
        }
        // Forschung wählen
        if self.realms[r as usize].research_focus.is_none() && self.realms[r as usize].laws.magic_law != 0 {
            let school = self.preferred_school(r);
            self.realms[r as usize].research_focus = Some(school);
        }
        let tr = self.realms[r as usize].trade_tax;
        let want_tax = if self.realms[r as usize].personality == Personality::Merchant { 5.0 } else { 8.0 };
        if (tr - want_tax).abs() > 0.1 {
            self.realms[r as usize].trade_tax = want_tax;
        }
        let want_tariff = if self.goal_weight(r, GoalKind::TradeRoute, None) > 0.0 || self.realms[r as usize].personality == Personality::Merchant { 3.0 } else { 8.0 };
        self.realms[r as usize].tariff_import = want_tariff;
    }

    pub fn preferred_school(&self, r: RealmId) -> School {
        let rl = &self.realms[r as usize];
        match rl.faction.as_str() {
            "ember_crown" => School::Ember,
            "skarn_clans" => School::Stone,
            "thorn_covenant" => School::Growth,
            "saelith_compact" => School::Tide,
            _ => match rl.culture {
                Culture::Valemarker | Culture::Aschmaerker => School::Ember,
                Culture::Skarn => School::Stone,
                Culture::Dornwaldler | Culture::Veyl => School::Growth,
                Culture::Saelith => School::Tide,
                _ => School::Order,
            },
        }
    }

    // ------------------------------------------------------------------------------------
    // Rekrutierung
    // ------------------------------------------------------------------------------------
    pub fn realm_soldiers(&self, r: RealmId) -> u32 {
        self.armies.iter().filter(|a| a.owner == Some(r) && matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary)).map(|a| a.size()).sum()
    }

    fn ai_recruit(&mut self, r: RealmId) {
        let provs = self.provinces_of(r).to_vec();
        if provs.is_empty() {
            return;
        }
        let total_pop: f32 = provs.iter().map(|p| self.provinces[*p as usize].population()).sum();
        let at_war = self.in_any_war(r);
        let aggression = self.options.difficulty.mods().ai_aggression;
        let threatened = provs.iter().any(|p| self.provinces[*p as usize].neighbors.iter().any(|l| self.provinces[l.to as usize].owner.is_some_and(|o| is_antagonist(self.realms[o as usize].kind))));
        let rich = self.realms[r as usize].gold > (self.realms[r as usize].expenses * 12.0).max(1500.0);
        let mut frac = if at_war { 0.035 } else { 0.014 };
        if rich {
            frac *= 1.8;
        }
        if threatened {
            frac += 0.01;
        }
        frac *= 0.8 + 0.2 * aggression;
        if self.realms[r as usize].laws.martial_law == 1 {
            frac *= 1.2;
        }
        let target = (total_pop * frac) as u32;
        let have = self.realm_soldiers(r);
        if have >= target {
            return;
        }
        let want = ((target - have) as f32 * 0.35).ceil().max(10.0) as u32;
        // Provinz mit der größten Rekrutenbasis
        let best = provs.iter().copied().filter(|p| self.provinces[*p as usize].holder() == Some(r)).max_by(|a, b| {
            let f = |p: ProvId| {
                let pr = &self.provinces[p as usize];
                pr.pop[Stratum::Warriors.idx()].adults + pr.pop[Stratum::Freeholders.idx()].adults * 0.4
            };
            f(*a).total_cmp(&f(*b))
        });
        let Some(p) = best else { return };
        let roster = catalog::roster(self.realms[r as usize].culture);
        let pick = |class: UClass| roster.iter().copied().filter(|id| catalog::unit(id).is_some_and(|u| u.class == class && u.cost > 0.0)).collect::<Vec<&str>>();
        let infantry = pick(UClass::Infantry);
        let ranged = pick(UClass::Ranged);
        let cavalry = pick(UClass::Cavalry);
        let siege = pick(UClass::Siege);
        let mut plan: Vec<(&str, u32)> = Vec::new();
        let mut rest = want;
        if !infantry.is_empty() {
            let i = self.ri(0, infantry.len() as i32 - 1) as usize;
            let n = (want as f32 * 0.5) as u32;
            plan.push((infantry[i], n));
            rest = rest.saturating_sub(n);
        }
        if !ranged.is_empty() {
            let i = self.ri(0, ranged.len() as i32 - 1) as usize;
            let n = (want as f32 * 0.3) as u32;
            plan.push((ranged[i], n));
            rest = rest.saturating_sub(n);
        }
        if !cavalry.is_empty() {
            let i = self.ri(0, cavalry.len() as i32 - 1) as usize;
            let n = (want as f32 * 0.1) as u32;
            plan.push((cavalry[i], n));
        }
        if at_war && !siege.is_empty() && self.chance(0.3) {
            let i = self.ri(0, siege.len() as i32 - 1) as usize;
            plan.push((siege[i], 1 + want / 150));
        }
        if plan.is_empty() {
            plan.push(("worker", want));
        }
        let _ = rest;
        for (unit, n) in plan {
            if n == 0 {
                continue;
            }
            let mut n = n;
            while n > 0 {
                if self.recruit(r, p, unit, n).is_ok() {
                    break;
                }
                n /= 2;
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Kriegswille
    // ------------------------------------------------------------------------------------
    /// Kriegswille nach GDD 7.4. Gibt (Wille, Kriegsgrund, Ziel) zurück.
    pub fn war_will(&self, a: RealmId, t: RealmId) -> Option<(f32, CasusBelli, WarGoal)> {
        let (ra, rt) = (&self.realms[a as usize], &self.realms[t as usize]);
        if !rt.alive || a == t || is_antagonist(rt.kind) || self.at_war(a, t) {
            return None;
        }
        if self.has_treaty(TreatyKind::Truce, a, t) && ra.vassal_of != Some(t) {
            return None;
        }
        if self.has_treaty(TreatyKind::Alliance, a, t) || self.has_treaty(TreatyKind::SecretAlliance, a, t) || rt.vassal_of == Some(a) && ra.personality == Personality::Keeper {
            return None;
        }
        let cb = self.find_cb(a, t);
        let (cb, unjust) = match cb {
            Some(c) => (c, false),
            None => (CasusBelli::Unjustified, true),
        };
        // Ungerechtfertigt nur Eroberer mit viel Prestige
        if unjust && !(ra.personality == Personality::Conqueror && ra.prestige > 500.0) {
            return None;
        }
        let op = self.opinion(a, t);
        let mut util = 0.0f32;
        let mut goal = WarGoal::Tribute;
        let expand = self.goal_weight(a, GoalKind::Expand, Some(t));
        if expand > 0.0 {
            util = util.max(expand);
            goal = WarGoal::TakeProvince(self.best_target_province(a, t).unwrap_or(rt.capital));
        }
        let rec = self.goal_weight(a, GoalKind::Reconquer, Some(t));
        if rec > 0.0 && self.provinces.iter().any(|p| p.owner == Some(t) && p.original_owner == Some(a)) {
            util = util.max(rec + 10.0);
            goal = WarGoal::RevertConquests;
        }
        let nem = self.goal_weight(a, GoalKind::DestroyNemesis, Some(t));
        if nem > 0.0 {
            util = util.max(nem + 20.0);
            goal = WarGoal::Subjugate;
        }
        let rev = self.goal_weight(a, GoalKind::Revenge, Some(t));
        if rev > 0.0 {
            util = util.max(rev + 15.0);
            goal = WarGoal::TakeProvince(self.best_target_province(a, t).unwrap_or(rt.capital));
        }
        if ra.vassal_of == Some(t) {
            let ind = self.goal_weight(a, GoalKind::Independence, Some(t));
            let thr = self.goal_weight(a, GoalKind::ThroneOfValemark, Some(t));
            if thr > 0.0 {
                util = util.max(thr + 15.0);
                goal = WarGoal::Throne;
            } else if ind > 0.0 {
                util = util.max(ind + 15.0);
                goal = WarGoal::Independence;
            }
        }
        let faith = self.goal_weight(a, GoalKind::FaithPurity, None);
        if cb == CasusBelli::HolyWar && faith > 0.0 {
            util = util.max(faith + 10.0);
            goal = WarGoal::TakeProvince(self.best_target_province(a, t).unwrap_or(rt.capital));
        }
        let weaken = self.goal_weight(a, GoalKind::WeakenRival, Some(t));
        if weaken > 0.0 {
            util = util.max(weaken + 10.0);
            goal = WarGoal::Tribute;
        }
        if cb == CasusBelli::Coalition {
            util = util.max(60.0);
            goal = WarGoal::RevertConquests;
        }
        if util <= 0.0 {
            // Opportunismus: Reiche mit reicher, schwacher Nachbarschaft
            if rt.strength < ra.strength * 0.5 && self.realm_distance(a, t) == 1 {
                util = 30.0;
                goal = WarGoal::TakeProvince(self.best_target_province(a, t).unwrap_or(rt.capital));
            } else {
                return None;
            }
        }
        // Stärkeverhältnis mit Verbündeten
        let strength_of = |r: RealmId| -> f32 {
            let mut s = self.realms[r as usize].strength;
            for al in self.allies_of(r) {
                s += self.realms[al as usize].strength * 0.5;
            }
            for v in self.realms.iter().filter(|x| x.alive && x.vassal_of == Some(r)) {
                s += v.strength * 0.4;
            }
            s
        };
        let ratio = (strength_of(a) / strength_of(t).max(1.0)).clamp(0.0, 3.0);
        // Ungebundene Armeen: nur kämpfen, wenn die eigene Armee existiert
        if self.realm_army_power(a) < 80.0 {
            return None;
        }
        let aggr = self.options.difficulty.mods().ai_aggression;
        let will = util * ratio * aggr - op * 0.3 - ra.war_weariness * 0.5 + if unjust { -10.0 } else { 0.0 };
        Some((will, cb, goal))
    }

    fn best_target_province(&self, a: RealmId, t: RealmId) -> Option<ProvId> {
        let mut best: Option<(f32, ProvId)> = None;
        for p in self.provinces_of(t) {
            let pr = &self.provinces[*p as usize];
            let adj = pr.neighbors.iter().any(|l| self.provinces[l.to as usize].owner == Some(a));
            if !adj {
                continue;
            }
            let mut s = crate::military::province_value(pr, self.realms[t as usize].capital == *p);
            if pr.original_owner == Some(a) {
                s += 25.0;
            }
            if pr.culture == self.realms[a as usize].culture {
                s += 6.0;
            }
            s -= pr.fortification as f32 * 1.5;
            if best.is_none_or(|b| s > b.0) {
                best = Some((s, *p));
            }
        }
        best.map(|b| b.1)
    }

    fn ai_wars(&mut self, r: RealmId) {
        // höchstens zwei laufende Kriege, nie unter Regentschaft, nach Frieden Pause
        let active = self.wars_of(r).len();
        if active >= 1 || self.realms[r as usize].regency.is_some() {
            return;
        }
        let day = self.day;
        if day.saturating_sub(self.realms[r as usize].last_peace_day) < 2 * DAYS_PER_YEAR {
            return;
        }
        if self.realms[r as usize].war_weariness > 30.0 {
            return;
        }
        let provs = self.provinces_of(r).len();
        if provs == 0 {
            return;
        }
        let ruler = &self.chars[self.realms[r as usize].ruler as usize];
        // Friedensruf / Ruf des Friedens
        if ruler.has(Trait::Content) && self.chance(0.5) {
            return;
        }
        let thr = match self.realms[r as usize].personality {
            Personality::Conqueror => 30.0,
            Personality::Keeper => 60.0,
            _ => 40.0,
        };
        let mut cands: Vec<(f32, RealmId, CasusBelli, WarGoal)> = Vec::new();
        for t in self.realms.iter().filter(|x| x.alive && x.id != r && !is_antagonist(x.kind) && (self.realm_distance(r, x.id) == 1 || self.realms[r as usize].vassal_of == Some(x.id))).map(|x| x.id).collect::<Vec<_>>() {
            if let Some((w, cb, goal)) = self.war_will(r, t)
                && w > thr {
                    cands.push((w, t, cb, goal));
                }
        }
        cands.sort_by(|a, b| b.0.total_cmp(&a.0));
        if let Some((_, t, cb, goal)) = cands.first().copied() {
            // Zufallsschwelle: nicht jeden Monat
            if self.chance(0.35) {
                let _ = self.diplomacy(r, t, DiplomaticAction::DeclareWar { cb, goal });
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Frieden
    // ------------------------------------------------------------------------------------
    fn terms_for(&self, war: &War, winner_att: bool) -> PeaceTerms {
        let (winners, losers) = if winner_att { (&war.attackers, &war.defenders) } else { (&war.defenders, &war.attackers) };
        let score = if winner_att { war.score } else { -war.score };
        let mut terms = PeaceTerms::default();
        let mut budget = score;
        let loser_lead = if winner_att { war.leader_d } else { war.leader_a };
        let goal = war.goal;
        match goal {
            WarGoal::TakeProvince(p) => {
                if self.provinces[p as usize].owner.is_some_and(|o| losers.contains(&o)) {
                    let cap = self.provinces[p as usize].owner.is_some_and(|o| self.realms[o as usize].capital == p);
                    let c = crate::military::province_value(&self.provinces[p as usize], cap);
                    if c <= budget {
                        terms.cede.push(p);
                        budget -= c;
                    }
                }
            }
            WarGoal::Tribute => {
                if budget >= 20.0 {
                    terms.tribute = true;
                    budget -= 20.0;
                }
            }
            WarGoal::Subjugate => {
                if budget >= 45.0 {
                    terms.vassalize = true;
                    budget -= 45.0;
                }
            }
            WarGoal::Independence
                if budget >= 30.0 => {
                    terms.independence = true;
                    budget -= 30.0;
                }
            _ => {}
        }
        // Zusätzliche besetzte Provinzen, solange Punkte reichen
        let mut occ: Vec<(f32, ProvId)> = self
            .provinces
            .iter()
            .filter(|p| p.owner.is_some_and(|o| losers.contains(&o)) && p.controller.is_some_and(|c| winners.contains(&c)) && !terms.cede.contains(&p.id))
            .map(|p| (crate::military::province_value(p, self.realms[p.owner.unwrap() as usize].capital == p.id), p.id))
            .collect();
        occ.sort_by(|a, b| b.0.total_cmp(&a.0));
        if goal != WarGoal::Tribute && goal != WarGoal::Independence {
            for (c, p) in occ {
                if c <= budget {
                    terms.cede.push(p);
                    budget -= c;
                }
            }
        }
        let _ = loser_lead;
        if matches!(goal, WarGoal::Throne) {
            terms = PeaceTerms { cede: Vec::new(), tribute: false, vassalize: false, independence: false };
        }
        terms
    }

    fn ai_peace(&mut self, r: RealmId) {
        let day = self.day;
        for wid in self.wars_of(r) {
            let Some(w) = self.wars.iter().find(|w| w.id == wid).cloned() else { continue };
            let leader_a = w.attackers.first().copied().unwrap_or(w.leader_a);
            let leader_d = w.defenders.first().copied().unwrap_or(w.leader_d);
            // Nur der Kriegsführer entscheidet
            if r != leader_a && r != leader_d {
                continue;
            }
            let att = r == leader_a;
            let my_score = if att { w.score } else { -w.score };
            let opp = if att { leader_d } else { leader_a };
            let years = day.saturating_sub(w.start) / DAYS_PER_YEAR;
            let weariness = self.realms[r as usize].war_weariness;
            let opp_weariness = self.realms[opp as usize].war_weariness;
            let player_opp = self.is_player(opp);
            // Siegreich: Bedingungen diktieren
            let terms = self.terms_for(&w, att);
            let cost = self.peace_cost(&terms);
            let goal_ok = cost > 0.0 && cost <= my_score;
            let mut want_peace = false;
            let mut offer = PeaceTerms::default();
            if goal_ok {
                want_peace = true;
                offer = terms;
            } else if my_score >= 25.0 && (years >= 2 || weariness > 50.0 || opp_weariness > 70.0) {
                // Teilerfolg: Tribut oder besetzte Provinz
                let t2 = PeaceTerms { tribute: my_score >= 20.0, ..Default::default() };
                want_peace = true;
                offer = t2;
            } else if (weariness > 55.0 || (years >= 3 && my_score.abs() < 20.0)) && my_score > -35.0 {
                // Erschöpfung: Weißer Friede
                want_peace = true;
            } else if my_score < -40.0 && weariness > 40.0 {
                // Aussichtslos: den Gegner bitten – er erhält Bedingungen nach seinem Punktestand
                let opp_att = !att;
                let mut t = self.terms_for(&w, opp_att);
                if t == PeaceTerms::default() {
                    t.tribute = true;
                }
                // Der Gegner diktiert, wenn er will
                if self.chance(0.5) {
                    if self.is_player(opp) {
                        self.add_peace_offer(opp, r, wid, t);
                    } else {
                        let _ = self.diplomacy(opp, r, DiplomaticAction::OfferPeace { war: wid, terms: t });
                    }
                }
                continue;
            }
            if want_peace && self.chance(0.7) {
                if player_opp {
                    self.add_peace_offer(opp, r, wid, offer);
                } else {
                    let _ = self.diplomacy(r, opp, DiplomaticAction::OfferPeace { war: wid, terms: offer });
                }
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Armeen
    // ------------------------------------------------------------------------------------
    fn enemy_realms(&self, r: RealmId) -> Vec<RealmId> {
        let mut v = Vec::new();
        for w in self.wars.iter().filter(|w| w.active) {
            if w.attackers.contains(&r) {
                v.extend(w.defenders.iter().copied());
            } else if w.defenders.contains(&r) {
                v.extend(w.attackers.iter().copied());
            }
        }
        v.sort_unstable();
        v.dedup();
        v
    }

    fn ai_armies(&mut self, r: RealmId) {
        let day = self.day;
        let enemies = self.enemy_realms(r);
        let crusade = self.crusading(r);
        // Zusammenlegen am selben Ort
        let ids = self.realm_armies(r);
        let mut by_loc: std::collections::BTreeMap<ProvId, Vec<ArmyId>> = std::collections::BTreeMap::new();
        for id in &ids {
            if let Some(a) = self.army(*id)
                && a.path.is_empty() && a.in_battle.is_none() && a.order != ArmyOrder::Siege {
                    by_loc.entry(a.location).or_default().push(*id);
                }
        }
        for (_, list) in by_loc {
            for w in list.windows(2) {
                if let (Some(a), Some(b)) = (self.army(w[0]), self.army(w[1]))
                    && a.location == b.location && (!enemies.is_empty() || a.size() + b.size() < 800) {
                        let _ = self.merge_armies(w[0], w[1]);
                    }
            }
        }
        let ids = self.realm_armies(r);
        for id in ids {
            let Some(a) = self.army(id).cloned() else { continue };
            if !a.path.is_empty() || a.in_battle.is_some() || a.locked_until > day || a.order == ArmyOrder::Siege {
                continue;
            }
            if a.size() < 25 {
                continue;
            }
            // Befehlshaber
            self.assign_commander(id);
            let power = army_power_of(&a);
            // 1. Aufständische im eigenen Land
            if let Some(target) = self.nearest_rebel_province(r, a.location, power) {
                let _ = self.order_move(id, target);
                continue;
            }
            // 2. Krieg
            if !enemies.is_empty()
                && let Some(t) = self.pick_offensive_target(r, &a, &enemies) {
                    if t != a.location {
                        let _ = self.order_move(id, t);
                    }
                    continue;
                }
            // 3. Kreuzzug gegen Hohlland
            if crusade && power > 900.0
                && let Some(t) = self.pick_hollow_target(r, &a) {
                    if t != a.location {
                        let _ = self.order_move(id, t);
                    }
                    continue;
                }
            // 4. Heimkehr / Grenzschutz
            let home = self.guard_province(r);
            if a.location != home && (self.provinces[a.location as usize].holder() != Some(r) || self.chance(0.05)) {
                let _ = self.order_move(id, home);
            }
        }
    }

    fn crusading(&self, r: RealmId) -> bool {
        let rl = &self.realms[r as usize];
        self.goal_weight(r, GoalKind::SealVeil, None) > 0.0 || self.goal_weight(r, GoalKind::FaithPurity, None) > 0.0 && rl.religion == Religion::EternalFlame || self.endgame.hollow_king_awake.is_some()
    }

    fn guard_province(&self, r: RealmId) -> ProvId {
        let rl = &self.realms[r as usize];
        // Gefährdetste Grenzprovinz, sonst Hauptstadt
        let mut best: Option<(f32, ProvId)> = None;
        for p in self.provinces_of(r) {
            let pr = &self.provinces[*p as usize];
            let mut danger = 0.0;
            for l in &pr.neighbors {
                let q = &self.provinces[l.to as usize];
                if let Some(o) = q.owner
                    && o != r && (self.at_war(r, o) || is_antagonist(self.realms[o as usize].kind)) {
                        danger += 2.0;
                    }
            }
            if pr.unrest > 50.0 {
                danger += 1.0;
            }
            if danger > 0.0 && best.is_none_or(|b| danger > b.0) {
                best = Some((danger, *p));
            }
        }
        best.map(|b| b.1).unwrap_or(rl.capital)
    }

    fn nearest_rebel_province(&self, r: RealmId, from: ProvId, power: f32) -> Option<ProvId> {
        let mut best: Option<(f32, ProvId)> = None;
        for a in self.armies.iter().filter(|a| a.kind == ArmyKind::Rebels && a.target_realm == Some(r)) {
            let rp = army_power_of(a);
            if power < rp * 1.1 {
                continue;
            }
            let d = (self.provinces[from as usize].centroid - self.provinces[a.location as usize].centroid).length();
            if best.is_none_or(|b| d < b.0) {
                best = Some((d, a.location));
            }
        }
        // Provinzen unter Rebellenkontrolle
        for p in self.provinces_of(r) {
            if self.provinces[*p as usize].rebel_held {
                let d = (self.provinces[from as usize].centroid - self.provinces[*p as usize].centroid).length();
                if best.is_none_or(|b| d < b.0) {
                    best = Some((d, *p));
                }
            }
        }
        best.map(|b| b.1)
    }

    fn pick_offensive_target(&self, r: RealmId, a: &Army, enemies: &[RealmId]) -> Option<ProvId> {
        let my_power = army_power_of(a);
        let owner = r;
        let mut best: Option<(f32, ProvId)> = None;
        // Eigene besetzte Provinzen zurückerobern: höchste Priorität
        for p in &self.provinces {
            let held_by_enemy = p.owner == Some(r) && p.controller.is_some_and(|c| enemies.contains(&c));
            let enemy_land = p.owner.is_some_and(|o| enemies.contains(&o)) && p.controller != Some(r);
            if !held_by_enemy && !enemy_land {
                continue;
            }
            // Feindliche Heere dort?
            let enemy_power: f32 = self.armies.iter().filter(|x| x.location == p.id && self.armies_hostile(a, x)).map(army_power_of).sum();
            let garrison = p.garrison as f32 * 28.0 * (1.0 + p.fortification as f32 * 0.1) * if enemy_land && p.settlement.rank() >= 1 { 0.5 } else { 0.2 };
            if my_power < (enemy_power + garrison) * 0.8 {
                continue;
            }
            let dist = (self.provinces[a.location as usize].centroid - p.centroid).length();
            let mut s = crate::military::province_value(p, p.owner.is_some_and(|o| self.realms[o as usize].capital == p.id)) - dist * 0.04;
            if held_by_enemy {
                s += 40.0;
            }
            // Kriegsziel
            for w in self.wars.iter().filter(|w| w.active && (w.attackers.contains(&owner) || w.defenders.contains(&owner))) {
                if let WarGoal::TakeProvince(g) = w.goal
                    && g == p.id {
                        s += 25.0;
                    }
            }
            if self.find_path(owner, a.location, p.id).is_none() {
                continue;
            }
            if best.is_none_or(|b| s > b.0) {
                best = Some((s, p.id));
            }
        }
        best.map(|b| b.1)
    }

    fn pick_hollow_target(&self, r: RealmId, a: &Army) -> Option<ProvId> {
        let my_power = army_power_of(a);
        let mut best: Option<(f32, ProvId)> = None;
        for p in &self.provinces {
            let Some(o) = p.owner else { continue };
            if !is_antagonist(self.realms[o as usize].kind) {
                continue;
            }
            let adj = p.neighbors.iter().any(|l| self.provinces[l.to as usize].owner == Some(r)) || self.armies.iter().any(|x| x.owner == Some(r) && x.location == p.id);
            if !adj {
                continue;
            }
            let enemy: f32 = self.armies.iter().filter(|x| x.location == p.id && self.armies_hostile(a, x)).map(army_power_of).sum();
            if my_power < enemy * 1.2 + p.garrison as f32 * 20.0 + 600.0 {
                continue;
            }
            let dist = (self.provinces[a.location as usize].centroid - p.centroid).length();
            let s = 20.0 - dist * 0.05 + p.hollow_force * 0.05;
            if self.find_path(r, a.location, p.id).is_none() {
                continue;
            }
            if best.is_none_or(|b| s > b.0) {
                best = Some((s, p.id));
            }
        }
        best.map(|b| b.1)
    }

    // ------------------------------------------------------------------------------------
    // Diplomatie der KI
    // ------------------------------------------------------------------------------------
    fn ai_diplomacy(&mut self, r: RealmId) {
        let neighbors: Vec<RealmId> = self.realms.iter().filter(|x| x.alive && x.id != r && !is_antagonist(x.kind) && self.realm_distance(r, x.id) <= 2 && !self.at_war(r, x.id)).map(|x| x.id).collect();
        if neighbors.is_empty() {
            return;
        }
        let pers = self.realms[r as usize].personality;
        let trade_goal = self.goal_weight(r, GoalKind::TradeRoute, None) > 0.0 || self.goal_weight(r, GoalKind::Wealth, None) > 0.0;
        let survive = self.goal_weight(r, GoalKind::Survive, None) > 0.0 || self.goal_weight(r, GoalKind::Balance, None) > 0.0;
        // 1. Gesandter zur Verbesserung der Beziehungen zu wichtigen Nachbarn
        for &t in &neighbors {
            let op = self.opinion(t, r);
            if op < 20.0 && self.realms[r as usize].gold > 200.0 && self.chance(0.1) && self.realm_distance(r, t) == 1 && !self.at_war(r, t) {
                let _ = self.diplomacy(r, t, DiplomaticAction::Envoy);
            }
        }
        // 2. Handelsabkommen
        if (trade_goal || pers == Personality::Merchant || self.chance(0.2)) && !self.has_trade_with_neighbors(r) {
            let cands: Vec<RealmId> = neighbors.iter().copied().filter(|t| !self.has_treaty(TreatyKind::Trade, r, *t) && self.opinion(*t, r) >= 0.0).collect();
            if let Some(&t) = cands.first() {
                let _ = self.diplomacy(r, t, DiplomaticAction::Propose(TreatyKind::Trade));
            }
        }
        // 3. Nichtangriffspakt mit stärkeren Nachbarn
        if survive || pers == Personality::Keeper {
            for &t in &neighbors {
                if self.realms[t as usize].strength > self.realms[r as usize].strength && !self.has_treaty(TreatyKind::NonAggression, r, t) && !self.has_treaty(TreatyKind::Alliance, r, t) && self.opinion(t, r) >= 10.0 {
                    let _ = self.diplomacy(r, t, DiplomaticAction::Propose(TreatyKind::NonAggression));
                    break;
                }
            }
        }
        // 4. Bündnis mit gemeinsamem Feind oder starkem Wohlwollen
        let enemies = self.enemy_realms(r);
        let threats: Vec<RealmId> = self.realms.iter().filter(|x| x.alive && x.id != r && !is_antagonist(x.kind) && self.threat_view(r, x.id) > 50.0).map(|x| x.id).collect();
        let ally_cands: Vec<RealmId> = neighbors.iter().copied().filter(|t| self.allies_of(r).len() < 3 && !self.has_treaty(TreatyKind::Alliance, r, *t) && self.opinion(*t, r) >= 50.0 && (enemies.iter().any(|e| self.at_war(*t, *e) || self.opinion(*t, *e) < 0.0) || threats.iter().any(|x| *x != *t && self.opinion(*t, *x) < 0.0))).collect();
        if let Some(&t) = ally_cands.first() {
            let _ = self.diplomacy(r, t, DiplomaticAction::Propose(TreatyKind::Alliance));
        }
        // 5. Gemeinsame Front gegen das Tief
        let hollow_border = self.provinces.iter().any(|p| p.owner == Some(r) && p.neighbors.iter().any(|l| self.provinces[l.to as usize].owner.is_some_and(|o| is_antagonist(self.realms[o as usize].kind))));
        if hollow_border {
            for &t in &neighbors {
                if !self.has_treaty(TreatyKind::AgainstHollow, r, t) && self.chance(0.3) {
                    let _ = self.diplomacy(r, t, DiplomaticAction::Propose(TreatyKind::AgainstHollow));
                    break;
                }
            }
        }
        // 6. Boykott gegen Rivalen (WeakenRival, Handel)
        if let Some(g) = self.realms[r as usize].goals.iter().find(|g| g.kind == GoalKind::WeakenRival).copied()
            && let Some(t) = g.target_realm
                && self.realms[t as usize].alive && !self.realms[r as usize].embargo.contains(&t) && pers == Personality::Merchant && self.chance(0.1) {
                    let _ = self.diplomacy(r, t, DiplomaticAction::Embargo(true));
                }
        // 7. Tribut fordern von sehr schwachen Nachbarn
        if pers == Personality::Conqueror {
            for &t in &neighbors {
                if self.realms[t as usize].vassal_of.is_none() && !self.has_treaty(TreatyKind::Tribute, r, t) && self.realms[r as usize].strength > self.realms[t as usize].strength * 2.2 && self.chance(0.08) {
                    let _ = self.diplomacy(r, t, DiplomaticAction::DemandTribute);
                    break;
                }
            }
        }
        // Boykotte beenden, wenn die Meinung besser ist
        let embargo = self.realms[r as usize].embargo.clone();
        for t in embargo {
            if self.opinion(r, t) > 0.0 {
                self.realms[r as usize].embargo.retain(|x| *x != t);
            }
        }
    }

    fn has_trade_with_neighbors(&self, r: RealmId) -> bool {
        self.treaties.iter().any(|t| t.kind == TreatyKind::Trade && (t.a == r || t.b == r) && t.end.is_none_or(|e| e > self.day))
    }

    // ------------------------------------------------------------------------------------
    // Dynastiepolitik der KI
    // ------------------------------------------------------------------------------------
    fn ai_marry_dynasty(&mut self, r: RealmId) {
        // Heiratsbündnisse über Herrscherfamilien: Erben/Kinder mit Familie eines freundlichen Nachbarn
        if self.goal_weight(r, GoalKind::SecureDynasty, None) <= 0.0 && !self.chance(0.1) {
            return;
        }
        let ruler = self.realms[r as usize].ruler;
        let own = self.chars[ruler as usize].children.iter().copied().chain(std::iter::once(ruler)).find(|c| {
            let ch = &self.chars[*c as usize];
            ch.alive() && ch.spouse.is_none() && ch.adult(self.day) && ch.realm == Some(r)
        });
        let Some(own) = own else { return };
        let neighbors: Vec<RealmId> = self.realms.iter().filter(|x| x.alive && x.id != r && !is_antagonist(x.kind) && self.realm_distance(r, x.id) <= 2 && !self.at_war(r, x.id) && !self.is_player(x.id)).map(|x| x.id).collect();
        for t in neighbors {
            let want = if self.chars[own as usize].gender == Gender::Male { Gender::Female } else { Gender::Male };
            let cand = self.realms[t as usize].court.iter().copied().find(|c| {
                let ch = &self.chars[*c as usize];
                ch.alive() && ch.spouse.is_none() && ch.gender == want && ch.adult(self.day) && ch.age(self.day) < 45
            });
            if let Some(o) = cand {
                let dowry = (self.realms[r as usize].gold * 0.1).min(400.0);
                let res = self.diplomacy(r, t, DiplomaticAction::ProposeMarriage { own, other: o, dowry });
                if res.accepted {
                    break;
                }
            }
        }
    }

    fn ai_succession_law(&mut self, r: RealmId) {
        // Herrscher ohne männliche/ersten Erben: Erbgesetz auf Designation, wenn Legitimität hoch
        let rl = &self.realms[r as usize];
        if rl.laws.succession == SuccLaw::Designation || rl.legitimacy < 70.0 || self.day.saturating_sub(rl.laws.last_succession_change) < 20 * DAYS_PER_YEAR {
            return;
        }
        let heirs = self.chars[rl.ruler as usize].children.iter().filter(|c| self.chars[**c as usize].alive()).count();
        if heirs >= 2 && rl.gov == Government::Feudal && self.chance(0.02) && self.council_approves(r, None) {
            self.realms[r as usize].laws.succession = SuccLaw::Designation;
            self.realms[r as usize].laws.last_succession_change = self.day;
            self.realms[r as usize].prestige -= 500.0;
        }
    }

    // ------------------------------------------------------------------------------------
    // Intrigen der KI
    // ------------------------------------------------------------------------------------
    pub(crate) fn ai_plots_month(&mut self) {
        let active = self.plots.iter().filter(|p| !p.done).count();
        if active >= 14 {
            return;
        }
        let ids: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind) && !self.is_player(r.id)).map(|r| r.id).collect();
        for r in ids {
            let pers = self.realms[r as usize].personality;
            let spymaster = self.realms[r as usize].offices[Office::Spymaster.idx()];
            let leader = spymaster.unwrap_or(self.realms[r as usize].ruler);
            if !self.chars[leader as usize].alive() {
                continue;
            }
            // Außenpolitische Intrigen
            let rival = self.realms[r as usize].goals.iter().find(|g| matches!(g.kind, GoalKind::WeakenRival | GoalKind::Revenge | GoalKind::DestroyNemesis)).and_then(|g| g.target_realm);
            let p = if pers == Personality::Schemer { 0.06 } else { 0.015 };
            if let Some(t) = rival
                && self.realms[t as usize].alive && self.chance(p) {
                    let kind = match self.ri(0, 5) {
                        0 => PlotKind::Slander,
                        1 => PlotKind::Espionage,
                        2 => PlotKind::Incite,
                        3 => PlotKind::Steal,
                        4 => PlotKind::Murder,
                        _ => PlotKind::Bribery,
                    };
                    let target = match kind {
                        PlotKind::Incite => {
                            let ps = self.provinces_of(t).to_vec();
                            ps.first().map(|p| PlotTarget::Prov(*p))
                        }
                        PlotKind::Espionage | PlotKind::Steal => Some(PlotTarget::Realm(t)),
                        PlotKind::Bribery => self.realms[t as usize].court.iter().copied().find(|c| self.chars[*c as usize].house.is_some() && self.chars[*c as usize].alive()).map(PlotTarget::Char),
                        _ => {
                            let heir = self.chars[self.realms[t as usize].ruler as usize].children.iter().copied().find(|c| self.chars[*c as usize].alive());
                            let tchar = if kind == PlotKind::Murder && self.chance(0.5) { heir.unwrap_or(self.realms[t as usize].ruler) } else { self.realms[t as usize].ruler };
                            Some(PlotTarget::Char(tchar))
                        }
                    };
                    if let Some(tg) = target {
                        let _ = self.start_plot(leader, kind, tg, Some(r));
                    }
                }
            // Innenpolitische Intrigen: Putsch ehrgeiziger Häuser
            let houses: Vec<HouseId> = self.houses.iter().filter(|h| h.alive && h.realm == r && (h.throne_claim || h.loyalty < 22.0) && h.power > 6.0).map(|h| h.id).collect();
            for h in houses {
                if self.chance(0.015) {
                    let head = self.houses[h as usize].head;
                    if self.chars[head as usize].alive() {
                        let _ = self.start_plot(head, PlotKind::Usurp, PlotTarget::Throne(r), None);
                    }
                }
            }
            // Heimliche Bündnisse zwischen Schemern
            if pers == Personality::Schemer && self.chance(0.01) {
                let t = self.realms.iter().filter(|x| x.alive && x.id != r && !is_antagonist(x.kind) && self.realm_distance(r, x.id) <= 2 && self.opinion(r, x.id) > 20.0 && !self.has_treaty(TreatyKind::SecretAlliance, r, x.id)).map(|x| x.id).next();
                if let Some(t) = t {
                    let _ = self.start_plot(leader, PlotKind::SecretAlliance, PlotTarget::Realm(t), Some(r));
                }
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Friedensangebote an den Spieler
    // ------------------------------------------------------------------------------------
    pub fn add_peace_offer(&mut self, to: RealmId, from: RealmId, war: WarId, terms: PeaceTerms) {
        if self.peace_offers.iter().any(|o| o.war == war && o.to == to) {
            return;
        }
        let id = self.next_peace_offer;
        self.next_peace_offer += 1;
        self.peace_offers.push(PeaceOffer { id, war, from, to, terms, day: self.day });
        let e = crate::world::StrategyEvent::Notice { realm: Some(to), kind: crate::chronicle::ChronicleKind::Peace, text: format!("{} bietet Frieden an.", self.rname(from)), province: None };
        self.emit(e);
    }

    /// Antwort des Spielers auf ein Friedensangebot.
    pub fn answer_peace_offer(&mut self, id: u32, accept: bool) -> Result<(), String> {
        let idx = self.peace_offers.iter().position(|o| o.id == id).ok_or("Angebot nicht gefunden")?;
        let o = self.peace_offers.remove(idx);
        if accept {
            if self.wars.iter().any(|w| w.id == o.war && w.active) {
                // Die Bedingungen gelten zu Lasten des Spielers (Angebotsteller diktiert)
                self.make_peace(o.war, o.from, o.terms);
            }
        } else {
            self.add_opinion(o.from, o.to, OpKind::Insult, -5.0, Some(self.day + 3 * DAYS_PER_YEAR));
        }
        Ok(())
    }

    fn expire_peace_offers(&mut self) {
        let day = self.day;
        self.peace_offers.retain(|o| day.saturating_sub(o.day) < 90 && self.wars.iter().any(|w| w.id == o.war && w.active));
    }
}

fn army_power_of(a: &Army) -> f32 {
    crate::military::army_power(a)
}
