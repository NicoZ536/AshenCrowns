//! Missionen der Zeitalter V bis VII (geteilter Thron, gefallene Götter, letzte Krone).

use crate::campaign_forge::*;
use crate::forge::{area, group, guard, pl, tagged, trig, wall_ring, wall_ring_gaps, Canvas};
use ashen_sim::data::{Cost, GameData};
use ashen_sim::entity::NodeKind;
use ashen_sim::genmap::GenPreset;
use ashen_sim::map::{DecorKind, Terrain};
use ashen_sim::mission::{Action, Condition, MissionDef};
use ashen_sim::scenario::StartPos;

pub(crate) fn all(data: &GameData, s: &mut Script) -> Vec<MissionDef> {
    vec![m17(data, s), m19(data, s), m21(data, s), m25(data, s), m26(data, s), m28(data, s), m29(data, s)]
}

fn hero_at(x: f32, y: f32) -> ashen_sim::mission::Placement {
    tagged(pl("hero_isolde", 0, x, y), "hero")
}

/// Mauer aus Felsbrocken (blockierende Dekoration) für Stollen und Hohlwege.
fn boulder_line(c: &mut Canvas, x0: i32, y0: i32, x1: i32, y1: i32, gaps: &[(i32, i32)]) {
    for y in y0..=y1 {
        for x in x0..=x1 {
            if gaps.iter().any(|(a, b)| (y == y0 && y == y1 && x >= *a && x <= *b) || (x == x0 && x == x1 && y >= *a && y <= *b)) {
                continue;
            }
            c.clear(x, y, x, y);
            c.deco(DecorKind::Boulder, x as f32 + 0.5, y as f32 + 0.5);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Zeitalter V – Mission 17: Tod eines Königs (Ermittlung)
// ---------------------------------------------------------------------------------------------

fn m17(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m17_death_of_a_king";
    s.title("m17", "Mission 17 – Tod eines Königs");
    let mut c = Canvas::new(72, 72, 91617, GenPreset::Heartland);
    c.flatten(6, 6, 66, 66, Terrain::Stone);
    c.disc(36.0, 36.0, 11.0, Terrain::Sand);
    c.disc(36.0, 36.0, 7.0, Terrain::Stone);
    c.path(&[(36.0, 70.0), (36.0, 48.0)], 1.8, Terrain::Road);
    c.path(&[(36.0, 24.0), (36.0, 8.0)], 1.8, Terrain::Road);
    c.path(&[(24.0, 36.0), (10.0, 58.0)], 1.6, Terrain::Road);
    c.path(&[(48.0, 36.0), (58.0, 56.0)], 1.6, Terrain::Road);
    for i in 0..10 {
        let a = i as f32 / 10.0 * std::f32::consts::TAU;
        c.deco(DecorKind::Pillar, 36.0 + a.cos() * 12.5, 36.0 + a.sin() * 12.5);
    }
    for (x, y) in [(14.5, 18.5), (58.5, 18.5), (14.5, 50.5), (62.5, 30.5), (20.5, 62.5)] {
        c.deco(DecorKind::Ruin, x, y);
    }
    for (x, y) in [(30.5, 62.5), (42.5, 62.5), (30.5, 14.5), (42.5, 14.5)] {
        c.deco(DecorKind::Lantern, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Tod eines Königs",
        916,
        "Ermittlung (Heldenmission)",
        "König Aldric III. ist tot – vergiftet, sagt der Hof. Sucht im Flammendom von Kaldris die Spuren und entscheidet, wen Ihr beschuldigt.",
        "Kaldris, Jahr 916 n.F. König Aldric III. wurde in der Nacht vergiftet. Der Hof schweigt, die Herzöge ziehen ihre Banner zusammen, und Kassian Vale ist verschwunden. Ihr habt keine Armee, nur Isolde Raventhorn und eine kleine Eskorte – und drei Spuren: den Weinkeller, die Kapelle der Ewigen Flamme und die Krypta unter dem Dom. Je mehr Beweise Ihr findet, desto mehr Möglichkeiten habt Ihr: Wer die Wahrheit kennt, kann sie aussprechen. Wer sie nicht kennt, kann nur wählen, wem er glaubt.",
        "Die Glocken von Kaldris läuteten für einen toten König – und der Hof hörte zum ersten Mal zu, was die Wächterin der Aschmark zu sagen hatte.",
        "Die Spur verlor sich im Dunkel der Gassen. Kaldris hat den Mörder des Königs nie gefunden.",
        &["intrigue_shadows", "mystery_ruins"],
    );
    m.map_seed = 91617;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.88);
    m.placements = vec![
        hero_at(36.0, 66.5),
        group("swordsman", 0, 33.5, 65.0, 6),
        group("archer", 0, 38.5, 65.0, 4),
        group("halberdier", 0, 36.0, 68.5, 3),
        group("knight", 0, 30.5, 67.0, 2),
        group("priest", 0, 41.5, 67.5, 1),
        // Weinkeller.
        tagged(guard("swordsman", 1, 11.0, 58.0, 1), "wache1"),
        guard("swordsman", 1, 9.0, 56.0, 1),
        guard("archer", 1, 13.0, 60.5, 1),
        // Kapelle der Ewigen Flamme.
        tagged(guard("spearman", 1, 58.0, 56.0, 1), "wache2"),
        guard("spearman", 1, 60.0, 54.0, 2),
        guard("crossbowman", 1, 56.0, 58.5, 1),
        // Krypta unter dem Dom.
        tagged(guard("knight", 1, 36.0, 10.0, 1), "wache3"),
        guard("swordsman", 1, 33.0, 12.0, 2),
        guard("archer", 1, 39.5, 12.0, 2),
        guard("crossbowman", 1, 36.0, 14.5, 1),
    ];
    m.areas = vec![
        area("keller", "Der Weinkeller", 5, 53, 12, 10),
        area("kapelle", "Die Kapelle der Ewigen Flamme", 53, 51, 12, 10),
        area("krypta", "Die Krypta unter dem Dom", 29, 6, 14, 10),
        area("dom", "Der Flammendom", 28, 28, 16, 16),
    ];
    m.starts = vec![StartPos { owner: 0, x: 36.0, y: 66.0 }];
    m.objectives = vec![
        objective("obj_e1", "Untersucht den Weinkeller.", false),
        objective("obj_e2", "Untersucht die Kapelle der Ewigen Flamme.", false),
        objective("obj_e3", "Findet das Siegel in der Krypta unter dem Dom.", true),
        objective("obj_dom", "Tretet vor den Hof im Flammendom.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m17", "nacht", &[
        (Sp::Narrator, "Der König starb im Winter, und die Glocken von Kaldris waren die Einzigen, die es laut sagten. Im Flammendom brannten die Kerzen weiter, als hätte niemand ihnen Bescheid gegeben.", "gedämpft, schwer"),
        (Sp::Isolde, "Drei Spuren, ein Mörder. Der Wein, die Flamme, die Gruft. Wir gehen leise vor – wer hier Zeugen hat, hat auch Feinde.", "leise, konzentriert"),
    ]);
    s.note("m17", "nacht", "Kaldris, Nacht des Todes von König Aldric III.");
    let e1 = s.dlg("m17", "keller", &[(Sp::Isolde, "Schierling im Wein. Frisch angesetzt, und nur ein einziger Krug. Wer das getan hat, kannte seinen Geschmack – und wusste, dass er Gold trinkt, kein Wasser.", "kühl, analytisch")]);
    s.note("m17", "keller", "Im Weinkeller des Palasts.");
    let e2 = s.dlg("m17", "kapelle", &[(Sp::Isolde, "Ein Brief unter dem Altarstein, mit dem Siegel Nordmarks. Aber die Tinte ist zu frisch, das Wachs zu sauber. Das hat jemand für mich hingelegt.", "misstrauisch, leise")]);
    s.note("m17", "kapelle", "In der Kapelle der Ewigen Flamme.");
    let e3 = s.dlg("m17", "krypta", &[(Sp::Isolde, "Das Siegel der Sonnenhäresie, in den Stein der Gruft geritzt. Jemand hat dem König Gift gegeben – und die Hohlen haben es gewusst, bevor er starb.", "ernst, erschüttert")]);
    s.note("m17", "krypta", "In der Krypta unter dem Dom, optional.");
    let kassian_hint = s.dlg("m17", "hinweis", &[(Sp::Kassian, "Ein Wort aus der Dunkelheit, Wächterin: Die Krypta. Ihr werdet dort etwas finden, das ich Euch nicht beweisen konnte. Ich sehe Euch im Dom.", "flüsternd, aus dem Schatten")]);
    s.note("m17", "hinweis", "Nur wenn Kassian Isolde einen Gefallen schuldet (Entscheidung in Mission 6).");
    let court = s.dlg("m17", "hof", &[
        (Sp::Narrator, "Im Flammendom waren sie alle versammelt: die Herzöge, die Flammenwahrer, die Räte. Und in der Mitte der Sarg des Königs, leer – der Leichnam lag noch im Nebenraum, damit niemand ihn küsste.", "feierlich, kalt"),
        (Sp::Mireille, "Wächterin. Man sagt, Ihr wisst, wer es war. Sagt es – oder lasst uns dem Mörder zuprosten, so lange er bei uns sitzt.", "scharf, souverän"),
    ]);
    s.note("m17", "hof", "Im Flammendom vor dem versammelten Hof.");
    let r_kassian = vec![s.dlg("m17", "urteil_a", &[
        (Sp::Isolde, "Kassian Vale hat es gewusst. Er hat gewarnt, wo er hätte handeln müssen. Sucht ihn – und fragt ihn nach seiner Rolle.", "gemessen, hart"),
        (Sp::Mireille, "Ein Spion, der zu viel wusste. Wie vorhersehbar – und wie bequem.", "zufrieden, kalt"),
    ])];
    let r_mireille = vec![s.dlg("m17", "urteil_b", &[
        (Sp::Isolde, "Der Brief weist nach Nordmark. Herzogin Mireille, erklärt Euch.", "ruhig, schneidend"),
        (Sp::Mireille, "Ihr beschuldigt eine Frau, die fünfhundert Speere in Euren Dienst gestellt hat. Ihr werdet es bereuen.", "eisig, bedrohlich"),
    ])];
    let r_heretics = vec![s.dlg("m17", "urteil_c", &[
        (Sp::Isolde, "Es waren die Sonnenhäretiker. Ihre Zeichen sind im Stein der Gruft. Kaldris hat Verräter in den eigenen Mauern.", "klar, laut"),
        (Sp::Mireille, "Dann reinigt die Stadt. Aber lasst die Herzöge auch ruhig schlafen.", "abwägend"),
    ])];
    let r_truth = vec![s.dlg("m17", "urteil_d", &[
        (Sp::Isolde, "Ich kenne die ganze Wahrheit: Gift im Wein, ein falscher Brief aus Nordmark, das Siegel der Häresie – und einer, der sie alle vermittelte. Seht auf die, die von Verwirrung leben.", "ruhig, zwingend"),
        (Sp::Mireille, "Ihr habt mehr Mut, als ich Euch zutraute. Und mehr Beweise, als mir lieb ist.", "widerwillig respektvoll"),
    ])];
    s.note("m17", "urteil_a", "Beschuldigung: Kassian Vale.");
    s.note("m17", "urteil_b", "Beschuldigung: Herzogin Mireille.");
    s.note("m17", "urteil_c", "Beschuldigung: die Sonnenhäretiker.");
    s.note("m17", "urteil_d", "Die ganze Wahrheit (alle drei Beweise).");
    let blame = s.decision(
        "m17",
        "hof",
        "blame",
        "Wem gilt die Anklage?",
        (Sp::Hal, "Du darfst nur einen Namen nennen, Isolde. Es ist die Aufgabe einer Wächterin, zu entscheiden – und den Preis zu tragen.", "ernst, väterlich"),
        vec![
            opt("blame_kassian", "Kassian Vale beschuldigen", "Kassian wird zum Feind; die Hohlen lachen im Verborgenen.", &[], r_kassian),
            opt("blame_mireille", "Mireille Corvane beschuldigen", "Der Bürgerkrieg beginnt; Nordmark wird zum Gegner.", &[], r_mireille),
            opt("blame_heretics", "Die Sonnenhäretiker beschuldigen", "Die Stadt wird gesäubert – die Häresie geht in den Untergrund.", &[], r_heretics),
            opt("blame_truth", "Die ganze Wahrheit sagen", "Nur mit allen drei Beweisen: Der Hof muss Euch folgen, und die Häretiker haben nichts mehr zu verbergen.", &["evid_3"], r_truth),
        ],
    );
    m.triggers = vec![
        trig("nacht", Condition::AtTime(1.0), vec![intro, add_obj("obj_e1"), add_obj("obj_e2"), add_obj("obj_e3"), add_obj("obj_dom"), add_obj("obj_hero"), chron("Isolde Raventhorn untersuchte den Tod König Aldrics III.")]),
        trig("keller", in_area("keller", 1), vec![Action::SetFlag("evid_1".into()), done_obj("obj_e1"), e1]),
        trig("kapelle", in_area("kapelle", 1), vec![Action::SetFlag("evid_2".into()), done_obj("obj_e2"), e2]),
        trig("krypta", in_area("krypta", 1), vec![Action::SetFlag("evid_3".into()), done_obj("obj_e3"), e3]),
        trig("dom", all_of(vec![in_area("dom", 1), Condition::ObjectiveCompleted("obj_e1".into()), Condition::ObjectiveCompleted("obj_e2".into())]), vec![done_obj("obj_dom"), court, blame]),
        trig("sieg", any_flag(&["blame_kassian", "blame_mireille", "blame_heretics", "blame_truth"]), vec![done_obj("obj_hero"), chron("Isolde Raventhorn sprach den Namen des Schuldigen aus."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel in den Gassen von Kaldris."),
    ];
    let mut v1 = variant(&["kassian_leverage"], &[], "Kassian, der Euch einen Gefallen schuldet, hat den Hinweis auf die Krypta hinterlassen: Der Beweis aus der Gruft steht Euch bereits offen.");
    v1.triggers = vec![at("hinweis", 4.0, vec![kassian_hint, Action::SetFlag("evid_3".into()), done_obj("obj_e3")])];
    v1.remove_tags = vec!["wache3".into()];
    let mut v2 = variant(&["kassian_warned_king"], &[], "Der König hat vor seinem Tod noch einen Wachtrupp zu Eurer Verfügung gestellt.");
    v2.placements = vec![group("knight", 0, 36.0, 69.0, 2), group("crossbowman", 0, 32.0, 68.0, 2)];
    let mut v3 = variant(&["oath_corvane"], &[], "Mireille Corvane ist die Gastgeberin: Nordmarker Wachen ergänzen Eure Eskorte.");
    v3.placements = vec![group("spearman", 0, 40.0, 68.0, 3)];
    let mut v4 = variant(&["kassian_silent"], &[], "Ohne Kassians Hinweis sind die Wachen der Krypta aufmerksam: Zusätzliche Häretiker sichern den Zugang.");
    v4.placements = vec![guard("swordsman", 1, 30.0, 18.0, 3), guard("crossbowman", 1, 42.0, 18.0, 2)];
    m.variants = vec![v1, v2, v3, v4];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter V – Mission 19: Die Furt von Kaldris (Feldschlacht)
// ---------------------------------------------------------------------------------------------

fn m19(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m19_kaldris_ford";
    s.title("m19", "Mission 19 – Die Furt von Kaldris");
    let mut c = Canvas::new(96, 72, 91719, GenPreset::Riverlands);
    c.flatten(0, 0, 95, 71, Terrain::Grass);
    let river = [(0.0, 35.0), (28.0, 37.0), (52.0, 33.0), (76.0, 35.0), (96.0, 34.0)];
    c.path(&river, 3.6, Terrain::Sand);
    c.path(&river, 2.4, Terrain::Water);
    c.path(&river, 1.3, Terrain::DeepWater);
    c.disc(22.0, 36.0, 3.6, Terrain::Ford);
    c.disc(48.0, 34.0, 3.8, Terrain::Ford);
    c.disc(74.0, 34.0, 3.6, Terrain::Ford);
    c.forest(0, 4, 14, 26, 3);
    c.forest(80, 46, 95, 68, 3);
    c.path(&[(48.0, 68.0), (48.0, 34.0), (48.0, 4.0)], 1.4, Terrain::Road);
    for (x, y) in [(30.5, 18.5), (66.5, 18.5), (30.5, 54.5), (66.5, 54.5)] {
        c.deco(DecorKind::Lantern, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Die Furt von Kaldris",
        917,
        "Feldschlacht",
        "Die Entscheidungsschlacht um den Thron: Zwei Heere stehen sich an der Furt vor Kaldris gegenüber. Nehmt Partei – und schlagt die Schlacht.",
        "Kaldris, Jahr 917 n.F. Der Erbfolgekrieg ist da. Herzogin Mireille Corvane steht mit Nordmarks Heer vor den Toren; in Kaldris halten die Getreuen des jungen Prinzen Aldren die Mauern, und vor beiden lagert die Aschenlegion, die den Krieg zu nutzen weiß. Ihr führt das größte Heer der Kampagne bis hierher, mit Isolde Raventhorn und Hal Venn an der Spitze. Zu Beginn wählt Ihr Partei: Mireilles Heer steht entweder an Eurer Seite oder Euch gegenüber. Schlagt den Feind an der Furt – haltet Euer Zentrum, setzt die Reiter an die Flügel.",
        "Das Heer des Feindes zerbrach an der Furt. Als der Rauch sich lichtete, wehten auf beiden Ufern Fahnen – aber nur noch eine sprach für den Thron.",
        "Die Furt von Kaldris färbte sich rot, und der Erbfolgekrieg ging weiter – ohne die Wächterin.",
        &["battle_large", "faction_ember_crown_battle"],
    );
    m.map_seed = 91719;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.44);
    m.placements = vec![
        hero_at(48.0, 60.0),
        tagged(pl("hero_hal_venn", 0, 46.0, 62.0), "hero_hal"),
        group("swordsman", 0, 42.0, 56.0, 12),
        group("halberdier", 0, 48.0, 54.0, 8),
        group("crossbowman", 0, 46.0, 63.0, 8),
        group("archer", 0, 52.0, 62.0, 6),
        group("knight", 0, 36.0, 58.0, 4),
        group("knight", 0, 60.0, 58.0, 4),
        group("priest", 0, 50.0, 66.0, 2),
        // Der Feind jenseits der Furt: Legion und Sonnenhäretiker.
        tagged(guard("ashen_legionnaire", 1, 48.0, 22.0, 1), "marshal"),
        guard("ashen_legionnaire", 1, 44.0, 20.0, 4),
        guard("ashen_legionnaire", 1, 52.0, 20.0, 4),
        guard("hollow_archer", 1, 44.0, 14.0, 4),
        guard("hollow_archer", 1, 52.0, 14.0, 4),
        guard("swordsman", 1, 38.0, 24.0, 5),
        guard("swordsman", 1, 58.0, 24.0, 5),
        guard("knight", 1, 30.0, 20.0, 2),
        guard("knight", 1, 66.0, 20.0, 2),
        guard("ash_witch", 1, 48.0, 12.0, 1),
    ];
    m.areas = vec![
        area("lager", "Das Lager", 30, 52, 36, 16),
        area("furt", "Die Furt", 44, 30, 8, 8),
        area("feind", "Das Heer des Feindes", 28, 8, 40, 20),
        area("bund", "Hinter Mireilles Linien", 20, 50, 10, 10),
        area("bund_ost", "Östlicher Flügel", 66, 50, 10, 10),
        area("rebellen", "Die Rebellen vor dem Fluss", 24, 12, 12, 8),
        area("rebellen_ost", "Rebellen im Osten", 60, 12, 12, 8),
    ];
    m.starts = vec![StartPos { owner: 0, x: 48.0, y: 60.0 }, StartPos { owner: 1, x: 48.0, y: 18.0 }];
    m.objectives = vec![
        objective("obj_side", "Wählt Partei im Erbfolgekrieg.", false),
        objective("obj_win", "Besiegt das Heer des Feindes an der Furt.", false),
        objective("obj_marshal", "Erschlagt den Marschall der Legion.", true),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m19", "thron", &[
        (Sp::Aldric, "Isolde. Kommt näher. Meine Augen … sind nicht mehr, was sie waren.", "sterbend, flach, mit Atempausen"),
        (Sp::Isolde, "Ich bin hier, Majestät. Ruht Euch aus.", "sanft, ungewohnt weich"),
        (Sp::Aldric, "Mireille steht vor den Toren. Nimmt sie den Thron mit dem Schwert, glaubt jeder Herzog, er dürfe es auch. Valemark wird sich zerfleischen.", "ernst, leise, schwer atmend"),
        (Sp::Mireille, "Öffnet die Tore, Wächterin! Ich bin nicht gekommen, einen Sterbenden zu töten. Ich bin gekommen, ein sterbendes Königreich zu retten!", "laut, fordernd, vom Feld"),
    ]);
    s.note("m19", "thron", "Vor der Schlacht: Aldrics Sterbebett, Mireille vor den Toren.");
    let mireille = vec![
        s.dlg("m19", "thron_a", &[
            (Sp::Isolde, "Öffnet die Tore. Valemark braucht eine starke Hand mehr als eine saubere.", "bitter, entschlossen"),
            (Sp::Mireille, "Danke, Wächterin. Die Geschichte wird es Verrat oder Weisheit nennen. Sorgen wir dafür, dass es Weisheit wird.", "ruhig, mit Würde"),
            (Sp::Aldric, "So sei es … Haltet … die Feuer am Brennen …", "letzte Worte, verhauchend"),
        ]),
        sp_units("knight", 0, "bund", 6, false, None),
        sp_units("swordsman", 0, "bund", 8, false, None),
        sp_units("crossbowman", 0, "bund", 6, false, None),
        sp_units("knight", 1, "rebellen", 4, true, None),
        sp_units("halberdier", 1, "rebellen_ost", 6, true, None),
    ];
    let aldric = vec![
        s.dlg("m19", "thron_b", &[
            (Sp::Isolde, "Die Tore bleiben zu. Die Krone geht nach Gesetz über, nicht durch Belagerung. Besetzt die Mauern.", "klar, unerbittlich"),
            (Sp::Mireille, "Dann habt Ihr eine Leiche einem Land vorgezogen. Wir sehen uns auf dem Feld, Wächterin.", "kalt, enttäuscht"),
            (Sp::Aldric, "Euer Vater … wäre stolz. Es tut mir … leid, dass es so viel kostet.", "dankbar, schwach"),
        ]),
        sp_units("halberdier", 0, "bund", 6, false, None),
        sp_units("knight", 0, "bund_ost", 4, false, None),
        sp_units("crossbowman", 0, "bund_ost", 6, false, None),
        sp_units("knight", 1, "rebellen", 6, true, None),
        sp_units("swordsman", 1, "rebellen_ost", 6, true, None),
        sp_units("crossbowman", 1, "rebellen_ost", 4, true, None),
    ];
    s.note("m19", "thron_a", "Entscheidung: Mireille stützen.");
    s.note("m19", "thron_b", "Entscheidung: Aldrens Linie verteidigen.");
    let side = s.decision(
        "m19",
        "thron",
        "throne",
        "Der Thron von Valemark",
        (Sp::Hal, "Mireille oder die Linie des Königs, Isolde. Beides hat seinen Preis – und beides wird Menschen töten.", "dunkel, ernst"),
        vec![
            opt("throne_mireille", "Mireille Corvane stützen", "Nordmarks Heer kämpft an Eurer Seite; die Getreuen Aldrens stellen sich Euch entgegen.", &[], mireille),
            opt("throne_aldren", "Aldrens Linie verteidigen", "Die Königsgarde kämpft für Euch; Mireilles Heer stellt sich Euch entgegen.", &[], aldric),
        ],
    );
    let start = s.dlg("m19", "schlacht", &[
        (Sp::Narrator, "Zwei Heere an einem Fluss. Das Wasser war flach, die Furt breit, und auf beiden Ufern standen Menschen, die mit demselben Gebet in dieselbe Schlacht zogen.", "ruhig, schwer"),
        (Sp::Isolde, "Haltet die Furt! Die Reiter an die Flanken, Armbrüste in die Mitte – und lasst sie kommen!", "laut, kommandierend"),
    ]);
    s.note("m19", "schlacht", "Beginn der Schlacht.");
    let fin = s.dlg("m19", "sieg", &[(Sp::Narrator, "Als der Rauch sich über der Furt lichtete, trug der Fluss seine Toten in die Ebene. Das Banner der Raventhorns wehte im Zentrum, und die Herzöge zählten ihre Schwüre noch einmal nach.", "ruhig, gewichtig")]);
    s.note("m19", "sieg", "Schlusswort.");
    m.triggers = vec![
        trig("thron", Condition::AtTime(1.0), vec![intro, side, add_obj("obj_side"), add_obj("obj_hero")]),
        trig("schlacht", any_flag(&["throne_mireille", "throne_aldren"]), vec![done_obj("obj_side"), add_obj("obj_win"), add_obj("obj_marshal"), start, chron("An der Furt von Kaldris wurde die Entscheidungsschlacht des Erbfolgekriegs geschlagen.")]),
        trig("marschall", tag_dead("marshal"), vec![done_obj("obj_marshal"), Action::Message { speaker: "Isolde Raventhorn".into(), text: "Der Marschall ist gefallen! Jetzt drängt nach!".into(), voice: None }]),
        trig("sieg", all_of(vec![any_flag(&["throne_mireille", "throne_aldren"]), Condition::Any(vec![all_of(vec![Condition::AtTime(60.0), Condition::EnemyUnitsAtMost(4)]), tag_dead("marshal")])]), vec![done_obj("obj_win"), done_obj("obj_hero"), fin, chron("Die Furt von Kaldris fiel an die Wächterin."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel an der Furt von Kaldris."),
    ];
    let mut v1 = variant(&["blame_truth"], &[], "Der Hof folgt der Wahrheit: Zeugen des Mordes stärken die Moral Eurer Truppen, Priester heilen im Lager.");
    v1.placements = vec![group("priest", 0, 44.0, 66.0, 3)];
    let mut v2 = variant(&["blame_heretics"], &[], "Die Häretiker fürchten die Säuberung: Weniger Hexen stehen im Feindheer.");
    v2.remove_tags = vec![];
    v2.placements = vec![group("crossbowman", 0, 54.0, 66.0, 4)];
    let mut v3 = variant(&["blame_mireille"], &[], "Mireille ist Euch aufs Äußerste verfeindet: Ihre Reiter sind verstärkt.");
    v3.placements = vec![guard("knight", 1, 40.0, 14.0, 3)];
    let mut v4 = variant(&["blame_kassian"], &[], "Kassian Vale arbeitet im Verborgenen gegen Euch: Zusätzliche Armbrustschützen stehen auf der Gegenseite.");
    v4.placements = vec![guard("crossbowman", 1, 56.0, 14.0, 4)];
    let mut v5 = variant(&["kassian_warned_king"], &[], "Die Königsgarde, die Aldric nach der Warnung aufgestellt hat, steht in Eurem Zentrum.");
    v5.placements = vec![group("halberdier", 0, 52.0, 56.0, 4)];
    let mut v6 = variant(&["torvald_blood"], &[], "Torvald Eisenhand ist aus Skarnheim gekommen, den Schwur zu erfüllen.");
    v6.placements = vec![tagged(pl("hero_torvald", 0, 54.0, 58.0), "torvald"), group("shieldbearer", 0, 56.0, 60.0, 6)];
    m.variants = vec![v1, v2, v3, v4, v5, v6];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter VI – Mission 21: Der Schrein Oruuns (Stollen und Boss Gharun)
// ---------------------------------------------------------------------------------------------

fn m21(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m21_shrine_oruun";
    s.title("m21", "Mission 21 – Der Schrein Oruuns");
    let mut c = Canvas::new(88, 88, 92021, GenPreset::Highlands);
    c.flatten(0, 0, 87, 87, Terrain::Ash);
    c.flatten(21, 5, 67, 83, Terrain::Stone);
    // Der Stollen: Außenmauer und zwei Quermauern mit Durchgängen.
    boulder_line(&mut c, 20, 4, 68, 4, &[]);
    boulder_line(&mut c, 20, 84, 68, 84, &[]);
    boulder_line(&mut c, 20, 4, 20, 84, &[]);
    boulder_line(&mut c, 68, 4, 68, 84, &[]);
    boulder_line(&mut c, 21, 58, 67, 58, &[(40, 47)]);
    boulder_line(&mut c, 21, 32, 67, 32, &[(38, 49)]);
    for (x, y) in [(28.5, 70.5), (60.5, 70.5), (28.5, 44.5), (60.5, 44.5), (30.5, 16.5), (58.5, 16.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    for i in 0..7 {
        c.deco(DecorKind::Pillar, 31.0 + i as f32 * 4.0, 10.5);
    }
    for (x, y) in [(26.5, 62.5), (62.5, 62.5), (26.5, 38.5), (62.5, 38.5)] {
        c.deco(DecorKind::Bones, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Der Schrein Oruuns",
        920,
        "Stollen und Bosskampf",
        "Unter den Gipfeln Skarnheims liegt der Schrein des Steingotts Oruun – von Gharun, dem Steinernen Eid, in Besitz genommen. Kämpft Euch durch die Hallen und weiht den Schrein neu.",
        "Skarnheim, Jahr 920 n.F. Die Götter schweigen seit dem Aschenfall, doch ihre Schreine haben Wächter. Der Wächter von Oruuns Schrein heißt Gharun und war einst ein Eid, den die Ahnen in Runenstein gebunden haben: Er sollte den Schrein hüten. Das Hohle Tief hat den Eid verdreht – nun hütet er das Tiefe. Ihr kämpft Euch durch drei Hallen: ein Vorraum, eine Halle der Ahnen und der Schreinsaal. Wenn Torvald Eisenhand den Blutschwur gehalten hat, steht er an Eurer Seite; sonst stellen Söldner den Ersatz.",
        "Gharun zerfiel zu Staub, und der Eid fand Frieden. Im Schreinsaal sprach Oruun nicht – aber der Berg hörte auf, zu zittern.",
        "Gharun hielt seinen verdrehten Eid. Die Halle blieb stumm, der Berg zitterte weiter.",
        &["boss_stone_oath", "mystery_old_gods"],
    );
    m.map_seed = 92021;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.55);
    m.placements = vec![
        hero_at(44.0, 78.0),
        group("swordsman", 0, 38.0, 77.0, 10),
        group("halberdier", 0, 50.0, 77.0, 6),
        group("crossbowman", 0, 44.0, 81.0, 8),
        group("shieldbearer", 0, 44.0, 74.0, 4),
        group("priest", 0, 40.0, 81.0, 2),
        // Vorraum.
        guard("hollow_thrall", 1, 34.0, 66.0, 5),
        guard("ghoul", 1, 52.0, 66.0, 3),
        guard("hollow_archer", 1, 44.0, 63.0, 4),
        // Halle der Ahnen.
        guard("hollow_thrall", 1, 32.0, 46.0, 6),
        guard("hollow_thrall", 1, 56.0, 46.0, 6),
        guard("ash_witch", 1, 44.0, 40.0, 2),
        guard("grave_brute", 1, 44.0, 44.0, 2),
        // Der Schreinsaal.
        tagged(pl("hollow_shrine", 1, 43.0, 12.0), "oruun_schrein"),
        tagged(guard("boss_stone_oath", 1, 44.0, 20.0, 1), "boss"),
        guard("ghoul", 1, 34.0, 18.0, 4),
        guard("ghoul", 1, 54.0, 18.0, 4),
        guard("ash_witch", 1, 44.0, 14.0, 2),
    ];
    m.areas = vec![
        area("eingang", "Der Eingang", 24, 70, 40, 12),
        area("vorraum", "Der Vorraum", 24, 59, 40, 12),
        area("ahnenhalle", "Die Halle der Ahnen", 24, 33, 40, 24),
        area("schreinsaal", "Der Schreinsaal", 24, 6, 40, 24),
    ];
    m.starts = vec![StartPos { owner: 0, x: 44.0, y: 78.0 }, StartPos { owner: 1, x: 44.0, y: 18.0 }];
    m.objectives = vec![
        objective("obj_halls", "Kämpft Euch durch den Vorraum und die Halle der Ahnen.", false),
        objective("obj_gharun", "Bezwingt Gharun, den Steinernen Eid.", false),
        objective("obj_shrine", "Weiht den Schrein Oruuns neu.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m21", "stollen", &[
        (Sp::Narrator, "Der Berg hatte seit dem Aschenfall nicht mehr gesprochen. Wer an seiner Flanke stand, spürte ihn manchmal zittern – wie ein Schlafender, der etwas Schlimmes träumt.", "tief, ruhig"),
        (Sp::Isolde, "Der Weg führt durch drei Hallen. Haltet die Reihen geschlossen, und wenn der Stein spricht: nicht antworten.", "leise, kontrolliert"),
    ]);
    s.note("m21", "stollen", "Der Eingang des Stollens unter Skarnheims Gipfeln.");
    let torvald = s.dlg("m21", "torvald", &[(Sp::Torvald, "Meine Väter haben diesen Schrein mit eigenen Händen gebaut. Dass ich ihn nun mit der Axt zurückfordern muss, ist eine Schande, die ich gern wasche.", "grimmig, ehrfürchtig")]);
    s.note("m21", "torvald", "Nur wenn Torvald den Blutschwur gehalten hat.");
    let mercs = s.dlg("m21", "soeldner", &[(Sp::Isolde, "Gold kauft Schwerter, aber keine Trauer. Dieser Berg ist nicht meiner – und doch kämpfe ich um ihn.", "nachdenklich, trocken")]);
    s.note("m21", "soeldner", "Nur wenn Torvald nicht mitkämpft (Gold-Variante).");
    let boss = s.dlg("m21", "gharun", &[
        (Sp::Gharun, "Wer wagt es, den Eid zu stören? Ich wurde gebunden, zu bewahren – und ich bewahre. Was ich bewahre, spricht nicht mehr mit euch.", "steinern, langsam, dröhnend, hallend"),
        (Sp::Isolde, "Du bewahrst den Falschen, Wächter. Wir sind gekommen, den Schrein zurückzuholen – und deinen Eid zu lösen.", "ruhig, gegenüber einem Berg"),
    ]);
    s.note("m21", "gharun", "Isolde und Gharun im Schreinsaal.");
    let fin = s.dlg("m21", "sieg", &[
        (Sp::Narrator, "Gharun zerfiel zu Staub, der in der Luft stand, als wollte er sich noch einmal verbeugen. Der Eid war nicht gebrochen, sondern erfüllt: Der Schrein war in Sicherheit.", "ruhig, gedämpft"),
        (Sp::Isolde, "Oruun, wenn du zuhörst: Der Schrein ist dein. Wir kommen wieder, wenn du uns brauchst.", "leise, ehrfürchtig"),
    ]);
    s.note("m21", "sieg", "Der Schrein wird neu geweiht.");
    m.triggers = vec![
        trig("stollen", Condition::AtTime(1.0), vec![intro, add_obj("obj_halls"), add_obj("obj_gharun"), add_obj("obj_shrine"), add_obj("obj_hero"), chron("Isolde Raventhorn drang in den Stollen unter den Gipfeln Skarnheims ein.")]),
        trig("torvald", all_of(vec![flag("torvald_blood"), Condition::AtTime(8.0)]), vec![torvald]),
        trig("soeldner", all_of(vec![flag("torvald_gold"), Condition::AtTime(8.0)]), vec![mercs]),
        trig("hallen", in_area("schreinsaal", 1), vec![done_obj("obj_halls"), boss, Action::SetWeather(ashen_sim::env::Weather::Fog)]),
        trig("beben", Condition::TagHealthBelow { tag: "boss".into(), percent: 50.0 }, vec![chron("Der Berg bebte unter dem Streit um Oruuns Schrein."), sp_units("ghoul", 1, "schreinsaal", 3, true, None)]),
        trig("sieg", tag_dead("boss"), vec![done_obj("obj_gharun"), done_obj("obj_shrine"), done_obj("obj_hero"), fin, chron("Der Schrein Oruuns wurde neu geweiht."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel im Schreinsaal Oruuns."),
    ];
    let mut v1 = variant(&["torvald_blood"], &[], "Torvald Eisenhand und seine Clankrieger kämpfen an Eurer Seite.");
    v1.placements = vec![tagged(pl("hero_torvald", 0, 40.0, 79.0), "torvald"), group("shieldbearer", 0, 48.0, 79.0, 6), group("halberdier", 0, 36.0, 74.0, 4)];
    let mut v2 = variant(&["torvald_gold"], &[], "Statt des Clanfürsten stehen gedungene Söldner im Vorraum.");
    v2.placements = vec![group("crossbowman", 0, 40.0, 79.0, 6), group("halberdier", 0, 48.0, 79.0, 4)];
    let mut v3 = variant(&["rootmother_pact"], &[], "Der Dornenbund hat Euch Wurzelheiler mitgegeben: Priester heilen im Gefecht.");
    v3.placements = vec![group("priest", 0, 44.0, 83.0, 3)];
    let mut v4 = variant(&["rootmother_burned"], &[], "Der Dornenbund versagt Euch die Hilfe; das Tiefe verstärkt das Nest im Vorraum.");
    v4.placements = vec![guard("hollow_thrall", 1, 40.0, 62.0, 5), guard("ghoul", 1, 48.0, 62.0, 2)];
    let mut v5 = variant(&["core_kept"], &[], "Das Aschglas aus Vael Dorn ruft die Hohlen: Zusätzliche Hexen bewachen die Ahnenhalle.");
    v5.placements = vec![guard("ash_witch", 1, 36.0, 38.0, 2), guard("ash_witch", 1, 52.0, 38.0, 1)];
    m.variants = vec![v1, v2, v3, v4, v5];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter VI – Mission 25: Ithrens letztes Licht (Erkundung in Solmere)
// ---------------------------------------------------------------------------------------------

fn m25(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m25_ithren_light";
    s.title("m25", "Mission 25 – Ithrens letztes Licht");
    let mut c = Canvas::new(96, 96, 92225, GenPreset::Ashlands);
    c.flatten(0, 0, 95, 95, Terrain::Ash);
    c.flatten(2, 76, 26, 94, Terrain::Sand);
    c.flatten(60, 4, 90, 30, Terrain::Stone);
    c.path(&[(14.0, 84.0), (30.0, 74.0), (44.0, 62.0), (56.0, 48.0), (66.0, 34.0), (74.0, 18.0)], 2.2, Terrain::Road);
    for i in 0..14 {
        let t = i as f32;
        c.deco(DecorKind::Pillar, 12.0 + (t * 29.0) % 70.0, 20.0 + (t * 17.0) % 56.0);
    }
    for i in 0..10 {
        let t = i as f32;
        c.deco(DecorKind::Ruin, 8.0 + (t * 31.0) % 80.0, 10.0 + (t * 41.0) % 76.0);
    }
    for i in 0..8 {
        let a = i as f32 / 8.0 * std::f32::consts::TAU;
        c.deco(DecorKind::Brazier, 75.0 + a.cos() * 9.0, 17.0 + a.sin() * 9.0);
    }
    let mut m = campaign_mission(
        id,
        "Ithrens letztes Licht",
        922,
        "Erkundung in den Außenbezirken von Solmere",
        "Unter dem Sonnenhof von Solmere liegt der Schrein Ithrens begraben. Durchquert die Ruinen, brecht die Wache der Sonnenhäretiker und lest Varkhans Tagebuch.",
        "Solmere, Außenbezirke, Jahr 922 n.F. Die Hauptstadt des Reiches Aureth liegt seit tausend Jahren in Trümmern. Unter dem Sonnenhof soll der Schrein Ithrens liegen, des Gottes des Lichts. Die Sonnenhäretiker kontrollieren die Ruinen – sie glauben, Varkhan Aurelith werde als Erlöser zurückkehren. Ihr habt keine Basis, nur Isolde Raventhorn, Hal Venn und einen Trupp. Durchquert die Ruinen und erreicht den Schrein. Was Ihr dort findet, könnte die Wahrheit über Varkhan ändern – und Ihr entscheidet, was Ihr damit tut.",
        "Der Schrein Ithrens glomm einen Atemzug lang auf. Dann war es wieder still – aber im Stein stand, was geschehen war.",
        "Solmere behielt seine Geheimnisse und Isolde Raventhorn ihr Grab in den Ruinen.",
        &["mystery_ruins", "story_revelation"],
    );
    m.map_seed = 92225;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.36);
    m.placements = vec![
        hero_at(14.0, 84.0),
        tagged(pl("hero_hal_venn", 0, 16.0, 85.5), "hero_hal"),
        group("swordsman", 0, 12.0, 82.0, 6),
        group("halberdier", 0, 18.0, 82.0, 4),
        group("crossbowman", 0, 14.0, 88.0, 5),
        group("priest", 0, 18.0, 88.0, 2),
        group("knight", 0, 10.0, 86.0, 2),
        // Wachen der Sonnenhäretiker.
        guard("swordsman", 1, 30.0, 72.0, 4),
        guard("archer", 1, 33.0, 75.0, 3),
        guard("spearman", 1, 46.0, 60.0, 4),
        guard("crossbowman", 1, 49.0, 63.0, 3),
        guard("ashen_legionnaire", 1, 58.0, 46.0, 4),
        guard("hollow_archer", 1, 60.0, 49.0, 3),
        tagged(guard("knight", 1, 74.0, 18.0, 1), "sun_guard"),
        guard("swordsman", 1, 70.0, 22.0, 4),
        guard("crossbowman", 1, 78.0, 20.0, 3),
        guard("ash_witch", 1, 75.0, 12.0, 2),
        tagged(guard("swordsman", 1, 40.0, 70.0, 4), "heretics"),
    ];
    m.areas = vec![
        area("lager", "Am Stadtrand", 2, 76, 24, 18),
        area("allee", "Die Allee der Sonne", 40, 54, 24, 20),
        area("schrein", "Der Schrein Ithrens", 66, 8, 20, 20),
    ];
    m.starts = vec![StartPos { owner: 0, x: 14.0, y: 84.0 }, StartPos { owner: 1, x: 74.0, y: 18.0 }];
    m.objectives = vec![
        objective("obj_shrine", "Erreicht den Schrein Ithrens unter dem Sonnenhof.", false),
        objective("obj_guard", "Bezwingt den Sonnenwächter des Schreins.", false),
        objective("obj_truth", "Entscheidet über Varkhans Tagebuch.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m25", "ruinen", &[
        (Sp::Narrator, "Solmere war einst eine Stadt aus Licht gewesen, Terrassen aus weißem Stein bis an den Himmel. Nun waren es Rippen, in denen der Wind klagte – und zwischen den Rippen standen Männer mit Sonnen auf den Brustpanzern, die auf einen toten König warteten.", "ehrfürchtig, schwermütig"),
        (Sp::Hal, "Sei vorsichtig, Isolde. Die Sonnenhäretiker töten Fremde, aber sie töten sie mit Gebeten. Das macht es nicht besser.", "trocken, besorgt"),
    ]);
    s.note("m25", "ruinen", "Am Stadtrand von Solmere.");
    let shrine = s.dlg("m25", "tagebuch", &[
        (Sp::Narrator, "Unter den Resten des Sonnenhofs lag das Licht Ithrens, kaum größer als eine Kerzenflamme. Daneben, in einer Nische aus Gold, ein Buch, dessen Seiten nach Rauch und Rosenwasser rochen.", "leise, ehrfürchtig"),
        (Sp::Hal, "Varkhans Tagebuch. Er schreibt … von seiner Tochter. Sie starb an der Schwindsucht, und er wollte … er wollte nicht den Tod besiegen, Isolde. Er wollte sie.", "betroffen, leise"),
        (Sp::Deep, "Ein Vater, der alles verbrannte, um ein Kind zu retten. So rührend. So nützlich.", "samtig, süß, bedrohlich"),
    ]);
    s.note("m25", "tagebuch", "Am Schrein Ithrens: Varkhans Tagebuch.");
    let truth = vec![s.dlg("m25", "urteil_a", &[
        (Sp::Isolde, "Die Wahrheit gehört den Menschen. Lasst sie lesen, was Varkhan getan hat – und warum. Wer ihn dann noch erlösen will, soll es wissen.", "ruhig, klar"),
        (Sp::Hal, "Das wird die Häresie spalten. Aber vielleicht ist das besser, als wenn sie an einer Lüge zerbricht.", "bedächtig")])];
    let faith = vec![s.dlg("m25", "urteil_b", &[
        (Sp::Isolde, "Sie glauben an etwas, das sie am Leben hält. Ich nehme ihnen das nicht, bevor ich es ersetzen kann.", "leise, widerwillig"),
        (Sp::Hal, "Gnade und Feigheit sind manchmal Geschwister. Ich hoffe, du weißt, welche du gerade wählst.", "sanft, mahnend")])];
    let used = vec![s.dlg("m25", "urteil_c", &[
        (Sp::Isolde, "Sie glauben, er werde zurückkehren. Gut. Sie sollen für ihn kämpfen – bis ich ihn selbst stelle.", "kalt, berechnend"),
        (Sp::Deep, "Wie praktisch du bist, kleine Flamme. Wie ähnlich uns.", "leise, amüsiert")])];
    s.note("m25", "urteil_a", "Entscheidung: Die Wahrheit veröffentlichen.");
    s.note("m25", "urteil_b", "Entscheidung: Den Glauben der Häretiker lassen.");
    s.note("m25", "urteil_c", "Entscheidung: Die Häretiker für den eigenen Feldzug nutzen.");
    let sun = s.decision(
        "m25",
        "tagebuch",
        "sun",
        "Das Tagebuch des Sonnenkönigs",
        (Sp::Hal, "Es liegt bei dir, Isolde. Das Buch enthält die Wahrheit, und die Wahrheit hat Folgen.", "ernst, bedächtig"),
        vec![
            opt("sun_truth", "Die Wahrheit offenlegen", "Die Häresie zerbricht; ihre Anhänger stehen später nicht mehr unter Waffen.", &[], truth),
            opt("sun_faith", "Den Glauben bewahren", "Die Häretiker stehen den Toren Solmeres wohlgesonnen gegenüber.", &[], faith),
            opt("sun_used", "Die Häretiker nutzen", "Die Häretiker kämpfen später an Eurer Seite – bis die Wahrheit sie einholt.", &[], used),
        ],
    );
    let fin = s.dlg("m25", "sieg", &[(Sp::Narrator, "Das Licht Ithrens erlosch nicht. Es wartete, mit der Geduld der Götter, auf jemanden, der bereit war, es zu tragen.", "leise, hoffnungsvoll")]);
    s.note("m25", "sieg", "Schlusswort.");
    m.triggers = vec![
        trig("ruinen", Condition::AtTime(1.0), vec![intro, add_obj("obj_shrine"), add_obj("obj_guard"), add_obj("obj_truth"), add_obj("obj_hero"), chron("Isolde Raventhorn drang in die Ruinen von Solmere vor.")]),
        trig("wache", tag_dead("sun_guard"), vec![done_obj("obj_guard")]),
        trig("schrein", all_of(vec![in_area("schrein", 1), tag_dead("sun_guard")]), vec![done_obj("obj_shrine"), shrine, sun]),
        trig("sieg", any_flag(&["sun_truth", "sun_faith", "sun_used"]), vec![done_obj("obj_truth"), done_obj("obj_hero"), fin, chron("Isolde Raventhorn las das Tagebuch des Sonnenkönigs."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel in den Ruinen von Solmere."),
    ];
    let mut v1 = variant(&["torvald_blood"], &[], "Torvald Eisenhand sichert mit seinen Clankriegern den Rückzug.");
    v1.placements = vec![tagged(pl("hero_torvald", 0, 12.0, 80.0), "torvald"), group("shieldbearer", 0, 14.0, 80.0, 4)];
    let mut v2 = variant(&["rootmother_pact"], &[], "Die Wurzelmutter schenkt Euch durch Sylaen verbündete Waldläufer.");
    v2.placements = vec![group("archer", 0, 18.0, 90.0, 5)];
    let mut v3 = variant(&["blame_truth"], &[], "Die Wahrheit über Aldrics Tod hat Euch Zeugen und Priester eingebracht.");
    v3.placements = vec![group("priest", 0, 20.0, 86.0, 2)];
    let mut v4 = variant(&["rift_harvested"], &[], "Die Hohlung ist in Euch gewachsen: Die Stimme aus dem Tief flüstert, und zusätzliche Legion bewacht die Allee.");
    v4.placements = vec![guard("ashen_legionnaire", 1, 52.0, 56.0, 4)];
    m.variants = vec![v1, v2, v3, v4];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter VII – Mission 26: Der Aschenmarsch (Halten oder Evakuieren)
// ---------------------------------------------------------------------------------------------

fn m26(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m26_ashen_march";
    s.title("m26", "Mission 26 – Der Aschenmarsch");
    let mut c = Canvas::new(96, 80, 92326, GenPreset::Aschmark);
    c.flatten(18, 12, 74, 66, Terrain::Dirt);
    c.flatten(24, 18, 68, 60, Terrain::Stone);
    c.path(&[(46.0, 60.0), (46.0, 79.0)], 1.8, Terrain::Road);
    c.path(&[(68.0, 40.0), (95.0, 40.0)], 1.8, Terrain::Road);
    c.path(&[(46.0, 2.0), (46.0, 18.0)], 1.8, Terrain::Road);
    c.path(&[(2.0, 38.0), (24.0, 38.0)], 1.8, Terrain::Road);
    c.node(NodeKind::GoldMine, 76, 14);
    c.node(NodeKind::StoneQuarry, 12, 64);
    c.forest(4, 4, 16, 28, 2);
    for (x, y) in [(30.5, 24.5), (62.5, 24.5), (30.5, 54.5), (62.5, 54.5), (46.5, 38.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Der Aschenmarsch",
        923,
        "Halten oder Evakuieren",
        "Die erwachte Aschenlegion marschiert auf Brennhollow. Haltet die Stadt – oder bringt die Bürger aus der Stadt, bevor die Legion sie erreicht.",
        "Brennhollow, Jahr 923 n.F. Aus der Siedlung ist eine Stadt geworden: Mauern, Märkte, zweitausend Bürger. Und nun kommt die Aschenlegion, in Reih und Glied, mit den Fahnen Aureths. Der Hohle König ist erwacht. Ihr habt zwei Möglichkeiten: Haltet Brennhollow bis zum Morgen – schwer, aber der Stadt Ehre – oder führt die Bürger durch das Osttor auf die Straße nach Kaldris, und überlasst die Stadt der Legion. Beides kostet etwas. Am Ende zählt, wer Euch folgt.",
        "Das Morgenlicht fand Brennhollow noch aufrecht. Die Legion wich, aber ihr Marsch war nicht zu Ende.",
        "Brennhollow fiel in Asche, und die Legion zog weiter nach Süden.",
        &["siege_defense", "faction_ashen_legion_battle"],
    );
    m.map_seed = 92326;
    m.start_resources = Cost { gold: 800.0, wood: 500.0, stone: 500.0, food: 500.0 };
    m.start_goods = vec![("weapons".into(), 30.0), ("armor".into(), 20.0), ("planks".into(), 40.0), ("tools".into(), 10.0)];
    m.defeat_tag = String::new();
    m.start_time = Some(0.70);
    let mut p = vec![
        tagged(pl("keep", 0, 44.0, 36.0), "player_keep"),
        pl("barracks", 0, 34.0, 28.0),
        pl("barracks", 0, 54.0, 28.0),
        pl("farmstead", 0, 33.0, 46.0),
        pl("farmstead", 0, 54.0, 46.0),
        pl("hut", 0, 30.0, 38.0),
        pl("hut", 0, 58.0, 38.0),
        pl("hut", 0, 38.0, 50.0),
        pl("house", 0, 48.0, 50.0),
        pl("watchtower", 0, 28.0, 22.0),
        pl("watchtower", 0, 62.0, 22.0),
        pl("watchtower", 0, 28.0, 56.0),
        pl("watchtower", 0, 62.0, 56.0),
        group("worker", 0, 46.0, 44.0, 24),
        group("swordsman", 0, 40.0, 33.0, 10),
        group("halberdier", 0, 50.0, 33.0, 8),
        group("crossbowman", 0, 46.0, 52.5, 8),
        group("archer", 0, 52.0, 53.0, 6),
        group("knight", 0, 56.0, 36.0, 4),
        group("priest", 0, 42.0, 40.0, 2),
        hero_at(46.0, 40.0),
        tagged(pl("hero_hal_venn", 0, 48.0, 41.0), "hero_hal"),
    ];
    wall_ring(&mut p, 24, 18, 68, 60, &[(46, 60), (68, 40)], &[(24, 18), (67, 18), (24, 59), (67, 59)]);
    m.placements = p;
    m.areas = vec![
        area("nord", "Nordstraße", 42, 2, 8, 5),
        area("west", "Weststraße", 2, 34, 5, 8),
        area("sued", "Südstraße", 42, 73, 8, 5),
        area("stadt", "Brennhollow", 24, 18, 45, 43),
        area("pass", "Die Straße nach Kaldris", 82, 32, 12, 16),
    ];
    m.starts = vec![StartPos { owner: 0, x: 46.0, y: 40.0 }, StartPos { owner: 1, x: 46.0, y: 4.0 }];
    m.objectives = vec![
        objective("obj_choice", "Entscheidet über das Schicksal Brennhollows.", false),
        objective("obj_hold", "Haltet Brennhollow bis zum Morgengrauen.", false),
        objective("obj_evac", "Führt mindestens zwölf Bürger durch das Osttor auf die Straße nach Kaldris.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m26", "marsch", &[
        (Sp::Narrator, "Sie kamen im Gleichschritt. Zum ersten Mal seit dem Aschenfall hörte man die Hohlen nicht durch die Nacht schlurfen, sondern marschieren – und der Boden der Aschmark antwortete mit einem Beben.", "schwer, getragen"),
        (Sp::Hal, "Es ist die Legion, Isolde. Die Wappen Aureths auf jeder Brust. Ich habe sie als Junge in den Liedern gesehen – nicht im Fleisch.", "bestürzt, leise"),
    ]);
    s.note("m26", "marsch", "Brennhollow, Abenddämmerung; die Aschenlegion naht.");
    let hold = vec![
        s.dlg("m26", "wahl_a", &[
            (Sp::Isolde, "Wir halten. Diese Mauern haben dreißig Jahre Feuer gesehen. Jeder Bürger auf die Wälle, jedes Kind in die Keller!", "laut, kommandierend"),
            (Sp::Hal, "Die Flamme steht mit Brennhollow. Sie hat es immer getan.", "fest, warm"),
        ]),
        Action::Grant(Cost { gold: 300.0, wood: 200.0, stone: 200.0, food: 0.0 }),
    ];
    let evac = vec![
        s.dlg("m26", "wahl_b", &[
            (Sp::Isolde, "Wir räumen die Stadt. Die Legion kann Steine erobern, aber keine Menschen. Das Osttor auf – alle zum Kaldris-Pass!", "kalt, schwer"),
            (Sp::Hal, "Ich habe diese Mauern mit eigenen Händen gesegnet. Es ist eine gute Entscheidung. Es wird nur lange wehtun.", "rau, gefasst"),
        ]),
    ];
    s.note("m26", "wahl_a", "Entscheidung: Brennhollow halten.");
    s.note("m26", "wahl_b", "Entscheidung: Brennhollow evakuieren.");
    let march = s.decision(
        "m26",
        "marsch",
        "march",
        "Halten oder Evakuieren",
        (Sp::Isolde, "Dies ist keine Entscheidung, die ich allein treffen will. Aber sie steht mir zu. Halten wir Brennhollow – oder retten wir die, die in ihm wohnen?", "schwer, nachdenklich"),
        vec![
            opt("march_hold", "Brennhollow halten", "Die Stadt kämpft bis zum Morgen; verliert sie, ist die Mission verloren.", &[], hold),
            opt("march_evacuate", "Die Bürger evakuieren", "Brennhollow fällt, aber die Menschen überleben und folgen Euch nach Solmere.", &[], evac),
        ],
    );
    let evac_start = s.dlg("m26", "flucht", &[(Sp::Isolde, "Haltet die Reihen, und lasst die Kinder zwischen den Wagen gehen! Die Soldaten decken den Rückzug – wer zurückbleibt, bleibt mit Ehre!", "laut, drängend")]);
    s.note("m26", "flucht", "Beginn der Evakuierung.");
    let dawn_hold = s.dlg("m26", "morgen", &[(Sp::Narrator, "Das Morgenlicht fand die Zinnen von Brennhollow zerschossen, aber nicht erstürmt. Die Legion zog sich zurück, dem Süden entgegen – ein Heer, das nicht müde wurde, aber wartete.", "ruhig, erschöpft")]);
    s.note("m26", "morgen", "Brennhollow hält bis zum Morgen.");
    let dawn_evac = s.dlg("m26", "pass", &[(Sp::Narrator, "Hinter ihnen brannte Brennhollow. Niemand sah sich um. Wer zurückblickte, hörte das Läuten der Kapelle – und erst später begriff man, dass sie von selbst läutete.", "leise, schmerzvoll")]);
    s.note("m26", "pass", "Die Evakuierung erreicht die Straße nach Kaldris.");
    let mk = |t: f32, id: &str, a: &str, u: &[(&str, u32)]| trig(id, Condition::AtTime(t), wave(a, u));
    m.triggers = vec![
        trig("marsch", Condition::AtTime(1.0), vec![intro, march, add_obj("obj_choice"), add_obj("obj_hero"), chron("Die Aschenlegion marschierte auf Brennhollow.")]),
        trig("hold", flag("march_hold"), vec![done_obj("obj_choice"), add_obj("obj_hold")]),
        trig("evac", flag("march_evacuate"), vec![done_obj("obj_choice"), add_obj("obj_evac"), evac_start]),
        mk(60.0, "w1", "nord", &[("ashen_legionnaire", 8), ("hollow_archer", 4)]),
        mk(120.0, "w2", "west", &[("ashen_legionnaire", 8), ("hollow_thrall", 8)]),
        mk(190.0, "w3", "sued", &[("ashen_legionnaire", 8), ("ash_witch", 2), ("grave_brute", 1)]),
        mk(260.0, "w4", "nord", &[("ashen_legionnaire", 10), ("hollow_archer", 6), ("ghoul", 4)]),
        mk(330.0, "w5", "west", &[("ashen_legionnaire", 10), ("grave_brute", 2), ("ash_witch", 2)]),
        mk(400.0, "w6", "sued", &[("ashen_legionnaire", 10), ("hollow_archer", 6), ("grave_brute", 2)]),
        trig("keep_lost", all_of(vec![flag("march_hold"), tag_dead("player_keep")]), vec![Action::Message { speaker: "Erzähler".into(), text: "Der Bergfried fiel. Brennhollow war verloren.".into(), voice: None }, Action::Defeat]),
        trig("civ_lost", all_of(vec![flag("march_evacuate"), Condition::AtTime(30.0), not(Condition::UnitCount { unit: "worker".into(), count: 9 })]), vec![Action::Message { speaker: "Erzähler".into(), text: "Zu wenige Bürger erreichten den Pass.".into(), voice: None }, Action::Defeat]),
        trig("sieg_hold", all_of(vec![flag("march_hold"), Condition::AtTime(480.0)]), vec![done_obj("obj_hold"), done_obj("obj_hero"), dawn_hold, chron("Brennhollow hielt gegen die Aschenlegion."), Action::Victory]),
        trig("sieg_evac", all_of(vec![flag("march_evacuate"), unit_in("worker", "pass", 12)]), vec![done_obj("obj_evac"), done_obj("obj_hero"), dawn_evac, chron("Die Bürger von Brennhollow erreichten die Straße nach Kaldris."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel vor Brennhollow."),
    ];
    let mut v1 = variant(&["torvald_blood"], &[], "Torvald Eisenhand ist mit seinem Clan nach Brennhollow gekommen.");
    v1.placements = vec![tagged(pl("hero_torvald", 0, 50.0, 42.0), "torvald"), group("shieldbearer", 0, 52.0, 44.0, 6)];
    let mut v2 = variant(&["rootmother_pact"], &[], "Sylaens Waldläufer halten die Waldseite.");
    v2.placements = vec![group("archer", 0, 38.0, 42.0, 6)];
    let mut v3 = variant(&["fleet_pact"], &[], "Saelith-Söldner sind per Schiff gekommen, als Ophiras Vertrag es verlangt.");
    v3.placements = vec![group("crossbowman", 0, 44.0, 54.0, 6), group("catapult", 0, 34.0, 52.0, 2)];
    let mut v4 = variant(&["rift_harvested"], &[], "Die Hohlung hat sich verbreitet: Die Legion bringt zusätzliche Hexen mit.");
    v4.triggers = vec![at("hexen", 150.0, wave("west", &[("ash_witch", 3)]))];
    let mut v5 = variant(&["nel_sheltered"], &[], "Nel hat die Legion kommen hören und die Bürger früh gewarnt: Mehr Zivilisten sind vorbereitet und bewaffnet.");
    v5.placements = vec![group("spearman", 0, 36.0, 44.0, 4)];
    m.variants = vec![v1, v2, v3, v4, v5];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter VII – Mission 28: Die Tore von Solmere (Belagerung)
// ---------------------------------------------------------------------------------------------

fn m28(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m28_gates_of_solmere";
    s.title("m28", "Mission 28 – Die Tore von Solmere");
    let mut c = Canvas::new(112, 96, 92328, GenPreset::Ashlands);
    c.flatten(0, 0, 111, 95, Terrain::Ash);
    c.flatten(30, 8, 82, 40, Terrain::Stone);
    c.flatten(10, 66, 100, 92, Terrain::Sand);
    c.path(&[(56.0, 92.0), (56.0, 60.0), (56.0, 40.0)], 2.4, Terrain::Road);
    for i in 0..9 {
        c.deco(DecorKind::Pillar, 34.0 + i as f32 * 5.0, 6.5);
    }
    for (x, y) in [(20.5, 50.5), (92.5, 50.5), (30.5, 62.5), (82.5, 62.5)] {
        c.deco(DecorKind::Ruin, x, y);
    }
    for (x, y) in [(40.5, 44.5), (72.5, 44.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Die Tore von Solmere",
        923,
        "Belagerung",
        "Der Bund der Letzten Flamme steht vor der Festung Solmere. Brecht das Tor der Hauptstadt Aureths, ehe der Hohle König sich erhebt.",
        "Solmere, Jahr 923 n.F. Hier endet die Aschmark – und hier beginnt der Bund der Letzten Flamme, den Isolde Raventhorn aus den Banner ihrer Verbündeten geflochten hat. Wer an Eurer Seite steht, hängt von Euren Entscheidungen ab: Torvald und sein Clan, Sylaens Waldläufer, Ophiras Söldner, Mireilles oder Aldrens Ritter, die Häretiker, die Eure Wahrheit gehört haben. Vor Euch die äußeren Mauern Solmeres: ein Tor, Türme, Legionäre, Hexen und Kolosse. Rammen und Katapulte stehen bereit. Brecht das Tor, nehmt den Torturm und öffnet den Weg zum Sonnenhof.",
        "Das Tor von Solmere brach, und der Bund der Letzten Flamme ergoss sich in die Stadt der Toten. Hinter den Mauern wartete der Thron.",
        "Das Tor von Solmere hielt. Der Bund zerbrach vor den Mauern der Hauptstadt.",
        &["siege_assault", "endgame_final_battle"],
    );
    m.map_seed = 92328;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.50);
    let mut p = vec![
        hero_at(56.0, 84.0),
        tagged(pl("hero_hal_venn", 0, 58.0, 85.5), "hero_hal"),
        group("swordsman", 0, 48.0, 84.0, 14),
        group("halberdier", 0, 64.0, 84.0, 10),
        group("crossbowman", 0, 52.0, 88.0, 10),
        group("archer", 0, 60.0, 88.0, 8),
        group("knight", 0, 44.0, 80.0, 6),
        group("priest", 0, 56.0, 90.0, 3),
        group("battering_ram", 0, 50.0, 78.0, 4),
        group("catapult", 0, 62.0, 78.0, 3),
        // Solmere: Ring mit Haupttor.
        tagged(pl("keep", 1, 55.0, 24.0), "gate_keep"),
        tagged(guard("ashen_legionnaire", 1, 52.0, 34.0, 4), "gate_guard"),
        guard("ashen_legionnaire", 1, 60.0, 34.0, 4),
        guard("ashen_legionnaire", 1, 56.0, 30.0, 6),
        guard("hollow_archer", 1, 46.0, 30.0, 4),
        guard("hollow_archer", 1, 66.0, 30.0, 4),
        guard("ash_witch", 1, 56.0, 26.0, 3),
        guard("grave_brute", 1, 50.0, 28.0, 2),
        guard("grave_brute", 1, 62.0, 28.0, 2),
        tagged(guard("swordsman", 1, 40.0, 30.0, 4), "heretics"),
        tagged(guard("swordsman", 1, 72.0, 30.0, 4), "heretics"),
    ];
    wall_ring_gaps(&mut p, 1, (34, 14, 78, 38), &[(56, 38)], &[(34, 14), (77, 14), (34, 37), (77, 37), (44, 38), (68, 38)], &[(56, 14)]);
    m.placements = p;
    m.areas = vec![
        area("lager", "Lager des Bundes", 36, 74, 40, 18),
        area("tor", "Das Haupttor", 46, 34, 20, 10),
        area("stadt", "Solmere", 34, 14, 45, 25),
        area("bund", "Hinter den Linien", 36, 70, 40, 6),
    ];
    m.starts = vec![StartPos { owner: 0, x: 56.0, y: 84.0 }, StartPos { owner: 1, x: 56.0, y: 44.0 }];
    m.objectives = vec![
        objective("obj_gate", "Brecht das Haupttor von Solmere.", false),
        objective("obj_keep", "Erobert den Torturm von Solmere.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m28", "lager", &[
        (Sp::Narrator, "Hinter dem Horizont wuchsen die Mauern Solmeres: schwarz, hoch, in den Fugen glimmend. Zum ersten Mal seit tausend Jahren sahen lebende Menschen die Hauptstadt Aureths aus der Nähe – und zählten die Zinnen, ohne sie zu Ende zu zählen.", "weit, ehrfürchtig"),
        (Sp::Isolde, "Rammen ans Tor, Katapulte auf die Zinnen. Und niemand betritt Solmere, ehe das Tor offen ist – ich will keinen Mann vor die Hexen schicken, der noch eine Mauer überwinden muss.", "klar, knapp"),
    ]);
    s.note("m28", "lager", "Das Lager des Bundes vor Solmere.");
    let torvald = s.dlg("m28", "torvald", &[(Sp::Torvald, "Meine Ahnen sehen zu, Isolde. Sie haben nie eine Mauer gesehen, vor der Eisenhand zögerte. Wir gehen zuerst – du brichst das Tor.", "grimmig, stolz")]);
    s.note("m28", "torvald", "Nur wenn Torvald den Blutschwur gehalten hat.");
    let sylaen = s.dlg("m28", "sylaen", &[(Sp::Sylaen, "Der Wald hat mich nicht verlassen, Wächterin. Meine Pfeile fliegen mit dem Wind – und der Wind kennt Solmere.", "ruhig, melodisch")]);
    s.note("m28", "sylaen", "Nur wenn die Wurzelmutter erwacht ist (Pakt).");
    let ophira = s.dlg("m28", "ophira", &[(Sp::Ophira, "Meine Schiffe liegen im Hafen, meine Söldner stehen bei Euch. Vergesst nicht, dass jeder Schuss in diesen Mauern ein Eintrag in meinem Buch ist.", "trocken, geschäftsmäßig")]);
    s.note("m28", "ophira", "Nur wenn der Vertrag mit Ophira geschlossen wurde.");
    let gate = s.dlg("m28", "tor", &[(Sp::Isolde, "Das Tor ist gebrochen! Schildträger voran, die Priester folgen den Schilden – und lasst keinen zurück!", "laut, triumphierend")]);
    s.note("m28", "tor", "Das Haupttor fällt.");
    let heretic = s.dlg("m28", "haeretiker", &[(Sp::Narrator, "Auf den Mauern Solmeres standen Männer mit Sonnen auf der Brust. Sie sahen die Banner des Bundes, hörten, was Isolde Raventhorn gelesen hatte, und senkten die Waffen. Man weiß nicht, ob aus Scham oder aus Hoffnung.", "ruhig, bedächtig")]);
    s.note("m28", "haeretiker", "Die Sonnenhäretiker auf den Mauern (Entscheidung in Mission 25).");
    let fin = s.dlg("m28", "sieg", &[(Sp::Hal, "Das war das Tor. Hinter ihm liegt der Thron. Ich muss dir etwas sagen, Isolde, bevor wir hineingehen: Danke. Für alles, was du getan hast – auch für das, was dich gekostet hat.", "leise, bewegt")]);
    s.note("m28", "sieg", "Nach der Eroberung des Torturms.");
    m.triggers = vec![
        trig("lager", Condition::AtTime(1.0), vec![intro, add_obj("obj_gate"), add_obj("obj_keep"), add_obj("obj_hero"), chron("Der Bund der Letzten Flamme lagerte vor den Mauern Solmeres.")]),
        trig("torvald", all_of(vec![flag("torvald_blood"), Condition::AtTime(8.0)]), vec![torvald]),
        trig("sylaen", all_of(vec![flag("rootmother_pact"), Condition::AtTime(14.0)]), vec![sylaen]),
        trig("ophira", all_of(vec![flag("fleet_pact"), Condition::AtTime(20.0)]), vec![ophira]),
        trig("haeretiker", all_of(vec![any_flag(&["sun_truth", "sun_faith"]), Condition::AtTime(26.0)]), vec![heretic]),
        trig("tor", Condition::TagDamaged("gate_keep".into()), vec![done_obj("obj_gate"), gate]),
        trig("sieg", tag_dead("gate_keep"), vec![done_obj("obj_keep"), done_obj("obj_hero"), fin, chron("Die Tore von Solmere fielen."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel vor den Toren von Solmere."),
    ];
    let mut v1 = variant(&["torvald_blood"], &[], "Torvald Eisenhand und seine Clankrieger bilden die erste Welle.");
    v1.placements = vec![tagged(pl("hero_torvald", 0, 54.0, 82.0), "torvald"), group("shieldbearer", 0, 52.0, 86.0, 8), group("halberdier", 0, 58.0, 82.0, 4)];
    let mut v2 = variant(&["torvald_gold"], &[], "Torvalds Clan blieb in Skarnheim; nur ein Söldnerhaufen kam.");
    v2.placements = vec![group("crossbowman", 0, 66.0, 86.0, 4)];
    let mut v3 = variant(&["rootmother_pact"], &[], "Sylaens Waldläufer und Wurzelheiler stehen im Bund.");
    v3.placements = vec![group("archer", 0, 44.0, 88.0, 8), group("priest", 0, 40.0, 88.0, 3)];
    let mut v4 = variant(&["rootmother_burned"], &[], "Der Dornenbund fehlt; die Waldwege sind unbesetzt.");
    v4.placements = vec![guard("hollow_archer", 1, 36.0, 44.0, 4)];
    let mut v5 = variant(&["fleet_pact"], &[], "Saelith-Söldner und Katapulte sind mit Ophiras Schiffen eingetroffen.");
    v5.placements = vec![group("crossbowman", 0, 70.0, 88.0, 8), group("catapult", 0, 70.0, 80.0, 2)];
    let mut v6 = variant(&["throne_mireille"], &[], "Mireille Corvanes Ritter sind dem Bund beigetreten.");
    v6.placements = vec![group("knight", 0, 40.0, 82.0, 6), group("swordsman", 0, 40.0, 86.0, 6)];
    let mut v7 = variant(&["throne_aldren"], &[], "Die Königsgarde Aldrens und ihre Armbrüste folgen dem Bund.");
    v7.placements = vec![group("halberdier", 0, 40.0, 82.0, 6), group("crossbowman", 0, 40.0, 86.0, 6)];
    let mut v8 = variant(&["sun_truth"], &[], "Die Wahrheit über Varkhan hat die Häretiker gespalten: Ihre Wachen auf den Mauern legen die Waffen nieder.");
    v8.remove_tags = vec!["heretics".into()];
    let mut v9 = variant(&["sun_faith"], &[], "Die Häretiker, deren Glauben Ihr bewahrt habt, öffnen Euch den Torhof: Die Wachen am Tor sind abgezogen.");
    v9.remove_tags = vec!["gate_guard".into()];
    let mut v10 = variant(&["sun_used"], &[], "Die Häretiker kämpfen unter Eurer Fahne – bis die Wahrheit sie einholt.");
    v10.remove_tags = vec!["heretics".into()];
    v10.placements = vec![group("swordsman", 0, 72.0, 88.0, 8), group("archer", 0, 74.0, 84.0, 6)];
    let mut v11 = variant(&["core_kept"], &[], "Kampfmagier der Flamme schleudern Feuer auf die Zinnen.");
    v11.placements = vec![group("battle_mage", 0, 56.0, 92.0, 3)];
    let mut v12 = variant(&["march_evacuate"], &[], "Die Bürger von Brennhollow folgen dem Heer und stellen Nachschub: mehr Hilfskräfte, aber weniger Versorgung der Kampftruppen.");
    v12.placements = vec![group("worker", 0, 30.0, 88.0, 8)];
    let mut v14 = variant(&["fleet_none"], &[], "Ohne Ophiras Schiffe fehlt dem Bund das Fernkampfgeschütz; die Verteidiger Solmeres haben zusätzliche Bogenschützen auf den Zinnen postiert.");
    v14.placements = vec![guard("hollow_archer", 1, 50.0, 36.0, 4), guard("hollow_archer", 1, 62.0, 36.0, 4)];
    let mut v13 = variant(&["march_hold"], &[], "Die Verteidiger von Brennhollow haben ihre Stadt gehalten: Veteranen verstärken das Heer.");
    v13.placements = vec![group("swordsman", 0, 30.0, 84.0, 6), group("halberdier", 0, 30.0, 88.0, 4)];
    m.variants = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

// ---------------------------------------------------------------------------------------------
// Zeitalter VII – Mission 29: Der Hohle Thron (Endschlacht)
// ---------------------------------------------------------------------------------------------

fn m29(data: &GameData, s: &mut Script) -> MissionDef {
    let id = "m29_hollow_throne";
    s.title("m29", "Mission 29 – Der Hohle Thron");
    let mut c = Canvas::new(96, 96, 92329, GenPreset::Ashlands);
    c.flatten(0, 0, 95, 95, Terrain::Ash);
    c.flatten(18, 4, 78, 92, Terrain::Stone);
    c.flatten(30, 4, 66, 26, Terrain::Sand);
    c.disc(48.0, 14.0, 9.0, Terrain::DarkGrass);
    c.path(&[(48.0, 90.0), (48.0, 60.0), (48.0, 30.0), (48.0, 16.0)], 3.0, Terrain::Road);
    boulder_line(&mut c, 18, 4, 18, 92, &[]);
    boulder_line(&mut c, 78, 4, 78, 92, &[]);
    boulder_line(&mut c, 18, 4, 78, 4, &[]);
    boulder_line(&mut c, 18, 92, 78, 92, &[]);
    boulder_line(&mut c, 19, 64, 77, 64, &[(42, 54)]);
    boulder_line(&mut c, 19, 44, 77, 44, &[(40, 56)]);
    boulder_line(&mut c, 19, 28, 77, 28, &[(40, 56)]);
    for i in 0..9 {
        c.deco(DecorKind::Pillar, 26.0 + i as f32 * 5.5, 8.5);
    }
    for (x, y) in [(24.5, 72.5), (72.5, 72.5), (24.5, 50.5), (72.5, 50.5), (30.5, 34.5), (66.5, 34.5)] {
        c.deco(DecorKind::Brazier, x, y);
    }
    for (x, y) in [(34.5, 18.5), (62.5, 18.5), (48.5, 22.5)] {
        c.deco(DecorKind::Bones, x, y);
    }
    let mut m = campaign_mission(
        id,
        "Der Hohle Thron",
        923,
        "Endschlacht",
        "Im Thronsaal von Solmere wartet Varkhan Aurelith, der Hohle König. Kämpft Euch durch vier Höfe – und wählt am Ende, was aus der Welt wird.",
        "Solmere, Jahr 923 n.F. Das letzte Tor ist gefallen. Vor Euch liegen vier Höfe: der Mauerhof, der Sonnenhof, der Hof des Schleiers und der Thronsaal, in dem Varkhan Aurelith sitzt – der Hohle König, Herr der Legion, Schlüssel und Gefangener des Tiefs. Mit Euch ziehen Isolde, Hal Venn und alle, die der Bund gewinnen konnte. Wenn der König fällt, bis zur Hälfte seiner Kraft, öffnet der Schleier hinter dem Thron seinen Mund, und die Stimme aus dem Tief bietet Euch an, was sie immer anbietet. Wählt: versiegeln, krönen, erlösen oder verbrennen – wenn die Wege, die Ihr gegangen seid, es erlauben.",
        "Der Hohle König fiel, und mit ihm schwiegen die Glocken Solmeres. Was auch immer von der Welt blieb: Es blieb, weil jemand die Wahl getroffen hatte.",
        "Der Thronsaal von Solmere blieb in den Händen der Toten. Das Hohle Tief sprach – und die Welt hörte zu.",
        &["boss_hollow_king", "endgame_last_crown"],
    );
    m.map_seed = 92329;
    m.start_resources = Cost::default();
    m.defeat_tag = String::new();
    m.start_time = Some(0.97);
    m.placements = vec![
        hero_at(48.0, 84.0),
        tagged(pl("hero_hal_venn", 0, 50.0, 85.5), "hero_hal"),
        group("swordsman", 0, 40.0, 84.0, 14),
        group("halberdier", 0, 56.0, 84.0, 10),
        group("crossbowman", 0, 44.0, 88.0, 10),
        group("archer", 0, 52.0, 88.0, 8),
        group("knight", 0, 36.0, 80.0, 6),
        group("shieldbearer", 0, 60.0, 80.0, 6),
        group("priest", 0, 48.0, 89.5, 4),
        // Der Mauerhof.
        guard("ashen_legionnaire", 1, 38.0, 72.0, 6),
        guard("ashen_legionnaire", 1, 58.0, 72.0, 6),
        guard("hollow_archer", 1, 48.0, 68.0, 5),
        guard("grave_brute", 1, 48.0, 70.0, 1),
        // Der Sonnenhof.
        guard("ashen_legionnaire", 1, 36.0, 52.0, 6),
        guard("ashen_legionnaire", 1, 60.0, 52.0, 6),
        guard("ash_witch", 1, 48.0, 48.0, 3),
        guard("grave_brute", 1, 44.0, 54.0, 2),
        // Der Hof des Schleiers.
        guard("ghoul", 1, 34.0, 36.0, 5),
        guard("ghoul", 1, 62.0, 36.0, 5),
        guard("bat_swarm", 1, 48.0, 34.0, 4),
        guard("ash_witch", 1, 48.0, 31.0, 2),
        // Der Thronsaal.
        tagged(guard("boss_hollow_king", 1, 48.0, 14.0, 1), "boss"),
        guard("ashen_legionnaire", 1, 40.0, 18.0, 4),
        guard("ashen_legionnaire", 1, 56.0, 18.0, 4),
        guard("ash_witch", 1, 48.0, 20.0, 2),
    ];
    m.areas = vec![
        area("lager", "Der Zugang", 26, 76, 44, 14),
        area("mauerhof", "Der Mauerhof", 20, 65, 56, 14),
        area("sonnenhof", "Der Sonnenhof", 20, 45, 56, 18),
        area("schleier", "Der Hof des Schleiers", 20, 29, 56, 14),
        area("thronsaal", "Der Thronsaal", 20, 6, 56, 21),
    ];
    m.starts = vec![StartPos { owner: 0, x: 48.0, y: 84.0 }, StartPos { owner: 1, x: 48.0, y: 14.0 }];
    m.objectives = vec![
        objective("obj_courts", "Kämpft Euch durch den Mauerhof, den Sonnenhof und den Hof des Schleiers.", false),
        objective("obj_king", "Stellt Varkhan Aurelith, den Hohlen König, im Thronsaal.", false),
        objective("obj_choice", "Trefft die letzte Wahl.", false),
        objective("obj_hero", "Isolde Raventhorn darf nicht fallen.", false),
    ];
    let intro = s.dlg("m29", "tor", &[
        (Sp::Narrator, "Hinter dem Tor lag, was von Aureth übrig war: vier Höfe, einer hinter dem anderen, wie Kammern eines Herzens. Und am Ende der Höfe der Thron, auf dem seit tausend Jahren jemand saß, der nicht schlief und nicht starb.", "feierlich, tonlos"),
        (Sp::Hal, "Gleich ist es so weit, Isolde. Wenn du Zweifel hast – jetzt wäre der Moment, sie auszusprechen.", "leise, warm"),
        (Sp::Isolde, "Zweifel habe ich seit Brennhollow. Aber sie halten mich nicht mehr auf. Vorwärts, für die Mark.", "ruhig, fest"),
    ]);
    s.note("m29", "tor", "Das Innere des Tores von Solmere.");
    let courts = s.dlg("m29", "hoefe", &[(Sp::Narrator, "Der dritte Hof war der Hof des Schleiers. Die Luft zitterte, als wäre sie nur gemalt, und über den Mauern hing ein Riss, durch den man die Nacht sah – eine, die nicht die eigene war.", "leise, gespannt")]);
    s.note("m29", "hoefe", "Beim Betreten des Hofs des Schleiers.");
    let king = s.dlg("m29", "thron", &[
        (Sp::HollowKing, "Ich war die Sonne von Aureth. Könige knieten vor meinem Schatten. Und nun betritt eine Grenzwächterin meinen Thronsaal.", "archaisch, königlich, hohl"),
        (Sp::Isolde, "Dein Schatten hat eine halbe Welt getötet, Varkhan. Ich bin hier, um ihn zurückzuschicken.", "ruhig, kalt"),
        (Sp::HollowKing, "Ich wollte den Tod beenden. Für alle. Verstehst du? Keine Gräber mehr. Keine trauernden Mütter. Es hat mir geantwortet … und es hat gelogen.", "plötzlich leise, gequält"),
        (Sp::Deep, "Ich lüge nie. Ich versprach ihm, dass niemand sterben würde. Und in Solmere ist niemand mehr gestorben.", "ruhig, entsetzlich ungerührt"),
    ]);
    s.note("m29", "thron", "Im Thronsaal: Varkhan und die Stimme aus dem Tief.");
    let veil = s.dlg("m29", "schleier", &[
        (Sp::Hal, "Isolde, der Schleier ist offen hinter dem Thron. Was immer du tun willst, tu es jetzt.", "dringlich"),
        (Sp::HollowKing, "Nimm meine Krone, Wächterin. Befehlige die Toten, und kein Dorf wird je wieder brennen. Oder tritt durch den Schleier und schließe ihn hinter dir. So oder so … jemand muss die Asche tragen.", "tonlos, erschöpft"),
        (Sp::Deep, "Wähle, kleine Flamme. Ich bin so oder so da.", "geduldig, gleichgültig"),
    ]);
    s.note("m29", "schleier", "Der König ist halb gefallen; der Schleier öffnet sich.");
    let seal = vec![s.dlg("m29", "ende_a", &[
        (Sp::Isolde, "Hal. Halt die Feuer für mich am Brennen. Jemand muss die Tür halten.", "Abschied, ruhig, sehr gefasst"),
        (Sp::Hal, "Nein … Isolde … die Flamme behüte dich, Kind. Die Flamme behüte dich.", "brechende Stimme, Tränen"),
        (Sp::Deep, "Tapfer. Kurz. Und nun … mein. Willkommen daheim.", "ruhig, endgültig"),
    ])];
    let crown = vec![s.dlg("m29", "ende_b", &[
        (Sp::Isolde, "Gib mir die Krone. Wenn die Toten marschieren müssen, dann für die Lebenden.", "hart, kalt entschlossen"),
        (Sp::HollowKing, "Genau das habe ich auch gesagt.", "hohles, trauriges Lachen, zerfallend"),
        (Sp::Hal, "Isolde … deine Augen. Die Flamme vergebe uns beiden.", "entsetzt, leise"),
    ])];
    let redeem = vec![s.dlg("m29", "ende_c", &[
        (Sp::Isolde, "Varkhan. Deine Tochter hieß Sereth. Sie hat dir nie gesagt, dass sie dir verzeiht, weil sie nie dachte, dass du es brauchst. Sie tut es.", "weich, ruhig, ohne Kälte"),
        (Sp::HollowKing, "Sereth … Sereth. Ich habe den Namen so lange nicht gehört. Wer – wer hat ihn dir gesagt?", "zitternd, menschlich"),
        (Sp::Nel, "Sie hat ihn mir gesagt. Im Schlaf. Sie wartet auf dich.", "kindlich, klar, leise"),
    ])];
    let destroy = vec![s.dlg("m29", "ende_d", &[
        (Sp::Isolde, "Es gibt kein Tor, das ich schließen muss, und keinen Thron, den ich besteigen will. Es gibt nur Feuer – und das ist alles, was ich je gut konnte.", "bitter, leidenschaftlich"),
        (Sp::Deep, "Du verbrennst dich selbst, kleine Flamme. Schade. Ich hätte dich so gern behalten.", "bedauernd, samtig"),
    ])];
    s.note("m29", "ende_a", "Letzte Wahl: Schleier versiegeln.");
    s.note("m29", "ende_b", "Letzte Wahl: Hohle Krone nehmen.");
    s.note("m29", "ende_c", "Letzte Wahl: Varkhan erlösen (nur mit Nel und der Wahrheit über die Sonnenhäresie).");
    s.note("m29", "ende_d", "Letzte Wahl: Das Tiefe verbrennen (nur mit zerschlagenem Kern und versiegeltem Riss).");
    let finale = s.decision(
        "m29",
        "schleier",
        "final",
        "Die letzte Wahl",
        (Sp::HollowKing, "Wähle, Wächterin. Es gibt keinen vierten Weg – außer denen, die du dir durch deine Wege verdient hast.", "tonlos, müde"),
        vec![
            opt("final_seal", "Den Schleier von innen versiegeln", "Isolde bleibt hinter dem Schleier zurück; die Welt ist sicher.", &[], seal),
            opt("final_crown", "Die Hohle Krone nehmen", "Die Toten gehorchen, aber die Stimme aus dem Tief bleibt.", &[], crown),
            opt("final_redeem", "Varkhan erlösen", "Nur möglich, wenn Nel in Brennhollow aufgenommen wurde und die Wahrheit über die Sonnenhäresie bekannt ist.", &["nel_sheltered", "sun_truth"], redeem),
            opt("final_destroy", "Das Tiefe verbrennen", "Nur möglich, wenn der Kern zerschlagen und der Riss von Marrowfen versiegelt wurde.", &["core_smashed", "rift_sealed"], destroy),
        ],
    );
    let win = s.dlg("m29", "sieg", &[(Sp::Narrator, "Der Hohle König war gefallen. Die Glocken von Solmere läuteten ein letztes Mal, und danach hörte man in der ganzen Welt nur noch den Wind.", "ruhig, still")]);
    s.note("m29", "sieg", "Schlusswort der Endschlacht.");
    m.triggers = vec![
        trig("tor", Condition::AtTime(1.0), vec![intro, add_obj("obj_courts"), add_obj("obj_king"), add_obj("obj_choice"), add_obj("obj_hero"), chron("Der Bund der Letzten Flamme betrat Solmere.")]),
        trig("schleier", in_area("schleier", 1), vec![courts, Action::SetWeather(ashen_sim::env::Weather::Storm)]),
        trig("thron", in_area("thronsaal", 1), vec![done_obj("obj_courts"), king]),
        trig("wahl", Condition::TagHealthBelow { tag: "boss".into(), percent: 50.0 }, vec![veil, finale]),
        trig("verstaerkung", Condition::TagHealthBelow { tag: "boss".into(), percent: 75.0 }, wave("thronsaal", &[("ashen_legionnaire", 4), ("ash_witch", 1)])),
        trig("sieg", all_of(vec![tag_dead("boss"), any_flag(&["final_seal", "final_crown", "final_redeem", "final_destroy"])]), vec![done_obj("obj_king"), done_obj("obj_choice"), done_obj("obj_hero"), win, chron("Varkhan Aurelith, der Hohle König, fiel in Solmere."), Action::Victory]),
        hero_lost("Isolde Raventhorn fiel im Thronsaal von Solmere."),
    ];
    let mut v1 = variant(&["torvald_blood"], &[], "Torvald Eisenhand und sein Clan kämpfen an der Spitze.");
    v1.placements = vec![tagged(pl("hero_torvald", 0, 44.0, 80.0), "torvald"), group("shieldbearer", 0, 40.0, 80.0, 6)];
    let mut v2 = variant(&["rootmother_pact"], &[], "Sylaens Waldläufer und die Wurzelmutter stehen im Rücken des Bundes.");
    v2.placements = vec![group("archer", 0, 62.0, 88.0, 8), group("priest", 0, 66.0, 88.0, 2)];
    let mut v3 = variant(&["fleet_pact"], &[], "Saelith-Söldner stärken die Fernkämpfer.");
    v3.placements = vec![group("crossbowman", 0, 36.0, 88.0, 8)];
    let mut v4 = variant(&["throne_mireille"], &[], "Mireille Corvane führt ihre Ritter in die Schlacht.");
    v4.placements = vec![group("knight", 0, 30.0, 84.0, 6)];
    let mut v5 = variant(&["throne_aldren"], &[], "Die Königsgarde sichert die Flanke.");
    v5.placements = vec![group("halberdier", 0, 66.0, 84.0, 6)];
    let mut v6 = variant(&["rift_harvested"], &[], "Die Hohlung wächst: Zusätzliche Legionäre stehen im Thronsaal.");
    v6.placements = vec![guard("ashen_legionnaire", 1, 48.0, 24.0, 6)];
    let mut v7 = variant(&["sun_truth"], &[], "Die gespaltenen Häretiker kämpfen nicht mehr für Varkhan: Der Hof des Schleiers ist schwächer besetzt.");
    v7.remove_tags = vec![];
    v7.placements = vec![group("priest", 0, 40.0, 90.0, 2)];
    let mut v8 = variant(&["march_hold"], &[], "Die Veteranen von Brennhollow haben ihre Stadt gehalten und sind dem Bund beigetreten.");
    v8.placements = vec![group("swordsman", 0, 30.0, 80.0, 6)];
    let mut v9 = variant(&["march_evacuate"], &[], "Die Bürger von Brennhollow kämpfen aus Zorn mit: Hilfskräfte folgen dem Heer.");
    v9.placements = vec![group("spearman", 0, 30.0, 88.0, 6)];
    m.variants = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9];
    m.map = Some(c.finish_with(&m.placements, data));
    m
}

/// Kampagnenplan: sieben Zeitalter, Missionen, Enden.
pub(crate) fn plan(s: &mut Script) -> crate::campaign_forge::Plan {
    let ch = |s: &mut Script, id: &str, text: &str| -> Option<String> { s.ln("age", id, Sp::Narrator, text, "getragen, ruhig, Chronistenton").voice };
    let t1 = "Jahr 897 nach dem Fall. Am Rand der Aschmark hält Brennhollow noch stand – dreißig Seelen, ein Bergfried, ein Feld. Und jede Nacht kommt der Nebel näher.";
    let t2 = "Brennhollow wuchs, und mit ihm der Ruf seiner Wächterin. Doch wer in der Aschmark Mauern baut, wird in Kaldris gezählt: Der König prüft seine Getreuen, und jeder Hof hat sein Messer.";
    let t3 = "Die Hohlen griffen nicht mehr in Wellen an, sondern wie ein Heer, das einen Plan verfolgt. An drei Grenzen brachen die Mauern – und die alten Feinde mussten lernen, einander zuzuhören.";
    let t4 = "Aus der Verteidigerin wurde eine Eroberin. Die Aschglasfelder lockten Legion und Händler zugleich, und an der Küste wartete etwas, das nicht ertrunken war, sondern wartete.";
    let t5 = "Der König war alt, und der Thron hat keine Freunde. In Kaldris wurden Zeugen leise und Glocken laut – und Isolde stand in der Mitte, mit dem Schwert in der einen und dem Gesetz in der anderen Hand.";
    let t6 = "Die Götter schwiegen seit dem Aschenfall. Nun regten sich Altäre unter Bergen und Ruinen, und mit ihnen erwachte, was die Götter einst gebunden hatten – und was sich an ihrer Stelle genährt hatte.";
    let t7 = "Das Hohle Tief erwachte nicht aus eigener Kraft: Es hatte immer einen Schlüssel. Brennhollow brannte noch einmal, und dann zog ein Bund aus Fahnen und Feuern nach Solmere, wo ein toter König auf seinem Thron wartete.";
    s.title("age", "Zeitalter der Kampagne – Kapiteleinleitungen und Enden");
    s.note("age", "age1", "Kapitelseite Zeitalter I.");
    let (v1, v2, v3, v4, v5, v6, v7) = (ch(s, "age1", t1), ch(s, "age2", t2), ch(s, "age3", t3), ch(s, "age4", t4), ch(s, "age5", t5), ch(s, "age6", t6), ch(s, "age7", t7));
    let mut c1 = cm("m01_last_settlement", &[], (0.34, 0.72));
    c1.finale = false;
    let m = |id: &str, needs: &[&str], pos: (f32, f32)| cm(id, needs, pos);
    let chapters = vec![
        chapter("age1", "I", "Die letzte Siedlung", "The Last Settlement", "897–899 n.F.", t1, v1, vec![c1, m("m02_harrowmere", &["m01_last_settlement"], (0.24, 0.66)), m("m03_grey_road", &["m02_harrowmere"], (0.38, 0.60))]),
        chapter("age2", "II", "Die aufsteigende Krone", "The Rising Crown", "899–902 n.F.", t2, v2, vec![m("m04_long_night", &["m03_grey_road"], (0.46, 0.68)), m("m06_tourney_kaldris", &["m04_long_night"], (0.50, 0.44)), m("m07_iron_hills", &["m06_tourney_kaldris"], (0.32, 0.48))]),
        chapter("age3", "III", "Die gebrochenen Grenzen", "The Broken Borders", "905–907 n.F.", t3, v3, vec![m("m09_black_winter", &["m07_iron_hills"], (0.52, 0.16)), m("m10_root_and_ash", &["m09_black_winter"], (0.16, 0.38)), m("m11_marrowfen_rift", &["m10_root_and_ash"], (0.22, 0.82))]),
        chapter("age4", "IV", "Das Zeitalter der Eroberung", "The Age of Conquest", "910–912 n.F.", t4, v4, vec![m("m13_ashglass_fields", &["m11_marrowfen_rift"], (0.70, 0.72)), m("m14_black_reef", &["m13_ashglass_fields"], (0.84, 0.50))]),
        chapter("age5", "V", "Der geteilte Thron", "The Divided Throne", "916–917 n.F.", t5, v5, vec![m("m17_death_of_a_king", &["m14_black_reef"], (0.60, 0.34)), m("m19_kaldris_ford", &["m17_death_of_a_king"], (0.68, 0.46))]),
        chapter("age6", "VI", "Die gefallenen Götter", "The Fallen Gods", "920–922 n.F.", t6, v6, vec![m("m21_shrine_oruun", &["m19_kaldris_ford"], (0.64, 0.20)), m("m25_ithren_light", &["m21_shrine_oruun"], (0.44, 0.88))]),
        chapter("age7", "VII", "Die letzte Krone", "The Last Crown", "923 n.F.", t7, v7, vec![m("m26_ashen_march", &["m25_ithren_light"], (0.40, 0.78)), m("m28_gates_of_solmere", &["m26_ashen_march"], (0.60, 0.84)), {
            let mut f = m("m29_hollow_throne", &["m28_gates_of_solmere"], (0.70, 0.92));
            f.finale = true;
            f
        }]),
    ];
    s.note("age", "ende", "Enden der Kampagne, gesprochen nach dem Abspann der letzten Mission.");
    let e = |s: &mut Script, id: &str, title: &str, text: &str, req: &[&str]| -> ashen_sim::campaign::Ending {
        let l = s.ln("age", "ende", Sp::Narrator, text, "ruhig, getragen, langsam im letzten Satz");
        ending(id, title, text, req, l.voice)
    };
    let endings = vec![
        e(s, "end_redeem", "Erlösung", "Varkhan Aurelith sprach zum ersten Mal seit tausend Jahren den Namen seiner Tochter – und das Tiefe verlor seinen Schlüssel. Es wurde still in Solmere, und im Frühjahr blühte in der Aschmark der erste Apfelbaum seit dem Fall.", &["final_redeem"]),
        e(s, "end_destroy", "Das Ende der Glut", "Das Feuer, das Isolde in Solmere entfachte, brannte neun Tage lang. Das Tiefe wich zurück, doch die Wunden der Welt blieben offen – und die Glut der Wächterin erlosch mit ihnen.", &["final_destroy"]),
        e(s, "end_crown", "Die Aschenkrone", "So wurde die letzte Krone Eldraeths aus Asche geschmiedet. Die Toten hielten Wache an jeder Grenze, und kein Dorf brannte mehr. Doch nachts, in den Hallen von Solmere, sprach die neue Herrin manchmal mit jemandem, den niemand sah.", &["final_crown"]),
        e(s, "end_seal", "Das versiegelte Tor", "Und so wurde der Schleier geschlossen. Niemand krönte die Wächterin der Aschmark. Aber in Brennhollow brennt seither jede Nacht ein Feuer, und niemand hat es je erlöschen lassen.", &[]),
    ];
    crate::campaign_forge::Plan {
        id: "age_of_the_fallen".into(),
        name: "Das Zeitalter der Gefallenen".into(),
        intro: "Eldraeth, Jahr 897 nach dem Fall. Eine Wächterin, ein sterbendes Grenzland und eine Welt, die vergessen hat, wie es ist, ohne Furcht zu schlafen.".into(),
        chapters,
        endings,
    }
}
