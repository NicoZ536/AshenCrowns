# ASHEN CROWNS – Echtzeit-Wirtschaft und Städtebau

Dieses Dokument beschreibt die Wirtschafts-, Bevölkerungs- und Befestigungssysteme der Echtzeitebene
(Roadmap-Phasen 6/7). Grundlage ist das GDD (Kapitel 8 Wirtschaft, 9 Bevölkerung, 10.10 Belagerung); hier steht,
was davon **tatsächlich im Spiel läuft**, mit allen Zahlen aus den Datendateien `assets/data/{goods,buildings,units,rules}.ron`.
Datenfelder und Typen sind in `docs/tech/data_model.md` (Abschnitt 1.4–1.6) beschrieben.

## 1. Überblick

```
Bäume ─Holzfäller─▶ Holz ─Sägewerk─▶ Bretter ─────────────┐ (Baukosten, Schmiede)
Lehm ─Ziegelei◀─ Kohle ◀─Köhlerei◀─ Holz                  │
Eisenerz ─Schmelzhütte◀─ Kohle ─▶ Eisenbarren ─▶ Schmiede ─▶ Waffen / Werkzeug
                                          └──▶ Rüstkammer ─▶ Rüstung        (Ausbildung von Soldaten)
Getreide ─Mühle─▶ Mehl ─Bäckerei─▶ Brot        Getreide ─Brauerei─▶ Bier
Wolle ─Weberei─▶ Tuch ─Schneiderei─▶ Kleidung  Kräuter ─Tempel (Weihrauch)─▶ Glaube
                                  Bewohner ─▶ Arbeitskräfte, Steuern (Gold)
```

* Es gibt **einen Lagerbestand je Spieler** (kein Einzeltransport). „Transport“ ist vereinfacht über die
  **Lagernähe** gelöst: Ein Produktionsgebäude arbeitet nur, wenn ein fertiges Lager (`storage > 0`: Bergfried oder
  Lagerhaus) im Umkreis von `city.logistics_radius` = 14 Kacheln steht. Sonst: Stillstand „kein Lager in Reichweite“.
* **Grundressourcen** (`gold`, `wood`, `stone`, `food`) bleiben `ResourceKind` und werden weiter von Leibeigenen
  gesammelt; **Waren** (`goods.ron`) sind frei erweiterbar (Mods). Beide heißen im Code `Item`
  (`Item::Res(ResourceKind)` oder `Item::Good(GoodId)`), Rezepte und Kosten nennen Items per ID-String.
* **Arbeitskräfte** sind Bewohner der Wohnhäuser. Leibeigene (Einheiten) bauen, sammeln Gold/Holz/Stein und
  bilden weiterhin die Baukolonne.

## 2. Waren

Preis = Grundpreis am Markt (Gold). Stufe: 1 Rohstoff, 2 Zwischengut, 3 Fertiggut (Gruppierung im Lagerfenster).

| ID | Name | Stufe | Preis | Hergestellt in | Verbraucht von |
|---|---|---|---|---|---|
| `clay` | Lehm | 1 | 3 | Lehmgrube | Ziegelei |
| `iron_ore` | Eisenerz | 1 | 5 | Eisenmine | Schmelzhütte |
| `coal` | Kohle | 1 | 4 | Köhlerei | Ziegelei, Schmelzhütte, Rüstkammer |
| `grain` | Getreide | 1 | 3 | Bauernhof | Mühle, Brauerei |
| `wool` | Wolle | 1 | 4 | Schäferei | Weberei |
| `herbs` | Kräuter | 1 | 5 | Kräutergarten | Tempel |
| `planks` | Bretter | 2 | 5 | Sägewerk | Schmiede, Baukosten |
| `bricks` | Ziegel | 2 | 6 | Ziegelei | Ausbau zur Burg |
| `iron` | Eisenbarren | 2 | 12 | Schmelzhütte | Schmiede, Rüstkammer |
| `flour` | Mehl | 2 | 5 | Mühle | Bäckerei |
| `cloth` | Tuch | 2 | 9 | Weberei | Schneiderei |
| `bread` | Brot | 3 | 7 | Bäckerei | Bürgerhaus |
| `beer` | Bier | 3 | 8 | Brauerei | Bürgerhaus |
| `clothing` | Kleidung | 3 | 14 | Schneiderei | Haus, Bürgerhaus |
| `tools` | Werkzeug | 3 | 14 | Schmiede | Baukosten (Eisenmine, …) |
| `weapons` | Waffen | 3 | 22 | Schmiede | Ausbildung |
| `armor` | Rüstung | 3 | 30 | Rüstkammer | Ausbildung |

Grundpreise der Grundressourcen (`rules.ron`, `city`): Holz 2, Stein 3, Nahrung 1,5.

### 2.1 Lager und Kapazität

* Kapazität **je Ware und Grundressource** (außer Gold): Summe von `storage` aller fertigen Lagergebäude
  (Bergfried 500, Lagerhaus 400) plus `storage_bonus` der erreichten Ausbaustufen (Burg +200).
* Ist ein Vorrat voll, stoppt die zuliefernde Produktion („Lager für X ist voll“); Sammelfahrten der Leibeigenen
  liefern den Überschuss nicht mehr ab (Meldung „Lager voll – baut ein Lagerhaus“). Bestände oberhalb der
  Kapazität (z. B. Startvorrat der Sandbox) bleiben erhalten, es kommt nur nichts hinzu.
* Gold ist unbegrenzt.

## 3. Produktionsgebäude

Alle Zeiten in Sekunden bei **voller Besetzung**. Mit weniger Arbeitern läuft der Zyklus anteilig langsamer
(`Arbeiter / Arbeitsplätze`); ohne Arbeiter steht er still. Eingaben werden zu **Zyklusbeginn** verbraucht, Ausgaben
am Zyklusende gutgeschrieben.

| Gebäude (ID) | Größe | Stufe | Arbeiter | Eingabe | Ausgabe | Zyklus | Kosten | Standort |
|---|---|---|---|---|---|---|---|---|
| Holzfällerlager (`woodcutter`) | 2×2 | 0 | 2 | – | 4 Holz | 30 | 20 Gold, 50 Holz | ≥ 4 Bäume im Umkreis 8; jeder Zyklus fällt 4 Holz Vorrat der nächsten Quelle |
| Lehmgrube (`clay_pit`) | 2×2 | 0 | 2 | – | 3 Lehm | 30 | 40 Gold, 40 Holz | ≥ 5 Kacheln Erde/Sand/Furt im Umkreis 2 |
| Köhlerei (`charcoal_burner`) | 2×2 | 0 | 1 | 3 Holz | 2 Kohle | 60 | 40 Gold, 70 Holz | – |
| Eisenmine (`iron_mine`) | 2×2 | 1 | 3 | – | 2 Eisenerz | 40 | 100 Gold, 60 Holz, 40 Stein, 2 Werkzeug | Steinbruch im Umkreis 7 |
| Kräutergarten (`herb_garden`) | 2×2 | 0 | 1 | – | 2 Kräuter | 40 | 30 Gold, 40 Holz | – |
| Bauernhof (`farm`) | 3×3 | 0 | 3 | – | 4 Getreide | 40 | 50 Gold, 70 Holz | – |
| Mühle (`windmill`) | 2×2 | 0 | 1 | 2 Getreide | 2 Mehl | 20 | 60 Gold, 90 Holz, 20 Stein | – |
| Bäckerei (`bakery`) | 2×2 | 0 | 2 | 2 Mehl | 4 Brot | 30 | 50 Gold, 50 Holz, 40 Stein | – |
| Brauerei (`brewery`) | 3×2 | 1 | 2 | 3 Getreide | 6 Bier | 60 | 70 Gold, 80 Holz, 20 Stein | – |
| Schäferei (`sheep_farm`) | 3×3 | 0 | 2 | – | 2 Wolle | 50 | 60 Gold, 70 Holz | – |
| Sägewerk (`sawmill`) | 3×2 | 0 | 2 | 2 Holz | 3 Bretter | 30 | 60 Gold, 80 Holz | – |
| Ziegelei (`brickworks`) | 3×2 | 0 | 2 | 2 Lehm + 1 Kohle | 4 Ziegel | 45 | 80 Gold, 60 Holz, 40 Stein | – |
| Schmelzhütte (`smelter`) | 2×2 | 1 | 2 | 2 Eisenerz + 2 Kohle | 2 Eisenbarren | 50 | 90 Gold, 40 Holz, 90 Stein | – |
| Schmiede (`smithy`) | 3×2 | 1 | 2 | Rezept *Waffen*: 2 Eisenbarren + 1 Brett → 2 Waffen (60 s); Rezept *Werkzeug*: 1 Eisenbarren + 1 Brett → 2 Werkzeug (45 s) | | | 110 Gold, 60 Holz, 70 Stein, 10 Bretter | Rezept im Auswahlfeld umschaltbar |
| Rüstkammer (`armory`) | 3×2 | 1 | 3 | 3 Eisenbarren + 1 Kohle | 1 Rüstung | 75 | 140 Gold, 50 Holz, 90 Stein, 10 Bretter | – |
| Weberei (`weaver`) | 3×2 | 0 | 2 | 2 Wolle | 2 Tuch | 40 | 60 Gold, 80 Holz | – |
| Schneiderei (`tailor`) | 2×2 | 0 | 2 | 1 Tuch | 2 Kleidung | 50 | 60 Gold, 60 Holz | – |
| Tempel (`temple`) | 3×2 | 0 | 1 | 1 Kräuter (Dienst, keine Ausgabe) | – | 45 | 120 Gold, 60 Holz, 120 Stein | wirkt auf Häuser im Umkreis 12, solange er arbeitet |

Lager und Handel: **Lagerhaus** (`storehouse`, 3×2, 40 Gold + 100 Holz, Kapazität 400, Annahmestelle für
Leibeigene) und **Markt** (`market`, 3×3, 100 Gold + 120 Holz + 40 Stein).

### 3.1 Betriebszustände

Jedes Gebäude mit Rezept hat einen Zustand (`ProdStatus`), der im Auswahlfeld im Klartext steht und als Symbol über
dem Gebäude erscheint:

| Zustand | Symbol | Ursache |
|---|---|---|
| Arbeitet | – | Zyklus läuft; das Gebäude raucht/leuchtet (Schmiede-Glut, Köhler-Meiler, Backofen …) |
| Fehlende Eingabe | roter Kasten | eine Eingabeware fehlt (das Auswahlfeld nennt sie) |
| Keine Arbeiter | gelbe Figur | alle Arbeitskräfte sind vergeben oder keine Bewohner vorhanden |
| Lager voll | blauer Pfeil | eine Ausgabeware hat die Lagerkapazität erreicht |
| Kein Lager | orange Haus | kein Lager im 14-Kachel-Umkreis |
| Kein Standort | grünes Blatt | Bäume/Erzvorkommen erschöpft oder nicht mehr in Reichweite |
| Stillgelegt | graue Pause | Spieler setzte das Arbeiterlimit auf 0 |

### 3.2 Arbeiter zuweisen

* Arbeitskräfte = `floor(Σ Bewohner × workforce_ratio)` mit `workforce_ratio` = 0,8.
* Verteilung einmal pro Sekunde in **Gebäudereihenfolge** (zuerst Gebautes zuerst): jedes Gebäude erhält bis zu
  `min(Arbeiterlimit, Arbeitsplätze)`, solange Kräfte frei sind.
* Das **Arbeiterlimit** setzt der Spieler im Auswahlfeld mit `-`/`+` (Befehl `SetWorkers`); `0` legt das Gebäude still
  und gibt die Arbeiter für andere Betriebe frei; volle Besetzung = „automatisch“.

## 4. Bevölkerung

### 4.1 Wohnstufen

Wohnhäuser sind drei Gebäudetypen derselben Grundfläche (2×2). **Nur die Hütte** wird gebaut; Haus und Bürgerhaus
entstehen durch Ausbau (Typwechsel an Ort und Stelle).

| Stufe | Gebäude | Bewohner | Steuer (Gold/Bewohner/s) | Bedürfnisse (Verbrauch je Bewohner und Minute) |
|---|---|---|---|---|
| 0 | Hütte (`hut`) | 5 | 0,010 | Nahrung 0,3 |
| 1 | Haus (`house`) | 8 | 0,020 | Nahrung 0,3, Kleidung 0,25, **Glaube** (Tempel ≤ 12 Kacheln) |
| 2 | Bürgerhaus (`townhouse`, Burg) | 12 | 0,036 | Brot 0,4, Kleidung 0,3, Bier 0,35, **Glaube**, **Schutz** (Turm/Kaserne/Bergfried ≤ 9–12 Kacheln) |

Bedürfnisarten (`NeedKind`): `Item(<id>)` verbraucht Vorrat; `Faith` und `Safety` sind Abdeckung durch Dienstgebäude
(`service`, z. B. Tempel arbeitet nur mit Priester und Kräutern; Wachturm, Kaserne, Bergfried, Mauerturm für Schutz).

### 4.2 Zufriedenheit

Pro Haus und Sekunde:

```
Erfüllung    = Σ(Gewicht × erfüllt) / Σ Gewicht           (erfüllt: 0..1; Waren anteilig nach verfügbarem Vorrat)
Zielwert     = clamp(100 × Erfüllung + Steuermodifikator, 0, 100)
Zufriedenheit += clamp(Zielwert − Zufriedenheit, −3, +3)   (happiness_rate)
```

* **Zuzug**: ab Zufriedenheit 35 wächst die Bewohnerzahl um 0,08/s bis zur Kapazität. Ein leeres Haus füllt sich nur,
  wenn seine Warenbedürfnisse im Lager gedeckt sind (Vorrat ≥ 1).
* **Unruhe** (Zufriedenheit < 20): keine Steuern, Abwanderung 0,12 Bewohner/s je Haus, Meldung „Unruhe in
  Brennhollow“ (höchstens alle 120 s) und Symbol über dem Haus.
* **Aufstieg**: Zufriedenheit ≥ 70 für 25 s, Ausbaustufe der Siedlung ausreichend (Bürgerhaus: Burg) **und** die
  Bedürfnisse der nächsten Stufe sind sofort erfüllbar (Waren im Lager, Tempel/Schutz in Reichweite).
* **Abstieg**: Zufriedenheit < 20 für 40 s → nächstniedrigere Stufe (Zufriedenheit 30); die Hütte steigt nicht ab.

### 4.3 Steuern

Einnahmen pro Haus und Sekunde: `Bewohner × Steuer(Stufe) × Steuerfaktor × clamp(Zufriedenheit / 60, 0, 1)`; unter
Unruhe 0. Der **Steuersatz** gilt für die ganze Siedlung (Befehl `SetTax`, Stadtfenster):

| Satz | Einnahmefaktor | Zufriedenheit |
|---|---|---|
| Keine | 0,0 | +15 |
| Niedrig | 0,5 | +6 |
| Normal (Standard) | 1,0 | 0 |
| Hoch | 1,6 | −14 |
| Drückend | 2,4 | −32 |

Der Bergfried liefert zusätzlich seine festen Abgaben (`gold_per_sec` 0,6).

## 5. Markt und Handel

Ein fertiger Markt erlaubt Handel mit allen Waren sowie Holz, Stein und Nahrung (Marktfenster erscheint bei Auswahl
des Marktes; `+1/+10` kauft, `-1/-10` verkauft; Befehl `Trade`).

```
Preis(Item) = Grundpreis × clamp(1 + Druck, 0,4 … 2,6)
Kaufpreis   = Preis × (1 + 0,12)     Verkaufserlös = Preis × (1 − 0,12)
Handel je Einheit: Druck ±0,02 (Kauf +, Verkauf −); große Mengen werden Stück für Stück teurer bzw. billiger
Erholung   : Druck verfällt um 1,2 % pro Sekunde (Halbwertszeit ≈ 1 Minute)
```

Beispiel: 40 Bretter verkaufen senkt den Brettpreis von 5,0 auf den Mindestwert (40 % des Grundpreises = 2,0 Gold); er erholt sich innerhalb weniger Minuten.
Kauf ist durch Gold und Lagerkapazität, Verkauf durch den Vorrat begrenzt.

## 6. Ausbildung mit Ausrüstung

Die Ausbildung kostet zusätzlich Waren (`goods_cost`); die Goldkosten sind entsprechend gesenkt. Abbruch erstattet alles.

| Einheit | Gold | Holz | Stein | Waren |
|---|---|---|---|---|
| Schwertkämpfer | 80 | 10 | – | 1 Waffen, 1 Rüstung |
| Speerträger | 60 | 30 | – | 1 Waffen |
| Bogenschütze | 70 | 40 | – | 1 Waffen |
| Ritter | 170 | 40 | 20 | 1 Waffen, 2 Rüstung |

Fehlt eine Ware, meldet das Spiel „Nicht genug Waffen.“ (bzw. Rüstung) und reiht nichts ein.

## 7. Ausbaustufen der Siedlung

`rules.ron → eras`. Stufe 0 „Siedlung“; **Stufe 1 „Burg“**: Bergfried ausbauen (Stadtfenster oder Befehlskarte des
Bergfrieds):

* Voraussetzungen: 2 Häuser und 1 Lagerhaus (fertig).
* Kosten: 450 Gold, 200 Holz, 300 Stein, 40 Bretter, 40 Ziegel; Bauzeit 60 s.
* Wirkung: Lagerkapazität +200 je Ware, Wohnraum für Einheiten +10, neues Modell des Bergfrieds (Burg).
* Schaltet frei (`era: 1`): Eisenmine, Schmelzhütte, Schmiede, Rüstkammer, Brauerei, Mauerturm, Bürgerhaus-Aufstieg.
  Im Baumenü erscheinen sie mit „gesperrt“; Tooltip nennt die Ausbaustufe.

Weitere Stufen lassen sich in `eras` datengetrieben ergänzen (`era: 2` an Gebäuden).

## 8. Befestigungen und Belagerung

| Gebäude | Größe | LP | Rüstung | Kosten | Verhalten |
|---|---|---|---|---|---|
| Mauer (`wall`) | 1×1 | 700 | 6 | 8 Stein | blockiert alle Einheiten |
| Tor (`gate`) | 1×1 | 600 | 6 | 30 Holz, 10 Stein | **offen**: eigene Einheiten passieren, Feinde nie; **geschlossen**: niemand |
| Mauerturm (`wall_tower`, Burg) | 2×2 | 900 | 7 | 80 Gold, 40 Holz, 100 Stein | blockiert, schießt (20 Pierce, Reichweite 8), Schutz-Dienst Radius 9 |

* **Mauer ziehen**: Mauer im Baumenü (Rubrik *Mauern*) wählen, Maustaste im Gelände gedrückt halten, Ziel wählen,
  loslassen. Die Linie (Bresenham, Diagonalen erlaubt) wird als `BuildLine` platziert; alle Teile werden bezahlt
  (nicht bezahlbare am Ende entfallen), die Bauarbeiter verteilen sich reihum. Die Vorschau färbt gültige/ungültige
  Teile. Shift = weitere Linie.
* **Wegfindung**: A* mit Torrechten je Spieler (`TileMap::gate_pass`); Diagonalschritte zwischen zwei Mauerkacheln
  sind verboten, diagonale Mauerketten sind also dicht. Torzustand im Auswahlfeld („Tor öffnen/verschließen“,
  `SetGate`); zerstörte Tore geben die Kachel frei.
* **Belagerung**: Findet eine angreifende Einheit (Angriffsbewegung) keinen Weg, sucht eine zweite Wegsuche den
  kürzesten Weg **durch** feindliche Befestigungen (Zusatzkosten 14 je Kachel); die erste Befestigung darauf wird
  Angriffsziel. Einheiten in Sichtweite haben Vorrang. Die Auto-Zielwahl ignoriert Mauern sonst, damit Angreifer
  keine Mauern „aus Langeweile“ beschießen. Schaden gegen Befestigungen: normale Rüstungs- und Schadenstabelle
  (Fortified: Siege ×1,5, Normal ×0,7, Pierce ×0,35); Grabkolosse sind die Ramme.

## 9. Bedienung

| Taste/Element | Wirkung |
|---|---|
| `B` (Arbeiter gewählt) / Befehlskarte „Bauen“ | Baumenü mit Rubriken Wohnen, Rohstoffe, Handwerk, Nahrung, Stadt, Militär, Mauern |
| `L` / Schaltfläche *Lager* | Lagerfenster: alle Waren, Vorrat, Tooltip mit Herkunft, Verbrauch, Marktpreis |
| `C` / Schaltfläche *Stadt* | Stadtfenster: Bewohner, Arbeitskräfte, Zufriedenheit, Steuersatz, häufigste Mängel, Ausbau zur Burg |
| Gebäude anklicken | Betriebsansicht: Rezept, Zustand, Arbeiter `-`/`+`, Rezeptwahl, Tor öffnen/schließen, Bewohner und Mängel |
| Markt anklicken | Marktfenster mit Kauf-/Verkaufstasten |
| Bergfried anklicken | Befehlskarte: „Zur Burg ausbauen“ |
| Mauer-Baumodus | Ziehen und Loslassen; `Esc`/Rechtsklick bricht ab |
| Statusleiste links unter der Hauptleiste | Bürgerzahl, Arbeitseinsatz, Stimmungssymbol |

## 10. Mission 1 und Sandbox

**Mission 1** startet mit zwei Hütten (6 Bewohner, Zuzug bis 10), einem Holzfällerlager am Westwald und dem Vorrat
10 Waffen, 4 Rüstungen, 4 Werkzeug, 20 Bretter, 8 Kleidung, 6 Kräuter. Das Heer von 8 Soldaten (Ziel) ist damit ohne
Schmiede erreichbar; wer Bretter/Kleidung/Bürger will, baut die Kette aus. Der Einheitenwohnraum (`pop_cap` 18) und
alle Kampagnenziele bleiben unverändert; neu sind die optionalen Ziele *dritte Hütte*, *Sägewerk* und *Mauer mit Tor*.

**Sandbox**: alle Ausbaustufen freigeschaltet, je 300 Stück aller Waren, 6 000 zusätzliche Lagerkapazität, kein Nebel.

## 11. Technische Anmerkungen

* Logik: `crates/ashen_sim/src/systems/city.rs` (Produktion, Arbeiter, Bevölkerung, Markt, Ausbau, Standort),
  `path.rs`/`map.rs` (Torrechte, Bresche), `systems/movement.rs` (Belagerungsziel), `systems/commands.rs`.
* Alle Tests: `crates/ashen_sim/tests/economy.rs` (Ketten Ende-zu-Ende, Stillstand, Wohnstufen, Steuer,
  Markt, Kapazität, Ausrüstung, Ausbau, Mauer/Tor, Belagerung, Mauerlinie, Spielstand, Determinismus, Mission 1).
* Spielstandformat 3 (neue Felder: Warenlager, Steuersatz, Ausbaustufe, Handelsdruck, Bewohner, Torrechte).
* Screenshots/Art-Review: `ashen_crowns --scene sandbox --demo-city --ui storage,city,build:wall,select:smithy
  --screenshot out.png --camera 31,56 --zoom 400` zeigt die Vorführstadt; Kontaktbogen/Atlas siehe
  `docs/art/3d_pixel_pipeline.md`.
