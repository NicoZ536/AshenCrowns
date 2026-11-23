//! Spieler-Aktionen als Befehlsdaten (`StrategyCommand`). Alle anderen Reiche handeln per KI.

use crate::catalog::StressAct;
use crate::diplomacy::{DiplomacyOutcome, DiplomaticAction};
use crate::model::*;
use crate::politics::HouseAction;
use crate::types::*;
use crate::world::StrategyWorld;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LawChange {
    CrownAuthority(u8),
    Succession(SuccLaw),
    Serfdom(u8),
    TownRights(u8),
    FaithLaw(u8),
    MagicLaw(u8),
    MartialLaw(u8),
    VassalTax(f32),
    Levy(f32),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StrategyCommand {
    // Reichsverwaltung
    SetTaxLevel(u8),
    SetTradeTax(f32),
    SetTariffs { import: f32, transit: f32 },
    SetLaw(LawChange),
    AppointOffice { office: Office, character: CharId },
    DismissOffice(Office),
    SetOfficeTask { office: Office, task: u8 },
    SetResearchFocus(Option<School>),
    TakeLoan { source: LoanSource, amount: f32, years: u32 },
    Build { province: ProvId, building: String },
    UpgradeFortification(ProvId),
    // Militär
    Recruit { province: ProvId, unit: String, count: u32 },
    MoveArmy { army: ArmyId, to: ProvId },
    MergeArmies { into: ArmyId, from: ArmyId },
    SplitArmy { army: ArmyId, take: Vec<(String, u32)> },
    DisbandArmy(ArmyId),
    AssignCommander { army: ArmyId, character: CharId },
    StormSiege(ProvId),
    HireMercenaries(u32),
    // Diplomatie
    Diplomacy { target: RealmId, action: DiplomaticAction },
    AnswerPeaceOffer { offer: u32, accept: bool },
    // Politik und Intrigen
    HouseAction { house: HouseId, action: HouseAction },
    StartPlot { leader: CharId, kind: PlotKind, target: PlotTarget },
    RecruitConspirator { plot: PlotId, character: CharId },
    TriggerPlot(PlotId),
    // Dynastie
    ArrangeMarriage { own: CharId, other: CharId, dowry: f32 },
    SetHeir(CharId),
    SetEducation { child: CharId, focus: Attr },
    GiftRegent(f32),
    Abdicate,
    // Religion und Magie
    CastRitual { ritual: String, province: ProvId },
    RestoreShrine(ProvId),
    PayDragonTribute,
    // Ereignisse
    ChooseEvent { pending: u32, option: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub ok: bool,
    pub message: String,
}

impl CommandResult {
    fn ok(m: impl Into<String>) -> CommandResult {
        CommandResult { ok: true, message: m.into() }
    }
    fn err(m: impl Into<String>) -> CommandResult {
        CommandResult { ok: false, message: m.into() }
    }
}

fn r2c(r: Result<String, String>) -> CommandResult {
    match r {
        Ok(m) => CommandResult::ok(m),
        Err(e) => CommandResult::err(e),
    }
}

fn d2c(o: DiplomacyOutcome) -> CommandResult {
    CommandResult { ok: o.accepted, message: o.message }
}

impl StrategyWorld {
    /// Führt einen Spielerbefehl für das Spielerreich aus.
    pub fn execute(&mut self, cmd: StrategyCommand) -> CommandResult {
        let Some(r) = self.player else { return CommandResult::err("Kein Spielerreich") };
        self.execute_for(r, cmd)
    }

    /// Führt einen Befehl für ein bestimmtes Reich aus (Mehrspieler, Tests).
    pub fn execute_for(&mut self, r: RealmId, cmd: StrategyCommand) -> CommandResult {
        if !self.realms[r as usize].alive {
            return CommandResult::err("Das Reich besteht nicht mehr");
        }
        let day = self.day;
        match cmd {
            StrategyCommand::SetTaxLevel(l) => {
                let l = l.min(4);
                let regency = self.realms[r as usize].regency.is_some();
                let cur = self.realms[r as usize].tax_level;
                if regency && (l as i32 - cur as i32).abs() > 1 {
                    return CommandResult::err("Unter Regentschaft nur eine Stufe pro Jahr");
                }
                if l > cur {
                    self.ruler_act(r, StressAct::RaiseTaxes);
                }
                self.realms[r as usize].tax_level = l;
                CommandResult::ok("Steuerstufe gesetzt")
            }
            StrategyCommand::SetTradeTax(t) => {
                self.realms[r as usize].trade_tax = (t / 5.0).round().clamp(0.0, 4.0) * 5.0;
                CommandResult::ok("Handelssteuer gesetzt")
            }
            StrategyCommand::SetTariffs { import, transit } => {
                let rl = &mut self.realms[r as usize];
                rl.tariff_import = import.clamp(0.0, 30.0);
                rl.tariff_transit = transit.clamp(0.0, 15.0);
                CommandResult::ok("Zölle gesetzt")
            }
            StrategyCommand::SetLaw(l) => self.set_law(r, l),
            StrategyCommand::AppointOffice { office, character } => r2c(self.appoint_office(r, office, character).map(|_| "Amt vergeben".into())),
            StrategyCommand::DismissOffice(o) => r2c(self.dismiss_office(r, o).map(|_| "Entlassen".into())),
            StrategyCommand::SetOfficeTask { office, task } => {
                let n = office.tasks().len() as u8;
                if task >= n {
                    return CommandResult::err("Unbekannte Aufgabe");
                }
                self.realms[r as usize].office_task[office.idx()] = task;
                CommandResult::ok("Aufgabe gesetzt")
            }
            StrategyCommand::SetResearchFocus(s) => {
                self.realms[r as usize].research_focus = s;
                CommandResult::ok("Forschung gesetzt")
            }
            StrategyCommand::TakeLoan { source, amount, years } => r2c(self.take_loan(r, source, amount, years).map(|_| "Kredit aufgenommen".into())),
            StrategyCommand::Build { province, building } => {
                if self.provinces[province as usize].owner != Some(r) {
                    return CommandResult::err("Provinz gehört nicht dem Reich");
                }
                let Some(bi) = crate::goods::building_index(&building) else { return CommandResult::err("Unbekanntes Gebäude") };
                r2c(self.build(province, bi, false).map(|_| "Bau abgeschlossen".into()))
            }
            StrategyCommand::UpgradeFortification(p) => {
                if self.provinces[p as usize].owner != Some(r) {
                    return CommandResult::err("Provinz gehört nicht dem Reich");
                }
                let lvl = self.provinces[p as usize].fortification;
                let cost = 120.0 * (lvl as f32 + 1.0);
                if lvl >= 10 {
                    return CommandResult::err("Maximale Befestigung");
                }
                if self.realms[r as usize].gold < cost {
                    return CommandResult::err("Nicht genug Gold");
                }
                self.realms[r as usize].gold -= cost;
                self.provinces[p as usize].fortification += 1;
                CommandResult::ok("Befestigung ausgebaut")
            }
            StrategyCommand::Recruit { province, unit, count } => r2c(self.recruit(r, province, &unit, count).map(|_| "Rekrutiert".into())),
            StrategyCommand::MoveArmy { army, to } => {
                if self.army(army).and_then(|a| a.owner) != Some(r) {
                    return CommandResult::err("Heer gehört nicht dem Reich");
                }
                r2c(self.order_move(army, to).map(|_| "Marschbefehl erteilt".into()))
            }
            StrategyCommand::MergeArmies { into, from } => {
                if self.army(into).and_then(|a| a.owner) != Some(r) {
                    return CommandResult::err("Heer gehört nicht dem Reich");
                }
                r2c(self.merge_armies(into, from).map(|_| "Heere vereint".into()))
            }
            StrategyCommand::SplitArmy { army, take } => {
                if self.army(army).and_then(|a| a.owner) != Some(r) {
                    return CommandResult::err("Heer gehört nicht dem Reich");
                }
                r2c(self.split_army(army, &take).map(|id| format!("Neues Heer {id}")))
            }
            StrategyCommand::DisbandArmy(id) => {
                if self.army(id).and_then(|a| a.owner) != Some(r) {
                    return CommandResult::err("Heer gehört nicht dem Reich");
                }
                self.disband_army(id);
                CommandResult::ok("Heer aufgelöst")
            }
            StrategyCommand::AssignCommander { army, character } => {
                if self.army(army).and_then(|a| a.owner) != Some(r) || self.chars[character as usize].realm != Some(r) || !self.chars[character as usize].alive() {
                    return CommandResult::err("Ungültige Zuordnung");
                }
                if let Some(old) = self.army(army).and_then(|a| a.commander) {
                    self.chars[old as usize].army = None;
                }
                self.armies.iter_mut().find(|a| a.id == army).unwrap().commander = Some(character);
                self.chars[character as usize].army = Some(army);
                CommandResult::ok("Befehlshaber ernannt")
            }
            StrategyCommand::StormSiege(p) => {
                let ok = self.provinces[p as usize].siege.as_ref().is_some_and(|s| s.besieger == r);
                if !ok {
                    return CommandResult::err("Keine eigene Belagerung");
                }
                r2c(self.storm_siege(p).map(|_| "Sturmangriff".into()))
            }
            StrategyCommand::HireMercenaries(c) => r2c(self.hire_mercenaries(r, c).map(|_| "Söldner angeworben".into())),
            StrategyCommand::Diplomacy { target, action } => {
                if matches!(action, DiplomaticAction::DeclareWar { .. }) && self.realms[r as usize].regency.is_some() {
                    return CommandResult::err("Unter Regentschaft ist Krieg erklären gesperrt");
                }
                d2c(self.diplomacy(r, target, action))
            }
            StrategyCommand::AnswerPeaceOffer { offer, accept } => {
                if self.peace_offers.iter().find(|o| o.id == offer).map(|o| o.to) != Some(r) {
                    return CommandResult::err("Angebot gehört nicht dem Reich");
                }
                r2c(self.answer_peace_offer(offer, accept).map(|_| "Antwort übermittelt".into()))
            }
            StrategyCommand::HouseAction { house, action } => r2c(self.house_action(r, house, action)),
            StrategyCommand::StartPlot { leader, kind, target } => {
                if self.chars[leader as usize].realm != Some(r) {
                    return CommandResult::err("Anführer gehört nicht zum Hof");
                }
                r2c(self.start_plot(leader, kind, target, Some(r)).map(|id| format!("Komplott {id} begonnen")))
            }
            StrategyCommand::RecruitConspirator { plot, character } => r2c(self.recruit_conspirator(plot, character).map(|_| "Mitverschwörer gewonnen".into())),
            StrategyCommand::TriggerPlot(p) => {
                let ok = self.plots.iter().find(|x| x.id == p).is_some_and(|x| self.chars[x.leader as usize].realm == Some(r));
                if !ok {
                    return CommandResult::err("Komplott nicht gefunden");
                }
                if self.trigger_plot(p) {
                    CommandResult::ok("Das Komplott gelang")
                } else {
                    CommandResult::err("Das Komplott scheiterte")
                }
            }
            StrategyCommand::ArrangeMarriage { own, other, dowry } => {
                let Some(t) = self.chars[other as usize].realm else { return CommandResult::err("Kandidat ohne Hof") };
                if t == r {
                    if self.chars[own as usize].spouse.is_some() || self.chars[other as usize].spouse.is_some() || self.close_kin(own, other) || self.chars[own as usize].gender == self.chars[other as usize].gender {
                        return CommandResult::err("Ungeeignetes Paar");
                    }
                    self.marry(own, other, false);
                    return CommandResult::ok("Vermählt");
                }
                d2c(self.diplomacy(r, t, DiplomaticAction::ProposeMarriage { own, other, dowry }))
            }
            StrategyCommand::SetHeir(c) => {
                if !self.chars[c as usize].alive() || self.chars[c as usize].realm != Some(r) {
                    return CommandResult::err("Ungültiger Erbe");
                }
                if self.realms[r as usize].laws.succession != SuccLaw::Designation {
                    return CommandResult::err("Erben bestimmen nur bei Erbgesetz „Ernennung“");
                }
                if self.realms[r as usize].legitimacy < 70.0 {
                    return CommandResult::err("Legitimität ≥ 70 nötig");
                }
                self.realms[r as usize].heir = Some(c);
                CommandResult::ok("Erbe bestimmt")
            }
            StrategyCommand::SetEducation { child, focus } => {
                if self.chars[child as usize].realm != Some(r) {
                    return CommandResult::err("Kind gehört nicht zum Hof");
                }
                self.chars[child as usize].education = Some(focus);
                CommandResult::ok("Erziehungsfokus gesetzt")
            }
            StrategyCommand::GiftRegent(gold) => {
                let Some(reg) = self.realms[r as usize].regency else { return CommandResult::err("Keine Regentschaft") };
                if gold <= 0.0 || self.realms[r as usize].gold < gold {
                    return CommandResult::err("Nicht genug Gold");
                }
                self.realms[r as usize].gold -= gold;
                let rg = self.realms[r as usize].regency.as_mut().unwrap();
                rg.loyalty = (reg.loyalty + gold / 30.0).min(100.0);
                CommandResult::ok("Regentenloyalität gestiegen")
            }
            StrategyCommand::Abdicate => {
                let ruler = self.realms[r as usize].ruler;
                match self.find_heir(r, ruler) {
                    Some((h, _)) if self.chars[h as usize].adult(day) => {
                        let name = self.full_name(ruler);
                        let e = crate::chronicle::Entry::new(crate::chronicle::ChronicleKind::Succession, "abdication", 50).realm(r).var("a", name);
                        self.chron(e);
                        self.kill_char(ruler, DeathCause::Abdicated);
                        CommandResult::ok("Abgedankt")
                    }
                    _ => CommandResult::err("Kein volljähriger Erbe"),
                }
            }
            StrategyCommand::CastRitual { ritual, province } => r2c(self.cast_ritual(r, &ritual, province)),
            StrategyCommand::RestoreShrine(p) => r2c(self.restore_shrine(r, p).map(|_| "Die Wiederherstellung hat begonnen (90 Tage)".into())),
            StrategyCommand::PayDragonTribute => {
                if self.realms[r as usize].gold < 400.0 || self.realms[r as usize].ashglass < 5.0 {
                    return CommandResult::err("400 Gold und 5 Aschglas nötig");
                }
                let Some(bi) = self.bosses.iter().position(|b| b.id == "boss_ashen_dragon" && b.state != BossState::Defeated) else { return CommandResult::err("Kein Drache") };
                self.realms[r as usize].gold -= 400.0;
                self.realms[r as usize].ashglass -= 5.0;
                self.bosses[bi].tribute_until = day + 2 * DAYS_PER_YEAR;
                CommandResult::ok("Der Drache schweigt zwei Jahre")
            }
            StrategyCommand::ChooseEvent { pending, option } => {
                if self.events.pending.iter().find(|p| p.id == pending).map(|p| p.realm) != Some(r) {
                    return CommandResult::err("Fenster gehört nicht dem Reich");
                }
                r2c(self.choose_event_option(pending, &option).map(|_| "Entschieden".into()))
            }
        }
    }

    fn set_law(&mut self, r: RealmId, l: LawChange) -> CommandResult {
        let day = self.day;
        let legit = self.realms[r as usize].legitimacy;
        let cost_mult = if legit > 80.0 { 0.5 } else { 1.0 };
        match l {
            LawChange::Succession(s) => {
                if day.saturating_sub(self.realms[r as usize].laws.last_succession_change) < 20 * DAYS_PER_YEAR && self.realms[r as usize].laws.last_succession_change > 0 {
                    return CommandResult::err("Erbgesetz kann nur alle 20 Jahre geändert werden");
                }
                let cost = 500.0 * cost_mult;
                if self.realms[r as usize].prestige < cost {
                    return CommandResult::err(format!("{cost:.0} Prestige nötig"));
                }
                if !self.council_approves(r, None) {
                    return CommandResult::err("Der Adelsrat verweigert die Zustimmung");
                }
                self.realms[r as usize].prestige -= cost;
                self.realms[r as usize].laws.succession = s;
                self.realms[r as usize].laws.last_succession_change = day;
            }
            LawChange::CrownAuthority(v) => {
                self.realms[r as usize].laws.crown_authority = v.min(3);
                let maxtax = [20.0, 30.0, 40.0, 50.0][v.min(3) as usize];
                let rl = &mut self.realms[r as usize];
                rl.laws.vassal_tax = rl.laws.vassal_tax.min(maxtax);
            }
            LawChange::Serfdom(v) => self.realms[r as usize].laws.serfdom = v.min(2),
            LawChange::TownRights(v) => self.realms[r as usize].laws.town_rights = v.min(2),
            LawChange::FaithLaw(v) => self.realms[r as usize].laws.faith_law = v.min(2),
            LawChange::MagicLaw(v) => self.realms[r as usize].laws.magic_law = v.min(2),
            LawChange::MartialLaw(v) => self.realms[r as usize].laws.martial_law = v.min(2),
            LawChange::VassalTax(v) => {
                let maxtax = [20.0, 30.0, 40.0, 50.0][self.realms[r as usize].laws.crown_authority.min(3) as usize];
                if v > 30.0 && !self.council_approves(r, None) {
                    return CommandResult::err("Abgaben über 30 % benötigen die Zustimmung des Adelsrats");
                }
                let old = self.realms[r as usize].laws.vassal_tax;
                self.realms[r as usize].laws.vassal_tax = v.clamp(0.0, maxtax);
                let _ = old;
            }
            LawChange::Levy(v) => self.realms[r as usize].laws.levy = v.clamp(0.0, 60.0),
        }
        CommandResult::ok("Gesetz geändert")
    }
}
