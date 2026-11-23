//! Charaktere, Dynastien, Nachfolge, Regentschaft, Heirat, Geburt, Altern und Tod (GDD Kapitel 5).

use crate::catalog::{self, Trait};
use crate::chronicle::{ChronicleKind, Entry};
use crate::model::*;
use crate::types::*;
use crate::world::StrategyWorld;

/// Dynastie-Vermächtnisse (je 1.000 Dynastieprestige eines).
pub const LEGACIES: [(&str, &str); 5] = [
    ("guardian_of_the_south", "Wächter des Südens: +15 % Kampfkraft gegen Hohle"),
    ("ember_blood", "Glutblut: Kinder werden mit 20 % flammengeweiht"),
    ("trading_house", "Handelshaus: +10 % Handelsertrag"),
    ("iron_will", "Eisenwille: −20 % Stressaufbau, +5 Legitimität"),
    ("spark_of_wisdom", "Funke der Weisheit: +10 % Forschung"),
];

impl StrategyWorld {
    // ------------------------------------------------------------------------------------
    // Erzeugung
    // ------------------------------------------------------------------------------------
    pub fn create_dynasty(&mut self, name: &str, culture: Culture) -> DynId {
        let id = self.dynasties.len() as DynId;
        let crest = self.rng_u32();
        self.dynasties.push(Dynasty { id, name: name.to_string(), culture, founded: self.day, prestige: 0.0, extinct: false, extinct_day: None, cadet_of: None, house: None, legacies: Vec::new(), crest });
        id
    }

    pub fn rng_u32(&mut self) -> u32 {
        use rand::RngExt;
        self.rng.random::<u32>()
    }

    pub fn random_name(&mut self, culture: Culture, gender: Gender) -> String {
        let n = catalog::names(culture);
        let list = if gender == Gender::Male { n.male } else { n.female };
        let i = self.ri(0, list.len() as i32 - 1) as usize;
        list[i].to_string()
    }

    pub fn random_family_name(&mut self, culture: Culture) -> String {
        let n = catalog::names(culture);
        // Bevorzugt einen noch nicht vergebenen Familiennamen; sonst Namenszusatz.
        for _ in 0..12 {
            let i = self.ri(0, n.family.len() as i32 - 1) as usize;
            let cand = n.family[i];
            if !self.dynasties.iter().any(|d| d.name == cand) {
                return cand.to_string();
            }
        }
        let i = self.ri(0, n.family.len() as i32 - 1) as usize;
        let suffix = self.dynasties.iter().filter(|d| d.name.starts_with(n.family[i])).count() + 1;
        format!("{} {}", n.family[i], ["der Ältere", "der Jüngere", "von Norden", "von Süden", "vom Fluss", "von der Höhe"][suffix % 6])
    }

    fn roll_attrs(&mut self) -> [i8; 6] {
        let mut a = [0i8; 6];
        for v in a.iter_mut() {
            *v = (3 + self.ri(0, 4) + self.ri(0, 4)) as i8;
        }
        a
    }

    fn roll_traits(&mut self) -> Vec<Trait> {
        let personality: Vec<Trait> = Trait::ALL.iter().copied().filter(|t| t.is_personality()).collect();
        let mut out: Vec<Trait> = Vec::new();
        let n = 2 + self.ri(0, 1);
        let mut guard = 0;
        while (out.len() as i32) < n && guard < 40 {
            guard += 1;
            let t = *self.pick(&personality);
            if out.contains(&t) || out.iter().any(|o| o.opposite() == Some(t)) {
                continue;
            }
            out.push(t);
        }
        let r = self.rf();
        if r < 0.01 {
            out.push(Trait::Genius);
        } else if r < 0.06 {
            out.push(Trait::Quick);
        } else if r < 0.11 {
            out.push(Trait::Slow);
        }
        if self.chance(0.02) {
            out.push(Trait::Giant);
        }
        if self.chance(0.03) {
            out.push(Trait::Sickly);
        }
        out
    }

    /// Erzeugt eine Person mit zufälligen Werten. `age` in Jahren.
    #[allow(clippy::too_many_arguments)]
    pub fn spawn_person(&mut self, name: Option<String>, culture: Culture, dynasty: DynId, gender: Gender, age: i32, realm: Option<RealmId>, house: Option<HouseId>, location: ProvId) -> CharId {
        let id = self.chars.len() as CharId;
        let name = name.unwrap_or_else(|| self.random_name(culture, gender));
        let attrs = self.roll_attrs();
        let traits = self.roll_traits();
        let mut health = 5.0 + self.rf() * 4.0;
        if traits.contains(&Trait::Sickly) {
            health -= 2.0;
        }
        if traits.contains(&Trait::Giant) {
            health -= 1.0;
        }
        let religion = realm.map(|r| self.realms[r as usize].religion).unwrap_or_else(|| catalog::default_religion(culture));
        let fert = 0.5 + self.rf() * 0.5;
        let born = self.day as i64 - age as i64 * DAYS_PER_YEAR as i64 - self.ri(0, 359) as i64;
        let ch = Character {
            id,
            name,
            dynasty,
            house,
            realm,
            culture,
            religion,
            gender,
            born,
            died: None,
            father: None,
            mother: None,
            spouse: None,
            children: Vec::new(),
            attrs,
            health: health.clamp(1.0, 10.0),
            fertility: if traits.contains(&Trait::Sickly) { fert * 0.9 } else { fert },
            traits,
            abilities: Vec::new(),
            prestige: 0.0,
            piety: 0.0,
            hollowing: 0.0,
            stress: 0.0,
            location,
            office: None,
            secrets: Vec::new(),
            captive_of: None,
            army: None,
            education: None,
            guardian: None,
            claims: Vec::new(),
            memories: Vec::new(),
            epithet: None,
            immortal: false,
            wounded_until: 0,
            is_hero: false,
            deeds: Deeds::default(),
            reign_start: None,
            reigns: Vec::new(),
        };
        self.chars.push(ch);
        self.roll_abilities(id);
        if let Some(r) = realm {
            self.court_add(r, id);
        }
        id
    }

    fn roll_abilities(&mut self, c: CharId) {
        let (m, i, l, s) = {
            let ch = &self.chars[c as usize];
            (ch.attr(Attr::Martial), ch.attr(Attr::Intrigue), ch.attr(Attr::Learning), ch.attr(Attr::Stewardship))
        };
        let mut ab = Vec::new();
        if m >= 11 && self.chance(0.6) {
            ab.push(Ability::Tactician);
        }
        if m >= 9 && self.chance(0.2) {
            ab.push(Ability::SiegeMaster);
        }
        if s >= 10 && self.chance(0.4) {
            ab.push(Ability::Logistician);
        }
        if s >= 11 && self.chance(0.3) {
            ab.push(Ability::Administrator);
        }
        if l >= 10 && self.chance(0.4) {
            ab.push(Ability::Healer);
        }
        if i >= 10 && self.chance(0.4) {
            ab.push(Ability::Spy);
        }
        if l >= 12 && self.chance(0.3) {
            let idx = self.ri(0, 8) as usize;
            ab.push(Ability::Mage(SCHOOLS[idx]));
        }
        if self.chars[c as usize].has(Trait::Brave) && self.chance(0.4) {
            ab.push(Ability::Bold);
        }
        self.chars[c as usize].abilities = ab;
    }

    pub fn court_add(&mut self, r: RealmId, c: CharId) {
        let realm = &mut self.realms[r as usize];
        if !realm.court.contains(&c) {
            realm.court.push(c);
        }
        self.chars[c as usize].realm = Some(r);
    }

    pub fn court_remove(&mut self, r: RealmId, c: CharId) {
        self.realms[r as usize].court.retain(|x| *x != c);
    }

    pub fn living_dynasty_members(&self, d: DynId) -> Vec<CharId> {
        self.chars.iter().filter(|c| c.dynasty == d && c.alive()).map(|c| c.id).collect()
    }

    // ------------------------------------------------------------------------------------
    // Meinungen zwischen Figuren
    // ------------------------------------------------------------------------------------
    pub fn char_opinion(&self, a: CharId, b: CharId) -> f32 {
        let (ca, cb) = (&self.chars[a as usize], &self.chars[b as usize]);
        let mut v = 0.0;
        // Grundsympathie
        let mut sym: f32 = 0.0;
        for t in &ca.traits {
            if !t.is_personality() {
                continue;
            }
            if cb.traits.contains(t) {
                sym += 10.0;
            }
            if let Some(o) = t.opposite()
                && cb.traits.contains(&o) {
                    sym -= 10.0;
                }
        }
        v += sym.clamp(-30.0, 30.0);
        for t in &cb.traits {
            v += t.opinion() as f32;
        }
        v += cb.attr(Attr::Charisma) as f32 * 0.5;
        if ca.spouse == Some(b) {
            v += 20.0;
        }
        if ca.father == Some(b) || ca.mother == Some(b) || cb.father == Some(a) || cb.mother == Some(a) || (ca.father.is_some() && ca.father == cb.father) {
            v += 30.0;
        }
        if ca.dynasty == cb.dynasty {
            v += 5.0;
            if cb.has(Trait::Kinslayer) {
                v -= 30.0;
            }
        }
        match ca.religion.relation(cb.religion) {
            1 => v += 10.0,
            -1 => v -= 20.0,
            _ => {}
        }
        if ca.culture == cb.culture {
            v += 5.0;
        }
        for m in &ca.memories {
            if m.subject == b && m.expires > self.day {
                v += m.value;
            }
        }
        if cb.hollowing >= 40.0 && ca.religion == Religion::EternalFlame {
            v -= 25.0;
        }
        v.clamp(-100.0, 100.0)
    }

    pub fn remember(&mut self, who: CharId, kind: MemoryKind, subject: CharId, value: f32, years: u32) {
        let expires = self.day + years * DAYS_PER_YEAR;
        let ch = &mut self.chars[who as usize];
        if ch.memories.len() > 40 {
            ch.memories.remove(0);
        }
        ch.memories.push(Memory { kind, subject, value, expires });
    }

    // ------------------------------------------------------------------------------------
    // Ehe und Geburt
    // ------------------------------------------------------------------------------------
    pub fn marry(&mut self, a: CharId, b: CharId, alliance: bool) {
        if self.chars[a as usize].spouse.is_some() || self.chars[b as usize].spouse.is_some() {
            return;
        }
        self.chars[a as usize].spouse = Some(b);
        self.chars[b as usize].spouse = Some(a);
        let (ra, rb) = (self.chars[a as usize].realm, self.chars[b as usize].realm);
        // Die Frau zieht an den Hof des Mannes bzw. des höherrangigen Partners.
        let (up, down) = (a, b);
        let ruler_a = ra.is_some_and(|r| self.realms[r as usize].ruler == a);
        let ruler_b = rb.is_some_and(|r| self.realms[r as usize].ruler == b);
        let (host, guest) = if ruler_b && !ruler_a { (down, up) } else { (up, down) };
        if let Some(hr) = self.chars[host as usize].realm {
            self.court_add(hr, guest);
            self.chars[guest as usize].location = self.chars[host as usize].location;
        }
        let na = self.full_name(a);
        let nb = self.full_name(b);
        let tpl = if alliance { "marriage_alliance" } else { "marriage" };
        let mag = if ruler_a || ruler_b { 45 } else { 15 };
        let mut e = Entry::new(ChronicleKind::Marriage, tpl, mag).actor(format!("char_{a}"), na.clone(), "Braut/Bräutigam").actor(format!("char_{b}"), nb.clone(), "Braut/Bräutigam").var("a", na).var("b", nb);
        if let Some(r) = ra.or(rb) {
            e = e.realm(r);
        }
        self.chron(e);
    }

    /// Jährliche Geburtenprüfung (GDD 5.6): Fruchtbarkeit_A × Fruchtbarkeit_B × 0,9, ab 40 −10 % je Jahr.
    fn births_year(&mut self) {
        let candidates: Vec<CharId> = self.chars.iter().filter(|c| c.alive() && c.gender == Gender::Female && c.spouse.is_some() && c.realm.is_some_and(|r| self.realms[r as usize].alive)).map(|c| c.id).collect();
        for m in candidates {
            let (f_id, age, fert_m, nkids) = {
                let c = &self.chars[m as usize];
                (c.spouse.unwrap(), c.age(self.day), c.fertility, c.children.len())
            };
            if !self.chars[f_id as usize].alive() {
                continue;
            }
            if !(16..=46).contains(&age) {
                continue;
            }
            let fert_f = self.chars[f_id as usize].fertility;
            let mut p = fert_m * fert_f * 0.9 * 0.30;
            if age > 40 {
                p *= (1.0 - 0.1 * (age - 40) as f32).max(0.0);
            }
            p *= (1.0 - nkids as f32 / 9.0).max(0.1);
            if self.chars[f_id as usize].age(self.day) > 60 {
                p *= 0.3;
            }
            // Hohlkrone senkt Fruchtbarkeit
            if self.chars[f_id as usize].hollowing >= 65.0 || self.chars[m as usize].hollowing >= 65.0 {
                p *= 0.25;
            }
            if self.chance(p) {
                self.give_birth(m, f_id);
            }
        }
    }

    pub fn give_birth(&mut self, mother: CharId, father: CharId) -> CharId {
        let gender = if self.chance(0.5) { Gender::Male } else { Gender::Female };
        // Dynastie: die des höherrangigen Partners (Herrscher/Haupt), sonst des Vaters
        let (dm, df) = (self.chars[mother as usize].dynasty, self.chars[father as usize].dynasty);
        let mother_ruler = self.chars[mother as usize].realm.is_some_and(|r| self.realms[r as usize].ruler == mother);
        let father_ruler = self.chars[father as usize].realm.is_some_and(|r| self.realms[r as usize].ruler == father);
        let dynasty = if mother_ruler && !father_ruler { dm } else { df };
        let culture = self.chars[father as usize].culture;
        let realm = self.chars[mother as usize].realm.or(self.chars[father as usize].realm);
        let house = if dynasty == df { self.chars[father as usize].house } else { self.chars[mother as usize].house };
        let loc = self.chars[mother as usize].location;
        let id = self.spawn_person(None, culture, dynasty, gender, 0, realm, house, loc);
        // Vererbung: Mittelwert der Eltern ±3
        for i in 0..6 {
            let avg = (self.chars[mother as usize].attrs[i] as i32 + self.chars[father as usize].attrs[i] as i32) / 2;
            let v = (avg + self.ri(-3, 3)).clamp(1, 20);
            self.chars[id as usize].attrs[i] = v as i8;
        }
        // Angeborene Eigenschaften erblich
        for t in [Trait::Genius, Trait::Giant, Trait::Sickly, Trait::Quick, Trait::Slow] {
            let parent_has = self.chars[mother as usize].has(t) || self.chars[father as usize].has(t);
            let p = if t == Trait::Genius { 0.15 } else { 0.3 };
            if parent_has && self.chance(p) && !self.chars[id as usize].has(t) {
                self.chars[id as usize].traits.push(t);
            }
        }
        // Glutblut: 20 % flammengeweiht
        if self.dynasties[dynasty as usize].legacies.iter().any(|l| l == "ember_blood") && self.chance(0.2) {
            self.chars[id as usize].traits.push(Trait::FlameSworn);
        }
        self.chars[id as usize].father = Some(father);
        self.chars[id as usize].mother = Some(mother);
        self.chars[mother as usize].children.push(id);
        self.chars[father as usize].children.push(id);
        self.stats.births += 1;
        let name = self.full_name(id);
        let hn = self.dynasties[dynasty as usize].name.clone();
        let heir_line = realm.is_some_and(|r| self.chars[self.realms[r as usize].ruler as usize].dynasty == dynasty);
        let mag = if heir_line { 30 } else { 12 };
        let mut e = Entry::new(ChronicleKind::Birth, "birth", mag).actor(format!("char_{id}"), name.clone(), "Neugeborenes").var("a", name).var("b", hn);
        if let Some(r) = realm {
            e = e.realm(r);
        }
        self.chron(e);
        id
    }

    // ------------------------------------------------------------------------------------
    // Tod
    // ------------------------------------------------------------------------------------
    /// Tötet eine Figur und regelt alle Folgen (Thronfolge, Hausoberhaupt, Ämter, Armeeführung).
    pub fn kill_char(&mut self, c: CharId, cause: DeathCause) {
        if self.chars[c as usize].died.is_some() || self.chars[c as usize].immortal {
            return;
        }
        self.chars[c as usize].died = Some((self.day, cause));
        self.stats.deaths += 1;
        let (realm, house, office, army, spouse, age) = {
            let ch = &self.chars[c as usize];
            (ch.realm, ch.house, ch.office, ch.army, ch.spouse, ch.age(self.day))
        };
        if let Some(s) = spouse {
            self.chars[s as usize].spouse = None;
        }
        if let Some(a) = army
            && let Some(ar) = self.armies.iter_mut().find(|x| x.id == a) {
                ar.commander = None;
            }
        if let Some((r, o)) = office {
            self.realms[r as usize].offices[o.idx()] = None;
        }
        // Armeekommandeure und Provinz-Hauptfiguren: nur Chronik für Bedeutende
        let name = self.full_name(c);
        let is_ruler = realm.is_some_and(|r| self.realms[r as usize].ruler == c);
        let notable = is_ruler || house.is_some_and(|h| self.houses[h as usize].head == c) || self.chars[c as usize].is_hero || office.is_some();
        if notable && !matches!(cause, DeathCause::Abdicated | DeathCause::Deposed) {
            let tpl = match cause {
                DeathCause::Natural | DeathCause::Accident => "death_natural",
                DeathCause::Battle => "death_battle",
                DeathCause::Murder => "death_murder",
                DeathCause::Disease => "death_disease",
                _ => "death_executed",
            };
            let mag = if is_ruler { 55 } else { 28 };
            let ploc = self.chars[c as usize].location;
            let mut e = Entry::new(ChronicleKind::Death, tpl, mag).actor(format!("char_{c}"), name.clone(), "Verstorbener").var("a", name.clone()).var("n", age.to_string()).prov(ploc, &self.pname(ploc)).num("age", age as f32);
            if let Some(r) = realm {
                e = e.realm(r);
            }
            self.chron(e);
        }
        // Hausoberhaupt
        if let Some(h) = house
            && self.houses[h as usize].head == c {
                self.new_house_head(h);
            }
        // Beinamen
        if is_ruler {
            let ep = self.compute_epithet(c);
            self.chars[c as usize].epithet = ep;
        }
        if let Some(r) = realm {
            self.court_remove(r, c);
            if is_ruler {
                self.succession(r, c, cause);
            } else if let Some(reg) = self.realms[r as usize].regency
                && reg.regent == c {
                    self.realms[r as usize].regency = None;
                    self.assign_regent(r);
                }
            if self.realms[r as usize].heir == Some(c) {
                self.realms[r as usize].heir = None;
            }
        }
    }

    fn new_house_head(&mut self, h: HouseId) {
        let dynasty = self.houses[h as usize].dynasty;
        let members: Vec<CharId> = self.living_dynasty_members(dynasty).into_iter().filter(|m| self.chars[*m as usize].house == Some(h) && self.chars[*m as usize].adult(self.day)).collect();
        let new = members.iter().copied().max_by_key(|m| {
            let c = &self.chars[*m as usize];
            (c.age(self.day) as i64) * 2 + c.attr(Attr::Martial) as i64
        });
        match new {
            Some(n) => self.houses[h as usize].head = n,
            None => {
                // Haus erlischt oder wird von einem Kind geführt
                let kid = self.living_dynasty_members(dynasty).into_iter().find(|m| self.chars[*m as usize].house == Some(h));
                match kid {
                    Some(k) => self.houses[h as usize].head = k,
                    None => {
                        self.houses[h as usize].alive = false;
                        for p in self.provinces.iter_mut().filter(|p| p.fief == Some(h)) {
                            p.fief = None;
                        }
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------------------------
    // Nachfolge
    // ------------------------------------------------------------------------------------
    fn first_alive_descendant(&self, c: CharId, oldest_first: bool, depth: u8) -> Option<CharId> {
        if depth > 5 {
            return None;
        }
        let mut kids: Vec<CharId> = self.chars[c as usize].children.clone();
        kids.sort_by_key(|k| self.chars[*k as usize].born);
        if !oldest_first {
            kids.reverse();
        }
        for k in kids {
            if self.chars[k as usize].alive() && self.chars[k as usize].captive_of.is_none() {
                return Some(k);
            }
            if let Some(d) = self.first_alive_descendant(k, oldest_first, depth + 1) {
                return Some(d);
            }
        }
        None
    }

    /// Nächster Erbe in Linie (Kinder, dann Geschwister und deren Linien, dann Onkel/Tanten …).
    fn line_heir(&self, ruler: CharId, oldest_first: bool) -> Option<CharId> {
        if let Some(d) = self.first_alive_descendant(ruler, oldest_first, 0) {
            return Some(d);
        }
        let mut cur = ruler;
        for _ in 0..3 {
            let ch = &self.chars[cur as usize];
            let parent = ch.father.filter(|p| self.chars[*p as usize].dynasty == ch.dynasty).or(ch.mother.filter(|p| self.chars[*p as usize].dynasty == ch.dynasty));
            let Some(p) = parent else { break };
            let mut sibs: Vec<CharId> = self.chars[p as usize].children.iter().copied().filter(|s| *s != cur).collect();
            sibs.sort_by_key(|k| self.chars[*k as usize].born);
            if !oldest_first {
                sibs.reverse();
            }
            for s in sibs {
                if self.chars[s as usize].alive() {
                    return Some(s);
                }
                if let Some(d) = self.first_alive_descendant(s, oldest_first, 0) {
                    return Some(d);
                }
            }
            cur = p;
        }
        None
    }

    fn char_score(&self, c: CharId) -> f32 {
        let ch = &self.chars[c as usize];
        ATTRS.iter().map(|a| ch.attr(*a) as f32).sum::<f32>() + ch.prestige / 50.0 - if ch.has(Trait::Lunatic) { 30.0 } else { 0.0 }
    }

    /// Bestimmt den Erben nach dem Erbgesetz. Gibt auch Kandidaten außerhalb der Dynastie zurück (Wahl).
    pub fn find_heir(&mut self, r: RealmId, ruler: CharId) -> Option<(CharId, bool)> {
        let law = self.realms[r as usize].laws.succession;
        let gov = self.realms[r as usize].gov;
        let dyn_id = self.chars[ruler as usize].dynasty;
        let members: Vec<CharId> = self.living_dynasty_members(dyn_id).into_iter().filter(|m| *m != ruler && self.chars[*m as usize].captive_of.is_none()).collect();
        let adult = |w: &Self, c: CharId| w.chars[c as usize].adult(w.day);
        // Republiken und Zirkel wählen
        if matches!(gov, Government::MerchantRepublic | Government::Druidic | Government::ClanCouncil) || law == SuccLaw::Elective {
            let mut cand: Vec<(CharId, f32)> = Vec::new();
            for m in &members {
                if adult(self, *m) {
                    cand.push((*m, self.char_score(*m) + 15.0));
                }
            }
            let houses: Vec<HouseId> = self.houses.iter().filter(|h| h.alive && h.realm == r).map(|h| h.id).collect();
            for h in houses {
                let head = self.houses[h as usize].head;
                if head != ruler && self.chars[head as usize].alive() && adult(self, head) {
                    let s = self.char_score(head) + self.houses[h as usize].power * 0.3;
                    cand.push((head, s));
                }
            }
            if gov == Government::Druidic {
                for c in cand.iter_mut() {
                    c.1 += self.chars[c.0 as usize].attr(Attr::Learning) as f32 * 1.5;
                }
            }
            // Wahl: bester Kandidat mit etwas Zufall
            let mut best: Option<(CharId, f32)> = None;
            for (c, s) in cand {
                let s = s + self.rf() * 6.0;
                if best.is_none_or(|b| s > b.1) {
                    best = Some((c, s));
                }
            }
            return best.map(|(c, _)| (c, true));
        }
        let pick = match law {
            SuccLaw::Primogeniture | SuccLaw::Partition => self.line_heir(ruler, true),
            SuccLaw::Ultimogeniture => self.line_heir(ruler, false),
            SuccLaw::Seniority => members.iter().copied().filter(|m| adult(self, *m)).max_by_key(|m| self.chars[*m as usize].age(self.day)),
            SuccLaw::Designation => {
                let h = self.realms[r as usize].heir.filter(|h| self.chars[*h as usize].alive());
                h.or_else(|| self.line_heir(ruler, true))
            }
            SuccLaw::Elective => None,
        };
        if let Some(p) = pick {
            return Some((p, false));
        }
        // Irgendein lebendes Dynastiemitglied
        members.iter().copied().max_by_key(|m| self.chars[*m as usize].age(self.day)).map(|m| (m, false))
    }

    /// Reaktion auf den Tod/Abgang des Herrschers.
    pub fn succession(&mut self, r: RealmId, old: CharId, cause: DeathCause) {
        if !self.realms[r as usize].alive {
            return;
        }
        self.stats.successions += 1;
        let old_regency = self.realms[r as usize].regency.take();
        let _ = old_regency;
        let heir = self.find_heir(r, old);
        let mut new_dynasty = false;
        let chosen = match heir {
            Some((h, _elected)) => Some(h),
            None => {
                // Dynastie ohne Erben: stärkstes Haus oder Statthalter übernimmt
                let mut best: Option<(CharId, f32)> = None;
                let hs: Vec<HouseId> = self.houses.iter().filter(|h| h.alive && h.realm == r).map(|h| h.id).collect();
                for h in hs {
                    let head = self.houses[h as usize].head;
                    if self.chars[head as usize].alive() {
                        let s = self.houses[h as usize].power + self.char_score(head) * 0.2;
                        if best.is_none_or(|b| s > b.1) {
                            best = Some((head, s));
                        }
                    }
                }
                new_dynasty = true;
                best.map(|b| b.0)
            }
        };
        let new_ruler = match chosen {
            Some(c) => c,
            None => {
                // Niemand übrig: ein Verwalter der Hauptstadt erhebt sich
                let cap = self.realms[r as usize].capital;
                let culture = self.realms[r as usize].culture;
                let fam = self.random_family_name(culture);
                let d = self.create_dynasty(&fam, culture);
                let g = if self.chance(0.5) { Gender::Male } else { Gender::Female };
                let age = 30 + self.ri(0, 15);
                new_dynasty = true;
                self.spawn_person(None, culture, d, g, age, Some(r), None, cap)
            }
        };
        let old_dyn = self.chars[old as usize].dynasty;
        if self.chars[new_ruler as usize].dynasty != old_dyn {
            new_dynasty = true;
        }
        // Teilung des Reiches bei Partition
        let mut partition_target: Option<CharId> = None;
        if self.realms[r as usize].laws.succession == SuccLaw::Partition && !new_dynasty {
            let sibs: Vec<CharId> = self.chars[old as usize].children.iter().copied().filter(|k| *k != new_ruler && self.chars[*k as usize].alive() && self.chars[*k as usize].adult(self.day)).collect();
            partition_target = sibs.first().copied();
        }
        self.install_ruler(r, new_ruler, Some(old), cause, new_dynasty);
        if let Some(p) = partition_target {
            self.partition_realm(r, p);
        }
    }

    /// Setzt einen neuen Herrscher ein, berechnet Legitimität und prüft Regentschaft.
    pub fn install_ruler(&mut self, r: RealmId, new: CharId, old: Option<CharId>, cause: DeathCause, new_dynasty: bool) {
        let day = self.day;
        // alte Ämterinhaber bleiben; Herrscher kann kein Amt halten
        if let Some((rr, o)) = self.chars[new as usize].office {
            self.realms[rr as usize].offices[o.idx()] = None;
            self.chars[new as usize].office = None;
        }
        self.court_add(r, new);
        let gens = self.realms[r as usize].stats.rulers.min(3) as f32;
        let usurp = new_dynasty;
        let base = 50.0 + 10.0 * gens.min(3.0) + if usurp { -30.0 } else { 20.0 };
        {
            let realm = &mut self.realms[r as usize];
            realm.ruler = new;
            realm.dynasty = self.chars[new as usize].dynasty;
            realm.legitimacy = base.clamp(5.0, 100.0);
            realm.stats.rulers += 1;
            realm.last_ruler_change = day;
            realm.heir = None;
        }
        self.chars[new as usize].reign_start = Some(day);
        self.chars[new as usize].reigns.push(r);
        self.chars[new as usize].guardian = None;
        let age = self.chars[new as usize].age(day);
        // Regentschaft
        let incapable = self.chars[new as usize].has(Trait::Lunatic) || self.chars[new as usize].captive_of.is_some();
        self.realms[r as usize].regency = None;
        if age < 16 || incapable {
            self.assign_regent(r);
        }
        // Reaktion der Häuser
        let hs: Vec<HouseId> = self.houses.iter().filter(|h| h.alive && h.realm == r).map(|h| h.id).collect();
        for h in hs {
            let head = self.houses[h as usize].head;
            let op = self.char_opinion(head, new);
            let hh = &mut self.houses[h as usize];
            hh.loyalty = (hh.loyalty + if usurp { -8.0 } else { 0.0 } + (op * 0.1).clamp(-6.0, 6.0)).clamp(0.0, 100.0);
        }
        let rn = self.realms[r as usize].name.clone();
        let nn = self.full_name(new);
        let tpl = if usurp { "dynasty_change" } else { "succession" };
        let dyn_name = self.dynasties[self.chars[new as usize].dynasty as usize].name.clone();
        let mut e = Entry::new(ChronicleKind::Succession, tpl, 55).realm(r).actor(format!("char_{new}"), nn.clone(), "Herrscher").var("a", if usurp { dyn_name } else { nn.clone() }).var("b", rn);
        if let Some(o) = old {
            e = e.actor(format!("char_{o}"), self.full_name(o), "Vorgänger");
        }
        self.chron(e);
        self.emit(crate::world::StrategyEvent::RulerChanged { realm: r, ruler: new });
        // Gebrochene Verbindungen: Ämter des alten Herrschers bleiben; Beziehungen zu Nachbarn: Meinungsmodifikatoren neu (strukturell, automatisch)
        let _ = cause;
        // Hohepriester-Investitur usw. entfallen
        if self.realms[r as usize].gov == Government::MerchantRepublic || self.realms[r as usize].gov == Government::Druidic {
            self.realms[r as usize].legitimacy = 70.0;
        }
        self.reconcile_house_head_for_ruler(r, new);
    }

    /// Wird der Herrscher aus einem Haus gewählt, wird sein Haus zum Königshaus (kein Amt als Hausoberhaupt mehr nötig).
    fn reconcile_house_head_for_ruler(&mut self, r: RealmId, new: CharId) {
        if let Some(h) = self.chars[new as usize].house
            && self.houses[h as usize].realm == r {
                // Das Haus des neuen Herrschers verliert Fraktions-/Thronanspruch und gewinnt Loyalität
                let hh = &mut self.houses[h as usize];
                hh.throne_claim = false;
                hh.loyalty = (hh.loyalty + 25.0).min(100.0);
                hh.faction = None;
            }
    }

    pub fn assign_regent(&mut self, r: RealmId) {
        let ruler = self.realms[r as usize].ruler;
        let day = self.day;
        // 1. überlebender Ehepartner des Vorgängers (Elternteil), 2. Kanzler, 3. mächtigstes Dynastiemitglied
        let mut regent: Option<CharId> = None;
        for p in [self.chars[ruler as usize].father, self.chars[ruler as usize].mother].into_iter().flatten() {
            if self.chars[p as usize].alive() && self.chars[p as usize].adult(day) && p != ruler {
                regent = Some(p);
                break;
            }
        }
        if regent.is_none() {
            regent = self.realms[r as usize].offices[Office::Chancellor.idx()].filter(|c| self.chars[*c as usize].alive());
        }
        if regent.is_none() {
            let dyn_id = self.chars[ruler as usize].dynasty;
            regent = self.living_dynasty_members(dyn_id).into_iter().filter(|m| *m != ruler && self.chars[*m as usize].adult(day)).max_by(|a, b| self.char_score(*a).total_cmp(&self.char_score(*b)));
        }
        if regent.is_none() {
            let hs: Vec<HouseId> = self.houses.iter().filter(|h| h.alive && h.realm == r).map(|h| h.id).collect();
            regent = hs.iter().map(|h| self.houses[*h as usize].head).filter(|c| self.chars[*c as usize].alive() && *c != ruler).max_by(|a, b| self.char_score(*a).total_cmp(&self.char_score(*b)));
        }
        let Some(rg) = regent else { return };
        let ch = &self.chars[rg as usize];
        let mut loyalty = 55.0 + self.char_opinion(rg, ruler) * 0.2;
        if ch.has(Trait::Ambitious) {
            loyalty -= 15.0;
        }
        if ch.has(Trait::Honest) || ch.has(Trait::Just) {
            loyalty += 10.0;
        }
        if self.chars[rg as usize].father == Some(ruler) || self.chars[ruler as usize].father == Some(rg) || self.chars[ruler as usize].mother == Some(rg) {
            loyalty += 10.0;
        }
        self.realms[r as usize].regency = Some(Regency { regent: rg, ward: ruler, loyalty: loyalty.clamp(0.0, 100.0), since: day });
        self.stats.regencies += 1;
        let (rn, wn) = (self.full_name(rg), self.full_name(ruler));
        let pn = self.realms[r as usize].name.clone();
        let e = Entry::new(ChronicleKind::Succession, "regency", 40).realm(r).actor(format!("char_{rg}"), rn.clone(), "Regent").actor(format!("char_{ruler}"), wn.clone(), "Mündel").var("a", rn).var("b", wn).var("p", pn);
        self.chron(e);
    }

    /// Teilt das Reich bei der Erbfolge „Teilung“: ein Geschwister erhält ein Drittel der Provinzen als Vasallenreich.
    fn partition_realm(&mut self, r: RealmId, heir2: CharId) {
        let cap = self.realms[r as usize].capital;
        let mut provs: Vec<ProvId> = self.provinces_of(r).iter().copied().filter(|p| *p != cap).collect();
        if provs.len() < 3 {
            return;
        }
        provs.truncate(provs.len() / 3);
        let nm = format!("Teilreich {}", self.chars[heir2 as usize].name);
        self.found_realm(&nm, r, heir2, provs, Some(r));
    }

    // ------------------------------------------------------------------------------------
    // Beinamen
    // ------------------------------------------------------------------------------------
    pub fn compute_epithet(&self, c: CharId) -> Option<String> {
        let ch = &self.chars[c as usize];
        let female = ch.gender == Gender::Female;
        let reign = ch.reign_start.map(|s| (self.day.saturating_sub(s)) / DAYS_PER_YEAR).unwrap_or(0);
        let mut cands: Vec<(f32, &str, &str)> = Vec::new();
        let d = ch.deeds;
        if ch.has(Trait::Kinslayer) {
            cands.push((100.0, "der Brudermörder", "die Brudermörderin"));
        }
        if ch.hollowing >= 40.0 {
            cands.push((90.0, "der Aschgezeichnete", "die Aschgezeichnete"));
        }
        if d.defensive_wins >= 3 {
            cands.push((d.defensive_wins as f32 * 12.0, "der Unbeugsame", "die Unbeugsame"));
        }
        if d.conquests >= 3 {
            cands.push((d.conquests as f32 * 10.0, "der Eroberer", "die Eroberin"));
        }
        if d.famines_averted >= 2 {
            cands.push((d.famines_averted as f32 * 14.0, "der Kornherr", "die Kornherrin"));
        }
        if d.alliances >= 3 {
            cands.push((d.alliances as f32 * 10.0, "der Brückenbauer", "die Brückenbauerin"));
        }
        if d.plots >= 2 {
            cands.push((d.plots as f32 * 11.0, "der Ränkeschmied", "die Ränkeschmiedin"));
        }
        if d.battles_won >= 5 {
            cands.push((d.battles_won as f32 * 6.0, "der Siegreiche", "die Siegreiche"));
        }
        if ch.has(Trait::Just) {
            cands.push((30.0, "der Gerechte", "die Gerechte"));
        }
        if ch.has(Trait::Cruel) {
            cands.push((35.0, "der Grausame", "die Grausame"));
        }
        if reign < 2 && ch.reign_start.is_some() {
            cands.push((50.0, "der Kurze", "die Kurze"));
        }
        cands.retain(|c| !c.1.is_empty());
        cands.into_iter().max_by(|a, b| a.0.total_cmp(&b.0)).map(|(_, m, f)| if female { f.to_string() } else { m.to_string() })
    }

    // ------------------------------------------------------------------------------------
    // Jahr / Monat
    // ------------------------------------------------------------------------------------
    pub(crate) fn characters_year(&mut self) {
        let day = self.day;
        let ids: Vec<CharId> = self.chars.iter().filter(|c| c.alive()).map(|c| c.id).collect();
        for c in ids {
            if !self.chars[c as usize].alive() {
                continue;
            }
            let (age, health, immortal) = {
                let ch = &self.chars[c as usize];
                (ch.age(day), ch.health, ch.immortal)
            };
            if immortal {
                continue;
            }
            // Gesundheit driftet mit dem Alter
            {
                let drift = if age > 55 { -0.12 } else if age > 40 { -0.04 } else { 0.0 } + if age < 25 { 0.05 } else { 0.0 };
                let ch = &mut self.chars[c as usize];
                ch.health = (ch.health + drift + if ch.has(Trait::Sickly) { -0.02 } else { 0.0 }).clamp(0.5, 10.0);
                // Hohlkrone: altert nicht mehr
                if ch.hollowing >= 65.0 && ch.health < 6.0 {
                    ch.health = 6.0;
                }
            }
            // Sterblichkeit
            let mut p = if age < 16 {
                0.012
            } else if age <= 30 {
                0.003
            } else {
                0.003 * 1.09f32.powi(age - 30)
            };
            p *= (11.0 - health) / 6.0;
            if self.chars[c as usize].hollowing >= 65.0 {
                p *= 0.1;
            }
            if self.chance(p) {
                let cause = if age > 50 { DeathCause::Natural } else if self.chance(0.4) { DeathCause::Disease } else { DeathCause::Accident };
                self.kill_char(c, cause);
                continue;
            }
            // Erziehung: ab 6 Jahre Fokus, mit 16 Bonus
            if age == 6 {
                let focus = ATTRS[self.ri(0, 5) as usize];
                self.chars[c as usize].education = Some(focus);
            }
            if age == 16 {
                self.finish_education(c);
            }
            // Prestige: bloß Alterung des Ansehens
            let ch = &mut self.chars[c as usize];
            ch.prestige *= 0.97;
            ch.piety *= 0.98;
        }
        self.births_year();
        self.marriages_year();
        self.abdications_year();
        self.cadet_houses_year();
        self.secrets_year();
    }

    fn finish_education(&mut self, c: CharId) {
        let Some(focus) = self.chars[c as usize].education else { return };
        let guardian_attr = self.chars[c as usize].guardian.map(|g| self.chars[g as usize].attr(focus)).unwrap_or(8);
        let bonus = ((guardian_attr - 8) / 2).clamp(-1, 3) as i8;
        let ch = &mut self.chars[c as usize];
        ch.attrs[focus.idx()] = (ch.attrs[focus.idx()] + bonus + 1).clamp(1, 20);
        let tr = match focus {
            Attr::Learning => Some(Trait::Scholar),
            Attr::Martial => Some(Trait::Strategist),
            Attr::Diplomacy => Some(Trait::Courtier),
            Attr::Stewardship => Some(Trait::Steward),
            Attr::Intrigue => Some(Trait::Schemer),
            Attr::Charisma => Some(Trait::PiousMind),
        };
        if let Some(t) = tr
            && !ch.traits.contains(&t) && self.rf_inline() < 0.45 {
                self.chars[c as usize].traits.push(t);
            }
    }

    fn rf_inline(&mut self) -> f32 {
        self.rf()
    }

    /// Herrscher mit schlechter Gesundheit oder Zweifler-Natur können abdanken (Erbe volljährig).
    fn abdications_year(&mut self) {
        let rs: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && r.regency.is_none()).map(|r| r.id).collect();
        for r in rs {
            if self.is_player(r) {
                continue;
            }
            let ruler = self.realms[r as usize].ruler;
            let ch = &self.chars[ruler as usize];
            if ch.immortal || !ch.alive() {
                continue;
            }
            let age = ch.age(self.day);
            let weary = age >= 62 && ch.health < 4.5 || ch.stress > 80.0 || (age >= 55 && ch.has(Trait::Content) && ch.health < 6.0);
            if !weary {
                continue;
            }
            if self.realms[r as usize].gov != Government::Feudal && self.realms[r as usize].gov != Government::Tyranny {
                continue;
            }
            let Some((heir, _)) = self.find_heir(r, ruler) else { continue };
            if !self.chars[heir as usize].adult(self.day) || self.chars[heir as usize].dynasty != self.chars[ruler as usize].dynasty {
                continue;
            }
            if self.chance(0.12) {
                let name = self.full_name(ruler);
                let e = Entry::new(ChronicleKind::Succession, "abdication", 50).realm(r).actor(format!("char_{ruler}"), name.clone(), "Herrscher").var("a", name);
                self.chron(e);
                self.kill_char_abdicate(ruler);
            }
        }
    }

    fn kill_char_abdicate(&mut self, c: CharId) {
        // Abdankung: Figur zieht sich zurück (als „Abgedankt“ aus dem Spiel genommen), Nachfolge läuft normal.
        self.kill_char(c, DeathCause::Abdicated);
    }

    /// Ehen innerhalb und außerhalb der Reiche (KI). Ledige Erwachsene des Hofes finden Partner.
    fn marriages_year(&mut self) {
        let day = self.day;
        let singles: Vec<CharId> = self.chars.iter().filter(|c| c.alive() && c.spouse.is_none() && c.realm.is_some() && !c.immortal && c.age(day) >= 16 && c.age(day) <= 45 && c.captive_of.is_none()).map(|c| c.id).collect();
        for c in singles {
            if self.chars[c as usize].spouse.is_some() {
                continue;
            }
            let Some(r) = self.chars[c as usize].realm else { continue };
            if self.realms[r as usize].kind == RealmKind::Hollowed || self.realms[r as usize].kind == RealmKind::Legion {
                continue;
            }
            // Spielerhof: Heiraten werden vom Spieler beschlossen (außer Autoverwaltung für Kinder).
            let ruler = self.realms[r as usize].ruler == c;
            if self.is_player(r) && !self.chance(0.0) {
                continue;
            }
            let p = if ruler { 0.7 } else { 0.35 };
            if !self.chance(p) {
                continue;
            }
            // Partner: erst im eigenen Reich (anderes Haus / Hof), sonst ein Import
            let gender = self.chars[c as usize].gender;
            let want = if gender == Gender::Male { Gender::Female } else { Gender::Male };
            let myage = self.chars[c as usize].age(day);
            let pool: Vec<CharId> = self.chars.iter().filter(|o| o.alive() && o.spouse.is_none() && o.gender == want && o.id != c && o.realm.is_some() && o.age(day) >= 16 && o.age(day) <= 45 && (o.age(day) - myage).abs() <= 14 && o.captive_of.is_none() && !o.immortal).map(|o| o.id).collect();
            // keine Verwandten
            let pool: Vec<CharId> = pool.into_iter().filter(|o| !self.close_kin(c, *o)).collect();
            // gleicher Rang bevorzugt: eigene Reichsmitglieder und Nachbarn
            let mut best: Option<(CharId, f32)> = None;
            for o in pool {
                let orr = self.chars[o as usize].realm.unwrap();
                if self.is_player(orr) {
                    continue;
                }
                let dist = self.realm_distance(r, orr);
                if dist > 2 && orr != r {
                    continue;
                }
                if self.at_war(r, orr) || matches!(self.realms[orr as usize].kind, RealmKind::Hollowed | RealmKind::Legion) {
                    continue;
                }
                let op = self.char_opinion(c, o);
                let s = op * 0.5 + if orr == r { 10.0 } else { 0.0 } - dist as f32 * 3.0 + self.rf() * 10.0 + self.char_score(o) * 0.2;
                if self.chars[c as usize].religion.relation(self.chars[o as usize].religion) == -1 {
                    continue;
                }
                if best.is_none_or(|b| s > b.1) {
                    best = Some((o, s));
                }
            }
            match best {
                Some((o, s)) if s > 6.0 => {
                    let orr = self.chars[o as usize].realm.unwrap();
                    let alliance = orr != r;
                    self.marry(c, o, alliance);
                    if alliance {
                        self.add_treaty(TreatyKind::Marriage, r, orr, None, c as i64);
                        self.add_opinion(r, orr, OpKind::Marriage, 25.0, None);
                        self.add_opinion(orr, r, OpKind::Marriage, 25.0, None);
                    }
                }
                _ => {
                    // Import: ein adeliger Partner aus der Nachbarschaft entsteht neu (neue Familie)
                    if self.chars[c as usize].alive() && (ruler || self.chars[c as usize].age(day) < 36) {
                        let culture = self.chars[c as usize].culture;
                        let fam = self.random_family_name(culture);
                        let d = self.create_dynasty(&fam, culture);
                        let loc = self.chars[c as usize].location;
                        let sage = (myage + self.ri(-4, 4)).clamp(16, 44);
                        let spouse = self.spawn_person(None, culture, d, want, sage, None, None, loc);
                        self.chars[spouse as usize].realm = None;
                        self.marry(c, spouse, false);
                    }
                }
            }
        }
    }

    pub fn close_kin(&self, a: CharId, b: CharId) -> bool {
        let (ca, cb) = (&self.chars[a as usize], &self.chars[b as usize]);
        if ca.father.is_some() && (ca.father == cb.father || ca.father == Some(b) || cb.father == Some(a)) {
            return true;
        }
        if ca.mother.is_some() && (ca.mother == cb.mother || ca.mother == Some(b) || cb.mother == Some(a)) {
            return true;
        }
        // Cousins: gemeinsamer Großelternteil
        let gp = |w: &Self, c: CharId| -> Vec<CharId> {
            let mut v = Vec::new();
            for p in [w.chars[c as usize].father, w.chars[c as usize].mother].into_iter().flatten() {
                for g in [w.chars[p as usize].father, w.chars[p as usize].mother].into_iter().flatten() {
                    v.push(g);
                }
            }
            v
        };
        let ga = gp(self, a);
        let gb = gp(self, b);
        ga.iter().any(|x| gb.contains(x))
    }

    /// Verheiratete Nebenlinien gründen Kadettenhäuser (GDD 5.6).
    fn cadet_houses_year(&mut self) {
        let day = self.day;
        let rs: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && !matches!(r.kind, RealmKind::Hollowed | RealmKind::Legion)).map(|r| r.id).collect();
        for r in rs {
            let n_houses = self.houses.iter().filter(|h| h.alive && h.realm == r).count();
            if n_houses >= 12 {
                continue;
            }
            let ruler = self.realms[r as usize].ruler;
            let dyn_id = self.chars[ruler as usize].dynasty;
            let heir = self.realms[r as usize].heir;
            let cands: Vec<CharId> = self
                .chars
                .iter()
                .filter(|c| c.alive() && c.dynasty == dyn_id && c.house.is_none() && c.spouse.is_some() && c.age(day) >= 26 && c.id != ruler && Some(c.id) != heir && c.realm == Some(r))
                .map(|c| c.id)
                .collect();
            for c in cands {
                // Nur wenn kein naher Erbe (kein Kind des Herrschers)
                if self.chars[ruler as usize].children.contains(&c) && self.chars[c as usize].age(day) < 30 {
                    continue;
                }
                if self.chance(0.06) {
                    self.found_cadet_house(r, c);
                    break;
                }
            }
        }
    }

    pub fn found_cadet_house(&mut self, r: RealmId, c: CharId) {
        let id = self.houses.len() as HouseId;
        let main_dyn = self.chars[c as usize].dynasty;
        let name = format!("{}-{}", self.dynasties[main_dyn as usize].name, self.chars[c as usize].name);
        let cad = self.create_dynasty(&name, self.chars[c as usize].culture);
        self.dynasties[cad as usize].cadet_of = Some(main_dyn);
        self.dynasties[cad as usize].house = Some(id);
        let seat = self.realms[r as usize].capital;
        let leaning = self.random_leaning();
        self.houses.push(NobleHouse {
            id,
            name: name.clone(),
            realm: r,
            dynasty: cad,
            head: c,
            alive: true,
            power: 2.0,
            loyalty: 70.0,
            wealth: 50.0,
            retinue: 15,
            leaning,
            rights: Vec::new(),
            throne_claim: false,
            faction: None,
            seat,
            cadet: true,
            grudge: 0.0,
        });
        // Figur und Familie wechseln in die Kadettenlinie (Nachkommen behalten die Hauptdynastie für Erbansprüche).
        let kids = self.chars[c as usize].children.clone();
        let spouse = self.chars[c as usize].spouse;
        for m in [Some(c), spouse].into_iter().flatten().chain(kids) {
            self.chars[m as usize].house = Some(id);
        }
        let hn = self.dynasties[cad as usize].name.clone();
        let e = Entry::new(ChronicleKind::Founding, "event_generic", 20).realm(r).var("a", self.realms[r as usize].name.clone()).var("p", self.pname(seat)).var("c", format!("Das Kadettenhaus {hn} wurde gegründet"));
        self.chron(e);
    }

    pub fn random_leaning(&mut self) -> HouseLeaning {
        match self.ri(0, 4) {
            0 => HouseLeaning::War,
            1 => HouseLeaning::Trade,
            2 => HouseLeaning::Faith,
            3 => HouseLeaning::Crown,
            _ => HouseLeaning::Separatist,
        }
    }

    /// Geheimnisse entstehen und kommen heraus.
    fn secrets_year(&mut self) {
        let ids: Vec<CharId> = self.chars.iter().filter(|c| c.alive() && c.realm.is_some()).map(|c| c.id).collect();
        for c in ids {
            if self.chars[c as usize].secrets.len() < 2 && self.chance(0.012) {
                let kind = match self.ri(0, 5) {
                    0 => SecretKind::Affair,
                    1 => SecretKind::Heresy,
                    2 => SecretKind::Illegitimate,
                    3 => SecretKind::Embezzlement,
                    4 => SecretKind::Murder,
                    _ => SecretKind::HollowCult,
                };
                if kind == SecretKind::HollowCult && self.chars[c as usize].hollowing < 10.0 {
                    continue;
                }
                self.chars[c as usize].secrets.push(Secret { kind, public: false, known_by: Vec::new() });
            }
            // Auffliegen
            let n = self.chars[c as usize].secrets.len();
            for i in 0..n {
                if !self.chars[c as usize].secrets[i].public && self.chance(0.03) {
                    self.chars[c as usize].secrets[i].public = true;
                    let (kind, r) = (self.chars[c as usize].secrets[i].kind, self.chars[c as usize].realm);
                    let pen = match kind {
                        SecretKind::HollowCult => -60.0,
                        SecretKind::Heresy => -30.0,
                        SecretKind::Murder => -40.0,
                        _ => -10.0,
                    };
                    self.chars[c as usize].prestige += pen * 3.0;
                    if self.realms[r.unwrap_or(0) as usize].ruler == c {
                        self.realms[r.unwrap() as usize].legitimacy = (self.realms[r.unwrap() as usize].legitimacy + pen * 0.1).max(0.0);
                    }
                }
            }
        }
    }

    pub(crate) fn dynasties_year(&mut self) {
        let day = self.day;
        let n = self.dynasties.len();
        // Mitglieder zählen (eine Passage)
        let mut alive = vec![0u32; n];
        let mut prestige = vec![0f32; n];
        for c in self.chars.iter().filter(|c| c.alive()) {
            alive[c.dynasty as usize] += 1;
            prestige[c.dynasty as usize] += c.prestige.max(0.0) * 0.1;
        }
        for d in 0..n {
            if self.dynasties[d].extinct {
                continue;
            }
            if alive[d] == 0 {
                self.dynasties[d].extinct = true;
                self.dynasties[d].extinct_day = Some(day);
                self.stats.dynasties_extinct += 1;
                if let Some(h) = self.dynasties[d].house {
                    self.houses[h as usize].alive = false;
                    for p in self.provinces.iter_mut().filter(|p| p.fief == Some(h)) {
                        p.fief = None;
                    }
                }
                let name = self.dynasties[d].name.clone();
                // letzter Vertreter
                let last = self.chars.iter().rev().find(|c| c.dynasty == d as DynId).map(|c| c.id);
                let b = last.map(|l| self.cname(l)).unwrap_or_default();
                if self.dynasties[d].prestige > 150.0 || self.dynasties[d].house.is_some() {
                    let e = Entry::new(ChronicleKind::Succession, "dynasty_extinct", 45).var("a", name).var("b", b);
                    self.chron(e);
                }
                continue;
            }
            self.dynasties[d].prestige = self.dynasties[d].prestige * 0.99 + prestige[d];
            // Vermächtnisse
            let target = (self.dynasties[d].prestige / 1000.0) as usize;
            while self.dynasties[d].legacies.len() < target.min(LEGACIES.len()) {
                let idx = self.dynasties[d].legacies.len();
                self.dynasties[d].legacies.push(LEGACIES[idx].0.to_string());
            }
        }
        // Legitimität steigt mit Regierungsjahren (+1/Jahr, bis Basis+20)
        for r in self.realms.iter_mut().filter(|r| r.alive) {
            r.legitimacy = (r.legitimacy + 1.0).min(100.0);
        }
    }

    /// Hat die Dynastie des Reiches ein bestimmtes Vermächtnis?
    pub fn has_legacy(&self, r: RealmId, id: &str) -> bool {
        let d = self.realms[r as usize].dynasty;
        self.dynasties.get(d as usize).is_some_and(|d| d.legacies.iter().any(|l| l == id))
    }

    pub(crate) fn characters_month(&mut self) {
        let day = self.day;
        // Regentschaften
        let rs: Vec<RealmId> = self.realms.iter().filter(|r| r.alive && r.regency.is_some()).map(|r| r.id).collect();
        for r in rs {
            let Some(reg) = self.realms[r as usize].regency else { continue };
            let ward_age = self.chars[reg.ward as usize].age(day);
            if !self.chars[reg.regent as usize].alive() {
                self.realms[r as usize].regency = None;
                self.assign_regent(r);
                continue;
            }
            if ward_age >= 16 && self.chars[reg.ward as usize].alive() && !self.chars[reg.ward as usize].has(Trait::Lunatic) {
                self.realms[r as usize].regency = None;
                let wn = self.full_name(reg.ward);
                let e = Entry::new(ChronicleKind::Succession, "event_generic", 30).realm(r).var("a", self.realms[r as usize].name.clone()).var("p", self.realms[r as usize].name.clone()).var("c", format!("{wn} wurde volljährig und übernahm die Regierung"));
                self.chron(e);
                continue;
            }
            // Loyalität driftet nach Eigenschaften
            let drift = {
                let rg = &self.chars[reg.regent as usize];
                (if rg.has(Trait::Ambitious) { -0.8 } else { 0.2 }) + if rg.has(Trait::Honest) { 0.4 } else { 0.0 } + if self.is_player(r) { 0.0 } else { 0.1 }
            };
            let loyalty = (reg.loyalty + drift).clamp(0.0, 100.0);
            self.realms[r as usize].regency.as_mut().unwrap().loyalty = loyalty;
            // Usurpationschance = max(0, (50 − Loyalität)/10) % × (1 + Ehrgeizig)
            let amb = if self.chars[reg.regent as usize].has(Trait::Ambitious) { 2.0 } else { 1.0 };
            let p = ((50.0 - loyalty) / 10.0).max(0.0) * 0.01 * amb;
            if self.chance(p) {
                self.regent_usurps(r, reg);
            }
        }
        // Stress, Zusammenbruch, Ehrgeiz, Misstrauen
        let rulers: Vec<(RealmId, CharId)> = self.realms.iter().filter(|r| r.alive).map(|r| (r.id, r.ruler)).collect();
        for (r, c) in rulers {
            if !self.chars[c as usize].alive() {
                continue;
            }
            let mut dstress = 0.0;
            {
                let ch = &self.chars[c as usize];
                if ch.has(Trait::Paranoid) {
                    dstress += 1.0;
                }
                if ch.has(Trait::Ambitious) && day.saturating_sub(self.realms[r as usize].last_ruler_change) > 5 * DAYS_PER_YEAR && self.realms[r as usize].stats.peak_provinces <= self.provinces_of(r).len() as u32 {
                    dstress += 2.0;
                }
                if ch.has(Trait::Drunkard) {
                    dstress -= 1.0;
                }
                dstress -= 1.0;
                if self.in_any_war(r) {
                    dstress += 1.5;
                }
            }
            if self.has_legacy(r, "iron_will") && dstress > 0.0 {
                dstress *= 0.8;
            }
            if self.chars[c as usize].has(Trait::Content) && dstress > 0.0 {
                dstress *= 0.8;
            }
            let ch = &mut self.chars[c as usize];
            ch.stress = (ch.stress + dstress).clamp(0.0, 100.0);
            let stress = ch.stress;
            if stress >= 50.0 && !self.chars[c as usize].has(Trait::Melancholic) && !self.chars[c as usize].has(Trait::Lunatic) && self.chance(0.02) {
                // Zusammenbruch
                if self.chance(0.05) {
                    self.chars[c as usize].traits.push(Trait::Lunatic);
                } else {
                    self.chars[c as usize].traits.push(Trait::Melancholic);
                }
                self.chars[c as usize].stress = 30.0;
            }
            // Hohlung des Herrschers
            let hol = self.chars[c as usize].has(Trait::HollowTouched);
            if hol {
                self.chars[c as usize].hollowing = (self.chars[c as usize].hollowing + 3.0 / 12.0).min(100.0);
            }
            let h = self.chars[c as usize].hollowing;
            if h >= 60.0 && !self.chars[c as usize].has(Trait::Lunatic) && !self.chars[c as usize].has(Trait::FlameSworn) {
                self.chars[c as usize].traits.push(Trait::Lunatic);
            }
            if self.chars[c as usize].has(Trait::FlameSworn) {
                self.chars[c as usize].hollowing = self.chars[c as usize].hollowing.min(20.0);
            }
        }
        // Verwundete heilen
        for ch in self.chars.iter_mut().filter(|c| c.alive() && c.wounded_until > 0 && c.wounded_until <= day) {
            ch.wounded_until = 0;
        }
        // Erziehung zuweisen (Kinder ab 6 erhalten Erzieher)
        let kids: Vec<CharId> = self.chars.iter().filter(|c| c.alive() && c.guardian.is_none() && c.realm.is_some() && (6..16).contains(&c.age(day)) && c.education.is_some()).map(|c| c.id).collect();
        for k in kids {
            let r = self.chars[k as usize].realm.unwrap();
            let focus = self.chars[k as usize].education.unwrap();
            let g = self.realms[r as usize].court.iter().copied().filter(|c| *c != k && self.chars[*c as usize].alive() && self.chars[*c as usize].adult(day)).max_by_key(|c| self.chars[*c as usize].attr(focus));
            self.chars[k as usize].guardian = g;
        }
    }

    fn regent_usurps(&mut self, r: RealmId, reg: Regency) {
        let ward = reg.ward;
        let regent = reg.regent;
        self.realms[r as usize].regency = None;
        let name = self.full_name(regent);
        let wname = self.full_name(ward);
        let e = Entry::new(ChronicleKind::Plot, "deposed", 60).realm(r).actor(format!("char_{regent}"), name.clone(), "Regent").actor(format!("char_{ward}"), wname.clone(), "Mündel").var("a", wname).var("b", name);
        self.chron(e);
        self.chars[ward as usize].claims.push(r);
        self.kill_char(ward, DeathCause::Deposed);
        self.chars[regent as usize].traits.push(Trait::Kinslayer);
        self.install_ruler(r, regent, Some(ward), DeathCause::Deposed, true);
    }

    /// Vollständig tote Linie → Ruf der Dynastie. Hilfsfunktion für Tests und Oberfläche.
    pub fn dynasty_alive_count(&self) -> usize {
        self.dynasties.iter().filter(|d| !d.extinct).count()
    }
}
