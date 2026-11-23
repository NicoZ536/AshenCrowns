//! Magie: neun Schulen mit Forschung, Rituale, Artefakte, Magierorden, Kosten und Risiken, Schleier
//! und die Hohlung als eigener Entwicklungspfad mit Nachteilen und Vorteilen (GDD Kapitel 12).

use crate::catalog::{self, ArtifactEffect, RitualEffect, GRADE_ASHGLASS, GRADE_POINTS};
use crate::chronicle::{ChronicleKind, Entry};
use crate::goods::{Good, ServiceKind};
use crate::model::*;
use crate::types::*;
use crate::world::StrategyWorld;

/// Schwellen der Reichshohlung für die Stufen 1–5.
pub const HOLLOW_STAGES: [f32; 5] = [10.0, 25.0, 45.0, 65.0, 85.0];

pub const HOLLOW_STAGE_NAMES: [&str; 6] = ["Unberührt", "Geflüster", "Gezeichnet", "Verhohlt", "Hohlkrone", "Das Tief erwacht"];

impl StrategyWorld {
    /// Forschungspunkte pro Monat (GDD 12.4).
    pub fn research_rate(&self, r: RealmId) -> f32 {
        let rl = &self.realms[r as usize];
        let mut scholars = 0.0;
        let mut libraries = 0.0;
        for p in self.provinces_of(r) {
            let pr = &self.provinces[*p as usize];
            scholars += pr.pop[Stratum::Scholars.idx()].adults;
            libraries += pr.service_count(ServiceKind::Library) as f32;
        }
        let books = libraries.min(self.books_available(r));
        let mut fp = scholars * 0.5 + books * 10.0;
        if let Some(c) = rl.offices[Office::Chancellor.idx()] {
            fp += self.chars[c as usize].attr(Attr::Learning) as f32 * 2.0;
        }
        fp += self.chars[rl.ruler as usize].attr(Attr::Learning) as f32;
        let law = match rl.laws.magic_law {
            0 => 0.5,
            1 => 1.0,
            _ => 1.3,
        };
        let mut f = law * (1.0 + 0.01 * self.chars[rl.ruler as usize].attr(Attr::Learning) as f32);
        if self.has_legacy(r, "spark_of_wisdom") {
            f += 0.1;
        }
        f += rl.mage_orders.len() as f32 * 0.05;
        f += self.artifact_bonus(r, |e| matches!(e, ArtifactEffect::ResearchBoost));
        fp * f
    }

    fn books_available(&self, r: RealmId) -> f32 {
        self.provinces_of(r).iter().map(|p| self.provinces[*p as usize].stock[Good::Books.idx()]).sum()
    }

    pub fn artifact_bonus(&self, r: RealmId, pred: impl Fn(&ArtifactEffect) -> bool) -> f32 {
        self.artifacts.iter().filter(|a| a.holder == Some(r) && pred(&a.effect)).map(|a| crate::catalog::ARTIFACTS[a.def as usize].amount).sum()
    }

    /// Gesamtbonus auf Kampfkraft durch Artefakte, Vermächtnisse (relativ, z. B. 0.08).
    pub fn military_bonus(&self, r: RealmId) -> f32 {
        self.artifact_bonus(r, |e| matches!(e, ArtifactEffect::MilitaryPower))
    }

    pub fn school_grade(&self, r: RealmId, s: School) -> u8 {
        self.realms[r as usize].research[s.idx()].grade
    }

    pub(crate) fn magic_month(&mut self) {
        let day = self.day;
        // Aschglas aus dem Lager der Provinzen in den Reichsvorrat überführen
        for r in 0..self.realms.len() {
            if !self.realms[r].alive || crate::diplomacy::is_antagonist(self.realms[r].kind) {
                continue;
            }
            let provs = self.provinces_of(r as RealmId).to_vec();
            let mut gathered = 0.0;
            for p in provs {
                let st = &mut self.provinces[p as usize].stock[Good::Ashglass.idx()];
                let take = *st * 0.5;
                *st -= take;
                gathered += take;
            }
            self.realms[r].ashglass += gathered;
            // Bücher verbrauchen die Bibliotheken
            let libs: f32 = self.provinces_of(r as RealmId).iter().map(|p| self.provinces[*p as usize].service_count(ServiceKind::Library) as f32).sum();
            let mut need = libs;
            for p in self.provinces_of(r as RealmId).to_vec() {
                if need <= 0.0 {
                    break;
                }
                let st = &mut self.provinces[p as usize].stock[Good::Books.idx()];
                let take = st.min(need);
                *st -= take;
                need -= take;
            }
        }
        // Forschung
        for r in 0..self.realms.len() as RealmId {
            if !self.realms[r as usize].alive || crate::diplomacy::is_antagonist(self.realms[r as usize].kind) {
                continue;
            }
            let Some(school) = self.realms[r as usize].research_focus else { continue };
            let mut fp = self.research_rate(r);
            if school.revered_by() == Some(self.realms[r as usize].culture) {
                fp /= 0.75;
            }
            let idx = school.idx();
            let grade = self.realms[r as usize].research[idx].grade as usize;
            if grade >= 4 {
                self.realms[r as usize].research_focus = None;
                continue;
            }
            self.realms[r as usize].research[idx].points += fp;
            let need = GRADE_POINTS[grade];
            let glass = GRADE_ASHGLASS[grade];
            if self.realms[r as usize].research[idx].points >= need && self.realms[r as usize].ashglass >= glass {
                self.realms[r as usize].ashglass -= glass;
                self.realms[r as usize].research[idx].points -= need;
                self.realms[r as usize].research[idx].grade += 1;
                let g = self.realms[r as usize].research[idx].grade;
                let e = Entry::new(ChronicleKind::Magic, "research", 30 + g * 5).realm(r).var("a", self.rname(r)).var("n", g.to_string()).var("c", school.name()).tag("research");
                self.chron(e);
                // Verbotene Künste: Entdeckungschance 30 % je Grad
                if school.forbidden() && self.realms[r as usize].laws.magic_law != 2 && self.chance(0.3) && !self.realms[r as usize].forbidden_discovered {
                    self.realms[r as usize].forbidden_discovered = true;
                    self.realms[r as usize].piety -= 50.0;
                    let ruler = self.realms[r as usize].ruler;
                    self.chars[ruler as usize].secrets.push(Secret { kind: SecretKind::Heresy, public: true, known_by: Vec::new() });
                    self.realms[r as usize].prestige -= 100.0;
                }
                // Nach dem Grad wechselt die KI das Forschungsziel
                if !self.is_player(r) {
                    let s2 = SCHOOLS[(school.idx() + 1 + self.ri(0, 2) as usize) % 9];
                    let next = if g >= 3 || self.chance(0.5) { s2 } else { school };
                    let pref = self.preferred_school(r);
                    self.realms[r as usize].research_focus = Some(if next.forbidden() { pref } else { next });
                }
            } else if self.realms[r as usize].research[idx].points > need * 1.5 {
                self.realms[r as usize].research[idx].points = need * 1.5;
            }
        }
        // Reichshohlung und Pfad
        for r in 0..self.realms.len() as RealmId {
            if !self.realms[r as usize].alive || crate::diplomacy::is_antagonist(self.realms[r as usize].kind) {
                continue;
            }
            self.hollow_path_month(r);
        }
        // Schleierstärke (12.7)
        self.veil_month();
        // Artefakte: Fund und Besitz
        self.artifacts_month();
        // Orden
        let n = self.mage_orders.len();
        for i in 0..n {
            let realm = self.mage_orders[i].realm;
            if let Some(r) = realm {
                if !self.realms[r as usize].alive {
                    self.mage_orders[i].realm = None;
                    continue;
                }
                self.mage_orders[i].members += 1;
                self.mage_orders[i].wealth += 5.0;
                if self.mage_orders[i].forbidden && self.realms[r as usize].laws.magic_law == 0 && self.chance(0.02) {
                    self.mage_orders[i].realm = None;
                    let r2 = &mut self.realms[r as usize];
                    r2.mage_orders.retain(|o| *o != i as u8);
                }
            }
        }
        let _ = day;
    }

    fn hollow_path_month(&mut self, r: RealmId) {
        let day = self.day;
        let rl = &self.realms[r as usize];
        let ruler_h = self.chars[rl.ruler as usize].hollowing;
        let provs = self.provinces_of(r);
        let avg_h = if provs.is_empty() { 0.0 } else { provs.iter().map(|p| self.provinces[*p as usize].hollowing).sum::<f32>() / provs.len() as f32 };
        let uses = rl.void_uses.iter().filter(|d| day.saturating_sub(**d) < 10 * DAYS_PER_YEAR).count() as f32;
        let value = 0.5 * ruler_h + 0.3 * avg_h + 0.2 * (uses * 5.0).min(100.0);
        let old = rl.hollow_path;
        let stage = HOLLOW_STAGES.iter().filter(|t| value >= **t).count() as u8;
        self.realms[r as usize].hollow_value = value;
        self.realms[r as usize].hollow_path = stage;
        if stage != old {
            let mut note = match stage {
                1 => "Die Stimme aus dem Tief erscheint in Ereignissen",
                2 => "Aschglasertrag steigt, Leerpriester dienen, Seelenasche tritt auf",
                3 => "Hohlknechte aus eigenen Gefallenen, Pakt mit dem Tief möglich",
                4 => "Die Hohlkrone altert nicht mehr",
                5 => "Das Tief erwacht – alle lebenden Reiche sammeln sich gegen dich",
                _ => "Reinigung gelingt: Das Tief zieht sich zurück",
            };
            if stage < old {
                note = "Der Schatten weicht";
            }
            let e = Entry::new(ChronicleKind::Hollowing, "hollow_stage", 35 + stage * 5).realm(r).var("a", self.rname(r)).var("n", stage.to_string()).var("c", note.to_string()).tag("hollow_stage");
            self.chron(e);
        }
        // Vorteile: Seelenasche (ab Stufe 2) aus Schlachtfeldern, Friedhöfen und Hohlschreinen
        if stage >= 2 {
            let mut ash = 0.0;
            for p in self.provinces_of(r).to_vec() {
                let pr = &self.provinces[p as usize];
                ash += pr.population() / 100.0 * 2.0 / 12.0 * (pr.hollowing / 100.0);
                if pr.shrine > 0 {
                    ash += 1.0;
                }
            }
            self.realms[r as usize].soul_ash += ash;
        }
        // Stufe 1+: Herrscher wird hohlberührt (Wissen +2, Hohlung steigt)
        if stage >= 1 {
            let ruler = self.realms[r as usize].ruler;
            if !self.chars[ruler as usize].has(crate::catalog::Trait::HollowTouched) && !self.chars[ruler as usize].has(crate::catalog::Trait::FlameSworn) && self.chars[ruler as usize].hollowing >= 10.0 {
                self.chars[ruler as usize].traits.push(crate::catalog::Trait::HollowTouched);
            }
        }
        // Stufe 3+: Hohlknechte aus Seelenasche (0,1 je Einheit und Monat Unterhalt)
        if stage >= 3 && self.realms[r as usize].soul_ash >= 1.0 {
            let n = (self.realms[r as usize].soul_ash).min(20.0) as u32;
            if n > 0 {
                self.realms[r as usize].soul_ash -= n as f32;
                let cap = self.realms[r as usize].capital;
                let existing = self.armies.iter().find(|a| a.owner == Some(r) && a.location == cap && a.kind == ArmyKind::Realm).map(|a| a.id);
                match existing {
                    Some(id) => {
                        let a = self.armies.iter_mut().find(|a| a.id == id).unwrap();
                        if let Some(g) = a.regiments.iter_mut().find(|g| g.unit == "hollow_thrall") {
                            g.count += n;
                        } else {
                            a.regiments.push(Regiment { unit: "hollow_thrall".into(), count: n, vet: 0, hp: 1.0 });
                        }
                    }
                    None => {
                        let nm = format!("Hohle von {}", self.rname(r));
                        self.new_army(ArmyKind::Realm, Some(r), cap, vec![Regiment { unit: "hollow_thrall".into(), count: n, vet: 0, hp: 1.0 }], &nm);
                    }
                }
            }
        }
        // Stufe 5: Bund der Letzten Flamme gegen das Reich
        if stage >= 5 && !self.endgame.league_targets.contains(&r) {
            let members: Vec<RealmId> = self.realms.iter().filter(|x| x.alive && x.id != r && !crate::diplomacy::is_antagonist(x.kind) && x.vassal_of != Some(r)).map(|x| x.id).collect();
            if members.len() >= 2 {
                self.endgame.league_targets.push(r);
                self.form_coalition(r, members, true);
            }
        }
        // Nachteile: Exkommunikation (Stufe 2+ jährlich) – siehe Religion; Fruchtbarkeit (Stufe 4+) siehe Charaktere.
        // Herrscherhohlung steigt langsam mit Aschglas-Fokus/Magienutzung
        let ruler = self.realms[r as usize].ruler;
        if stage >= 2 {
            self.chars[ruler as usize].hollowing = (self.chars[ruler as usize].hollowing + 0.3).min(100.0);
        }
    }

    /// Schleierstärke je Provinz nach 12.7 und globaler Schleierdruck.
    fn veil_month(&mut self) {
        let n = self.provinces.len();
        let rate = self.options.difficulty.mods().veil_rate;
        let pressure = self.endgame.veil_pressure;
        let mut delta = vec![0.0f32; n];
        for pi in 0..n {
            let p = &self.provinces[pi];
            let avg: f32 = if p.neighbors.is_empty() { p.veil } else { p.neighbors.iter().map(|l| self.provinces[l.to as usize].veil).sum::<f32>() / p.neighbors.len() as f32 };
            let mut d = 0.01 * (avg - p.veil);
            let faith = self.faith_strength(pi as ProvId);
            d -= 0.03 * faith * 0.3;
            // Reinigungsrituale: Wachfeuer in Gebieten mit Tempel
            if p.service_count(ServiceKind::Temple) > 0 && p.owner.is_some() {
                d -= 0.005;
            }
            // Globaler Druck
            if pressure > 20.0 {
                d += (pressure - 20.0) / 100.0 * 0.01 * rate;
            }
            // Natürliche Rückkehr zur Basis (hohe Werte nur durch Ursachen)
            if p.has_state("dead_rest", self.day) {
                d -= 0.005;
            }
            delta[pi] = d;
        }
        for pi in 0..n {
            let base: f32 = match self.provinces[pi].region {
                6 => 5.0,
                5 => 3.0,
                _ => 0.0,
            };
            let p = &mut self.provinces[pi];
            p.veil = (p.veil + delta[pi]).clamp(base.min(p.veil.max(base * 0.8)) * if base > 0.0 { 0.7 } else { 0.0 }, 5.0);
            // Hohlung der Bevölkerung wächst bei Schleier ≥ 3, sinkt sonst langsam
            if p.veil >= 3.0 {
                p.hollowing = (p.hollowing + 0.5 * rate).min(100.0);
            } else if p.veil < 1.5 {
                p.hollowing = (p.hollowing - 0.15).max(0.0);
            }
            // Gebiete jenseits der Schwelle: Ascheregen etc. werden über Ereignisse sichtbar
            if p.shrine == 0 && p.veil >= 3.0 && p.owner.is_some() {
                // Schreine entstehen mit 5 %/Jahr = ca. 0,4 %/Monat (Schleier ≥ 3)
            }
        }
    }

    fn artifacts_month(&mut self) {
        // Besitzer folgt dem Provinzbesitz; Funde über Chance bei Besitz
        for i in 0..self.artifacts.len() {
            let site = self.artifacts[i].site;
            let owner = self.provinces[site as usize].owner;
            if self.artifacts[i].holder.is_none() {
                if let Some(o) = owner
                    && !crate::diplomacy::is_antagonist(self.realms[o as usize].kind) {
                        let mut p = 0.004;
                        if self.provinces[site as usize].service_count(ServiceKind::Library) > 0 {
                            p *= 3.0;
                        }
                        if self.chance(p) {
                            self.artifacts[i].holder = Some(o);
                            self.realms[o as usize].artifacts.push(self.artifacts[i].def);
                            let def = &catalog::ARTIFACTS[self.artifacts[i].def as usize];
                            let name = self.pname(site);
                            let e = Entry::new(ChronicleKind::Discovery, "artifact", 38).prov(site, &name).realm(o).var("a", self.rname(o)).var("c", def.name.to_string());
                            self.chron(e);
                        }
                    }
            } else if let Some(h) = self.artifacts[i].holder {
                // Übergang bei Eroberung des Fundorts
                if let Some(o) = owner
                    && o != h && !crate::diplomacy::is_antagonist(self.realms[o as usize].kind) {
                        self.realms[h as usize].artifacts.retain(|a| *a != self.artifacts[i].def);
                        self.artifacts[i].holder = Some(o);
                        self.realms[o as usize].artifacts.push(self.artifacts[i].def);
                    }
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Rituale
    // ------------------------------------------------------------------------------------
    pub fn cast_ritual(&mut self, r: RealmId, ritual_id: &str, pid: ProvId) -> Result<String, String> {
        let def = catalog::ritual(ritual_id).ok_or("Unbekanntes Ritual")?;
        let day = self.day;
        if self.realms[r as usize].ritual_cooldown > day {
            return Err("Die Hofmagier ruhen noch (ein Ritual pro Monat)".into());
        }
        if self.school_grade(r, def.school) < def.grade {
            return Err(format!("Schule {} Grad {} nicht erforscht", def.school.name(), def.grade));
        }
        if self.realms[r as usize].laws.magic_law == 0 && !def.school.forbidden() && false {
            return Err("Magie ist verboten".into());
        }
        if self.provinces[pid as usize].owner != Some(r) && !matches!(def.effect, RitualEffect::SealOfTheVeil | RitualEffect::GrandPurge(_)) {
            return Err("Provinz gehört nicht dem Reich".into());
        }
        if self.realms[r as usize].gold < def.gold {
            return Err("Nicht genug Gold".into());
        }
        if self.realms[r as usize].ashglass < def.ashglass {
            return Err("Nicht genug Aschglas".into());
        }
        // Siegel nur in Solmere, mit allen drei Schulen Grad IV und fünf Götterschreinen
        if def.effect == RitualEffect::SealOfTheVeil {
            if self.provinces[pid as usize].key != "solmere" {
                return Err("Das Siegel kann nur in Solmere gewirkt werden".into());
            }
            for s in [School::Ember, School::Order, School::Passage] {
                if self.school_grade(r, s) < 4 {
                    return Err(format!("Schule {} Grad IV nötig", s.name()));
                }
            }
            if self.endgame.gods_shrines < 5 {
                return Err("Alle fünf Götterschreine müssen wiederhergestellt sein".into());
            }
            if self.provinces[pid as usize].owner != Some(r) {
                return Err("Solmere muss kontrolliert werden".into());
            }
        }
        self.realms[r as usize].gold -= def.gold;
        self.realms[r as usize].ashglass -= def.ashglass;
        self.realms[r as usize].ritual_cooldown = day + DAYS_PER_MONTH;
        let name = self.pname(pid);
        let note = match def.effect {
            RitualEffect::Purify(v) => {
                let p = &mut self.provinces[pid as usize];
                p.hollowing = (p.hollowing - v).max(0.0);
                format!("Die Hohlung in {name} sinkt um {v:.0}")
            }
            RitualEffect::HarvestBlessing => {
                self.provinces[pid as usize].states.insert("harvest_blessing".into(), day + 12 * DAYS_PER_MONTH);
                "Die Ernte wird gesegnet".to_string()
            }
            RitualEffect::Fortify => {
                let p = &mut self.provinces[pid as usize];
                p.fortification = (p.fortification + 1).min(10);
                p.states.insert("rune_wall".into(), day + 24 * DAYS_PER_MONTH);
                "Ein Runenwall erhebt sich".to_string()
            }
            RitualEffect::SealVeil(v) => {
                let p = &mut self.provinces[pid as usize];
                p.veil = (p.veil - v).max(0.0);
                "Der Schleier festigt sich".to_string()
            }
            RitualEffect::ResearchBurst(pts) => {
                if let Some(s) = self.realms[r as usize].research_focus {
                    self.realms[r as usize].research[s.idx()].points += pts;
                }
                "Gelehrte rechnen im Gleichklang".to_string()
            }
            RitualEffect::ShadowNetwork(v) => {
                if let Some(t) = self.realms.iter().filter(|x| x.alive && x.id != r && !crate::diplomacy::is_antagonist(x.kind) && self.realm_distance(r, x.id) == 1).map(|x| x.id).next() {
                    let e = self.realms[r as usize].spy_net.entry(t).or_insert(0.0);
                    *e = (*e + v).min(100.0);
                }
                "Flüstern in fremden Höfen".to_string()
            }
            RitualEffect::Calm(v) => {
                let p = &mut self.provinces[pid as usize];
                p.unrest = (p.unrest - v).max(0.0);
                "Das Volk beruhigt sich".to_string()
            }
            RitualEffect::DeadRest => {
                self.provinces[pid as usize].states.insert("dead_rest".into(), day + 12 * DAYS_PER_MONTH);
                "Die Toten ruhen".to_string()
            }
            RitualEffect::RendVeil => {
                let p = &mut self.provinces[pid as usize];
                p.veil = (p.veil + 1.0).min(5.0);
                p.hollowing = (p.hollowing + 30.0).min(100.0);
                p.shrine = p.shrine.max(1);
                "Der Schleier reißt".to_string()
            }
            RitualEffect::GrandPurge(v) => {
                for p in self.provinces_of(r).to_vec() {
                    let pr = &mut self.provinces[p as usize];
                    pr.hollowing = (pr.hollowing - v).max(0.0);
                }
                "Eine Ewige Wacht legt sich über das Reich".to_string()
            }
            RitualEffect::SealOfTheVeil => {
                self.endgame.seal_cast = Some(r);
                self.endgame.veil_pressure = (self.endgame.veil_pressure - 25.0).max(0.0);
                "Das Siegel des Schleiers ist gewirkt".to_string()
            }
            RitualEffect::CallHollow => {
                let nm = format!("Gerufene Hohle von {}", self.rname(r));
                self.new_army(ArmyKind::Realm, Some(r), pid, vec![Regiment { unit: "hollow_thrall".into(), count: 40, vet: 0, hp: 1.0 }], &nm);
                self.provinces[pid as usize].hollowing = (self.provinces[pid as usize].hollowing + 6.0).min(100.0);
                "Hohle folgen dem Ruf".to_string()
            }
        };
        // Risiken: Schleierriss, Hohlung des Wirkers
        let ruler = self.realms[r as usize].ruler;
        if self.chance(def.risk) {
            let p = &mut self.provinces[pid as usize];
            p.veil = (p.veil + 0.2).min(5.0);
            p.hollowing = (p.hollowing + 3.0).min(100.0);
            self.chars[ruler as usize].hollowing = (self.chars[ruler as usize].hollowing + 2.0).min(100.0);
        }
        if def.school == School::Void {
            self.realms[r as usize].void_uses.push(day);
            self.chars[ruler as usize].hollowing = (self.chars[ruler as usize].hollowing + 5.0).min(100.0);
            self.realms[r as usize].threat += 15.0;
            if self.realms[r as usize].religion == Religion::EternalFlame || self.realms[r as usize].laws.magic_law == 0 {
                self.realms[r as usize].piety -= 50.0;
            }
        }
        if def.school == School::Crystal && self.school_grade(r, School::Crystal) >= 4 {
            let p = &mut self.provinces[pid as usize];
            p.veil = (p.veil + 0.2).min(5.0);
        }
        let mag = if def.grade >= 4 { 55 } else { 22 };
        let e = Entry::new(ChronicleKind::Magic, "ritual", mag).prov(pid, &name).realm(r).var("a", self.rname(r)).var("c", def.name.to_string()).tag("ritual");
        self.chron(e);
        Ok(note)
    }

    /// Götterschrein wiederherstellen (5.000 Gold, 200 Aschglas, ein Held vor Ort; ab Endzeit).
    pub fn restore_shrine(&mut self, r: RealmId, pid: ProvId) -> Result<(), String> {
        let god = self.provinces[pid as usize].god_shrine.ok_or("Hier liegt kein Götterschrein")?;
        let day = self.day;
        if self.provinces[pid as usize].states.contains_key("shrine_restored") {
            return Err("Schrein bereits wiederhergestellt".into());
        }
        if self.provinces[pid as usize].owner != Some(r) {
            return Err("Provinz gehört nicht dem Reich".into());
        }
        if !self.endgame.started {
            return Err("Schreine können erst im Endspiel wiederhergestellt werden".into());
        }
        if self.shrine_quests.iter().any(|q| q.0 == pid) {
            return Err("Die Wiederherstellung läuft bereits".into());
        }
        if self.realms[r as usize].gold < 5000.0 || self.realms[r as usize].ashglass < 200.0 {
            return Err("5.000 Gold und 200 Aschglas nötig".into());
        }
        let hero = self.chars.iter().any(|c| c.alive() && c.realm == Some(r) && c.adult(day) && c.attr(Attr::Martial) >= 8 && c.location == pid && c.captive_of.is_none());
        let hero_near = hero || self.armies.iter().any(|a| a.owner == Some(r) && a.location == pid && a.commander.is_some());
        if !hero_near {
            return Err("Ein Held muss die Heldenquest vor Ort bestehen".into());
        }
        self.realms[r as usize].gold -= 5000.0;
        self.realms[r as usize].ashglass -= 200.0;
        let _ = god;
        // Heldenquest: 90 Tage Weltzeit
        self.shrine_quests.push((pid, r, day + 90));
        Ok(())
    }

    /// Schließt laufende Wiederherstellungen ab.
    pub(crate) fn shrines_month(&mut self) {
        let day = self.day;
        let done: Vec<(ProvId, RealmId, u32)> = self.shrine_quests.iter().copied().filter(|q| q.2 <= day).collect();
        self.shrine_quests.retain(|q| q.2 > day);
        for (pid, r, _) in done {
            if !self.realms[r as usize].alive || self.provinces[pid as usize].owner != Some(r) {
                continue;
            }
            let Some(god) = self.provinces[pid as usize].god_shrine else { continue };
            self.provinces[pid as usize].states.insert("shrine_restored".into(), u32::MAX);
            self.endgame.gods_shrines += 1;
            self.endgame.veil_pressure = (self.endgame.veil_pressure - 10.0).max(0.0);
            self.realms[r as usize].shrines_restored += 1;
            self.realms[r as usize].prestige += 300.0;
            let name = ["Ithren", "Morvaine", "Calyx", "Thal", "Oruun"].get(god as usize).copied().unwrap_or("der Götter");
            let pn = self.pname(pid);
            let e = Entry::new(ChronicleKind::Religion, "shrine_restored", 70).prov(pid, &pn).realm(r).var("a", self.rname(r)).var("c", format!("Schrein {name}"));
            self.chron(e);
        }
    }

    /// Wurde der Schrein eines Gottes (Index 0–4) wiederhergestellt?
    pub fn shrine_restored(&self, god: u8) -> bool {
        self.provinces.iter().any(|p| p.god_shrine == Some(god) && p.states.contains_key("shrine_restored"))
    }

    /// Orden und Artefakte anlegen (Startzustand).
    pub(crate) fn init_magic(&mut self) {
        let defs: [(&str, School, &str, bool); 8] = [
            ("Orden der Glutklinge", School::Ember, "ember_crown", false),
            ("Wächter der Schwelle", School::Passage, "ember_crown", false),
            ("Akademie von Arnvale", School::Crystal, "ember_crown", false),
            ("Zirkel der Wurzeln", School::Growth, "thorn_covenant", false),
            ("Konklave der Gezeiten", School::Tide, "saelith_compact", false),
            ("Runenmeister von Kharnfels", School::Stone, "skarn_clans", false),
            ("Hexenzirkel der Narbe", School::Shadow, "ember_crown", true),
            ("Orden der Leeren Hand", School::Void, "hollowed", true),
        ];
        for (i, (name, school, faction, forbidden)) in defs.iter().enumerate() {
            let realm = self.realms.iter().find(|r| r.key == *faction).map(|r| r.id);
            let realm = if *forbidden { if *faction == "hollowed" { realm } else { None } } else { realm };
            if let Some(r) = realm {
                self.realms[r as usize].mage_orders.push(i as u8);
            }
            self.mage_orders.push(MageOrder { id: i as u8, name: name.to_string(), school: *school, realm, members: 20 + i as u32 * 3, wealth: 200.0, forbidden: *forbidden });
        }
        self.artifacts.clear();
        for (i, d) in catalog::ARTIFACTS.iter().enumerate() {
            let site = self.prov_by_key(d.site).unwrap_or_else(|| (i as ProvId * 7 + 3) % self.provinces.len() as ProvId);
            self.artifacts.push(ArtifactState { def: i as u8, holder: None, site, effect: d.effect });
        }
    }
}
