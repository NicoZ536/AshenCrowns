//! Religion: sechs Glaubensrichtungen, Ausbreitung und Bekehrung, Toleranz und Verbot, Tempel,
//! Feiertage, Häresien, Kirche und Krone (GDD Kapitel 13).

use crate::chronicle::{ChronicleKind, Entry};
use crate::goods::ServiceKind;
use crate::model::*;
use crate::types::*;
use crate::world::StrategyWorld;

impl StrategyWorld {
    /// Glaubensstärke einer Provinz (0–1): Anteil der Staatsreligion × Tempelversorgung (GDD 13.2).
    pub fn faith_strength(&self, pid: ProvId) -> f32 {
        let p = &self.provinces[pid as usize];
        let Some(o) = p.owner else { return 0.0 };
        let share = p.faith[self.realms[o as usize].religion.idx()];
        let coverage = (p.service_count(ServiceKind::Temple) as f32 * 700.0 / p.population().max(1.0)).clamp(0.0, 1.0);
        let holy = if p.holy_site == Some(self.realms[o as usize].religion) { 0.15 } else { 0.0 };
        (share * (0.3 + 0.7 * coverage) + holy).clamp(0.0, 1.0)
    }

    /// Meinung des Oberhaupts der Ewigen Flamme (Erzflammenwahrer) zum Reich.
    pub fn archflame_opinion(&self, r: RealmId) -> f32 {
        let rl = &self.realms[r as usize];
        let ruler = &self.chars[rl.ruler as usize];
        let mut v = 0.0;
        if rl.religion == Religion::EternalFlame {
            v += 20.0 + rl.piety.min(500.0) / 10.0;
        } else {
            v -= 10.0;
        }
        if ruler.has(crate::catalog::Trait::FlameSworn) {
            v += 30.0;
        }
        v -= rl.hollow_path as f32 * 25.0;
        v -= rl.tyranny * 0.3;
        if rl.laws.magic_law == 2 {
            v -= 10.0;
        }
        if rl.forbidden_discovered {
            v -= 40.0;
        }
        if let Some(h) = rl.offices[Office::HighPriest.idx()] {
            let _ = h;
        }
        v.clamp(-100.0, 100.0)
    }

    pub(crate) fn religion_month(&mut self) {
        let day = self.day;
        let month = self.month();
        let n = self.provinces.len();
        // Kirche: Bann und Weihe
        for r in 0..self.realms.len() as RealmId {
            if !self.realms[r as usize].alive || crate::diplomacy::is_antagonist(self.realms[r as usize].kind) {
                continue;
            }
            if self.realms[r as usize].religion != Religion::EternalFlame {
                continue;
            }
            let op = self.archflame_opinion(r);
            let exc = self.realms[r as usize].excommunicated;
            if op < -50.0 && !exc {
                self.realms[r as usize].excommunicated = true;
                let e = Entry::new(ChronicleKind::Religion, "excommunication", 55).realm(r).var("a", self.rname(r));
                self.chron(e);
                // Vasallen dürfen ohne Malus rebellieren; Gläubige −30
                for h in self.houses.iter_mut().filter(|h| h.alive && h.realm == r) {
                    h.loyalty = (h.loyalty - 12.0).max(0.0);
                }
                for o in 0..self.realms.len() as RealmId {
                    if o != r && self.realms[o as usize].alive && self.realms[o as usize].religion == Religion::EternalFlame {
                        self.add_opinion(o, r, OpKind::Insult, -30.0, Some(day + 20 * DAYS_PER_YEAR));
                    }
                }
            } else if op > -20.0 && exc {
                self.realms[r as usize].excommunicated = false;
            }
        }
        let high_priest_attr: Vec<f32> = self
            .realms
            .iter()
            .map(|r| match r.offices[Office::HighPriest.idx()] {
                Some(c) if self.chars[c as usize].alive() => self.chars[c as usize].attr(Attr::Learning) as f32,
                _ => 0.0,
            })
            .collect();
        for pi in 0..n {
            let owner = self.provinces[pi].owner;
            let Some(o) = owner else { continue };
            if crate::diplomacy::is_antagonist(self.realms[o as usize].kind) {
                // Hohle Provinzen: Leere Liturgie breitet sich aus
                let p = &mut self.provinces[pi];
                p.faith = [0.0; 6];
                p.faith[Religion::HollowLiturgy.idx()] = 0.85;
                p.faith[Religion::SunHeresy.idx()] = 0.15;
                continue;
            }
            let state = self.realms[o as usize].religion;
            let law = self.realms[o as usize].laws.faith_law;
            let task = self.realms[o as usize].office_task[Office::HighPriest.idx()];
            let learning = high_priest_attr[o as usize];
            let zeal = self.zeal[state.idx()];
            let _fs = self.faith_strength(pi as ProvId);
            let p = &self.provinces[pi];
            let temples = p.service_count(ServiceKind::Temple).min(3) as f32;
            let avg_z = p.avg_satisfaction();
            let mut missionary = learning * 0.5 + temples * 2.0 + zeal / 10.0 + if law == 0 { 5.0 } else { 0.0 };
            if task == 0 {
                missionary += learning * 0.5;
            }
            // Widerstand: Glaubensstärke der alten Mehrheit × 10 + heilige Stätte + Zufriedenheit/20
            let resistance = {
                let major = p.faith_majority();
                let old = if major != state { p.faith[major.idx()] } else { 0.0 };
                old * 10.0 + if p.holy_site.is_some() && p.holy_site != Some(state) { 5.0 } else { 0.0 } + avg_z / 20.0
            };
            let mut progress = ((missionary - resistance) * 0.5).max(0.0);
            if law == 2 {
                progress *= 0.2;
            }
            // Zwangsbekehrung: Staatskirche + „Häresie verfolgen“
            let forced = law == 0 && task == 2;
            if forced {
                progress *= 2.0;
            }
            let share_before = self.provinces[pi].faith[state.idx()];
            if progress > 0.0 && share_before < 0.98 {
                let shift = (progress / 100.0 * 0.25).min(0.08);
                let p = &mut self.provinces[pi];
                let others: f32 = 1.0 - p.faith[state.idx()];
                if others > 0.0 {
                    for i in 0..6 {
                        if i != state.idx() {
                            let take = shift * p.faith[i] / others;
                            p.faith[i] -= take;
                            p.faith[state.idx()] += take;
                        }
                    }
                }
                if forced && p.faith_majority() != state {
                    p.tension[4] = (p.tension[4] + 3.0).min(100.0);
                }
            }
            // Natürliche Ausbreitung von Nachbarn (kleiner Einfluss)
            let nbs: Vec<ProvId> = self.provinces[pi].neighbors.iter().map(|l| l.to).collect();
            let mut drift = [0.0f32; 6];
            for nb in &nbs {
                let q = &self.provinces[*nb as usize];
                if q.population() < 50.0 {
                    continue;
                }
                let major = q.faith_majority();
                drift[major.idx()] += q.faith[major.idx()] * 0.0015;
            }
            let p = &mut self.provinces[pi];
            for i in 0..6 {
                if drift[i] > 0.0 && i != state.idx() {
                    let take = drift[i].min(p.faith[state.idx()] * 0.2);
                    p.faith[state.idx()] -= take;
                    p.faith[i] += take;
                }
            }
            // Leere Liturgie: wächst mit der Hohlung (verboten: sinkt durch Hohepriester)
            let hol = p.hollowing;
            if hol > 15.0 {
                let g = (hol - 15.0) / 100.0 * 0.01;
                let take = g.min(p.faith[state.idx()] * 0.1);
                p.faith[state.idx()] -= take;
                p.faith[Religion::HollowLiturgy.idx()] += take;
            } else {
                p.faith[Religion::HollowLiturgy.idx()] *= 0.97;
            }
            if task == 1 || task == 2 {
                p.faith[Religion::HollowLiturgy.idx()] *= 0.98;
            }
            // Normalisieren
            let s: f32 = p.faith.iter().sum();
            if s > 0.0 {
                for f in p.faith.iter_mut() {
                    *f /= s;
                }
            }
            // Mehrheitswechsel → Chronik
            let major_now = p.faith_majority();
            let prev = p.states.get("faith_major").copied().unwrap_or(u32::MAX);
            if prev != u32::MAX && prev != major_now.idx() as u32 + 1000 && p.population() > 100.0 && major_now.idx() as u32 + 1000 != prev {
                let name = p.name.clone();
                self.zeal[major_now.idx()] = (self.zeal[major_now.idx()] + 2.0).min(100.0);
                let e = Entry::new(ChronicleKind::Religion, "conversion", 30).prov(pi as ProvId, &name).realm(o).var("c", major_now.name());
                self.chron(e);
            }
            let p = &mut self.provinces[pi];
            p.states.insert("faith_major".into(), major_now.idx() as u32 + 1000);
            // Feiertage
            let (fm, _) = major_now.feast();
            if month == fm as u32 && p.service_count(ServiceKind::Temple) > 0 && !p.has_state("festival", day) && self.realms[o as usize].gold > 40.0 {
                self.realms[o as usize].gold -= 25.0;
                self.provinces[pi].states.insert("festival".into(), day + 30);
                if major_now == Religion::HollowLiturgy {
                    self.provinces[pi].hollowing = (self.provinces[pi].hollowing + 1.0).min(100.0);
                }
                self.realms[o as usize].piety += 3.0;
            }
        }
        // Eifer klingt zur Mitte ab
        for z in self.zeal.iter_mut() {
            *z += (50.0 - *z) * 0.01;
        }
        // Konflikte: Glaubensspannung bei Mischbevölkerung unter Staatskirche
        for pi in 0..n {
            let Some(o) = self.provinces[pi].owner else { continue };
            if self.realms[o as usize].laws.faith_law == 0 {
                let state = self.realms[o as usize].religion;
                let minority = 1.0 - self.provinces[pi].faith[state.idx()];
                if minority > 0.35 {
                    let p = &mut self.provinces[pi];
                    p.tension[4] = (p.tension[4] + minority * 1.5).min(100.0);
                }
            }
        }
    }

    /// Häresien entstehen emergent (GDD 13.5): jährlich in schwach gläubigen Flammen-Provinzen.
    pub(crate) fn heresy_year(&mut self) {
        for pi in 0..self.provinces.len() {
            let Some(o) = self.provinces[pi].owner else { continue };
            if self.realms[o as usize].religion != Religion::EternalFlame {
                continue;
            }
            let fs = self.faith_strength(pi as ProvId);
            let serf_z = self.provinces[pi].pop[Stratum::Serfs.idx()].satisfaction;
            if fs < 0.4 && serf_z < 40.0 {
                let mut p = (0.4 - fs) * 0.1;
                // Nähe zur Aurethwüste
                if self.provinces[pi].neighbors.iter().any(|l| self.provinces[l.to as usize].region == 5) {
                    p *= 1.5;
                }
                if self.chance(p) {
                    let pr = &mut self.provinces[pi];
                    let take = (pr.faith[Religion::EternalFlame.idx()] * 0.25).min(0.25);
                    pr.faith[Religion::EternalFlame.idx()] -= take;
                    pr.faith[Religion::SunHeresy.idx()] += take;
                    let name = pr.name.clone();
                    self.zeal[Religion::SunHeresy.idx()] = (self.zeal[Religion::SunHeresy.idx()] + 4.0).min(100.0);
                    let e = Entry::new(ChronicleKind::Religion, "heresy", 36).prov(pi as ProvId, &name).realm(o);
                    self.chron(e);
                }
            }
        }
    }

    /// Anteil der Provinzen, in denen eine Religion die Mehrheit stellt.
    pub fn faith_share(&self, rel: Religion) -> f32 {
        let total = self.provinces.len() as f32;
        self.provinces.iter().filter(|p| p.faith_majority() == rel).count() as f32 / total.max(1.0)
    }

    /// Alle heiligen Stätten einer Religion unter der Kontrolle von `r`?
    pub fn holds_all_holy_sites(&self, r: RealmId, rel: Religion) -> bool {
        let sites: Vec<&Province> = self.provinces.iter().filter(|p| p.holy_site == Some(rel)).collect();
        !sites.is_empty() && sites.iter().all(|p| p.owner == Some(r) || p.owner.is_some_and(|o| self.realms[o as usize].vassal_of == Some(r)))
    }
}
