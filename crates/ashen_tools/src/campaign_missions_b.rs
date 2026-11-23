//! Missionen der Zeitalter III und IV (gebrochene Grenzen, Eroberung).

use crate::campaign_forge::*;
use crate::forge::{area, group, guard, pl, tagged, trig, wall_ring, Canvas};
use ashen_sim::data::{Cost, GameData};
use ashen_sim::genmap::GenPreset;
use ashen_sim::map::{DecorKind, Terrain};
use ashen_sim::mission::{Action, Condition, MissionDef};
use ashen_sim::scenario::StartPos;

pub(crate) fn all(data: &GameData, s: &mut Script) -> Vec<MissionDef> {
    vec![m09(data, s), m10(data, s), m11(data, s), m13(data, s), m14(data, s)]
}

fn hero_at(x: f32, y: f32) -> ashen_sim::mission::Placement {
    tagged(pl("hero_isolde", 0, x, y), "hero")
}

// ---------------------------------------------------------------------------------------------
// Zeitalter III – Mission 9: Der Schwarze Winter (Verteidigung mit Verbündeten)
// ---------------------------------------------------------------------------------------------

fn m09(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m09_black_winter";
    s.title("m09", "Mission 9 – Der Schwarze Winter");
    let mut c = Canvas::new(80, 72, 90509, GenPreset::Highlands);
    c.flatten(18, 12, 62, 58, Terrain::Stone);
    c.flatten(26, 20, 54, 50, Terrain::Ash);
    c.path(&[(40.0, 50.0), (40.0, 71.0)], 1.5, Terrain::Road);
    c.path(&[(2.0, 35.0), (26.0, 35.0)], 1.5, Terrain::Road);
    c.path(&[(40.0, 2.0), (40.0, 20.0)], 1.5, Terrain::Road);
    c.path(&[(54.0, 35.0), (78.0, 35.0)], 1.5, Terrain::Road);
    for (x, y) in [(10.5, 16.5), (14.5, 52.5), (66.5, 18.5), (68.5, 54.5), (6.5, 40.5)] {
        c.deco(DecorKind::Boulder, x, y);
    }
    for (x, y) in [(30.5, 24.5), (50.5, 24.5), (30.5, 46.5), (50.5, 46.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    for (x, y) in [(34.5, 40.5), (46.5, 40.5), (40.5, 26.5)] {
        c.deco(DecorKind::Lantern, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Der Schwarze Winter",
        905,
        "Verteidigung mit Verbündeten",
        "Die Bergfeste Hrimgard hält die einzige Passstraße Skarnheims. Haltet die Mauern gegen eine Hohlenarmee im Schneetreiben – an der Seite Torvald Eisenhands, wenn der Schwur gelingt.",
        "Hrimgard, Skarnheim, Jahr 905 n.F. Die Hohlen sind über die Pässe gekommen, die Skarnheim seit tausend Jahren mit Schnee und Schwüren verschließt. Torvald Eisenhand hält Hrimgard mit der Hälfte seines Clans; die andere Hälfte liegt in den Gräbern. Er verlangt einen Preis für seinen Schild: Blut oder Gold. Entscheidet beim Torwächter, wie Ihr den Schwur leistet, und haltet dann die Bergfeste bis zum Morgen der dritten Stunde. Der Winter frisst alles Leben, aber nicht den Willen der Ahnen. Die Hohlen kommen aus drei Richtungen, und mit ihnen Hexen und Kolosse.",
        "Als der Morgen über Hrimgard anbrach, lagen die Hohlen in Reihen vor den Mauern – und die Ahnen sangen, so sagte man, bis ins Tal.",
        "Hrimgard fiel in einer Nacht aus Schnee und Asche. Die Pässe Skarnheims standen offen.",
        &["siege_defense", "faction_skarn_clans_battle"],
    );
    m.map_seed = 90509;
    m.start_resources = Cost { gold: 400.0, wood: 300.0, stone: 400.0, food: 400.0 };
    m.start_goods = vec![("weapons".into(), 16.0), ("armor".into(), 10.0), ("planks".into(), 20.0), ("tools".into(), 6.0)];
    m.defeat_tag = "player_keep".into();
    m.start_time = Some(0.80);
    let mut p = vec![
        tagged(pl("keep", 0, 38.0, 32.0), "player_keep"),
        pl("barracks", 0, 30.0, 26.0),
        pl("barracks", 0, 46.0, 26.0),
        pl("watchtower", 0, 31.0, 40.0),
        pl("watchtower", 0, 46.0, 40.0),
        group("worker", 0, 40.0, 44.0, 6),
        group("swordsman", 0, 34.0, 38.0, 9),
        group("spearman", 0, 44.0, 38.0, 6),
        group("archer", 0, 40.0, 47.0, 8),
        group("crossbowman", 0, 36.0, 47.0, 4),
        group("knight", 0, 48.0, 33.0, 2),
        hero_at(40.0, 40.0),
    ];
    wall_ring(&mut p, 24, 18, 56, 52, &[(40, 52), (24, 35)], &[(24, 18), (55, 18), (24, 51), (55, 51)]);
    m.placements = p;
    m.areas = vec![
        area("hof", "Der Burghof", 34, 36, 12, 8),
        area("nord", "Nordpass", 34, 2, 12, 5),
        area("west", "Westpass", 2, 30, 5, 10),
        area("ost", "Ostpass", 73, 30, 5, 10),
        area("sued", "Südpass", 34, 66, 12, 5),
    ];
    m.starts = vec![StartPos { owner: 0, x: 40.0, y: 40.0 }, StartPos { owner: 1, x: 40.0, y: 4.0 }];
    m.objectives = vec![
        objective("obj_oath", "Leistet den Schwur an Torvald Eisenhand.", false),
        objective("obj_hold", "Haltet Hrimgard bis zum Morgen.", false),
        objective("obj_keep", "Der Bergfried darf nicht fallen.", false),
    ];
    let intro = s.dlg("m09", "schwur", &[
        (Sp::Torvald, "Die Wächterin des Südens erklimmt also meinen Berg. Du bist kleiner, als die Lieder sagen.", "dröhnend, amüsiert"),
        (Sp::Isolde, "Lieder übertreiben. Eure machen Euch angeblich größer und weiser.", "trocken, furchtlos"),
        (Sp::Torvald, "Ha! Sie beißt! Gut. Die Toten kamen vor drei Nächten über den hohen Pass. Wir haben vierzig der Unseren begraben. Vierzig.", "grimmig, kurz auflachend, dann dunkel"),
        (Sp::Isolde, "Dann wollen wir dasselbe. Die Hohlen fort und unsere Toten in Frieden.", "ernst, respektvoll"),
        (Sp::Torvald, "Wollen ist billig. Der Ahnenpfad verlangt mehr. Südländer zahlen in Gold, Sippe zahlt in Blut. Was bist du?", "scharf, prüfend"),
    ]);
    s.note("m09", "schwur", "Der Torhof von Hrimgard. Torvald Eisenhand empfängt Isolde vor den Gräbern seiner Ahnen.");
    let blood = vec![
        s.dlg("m09", "schwur_a", &[
            (Sp::Isolde, "Gib mir das Messer. Mein Blut auf euren Steinen und mein Schwert an der Seite deines Clans.", "fest, ohne Zögern"),
            (Sp::Torvald, "Hört ihr das, Väter? Skarnheim hat eine neue Schwester! Bringt den Met – und die Äxte!", "jubelnd, laut"),
        ]),
        sp_units("hero_torvald", 0, "hof", 1, false, Some("torvald")),
        sp_units("shieldbearer", 0, "hof", 6, false, None),
        sp_units("halberdier", 0, "hof", 4, false, None),
    ];
    let gold = vec![
        s.dlg("m09", "schwur_b", &[
            (Sp::Isolde, "Ich bin keine Sippe, Torvald. Aber mein Gold ist gut und mein Wort ist besser. Nenne deinen Preis.", "ruhig, geschäftsmäßig"),
            (Sp::Torvald, "Gold also. Eisen bekommst du, und Krieger gegen Lohn. Aber erwarte nicht, dass sie für dich sterben.", "ernüchtert, abweisend"),
        ]),
        sp_units("halberdier", 0, "hof", 5, false, None),
        sp_units("crossbowman", 0, "hof", 7, false, None),
    ];
    s.note("m09", "schwur_a", "Entscheidung: Blutschwur. Torvald und seine Clankrieger kämpfen mit.");
    s.note("m09", "schwur_b", "Entscheidung: Bezahlung in Gold. Söldner statt Torvald.");
    let oath = s.decision(
        "m09",
        "schwur",
        "torvald_oath",
        "Der Schwur der Ahnen",
        (Sp::Torvald, "Wähle, bevor der Schnee die Gräber bedeckt, Wächterin. Meine Ahnen sind ungeduldig.", "leiser, drängend"),
        vec![
            opt("torvald_blood", "Den Blutschwur leisten", "Torvald und seine Clankrieger kämpfen an Eurer Seite – auch in späteren Schlachten.", &[], blood),
            opt("torvald_gold", "Mit Gold bezahlen", "Söldner verstärken Euch jetzt; Torvald bleibt kühl.", &[], gold),
        ],
    );
    let nightfall = s.dlg("m09", "sturm", &[
        (Sp::Narrator, "Der Schnee fiel schwarz, wo die Asche ihn berührte. Auf den Zinnen von Hrimgard sangen die Skaldinnen ihre Ahnenlieder gegen den Wind, und der Wind sang zurück – mit hohlen Stimmen.", "rau, getragen"),
    ]);
    s.note("m09", "sturm", "Der Sturm setzt ein; die erste Welle naht.");
    let colossi = s.dlg("m09", "kolosse", &[(Sp::Torvald, "Kolosse! Die Äxte an die Mauer, ihr Brüder – bricht der Koloss die Mauer, brechen wir den Koloss!", "brüllend, kampflustig")]);
    s.note("m09", "kolosse", "Grabkolosse erreichen die Mauern.");
    let dawn = s.dlg("m09", "morgen", &[
        (Sp::Narrator, "Als der Morgen kam, lagen die Hohlen wie umgekippte Statuen im Schnee. Auf der Mauer standen die Überlebenden und sahen nicht hinunter – sie sahen nach Osten, wo es hell wurde.", "ruhig, erschöpft"),
    ]);
    s.note("m09", "morgen", "Der Morgen über Hrimgard.");
    let mk = |t: f32, id: &str, a: &str, u: &[(&str, u32)]| trig(id, Condition::AtTime(t), wave(a, u));
    m.triggers = vec![
        trig("schwur", Condition::AtTime(1.0), vec![intro, oath, add_obj("obj_oath")]),
        trig("start", any_flag(&["torvald_blood", "torvald_gold"]), vec![done_obj("obj_oath"), add_obj("obj_hold"), add_obj("obj_keep"), Action::SetWeather(ashen_sim::env::Weather::Fog), nightfall]),
        mk(75.0, "w1", "west", &[("hollow_thrall", 10), ("hollow_archer", 3)]),
        mk(135.0, "w2", "nord", &[("hollow_thrall", 12), ("ghoul", 4)]),
        mk(200.0, "w3", "ost", &[("hollow_thrall", 10), ("ash_witch", 2), ("hollow_archer", 4)]),
        trig("w4", Condition::AtTime(265.0), {
            let mut a = vec![colossi];
            a.extend(wave("sued", &[("grave_brute", 2), ("hollow_thrall", 10), ("bat_swarm", 3)]));
            a
        }),
        mk(335.0, "w5", "nord", &[("ghoul", 6), ("ash_witch", 3), ("hollow_thrall", 12)]),
        mk(405.0, "w6", "west", &[("grave_brute", 3), ("hollow_thrall", 14), ("hollow_archer", 6)]),
        trig("morgen", Condition::AtTime(500.0), vec![done_obj("obj_hold"), done_obj("obj_keep"), dawn, chron("Hrimgard hielt im Schwarzen Winter."), Action::Victory]),
    ];
    let mut v1 = variant(&["core_kept"], &[], "Zwei Kampfmagier aus Brennhollow haben den Aschglas-Kern studiert und stehen auf den Zinnen.");
    v1.placements = vec![group("battle_mage", 0, 40.0, 29.0, 2)];
    let mut v2 = variant(&["oath_crown"], &[], "Königliche Armbrustschützen haben Hrimgard erreicht.");
    v2.placements = vec![group("crossbowman", 0, 36.0, 36.0, 4)];
    let mut v3 = variant(&["oath_corvane"], &[], "Nordmarker Speerträger verstärken die Tore.");
    v3.placements = vec![group("spearman", 0, 44.0, 36.0, 6)];
    let mut v4 = variant(&["core_smashed"], &[], "Hals Gebete werden nicht mehr vom Aschglas gestört: Zwei Priester der Flamme heilen auf den Wällen.");
    v4.placements = vec![group("priest", 0, 40.0, 29.0, 2)];
    m.variants = vec![v1, v2, v3, v4];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter III – Mission 10: Wurzel und Asche (Heldenmission im Dornwald)
// ---------------------------------------------------------------------------------------------

fn m10(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m10_root_and_ash";
    s.title("m10", "Mission 10 – Wurzel und Asche");
    let mut c = Canvas::new(88, 88, 90610, GenPreset::Heartland);
    c.forest(0, 0, 87, 87, 2);
    let path = [(8.0, 80.0), (20.0, 72.0), (30.0, 66.0), (38.0, 54.0), (44.0, 42.0), (52.0, 34.0), (60.0, 26.0), (70.0, 18.0)];
    c.path(&path, 3.2, Terrain::DarkGrass);
    c.path(&path, 1.0, Terrain::Dirt);
    c.disc(10.0, 78.0, 6.0, Terrain::Grass);
    c.disc(30.0, 66.0, 6.5, Terrain::DarkGrass);
    c.disc(44.0, 42.0, 7.5, Terrain::DarkGrass);
    c.disc(60.0, 26.0, 6.0, Terrain::DarkGrass);
    c.disc(74.0, 14.0, 12.0, Terrain::Ash);
    c.disc(74.0, 14.0, 8.0, Terrain::Grass);
    for i in 0..8 {
        let a = i as f32 / 8.0 * std::f32::consts::TAU;
        c.deco(DecorKind::DeadTree, 74.0 + a.cos() * 9.5, 14.0 + a.sin() * 9.5);
    }
    for (x, y) in [(70.5, 12.5), (78.5, 12.5), (74.5, 18.5), (74.5, 9.5)] {
        c.deco(DecorKind::Bones, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Wurzel und Asche",
        906,
        "Heldenmission ohne Basis",
        "Im Dornwald welken die Bäume von innen. Sylaen der Veyl führt Isolde zur Wurzelmutter, deren Wurzeln grau geworden sind – und die Wahl, was mit ihr geschehen soll, liegt bei Euch.",
        "Dornwald, Jahr 906 n.F. Am Waldrand sterben die Bäume von innen: Das Laub bleibt grün, die Rinde wird grau, und nachts hört man den Wald atmen. Sylaen der Veyl, Druidin des Dornenbundes, hat Isolde Raventhorn um Hilfe gebeten – mit der Warnung, dass sie nicht verspricht, was sie nicht halten kann. Ihr führt einen kleinen Spähtrupp, kein Heer, durch drei Lager der Welkenden bis zum Herzhain, wo die Wurzelmutter träumt. Dort entscheidet sich, ob Isolde sie verbrennt oder in den Traum tritt und versucht, sie zu wecken.",
        "Der Hain war still, und aus den grauen Wurzeln stieg ein Hauch, der nach Frühling roch. Wie teuer er bezahlt war, wusste nur, wer dabei gewesen war.",
        "Der Dornwald behielt seine Wanderer – und die Wurzelmutter ihren grauen Traum.",
        &["explore_wilds", "boss_root_mother"],
    );
    m.map_seed = 90610;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.34);
    m.placements = vec![
        hero_at(10.0, 78.5),
        tagged(pl("hero_hal_venn", 0, 12.0, 77.0), "hero_hal"),
        group("swordsman", 0, 8.0, 76.5, 5),
        group("halberdier", 0, 12.5, 80.5, 2),
        group("archer", 0, 9.0, 81.0, 5),
        group("priest", 0, 13.5, 78.5, 2),
        group("knight", 0, 6.5, 80.0, 2),
        // Welkende Lager.
        tagged(guard("hollow_thrall", 1, 30.0, 66.0, 1), "wk1"),
        guard("hollow_thrall", 1, 28.5, 64.5, 3),
        guard("hollow_archer", 1, 32.5, 68.0, 2),
        tagged(guard("ghoul", 1, 44.0, 42.0, 1), "wk2"),
        guard("ghoul", 1, 42.0, 44.0, 2),
        guard("hollow_thrall", 1, 46.5, 40.5, 3),
        tagged(guard("grave_brute", 1, 60.0, 26.0, 1), "wk3"),
        guard("hollow_thrall", 1, 58.0, 28.0, 3),
        guard("hollow_archer", 1, 62.5, 24.0, 2),
        tagged(guard("hollow_thrall", 1, 36.0, 56.0, 2), "ambush"),
    ];
    m.areas = vec![
        area("lichtung", "Die Lichtung der Veyl", 4, 72, 12, 12),
        area("lager1", "Erstes Lager der Welkenden", 24, 60, 12, 12),
        area("lager2", "Zweites Lager am Ring der Pilze", 38, 36, 12, 12),
        area("lager3", "Drittes Lager am Dornentor", 54, 20, 12, 12),
        area("hain", "Der Herzhain", 62, 4, 24, 22),
        area("hain_mitte", "Das Herz des Hains", 70, 10, 8, 8),
    ];
    m.starts = vec![StartPos { owner: 0, x: 10.0, y: 78.0 }, StartPos { owner: 1, x: 60.0, y: 26.0 }];
    m.objectives = vec![
        objective("obj_camps", "Räumt die drei Lager der Welkenden.", false),
        objective("obj_grove", "Erreicht den Herzhain und stellt Euch der Wurzelmutter.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m10", "wald", &[
        (Sp::Narrator, "Der Dornwald war älter als die Götter, und er hatte es nie vergessen. Wer ihn betrat, tat es auf Zehenspitzen – und wer ihn verließ, nahm ein wenig Rinde in den Knochen mit.", "ehrfürchtig, leise"),
        (Sp::Sylaen, "Geh leise, Wächterin. Hier sprach Calyx zuletzt zu uns, bevor die Götter verstummten. Was welkt, welkt von innen – nicht durch Feuer, sondern durch einen Traum.", "gedämpft, ernst, melodisch"),
    ]);
    s.note("m10", "wald", "Lichtung der Veyl am Waldrand.");
    let c1 = s.dlg("m10", "lager1", &[(Sp::Sylaen, "Sie sind nicht böse. Sie sind leer. Der Wald hat ihre Namen vergessen, und sie suchen sie in unseren Stimmen.", "traurig, gedämpft")]);
    s.note("m10", "lager1", "Das erste Lager ist geräumt.");
    let c3 = s.dlg("m10", "lager3", &[(Sp::Isolde, "Die Wurzeln sind grau. Wie das Aschglas bei Vael Dorn. Sylaen, was immer da im Hain wartet, es ist dasselbe Grau.", "erkennend, unbehaglich")]);
    s.note("m10", "lager3", "Das dritte Lager ist geräumt, der Herzhain liegt vor ihnen.");
    let grove = s.dlg("m10", "hain", &[
        (Sp::RootMother, "Kinder … so viele Kinder mit Feuer in den Händen … seid ihr gekommen, um mich zu halten oder um mich zu verbrennen?", "uralt, zerrissen, mehrstimmig"),
        (Sp::Sylaen, "Sie träumt den Traum des Tiefs. Erwacht sie so, wird der Dornwald zu einer zweiten Schleiernarbe.", "flüsternd, drängend"),
        (Sp::Deep, "Eure Götter wurden nicht getötet, kleine Flamme. Sie wurden … verdaut. Calyx schmeckte nach Frühling.", "intim, kalt, genüsslich"),
        (Sp::Sylaen, "Es gibt zwei Wege, und ich hasse beide. Verbrenne die grauen Wurzeln, und sie stirbt. Oder tritt in ihren Traum und versuche, sie selbst zu wecken.", "gequält, nüchtern"),
    ]);
    s.note("m10", "hain", "Der Herzhain: Die Wurzelmutter spricht, die Stimme aus dem Tief mischt sich ein.");
    let burn = vec![
        s.dlg("m10", "hain_a", &[
            (Sp::Isolde, "Es tut mir leid, Sylaen. Ich kann nicht einen ganzen Wald auf einen Traum setzen. Bringt die Fackeln.", "schwer, entschlossen"),
            (Sp::RootMother, "Warm … so warm … schlaft gut, Kinder …", "verlöschend, friedlich"),
            (Sp::Sylaen, "Der Dornwald wird sich an dieses Feuer erinnern, Wächterin. Und ich auch.", "Trauer, eisig"),
        ]),
        sp_units("boss_root_mother", 1, "hain_mitte", 1, true, Some("boss")),
        Action::SetFlag("rootmother_burn".into()),
    ];
    let pact = vec![
        s.dlg("m10", "hain_b", &[
            (Sp::Isolde, "Keine Feuer mehr. Zeig mir den Weg hinein.", "entschlossen, mit Angst"),
            (Sp::Deep, "Ja … komm herein. Ich habe so lange gewartet, dich richtig kennenzulernen.", "intim, flüsternd"),
            (Sp::Sylaen, "Halt meine Hand und lass nicht los. Was immer du dort drin hörst … es ist nicht dein Freund.", "eindringlich, leise"),
        ]),
        sp_units("hollow_thrall", 1, "hain", 8, true, Some("traum")),
        sp_units("ghoul", 1, "hain", 3, true, Some("traum")),
        Action::SetFlag("rootmother_dream".into()),
    ];
    s.note("m10", "hain_a", "Entscheidung: Wurzeln verbrennen – die Wurzelmutter erwacht als Boss.");
    s.note("m10", "hain_b", "Entscheidung: In den Traum treten – Traumgestalten bedrängen die Gruppe.");
    let root = s.decision(
        "m10",
        "hain",
        "rootmother",
        "Die Wurzelmutter",
        (Sp::Sylaen, "Entscheide, Wächterin. Der Wald wartet nicht – und sie auch nicht.", "leise, drängend"),
        vec![
            opt("rootmother_burned", "Die grauen Wurzeln verbrennen", "Die Wurzelmutter stirbt; der Wald überlebt verwundet. Der Dornenbund wird Euch das nicht vergessen.", &[], burn),
            opt("rootmother_pact", "In ihren Traum eintreten", "Sie erwacht – vielleicht. Der Dornenbund steht dann an Eurer Seite.", &[], pact),
        ],
    );
    let win_burn = s.dlg("m10", "sieg_a", &[(Sp::Narrator, "Der Hain brannte drei Tage lang. Als die Asche kalt war, wuchs zwischen den Wurzeln ein einzelner heller Trieb – und niemand wagte es, ihn zu fällen.", "ruhig, bittersüß")]);
    let win_pact = s.dlg("m10", "sieg_b", &[
        (Sp::Narrator, "Isolde Raventhorn kehrte nach drei Atemzügen aus dem Traum zurück – und sprach in den folgenden Wochen nicht mehr als nötig. Die Wurzelmutter aber erwachte, und der Dornwald antwortete mit Blüten.", "leise, geheimnisvoll"),
    ]);
    s.note("m10", "sieg_a", "Schlusswort: Wurzelmutter verbrannt.");
    s.note("m10", "sieg_b", "Schlusswort: Traum betreten.");
    m.triggers = vec![
        trig("wald", Condition::AtTime(1.0), vec![intro, add_obj("obj_camps"), add_obj("obj_grove"), add_obj("obj_hero"), chron("Isolde Raventhorn folgte Sylaen in den Dornwald.")]),
        trig("lager1", tag_dead("wk1"), vec![c1]),
        trig("lager3", tag_dead("wk3"), vec![c3]),
        trig("lager_alle", all_of(vec![tag_dead("wk1"), tag_dead("wk2"), tag_dead("wk3")]), vec![done_obj("obj_camps")]),
        trig("hain", all_of(vec![in_area("hain", 2), tag_dead("wk3")]), vec![grove, root]),
        trig("sieg_burn", all_of(vec![flag("rootmother_burn"), tag_dead("boss")]), vec![done_obj("obj_grove"), done_obj("obj_hero"), win_burn, chron("Die Wurzelmutter starb im Feuer."), Action::Victory]),
        trig("sieg_dream", all_of(vec![flag("rootmother_dream"), Condition::EnemyUnitsAtMost(0)]), vec![done_obj("obj_grove"), done_obj("obj_hero"), win_pact, chron("Isolde Raventhorn trat in den Traum der Wurzelmutter."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel im Dornwald."),
    ];
    let mut v1 = variant(&["wendel_recruited"], &[], "Zwei Späher der Grauen Kompanie begleiten den Trupp als Fährtensucher.");
    v1.placements = vec![group("archer", 0, 7.5, 79.5, 3), group("scout_rider", 0, 6.0, 77.0, 2)];
    let mut v2 = variant(&["core_kept"], &[], "Das Tiefe wittert die Scherbe des Aschglases: Ein zusätzlicher Hinterhalt wartet vor dem zweiten Lager.");
    v2.placements = vec![guard("hollow_thrall", 1, 36.0, 50.0, 4), guard("hollow_archer", 1, 38.5, 52.0, 3)];
    let mut v3 = variant(&["nel_sheltered"], &[], "Nel hat einen Weg durch den Wald geträumt; der Hinterhalt vor dem zweiten Lager entfällt.");
    v3.remove_tags = vec!["ambush".into()];
    let mut v4 = variant(&["core_smashed"], &[], "Hal Venn schläft ruhig, seit der Kern zerschlagen ist: Seine Gebete tragen weiter, zwei zusätzliche Priester der Flamme begleiten den Trupp.");
    v4.placements = vec![group("priest", 0, 14.0, 80.5, 2)];
    m.variants = vec![v1, v2, v3, v4];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter III – Mission 11: Der Riss von Marrowfen (Rettung)
// ---------------------------------------------------------------------------------------------

fn m11(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m11_marrowfen_rift";
    s.title("m11", "Mission 11 – Der Riss von Marrowfen");
    let mut c = Canvas::new(88, 88, 90711, GenPreset::Lakes);
    // Das Moor von Marrowfen: viele Tümpel, ein fester Damm führt zum Ausgang.
    for (x, y, r) in [(20.0, 24.0, 5.0), (30.0, 48.0, 6.0), (60.0, 44.0, 6.0), (48.0, 74.0, 5.5), (72.0, 38.0, 4.5), (16.0, 60.0, 4.0)] {
        c.disc(x, y, r + 1.6, Terrain::Sand);
        c.disc(x, y, r, Terrain::Water);
    }
    let dam = [(8.0, 80.0), (16.0, 72.0), (28.0, 64.0), (40.0, 56.0), (44.0, 44.0), (52.0, 32.0), (62.0, 22.0), (74.0, 10.0)];
    c.path(&dam, 2.2, Terrain::Dirt);
    c.disc(44.0, 42.0, 10.0, Terrain::Ash);
    c.disc(44.0, 42.0, 6.0, Terrain::DarkGrass);
    for (x, y) in [(14.0, 40.0), (30.0, 74.0), (70.0, 58.0)] {
        c.flatten(x as i32 - 3, y as i32 - 3, x as i32 + 3, y as i32 + 3, Terrain::Dirt);
    }
    c.flatten(70, 4, 84, 16, Terrain::Stone);
    for i in 0..10 {
        let a = i as f32 / 10.0 * std::f32::consts::TAU;
        c.deco(DecorKind::Reeds, 44.0 + a.cos() * 8.0, 42.0 + a.sin() * 8.0);
        c.deco(DecorKind::Bones, 44.0 + a.cos() * 5.0, 42.0 + a.sin() * 5.0);
    }
    let mut m = campaign_mission(
        id,
        "Der Riss von Marrowfen",
        907,
        "Rettung",
        "Aus dem Moor von Marrowfen strömen Schleiergeister. Rettet die Dorfbewohner, führt sie über den Damm – und entscheidet, ob der Riss versiegelt oder geerntet wird.",
        "Marrowfen, Jahr 907 n.F. Im Moor ist ein Riss aufgegangen, schmal wie ein Fingernagel und tief wie die Nacht. Aus ihm quellen Schleiergeister und Fledermausschwärme, und in drei Weilern harren Dorfbewohner aus, die nicht fliehen konnten. Ihr führt Isolde und Hal Venn mit einem kleinen Trupp: Erreicht die Weiler, nehmt die Menschen mit und bringt sie über den Damm im Nordosten. Der Riss selbst ist reich an Aschglas – und die Stimme aus dem Tief macht ein Angebot. Entscheidet, ob Ihr den Riss schließt oder anzapft. Mindestens acht Menschen müssen den Damm erreichen.",
        "Der Damm von Marrowfen trug die Geretteten ins Trockene. Hinter ihnen schloss oder öffnete sich das Moor, je nachdem, wen man fragte.",
        "Marrowfen versank im Schleier. Wer den Damm nicht erreichte, kehrte nicht wieder.",
        &["mystery_veilscar", "explore_wilds"],
    );
    m.map_seed = 90711;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.52);
    m.placements = vec![
        hero_at(10.0, 78.5),
        tagged(pl("hero_hal_venn", 0, 12.0, 77.0), "hero_hal"),
        group("swordsman", 0, 8.0, 76.5, 4),
        group("halberdier", 0, 12.5, 80.5, 3),
        group("archer", 0, 9.0, 81.0, 4),
        group("priest", 0, 13.5, 78.5, 2),
        tagged(group("worker", 2, 14.0, 40.0, 5), "dorf1"),
        tagged(group("worker", 2, 30.0, 74.0, 5), "dorf2"),
        tagged(group("worker", 2, 70.0, 58.0, 5), "dorf3"),
        guard("hollow_thrall", 1, 16.0, 38.0, 3),
        guard("ghoul", 1, 28.5, 72.0, 3),
        guard("hollow_thrall", 1, 68.0, 56.0, 4),
        tagged(guard("ghoul", 1, 36.5, 52.0, 3), "wk_ambush"),
        tagged(guard("ash_witch", 1, 46.0, 46.0, 2), "riss_wache"),
        guard("ghoul", 1, 48.0, 40.0, 3),
        guard("hollow_thrall", 1, 41.0, 40.0, 4),
        tagged(pl("hollow_shrine", 1, 43.0, 41.0), "rift"),
        guard("hollow_archer", 1, 56.0, 28.0, 3),
    ];
    m.areas = vec![
        area("dorf1", "Weiler am Schilf", 10, 36, 9, 9),
        area("dorf2", "Weiler an den Birken", 26, 70, 9, 9),
        area("dorf3", "Weiler am Hang", 66, 54, 9, 9),
        area("riss", "Der Riss von Marrowfen", 38, 36, 12, 12),
        area("damm", "Der Damm", 66, 4, 20, 16),
        area("schilf", "Das Schilf am Riss", 40, 32, 8, 4),
    ];
    m.starts = vec![StartPos { owner: 0, x: 10.0, y: 78.0 }, StartPos { owner: 1, x: 44.0, y: 42.0 }];
    m.objectives = vec![
        objective("obj_rescue", "Rettet mindestens acht Dorfbewohner über den Damm im Nordosten.", false),
        objective("obj_rift", "Entscheidet über den Riss von Marrowfen.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m11", "moor", &[
        (Sp::Narrator, "In Marrowfen riecht es nach nasser Asche und nach Dingen, die zu lange unter Wasser geschlafen haben. Wo der Riss aufgegangen war, wuchs kein Schilf mehr – nur ein weißer Ring aus Knochenstaub.", "leise, unheimlich"),
        (Sp::Hal, "Drei Weiler, Isolde. Wenn wir schnell sind, schaffen wir alle drei vor der Dämmerung. Und wenn wir langsam sind … schaffen wir es gar nicht.", "angespannt, nüchtern"),
    ]);
    s.note("m11", "moor", "Der Südrand des Moores von Marrowfen.");
    let joined = s.dlg("m11", "dorf", &[(Sp::Hal, "Kommt, kommt – langsam, haltet euch an den Händen. Wir bringen euch über den Damm, im Namen der Flamme!", "warm, rufend")]);
    s.note("m11", "dorf", "Ein Weiler schließt sich dem Trupp an.");
    let rift_scene = s.dlg("m11", "riss", &[
        (Sp::Deep, "Du bist weit gekommen, kleine Flamme. Sieh, was ich dir zeige: Aschglas, so rein, dass es singt. Nimm davon, und deine Heere werden nie wieder hungern.", "schmeichelnd, tief, mehrschichtig"),
        (Sp::Hal, "Hör nicht hin, Isolde. Was dort flüstert, hat Harrowmere verbrannt.", "scharf, besorgt"),
    ]);
    s.note("m11", "riss", "Isolde erreicht den Riss; die Stimme aus dem Tief macht ihr Angebot.");
    let sealed = vec![s.dlg("m11", "riss_a", &[
        (Sp::Isolde, "Du bietest mir Brot, aus dem Mund eines Hungernden. Nein. Hal, schließ ihn.", "kalt, entschlossen"),
        (Sp::Deep, "Wie schade. Ich hatte mich gefreut, dich zu verwöhnen.", "leicht gekränkt, spöttisch"),
    ])];
    let harvest = vec![
        s.dlg("m11", "riss_b", &[
            (Sp::Isolde, "Nur ein wenig. Nur so viel, wie die Aschmark in zehn Wintern verbraucht.", "leise, mit schlechtem Gewissen"),
            (Sp::Deep, "Ein wenig. Natürlich. Ich bin geduldig, Kind. Ich war es immer.", "zufrieden, samtig"),
        ]),
        Action::Grant(Cost { gold: 600.0, wood: 300.0, stone: 300.0, food: 300.0 }),
    ];
    s.note("m11", "riss_a", "Entscheidung: Riss versiegeln.");
    s.note("m11", "riss_b", "Entscheidung: Riss anzapfen und Aschglas ernten.");
    let rift = s.decision(
        "m11",
        "riss",
        "rift",
        "Der Riss von Marrowfen",
        (Sp::Hal, "Der Riss kann geschlossen werden, wenn wir den Schrein an seinem Grund zerschlagen. Oder wir nehmen, was er uns reicht – und tragen die Folgen.", "ernst, beschwörend"),
        vec![
            opt("rift_sealed", "Den Riss versiegeln", "Der Schrein muss zerstört werden; die Hohlung bleibt fern.", &[], sealed),
            opt("rift_harvested", "Das Aschglas ernten", "Reiche Beute – aber das Tiefe bleibt in Reichweite und die Hohlung wächst.", &[], harvest),
        ],
    );
    let nel = s.dlg("m11", "nel", &[(Sp::Nel, "Hört ihr es? Es klingt wie eine Spieluhr, die jemand zu langsam aufzieht. Dort entlang geht es – das Schilf weiß den Weg.", "kindlich, abwesend")]);
    s.note("m11", "nel", "Nel, die in Brennhollow aufgenommene Waise, weist den Weg (nur wenn sie bei Hal geblieben ist).");
    let win = s.dlg("m11", "sieg", &[(Sp::Narrator, "Der Damm trug die Geretteten ins Trockene. Hinter ihnen schloss sich das Moor – oder öffnete sich, je nachdem, wen man später fragte.", "ruhig, vieldeutig")]);
    s.note("m11", "sieg", "Schlusswort.");
    let spawn_rift = |id: &str, t: f32, units: &[(&str, u32)]| trig(id, all_of(vec![Condition::AtTime(t), not(flag("rift_harvested")), not(tag_dead("rift"))]), wave("riss", units));
    m.triggers = vec![
        trig("moor", Condition::AtTime(1.0), vec![intro, add_obj("obj_rescue"), add_obj("obj_rift"), add_obj("obj_hero"), chron("Isolde Raventhorn zog nach Marrowfen, um den Riss zu schließen.")]),
        trig("dorf1", in_area("dorf1", 1), vec![Action::ChangeOwner { tag: "dorf1".into(), owner: 0 }, joined.clone()]),
        trig("dorf2", in_area("dorf2", 1), vec![Action::ChangeOwner { tag: "dorf2".into(), owner: 0 }]),
        trig("dorf3", in_area("dorf3", 1), vec![Action::ChangeOwner { tag: "dorf3".into(), owner: 0 }]),
        spawn_rift("riss1", 70.0, &[("bat_swarm", 3), ("hollow_thrall", 4)]),
        spawn_rift("riss2", 130.0, &[("ghoul", 3), ("hollow_archer", 3)]),
        spawn_rift("riss3", 200.0, &[("bat_swarm", 4), ("ghoul", 3)]),
        spawn_rift("riss4", 280.0, &[("ash_witch", 1), ("hollow_thrall", 6), ("ghoul", 3)]),
        trig("riss", Condition::TagDiscovered("rift".into()), vec![rift_scene, rift]),
        trig("nel_weg", all_of(vec![flag("nel_sheltered"), Condition::AtTime(8.0)]), vec![nel]),
        trig("sieg_sealed", all_of(vec![flag("rift_sealed"), tag_dead("rift"), unit_in("worker", "damm", 8)]), vec![done_obj("obj_rescue"), done_obj("obj_rift"), done_obj("obj_hero"), win.clone(), chron("Der Riss von Marrowfen wurde versiegelt."), Action::Victory]),
        trig("sieg_harvest", all_of(vec![flag("rift_harvested"), unit_in("worker", "damm", 8)]), vec![done_obj("obj_rescue"), done_obj("obj_rift"), done_obj("obj_hero"), win, chron("Aschglas aus Marrowfen füllte die Wagen der Aschmark."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel im Moor von Marrowfen."),
    ];
    let mut v1 = variant(&["nel_sheltered"], &[], "Nel hört den Riss und führt den Trupp am Hinterhalt im Schilf vorbei.");
    v1.remove_tags = vec!["wk_ambush".into()];
    let mut v2 = variant(&["rootmother_pact"], &[], "Der Dornenbund schickt Waldläufer, die das Moor kennen.");
    v2.placements = vec![group("archer", 0, 7.0, 79.0, 4)];
    let mut v3 = variant(&["rootmother_burned"], &[], "Der Dornenbund wendet sich ab; zusätzliche Hohle streunen am Weg.");
    v3.placements = vec![guard("hollow_thrall", 1, 22.0, 66.0, 4)];
    let mut v4 = variant(&["nel_sent"], &[], "Ohne Nel, die den Riss hören kann, droht ein zusätzlicher Hinterhalt am Weg zum Weiler an den Birken.");
    v4.placements = vec![guard("ghoul", 1, 32.0, 62.0, 3), guard("hollow_thrall", 1, 24.0, 68.0, 3)];
    m.variants = vec![v1, v2, v3, v4];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter IV – Mission 13: Die Aschglasfelder (Eroberung)
// ---------------------------------------------------------------------------------------------

fn m13(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m13_ashglass_fields";
    s.title("m13", "Mission 13 – Die Aschglasfelder");
    let mut c = Canvas::new(96, 80, 91013, GenPreset::Ashlands);
    c.flatten(0, 0, 95, 79, Terrain::Ash);
    c.flatten(4, 56, 28, 76, Terrain::Sand);
    for (x, y) in [(30, 54), (62, 36), (80, 14)] {
        c.flatten(x - 8, y - 7, x + 8, y + 7, Terrain::Stone);
    }
    c.path(&[(16.0, 66.0), (30.0, 56.0), (46.0, 46.0), (62.0, 36.0), (72.0, 26.0), (80.0, 14.0)], 1.4, Terrain::Road);
    for i in 0..26 {
        let t = i as f32;
        let (x, y) = (10.0 + (t * 37.0) % 80.0, 6.0 + (t * 23.0) % 60.0);
        c.deco(DecorKind::Rock, x, y);
    }
    for (x, y) in [(30.5, 46.5), (62.5, 28.5), (80.5, 8.5), (24.5, 60.5)] {
        c.deco(DecorKind::Pillar, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Die Aschglasfelder",
        910,
        "Eroberung",
        "Am Rand der Aurethwüste hat die Aschenlegion drei Außenposten errichtet. Erobert sie, ehe das Aschglas in ihre Hände fällt.",
        "Aurethwüste, Jahr 910 n.F. Zum ersten Mal trägt der Feind die Wappen Aureths: schwarzer Harnisch, glimmende Fugen, Soldaten, die nur das Feuer endgültig tötet. Die Aschenlegion hat am Rand der Aurethwüste drei Außenposten besetzt, um die Aschglasfelder zu sichern. Ihr führt die Duchess-Garde der Aschmark: Hellebardiere, Armbrustschützen, Ritter und Priester. Fällt ein Posten, ist der nächste nur noch eine Marschstunde entfernt. Der Ascheregen frisst die Sicht – aber nicht den Willen.",
        "Die drei Posten fielen. Auf den Aschglasfeldern wehte nun das Banner der Aschmark – und in den Fugen der erschlagenen Legionäre glomm es noch lange rot.",
        "Die Legion hielt ihre Posten. Das Aschglas blieb in ihren Händen, und die Wüste behielt die Besiegten.",
        &["faction_ashen_legion_theme", "siege_assault"],
    );
    m.map_seed = 91013;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.46);
    let mut p = vec![
        hero_at(16.0, 66.0),
        group("swordsman", 0, 12.0, 66.0, 8),
        group("halberdier", 0, 18.0, 70.0, 6),
        group("crossbowman", 0, 14.0, 62.0, 6),
        group("knight", 0, 20.0, 64.0, 4),
        group("priest", 0, 10.0, 70.0, 2),
        group("catapult", 0, 8.0, 64.0, 1),
    ];
    for (n, (x, y)) in [(30.0f32, 54.0f32), (62.0, 36.0), (80.0, 14.0)].into_iter().enumerate() {
        let tag = format!("op{}", n + 1);
        p.push(tagged(pl("keep", 1, x, y), &tag));
        p.push(pl("watchtower", 1, x - 5.0, y + 3.0));
        p.push(pl("watchtower", 1, x + 5.0, y + 3.0));
        p.push(guard("ashen_legionnaire", 1, x - 1.0, y + 5.0, 4 + n as u32));
        p.push(guard("hollow_archer", 1, x + 2.0, y - 2.0, 3));
        p.push(guard("ash_witch", 1, x, y - 4.0, 1 + (n as u32).min(1)));
    }
    m.placements = p;
    m.areas = vec![
        area("lager", "Lager der Aschmark", 4, 56, 24, 20),
        area("posten1", "Erster Posten", 22, 46, 18, 18),
        area("posten2", "Zweiter Posten", 54, 28, 18, 18),
        area("posten3", "Dritter Posten", 72, 6, 20, 18),
    ];
    m.starts = vec![StartPos { owner: 0, x: 16.0, y: 66.0 }, StartPos { owner: 1, x: 62.0, y: 36.0 }];
    m.objectives = vec![
        objective("obj_op1", "Erobert den ersten Außenposten der Legion.", false),
        objective("obj_op2", "Erobert den zweiten Außenposten.", false),
        objective("obj_op3", "Erobert den dritten Außenposten.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m13", "wueste", &[
        (Sp::Narrator, "Es war ein Land, das einmal Reich geheißen hatte. Nun lag es als flache Asche bis zum Horizont, und in der Asche funkelten Splitter – Aschglas, wo der Schleier dünn war, und Wachfeuer, wo die Legion stand.", "weit, ehrfürchtig, leise"),
        (Sp::Isolde, "Schwarze Rüstungen, Glut in den Fugen, und das Wappen Aureths auf der Brust. Das sind keine Wiedergänger, das ist eine Armee. Zeigt ihnen Stahl – und Feuer.", "nüchtern, kalt"),
    ]);
    s.note("m13", "wueste", "Das Lager am Rand der Aurethwüste.");
    let wendel = s.dlg("m13", "kompanie", &[(Sp::Wendel, "Die Graue Kompanie kennt Wüstenräuber, Wächterin – Tote in Rüstung sind neu. Aber Tote bluten wenigstens nicht auf den Sold.", "trocken, kampfeslustig")]);
    s.note("m13", "kompanie", "Wendel, wenn er in Isoldes Dienst getreten ist.");
    let fin = s.dlg("m13", "sieg", &[(Sp::Narrator, "Aus den Fugen der erschlagenen Legionäre stieg noch lange rotes Licht. Wer es sah, sprach später nicht davon.", "leise, bedrückt")]);
    s.note("m13", "sieg", "Schlusswort.");
    m.triggers = vec![
        trig("wueste", Condition::AtTime(1.0), vec![intro, add_obj("obj_op1"), add_obj("obj_op2"), add_obj("obj_op3"), add_obj("obj_hero"), Action::SetWeather(ashen_sim::env::Weather::Ashfall), chron("Die Aschmark marschierte gegen die Außenposten der Aschenlegion.")]),
        trig("kompanie", all_of(vec![flag("wendel_recruited"), Condition::AtTime(6.0)]), vec![wendel]),
        trig("op1", tag_dead("op1"), vec![done_obj("obj_op1")]),
        trig("op2", tag_dead("op2"), vec![done_obj("obj_op2")]),
        trig("op3", tag_dead("op3"), vec![done_obj("obj_op3")]),
        trig("sieg", all_of(vec![tag_dead("op1"), tag_dead("op2"), tag_dead("op3")]), vec![done_obj("obj_hero"), fin, chron("Die Aschglasfelder am Rand der Aurethwüste fielen an die Aschmark."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel in den Aschglasfeldern."),
    ];
    let mut v1 = variant(&["wendel_recruited"], &[], "Die Graue Kompanie unter Ser Wendel ergänzt das Heer.");
    v1.placements = vec![group("swordsman", 0, 10.0, 60.0, 8), group("archer", 0, 8.0, 68.0, 6)];
    let mut v2 = variant(&["core_kept"], &[], "Kampfmagier der Flamme schleudern Feuer auf die Legion.");
    v2.placements = vec![group("battle_mage", 0, 14.0, 72.0, 3)];
    let mut v3 = variant(&["oath_corvane"], &[], "Mireilles Nordmarker Speerträger schützen die Flanke.");
    v3.placements = vec![group("spearman", 0, 22.0, 60.0, 8)];
    let mut v4 = variant(&["oath_crown"], &[], "Die Krone entsendet ihre Ritter und ein zweites Katapult.");
    v4.placements = vec![group("knight", 0, 22.0, 68.0, 3), group("catapult", 0, 8.0, 68.0, 1)];
    let mut v5 = variant(&["torvald_blood"], &[], "Torvald Eisenhand und sein Clan haben den Schwur gehalten und marschieren mit.");
    v5.placements = vec![tagged(pl("hero_torvald", 0, 18.0, 60.0), "torvald"), group("shieldbearer", 0, 20.0, 60.0, 4)];
    m.variants = vec![v1, v2, v3, v4, v5];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter IV – Mission 14: Das Schwarze Riff (Bossjagd an der Saelith-Küste)
// ---------------------------------------------------------------------------------------------

fn m14(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m14_black_reef";
    s.title("m14", "Mission 14 – Das Schwarze Riff");
    let mut c = Canvas::new(88, 72, 91214, GenPreset::Heartland);
    // Die Küste: offenes Meer im Osten und Süden, ein Strand, eine Insel mit Riffstufe, ein Damm bei Ebbe.
    c.flatten(0, 0, 87, 71, Terrain::Sand);
    c.flatten(0, 0, 40, 71, Terrain::Grass);
    c.flatten(26, 20, 44, 60, Terrain::Sand);
    c.disc(78.0, 44.0, 34.0, Terrain::DeepWater);
    c.disc(78.0, 44.0, 28.0, Terrain::Water);
    c.disc(62.0, 28.0, 13.0, Terrain::Sand);
    c.disc(62.0, 28.0, 9.0, Terrain::Stone);
    c.path(&[(40.0, 38.0), (50.0, 34.0), (56.0, 30.0)], 2.4, Terrain::Ford);
    c.path(&[(6.0, 40.0), (20.0, 40.0), (36.0, 38.0)], 1.4, Terrain::Road);
    for (x, y) in [(16.5, 52.5), (28.5, 58.5), (36.5, 30.5), (32.5, 14.5)] {
        c.deco(DecorKind::Ruin, x, y);
    }
    for (x, y) in [(18.5, 30.5), (24.5, 46.5), (38.5, 50.5)] {
        c.deco(DecorKind::Bones, x, y);
    }
    for i in 0..6 {
        c.deco(DecorKind::Pillar, 54.0 + i as f32 * 2.6, 20.5);
    }
    let mut m = campaign_mission(
        id,
        "Das Schwarze Riff",
        912,
        "Bossjagd an der Küste",
        "Vor der Saelith-Küste hat sich Isarre Mol, Herrin des Schwarzen Riffs, dem Hohlen Tief verschrieben. Stellt sie auf ihrer Riffinsel – mit oder ohne Ophiras Flotte.",
        "Saelith-Küste, Jahr 912 n.F. Das Schwarze Riff war einmal ein Gezeitenheiligtum. Dann kam Isarre Mol, Priesterin des Hauses der Gezeiten, die dem Hohlen Tief die Flut verpfändete: Seither laufen die Wellen bergauf, und die Ertrunkenen steigen nachts an Land. Ophira Saelith-Varo bietet Isolde Raventhorn Schiffe, Söldner und Katapulte – gegen einen Anteil am Aschglas der Aurethküste. Wer ablehnt, marschiert allein. Das Riff liegt auf einer Insel; ein Damm aus Furt und Stein führt bei Ebbe hinüber. Isarre beschwört Fluten, entzieht Leben und ruft Ertrunkene.",
        "Isarre Mol sank mit dem Rest ihrer Flut. Über dem Schwarzen Riff stieg das Meer langsam, aber freiwillig wieder an die Küste.",
        "Das Schwarze Riff behielt seine Herrin. Die Flut stieg über die Strände, und die Küste verstummte.",
        &["boss_reef_mistress", "battle_large"],
    );
    m.map_seed = 91214;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.62);
    m.placements = vec![
        hero_at(8.0, 40.5),
        group("swordsman", 0, 6.0, 38.0, 8),
        group("halberdier", 0, 6.0, 43.0, 6),
        group("crossbowman", 0, 4.0, 40.0, 8),
        group("knight", 0, 10.0, 36.0, 4),
        group("priest", 0, 4.0, 44.0, 2),
        // Küstenwachen der Hohlen.
        guard("hollow_thrall", 1, 20.0, 34.0, 5),
        guard("hollow_archer", 1, 22.5, 36.5, 3),
        guard("ghoul", 1, 26.0, 46.0, 4),
        guard("hollow_thrall", 1, 34.0, 38.5, 6),
        guard("ash_witch", 1, 36.0, 36.0, 2),
        // Die Riffinsel.
        tagged(pl("hollow_shrine", 1, 60.0, 24.0), "riff_schrein"),
        tagged(guard("boss_reef_mistress", 1, 62.0, 28.0, 1), "boss"),
        guard("hollow_thrall", 1, 58.0, 30.0, 6),
        guard("ghoul", 1, 66.0, 30.0, 4),
        guard("hollow_archer", 1, 60.0, 33.0, 4),
    ];
    m.areas = vec![
        area("strand", "Der Strand", 2, 32, 14, 16),
        area("damm", "Der Riffdamm", 40, 30, 14, 12),
        area("insel", "Die Riffinsel", 54, 18, 18, 20),
        area("kai", "Der Kai", 2, 34, 4, 10),
    ];
    m.starts = vec![StartPos { owner: 0, x: 8.0, y: 40.0 }, StartPos { owner: 1, x: 62.0, y: 28.0 }];
    m.objectives = vec![
        objective("obj_dam", "Überquert den Riffdamm.", false),
        objective("obj_boss", "Bezwingt Isarre Mol, Herrin des Schwarzen Riffs.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m14", "tintenfass", &[
        (Sp::Ophira, "Willkommen am Kai, Wächterin. Vorsicht mit den Tauen, sie kosten mehr als Eure Stiefel.", "amüsiert, Hafenakustik"),
        (Sp::Isolde, "Alles hier kostet mehr als meine Stiefel. Das macht mir Sorgen.", "trocken"),
        (Sp::Ophira, "Vierzig Schiffe, dreitausend Söldner und das Riff in neun Tagen statt in neunzig. Der Preis ist ein Anteil an jedem Splitter Aschglas, den Ihr aus den Ruinen holt.", "lachend, geschäftsmäßig"),
        (Sp::Kassian, "Lest das Kleingedruckte, Wächterin. In Saelith bewahrt man im Kleingedruckten die Messer auf.", "trocken, flüsternd"),
    ]);
    s.note("m14", "tintenfass", "Der Kai von Tarn Saelith; Ophira legt den Vertrag vor. Kassian beobachtet.");
    let pact = vec![
        s.dlg("m14", "flotte_a", &[
            (Sp::Isolde, "Gebt mir die Feder. Jeder Marschtag ist ein weiteres verlorenes Dorf.", "entschlossen, Bitterkeit"),
            (Sp::Ophira, "Großartig. Die Flut kommt immer zurück, um zu holen, was ihr geschuldet wird, Wächterin. Immer.", "warm, mit schneidendem Unterton"),
        ]),
        sp_units("crossbowman", 0, "kai", 8, false, None),
        sp_units("halberdier", 0, "kai", 4, false, None),
        sp_units("catapult", 0, "kai", 2, false, None),
    ];
    let alone = vec![s.dlg("m14", "flotte_b", &[
        (Sp::Isolde, "Wir laufen. Ich schulde lieber meinen Blasen etwas als Euch.", "trocken, stolz"),
        (Sp::Ophira, "Stolz. Das Teuerste, was ein Herrscher kaufen kann. Gute Winde, Wächterin.", "ehrlich amüsiert"),
    ])];
    s.note("m14", "flotte_a", "Entscheidung: Die Flotte annehmen.");
    s.note("m14", "flotte_b", "Entscheidung: Ohne Flotte marschieren.");
    let fleet = s.decision(
        "m14",
        "tintenfass",
        "fleet",
        "Ophiras Vertrag",
        (Sp::Ophira, "Die Flut wartet auf niemanden, Wächterin. Unterschreibt oder lauft.", "plötzlich kühl"),
        vec![
            opt("fleet_pact", "Die Flotte annehmen", "Saelith-Söldner und Katapulte verstärken Euch – und Ihr schuldet Ophira Aschglas.", &[], pact),
            opt("fleet_none", "Ohne Flotte marschieren", "Keine Schuld, aber weniger Truppen.", &[], alone),
        ],
    );
    let boss = s.dlg("m14", "isarre", &[
        (Sp::Isarre, "Ihr kommt auf trockenen Füßen zu meiner Tür. Wie rührend. Das Meer hat mir jedes meiner Kinder zurückgegeben – ich bin nur gekommen, es zu tränken.", "tief, nass, gurgelnd, mehrstimmig"),
        (Sp::Isolde, "Dann bin ich hier, um dich zu erlösen. Eine Flut kann man dämmen.", "kalt, ruhig"),
    ]);
    s.note("m14", "isarre", "Isarre Mol, auf der Riffinsel.");
    let fin = s.dlg("m14", "sieg", &[(Sp::Narrator, "Das Wasser zog sich zurück, als hätte es nie anderes vorgehabt. Auf der Riffinsel blieben Muscheln, Knochen und eine Perle von der Größe einer Faust, die später in Brennhollow in eine Rüstung eingelegt wurde.", "ruhig, bildhaft")]);
    s.note("m14", "sieg", "Schlusswort.");
    m.triggers = vec![
        trig("kai", Condition::AtTime(1.0), vec![intro, fleet, chron("Ophira Saelith-Varo bot Isolde Raventhorn die Flotte des Saelith-Paktes an.")]),
        trig("start", any_flag(&["fleet_pact", "fleet_none"]), vec![add_obj("obj_dam"), add_obj("obj_boss"), add_obj("obj_hero"), chron("Der Marsch zum Schwarzen Riff begann.")]),
        trig("damm", in_area("damm", 3), vec![done_obj("obj_dam")]),
        trig("boss_begegnung", in_area("insel", 1), vec![boss]),
        trig("sieg", tag_dead("boss"), vec![done_obj("obj_boss"), done_obj("obj_hero"), fin, chron("Isarre Mol, Herrin des Schwarzen Riffs, fiel."), Action::Victory]),
        hero_lost("Isolde Raventhorn versank im Schwarzen Riff."),
    ];
    let mut v1 = variant(&["rift_harvested"], &[], "Die Hohlung hat sich verbreitet: Das Riff wird von zusätzlichen Ertrunkenen und Aschhexen bewacht.");
    v1.placements = vec![guard("ghoul", 1, 56.0, 26.0, 4), guard("ash_witch", 1, 64.0, 24.0, 2)];
    let mut v2 = variant(&["rift_sealed"], &[], "Der versiegelte Riss von Marrowfen hat dem Heer Geweihte der Flamme beschert.");
    v2.placements = vec![group("priest", 0, 4.5, 46.5, 2)];
    let mut v3 = variant(&["torvald_blood"], &[], "Torvald Eisenhand und seine Clankrieger sind über das Meer gekommen.");
    v3.placements = vec![tagged(pl("hero_torvald", 0, 6.5, 41.5), "torvald"), group("shieldbearer", 0, 7.0, 45.0, 4)];
    let mut v4 = variant(&["wendel_recruited"], &[], "Die Graue Kompanie ergänzt das Heer um erfahrene Schützen.");
    v4.placements = vec![group("archer", 0, 4.0, 36.0, 6)];
    let mut v5 = variant(&["fleet_pact"], &[], "Der Vertrag liegt: Ophiras Schiffe bringen den Nachschub, aber die Kapitäne verlangen schon den ersten Anteil. Ihre Schiffsmannschaften verstärken das Ufer.");
    v5.placements = vec![group("halberdier", 0, 3.5, 47.0, 2)];
    let mut v6 = variant(&["fleet_none"], &[], "Ohne Flotte nimmt der Vormarsch den Landweg: Die Küstenwachen sind besser vorbereitet, weil kein Überraschungsmoment bleibt.");
    v6.placements = vec![guard("hollow_thrall", 1, 28.0, 40.0, 4), guard("hollow_archer", 1, 30.0, 42.0, 2)];
    m.variants = vec![v1, v2, v3, v4, v5, v6];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}
