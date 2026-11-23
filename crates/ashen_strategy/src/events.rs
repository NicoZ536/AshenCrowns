//! Ereignissystem: datengetriebene Ereignisse als Interventionspunkte der Simulation (GDD 15.6).
//! Bedingungen lesen Simulationsvariablen, Optionen verändern ausschließlich Variablen. Die KI wählt
//! Optionen über Gewichte; für den Spieler entstehen Ereignisfenster (`PendingEvent`).

use crate::catalog::Trait;
use crate::chronicle::{ChronicleKind, Entry};
use crate::data::{Cond, Effect, EventDef, EventScope};
use crate::diplomacy::is_antagonist;
use crate::goods::Good;
use crate::model::*;
use crate::types::*;
use crate::world::{PendingEvent, StrategyEvent, StrategyWorld};

#[derive(Clone, Copy)]
struct Ctx {
    realm: RealmId,
    province: Option<ProvId>,
}

impl StrategyWorld {
    // ------------------------------------------------------------------------------------
    // Variablen
    // ------------------------------------------------------------------------------------
    /// Liest eine Simulationsvariable (siehe `docs/tech/strategy_layer.md`).
    pub fn event_var(&self, realm: RealmId, province: Option<ProvId>, name: &str) -> Option<f32> {
        let rl = &self.realms[realm as usize];
        let _ruler = &self.chars[rl.ruler as usize];
        let day = self.day;
        let provs = self.provinces_of(realm);
        let np = provs.len().max(1) as f32;
        let avg = |f: &dyn Fn(&Province) -> f32| -> f32 { provs.iter().map(|p| f(&self.provinces[*p as usize])).sum::<f32>() / np };
        if let Some(pid) = province {
            let p = &self.provinces[pid as usize];
            let v = match name {
                "rainfall_ratio" => p.rainfall,
                "dry_months" => p.dry_months as f32,
                "harvest" => p.harvest,
                "food_ratio" => p.food_ratio,
                "unrest" => p.unrest,
                "satisfaction" => p.avg_satisfaction(),
                "hollowing" => p.hollowing,
                "veil" => p.veil,
                "disease" => p.disease,
                "hygiene" => p.hygiene,
                "pop" => p.population(),
                "devastation" => p.devastation,
                "fortification" => p.fortification as f32,
                "grain_price_ratio" => self.price_in(pid, Good::Grain) / Good::Grain.base_price(),
                "tension_max" => p.tension.iter().cloned().fold(0.0, f32::max),
                "fertility" => p.fertility,
                "forest_cover" => p.forest_cover,
                "refugees" => p.refugees,
                "garrison_ratio" => p.garrison as f32 / p.population().max(1.0) * 100.0,
                "faith_strength" => self.faith_strength(pid),
                "settlement_rank" => p.settlement.rank() as f32,
                "shrine" => p.shrine as f32,
                "hollow_force" => p.hollow_force,
                "scholars" => p.pop[Stratum::Scholars.idx()].total(),
                _ => return self.event_var_realm(realm, name, day),
            };
            return Some(v);
        }
        let _ = avg;
        self.event_var_realm(realm, name, day)
    }

    fn event_var_realm(&self, realm: RealmId, name: &str, day: u32) -> Option<f32> {
        let rl = &self.realms[realm as usize];
        let ruler = &self.chars[rl.ruler as usize];
        let provs = self.provinces_of(realm);
        let np = provs.len().max(1) as f32;
        let avg = |f: &dyn Fn(&Province) -> f32| -> f32 { provs.iter().map(|p| f(&self.provinces[*p as usize])).sum::<f32>() / np };
        let heir_age = ruler.children.iter().filter(|c| self.chars[**c as usize].alive()).map(|c| self.chars[*c as usize].age(day)).max().unwrap_or(0) as f32;
        let debt: f32 = rl.loans.iter().map(|l| l.principal).sum();
        let v = match name {
            "gold" => rl.gold,
            "net_income" => rl.income - rl.expenses,
            "legitimacy" => rl.legitimacy,
            "war_weariness" => rl.war_weariness,
            "tyranny" => rl.tyranny,
            "prestige" => rl.prestige,
            "piety" => rl.piety,
            "provinces" => provs.len() as f32,
            "house_loyalty_min" => self.houses.iter().filter(|h| h.alive && h.realm == realm).map(|h| h.loyalty).fold(100.0, f32::min),
            "debt_ratio" => debt / (rl.income.max(50.0) * 12.0),
            "tax_level" => rl.tax_level as f32,
            "ruler_age" => ruler.age(day) as f32,
            "ruler_health" => ruler.health,
            "ruler_stress" => ruler.stress,
            "ruler_hollowing" => ruler.hollowing,
            "heir_age" => heir_age,
            "regency" => rl.regency.is_some() as u8 as f32,
            "threat" => rl.threat,
            "great_power" => rl.great_power as u8 as f32,
            "hollow_path" => rl.hollow_path as f32,
            "ashglass" => rl.ashglass,
            "satisfaction" => avg(&|p| p.avg_satisfaction()),
            "unrest" => avg(&|p| p.unrest),
            "hollowing" => avg(&|p| p.hollowing),
            "veil" => avg(&|p| p.veil),
            "grain_price_ratio" => {
                let cap = rl.capital;
                self.price_in(cap, Good::Grain) / Good::Grain.base_price()
            }
            "scholars" => provs.iter().map(|p| self.provinces[*p as usize].pop[Stratum::Scholars.idx()].total()).sum::<f32>(),
            "plague_provinces" => provs.iter().filter(|p| self.provinces[**p as usize].plague.is_some()).count() as f32,
            "hungry_provinces" => provs.iter().filter(|p| self.provinces[**p as usize].food_ratio < 0.6).count() as f32,
            "veil_pressure" => self.endgame.veil_pressure,
            "year" => self.year() as f32,
            "at_war" => self.in_any_war(realm) as u8 as f32,
            _ => return None,
        };
        Some(v)
    }

    fn cond_ok(&self, c: &Cond, ctx: Ctx) -> bool {
        let rl = &self.realms[ctx.realm as usize];
        let ruler = &self.chars[rl.ruler as usize];
        match c {
            Cond::VarBelow(n, v) => self.event_var(ctx.realm, ctx.province, n).is_some_and(|x| x < *v),
            Cond::VarAbove(n, v) => self.event_var(ctx.realm, ctx.province, n).is_some_and(|x| x > *v),
            Cond::DryMonths(m) => ctx.province.is_some_and(|p| self.provinces[p as usize].dry_months >= *m),
            Cond::Season(s) => s.contains(&self.season()),
            Cond::HasState(n) => ctx.province.is_some_and(|p| self.provinces[p as usize].has_state(n, self.day)),
            Cond::NotState(n) => ctx.province.is_none_or(|p| !self.provinces[p as usize].has_state(n, self.day)),
            Cond::Chance(_) => true, // wird beim Auslösen gewürfelt
            Cond::Religion(id) => rl.religion.id() == id,
            Cond::Faction(f) => &rl.faction == f,
            Cond::YearFrom(y) => self.year() >= *y,
            Cond::RulerTrait(t) => Trait::from_id(t).is_some_and(|t| ruler.has(t)),
            Cond::RulerNotTrait(t) => Trait::from_id(t).is_none_or(|t| !ruler.has(t)),
            Cond::AtWar(w) => self.in_any_war(ctx.realm) == *w,
            Cond::Biome(b) => ctx.province.is_some_and(|p| format!("{:?}", self.provinces[p as usize].biome) == *b),
            Cond::Coastal => ctx.province.is_some_and(|p| self.provinces[p as usize].coastal),
            Cond::PlayerOnly => self.is_player(ctx.realm),
        }
    }

    fn event_allowed(&self, def: &EventDef, ctx: Ctx) -> bool {
        def.conditions.iter().all(|c| self.cond_ok(c, ctx))
    }

    // ------------------------------------------------------------------------------------
    // Monatsauswertung
    // ------------------------------------------------------------------------------------
    pub(crate) fn events_month(&mut self) {
        let defs = self.data.clone();
        let density = self.options.difficulty.mods().event_density;
        let day = self.day;
        // Offene Fenster des Spielers nach 60 Tagen automatisch entscheiden
        let stale: Vec<u32> = self.events.pending.iter().filter(|p| day.saturating_sub(p.day) > 60).map(|p| p.id).collect();
        for id in stale {
            self.auto_resolve_pending_event(id);
        }
        for def in defs.events.iter() {
            let chance_cond = def.conditions.iter().find_map(|c| if let Cond::Chance(p) = c { Some(*p) } else { None }).unwrap_or(1.0);
            let p_fire = (1.0 - (-1.0 / def.mtth_months.max(0.05)).exp()) * density;
            match def.scope {
                EventScope::Province => {
                    for pi in 0..self.provinces.len() {
                        let Some(o) = self.provinces[pi].owner else { continue };
                        if is_antagonist(self.realms[o as usize].kind) || self.provinces[pi].population() < 30.0 {
                            continue;
                        }
                        let ctx = Ctx { realm: o, province: Some(pi as ProvId) };
                        let key = format!("{}@p{}", def.id, pi);
                        if self.events.cooldowns.get(&key).is_some_and(|d| *d > day) {
                            continue;
                        }
                        if !self.event_allowed(def, ctx) {
                            continue;
                        }
                        if chance_cond < 1.0 && !self.chance(chance_cond) {
                            continue;
                        }
                        if self.chance(p_fire) {
                            self.fire_event(def, ctx, key);
                        }
                    }
                }
                EventScope::Realm | EventScope::Character => {
                    for r in 0..self.realms.len() as RealmId {
                        let rl = &self.realms[r as usize];
                        if !rl.alive || is_antagonist(rl.kind) {
                            continue;
                        }
                        let capital = rl.capital;
                        let ctx = Ctx { realm: r, province: None };
                        let key = format!("{}@r{}", def.id, r);
                        if self.events.cooldowns.get(&key).is_some_and(|d| *d > day) {
                            continue;
                        }
                        if !self.event_allowed(def, ctx) {
                            continue;
                        }
                        if chance_cond < 1.0 && !self.chance(chance_cond) {
                            continue;
                        }
                        if self.chance(p_fire) {
                            // Ereignisse ohne Provinzbezug nutzen die Hauptstadt als Ort
                            let ctx = Ctx { realm: r, province: Some(capital) };
                            self.fire_event(def, ctx, key);
                        }
                    }
                }
            }
        }
    }

    fn format_text(&self, t: &str, ctx: Ctx) -> String {
        let pn = ctx.province.map(|p| self.pname(p)).unwrap_or_default();
        let rn = self.rname(ctx.realm);
        let ruler = self.cname(self.realms[ctx.realm as usize].ruler);
        t.replace("{province}", &pn).replace("{realm}", &rn).replace("{ruler}", &ruler)
    }

    fn fire_event(&mut self, def: &EventDef, ctx: Ctx, key: String) {
        let day = self.day;
        self.events.cooldowns.insert(key, day + def.cooldown_months.max(1) * DAYS_PER_MONTH);
        self.events.fired += 1;
        self.stats.events_fired += 1;
        if let (Some((s, months)), Some(p)) = (&def.sets_state, ctx.province) {
            self.provinces[p as usize].states.insert(s.clone(), day + months * DAYS_PER_MONTH);
        }
        let title = self.format_text(&def.title, ctx);
        // Kausalketten-Anker in der Chronik
        let chain_id = self.event_chronicle(def, ctx, &title);
        let _ = chain_id;
        if self.is_player(ctx.realm) && def.window && self.events.pending.len() < 12 {
            let id = self.events.next_id;
            self.events.next_id += 1;
            let options: Vec<(String, String, bool)> = def.options.iter().map(|o| (o.id.clone(), o.text.clone(), self.option_available(o, ctx))).collect();
            let text = self.format_text(&def.text, ctx);
            self.events.pending.push(PendingEvent { id, event_id: def.id.clone(), realm: ctx.realm, province: ctx.province, title, text, options, day });
            self.emit(StrategyEvent::EventWindow { pending: id });
            return;
        }
        // KI-Entscheidung
        let opt = self.ai_pick_option(def, ctx);
        let effects = def.options[opt].effects.clone();
        for e in effects {
            self.apply_effect(&e, ctx);
        }
    }

    fn option_available(&self, o: &crate::data::EventOption, ctx: Ctx) -> bool {
        self.realms[ctx.realm as usize].gold >= o.cost && o.requires.iter().all(|c| self.cond_ok(c, ctx))
    }

    fn ai_pick_option(&mut self, def: &EventDef, ctx: Ctx) -> usize {
        let mut best = (0usize, f32::MIN);
        for (i, o) in def.options.iter().enumerate() {
            if !self.option_available(o, ctx) {
                continue;
            }
            let mut w = o.ai_weight;
            for (c, d) in &o.ai_when {
                if self.cond_ok(c, ctx) {
                    w += d;
                }
            }
            w += self.rf() * 1.5;
            if w > best.1 {
                best = (i, w);
            }
        }
        best.0
    }

    /// Chronikeintrag eines Ereignisses; speichert Kettenglieder an der Provinz.
    fn event_chronicle(&mut self, def: &EventDef, ctx: Ctx, title: &str) -> Option<u64> {
        if def.chronicle_weight < 28 {
            return None;
        }
        let pid = ctx.province;
        let (cause, tag): (Option<u64>, &'static str) = {
            let ch = pid.map(|p| self.provinces[p as usize].chain).unwrap_or_default();
            match def.id.as_str() {
                "drought_province" => (None, "drought"),
                "bread_price_rises" => (ch.drought, "price"),
                "famine_province" => (ch.price_spike.or(ch.drought), "famine"),
                "bread_riot" | "rebel_demands" => (ch.famine.or(ch.price_spike).or(ch.drought), "unrest"),
                _ => (None, "event"),
            }
        };
        let tpl = if def.id == "drought_province" { "drought" } else { "event_generic" };
        let rn = self.rname(ctx.realm);
        let mut e = Entry::new(ChronicleKind::Disaster, tpl, def.chronicle_weight).realm(ctx.realm).var("a", rn.clone()).var("c", title.to_string()).cause(cause).tag("event");
        if let Some(p) = pid {
            let pn = self.pname(p);
            e = e.prov(p, &pn);
        } else {
            e = e.var("p", rn);
        }
        let id = self.chron(e);
        if let Some(p) = pid {
            let ch = &mut self.provinces[p as usize].chain;
            match tag {
                "drought" => ch.drought = Some(id),
                "price" => ch.price_spike = Some(id),
                "famine" => ch.famine = Some(id),
                "unrest" => ch.unrest = Some(id),
                _ => {}
            }
        }
        Some(id)
    }

    /// Wendet die gewählte Option eines Spieler-Ereignisfensters an.
    pub fn choose_event_option(&mut self, pending_id: u32, option_id: &str) -> Result<(), String> {
        let idx = self.events.pending.iter().position(|p| p.id == pending_id).ok_or("Ereignisfenster nicht gefunden")?;
        let pe = self.events.pending[idx].clone();
        // Fraktionsultimatum
        if let Some(fid) = pe.event_id.strip_prefix("faction_ultimatum:") {
            let fid: u32 = fid.parse().map_err(|_| "Ungültige Fraktion")?;
            self.events.pending.remove(idx);
            self.resolve_faction_ultimatum(pe.realm, fid, option_id == "yield");
            return Ok(());
        }
        let def = self.data.events.iter().find(|d| d.id == pe.event_id).cloned().ok_or("Ereignis unbekannt")?;
        let opt = def.options.iter().find(|o| o.id == option_id).ok_or("Option unbekannt")?.clone();
        let ctx = Ctx { realm: pe.realm, province: pe.province };
        if !self.option_available(&opt, ctx) {
            return Err("Option nicht verfügbar (Gold oder Bedingung)".into());
        }
        self.events.pending.remove(idx);
        for e in &opt.effects {
            self.apply_effect(e, ctx);
        }
        Ok(())
    }

    fn auto_resolve_pending_event(&mut self, id: u32) {
        let Some(idx) = self.events.pending.iter().position(|p| p.id == id) else { return };
        let pe = self.events.pending[idx].clone();
        if let Some(fid) = pe.event_id.strip_prefix("faction_ultimatum:") {
            let fid: u32 = fid.parse().unwrap_or(0);
            self.events.pending.remove(idx);
            self.resolve_faction_ultimatum(pe.realm, fid, false);
            return;
        }
        let Some(def) = self.data.events.iter().find(|d| d.id == pe.event_id).cloned() else {
            self.events.pending.remove(idx);
            return;
        };
        let ctx = Ctx { realm: pe.realm, province: pe.province };
        self.events.pending.remove(idx);
        let opt = self.ai_pick_option(&def, ctx);
        for e in def.options[opt].effects.clone() {
            self.apply_effect(&e, ctx);
        }
    }

    /// Löst ein Ereignis gezielt aus (Mods, Tests, Szenarien). Für Spielerreiche entsteht ein Fenster.
    pub fn force_event(&mut self, event_id: &str, realm: RealmId, province: Option<ProvId>) -> Result<(), String> {
        let def = self.data.events.iter().find(|d| d.id == event_id).cloned().ok_or("Ereignis unbekannt")?;
        let province = province.or_else(|| Some(self.realms[realm as usize].capital));
        let ctx = Ctx { realm, province };
        self.fire_event(&def, ctx, format!("{}@forced", def.id));
        Ok(())
    }

    /// Ereignisoptionen für die Oberfläche: die offenen Fenster.
    pub fn pending_events(&self) -> &[PendingEvent] {
        &self.events.pending
    }

    // ------------------------------------------------------------------------------------
    // Effekte
    // ------------------------------------------------------------------------------------
    fn apply_effect(&mut self, e: &Effect, ctx: Ctx) {
        let r = ctx.realm;
        let day = self.day;
        let prov_list: Vec<ProvId> = match ctx.province {
            Some(p) => vec![p],
            None => self.provinces_of(r).to_vec(),
        };
        let ruler = self.realms[r as usize].ruler;
        match e {
            Effect::AddGold(v) => self.realms[r as usize].gold += *v,
            Effect::AddPrestige(v) => self.realms[r as usize].prestige += *v,
            Effect::AddPiety(v) => self.realms[r as usize].piety += *v,
            Effect::AddLegitimacy(v) => self.realms[r as usize].legitimacy = (self.realms[r as usize].legitimacy + *v).clamp(0.0, 100.0),
            Effect::AddTyranny(v) => self.realms[r as usize].tyranny = (self.realms[r as usize].tyranny + *v).max(0.0),
            Effect::AddUnrest(v) => {
                for p in &prov_list {
                    let pr = &mut self.provinces[*p as usize];
                    pr.unrest = (pr.unrest + *v).clamp(0.0, 100.0);
                }
            }
            Effect::AddSatisfaction(v) => {
                for p in &prov_list {
                    for s in self.provinces[*p as usize].pop.iter_mut() {
                        s.satisfaction = (s.satisfaction + *v).clamp(0.0, 100.0);
                    }
                }
            }
            Effect::AddStock(g, v) => {
                if let Some(g) = Good::from_id(g) {
                    let p = ctx.province.unwrap_or(self.realms[r as usize].capital);
                    let st = &mut self.provinces[p as usize].stock[g.idx()];
                    *st = (*st + *v).max(0.0);
                }
            }
            Effect::PriceFactor(g, f, months) => {
                if let Some(g) = Good::from_id(g) {
                    let p = ctx.province.unwrap_or(self.realms[r as usize].capital);
                    self.set_price_factor(p, g, *f, *months);
                }
            }
            Effect::SetState(s, m) => {
                for p in &prov_list {
                    self.provinces[*p as usize].states.insert(s.clone(), day + *m * DAYS_PER_MONTH);
                }
            }
            Effect::ClearState(s) => {
                for p in &prov_list {
                    self.provinces[*p as usize].states.remove(s);
                }
            }
            Effect::AddHollowing(v) => {
                for p in &prov_list {
                    let pr = &mut self.provinces[*p as usize];
                    pr.hollowing = (pr.hollowing + *v).clamp(0.0, 100.0);
                }
            }
            Effect::AddVeil(v) => {
                for p in &prov_list {
                    let pr = &mut self.provinces[*p as usize];
                    pr.veil = (pr.veil + *v).clamp(0.0, 5.0);
                }
            }
            Effect::AddPopulationPct(v) => {
                for p in &prov_list {
                    let pop = self.provinces[*p as usize].population();
                    self.add_population(*p as usize, pop * *v / 100.0, false);
                }
            }
            Effect::AddFortification(v) => {
                for p in &prov_list {
                    let pr = &mut self.provinces[*p as usize];
                    pr.fortification = (pr.fortification as i32 + *v).clamp(0, 10) as u8;
                }
            }
            Effect::HouseLoyalty(v) => {
                for h in self.houses.iter_mut().filter(|h| h.alive && h.realm == r) {
                    h.loyalty = (h.loyalty + *v).clamp(0.0, 100.0);
                }
            }
            Effect::AddStress(v) => {
                let c = &mut self.chars[ruler as usize];
                c.stress = (c.stress + *v).clamp(0.0, 100.0);
            }
            Effect::AddHealth(v) => {
                let c = &mut self.chars[ruler as usize];
                c.health = (c.health + *v).clamp(0.5, 10.0);
            }
            Effect::GrantTrait(t) => {
                if let Some(t) = Trait::from_id(t)
                    && !self.chars[ruler as usize].traits.contains(&t) {
                        self.chars[ruler as usize].traits.push(t);
                    }
            }
            Effect::SpawnRebels(pct) => {
                if let Some(p) = ctx.province {
                    self.spawn_rebels(p, *pct / 100.0);
                }
            }
            Effect::StartPlague(k, s) => {
                if let Some(p) = ctx.province {
                    self.start_plague(p, *k, *s);
                }
            }
            Effect::AddWarWeariness(v) => {
                let w = &mut self.realms[r as usize].war_weariness;
                *w = (*w + *v).clamp(0.0, 100.0);
            }
            Effect::NeighborOpinion(v) => {
                let ns: Vec<RealmId> = self.realms.iter().filter(|x| x.alive && x.id != r && !is_antagonist(x.kind) && self.realm_distance(r, x.id) == 1).map(|x| x.id).collect();
                for n in ns {
                    let kind = if *v >= 0.0 { OpKind::Envoy } else { OpKind::Insult };
                    self.add_opinion(n, r, kind, *v, Some(day + 5 * DAYS_PER_YEAR));
                }
            }
            Effect::AddDisease(v) => {
                for p in &prov_list {
                    let pr = &mut self.provinces[*p as usize];
                    pr.disease = (pr.disease + *v).clamp(0.0, 100.0);
                }
            }
            Effect::Devastate(v) => {
                for p in &prov_list {
                    let pr = &mut self.provinces[*p as usize];
                    pr.devastation = (pr.devastation + *v).clamp(0.0, 1.0);
                }
            }
            Effect::AddAshglass(v) => self.realms[r as usize].ashglass += *v,
            Effect::AddResearch(v) => {
                let s = self.realms[r as usize].research_focus.unwrap_or(self.preferred_school(r));
                self.realms[r as usize].research[s.idx()].points += *v;
            }
            Effect::SpawnNeutral(k, strength) => {
                let kind = match k.as_str() {
                    "AshWolves" => NeutralKind::AshWolves,
                    "GraveWorms" => NeutralKind::GraveWorms,
                    "Salamanders" => NeutralKind::Salamanders,
                    "MoorLights" => NeutralKind::MoorLights,
                    "Bandits" => NeutralKind::Bandits,
                    "Cultists" => NeutralKind::Cultists,
                    _ => NeutralKind::Bandits,
                };
                if let Some(p) = ctx.province {
                    self.spawn_neutral(kind, p, *strength);
                }
            }
            Effect::AddFaith(rel, v) => {
                if let Some(rel) = RELIGIONS.iter().find(|x| x.id() == rel) {
                    for p in &prov_list {
                        let pr = &mut self.provinces[*p as usize];
                        pr.faith[rel.idx()] = (pr.faith[rel.idx()] + *v).clamp(0.0, 1.0);
                        let s: f32 = pr.faith.iter().sum();
                        for f in pr.faith.iter_mut() {
                            *f /= s.max(0.001);
                        }
                    }
                }
            }
            Effect::AddTension(i, v) => {
                for p in &prov_list {
                    let pr = &mut self.provinces[*p as usize];
                    let idx = (*i as usize).min(4);
                    pr.tension[idx] = (pr.tension[idx] + *v).clamp(0.0, 100.0);
                }
            }
            Effect::AddRefugees(v) => {
                if let Some(p) = ctx.province {
                    let pr = &mut self.provinces[p as usize];
                    pr.refugees = (pr.refugees + *v).max(0.0);
                }
            }
            Effect::Note(t) => {
                let rn = self.rname(r);
                let e = Entry::new(ChronicleKind::Omen, "event_generic", 20).realm(r).var("a", rn.clone()).var("p", rn).var("c", t.clone());
                self.chron(e);
            }
            Effect::AddFertility(v) => {
                for p in &prov_list {
                    let pr = &mut self.provinces[*p as usize];
                    pr.fertility = (pr.fertility + *v).clamp(0.0, 1.0);
                    pr.states.insert("harvest_blessing".into(), day + 12 * DAYS_PER_MONTH);
                }
            }
            Effect::AddRulerHollowing(v) => {
                let c = &mut self.chars[ruler as usize];
                c.hollowing = (c.hollowing + *v).clamp(0.0, 100.0);
            }
            Effect::AddTradeBoost(v) => self.realms[r as usize].gold += *v * 1500.0,
            Effect::RaiseSettlement => {
                for p in &prov_list {
                    if let Some(n) = self.provinces[*p as usize].settlement.next()
                        && n != SettlementLevel::Capital {
                            self.provinces[*p as usize].settlement = n;
                        }
                }
            }
            Effect::SpawnArmyPct(f) => {
                let cap = self.realms[r as usize].capital;
                let roster = crate::catalog::roster(self.realms[r as usize].culture);
                let unit = roster.first().copied().unwrap_or("spearman");
                let n = (200.0 * *f) as u32 + 20;
                let nm = format!("Angeworbene Truppen von {}", self.rname(r));
                self.new_army(ArmyKind::Mercenary, Some(r), cap, vec![Regiment { unit: unit.to_string(), count: n, vet: 1, hp: 1.0 }], &nm);
            }
            Effect::AddSpyNetwork(v) => {
                if let Some(t) = self.realms.iter().find(|x| x.alive && x.id != r && !is_antagonist(x.kind) && self.realm_distance(r, x.id) == 1).map(|x| x.id) {
                    let e = self.realms[r as usize].spy_net.entry(t).or_insert(0.0);
                    *e = (*e + *v).min(100.0);
                }
            }
            Effect::AddFood(v) => {
                let p = ctx.province.unwrap_or(self.realms[r as usize].capital);
                self.provinces[p as usize].stock[Good::Grain.idx()] += *v;
                self.provinces[p as usize].stock[Good::Bread.idx()] += *v * 0.2;
            }
            Effect::AddPopularity(v) => {
                self.realms[r as usize].prestige += *v * 5.0;
                self.realms[r as usize].legitimacy = (self.realms[r as usize].legitimacy + *v * 0.5).min(100.0);
            }
        }
    }
}
