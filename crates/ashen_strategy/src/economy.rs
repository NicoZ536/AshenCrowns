//! Wirtschaft: Produktionsketten auf Provinzebene, regionale Märkte mit Preisen aus Angebot und
//! Nachfrage, Handelsrouten, Zölle, Jahreszeiten, Knappheit, Steuern und Kredite (GDD Kapitel 8).

use crate::goods::*;
use crate::mapgen::MARKET_COUNT;
use crate::model::*;
use crate::types::*;
use crate::world::StrategyWorld;

/// Sekunden RTS-Zeit pro Welt-Monat (1 Tag = 8 s).
const SECS_PER_MONTH: f32 = 240.0;
/// Steuergewicht (Kalibrierung gegen die Ausgabenseite).
pub const TAX_SCALE: f32 = 0.30;

pub fn tax_multiplier(level: u8) -> f32 {
    match level {
        0 => 0.0,
        1 => 0.5,
        2 => 1.0,
        3 => 1.5,
        _ => 2.0,
    }
}

/// Zufriedenheitswirkung der Steuerstufe.
pub fn tax_satisfaction(level: u8) -> f32 {
    match level {
        0 => 8.0,
        1 => 5.0,
        2 => 0.0,
        3 => -8.0,
        _ => -20.0,
    }
}

fn productivity(z: f32) -> f32 {
    if z < 20.0 {
        0.5
    } else if z < 40.0 {
        0.8
    } else if z < 60.0 {
        1.0
    } else if z < 80.0 {
        1.1
    } else {
        1.2
    }
}

fn is_crop(g: Good) -> bool {
    matches!(g, Good::Grain | Good::Flax | Good::Hops | Good::Grapes)
}

impl Province {
    /// Arbeitsplatzbedarf einer Schicht.
    pub fn labor_demand(&self, s: Stratum) -> f32 {
        self.buildings.iter().map(|(i, c)| {
            let b = &BUILDINGS[*i as usize];
            if b.stratum == s {
                b.workers as f32 * *c as f32
            } else {
                0.0
            }
        }).sum()
    }
    pub fn labor_supply(&self, s: Stratum) -> f32 {
        self.pop[s.idx()].labor() * 0.92
    }
    pub fn service_count(&self, k: ServiceKind) -> u16 {
        self.building_count(service_index(k))
    }
    pub fn housing_capacity(&self) -> f32 {
        self.settlement.housing() + self.service_count(ServiceKind::Housing) as f32 * 120.0
    }
    /// Monatlicher Nahrungsbedarf (Nährwerte) der Bevölkerung.
    pub fn food_need(&self) -> f32 {
        let mut f = 0.0;
        for s in STRATA {
            let n100 = self.pop[s.idx()].total() / 100.0;
            for nd in needs_of(s) {
                if nd.kind == NeedKind::Food {
                    f += nd.per100 * n100;
                }
            }
        }
        f
    }

    /// Anteil der Nahrungsvorräte, der über der Reserve von drei Monatsrationen liegt.
    pub fn food_free_fraction(&self, food_need: f32) -> f32 {
        let total: f32 = FOODS.iter().map(|g| self.stock[g.idx()] * g.nutrition()).sum();
        if total <= 0.0 {
            return 0.0;
        }
        (1.0 - food_need * 3.0 / total).clamp(0.0, 1.0)
    }

    pub fn workshop_input_demand(&self, g: Good) -> f32 {
        let mut d = 0.0;
        for (i, c) in &self.buildings {
            let b = &BUILDINGS[*i as usize];
            for (ig, amt) in b.inputs {
                if *ig == g {
                    d += *amt * (SECS_PER_MONTH / b.time) * *c as f32;
                }
            }
        }
        d
    }
}

impl StrategyWorld {
    // ------------------------------------------------------------------------------------
    // Wetter
    // ------------------------------------------------------------------------------------
    pub(crate) fn weather_month(&mut self) {
        let day = self.day;
        for i in 0..self.provinces.len() {
            // Autokorrelation: Dürren dauern
            let noise = (self.rf() + self.rf() + self.rf() - 1.5) * 0.9; // ≈ ±0.7, Std ≈ 0.45
            let p = &mut self.provinces[i];
            let mean = match p.biome {
                Biome::Desert => 0.85,
                Biome::Swamp => 1.1,
                _ => 1.0,
            };
            let target = mean + noise;
            let mut r = p.rainfall * 0.55 + target * 0.45;
            if p.has_state("drought_override", day) {
                r = r.min(0.45);
            }
            p.rainfall = r.clamp(0.05, 2.0);
            if p.rainfall < 0.6 {
                p.dry_months = p.dry_months.saturating_add(1);
            } else {
                p.dry_months = 0;
            }
            // Ernte-Index
            let rain_factor = (0.25 + 0.75 * p.rainfall.min(1.3)).clamp(0.2, 1.3);
            p.harvest = (p.harvest * 0.6 + rain_factor * (1.0 - p.devastation * 0.8) * 0.4).clamp(0.1, 1.5);
            p.devastation = (p.devastation - 1.0 / 24.0).max(0.0);
        }
    }

    // ------------------------------------------------------------------------------------
    // Monat: Produktion, Verbrauch, Märkte
    // ------------------------------------------------------------------------------------
    pub(crate) fn economy_month(&mut self) {
        for r in self.realms.iter_mut() {
            r.income = 0.0;
        }
        let month = self.month();
        let season = self.season();
        let day = self.day;
        let n = self.provinces.len();
        let gc = Good::COUNT;
        // Realmbezogene Faktoren vorab
        let mut realm_factor = vec![1.0f32; self.realms.len()];
        for r in 0..self.realms.len() {
            let mut f = 1.0;
            let steward = self.realms[r].offices[Office::Steward.idx()];
            if let Some(s) = steward
                && self.chars[s as usize].alive() {
                    f += (self.chars[s as usize].attr(Attr::Stewardship) as f32 - 10.0) * 0.01;
                    if self.realms[r].office_task[Office::Steward.idx()] == 2 {
                        f += 0.0; // Erntepflege wirkt unten auf Nahrung
                    }
                }
            let diff = self.options.difficulty.mods();
            f *= if self.is_player(r as RealmId) { diff.player_yield } else { diff.ai_yield };
            if self.realms[r].hollow_path >= 2 {
                f += 0.0;
            }
            realm_factor[r] = f;
        }
        let steward_food: Vec<f32> = (0..self.realms.len())
            .map(|r| {
                let s = self.realms[r].offices[Office::Steward.idx()];
                match s {
                    Some(c) if self.realms[r].office_task[Office::Steward.idx()] == 2 && self.chars[c as usize].alive() => 1.0 + self.chars[c as usize].attr(Attr::Stewardship) as f32 * 0.01,
                    _ => 1.0,
                }
            })
            .collect();

        // 1. Produktion und lokaler Verbrauch
        let mut cons_req: Vec<Vec<f32>> = vec![vec![0.0; gc]; n];
        let mut input_req: Vec<Vec<f32>> = vec![vec![0.0; gc]; n];
        let mut need_total: Vec<Vec<f32>> = vec![vec![0.0; gc]; n];
        let mut food_need: Vec<f32> = vec![0.0; n];
        let mut food_eaten: Vec<f32> = vec![0.0; n];
        let mut food_kinds: Vec<[f32; 6]> = vec![[0.0; 6]; n];
        let mut consumed: Vec<Vec<f32>> = vec![vec![0.0; gc]; n];
        let mut ashglass_mined = vec![0.0f32; n];
        for pi in 0..n {
            let (owner, holder) = (self.provinces[pi].owner, self.provinces[pi].holder());
            let rf = owner.map(|o| realm_factor[o as usize]).unwrap_or(1.0);
            let food_boost = owner.map(|o| steward_food[o as usize]).unwrap_or(1.0);
            let occupied = holder != owner || self.provinces[pi].rebel_held;
            let occ_f = if occupied || self.provinces[pi].siege.is_some() { 0.3 } else { 1.0 };
            let ashgl_bonus = owner.map(|o| if self.realms[o as usize].hollow_path >= 2 { 1.5 } else { 1.0 }).unwrap_or(1.0);
            let blessing = self.provinces[pi].has_state("harvest_blessing", day);
            let p = &mut self.provinces[pi];
            for v in p.produced.iter_mut() {
                *v = 0.0;
            }
            for v in p.unmet.iter_mut() {
                *v = 0.0;
            }
            // Lagerverlust
            for g in Good::ALL {
                let d = g.decay();
                if d > 0.0 {
                    p.stock[g.idx()] *= 1.0 - d;
                }
            }
            // Arbeitskräfte
            let mut staffing = [1.0f32; 8];
            let mut prod = [1.0f32; 8];
            for s in STRATA {
                let demand = p.labor_demand(s);
                let supply = p.labor_supply(s);
                staffing[s.idx()] = if demand <= 0.0 { 1.0 } else { (supply / demand).min(1.0) };
                prod[s.idx()] = productivity(p.pop[s.idx()].satisfaction);
            }
            let buildings = p.buildings.clone();
            for (bi, cnt) in buildings {
                let b = &BUILDINGS[bi as usize];
                if !matches!(b.kind, BKind::Production) || cnt == 0 {
                    continue;
                }
                let cycles_month = SECS_PER_MONTH / b.time;
                let mut cap = cycles_month * cnt as f32 * staffing[b.stratum.idx()] * prod[b.stratum.idx()] * rf * occ_f;
                // Vorkommen
                let out_good = b.outputs[0].0;
                if let Some(dep) = b.deposit {
                    let rich = p.deposit(dep) as f32;
                    if rich <= 0.0 {
                        continue;
                    }
                    cap *= rich / 3.0;
                    if dep == Good::Grain {
                        cap *= p.fertility.max(0.2) * p.harvest * 1.15 * food_boost;
                        if blessing {
                            cap *= 1.2;
                        }
                    }
                    if is_crop(out_good) && dep != Good::Grain {
                        cap *= p.harvest;
                    }
                    if dep == Good::Fish {
                        cap *= 0.8 + 0.2 * p.harvest;
                    }
                    if dep == Good::AshglassRaw {
                        cap *= ashgl_bonus;
                    }
                }
                if is_crop(out_good) {
                    cap *= season.crop_factor(month);
                }
                if cap <= 0.0 {
                    continue;
                }
                // Eingaben
                let mut cycles = cap;
                for (ig, amt) in b.inputs {
                    cycles = cycles.min(p.stock[ig.idx()] / *amt);
                }
                let cycles = cycles.max(0.0);
                if cycles < cap {
                    for (ig, amt) in b.inputs {
                        input_req[pi][ig.idx()] += (cap - cycles) * *amt;
                    }
                }
                for (ig, amt) in b.inputs {
                    p.stock[ig.idx()] -= cycles * *amt;
                }
                for (og, amt) in b.outputs {
                    p.stock[og.idx()] += cycles * *amt;
                    p.produced[og.idx()] += cycles * *amt;
                    if *og == Good::AshglassRaw {
                        ashglass_mined[pi] += cycles * *amt;
                    }
                }
            }
            // 2. Verbrauch (Bedürfnisse)
            let pop_total = p.population();
            let mut item_need = vec![0.0f32; gc];
            let mut f_need = 0.0;
            for s in STRATA {
                let n100 = p.pop[s.idx()].total() / 100.0;
                for nd in needs_of(s) {
                    match nd.kind {
                        NeedKind::Food => f_need += nd.per100 * n100,
                        NeedKind::Item(g) => item_need[g.idx()] += nd.per100 * n100,
                        _ => {}
                    }
                }
            }
            food_need[pi] = f_need;
            // Nahrungskonto: beste Nahrung zuerst, aber Vielfalt: Anteile pro Nahrungsart
            let mut remaining = f_need;
            for (k, g) in FOODS.iter().enumerate() {
                if remaining <= 0.0 {
                    break;
                }
                let nut = g.nutrition();
                let avail = p.stock[g.idx()] * nut;
                // Brei (Getreide) nur, wenn nichts Besseres da ist: nicht mehr als nötig
                let take_nut = avail.min(remaining);
                if take_nut > 0.0 {
                    p.stock[g.idx()] -= take_nut / nut;
                    consumed[pi][g.idx()] += take_nut / nut;
                    food_kinds[pi][k] += take_nut;
                    remaining -= take_nut;
                }
            }
            food_eaten[pi] = f_need - remaining;
            cons_req[pi][Good::Grain.idx()] += 0.0; // Nahrung wird über `food_need` gehandelt
            for g in Good::ALL {
                let need = item_need[g.idx()];
                need_total[pi][g.idx()] = need;
                if need <= 0.0 {
                    continue;
                }
                let take = p.stock[g.idx()].min(need);
                p.stock[g.idx()] -= take;
                consumed[pi][g.idx()] += take;
                cons_req[pi][g.idx()] = need - take;
            }
            let _ = pop_total;
        }

        // Aschglasabbau → Schleier
        for pi in 0..n {
            if ashglass_mined[pi] > 0.0 {
                let rate = self.options.difficulty.mods().veil_rate;
                self.provinces[pi].veil = (self.provinces[pi].veil + 0.002 * ashglass_mined[pi] * rate).min(5.0);
                self.provinces[pi].hollowing = (self.provinces[pi].hollowing + 0.05 + ashglass_mined[pi] * 0.01).min(100.0);
            }
        }

        // 3. Marktabgleich je Region
        let regions = self.regions.clone();
        for reg in &regions {
            let Some(mi) = reg.market else { continue };
            self.clear_market(mi, &reg.provinces, &mut cons_req, &mut input_req, &food_need, &mut food_eaten, &mut food_kinds, &need_total, &mut consumed);
        }
        // Provinzen ohne Markt (Schleiernarbe): nur lokal
        // 4. Erfüllungsgrade und Nahrungsbilanz
        for pi in 0..n {
            self.fulfil_needs(pi, &need_total[pi], &consumed[pi], food_need[pi], food_eaten[pi], &food_kinds[pi], &cons_req[pi]);
            let p = &mut self.provinces[pi];
            for g in 0..gc {
                p.unmet[g] = cons_req[pi][g] + input_req[pi][g];
            }
        }
        // Rundungsfehler beseitigen
        for p in self.provinces.iter_mut() {
            for s in p.stock.iter_mut() {
                if *s < 0.0 || !s.is_finite() {
                    *s = 0.0;
                }
            }
        }
        // 5. Handelsrouten
        self.trade_month();
        // Bildung der Preis-Chronik (Preissprung)
        self.price_watch();
    }

    #[allow(clippy::too_many_arguments)]
    fn clear_market(&mut self, mi: usize, provs: &[ProvId], cons_req: &mut [Vec<f32>], input_req: &mut [Vec<f32>], food_need: &[f32], food_eaten: &mut [f32], food_kinds: &mut [[f32; 6]], need_total: &[Vec<f32>], consumed: &mut [Vec<f32>]) {
        let gc = Good::COUNT;
        let day = self.day;
        // Reserve und freier Bestand je Provinz
        let mut free = vec![vec![0.0f32; gc]; provs.len()];
        let mut sum_free = vec![0.0f32; gc];
        let mut produced = vec![0.0f32; gc];
        let mut n_demand = vec![0.0f32; gc];
        for (k, &pid) in provs.iter().enumerate() {
            let p = &self.provinces[pid as usize];
            for g in Good::ALL {
                let gi = g.idx();
                let reserve = need_total[pid as usize][gi] + p.workshop_input_demand(*g) * 1.0;
                let mut f = (p.stock[gi] - reserve).max(0.0);
                if FOODS.contains(g) {
                    f = p.stock[gi] * p.food_free_fraction(food_need[pid as usize]);
                }
                free[k][gi] = f;
                sum_free[gi] += f;
                produced[gi] += p.produced[gi];
                n_demand[gi] += need_total[pid as usize][gi] + p.workshop_input_demand(*g);
            }
        }
        // Nahrungsnachfrage (in Nährwerten) nach Verfügbarkeit auf Güter verteilen
        let total_food_need: f32 = provs.iter().map(|p| food_need[*p as usize]).sum();
        let food_goods: Vec<Good> = FOODS.to_vec();
        let avail_nut: f32 = food_goods.iter().map(|g| (produced[g.idx()] + self.markets[mi].pool[g.idx()] + 0.2 * sum_free[g.idx()]) * g.nutrition()).sum();
        for g in &food_goods {
            let gi = g.idx();
            let a = (produced[gi] + self.markets[mi].pool[gi] + 0.2 * sum_free[gi]) * g.nutrition();
            let share = if avail_nut > 0.0 { a / avail_nut } else if *g == Good::Grain { 1.0 } else { 0.0 };
            n_demand[gi] += total_food_need * share / g.nutrition();
        }
        // Preise
        let m = &mut self.markets[mi];
        for g in Good::ALL {
            let gi = g.idx();
            let a = produced[gi] + m.pool[gi] + 0.2 * sum_free[gi];
            let nd = n_demand[gi];
            m.supply[gi] = a;
            m.demand[gi] = nd;
            let ratio = if nd < 0.5 {
                0.25
            } else if a < 0.01 {
                4.0
            } else {
                (nd / a).powf(0.7).clamp(0.25, 4.0)
            };
            let (ef, until) = m.event_factor[gi];
            let efac = if until > day { ef } else { 1.0 };
            let target = g.base_price() * ratio * (1.0 + m.inflation) * efac;
            let old = m.prices[gi].max(0.01);
            m.prices[gi] = target.clamp(old * 0.75, old * 1.25);
        }
        // Verteilung: Nachfrage der Provinzen aus freien Beständen
        // Pool zählt als frei verfügbar (Importe)
        for g in Good::ALL {
            let gi = g.idx();
            let pool = self.markets[mi].pool[gi];
            sum_free[gi] += pool;
            self.markets[mi].shortage[gi] = 0.0;
        }
        // Nahrungsanfrage: Defizit je Provinz in Nährwerten
        let mut deficits: Vec<(usize, f32)> = Vec::new();
        for (k, &pid) in provs.iter().enumerate() {
            let d = food_need[pid as usize] - food_eaten[pid as usize];
            if d > 0.01 {
                deficits.push((k, d));
            }
        }
        let tot_def: f32 = deficits.iter().map(|d| d.1).sum();
        if tot_def > 0.0 {
            // Verfügbare Nahrung: günstigste je Nährwert zuerst
            let mut order = food_goods.clone();
            order.sort_by(|a, b| (self.markets[mi].prices[a.idx()] / a.nutrition()).total_cmp(&(self.markets[mi].prices[b.idx()] / b.nutrition())));
            // bunt mischen: Brot/Fleisch/Fisch/Pökel/Wild bevorzugt vor Getreide für Vielfalt
            let mut remaining_def = tot_def;
            let mut delivered_nut = 0.0;
            let mut by_good: Vec<(Good, f32)> = Vec::new();
            for g in order {
                if remaining_def <= 0.0 {
                    break;
                }
                let gi = g.idx();
                let avail_nut = sum_free[gi] * g.nutrition();
                let take_nut = avail_nut.min(remaining_def);
                if take_nut > 0.0 {
                    by_good.push((g, take_nut / g.nutrition()));
                    remaining_def -= take_nut;
                    delivered_nut += take_nut;
                }
            }
            let fill = (delivered_nut / tot_def).min(1.0);
            // Bestände abziehen (proportional aus freien Beständen und Pool)
            for (g, units) in &by_good {
                self.withdraw(mi, provs, &mut free, &mut sum_free, *g, *units);
            }
            for (k, d) in &deficits {
                let pid = provs[*k] as usize;
                let got = d * fill;
                food_eaten[pid] += got;
                // Vielfalt: Anteil der gelieferten Nahrung nach Art
                let tot: f32 = by_good.iter().map(|(g, u)| u * g.nutrition()).sum();
                for (g, u) in &by_good {
                    let share = if tot > 0.0 { u * g.nutrition() / tot } else { 0.0 };
                    if let Some(kidx) = FOODS.iter().position(|x| x == g) {
                        food_kinds[pid][kidx] += got * share;
                    }
                    consumed[pid][g.idx()] += got * share / g.nutrition();
                }
                self.provinces[pid].turnover += got * 2.5;
            }
            self.markets[mi].volume += delivered_nut;
            let short = (1.0 - fill).max(0.0);
            for g in &food_goods {
                self.markets[mi].shortage[g.idx()] = short;
            }
        }
        // Gütermarkt (Konsum und Werkstätten)
        for g in Good::ALL {
            let gi = g.idx();
            if FOODS.contains(g) {
                // Nahrung wurde bereits als Konsum behandelt – Werkstatt-Eingaben unten
            }
            // Gesamtanfrage der Provinzen
            let mut total_req = 0.0;
            for &pid in provs {
                let mut r = cons_req[pid as usize][gi] + input_req[pid as usize][gi];
                if FOODS.contains(g) {
                    r = input_req[pid as usize][gi];
                }
                total_req += r;
            }
            if total_req <= 0.0 {
                continue;
            }
            let avail = sum_free[gi];
            let fill = (avail / total_req).min(1.0);
            self.withdraw(mi, provs, &mut free, &mut sum_free, *g, total_req * fill);
            let price = self.markets[mi].prices[gi];
            for &pid in provs {
                let pu = pid as usize;
                let creq = if FOODS.contains(g) { 0.0 } else { cons_req[pu][gi] };
                let ireq = input_req[pu][gi];
                let cgot = creq * fill;
                let igot = ireq * fill;
                consumed[pu][gi] += cgot;
                cons_req[pu][gi] -= cgot;
                input_req[pu][gi] -= igot;
                self.provinces[pu].stock[gi] += igot;
                self.provinces[pu].turnover += (cgot + igot) * price;
            }
            self.markets[mi].volume += total_req * fill;
            self.markets[mi].shortage[gi] = if total_req > 0.0 { 1.0 - fill } else { 0.0 };
        }
        // Handelstiefe: jeder gehandelte Teil verschiebt Preise bereits über N/A; Pool zerfällt
        for g in Good::ALL {
            let gi = g.idx();
            // Rest des Pools verfällt langsam
            self.markets[mi].pool[gi] = (self.markets[mi].pool[gi] * 0.5).max(0.0);
        }
    }

    /// Entnimmt `units` einer Ware aus Pool und freien Beständen der Provinzen der Region.
    fn withdraw(&mut self, mi: usize, provs: &[ProvId], free: &mut [Vec<f32>], sum_free: &mut [f32], g: Good, units: f32) {
        let gi = g.idx();
        let total = sum_free[gi];
        if total <= 0.0 || units <= 0.0 {
            return;
        }
        let ratio = (units / total).min(1.0);
        // Pool zuerst
        let pool = self.markets[mi].pool[gi];
        self.markets[mi].pool[gi] = pool * (1.0 - ratio);
        for (k, &pid) in provs.iter().enumerate() {
            let take = free[k][gi] * ratio;
            self.provinces[pid as usize].stock[gi] = (self.provinces[pid as usize].stock[gi] - take).max(0.0);
            free[k][gi] -= take;
        }
        sum_free[gi] -= total * ratio;
    }

    #[allow(clippy::too_many_arguments)]
    fn fulfil_needs(&mut self, pi: usize, need_total: &[f32], consumed: &[f32], food_need: f32, food_eaten: f32, food_kinds: &[f32; 6], cons_req: &[f32]) {
        let p = &mut self.provinces[pi];
        let food_frac = if food_need > 0.01 { (food_eaten / food_need).clamp(0.0, 1.0) } else { 1.0 };
        p.food_ratio = food_frac;
        // Faith/Security/Market
        let pop = p.population().max(1.0);
        let temples = p.service_count(ServiceKind::Temple) as f32;
        let faith = (temples * 700.0 / pop).clamp(0.0, 1.0).max(if p.holy_site.is_some() { 0.6 } else { 0.0 });
        let towers = p.service_count(ServiceKind::Watchtower) as f32;
        let security = if p.garrison as f32 >= pop / 50.0 || towers >= 1.0 + pop / 2500.0 { 1.0 } else { (p.garrison as f32 / (pop / 50.0)).clamp(0.0, 1.0).max(towers * 0.4).min(1.0) };
        let market = if p.service_count(ServiceKind::Marketplace) > 0 || p.settlement != SettlementLevel::Village { 1.0 } else { 0.3 };
        let mut item_frac = vec![1.0f32; Good::COUNT];
        for g in 0..Good::COUNT {
            if need_total[g] > 0.01 {
                item_frac[g] = (consumed[g] / need_total[g]).clamp(0.0, 1.0);
            }
        }
        let _ = cons_req;
        for s in STRATA {
            let mut sums = [0.0f32; 3];
            let mut wts = [0.0f32; 3];
            for nd in needs_of(s) {
                let t = nd.tier as usize;
                let (f, w) = match nd.kind {
                    NeedKind::Food => (food_frac, 3.0),
                    NeedKind::Item(g) => (item_frac[g.idx()], 1.0),
                    NeedKind::Faith => (faith, 1.5),
                    NeedKind::Security => (security, 1.5),
                    NeedKind::Market => (market, 1.0),
                };
                sums[t] += f * w;
                wts[t] += w;
            }
            let st = &mut p.pop[s.idx()];
            st.basic = if wts[0] > 0.0 { sums[0] / wts[0] } else { 1.0 };
            st.comfort = if wts[1] > 0.0 { sums[1] / wts[1] } else { 1.0 };
            st.luxury = if wts[2] > 0.0 { sums[2] / wts[2] } else { 1.0 };
        }
        // Vielfalt: Brot, Fleisch, Fisch, Pökelware je ≥ 15 % des Verbrauchs
        let total: f32 = food_kinds.iter().sum();
        let mut variety: f32 = 0.0;
        if total > 0.0 {
            for (k, v) in food_kinds.iter().enumerate() {
                // FOODS: Bread, Meat, Preserved, Fish, Game, Grain
                if k <= 3 && v / total >= 0.15 {
                    variety += 2.0;
                }
            }
        }
        let porridge = if total > 0.0 { food_kinds[5] / total } else { 0.0 };
        p.variety = variety.min(8.0) - if porridge > 0.5 { 5.0 } else { 0.0 };
    }

    /// Handelsrouten zwischen Marktgebieten: Waren fließen vom billigen zum teuren Markt.
    fn trade_month(&mut self) {
        let day = self.day;
        let winter = self.season() == Season::Winter;
        let routes = self.routes.len();
        for ri in 0..routes {
            let (a, b, days, sea, cap, blocked, from_p, to_p) = {
                let r = &self.routes[ri];
                (r.a, r.b, r.days, r.sea, r.capacity, r.blocked_until, r.from_prov, r.to_prov)
            };
            self.routes[ri].volume_last = 0.0;
            self.routes[ri].profit_last = 0.0;
            if blocked > day {
                continue;
            }
            // Besetzte oder verfeindete Eintrittsprovinzen blockieren
            let (oa, ob) = (self.provinces[from_p as usize].holder(), self.provinces[to_p as usize].holder());
            let (Some(oa), Some(ob)) = (oa, ob) else { continue };
            if self.at_war(oa, ob) || self.realms[oa as usize].embargo.contains(&ob) || self.realms[ob as usize].embargo.contains(&oa) {
                continue;
            }
            if matches!(self.realms[oa as usize].kind, RealmKind::Hollowed | RealmKind::Legion) || matches!(self.realms[ob as usize].kind, RealmKind::Hollowed | RealmKind::Legion) {
                continue;
            }
            // Dornwald geschlossen (Welken)
            if self.endgame.withering && (self.regions[3].provinces.contains(&from_p) || self.regions[3].provinces.contains(&to_p)) {
                continue;
            }
            let mut tariff = self.realms[ob as usize].tariff_import / 100.0;
            if self.has_treaty(TreatyKind::Trade, oa, ob) {
                tariff *= 0.5;
            }
            // Handelsboykott der Großmacht-Reaktion
            if self.endgame.great_power == Some(oa) && self.realms[ob as usize].faction == "saelith_compact" && self.opinion(ob, oa) < 20.0 {
                tariff = tariff.max(0.2);
            }
            let route_factor = (1.0 - tariff * 1.5).max(0.0) * if sea && winter { 0.6 } else { 1.0 };
            let mut vol_total = 0.0;
            let mut profit_total = 0.0;
            for g in Good::ALL {
                let gi = g.idx();
                let (pa, pb) = (self.markets[a].prices[gi], self.markets[b].prices[gi]);
                let transport = 0.02 * (days / 10.0) * pa * if sea { 0.7 } else { 1.0 };
                let (cheap, dear, from, to) = if pb - pa > transport + tariff * pb { (pa, pb, a, b) } else if pa - pb > transport + tariff * pa { (pb, pa, b, a) } else { continue };
                // Verfügbares Angebot im billigen Markt: freier Bestand der Region
                let mut avail = 0.0;
                let prov_from: Vec<ProvId> = self.regions.iter().find(|r| r.market == Some(from)).map(|r| r.provinces.clone()).unwrap_or_default();
                for &pid in &prov_from {
                    let p = &self.provinces[pid as usize];
                    let free = if FOODS.contains(g) { p.stock[gi] * p.food_free_fraction(p.food_need()) } else { (p.stock[gi] - p.workshop_input_demand(*g) - need_of(p, *g)).max(0.0) };
                    avail += free * 0.4;
                }
                let want = self.markets[to].demand[gi] - self.markets[to].supply[gi];
                let vol = avail.min(cap).min(if want > 0.0 { want } else { cap * 0.25 }).max(0.0) * route_factor;
                if vol <= 0.01 {
                    continue;
                }
                // Entnahme proportional aus der Quellregion
                let tot_av: f32 = avail / 0.4;
                if tot_av <= 0.0 {
                    continue;
                }
                let ratio = (vol / 0.4 / tot_av).min(1.0);
                for &pid in &prov_from {
                    let p = &mut self.provinces[pid as usize];
                    let free = if FOODS.contains(g) { p.stock[gi] * p.food_free_fraction(p.food_need()) } else { (p.stock[gi] - p.workshop_input_demand(*g) - need_of(p, *g)).max(0.0) };
                    p.stock[gi] -= free * ratio * 0.4;
                }
                self.markets[to].pool[gi] += vol;
                let margin = dear - cheap - transport;
                let profit = vol * margin.max(0.0);
                profit_total += profit;
                vol_total += vol;
                // Zölle und Handelssteuer
                let to_owner = if to == b { ob } else { oa };
                let tariff_income = vol * dear * tariff;
                let trade_tax = vol * dear * (self.realms[to_owner as usize].trade_tax / 100.0) * 0.5;
                let treasurer = 1.0 + self.treasurer_trade_bonus(to_owner);
                self.realms[to_owner as usize].gold += (tariff_income + trade_tax) * treasurer * TAX_SCALE;
                self.realms[to_owner as usize].income += (tariff_income + trade_tax) * treasurer * TAX_SCALE;
                // Händlerertrag: Gewinn an das Marktgebiet der Quelle (Kaufleute) → Handelsposten
                let from_owner = if from == a { oa } else { ob };
                let merchant = profit * 0.04 * TAX_SCALE;
                self.realms[from_owner as usize].gold += merchant;
                self.realms[from_owner as usize].income += merchant;
            }
            self.routes[ri].volume_last = vol_total;
            self.routes[ri].profit_last = profit_total;
        }
    }

    fn treasurer_trade_bonus(&self, r: RealmId) -> f32 {
        let rl = &self.realms[r as usize];
        let mut b = 0.0;
        if let Some(c) = rl.offices[Office::Treasurer.idx()]
            && self.chars[c as usize].alive() {
                b += self.chars[c as usize].attr(Attr::Stewardship) as f32 * 0.005;
                if rl.office_task[Office::Treasurer.idx()] == 1 {
                    b += self.chars[c as usize].attr(Attr::Stewardship) as f32 * 0.01;
                }
            }
        if rl.laws.town_rights == 1 {
            b += 0.1;
        } else if rl.laws.town_rights == 2 {
            b += 0.25;
        }
        if self.has_legacy(r, "trading_house") {
            b += 0.1;
        }
        if rl.faction == "saelith_compact" {
            b += 0.15;
        }
        b
    }

    /// Chronik der Teuerung: Preissprung bei Grundnahrung.
    fn price_watch(&mut self) {
        let day = self.day;
        for ri in 0..self.regions.len() {
            let Some(mi) = self.regions[ri].market else { continue };
            let ratio = self.markets[mi].prices[Good::Grain.idx()] / Good::Grain.base_price();
            if ratio < 1.6 {
                continue;
            }
            let provs = self.regions[ri].provinces.clone();
            for pid in provs {
                let p = &mut self.provinces[pid as usize];
                if p.population() < 100.0 || p.owner.is_none() {
                    continue;
                }
                if p.has_state("price_spike_logged", day) {
                    continue;
                }
                p.states.insert("price_spike_logged".into(), day + 6 * DAYS_PER_MONTH);
                let cause = p.chain.drought.or(p.chain.famine);
                let name = p.name.clone();
                let owner = p.owner;
                let e = crate::chronicle::Entry::new(crate::chronicle::ChronicleKind::Economy, "price_spike", 28).prov(pid, &name).var("n", format!("{:.1}", ratio)).cause(cause).num("ratio", ratio).tag("price_spike");
                let e = if let Some(o) = owner { e.realm(o) } else { e };
                let id = self.chron(e);
                self.provinces[pid as usize].chain.price_spike = Some(id);
                break;
            }
        }
    }

    pub(crate) fn economy_year(&mut self) {
        // Inflation klingt ab (0,5 Prozentpunkte/Jahr), Böden erholen sich
        for m in self.markets.iter_mut() {
            m.inflation = (m.inflation - 0.005).max(0.0);
        }
        for p in self.provinces.iter_mut() {
            if p.fertility < 0.95 && !matches!(p.biome, Biome::Desert | Biome::Hollowland | Biome::Volcanic | Biome::Mountain) {
                p.fertility = (p.fertility + 0.01).min(match p.biome {
                    Biome::Plains => 0.95,
                    Biome::Hills => 0.7,
                    Biome::Forest => 0.6,
                    _ => 0.7,
                });
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Bauen und Verwaltung
    // ------------------------------------------------------------------------------------
    /// Kann dieses Gebäude in der Provinz gebaut werden?
    pub fn can_build(&self, pid: ProvId, bi: usize) -> Result<(), String> {
        let p = &self.provinces[pid as usize];
        let b = &BUILDINGS[bi];
        if let Some(d) = b.deposit
            && p.deposit(d) == 0 {
                return Err(format!("Kein Vorkommen von {} in {}", d.name(), p.name));
            }
        if let BKind::Service(k) = b.kind {
            match k {
                ServiceKind::Harbor if !p.coastal => return Err("Hafen benötigt Küste".into()),
                ServiceKind::MageTower if p.settlement == SettlementLevel::Village => return Err("Magierturm benötigt Stadt".into()),
                ServiceKind::Library if p.settlement == SettlementLevel::Village => return Err("Bibliothek benötigt Stadt".into()),
                ServiceKind::Marketplace if p.settlement == SettlementLevel::Village => return Err("Marktplatz benötigt Stadt".into()),
                _ => {}
            }
        }
        if b.id == "ashglass_mine" && p.veil < 2.0 {
            return Err("Aschglas nur bei Schleierstärke ab 2".into());
        }
        if matches!(b.id, "spice_plantation" | "silk_house") && !matches!(p.culture, Culture::Saelith) {
            return Err("Nur auf den Saelith-Inseln".into());
        }
        // Produktionsstätten sind durch Fläche begrenzt, Dienstgebäude nicht
        if matches!(b.kind, BKind::Production) {
            let prod: u32 = p.buildings.iter().filter(|(i, _)| matches!(BUILDINGS[*i as usize].kind, BKind::Production)).map(|(_, c)| *c as u32).sum();
            if prod > 12 + p.settlement.rank() as u32 * 30 + (p.population() / 60.0) as u32 {
                return Err("Provinz ist bebaut".into());
            }
        }
        Ok(())
    }

    pub fn build(&mut self, pid: ProvId, bi: usize, free: bool) -> Result<(), String> {
        self.can_build(pid, bi)?;
        let cost = BUILDINGS[bi].cost;
        if !free {
            let Some(o) = self.provinces[pid as usize].owner else { return Err("Provinz ohne Besitzer".into()) };
            if self.realms[o as usize].gold < cost {
                return Err("Nicht genug Gold".into());
            }
            self.realms[o as usize].gold -= cost;
        }
        self.provinces[pid as usize].add_building(bi, 1);
        Ok(())
    }

    /// Verwaltung einer Provinz: baut höchstens ein Gebäude je Aufruf nach dem größten Bedarf.
    pub fn govern_province(&mut self, pid: ProvId, free: bool) -> bool {
        let day = self.day;
        let owner = self.provinces[pid as usize].owner;
        let budget = match owner {
            Some(o) if !free => self.realms[o as usize].gold * 0.35,
            _ => f32::MAX,
        };
        let p = &self.provinces[pid as usize];
        if p.population() < 20.0 || (p.holder() != p.owner && !free) || p.siege.is_some() {
            return false;
        }
        let pop = p.population();
        let hungry = p.food_ratio < 0.97;
        let mut want: Vec<(f32, usize)> = Vec::new();
        let idx = |id: &str| building_index(id).unwrap_or(0);
        let staffing_ok = |s: Stratum| p.labor_demand(s) < p.labor_supply(s) * 0.97;
        // Wohnraum
        if pop > p.housing_capacity() * 0.93 {
            want.push((9.0, service_index(ServiceKind::Housing)));
        }
        // Nahrung
        let grain_dep = p.deposit(Good::Grain);
        if hungry || p.food_ratio < 1.0 {
            let urg = 10.0 + (1.0 - p.food_ratio) * 10.0;
            if grain_dep > 0 && staffing_ok(Stratum::Serfs) {
                want.push((urg, idx("farmstead")));
            }
            if p.deposit(Good::Fish) > 0 && staffing_ok(Stratum::Freeholders) {
                want.push((urg - 1.0, idx("fishery")));
            }
            if p.deposit(Good::Game) > 0 && staffing_ok(Stratum::Freeholders) {
                want.push((urg - 3.0, idx("hunting_lodge")));
            }
            if p.deposit(Good::Livestock) > 0 && p.stock[Good::Grain.idx()] > 200.0 {
                want.push((urg - 4.0, idx("pasture")));
            }
        }
        if p.stock[Good::Grain.idx()] > 400.0 && p.building_count(idx("windmill")) < p.building_count(idx("farmstead")) / 2 + 1 && staffing_ok(Stratum::Freeholders) {
            want.push((5.0, idx("windmill")));
        }
        if p.stock[Good::Flour.idx()] > 150.0 && staffing_ok(Stratum::Artisans) {
            want.push((5.0, idx("bakery")));
        }
        // Dienste
        if p.service_count(ServiceKind::Temple) == 0 && pop > 250.0 {
            want.push((7.0, service_index(ServiceKind::Temple)));
        }
        if p.service_count(ServiceKind::Marketplace) == 0 && p.settlement != SettlementLevel::Village {
            want.push((6.0, service_index(ServiceKind::Marketplace)));
        }
        if p.service_count(ServiceKind::Watchtower) == 0 && (p.unrest > 25.0 || p.veil > 1.0) && pop > 100.0 {
            want.push((6.5, service_index(ServiceKind::Watchtower)));
        }
        if p.disease > 45.0 && p.service_count(ServiceKind::Hospital) == 0 && pop > 800.0 {
            want.push((8.0, service_index(ServiceKind::Hospital)));
        }
        if p.disease > 30.0 && p.service_count(ServiceKind::Wells) == 0 && pop > 600.0 {
            want.push((5.5, service_index(ServiceKind::Wells)));
        }
        if p.service_count(ServiceKind::Granary) == 0 && pop > 500.0 {
            want.push((4.0, service_index(ServiceKind::Granary)));
        }
        if p.pop[Stratum::Scholars.idx()].total() > 30.0 && p.service_count(ServiceKind::Library) == 0 && p.settlement.rank() >= 1 {
            want.push((5.0, service_index(ServiceKind::Library)));
        }
        if p.service_count(ServiceKind::Harbor) == 0 && p.coastal && pop > 600.0 {
            want.push((4.5, service_index(ServiceKind::Harbor)));
        }
        if p.pop[Stratum::Warriors.idx()].total() > 20.0 && p.service_count(ServiceKind::Barracks) == 0 && pop > 1000.0 {
            want.push((3.5, service_index(ServiceKind::Barracks)));
        }
        // Mangelgüter: Gebäude, die das Gut herstellen und lokal betreibbar sind
        for g in Good::ALL {
            let un = p.unmet[g.idx()];
            if un < 4.0 || FOODS.contains(g) {
                continue;
            }
            for (bi, b) in BUILDINGS.iter().enumerate() {
                if !matches!(b.kind, BKind::Production) || !b.outputs.iter().any(|(og, _)| og == g) {
                    continue;
                }
                if let Some(d) = b.deposit
                    && p.deposit(d) == 0 {
                        continue;
                    }
                if !staffing_ok(b.stratum) {
                    continue;
                }
                // Eingaben lokal vorhanden oder vom Markt beziehbar
                let inputs_ok = b.inputs.iter().all(|(ig, _)| p.stock[ig.idx()] > 5.0 || p.produced[ig.idx()] > 0.0);
                if !inputs_ok {
                    continue;
                }
                let urgency = 4.0 + (un / (pop / 100.0 + 1.0)).min(4.0);
                want.push((urgency, bi));
            }
        }
        // Spezialisierung: Rohstoffe mit reichen Vorkommen
        for (g, rich) in &p.deposits {
            let rich = *rich as f32;
            for (bi, b) in BUILDINGS.iter().enumerate() {
                if b.deposit == Some(*g) && matches!(b.kind, BKind::Production) && b.id != "farmstead" && b.id != "ashglass_mine" && b.id != "spice_plantation" && b.id != "silk_house" {
                    let have = p.building_count(bi) as f32;
                    if have < rich * 1.2 && staffing_ok(b.stratum) {
                        want.push((1.5 + rich * 0.3 - have * 0.2, bi));
                    }
                }
            }
        }
        if (p.culture == Culture::Saelith) && p.deposit(Good::Spices) > 0 && staffing_ok(Stratum::Freeholders) && p.building_count(idx("spice_plantation")) < 3 {
            want.push((3.0, idx("spice_plantation")));
        }
        if p.deposit(Good::Silk) > 0 && p.culture == Culture::Saelith && staffing_ok(Stratum::Artisans) && p.building_count(idx("silk_house")) < 2 {
            want.push((3.0, idx("silk_house")));
        }
        // Verarbeitungsketten: Eingangsprodukt wird in der Provinz erzeugt oder lagert in Menge
        for (bi, b) in BUILDINGS.iter().enumerate() {
            if !matches!(b.kind, BKind::Production) || b.deposit.is_some() || b.inputs.is_empty() {
                continue;
            }
            if !staffing_ok(b.stratum) {
                continue;
            }
            let have = p.building_count(bi) as f32;
            let (ig, amt) = b.inputs[0];
            let cap_month = amt * (SECS_PER_MONTH / b.time) * have;
            let supply = p.produced[ig.idx()] + p.stock[ig.idx()] / 6.0;
            let first_needed = have < 1.0 && supply > amt * 2.0 && p.settlement != SettlementLevel::Village || have < 1.0 && supply > amt * 8.0;
            if first_needed || (have >= 1.0 && supply > cap_month * 1.25 + amt * 2.0 && have < 6.0 + p.settlement.rank() as f32 * 6.0) {
                // Brot, Fleisch, Bier und Kleidung zuerst (Bedürfnisse), dann Luxus
                let prio = match b.outputs[0].0 {
                    Good::Bread | Good::Flour | Good::Meat | Good::PreservedFood => 6.5,
                    Good::Clothing | Good::Cloth | Good::Yarn | Good::Beer | Good::Tools | Good::Planks => 4.5,
                    _ => 2.5,
                };
                want.push((prio, bi));
            }
        }
        // Zivile Aufwertung der Siedlung
        want.sort_by(|a, b| b.0.total_cmp(&a.0));
        let mut built = false;
        for (_, bi) in want {
            let cost = BUILDINGS[bi].cost;
            if cost > budget && !free {
                continue;
            }
            if self.build(pid, bi, free).is_ok() {
                built = true;
                break;
            }
        }
        // Siedlungsstufe anheben
        if !built || free {
            let p = &self.provinces[pid as usize];
            if let Some(next) = p.settlement.next() {
                let cost = match next {
                    SettlementLevel::Town => 900.0,
                    SettlementLevel::City => 2800.0,
                    SettlementLevel::Capital => 7000.0,
                    SettlementLevel::Village => 0.0,
                };
                let is_cap = owner.is_some_and(|o| self.realms[o as usize].capital == pid);
                let allowed = next != SettlementLevel::Capital || is_cap;
                let afford = free || owner.is_some_and(|o| self.realms[o as usize].gold > cost * 1.5);
                if allowed && afford && p.population() > p.housing_capacity() * 0.85 && p.avg_satisfaction() > 45.0 && !free {
                    if let Some(o) = owner {
                        self.realms[o as usize].gold -= cost;
                    }
                    self.provinces[pid as usize].settlement = next;
                    built = true;
                    if next == SettlementLevel::City || next == SettlementLevel::Capital {
                        let name = self.provinces[pid as usize].name.clone();
                        let e = crate::chronicle::Entry::new(crate::chronicle::ChronicleKind::Construction, "event_generic", 22).prov(pid, &name).var("a", name.clone()).var("c", format!("{name} wuchs zur {}", next.name()));
                        let e = if let Some(o) = owner { e.realm(o) } else { e };
                        self.chron(e);
                    }
                }
            }
        }
        let _ = day;
        built
    }

    /// Weist alle Provinzen der Reiche (ohne Spieler bei abgeschalteter Autoverwaltung) an zu bauen.
    pub(crate) fn govern_all(&mut self) {
        let rules_auto = self.options.rules.auto_govern_player;
        for pi in 0..self.provinces.len() {
            let Some(o) = self.provinces[pi].owner else { continue };
            if self.is_player(o) && !rules_auto {
                continue;
            }
            if matches!(self.realms[o as usize].kind, RealmKind::Hollowed | RealmKind::Legion) {
                continue;
            }
            // Nicht jeden Monat: je Provinz im Wechsel
            if !(self.day / DAYS_PER_MONTH + pi as u32).is_multiple_of(2) {
                continue;
            }
            self.govern_province(pi as ProvId, false);
        }
    }

    // ------------------------------------------------------------------------------------
    // Märkte erzeugen
    // ------------------------------------------------------------------------------------
    pub(crate) fn init_markets(&mut self) {
        self.markets.clear();
        for mi in 0..MARKET_COUNT {
            let region = self.regions.iter().position(|r| r.market == Some(mi)).unwrap_or(0);
            let prices: Vec<f32> = Good::ALL.iter().map(|g| g.base_price()).collect();
            self.markets.push(Market {
                region,
                name: format!("Markt {}", self.regions[region].name),
                prices,
                supply: vec![0.0; Good::COUNT],
                demand: vec![0.0; Good::COUNT],
                pool: vec![0.0; Good::COUNT],
                inflation: 0.0,
                event_factor: vec![(1.0, 0); Good::COUNT],
                volume: 0.0,
                shortage: vec![0.0; Good::COUNT],
            });
        }
        // Routen: zwischen benachbarten Marktgebieten über Grenzprovinzen
        self.routes.clear();
        let mut seen: Vec<(usize, usize)> = Vec::new();
        for p in &self.provinces {
            let Some(ma) = self.regions[p.region as usize].market else { continue };
            for l in &p.neighbors {
                let q = &self.provinces[l.to as usize];
                let Some(mb) = self.regions[q.region as usize].market else { continue };
                if ma == mb || seen.contains(&(ma.min(mb), ma.max(mb))) && l.kind == LinkKind::Land {
                    continue;
                }
                if seen.contains(&(ma.min(mb), ma.max(mb))) && !(l.kind == LinkKind::Sea && p.id < q.id && !seen.is_empty()) {
                    continue;
                }
                seen.push((ma.min(mb), ma.max(mb)));
                let sea = l.kind == LinkKind::Sea;
                self.routes.push(TradeRoute { a: ma, b: mb, from_prov: p.id, to_prov: q.id, days: l.dist / if sea { 20.0 } else { 11.0 }, sea, capacity: if sea { 320.0 } else { 220.0 }, blocked_until: 0, volume_last: 0.0, profit_last: 0.0 });
            }
        }
    }

    /// Eine aktive Handelsroute vorhanden?
    pub fn route_exists(&self, ma: usize, mb: usize) -> bool {
        self.routes.iter().any(|r| (r.a == ma && r.b == mb) || (r.a == mb && r.b == ma))
    }

    /// Anteil des Handelsvolumens am Gesamtvolumen (für den Handelssieg), je Reich: Summe der Route-Volumen an
    /// Marktgebieten, deren Eintrittsprovinzen das Reich kontrolliert.
    pub fn trade_share(&self, r: RealmId) -> f32 {
        let total: f32 = self.routes.iter().map(|x| x.volume_last).sum::<f32>() + self.markets.iter().map(|m| m.volume).sum::<f32>() * 0.0;
        if total <= 0.0 {
            return 0.0;
        }
        let mine: f32 = self.routes.iter().filter(|x| self.provinces[x.from_prov as usize].holder() == Some(r) || self.provinces[x.to_prov as usize].holder() == Some(r)).map(|x| x.volume_last).sum();
        mine / total
    }

    /// Aktueller Preis einer Ware auf dem Markt der Provinz.
    pub fn price_in(&self, pid: ProvId, g: Good) -> f32 {
        let region = self.provinces[pid as usize].region as usize;
        match self.regions[region].market {
            Some(m) => self.markets[m].prices[g.idx()],
            None => g.base_price(),
        }
    }

    /// Setzt einen Ereignisfaktor (z. B. Kriegsfolgen, Ereignisse) auf den Markt der Provinz.
    pub fn set_price_factor(&mut self, pid: ProvId, g: Good, factor: f32, months: u32) {
        let region = self.provinces[pid as usize].region as usize;
        if let Some(m) = self.regions[region].market {
            self.markets[m].event_factor[g.idx()] = (factor, self.day + months * DAYS_PER_MONTH);
        }
    }

    // ------------------------------------------------------------------------------------
    // Finanzen: Steuern, Zölle, Kredite
    // ------------------------------------------------------------------------------------
    pub fn corruption(&self, r: RealmId) -> f32 {
        let rl = &self.realms[r as usize];
        let provs = self.provinces_of(r).len() as f32;
        let mut limit = 6.0;
        if let Some(c) = rl.offices[Office::Chancellor.idx()] {
            limit += self.chars[c as usize].attr(Attr::Diplomacy) as f32 / 4.0;
        }
        limit += self.chars[rl.ruler as usize].attr(Attr::Stewardship) as f32 / 2.0;
        if let Some(c) = rl.offices[Office::Steward.idx()] {
            limit += self.chars[c as usize].attr(Attr::Stewardship) as f32 / 4.0;
        }
        if rl.great_power {
            limit *= 0.75;
        }
        let treasurer = rl.offices[Office::Treasurer.idx()].map(|c| self.chars[c as usize].attr(Attr::Stewardship) as f32).unwrap_or(0.0);
        let mut c = 5.0 + (provs - limit).max(0.0) - treasurer * 0.5;
        if rl.great_power {
            c += 5.0;
        }
        if let Some(t) = rl.offices[Office::Treasurer.idx()]
            && rl.office_task[Office::Treasurer.idx()] == 2 {
                c -= self.chars[t as usize].attr(Attr::Stewardship) as f32 * 0.5;
            }
        c.clamp(0.0, 40.0) / 100.0
    }

    /// Monatliche Finanzen: Steuern, Unterhalt, Zinsen, Tribute.
    pub(crate) fn finances_month(&mut self) {
        let day = self.day;
        let nr = self.realms.len();
        let mut income = vec![0.0f32; nr];
        let mut expense = vec![0.0f32; nr];
        let mut house_income: Vec<(HouseId, f32)> = Vec::new();
        let corr: Vec<f32> = (0..nr).map(|r| self.corruption(r as RealmId)).collect();
        for pi in 0..self.provinces.len() {
            let p = &self.provinces[pi];
            let Some(o) = p.owner else { continue };
            let holder = p.holder();
            if holder != Some(o) || p.rebel_held {
                continue;
            }
            let rl = &self.realms[o as usize];
            if matches!(rl.kind, RealmKind::Hollowed | RealmKind::Legion) {
                continue;
            }
            let mult = tax_multiplier(rl.tax_level);
            let mut tax = 0.0;
            for s in STRATA {
                let st = &p.pop[s.idx()];
                let z = st.satisfaction;
                let zf = if z >= 80.0 { 1.1 } else if z >= 40.0 { 1.0 } else if z >= 20.0 { 0.8 } else { 0.5 };
                tax += st.total() * s.head_tax() * mult * zf;
            }
            tax *= TAX_SCALE;
            // Zehnt der Kirche (10 % bei Staatskirche) – vereinfacht als Frömmigkeitsgewinn
            let mut bonus = 1.0;
            if let Some(t) = rl.offices[Office::Treasurer.idx()] {
                bonus += self.chars[t as usize].attr(Attr::Stewardship) as f32 * 0.005;
                if rl.office_task[Office::Treasurer.idx()] == 0 {
                    bonus += self.chars[t as usize].attr(Attr::Stewardship) as f32 * 0.01;
                }
            }
            bonus += self.chars[rl.ruler as usize].attr(Attr::Stewardship) as f32 * 0.005;
            if self.chars[rl.ruler as usize].has(crate::catalog::Trait::Greedy) {
                bonus += 0.1;
            }
            if self.chars[rl.ruler as usize].has(crate::catalog::Trait::Generous) {
                bonus -= 0.05;
            }
            let trade_tax = p.turnover * (rl.trade_tax / 100.0) * TAX_SCALE;
            let total = (tax * bonus + trade_tax) * (1.0 - corr[o as usize]);
            match p.fief {
                Some(h) if self.houses[h as usize].alive => {
                    let share = rl.laws.vassal_tax / 100.0;
                    income[o as usize] += total * share;
                    house_income.push((h, total * (1.0 - share)));
                }
                _ => income[o as usize] += total,
            }
            // Gebäudeunterhalt
            let up: f32 = p.buildings.iter().map(|(i, c)| BUILDINGS[*i as usize].upkeep * *c as f32).sum::<f32>() * 0.2;
            expense[o as usize] += up;
        }
        for (h, g) in house_income {
            self.houses[h as usize].wealth += g * 0.25;
        }
        for pi in 0..self.provinces.len() {
            self.provinces[pi].turnover = 0.0;
        }
        // Armeeunterhalt
        for a in &self.armies {
            if let (Some(o), true) = (a.owner, matches!(a.kind, ArmyKind::Realm | ArmyKind::Mercenary)) {
                let up: f32 = a.regiments.iter().map(|r| crate::catalog::unit(&r.unit).map(|u| u.upkeep).unwrap_or(1.0) * r.count as f32).sum();
                let disc = self.realms[o as usize].offices[Office::Marshal.idx()].map(|m| self.chars[m as usize].attr(Attr::Martial) as f32 * 0.01).unwrap_or(0.0);
                expense[o as usize] += up * (1.0 - disc) * 1.3;
            }
        }
        // Hof und Gehälter
        for r in 0..nr {
            let rl = &self.realms[r];
            if !rl.alive || matches!(rl.kind, RealmKind::Hollowed | RealmKind::Legion) {
                continue;
            }
            let courtiers = rl.court.len() as f32;
            let mut e = courtiers.min(24.0) * 1.2;
            for (i, o) in rl.offices.iter().enumerate() {
                if let Some(c) = o {
                    e += 10.0 + self.chars[*c as usize].attr(OFFICES[i].attr()) as f32;
                }
            }
            expense[r] += e * 0.5;
        }
        // Zinsen und Tilgung
        for r in 0..nr {
            let mut interest = 0.0;
            for l in self.realms[r].loans.iter() {
                interest += l.principal * l.rate / 12.0 + l.principal / (l.months_left.max(1) as f32) * 0.0;
            }
            expense[r] += interest;
        }
        // Vasallenabgaben und Tribute (Vasall zahlt Teil seines Nettoeinkommens)
        let mut transfers: Vec<(RealmId, RealmId, f32)> = Vec::new();
        for r in 0..nr {
            let Some(o) = self.realms[r].vassal_of else { continue };
            if !self.realms[r].alive || !self.realms[o as usize].alive {
                continue;
            }
            let net = (income[r] - expense[r]).max(0.0);
            let share = self.realms[o as usize].laws.vassal_tax / 100.0;
            transfers.push((r as RealmId, o, net * share));
        }
        for t in self.treaties.iter().filter(|t| t.kind == TreatyKind::Tribute && t.end.is_none_or(|e| e > day)) {
            // a = Empfänger, b = Zahler
            let net = (income[t.b as usize] - expense[t.b as usize]).max(0.0);
            transfers.push((t.b, t.a, net * 0.10));
        }
        for (from, to, amt) in transfers {
            income[from as usize] -= amt;
            income[to as usize] += amt;
            self.realms[from as usize].tribute_paid += amt;
        }
        // Verbuchen
        for r in 0..nr {
            let rl = &mut self.realms[r];
            if !rl.alive {
                continue;
            }
            // Handelserträge dieses Monats wurden bereits in `income` verbucht (Zölle); hier den Rest addieren
            rl.income += income[r];
            rl.expenses = expense[r];
            rl.gold += income[r] - expense[r];
        }
        // Tilgung und Bankrott
        for r in 0..nr {
            if !self.realms[r].alive {
                continue;
            }
            self.service_loans(r as RealmId);
        }
        // Eisernes Gesetz gegen unendlichen Gold-Anstieg nur über Ausgaben der KI (siehe ai.rs)
    }

    /// Kredite: Laufzeit herunterzählen, Rate zahlen, Bankrott bei Zahlungsunfähigkeit.
    fn service_loans(&mut self, r: RealmId) {
        let day = self.day;
        let mut paid_off = Vec::new();
        let n = self.realms[r as usize].loans.len();
        for i in 0..n {
            let rl = &mut self.realms[r as usize];
            let l = &mut rl.loans[i];
            let installment = l.principal / l.months_left.max(1) as f32;
            if rl.gold >= installment {
                rl.gold -= installment;
                l.principal -= installment;
                l.months_left = l.months_left.saturating_sub(1);
                if l.months_left == 0 || l.principal < 1.0 {
                    paid_off.push(i);
                }
            }
        }
        for i in paid_off.into_iter().rev() {
            self.realms[r as usize].loans.remove(i);
        }
        // Bankrott: negatives Gold bleibt über 6 Monate? Einfach: Gold < −Kreditgrenze/2
        let limit = self.credit_limit(r);
        if self.realms[r as usize].gold < -limit * 0.5 {
            self.bankrupt(r);
        } else if self.realms[r as usize].gold < 0.0 {
            // Kreditwürdige Reiche leihen automatisch
            if self.realms[r as usize].credit_blocked_until <= day {
                let amt = (-self.realms[r as usize].gold + 200.0).min(limit);
                let _ = self.take_loan(r, LoanSource::SaelithBanks, amt, 5);
            }
        }
    }

    pub fn credit_limit(&self, r: RealmId) -> f32 {
        let rl = &self.realms[r as usize];
        (rl.income.max(300.0) * 12.0 * 3.0 / 12.0).max(1500.0)
    }

    /// Nimmt einen Kredit auf (Laufzeit in Jahren).
    pub fn take_loan(&mut self, r: RealmId, source: LoanSource, amount: f32, years: u32) -> Result<(), String> {
        let day = self.day;
        let rl = &self.realms[r as usize];
        if rl.credit_blocked_until > day {
            return Err("Kreditwürdigkeit verloren".into());
        }
        let debt: f32 = rl.loans.iter().map(|l| l.principal).sum();
        let annual = (rl.income - rl.expenses).max(100.0) * 12.0;
        if debt + amount > annual * 3.0 + 1500.0 {
            return Err("Kreditgrenze überschritten".into());
        }
        let treasurer = rl.offices[Office::Treasurer.idx()].map(|c| self.chars[c as usize].attr(Attr::Stewardship) as f32 * 0.001).unwrap_or(0.0);
        let rate = match source {
            LoanSource::SaelithBanks => 0.04 + 0.05 * (debt / annual.max(1.0)) - treasurer,
            LoanSource::Church => 0.02,
            LoanSource::House(_) => 0.06,
            LoanSource::War => 0.03,
        }
        .max(0.01);
        match source {
            LoanSource::Church if rl.religion != Religion::EternalFlame || rl.piety < 300.0 => return Err("Kirche verleiht nicht".into()),
            LoanSource::House(h) if self.houses[h as usize].power < 30.0 => return Err("Haus zu schwach".into()),
            LoanSource::War if !self.in_any_war(r) => return Err("Kriegsanleihe nur im Krieg".into()),
            _ => {}
        }
        let rl = &mut self.realms[r as usize];
        rl.loans.push(Loan { lender: source, principal: amount, rate, months_left: years.max(1) * 12 });
        rl.gold += amount;
        if let LoanSource::House(h) = source {
            if !self.houses[h as usize].rights.contains(&HouseRight::Creditor) {
                self.houses[h as usize].rights.push(HouseRight::Creditor);
            }
            self.houses[h as usize].loyalty = (self.houses[h as usize].loyalty + 10.0).min(100.0);
        }
        Ok(())
    }

    fn bankrupt(&mut self, r: RealmId) {
        let day = self.day;
        let rl = &mut self.realms[r as usize];
        rl.loans.clear();
        rl.gold = 0.0;
        rl.credit_blocked_until = day + 20 * DAYS_PER_YEAR;
        rl.prestige -= 300.0;
        let name = rl.name.clone();
        for pid in self.provinces_of(r).to_vec() {
            for s in self.provinces[pid as usize].pop.iter_mut() {
                s.satisfaction = (s.satisfaction - 10.0).max(0.0);
            }
            self.provinces[pid as usize].unrest = (self.provinces[pid as usize].unrest + 8.0).min(100.0);
        }
        for a in self.armies.iter_mut().filter(|a| a.owner == Some(r)) {
            a.morale = (a.morale - 20.0).max(0.0);
            a.unpaid_months = a.unpaid_months.saturating_add(6);
        }
        let e = crate::chronicle::Entry::new(crate::chronicle::ChronicleKind::Economy, "bankruptcy", 40).realm(r).var("a", name);
        self.chron(e);
    }
}

fn need_of(p: &Province, g: Good) -> f32 {
    let mut n = 0.0;
    for s in STRATA {
        let n100 = p.pop[s.idx()].total() / 100.0;
        for nd in needs_of(s) {
            if nd.kind == NeedKind::Item(g) {
                n += nd.per100 * n100;
            }
        }
    }
    n
}
