# ASHEN CROWNS – Strategieebene („Weltkarte“, Grand Campaign)

Crate: `crates/ashen_strategy` – vollständige, deterministische Simulation der Weltkarten-Ebene ohne Grafik.
Spezifikation: [`../design/game_design_document.md`](../design/game_design_document.md) (Kapitel 4–9, 12–17, 19, 22),
Kanon: [`../canon.md`](../canon.md), Welt: [`../lore/world_bible.md`](../lore/world_bible.md). Stil und Speicherformat
folgen `ashen_sim` ([`architecture.md`](architecture.md)).

Abhängigkeiten: nur Workspace-Crates (`ashen_core`, `glam`, `serde`, `ron`, `bincode`, `rand`, `rand_chacha`,
`crc32fast`, `log`, `uuid`). `ashen_sim` wird nur in den Tests verwendet (Abgleich der Einheiten-IDs mit
`assets/data/units.ron`).

---

## 1. Überblick

| Aspekt | Umsetzung |
|---|---|
| Zeit | Tagestakt (`tick_day`); 30 Tage = Monat, 12 Monate = Jahr (360 Tage); Start 1. Frostmond 897 n.F. |
| Zufall | Ein `ChaCha8Rng` im Weltzustand (Seed aus `StrategyOptions`), Kartenerzeugung mit eigenem Seed-Zweig |
| Eingaben | Spieleraktionen als Daten: `StrategyCommand`; alle anderen Reiche handeln per KI |
| Ausgaben | `StrategyEvent` (`drain_events`), Chronik, Ereignisfenster (`pending_events`), Schlachtanfragen |
| Zustand | `StrategyWorld` (alles serialisierbar, IDs statt Referenzen, nur `Vec`/`BTreeMap` → reproduzierbar) |
| Prüfsumme | `StrategyWorld::state_hash()` (FNV-1a über den bincode-Zustand) |
| Leistung | 200 Spieljahre in ca. 2,3–3,2 s (Release, 12 getestete Seeds), Weltaufbau inkl. Wirtschafts-Einschwingen ca. 0,1 s |

Verarbeitung pro Tag: Heere bewegen, Schlachten/Belagerungen auflösen. Am Monatsanfang (`monthly`) laufen der Reihe
nach: Wetter → Wirtschaft (Produktion, Verbrauch, Märkte, Handel) → Bevölkerung (Zufriedenheit, Demografie, Seuchen,
Migration, Spannungen, Verwaltung, Finanzen) → Ordnung/Aufstände → Religion → Magie/Schleier → Charaktere → Reiche/Ämter →
Politik (Häuser, Fraktionen) → Intrigen → Diplomatie/Bedrohung/Koalitionen → Krieg → Neutrale/Bosse/Hohle → KI →
Ereignisse → Endgame → Siegprüfung. Am Jahresanfang: Altern, Geburten, Ehen, Abdankungen, Kadettenhäuser, Dynastien,
Wirtschaft, Bosse, KI-Ziele, Endgame-Jahresereignisse.

## 2. Moduleübersicht

| Modul | Inhalt |
|---|---|
| `types.rs` | IDs, Kalender (`Date`, Monatsnamen), `Biome`, `Culture`, `Religion` (6), `Stratum` (8), `Office` (7), `Attr` (6), `School` (9), `Rank` |
| `goods.rs` | 57 Waren (IDs wie GDD 8.2), 54 Gebäude mit Rezepten (GDD 8.3), Bedürfnisse je Schicht (GDD 9.3) |
| `catalog.rs` | strategische Einheitendefinitionen (IDs aus `units.ron` + GDD 10.3), 37 Eigenschaften, Namen je Kultur, Rituale, Artefakte |
| `mapgen.rs` | Kartenerzeugung: 8 Regionen, 50 Kern- und 14 optionale Provinzen, Voronoi-Polygone, Nachbarschaftsgraph, Seewege |
| `model.rs` | Datenmodell (`Province`, `Realm`, `Character`, `Dynasty`, `NobleHouse`, `Army`, `War`, `Treaty`, `Plot`, `Market` …) |
| `world.rs` | `StrategyWorld`, `StrategyOptions`, Schwierigkeit, Regeln, Tagestakt |
| `scenario.rs` | Startzustand 897 n.F. (12 Reiche, 17 Adelshäuser, Kanon-Figuren), Wirtschafts-Einschwingen |
| `character.rs` | Charaktere, Nachfolge, Regentschaft, Heirat, Geburt, Tod, Beinamen, Dynastien/Vermächtnisse |
| `politics.rs` | Ämter, Häuser (Macht/Loyalität), Fraktionen, Rebellion, Gründung von Reichen, Intrigen |
| `diplomacy.rs` | Meinungsmodell, Verträge, Kriegsgründe, Krieg/Frieden, Bedrohung, Koalitionen, Großmacht |
| `ai.rs` | KI: langfristige Ziele, Finanzen, Rekrutierung, Kriegswille, Heerführung, Frieden, Diplomatie, Intrigen |
| `economy.rs` | Produktion, Verbrauch, Märkte, Handelsrouten, Zölle, Steuern, Kredite, Bauen/Verwaltung |
| `population.rs` | Zufriedenheit, Demografie, Seuchen, Migration, Spannungen, Unruhe, Aufstände |
| `religion.rs` | Bekehrung, Glaubensstärke, Häresie, Feiertage, Kirche (Bann) |
| `magic.rs` | Forschung, Rituale, Hohlungspfad, Schleierstärke, Artefakte, Orden, Götterschreine |
| `military.rs` | Heere, Pfade, Belagerung, Auto-Resolve, `BattleRequest`/`BattleResult` |
| `neutral.rs` | Monster, Banditen, Kultisten, Söldner, Hohle, Aschenlegion, Bosse, Waldkette |
| `events.rs`, `data.rs` | datengetriebenes Ereignissystem, RON-Laden und Mod-Overlays |
| `endgame.rs` | Großmacht-Reaktion, Schleierdruck-Eskalation, 8 Siegziele |
| `chronicle.rs` | Chronik mit Vorlagen, Kausalketten, Jahres-/Herrscherchronik |
| `command.rs` | `StrategyCommand` und Ausführung |
| `save.rs` | Speichern/Laden (Header, CRC32, atomar, Backup, Autosave) |

## 3. Datenmodell (Auszug)

* **Provinz** (`Province`): Schlüssel (`key`, z. B. `brennhollow`), Region, Schwerpunkt `centroid` und Grenzpolygon
  `polygon` in Kartenkoordinaten (1000 × 700), `neighbors` (`Link{to, kind: Land|Sea, dist}`), Biom, Küste, Vorkommen
  (`deposits`), Fruchtbarkeit, Siedlung (`Dorf → Stadt → Großstadt → Hauptstadt`), Gebäude (`(Index, Anzahl)`),
  Befestigung 0–10, Garnison, **8 Schichten** mit Kindern/Erwachsenen/Alten, Zufriedenheit und Erfüllungsgraden,
  Religionsanteile (6), Besitzer/Besatzer/Lehnshaus, Unruhe, **Hohlung** (0–100), **Schleierstärke** (0–5), Seuche,
  Spannungsachsen (5), Lager/Mangel/Produktion je Ware, Zustände (`states`), Kettenglieder (`chain`).
* **Reich** (`Realm`): Kanon-Fraktion, Art (`Kingdom`, `March`, `Clan`, `Covenant`, `Republic`, `Duchy`, `Free`, `Hollowed`,
  `Legion`, `Rebel`), Regierungsform, Rang, Herrscher, Dynastie, Hauptstadt, Lehnsherr, Staatskasse, Steuern/Zölle/Kredite,
  Gesetze (Kronautorität, Erbgesetz, Leibeigenschaft, Stadtrechte, Glaubensgesetz, Magiegesetz, Kriegsrecht, Abgaben,
  Heerfolge), **7 Ämter** samt aktiver Aufgabe, Prestige/Frömmigkeit/Tyrannei, Legitimität, Kriegsmüdigkeit, KI-Ziele und
  -Persönlichkeit, Regentschaft, Forschung (9 Schulen × Grad 0–4), Aschglas/Seelenasche, Hohlungspfad (0–5), Spionagenetze.
* **Charakter** (`Character`): Name je Kultur, Geschlecht, Geburtstag, **6 Attribute**, Gesundheit, Fruchtbarkeit,
  **Eigenschaften** (`catalog::Trait`: 16 Persönlichkeiten + 21 erworbene/angeborene), Fähigkeiten, Prestige, Frömmigkeit,
  Hohlung, Stress, Ehe/Kinder/Eltern, Anspruch, Geheimnisse, Erinnerungen, Beiname, Todesursache.
* **Heer** (`Army`): Regimenter mit **Einheiten-ID**, Anzahl, Veteranenstufe, Lebenspunkten; Standort, Pfad, Moral,
  Versorgung, Befehlshaber, Art (`Realm`, `Rebels`, `Monster`, `Bandits`, `Mercenary`, `Cultists`, `Legion`, `Hollow`, `Boss`).
* Alle IDs sind Indizes in `Vec`s (`ProvId`, `RealmId`, `CharId`, …); Provinzschlüssel und Reichsschlüssel sind stabil
  (`prov_by_key`, `realm_by_key`).

## 4. Welt Eldraeth

Acht handgefertigte Regionen (`valemark`, `ashmarch`, `skarnheim`, `thornwild`, `saelith`, `aureth_wastes`, `veilscar`,
`cinderreach`) mit **50 Kernprovinzen** und **14 optionalen**, die der Seed einzeln auswählt (50–64 Provinzen).
Lage der Provinzen wird je Seed leicht verschoben, Vorkommen variieren (±1 Stufe, Zusatzvorkommen, Schleierwerte), die
Polygone entstehen als Voronoi-Zellen (Inseln begrenzt, Provinzradius 86/46). Nachbarschaft: gemeinsame Kanten innerhalb
der Region, **Tore** zwischen Regionen (z. B. Brennender Pass, Wächtertor, Dornpforte), **Seewege** zu den Saelith-Inseln
(`Strathhaven ↔ Thals Auge/Selvaris`, `Emberth ↔ Varo` …) und zwischen Inseln; Zusammenhang wird erzwungen.
Marktgebiete: je Region eines (Schleiernarbe ohne Markt).

Startzustand (897 n.F.): `ember_crown` (König Aldric III., Erbin Elyane, Spionagemeister Kassian Vale; Häuser Corvane,
Melbrecht, Strathgar, Ilverholt, Varenne), `ashmarch_march` (Isolde Raventhorn, Vasallin der Krone, Hal Venn als Hohepriester,
41 Bewaffnete, Brennhollow mit ca. 30 Einwohnern), `skarn_clans` (Torvald Eisenhand), `wolfspass_clan` (Vasall),
`thorn_covenant` (Sylaen der Veyl), `veyl_free`, `saelith_compact` (Ophira Saelith-Varo, neun Händlerhäuser),
`free_port_emberth`, `grey_nomads`, `slag_brood` sowie die nicht verhandelbaren `hollowed` und `ashen_legion`.
Startbeziehungen entsprechen der Matrix der Weltbibel 4.0.

## 5. Zeit und Wirtschaft

**Produktion.** Je Gebäudetyp (`goods::BUILDINGS`) gilt: Zyklen pro Monat = `240 s / Zykluszeit`; Output ∝ Anzahl ×
Besetzung (Arbeitskräfte der Schicht) × Produktivität (Zufriedenheit: 0,5/0,8/1,0/1,1/1,2) × Vorkommen/3 × Saison
(Feldfrüchte: Winter 0, Frühling 0,8, Sommer 1,2, Herbst 1,6–2,0) × Ernte-Index × Besetzungsfaktor. Eingaben kommen aus dem
Provinzlager; Fehlmengen werden als Nachfrage am Markt angemeldet. Besetzte oder belagerte Provinzen produzieren zu 30 %.

**Bedürfnisse.** Pro 100 Bewohner und Monat (`goods::needs_of`, Werte aus GDD 9.3); Nahrung als Nahrungskonto (Nährwerte,
Vorrat über drei Monate wird gehalten). Erfüllungsgrade Grund/Komfort/Luxus je Schicht.

**Märkte** (`Market`, GDD 8.6): `P = P_basis × clamp((N/A)^0,7; 0,25; 4) × (1+Inflation) × Ereignisfaktor`, monatlich
auf ±25 % begrenzt. `N` = Verbrauch + Werkstattbedarf, `A` = Produktion + Importe + 0,2 × freie Lager. Defizite werden aus
freien Beständen der Region gedeckt, Rest = Knappheit (`shortage`). **Handelsrouten** zwischen Marktgebieten
(`TradeRoute`): Waren fließen, solange `P_Ziel − P_Quelle > Transport + Zoll` (Transport 2 % je 10 Tage), Volumen
× `(1 − Zoll × 1,5)`, Winter hemmt Seewege; Krieg, Boykott, Hohle und das Welken (Dornwald) sperren Routen; Zölle und
Handelssteuer gehen an den Zielreich.

**Steuern** (`economy::finances_month`): Kopfsteuer je Schicht × Stufe (×0 … ×2) × Zufriedenheitsfaktor, Handelssteuer, Korruption
`5 % + 1 % je Provinz über dem Verwaltungslimit − Schatzmeister × 0,5 %`; Lehen geben `vassal_tax` ab, Vasallen zahlen
20 % des Nettos, Tribute 10 %. Ausgaben: Heer, Gebäude, Hof, Gehälter, Zinsen. **Kredite** (`take_loan`): Saelith-Banken
`4 % + 5 % × Schulden/Einkommen`, Kirche 2 %, Häuser 6 %, Kriegsanleihe 3 %; Bankrott bei tiefen Schulden
(Kredite gestrichen, 20 Jahre kein Kredit, Moral/Zufriedenheit/Prestige sinken).

## 6. Bevölkerung

`population.rs` (GDD 9): Zufriedenheit nach der Formel `Z = 40 + 30·G + 20·K + 10·L − 25 (G<0,5) − Hunger + Steuer +
Gesetze + Sicherheit + Glaube + Herrscher + Vielfalt − Überfüllung − Kriegsmüdigkeit/5 − Hohlung/5 ± Ereignisse`
(Glättung 50 % je Monat). Demografie: Geburten `Erwachsene × 7 %/12` (Faktor Nahrung/Wohnraum), Sterbefälle nach
Altersklasse (1,5 % / 1 % / 8 %, × Seuche, × Hunger, × 0,8 mit Hospital), Altern 1/16 und 1/39, Aufstieg/Abstieg der
Schichten, Nachwachsen der Sonderschichten. Migration folgt Zufriedenheit/Hunger; Flüchtlinge entstehen durch Besetzung
und Raubzüge. **Seuchen**: Seuchendruck aus Hygiene, Hunger, Belagerung; Ausbruch ab 60, Ausbreitung entlang der
Nachbarn, Ende nach Monaten; Namen Aschenfieber, Grauhusten, Schwarze Schleimpest, Magische Seuche.
**Spannungsachsen** (5): Wachstum durch Treiber, Abbau 5 %/Monat + 1; ≥ 40 Streit, ≥ 60 Streik, ≥ 75 Unruhen,
≥ 90 Aufstand. **Unruhe**: `+ (50 − Ø Z)/10 + Σ Spannung/20 − Garnison/Bevölkerung × 60 − Statthalter`; Aufstandsprüfung
`(20 − Z) × 1 % × (1 + Unruhe/100) − Garnison/Bevölkerung × 50 %` für Schichten mit ≥ 12 % Anteil. Rebellen (`ArmyKind::Rebels`)
besetzen Provinzen; nach zehn Monaten entsteht ein eigenes Reich.

## 7. Charaktere, Dynastien, Nachfolge

* Namen je Kultur (10 Kulturen), 6 Attribute (Mittel 7), Eigenschaften mit Attributsmodifikatoren und Stress-Auslösern
  (`Trait::stress_act`), Hohlung, Stress (Zusammenbruch → `melancholic`/`lunatic`).
* Altern: Sterblichkeit `0,3 % × 1,09^(Alter − 30) × (11 − Gesundheit)/6`; Todesursachen `Natural`, `Battle`, `Murder`,
  `Disease`, `Executed`, `Deposed`, `Abdicated`, `Accident`.
* Geburten: `Fruchtbarkeit_A × Fruchtbarkeit_B × 0,9 × 0,30`, ab 40 −10 %/Jahr; Vererbung: Attribut = Mittel der Eltern ± 3,
  angeborene Eigenschaften erblich (Genie 15 %), Glutblut-Vermächtnis → Flammenweihe.
* Erziehung: ab 6 Jahren Fokus und Erzieher, mit 16 Bonus `(Attribut_Erzieher − 8)/2` und Erziehungseigenschaft.
* **Erbgesetze** (`SuccLaw`): Primogenitur, Ultimogenitur, Teilung (Teilreich für Geschwister), Wahlmonarchie, Seniorat,
  Ernennung (Legitimität ≥ 70). Republiken/Clanräte/Druidenzirkel wählen. Ohne Erben übernimmt das stärkste Haus
  (**Dynastiewechsel**); Dynastien erlöschen (`Dynasty::extinct`), Kadettenhäuser entstehen aus verheirateten Nebenlinien.
* **Legitimität** bei Thronbesteigung: `50 + 10 × Generation (max. 30) + 20 (Erbe nach Gesetz) − 30 (Usurpation)`, +1/Jahr.
* **Regentschaft** bei Erben < 16 Jahre oder Handlungsunfähigkeit: Ehepartner/Elternteil → Kanzler → mächtigstes
  Dynastiemitglied; monatliche Usurpationschance `max(0, (50 − Loyalität)/10) % × (1 + Ehrgeizig)`; gesperrt:
  Kriegserklärung, Erbgesetz, Steuersprung > 1 Stufe.
* Heirat: Annahmeformel GDD 5.7 (`propose_marriage`), Bündnisheirat erzeugt Familienbund (+25 Meinung).
* Beinamen aus Taten (`compute_epithet`), Dynastieprestige und fünf Vermächtnisse (`character::LEGACIES`).

## 8. Politik und Intrigen

* **Adelshäuser** (`NobleHouse`): Macht (Fläche der Lehen + Gefolge + Vermögen), Loyalität nach GDD 6.1
  (`50 + Meinung × 0,4 + 5 × Rechte − (Abgabe − 20) − (Heerfolge − 25) × 0,8 + Legitimität × 0,1 − 5 − 15 (Thronanspruch)
  + 10 (Amt)` plus Gesetze, Tyrannei, Großmacht, Regentschaft, Kriegsmüdigkeit), Gesinnung, Rechte, Gefolge.
* **Aktionen** (`HouseAction`): belohnen, Rechte gewähren/entziehen, Lehen verleihen/enteignen, bestrafen, verheiraten,
  ins Amt/den Rat berufen. Adelsrat (`council_approves`): Mehrheit nach Macht; ohne Zustimmung −20 Loyalität der Gegner
  und 200 Prestige.
* **Fraktionen**: Loyalität < 30 → Fraktion mit Forderung; Ultimatum bei Stärke ≥ 1,2 × Kronstärke; Nachgeben/Ablehnen
  (Bürgerkrieg). **Offene Rebellion** bei Loyalität < 15 und Macht > 10: `house_rebellion` gründet ein Reich im Krieg
  gegen den Lehnsherrn (`found_realm`).
* **Ämter** (7): Grundeffekte im Code (Wirtschaft, Militär, Diplomatie, Religion) und je drei Aufgaben nach GDD 6.3;
  Gehalt `10 + Attribut`.
* **Intrigen** (`PlotKind`): Attentat, Entführung, Rufmord, Anspruch fälschen, Aufwiegeln, Putsch, Diebstahl, Verführung,
  Spionage, Bestechung, Verrat, Geheimes Bündnis, Erpressung, Verschwörung. Formeln wörtlich nach GDD 6.6:
  `K = Intrige_Anführer × 3 + Σ(Intrige_Mitverschwörer + Macht × 0,5) + Netzwerk × 0,3` (Verschlagen +15 %, Ehrlich −15 %),
  `A = Spionagemeister × 3 + Intrige_Ziel × 1,5 + Leibwache + 20 (Misstrauisch)`, `P = clamp(K − A − Schwierigkeit + 50; 5; 95)`,
  Fortschritt `K/10` je Monat, Entdeckung `A/(K+A) × 8 % × (1 + 0,1 × n)`. Folgen: Tod, Nachfolge, Geheimnisse
  (Mord), Gefangennahme, Meinung −50 (30 Jahre), Erinnerungen. KI-Komplotte folgen Zielen und Persönlichkeit; das
  Spielerreich löst manuell aus (`TriggerPlot`).

## 9. Diplomatie und KI

* **Meinung** (`opinion(a, b)`, −100…+100): Persönlichkeit, Diplomatie von B, Religion/Kultur, Familienbund, Blutsverwandtschaft,
  Modifikatoren mit linearem Abklingen (`OpinionMod`: Geschenk 5 J., Gesandter 10 J., Krieg 20–40 J., Vertragsbruch 50 J.,
  abgelehnter Bündnisruf 10 J. …), gemeinsame Feinde, Grenzspannung, Ansprüche, Bedrohung × −0,5, Friedensruf, Tyrannei,
  Hohlung ≥ 40.
* **Verträge** (`TreatyKind`): Bündnis, Verteidigungspakt, Nichtangriff, Handel, Heirat, Tribut, militärischer Zugang,
  Garantie, gemeinsamer Feldzug, Waffenstillstand, geheimes Bündnis, Front gegen das Tief. Aktionen in
  `DiplomaticAction` inkl. Gesandte, Geschenke, Tribut fordern, Vasallisierung, Garantie, Ultimatum, Boykott,
  Aufstände unterstützen, Bündnisruf, Koalition beitreten.
* **Krieg**: Kriegsgründe (Anspruch, Heiliger Krieg, Unabhängigkeit, Grenzkonflikt, Vergeltung, Hohlland, Koalition,
  Thron, ohne Grund: 300 Prestige + 30 Tyrannei); Kriegsziele (`WarGoal`); **Kriegspunkte** −100…+100 aus Schlachten,
  Besetzungen und gehaltenem Kriegsziel; Kriegsmüdigkeit. Friedensbedingungen kosten Kriegspunkte (`peace_cost`); die KI
  nimmt an, wenn `Punkte ≥ Kosten` oder `Müdigkeit > 70 ∧ Kosten ≤ Punkte + 20`. Waffenstillstand 5 Jahre.
* **Bedrohung & Koalitionen**: `Bedrohung = Eroberungen (30 J.) × 5 + (Stärke/Ø − 1) × 20 + Vertragsbrüche × 10 + Leermagie × 15`,
  Nähefaktor 1/0,6/0,3. Ab 50 mögliche, ab 75 eintretende Koalitionsmitglieder; Präventivkrieg bei ≥ 1,5 × Stärke.
* **Langfristige KI-Ziele** (`GoalKind`): Überleben, Expansion, Thron von Valemark, Reichtum, Glaubensreinheit,
  Unabhängigkeit, Rache, Schleier schließen, Gleichgewicht, **Rückeroberung, Handelsweg, Religion verbreiten, Rivalen
  schwächen, Dynastie sichern, Erzfeind vernichten**. Jährlich neu bewertet (bis zu 3, mit Trägheit); sie steuern
  Kriegswille, Verträge, Komplotte, Ämteraufgaben, Rekrutierung und Zölle. **Kriegswille** nach GDD 7.4:
  `Nutzen × Stärkeverhältnis × Aggression − Meinung × 0,3 − Müdigkeit × 0,5`, Schwelle 40 (Eroberer 30, Bewahrer 60).

## 10. Religion, Magie, Hohlung

* **Religion** (6): Bekehrung monatlich `(Missionarsstärke − Widerstand) × 0,5 %`; Missionar `Gelehrsamkeit × 0,5 + Tempel × 2
  + Eifer/10 + 5`, Widerstand `alter Anteil × 10 + heilige Stätte + Z/20`; Toleranzgesetze (Staatskirche/Duldung/Freiheit),
  Zwangsbekehrung verdoppelt (+Spannung), Verbot und Wachstum der Leeren Liturgie mit der Hohlung, Feiertage (Fest
  → Zufriedenheit), emergente **Sonnenhäresie** (`heresy_year`), Erzflammenwahrer (`archflame_opinion`, Bann
  bei < −50), Glaubensstärke (Anteil × Tempelversorgung), Eifer je Religion.
* **Magie**: 9 Schulen × 4 Grade (Punkte 200/600/1500/4000, Aschglas 5/15/40/100); Forschungsrate nach GDD 12.4
  (Gelehrte × 0,5 + Bücher × 10 + Kanzler × 2, Magiegesetz, +33 % in verehrten Schulen); verbotene Schulen: 30 %
  Entdeckung je Grad. **Rituale** (`catalog::RITUALS`: Läuterndes Feuer, Ewige Wacht, Bannkreis, Siegel, Totenruhe,
  Ernteritual, Gezeitenfrieden, Runenwall, Resonanzfeld, Flüsternetz, Ruf aus dem Tief, Schleierriss) kosten Gold und
  Aschglas, einmal pro Monat, mit Risiko (Schleier, Hohlung). **Artefakte** (8) werden im Besitz der Fundprovinz
  geborgen; **Magierorden** (8). **Götterschreine**: Wiederherstellung (5.000 Gold, 200 Aschglas, Held vor Ort, 90 Tage,
  nur ab Endzeit).
* **Schleierstärke** je Provinz (0–5): `+ 0,01 × (Ø Nachbarn − eigene) − Glaubensstärke − Wachfeuer + globaler Druck + Abbau`;
  Aschglasabbau hebt Schleier und Hohlung; ab 3 entstehen Hohlschreine.
* **Hohlungspfad** (`Realm::hollow_path`, 0–5): `Reichshohlung = 0,5 × Herrscher + 0,3 × Ø Provinz + 0,2 × min(100, Leermagie(10 J.) × 5)`,
  Schwellen 10/25/45/65/85. **Vorteile**: Stimme aus dem Tief (Ereignisse, Forschung), Seelenasche (ab 2), Aschglas
  +50 % (ab 2), Hohlknechte aus Seelenasche (ab 3), Herrscher altert nicht (ab 4), Siegziel „Die Hohle Krone“ (5).
  **Nachteile**: Meinung −40/−60, Exkommunikation, Bund der Letzten Flamme gegen Stufe 5, Fruchtbarkeit (ab 4),
  Wahnsinn des Herrschers bei Hohlung ≥ 60.

## 11. Ereignisse (Daten, Mods)

Dateien: `assets/data/world/events/*.ron` (eingebettet in das Crate, identisch geladen über
`StrategyData::builtin()`; Mods: `StrategyData::load_overlay(dir)` ersetzt Einträge gleicher `id` und fügt neue hinzu,
`StrategyWorld::new_with_data` nutzt sie). Aktuell **53 Ereignisse** in sechs Dateien (Natur, Wirtschaft, Menschen,
Politik, Glaube/Magie, Bedrohungen).

```ron
(
    id: "drought_province", scope: Province,            // Province | Realm | Character
    title: "Dürre in {province}", text: "...",           // Platzhalter {province} {realm} {ruler}
    conditions: [VarBelow("rainfall_ratio", 0.6), DryMonths(3), Season([Spring, Summer]), NotState("drought")],
    mtth_months: 1.0, cooldown_months: 12, chronicle_weight: 30,
    sets_state: Some(("drought", 8)),
    options: [
        (id: "import_grain", text: "...", cost: 120.0,
         effects: [AddGold(-120.0), AddStock("grain", 320.0)],
         ai_weight: 4.0, ai_when: [(VarAbove("gold", 1500.0), 6.0)], requires: []),
    ],
)
```

* **Bedingungen** (`data::Cond`): `VarBelow/VarAbove(Variable, Wert)`, `DryMonths`, `Season`, `HasState`, `NotState`, `Chance`,
  `Religion`, `Faction`, `YearFrom`, `RulerTrait`, `RulerNotTrait`, `AtWar`, `Biome`, `Coastal`, `PlayerOnly`.
* **Variablen**: Provinz – `rainfall_ratio`, `dry_months`, `harvest`, `food_ratio`, `unrest`, `satisfaction`, `hollowing`,
  `veil`, `disease`, `hygiene`, `pop`, `devastation`, `fortification`, `grain_price_ratio`, `tension_max`, `fertility`,
  `forest_cover`, `refugees`, `garrison_ratio`, `faith_strength`, `settlement_rank`, `shrine`, `hollow_force`, `scholars`;
  Reich – `gold`, `net_income`, `legitimacy`, `war_weariness`, `tyranny`, `prestige`, `piety`, `provinces`,
  `house_loyalty_min`, `debt_ratio`, `tax_level`, `ruler_age`, `ruler_health`, `ruler_stress`, `ruler_hollowing`, `heir_age`,
  `regency`, `threat`, `great_power`, `hollow_path`, `ashglass`, `plague_provinces`, `hungry_provinces`, `veil_pressure`, `year`,
  `at_war` (Provinzvariablen sind im Reichsscope als Durchschnitt verfügbar).
* **Effekte** (`data::Effect`): Gold, Prestige, Frömmigkeit, Legitimität, Tyrannei, Unruhe, Zufriedenheit, Lager, Preisfaktor,
  Zustände, Hohlung, Schleier, Bevölkerung, Befestigung, Hauslloyalität, Stress, Gesundheit, Eigenschaft, Rebellen,
  Seuche, Kriegsmüdigkeit, Nachbarmeinung, Krankheitsdruck, Verwüstung, Aschglas, Forschung, neutrale Gruppen,
  Glaubensanteile, Spannung, Flüchtlinge, Notiz, Fruchtbarkeit, Herrscherhohlung, Handelsschub, Siedlungsausbau, Söldnerheer,
  Spionagenetz, Nahrung, Beliebtheit.
* Die Auslösewahrscheinlichkeit je Monat ist `1 − exp(−1/mtth) × Ereignisdichte`. Für das Spielerreich entsteht ein
  `PendingEvent` (Optionen mit „bezahlbar“-Flag); `StrategyCommand::ChooseEvent` wendet die Optionswirkung an.
  Unbeantwortete Fenster werden nach 60 Tagen per KI-Gewichten entschieden. KI-Reiche wählen sofort über `ai_weight` +
  bedingte Zuschläge (`ai_when`). `StrategyWorld::force_event` löst Ereignisse gezielt aus (Mods, Tests).

**Kausalketten sind keine Skripte.** Dürre → Missernte → Preise → Unzufriedenheit → Unruhe → Aufstand → gebundene Armee →
Nachbarkrieg entstehen aus gekoppelten Systemen (`weather_month`, `economy_month`, `satisfaction_of`, `order_month`,
`ai_wars`); Ereignisfenster sind Interventionspunkte. Kettenglieder werden in der Chronik über `causes` verknüpft
(`Chronicle::chain`), z. B. `drought → event_generic (Brotpreis) → famine → unrest → revolt`.
Weitere Ketten: Seuche (Hunger → Ausbruch → Ausbreitung), Schleier (Aschglas → Schleier → Hohlschreine → Flüchtlinge),
Wald (Abholzung → Zorn der Wurzelmutter → Erwachen → Krieg des Dornenbundes, `neutral::forest_month`), Drache
(Flug → Ernteausfall in drei Provinzen → Tribut), Handel (Seegeister des Schwarzen Riffs → Preise für Gewürze/Seide).

## 12. Neutrale Akteure, Bosse, Endgame

* **Neutrale** (`NeutralGroup`): Aschwölfe, Grabwürmer, Glutsalamander, Moorlichter, Banditen, Kultisten, Aschglas-Sucher,
  Veyl-Sippen, Flüchtlingszüge, Ruinen; Raubzüge gegen Garnison/Heer, Verluste, Handelsstörung. **Söldnerkompanien**
  (`hire_mercenaries`: drei Monate Sold im Voraus; bei ≥ 3 Monaten Verzug Seitenwechsel oder Banditen).
* **Hohle**: Schreine (`Province::shrine`) erzeugen `5 + Schleier × 3` Hohle/Monat bis `40 × Schleier`; über 50 % des Limits
  entstehen Heere, die lebende Nachbarprovinzen erobern (die Provinz wird dann Hohlland, Hohlung +20). Im Kriegszustand mit
  allen Lebenden (`at_war`), nicht verhandelbar. **Aschenlegion**: schlafende Garnisonen, erwacht bei Schleierdruck ≥ 40.
* **Bosse** (`Boss`, jeweils mit eigenem `Army` aus Boss-Einheit): `boss_pale_reeve`, `boss_ashen_dragon` (Flüge alle 3–7
  Jahre, Tribut 400 Gold + 5 Aschglas), `boss_root_mother`, `boss_hollow_king`, `boss_stone_oath` (Gharun),
  `boss_reef_mistress` (Isarre Mol). Bosse sind gegenüber dem Herrn der Provinz und seinen Verbündeten friedlich.
* **Endgame**: Start bei Großmacht (≥ 30 % der Provinzen direkt oder doppelte Stärke + 40 % mit Vasallen), Schleierdruck
  > 60 oder Jahr 1000. Großmacht-Reaktion: Pakt der Kronen (erzwungene Koalition), innere Last, Separatismus,
  Handelsboykott (20 % Zoll), kirchliche Wachsamkeit. **Schleierdruck** (0–100): +1/5 Jahre, +0,5/J. je Provinz mit
  Schleier ≥ 4, −10 je Götterschrein, −5 Bosswächter; Stufen 40 (Legion), 60 (Hohler König, Bund der Letzten Flamme,
  jährlicher Marsch mit `300 + 10 × Jahre`), 70 (Drachenzorn), 80 (Welken), 100 (Schleierbruch: 3 Provinzen/Jahr, nach 30
  Jahren `VictoryKind::WorldFalls`). Magische Seuche ab 55.
* **Siegziele** (`VictoryKind`, `victory_progress(realm)`): Krone von Eldraeth, Der Hohle König fällt, Siegel des Schleiers,
  Handelsherrschaft, Glaubenssieg, Dynastische Legende, Die Hohle Krone, Überleben. Aktive Ziele je Partie über
  `RuleSet::victories`; der erste erfüllte beendet die Partie (`StrategyWorld::victory`), Weiterspielen ist möglich.

## 13. Chronik

`Chronicle` (`entries`): fortlaufende ID, Datum, `ChronicleKind` (28 Kategorien), Akteure, Provinz, `magnitude` 0–100,
`causes`, `numbers`, `tags`, Vorlage, **fertiger Erzähltext** (Jahr, Monat, Akteure; mindestens 6 Varianten je Vorlage (Test `every_template_has_at_least_six_distinct_variants`), Variante
deterministisch aus der ID), Sichtbarkeit (`Public`/`Secret`). `year(y, min)` liefert Jahres-/Herrscher-/Zeitalterchronik
(≥ 25/50/75), `narrative(min)` Fließtext, `chain(id)` die Kausalkette. Geheime Taten (Mord) erscheinen als Wirkung, der
Urheber bleibt `Secret`. Die Bedeutsamkeit wird durch Folgeereignisse nachträglich aufgewertet (+5 je Folge, max. +30).

## 14. Schlachten und Integration in die Echtzeitebene

Treffen feindliche Heere in einer Provinz aufeinander, entsteht eine Schlacht. Betreten Heere Feindesland, wird besetzt
(unbefestigt) oder belagert (Fortschritt `Belagerungsstärke − Befestigung × 2 %` pro Tag, mindestens 0,5; Geräte × 3,
Pioniere × 1; ohne Nahrung −2 %/Tag Garnison; `StrategyCommand::StormSiege`). Zwei Betriebsarten (`RuleSet::battle_mode`):

1. `BattleMode::AutoResolveAll`: sofortige Auflösung nach GDD 4.6 (Runden à 10 s, `KK = (DPS × HP)^0,5 × Veteranenfaktor`, Moral,
   Gelände, Befestigung, Kommandant, Wetter, Nacht ±, Zufall 0,85…1,15, Heldenverletzung, Verluste `KK_Gegner × 0,8`).
2. `BattleMode::PlayerBattlesRequested`: Schlachten mit Beteiligung des Spielerreichs werden als `BattleRequest` hinterlegt
   (`pending_battles()`), beteiligte Heere stehen still (`Army::in_battle`).

```rust
let req: &BattleRequest = &world.pending_battles()[0];
// req.province / province_key / terrain (Biome) / season / weather / hour / siege / fortification / breaches
// req.attackers / req.defenders: Vec<BattleArmy { army, realm, regiments: Vec<Regiment{unit, count, vet, hp}>, commander, heroes, morale, supply, power }>
// req.preview_loss_attacker / preview_loss_defender (Erwartungswert %)
// a) automatisch berechnen:
world.auto_resolve_pending(req.id)?;
// b) in der Echtzeitebene spielen und zurückmelden:
world.apply_battle_result(BattleResult {
    id: req.id, winner: BattleSide::Attackers,
    armies: vec![ArmyResult { army: a, regiments: survivors, retreated: false }, /* … */],
    heroes: vec![(char_id, HeroOutcome::Wounded)], duration_minutes: 14.0, city_taken: false,
})?;
```

Die Übernahme (GDD 4.7): Überlebende exakt pro Regiment, 30 % der Gefallenen kehren nach 30 Tagen heim, Schlachtdauer wird als
1/2/3 Tage Sperre verbucht, Moral +10/−20, Veteranen, Helden (verletzt/gefangen/tot), Beute (10 % des Unterhaltswerts),
Kriegspunkte, verbrannte Erde (> 100 Tote), Hohlung bei Schleier ≥ 3, Chronik, Rückzug des Verlierers, Sturm auf Städte
(`city_taken`). Unbeantwortete Anfragen werden nach 30 Tagen automatisch berechnet. Die Weltzeit läuft während einer gespielten
Schlacht nicht weiter (die Oberfläche ruft `tick_day` nicht auf); `apply_battle_result` verbucht die Dauer.

## 15. API für die Spielintegration

```rust
use ashen_strategy::*;
let mut world = StrategyWorld::new(StrategyOptions::campaign(seed)); // Isolde, Wächterin der Aschmark
// Optionen: PlayerStart::{AshmarchWarden, Realm("skarn_clans"), None}, Difficulty::{Pilgrim..Ashfall, Custom},
// RuleSet::{campaign(), sandbox(vec![VictoryKind::..]), challenge(Challenge::{NoBlood, CadetHouse, Survival(j)})}
world.tick_day();                                      // oder advance_days(n)
let r = world.execute(StrategyCommand::Recruit { province, unit: "spearman".into(), count: 10 });
for e in world.drain_events() { /* StrategyEvent::{Notice, EventWindow, BattleRequested, RulerChanged, WarDeclared, … } */ }
let events = world.pending_events();                   // Ereignisfenster
let battles = world.pending_battles();                 // Schlachtanfragen (PlayerBattlesRequested)
let progress = world.victory_progress(player);         // Siegfortschritt
save::save_to(dir, "slot", &world)?;                   // *.acstrat
let (header, world) = save::load_from(path, StrategyData::builtin())?;
```

Lesende Sicht für die Oberfläche: `world.provinces` (Polygon, Schwerpunkt, Besitzer, Bevölkerung, Religion …),
`realms`, `armies`, `wars`, `treaties`, `chars`, `houses`, `markets` (`prices`), `chronicle`, `endgame`, `price_in(prov, good)`,
`opinion(a, b)`, `plot_success_chance`, `war_will`. Befehle (`StrategyCommand`): Steuern/Zölle/Gesetze, Ämter und Aufgaben,
Forschung, Kredite, Bauen, Befestigen, Rekrutieren, Heere bewegen/teilen/vereinen/auflösen/Befehlshaber, Sturmangriff,
Söldner, Diplomatie (`DiplomaticAction`), Friedensangebote beantworten, Hausaktionen, Komplotte (starten, anwerben,
auslösen), Heiraten, Erben bestimmen, Erziehung, Abdanken, Rituale, Schreine wiederherstellen, Drachentribut,
Ereignisoptionen wählen. `execute_for(realm, cmd)` erlaubt mehrere Spieler.

Kopplung zur RTS-Ebene (GDD 4.3/4.5): Provinzdaten (`buildings`, Lager, Bevölkerung nach Schicht, Zufriedenheit, Hohlung,
Schleier, Wetter) und Heeres-/Heldendaten (Einheiten-IDs aus `units.ron`, `Character::is_hero`) sind direkt lesbar. Die
strategische Welt kennt zusätzliche Kreaturen-/Boss-IDs (`ash_wolf`, `grave_worm`, `glow_salamander`, `moor_light`,
`boss_ashen_dragon`, `boss_root_mother`, `boss_hollow_king`, `boss_stone_oath`, `boss_reef_mistress`), die nur als Daten
im Katalog (`catalog::UNITS`) existieren.

## 16. Speicherformat

`save.rs`: `"ASTR" | u32 Formatversion | u32 Header-Länge | Header (RON) | u32 CRC32 | u64 Länge | Payload (bincode)`.
Der Header enthält Seed, Jahr, Tag, Spielerreich, Schwierigkeit, Modus, Zeitstempel, Prüfsumme der Ereignisliste
(Mods) und `state_hash`. Atomares Schreiben (tmp → fsync → `.bak` → rename), automatischer Backup-Versuch bei Beschädigung,
rotierende Autosaves, Spielstandliste. Die Daten (`StrategyData`) werden nicht gespeichert; beim Laden wird die
Ereignisliste verglichen (`SaveError::Incompatible`).

## 17. Erweiterung per Daten/Mods

* Ereignisse: RON unter `events/` (siehe Abschnitt 11) – ersetzen/ergänzen ohne Code.
* Tabellen im Code mit einfachem Anhängen neuer Einträge: Waren/Gebäude (`goods.rs`), Einheiten/Rituale/Artefakte/Namen
  (`catalog.rs`), Provinzen (`mapgen.rs`), Reiche/Häuser (`scenario.rs`), Chronikvorlagen (`chronicle::TEMPLATES`).
* Neue Siegziele: `VictoryKind` + `victory_progress`; neue KI-Ziele: `GoalKind` + `choose_goals`/`war_will`.

## 18. Tests

`cargo test -p ashen_strategy` (34 Integrationstests in `tests/world.rs`): Karte (50–64 Provinzen, zusammenhängend, Seewege),
Kanonwelt, Determinismus (gleicher Seed + Befehle → gleicher Hash), 200-Jahre-Lauf mit Kennzahlen, Nachfolge/Regentschaft/
Erlöschen, Krieg→Frieden-Zyklus, Friedensbedingungen, Koalition gegen Übermacht, Preisreaktion auf Knappheit,
Kausalkette Dürre → Unruhe, Intrigenformeln und -folgen, Hausaktionen/Rebellion, Speichern/Laden/Backup/Beschädigung,
Ereignisdaten/Mod-Overlay/Optionswirkung, Schlachtübergabe (`BattleRequest`/`BattleResult`), Auto-Resolve, Heerbewegung
und Belagerung, Religion, Magie/Hohlungspfad, Neutrale/Bosse, Endgame/Siege, Spielmodi, Spielerbefehle, Chronik,
Einheitenkatalog gegen `units.ron`.

## 19. Balancemessung der Siegbedingungen

Reproduzierbar mit `cargo run --release -p ashen_tools -- strategy-stats --seeds 12 --years 200` (Seeds 1–12, reine KI-Läufe ohne
Spielerreich, Schwierigkeit Lord; derselbe Lauf als Test `observer_victories_are_rare_and_late`). Die Läufe enden beim ersten Sieg.

| Stand | Läufe mit KI-Sieg (12 Seeds, 200 Jahre) | Früheste Entscheidung | Siege nach Art |
|---|---|---|---|
| Vorher (Krone 60 %, Handel 50 % / 100 000 Gold / 120 Monate) | 9 von 12 (75 %) | Jahr 21 nach Spielbeginn | Krone von Eldraeth 7, Handelsherrschaft 2 |
| Nachher | 2 von 12 (17 %) | Jahr 80 | Krone von Eldraeth 1, Glaubenssieg 1 |

Änderungen (`endgame.rs`, Konstanten `CROWN_SHARE`, `CROWN_HOLD_MONTHS`, `TRADE_*`, `AI_VICTORY_MIN_YEARS`):

* **Krone von Eldraeth**: 70 % der Provinzen (vorher 60 %) und Hochkönigstitel müssen **144 Monate (12 Jahre)** gehalten werden (neuer Zähler `crown_hold`, bricht bei Unterschreitung ab). Der Fortschrittsbalken zeigt Anteil und Haltedauer.
* **Handelsherrschaft**: Handelsanteil 60 % (vorher 50 %), Schatz 250 000 (vorher 100 000), **180 Monate** gehalten (vorher 120).
* **Reifezeit der KI**: KI-Reiche können frühestens nach `AI_VICTORY_MIN_YEARS` = 80 Jahren siegen; für das Spielerreich gilt die Sperre nicht.

Die Schwellen sind die einzigen Stellschrauben; Test und Werkzeug schlagen bei mehr als einem Drittel KI-Siegen oder einem Sieg vor Jahr 80 an.
