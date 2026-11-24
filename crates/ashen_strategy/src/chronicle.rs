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
        "{when} stand {a} bei {p} dem Heer von {b} gegenüber. Als die Sonne sank, zählte man {n} Gefallene und fragte nicht mehr nach dem Sieger.",
        "{when} warf {a} bei {p} die Banner von {b} in den Staub; {n} Seelen gingen ungezählt zu den Raben.",
    ]),
    ("battle_win_defense", &[
        "{when} hielt {a} die Linie bei {p} und warf {b} zurück; {n} Männer und Frauen fielen.",
        "{when} wies {a} den Angriff von {b} bei {p} ab. Die Mauern standen, die Reihen lichteten sich.",
        "{when} verteidigte {a} {p} gegen {b} und bezahlte den Sieg mit {n} Toten.",
        "{when} brach sich der Zorn von {b} an den Reihen von {a} bei {p}. {n} Tote bezahlten, was die Chronisten Standhaftigkeit nennen.",
        "{when} stand {a} bei {p} wie ein Fels im Fluss; {b} zog ab, und {n} lagen im Gras.",
        "{when} schlug {a} den Sturm von {b} bei {p} zurück. Die Verwundeten füllten die Kapellen, die Toten ({n}) füllten die Gräben.",
    ]),
    ("battle_boss", &[
        "{when} fiel {b} vor den Heeren von {a} bei {p}. Die Chronisten schrieben den Namen zweimal.",
        "{when} zog {a} gegen {b} bei {p}. {n} fielen, doch {b} fiel zuerst.",
        "{when} besiegte {a} bei {p} das Grauen von {b}. Man sang davon noch Jahrzehnte später.",
        "{when} stellte {a} bei {p} das Ungeheuer {b} zum Kampf. {n} Streiter fielen, bevor das Untier den Boden küsste.",
        "{when} hallte bei {p} das Todesgebrüll von {b} wider; {a} behielt das Feld, doch {n} Männer und Frauen kehrten nicht zurück.",
        "{when} erschlug {a} bei {p} das Grauen namens {b}. Den Sängern fehlten die Worte, den Schmieden das Eisen für die vielen Grabzeichen ({n}).",
    ]),
    ("siege_fall", &[
        "{when} fiel {p} nach langer Belagerung an {a}.",
        "{when} öffneten die Verteidiger von {p} ihre Tore vor {a}; das Lager war leer, die Zuversicht auch.",
        "{when} ergab sich {p} dem Heer von {a}. Die Banner wurden gewechselt, die Gesichter blieben.",
        "{when} stürzten die Mauern von {p}; {a} zog durch die Bresche, und kein Glockenschlag mahnte mehr.",
        "{when} sank {p} nach langem Hungern in die Hand von {a}. Die letzten Vorräte waren längst aufgezehrt, die Hoffnung noch früher.",
        "{when} fiel {p} an {a}. Wer die Tore nicht öffnete, wurde ihnen zum Vorhang.",
    ]),
    ("siege_start", &[
        "{when} schloss {a} die Stadt {p} ein.",
        "{when} zog {a} Gräben um {p} und wartete auf den Hunger.",
        "{when} legte sich das Heer von {a} vor die Mauern von {p}; die Wachfeuer brannten bis zum Horizont.",
        "{when} rollten die Belagerungsmaschinen von {a} vor {p}. Die Stadt zählte ihre Brote.",
        "{when} sperrte {a} alle Wege nach {p}. Fortan sprach man dort nur noch vom Warten.",
        "{when} begann vor {p} die Belagerung durch {a}; die Brunnen wurden bewacht wie Schätze.",
    ]),
    ("province_conquered", &[
        "{when} nahm {a} die Provinz {p} von {b} in Besitz.",
        "{when} wechselte {p} den Herrn: {a} löste {b} ab.",
        "{when} wehte über {p} das Banner von {a}; {b} zog ab.",
        "{when} ging {p} an {a} über, und {b} verlor ein Land, das ihm nie ganz gehört hatte.",
        "{when} zog {a} in {p} ein; die Steuerlisten von {b} wurden im Hof verbrannt.",
        "{when} riss {a} die Provinz {p} an sich. {b} schrieb den Verlust in das Buch der Schulden.",
    ]),
    ("death_natural", &[
        "{when} starb {a} im Alter von {n} Jahren, friedlich, wie es hieß.",
        "{when} schloss {a} die Augen, {n} Jahre alt. Der Hof legte Trauer an.",
        "{when} wurde {a} zu Grabe getragen; {n} Winter hatte die Person gesehen.",
        "{when} erlosch {a}, {n} Jahre alt. Ein Kapitel des Hauses war zu Ende.",
        "{when} ging {a} zur letzten Ruhe, {n} Winter alt. Die Totenglocke läutete länger als erwartet.",
        "{when} verließ {a} die Welt der Lebenden, nach {n} Jahren und manchem Streit. Die Feinde sprachen leise, die Freunde lauter.",
    ]),
    ("death_battle", &[
        "{when} fiel {a} im Kampf, {n} Jahre alt.",
        "{when} kehrte {a} nicht aus der Schlacht zurück. Das Schwert hatte die letzte Antwort gegeben.",
        "{when} starb {a} auf dem Schlachtfeld, den Blick zum Feind gewandt.",
        "{when} blieb {a} auf dem Schlachtfeld zurück, {n} Jahre alt; die Raben waren pünktlich.",
        "{when} fiel {a} an der Spitze der Seinen. Man trug nur noch das Banner heim.",
        "{when} zahlte {a} den höchsten Preis im Lärm der Waffen. Die Jahre waren {n}, der Nachruhm kurz.",
    ]),
    ("death_murder", &[
        "{when} starb {a} an einem Becher Wein, den niemand eingeschenkt haben wollte.",
        "{when} wurde {a} tot aufgefunden. Die Klinge war nicht die eigene.",
        "{when} fiel {a} einem Mordanschlag zum Opfer; der Hof schwieg wie ein Grab.",
        "{when} fand man {a} im Morgengrauen, kalt und ohne Zeugen.",
        "{when} erwachte der Hof ohne {a}. Der Dolch war fort, die Fragen blieben.",
        "{when} starb {a} unter ungeklärten Umständen; die Diener wussten nichts, und das war verdächtig genug.",
    ]),
    ("death_disease", &[
        "{when} raffte das Fieber {a} dahin.",
        "{when} erlag {a} der Seuche, die {p} heimsuchte.",
        "{when} starb {a} nach Wochen auf dem Krankenlager.",
        "{when} legte sich das Fieber auf {a}, und kein Heiler fand ein Mittel.",
        "{when} verlor der Hof {a} an ein Leiden, dessen Namen man nur im Flüsterton nannte.",
        "{when} verzehrte die Krankheit {a}; die Kerzen am Lager brannten drei Nächte lang.",
    ]),
    ("death_executed", &[
        "{when} wurde {a} auf Befehl des Hofes hingerichtet.",
        "{when} endete {a} auf dem Richtplatz; die Menge schwieg.",
        "{when} fiel das Beil über {a}; der Henker wusch sich die Hände länger als sonst.",
        "{when} wurde {a} zum Richtplatz geführt. Nicht jeder in der Menge war überzeugt von der Schuld.",
        "{when} sprach das Gericht sein Urteil über {a}, und die Glocke schlug zur Stunde der Vollstreckung.",
        "{when} bezahlte {a} mit dem Leben für das, was der Hof Verrat nannte.",
    ]),
    ("abdication", &[
        "{when} legte {a} die Krone nieder und zog sich zurück.",
        "{when} dankte {a} ab, müde der Last der Herrschaft.",
        "{when} übergab {a} das Zepter und ging in die Stille eines Klosters.",
        "{when} streifte {a} den Reif vom Haupt und sagte, die Last sei zu schwer geworden.",
        "{when} trat {a} vom Thron zurück. Manche priesen die Weisheit, andere witterten Gift.",
        "{when} entsagte {a} der Macht; die Krone wanderte weiter, die Sorgen blieben beim Alten.",
    ]),
    ("deposed", &[
        "{when} wurde {a} abgesetzt; {b} griff nach der Macht.",
        "{when} stürzten die Häuser {a} vom Thron. {b} ließ sich ausrufen.",
        "{when} verlor {a} den Thron an {b}; die Wachen sahen weg, wie es mit Wachen so ist.",
        "{when} zerbrach die Herrschaft von {a}. {b} setzte sich auf den Platz, noch warm vom Vorgänger.",
        "{when} stürzte {a} über die eigene Härte; {b} trug die Krone aus dem Saal.",
        "{when} wurde {a} vom Thron gestoßen, und {b} nannte es Notwendigkeit.",
    ]),
    ("birth", &[
        "{when} wurde {a} im Haus {b} geboren.",
        "{when} kam {a} zur Welt, Kind des Hauses {b}.",
        "{when} schenkte das Haus {b} der Welt {a}.",
        "{when} schrie in den Gemächern des Hauses {b} ein neues Kind: {a}. Die Hebamme las in den Zeichen und schwieg.",
        "{when} bekam das Haus {b} Nachwuchs. {a} wurde mit Wein und gemischten Gefühlen empfangen.",
        "{when} sah {a} das Licht der Welt, Spross des Hauses {b}; die Ahnen schauten aus ihren Rahmen.",
    ]),
    ("marriage", &[
        "{when} wurden {a} und {b} vermählt.",
        "{when} feierte man die Hochzeit von {a} und {b}; die Gäste zählten ihre Vorteile.",
        "{when} verband die Ehe von {a} und {b} zwei Häuser.",
        "{when} sprachen {a} und {b} vor dem Altar die Worte; die Gäste verstanden mehr von Politik als von Liebe.",
        "{when} tauschten {a} und {b} die Ringe, und die Häuser tauschten Hoffnungen.",
        "{when} schloss man den Bund zwischen {a} und {b} bei Fackelschein; der Wein floss, die Verträge ruhten daneben.",
    ]),
    ("marriage_alliance", &[
        "{when} besiegelten {a} und {b} ihr Bündnis durch eine Heirat.",
        "{when} knüpfte die Ehe zwischen {a} und {b} ein Band, das Verträge allein nie geschaffen hätten.",
        "{when} wurden {a} und {b} getraut, und mit ihnen verschmolzen Heere und Hoffnungen.",
        "{when} bekräftigte die Hochzeit von {a} und {b} den Pakt der Häuser; wer ihn brach, brach auch das Brautbett.",
        "{when} legten {a} und {b} die Hände ineinander. Die Chronisten notierten es als Bündnis, die Dichter als Liebe.",
        "{when} sollte die Vermählung von {a} und {b} erreichen, was Schwerter nicht vermocht hatten.",
    ]),
    ("succession", &[
        "{when} bestieg {a} den Thron von {b}.",
        "{when} folgte {a} auf den Thron von {b}.",
        "{when} wurde {a} zum Herrscher über {b} ausgerufen.",
        "{when} nahm {a} die Krone von {b} auf, nicht jeder Blick im Saal war freundlich.",
        "{when} trat {a} das schwere Erbe in {b} an; die Hofschreiber wechselten die Siegel.",
        "{when} setzte sich {a} auf den Thron von {b}. Die Gerüchte kamen schneller als die Gratulanten.",
    ]),
    ("regency", &[
        "{when} übernahm {a} die Regentschaft für das Kind {b}.",
        "{when} führte {a} die Geschäfte von {p}, bis {b} alt genug sein würde.",
        "{when} legte man die Zügel in die Hand von {a}; das Kind {b} sollte Zeit zum Wachsen haben.",
        "{when} hielt {a} die Macht in Verwahrung für {b}. Man fragte sich, wie lange man Verwahrung nennen würde.",
        "{when} regierte {a} fortan an Stelle des unmündigen {b}, und der Hof hielt den Atem an.",
        "{when} wurde {a} zum Vormund für {b} bestellt; das Siegel lag bei {a}, die Zukunft bei den Göttern.",
    ]),
    ("dynasty_extinct", &[
        "{when} erlosch das Haus {a} mit dem Tod des Letzten seines Blutes.",
        "{when} starb mit {b} das Haus {a} aus. Sein Wappen hängt noch, sein Name verblasst.",
        "{when} verlosch das Haus {a}; mit {b} schloss sich das letzte Buch der Ahnen.",
        "{when} fand das Haus {a} sein Ende. Die Ahnengalerie blieb, die Erben fehlten, und {b} war der Letzte.",
        "{when} sank das Geschlecht {a} ins Grab, als {b} die Augen schloss.",
        "{when} starb das Haus {a} aus. Wer sich als Erbe ausgab, log, und {b} konnte nicht mehr widersprechen.",
    ]),
    ("dynasty_change", &[
        "{when} ging die Krone von {b} an das Haus {a} über.",
        "{when} begann in {b} die Herrschaft des Hauses {a}.",
        "{when} wechselte die Herrschaft in {b} zum Haus {a}, und die Wappen an den Toren wurden neu gemalt.",
        "{when} begann für {b} eine neue Ära unter dem Haus {a}; die alten Eide wurden still begraben.",
        "{when} fiel das Zepter von {b} an das Haus {a}. Das Volk zuckte mit den Schultern, die Adligen nicht.",
        "{when} kam das Haus {a} an die Macht, und {b} lernte einen neuen Namen auf den Münzen.",
    ]),
    ("war_declared", &[
        "{when} erklärte {a} dem Reich {b} den Krieg ({n}).",
        "{when} zog {a} gegen {b} ins Feld; der Grund war {c}.",
        "{when} sagte {a} {b} die Fehde an. {c} hieß der Vorwand.",
        "{when} brach der Krieg zwischen {a} und {b} aus.",
        "{when} verbrannte {a} die Briefe an {b}. Der Krieg hatte einen Grund: {c}.",
        "{when} wehte das Kriegsbanner von {a} über der Grenze zu {b}; {c} nannte man als Anlass, die Waffenkammern kannten andere.",
    ]),
    ("peace_white", &[
        "{when} schlossen {a} und {b} Frieden, ohne dass einer gewonnen hätte.",
        "{when} endete der Krieg zwischen {a} und {b} ohne Sieger.",
        "{when} legten {a} und {b} die Schwerter nieder, ohne dass ein Sieger zu feiern gewesen wäre.",
        "{when} einigten sich {a} und {b} auf Frieden. Beide sagten, sie hätten gewonnen, und beide logen.",
        "{when} schwiegen die Waffen zwischen {a} und {b}; die Gräber auf beiden Seiten waren die einzigen Gewinner.",
        "{when} beendeten {a} und {b} den Krieg aus Erschöpfung, nicht aus Einsicht.",
    ]),
    ("peace_win", &[
        "{when} diktierte {a} {b} den Frieden.",
        "{when} unterwarf sich {b} im Frieden dem Willen von {a}.",
        "{when} endete der Krieg: {a} erhielt, was es gefordert hatte, {b} blieb die Rechnung.",
        "{when} musste {b} dem Frieden zustimmen, den {a} in Tinte und Drohung schrieb.",
        "{when} beugte sich {b} dem Willen von {a}; der Friede war ein Kleid, das nur einem passte.",
        "{when} unterzeichnete {b} die Bedingungen von {a}. Die Feder zitterte, die Hand auch.",
    ]),
    ("plot_success", &[
        "{when} gelang {a} ein Komplott ({c}) gegen {b}.",
        "{when} schlug {a} zu: Das Komplott ({c}) gegen {b} ging auf.",
        "{when} zahlte sich die Geduld von {a} aus: Das Komplott ({c}) gegen {b} fand sein Ziel.",
        "{when} traf {b} der Schlag von {a}, still vorbereitet und durch {c} ausgeführt.",
        "{when} rächte sich {a} an {b}. Das Mittel ({c}) war leise, der Hof ahnte nichts.",
        "{when} ging das Spiel um die Macht zugunsten von {a} aus; {c} war der Hebel gewesen, {b} der Preis.",
    ]),
    ("plot_discovered", &[
        "{when} wurde das Komplott ({c}) von {a} gegen {b} aufgedeckt.",
        "{when} flog die Verschwörung ({c}) des {a} auf; {b} ließ Wachen aufziehen.",
        "{when} fielen die Pläne von {a} gegen {b} in fremde Hände; {c} wurde zum Beweisstück.",
        "{when} brachten die Spione von {b} das Geflecht von {a} ans Licht ({c}).",
        "{when} wurde ruchbar, dass {a} gegen {b} sann ({c}); Siegel und Briefe sprachen deutlicher als jeder Eid.",
        "{when} verriet ein Diener die Verschwörung ({c}) von {a} gegen {b}.",
    ]),
    ("plot_failed", &[
        "{when} scheiterte {a} mit einem Komplott ({c}) gegen {b}.",
        "{when} misslang {a} der Anschlag auf {b} ({c}); die Hand zitterte, der Wein war zu bitter oder zu spät.",
        "{when} brach das Komplott ({c}) von {a} gegen {b} zusammen, bevor es recht begonnen hatte.",
        "{when} ging der Plan ({c}) von {a} gegen {b} schief; mancher bezahlte mit dem Kopf.",
        "{when} entging {b} dem Zugriff von {a}; das Komplott ({c}) verlief im Sand.",
        "{when} stolperte {a} mit der Intrige ({c}) gegen {b} über die eigene Klugheit.",
    ]),
    ("drought", &[
        "{when} blieb der Regen über {p} aus, und die Brunnen sanken.",
        "{when} verdorrte das Land um {p}: Drei Monde lang fiel kein Tropfen.",
        "{when} brannte die Sonne auf {p}; das Korn stand welk auf den Feldern.",
        "{when} nannte man den Himmel über {p} ein Kupferdach.",
        "{when} dörrte die Hitze das Land um {p}; selbst die Raben suchten Schatten.",
        "{when} stand in {p} die Erde offen wie ein Mund, der nach Wasser schreit.",
    ]),
    ("harvest_failed", &[
        "{when} verdarb die Ernte in {p}, nur {n} von hundert Teilen kamen in die Scheunen.",
        "{when} brachte {p} kaum die Hälfte der Ernte ein ({n} von hundert).",
        "{when} faulte in {p} das Korn auf dem Halm; kaum {n} von hundert Teilen erreichten die Speicher.",
        "{when} sprach man in {p} von der Missernte ({n} von hundert), und die Bauern zählten Säcke, die nicht da waren.",
        "{when} lagen die Felder von {p} bleich und leer; die Ernte erreichte nur {n} von hundert.",
        "{when} schwiegen in {p} die Dreschflegel. {n} von hundert Teilen waren das ganze Jahr.",
    ]),
    ("price_spike", &[
        "{when} stieg in {p} der Preis für Brot auf das {n}-fache der Zeit davor.",
        "{when} wurde das Brot in {p} unbezahlbar; die Händler nannten Zahlen, die Armen nannten sie Hohn.",
        "{when} verteuerte sich das Brot in {p} auf das {n}-fache; wer keines kaufte, stahl.",
        "{when} schossen die Preise in {p} in die Höhe ({n}-fach), und der Markt roch nach Angst.",
        "{when} zahlte man in {p} das {n}-fache für einen Laib. Die Bäcker verschlossen ihre Türen und die Armen ihre Münder.",
        "{when} wurde das Getreide in {p} zu Gold; der Preis stand beim {n}-fachen, die Geduld bei null.",
    ]),
    ("famine", &[
        "{when} brach in {p} die Hungersnot aus; {n} Menschen starben.",
        "{when} leerten sich die Speicher von {p}, und der Hunger hielt Einzug.",
        "{when} starben in {p} die Schwachen zuerst, dann die Hoffnungsvollen.",
        "{when} hielt der Hunger in {p} Hof, und {n} Seelen sahen den nächsten Frühling nicht.",
        "{when} klagte {p} über leere Scheuern; {n} Tote zählte man, die Lebenden zählten nicht mehr.",
        "{when} nagte in {p} der Hunger an Hütten und Palästen gleichermaßen; {n} fanden den Tod.",
    ]),
    ("unrest", &[
        "{when} gärte es in {p}; auf den Märkten wurde nicht mehr gehandelt, sondern geflüstert.",
        "{when} brannten in {p} die ersten Speicher.",
        "{when} wurde es in {p} unruhig; an den Brunnen wechselten die Gerüchte schneller als das Wasser.",
        "{when} flüsterten die Gassen von {p} Worte, die vor einem Jahr noch Verrat gewesen wären.",
        "{when} warfen Unbekannte in {p} Steine gegen die Tore der Obrigkeit.",
        "{when} brannte in {p} kein Feuer mehr im Hof des Vogts; die Menge stand davor.",
    ]),
    ("revolt", &[
        "{when} erhob sich das Volk von {p} gegen {b}; {n} Aufständische griffen zu den Waffen.",
        "{when} begann in {p} der Aufstand gegen {b}.",
        "{when} stürmten die Bauern von {p} die Burg, die Fahnen von {b} fielen.",
        "{when} schlug in {p} die Stunde der Aufständischen: {n} Männer und Frauen kündigten {b} den Gehorsam.",
        "{when} läuteten in {p} die Sturmglocken. {n} Mutige trotzten {b}, und kein Siegel hielt sie auf.",
        "{when} entzündete sich in {p} die Wut gegen {b}; {n} Hände griffen nach Sensen und Spießen.",
    ]),
    ("revolt_crushed", &[
        "{when} schlug {a} den Aufstand in {p} nieder.",
        "{when} endete der Aufstand von {p}; {a} hielt Gericht.",
        "{when} wurde der Aufstand in {p} von {a} erstickt; auf den Plätzen standen die Galgen zahlreicher als die Brunnen.",
        "{when} warf {a} die Empörung in {p} nieder. Die Ordnung kehrte zurück, still wie ein Friedhof.",
        "{when} brach {a} den Widerstand von {p}; die Rädelsführer büßten, der Rest schwieg.",
        "{when} unterdrückte {a} den Aufruhr in {p} mit eiserner Hand.",
    ]),
    ("rebel_state", &[
        "{when} sagte sich {p} von {b} los und gründete das Reich {a}.",
        "{when} entstand aus den Aufständischen von {p} das neue Reich {a}.",
        "{when} trat {p} aus dem Schatten von {b} und rief das Reich {a} aus; die Fahnen waren noch feucht von Farbe.",
        "{when} sagte sich {p} von {b} los. Aus der Empörung wuchs das Reich {a}.",
        "{when} erhob sich in {p} ein neues Banner: das Reich {a}. {b} bekam die Rechnung der Zerrissenheit.",
        "{when} gründeten die Aufständischen von {p} das Reich {a} und kündigten {b} den Zins.",
    ]),
    ("plague_start", &[
        "{when} brach in {p} die Seuche aus: {c} ging um.",
        "{when} zog {c} durch {p}, und die Totenglocken fanden kein Ende.",
        "{when} trug der Wind {c} nach {p}.",
        "{when} schlich {c} durch die Gassen von {p}; die Kreidekreuze an den Türen mehrten sich.",
        "{when} brachte ein Fremder {c} nach {p}; die Kranken lagen bald in Reihen.",
        "{when} lähmte {c} die Stadt {p}; Ärzte flohen, Priester blieben.",
    ]),
    ("plague_end", &[
        "{when} erlosch die Seuche in {p} nach {n} Toten.",
        "{when} schwand das Fieber aus {p}. Es hatte {n} Leben genommen.",
        "{when} verebbte in {p} die Seuche. {n} Gräber kündeten von dem, was nicht wiederkommen sollte.",
        "{when} atmete {p} auf: Die Seuche war vorüber, doch {n} Stühle blieben leer.",
        "{when} schwiegen die Totenglocken von {p}. Die Seuche hatte {n} Opfer gefordert und zog weiter.",
        "{when} erlosch in {p} das Fieber, nachdem es {n} Leben verschlungen hatte.",
    ]),
    ("hollowing_rise", &[
        "{when} sank {p} tiefer in die Hohlung (nun {n}).",
        "{when} wurde die Luft über {p} grau; die Hohlung stieg auf {n}.",
        "{when} wurde es in {p} still, als würde die Welt den Atem anhalten; die Hohlung stieg auf {n}.",
        "{when} verblasste das Licht über {p}; die Hohlung wuchs auf {n}, und die Schatten wurden dichter.",
        "{when} flüsterte die Hohlung in {p} lauter als zuvor (nun {n}); die Hunde winselten.",
        "{when} sahen die Menschen in {p} ihre Gesichter im Spiegel gräulicher werden: Die Hohlung erreichte {n}.",
    ]),
    ("shrine_appears", &[
        "{when} erhob sich in {p} ein Hohlschrein aus der Erde.",
        "{when} entstand bei {p} ein neuer Hohlschrein, nachts flackerte fahles Licht.",
        "{when} brach in {p} ein Hohlschrein aus dem Boden, und der Boden roch nach kalter Asche.",
        "{when} wuchs bei {p} ein Hohlschrein aus der Erde wie ein Zahn aus fauligem Fleisch.",
        "{when} stand über Nacht ein neuer Hohlschrein bei {p}; die Hirten mieden die Weide.",
        "{when} erhob sich ein Schrein der Hohlen in {p}, und wer nachts an ihm vorbeiging, hörte seinen Namen.",
    ]),
    ("hollow_conquest", &[
        "{when} fiel {p} an die Hohlen.",
        "{when} gingen die Lichter von {p} aus; die Hohlen hielten das Land.",
        "{when} verlor {p} sein Licht an die Hohlen; die Türen standen offen, die Betten waren leer.",
        "{when} schluckte die Hohlung {p}. Aus den Fenstern sah nichts mehr heraus.",
        "{when} erlag {p} dem Ansturm der Hohlen; die Glocken verstummten mitten im Schlag.",
        "{when} sank {p} in die Finsternis und gehörte fortan den Hohlen.",
    ]),
    ("hollow_liberated", &[
        "{when} befreite {a} {p} aus dem Griff der Hohlen.",
        "{when} wurde {p} unter Feuer gereinigt; {a} zog ein.",
        "{when} schlug {a} die Hohlen aus {p} zurück; die Schreine fielen, die Gebete kehrten wieder.",
        "{when} trug {a} das Licht zurück nach {p}. Die Hohlen wichen, aber die Narben blieben.",
        "{when} entriss {a} die Stadt {p} der Hohlung; die Überlebenden weinten und merkten es nicht.",
        "{when} rang {a} in {p} die Hohlen nieder und weihte das Land neu mit Feuer und Salz.",
    ]),
    ("hollow_stage", &[
        "{when} erreichte {a} die Stufe {n} der Hohlung: {c}.",
        "{when} neigte sich das Reich {a} dem Tief zu ({c}).",
        "{when} sank {a} in der Hohlung weiter: Stufe {n}, {c}. Man sah es zuerst in den Augen der Kinder.",
        "{when} gelangte {a} auf Stufe {n} der Hohlung. Der Chronist notierte: {c}. Seine Feder zitterte.",
        "{when} verlor {a} ein weiteres Stück seiner Seele: Stufe {n} der Hohlung ({c}).",
        "{when} legte sich der Schatten dichter über {a}; die Hohlung erreichte Stufe {n}: {c}.",
    ]),
    ("boss_defeated", &[
        "{when} fiel {b} durch die Hand von {a} bei {p}.",
        "{when} ward {b} bei {p} bezwungen; {a} nahm den Ruhm.",
        "{when} streckte {a} den Unhold {b} bei {p} nieder; die Schmiede hämmerten drei Tage lang Lieder.",
        "{when} fiel das Ungeheuer {b} bei {p} unter den Klingen von {a}. Der Boden blieb noch Jahre unfruchtbar.",
        "{when} bezwang {a} das Grauen {b} in {p}; mit dem Jubel kam die Erschöpfung.",
        "{when} besiegte {a} bei {p} {b}, und die Barden fanden Reime für das Unsagbare.",
    ]),
    ("boss_awakes", &[
        "{when} erwachte {b} in {p}; die Welt hielt den Atem an.",
        "{when} regte sich {b} bei {p}.",
        "{when} sprengte {b} die Ketten des Schlafs bei {p}; die Erde bebte, die Vögel schwiegen.",
        "{when} öffnete {b} bei {p} die Augen; Hirten sahen Glut, wo keine war.",
        "{when} erwachte in {p} {b}, und mit ihm die alten Verwünschungen.",
        "{when} drang bei {p} ein Brüllen aus der Tiefe: {b} hatte den Schlaf abgeschüttelt.",
    ]),
    ("dragon_flight", &[
        "{when} brannte der Himmel über {p} drei Tage lang; Vyrmathax zog über die Felder.",
        "{when} flog Vyrmathax über {p}. Als er in die Krater zurückkehrte, lagen die Felder in Asche.",
        "{when} verdunkelte Vyrmathax den Himmel über {p}; wo seine Flügel fielen, standen Dörfer in Flammen.",
        "{when} zog Vyrmathax glühend über {p}. Der Rauch hielt sich Wochen, und die Asche fiel wie Schnee.",
        "{when} hörte man in {p} das Rauschen gewaltiger Schwingen; Vyrmathax zeichnete den Himmel mit Feuer.",
        "{when} nahm sich Vyrmathax, was er brauchte, bei {p}, und ließ als Dank verkohlte Felder.",
    ]),
    ("dragon_tribute", &[
        "{when} zahlte {a} dem Drachen Tribut; Vyrmathax schwieg {n} Jahre.",
        "{when} füllte {a} die Schatzkammern für den Drachen Vyrmathax. Er schlief danach {n} Jahre, und ganze Dörfer sprachen vom Frieden.",
        "{when} schickte {a} Gold und Vieh in die Krater; Vyrmathax ließ sich besänftigen, für {n} Jahre.",
        "{when} bezahlte {a} den Preis der Ruhe: {n} Jahre Schweigen des Drachen, erkauft mit Gold und Scham.",
        "{when} beugte sich {a} dem Schatten über den Kratern; Vyrmathax nahm die Gabe und versprach {n} Jahre Ruhe.",
        "{when} sandte {a} Tribut an Vyrmathax. Die Bauern atmeten auf, die Schatzmeister nicht ({n} Jahre Frist).",
    ]),
    ("legion_march", &[
        "{when} marschierte die Aschenlegion gegen {a}; {n} Mann stark.",
        "{when} setzte sich die Legion des Hohlen Königs in Bewegung, Ziel war {a}.",
        "{when} setzte sich die Aschenlegion in Marsch, {n} Mann, und {a} sah den Staub am Horizont.",
        "{when} dröhnten die Trommeln der Aschenlegion; {n} Schritte im Gleichtakt gingen auf {a} zu.",
        "{when} brach die Legion der Asche auf, {n} Mann stark, und {a} schloss die Tore zu früh oder zu spät.",
        "{when} zog die Aschenlegion gegen {a}. {n} graue Gesichter, kein Laut außer dem Marschtritt.",
    ]),
    ("endgame_stage", &[
        "{when} geschah, was die Alten fürchteten: {c}.",
        "{when} änderte sich die Welt: {c}.",
        "{when} begann ein neuer Abschnitt der Not: {c}.",
        "{when} erfüllte sich, was die Propheten raunten: {c}.",
        "{when} brach die Zeit der Prüfung an: {c}.",
        "{when} schlug die Stunde der Entscheidung: {c}.",
    ]),
    ("great_power", &[
        "{when} wurde {a} zur Großmacht; die Kronen schlossen sich zusammen.",
        "{when} warf {a} den Schatten über Eldraeth, den alle sahen.",
        "{when} stieg {a} zur Großmacht empor; die Nachbarn zählten ihre Soldaten und ihre Möglichkeiten.",
        "{when} rückte {a} in die Reihe der Mächtigen; selbst die Skeptiker verneigten sich, zumindest zum Schein.",
        "{when} warf {a} einen Schatten, der bis in die fernsten Höfe reichte; die Großmächte nannten es Aufstieg, die Kleinen Bedrohung.",
        "{when} wurde {a} zum Gewicht auf der Waage Eldraeths; wer es nicht fürchtete, hatte es nicht verstanden.",
    ]),
    ("coalition", &[
        "{when} schlossen sich {c} gegen {b} zusammen.",
        "{when} entstand eine Koalition gegen {b}: {c}.",
        "{when} sammelte sich der Zorn gegen {b}: {c} schworen, dem Aufstieg Einhalt zu gebieten.",
        "{when} traten {c} zusammen, um {b} Schranken zu setzen. Die Eide waren neu, das Misstrauen alt.",
        "{when} bildete sich gegen {b} ein Bund: {c} legten den Streit beiseite, vorläufig.",
        "{when} verbündeten sich {c} gegen {b}, und die Waagschale Eldraeths schwankte.",
    ]),
    ("alliance", &[
        "{when} schlossen {a} und {b} ein Bündnis.",
        "{when} reichten sich {a} und {b} die Hand zum Bund.",
        "{when} besiegelten {a} und {b} ihr Bündnis; die Siegel waren kaum trocken, als die ersten Zweifel kamen.",
        "{when} schworen {a} und {b} einander Beistand. Ob die Schwüre hielten, würde die Not zeigen.",
        "{when} traten {a} und {b} Schulter an Schulter, vereint durch Furcht und Vorteil.",
        "{when} unterzeichneten {a} und {b} einen Pakt, den ihre Feinde nicht vergessen würden.",
    ]),
    ("treaty_broken", &[
        "{when} brach {a} den Vertrag mit {b}.",
        "{when} zerriss {a} den Vertrag mit {b}; Tinte, die nichts galt, war schon vergessen.",
        "{when} wandte {a} sich ab von den Zusagen an {b} und nannte es Staatsräson.",
        "{when} kündigte {a} den Pakt mit {b} und ließ keine Ausrede zu.",
        "{when} sprach {b} von Treuebruch: {a} hatte den Vertrag verletzt, ohne Scham und ohne Frist.",
        "{when} warf {a} die Abmachung mit {b} ins Feuer. Der Rauch war der einzige Zeuge.",
    ]),
    ("boycott", &[
        "{when} verhängte {a} einen Handelsboykott gegen {b}.",
        "{when} sperrte {a} seine Häfen für die Waren von {b}.",
        "{when} sperrte {a} den Handel mit {b}; die Karawanen kehrten um, die Kassen leerten sich.",
        "{when} schloss {a} die Märkte für {b}. Wer dennoch lieferte, zahlte Schmuggelzoll.",
        "{when} ächtete {a} den Handel mit {b}; die Zöllner prüften jeden Karren doppelt.",
        "{when} zog {a} das Siegel von allen Verträgen mit {b}, und die Lager füllten sich mit unverkäuflicher Ware.",
    ]),
    ("tribute", &[
        "{when} verpflichtete sich {b}, {a} Tribut zu zahlen.",
        "{when} musste {b} fortan Abgaben an {a} entrichten; die Münzen wogen schwer, und das Schweigen schwerer.",
        "{when} beugte sich {b} der Forderung von {a}: Jedes Jahr würde Gold über die Grenze rollen.",
        "{when} versprach {b} dem Reich {a} regelmäßigen Tribut, und der Kämmerer schrieb es mit zitternder Hand.",
        "{when} verpflichtete {a} das Reich {b} zur Zahlung; ob als Schutz oder als Strafe, blieb eine Frage der Betrachtung.",
        "{when} schloss {b} den Handel mit der Demut: Tribut an {a}, Jahr für Jahr.",
    ]),
    ("vassalized", &[
        "{when} beugte {b} das Knie vor {a} und wurde dessen Vasall.",
        "{when} schwor {b} dem Reich {a} die Treue; das Banner wurde gesenkt, die Krone blieb auf dem Haupt, aber ohne Gewicht.",
        "{when} wurde {b} zum Lehen von {a}; die Fürsten sanken auf die Knie, und die Ahnen im Gruftgewölbe drehten sich um.",
        "{when} ging {b} in die Lehnspflicht von {a} über. Die Unabhängigkeit war ein Preis, den es zahlte.",
        "{when} unterwarf sich {b} dem Willen von {a}, und der Schwur klang wie ein Seufzer.",
        "{when} bekannte sich {b} als Vasall von {a}; nur die Schwerter an den Wänden erinnerten noch an Stolz.",
    ]),
    ("independence", &[
        "{when} erkämpfte sich {a} die Unabhängigkeit von {b}.",
        "{when} zerbrach {a} die Kette zu {b}; die Fahnen wurden gehisst, und niemand wagte zu zögern.",
        "{when} erklärte sich {a} frei von {b}, und die Glocken läuteten zum ersten Mal für das eigene Land.",
        "{when} schüttelte {a} die Oberhoheit von {b} ab; der Preis war Blut, der Gewinn Würde.",
        "{when} entkam {a} dem Griff von {b} und schrieb sich selbst in die Chroniken.",
        "{when} stand {a} wieder auf eigenen Füßen: Von {b} wollte es nichts mehr wissen.",
    ]),
    ("realm_fell", &[
        "{when} verlor {a} den letzten Landstrich und hörte auf zu bestehen.",
        "{when} endete das Reich {a}; was blieb, waren Lieder und Schulden.",
        "{when} sank das Reich {a} in die Vergessenheit; seine Namen wurden von den Grenzsteinen gekratzt.",
        "{when} erlosch {a} als Reich. Die Barden sangen noch ein Jahr von ihm, dann nicht mehr.",
        "{when} verschwand {a} von den Karten, und die Schreiber strichen die Linien mit scharfem Messer.",
        "{when} ging {a} unter, Stück für Stück, bis nichts blieb als ein Name und eine Erinnerung.",
    ]),
    ("realm_founded", &[
        "{when} entstand das Reich {a}.",
        "{when} erhob sich das Reich {a} aus Trümmern und Hoffnung; die ersten Münzen trugen ein ungeübtes Gesicht.",
        "{when} wurde das Reich {a} ausgerufen, und die Nachbarn rieben sich die Stirn.",
        "{when} trat {a} in die Reihe der Reiche; die Fahnen waren neu, die Feinde alt.",
        "{when} gründete man {a}. Ob es ein Jahrhundert überdauern würde, wusste nur die Zeit.",
        "{when} schrieb sich {a} in die Karten Eldraeths ein, mit frischer Tinte und schwerem Herzen.",
    ]),
    ("bankruptcy", &[
        "{when} erklärte {a} den Staatsbankrott; die Gläubiger tobten.",
        "{when} stand {a} vor leeren Truhen; die Gläubiger belagerten den Hof, und der Kämmerer verschwand bei Nacht.",
        "{when} brach die Staatskasse von {a} zusammen; der Sold blieb aus, und die Soldaten blieben nicht.",
        "{when} sah {a} dem Bankrott ins Gesicht; die Münzen waren zu leicht geworden und die Schulden zu schwer.",
        "{when} zahlte {a} seine Schulden nicht mehr. Das Vertrauen der Händler war das erste Opfer.",
        "{when} ging der Schatz von {a} zur Neige; die Steuern stiegen, die Zuneigung sank.",
    ]),
    ("conversion", &[
        "{when} wandte sich {p} dem {c} zu.",
        "{when} wechselte {p} den Glauben: Nun galt dort der {c}.",
        "{when} legte {p} die alten Opfergaben nieder und folgte einem neuen Glauben: {c}.",
        "{when} sprachen die Menschen in {p} neue Gebete, und {c} hieß nun der Glaube dort.",
        "{when} riss man in {p} die alten Zeichen von den Wänden; {c} zog in die Herzen ein.",
        "{when} wandte sich das Volk von {p} ab vom alten Weg: {c} war, was man nun anrief.",
    ]),
    ("heresy", &[
        "{when} keimte in {p} die Sonnenhäresie auf; Wanderprediger sprachen von der Rückkehr des Letzten Sonnenkönigs.",
        "{when} wisperten Prediger in {p} von einer verbotenen Wahrheit; die Sonnenhäresie fand offene Ohren.",
        "{when} loderte in {p} die Sonnenhäresie auf. Die Kirche nannte es Ketzerei, das Volk nannte es Trost.",
        "{when} sammelten sich in {p} Anhänger der Sonnenhäresie; ihre Lieder handelten vom Gold der Morgenröte.",
        "{when} fanden die Inquisitoren in {p} Zeichen der Sonnenhäresie an Tür und Herd.",
        "{when} erwachte in {p} ein alter Glaube an den Letzten Sonnenkönig, und niemand wusste, wer ihn gesät hatte.",
        "{when} geriet {p} unter den Einfluss der Sonnenhäresie; die Priester warnten, die Bauern lauschten.",
    ]),
    ("excommunication", &[
        "{when} sprach der Erzflammenwahrer den Bann über {a} aus.",
        "{when} schloss die Kirche {a} aus der Gemeinschaft der Gläubigen aus; die Glocken läuteten rückwärts.",
        "{when} traf der Bannstrahl des Erzflammenwahrers {a}; Kerzen erloschen, Türen schlossen sich.",
        "{when} verhängte der Erzflammenwahrer den Kirchenbann über {a}, und die Priester wandten sich ab.",
        "{when} sprach man in den Hallen des Glaubens den Namen {a} nur noch mit dem Zeichen gegen das Böse.",
        "{when} sprach die Kirche den Fluch über {a} aus; die Gläubigen hörten das Schweigen lauter als jeden Gesang.",
        "{when} stieß der Erzflammenwahrer {a} aus dem Schoß der Kirche; die Fürsten flüsterten, die Pfaffen schwiegen.",
    ]),
    ("holy_war", &[
        "{when} rief der Erzflammenwahrer den Heiligen Krieg gegen {b} aus.",
        "{when} entzündete der Erzflammenwahrer den Heiligen Krieg gegen {b}; Banner mit der Flamme zogen aus allen Landen.",
        "{when} rief die Kirche zum Kreuzzug gegen {b}; die Gläubigen griffen zu Schwert und Gebet.",
        "{when} erklang von den Kanzeln der Ruf zum Heiligen Krieg wider {b}.",
        "{when} nannte der Erzflammenwahrer {b} den Feind des Lichts und segnete alle, die es bekämpften.",
        "{when} sandte die Kirche die Flammenträger aus: Der Heilige Krieg gegen {b} hatte begonnen.",
        "{when} erhob sich der Heilige Zorn gegen {b}; die Glocken läuteten zum Kampf.",
    ]),
    ("research", &[
        "{when} erreichte {a} den Grad {n} der Schule {c}.",
        "{when} gelang den Gelehrten von {a} ein Durchbruch in der Schule {c} (Grad {n}).",
        "{when} schlossen die Magister von {a} Grad {n} der Schule {c} ab; die Bibliothek roch nach Kerzenwachs und neuer Macht.",
        "{when} stieß {a} in der Schule {c} bis zu Grad {n} vor, und die Nachbarn hörten es mit Unbehagen.",
        "{when} erschloss {a} Grad {n} in der Schule {c}; manches Geheimnis blieb besser verschlossen.",
        "{when} brachte die Forschung von {a} in der Schule {c} den Grad {n} hervor, und die Gelehrten feierten leise.",
    ]),
    ("ritual", &[
        "{when} wirkte {a} in {p} das Ritual {c}.",
        "{when} sprach {a} in {p} die Worte des Rituals {c}; die Luft vibrierte, und wer es sah, schwieg.",
        "{when} vollendete {a} in {p} das Ritual {c}; die Kerzen erloschen von selbst.",
        "{when} wirkte {a} bei {p} das Ritual {c}, und der Himmel antwortete mit einem Beben.",
        "{when} brachte {a} in {p} das Ritual {c} dar; die Asche des Opfers fiel nach oben.",
        "{when} zeichnete {a} in {p} die Runen des Rituals {c} und bezahlte, was zu zahlen war.",
        "{when} sprach {a} in {p} die verbotene Liturgie des Rituals {c} und verlor dabei einen Teil von sich.",
    ]),
    ("artifact", &[
        "{when} wurde in {p} ein Artefakt geborgen: {c}.",
        "{when} grub man in {p} das Artefakt {c} aus der Erde; die Arbeiter sagten, es habe geflüstert.",
        "{when} fand sich in {p} ein Fund von Gewicht: {c}. Wer es berührte, träumte von fernen Königen.",
        "{when} barg man in {p} {c}, ein Artefakt aus Tagen, die kein Chronist erinnert.",
        "{when} brachte eine Grabung in {p} das Artefakt {c} ans Licht, und Begehrlichkeiten erwachten.",
        "{when} entdeckte man in den Ruinen von {p} {c}; der Staub zerfiel, das Metall nicht.",
        "{when} förderte das Land bei {p} ein Relikt zutage: {c}. Das Gold daran war das Geringste.",
    ]),
    ("shrine_restored", &[
        "{when} wurde der Schrein {c} von {a} wiederhergestellt; ein Segen legte sich über Eldraeth.",
        "{when} stellte {a} den Schrein {c} wieder her, und Eldraeth atmete leichter.",
        "{when} flammte der Schrein {c} unter der Hand von {a} wieder auf; ein warmer Wind ging über das Land.",
        "{when} weihte {a} den Schrein {c} neu; Pilger zogen aus allen Landen herbei.",
        "{when} baute {a} den Schrein {c} aus den Trümmern auf; wo Asche lag, wuchs wieder Hoffnung.",
        "{when} gab {a} dem Schrein {c} seinen Segen zurück, und die Alten weinten.",
        "{when} erstrahlte der Schrein {c} dank {a} in altem Glanz; für einen Augenblick schien die Welt geheilt.",
    ]),
    ("victory", &[
        "{when} erfüllte {a} das Siegziel: {c}.",
        "{when} errang {a} den Sieg, den es gesucht hatte: {c}. Die Chronik schloss dieses Kapitel mit goldener Tinte.",
        "{when} erreichte {a} sein Ziel: {c}. Die Rivalen sahen zu, wie die Geschichte sich neigte.",
        "{when} krönte sich {a} mit dem Sieg: {c}. Eldraeth hielt den Atem an.",
        "{when} vollbrachte {a}, was keiner ihm zugetraut hatte: {c}.",
        "{when} stand {a} am Ziel der Wege: {c}. Man schrieb den Namen in Gold, und die Gegner in Asche.",
        "{when} triumphierte {a}; {c} ward erfüllt, und der Ruhm hallte über alle Grenzen.",
    ]),
    ("legend", &[
        "{when} wurde aus {p} eine Legende: {c}.",
        "{when} erzählte man in {p} fortan von einem Wunder, das keiner belegen konnte: {c}.",
        "{when} entstand in {p} eine Geschichte, die die Barden sofort aufgriffen: {c}.",
        "{when} wurde in {p} ein Name zur Sage: {c}. Wahr war es nur zur Hälfte, aber das reichte.",
        "{when} sang man in {p} das erste Lied von {c}. Das Lied überdauerte die Wahrheit.",
        "{when} verwuchs in {p} die Wirklichkeit mit dem Märchen: {c}.",
        "{when} flüsterten die Alten in {p} von {c}; die Kinder hörten zu, und die Legende war geboren.",
    ]),
    ("event_generic", &[
        "{when} ereignete sich in {p}: {c}.",
        "{when} trug sich in {a} zu: {c}.",
        "{when} verzeichnete die Chronik ein Ereignis von Gewicht: {c}.",
        "{when} trug sich zu, was die Schreiber festhielten: {c}.",
        "{when} geschah, woran sich die Alten noch lange erinnerten: {c}.",
        "{when} kam es zu einem Vorfall, den man nicht überging: {c}.",
    ]),
    ("faction_ultimatum", &[
        "{when} stellten die Häuser von {a} der Krone ein Ultimatum: {c}.",
        "{when} verlangten die Fürsten von {a} Gehör: {c}. Die Frist war kurz und die Höflichkeit knapp.",
        "{when} legte eine Gruppe von Edlen in {a} der Krone ein Ultimatum auf den Tisch: {c}.",
        "{when} drohte in {a} der Bruch mit dem Hof; die Forderung lautete: {c}.",
        "{when} traten die Parteien von {a} vor den Thron und forderten: {c}.",
        "{when} stellte das Lager der Unzufriedenen in {a} sein Ultimatum, und die Antwort würde über das Haus entscheiden: {c}.",
        "{when} sprachen die Großen in {a} mit einer Stimme zur Krone: {c}, sonst nichts.",
    ]),
    ("house_rebels", &[
        "{when} erhob sich das Haus {a} gegen {b}.",
        "{when} erklärte Haus {a} den Gehorsam für beendet und griff zu den Waffen gegen {b}.",
        "{when} entzog das Haus {a} der Krone von {b} die Gefolgschaft; die Fahnen wechselten, die Gesichter blieben.",
        "{when} brach das Haus {a} den Lehnseid gegen {b}, und die Burgen schlossen ihre Tore.",
        "{when} sagte das Haus {a} dem Thron von {b} den Dienst auf und nannte es Notwehr.",
        "{when} stellte sich das Haus {a} offen gegen {b}; die Bündnisse zerbrachen leise.",
    ]),
    ("bandits", &[
        "{when} überfielen Banditen die Straßen um {p}.",
        "{when} zogen Räuberbanden durch die Wälder um {p}, und die Händler zählten ihre Verluste.",
        "{when} überfielen Wegelagerer die Karawanen bei {p}; wer reiste, tat es bewaffnet.",
        "{when} plünderten Räuber die Landstraßen um {p}; die Vögte schickten Wachen, die nicht zurückkehrten.",
        "{when} wurde die Gegend um {p} zum Revier von Banditen; die Dörfer zahlten Schutzgeld an die Falschen.",
        "{when} fürchtete man in {p} nach Einbruch der Dämmerung die Straßen; Banditen herrschten dort.",
    ]),
    ("raid", &[
        "{when} fielen {c} aus den Schatten über {p} her; {n} fielen.",
        "{when} brachen {c} bei {p} aus der Dunkelheit; {n} Menschen büßten dafür mit dem Leben.",
        "{when} fielen {c} über {p} her, und {n} Seelen löschte die Nacht.",
        "{when} suchten {c} die Siedlungen bei {p} heim; {n} Tote blieben im Staub.",
        "{when} zogen {c} gegen {p}, und die Schreie hallten bis zum Morgen. Man zählte {n} Tote.",
        "{when} schlugen {c} bei {p} zu, rasch und gnadenlos; {n} fanden das Ende der Nacht nicht mehr.",
    ]),
    ("trade_route", &[
        "{when} öffnete sich ein neuer Handelsweg zwischen {a} und {b}.",
        "{when} schlossen {a} und {b} eine neue Handelsstraße; auf den Pfaden klapperten bald Wagen mit Gewürzen und Gerüchten.",
        "{when} öffneten {a} und {b} einen neuen Weg für Händler; wo gestern Wildnis lag, rollten nun Räder.",
        "{when} entstand zwischen {a} und {b} eine neue Handelsroute, und die Zöllner rieben sich die Hände.",
        "{when} verband eine neue Straße {a} mit {b}; die Karawanen brachten Silber und Neuigkeiten.",
        "{when} verknüpften {a} und {b} ihre Märkte; die Kaufleute sprachen von goldenen Zeiten.",
    ]),
    ("omen", &[
        "{when} meldeten Boten aus {p} üble Vorzeichen: {c}.",
        "{when} blutete in {p} der Mond; die Alten sagten, nun hätte es begonnen: {c}.",
        "{when} sah man in {p} die Zeichen: {c}. Wer sie deutete, bat um Verschwiegenheit.",
        "{when} erschienen in {p} düstere Vorzeichen: {c}; Mütter schlossen ihre Kinder ein.",
        "{when} flatterten Raben in Scharen über {p}, und {c} kündete von Unheil.",
        "{when} brachte die Dämmerung in {p} ein Gesicht des Grauens: {c}.",
        "{when} zeigte sich in {p} ein Zeichen am Himmel: {c}. Die Priester murmelten Gebete.",
    ]),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_has_at_least_six_distinct_variants() {
        for (name, variants) in TEMPLATES {
            assert!(variants.len() >= 6, "Vorlage {name} hat nur {} Varianten", variants.len());
            let mut sorted: Vec<&&str> = variants.iter().collect();
            sorted.sort();
            sorted.dedup();
            assert_eq!(sorted.len(), variants.len(), "Vorlage {name} enthält doppelte Varianten");
            for v in *variants {
                assert!(v.starts_with("{when}"), "Vorlage {name}: Variante beginnt nicht mit {{when}}: {v}");
            }
        }
        let mut names: Vec<&str> = TEMPLATES.iter().map(|(k, _)| *k).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), TEMPLATES.len(), "Vorlagennamen müssen eindeutig sein");
    }

    #[test]
    fn narrate_is_deterministic_and_fills_placeholders() {
        let vars = [("a", "Aschmark".to_string()), ("b", "Valdrys".to_string()), ("p", "Brennhollow".to_string()), ("n", "12".to_string())];
        let mut seen = std::collections::BTreeSet::new();
        for id in 0..200 {
            let t = narrate("battle_win", id, Date::from_day(400), &vars);
            assert_eq!(t, narrate("battle_win", id, Date::from_day(400), &vars));
            assert!(!t.contains('{') && !t.contains('}'), "{t}");
            seen.insert(t);
        }
        assert!(seen.len() >= 6, "Varianten sollten tatsächlich gewählt werden: {}", seen.len());
    }
}
