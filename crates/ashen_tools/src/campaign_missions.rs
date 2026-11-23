//! Missionen der Zeitalter I und II (Brennhollow und der Aufstieg der Krone).

use crate::campaign_forge::*;
use crate::forge::{area, group, guard, pl, tagged, trig, wall_ring, wall_ring_gaps, Canvas};
use ashen_sim::data::{Cost, GameData};
use ashen_sim::entity::NodeKind;
use ashen_sim::genmap::GenPreset;
use ashen_sim::map::{DecorKind, Terrain};
use ashen_sim::mission::{Action, Condition, MissionDef};
use ashen_sim::scenario::StartPos;

pub(crate) use crate::campaign_missions_c::plan;

pub(crate) fn all(data: &GameData, s: &mut Script) -> Vec<MissionDef> {
    let mut v = vec![m02(data, s), m03(data, s), m04(data, s), m06(data, s), m07(data, s)];
    v.extend(crate::campaign_missions_b::all(data, s));
    v.extend(crate::campaign_missions_c::all(data, s));
    v
}

fn hero_at(x: f32, y: f32) -> ashen_sim::mission::Placement {
    tagged(pl("hero_isolde", 0, x, y), "hero")
}

// ---------------------------------------------------------------------------------------------
// Zeitalter I – Mission 2: Die Flüchtlinge von Harrowmere (Eskorte)
// ---------------------------------------------------------------------------------------------

fn m02(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m02_harrowmere";
    s.title("m02", "Mission 2 – Die Flüchtlinge von Harrowmere");
    let mut c = Canvas::new(96, 72, 89702, GenPreset::Ashlands);
    // Das Aschmoor: zwei Teiche, ein Fluss mit Furt, die alte Straße schlängelt sich nach Nordosten.
    c.disc(38.0, 24.0, 6.5, Terrain::Sand);
    c.disc(38.0, 24.0, 4.8, Terrain::Water);
    c.disc(62.0, 58.0, 7.5, Terrain::Sand);
    c.disc(62.0, 58.0, 5.6, Terrain::Water);
    let river = [(46.0, 0.0), (52.0, 28.0), (48.0, 46.0), (56.0, 72.0)];
    c.path(&river, 3.4, Terrain::Sand);
    c.path(&river, 2.2, Terrain::Water);
    c.path(&river, 1.2, Terrain::DeepWater);
    c.disc(48.0, 46.0, 3.4, Terrain::Ford);
    let road = [(10.0, 56.0), (22.0, 53.0), (34.0, 48.0), (48.0, 46.0), (58.0, 38.0), (66.0, 30.0), (76.0, 22.0), (86.0, 14.0)];
    c.path(&road, 1.1, Terrain::Road);
    c.flatten(3, 44, 21, 66, Terrain::Dirt);
    c.flatten(78, 5, 94, 24, Terrain::Stone);
    for (x, y) in [(6.5, 48.5), (10.5, 62.5), (16.5, 50.5), (18.5, 60.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    for (x, y) in [(8.5, 52.5), (14.5, 56.5), (5.5, 60.5), (19.5, 46.5), (12.5, 46.5)] {
        c.deco(DecorKind::Ruin, x, y);
    }
    for (x, y) in [(30.5, 50.5), (40.5, 44.5), (60.5, 40.5), (70.5, 26.5)] {
        c.deco(DecorKind::Bones, x, y);
    }
    for (x, y) in [(84.5, 8.5), (90.5, 8.5), (84.5, 20.5), (90.5, 20.5)] {
        c.deco(DecorKind::Lantern, x, y);
    }
    c.forest(60, 44, 72, 52, 3);
    c.forest(26, 30, 36, 40, 3);

    let mut m = campaign_mission(
        id,
        "Die Flüchtlinge von Harrowmere",
        897,
        "Eskorte",
        "Harrowmere im Aschmoor brennt. Führt die Überlebenden über Furt und Moorstraße nach Brennhollow – keine Wagen zurücklassen, kein Kind.",
        "Aschmoor, Jahr 897 n.F. Wenige Tage nach dem Fall des Hohlschreins von Vael Dorn kommt ein Reiter aus dem Süden: Harrowmere, das Dorf im Moor, wird von den Hohlen überrannt. Wer noch gehen kann, hat sich am Brunnen gesammelt. Ihr führt sechzehn Überlebende und eine kleine Eskorte unter Isolde Raventhorn. Die alte Straße windet sich nordostwärts durch das Moor, über eine Furt und an drei Wachlagern der Hohlen vorbei. Bringt mindestens elf Menschen nach Brennhollow. Ascheregen verschluckt Geräusche – die Toten kommen oft zuerst als Schatten. Nach der Ankunft steht noch eine Entscheidung an, die Brennhollow prägen wird.",
        "Die Fackeln von Brennhollow nahmen die Erschöpften auf. Harrowmere aber blieb im Moor zurück – ein Name mehr in der Liste der Verlorenen.",
        "Das Moor schloss sich über dem Zug der Flüchtlinge. Niemand aus Harrowmere erreichte Brennhollow.",
        &["escort_march", "build_tension"],
    );
    m.map_seed = 89702;
    m.start_resources = Cost { gold: 0.0, wood: 0.0, stone: 0.0, food: 200.0 };
    m.defeat_tag = String::new();
    m.start_time = Some(0.34);
    m.placements = vec![
        group("worker", 0, 9.0, 56.0, 15),
        tagged(pl("worker", 0, 11.0, 57.0), "nel"),
        group("swordsman", 0, 13.0, 55.0, 4),
        group("spearman", 0, 12.0, 58.5, 2),
        group("archer", 0, 9.5, 59.5, 3),
        hero_at(14.5, 56.5),
        // Wachlager der Hohlen entlang der Moorstraße.
        guard("hollow_thrall", 1, 29.0, 49.5, 4),
        guard("hollow_archer", 1, 31.5, 51.5, 2),
        guard("hollow_thrall", 1, 58.5, 38.5, 5),
        guard("hollow_archer", 1, 56.0, 41.0, 2),
        guard("grave_brute", 1, 61.0, 36.0, 1),
        guard("hollow_thrall", 1, 72.0, 24.5, 4),
        guard("hollow_archer", 1, 74.5, 22.0, 2),
    ];
    m.areas = vec![
        area("dorf", "Harrowmere", 3, 44, 18, 22),
        area("brennhollow", "Das Tor von Brennhollow", 78, 5, 16, 20),
        area("furt", "Die Furt", 44, 43, 8, 7),
        area("hinterhalt", "Hinterhalt im Moor", 36, 44, 7, 7),
        area("flanke", "Flanke im Röhricht", 28, 36, 5, 5),
    ];
    m.starts = vec![StartPos { owner: 0, x: 12.0, y: 56.0 }, StartPos { owner: 1, x: 30.0, y: 50.0 }];
    m.objectives = vec![
        objective("obj_escort", "Führt mindestens elf Überlebende von Harrowmere nach Brennhollow.", false),
        objective("obj_ford", "Überquert die Furt im Aschmoor.", true),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let core_scene = s.dlg(
        "m02",
        "kern",
        &[
            (Sp::Hal, "Das Feuer ist aus, aber die Steine sind noch warm. Das ist nicht natürlich, Isolde.", "beunruhigt, leise"),
            (Sp::Isolde, "Nichts an diesem Ort ist natürlich. Sieh dir den Kristall an. Er … atmet.", "angespannt, misstrauisch"),
            (Sp::Deep, "Wächterin … du hast meine kleine Glocke zerbrochen. Wie unhöflich.", "zärtlich-spöttisch, mehrschichtiger Effekt (Profil Deep Voice)"),
            (Sp::Isolde, "Hal. Sag mir, dass du das gehört hast.", "kaum hörbar, kalter Schreck"),
            (Sp::Hal, "Was gehört? Isolde, deine Hand zittert. Geh weg davon.", "besorgt, väterlich"),
            (Sp::Deep, "Er kann mich nicht hören. Nur wer im Fieber gebrannt hat, kann es. Erinnerst du dich an das Fieber, Kind? Ich erinnere mich an dich.", "leise, vertraut, bedrohlich"),
        ],
    );
    s.note("m02", "kern", "Nacht nach der Zerstörung des Hohlschreins. Isolde und Hal Venn stehen in den Trümmern; im Zentrum pulsiert ein Aschglas-Kern. Die Stimme aus dem Tief spricht zum ersten Mal – nur Isolde hört sie.");
    s.note("m02", "kern_a", "Reaktion auf die Entscheidung: Kern zerschlagen.");
    s.note("m02", "kern_b", "Reaktion auf die Entscheidung: Kern bergen.");
    let smash = vec![s.dlg("m02", "kern_a", &[
        (Sp::Isolde, "Nein. Was immer darin ist, es kennt meinen Namen. Zerschlagt es.", "hart, entschlossen"),
        (Sp::Deep, "Zerbrich das Glas, kleine Flamme. Unter dir ist so viel mehr von mir.", "leises, hohles Lachen"),
        (Sp::Hal, "Gut. Manches Wissen bleibt besser in der Asche.", "erleichtert"),
    ])];
    let keep = vec![s.dlg("m02", "kern_b", &[
        (Sp::Isolde, "Wir nehmen es mit. Wenn wir nicht verstehen, wogegen wir kämpfen, haben wir schon verloren.", "ruhig, entschieden"),
        (Sp::Deep, "Ja … trag mich heim. Halte mich nah an deinem Feuer.", "zufrieden, verführerisch"),
        (Sp::Hal, "Dann schläft es unter der Kapelle, in Eisen gehüllt, und ich bete jede Nacht darüber.", "beklommen, entschlossen"),
    ])];
    let core_decision = s.decision(
        "m02",
        "kern",
        "core",
        "Der Aschglas-Kern",
        (Sp::Hal, "Dieses Aschglas könnte uns verraten, wie der Schrein erbaut wurde. Oder es ist das Letzte, was wir je untersuchen. Deine Wahl, Wächterin.", "ernst, väterlich"),
        vec![
            opt("core_smashed", "Den Kern zerschlagen", "Nichts bleibt zurück, was das Tiefe nutzen könnte.", &[], smash),
            opt("core_kept", "Den Kern bergen und erforschen", "Brennhollow gewinnt Kampfmagier – und die Stimme bleibt in Reichweite.", &[], keep),
        ],
    );
    let intro = s.dlg("m02", "aufbruch", &[
        (Sp::Narrator, "Drei Tage nach dem Fall des Schreins kam ein Reiter aus dem Süden. Harrowmere, das Dorf im Aschmoor, brenne. Wer noch gehen könne, warte am Brunnen – auf ein Wunder oder auf jemanden, der keines verspreche.", "getragen, Trauer unter der Ruhe"),
        (Sp::Isolde, "Wir führen sie nach Brennhollow. Kein Wagen bleibt zurück, kein Kind. Bleibt auf der Straße, und wenn der Ascheregen fällt, bleibt zusammen.", "bestimmt, trocken"),
    ]);
    s.note("m02", "aufbruch", "Beginn der Eskorte, Ascheregen setzt ein.");
    let ford = s.dlg("m02", "furt", &[(Sp::Isolde, "Das Wasser reicht bis zu den Knien. Tragt die Kinder, und redet nicht – das Moor hört zu.", "gedämpft, konzentriert")]);
    s.note("m02", "furt", "Der Zug erreicht die Furt.");
    let ambush = s.dlg("m02", "hinterhalt", &[(Sp::Isolde, "Im Röhricht, links! Schließt die Reihen, die Schwachen in die Mitte!", "scharf, alarmiert")]);
    s.note("m02", "hinterhalt", "Hinterhalt im Röhricht vor der Furt.");
    let arrive = s.dlg("m02", "ankunft", &[
        (Sp::Hal, "Die Tore auf! Öffnet die Tore – Wasser, Decken, Brot! Sie leben, Isolde. Sie leben.", "warm, erleichtert, rufend"),
        (Sp::Nel, "Der Mann im Moor hat gesagt, ich soll dir ausrichten: Es war nett, dich kennenzulernen.", "kindlich, monoton, unheimlich"),
        (Sp::Isolde, "Welcher Mann, Nel?", "behutsam, aber angespannt"),
    ]);
    s.note("m02", "ankunft", "Brennhollow, Abend. Das Mädchen Nel spricht im Halbschlaf mit etwas, das nicht hier ist.");
    let nel_hold = vec![s.dlg("m02", "nel_a", &[
        (Sp::Hal, "Dann bleibt sie bei uns, und ich werde ihre Träume bewachen. Die Flamme verzeiht eher Mut als Furcht.", "sanft, entschlossen"),
        (Sp::Nel, "Danke. Hier ist es warm.", "leise, kindlich"),
    ])];
    let nel_away = vec![s.dlg("m02", "nel_b", &[
        (Sp::Hal, "Die Flammenschwestern in Kaldris kennen Kinder wie sie. Es ist vernünftig – und es bricht mir das Herz.", "bedrückt, ehrlich"),
        (Sp::Nel, "Kommst du mich besuchen?", "klein, hoffnungsvoll"),
    ])];
    s.note("m02", "nel_a", "Entscheidung: Nel bleibt in Brennhollow.");
    s.note("m02", "nel_b", "Entscheidung: Nel geht zu den Flammenschwestern.");
    let nel_decision = s.decision(
        "m02",
        "ankunft",
        "nel",
        "Das Mädchen Nel",
        (Sp::Hal, "Das Kind hört etwas, das uns nicht hört. Wir können es bei uns behalten und wachen – oder es den Flammenschwestern in Kaldris anvertrauen.", "behutsam, ernst"),
        vec![
            opt("nel_sheltered", "Nel in Brennhollow aufnehmen", "Nel kann das Tiefe später „hören“ – und wird zur Zielscheibe.", &[], nel_hold),
            opt("nel_sent", "Nel nach Kaldris schicken", "Sie ist sicher, doch ihre Gabe bleibt ungenutzt.", &[], nel_away),
        ],
    );
    let win = s.dlg("m02", "sieg", &[(Sp::Narrator, "Sechzehn Menschen mehr in Brennhollow, und kein Vorrat für sie. Doch an diesem Abend zählte niemand nach: Es brannten zwei Feuer mehr auf dem Marktplatz als am Morgen.", "warm, leise Hoffnung")]);
    s.note("m02", "sieg", "Schlusswort der Mission.");

    m.triggers = vec![
        trig("kern", Condition::AtTime(1.0), vec![core_scene, core_decision]),
        trig("aufbruch", any_flag(&["core_smashed", "core_kept"]), vec![
            intro,
            add_obj("obj_escort"),
            add_obj("obj_ford"),
            add_obj("obj_hero"),
            Action::SetWeather(ashen_sim::env::Weather::Ashfall),
            chron("Der Zug der Flüchtlinge von Harrowmere brach nach Brennhollow auf."),
        ]),
        trig("jagd1", all_of(vec![any_flag(&["core_smashed", "core_kept"]), Condition::AtTime(90.0)]), wave("dorf", &[("hollow_thrall", 8)])),
        trig("jagd2", all_of(vec![any_flag(&["core_smashed", "core_kept"]), Condition::AtTime(165.0)]), wave("dorf", &[("hollow_thrall", 6), ("hollow_archer", 2)])),
        trig("jagd3", all_of(vec![any_flag(&["core_smashed", "core_kept"]), Condition::AtTime(250.0)]), wave("dorf", &[("hollow_thrall", 8), ("grave_brute", 1)])),
        trig("furt", in_area("furt", 3), vec![done_obj("obj_ford"), ford]),
        trig("hinterhalt", in_area("hinterhalt", 1), vec![ambush, sp_units("hollow_thrall", 1, "flanke", 4, true, None)]),
        trig("verloren", all_of(vec![Condition::AtTime(10.0), not(Condition::UnitCount { unit: "worker".into(), count: 9 })]), vec![
            Action::Message { speaker: "Erzähler".into(), text: "Zu wenige erreichten Brennhollow. Das Moor behielt seine Toten.".into(), voice: None },
            Action::Defeat,
        ]),
        hero_lost("Isolde Raventhorn fiel im Moor, und mit ihr die letzte Hoffnung des Zuges."),
        trig("ankunft", unit_in("worker", "brennhollow", 11), vec![done_obj("obj_escort"), arrive, nel_decision]),
        trig("sieg", any_flag(&["nel_sheltered", "nel_sent"]), vec![win, chron("Die Überlebenden von Harrowmere fanden Zuflucht in Brennhollow."), Action::Victory]),
    ];
    // Folgen früherer Entscheidungen: Wer den Kern barg, besitzt Kampfmagier.
    let mut v = variant(&["core_kept"], &[], "Der geborgene Aschglas-Kern lässt zwei Kampfmagier der Flamme ihre Kraft mit der Eskorte teilen.");
    v.placements = vec![group("battle_mage", 0, 11.5, 61.0, 2)];
    m.variants = vec![v];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter I – Mission 3: Die Graue Straße (Heldenmission ohne Basis)
// ---------------------------------------------------------------------------------------------

fn m03(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m03_grey_road";
    s.title("m03", "Mission 3 – Die Graue Straße");
    let mut c = Canvas::new(96, 96, 89803, GenPreset::Heartland);
    let road = [(46.0, 92.0), (44.0, 80.0), (46.0, 70.0), (54.0, 58.0), (60.0, 46.0), (56.0, 34.0), (50.0, 24.0), (48.0, 12.0), (50.0, 4.0)];
    c.path(&road, 1.4, Terrain::Road);
    // Der Bach der Graurinne quert die Straße mit einer Furt.
    let creek = [(0.0, 52.0), (30.0, 56.0), (54.0, 58.0), (76.0, 64.0), (96.0, 62.0)];
    c.path(&creek, 2.4, Terrain::Sand);
    c.path(&creek, 1.4, Terrain::Water);
    c.disc(54.0, 58.0, 2.6, Terrain::Ford);
    c.flatten(36, 62, 52, 76, Terrain::Dirt);
    c.flatten(52, 38, 68, 52, Terrain::Dirt);
    c.flatten(40, 8, 58, 24, Terrain::Dirt);
    c.forest(8, 8, 34, 90, 2);
    c.forest(66, 6, 92, 50, 2);
    for (x, y) in [(40.5, 66.5), (48.5, 72.5), (58.5, 44.5), (64.5, 48.5), (44.5, 14.5), (54.5, 18.5)] {
        c.deco(DecorKind::Fence, x, y);
    }
    for (x, y) in [(57.5, 42.5), (62.5, 42.5)] {
        c.deco(DecorKind::Ruin, x, y);
    }
    for (x, y) in [(42.5, 12.5), (52.5, 10.5), (46.5, 20.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Die Graue Straße",
        898,
        "Heldenmission ohne Basis",
        "Die alte Handelsstraße nach Valemark ist in der Hand von Raubrittern. Brecht die drei Straßenposten – und entscheidet über Ser Wendel den Grauen.",
        "Die Graue Straße, Jahr 898 n.F. Die alte Handelsstraße nach Valemark ist Brennhollows einzige Lebensader – und seit dem Frühjahr von Raubrittern und Aschwölfen besetzt. Ihr habt keine Siedlung im Rücken: Isolde Raventhorn und acht Getreue gegen drei Straßenposten. Der erste liegt an der Furt im Süden, der zweite in einer Gehöftruine, der dritte im Norden unter Ser Wendel dem Grauen, einem Raubritter mit zu gutem Gedächtnis. Im zweiten Posten liegen Briefe, deren Siegel nach Norden weisen. Wenn Wendel verhandelt, hört ihn an: Ob er hängt oder reitet, entscheidet Ihr.",
        "Die Straße nach Valemark lag frei. Fuhrleute fassten Mut, und in den Schenken der Aschmark sprach man wieder von Salz.",
        "Die Graue Straße behielt ihre Wegelagerer – und Isolde Raventhorn ihr Grab am Straßenrand.",
        &["explore_wilds", "battle_small"],
    );
    m.map_seed = 89803;
    m.start_resources = Cost { gold: 0.0, wood: 0.0, stone: 0.0, food: 100.0 };
    m.defeat_tag = String::new();
    m.start_time = Some(0.30);
    m.placements = vec![
        hero_at(46.0, 88.5),
        group("swordsman", 0, 44.0, 87.0, 3),
        group("spearman", 0, 48.0, 87.5, 2),
        group("archer", 0, 46.0, 90.5, 4),
        // Erster Posten an der Furt.
        tagged(guard("swordsman", 1, 49.5, 64.5, 1), "post1"),
        guard("swordsman", 1, 47.0, 66.0, 3),
        guard("archer", 1, 51.0, 68.0, 3),
        // Zweiter Posten: Gehöftruine mit Aschwölfen (Ghule).
        tagged(guard("spearman", 1, 60.5, 45.5, 1), "post2"),
        guard("spearman", 1, 62.5, 47.0, 3),
        guard("ghoul", 1, 58.0, 49.5, 3),
        guard("archer", 1, 64.0, 44.0, 2),
        // Dritter Posten: Wendels Lager (verhandlungsbereit, s. Auslöser).
        tagged(guard("knight", 1, 49.5, 14.5, 1), "wendel"),
        tagged(guard("swordsman", 1, 46.0, 16.5, 4), "wendel_men"),
        tagged(guard("archer", 1, 52.5, 17.5, 3), "wendel_men"),
    ];
    m.areas = vec![
        area("posten1", "Straßenposten an der Furt", 42, 62, 12, 10),
        area("posten2", "Gehöftruine", 54, 40, 14, 12),
        area("wendel_lager", "Wendels Lager", 40, 8, 18, 14),
        area("wendel_zugang", "Zugang zu Wendels Lager", 44, 24, 12, 8),
    ];
    m.starts = vec![StartPos { owner: 0, x: 46.0, y: 88.0 }, StartPos { owner: 1, x: 49.0, y: 15.0 }];
    m.objectives = vec![
        objective("obj_post1", "Räumt den ersten Straßenposten an der Furt.", false),
        objective("obj_post2", "Räumt den zweiten Posten in der Gehöftruine.", false),
        objective("obj_wendel", "Stellt Ser Wendel den Grauen im Norden.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m03", "aufbruch", &[
        (Sp::Narrator, "Die Graue Straße führte einmal von Brennhollow bis zu den Salzmärkten Valemarks. Nun führte sie vor allem zu Gräbern am Straßenrand – und an ihrem Ende saß ein Ritter, der mit seinen Opfern Buch führte.", "trocken, düster"),
        (Sp::Isolde, "Neun Leute für eine Straße von dreißig Meilen. Das ist keine Armee, das ist ein Spaziergang mit Absicht.", "trocken, entschlossen"),
    ]);
    s.note("m03", "aufbruch", "Start am Südende der Grauen Straße.");
    let post2_dead = s.dlg("m03", "briefe", &[
        (Sp::Isolde, "Briefe. Das Wachs trägt das Siegel von Nordmark – und die Summe reicht, um eine Kompanie ein Jahr lang zu bezahlen.", "nachdenklich, kühl"),
        (Sp::Isolde, "Jemand bezahlt die Räuber dafür, dass Brennhollow hungert. Das merke ich mir.", "leise, hart"),
    ]);
    s.note("m03", "briefe", "Isolde findet im zweiten Posten die Briefe mit dem Siegel von Nordmark.");
    let parley = s.dlg("m03", "wendel", &[
        (Sp::Wendel, "Haltet ein, Wächterin! Ich lege die Klinge nieder – ich sehe, wann ein Spiel verloren ist. Und ich hasse es, für Verlierer zu arbeiten.", "heiser, spöttisch, erschöpft"),
        (Sp::Isolde, "Du hast Fuhrleute erschlagen, Wendel. Ich habe die Karren gesehen.", "kalt"),
        (Sp::Wendel, "Ich habe Befehle befolgt, die mit Gold kamen. Das ist kein Freispruch, ich weiß. Aber ich kenne jeden Pfad zwischen hier und Kaldris – und meine Männer können kämpfen.", "ehrlich, müde"),
    ]);
    s.note("m03", "wendel", "Ser Wendel streckt die Waffen und verhandelt; seine Männer halten still.");
    let hang_scene = vec![s.dlg("m03", "wendel_a", &[
        (Sp::Isolde, "Du hängst, Wendel. Für jeden Karren, den du verbrannt hast. Die Straße braucht keine Reue, sie braucht Ruhe.", "hart, endgültig"),
        (Sp::Wendel, "Gerecht. Fast. Sagt den Fuhrleuten … dass ich sie gezählt habe.", "gefasst, leise"),
    ])];
    let recruit_scene = vec![s.dlg("m03", "wendel_b", &[
        (Sp::Isolde, "Du dienst mir, bis die Straße sicher ist. Wenn du uns verrätst, hole ich dich persönlich.", "nüchtern, ohne Drohgebärde"),
        (Sp::Wendel, "Die Graue Kompanie steht Euch zu Diensten, Wächterin. Es ist das erste Mal seit Jahren, dass ich bei Tag schlafen kann.", "erleichtert, trocken"),
    ])];
    s.note("m03", "wendel_a", "Entscheidung: Wendel wird gehängt.");
    s.note("m03", "wendel_b", "Entscheidung: Wendel und seine Männer werden in Dienst genommen.");
    let wendel_decision = s.decision(
        "m03",
        "wendel",
        "wendel",
        "Ser Wendel der Graue",
        (Sp::Narrator, "Ein Wegelagerer, der um sein Leben handelt. Die Flamme verlangt Gerechtigkeit – aber Brennhollow braucht Schwerter. Die Entscheidung liegt bei der Wächterin.", "ruhig, abwägend"),
        vec![
            opt("wendel_hanged", "Ser Wendel hängen", "Die Straße ist gerächt, doch Wendels Männer zerstreuen sich.", &[], hang_scene),
            opt("wendel_recruited", "Die Graue Kompanie in Dienst nehmen", "Die Männer kämpfen später an Eurer Seite.", &[], recruit_scene),
        ],
    );
    let win = s.dlg("m03", "sieg", &[(Sp::Narrator, "Am Abend zogen die ersten Karren seit zwei Jahren nordwärts. Die Aschmark war immer noch ein Grenzland – aber nun eines mit einer Straße.", "ruhig, Anerkennung")]);
    s.note("m03", "sieg", "Schlusswort.");
    m.triggers = vec![
        trig("aufbruch", Condition::AtTime(1.0), vec![intro, add_obj("obj_post1"), add_obj("obj_post2"), add_obj("obj_wendel"), add_obj("obj_hero"), chron("Isolde Raventhorn brach auf, die Graue Straße zu befreien.")]),
        trig("post1", tag_dead("post1"), vec![done_obj("obj_post1"), Action::Message { speaker: "Isolde Raventhorn".into(), text: "Einer. Die Straße ist lang.".into(), voice: None }]),
        trig("post2", tag_dead("post2"), vec![done_obj("obj_post2"), post2_dead]),
        trig("parley", in_area("wendel_zugang", 1), vec![
            Action::ChangeOwner { tag: "wendel".into(), owner: 2 },
            Action::ChangeOwner { tag: "wendel_men".into(), owner: 2 },
            parley,
            wendel_decision,
        ]),
        trig("recruit_note", flag("wendel_recruited"), vec![Action::ChangeOwner { tag: "wendel".into(), owner: 0 }, Action::ChangeOwner { tag: "wendel_men".into(), owner: 0 }]),
        trig("sieg", all_of(vec![any_flag(&["wendel_hanged", "wendel_recruited"]), Condition::ObjectiveCompleted("obj_post1".into()), Condition::ObjectiveCompleted("obj_post2".into())]), vec![
            done_obj("obj_wendel"),
            done_obj("obj_hero"),
            win,
            chron("Die Graue Straße wurde befreit."),
            Action::Victory,
        ]),
        hero_lost("Isolde Raventhorn fiel auf der Grauen Straße."),
    ];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter II – Mission 4: Die Lange Nacht (Überleben bis Morgengrauen)
// ---------------------------------------------------------------------------------------------

fn m04(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m04_long_night";
    s.title("m04", "Mission 4 – Die Lange Nacht");
    let mut c = Canvas::new(80, 80, 89904, GenPreset::Aschmark);
    c.flatten(18, 18, 62, 62, Terrain::Dirt);
    c.flatten(24, 24, 56, 56, Terrain::Stone);
    c.path(&[(40.0, 56.0), (40.0, 78.0)], 1.4, Terrain::Road);
    c.path(&[(2.0, 40.0), (24.0, 40.0)], 1.4, Terrain::Road);
    c.path(&[(40.0, 2.0), (40.0, 24.0)], 1.4, Terrain::Road);
    c.path(&[(56.0, 40.0), (78.0, 40.0)], 1.4, Terrain::Road);
    c.node(NodeKind::GoldMine, 62, 20);
    c.node(NodeKind::StoneQuarry, 14, 60);
    c.forest(6, 6, 16, 30, 2);
    c.forest(64, 54, 76, 74, 3);
    for (x, y) in [(26.5, 26.5), (53.5, 26.5), (26.5, 53.5), (53.5, 53.5), (33.5, 40.5), (46.5, 40.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Die Lange Nacht",
        899,
        "Überleben bis zum Morgengrauen",
        "Die größte Hohlenwelle des Zeitalters rollt auf das ausgebaute Brennhollow zu. Haltet die Mauern bis zum Morgen – dann trifft der Gesandte des Königs ein.",
        "Brennhollow, Jahr 899 n.F. Zwei Jahre nach dem ersten Schrein ist Brennhollow gewachsen: Mauern, Kaserne, Gehöfte, hundert Stimmen am Marktplatz. Doch in dieser Nacht zieht das Hohle Tief alles zusammen, was es hat. Der König hat einen Gesandten geschickt, der im Morgengrauen eintreffen soll – wenn es einen Morgen gibt. Haltet Mauern und Bergfried bis zum ersten Licht. Von allen vier Seiten rollen die Wellen heran, und mit ihnen kommen Grabkolosse, die Mauern einreißen. Nutzt Eure Helden und Eure Schützen auf den Türmen.",
        "Das erste Licht traf die Zinnen von Brennhollow. Die Mauern standen, die Fahne wehte – und auf der Südstraße näherte sich ein Reiter in Schwarz.",
        "Die Mauern fielen, die Glocke verstummte. Brennhollow ging in der Langen Nacht unter.",
        &["build_night", "siege_defense"],
    );
    m.map_seed = 89904;
    m.start_resources = Cost { gold: 600.0, wood: 400.0, stone: 400.0, food: 500.0 };
    m.start_goods = vec![("weapons".into(), 20.0), ("armor".into(), 12.0), ("planks".into(), 30.0), ("tools".into(), 8.0)];
    m.defeat_tag = "player_keep".into();
    m.start_time = Some(0.72);
    let mut p = vec![
        tagged(pl("keep", 0, 38.0, 38.0), "player_keep"),
        pl("barracks", 0, 30.0, 30.0),
        pl("barracks", 0, 46.0, 30.0),
        pl("farmstead", 0, 31.0, 44.0),
        pl("farmstead", 0, 46.0, 44.0),
        pl("hut", 0, 28.0, 38.0),
        pl("hut", 0, 49.0, 38.0),
        pl("hut", 0, 34.0, 48.0),
        pl("watchtower", 0, 25.0, 25.0),
        pl("watchtower", 0, 53.0, 25.0),
        pl("watchtower", 0, 25.0, 53.0),
        pl("watchtower", 0, 53.0, 53.0),
        group("worker", 0, 40.5, 45.0, 8),
        group("swordsman", 0, 36.0, 34.0, 8),
        group("spearman", 0, 42.0, 34.0, 5),
        group("archer", 0, 39.0, 50.5, 7),
        group("knight", 0, 44.0, 36.0, 2),
        hero_at(40.0, 41.5),
    ];
    wall_ring(&mut p, 22, 22, 58, 58, &[(40, 58), (22, 40)], &[(22, 22), (57, 22), (22, 57), (57, 57)]);
    m.placements = p;
    m.areas = vec![
        area("nord", "Nordstraße", 36, 2, 8, 5),
        area("ost", "Oststraße", 73, 36, 5, 8),
        area("west", "Weststraße", 2, 36, 5, 8),
        area("sued", "Südstraße", 36, 73, 8, 5),
        area("stadt", "Brennhollow", 22, 22, 37, 37),
    ];
    m.starts = vec![StartPos { owner: 0, x: 40.0, y: 41.0 }, StartPos { owner: 1, x: 40.0, y: 4.0 }];
    m.objectives = vec![
        objective("obj_dawn", "Haltet Brennhollow bis zum Morgengrauen.", false),
        objective("obj_keep", "Der Bergfried darf nicht fallen.", false),
        objective("obj_brutes", "Zerschlagt die Grabkolosse, bevor sie die Mauer einreißen.", true),
    ];
    let intro = s.dlg("m04", "nacht", &[
        (Sp::Narrator, "Die Hohlen kamen nicht mehr in Wellen. In dieser Nacht kamen sie wie eine Flut, die einen Deich gefunden hat. Auf den Zinnen von Brennhollow zählten die Wächter ihre Fackeln und sagten nichts.", "schwer, ruhig"),
        (Sp::Hal, "Der Gesandte des Königs soll bei Sonnenaufgang eintreffen, Isolde. Wenn er uns lebend findet, haben wir genug erreicht.", "trocken, müde, väterlich"),
        (Sp::Isolde, "Dann sorgen wir dafür, dass er uns lebend findet. Bogenschützen auf die Türme, Schwertträger ans Tor!", "bestimmt"),
    ]);
    s.note("m04", "nacht", "Dämmerung über Brennhollow; die ersten Hohlen sind am Horizont.");
    let mid = s.dlg("m04", "mitte", &[
        (Sp::Hal, "Sie bringen die Kolosse! Haltet die Tore, Isolde – ich singe die Mauer fest, so lange meine Stimme trägt!", "rufend, eindringlich"),
    ]);
    s.note("m04", "mitte", "Die Grabkolosse erreichen die Mauern.");
    let dawn = s.dlg("m04", "morgen", &[
        (Sp::Narrator, "Der Himmel über der Aschmark wurde grau, dann fahl, dann golden an den Rändern. Die Letzten der Hohlen sanken in die Erde zurück, als hätten sie nur geträumt, lebendig zu sein.", "getragen, Erleichterung"),
        (Sp::Kassian, "Eine Nacht, die man in Kaldris noch in Jahren erzählen wird – leider ohne Wahrheit. Ich bin Kassian Vale. Seine Majestät bittet Euch, Wächterin, um Euren Treueeid. Und bevor Ihr antwortet: Herzogin Mireille Corvane bietet Euch fünfhundert Speere und Korn für ein Jahr.", "geschmeidig, höflich, leicht amüsiert"),
    ]);
    s.note("m04", "morgen", "Morgengrauen. Der Gesandte des Königs, Kassian Vale, trifft am Südtor ein.");
    let crown = vec![s.dlg("m04", "eid_a", &[
        (Sp::Isolde, "Mein Vater starb, als er Valemarks Grenze hielt. Ich werde sie auch halten. Für die Glutkrone, Majestät.", "fest, förmlich"),
        (Sp::Kassian, "Wie standhaft. Der König wird es zu schätzen wissen – und die Herzogin sich Euren Namen merken.", "höflich, mit Unterton"),
    ])];
    let corvane = vec![s.dlg("m04", "eid_b", &[
        (Sp::Isolde, "Meine Leute können keine Eide essen. Richtet der Herzogin aus, dass ich ihre Speere annehme.", "trocken, pragmatisch"),
        (Sp::Kassian, "Eine kluge Entscheidung – und eine sehr teure. Nordmark erinnert sich an Freunde, Wächterin. Und an Schuldner.", "ruhig, vieldeutig"),
    ])];
    s.note("m04", "eid_a", "Entscheidung: Treueeid auf die Krone.");
    s.note("m04", "eid_b", "Entscheidung: Bündnis mit Herzogin Mireille Corvane.");
    let oath = s.decision(
        "m04",
        "morgen",
        "oath",
        "Treueeid oder Bündnis",
        (Sp::Kassian, "Zwei Hände strecken sich Euch entgegen, Wächterin. Die Krone fordert Euren Eid, Nordmark bietet Speere. Welche Hand Ihr heute nehmt – die andere hält morgen ein Messer.", "leise, ruhig, eindringlich"),
        vec![
            opt("oath_crown", "Der Krone den Treueeid schwören", "Die Krone unterstützt Euch mit einer königlichen Garde; Nordmark wird kühl.", &[], crown),
            opt("oath_corvane", "Mireilles Bündnis annehmen", "Nordmarks Speere stehen Euch bei – doch Mireille wird Gegenleistungen verlangen.", &[], corvane),
        ],
    );
    let mk = |t: f32, id: &str, a: &str, u: &[(&str, u32)]| -> ashen_sim::mission::TriggerDef { trig(id, Condition::AtTime(t), wave(a, u)) };
    m.triggers = vec![
        trig("nacht", Condition::AtTime(1.0), vec![intro, add_obj("obj_dawn"), add_obj("obj_keep"), add_obj("obj_brutes"), chron("In der Langen Nacht sammelte das Hohle Tief alles, was es besaß.")]),
        mk(40.0, "w1", "west", &[("hollow_thrall", 10), ("hollow_archer", 3)]),
        mk(85.0, "w2", "nord", &[("hollow_thrall", 12), ("hollow_archer", 4)]),
        mk(135.0, "w3", "ost", &[("hollow_thrall", 10), ("ghoul", 3), ("hollow_archer", 3)]),
        trig("kolosse", Condition::AtTime(190.0), {
            let mut a = vec![mid];
            a.extend(wave("sued", &[("grave_brute", 2), ("hollow_thrall", 10)]));
            a
        }),
        mk(250.0, "w5", "west", &[("hollow_thrall", 14), ("ash_witch", 2), ("hollow_archer", 5)]),
        mk(310.0, "w6", "nord", &[("grave_brute", 2), ("hollow_thrall", 12), ("ghoul", 4)]),
        mk(380.0, "w7", "ost", &[("hollow_thrall", 14), ("hollow_archer", 6), ("bat_swarm", 3)]),
        mk(450.0, "w8", "sued", &[("grave_brute", 3), ("hollow_thrall", 14), ("ash_witch", 2)]),
        mk(510.0, "w9", "west", &[("hollow_thrall", 12), ("hollow_archer", 6), ("ghoul", 4)]),
        trig("kolosse_tot", Condition::All(vec![Condition::AtTime(200.0), Condition::EnemyUnitsAtMost(0)]), vec![done_obj("obj_brutes")]),
        trig("morgen", Condition::AtTime(560.0), vec![done_obj("obj_dawn"), done_obj("obj_keep"), dawn, oath]),
        trig("sieg", any_flag(&["oath_crown", "oath_corvane"]), vec![chron("Brennhollow überstand die Lange Nacht und erhielt den Besuch des Königs."), Action::Victory]),
    ];
    // Folgen früherer Entscheidungen.
    let mut v1 = variant(&["wendel_recruited"], &[], "Die Graue Kompanie unter Ser Wendel hält die Nordmauer.");
    v1.placements = vec![group("swordsman", 0, 40.0, 25.5, 4), group("archer", 0, 43.0, 25.5, 3)];
    let mut v2 = variant(&["wendel_hanged"], &[], "Ohne Wendels Männer bleibt der Nordstreifen dünn besetzt; die Straßenhändler spenden Gold für Palisaden.");
    v2.grant = Cost { gold: 250.0, wood: 150.0, stone: 100.0, food: 0.0 };
    let mut v3 = variant(&["nel_sheltered"], &[], "Nel warnt in der Nacht vor jedem Angriff: Die Wachen stehen früh auf den Mauern.");
    v3.grant = Cost { gold: 0.0, wood: 0.0, stone: 0.0, food: 150.0 };
    v3.placements = vec![group("spearman", 0, 36.0, 41.0, 3)];
    m.variants = vec![v1, v2, v3];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter II – Mission 6: Das Turnier von Kaldris (Wettlauf)
// ---------------------------------------------------------------------------------------------

fn m06(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m06_tourney_kaldris";
    s.title("m06", "Mission 6 – Das Turnier von Kaldris");
    let mut c = Canvas::new(72, 64, 90106, GenPreset::Heartland);
    c.flatten(10, 10, 62, 54, Terrain::Stone);
    c.disc(36.0, 32.0, 10.0, Terrain::Dirt);
    c.disc(36.0, 32.0, 7.0, Terrain::Sand);
    c.path(&[(2.0, 32.0), (26.0, 32.0)], 1.6, Terrain::Road);
    c.path(&[(46.0, 32.0), (70.0, 30.0)], 1.6, Terrain::Road);
    c.path(&[(36.0, 22.0), (34.0, 12.0)], 1.4, Terrain::Road);
    for i in 0..12 {
        let a = i as f32 / 12.0 * std::f32::consts::TAU;
        c.deco(DecorKind::Pillar, 36.0 + a.cos() * 11.5, 32.0 + a.sin() * 11.5);
    }
    for (x, y) in [(14.5, 14.5), (58.5, 14.5), (14.5, 50.5), (58.5, 50.5), (22.5, 46.5), (50.5, 46.5)] {
        c.deco(DecorKind::Lantern, x, y);
    }
    for (x, y) in [(12.5, 24.5), (60.5, 40.5), (24.5, 12.5), (48.5, 52.5)] {
        c.deco(DecorKind::Ruin, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Das Turnier von Kaldris",
        900,
        "Wettlauf",
        "Im Turnier der Glutkrone besteht Ihr die Zweikämpfe – und in der Nacht danach jagt Ihr die Attentäter, bevor sie das Tor der Flucht erreichen.",
        "Kaldris, Jahr 900 n.F. Die heilige Stadt der Ewigen Flamme feiert das Turnier der Glutkrone. Isolde Raventhorn tritt mit ihrer Eskorte in der Arena an: zwei Zweikampfrunden gegen die Champions der Herzöge. Kassian Vale hat angedeutet, dass bei der Abschlussfeier etwas geschehen wird – er will wissen, was Ihr damit tut. Wenn die Attentäter erscheinen, zählt jede Sekunde: Drei Mörder fliehen vom Balkon zum Tor der Flucht im Osten. Erreicht auch nur einer das Tor, ist der König verloren. Schnelle Reiter holen Läufer ein.",
        "Die drei Mörder lagen im Staub der Arena. Der König lebte – und fragte sich laut, wer ihm das Leben verdankte.",
        "Der letzte Mörder erreichte das Tor der Flucht. In Kaldris läuteten die Glocken für einen toten König.",
        &["diplomacy_court", "intrigue_shadows"],
    );
    m.map_seed = 90106;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.45);
    m.placements = vec![
        hero_at(8.0, 32.5),
        group("swordsman", 0, 6.0, 31.0, 3),
        group("scout_rider", 0, 5.0, 34.5, 3),
        group("horse_archer", 0, 8.5, 35.5, 2),
        group("archer", 0, 6.5, 29.0, 2),
    ];
    m.areas = vec![
        area("arena", "Die Arena", 26, 22, 20, 20),
        area("balkon", "Der Balkon der Krone", 30, 10, 10, 6),
        area("flucht", "Das Tor der Flucht", 64, 25, 7, 10),
        area("weg", "Die Gasse der Händler", 44, 17, 12, 12),
        area("tor", "Das Westtor", 2, 28, 8, 10),
    ];
    m.starts = vec![StartPos { owner: 0, x: 8.0, y: 32.0 }];
    m.objectives = vec![
        objective("obj_duel1", "Besteht die erste Zweikampfrunde.", false),
        objective("obj_duel2", "Besteht die zweite Zweikampfrunde.", false),
        objective("obj_assassins", "Stellt die drei Attentäter, bevor einer das Tor der Flucht erreicht.", false),
    ];
    let intro = s.dlg("m06", "turnier", &[
        (Sp::Narrator, "Kaldris trug Gold und Weiß. Über der Arena hingen die Banner der zwölf Herzogtümer, und auf der Tribüne saß ein alternder König, der aussah, als warte er auf etwas, das nicht der Sieger war.", "festlich, mit leisem Unterton"),
        (Sp::Kassian, "Eine Beobachtung, Wächterin, ganz unverbindlich: Heute Abend wird der Balkon der Krone nicht jeden verlassen, der ihn betritt. Ich überlasse es Euch, was Ihr mit diesem Wissen anfangt.", "flüsternd, geschmeidig"),
    ]);
    s.note("m06", "turnier", "Die Arena von Kaldris, Turniertag.");
    let r2 = s.dlg("m06", "runde2", &[(Sp::Aldric, "Die Wächterin der Aschmark! Man erzählt, sie schlage die Besten, ohne zu schwitzen. Zeigt es uns noch einmal!", "laut, belebt, mit alter Stimme")]);
    s.note("m06", "runde2", "Der König ruft aus der Loge zur zweiten Runde.");
    let bell = s.dlg("m06", "glocke", &[
        (Sp::Narrator, "Es war das Läuten der Abendglocke, das ihn rettete: ein falscher Ton, eine Sekunde zu früh. Auf dem Balkon der Krone regte sich ein Schatten, dann zwei, dann drei.", "gespannt, rasch"),
    ]);
    s.note("m06", "glocke", "Abendglocke: Die Attentäter am Balkon der Krone regen sich.");
    let attack = s.dlg("m06", "anschlag", &[
        (Sp::Isolde, "Dort oben! Die Reiter voran – holt sie ein, bevor sie die Gassen erreichen!", "scharf"),
    ]);
    s.note("m06", "anschlag", "Die Attentäter fliehen vom Balkon der Krone.");
    let warned = vec![s.dlg("m06", "kassian_a", &[
        (Sp::Isolde, "Majestät, ein Wort unter vier Augen: Es gab eine Warnung, und sie war nicht ohne Eigennutz.", "ernst, leise"),
        (Sp::Aldric, "Ihr habt mir das Leben gerettet und das Spiel verdorben. Ich weiß nicht, was mich mehr bewegt.", "müde, dankbar"),
    ])];
    let leverage = vec![s.dlg("m06", "kassian_b", &[
        (Sp::Isolde, "Kassian. Ihr habt mehr gewusst, als Ihr gesagt habt. Dafür schuldet Ihr mir etwas.", "kühl"),
        (Sp::Kassian, "So ist es. Es ist angenehm, wenn jemand begreift, dass Wissen eine Währung ist. Ich werde zahlen, wenn die Stunde kommt.", "mit feinem Lächeln"),
    ])];
    let silent = vec![s.dlg("m06", "kassian_c", &[
        (Sp::Isolde, "Es war ein Anschlag, mehr nicht. Ich habe nichts gesehen, was zu erzählen wäre.", "neutral, abweisend"),
        (Sp::Kassian, "Wie diskret. Ich bin beinahe enttäuscht – und beeindruckt.", "trocken, flüchtig"),
    ])];
    s.note("m06", "kassian_a", "Entscheidung: Der König wird gewarnt.");
    s.note("m06", "kassian_b", "Entscheidung: Kassian wird in der Hand behalten.");
    s.note("m06", "kassian_c", "Entscheidung: Schweigen.");
    let secret = s.decision(
        "m06",
        "ende",
        "kassian_secret",
        "Kassians Geheimnis",
        (Sp::Kassian, "Nun wisst Ihr, dass ich es wusste. Was macht Ihr damit, Wächterin: Sagt Ihr es dem König, behaltet Ihr es als Pfand – oder schweigt Ihr?", "leise, belustigt"),
        vec![
            opt("kassian_warned_king", "Dem König die Wahrheit sagen", "Aldric steht Euch bei; Kassian wird zum Feind.", &[], warned),
            opt("kassian_leverage", "Das Wissen als Pfand behalten", "Kassian schuldet Euch einen Gefallen.", &[], leverage),
            opt("kassian_silent", "Schweigen", "Keine Verpflichtung, keine Verbündeten.", &[], silent),
        ],
    );
    let assassin = |n: u32| sp_units("worker", 1, &format!("balkon{n}"), 1, false, Some(&format!("assassin{n}")));
    m.areas.push(area("balkon1", "Balkon links", 30, 11, 2, 2));
    m.areas.push(area("balkon2", "Balkon Mitte", 34, 11, 2, 2));
    m.areas.push(area("balkon3", "Balkon rechts", 38, 11, 2, 2));
    m.triggers = vec![
        trig("turnier", Condition::AtTime(1.0), vec![intro, add_obj("obj_duel1"), add_obj("obj_duel2"), add_obj("obj_assassins"), chron("Isolde Raventhorn trat beim Turnier der Glutkrone an.")]),
        trig("runde1", Condition::AtTime(14.0), wave("arena", &[("swordsman", 4), ("spearman", 2)])),
        trig("runde1_ok", all_of(vec![Condition::AtTime(20.0), Condition::EnemyUnitsAtMost(0)]), vec![done_obj("obj_duel1"), r2]),
        trig("runde2", all_of(vec![Condition::ObjectiveCompleted("obj_duel1".into()), Condition::AtTime(40.0)]), wave("arena", &[("swordsman", 4), ("archer", 2), ("knight", 1)])),
        trig("runde2_ok", all_of(vec![Condition::ObjectiveCompleted("obj_duel1".into()), Condition::AtTime(60.0), Condition::EnemyUnitsAtMost(0)]), vec![done_obj("obj_duel2")]),
        trig("glocke", Condition::ObjectiveCompleted("obj_duel2".into()), vec![bell, Action::SetFlag("glocke".into())]),
        trig("anschlag", all_of(vec![flag("glocke"), Condition::DialogueDone]), vec![
            attack,
            assassin(1),
            assassin(2),
            assassin(3),
            Action::MoveTag { tag: "assassin1".into(), area: "flucht".into(), attack: false },
            Action::MoveTag { tag: "assassin2".into(), area: "flucht".into(), attack: false },
            Action::MoveTag { tag: "assassin3".into(), area: "flucht".into(), attack: false },
        ]),
        trig("verloren", Condition::Any(vec![
            Condition::TagInArea { tag: "assassin1".into(), area: "flucht".into() },
            Condition::TagInArea { tag: "assassin2".into(), area: "flucht".into() },
            Condition::TagInArea { tag: "assassin3".into(), area: "flucht".into() },
        ]), vec![Action::Message { speaker: "Erzähler".into(), text: "Ein Mörder erreichte das Tor. Zu spät.".into(), voice: None }, Action::Defeat]),
        hero_lost("Isolde Raventhorn fiel in den Gassen von Kaldris."),
        trig("gestellt", all_of(vec![tag_dead("assassin1"), tag_dead("assassin2"), tag_dead("assassin3")]), vec![done_obj("obj_assassins"), secret]),
        trig("sieg", any_flag(&["kassian_warned_king", "kassian_leverage", "kassian_silent"]), vec![chron("Der Anschlag auf König Aldric III. wurde vereitelt."), Action::Victory]),
    ];
    // Folgen früherer Entscheidungen.
    let mut v1 = variant(&["oath_crown"], &[], "Königliche Ritter begleiten Isolde als Ehrengarde.");
    v1.placements = vec![group("knight", 0, 4.5, 31.0, 2)];
    let mut v2 = variant(&["oath_corvane"], &[], "Mireille Corvane schickt Nordmarker Reiter mit – als Geschenk und als Beobachter.");
    v2.placements = vec![group("horse_archer", 0, 4.5, 33.5, 3)];
    m.variants = vec![v1, v2];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter II – Mission 7: Eisen in den Hügeln (Belagerung)
// ---------------------------------------------------------------------------------------------

fn m07(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m07_iron_hills";
    s.title("m07", "Mission 7 – Eisen in den Hügeln");
    let mut c = Canvas::new(80, 80, 90207, GenPreset::Highlands);
    c.flatten(44, 14, 76, 46, Terrain::Stone);
    c.flatten(4, 54, 26, 76, Terrain::Dirt);
    c.path(&[(14.0, 62.0), (28.0, 56.0), (40.0, 50.0), (52.0, 44.0)], 1.6, Terrain::Road);
    c.node(NodeKind::GoldMine, 66, 52);
    c.node(NodeKind::StoneQuarry, 30, 64);
    c.forest(2, 40, 14, 52, 3);
    for (x, y) in [(40.5, 52.5), (46.5, 52.5), (36.5, 48.5)] {
        c.deco(DecorKind::Boulder, x, y);
    }
    for (x, y) in [(8.5, 58.5), (22.5, 60.5), (14.5, 72.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Eisen in den Hügeln",
        902,
        "Belagerung",
        "Die Eisenmine von Kargsted liegt in der Hand eines Raubritters hinter einer Steinfeste. Rammen, Katapulte und eine ruhige Hand – brecht das Tor.",
        "Kargsted, Jahr 902 n.F. Brennhollows Schmieden verlangen Eisen, und das einzige gute Erz der Gegend liegt unter der Steinfeste Kargsted, die ein Raubritter mit eigener Garnison hält. Ihr führt eine echte Belagerung: drei Rammböcke, zwei Katapulte, Fußvolk und Schützen. Das Tor im Süden ist die schwächste Stelle, die Türme an den Ecken tragen Schützen. Brecht das Tor mit den Rammen, haltet die Schützen mit den Katapulten nieder und nehmt den Bergfried der Feste.",
        "Das Tor von Kargsted fiel, und mit ihm die Hoffnung des Raubritters. In den Schmieden von Brennhollow brannte noch in derselben Nacht das erste Feuer mit eigenem Eisen.",
        "Der Sturm auf Kargsted scheiterte. Die Rammen brannten, und der Raubritter lachte über die Mauern.",
        &["siege_assault", "war_preparation"],
    );
    m.map_seed = 90207;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.40);
    let mut p = vec![
        hero_at(14.5, 62.5),
        group("swordsman", 0, 11.0, 64.0, 8),
        group("spearman", 0, 16.0, 66.0, 6),
        group("archer", 0, 12.5, 59.0, 6),
        group("crossbowman", 0, 17.5, 61.0, 3),
        group("knight", 0, 9.0, 61.0, 2),
        group("battering_ram", 0, 18.0, 64.0, 3),
        group("catapult", 0, 20.0, 60.5, 2),
        // Kargsted: Mauerring mit Südtor und Eckschützen.
        tagged(pl("keep", 1, 58.0, 28.0), "kargsted_keep"),
        guard("swordsman", 1, 56.0, 33.0, 5),
        guard("spearman", 1, 61.0, 33.0, 4),
        guard("archer", 1, 52.0, 26.0, 4),
        guard("archer", 1, 64.0, 26.0, 4),
        guard("crossbowman", 1, 58.0, 24.0, 3),
        tagged(guard("knight", 1, 59.0, 31.0, 2), "garrison_knights"),
        guard("swordsman", 1, 57.0, 41.0, 3),
        guard("archer", 1, 60.0, 41.0, 2),
    ];
    // Das Nordtor (Ausfalltor) ist nur eine schmale Lücke: Die Feste bleibt auf dem Landweg erreichbar.
    wall_ring_gaps(&mut p, 1, (50, 20, 68, 38), &[(59, 38)], &[(50, 20), (67, 20), (50, 37), (67, 37)], &[(59, 20)]);
    m.placements = p;
    m.areas = vec![
        area("lager", "Belagerungslager", 4, 54, 24, 22),
        area("kargsted", "Kargsted", 48, 18, 24, 24),
        area("tor", "Das Südtor", 54, 38, 12, 8),
    ];
    m.starts = vec![StartPos { owner: 0, x: 14.0, y: 62.0 }, StartPos { owner: 1, x: 59.0, y: 42.0 }];
    m.objectives = vec![
        objective("obj_gate", "Brecht das Südtor von Kargsted.", false),
        objective("obj_keep", "Nehmt den Bergfried von Kargsted ein.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m07", "lager", &[
        (Sp::Narrator, "Eisen war in der Aschmark kostbarer als Brot. Wer die Schmiede befeuern wollte, musste durch Kargsted – und Kargsted hatte Mauern, die ein Schwarm Toter nie gebraucht hätte.", "trocken, sachlich"),
        (Sp::Isolde, "Die Rammen zuerst ans Tor, die Katapulte auf die Eckschützen. Wer die Mauer mit der Brust einreißen will, lernt es heute auf die harte Weise.", "knapp, entschlossen"),
    ]);
    s.note("m07", "lager", "Belagerungslager vor Kargsted.");
    let gate = s.dlg("m07", "tor", &[(Sp::Isolde, "Das Tor ist gebrochen! Schildträger vor, Schützen nach – keine Gefangenen, aber lasst die Bauern am Leben!", "triumphierend, scharf")]);
    s.note("m07", "tor", "Das Südtor fällt.");
    let fin = s.dlg("m07", "sieg", &[(Sp::Hal, "Eisen. Wirkliches Eisen. Ich muss es dir lassen, Isolde: Du verstehst, wann man eine Mauer überredet und wann man sie einreißt.", "warm, anerkennend")]);
    s.note("m07", "sieg", "Nach dem Sieg im Hof von Kargsted.");
    m.triggers = vec![
        trig("lager", Condition::AtTime(1.0), vec![intro, add_obj("obj_gate"), add_obj("obj_keep"), add_obj("obj_hero"), chron("Isolde Raventhorn belagerte die Steinfeste Kargsted.")]),
        trig("tor", Condition::Any(vec![Condition::TagDamaged("kargsted_keep".into())]), vec![done_obj("obj_gate"), gate]),
        trig("sieg", tag_dead("kargsted_keep"), vec![done_obj("obj_keep"), done_obj("obj_hero"), fin, chron("Kargsted fiel, und mit ihm das Eisenmonopol der Raubritter."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel vor den Mauern von Kargsted."),
    ];
    // Folgen früherer Entscheidungen.
    let mut v1 = variant(&["oath_corvane"], &[], "Mireilles Nordmarker Speerträger verstärken die Belagerer.");
    v1.placements = vec![group("spearman", 0, 20.5, 66.5, 10), group("halberdier", 0, 23.0, 63.0, 3)];
    let mut v2 = variant(&["oath_crown"], &[], "Die Krone schickt königliche Ritter und ein weiteres Katapult.");
    v2.placements = vec![group("knight", 0, 8.5, 66.5, 3), group("catapult", 0, 22.0, 58.0, 1)];
    let mut v3 = variant(&["wendel_recruited"], &[], "Die Graue Kompanie kennt die Schwächen der Mauern.");
    v3.placements = vec![group("swordsman", 0, 10.5, 56.0, 4), group("archer", 0, 14.0, 55.0, 3)];
    let mut v4 = variant(&["kassian_leverage"], &[], "Kassians Hinweise: Ein Tor-Wächter ist bestochen, die Garnison ist geschwächt.");
    v4.remove_tags = vec!["garrison_knights".into()];
    m.variants = vec![v1, v2, v3, v4];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}
