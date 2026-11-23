//! Bevölkerung: acht Schichten, Zufriedenheit, Demografie, Migration, Seuchen, Hunger, Spannungen,
//! Unruhe und Aufstände (GDD Kapitel 9 und 15).

use crate::catalog::PLAGUES;
use crate::chronicle::{ChronicleKind, Entry};
use crate::goods::*;
use crate::model::*;
use crate::types::*;
use crate::world::StrategyWorld;

/// Zielanteile der Schichten je Siedlungsstufe.
fn target_share(s: Stratum, level: SettlementLevel) -> f32 {
    let town = level.rank() as f32;
    match s {
        Stratum::Clergy => 0.025 + town * 0.004,
        Stratum::Scholars => 0.004 + town * 0.008,
        Stratum::Warriors => 0.04,
        Stratum::Nobility => 0.004 + town * 0.001,
        _ => 0.0,
    }
}

impl StrategyWorld {
    /// Anfangsbevölkerung einer Provinz je Siedlungsstufe verteilen.
    pub(crate) fn populate_province(&mut self, pid: ProvId) {
        let jitter = 0.8 + self.rf() * 0.4;
        let p = &self.provinces[pid as usize];
        let base = match p.settlement {
            SettlementLevel::Village => 260.0,
            SettlementLevel::Town => 1500.0,
            SettlementLevel::City => 3800.0,
            SettlementLevel::Capital => 7500.0,
        } * jitter;
        let biome_f = match p.biome {
            Biome::Hollowland => 0.2,
            Biome::Desert | Biome::Ruins | Biome::Volcanic => 0.6,
            Biome::Mountain | Biome::Tundra => 0.8,
            _ => 1.0,
        };
        let total = base * biome_f;
        let level = p.settlement;
        let shares: [f32; 8] = match level {
            SettlementLevel::Village => [0.55, 0.28, 0.05, 0.01, 0.025, 0.005, 0.07, 0.004],
            SettlementLevel::Town => [0.38, 0.27, 0.15, 0.08, 0.035, 0.015, 0.06, 0.006],
            SettlementLevel::City => [0.28, 0.25, 0.20, 0.13, 0.04, 0.03, 0.05, 0.008],
            SettlementLevel::Capital => [0.22, 0.22, 0.23, 0.16, 0.05, 0.04, 0.05, 0.012],
        };
        let p = &mut self.provinces[pid as usize];
        for (i, st) in p.pop.iter_mut().enumerate() {
            let n = total * shares[i];
            st.children = n * 0.25;
            st.adults = n * 0.62;
            st.old = n * 0.13;
            st.satisfaction = 62.0;
            st.basic = 1.0;
            st.comfort = 0.5;
            st.luxury = 0.2;
        }
        p.housing = p.housing_capacity();
        p.garrison = (p.pop[Stratum::Warriors.idx()].adults * 0.5) as u32;
    }

    // ------------------------------------------------------------------------------------
    // Monat
    // ------------------------------------------------------------------------------------
    pub(crate) fn population_month(&mut self) {
        self.population_core();
        self.govern_all();
        self.finances_month();
    }

    /// Demografie, Zufriedenheit, Seuchen, Migration und Spannungen eines Monats.
    pub(crate) fn population_core(&mut self) {
        let day = self.day;
        let n = self.provinces.len();
        // Zufriedenheit zuerst (benötigt Erfüllungsgrade aus der Wirtschaft)
        for pi in 0..n {
            self.satisfaction_of(pi);
        }
        for pi in 0..n {
            self.demography(pi);
        }
        for pi in 0..n {
            self.disease_step(pi);
        }
        self.disease_spread();
        self.migration();
        for pi in 0..n {
            self.tension_step(pi);
        }
        // Flüchtlinge in Bevölkerung übernehmen
        for pi in 0..n {
            let r = self.provinces[pi].refugees;
            if r > 0.0 {
                let taken = (r * 0.5).min((self.provinces[pi].housing_capacity() - self.provinces[pi].population()).max(0.0));
                if taken > 0.0 {
                    self.add_population(pi, taken, true);
                    self.provinces[pi].refugees -= taken;
                    self.provinces[pi].tension[4] = (self.provinces[pi].tension[4] + taken / self.provinces[pi].population().max(1.0) * 80.0).min(100.0);
                }
                self.provinces[pi].refugees *= 0.7;
            }
            let p = &mut self.provinces[pi];
            p.states.retain(|_, v| *v > day);
            // Garnison aus Kriegerschicht
            p.garrison = (p.pop[Stratum::Warriors.idx()].adults * 0.5).max(0.0) as u32;
        }
    }

    /// Fügt Bevölkerung (anteilig nach Struktur) hinzu oder entfernt sie (negativ).
    pub fn add_population(&mut self, pi: usize, amount: f32, refugees: bool) {
        let p = &mut self.provinces[pi];
        let total = p.population();
        if total <= 0.0 && amount < 0.0 {
            return;
        }
        let dist: [f32; 8] = if refugees { [0.45, 0.3, 0.1, 0.05, 0.02, 0.01, 0.06, 0.01] } else { [0.0; 8] };
        for (i, st) in p.pop.iter_mut().enumerate() {
            let share = if refugees { dist[i] } else { st.total() / total.max(1.0) };
            let delta = amount * share;
            let ts = st.total().max(1.0);
            let (c, a, o) = (st.children / ts, st.adults / ts, st.old / ts);
            if delta >= 0.0 {
                st.children += delta * if refugees { 0.25 } else { c };
                st.adults += delta * if refugees { 0.65 } else { a };
                st.old += delta * if refugees { 0.1 } else { o };
            } else {
                st.children = (st.children + delta * c).max(0.0);
                st.adults = (st.adults + delta * a).max(0.0);
                st.old = (st.old + delta * o).max(0.0);
            }
        }
    }

    /// Satzung der Zufriedenheit nach GDD 9.4.
    pub(crate) fn satisfaction_of(&mut self, pi: usize) {
        let day = self.day;
        let p = &self.provinces[pi];
        let owner = p.owner;
        let holder = p.holder();
        let enemy_inside = holder != owner || p.siege.is_some();
        let rl = owner.map(|o| &self.realms[o as usize]);
        let (tax_eff, serf_law, town_law, faith_state, martial, wear, tolerance, legit_rank, ruler_charm, is_cap, ruler_rel) = match rl {
            Some(r) => (
                crate::economy::tax_satisfaction(r.tax_level),
                r.laws.serfdom,
                r.laws.town_rights,
                r.laws.faith_law == 0,
                r.laws.martial_law,
                r.war_weariness,
                r.laws.faith_law,
                {
                    let pr = self.chars[r.ruler as usize].prestige;
                    if pr >= 2000.0 {
                        6.0
                    } else if pr >= 500.0 {
                        4.0
                    } else if pr >= 100.0 {
                        2.0
                    } else {
                        0.0
                    }
                },
                self.chars[r.ruler as usize].attr(Attr::Charisma) as f32,
                r.capital == p.id,
                r.religion,
            ),
            None => (0.0, 1, 1, false, 0, 0.0, 1, 0.0, 0.0, false, p.faith_majority()),
        };
        let faith_match = p.faith[ruler_rel.idx()];
        let overcrowd = ((p.population() / p.housing_capacity().max(1.0)) - 1.0).max(0.0) * 10.0 * 2.0;
        let hollow_malus = p.hollowing / 5.0;
        let plague = p.plague.as_ref().map(|_| 15.0).unwrap_or(0.0);
        let festival = if p.has_state("festival", day) { 10.0 } else { 0.0 };
        let famine = p.food_ratio;
        let mut updates = [0.0f32; 8];
        for s in STRATA {
            let st = &p.pop[s.idx()];
            let mut z = 40.0 + 30.0 * st.basic + 20.0 * st.comfort + 10.0 * st.luxury;
            if st.basic < 0.5 {
                z -= 25.0;
            }
            if famine < 0.8 {
                z -= (0.8 - famine) * 60.0;
            }
            if famine < 0.2 {
                z -= 25.0;
            }
            z += tax_eff;
            z += match s {
                Stratum::Serfs => match serf_law {
                    0 => -10.0,
                    1 => 0.0,
                    _ => 10.0,
                },
                Stratum::Burghers => match town_law {
                    0 => -5.0,
                    1 => 5.0,
                    _ => 15.0,
                },
                _ => 0.0,
            };
            z += match martial {
                1 => -5.0 * 0.5,
                2 => -15.0,
                _ => 0.0,
            };
            if enemy_inside {
                z -= 30.0;
            }
            // Glaube
            if faith_match > 0.5 && p.service_count(ServiceKind::Temple) > 0 {
                z += 10.0;
            } else if faith_state && faith_match < 0.4 && tolerance == 0 {
                z -= 10.0;
            }
            if s == Stratum::Serfs && p.faith[Religion::SunHeresy.idx()] > 0.25 {
                z += 10.0 * p.faith[Religion::SunHeresy.idx()];
            }
            if is_cap {
                z += ruler_charm * 0.5;
            }
            z += legit_rank;
            z += p.variety;
            z -= overcrowd;
            z -= wear / 5.0;
            z -= hollow_malus;
            z -= plague;
            z += festival;
            // Spannungen ≥ 40: −5 für beteiligte Schichten
            let involved = match s {
                Stratum::Serfs | Stratum::Nobility => p.tension[0] >= 40.0,
                Stratum::Burghers => p.tension[1] >= 40.0 || p.tension[3] >= 40.0,
                Stratum::Clergy | Stratum::Scholars => p.tension[2] >= 40.0,
                Stratum::Warriors => p.tension[3] >= 40.0,
                _ => p.tension[4] >= 40.0,
            };
            if involved {
                z -= 5.0;
            }
            updates[s.idx()] = z.clamp(0.0, 100.0);
        }
        let p = &mut self.provinces[pi];
        for s in STRATA {
            let st = &mut p.pop[s.idx()];
            st.satisfaction = st.satisfaction * 0.5 + updates[s.idx()] * 0.5;
        }
    }

    fn demography(&mut self, pi: usize) {
        let p = &mut self.provinces[pi];
        let total = p.population();
        if total <= 0.0 {
            return;
        }
        let cap = p.housing_capacity();
        let food_factor = if p.food_ratio < 0.7 { 0.5 } else if p.food_ratio > 0.99 && p.variety >= 2.0 { 1.2 } else { 1.0 };
        let house_factor = if total >= cap { 0.0 } else { (1.0 - total / cap).min(0.5) * 2.0 };
        let hospital = if p.service_count(ServiceKind::Hospital) > 0 { 0.8 } else { 1.0 };
        let plague_f = p.plague.as_ref().map(|pl| 1.0 + pl.severity * 2.0).unwrap_or(1.0);
        let hunger_f = if p.food_ratio < 0.9 { 1.0 + (0.9 - p.food_ratio) * 12.0 } else { 1.0 };
        let hollow_f = if p.hollowing > 50.0 { 1.3 } else { 1.0 };
        let mut deaths_total = 0.0;
        for st in p.pop.iter_mut() {
            let births = st.adults * 0.07 / 12.0 * food_factor * house_factor;
            let d_child = st.children * 0.015 / 12.0 * plague_f * hunger_f * hospital;
            let d_adult = st.adults * 0.010 / 12.0 * plague_f * hunger_f * hospital * hollow_f;
            let d_old = st.old * 0.08 / 12.0 * plague_f * hunger_f * hospital;
            let grow_up = st.children / 16.0 / 12.0;
            let grow_old = st.adults / 39.0 / 12.0;
            st.children = (st.children + births - d_child - grow_up).max(0.0);
            st.adults = (st.adults + grow_up - d_adult - grow_old).max(0.0);
            st.old = (st.old + grow_old - d_old).max(0.0);
            deaths_total += d_child + d_adult + d_old;
        }
        // Aufstieg und Abstieg zwischen Schichten
        let order = [Stratum::Serfs, Stratum::Freeholders, Stratum::Artisans, Stratum::Burghers];
        for w in 0..3 {
            let (lo, hi) = (order[w], order[w + 1]);
            let z = p.pop[lo.idx()].satisfaction;
            if z >= 60.0 {
                let mv = p.pop[lo.idx()].adults * 0.005 * (z - 60.0) / 20.0;
                // Zielschicht hat Wohnraum und Arbeit? Vereinfacht: Arbeitsplätze der Zielschicht
                let jobs_free = p.labor_demand(hi) > p.labor_supply(hi) * 0.8 || hi == Stratum::Freeholders;
                if jobs_free {
                    p.pop[lo.idx()].adults -= mv;
                    p.pop[hi.idx()].adults += mv;
                }
            } else if z < 30.0 {
                let mv = p.pop[hi.idx()].adults * 0.005;
                p.pop[hi.idx()].adults -= mv;
                p.pop[lo.idx()].adults += mv;
            }
        }
        // Sonderschichten wachsen aus Freibauern/Leibeigenen nach (Gelehrte, Klerus, Krieger, Adel)
        let level = p.settlement;
        let tot = p.population().max(1.0);
        for s in [Stratum::Clergy, Stratum::Scholars, Stratum::Warriors, Stratum::Nobility] {
            let share = p.pop[s.idx()].total() / tot;
            let tgt = target_share(s, level);
            if share < tgt {
                let mv = (tgt - share) * tot * 0.03;
                let src = if p.pop[Stratum::Freeholders.idx()].adults > mv { Stratum::Freeholders } else { Stratum::Serfs };
                if p.pop[src.idx()].adults > mv {
                    p.pop[src.idx()].adults -= mv;
                    p.pop[s.idx()].adults += mv;
                }
            }
        }
        // Stadtwachstum aus Land: Siedlungsstufe Dorf wächst nur über Gebäude
        let _ = deaths_total;
        // Hunger: Chronik
        let (hungry, pid) = (p.food_ratio < 0.5, p.id);
        if hungry && p.population() > 100.0 && !p.has_state("famine_logged", self.day) {
            let day = self.day;
            let p = &mut self.provinces[pi];
            p.states.insert("famine_logged".into(), day + 9 * DAYS_PER_MONTH);
            let name = p.name.clone();
            let owner = p.owner;
            let cause = p.chain.price_spike.or(p.chain.drought);
            let est = (p.population() * 0.03).round();
            let mut e = Entry::new(ChronicleKind::Famine, "famine", 38).prov(pid, &name).num("deaths", est).cause(cause).tag("famine");
            e = e.var("n", format!("{}", est as i64));
            let e = if let Some(o) = owner { e.realm(o) } else { e };
            let id = self.chron(e);
            self.provinces[pi].chain.famine = Some(id);
            self.stats.famines += 1;
        }
    }

    fn disease_step(&mut self, pi: usize) {
        let day = self.day;
        let p = &self.provinces[pi];
        let pop = p.population();
        let wells = p.service_count(ServiceKind::Wells) as f32;
        let hospital = p.service_count(ServiceKind::Hospital) as f32;
        let density = pop / p.housing_capacity().max(1.0);
        let hygiene = (80.0 - density * 25.0 + wells * 8.0 + hospital * 4.0 - if p.stock[Good::Remedies.idx()] > 5.0 { -5.0 } else { 0.0 }).clamp(5.0, 100.0);
        let siege = if p.siege.is_some() { 20.0 } else { 0.0 };
        let mut pressure = p.disease * 0.8 + (100.0 - hygiene) * 0.06 + (1.0 - p.food_ratio).max(0.0) * 14.0 + siege * 0.3 + if p.devastation > 0.3 { 2.0 } else { 0.0 };
        if p.has_state("quarantine", day) {
            pressure *= 0.85;
        }
        let rl = p.owner;
        let mut resist = 1.0;
        if let Some(o) = rl
            && let Some(h) = self.realms[o as usize].offices[Office::HighPriest.idx()]
                && self.chars[h as usize].alive() {
                    resist -= 0.0;
                }
        let p = &mut self.provinces[pi];
        p.hygiene = hygiene;
        p.disease = (pressure * resist).clamp(0.0, 100.0);
        // Seuche beenden
        let mut ended: Option<(f32, u32)> = None;
        if let Some(pl) = &p.plague {
            let months = day.saturating_sub(pl.start_day) / DAYS_PER_MONTH;
            let end_p = 0.12 * (1.0 + months as f32 / 6.0) + if hospital > 0.0 { 0.1 } else { 0.0 };
            // Deterministische Auswertung unten (benötigt RNG)
            ended = Some((end_p, months));
        }
        if let Some((end_p, months)) = ended
            && self.chance(end_p) && months >= 3 {
                let pl = self.provinces[pi].plague.take().unwrap();
                let pop_now = self.provinces[pi].population();
                let est = (pop_now * 0.06 * pl.severity).round();
                let name = self.provinces[pi].name.clone();
                let owner = self.provinces[pi].owner;
                let cause = self.provinces[pi].chain.plague;
                let mut e = Entry::new(ChronicleKind::Plague, "plague_end", 30).prov(pi as ProvId, &name).cause(cause).var("n", format!("{}", est as i64));
                if let Some(o) = owner {
                    e = e.realm(o);
                }
                self.chron(e);
                self.provinces[pi].disease = 25.0;
                return;
            }
        // Ausbruch
        if self.provinces[pi].plague.is_none() && self.provinces[pi].disease > 60.0 {
            let over = (self.provinces[pi].disease - 60.0) / 40.0;
            if self.chance(0.06 + over * 0.2) {
                let kind = if self.provinces[pi].region >= 5 { 0 } else { (self.ri(0, 2)) as u8 };
                let sev = 0.4 + self.rf() * 0.5;
                self.start_plague(pi as ProvId, kind, sev);
            }
        }
    }

    /// Lässt eine Seuche in der Provinz ausbrechen.
    pub fn start_plague(&mut self, pid: ProvId, kind: u8, severity: f32) {
        if self.provinces[pid as usize].plague.is_some() {
            return;
        }
        let day = self.day;
        let cause = self.provinces[pid as usize].chain.famine;
        let name = self.provinces[pid as usize].name.clone();
        let owner = self.provinces[pid as usize].owner;
        let mut e = Entry::new(ChronicleKind::Plague, "plague_start", 36).prov(pid, &name).var("c", PLAGUES[(kind as usize).min(3)]).cause(cause).tag("plague");
        if let Some(o) = owner {
            e = e.realm(o);
        }
        let id = self.chron(e);
        let p = &mut self.provinces[pid as usize];
        p.plague = Some(Plague { kind, start_day: day, severity });
        p.chain.plague = Some(id);
        self.stats.plagues += 1;
    }

    fn disease_spread(&mut self) {
        let n = self.provinces.len();
        for pi in 0..n {
            let Some(pl) = self.provinces[pi].plague.clone() else { continue };
            let nbs: Vec<(ProvId, LinkKind)> = self.provinces[pi].neighbors.iter().map(|l| (l.to, l.kind)).collect();
            for (nb, kind) in nbs {
                if self.provinces[nb as usize].plague.is_some() || self.provinces[nb as usize].has_state("quarantine", self.day) || self.provinces[nb as usize].population() < 50.0 {
                    continue;
                }
                let p = pl.severity * if kind == LinkKind::Sea { 0.1 } else { 0.2 } * (0.5 + self.provinces[nb as usize].disease / 100.0);
                if self.chance(p) && self.provinces[nb as usize].biome != Biome::Hollowland {
                    self.start_plague(nb, pl.kind, (pl.severity * 0.9).max(0.25));
                }
            }
        }
    }

    fn migration(&mut self) {
        let n = self.provinces.len();
        let mut moves: Vec<(usize, usize, f32)> = Vec::new();
        for pi in 0..n {
            let p = &self.provinces[pi];
            let pop = p.population();
            if pop < 80.0 {
                continue;
            }
            let z = p.avg_satisfaction();
            let hunger_push = if p.food_ratio < 0.8 { 20.0 } else { 0.0 };
            let z = z - hunger_push;
            let mut best: Option<(usize, f32)> = None;
            for l in &p.neighbors {
                let q = &self.provinces[l.to as usize];
                if q.population() < 20.0 || q.population() >= q.housing_capacity() * 0.98 || q.biome == Biome::Hollowland {
                    continue;
                }
                if let (Some(a), Some(b)) = (p.owner, q.owner)
                    && a != b && self.at_war(a, b) {
                        continue;
                    }
                let zq = q.avg_satisfaction() + (q.food_ratio - p.food_ratio).max(0.0) * 40.0;
                if best.is_none_or(|b| zq > b.1) {
                    best = Some((l.to as usize, zq));
                }
            }
            if let Some((q, zq)) = best
                && zq - z > 6.0 {
                    let amount = (zq - z) * 0.0005 * pop;
                    moves.push((pi, q, amount.min(pop * 0.02)));
                }
        }
        for (a, b, amt) in moves {
            self.add_population(a, -amt, false);
            self.add_population(b, amt, false);
        }
    }

    fn tension_step(&mut self, pi: usize) {
        let day = self.day;
        let owner = self.provinces[pi].owner;
        let (serf, town, magic, wear_unpaid) = match owner {
            Some(o) => {
                let r = &self.realms[o as usize];
                let unpaid = self.armies.iter().any(|a| a.owner == Some(o) && a.unpaid_months > 0);
                (r.laws.serfdom, r.laws.town_rights, r.laws.magic_law, unpaid)
            }
            None => (1, 1, 1, false),
        };
        let tax = owner.map(|o| self.realms[o as usize].tax_level).unwrap_or(2);
        let trade_tax = owner.map(|o| self.realms[o as usize].trade_tax).unwrap_or(5.0);
        let occupied = self.provinces[pi].holder() != owner;
        let p = &mut self.provinces[pi];
        let zs = |p: &Province, s: Stratum| p.pop[s.idx()].satisfaction;
        let mut d = [0.0f32; 5];
        if p.food_ratio < 0.8 {
            d[0] += 5.0;
        }
        d[0] += match serf {
            0 => 2.0,
            1 => 0.0,
            _ => -2.0,
        };
        if tax >= 3 {
            d[0] += 3.0;
        }
        if zs(p, Stratum::Nobility) - zs(p, Stratum::Serfs) > 30.0 {
            d[0] += 3.0;
        }
        if p.has_state("famine_relief", day) || p.has_state("festival", day) {
            d[0] -= 4.0;
        }
        d[1] += match town {
            0 => 2.0,
            1 => 0.0,
            _ => -3.0,
        };
        if trade_tax > 10.0 {
            d[1] += 2.0;
        }
        d[2] += match magic {
            2 => 2.0,
            0 => 0.5,
            _ => -1.0,
        };
        if p.building_count(building_index("ashglass_cutter").unwrap_or(0)) > 0 {
            d[2] += 1.0;
        }
        if p.faith[Religion::SunHeresy.idx()] > 0.2 {
            d[2] += 1.0;
        }
        if wear_unpaid {
            d[3] += 3.0;
        }
        if occupied {
            d[3] += 2.0;
        }
        let ref_share = p.refugees / p.population().max(1.0);
        if ref_share > 0.1 || p.tension[4] > 30.0 && p.food_ratio < 0.9 {
            d[4] += 3.0;
        }
        if p.service_count(ServiceKind::Temple) > 0 {
            d[4] -= 1.0;
        }
        // Spannung wächst durch Treiber und baut sich zu 5 % pro Monat plus 1 Punkt natürlich ab
        for i in 0..5 {
            p.tension[i] = (p.tension[i] + d[i] - 1.0 - p.tension[i] * 0.05).clamp(0.0, 100.0);
        }
        // Streik: ab 60
        let strong = p.tension.iter().cloned().fold(0.0f32, f32::max);
        if strong >= 60.0 && !p.has_state("strike", day) && self.chance(0.1) {
            self.provinces[pi].states.insert("strike".into(), day + 30);
        }
        if strong >= 75.0 && self.chance(0.2) {
            // Unruhen: Lagerplünderung, Brandstiftung
            let p = &mut self.provinces[pi];
            for g in [Good::Grain, Good::Bread, Good::Wood, Good::Planks] {
                p.stock[g.idx()] *= 0.9;
            }
            p.unrest = (p.unrest + 5.0).min(100.0);
        }
    }

    // ------------------------------------------------------------------------------------
    // Ordnung: Unruhe, Aufstände, Adelsrebellionen
    // ------------------------------------------------------------------------------------
    pub(crate) fn order_month(&mut self) {
        let n = self.provinces.len();
        let day = self.day;
        for pi in 0..n {
            let owner = self.provinces[pi].owner;
            let Some(o) = owner else { continue };
            if matches!(self.realms[o as usize].kind, RealmKind::Hollowed | RealmKind::Legion) {
                continue;
            }
            let steward_calm = {
                let r = &self.realms[o as usize];
                match r.offices[Office::Steward.idx()] {
                    Some(c) if r.office_task[Office::Steward.idx()] == 1 && self.chars[c as usize].alive() => self.chars[c as usize].attr(Attr::Stewardship) as f32 * 0.3,
                    _ => 0.0,
                }
            };
            let sep = self.endgame.great_power == Some(o) && self.provinces[pi].culture != self.realms[o as usize].culture;
            let army_pow: f32 = self.armies.iter().filter(|a| a.location == pi as ProvId && a.owner == Some(o) && a.kind == ArmyKind::Realm).map(|a| a.size() as f32).sum();
            let p = &mut self.provinces[pi];
            let pop = p.population().max(1.0);
            let avgz = p.avg_satisfaction();
            let tens: f32 = p.tension.iter().sum();
            let garrison = (p.garrison as f32 + army_pow) / pop * 100.0;
            let mut du = (50.0 - avgz) / 10.0 + tens / 20.0 - garrison * 0.6 - steward_calm;
            if sep {
                du += 1.0;
            }
            if p.has_state("riot_calmed", day) {
                du -= 2.0;
            }
            p.unrest = (p.unrest + du).clamp(0.0, 100.0);
            let unrest = p.unrest;
            // Unruhe-Chronik (Kettenglied)
            if unrest > 55.0 && !p.has_state("unrest_logged", day) && p.population() > 100.0 {
                p.states.insert("unrest_logged".into(), day + 10 * DAYS_PER_MONTH);
                let name = p.name.clone();
                let cause = p.chain.famine.or(p.chain.price_spike).or(p.chain.drought);
                let e = Entry::new(ChronicleKind::Revolt, "unrest", 26).prov(pi as ProvId, &name).realm(o).cause(cause).tag("unrest");
                let id = self.chron(e);
                self.provinces[pi].chain.unrest = Some(id);
            }
            // Aufstandsprüfung
            let p = &self.provinces[pi];
            let ptot = p.population().max(1.0);
            let min_z = STRATA.iter().filter(|s| p.pop[s.idx()].total() > 30.0 && p.pop[s.idx()].total() / ptot >= 0.12).map(|s| p.pop[s.idx()].satisfaction).fold(100.0f32, f32::min);
            let tension_max = p.tension.iter().cloned().fold(0.0f32, f32::max);
            let mut chance = 0.0;
            if min_z < 20.0 {
                chance = (20.0 - min_z) * 0.01 * (1.0 + unrest / 100.0) - p.garrison as f32 / pop * 0.5;
            }
            if tension_max >= 90.0 || unrest >= 92.0 {
                chance = chance.max(0.5);
            }
            let rebelling = self.armies.iter().any(|a| a.kind == ArmyKind::Rebels && a.location == pi as ProvId);
            if chance > 0.0 && !rebelling && self.provinces[pi].population() > 120.0 && self.chance(chance) {
                self.spawn_rebels(pi as ProvId, 0.1);
            }
            // Rebellen halten die Provinz lange → Unabhängigkeit
            if self.provinces[pi].rebel_held && day.saturating_sub(self.provinces[pi].captured_day) > 10 * DAYS_PER_MONTH {
                self.rebel_independence(pi as ProvId);
            }
        }
    }

    /// Rebellen entstehen: 10 % der Bevölkerung (Schicht mit geringster Zufriedenheit zählt), mindestens 30.
    pub fn spawn_rebels(&mut self, pid: ProvId, pct: f32) {
        let day = self.day;
        let p = &self.provinces[pid as usize];
        let owner = p.owner;
        let name = p.name.clone();
        let ptot = p.population().max(1.0);
        let worst = STRATA.iter().copied().filter(|s| p.pop[s.idx()].total() > 30.0 && p.pop[s.idx()].total() / ptot >= 0.12).min_by(|a, b| p.pop[a.idx()].satisfaction.total_cmp(&p.pop[b.idx()].satisfaction)).unwrap_or(Stratum::Serfs);
        let size = (p.pop[worst.idx()].adults * pct).max(p.population() * 0.025).clamp(30.0, 900.0) as u32;
        let cause = p.chain.unrest.or(p.chain.famine).or(p.chain.price_spike).or(p.chain.drought);
        let regs = vec![Regiment { unit: "worker".into(), count: size * 2 / 3, vet: 0, hp: 1.0 }, Regiment { unit: "spearman".into(), count: size - size * 2 / 3, vet: 0, hp: 1.0 }];
        let id = self.new_army(ArmyKind::Rebels, None, pid, regs, "Aufständische");
        if let Some(a) = self.armies.iter_mut().find(|a| a.id == id) {
            a.target_realm = owner;
            a.order = ArmyOrder::Siege;
            a.morale = 75.0;
        }
        self.add_population(pid as usize, -(size as f32) * 0.5, false);
        let on = owner.map(|o| self.rname(o)).unwrap_or_default();
        let mut e = Entry::new(ChronicleKind::Revolt, "revolt", (20 + size / 20).min(60) as u8).prov(pid, &name).var("b", on).var("n", size.to_string()).cause(cause).tag("revolt").num("rebels", size as f32);
        if let Some(o) = owner {
            e = e.realm(o);
        }
        let cid = self.chron(e);
        self.provinces[pid as usize].chain.revolt = Some(cid);
        self.provinces[pid as usize].unrest = (self.provinces[pid as usize].unrest - 15.0).max(0.0);
        self.stats.revolts += 1;
        let _ = day;
    }

    fn rebel_independence(&mut self, pid: ProvId) {
        let culture = self.provinces[pid as usize].culture;
        let fam = self.random_family_name(culture);
        let name = self.provinces[pid as usize].name.clone();
        let old = self.provinces[pid as usize].owner;
        let d = self.create_dynasty(&fam, culture);
        let g = if self.chance(0.5) { Gender::Male } else { Gender::Female };
        let age = 28 + self.ri(0, 14);
        let ruler = self.spawn_person(None, culture, d, g, age, None, None, pid);
        self.chars[ruler as usize].realm = None;
        let nm = format!("Freie Gemeinde {name}");
        let r = self.found_realm(&nm, 0, ruler, vec![pid], None);
        // Rebellenarmee in die neue Armee eingliedern
        let ids: Vec<usize> = self.armies.iter().enumerate().filter(|(_, a)| a.kind == ArmyKind::Rebels && a.location == pid).map(|(i, _)| i).collect();
        for i in ids {
            self.armies[i].kind = ArmyKind::Realm;
            self.armies[i].owner = Some(r);
            self.armies[i].target_realm = None;
            self.armies[i].order = ArmyOrder::Garrison;
        }
        self.provinces[pid as usize].rebel_held = false;
        if let Some(o) = old {
            let e = Entry::new(ChronicleKind::Realm, "rebel_state", 50).prov(pid, &name).realm(r).var("a", nm).var("b", self.rname(o)).cause(self.provinces[pid as usize].chain.revolt);
            self.chron(e);
            self.add_opinion(r, o, OpKind::WarAgainst, -40.0, Some(self.day + 20 * DAYS_PER_YEAR));
        }
    }
}
