//! Weltchronik: aus der Simulation erzeugte Erzählsätze (GDD Kapitel 17).
//!
//! Jeder Eintrag besitzt Jahr, Akteure, Kategorie, Bedeutsamkeit und Ursachenverweise. Die Texte
//! entstehen aus Vorlagen mit Varianten; die Variante wird deterministisch aus der Eintrags-ID gewählt.

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ChronicleKind {
    Battle,
    Siege,
    Death,
    Birth,
    Marriage,
    Succession,
    Coronation,
    Plot,
    War,
    Peace,
    Disaster,
    Plague,
    Famine,
    Revolt,
    Founding,
    Construction,
    Discovery,
    Religion,
    Hollowing,
    BossDefeated,
    Legend,
    Diplomacy,
    Economy,
    Magic,
    Endgame,
    Realm,
    Victory,
    Omen,
}

impl ChronicleKind {
    pub fn label(self) -> &'static str {
        match self {
            ChronicleKind::Battle => "Schlacht",
            ChronicleKind::Siege => "Belagerung",
            ChronicleKind::Death => "Tod",
            ChronicleKind::Birth => "Geburt",
            ChronicleKind::Marriage => "Hochzeit",
            ChronicleKind::Succession => "Thronfolge",
            ChronicleKind::Coronation => "Krönung",
            ChronicleKind::Plot => "Intrige",
            ChronicleKind::War => "Krieg",
            ChronicleKind::Peace => "Frieden",
            ChronicleKind::Disaster => "Katastrophe",
            ChronicleKind::Plague => "Seuche",
            ChronicleKind::Famine => "Hungersnot",
            ChronicleKind::Revolt => "Aufstand",
            ChronicleKind::Founding => "Gründung",
            ChronicleKind::Construction => "Bau",
            ChronicleKind::Discovery => "Entdeckung",
            ChronicleKind::Religion => "Glaube",
            ChronicleKind::Hollowing => "Hohlung",
            ChronicleKind::BossDefeated => "Bosssieg",
            ChronicleKind::Legend => "Legende",
            ChronicleKind::Diplomacy => "Diplomatie",
            ChronicleKind::Economy => "Wirtschaft",
            ChronicleKind::Magic => "Magie",
            ChronicleKind::Endgame => "Endzeit",
            ChronicleKind::Realm => "Reich",
            ChronicleKind::Victory => "Sieg",
            ChronicleKind::Omen => "Vorzeichen",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChronicleActor {
    pub id: String,
    pub name: String,
    pub role: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Secret,
    Rumor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChronicleEntry {
    pub id: u64,
    pub date: Date,
    pub day: u32,
    pub kind: ChronicleKind,
    pub actors: Vec<ChronicleActor>,
    pub province: Option<ProvId>,
    pub realm: Option<RealmId>,
    /// Bedeutsamkeit 0–100.
    pub magnitude: u8,
    pub causes: Vec<u64>,
    pub numbers: BTreeMap<String, f32>,
    pub tags: Vec<String>,
    pub template: String,
    pub text: String,
    pub visibility: Visibility,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Chronicle {
    pub entries: Vec<ChronicleEntry>,
    pub next_id: u64,
}

/// Bauplan eines Eintrags.
pub struct Entry {
    pub kind: ChronicleKind,
    pub template: &'static str,
    pub magnitude: u8,
    pub province: Option<ProvId>,
    pub realm: Option<RealmId>,
    pub actors: Vec<ChronicleActor>,
    pub causes: Vec<u64>,
    pub numbers: Vec<(&'static str, f32)>,
    pub tags: Vec<&'static str>,
    pub vars: Vec<(&'static str, String)>,
    pub visibility: Visibility,
}

impl Entry {
    pub fn new(kind: ChronicleKind, template: &'static str, magnitude: u8) -> Entry {
        Entry { kind, template, magnitude, province: None, realm: None, actors: Vec::new(), causes: Vec::new(), numbers: Vec::new(), tags: Vec::new(), vars: Vec::new(), visibility: Visibility::Public }
    }
    pub fn prov(mut self, p: ProvId, name: &str) -> Entry {
        self.province = Some(p);
        self.vars.push(("p", name.to_string()));
        self
    }
    pub fn realm(mut self, r: RealmId) -> Entry {
        self.realm = Some(r);
        self
    }
    pub fn var(mut self, k: &'static str, v: impl Into<String>) -> Entry {
        self.vars.push((k, v.into()));
        self
    }
    pub fn actor(mut self, id: impl Into<String>, name: impl Into<String>, role: &str) -> Entry {
        self.actors.push(ChronicleActor { id: id.into(), name: name.into(), role: role.to_string() });
        self
    }
    pub fn cause(mut self, c: Option<u64>) -> Entry {
        if let Some(c) = c {
            self.causes.push(c);
        }
        self
    }
    pub fn num(mut self, k: &'static str, v: f32) -> Entry {
        self.numbers.push((k, v));
        self
    }
    pub fn tag(mut self, t: &'static str) -> Entry {
        self.tags.push(t);
        self
    }
    pub fn secret(mut self) -> Entry {
        self.visibility = Visibility::Secret;
        self
    }
}

impl Chronicle {
    pub fn push(&mut self, day: u32, e: Entry) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let date = Date::from_day(day);
        let mut vars = e.vars.clone();
        vars.push(("n", e.numbers.first().map(|(_, v)| format!("{}", *v as i64)).unwrap_or_default()));
        let text = narrate(e.template, id, date, &vars);
        let mut magnitude = e.magnitude;
        // Verkettung: Ursachenereignisse werden nachträglich aufgewertet (+5 je Folge, max. +30).
        for c in &e.causes {
            if let Some(ce) = self.entries.get_mut(*c as usize) {
                let f = ce.numbers.get("followups").copied().unwrap_or(0.0);
                if f < 6.0 {
                    ce.numbers.insert("followups".into(), f + 1.0);
                    ce.magnitude = ce.magnitude.saturating_add(5).min(100);
                }
            }
        }
        magnitude = magnitude.min(100);
        self.entries.push(ChronicleEntry {
            id,
            date,
            day,
            kind: e.kind,
            actors: e.actors,
            province: e.province,
            realm: e.realm,
            magnitude,
            causes: e.causes,
            numbers: e.numbers.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
            tags: e.tags.iter().map(|t| t.to_string()).collect(),
            template: e.template.to_string(),
            text,
            visibility: e.visibility,
        });
        id
    }

    pub fn get(&self, id: u64) -> Option<&ChronicleEntry> {
        // Einträge sind nach ID sortiert und lückenlos.
        self.entries.get(id as usize).filter(|e| e.id == id)
    }

    /// Einträge eines Jahres ab einer Bedeutsamkeit (Jahreschronik ≥ 25, Herrscherchronik ≥ 50, Zeitalter ≥ 75).
    pub fn year(&self, year: i32, min_magnitude: u8) -> Vec<&ChronicleEntry> {
        self.entries.iter().filter(|e| e.date.year == year && e.magnitude >= min_magnitude && e.visibility == Visibility::Public).collect()
    }

    /// Lesbarer Fließtext der bedeutsamen Ereignisse.
    pub fn narrative(&self, min_magnitude: u8) -> String {
        self.entries.iter().filter(|e| e.magnitude >= min_magnitude && e.visibility == Visibility::Public).map(|e| e.text.clone()).collect::<Vec<_>>().join("\n")
    }

    /// Kausalkette eines Eintrags (von der Ursache zur Wirkung).
    pub fn chain(&self, id: u64) -> Vec<&ChronicleEntry> {
        let mut out = Vec::new();
        let mut cur = self.get(id);
        let mut guard = 0;
        while let Some(e) = cur {
            out.push(e);
            guard += 1;
            if guard > 32 {
                break;
            }
            cur = e.causes.first().and_then(|c| self.get(*c));
        }
        out.reverse();
        out
    }

    pub fn count_kind(&self, k: ChronicleKind) -> usize {
        self.entries.iter().filter(|e| e.kind == k).count()
    }
}

fn when_phrase(date: Date, variant: u64) -> String {
    match variant % 4 {
        0 => format!("Im Jahr {} n.F.", date.year),
        1 => format!("Im {} des Jahres {} n.F.", date.month_name(), date.year),
        2 => format!("{} n.F.", date.year),
        _ => format!("Im {} {} n.F.", date.month_name(), date.year),
    }
}

/// Wendet eine Vorlage an. Unbekannte Vorlagen ergeben einen neutralen Satz.
pub fn narrate(template: &str, id: u64, date: Date, vars: &[(&'static str, String)]) -> String {
    let variants = TEMPLATES.iter().find(|(k, _)| *k == template).map(|(_, v)| *v).unwrap_or(&["{when} geschah Bemerkenswertes in {p}."]);
    let hash = id.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 33;
    let t = variants[(hash as usize) % variants.len()];
    let mut out = t.replace("{when}", &when_phrase(date, hash >> 3));
    for (k, v) in vars {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

/// Textvorlagen mit Varianten.
pub const TEMPLATES: &[(&str, &[&str])] = &[
    ("battle_win", &[
        "{when} schlug {a} bei {p} das Heer von {b}; {n} Gefallene blieben auf dem Feld.",
        "{when} siegte {a} in der Schlacht um {p} über {b}. Das Feld gehörte am Abend den Raben.",
        "{when} trafen bei {p} {a} und {b} aufeinander. {a} behielt das Feld und zählte {n} Tote.",
        "{when} brach {b} unter dem Ansturm von {a} bei {p}. Man nennt den Ort seither blutig.",
    ]),
    ("battle_win_defense", &[
        "{when} hielt {a} die Linie bei {p} und warf {b} zurück; {n} Männer und Frauen fielen.",
        "{when} wies {a} den Angriff von {b} bei {p} ab. Die Mauern standen, die Reihen lichteten sich.",
        "{when} verteidigte {a} {p} gegen {b} und bezahlte den Sieg mit {n} Toten.",
    ]),
    ("battle_boss", &[
        "{when} fiel {b} vor den Heeren von {a} bei {p}. Die Chronisten schrieben den Namen zweimal.",
        "{when} zog {a} gegen {b} bei {p}. {n} fielen, doch {b} fiel zuerst.",
        "{when} besiegte {a} bei {p} das Grauen von {b}. Man sang davon noch Jahrzehnte später.",
    ]),
    ("siege_fall", &[
        "{when} fiel {p} nach langer Belagerung an {a}.",
        "{when} öffneten die Verteidiger von {p} ihre Tore vor {a}; das Lager war leer, die Zuversicht auch.",
        "{when} ergab sich {p} dem Heer von {a}. Die Banner wurden gewechselt, die Gesichter blieben.",
    ]),
    ("siege_start", &[
        "{when} schloss {a} die Stadt {p} ein.",
        "{when} zog {a} Gräben um {p} und wartete auf den Hunger.",
    ]),
    ("province_conquered", &[
        "{when} nahm {a} die Provinz {p} von {b} in Besitz.",
        "{when} wechselte {p} den Herrn: {a} löste {b} ab.",
        "{when} wehte über {p} das Banner von {a}; {b} zog ab.",
    ]),
    ("death_natural", &[
        "{when} starb {a} im Alter von {n} Jahren, friedlich, wie es hieß.",
        "{when} schloss {a} die Augen, {n} Jahre alt. Der Hof legte Trauer an.",
        "{when} wurde {a} zu Grabe getragen; {n} Winter hatte die Person gesehen.",
        "{when} erlosch {a}, {n} Jahre alt. Ein Kapitel des Hauses war zu Ende.",
    ]),
    ("death_battle", &[
        "{when} fiel {a} im Kampf, {n} Jahre alt.",
        "{when} kehrte {a} nicht aus der Schlacht zurück. Das Schwert hatte die letzte Antwort gegeben.",
        "{when} starb {a} auf dem Schlachtfeld, den Blick zum Feind gewandt.",
    ]),
    ("death_murder", &[
        "{when} starb {a} an einem Becher Wein, den niemand eingeschenkt haben wollte.",
        "{when} wurde {a} tot aufgefunden. Die Klinge war nicht die eigene.",
        "{when} fiel {a} einem Mordanschlag zum Opfer; der Hof schwieg wie ein Grab.",
        "{when} fand man {a} im Morgengrauen, kalt und ohne Zeugen.",
    ]),
    ("death_disease", &[
        "{when} raffte das Fieber {a} dahin.",
        "{when} erlag {a} der Seuche, die {p} heimsuchte.",
        "{when} starb {a} nach Wochen auf dem Krankenlager.",
    ]),
    ("death_executed", &[
        "{when} wurde {a} auf Befehl des Hofes hingerichtet.",
        "{when} endete {a} auf dem Richtplatz; die Menge schwieg.",
    ]),
    ("abdication", &[
        "{when} legte {a} die Krone nieder und zog sich zurück.",
        "{when} dankte {a} ab, müde der Last der Herrschaft.",
        "{when} übergab {a} das Zepter und ging in die Stille eines Klosters.",
    ]),
    ("deposed", &[
        "{when} wurde {a} abgesetzt; {b} griff nach der Macht.",
        "{when} stürzten die Häuser {a} vom Thron. {b} ließ sich ausrufen.",
    ]),
    ("birth", &[
        "{when} wurde {a} im Haus {b} geboren.",
        "{when} kam {a} zur Welt, Kind des Hauses {b}.",
        "{when} schenkte das Haus {b} der Welt {a}.",
    ]),
    ("marriage", &[
        "{when} wurden {a} und {b} vermählt.",
        "{when} feierte man die Hochzeit von {a} und {b}; die Gäste zählten ihre Vorteile.",
        "{when} verband die Ehe von {a} und {b} zwei Häuser.",
    ]),
    ("marriage_alliance", &[
        "{when} besiegelten {a} und {b} ihr Bündnis durch eine Heirat.",
        "{when} knüpfte die Ehe zwischen {a} und {b} ein Band, das Verträge allein nie geschaffen hätten.",
    ]),
    ("succession", &[
        "{when} bestieg {a} den Thron von {b}.",
        "{when} folgte {a} auf den Thron von {b}.",
        "{when} wurde {a} zum Herrscher über {b} ausgerufen.",
        "{when} nahm {a} die Krone von {b} auf, nicht jeder Blick im Saal war freundlich.",
    ]),
    ("regency", &[
        "{when} übernahm {a} die Regentschaft für das Kind {b}.",
        "{when} führte {a} die Geschäfte von {p}, bis {b} alt genug sein würde.",
    ]),
    ("dynasty_extinct", &[
        "{when} erlosch das Haus {a} mit dem Tod des Letzten seines Blutes.",
        "{when} starb mit {b} das Haus {a} aus. Sein Wappen hängt noch, sein Name verblasst.",
    ]),
    ("dynasty_change", &[
        "{when} ging die Krone von {b} an das Haus {a} über.",
        "{when} begann in {b} die Herrschaft des Hauses {a}.",
    ]),
    ("war_declared", &[
        "{when} erklärte {a} dem Reich {b} den Krieg ({n}).",
        "{when} zog {a} gegen {b} ins Feld; der Grund war {c}.",
        "{when} sagte {a} {b} die Fehde an. {c} hieß der Vorwand.",
        "{when} brach der Krieg zwischen {a} und {b} aus.",
    ]),
    ("peace_white", &[
        "{when} schlossen {a} und {b} Frieden, ohne dass einer gewonnen hätte.",
        "{when} endete der Krieg zwischen {a} und {b} ohne Sieger.",
    ]),
    ("peace_win", &[
        "{when} diktierte {a} {b} den Frieden.",
        "{when} unterwarf sich {b} im Frieden dem Willen von {a}.",
        "{when} endete der Krieg: {a} erhielt, was es gefordert hatte, {b} blieb die Rechnung.",
    ]),
    ("plot_success", &[
        "{when} gelang {a} ein Komplott ({c}) gegen {b}.",
        "{when} schlug {a} zu: Das Komplott ({c}) gegen {b} ging auf.",
    ]),
    ("plot_discovered", &[
        "{when} wurde das Komplott ({c}) von {a} gegen {b} aufgedeckt.",
        "{when} flog die Verschwörung ({c}) des {a} auf; {b} ließ Wachen aufziehen.",
    ]),
    ("plot_failed", &[
        "{when} scheiterte {a} mit einem Komplott ({c}) gegen {b}.",
    ]),
    ("drought", &[
        "{when} blieb der Regen über {p} aus, und die Brunnen sanken.",
        "{when} verdorrte das Land um {p}: Drei Monde lang fiel kein Tropfen.",
        "{when} brannte die Sonne auf {p}; das Korn stand welk auf den Feldern.",
        "{when} nannte man den Himmel über {p} ein Kupferdach.",
    ]),
    ("harvest_failed", &[
        "{when} verdarb die Ernte in {p}, nur {n} von hundert Teilen kamen in die Scheunen.",
        "{when} brachte {p} kaum die Hälfte der Ernte ein ({n} von hundert).",
    ]),
    ("price_spike", &[
        "{when} stieg in {p} der Preis für Brot auf das {n}-fache der Zeit davor.",
        "{when} wurde das Brot in {p} unbezahlbar; die Händler nannten Zahlen, die Armen nannten sie Hohn.",
    ]),
    ("famine", &[
        "{when} brach in {p} die Hungersnot aus; {n} Menschen starben.",
        "{when} leerten sich die Speicher von {p}, und der Hunger hielt Einzug.",
        "{when} starben in {p} die Schwachen zuerst, dann die Hoffnungsvollen.",
    ]),
    ("unrest", &[
        "{when} gärte es in {p}; auf den Märkten wurde nicht mehr gehandelt, sondern geflüstert.",
        "{when} brannten in {p} die ersten Speicher.",
    ]),
    ("revolt", &[
        "{when} erhob sich das Volk von {p} gegen {b}; {n} Aufständische griffen zu den Waffen.",
        "{when} begann in {p} der Aufstand gegen {b}.",
        "{when} stürmten die Bauern von {p} die Burg, die Fahnen von {b} fielen.",
    ]),
    ("revolt_crushed", &[
        "{when} schlug {a} den Aufstand in {p} nieder.",
        "{when} endete der Aufstand von {p}; {a} hielt Gericht.",
    ]),
    ("rebel_state", &[
        "{when} sagte sich {p} von {b} los und gründete das Reich {a}.",
        "{when} entstand aus den Aufständischen von {p} das neue Reich {a}.",
    ]),
    ("plague_start", &[
        "{when} brach in {p} die Seuche aus: {c} ging um.",
        "{when} zog {c} durch {p}, und die Totenglocken fanden kein Ende.",
        "{when} trug der Wind {c} nach {p}.",
    ]),
    ("plague_end", &[
        "{when} erlosch die Seuche in {p} nach {n} Toten.",
        "{when} schwand das Fieber aus {p}. Es hatte {n} Leben genommen.",
    ]),
    ("hollowing_rise", &[
        "{when} sank {p} tiefer in die Hohlung (nun {n}).",
        "{when} wurde die Luft über {p} grau; die Hohlung stieg auf {n}.",
    ]),
    ("shrine_appears", &[
        "{when} erhob sich in {p} ein Hohlschrein aus der Erde.",
        "{when} entstand bei {p} ein neuer Hohlschrein, nachts flackerte fahles Licht.",
    ]),
    ("hollow_conquest", &[
        "{when} fiel {p} an die Hohlen.",
        "{when} gingen die Lichter von {p} aus; die Hohlen hielten das Land.",
    ]),
    ("hollow_liberated", &[
        "{when} befreite {a} {p} aus dem Griff der Hohlen.",
        "{when} wurde {p} unter Feuer gereinigt; {a} zog ein.",
    ]),
    ("hollow_stage", &[
        "{when} erreichte {a} die Stufe {n} der Hohlung: {c}.",
        "{when} neigte sich das Reich {a} dem Tief zu ({c}).",
    ]),
    ("boss_defeated", &[
        "{when} fiel {b} durch die Hand von {a} bei {p}.",
        "{when} ward {b} bei {p} bezwungen; {a} nahm den Ruhm.",
    ]),
    ("boss_awakes", &[
        "{when} erwachte {b} in {p}; die Welt hielt den Atem an.",
        "{when} regte sich {b} bei {p}.",
    ]),
    ("dragon_flight", &[
        "{when} brannte der Himmel über {p} drei Tage lang; Vyrmathax zog über die Felder.",
        "{when} flog Vyrmathax über {p}. Als er in die Krater zurückkehrte, lagen die Felder in Asche.",
    ]),
    ("dragon_tribute", &[
        "{when} zahlte {a} dem Drachen Tribut; Vyrmathax schwieg {n} Jahre.",
    ]),
    ("legion_march", &[
        "{when} marschierte die Aschenlegion gegen {a}; {n} Mann stark.",
        "{when} setzte sich die Legion des Hohlen Königs in Bewegung, Ziel war {a}.",
    ]),
    ("endgame_stage", &[
        "{when} geschah, was die Alten fürchteten: {c}.",
        "{when} änderte sich die Welt: {c}.",
        "{when} begann ein neuer Abschnitt der Not: {c}.",
    ]),
    ("great_power", &[
        "{when} wurde {a} zur Großmacht; die Kronen schlossen sich zusammen.",
        "{when} warf {a} den Schatten über Eldraeth, den alle sahen.",
    ]),
    ("coalition", &[
        "{when} schlossen sich {c} gegen {b} zusammen.",
        "{when} entstand eine Koalition gegen {b}: {c}.",
    ]),
    ("alliance", &[
        "{when} schlossen {a} und {b} ein Bündnis.",
        "{when} reichten sich {a} und {b} die Hand zum Bund.",
    ]),
    ("treaty_broken", &[
        "{when} brach {a} den Vertrag mit {b}.",
    ]),
    ("boycott", &[
        "{when} verhängte {a} einen Handelsboykott gegen {b}.",
        "{when} sperrte {a} seine Häfen für die Waren von {b}.",
    ]),
    ("tribute", &[
        "{when} verpflichtete sich {b}, {a} Tribut zu zahlen.",
    ]),
    ("vassalized", &[
        "{when} beugte {b} das Knie vor {a} und wurde dessen Vasall.",
    ]),
    ("independence", &[
        "{when} erkämpfte sich {a} die Unabhängigkeit von {b}.",
    ]),
    ("realm_fell", &[
        "{when} verlor {a} den letzten Landstrich und hörte auf zu bestehen.",
        "{when} endete das Reich {a}; was blieb, waren Lieder und Schulden.",
    ]),
    ("realm_founded", &[
        "{when} entstand das Reich {a}.",
    ]),
    ("bankruptcy", &[
        "{when} erklärte {a} den Staatsbankrott; die Gläubiger tobten.",
    ]),
    ("conversion", &[
        "{when} wandte sich {p} dem {c} zu.",
        "{when} wechselte {p} den Glauben: Nun galt dort der {c}.",
    ]),
    ("heresy", &[
        "{when} keimte in {p} die Sonnenhäresie auf; Wanderprediger sprachen von der Rückkehr des Letzten Sonnenkönigs.",
    ]),
    ("excommunication", &[
        "{when} sprach der Erzflammenwahrer den Bann über {a} aus.",
    ]),
    ("holy_war", &[
        "{when} rief der Erzflammenwahrer den Heiligen Krieg gegen {b} aus.",
    ]),
    ("research", &[
        "{when} erreichte {a} den Grad {n} der Schule {c}.",
        "{when} gelang den Gelehrten von {a} ein Durchbruch in der Schule {c} (Grad {n}).",
    ]),
    ("ritual", &[
        "{when} wirkte {a} in {p} das Ritual {c}.",
    ]),
    ("artifact", &[
        "{when} wurde in {p} ein Artefakt geborgen: {c}.",
    ]),
    ("shrine_restored", &[
        "{when} wurde der Schrein {c} von {a} wiederhergestellt; ein Segen legte sich über Eldraeth.",
    ]),
    ("victory", &[
        "{when} erfüllte {a} das Siegziel: {c}.",
    ]),
    ("legend", &[
        "{when} wurde aus {p} eine Legende: {c}.",
    ]),
    ("event_generic", &[
        "{when} ereignete sich in {p}: {c}.",
        "{when} trug sich in {a} zu: {c}.",
    ]),
    ("faction_ultimatum", &[
        "{when} stellten die Häuser von {a} der Krone ein Ultimatum: {c}.",
    ]),
    ("house_rebels", &[
        "{when} erhob sich das Haus {a} gegen {b}.",
        "{when} erklärte Haus {a} den Gehorsam für beendet und griff zu den Waffen gegen {b}.",
    ]),
    ("bandits", &[
        "{when} überfielen Banditen die Straßen um {p}.",
    ]),
    ("raid", &[
        "{when} fielen {c} aus den Schatten über {p} her; {n} fielen.",
    ]),
    ("trade_route", &[
        "{when} öffnete sich ein neuer Handelsweg zwischen {a} und {b}.",
    ]),
    ("omen", &[
        "{when} meldeten Boten aus {p} üble Vorzeichen: {c}.",
    ]),
];
