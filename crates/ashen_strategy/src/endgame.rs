//! Endgame: Großmacht-Reaktion, uralte Bedrohungen mit Eskalation (Schleierdruck) und die
//! alternativen Siegziele mit Prüfung (GDD Kapitel 16).

use crate::chronicle::{ChronicleKind, Entry};
use crate::diplomacy::is_antagonist;
use crate::model::*;
use crate::types::*;
use crate::world::{StrategyEvent, StrategyWorld};
use serde::{Deserialize, Serialize};

/// Die Siegziele der Spezifikation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum VictoryKind {
    /// 60 % der Provinzen direkt oder durch Vasallen und der Titel Hochkönig.
    CrownOfEldraeth,
    /// Der Hohle König besiegt und der Hohlschrein in Solmere zerstört.
    HollowKingFalls,
    /// Fünf Götterschreine und das Erzritual „Siegel“ in Solmere, Schleierdruck unter 20.
    SealOfTheVeil,
    /// 50 % des Handelsvolumens, 100.000 Gold, zehn Jahre gehalten.
    TradeDominion,
    /// Eigene Religion in 65 % der Provinzen und alle heiligen Stätten kontrolliert.
    FaithVictory,
    /// Dynastieprestige ≥ 25.000 und Dynastiemitglieder auf den Thronen der vier lebenden Hauptfraktionen.
    DynasticLegend,
    /// Hohlungsstufe 5 und Solmere kontrollieren.
    HollowCrown,
    /// Festgelegte Zeitspanne überstehen (Survival/Challenge).
    Survival,
    /// Niederlage aller: Eldraeth fällt ins Hohle Tief (kein Siegziel, nur Endzustand).
    WorldFalls,
}

impl VictoryKind {
    pub fn name(self) -> &'static str {
        match self {
            VictoryKind::CrownOfEldraeth => "Krone von Eldraeth",
            VictoryKind::HollowKingFalls => "Der Hohle König fällt",
            VictoryKind::SealOfTheVeil => "Siegel des Schleiers",
            VictoryKind::TradeDominion => "Handelsherrschaft",
            VictoryKind::FaithVictory => "Glaubenssieg",
            VictoryKind::DynasticLegend => "Dynastische Legende",
            VictoryKind::HollowCrown => "Die Hohle Krone",
            VictoryKind::Survival => "Überleben",
            VictoryKind::WorldFalls => "Der Schleierbruch",
        }
    }
    pub const ALL: [VictoryKind; 8] = [VictoryKind::CrownOfEldraeth, VictoryKind::HollowKingFalls, VictoryKind::SealOfTheVeil, VictoryKind::TradeDominion, VictoryKind::FaithVictory, VictoryKind::DynasticLegend, VictoryKind::HollowCrown, VictoryKind::Survival];
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct VictoryRecord {
    pub realm: RealmId,
    pub kind: VictoryKind,
    pub day: u32,
}

/// Fortschritt eines Siegziels für die Oberfläche.
#[derive(Clone, Debug)]
pub struct VictoryProgress {
    pub kind: VictoryKind,
    /// 0–1
    pub progress: f32,
    pub detail: String,
    pub achieved: bool,
}

impl StrategyWorld {
    /// Anteil der Provinzen (direkt + Vasallen) eines Reiches.
    pub fn province_share(&self, r: RealmId) -> f32 {
        let total = self.provinces.len() as f32;
        let mut n = self.provinces_of(r).len() as f32;
        for v in self.realms.iter().filter(|x| x.alive && x.vassal_of == Some(r)) {
            n += self.provinces_of(v.id).len() as f32;
        }
        n / total.max(1.0)
    }

    pub(crate) fn endgame_month(&mut self) {
        self.shrines_month();
        let day = self.day;
        let diff = self.options.difficulty.mods();
        // Schleierdruck: +1 je 5 Jahre, +0,5 je Provinz mit Schleier ≥ 4 (pro Jahr)
        let deep = self.provinces.iter().filter(|p| p.veil >= 4.0).count() as f32;
        self.endgame.veil_pressure += (1.0 / 60.0 + 0.5 * deep / 12.0 * 0.5) * diff.veil_rate;
        self.endgame.veil_pressure = self.endgame.veil_pressure.clamp(0.0, 100.0);
        let pressure = self.endgame.veil_pressure;
        // Start des Endgames
        if !self.endgame.started && (self.endgame.great_power.is_some() || pressure > 60.0 || self.year() >= 1000) {
            self.endgame.started = true;
            self.endgame.started_day = Some(day);
            let e = Entry::new(ChronicleKind::Endgame, "endgame_stage", 70).var("a", "Eldraeth").var("c", "Das Endspiel beginnt: Die Schatten der alten Zeit werden länger");
            self.chron(e);
        }
        // Schwellen
        if pressure >= 40.0 && !self.endgame.legion_awake {
            self.endgame.legion_awake = true;
            self.zeal[Religion::SunHeresy.idx()] = (self.zeal[Religion::SunHeresy.idx()] + 20.0).min(100.0);
            let e = Entry::new(ChronicleKind::Endgame, "endgame_stage", 65).var("a", "Eldraeth").var("c", "Ruf aus Solmere: Die Aschenlegion verlässt ihre Patrouillenrouten");
            self.chron(e);
            self.endgame.started = true;
        }
        if pressure >= 60.0 && self.endgame.hollow_king_awake.is_none() && self.boss("boss_hollow_king").is_some_and(|b| b.state != BossState::Defeated) {
            self.endgame.hollow_king_awake = Some(day);
            let e = Entry::new(ChronicleKind::Endgame, "endgame_stage", 80).var("a", "Eldraeth").var("c", "Das Erwachen des Hohlen Königs: Varkhan Aurelith regt sich in Solmere");
            self.chron(e);
            self.found_league();
        }
        if pressure >= 70.0 && !self.endgame.dragon_wrath {
            self.endgame.dragon_wrath = true;
            let e = Entry::new(ChronicleKind::Endgame, "endgame_stage", 60).var("a", "Eldraeth").var("c", "Vyrmathax' Zorn: Der Aschdrache fliegt nun jährlich und frisst Aschglas");
            self.chron(e);
        }
        if pressure >= 80.0 && !self.endgame.withering {
            self.endgame.withering = true;
            if let Some(b) = self.bosses.iter_mut().find(|b| b.id == "boss_root_mother")
                && b.state == BossState::Dormant {
                    b.state = BossState::Active;
                }
            let e = Entry::new(ChronicleKind::Endgame, "endgame_stage", 60).var("a", "Eldraeth").var("c", "Das Welken: Die Wurzelmutter schließt den Dornwald");
            self.chron(e);
        }
        if pressure >= 100.0 && !self.endgame.veil_break {
            self.endgame.veil_break = true;
            self.endgame.last_veil_break_year = self.year();
            let e = Entry::new(ChronicleKind::Endgame, "endgame_stage", 90).var("a", "Eldraeth").var("c", "Der Schleierbruch beginnt: Jedes Jahr zerreißt der Schleier in drei Provinzen");
            self.chron(e);
        }
        // Wiederkehr der Stimmen: fünf Schreine
        if self.endgame.gods_shrines >= 5 && self.endgame.plague_magic_year != Some(-1) {
            self.endgame.plague_magic_year = Some(-1);
            let e = Entry::new(ChronicleKind::Endgame, "endgame_stage", 85).var("a", "Eldraeth").var("c", "Die Stimmen der alten Götter kehren zurück, und der Schleierdruck weicht");
            self.chron(e);
            self.endgame.veil_pressure = (self.endgame.veil_pressure - 10.0).max(0.0);
        }
        // Magische Seuche
        if pressure >= 55.0 && self.endgame.plague_magic_year != Some(self.year()) && self.endgame.plague_magic_year != Some(-1) && self.chance(0.012) {
            let cands: Vec<ProvId> = self.provinces.iter().filter(|p| p.hollowing > 30.0 && p.plague.is_none() && p.population() > 200.0).map(|p| p.id).collect();
            if !cands.is_empty() {
                let i = self.ri(0, cands.len() as i32 - 1) as usize;
                self.start_plague(cands[i], 3, 0.9);
                self.endgame.plague_magic_year = Some(self.year());
            }
        }
        for p in self.provinces.iter_mut().filter(|p| p.plague.as_ref().is_some_and(|pl| pl.kind == 3)) {
            p.hollowing = (p.hollowing + 1.0).min(100.0);
        }
        // Großmacht: Kirchliche Wachsamkeit
        if let Some(g) = self.endgame.great_power
            && self.realms[g as usize].hollow_path >= 2 {
                let flame: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && r.religion == Religion::EternalFlame && r.id != g && r.vassal_of != Some(g) && !is_antagonist(r.kind) && !self.at_war(r.id, g)).map(|r| r.id).collect();
                for f in flame {
                    if self.chance(0.08) && !self.has_treaty(TreatyKind::Truce, f, g) {
                        let _ = self.declare_war(f, g, CasusBelli::HolyWar, WarGoal::RevertConquests);
                        let e = Entry::new(ChronicleKind::Religion, "holy_war", 60).realm(f).var("b", self.rname(g));
                        self.chron(e);
                    }
                }
            }
        // Bund der Letzten Flamme: Kriegskasse und „Verrat am Licht“
        self.league_month();
        // Rang: Hochkönig
        let ids: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind)).map(|r| r.id).collect();
        for r in ids {
            let share = self.province_share(r);
            if self.realms[r as usize].rank == Rank::King && share >= 0.45 {
                self.realms[r as usize].rank = Rank::HighKing;
                let rn = self.rname(r);
                let e = Entry::new(ChronicleKind::Coronation, "event_generic", 70).realm(r).var("a", rn.clone()).var("p", rn.clone()).var("c", format!("Der Herrscher von {rn} wurde zum Hochkönig ausgerufen"));
                self.chron(e);
            }
        }
    }

    fn found_league(&mut self) {
        let members: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind) && r.hollow_path < 3).map(|r| r.id).collect();
        self.endgame.league_of_last_flame = members;
        let e = Entry::new(ChronicleKind::Endgame, "endgame_stage", 75).var("a", "Eldraeth").var("c", "Der Bund der Letzten Flamme wird gegründet: Die Lebenden stehen gegen die Legion");
        self.chron(e);
    }

    fn league_month(&mut self) {
        if self.endgame.hollow_king_awake.is_none() {
            return;
        }
        let members = self.endgame.league_of_last_flame.clone();
        let mut chest = 0.0;
        for &m in &members {
            if !self.realms[m as usize].alive {
                continue;
            }
            let c = self.realms[m as usize].income.max(0.0) * 0.05;
            self.realms[m as usize].gold -= c;
            chest += c;
        }
        self.league_chest += chest;
        // Verrat am Licht: Kriege untereinander
        let day = self.day;
        let wars: Vec<(RealmId, RealmId)> = self.wars.iter().filter(|w| w.active && w.start + 5 > day.saturating_sub(DAYS_PER_MONTH)).map(|w| (w.leader_a, w.leader_d)).collect();
        for (a, d) in wars {
            if members.contains(&a) && members.contains(&d) {
                for &m in &members {
                    if m != a {
                        self.add_opinion(m, a, OpKind::TreasonToLight, -30.0, Some(day + 50 * DAYS_PER_YEAR));
                    }
                }
            }
        }
        // Kriegskasse verteilen: an das Mitglied an der Front
        if self.league_chest > 800.0 {
            let target = members.iter().copied().filter(|m| self.realms[*m as usize].alive).max_by(|a, b| {
                let f = |r: RealmId| self.provinces_of(r).iter().filter(|p| self.provinces[**p as usize].neighbors.iter().any(|l| self.provinces[l.to as usize].owner.is_some_and(|o| is_antagonist(self.realms[o as usize].kind)))).count();
                f(*a).cmp(&f(*b))
            });
            if let Some(t) = target {
                self.realms[t as usize].gold += self.league_chest;
                self.league_chest = 0.0;
            }
        }
    }

    pub(crate) fn endgame_year(&mut self) {
        self.heresy_year();
        let year = self.year();
        // Die Legion marschiert jährlich gegen das stärkste lebende Reich
        if let Some(since) = self.endgame.hollow_king_awake
            && self.endgame.last_legion_march_year != year {
                self.endgame.last_legion_march_year = year;
                let yrs = (self.day.saturating_sub(since)) / DAYS_PER_YEAR;
                let size = 300 + 10 * yrs;
                let target = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind)).max_by(|a, b| a.strength.total_cmp(&b.strength)).map(|r| r.id);
                let legion = self.realm_of_kind(RealmKind::Legion);
                if let (Some(t), Some(l)) = (target, legion) {
                    let start = self.prov_by_key("solmere").unwrap_or(self.realms[l as usize].capital);
                    let regs = vec![Regiment { unit: "ashen_legionnaire".into(), count: size * 7 / 10, vet: 2, hp: 1.0 }, Regiment { unit: "cinder_knight".into(), count: size * 2 / 10, vet: 2, hp: 1.0 }, Regiment { unit: "hollow_archer".into(), count: size / 10, vet: 1, hp: 1.0 }];
                    let aid = self.new_army(ArmyKind::Realm, Some(l), start, regs, "Marsch der Aschenlegion");
                    // Ziel: nächste Provinz des Zielreichs
                    let goal = self.provinces_of(t).iter().copied().min_by(|a, b| {
                        let d = |p: ProvId| (self.provinces[p as usize].centroid - self.provinces[start as usize].centroid).length();
                        d(*a).total_cmp(&d(*b))
                    });
                    if let Some(g) = goal {
                        let _ = self.order_move(aid, g);
                    }
                    let e = Entry::new(ChronicleKind::Endgame, "legion_march", 65).realm(t).var("a", self.rname(t)).var("n", size.to_string());
                    self.chron(e);
                }
            }
        // Schleierbruch: 3 zufällige Provinzen +1; nach ~30 Jahren fällt Eldraeth
        if self.endgame.veil_break && self.endgame.last_veil_break_year != year {
            self.endgame.last_veil_break_year = year;
            for _ in 0..3 {
                let i = self.ri(0, self.provinces.len() as i32 - 1) as usize;
                let p = &mut self.provinces[i];
                p.veil = (p.veil + 1.0).min(5.0);
            }
            self.veil_break_years += 1;
            if self.veil_break_years >= 30 && self.victory.is_none() {
                let hollow = self.realm_of_kind(RealmKind::Hollowed).unwrap_or(0);
                self.victory = Some(VictoryRecord { realm: hollow, kind: VictoryKind::WorldFalls, day: self.day });
                let e = Entry::new(ChronicleKind::Victory, "victory", 100).var("a", "Das Hohle Tief").var("c", "Eldraeth fällt ins Hohle Tief");
                self.chron(e);
                self.emit(StrategyEvent::Victory { realm: hollow, kind: VictoryKind::WorldFalls });
            }
        }
        // Kriegerische Auswirkungen auf die Weltkarte: Wiederkehr der alten Ordnung entfällt
    }

    // ------------------------------------------------------------------------------------
    // Siegbedingungen
    // ------------------------------------------------------------------------------------
    /// Fortschritt aller Siegziele eines Reiches.
    pub fn victory_progress(&self, r: RealmId) -> Vec<VictoryProgress> {
        let rl = &self.realms[r as usize];
        let mut out = Vec::new();
        let share = self.province_share(r);
        out.push(VictoryProgress { kind: VictoryKind::CrownOfEldraeth, progress: (share / 0.6).min(1.0), detail: format!("{:.0} % der Provinzen, Rang {}", share * 100.0, rl.rank.name()), achieved: share >= 0.6 && rl.rank == Rank::HighKing });
        let king_down = self.boss("boss_hollow_king").is_some_and(|b| b.state == BossState::Defeated && b.defeated_by.is_some_and(|d| d == r || self.same_lord(d, r) || self.has_treaty(TreatyKind::Alliance, d, r) || self.endgame.league_of_last_flame.contains(&d) && self.endgame.league_of_last_flame.contains(&r)));
        let solmere_shrine_gone = self.prov_by_key("solmere").is_some_and(|p| self.provinces[p as usize].shrine == 0 && self.provinces[p as usize].owner.is_some_and(|o| !is_antagonist(self.realms[o as usize].kind)));
        out.push(VictoryProgress { kind: VictoryKind::HollowKingFalls, progress: (king_down as u8 as f32 + solmere_shrine_gone as u8 as f32) / 2.0, detail: format!("Hohler König besiegt: {king_down}, Hohlschrein von Solmere zerstört: {solmere_shrine_gone}"), achieved: king_down && solmere_shrine_gone });
        let grades = [School::Ember, School::Order, School::Passage].iter().map(|s| self.school_grade(r, *s) as f32 / 4.0).sum::<f32>() / 3.0;
        let seal = self.endgame.seal_cast == Some(r) && self.endgame.gods_shrines >= 5 && self.endgame.veil_pressure < 20.0;
        out.push(VictoryProgress { kind: VictoryKind::SealOfTheVeil, progress: ((self.endgame.gods_shrines as f32 / 5.0) * 0.5 + grades * 0.3 + if self.endgame.seal_cast == Some(r) { 0.2 } else { 0.0 }).min(1.0), detail: format!("{} von 5 Schreinen, Schleierdruck {:.0}", self.endgame.gods_shrines, self.endgame.veil_pressure), achieved: seal });
        let ts = self.trade_share(r);
        let months = self.trade_hold.get(&r).copied().unwrap_or(0);
        out.push(VictoryProgress { kind: VictoryKind::TradeDominion, progress: ((ts / 0.5).min(1.0) * 0.4 + (rl.gold / 100_000.0).clamp(0.0, 1.0) * 0.3 + (months as f32 / 120.0).min(1.0) * 0.3).min(1.0), detail: format!("Handelsanteil {:.0} %, Schatz {:.0}, {} Monate gehalten", ts * 100.0, rl.gold, months), achieved: months >= 120 });
        let fs = self.faith_share(rl.religion);
        let sites = self.holds_all_holy_sites(r, rl.religion);
        out.push(VictoryProgress { kind: VictoryKind::FaithVictory, progress: (fs / 0.65).min(1.0) * if sites { 1.0 } else { 0.7 }, detail: format!("{:.0} % der Provinzen, heilige Stätten kontrolliert: {sites}", fs * 100.0), achieved: fs >= 0.65 && sites });
        let dyn_id = self.chars[rl.ruler as usize].dynasty;
        let prestige = self.dynasties[dyn_id as usize].prestige;
        let thrones = self.main_thrones_of(dyn_id);
        out.push(VictoryProgress { kind: VictoryKind::DynasticLegend, progress: ((prestige / 25_000.0).min(1.0) + thrones as f32 / 4.0) / 2.0, detail: format!("Prestige {:.0}, Throne {thrones} von 4", prestige), achieved: prestige >= 25_000.0 && thrones >= 4 });
        let solmere_mine = self.prov_by_key("solmere").is_some_and(|p| self.provinces[p as usize].owner == Some(r));
        out.push(VictoryProgress { kind: VictoryKind::HollowCrown, progress: (rl.hollow_path as f32 / 5.0) * if solmere_mine { 1.0 } else { 0.6 }, detail: format!("Hohlungsstufe {}, Solmere: {solmere_mine}", rl.hollow_path), achieved: rl.hollow_path >= 5 && solmere_mine });
        if let Some(crate::world::Challenge::Survival(years)) = self.options.rules.challenge {
            let el = (self.year() - START_YEAR) as f32;
            out.push(VictoryProgress { kind: VictoryKind::Survival, progress: (el / years as f32).min(1.0), detail: format!("{el:.0} von {years} Jahren"), achieved: el >= years as f32 });
        }
        out
    }

    /// Anzahl der Hauptfraktions-Throne, auf denen Mitglieder der Dynastie sitzen.
    fn main_thrones_of(&self, d: DynId) -> u32 {
        let mut n = 0;
        for f in ["ember_crown", "skarn_clans", "thorn_covenant", "saelith_compact"] {
            if let Some(r) = self.realms.iter().find(|r| r.alive && r.faction == f && matches!(r.kind, RealmKind::Kingdom | RealmKind::Clan | RealmKind::Covenant | RealmKind::Republic))
                && self.chars[r.ruler as usize].dynasty == d {
                    n += 1;
                }
        }
        n
    }

    pub(crate) fn victory_check(&mut self) {
        if self.victory.is_some() {
            return;
        }
        // Handelsjahre zählen
        let ids: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !is_antagonist(r.kind)).map(|r| r.id).collect();
        for &r in &ids {
            let ok = self.trade_share(r) >= 0.5 && self.realms[r as usize].gold >= 100_000.0;
            let e = self.trade_hold.entry(r).or_insert(0);
            if ok {
                *e += 1;
            } else {
                *e = 0;
            }
        }
        let rules = self.options.rules.victories.clone();
        // Spieler zuerst, dann KI nach ID
        let mut order: Vec<RealmId> = Vec::new();
        if let Some(p) = self.player {
            order.push(p);
        }
        order.extend(ids.iter().copied().filter(|r| Some(*r) != self.player));
        for r in order {
            if !self.realms[r as usize].alive {
                continue;
            }
            // Spielerbezogene Herausforderungen: nur der Spieler kann gewinnen
            if self.options.rules.challenge.is_some() && Some(r) != self.player {
                continue;
            }
            let prog = self.victory_progress(r);
            for p in prog {
                if p.achieved && rules.contains(&p.kind) {
                    self.victory = Some(VictoryRecord { realm: r, kind: p.kind, day: self.day });
                    let e = Entry::new(ChronicleKind::Victory, "victory", 100).realm(r).var("a", self.rname(r)).var("c", p.kind.name().to_string()).tag("victory");
                    self.chron(e);
                    self.emit(StrategyEvent::Victory { realm: r, kind: p.kind });
                    return;
                }
            }
        }
    }
}
