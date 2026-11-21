# ASHEN CROWNS – The Age of the Fallen
## Game Design Document (GDD)

| Feld | Wert |
|---|---|
| Dokument | `docs/design/game_design_document.md` |
| Status | Verbindliche Designgrundlage, Version 1.0 |
| Verantwortlich | Lead Game Design |
| Plattform | macOS (Apple Silicon und Intel), eigene Rust-Engine |
| Genre | Echtzeitstrategie mit Großstrategie-Ebene, Wirtschaftssimulation und Rollenspielelementen |
| Kanon-Referenz | `docs/canon.md` – alle Eigennamen und IDs in diesem Dokument folgen der Kanon-Datei |
| Ergänzend | `docs/design/campaign.md` (Kampagne und Mission 1 im Detail) |

> Konvention: Zahlenwerte in diesem Dokument sind die **Startwerte für das Balancing**. Sie werden in den
> Datendateien (RON) gepflegt und dürfen dort im Rahmen der Balancing-Grundsätze (Kapitel 24) verändert
> werden. Eigennamen und IDs dürfen nur über eine Änderung von `docs/canon.md` verändert werden.

---

## Inhaltsverzeichnis

1. Vision und Säulen
2. Zielgruppe
3. Spielphilosophie
4. Die zwei Ebenen: Weltkarte und Echtzeitwelt
5. Dynastien und Herrscher
6. Politik, Adelshäuser, Ämter und Intrigen
7. Diplomatie
8. Wirtschaft
9. Bevölkerung
10. Militär
11. Helden und Rollenspielsystem
12. Magie und die Hohlung
13. Religion
14. Monster, neutrale Fraktionen und Bosse
15. Dynamische Ereignisse und Kausalketten
16. Endgame
17. Das Chroniksystem
18. Wiederspielwert
19. Spielmodi
20. UI/UX-Konzept
21. Tastenkürzel
22. Schwierigkeitsgrade
23. Barrierefreiheit und Lesbarkeit
24. Balancing-Grundsätze
25. Anhang: Glossar und ID-Übersicht

---

## 1. Vision und Säulen

### 1.1 Vision in einem Satz

*ASHEN CROWNS* ist ein düsteres Strategiespiel, in dem der Spieler aus einer einzigen verzweifelten
Grenzsiedlung in der Aschmark über Generationen eine Dynastie aufbaut, deren Städte er Stein für Stein in
Echtzeit errichtet, deren Schlachten er selbst schlägt und deren Geschichte die Welt Eldraeth sich noch
lange erzählt – während unter der Erde das Hohle Tief erwacht.

### 1.2 Die Prämisse

Im Jahr 897 n.F. liegt der Aschenfall fast neun Jahrhunderte zurück. Varkhan Aurelith, der Letzte
Sonnenkönig Aureths, wollte den Tod überwinden und riss mit seinem Ritual den Schleier zum Hohlen Tief auf.
Solmere, die strahlende Hauptstadt, ist eine Ruine in der Aurethwüste; Aschglas wächst dort, wo der Schleier
dünn ist, und die Hohlung frisst sich in Land, Tier und Mensch. Die Nachfolgereiche – allen voran die
Glutkrone von Valemark – sind zerstritten, ihre Herrscher alt oder gierig. In der Aschmark, dem Grenzland zum
verfluchten Süden, hält Brennhollow mit etwa 30 Seelen die letzte Stellung. Isolde Raventhorn, Wächterin der
Aschmark, und Bruder Hal Venn sind alles, was zwischen den Hohlen und den fruchtbaren Tälern Valemarks steht.

Aus diesem Ausgangspunkt entwickelt sich eine Geschichte, die der Spieler nicht nacherzählt bekommt, sondern
selbst schreibt: als Verteidiger, Eroberer, Händler, Heiliger oder als jemand, der dem Hohlen Tief die Hand
reicht.

### 1.3 Designsäulen

Jede Designentscheidung wird gegen diese fünf Säulen geprüft. Ein Feature, das keine Säule stärkt, wird
gestrichen; ein Feature, das einer Säule widerspricht, wird umgebaut.

| # | Säule | Bedeutung | Prüffrage |
|---|---|---|---|
| S1 | **Asche hat Gewicht** | Jede Handlung hat Konsequenzen, die bleiben. Gefallene Helden bleiben tot (wenn gewählt), verbrannte Felder liefern keine Ernte, gebrochene Schwüre werden erinnert. | „Wird sich das Spiel in 20 Jahren Spielzeit an diese Entscheidung erinnern?“ |
| S2 | **Zwei Hände, eine Krone** | Weltkarte und Echtzeitwelt sind keine getrennten Spiele, sondern zwei Blickwinkel auf denselben Simulationszustand. | „Verändert das, was ich hier tue, die andere Ebene messbar?“ |
| S3 | **Die Welt atmet ohne dich** | KI-Reiche, Bevölkerungen, Märkte, Monster und Glaubensgemeinschaften verfolgen eigene Ziele. Der Spieler ist ein Akteur unter vielen, nicht der Mittelpunkt. | „Würde das auch passieren, wenn der Spieler nicht hinsieht?“ |
| S4 | **Taktik mit Händen** | Kämpfe in Echtzeit belohnen Mikromanagement, Positionierung, Kombination von Einheiten und den klugen Einsatz von Helden – auf der Ebene eines kompetitiven RTS. | „Kann ein guter Spieler hier eine schlechte Ausgangslage drehen?“ |
| S5 | **Schönheit im Verfall** | Handgezeichnete Pixel-Art mit flüssigen Animationen, starke Silhouetten, Licht als Erzählmittel: Glut gegen Asche, Wärme gegen fahles Grün. | „Ist der Zustand auf einen Blick lesbar und trotzdem atmosphärisch?“ |

### 1.4 Ästhetische Richtlinie (Kurzfassung)

- **Pixel-Art** in hoher Auflösung (Einheiten 32–48 px hoch, Helden 48–64 px, Bosse bis 192 px), animiert mit
  12–16 Bildern pro Sekunde pro Animation, flüssige Sekundäranimationen (Umhänge, Funken, Ascheflocken).
- **Farbsprache**: Glutkrone karmesin/gold, Skarn-Clans eisblau/stahl, Dornenbund moosgrün/bernstein,
  Saelith-Pakt türkis/silber, Hohle aschgrau/fahlgrün, Aschenlegion schwarz/glutorange.
- **Licht** ist spielmechanisch relevant: Fackeln, Feuerstellen und Glutmagie erzeugen dynamische Lichtkreise,
  die nachts Sichtweite und Moral beeinflussen (Kapitel 10.8).
- **Asche** als Partikelsystem: Ascheniederschlag zeigt die Nähe des Schleiers an, Hohlung färbt Gelände
  sichtbar fahl.
- Eigene Ikonografie: Kronen aus Glut, zerbrochene Sonnenscheiben (Aureth), Wurzelringe (Dornenbund),
  Wellenknoten (Saelith), Runenkreise (Skarnheim), offene Hohlräume ohne Mittelpunkt (Hohle).

### 1.5 Alleinstellungsmerkmale

1. **Nahtlose Kopplung** von Großstrategie (Dynastie, Diplomatie, Provinzen) und RTS-Stadtbau sowie RTS-Schlacht
   auf demselben Simulationszustand.
2. **Generationenspiel**: Herrscher altern, sterben und vererben; Helden können Herrscher werden und umgekehrt.
3. **Simulierte Kausalketten** statt geskripteter Ereignisse: Dürre, Hunger, Preise, Unruhe und Krieg entstehen
   aus Zahlen, nicht aus Drehbüchern.
4. **Die Hohlung** als vollwertiger, alternativer Entwicklungspfad mit eigener Wirtschaft, eigenen Einheiten
   und eigenem Ende.
5. **Die Chronik**: Jede Partie erzeugt ein lesbares Geschichtsbuch, das als Text exportiert werden kann.

---

## 2. Zielgruppe

### 2.1 Primäre Zielgruppe

| Segment | Beschreibung | Was sie suchen | Wie ASHEN CROWNS es liefert |
|---|---|---|---|
| **RTS-Veteranen** (25–45 Jahre) | Spieler klassischer Echtzeitstrategie mit Helden und Fraktionen | Taktische Tiefe, Einheitenkonter, Mikromanagement, starke Kampagne | Echtzeitschlachten mit Helden, Schadens-/Rüstungsmatrix, Moral, Flankierung |
| **Großstrategen** (25–50 Jahre) | Spieler von Dynastie- und Reichssimulationen | Emergente Geschichten, Charaktere, Intrigen, langfristige Planung | Dynastien, Ämter, Intrigen, Meinungsmodell, Chronik |
| **Aufbauspieler** (20–55 Jahre) | Spieler von Städtebau- und Wirtschaftsspielen | Produktionsketten, Optimierung, schöne Städte, friedliches Spiel | Achtschichtige Bevölkerung, vollständige Ketten, Markt, friedliche Siegbedingungen |

### 2.2 Sekundäre Zielgruppe

- **Dark-Fantasy-Fans**, die eine eigenständige, ernsthafte Welt ohne Ironiebrüche suchen.
- **Pixel-Art-Enthusiasten**, die hochwertige, animationsreiche 2D-Grafik schätzen.
- **Erzählspieler**, die eine Kampagne mit echten Entscheidungen und mehreren Enden spielen wollen.

### 2.3 Plattformannahmen (macOS)

- Bedienung primär mit Maus und Tastatur; Trackpad-Gesten (Zwei-Finger-Scrollen = Kamera schwenken,
  Pinch = Zoom, Force-Click = Kontextinfo) sind vollwertig unterstützt.
- Zielhardware: ab Apple M1 mit 8 GB RAM bei 60 FPS in 1440p; Intel-Macs ab 2018 bei 30–60 FPS in 1080p.
- Sitzungslänge: 30–90 Minuten typisch; jederzeit speicherbar, Autospeicherung alle 5 Minuten Echtzeit
  sowie bei jedem Ebenenwechsel.

### 2.4 Spielertypen innerhalb der Zielgruppe

| Typ | Spielweise | Gewünschte Erfahrung | Unterstützende Systeme |
|---|---|---|---|
| Feldherr | Aggressiv, schnelle Expansion | Siege, Eroberung, Heldentaten | Militär, Helden, Belagerungen |
| Baumeister | Wirtschaftsfokus, friedlich | Blühende Städte, effiziente Ketten | Wirtschaft, Bevölkerung, Handelssieg |
| Ränkeschmied | Indirekt, politisch | Kontrolle durch Einfluss | Ämter, Intrigen, Diplomatie |
| Chronist | Rollenspiel, Geschichte | Persönliche Saga einer Dynastie | Dynastie, Chronik, Ereignisse |
| Meister | Höchste Schwierigkeit, Ironman | Bewältigung harter Herausforderungen | Permadeath, Herausforderungsmodus |

---

## 3. Spielphilosophie

### 3.1 Konsequenzen statt Wiederholung

- **Kein Rückgängig in der Simulation.** Entscheidungen werden sofort in den Zustand geschrieben. Speichern und
  Laden ist erlaubt (außer im Ironman-Modus), aber das Spiel entwirft keine Situationen, die zum Neuladen
  zwingen. Niederlagen sind spielbar: Ein verlorener Krieg führt zu Tributen, Gebietsverlust, Gefangenschaft
  oder Exil – nicht zum Game Over.
- **Game Over** entsteht ausschließlich, wenn die Dynastie keinen spielbaren Erben mehr hat und der Spieler
  keinen Nebenzweig übernehmen kann, oder wenn die letzte Provinz verloren ist und kein Exilhof existiert.
  Selbst dann wird die Chronik fertiggestellt und kann gelesen werden.
- **Langzeitgedächtnis**: Das Meinungsmodell (Kapitel 7.3) speichert Erinnerungen mit Abklingzeiten von bis zu
  100 Jahren (Verrat) oder dauerhaft (Ermordung eines Familienmitglieds, Bündnis mit dem Hohlen Tief).

### 3.2 Die Welt existiert unabhängig vom Spieler

- Alle KI-Reiche folgen denselben Regeln wie der Spieler: Sie bauen ihre Städte nach denselben
  Produktionsregeln (abstrakt simuliert), zahlen dieselben Kosten und unterliegen denselben Ereignissen.
  Es gibt **keine versteckten KI-Boni** auf den Schwierigkeitsgraden bis einschließlich „Lehnsherr“
  (Kapitel 22).
- Die Hohlen breiten sich nach einer eigenen Simulation (Schleierstärke, Kapitel 12.7) aus, unabhängig davon,
  ob der Spieler sie bekämpft.
- Märkte haben Angebot und Nachfrage aller Reiche; eine Missernte in Skarnheim verteuert Getreide in Valemark.
- Kriege, Erbfolgen, Hochzeiten und Seuchen geschehen auch in Gebieten, die der Spieler nie betritt, und werden
  in der Chronik verzeichnet.

### 3.3 Friedliche Spielweise ist vollwertig

Ein Spieler kann ASHEN CROWNS ohne einen einzigen Angriffskrieg gewinnen. Voraussetzungen dafür:

- **Verteidigung ist Pflicht, Eroberung nicht.** Die Hohlen und Monster greifen an; Verteidigungsanlagen,
  Bündnisse und Söldner reichen aus, um zu bestehen.
- **Friedliche Siegbedingungen** (Kapitel 16.4): Handelssieg, Glaubenssieg, Siegel des Schleiers (gemeinsam
  mit Verbündeten), Dynastischer Sieg über Heiratspolitik.
- **Expansion ohne Krieg**: Neue Provinzen werden durch Besiedlung herrenlosen Landes, Heirat und Erbe,
  Kauf (Saelith-Pakt verkauft Handelsposten), freiwilligen Anschluss (Provinz mit Meinung ≥ +60 und eigener
  Zufriedenheit ≥ 70 kann sich anschließen) und Rückgewinnung von Hohlland gewonnen.
- **Friedensbonus**: Reiche ohne Angriffskrieg in den letzten 20 Jahren erhalten „Ruf des Friedens“
  (+10 Meinung bei allen nicht-feindlichen Reichen, −15 % Bedrohung, Kapitel 7.5).

### 3.4 Lesbarkeit vor Komplexität

Tiefe entsteht durch die Verknüpfung einfacher Systeme, nicht durch undurchsichtige Einzelsysteme. Jede Zahl,
die das Spiel verwendet, ist per Tooltip mit ihrer vollständigen Herleitung einsehbar („Warum ist das so?“-
Prinzip). Formeln in diesem Dokument werden im Spiel als aufgeschlüsselte Liste angezeigt.

### 3.5 Fairness

- Zufall wird sichtbar gemacht: Wahrscheinlichkeiten werden immer angezeigt.
- Die Engine verwendet deterministische Simulation mit festem Seed pro Partie (wichtig für Wiederholungen,
  Herausforderungsmodus und Fehleranalyse).
- Kritische Ereignisse kündigen sich an (Vorzeichen-System, Kapitel 15.5): Keine Seuche, kein Aufstand und kein
  Hohlenangriff erscheint ohne mindestens ein vorheriges Warnsignal.

---

## 4. Die zwei Ebenen: Weltkarte und Echtzeitwelt

### 4.1 Überblick

| Ebene | Darstellung | Zeitmodell | Hauptaufgaben |
|---|---|---|---|
| **Weltkarte** | Gemalte Pixel-Art-Karte Eldraeths, Provinzen, Armeen als Figuren, Städte als Wahrzeichen | Pausierbare Echtzeit in Tagen; 5 Geschwindigkeitsstufen | Dynastie, Politik, Diplomatie, Handel, Armeebewegung, Forschung, Religion, Ereignisse |
| **Echtzeitwelt (RTS)** | Seitlich-isometrische Pixel-Art (3/4-Ansicht), Kartengröße 128×128 bis 320×320 Felder | Echtzeit in Sekunden, pausierbar im Einzelspieler | Stadtbau, Rohstoffabbau, Produktion, Echtzeitschlachten, Belagerungen, Heldenquests |

Beide Ebenen greifen auf **einen gemeinsamen Simulationszustand** zu (Entity-Datenbank der Engine). Es gibt
keine Kopien, die synchronisiert werden müssen; die Weltkarte liest dieselben Gebäude, Lager und Einheiten,
die in der Echtzeitwelt existieren, in aggregierter Form.

### 4.2 Zeitkopplung

| Größe | Wert | Erläuterung |
|---|---|---|
| Weltkarten-Tag | Grundtakt | 1 Tag = 0,25 s (Stufe 5) bis 2,0 s (Stufe 1) Echtzeit auf der Weltkarte |
| Äquivalenzfaktor Produktion | **1 Welttag = 8 s RTS-Simulationszeit** | Ein Gebäude, das im RTS 1 Einheit pro 8 s erzeugt, erzeugt auf der Weltkarte 1 Einheit pro Tag |
| Weltkarten-Monat | 30 Tage = 240 s RTS-Äquivalent | Abrechnungstakt für Steuern, Bevölkerung, Meinung, Unterhalt |
| Weltkarten-Jahr | 12 Monate = 360 Tage | Altern, Ernte, Jahreschronik |
| Stadtansicht (RTS) | Weltzeit läuft mit **1 Tag pro 8 s** weiter | Andere Reiche handeln weiter, während der Spieler baut |
| Gespielte Schlacht | Weltzeit **pausiert** | Schlachtdauer wird nach Ende als 1 Tag (unter 10 min), 2 Tage (10–25 min) oder 3 Tage (über 25 min) verbucht |
| Gespielte Belagerungserstürmung | Weltzeit pausiert | wie Schlacht |
| Heldenquest (RTS) | Weltzeit pausiert | Quest wird als 3–10 Tage verbucht (abhängig vom Questtyp) |

Option „Strenge Weltzeit“ (Einstellung): Auch während der Stadtansicht pausiert die Welt; stattdessen läuft
die Weltzeit mit 1 Tag pro 8 s nur, wenn der Spieler sie aktiv laufen lässt. Für Ironman fest auf
Standard gesetzt.

### 4.3 Kopplung Stadt (Weltkarte) ↔ RTS-Stadt

Jede Siedlung auf der Weltkarte besitzt eine **dauerhafte RTS-Stadtkarte** (Stadtgebiet). Die Stadtkarte wird
beim ersten Betreten aus der Provinz erzeugt und danach gespeichert.

**Erzeugung der Stadtkarte**

| Eingang (Weltkarte) | Wirkung auf die Stadtkarte |
|---|---|
| Provinzgelände (Ebene, Hügel, Wald, Gebirge, Sumpf, Küste, Wüste, Hohlland) | Grundtopografie, Baugrundanteil, Startvorkommen |
| Fluss in der Provinz | Fluss über die Stadtkarte, Mühlen- und Hafenplätze |
| Rohstoffvorkommen der Provinz (z. B. „Eisen 3“) | Anzahl und Größe der Erzadern auf der Karte (Stufe × 1 Ader à 4000 Einheiten) |
| Schleierstärke der Provinz | Aschglasadern, Hohlungsflächen, nächtliche Spawnpunkte |
| Provinzgröße | Kartengröße: klein 128², mittel 192², groß 256², Hauptstadtprovinzen 320² |
| Klima | Tageslänge, Wetterwahrscheinlichkeiten, Ernteperioden |

**Was die Weltkarte aus der RTS-Stadt liest** (jeden Welttag, aggregiert):

| RTS-Zustand | Weltkarten-Wert |
|---|---|
| Summe der Gebäudeproduktion (tatsächlich besetzte Arbeitsplätze × Effizienz) | Provinzertrag je Ressource pro Tag |
| Lagerbestände aller Lagergebäude | Provinzlager (handelbar, für Armeen nutzbar) |
| Wohngebäude und Bewohner je Schicht | Bevölkerung, Steueraufkommen, Rekrutierungspotenzial |
| Mauern, Türme, Tore (Gesamt-Befestigungspunkte) | Befestigungsstufe der Provinz (0–10) und Belagerungsdauer |
| Tempel, Schreine, Bibliotheken | Frömmigkeit, Forschung, Glaubensausbreitung |
| Kasernen und Ställe | Rekrutierbare Einheitentypen auf der Weltkarte |
| Stationierte Einheiten | Garnison (gleichzeitig Weltkarten-Garnison) |
| Stadtrang (abgeleitet) | Siedlung < 50 Einwohner, Dorf 50–199, Marktflecken 200–799, Stadt 800–2.999, Großstadt 3.000–9.999, Metropole ab 10.000 |

**Was die RTS-Stadt aus der Weltkarte liest**:

| Weltkarten-Zustand | RTS-Wirkung |
|---|---|
| Statthalter (Amt) und sein Verwaltungswert | Effizienz aller Gebäude: +1 % pro Punkt Verwaltung über 10 |
| Gesetze und Steuersätze | Zufriedenheit, Goldertrag pro Bewohner |
| Handelsrouten | Ankunft von Handelskarren/-schiffen an Marktplatz und Hafen (sichtbar), Importwaren im Lager |
| Kriegszustand | Feindliche Armeen in der Provinz erzeugen Plünderer an den Kartenrändern; Belagerung schaltet die Stadt in den Belagerungsmodus |
| Ereignisse (Seuche, Dürre, Feuer) | Sichtbare Folgen: kranke Bewohner, verdorrte Felder, Brände in Gebäuden |
| Forschung | Freigeschaltete Gebäude, Einheiten, Verbesserungen |
| Religion der Provinz | Verfügbare Sakralbauten, Glaubenseinheiten |

**Abstrakte Simulation ohne Besuch**: Städte, die der Spieler nicht geöffnet hat, laufen in der
*Hintergrundsimulation*: Gebäude produzieren nach exakt denselben Regeln, aber ohne Wegezeiten der Träger;
stattdessen gilt ein pauschaler Logistikabzug von 10 % (große Städte über 3.000 Einwohner: 15 %). Ein Spieler, der
seine Stadt im RTS optimal anlegt (kurze Wege, Lager nahe an Produktion, Straßen), erreicht dadurch messbar mehr
(bis zu 0 % Logistikverlust bei optimaler Anlage, angezeigt als „Logistikeffizienz“ im Stadtbericht).

**Automatischer Stadtausbau („Statthalterbau“)**: Für Provinzen, die der Spieler nicht selbst bauen will,
übernimmt der Statthalter den Ausbau nach einem wählbaren Schwerpunkt (Nahrung, Handwerk, Militär, Glaube,
Handel, Ausgewogen). Er baut auf der echten Stadtkarte mit Vorlagen (Bezirksbausteine) und ist dabei
10 % weniger effizient als ein guter Spieler. Der Spieler kann jederzeit eingreifen; seine eigenen Gebäude
werden vom Statthalter nie abgerissen.

### 4.4 Ebenenwechsel

- **Weltkarte → Stadt**: Doppelklick auf das Stadtsymbol, Taste `Enter` bei ausgewählter Stadt oder Zoom über
  die maximale Weltkartenstufe hinaus. Übergang: 1,2 s Kamerafahrt mit Überblendung (Weltkarten-Pixelkarte
  zoomt in die Stadtkarte).
- **Stadt → Weltkarte**: Taste `Tab` oder Herauszoomen über die minimale Stadtstufe.
- **Ladeziel**: Ebenenwechsel unter 2 s auf Zielhardware; die Stadtkarte des aktuellen Hofes und der beiden
  zuletzt besuchten Städte werden im Speicher gehalten.

### 4.5 Kopplung Armeen und Schlachten

**Zuordnung**: Eine Armee auf der Weltkarte besteht aus **Regimentern**. Ein Regiment ist eine Einheitengruppe
eines Typs (z. B. 12 `swordsman`). Jede Figur im Regiment entspricht **genau einer RTS-Einheit** (1:1). Helden
reisen mit der Armee als eigene Figur. Damit gibt es keine Umrechnungsverluste: Wer 40 Bogenschützen auf der
Weltkarte hat, hat 40 Bogenschützen auf dem Schlachtfeld.

**Frontbreite**: Auf dem Schlachtfeld stehen pro Seite höchstens **160 Bevölkerungspunkte** gleichzeitig (Ritter
zählen 3, Belagerungsgerät 4, Infanterie 1). Überzählige Regimenter bilden die **Reserve** und rücken in Wellen
nach, sobald Platz frei wird (Reserve erscheint am eigenen Kartenrand, Verzögerung 20 s pro Welle). Dadurch
bleiben auch große Schlachten spielbar und Masse allein entscheidet nicht.

**Auslösung einer Schlacht**: Treffen zwei feindliche Armeen in einer Provinz aufeinander, oder greift eine
Armee eine Stadt mit Garnison an, erscheint der **Schlachtdialog**:

| Option | Wirkung |
|---|---|
| **Schlacht schlagen** | Lädt eine RTS-Schlachtkarte aus Provinzgelände, Wetter, Tageszeit und Jahreszeit; beide Armeen erscheinen entsprechend ihrer Anmarschrichtung |
| **Automatisch berechnen** | Sofortige Auflösung über die Auto-Resolve-Formel (4.6); Ergebnis wird angezeigt, Vorschau zeigt Erwartungswert und Streuung |
| **Rückzug** | Nur möglich, wenn die eigene Armee nicht eingekesselt ist (mindestens eine angrenzende freundliche oder neutrale Provinz); kostet 10 % der Truppen als Nachhutverlust und −10 Moral |
| **Verhandeln** | Nur gegen Menschen-KI; Tribut anbieten oder fordern (Annahme abhängig von Kräfteverhältnis und Meinung) |

Die Schlachtvorschau zeigt: Kräfteverhältnis, erwartete eigene Verluste bei automatischer Berechnung
(Mittelwert und 80-%-Intervall), Geländemodifikatoren, Wetter, Tageszeit, Moral beider Seiten.

**Schlachtkarte**: Die RTS-Schlachtkarte (Größe 96×96 bis 192×192 Felder) wird aus der Provinz erzeugt.
Liegt die Provinz im Einzugsgebiet einer bereits erzeugten Stadtkarte und findet die Schlacht dort statt,
wird die echte Stadtkarte verwendet: Der Spieler verteidigt seine tatsächlich gebauten Mauern.

### 4.6 Auto-Resolve-Formel

Die automatische Berechnung simuliert die Schlacht in **Runden** (je Runde entspricht 10 s RTS-Kampf).

**Kampfkraft einer Einheit**:

```
KK_einheit = (DPS_eff × HP_eff)^0.5 × Veteranenfaktor × Ausrüstungsfaktor
DPS_eff    = Schaden / Angriffsintervall × Matrixfaktor(Schadenstyp, gegnerischer Haupt-Rüstungstyp)
HP_eff     = HP × (1 + Rüstungswert × 0,06)
Veteranenfaktor = 1,00 / 1,10 / 1,20 / 1,35 (Rekrut / Erfahren / Veteran / Elite)
```

**Kampfkraft einer Seite in Runde r**:

```
KK_seite = Σ KK_einheit (aller Einheiten an der Front)
         × Gelände × Befestigung × Kommandant × Moralfaktor × Versorgung × Tageszeit × Wetter
         × Zufall(0,85 … 1,15)        // pro Runde neu, deterministisch aus Seed
Moralfaktor = 0,5 + Moral/200          // Moral 0–100 → 0,5–1,0
Kommandant  = 1 + 0,02 × (Kriegskunst des Anführers) + Heldenboni
```

**Verluste**: Seite A verliert in Runde r Trefferpunkte in Höhe von `KK_B × 0,8`, verteilt auf die Front
gemäß Zielpriorität (Nahkampf trifft vordere Reihe, Fernkampf und Magie verteilen gleichmäßig). Einheiten
sterben, wenn ihre HP-Summe aufgebraucht ist. Moral sinkt pro Runde um `10 × (Verluste der Runde / Anfangsstärke)`
plus 5, wenn KK_gegner > 1,5 × KK_eigen. Bei Moral unter 20 bricht die Seite; die Schlacht endet.

**Helden** gehen mit `KK_held = KK_einheit × 2` ein und haben pro Runde eine Verletzungswahrscheinlichkeit von
`2 % + 1 % × (verlorener HP-Anteil der Armee in %)/10`. Ihre Fähigkeiten werden als pauschaler Bonus
verrechnet (z. B. Isoldes Sammelbanner: +8 % Kampfkraft und +5 Moral zu Beginn).

**Auto-Resolve-Malus**: Um das Selbstspielen zu belohnen, ohne Auto-Resolve unbrauchbar zu machen, erhält der
Spieler bei automatischer Berechnung **keinen** Malus, aber auch keine Taktikboni (Flankierung,
Hinterhalt, Fokusfeuer). Die KI rechnet untereinander mit derselben Formel. Erfahrungsgemäß erzielt ein
geübter Spieler in einer gespielten Schlacht 20–40 % geringere Verluste.

### 4.7 Ergebnisübernahme

Nach jeder Schlacht – gespielt oder berechnet – werden folgende Werte in die Weltkarte geschrieben:

| Ergebnis | Übernahme |
|---|---|
| Überlebende Einheiten | Exakt, pro Figur; verwundete Einheiten behalten ihre HP und heilen auf der Weltkarte mit 5 % HP pro Tag (in eigener Provinz 10 %, mit Feldscher 15 %) |
| Gefallene Einheiten | 70 % tot, 30 % „verwundet heimgekehrt“ (kehren nach 30 Tagen in die Bevölkerung der Herkunftsprovinz zurück, nicht in die Armee) |
| Erfahrung | Einheiten erhalten Veteranenpunkte; Helden Erfahrungspunkte (Kapitel 11.2) |
| Helden | Zustand: unverletzt, verletzt (mit Verletzungstyp), gefangen, tot (Kapitel 11.6) |
| Gefangene | Gefangene Adlige und Helden werden zu Verhandlungsobjekten (Lösegeld, Austausch, Hinrichtung) |
| Beute | Gold und Ausrüstung: 10 % des Unterhaltswertes der besiegten Armee, plus Gegenstände getöteter Helden |
| Moral | Siegerarmee +10, Verlierer −20 (Nachwirkung 60 Tage) |
| Kriegsziel-Punkte | Kriegspunkte (Kapitel 7.6) entsprechend der Verlustdifferenz |
| Gelände | Schlachtfelder hinterlassen „verbrannte Erde“ (−20 % Ernte für 1 Jahr) bei mehr als 100 Toten; in Provinzen mit Schleierstärke ≥ 3 steigt die Hohlung um 1 pro 50 Tote (unbestattete Gefallene) |
| Chronik | Eintrag mit Namen der Schlacht (generiert aus Ort und Ereignis), Anführern, Stärken, Verlusten und Heldentaten |

**Rückzug während einer gespielten Schlacht**: Der Spieler kann jederzeit den Rückzug befehlen (Taste `F10`
→ „Rückzug“). Einheiten, die den eigenen Kartenrand innerhalb von 30 s erreichen, überleben; die übrigen
gelten als gefallen oder gefangen. Rückzug ist nicht möglich, wenn die Armee eingekesselt ist.

### 4.8 Belagerungen über beide Ebenen

1. Eine Armee betritt eine Provinz mit befestigter Stadt → **Belagerung beginnt** auf der Weltkarte.
2. Fortschritt pro Tag: `Belagerungsfortschritt += (Belagerungsstärke − Befestigungsstufe × 2) %`, mindestens
   0,5 % pro Tag; Belagerungsstärke = Anzahl Belagerungsgeräte × 3 + Pioniere × 1 + Heldenboni.
3. Die belagerte Stadt verbraucht ihr Lager; ist Nahrung erschöpft, verliert die Garnison 2 % pro Tag und die
   Zufriedenheit sinkt um 3 pro Tag.
4. Der Belagerer kann jederzeit **stürmen** → RTS-Belagerungsschlacht auf der echten Stadtkarte. Bereits erzielter
   Belagerungsfortschritt öffnet Breschen: je 25 % Fortschritt ist ein Mauerabschnitt eingestürzt.
5. Der Verteidiger kann einen **Ausfall** wagen → RTS-Schlacht vor den Mauern, eigene Türme feuern mit.
6. Bei 100 % Fortschritt kapituliert die Stadt automatisch, sofern der Verteidiger nicht stürmt.

---

## 5. Dynastien und Herrscher

### 5.1 Grundprinzip

Der Spieler verkörpert nicht eine Figur, sondern **eine Dynastie**. Der aktuelle Herrscher ist die
Spielfigur auf der Weltkarte; stirbt er, übernimmt der Spieler den Erben. In der Grand Campaign beginnt die
Dynastie **Raventhorn** mit Isolde Raventhorn als Wächterin der Aschmark (Rang: Grenzlady, Vasallin der
Glutkrone). Im Sandbox-Modus kann jede Dynastie jeder Fraktion gewählt oder neu gegründet werden.

### 5.2 Herrscher-Attribute

Jede Figur (Herrscher, Familienmitglied, Höfling, Held) besitzt sechs Attribute im Bereich **0–20**
(Durchschnitt eines Erwachsenen: 6–8; außergewöhnlich: 14+).

| Attribut | ID | Wirkung je Punkt (als Herrscher) | Wirkung im Amt |
|---|---|---|---|
| Kriegskunst | `martial` | +1 % Kampfkraft aller Armeen unter persönlichem Kommando; +0,5 % Rekrutierungsgeschwindigkeit | Marschall |
| Verwaltung | `stewardship` | +0,5 % Steuerertrag; +1 Provinz Verwaltungslimit je 2 Punkte | Schatzmeister, Statthalter |
| Diplomatie | `diplomacy` | +1 Meinung bei allen Herrschern; +1 % Annahmechance diplomatischer Angebote | Diplomat, Kanzler |
| Intrige | `intrigue` | +1 % Verschwörungsstärke eigener Komplotte; +1 % Entdeckungschance fremder Komplotte | Spionagemeister |
| Gelehrsamkeit | `learning` | +1 % Forschungsgeschwindigkeit; +0,5 % Magieresistenz der Hofmagier | Kanzler, Hohepriester |
| Charisma | `charisma` | +0,5 Zufriedenheit in der Hauptstadt; +1 Meinung bei Vasallen; +1 % Heldenrekrutierungschance | alle Ämter (Nebenwirkung: Meinung der Amtsgruppe) |

Zusätzliche Werte jeder Figur:

| Wert | Bereich | Bedeutung |
|---|---|---|
| Gesundheit | 0–10 | Sterbewahrscheinlichkeit (5.6); unter 3 „kränklich“ |
| Fruchtbarkeit | 0–100 % | Kinderwahrscheinlichkeit pro Jahr in Ehe |
| Prestige | 0–∞ | Ansehen der Person; Ränge: Unbekannt (<100), Geachtet (100), Gerühmt (500), Legendär (2.000) |
| Frömmigkeit | 0–∞ | Ansehen beim Glauben; Währung für religiöse Handlungen |
| Hohlung | 0–100 | Persönliche Korruption durch das Hohle Tief (Kapitel 12.8) |
| Stress | 0–100 | Handeln gegen die eigenen Eigenschaften erzeugt Stress; ab 50 Gefahr eines Zusammenbruchs |
| Alter | Jahre | Volljährigkeit mit 16, Alter ab 55, Greisenalter ab 70 |

### 5.3 Eigenschaften (Traits)

Jede Figur hat **2–5 Eigenschaften**. Persönlichkeitseigenschaften (höchstens 3) erzeugen Stress, wenn der
Herrscher gegen sie handelt. Erworbene Eigenschaften entstehen durch Ereignisse.

**Persönlichkeit**

| Eigenschaft | ID | Modifikatoren | Stress-Auslöser |
|---|---|---|---|
| Tapfer | `brave` | +2 Kriegskunst, +10 Moral der eigenen Armee, +5 % Verletzungsrisiko | Rückzug befehlen (+10 Stress) |
| Feige | `craven` | −2 Kriegskunst, +2 Intrige, −10 Moral eigener Armee, −50 % Verletzungsrisiko | Persönlich kämpfen (+10) |
| Gerecht | `just` | +2 Verwaltung, +5 Meinung bei Bauern und Bürgern, −10 % Korruption | Ansprüche fälschen, Gefangene ohne Prozess hinrichten (+15) |
| Grausam | `cruel` | +2 Intrige, −10 Meinung allgemein, Hinrichtungen ohne Tyrannei-Malus, Furcht +20 % | Begnadigen (+10) |
| Gnädig | `merciful` | +5 Meinung allgemein, −20 % Aufstandswahrscheinlichkeit nach Eroberung | Hinrichten (+20) |
| Ehrgeizig | `ambitious` | +1 auf alle Attribute, −10 Meinung bei Vasallen, Stress +2/Monat ohne Machtgewinn in 5 Jahren | Friedensangebote ohne Gewinn annehmen (+5) |
| Zufrieden | `content` | −1 Intrige, +10 Meinung bei Vasallen, −20 % Stressaufbau | Angriffskrieg erklären (+10) |
| Fromm | `zealous` | +20 % Frömmigkeitsgewinn, +15 Meinung beim eigenen Klerus, −15 Meinung bei Andersgläubigen | Pakt mit Andersgläubigen (+10) |
| Zweifler | `cynical` | +2 Intrige, +2 Gelehrsamkeit, −10 % Frömmigkeitsgewinn, Bekehrung anderer Religionen ohne Malus | Pilgerfahrt (+5) |
| Gierig | `greedy` | +10 % Steuerertrag, −10 Meinung bei Vasallen | Großzügige Geschenke machen (+10) |
| Großzügig | `generous` | +2 Diplomatie, Geschenke wirken 25 % stärker, −5 % Steuerertrag | Steuern erhöhen (+10) |
| Misstrauisch | `paranoid` | +3 Intrige, +25 % Entdeckungschance fremder Komplotte, −2 Diplomatie, Stress +1/Monat | Spionagemeister entlassen (+5), keine |
| Geduldig | `patient` | +2 Verwaltung, +2 Gelehrsamkeit, Belagerungsfortschritt +10 % | Krieg innerhalb von 2 Jahren nach Frieden erklären (+10) |
| Jähzornig | `wrathful` | +3 Kriegskunst, −2 Diplomatie, +25 % Schaden im persönlichen Kampf | Beleidigungen ignorieren (+10) |
| Ehrlich | `honest` | +2 Diplomatie, −4 Intrige, Bündnistreue: KI vertraut Verträgen +20 % | Lügen in Verhandlungen/Komplotte (+15) |
| Verschlagen | `deceitful` | +4 Intrige, −2 Diplomatie, Komplott-Geheimhaltung +15 % | Geständnis ablegen (+10) |

**Erworbene und angeborene Eigenschaften**

| Eigenschaft | ID | Modifikatoren | Entstehung |
|---|---|---|---|
| Aschenfieber-Narbe | `ash_fever_scar` | +2 Gesundheitsresistenz gegen Seuchen, −1 Charisma, +5 % Hohlungsresistenz | Überleben des Aschenfiebers (Isolde trägt sie) |
| Genie | `genius` | +5 auf alle Attribute, erblich (15 %) | Angeboren, 1 % |
| Kluger Kopf | `quick` | +2 auf alle Attribute | Angeboren, 5 % |
| Einfältig | `slow` | −3 auf alle Attribute | Angeboren, 5 % |
| Riese | `giant` | +3 Kriegskunst, +20 % HP als Held, −1 Gesundheit | Angeboren, 2 % |
| Kränklich | `sickly` | −2 Gesundheit, −10 % Fruchtbarkeit | Angeboren 3 % oder nach Seuche |
| Einäugig | `one_eyed` | −1 Diplomatie, +1 Kriegskunst (Prestige der Narbe), −10 % Fernkampfgenauigkeit als Held | Schwere Verletzung |
| Verkrüppelt | `maimed` | −3 Kriegskunst, kann nicht mehr persönlich kämpfen | Schwere Verletzung |
| Kriegsheld | `war_hero` | +100 Prestige, +5 Moral aller Armeen, +10 Meinung beim Adel | Entscheidender Sieg gegen Übermacht |
| Sippenmörder | `kinslayer` | −30 Meinung bei allen Mitgliedern der eigenen Dynastie, −15 allgemein | Mord an Verwandten |
| Hohlberührt | `hollow_touched` | +2 Gelehrsamkeit, +3 Hohlung/Jahr, −10 Meinung bei Gläubigen der Ewigen Flamme | Kontakt mit Aschglas-Artefakten oder Hohlen |
| Flammengeweiht | `flame_sworn` | +20 % Glutmagie-Wirkung, +15 Meinung beim Klerus der Flamme, Hohlung kann nicht über 20 steigen | Weihe durch den Erzflammenwahrer in Kaldris |
| Gelehrter | `scholar` | +3 Gelehrsamkeit, +10 % Forschung | Erziehung mit Fokus Gelehrsamkeit |
| Strategin/Stratege | `strategist` | +3 Kriegskunst, Flankierungsbonus der Armee +10 % | Erziehung mit Fokus Kriegskunst |
| Höfling | `courtier` | +3 Diplomatie, Heiratsakzeptanz +10 % | Erziehung mit Fokus Diplomatie |
| Trinker | `drunkard` | −1 Verwaltung, −20 % Stress | Stressbewältigung |
| Melancholisch | `melancholic` | −2 auf alle Attribute | Stresszusammenbruch |
| Verrückt | `lunatic` | zufällige Entscheidungen bei Ereignissen, −10 Meinung allgemein | Hohlung ≥ 60 oder Stresszusammenbruch (5 %) |

### 5.4 Nachfolge

Die Nachfolge richtet sich nach dem **Erbgesetz** des Reiches. Ein Wechsel des Erbgesetzes erfordert
Zustimmung des Adelsrats (Kapitel 6.2) und kostet 500 Prestige; nach einem Wechsel ist 20 Jahre kein erneuter
Wechsel möglich.

| Erbgesetz | ID | Regel | Vorteile | Nachteile |
|---|---|---|---|---|
| Primogenitur | `primogeniture` | Ältestes Kind erbt alles | Reichseinheit | Enttäuschte jüngere Kinder (+10 % Verschwörungsneigung) |
| Ultimogenitur | `ultimogeniture` | Jüngstes Kind erbt | Längere Regentschaften selten nötig | −10 Meinung bei Erstgeborenen |
| Teilung | `partition` | Titel werden unter allen Söhnen und Töchtern aufgeteilt; Haupttitel an das älteste | +20 Meinung der Kinder | Zerfall des Reiches |
| Wahlmonarchie | `elective` | Adelsrat wählt aus Dynastiemitgliedern und mächtigen Vasallen | Fähigster Erbe möglich | Andere Häuser können den Thron gewinnen |
| Seniorat | `seniority` | Ältestes Mitglied der Dynastie erbt | Erfahrene Herrscher | Kurze Regierungszeiten |
| Ernennung | `designation` | Herrscher ernennt Erben frei (nur mit Legitimität ≥ 70) | Volle Kontrolle | −20 Meinung bei übergangenen Erben |

**Geschlecht**: In Valemark gilt seit dem Erlass der Grenzladys (Jahr 612 n.F.) **gleichberechtigte Erbfolge**;
Skarnheim folgt dem Clanrat (gleichberechtigt), der Saelith-Pakt vererbt keine Titel (Händlerfürsten werden
gewählt), der Dornenbund bestimmt Erben durch den Zirkel. Spielregeln, die Geschlecht benachteiligen, gibt es
nicht; kulturelle Unterschiede wirken über Erbgesetze, nicht über Geschlecht.

**Legitimität (0–100)** des Herrschers: Start 50 + 10 je Generation der Dynastie auf dem Thron (max. +30),
+20 bei Erbe nach geltendem Gesetz, −30 bei Usurpation, +1 pro Jahr Regierung (max. +20). Unter 30 drohen
Thronansprüche anderer Häuser; über 80 sind Steuer- und Gesetzesänderungen 50 % billiger.

### 5.5 Regentschaft

Ist der Erbe beim Tod des Herrschers jünger als 16 Jahre oder handlungsunfähig (Verrückt, Gefangenschaft),
tritt eine **Regentschaft** ein.

- **Regent** wird automatisch der überlebende Ehepartner, sonst der Kanzler, sonst das mächtigste Mitglied der
  Dynastie. Der Spieler steuert weiter, aber:
  - Krieg erklären und Erbgesetze ändern ist gesperrt.
  - Steuersätze können nur um eine Stufe pro Jahr verändert werden.
  - Der Regent hat eine eigene **Regentenloyalität** (0–100). Pro Monat: `Usurpationschance = max(0, (50 − Loyalität)/10) % × (1 + Ehrgeizig)`.
  - Der Spieler kann die Loyalität über den Erziehungsfokus des Mündels, Geschenke (Gold) und Ämterverteilung
    beeinflussen.
- **Erziehung**: Kinder ab 6 Jahren erhalten einen Erzieher (Höfling mit hohem Attribut) und einen Fokus. Mit
  16 Jahren erhält das Kind Attributsboni: `Bonus = (Attribut des Erziehers − 8)/2` im Fokusattribut und eine
  Erziehungseigenschaft (Gelehrter, Strategin/Stratege, Höfling, Verwalter, Ränkeschmied, Frommer Geist).

### 5.6 Familie und Generationen

- **Hof**: Der Hof umfasst Familie, Höflinge (max. 20 + 2 je Stadtrang der Hauptstadt) und Gäste.
- **Kinder**: Jährliche Wahrscheinlichkeit eines Kindes in einer Ehe: `Fruchtbarkeit_A × Fruchtbarkeit_B × 0,9`,
  verringert ab 40 Jahren um 10 % pro Jahr.
- **Sterblichkeit** pro Jahr: `0,3 % × 1,09^(Alter − 30)` für Alter > 30, multipliziert mit `(11 − Gesundheit)/6`;
  zusätzlich Seuchen, Kampf und Mord. Erwartete Lebensdauer bei Gesundheit 6: etwa 65 Jahre.
- **Vererbung**: Kinder erben je Attribut den Mittelwert der Eltern ±3 (Zufall), angeborene Eigenschaften mit
  15–50 % Wahrscheinlichkeit.
- **Dynastieprestige**: Summe der Leistungen aller Mitglieder. Stufen schalten **Dynastie-Vermächtnisse** frei
  (je 1.000 Punkte eines von 5 Vermächtnissen, z. B. „Wächter des Südens“: +15 % Kampfkraft gegen Hohle;
  „Glutblut“: Kinder erhalten mit 20 % die Eigenschaft Flammengeweiht; „Handelshaus“: +10 % Handelsertrag).
- **Nebenzweige**: Verheiratete Kinder ohne Thronanspruch gründen Kadettenhäuser. Kadettenhäuser sind
  Adelshäuser (Kapitel 6.1) mit Grundmeinung +20 zum Hauptzweig und spielbare Fortsetzung, wenn der Hauptzweig
  ausstirbt.

### 5.7 Heirat

| Heiratsform | Wirkung |
|---|---|
| Standardehe | Kinder gehören zur Dynastie des höherrangigen Partners; bei Gleichrang wählbar |
| Matrilineare/patrilineare Ehe | Festlegung, welcher Dynastie Kinder angehören |
| Bündnisheirat | Eheschließung zwischen Herrscherfamilien erzeugt **Familienbund**: +25 Meinung, automatisches Verteidigungsangebot (kann abgelehnt werden, −20 Meinung) |
| Erbheirat | Ehe mit einem Erben eines anderen Reiches: Kinder erhalten Anspruch auf dessen Titel |

**Annahme eines Heiratsangebots** (Wahrscheinlichkeit in %):

```
Annahme = 30 + (Rang_Antragsteller − Rang_Ziel) × 15
             + Meinung_Ziel_zu_Antragsteller × 0,5
             + (Attributsumme_Kandidat − 40) × 0,5
             + 20 bei gleicher Religion, −30 bei verfeindeter Religion
             + Mitgift_Gold / (Jahreseinkommen_Ziel × 0,1) × 5   (max. +25)
             − 50, falls der Kandidat Hohlung ≥ 40 besitzt
```

Ränge: Grenzherr 1, Graf 2, Herzog 3, König 4, Hochkönig 5.

### 5.8 Generationenwechsel als Spielrhythmus

Ein Spielzug der Grand Campaign umfasst typischerweise **150–250 Jahre** (5–10 Herrscher). Jeder
Herrscherwechsel ist ein bewusster Spielmoment: Der Bildschirm „Nachfolge“ zeigt den Erben, seine
Eigenschaften, die Reaktion der Adelshäuser (Meinungsänderung), offene Thronansprüche und die
Chronik-Zusammenfassung der vorherigen Regentschaft mit Beinamen (z. B. „Isolde die Unbeugsame“), der aus den
wichtigsten Taten abgeleitet wird (Kapitel 17.4).

---

## 6. Politik, Adelshäuser, Ämter und Intrigen

### 6.1 Adelshäuser

Jedes Reich besteht aus dem Kronland des Herrschers und den Lehen der **Adelshäuser** (Vasallen). Ein Adelshaus
hat:

| Wert | Bereich | Bedeutung |
|---|---|---|
| Macht | 0–100 | Anteil an Provinzen, Truppen und Einkommen des Reiches (gewichtet) |
| Loyalität | 0–100 | Bereitschaft, Befehlen zu folgen; abgeleitet aus Meinung des Hausoberhaupts und Rechten |
| Ansprüche | Liste | Ansprüche auf Titel (auch auf den Thron) |
| Rechte | Liste | Privilegien des Hauses (Steuerbefreiung, eigene Armee, Sitz im Rat, Stadtrecht) |
| Gesinnung | Kategorie | Kriegspartei, Handelspartei, Glaubenspartei, Kronpartei, Separatisten |

**Vasallenpflichten**: Abgaben (Standard 20 % des Lehenseinkommens) und Heerfolge (Standard 25 % der
Lehenstruppen). Beide können per Gesetz geändert werden: +5 Prozentpunkte Abgabe = −5 Loyalität aller Häuser.

**Loyalitätsformel** (monatlich neu berechnet):

```
Loyalität = 50 + Meinung_Oberhaupt_zu_Herrscher × 0,4
              + 5 × Anzahl gewährter Rechte
              − (Abgabe − 20) × 1
              − (Heerfolge − 25) × 0,8
              + Legitimität_Herrscher × 0,1 − 5
              − 15, wenn das Haus einen unerfüllten Anspruch auf den Thron hält
              + 10, wenn ein Mitglied des Hauses ein Amt bekleidet
```

Häuser mit Loyalität < 30 können Fraktionen bilden (6.5); Häuser mit Loyalität < 15 und Macht > 20 können
offen rebellieren.

### 6.2 Der Adelsrat

Reiche mit mindestens drei Adelshäusern besitzen einen Adelsrat. Bestimmte Handlungen benötigen Zustimmung
(Mehrheit nach Macht gewichtet): Erbgesetz ändern, Abgaben über 30 % festlegen, Angriffskrieg gegen ein
Reich gleicher Religion, Rechte entziehen. Der Herrscher kann die Zustimmung über Gefälligkeiten
(„Schuldscheine“ aus früheren Gefallen), Gold oder Drohungen gewinnen. Ein Ratsentscheid ohne Mehrheit
durchzusetzen ist möglich, kostet aber −20 Loyalität bei allen ablehnenden Häusern und 200 Prestige.

### 6.3 Ämter

Der Herrscher besetzt sieben Ämter. Jedes Amt hat eine **Aufgabe** (aktiv gewählte Tätigkeit) und einen
**Grundeffekt**. Wirkungen skalieren mit dem relevanten Attribut des Amtsinhabers (A).

| Amt | ID | Attribut | Grundeffekt | Aufgaben (eine aktiv) |
|---|---|---|---|---|
| **Kanzler** | `office_chancellor` | Diplomatie (Gelehrsamkeit als Zweitwert ×0,5) | +A × 0,5 Meinung aller fremden Herrscher; Verwaltungslimit +A/4 Provinzen | *Beziehungen pflegen*: +A Meinung eines gewählten Reiches/Jahr (max. +30). *Ansprüche erarbeiten*: Anspruch auf eine Grenzprovinz in `max(6, 36 − A) Monaten`. *Gesetze verwalten*: Kosten von Gesetzesänderungen −A × 2 %. |
| **Marschall** | `office_marshal` | Kriegskunst | Moral aller Armeen +A/2; Rekrutierungskosten −A × 1 % | *Truppen drillen*: Erfahrung aller Garnisonen +A × 0,5 Veteranenpunkte/Monat. *Rekrutierung leiten*: Rekrutierungszeit −A × 2 %. *Befestigungen inspizieren*: Befestigungsstufe in einer Provinz +1 (alle `24 − A/2` Monate, max. Stufe 10). |
| **Schatzmeister** | `office_treasurer` | Verwaltung | Steuerertrag +A × 0,5 %; Kreditzins −A × 0,1 Prozentpunkte | *Steuern eintreiben*: +A × 1 % Steuern, −A/4 Zufriedenheit in Krongebiet. *Handel fördern*: Handelsertrag +A × 1 %. *Korruption bekämpfen*: Korruptionsverlust −A × 0,5 Prozentpunkte. |
| **Diplomat** | `office_diplomat` | Diplomatie | Annahmechance diplomatischer Vorschläge +A × 1 %; Reisezeit von Gesandten −A × 3 % | *Gesandtschaft*: Ständige Vertretung bei einem Reich: +A Meinung, sieht dessen Kriegsziele. *Bündnis schmieden*: Bündnisannahme mit Ziel +A × 2 %. *Frieden vermitteln*: Kriegsmüdigkeit beider Seiten in einem Fremdkrieg +A × 0,5/Monat (ermöglicht Vermittlerprestige). |
| **Spionagemeister** | `office_spymaster` | Intrige | Eigene Verteidigung gegen Komplotte +A × 2 %; sieht fremde Armeen in Nachbarprovinzen | *Komplotte aufdecken*: Entdeckungschance im eigenen Reich +A × 3 %. *Spionagenetz*: Netzwerkstärke in einem Zielreich +A/Monat (0–100), Netz ermöglicht Komplotte und deckt Armeen, Bündnisse, Geheimnisse auf. *Sabotage*: Produktion einer feindlichen Provinz −A × 1 % (aufgedeckt mit `30 − A %` pro Monat). |
| **Hohepriester** | `office_high_priest` | Gelehrsamkeit (Frömmigkeit ab 500 als Bonus +2) | Frömmigkeit +A × 0,1/Monat; Glaubenszufriedenheit +A/4 | *Glauben verbreiten*: Bekehrungsfortschritt in einer Provinz +A × 0,5 %/Monat. *Hohlung reinigen*: Hohlung einer Provinz −A × 0,2/Monat. *Häresie verfolgen*: Häresiewahrscheinlichkeit −A × 2 %, aber −A/4 Zufriedenheit bei Gelehrten. |
| **Statthalter** | `office_steward` (pro Provinz ein Statthalter möglich, der „Oberste Statthalter“ ist Ratsamt) | Verwaltung | Gebäudeeffizienz der verwalteten Provinz +1 % je Punkt über 10, −1 % je Punkt unter 10; Ausbau nach Schwerpunkt (4.3) | *Ausbau*: Bautempo +A × 2 %. *Ordnung*: Unruhe −A × 0,3/Monat. *Erntepflege*: Nahrungsertrag +A × 1 %. |

**Amtsmissbrauch**: Inhaber mit der Eigenschaft Gierig unterschlagen `(20 − Verwaltung)%` ihres Mehrertrags;
Ehrgeizige Inhaber erhalten +5 % Verschwörungsneigung, weil sie Zugang zu Macht haben. Entlassungen kosten
−20 Meinung beim Entlassenen und −5 bei seinem Haus.

**Ämterbesetzung durch Häuser**: Ein Amt, das einem Mitglied eines Adelshauses verliehen wird, gibt diesem Haus
+10 Loyalität (siehe Formel 6.1). Die Ämterverteilung ist damit das wichtigste Werkzeug zur Befriedung des Adels.

### 6.4 Gesetze

Gesetze werden in Kategorien festgelegt (je Kategorie eine Stufe):

| Kategorie | Stufen (Auswahl) | Hauptwirkung |
|---|---|---|
| Kronautorität | Schwach / Begrenzt / Hoch / Absolut | Abgaben-Maximum 20/30/40/50 %; Loyalitätsmalus 0/−5/−10/−20 |
| Erbgesetz | siehe 5.4 | Nachfolge |
| Leibeigenschaft | Leibeigenschaft / Gebundene Pacht / Freie Bauern | Produktivität Leibeigene 80/90/100 %; Zufriedenheit Leibeigene −10/0/+10; Adelsloyalität +10/0/−10 |
| Stadtrechte | Keine / Marktrecht / Freie Städte | Handelsertrag +0/+10/+25 %; Bürgerzufriedenheit −5/+5/+15; Krongewalt über Städte voll/voll/eingeschränkt |
| Glaubensgesetz | Staatskirche / Duldung / Glaubensfreiheit | Glaubenseinheit vs. Zufriedenheit Andersgläubiger |
| Magiegesetz | Verbot / Lizenz / Freiheit | Magierzufriedenheit, Forschungsgeschwindigkeit, Hohlungsrisiko |
| Kriegsrecht | Friedensordnung / Wehrbereitschaft / Kriegsrecht | Rekrutierung +0/+20/+50 %, Zufriedenheit 0/−5/−15 |

### 6.5 Fraktionen

Unzufriedene Häuser (Loyalität < 30) schließen sich zu Fraktionen mit einer Forderung zusammen:
„Weniger Kronautorität“, „Unabhängigkeit“, „Anderer Thronerbe“, „Krieg gegen X“, „Religiöse Reform“.

```
Fraktionsstärke = Σ Macht der Mitglieder × (1 + 0,02 × Intrige des Anführers)
Ultimatum, wenn Fraktionsstärke ≥ 1,2 × Kronstärke (Krongebietstruppen + loyale Häuser)
```

Bei einem Ultimatum kann der Herrscher nachgeben (Forderung erfüllt, −100 Prestige) oder ablehnen
(Bürgerkrieg). Bürgerkriege laufen auf beiden Ebenen: Die Fraktion stellt eine Armee aus ihren Lehen auf und
kann zu RTS-Schlachten gezwungen werden.

### 6.6 Intrigen und Verschwörungen

#### Komplottarten

| Komplott | ID | Ziel | Grundschwierigkeit | Erfolgswirkung |
|---|---|---|---|---|
| Mord | `plot_murder` | Figur | 60 | Zielfigur stirbt |
| Entführung | `plot_abduct` | Figur | 70 | Ziel wird Gefangener des Anführers |
| Rufmord | `plot_slander` | Figur | 30 | −150 Prestige und −20 Meinung beim Lehnsherrn des Ziels |
| Anspruch fälschen | `plot_fabricate` | Titel | 40 | Anspruch auf einen Titel |
| Aufwiegeln | `plot_incite` | Provinz | 45 | Unruhe +30, Aufstand wahrscheinlich |
| Usurpation | `plot_usurp` | Thron | 90 | Anführer oder sein Kandidat besteigt den Thron |
| Diebstahl | `plot_steal` | Artefakt/Forschung | 50 | Gegenstand oder Forschungsfortschritt (25 %) wechselt den Besitzer |
| Heiratsverführung | `plot_seduce` | Figur | 35 | Affäre: Geheimnis als Druckmittel oder Meinung +40 |

#### Komplottstärke und Erfolg

Jedes Komplott hat einen **Anführer** und bis zu sechs **Mitverschwörer**. Mitverschwörer werden über
Überredung (Meinung), Druckmittel (Geheimnisse) oder Bestechung (Gold) angeworben.

```
Komplottstärke K = Intrige_Anführer × 3
                 + Σ (Intrige_Mitverschwörer × 1 + Macht_Mitverschwörer × 0,5)
                 + Netzwerkstärke_im_Zielreich × 0,3
                 + Eigenschaftsboni (Verschlagen +15 %, Ehrlich −15 %)

Abwehr A          = Intrige_Spionagemeister_Ziel × 3
                 + Intrige_Ziel × 1,5
                 + Leibwache (Helden in der Nähe: +10 je Held, max. +30)
                 + 20, falls das Ziel Misstrauisch ist

Erfolgschance P   = clamp(Grundwert + (K − A) − Grundschwierigkeit + 50, 5 %, 95 %)
                    mit Grundwert = 0
```

Ein Komplott braucht **Vorbereitung**: Fortschritt pro Monat = `K / 10` Prozentpunkte; bei 100 % kann es
ausgelöst werden (manuell oder automatisch). Je länger es läuft, desto größer die Entdeckungsgefahr:

```
Entdeckungschance pro Monat = (A / (K + A)) × 8 % × (1 + 0,1 × Anzahl Mitverschwörer)
                              × (1,5, falls ein Mitverschwörer Ehrlich ist)
```

**Entdeckung**: Das Ziel erfährt Anführer (wenn `Intrige_Spionagemeister ≥ Intrige_Anführer`) oder nur
einen Mitverschwörer. Folgen: Gefangennahme als Rechtfertigung (Anspruch auf Haft ohne Tyrannei-Malus),
Meinung −50 (Erinnerung „Verschwörer“, Abklingzeit 30 Jahre), Kriegsgrund.

**Scheitern beim Auslösen**: Ausführende werden mit 50 % gefasst; Anführer wird mit
`50 % × (1 − Intrige_Anführer/25)` enttarnt.

#### Geheimnisse

Figuren können **Geheimnisse** besitzen (Affäre, Mord, Ketzerei, Hohlkult, Unehelichkeit, Unterschlagung).
Aufgedeckte Geheimnisse sind Druckmittel: Eine erpresste Figur hat für Komplottanwerbungen und
Ratsabstimmungen eine Pflicht-Zustimmung („Haken“), solange das Geheimnis nicht öffentlich wird. Öffentliche
Geheimnisse wirken als Meinungsmalus (Hohlkult −60 bei Gläubigen der Ewigen Flamme, Ketzerei −30, Mord −40).

---

## 7. Diplomatie

### 7.1 Akteure

Diplomatie findet zwischen **Reichen** (vertreten durch Herrscher) statt; Adelshäuser innerhalb eines
Reiches nutzen die Politikmechanik (Kapitel 6). Die Hohlen und die Aschenlegion sind **nicht verhandelbar** –
mit einer Ausnahme: Spieler mit Hohlung-Pfad (Kapitel 12.8) können ab Stufe 3 mit der Stimme aus dem Tief
in Kontakt treten.

### 7.2 Diplomatische Aktionen

| Aktion | Kosten | Voraussetzungen | Wirkung |
|---|---|---|---|
| Gesandten schicken | 25 Gold | – | +10 Meinung (Abklingzeit 10 Jahre, stapelbar bis +30) |
| Geschenk | Gold (frei) | – | Meinung +`Gold / (Monatseinkommen_Ziel) × 10`, max. +40 |
| Handelsabkommen | – | Meinung ≥ 0 | Beidseitige Handelsrouten, −50 % Zölle, +5 Meinung/Jahr bis +20 |
| Nichtangriffspakt | 50 Prestige | Meinung ≥ 10 | 10 Jahre kein Krieg ohne Vertragsbruch |
| Militärischer Zugang | – | Meinung ≥ 20 | Truppen dürfen Gebiet durchqueren |
| Verteidigungsbündnis | – | Meinung ≥ 40, kein Krieg gegeneinander | Beistand im Verteidigungsfall |
| Volles Bündnis | 100 Prestige | Meinung ≥ 60 oder Familienbund | Beistand in allen Kriegen (Aufruf kann abgelehnt werden: −25 Meinung, −100 Prestige) |
| Heirat | Mitgift | siehe 5.7 | Familienbund |
| Tribut fordern | – | Stärkeverhältnis ≥ 2:1 | Ziel zahlt 10 % des Einkommens; Annahme wenn `Stärke_Antragsteller/Stärke_Ziel × 25 + Meinung × 0,3 > 60` |
| Vasallisierung anbieten | 200 Prestige | Ziel kleiner, Meinung ≥ 50 oder Stärkeverhältnis ≥ 4:1 | Ziel wird Vasall |
| Garantie aussprechen | 25 Prestige | – | Kriegserklärung gegen das Garantierte wird als Angriff auf den Garanten gewertet |
| Anspruch durchsetzen | Kriegsgrund | gültiger Anspruch | Krieg mit Kriegsziel „Titel übernehmen“ |
| Krieg erklären | Kriegsgrund oder 300 Prestige + Tyrannei | siehe 7.6 | Krieg |
| Frieden anbieten | – | im Krieg | siehe 7.6 |
| Gemeinsame Front gegen das Tief | – | beide grenzen an Hohlland | Bündnis gegen Hohle: +30 Meinung, gemeinsame Sicht, Bedrohung ruht gegeneinander |
| Gefangenen freikaufen/austauschen | Gold | Gefangener vorhanden | Rückkehr der Figur |
| Exil gewähren | – | Flüchtling vorhanden | Höfling mit Anspruch wird aufgenommen (Kriegsgrund für später) |

### 7.3 Meinungsmodell

Die Meinung eines Herrschers A über B liegt zwischen **−100 und +100** und ist die Summe aller aktiven
Modifikatoren. Jeder Modifikator hat einen Wert, eine **Abklingzeit** (linear auf 0) oder ist dauerhaft.

| Modifikator | Wert | Abklingzeit |
|---|---|---|
| Grundsympathie (Persönlichkeiten kompatibel: je gemeinsamer Persönlichkeitseigenschaft +10, je gegensätzlicher −10) | ±10 bis ±30 | dauerhaft (solange Eigenschaften bestehen) |
| Diplomatie-Attribut von B | +1 je Punkt | dauerhaft |
| Gleiche Religion / verwandte Religion / verfeindete Religion | +15 / +5 / −25 | dauerhaft |
| Gleiche Kultur | +10 | dauerhaft |
| Familienbund | +25 | solange die Ehe besteht |
| Blutsverwandtschaft (Eltern/Geschwister/Kinder) | +30 | dauerhaft |
| Geschenk | variabel (max. +40) | 5 Jahre |
| Gesandter | +10 | 10 Jahre |
| Gemeinsamer Feind (beide im Krieg mit demselben Reich) | +20 | solange der Krieg läuft |
| Grenzspannung (gemeinsame Grenze, B stärker) | −10 | dauerhaft, solange zutreffend |
| Ansprüche von B auf Titel von A | −20 je Anspruch (max. −40) | dauerhaft |
| Krieg gegen A | −40 | 20 Jahre nach Frieden |
| Gebrochener Vertrag | −50 | 50 Jahre |
| Bündnisaufruf abgelehnt | −25 | 10 Jahre |
| Gefangener befreit/zurückgegeben | +20 | 15 Jahre |
| Familienmitglied hingerichtet | −80 | dauerhaft |
| Verwandten ermordet (aufgedeckt) | −100 | dauerhaft |
| Hilfe gegen die Hohlen geleistet | +15 je Schlacht (max. +45) | 30 Jahre |
| B nutzt Leermagie oder Hohlung ≥ 40 | −40 (−60 bei Frommen) | dauerhaft solange zutreffend |
| Bedrohung durch B (siehe 7.5) | `−Bedrohung × 0,5` | dauerhaft (folgt dem Wert) |
| Ruf des Friedens | +10 | solange aktiv |
| Prestige-Unterschied (B legendär) | +10 | dauerhaft solange zutreffend |
| Tyrannei von B | −1 je Tyranneipunkt (max. −50) | Tyrannei klingt mit 1/Jahr ab |

Meinung beeinflusst Annahmechancen (direkt als Prozentsummand ×0,5), Bündnisverhalten, Kriegsneigung der KI
und Koalitionsbeitritt.

### 7.4 KI-Ziele und Persönlichkeiten

Jede KI-Herrschaft verfolgt eine Rangliste von **Zielen**, gewichtet nach Persönlichkeit, Lage und Kultur.
Die KI bewertet monatlich Ziele nach Nutzen und Erreichbarkeit und wählt höchstens drei aktive Ziele.

| Ziel | Typische Akteure | Handlungsmuster |
|---|---|---|
| Überleben | kleine Reiche an der Hohlfront | Bündnisse, Garantien suchen, Befestigung |
| Expansion | ehrgeizige Herzöge (z. B. Herzogin Mireille Corvane) | Ansprüche fabrizieren, schwache Nachbarn angreifen |
| Thron von Valemark | Herzogshäuser, Mireille | Fraktionen gründen, Usurpation, Heiratspolitik |
| Reichtum | Saelith-Pakt (Ophira Saelith-Varo) | Handelsabkommen, Kredite vergeben, Monopole bilden |
| Glaubensreinheit | Erzflammenwahrer in Kaldris, eiferige Herrscher | Heilige Kriege gegen Leere Liturgie und Sonnenhäresie, Bekehrung |
| Unabhängigkeit | Skarn-Clans (Torvald Eisenhand), Dornenbund | Bündnisse gegen Großmächte, Verteidigung |
| Rache | Herrscher mit Erinnerung „Familienmitglied getötet“ | Komplotte und Krieg gegen den Verursacher |
| Schleier schließen | Dornenbund (Sylaen der Veyl), fromme Herrscher | Feldzüge gegen Hohlland, Reinigungsrituale |
| Gleichgewicht | Mittelmächte | Koalitionen gegen den Stärksten (7.5) |

**KI-Persönlichkeiten** (aus Eigenschaften abgeleitet): *Eroberer* (Ehrgeizig + Tapfer), *Händler*
(Gierig/Geduldig), *Eiferer* (Fromm), *Ränkeschmied* (Verschlagen/Misstrauisch), *Bewahrer* (Zufrieden/Gerecht).
Die Persönlichkeit verändert Gewichtungen: Ein Eroberer bewertet Expansion ×1,5, ein Bewahrer ×0,5.

**Kriegsbereitschaft der KI**:

```
Kriegswille = Zielnutzen × (Stärke_eigen / Stärke_Ziel_mit_Verbündeten)
            − Meinung × 0,3 − Kriegsmüdigkeit × 0,5 − 20 bei laufendem Hohlenangriff
Krieg, wenn Kriegswille > 40 (Eroberer > 30, Bewahrer > 60) und gültiger Kriegsgrund
```

Die KI lügt nicht über Verträge (außer Persönlichkeit Ränkeschmied) und bricht Verträge nur, wenn
`Kriegswille > 80`.

### 7.5 Bedrohung und Koalitionen

Jedes Reich hat einen **Bedrohungswert** (0–100) in den Augen jedes anderen Reiches.

```
Bedrohung_A_aus_Sicht_B = Grundbedrohung_A × Nähefaktor
Grundbedrohung_A        = Σ eroberte Provinzen der letzten 30 Jahre × Gewicht (5 je Provinz, 10 bei Provinzen gleicher Religion)
                        + (Stärke_A / Durchschnittsstärke_aller_Reiche − 1) × 20   (nur wenn > 0)
                        + Vertragsbrüche × 10
                        + Leermagie-Einsatz × 15
Nähefaktor              = 1,0 (Nachbar) / 0,6 (2 Provinzen entfernt) / 0,3 (weiter)
Abbau: −1 pro Jahr ohne Eroberung; −15 % bei Ruf des Friedens
```

**Koalitionsmechanik**:

- Ab **Bedrohung 50** kann ein Reich einer **Koalition** gegen A beitreten, ab **75** tut es dies, sofern
  `Meinung zu A < 0` oder `Bedrohung > 90`.
- Koalitionen sind defensiv: Sie greifen erst an, wenn A ein Koalitionsmitglied oder dessen Verbündeten angreift
  oder wenn die gesamte Koalitionsstärke ≥ 1,5 × Stärke A ist (dann Präventivkrieg mit Kriegsziel
  „Eroberungen rückgängig machen“).
- Eine Koalition löst sich auf, wenn die Bedrohung aller Mitglieder unter 40 fällt.
- **Großmacht-Reaktion** im Endgame siehe Kapitel 16.1.

### 7.6 Krieg und Frieden

**Kriegsgründe**: Anspruch, Heiliger Krieg, Unabhängigkeit, Grenzkonflikt (Grenzprovinz mit eigener
Kultur), Vergeltung (Erinnerung Familienmitglied getötet/Vertragsbruch), Befreiung von Hohlland
(kein Kriegsgrund nötig, gilt gegen Hohle), Koalitionskrieg. Krieg ohne Grund: 300 Prestige und +30 Tyrannei.

**Kriegspunkte (−100 bis +100)**: Schlachtsiege (`Verlustdifferenz / Gesamtstärke × 50`, max. 20 je Schlacht),
besetzte Provinzen (Wert der Provinz, Hauptstadt ×3), gefangene Herrscher (+50), Kriegsziel gehalten
(+1/Monat).

**Kriegsmüdigkeit** (0–100): +0,5 je Monat Krieg, +1 je 1 % verlorene Truppen, +2 je besetzte eigene
Provinz; senkt Zufriedenheit um `Kriegsmüdigkeit/5`.

**Frieden**: Bedingungen kosten Kriegspunkte (Provinz: Provinzwert; Tribut 10 Jahre: 20; Geisel: 15; Anspruch
durchsetzen: Titelwert). KI nimmt an, wenn `Kriegspunkte des Gegners ≥ Kosten` oder
`Kriegsmüdigkeit > 70 und Kosten ≤ Kriegspunkte + 20`.

---

## 8. Wirtschaft

### 8.1 Grundprinzipien

1. **Physische Güter**: Jede Ware existiert als Menge in einem Lager und wird von Trägern bewegt. Es gibt
   keinen globalen Topf, außer Gold (das in der Schatzkammer liegt, aber von Steuereintreibern sichtbar
   transportiert wird).
2. **Arbeit ist Bevölkerung**: Gebäude produzieren nur mit Arbeitern aus der passenden Schicht (Kapitel 9).
3. **Verarbeitung schafft Wert**: Jede Verarbeitungsstufe erhöht den Marktwert über die Summe der Inputs
   (Wertschöpfung 20–60 %).
4. **Vertical-Slice-Vereinfachung**: In Mission 1 und im Skirmish-Modus „Klassisch“ werden nur `gold`, `wood`,
   `stone` und `food` verwendet. Das Gehöft (`farmstead`) liefert dort direkt `food`, Arbeiter (`worker`)
   sammeln Holz und Stein von Ressourcenfeldern, Gold entsteht aus Steuern der Bewohner und aus Goldadern.
   Im vollen Spiel wird `food` zum **Nahrungskonto**, in das alle Nahrungsmittel mit ihrem Nährwert einfließen.

### 8.2 Ressourcenstufen

| Stufe | Bezeichnung | Güter (ID) | Herkunft |
|---|---|---|---|
| 0 | Grundwerte | `gold`, `population`, `population_cap` | Steuern, Handel, Münzprägung; Wohngebäude |
| 1 | Rohstoffe | `wood` Holz, `stone` Stein, `grain` Getreide, `fish` Fisch, `game` Wild, `livestock` Vieh, `wool` Wolle, `flax` Flachs, `hops` Hopfen, `grapes` Trauben, `herbs` Kräuter, `wax` Wachs, `salt` Salz, `clay` Ton, `iron_ore` Eisenerz, `coal` Kohle, `silver_ore` Silbererz, `sulfur` Schwefel, `ashglass_raw` Rohaschglas | Gewinnungsgebäude, Arbeiter, Ressourcenfelder |
| 2 | Zwischengüter | `planks` Bretter, `cut_stone` Werkstein, `bricks` Ziegel, `flour` Mehl, `malt` Malz, `meat` Fleisch, `hides` Häute, `leather` Leder, `yarn` Garn, `linen` Leinen, `charcoal` Holzkohle, `iron` Eisenbarren, `silver` Silberbarren, `parchment` Pergament, `ashglass` geschliffenes Aschglas | Verarbeitungsbetriebe |
| 3 | Fertiggüter | `bread` Brot, `preserved_food` Pökelware, `beer` Bier, `wine` Wein, `cloth` Tuch, `clothing` Kleidung, `tools` Werkzeug, `weapons` Waffen, `armor` Rüstung, `candles` Kerzen, `books` Bücher, `remedies` Heilmittel, `furniture` Möbel | Werkstätten |
| 4 | Luxus- und Strategiegüter | `fine_garments` Feingewänder, `jewelry` Schmuck, `spices` Gewürze*, `silk` Seide*, `relics` Reliquien, `ashglass_focus` Aschglas-Fokus, `warhorse` Schlachtross, `siege_parts` Belagerungsteile, `alchemical_fire` Alchemistenfeuer | Meisterwerkstätten, Import |

\* Gewürze und Seide werden in Eldraeth nur auf den Saelith-Inseln erzeugt (Gewürzplantage, Seidenhaus) und
müssen von allen anderen Fraktionen importiert werden.

**Nährwerte** (Beitrag einer Einheit zum Nahrungskonto `food`):

| Gut | Nährwert | Haltbarkeit (Verlust pro Monat im Lager) | Besonderheit |
|---|---|---|---|
| `grain` (als Brei) | 1 | 0,5 % | −5 Zufriedenheit, wenn es mehr als 50 % der Nahrung ausmacht |
| `fish` | 1 | 10 % | – |
| `game` | 1 | 10 % | – |
| `meat` | 2 | 8 % | – |
| `bread` | 2 | 5 % | Grundbedürfnis „Brot“ |
| `preserved_food` | 1,5 | 0,5 % | Ideal für Belagerungen und Armeen |
| `beer` | 0,5 | 2 % | Zählt zusätzlich als Komfortbedürfnis |

**Nahrungsvielfalt**: Für jede Nahrungsart (Brot, Fleisch, Fisch, Pökelware), die mindestens 15 % des
Verbrauchs einer Stadt deckt, erhalten alle Schichten +2 Zufriedenheit (max. +8).

### 8.3 Produktionsketten

Zeiten in Sekunden RTS-Simulationszeit (1 Welttag = 8 s). „Arbeiter“ nennt die Zahl der Arbeitsplätze und die
benötigte Schicht. Produktion skaliert linear mit der Besetzung: Ein Gebäude mit 2 von 4 Arbeitern produziert
mit 50 % Geschwindigkeit. Größere Ausbaustufen (II, III) erhöhen Arbeitsplätze und Output proportional
(Stufe II: ×1,8 Output bei ×1,5 Arbeitern; Stufe III: ×2,6 bei ×2).

#### 8.3.1 Gewinnung (Stufe 1)

| Gebäude | ID | Input | Output | Zeit | Arbeiter | Standort/Besonderheit |
|---|---|---|---|---|---|---|
| Holzfällerhütte | `woodcutter` | – | 4 `wood` | 30 s | 2 Freibauern | Wald im Radius 8; Wald wächst nach (1 Baum/90 s mit Förster) |
| Försterei | `forester` | – | pflanzt Bäume | 45 s/Baum | 1 Freibauer | Verhindert Kahlschlag; ohne Förster sinkt der Waldbestand |
| Steinbruch | `quarry` | – | 3 `stone` | 40 s | 3 Leibeigene | Felsvorkommen, endlich (4.000 je Feld) |
| Gehöft | `farmstead` | – | 4 `grain` | 40 s | 4 Leibeigene oder Freibauern | 12 Felder Ackerland; Ertrag nur Frühjahr bis Herbst, Winter 0; +5 `population_cap` |
| Fischerhütte | `fishery` | – | 3 `fish` | 30 s | 2 Freibauern | Ufer; Fischgründe erholen sich (max. 6 Hütten je Gewässer) |
| Jagdhütte | `hunting_lodge` | – | 2 `game` | 35 s | 1 Freibauer | Wildbestand im Radius 12 |
| Viehweide | `pasture` | 2 `grain` | 2 `livestock` | 60 s | 2 Freibauern | 9 Felder Weide |
| Schäferei | `sheep_farm` | – | 2 `wool` | 50 s | 2 Freibauern | 9 Felder Weide, Hügel +20 % |
| Flachsfeld | `flax_farm` | – | 3 `flax` | 50 s | 2 Leibeigene | Fruchtbarer Boden |
| Hopfengarten | `hop_garden` | – | 2 `hops` | 45 s | 2 Freibauern | Gemäßigtes Klima |
| Weinberg | `vineyard` | – | 3 `grapes` | 60 s | 3 Freibauern | Südhang (Valemark, Saelith) |
| Kräutergarten | `herb_garden` | – | 2 `herbs` | 40 s | 1 Freibauer | Dornwald +30 % |
| Imkerei | `apiary` | – | 2 `wax` | 60 s | 1 Freibauer | Blühende Felder in der Nähe +20 % |
| Saline | `saltworks` | – | 2 `salt` | 45 s | 2 Leibeigene | Küste oder Salzquelle |
| Tongrube | `clay_pit` | – | 3 `clay` | 30 s | 2 Leibeigene | Flussufer |
| Eisenmine | `iron_mine` | – | 2 `iron_ore` | 40 s | 4 Leibeigene | Erzader, endlich (4.000 je Ader) |
| Kohlenmine | `coal_mine` | – | 3 `coal` | 40 s | 4 Leibeigene | Nur Gebirge (Skarnheim, Glutweite) |
| Silbermine | `silver_mine` | – | 1 `silver_ore` | 60 s | 4 Leibeigene | Seltene Adern |
| Schwefelgrube | `sulfur_pit` | – | 2 `sulfur` | 50 s | 3 Leibeigene | Nur Glutweite und vulkanische Felder; −5 Zufriedenheit der Arbeiter |
| Aschglasschacht | `ashglass_mine` | – | 1 `ashglass_raw` | 90 s | 3 Leibeigene | Nur wo Schleierstärke ≥ 2; jeder Arbeiter +0,1 Hohlung/Monat; Provinz-Hohlung +0,05/Monat |

#### 8.3.2 Nahrungsketten

**Brot**

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Gehöft | `farmstead` | – | 4 `grain` | 40 s | 4 Leibeigene/Freibauern |
| 2 | Windmühle | `windmill` | 2 `grain` | 2 `flour` | 20 s | 1 Freibauer |
| 3 | Bäckerei | `bakery` | 2 `flour` | 4 `bread` | 30 s | 2 Handwerker |

Verhältnis: 2 Gehöfte versorgen 1 Windmühle und 1,3 Bäckereien. Ein Gehöft-Zyklus (4 Getreide = 4 Nährwert
als Brei) wird über die Kette zu 8 Brot = 16 Nährwert: Verarbeitung vervierfacht die Nahrung je Ackerfläche.

**Fisch und Pökelware**

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Fischerhütte | `fishery` | – | 3 `fish` | 30 s | 2 Freibauern |
| 2 | Saline | `saltworks` | – | 2 `salt` | 45 s | 2 Leibeigene |
| 3 | Räucherei | `smokehouse` | 2 `fish` + 1 `salt` | 3 `preserved_food` | 40 s | 2 Handwerker |

**Fleisch, Häute und Leder**

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Viehweide | `pasture` | 2 `grain` | 2 `livestock` | 60 s | 2 Freibauern |
| 1b | Jagdhütte | `hunting_lodge` | – | 2 `game` | 35 s | 1 Freibauer |
| 2 | Metzgerei | `butcher` | 1 `livestock` | 3 `meat` + 1 `hides` | 30 s | 2 Handwerker |
| 2b | Metzgerei (Wild) | `butcher` | 2 `game` | 2 `meat` + 1 `hides` | 30 s | 2 Handwerker |
| 3 | Gerberei | `tannery` | 2 `hides` + 1 `salt` | 2 `leather` | 40 s | 2 Handwerker |

**Bier und Wein**

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Hopfengarten | `hop_garden` | – | 2 `hops` | 45 s | 2 Freibauern |
| 2 | Mälzerei | `malthouse` | 2 `grain` | 2 `malt` | 30 s | 1 Handwerker |
| 3 | Brauerei | `brewery` | 2 `malt` + 1 `hops` | 4 `beer` | 60 s | 2 Handwerker |
| 1 | Weinberg | `vineyard` | – | 3 `grapes` | 60 s | 3 Freibauern |
| 2 | Kelterei | `winery` | 3 `grapes` | 2 `wine` | 90 s | 2 Handwerker |

#### 8.3.3 Bau- und Brennstoffketten

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Holzfällerhütte | `woodcutter` | – | 4 `wood` | 30 s | 2 Freibauern |
| 2 | Sägewerk | `sawmill` | 2 `wood` | 3 `planks` | 30 s | 2 Handwerker |
| 1 | Steinbruch | `quarry` | – | 3 `stone` | 40 s | 3 Leibeigene |
| 2 | Steinmetz | `stonemason` | 2 `stone` | 2 `cut_stone` | 40 s | 2 Handwerker |
| 1 | Tongrube | `clay_pit` | – | 3 `clay` | 30 s | 2 Leibeigene |
| 2 | Köhlerei | `charcoal_burner` | 3 `wood` | 2 `charcoal` | 60 s | 1 Freibauer |
| 3 | Ziegelei | `brickworks` | 2 `clay` + 1 `charcoal` | 4 `bricks` | 45 s | 2 Handwerker |
| 3b | Schreinerei | `carpenter` | 3 `planks` + 1 `tools` | 2 `furniture` | 60 s | 2 Handwerker |

Bauwesen: Einfache Gebäude (Stufe I) verwenden `wood`/`stone`; Stufe II benötigt `planks`/`cut_stone`;
Stufe III und Steinmauern benötigen `bricks`/`cut_stone` und `tools`.

#### 8.3.4 Metallketten

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Eisenmine | `iron_mine` | – | 2 `iron_ore` | 40 s | 4 Leibeigene |
| 1b | Köhlerei / Kohlenmine | `charcoal_burner` / `coal_mine` | 3 `wood` / – | 2 `charcoal` / 3 `coal` | 60 s / 40 s | 1 Freibauer / 4 Leibeigene |
| 2 | Schmelzhütte | `smelter` | 2 `iron_ore` + 2 `charcoal` (oder 1 `coal`) | 2 `iron` | 50 s | 2 Handwerker |
| 3 | Werkzeugschmiede | `toolsmith` | 1 `iron` + 1 `planks` | 2 `tools` | 45 s | 2 Handwerker |
| 3 | Waffenschmiede | `weaponsmith` | 2 `iron` + 1 `planks` | 2 `weapons` | 60 s | 2 Handwerker |
| 3 | Rüstungsschmiede | `armorer` | 3 `iron` + 1 `leather` | 1 `armor` | 75 s | 3 Handwerker |
| 1 | Silbermine | `silver_mine` | – | 1 `silver_ore` | 60 s | 4 Leibeigene |
| 2 | Silberhütte | `silver_smelter` | 2 `silver_ore` + 1 `charcoal` | 1 `silver` | 60 s | 2 Handwerker |
| 3 | Münzstätte | `mint` | 1 `silver` | 120 `gold` | 90 s | 2 Bürger |
| 4 | Juwelier | `jeweler` | 1 `silver` + 1 `ashglass` | 1 `jewelry` | 120 s | 1 Handwerker + 1 Bürger |

Münzstätte: Erfordert das Recht „Münzhoheit“ (Königstitel oder Privileg). Jede geprägte Charge erhöht die
regionale **Inflation** um 0,02 Prozentpunkte (Inflation erhöht alle Preise; sinkt um 0,5 Prozentpunkte pro Jahr).

#### 8.3.5 Textilketten

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Schäferei | `sheep_farm` | – | 2 `wool` | 50 s | 2 Freibauern |
| 2 | Spinnerei | `spinnery` | 2 `wool` | 2 `yarn` | 30 s | 2 Handwerker |
| 3 | Weberei | `weaver` | 2 `yarn` | 1 `cloth` | 40 s | 2 Handwerker |
| 1 | Flachsfeld | `flax_farm` | – | 3 `flax` | 50 s | 2 Leibeigene |
| 2 | Leinenweberei | `linen_weaver` | 3 `flax` | 2 `linen` | 45 s | 2 Handwerker |
| 4 | Schneiderei | `tailor` | 1 `cloth` + 1 `leather` | 2 `clothing` | 50 s | 2 Handwerker |
| 5 | Gewandmeisterei | `couturier` | 1 `linen` + 1 `silk` | 1 `fine_garments` | 90 s | 2 Handwerker (Meister) |

#### 8.3.6 Wissen, Licht und Heilung

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Imkerei | `apiary` | – | 2 `wax` | 60 s | 1 Freibauer |
| 2 | Kerzenzieherei | `chandler` | 2 `wax` | 4 `candles` | 40 s | 1 Handwerker |
| 2 | Pergamenterei | `parchment_maker` | 2 `hides` | 3 `parchment` | 50 s | 1 Handwerker |
| 3 | Skriptorium | `scriptorium` | 2 `parchment` + 1 `candles` | 1 `books` | 120 s | 2 Gelehrte oder Klerus |
| 1 | Kräutergarten | `herb_garden` | – | 2 `herbs` | 40 s | 1 Freibauer |
| 2 | Apotheke | `apothecary` | 2 `herbs` + 1 `beer` | 2 `remedies` | 60 s | 1 Gelehrter + 1 Handwerker |

Bücher sind Forschungsressource: Jede Bibliothek verbraucht 1 `books` pro Monat und liefert dafür +10
Forschungspunkte. Heilmittel senken die Seuchensterblichkeit (Kapitel 15) und beschleunigen die Heilung von
Verwundeten um +50 %.

#### 8.3.7 Aschglas und Alchemie

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Aschglasschacht | `ashglass_mine` | – | 1 `ashglass_raw` | 90 s | 3 Leibeigene |
| 2 | Aschglasschleiferei | `ashglass_cutter` | 2 `ashglass_raw` | 1 `ashglass` | 120 s | 1 Gelehrter + 1 Handwerker |
| 3 | Arkanwerkstatt | `arcane_workshop` | 2 `ashglass` + 1 `silver` | 1 `ashglass_focus` | 180 s | 2 Gelehrte |
| 3 | Alchemistenlabor | `alchemist` | 2 `sulfur` + 1 `ashglass_raw` + 1 `charcoal` | 2 `alchemical_fire` | 120 s | 2 Gelehrte |

Aschglas ist Treibstoff der Magie (Kapitel 12.4). Rohaschglas, das länger als 6 Monate ungeschliffen lagert,
erhöht die Hohlung der Provinz um 0,02 je 10 Einheiten und Monat.

#### 8.3.8 Militärische Güter

| Schritt | Gebäude | ID | Input | Output | Zeit | Arbeiter |
|---|---|---|---|---|---|---|
| 1 | Gestüt | `stud_farm` | 4 `grain` + 1 `livestock` | 1 `warhorse` | 240 s | 2 Freibauern + 1 Krieger |
| 2 | Belagerungswerkstatt | `siege_workshop` | 4 `planks` + 2 `iron` + 1 `tools` | 1 `siege_parts` | 120 s | 3 Handwerker |

**Einheitenkosten im vollen Spiel**: Neben `gold` und `food` benötigen Einheiten physische Ausrüstung:
`swordsman` 1 `weapons` + 1 `armor`/2 (halbe Rüstung = Kettenhemd-Zuschnitt), `spearman` 1 `weapons`, `archer`
1 `weapons` (Bogen zählt als Waffe), `knight` 1 `weapons` + 1 `armor` + 1 `warhorse`, Belagerungsgerät
2–6 `siege_parts`. In Mission 1 und im klassischen Skirmish sind diese Kosten in Gold/Holz eingepreist.

### 8.4 Lager

| Lager | ID | Kapazität | Besonderheit |
|---|---|---|---|
| Bergfried | `keep` | 500 je Rohstoff Stufe 1, 200 je anderes Gut | Abgabepunkt für alle Waren; Schatzkammer |
| Lagerhaus | `storehouse` | 400 je Gut, 8 Gütertypen frei wählbar | Abgabepunkt; Träger: 4 |
| Kornspeicher | `granary` | 1.500 Nahrungsgüter | Verderb −50 % |
| Eiskeller | `ice_cellar` | 600 Fisch/Fleisch | Verderb −80 %; nur in Skarnheim oder mit Forschung „Kellerbau“ |
| Zeughaus | `armory_store` | 300 je Militärgut | Garnison rüstet sich hier aus; Belagerungsbonus +5 % |
| Handelskontor | `trade_office` | 800 je Handelsgut | Importe und Exporte laufen hierüber |
| Aschglasgewölbe | `ashglass_vault` | 200 `ashglass_raw`/`ashglass` | Verhindert Hohlungsabstrahlung gelagerten Rohaschglases |

Überschreitet ein Lager seine Kapazität, stoppen zuliefernde Gebäude (Warnsymbol „Lager voll“).

### 8.5 Transport

**In der RTS-Stadt**: Jedes Produktionsgebäude hat **Träger** (Teil der Belegschaft, nicht zusätzliche
Bewohner), die Waren zum nächsten passenden Lager oder direkt zum nächsten Verbraucher tragen.

| Transportmittel | Kapazität | Geschwindigkeit (Felder/s) | Voraussetzung |
|---|---|---|---|
| Träger zu Fuß | 5 Einheiten | 2,0 | – |
| Handkarren | 10 | 1,8 | Forschung „Radmacher“ |
| Ochsenkarren | 25 | 1,4 | Lagerhaus mit Stall |
| Flusskahn | 60 | 2,5 flussabwärts, 1,0 flussaufwärts | Anlegestelle |

| Untergrund | Geschwindigkeitsfaktor |
|---|---|
| Gelände ohne Weg | 0,7 |
| Trampelpfad | 1,0 |
| Schotterstraße (2 `stone` je Feld) | 1,3 |
| Pflasterstraße (1 `cut_stone` je Feld) | 1,6 |
| Hohlland, Sumpf | 0,5 |

**Auf der Weltkarte**: Waren werden über **Handelsrouten** zwischen Städten bewegt. Eine Route hat eine
Kapazität in Einheiten pro Monat:

```
Kapazität = Basis (Landweg 200 / Fluss 500 / See 1.000)
          × Straßenstufe der Provinzen (Durchschnitt 1,0–1,6)
          × (1 − Banditen- und Monsterrisiko)
Reisezeit = Σ Provinzdurchquerung (Land 6 Tage, Fluss 3, See 2 je Seezone)
```

Karawanen und Schiffe sind auf der Weltkarte sichtbar und können überfallen werden (von Banditen, Monstern,
Kaperern oder feindlichen Armeen). Ein Überfall vernichtet die Ladung oder raubt sie und senkt die
Routensicherheit für 6 Monate um 20 %.

### 8.6 Markt und Preise

Eldraeth ist in **Marktgebiete** unterteilt (je Region eines: Valemark, Aschmark, Skarnheim, Dornwald,
Saelith-Inseln, Aurethwüste, Glutweite; die Schleiernarbe hat keinen Markt). Jedes Marktgebiet hat für jedes
Gut einen Preis, der monatlich neu berechnet wird:

```
P = P_basis × clamp( (N / A) ^ 0,7 ; 0,25 ; 4,0 ) × (1 + Inflation) × Ereignisfaktor

N = Monatsnachfrage = Verbrauch aller Bewohner + Bedarf der Werkstätten + Kaufaufträge (Spieler, KI, Armeen)
A = Monatsangebot   = Monatsproduktion + Importe + 0,2 × freie Lagerbestände (zum Verkauf freigegeben)
Ereignisfaktor      = z. B. 1,5 für Nahrung bei Hungersnot im Marktgebiet, 1,3 für Waffen im Krieg
```

Preisänderung pro Monat ist auf ±25 % begrenzt (Trägheit). Zwischen Marktgebieten gleicht Handel Preise
aus: Eine Handelsroute verschiebt Waren vom billigen zum teuren Markt, solange
`P_Ziel − P_Quelle > Transportkosten + Zoll`. Transportkosten = 2 % des Warenwerts je 10 Tage Reisezeit.

**Basispreise** (Gold je Einheit)

| Gut | Preis | Gut | Preis | Gut | Preis |
|---|---|---|---|---|---|
| `wood` | 2 | `planks` | 4 | `furniture` | 30 |
| `stone` | 3 | `cut_stone` | 6 | `tools` | 14 |
| `grain` | 2 | `flour` | 3 | `bread` | 3 |
| `fish` | 2 | `preserved_food` | 5 | `meat` | 5 |
| `livestock` | 8 | `hides` | 4 | `leather` | 9 |
| `wool` | 3 | `yarn` | 5 | `cloth` | 12 |
| `flax` | 2 | `linen` | 6 | `clothing` | 20 |
| `hops` | 3 | `malt` | 4 | `beer` | 4 |
| `grapes` | 3 | `wine` | 12 | `candles` | 4 |
| `wax` | 4 | `parchment` | 5 | `books` | 45 |
| `herbs` | 3 | `remedies` | 10 | `salt` | 4 |
| `clay` | 1 | `bricks` | 3 | `charcoal` | 3 |
| `coal` | 3 | `iron_ore` | 4 | `iron` | 12 |
| `weapons` | 30 | `armor` | 55 | `silver_ore` | 20 |
| `silver` | 60 | `jewelry` | 220 | `sulfur` | 6 |
| `ashglass_raw` | 40 | `ashglass` | 110 | `ashglass_focus` | 400 |
| `alchemical_fire` | 70 | `warhorse` | 120 | `siege_parts` | 60 |
| `spices` | 35 | `silk` | 50 | `fine_garments` | 160 |
| `relics` | 300 | | | | |

**Spielerhandel**: Am Marktplatz (`marketplace`) kann der Spieler Kauf- und Verkaufsaufträge mit Preisgrenzen
setzen (z. B. „verkaufe Bier, solange Preis ≥ 5, Lagerbestand bleibt ≥ 100“). Der Markt hat Tiefe: Jede
gehandelte Einheit verschiebt den Preis um `1 / (A + N) × 50 %` – Großverkäufe drücken den Preis.

### 8.7 Steuern, Zölle und Kredite

**Steuerarten**

| Steuer | Bemessung | Regelsatz | Stufen |
|---|---|---|---|
| Kopfsteuer | Pro Bewohner und Monat, je Schicht (siehe 9.2) | Mittel | Keine / Niedrig (×0,5) / Mittel (×1) / Hoch (×1,5) / Erdrückend (×2) |
| Grundsteuer | Pro Gebäude-Grundfläche bei Bürgern und Adel | 1 Gold/Feld/Jahr | wie oben |
| Handelssteuer | Anteil am Marktumsatz der Stadt | 5 % | 0–20 % in 5er-Schritten |
| Zehnt | 10 % der Kopfsteuer geht an die Kirche | – | Nur bei Staatskirche; kann verweigert werden (−30 Meinung Klerus, −20 Frömmigkeit/Jahr) |

**Zufriedenheitswirkung der Steuern** je Schicht: Niedrig +5, Mittel 0, Hoch −8, Erdrückend −20; Handelssteuer
über 10 %: −1 Zufriedenheit bei Bürgern je Prozentpunkt.

**Zölle**: Jede Provinz mit Handelskontor oder Grenzposten erhebt Zölle auf Importe (0–30 %) und Transit
(0–15 %). Zölle senken den Handelsverkehr: `Routenvolumen × (1 − Zoll × 1,5)`. Handelsabkommen halbieren
Zölle. Hohe Zölle gegen ein Reich: −1 Meinung je 5 % Zoll.

**Korruption**: `Korruption = 5 % + 1 % je Provinz über Verwaltungslimit − Verwaltung_Schatzmeister × 0,5 %`
(min. 0 %, max. 40 %). Korruption ist ein Prozentsatz des Steueraufkommens, der verloren geht.

**Kredite**

| Kreditgeber | Verfügbarkeit | Zins pro Jahr | Laufzeit | Besonderheit |
|---|---|---|---|---|
| Saelith-Handelsbanken | Immer, wenn Meinung des Saelith-Pakts ≥ −20 | `4 % + 5 % × (Schulden / Jahreseinkommen)` | 5 / 10 / 20 Jahre | Bei Zahlungsverzug: Pfand auf Handelskontor |
| Kirche der Ewigen Flamme | Nur Staatskirche, Frömmigkeit ≥ 300 | 2 % fest | 10 Jahre | Bei Verzug: −50 Meinung Klerus, Exkommunikationsgefahr |
| Adelshäuser | Haus mit Macht ≥ 30 | 6 % | 5 Jahre | Haus erhält Recht „Gläubiger“ (+10 Loyalität), bei Verzug Fraktionsbildung |
| Kriegsanleihe | Nur im Verteidigungskrieg | 3 % | 15 Jahre | Bürger zeichnen; +5 Zufriedenheit Bürger bei Rückzahlung |

**Kreditgrenze** = 3 × Jahreseinkommen. **Bankrott** tritt ein, wenn eine Rate nicht bezahlt werden kann und
keine Verlängerung gewährt wird: Alle Kredite werden gestrichen, Kreditwürdigkeit 20 Jahre bei 0, −30 Meinung
aller Gläubiger, −25 Zufriedenheit Bürger, Armeen erhalten 6 Monate keinen Sold (Desertion 5 %/Monat),
−300 Prestige.

**Ausgaben**: Armeeunterhalt (siehe Einheitentabelle, Gold pro Monat), Gebäudeunterhalt (1–10 Gold/Monat),
Hofhaltung (5 Gold je Höfling), Amtsgehälter (je Amt 10 + Attribut Gold/Monat), Zinsen, Tribute.

---

## 9. Bevölkerung

### 9.1 Grundprinzip

Bewohner sind keine steuerbaren Einheiten (außer `worker` und Militär), sondern leben in Wohngebäuden,
arbeiten in Betrieben und konsumieren Güter. In der RTS-Stadt sind sie als Passanten, Träger und Arbeiter
sichtbar (repräsentativ: ein sichtbarer Bewohner je 5 tatsächlichen). Jede Schicht hat eigene
Wohngebäude, Arbeitsplätze, Bedürfnisse und Steuersätze.

### 9.2 Die acht Schichten

| # | Schicht | ID | Wohngebäude (Bewohner) | Arbeit | Kopfsteuer (Mittel, Gold/Monat) | Aufstieg zu |
|---|---|---|---|---|---|---|
| 1 | Leibeigene | `serfs` | Kate (8) | Felder, Minen, Steinbruch | 0,1 | Freibauern |
| 2 | Freibauern | `freeholders` | Bauernhaus (10) | Land- und Forstwirtschaft, einfache Verarbeitung | 0,3 | Handwerker |
| 3 | Handwerker | `artisans` | Werkstatthaus (8) | Werkstätten Stufe 2–3 | 0,6 | Bürger |
| 4 | Bürger | `burghers` | Bürgerhaus (6) | Handel, Verwaltung, Münze, Markt | 1,2 | Adel (nur durch Erhebung) |
| 5 | Klerus | `clergy` | Pfarrhaus/Kloster (6) | Tempel, Skriptorium, Hospital | 0 (Zehnt) | – (Aufnahme aus jeder Schicht) |
| 6 | Gelehrte | `scholars` | Gelehrtenturm (4) | Bibliothek, Akademie, Magierturm, Aschglasschleiferei | 0,8 | – (Aufnahme aus Bürgern, Handwerkern, Klerus) |
| 7 | Krieger | `warriors` | Wehrhof (8) | Garnison, Wachdienst, Ausbildung, Gestüt | 0 (erhalten Sold) | Adel (durch Ritterschlag) |
| 8 | Adel | `nobility` | Herrenhaus (2) | Ämter, Kommando, Verwaltung | 3,0 | – |

Militäreinheiten werden aus den Schichten rekrutiert: `worker` aus Leibeigenen oder Freibauern, `spearman`
und `archer` aus Freibauern (Miliz) oder Kriegern (Berufssoldaten), `swordsman` aus Kriegern, `knight` aus
Adel und Kriegern, Magier aus Gelehrten, Priester aus Klerus. Rekrutierung entfernt die Bewohner aus ihrer
Schicht; Gefallene fehlen dauerhaft (demografische Folgen).

### 9.3 Bedürfnisse

Jede Schicht hat **Grund-**, **Komfort-** und **Luxusbedürfnisse**. Ein Bedürfnis ist erfüllt, wenn das Gut im
Marktbereich verfügbar ist (Verbrauch pro 100 Bewohner und Monat in Klammern) oder ein Dienstgebäude in
Reichweite liegt.

| Schicht | Grundbedürfnisse | Komfortbedürfnisse | Luxusbedürfnisse |
|---|---|---|---|
| Leibeigene | Nahrung (60 `food`), Brennholz (10 `wood`), Sicherheit | Bier (5), Kapelle | Feiertage (Festplatz) |
| Freibauern | Nahrung (60), Brennholz (10), Kleidung (2), Kapelle | Bier (8), Brot (20 als Teil der Nahrung), Werkzeug (1) | Fleisch (10), Markt in Reichweite |
| Handwerker | Nahrung (55), Kleidung (4), Werkzeug (2), Tempel | Bier (10), Fleisch (12), Kerzen (4), Taverne | Möbel (1), Wein (3) |
| Bürger | Nahrung (50), Kleidung (5), Kerzen (5), Tempel, Markt | Wein (6), Möbel (2), Bücher (1), Badehaus | Schmuck (0,5), Feingewänder (1), Gewürze (2) |
| Klerus | Nahrung (45), Kerzen (8), Tempel | Bücher (3), Wein (4) | Reliquien (0,1), Kathedrale |
| Gelehrte | Nahrung (45), Kerzen (8), Bücher (4), Bibliothek | Wein (4), Heilmittel (2), Aschglas (0,5) | Schmuck (0,5), Akademie |
| Krieger | Nahrung (70), Bier (10), Waffenpflege (1 `weapons` je 50), Wehrhof | Fleisch (15), Kleidung (4), Taverne | Schlachtrösser (für Adelsaufstieg), Turnierplatz |
| Adel | Nahrung (40, davon 50 % Fleisch/Brot), Feingewänder (3), Wein (8), Herrenhaus | Schmuck (1), Gewürze (4), Möbel (3) | Seide (2), Reliquien (0,2), Turnierplatz |

**Sicherheit** (alle Schichten): Erfüllt, wenn die Stadt eine Garnison ≥ 1 Krieger je 50 Bewohner oder einen
Wachturm in Reichweite jedes Wohnhauses hat. Im Vertical Slice (`watchtower`) bereits wirksam.

### 9.4 Zufriedenheit

Zufriedenheit wird **pro Schicht und Stadt** im Bereich 0–100 berechnet (monatlich, in der RTS-Ansicht
geglättet täglich):

```
Z_s = 40
    + 30 × G_s                      // G = gewichteter Erfüllungsgrad der Grundbedürfnisse (0–1)
    + 20 × K_s                      // K = Erfüllungsgrad Komfort (0–1)
    + 10 × L_s                      // L = Erfüllungsgrad Luxus (0–1)
    − 25, falls G_s < 0,5           // Mangel
    − 40, falls Nahrungskonto leer  // Hunger (zusätzlich)
    + Steuerwirkung (+5 … −20)
    + Gesetzeswirkung (siehe 6.4)
    + Sicherheit (+5 erfüllt, −15 unerfüllt, −30 bei Feinden in der Stadt)
    + Glaube (+10 gleiche Religion wie Herrscher und Tempel erfüllt, −10 Andersgläubige bei Staatskirche)
    + Herrscherwirkung (Charisma × 0,5 in Hauptstadt, Prestige-Rang +0/+2/+4/+6)
    + Nahrungsvielfalt (+2 je Art, max. +8)
    − Überfüllung (−2 je 10 % über Wohnkapazität)
    − Kriegsmüdigkeit / 5
    − Provinz-Hohlung / 5
    + Ereignismodifikatoren (Feste +10, Seuche −15, Hinrichtung −5 …)
Z_s = clamp(Z_s, 0, 100)
```

| Zufriedenheit | Zustand | Wirkung |
|---|---|---|
| 0–19 | Aufruhr | Produktivität −50 %, Aufstandsprüfung monatlich (9.6), Abwanderung 3 %/Monat |
| 20–39 | Unruhig | Produktivität −20 %, Streikgefahr, Abwanderung 1 %/Monat |
| 40–59 | Gleichgültig | Produktivität 100 % |
| 60–79 | Zufrieden | Produktivität +10 %, Zuwanderung, Aufstieg möglich |
| 80–100 | Begeistert | Produktivität +20 %, Steuern +10 %, Aufstieg beschleunigt, Freiwillige (+10 % Rekrutierungstempo) |

### 9.5 Demografie

Jede Schicht einer Stadt hat drei Altersgruppen: **Kinder** (0–15), **Erwachsene** (16–54, arbeiten),
**Alte** (55+, arbeiten zu 30 %).

```
Geburten pro Jahr  = Erwachsene × 4 % × Nahrungsfaktor (0,5 bei Mangel, 1,0 normal, 1,2 bei Überfluss)
                     × Wohnfaktor (0 wenn keine freie Wohnkapazität, sonst 1)
Sterbefälle/Jahr   = Kinder × 1,5 % + Erwachsene × 1 % + Alte × 8 %
                     × (1 + Seuchenfaktor) × (1,5 bei Hunger) × (0,8 mit Hospital)
Altern             = 1/16 der Kinder werden pro Jahr erwachsen; 1/39 der Erwachsenen werden alt
Migration/Monat    = (Z_Stadt − Z_Nachbarschnitt) × 0,05 % × Bevölkerung, begrenzt durch freie Wohnungen
                     + Flüchtlinge aus Krieg und Hohlland (Ereignis)
```

**Aufstieg**: Pro Monat steigen `0,5 % × (Z − 60)/20` der Erwachsenen einer Schicht in die nächsthöhere auf,
wenn Z ≥ 60, freie Arbeitsplätze und Wohnungen der Zielschicht vorhanden sind und die Bildungsvoraussetzung
erfüllt ist (Handwerker benötigen eine Zunfthalle, Gelehrte eine Schule).
**Abstieg**: Bei Z < 30 steigen `0,5 %` pro Monat ab (Bürger zu Handwerkern usw.; Leibeigene werden zu
Bettlern, die keine Steuern zahlen und Unruhe +1 je 10 erzeugen).

### 9.6 Konflikte zwischen Schichten

Zwischen Schichten bestehen **Spannungsachsen**. Jede Achse hat einen Wert 0–100:

| Achse | Treiber (steigt durch) | Senkung durch |
|---|---|---|
| Leibeigene ↔ Adel | Leibeigenschaft-Gesetz, Hunger, hohe Kopfsteuer, Adel Z > Leibeigene Z + 30 | Gebundene Pacht/Freie Bauern, Feste, Brotverteilung |
| Bürger ↔ Adel | Fehlende Stadtrechte, Handelssteuer > 10 %, Adelsprivilegien | Marktrecht, Bürger im Amt, Kredite zurückzahlen |
| Klerus ↔ Gelehrte | Magiegesetz „Freiheit“, Aschglasverarbeitung, Häresie | Magiegesetz „Lizenz“, gemeinsame Hohlungsforschung |
| Krieger ↔ Bürger | Ausbleibender Sold, Plünderungen, Einquartierung | Sold pünktlich, Wehrhöfe außerhalb der Stadt |
| Einheimische ↔ Flüchtlinge | Flüchtlingsanteil > 10 %, Hohlungsangst, Nahrungsmangel | Ausreichend Wohnraum, Integration (2 % pro Jahr), Tempel |

```
Spannung_Achse (monatlich) += Σ Treiber − Σ Senkungen − 1 (natürlicher Abbau)

Konfliktstufe:
  ≥ 40  Streit:   −5 Zufriedenheit beider Schichten
  ≥ 60  Streik:   unterlegene Schicht arbeitet 30 Tage nicht (Chance 10 %/Monat)
  ≥ 75  Unruhen:  Brandstiftung (Gebäude-Schaden in der RTS-Stadt), Plünderung von Lagern
  ≥ 90  Aufstand: Rebellenarmee aus der unterlegenen Schicht entsteht (Stärke = 10 % der Schicht)
```

**Aufstandsprüfung** einer Stadt (monatlich bei Z < 20 einer Schicht):
`Chance = (20 − Z) × 1 % × (1 + Unruhe/100) − Garnisonsstärke/Bevölkerung × 50 %`. Aufständische erscheinen in der
RTS-Stadt als feindliche Einheiten (bewaffnete Bauern, Deserteure) und auf der Weltkarte als Rebellenarmee.
Aufständische können auch verhandelt werden: Erfüllung einer Forderung (Steuersenkung, Gesetz, Brot) beendet
den Aufstand ohne Kampf.

---

## 10. Militär

### 10.1 Steuerung

| Funktion | Bedienung | Beschreibung |
|---|---|---|
| Auswahl | Linksklick, Rahmen ziehen | Einzel- und Gruppenauswahl; max. 24 Einheiten in der Auswahlleiste, darüber Gruppierung nach Typ |
| Typauswahl | Doppelklick / `Cmd`+Klick | Alle sichtbaren Einheiten desselben Typs |
| Kontrollgruppen | `Cmd`+`1`…`0` zuweisen, `1`…`0` abrufen, zweimal drücken zentriert | 10 Gruppen; Gebäude zuweisbar |
| Bewegen | Rechtsklick | Bewegung in aktueller Formation |
| Angriffsbewegung | `A` + Klick | Bewegung mit Angriff auf alle Feinde unterwegs |
| Wegpunkte | `Shift` + Befehl | Befehlskette |
| Formation drehen | Rechtsklick halten und ziehen | Richtung und Breite der Formation festlegen |
| Patrouille | `P` + Klick | Hin- und Herbewegung mit Angriffsbewegung |
| Halten | `H` | Position halten, nur Ziele in Reichweite angreifen |
| Stopp | `S` | Befehl abbrechen |
| Rückzug | `R` | Ungeordneter Rückzug zum Sammelpunkt; +30 % Tempo, kein Gegenangriff, −5 Moral |
| Fähigkeiten | `Q`, `W`, `E`, `T` (Held) / Rechtsklick auf Fähigkeit = Autocast | siehe Tastenkürzel |

**Formationen**: Linie (Standard, Fernkämpfer hinter Nahkämpfern), Keil (Kavallerie: +15 % Ansturmschaden,
−10 % Rüstung an den Flanken), Kolonne (+15 % Tempo auf Straßen, −20 % Verteidigung), Quadrat/Schildwall
(Infanterie: keine Flankierungsboni für Gegner, −40 % Tempo), Lockere Ordnung (−30 % Flächenschaden,
−10 % Nahkampfschaden).

**Haltungen**: Aggressiv (verfolgt bis 15 Felder), Defensiv (verfolgt bis 6 Felder, kehrt zurück), Position halten
(keine Verfolgung), Passiv (greift nicht an; für Arbeiter Standard).

**Spezialhaltungen**: `spearman` und `halberdier` besitzen **Speerwall** (`F`): stehend, −100 % Tempo,
konternder Ansturm gegen Kavallerie (Reiter erleiden 150 % ihres Ansturmschadens). `archer` besitzt
**Salvenfeuer** (Flächenbeschuss auf einen Punkt, 70 % Schaden, Radius 2).

### 10.2 Rüstungs- und Schadensmodell

**Schadensberechnung** pro Treffer:

```
Schaden = Basisschaden × Matrix(Schadenstyp, Rüstungstyp) × Rüstungsreduktion × Situationsfaktoren
Rüstungsreduktion = 1 / (1 + 0,06 × Rüstungswert)          // Rüstung 5 → 77 % Schaden, 10 → 63 %
Situationsfaktoren = Flankierung × Höhe × Tageszeit × Moralzustand × Erschöpfung × Veteranenstufe
Mindestschaden = 1
```

**Schadens-/Rüstungsmatrix** (Prozent des Basisschadens)

| Schaden ↓ / Rüstung → | Ungerüstet | Leicht | Mittel | Schwer | Beritten | Untot | Ätherisch | Befestigt | Held |
|---|---|---|---|---|---|---|---|---|---|
| **Hieb** | 120 | 110 | 100 | 70 | 100 | 90 | 25 | 30 | 100 |
| **Stich** | 110 | 120 | 80 | 60 | 150 | 50 | 25 | 15 | 90 |
| **Wucht** | 100 | 90 | 110 | 130 | 90 | 150 | 25 | 60 | 100 |
| **Belagerung** | 100 | 80 | 80 | 80 | 60 | 80 | 10 | 250 | 50 |
| **Feuer** | 130 | 120 | 100 | 90 | 110 | 140 | 75 | 120 | 100 |
| **Arkan** | 100 | 100 | 110 | 120 | 100 | 100 | 150 | 60 | 80 |
| **Leere** | 110 | 110 | 110 | 110 | 110 | 50 | 50 | 80 | 120 |

Leitlinien, die aus der Matrix folgen: Pfeile sind schwach gegen Hohle (Untot 50 %), Streitkolben und
Wuchtwaffen zerschmettern Knochen (150 %), Feuer ist das universelle Mittel gegen die Hohlen (140 %),
Ätherische Gegner verlangen Magie, Mauern verlangen Belagerungsgerät.

### 10.3 Einheitenklassen und Werte

Spalten: HP · Rüstung (Typ Wert) · Schaden (Typ Wert) · Angriffsintervall (s) · Reichweite (Felder; 0,8 =
Nahkampf) · Tempo (Felder/s) · Sicht · Kosten (klassische Wertung: Gold/Holz/Stein/Nahrung) · Ausbildungszeit ·
Bevölkerung · Unterhalt (Gold/Monat Weltkarte) · Grundmoral.

#### Glutkrone von Valemark (`ember_crown`)

| ID | Name | Klasse | HP | Rüstung | Schaden | Int. | Rw. | Tempo | Sicht | Kosten G/H/S/N | Zeit | Pop | Unt. | Moral | Besonderheit |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `worker` | Arbeiter (Leibeigener) | Zivil | 60 | Ungerüstet 0 | Wucht 4 | 1,5 | 0,8 | 2,6 | 7 | 0/0/0/50 | 18 s | 1 | 0,2 | 40 | Baut, sammelt (Holz 10/Gang, Stein 8/Gang), repariert |
| `swordsman` | Schwertkämpfer | Schwere Infanterie | 180 | Mittel 3 | Hieb 14 | 1,2 | 0,8 | 2,4 | 8 | 35/0/0/50 | 22 s | 1 | 1,0 | 60 | Schild: −25 % Stichschaden von vorn |
| `spearman` | Speerträger | Anti-Kavallerie | 160 | Mittel 2 | Stich 11 | 1,3 | 1,5 | 2,4 | 8 | 15/20/0/40 | 20 s | 1 | 0,8 | 55 | Speerwall; +100 % Schaden gegen Beritten (zusätzlich zur Matrix) |
| `archer` | Bogenschütze | Fernkampf | 100 | Leicht 1 | Stich 9 | 1,6 | 7 | 2,5 | 10 | 15/30/0/35 | 22 s | 1 | 0,8 | 45 | Salvenfeuer; Brandpfeile (Forschung): Schaden wird Feuer |
| `crossbowman` | Armbrustschütze | Panzerbrecher | 120 | Leicht 2 | Stich 18 | 2,6 | 7,5 | 2,2 | 9 | 40/30/0/40 | 28 s | 1 | 1,2 | 50 | Ignoriert 3 Rüstungspunkte |
| `halberdier` | Hellebardier | Schwere Stangenwaffe | 220 | Mittel 3 | Hieb 16 | 1,6 | 1,5 | 2,2 | 8 | 50/20/0/60 | 30 s | 1 | 1,4 | 60 | +75 % gegen Beritten, trifft 1 Ziel dahinter mit 50 % |
| `scout_rider` | Grenzreiter | Leichte Kavallerie | 220 | Beritten 1 | Hieb 10 | 1,2 | 0,8 | 4,6 | 13 | 50/0/0/60 | 26 s | 2 | 1,5 | 50 | Fackelträger (Lichtradius 5), Aufklärung |
| `knight` | Ritter | Schwere Kavallerie | 380 | Beritten 5 | Hieb 22 | 1,4 | 0,8 | 4,2 | 9 | 90/20/0/80 | 40 s | 3 | 3,0 | 80 | Ansturm: +100 % erster Treffer + 20 Wucht, Betäubung 0,5 s |
| `ember_adept` | Glutklinge | Elite-Infanterie | 260 | Mittel 3 | Feuer 18 | 1,3 | 0,8 | 2,5 | 9 | 80/0/0/60 | 35 s | 2 | 2,0 | 70 | Benötigt Flammentempel; Glutschlag (Radius 1,5, 30 Feuer, Abklingzeit 12 s) |
| `flame_priest` | Flammenpriester | Unterstützung | 140 | Ungerüstet 0 | Feuer 8 | 2,0 | 6 | 2,3 | 10 | 70/0/0/40 | 30 s | 1 | 1,5 | 60 | Heilt 12 HP/2 s; Läuterung: 60 Feuer gegen Untot, Abklingzeit 20 s |
| `ember_mage` | Glutmagier | Kampfmagier | 120 | Ungerüstet 0 | Feuer 24 (Radius 1,5) | 2,4 | 8 | 2,2 | 11 | 120/0/0/50 + 1 Aschglas | 40 s | 2 | 2,5 | 45 | Zauberbuch der Glutschule (12.3) |
| `battering_ram` | Rammbock | Belagerung | 600 | Befestigt 6 | Belagerung 80 | 3,0 | 0,8 | 1,4 | 5 | 60/160/0/0 | 45 s | 4 | 2,0 | – | Schützt 4 Infanteristen im Inneren vor Fernkampf |
| `mangonel` | Mangonel | Belagerung | 300 | Befestigt 2 | Belagerung 55 (Radius 2) | 4,5 | 12 (min. 3) | 1,2 | 9 | 80/200/50/0 | 50 s | 4 | 3,0 | – | Wirksam gegen Formationen und Mauern |
| `trebuchet` | Tribok | Schwere Belagerung | 400 | Befestigt 3 | Belagerung 140 | 7,0 | 18 (min. 5) | 0,8 | 10 | 150/300/100/0 | 70 s | 4 | 4,0 | – | Muss 8 s aufgebaut werden |
| `hero_isolde` | Isolde Raventhorn | Heldin (Ritterin) | 620 (Stufe 3) | Held 5 | Hieb 28 | 1,1 | 0,8 | 2,8 | 11 | – | – | 0 | 5,0 | Aura | siehe 11.4 |

#### Weitere Fraktionen (Kernauswahl)

| ID | Fraktion | Name | HP | Rüstung | Schaden | Int. | Rw. | Tempo | Pop | Besonderheit |
|---|---|---|---|---|---|---|---|---|---|---|
| `skarn_ironguard` | `skarn_clans` | Eisengarde | 300 | Schwer 6 | Wucht 20 | 1,6 | 0,8 | 2,0 | 2 | Schildwall ohne Tempoverlust, Kälteresistenz |
| `skarn_rune_thrower` | `skarn_clans` | Runenwerfer | 130 | Leicht 1 | Arkan 14 (Speer) | 2,0 | 5 | 2,4 | 1 | Runenspeer verlangsamt Ziel um 30 % |
| `veyl_stalker` | `thorn_covenant` | Veyl-Pirscher | 120 | Leicht 1 | Stich 12 | 1,4 | 7 | 2,9 | 1 | Unsichtbar im Wald bis zum ersten Schuss |
| `thorn_warden` | `thorn_covenant` | Dornenhüter | 280 | Mittel 3 | Hieb 15 | 1,4 | 0,8 | 2,3 | 2 | Regeneriert 2 HP/s im Wald, Dornenrüstung (reflektiert 15 % Nahkampfschaden) |
| `saelith_marine` | `saelith_compact` | Seegardist | 200 | Mittel 3 | Stich 14 | 1,2 | 1,5 | 2,5 | 1 | Entern, +20 % Schaden in Küstennähe |
| `saelith_arbalest` | `saelith_compact` | Söldner-Arbalestier | 140 | Mittel 2 | Stich 26 | 3,2 | 8,5 | 2,0 | 1 | Ignoriert 5 Rüstung; Söldner (Sold doppelt, Meinung zum Pakt nötig) |
| `hollow_thrall` | `hollowed` | Hohlknecht | 120 | Untot 1 | Hieb 8 | 1,5 | 0,8 | 2,2 | 1 | Furchtlos (keine Moral), Bindung, nachts +15 % Schaden |
| `hollow_archer` | `hollowed` | Knochenschütze | 80 | Untot 0 | Stich 8 | 1,8 | 6,5 | 2,3 | 1 | Fahle Pfeile: −10 % Heilung beim Ziel für 6 s |
| `grave_brute` | `hollowed` | Grabkoloss | 650 | Untot 4 | Wucht 32 (Radius 1,2) | 2,4 | 1,0 | 1,6 | 3 | Doppelter Gebäudeschaden, Schreckensaura (−10 Moral, Radius 5) |
| `veil_wraith` | `hollowed` | Schleiergeist | 200 | Ätherisch 0 | Leere 14 | 1,6 | 0,8 | 3,0 | 2 | Schwebt über Mauern und Wasser, −2 Moral je Treffer |
| `hollow_priest` | `hollowed` | Leerpriester | 180 | Leicht 1 | Leere 12 | 2,0 | 6 | 2,2 | 2 | Erweckt Gefallene als `hollow_thrall` (1 je 8 s, max. 6) |
| `ashen_legionnaire` | `ashen_legion` | Aschenlegionär | 340 | Schwer 5 | Hieb 18 | 1,3 | 0,8 | 2,2 | 2 | Unsterbliche Legion: Erhebt sich einmal mit 30 % HP, sofern nicht durch Feuer getötet |
| `cinder_knight` | `ashen_legion` | Glutritter Aureths | 520 | Beritten 6 | Feuer 26 | 1,4 | 0,8 | 4,0 | 4 | Brennender Ansturm hinterlässt Glutspur (5 Feuer/s, 6 s) |

#### Gebäude mit Kampfwerten (Vertical Slice)

| ID | Name | HP | Rüstung | Kosten G/H/S/N | Bauzeit | Funktion |
|---|---|---|---|---|---|---|
| `keep` | Bergfried | 2.000 | Befestigt 8 | – (Startgebäude) / Stufe II: 150/100/200/0 | 90 s (Stufe II) | Hauptgebäude, bildet `worker` aus, +10 `population_cap`, Abgabepunkt, Lager; Stufe II schaltet Forschung „Ritterschwur“ frei |
| `farmstead` | Gehöft | 600 | Befestigt 2 | 25/100/0/0 | 35 s | +5 `population_cap`, Nahrungsproduktion (Vertical Slice: 0,4 `food`/s je zugewiesenem Arbeiter, max. 4 Arbeiter) |
| `barracks` | Kaserne | 1.200 | Befestigt 5 | 50/150/50/0 | 50 s | Bildet `swordsman`, `spearman`, `archer` aus; `knight` nach „Ritterschwur“ |
| `watchtower` | Wachturm | 700 | Befestigt 6 | 0/75/75/0 | 40 s | Stich 10 je 1,5 s, Reichweite 8, Sicht 12, Lichtradius 6; bis zu 2 `archer` als Besatzung (+1 Reichweite und je Bogenschütze +6 Schaden) |
| `hollow_shrine` | Hohlschrein | 3.000 | Befestigt 7 | – | – | Hohle Basis; erzeugt Hohlknechte, Bindungsquelle (Radius 20) |
| `bone_spire` | Knochenturm | 900 | Befestigt 5 | – | – | Hohler Verteidigungsturm, Leere 14 je 2 s, Reichweite 7, Bindungsquelle (Radius 12) |

### 10.4 Veteranenstufen

| Stufe | Erfahrung (Punkte) | Bonus |
|---|---|---|
| Rekrut | 0 | – |
| Erfahren | 100 | +10 % Schaden, +5 Moral |
| Veteran | 300 | +20 % Schaden, +10 % HP, +10 Moral |
| Elite | 700 | +30 % Schaden, +20 % HP, +20 Moral, +1 Rüstung |

Erfahrung: Getöteter Gegner gibt `Bevölkerungspunkte × 20`, aufgeteilt auf Einheiten im Radius 6; überlebte
Schlacht +20; Drill durch den Marschall (6.3).

### 10.5 Moral

Jede lebende Einheit hat Moral 0–100 (Startwert = Grundmoral der Einheit, durch Modifikatoren veränderbar).

| Modifikator | Wert |
|---|---|
| Held im Radius 8 | +10 (+ Aura-Boni, z. B. Sammelbanner +20) |
| Fackel- oder Lichtkreis bei Nacht | +5 |
| Nacht ohne Licht | −10 |
| Eigene Verluste | −1 je 1 % Verlust der eigenen Gruppe innerhalb von 10 s |
| Flankiert / von hinten angegriffen | −10 / −20 (einmalig je Angriff, erholt sich) |
| Umzingelt (Feinde in 3 von 4 Richtungen) | −25 |
| Schreckensaura (Grabkoloss, Bosse) | −10 / −15 |
| Eigener Anführer gefallen | −20 |
| Kriegsheld-Herrscher führt persönlich | +10 |
| Feind flieht in der Nähe | +5 |
| Hohlland | −1 je 10 s |

| Moralzustand | Bereich | Wirkung |
|---|---|---|
| Entschlossen | > 80 | +10 % Schaden |
| Standhaft | 40–80 | normal |
| Wankend | 20–39 | −20 % Schaden, −10 % Tempo, Fernkämpfer feuern 25 % langsamer |
| Fliehend | < 20 | Einheit ist 10 s unkontrollierbar und flieht zum Sammelpunkt; sammelt sich bei Moral ≥ 35 (Erholung +3/s außerhalb des Kampfes) |

**Hohle und Moral**: Hohle sind furchtlos, unterliegen aber der **Bindung**: Jeder Hohle muss sich im Radius einer
Bindungsquelle (`hollow_shrine` 20 Felder, `bone_spire` 12 Felder, `hollow_priest` 8 Felder, Bosse 15 Felder)
befinden oder verliert 1 % seiner maximalen HP pro Sekunde. Bei Nacht verdoppeln sich alle Bindungsradien.
Daraus folgt die Kernstrategie gegen die Hohlen: Bindungsquellen zerstören, Kämpfe bei Tag führen.

### 10.6 Gelände

| Gelände | Bewegung | Kampf | Sonstiges |
|---|---|---|---|
| Ebene | 100 % | – | Kavallerie optimal |
| Straße | 120 % | – | – |
| Anhöhe | bergauf 80 % | Fernkampf von oben: +1 Reichweite, +15 % Schaden; Nahkampf bergauf −10 % | Sichtvorteil +3 |
| Wald | 75 % (Kavallerie 60 %) | Fernkampf gegen Ziele im Wald −30 % Trefferchance | Sicht im Wald −50 %, Veyl unsichtbar |
| Sumpf | 60 % | Nahkampf −10 % | Erschöpfung ×1,5 |
| Furt | 50 % | Einheiten im Wasser −20 % Schaden | Kavallerie verliert Ansturm |
| Ruinen | 85 % | Deckung: −25 % Fernkampfschaden gegen Einheiten darin | Häufig Aschglas, Fallen |
| Schnee | 80 % | – | Erschöpfung ×1,5, Lebende ohne Winterausrüstung −1 HP/10 s |
| Hohlland | 90 % | Hohle regenerieren 1 HP/s | Lebende −1 Moral/10 s |
| Lava-/Glutfeld (Glutweite) | 70 % | 4 Feuerschaden/s für Nichtresistente | Nur passierbar mit Forschung „Glutstiefel“ oder Aschenlegion |

### 10.7 Wetter

| Wetter | Fernkampf | Bewegung | Sicht | Sonstiges |
|---|---|---|---|---|
| Klar | – | – | – | – |
| Regen | −15 % Treffer | −10 % | −10 % | Feuerschaden −25 %, Brände erlöschen schneller |
| Sturm | −30 % Treffer | −15 % | −25 % | Belagerungsgerät −20 % Genauigkeit |
| Nebel | −10 % | – | −60 % | Hinterhalte: erster Angriff aus dem Nebel +25 % Schaden |
| Schnee | −10 % | −20 % | −20 % | Erschöpfung ×1,5 |
| Ascheregen | −10 % | – | −30 % | Hohle +10 % Schaden, Lebende −5 Moral; tritt bei Schleierstärke ≥ 2 auf |
| Hitze (Glutweite, Aurethwüste) | – | −10 % | – | Erschöpfung ×1,5 für Schwer- und Beritten-Rüstung |

Wetterwechsel werden 60 s vorher angekündigt (Himmel verfärbt sich, Symbol im HUD).

### 10.8 Tageszeit

Ein RTS-Tageszyklus dauert standardmäßig **12 Minuten**: 7 min Tag, 1 min Dämmerung, 3 min Nacht,
1 min Morgengrauen (missionsspezifisch anpassbar).

| Phase | Wirkung |
|---|---|
| Tag | Normal |
| Dämmerung | Sicht −20 %; Ankündigung „Die Nacht naht“ |
| Nacht | Sicht −40 % außerhalb von Lichtquellen; Hohle +15 % Schaden, +10 % Tempo, Bindungsradien ×2; Lebende ohne Licht −10 Moral |
| Morgengrauen | „Erster Strahl“: Hohle −10 % Schaden und −10 % Tempo für 60 s |

**Lichtquellen**: Bergfried (Radius 10), Wachturm (6), Fackelträger/Grenzreiter (5), Feuerstelle (4,
baubar für 10 Holz), Glutmagie (Zauber erzeugen temporäres Licht), brennende Gebäude (8).

### 10.9 Flankierung, Ansturm und Erschöpfung

**Flankierung** (gemessen am Winkel zwischen Angriffsrichtung und Blickrichtung des Ziels):

| Winkel | Bezeichnung | Schaden | Moral des Ziels |
|---|---|---|---|
| 0°–90° | Frontal | 100 % | – |
| 90°–135° | Flanke | 125 % | −10 |
| > 135° | Rücken | 150 % | −20 |

**Ansturm**: Kavallerie, die mindestens 4 Felder geradlinig anläuft, verursacht beim ersten Treffer +100 %
Schaden plus 20 Wucht und betäubt 0,5 s. Abklingzeit 15 s (die Einheit muss sich lösen und neu anlaufen).
Speerwall und Hellebarden kontern den Ansturm.

**Erschöpfung** (Ausdauer 0–100):

| Tätigkeit | Ausdauer |
|---|---|
| Gehen | ±0 |
| Rennen (Doppelklick-Bewegung) | −2/s (Tempo +30 %) |
| Kampf | −1/s |
| Ansturm | −15 je Ansturm |
| Ruhe | +3/s |
| Schwere oder berittene Rüstung | Verbrauch ×1,3 |

Unter 30 Ausdauer: −15 % Schaden, −20 % Tempo. Bei 0: −40 % Schaden und Tempo, keine Fähigkeiten.

### 10.10 Belagerungen in der Echtzeitwelt

| Element | Werte | Funktion |
|---|---|---|
| Palisade | 400 HP/Feld, Befestigt 4, 10 Holz/Feld | Frühe Verteidigung, brennbar (Feuer 150 %) |
| Steinmauer | 1.200 HP/Feld, Befestigt 8, 8 Stein/Feld | Begehbar; Fernkämpfer auf Mauern +2 Rüstung, Anhöhenbonus |
| Torhaus | 2.000 HP, Befestigt 9 | Öffnet nur für eigene Einheiten |
| Mauerturm | 1.500 HP, Stich 14/1,5 s, Rw. 9 | Besatzung bis 4 Fernkämpfer |
| Pechnase | Am Torhaus, 40 Feuer Radius 2, Abklingzeit 20 s | Verteidigung gegen Rammböcke |
| Sturmleiter | Infanterie kann in 15 s eine Leiter anlegen | 2 Einheiten gleichzeitig steigen hoch; Leiter kann umgestoßen werden (Wucht 50) |
| Belagerungsturm | 1.000 HP, transportiert 12 Einheiten, 2 `siege_parts` | Brücke auf Mauerkrone |
| Untergraben | Pioniere (`worker` mit Forschung „Sappeure“) graben 60 s an einem Mauerabschnitt | Mauerabschnitt stürzt ein |
| Breschen | Je 25 % Weltkarten-Belagerungsfortschritt ein eingestürzter Mauerabschnitt (4.8) | Verbindung beider Ebenen |
| Hunger | Eingeschlossene Garnison: −2 % Moral/Minute in der RTS-Erstürmung, wenn das Weltkartenlager leer ist | – |

Siegbedingung einer Erstürmung: Bergfried oder Zitadelle einnehmen (Feindeinheiten im Radius 6 = 0, eigene
Einheiten im Radius 6 für 30 s) oder Kapitulation durch Moralbruch der Garnison.

---

## 11. Helden und Rollenspielsystem

### 11.1 Rolle der Helden

Helden sind benannte Figuren mit Attributen (Kapitel 5.2), Eigenschaften, Stufen, Fähigkeiten und Ausrüstung.
Sie sind gleichzeitig **Weltkarten-Figuren** (Heerführer, Amtsträger, Familienmitglieder) und
**RTS-Einheiten**. Ein Held kann Herrscher sein, ein Herrscher kann als Held kämpfen (mit allen Risiken).
Pro Armee sind bis zu 3 Helden erlaubt, pro RTS-Schlacht bis zu 5 Helden je Seite.

**Heldenrekrutierung**: Taverne (Söldnerhelden), Hof (Familienmitglieder ab 16 mit Kriegskunst ≥ 8),
Ereignisse (wandernde Ritter, Veyl-Druiden, Runenmeister), Kampagnenfiguren.

### 11.2 Stufen und Erfahrung

Stufen 1–20. Benötigte Erfahrung von Stufe n auf n+1: `XP(n) = 100 × n^1,5` (gerundet).

| Stufe | XP bis nächste Stufe | Kumuliert |
|---|---|---|
| 1 → 2 | 100 | 100 |
| 2 → 3 | 283 | 383 |
| 3 → 4 | 520 | 903 |
| 4 → 5 | 800 | 1.703 |
| 5 → 6 | 1.118 | 2.821 |
| 10 → 11 | 3.162 | 14.268 |
| 19 → 20 | 8.282 | 67.135 |

**Erfahrungsquellen**: Getöteter Gegner im Radius 10 `= Bevölkerungspunkte × 15`; Boss 300–2.000; Schlacht
gewonnen 100 × Stufe des stärksten Gegners; Quest 200–1.500; Weltkarte: Jahr als Heerführer im Krieg 200,
Jahr im Amt 50.

**Je Stufe**: +8 % HP (multiplikativ auf Grundwert), +5 % Schaden, +1 Talentpunkt, +1 Attributspunkt alle
2 Stufen (frei verteilbar). Fähigkeiten: 3 Grundfähigkeiten (Stufe 1, 1, 3), Meisterfähigkeit ab Stufe 10.

### 11.3 Talente

Jeder Held besitzt **drei Talentbäume** à 5 Reihen (Reihe n erfordert 2 × (n−1) Punkte im Baum). Zusätzlich gibt
es einen allgemeinen Baum „Feldherr“ (Weltkarte), der allen Helden offensteht.

**Allgemeiner Baum „Feldherr“** (Auswahl)

| Reihe | Talent | Wirkung |
|---|---|---|
| 1 | Marschordnung | Armee +10 % Weltkarten-Bewegung |
| 1 | Quartiermeister | −15 % Versorgungsverbrauch |
| 2 | Späherblick | Sieht Armeen in 2 Provinzen Entfernung |
| 3 | Belagerungskunde | +15 % Belagerungsfortschritt |
| 4 | Gewaltmarsch | Einmal pro Monat 3 Tage doppelte Bewegung, −10 Ausdauer der Armee zu Schlachtbeginn |
| 5 | Legende des Feldes | +10 % Kampfkraft bei automatischer Berechnung, +10 Moral |

### 11.4 Beispielheldin: Isolde Raventhorn (`hero_isolde`)

| Wert | Stufe 1 | je Stufe | Stufe 3 (Start Mission 1) | Stufe 10 |
|---|---|---|---|---|
| HP | 535 | +8 % des Grundwerts (≈ +43) | 620 | ≈ 920 |
| Rüstung | Held 5 | +1 alle 4 Stufen | Held 5 | Held 7 |
| Schaden | Hieb 25 | +5 % | Hieb 28 | ≈ Hieb 36 |
| Energie (Glut) | 100, Regeneration 2/s | +5 Maximum | 110 | 145 |
| Attribute | Kriegskunst 13, Verwaltung 9, Diplomatie 8, Intrige 5, Gelehrsamkeit 7, Charisma 12 | | | |
| Eigenschaften | Tapfer, Gerecht, Ehrlich, Aschenfieber-Narbe | | | |

**Fähigkeiten**

| Fähigkeit | ID | Taste | Kosten | Abklingzeit | Wirkung |
|---|---|---|---|---|---|
| Glutspalter | `ember_cleave` | `Q` | 35 Glut | 14 s | Kegel 120° vor Isolde, Reichweite 2,5: 60 Feuerschaden (+10 je Fähigkeitsstufe), setzt Ziele 4 s in Brand (5 Feuer/s); Untote werden 1 s zurückgestoßen |
| Sammelbanner | `rallying_banner` | `W` | 50 Glut | 45 s | Setzt ein Banner für 20 s: Radius 6, +20 Moral, +15 % Angriff (+5 % je Stufe), Fliehende im Radius sammeln sich sofort; Banner leuchtet (Lichtradius 6) |
| Aschenschild | `ashen_ward` | `E` | 40 Glut | 30 s | Schild auf Isolde oder Verbündeten (Reichweite 5) absorbiert 150 Schaden (+25 je Stufe) für 10 s; bei Ablauf/Bruch Aschewelle: 20 Feuer Radius 2 |
| Meisterfähigkeit (Stufe 10): Letzte Glut der Aschmark | `last_ember` | `T` | 100 Glut | 180 s | Isolde entfacht für 12 s eine Glutaura (Radius 5): Verbündete +25 % Schaden und unverwundbar gegen Moralverlust; Untote im Radius erleiden 25 Feuer/s |

Fähigkeitsstufen: Jede Grundfähigkeit hat 3 Stufen (Talentpunkte im zugehörigen Baum schalten Stufe 2 und 3 frei).

**Talentbäume Isoldes**: *Glutklinge* (Feuerschaden, Brand, Glutspalter-Verbesserungen), *Wächterin der Aschmark*
(Aschenschild, Rüstung, Abwehr gegen Hohle, Bindungsunterbrechung), *Banner der Grenzlande* (Moral, Aura,
Sammelbanner, Milizboni: „Grenzerblut – Miliz-Einheiten im Radius 8 erhalten +2 Rüstung“).

### 11.5 Ausrüstung

| Slot | Beispiel-Gegenstand | Wirkung |
|---|---|---|
| Waffe | Glutgeschmiedetes Langschwert | +6 Schaden, 10 % Chance auf Brand |
| Nebenhand | Wappenschild der Raventhorns | +2 Rüstung, −25 % Fernkampfschaden von vorn |
| Kopf | Grenzhelm mit Aschvisier | +1 Rüstung, Immunität gegen Ascheregen-Sichtmalus |
| Brust | Schuppenpanzer aus Skarnstahl | +3 Rüstung, −5 % Tempo |
| Hände | Handschuhe des Torwächters | +10 % Angriffstempo |
| Füße | Glutstiefel | Glutfelder passierbar |
| Umhang | Mantel der Ewigen Flamme | +15 % Feuerresistenz, +5 Moral im Radius 6 |
| Amulett | Splitter von Ithrens Siegel | −50 % Hohlungsgewinn |
| Ring (2 Slots) | Siegelring der Wächterin; Ring der Gezeiten | +2 Charisma; +1 Glut/s Regeneration |
| Reittier | Kriegsross aus Valemark | Held wird beritten: Tempo 4,0, Ansturm, Rüstungstyp Beritten |
| Relikt | Aschglasherz | +20 % Fähigkeitsschaden, +2 Hohlung pro Jahr |

**Seltenheit**: Gewöhnlich (grau), Gut (weiß), Meisterhaft (grün), Selten (blau), Legendär (gold),
**Aschgezeichnet** (fahlgrün: sehr stark, erzeugt Hohlung). Gegenstände werden von Schmieden hergestellt
(Kosten aus der Wirtschaft, z. B. Meisterschwert: 4 `iron`, 1 `ashglass`, 300 Gold), in Ruinen gefunden oder
von Bossen erbeutet. Gegenstände getöteter oder gefangener Helden fallen dem Sieger zu.

### 11.6 Verletzung, Gefangenschaft und Tod

Fällt die HP eines Helden auf 0, wird er **niedergestreckt** (nicht sofort tot). Er bleibt 6 s am Boden; eigene
Einheiten können ihn in dieser Zeit bergen (Einheit neben dem Helden, 3 s). Danach wird er automatisch vom
Schlachtfeld getragen, wenn das eigene Heer das Feld hält, oder er fällt in Feindeshand.

**Ausgangswurf nach dem Niederstrecken** (Standard-Einstellung):

| Ergebnis | Wahrscheinlichkeit | Folge |
|---|---|---|
| Leicht verwundet | 50 % | 60 s RTS-Abwesenheit (Wiederbelebung am Bergfried/Altar gegen Gold `50 × Stufe`) bzw. 15 Tage auf der Weltkarte |
| Schwer verwundet | 30 % | 30–90 Tage Genesung; 25 % Chance auf eine dauerhafte Eigenschaft (Einäugig, Verkrüppelt, Narbe) |
| Gefangen | 10 % (nur wenn der Gegner das Feld hält, sonst „Schwer verwundet“) | siehe unten |
| Tot | 10 % + 5 % je früherer schwerer Verwundung, −5 % mit Feldscher/Flammenpriester in der Armee | Held stirbt; Chronikeintrag, Gegenstände verbleiben auf dem Schlachtfeld |

**Gefangenschaft**: Der Fänger kann wählen: Lösegeld fordern (Standard: `Stufe × 100 + Prestige/2` Gold),
gefangen halten (Druckmittel), austauschen, anwerben (nur bei Meinung ≥ 0 und gleicher Religion; Chance
`Charisma_Fänger × 2 + Meinung/2 %`), hinrichten (Tyrannei +10, Meinung der Dynastie des Opfers −80 dauerhaft).
Fluchtchance pro Monat: `2 % + Intrige_Gefangener × 0,5 % − Kerkerstufe × 1 %`. Befreiung durch Kommandoquest
(RTS-Heldenmission) möglich.

**Konfigurierbarer Permadeath** (Einstellung zu Beginn einer Partie, im Ironman fixiert):

| Stufe | Name | Regel |
|---|---|---|
| 0 | **Gnädig** | Helden sterben nie durch Kampf; „Tot“ wird zu „Schwer verwundet“. Tod nur durch Alter, Seuche oder bewusste Entscheidungen (Duell, Hinrichtung durch den Spieler) |
| 1 | **Standard** | Tabelle oben |
| 2 | **Hart** | Todeschance 25 % + 10 % je früherer schwerer Verwundung; Wiederbelebung im RTS kostet doppelt |
| 3 | **Aschenfall** | Jedes Niederstrecken in einer Schlacht ist tödlich, sofern der Held nicht innerhalb von 6 s geborgen wird; Kampagnenhelden eingeschlossen |

**Kampagnenhelden**: Stirbt ein Kampagnenheld (z. B. Isolde), läuft die Kampagne weiter; die Geschichte passt sich
über Nachfolgefiguren an (siehe `campaign.md`, Abschnitt „Tod von Kernfiguren“). Missionen, die einen
bestimmten Helden voraussetzen, verwenden dessen Erben oder eine Ersatzfigur mit eigener Vertonung.

---

## 12. Magie und die Hohlung

### 12.1 Grundsätze

- Magie in Eldraeth ist **Nachhall der verstummten Götter** und **Durchsickern des Hohlen Tiefs**. Seit dem
  Aschenfall ist der Schleier dünn; Aschglas speichert die Kraft, die dabei frei wird.
- Jede Magie hat einen Preis: Energie des Wirkers, Aschglas, gesellschaftliche Akzeptanz – und bei zwei
  Schulen Hohlung.
- Magie wirkt auf **beiden Ebenen**: Kampfzauber in der Echtzeitwelt, **Rituale** auf der Weltkarte
  (Provinz- oder Reichseffekte über Monate).

### 12.2 Wirker und Energie

| Wirker | Energie (Maximum / Regeneration) | Herkunft |
|---|---|---|
| Held mit Magieschule | 100–250 / 2–4 pro s | Stufe, Gelehrsamkeit |
| `ember_mage` und andere Kampfmagier | 80 / 1,5 pro s | Gelehrte (Magierturm) |
| `flame_priest` | 60 / 1 pro s | Klerus |
| Hofmagier (Weltkarte) | Ritualkraft 10 + Gelehrsamkeit pro Monat | Amt „Hofmagier“ (Nebenposten am Hof) |

Aschglas-Fokus (`ashglass_focus`) als Ausrüstung: +25 % Energie, +15 % Zauberwirkung.

### 12.3 Die neun Schulen

| # | Schule | ID | Ursprung | Haltung der Kulturen | Beispielzauber (Echtzeit) | Ritual (Weltkarte) |
|---|---|---|---|---|---|---|
| 1 | **Glutmagie** | `school_ember` | Ewige Flamme, Glutklingen | Valemark: verehrt | *Glutpfeil* (40 Feuer, 15 Energie); *Flammenwand* (Linie 6 Felder, 20 Feuer/s, 8 s, 50 Energie); *Läuterndes Feuer* (Radius 3, 80 Feuer gegen Untot, heilt Lebende 30, 60 Energie); Meister: *Phönixasche* (gefallene Verbündete im Radius 5 stehen mit 30 % HP wieder auf, 150 Energie) | *Wachfeuer der Grenze*: Provinz −50 % Hohlenspawns für 12 Monate (20 `ashglass`) |
| 2 | **Ordnungsmagie Ithrens** | `school_order` | Altes Licht- und Gesetzesgebet | Valemark, Kaldris: geduldet | *Klares Auge* (enthüllt Unsichtbare, Radius 8); *Siegel der Stille* (Ziel kann 6 s nicht zaubern); *Bannkreis* (Radius 4, 15 s: Hohle können nicht eintreten); Meister: *Ithrens Urteil* (Ziel erleidet Schaden in Höhe seiner zuletzt verursachten 10 s) | *Gesetzestafel*: Korruption −10 Prozentpunkte in einer Provinz, 24 Monate |
| 3 | **Übergangskunst Morvaines** | `school_passage` | Totenwächter, Grabhüter | Überall misstrauisch geduldet | *Totenruhe* (Gefallene im Radius 6 können 60 s nicht erweckt werden); *Seelengeleit* (Untoter unter 30 % HP wird sofort erlöst); *Letztes Wort* (Sicht auf den Todesort des letzten Gefallenen); Meister: *Morvaines Schwelle* (Verbündete im Radius 5 können 5 s nicht unter 1 HP fallen) | *Grablegung*: Hohlungsgewinn durch Schlachten in einer Provinz −100 %, 12 Monate |
| 4 | **Wachstumsmagie Calyx'** | `school_growth` | Grüner Kreis, Veyl | Dornenbund: heilig | *Wurzelgriff* (festhalten 4 s, Radius 2); *Heilende Sporen* (Radius 4, 6 HP/s, 10 s); *Dornenwall* (Barriere 8 Felder, 300 HP je Feld); Meister: *Erwachen des Hains* (3 Baumhüter für 45 s) | *Ernteritual*: Ernte +30 % in einer Provinz oder Dürre-Folgen halbiert (10 `herbs`, 5 `ashglass`) |
| 5 | **Gezeitenmagie Thals** | `school_tide` | Haus der Gezeiten | Saelith: verehrt | *Nebelschleier* (Radius 6, Sicht −80 %, 20 s); *Strömung* (Verbündete +30 % Tempo, 10 s); *Sturmruf* (Radius 5, Fernkampf −50 %, 12 s); Meister: *Wandelgestalt* (Held verwandelt sich 20 s in Sturmgestalt: Ätherisch, fliegend) | *Günstige Winde*: Handelsrouten zur See +25 % Kapazität, 12 Monate |
| 6 | **Steinkunst Oruuns** | `school_stone` | Runenmeister Skarnheims | Skarnheim: verehrt | *Steinhaut* (+5 Rüstung, 12 s); *Runenwall* (Mauerabschnitt 4 Felder, 800 HP, 30 s); *Erdbeben* (Radius 4, 60 Wucht, Gebäude ×2, Betäubung 1 s); Meister: *Bergherz* (Ziel ist 8 s unverwundbar, aber unbeweglich) | *Grundstein*: Befestigungsstufe +2 in einer Provinz, dauerhaft (40 `cut_stone`, 10 `ashglass`) |
| 7 | **Aschglas-Arkanistik** | `school_crystal` | Gelehrte Aureths, Akademien | Valemark: lizenziert, Kirche skeptisch | *Kristallsplitter* (35 Arkan, durchschlägt 3 Ziele); *Arkaner Fokus* (nächster Zauber eines Verbündeten +50 %); *Resonanzfeld* (Radius 4: feindliche Zauber kosten +100 %, 15 s); Meister: *Aschglaskonstrukt* (beschwört Kristallwächter, 900 HP, 60 s) | *Aschglas-Resonanz*: Forschung +25 % für 6 Monate; Provinz-Hohlung +2 |
| 8 | **Schattenkunst** | `school_shadow` | Spione, Hexenzirkel | Überall verboten außer per Lizenz | *Trugbild* (2 Doppelgänger des Ziels, 15 s); *Schattenschritt* (Teleport 8 Felder); *Fluch der Schwäche* (−30 % Schaden, 10 s); Meister: *Massenpanik* (Radius 5, −40 Moral) | *Gedankenflüstern*: Komplottstärke eines laufenden Komplotts +20; bei Entdeckung Tyrannei +10 |
| 9 | **Leermagie** | `school_void` | Das Hohle Tief, Leere Liturgie | Überall verboten | *Seelenentzug* (40 Leere über 4 s, heilt den Wirker um 100 %); *Hohlruf* (2 `hollow_thrall` aus Leichen); *Schleierriss* (Radius 3, 30 Leere/s, 6 s, erzeugt Hohlland); Meister: *Tiefe Umarmung* (lebende Einheit unter 25 % HP wird dauerhaft zum Hohlen des Wirkers) | *Ruf aus dem Tief*: Hohle Armee (Stärke 20 + 2 × Hohlungsstufe) erscheint in einer Provinz; +10 Hohlung des Herrschers |

### 12.4 Forschung und Kosten

Jede Schule hat **vier Grade**. Forschung erfolgt in Bibliotheken (Forschungspunkte aus Gelehrten und Büchern),
Magiertürmen und Tempeln (Glut, Ordnung, Übergang, Wachstum, Gezeiten, Stein).

| Grad | Forschungspunkte | Aschglas | Schaltet frei |
|---|---|---|---|
| I Novize | 200 | 5 | 1–2 Grundzauber, Kampfmagier-Einheit der Schule |
| II Adept | 600 | 15 | 3. Zauber, 1. Ritual |
| III Meister | 1.500 | 40 | Meisterzauber, verbesserte Rituale |
| IV Erzmagie | 4.000 | 100 | Erzritual (einmalig pro Reich und Jahrhundert, z. B. Glutschule: *Ewige Wacht* – alle Provinzen Hohlung −10) |

**Forschungsgeschwindigkeit**: `FP/Monat = Σ Gelehrte × 0,5 + Bücher-Verbrauch × 10 + Gelehrsamkeit_Kanzler × 2 + Magiegesetz-Bonus (Verbot −50 %, Lizenz 0 %, Freiheit +30 %)`.

**Kulturelle Zugänglichkeit**: Schulen, die eine Kultur verehrt, kosten 25 % weniger Forschung; verbotene Schulen
erzeugen bei Erforschung 30 % Entdeckungschance pro Grad (Geheimnis „Verbotene Künste“).

### 12.5 Aschglas als Magiewährung

- Rituale kosten Aschglas (`ashglass`), Kampfzauber nur Energie – außer Meisterzaubern (je Wirkung 1 `ashglass`
  aus dem Armeelager).
- Aschglas ist knapp: Es entsteht nur bei Schleierstärke ≥ 2, also an der Grenze zur Gefahr. Wer viel Magie will,
  muss in die Nähe des Tiefs.
- Diese Spannung ist gewollt: Magische Stärke und Hohlungsrisiko wachsen gemeinsam.

### 12.6 Magische Risiken

| Risiko | Auslöser | Folge |
|---|---|---|
| Überladung | Zauber bei Energie < 10 % | 10 % Chance: Wirker erleidet 20 % seiner HP als Schaden |
| Aschglasvergiftung | Wirker trägt Aschglas-Fokus | +1 Hohlung/Jahr |
| Schleierriss | Leermagie oder Aschglas-Arkanistik Grad IV | Provinz-Schleierstärke +0,2 |
| Kirchenzorn | Öffentliche Nutzung verbotener Schulen | Meinung Klerus −20, Frömmigkeit −50 |

### 12.7 Schleierstärke

Jede Provinz hat eine **Schleierstärke** von 0 (fest) bis 5 (zerrissen). Sie bestimmt Aschglasvorkommen,
Hohlenaktivität und Ascheregen.

| Stufe | Name | Wirkung |
|---|---|---|
| 0 | Fest | Keine Hohlen, kein Aschglas |
| 1 | Brüchig | Gelegentlich Hohle nachts (1 kleine Gruppe pro Monat) |
| 2 | Dünn | Aschglasadern, Ascheregen 10 % der Tage, nächtliche Angriffe |
| 3 | Rissig | Hohlschreine entstehen, Hohlung der Bevölkerung +0,5/Monat |
| 4 | Offen | Hohlland breitet sich aus, Lebende −10 % Wachstum |
| 5 | Zerrissen | Schleiernarbe: Realität instabil, nur Hohle und geschützte Expeditionen |

```
ΔSchleier pro Monat = 0,01 × (Schnitt der Nachbarprovinzen − eigene Stärke)
                    + 0,002 × Aschglasabbau (Einheiten/Monat)
                    + 0,02 × Leermagie-Einsätze
                    + 0,01 × (Tote der Schlacht / 100) bei Stärke ≥ 2
                    − 0,03 × Glaubensstärke (0–1, Tempel und Klerus)
                    − Reinigungsrituale (Wachfeuer, Grablegung, Hohepriester-Aufgabe)
                    + globaler Schleierdruck (Kapitel 16.2)
```

### 12.8 Die Hohlung als alternativer Entwicklungspfad

Die Hohlung existiert auf drei Ebenen: **persönlich** (Wert jeder Figur 0–100), **Provinz** (Hohlung der
Bevölkerung 0–100) und **Reich** (Hohlungspfad-Stufe, abgeleitet aus Herrscher-Hohlung, Anteil hohlgezeichneter
Provinzen und Nutzung der Leermagie). Ein Reich kann diesen Pfad bewusst gehen – mit eigener Wirtschaft,
eigenen Einheiten und einem eigenen Ende.

```
Reichshohlung = 0,5 × Hohlung_Herrscher + 0,3 × Ø Provinz-Hohlung + 0,2 × min(100, Leermagie-Einsätze_der_letzten_10_Jahre × 5)
```

| Stufe | Name | Reichshohlung | Neue Möglichkeiten | Kosten und Folgen |
|---|---|---|---|---|
| 0 | Unberührt | 0–9 | – | – |
| 1 | Geflüster | 10–24 | Die Stimme aus dem Tief erscheint in Ereignissen; Leermagie Grad I erforschbar ohne Entdeckungsrisiko; +2 Gelehrsamkeit des Herrschers | Geheimnis „Hohlkult“ kann aufgedeckt werden (−60 Meinung bei Gläubigen der Ewigen Flamme) |
| 2 | Gezeichnet | 25–44 | Aschglasertrag +50 %; Leerpriester (`hollow_priest`) rekrutierbar; Gebäude `bone_spire` baubar; neue Ressource **Seelenasche** (`soul_ash`) aus Schlachtfeldern und Friedhöfen | Kirche: Exkommunikationsprüfung jährlich (Chance = Reichshohlung %); Bevölkerung: neue Teilschicht „Hohlgezeichnete“ (arbeiten ohne Nahrung, zahlen keine Steuern, −10 Zufriedenheit der übrigen Schichten) |
| 3 | Verhohlt | 45–64 | Nichtangriffspakt mit den Hohlen (Stimme aus dem Tief als diplomatischer Partner); `hollow_thrall` aus eigenen Gefallenen rekrutierbar (1 Seelenasche je Knecht); `hollow_shrine` baubar; Leermagie Grad III | Alle Reiche mit Ewiger Flamme: Heiliger Krieg als Kriegsgrund gegen dich; Bevölkerungswachstum −50 %; Reinigung nur noch durch Erzritual |
| 4 | Hohlkrone | 65–84 | Herrscher altert nicht mehr; `grave_brute` und `veil_wraith` rekrutierbar; Ritual „Ruf aus dem Tief“ halbiert im Preis; Provinzen werden zu Hohlland mit eigener Produktion (Seelenasche statt Nahrung) | Fruchtbarkeit des Herrschers 0 – Nachfolge nur durch „Hohle Erwählung“ (ein Höfling mit Hohlung ≥ 50); Bedrohung +30 bei allen Reichen; Rückweg ausgeschlossen |
| 5 | Das Tief erwacht | 85–100 | Siegbedingung „Die Hohle Krone“ (16.4) wird aktiv; Schleiernarbe dehnt sich auf eigene Provinzen aus | Alle lebenden Reiche bilden automatisch den „Bund der Letzten Flamme“ gegen dich |

**Hohle Wirtschaft**: Hohlgezeichnete Provinzen produzieren `soul_ash` (Schlachtfelder: 1 je 10 Tote;
Friedhof: 2/Monat je 100 Einwohner; Hohlschrein: wandelt 10 Bewohner in 1 `soul_ash` + 1 `hollow_thrall` um). Hohle
Einheiten haben keinen Gold-Unterhalt, benötigen aber Bindungsquellen und Seelenasche (0,1 je Einheit und Monat).

**Rückweg**: Bis Stufe 2 durch Reinigung (Hohepriester-Aufgabe „Hohlung reinigen“, Ritual *Läuterndes Feuer*,
Pilgerfahrt nach Kaldris: −20 persönliche Hohlung). Auf Stufe 3 nur durch das Erzritual der Glutschule oder den Tod
des Herrschers mit einem Erben unter 20 Hohlung.

---

## 13. Religion

### 13.1 Glaubensgemeinschaften

| Religion | ID | Verbreitung | Oberhaupt / Heilige Stätten | Doktrinen (spielmechanisch) | Besondere Einheiten/Gebäude |
|---|---|---|---|---|---|
| Der Glaube der Ewigen Flamme | `eternal_flame` | Valemark, Aschmark | Erzflammenwahrer in **Kaldris**; Stätten: Kaldris (Flammendom), Brennhollow (Grenzschrein), Solmere (verloren) | Wachsamkeit: +15 % Schaden gegen Hohle; Zehnt; Kirchenhierarchie kann Herrscher exkommunizieren; Glutmagie verehrt | `flame_priest`, `ember_adept`, Flammentempel, Flammendom |
| Der Ahnenpfad | `ancestor_path` | Skarnheim | Kein Oberhaupt, Clanrat der Ältesten; Stätten: Ahnenhallen der Clans | Ahnenehre: Gefallene geben Prestige; +10 % Moral bei Verteidigung der eigenen Provinz; Steinkunst verehrt | Ahnenhalle (Helden-Wiederbelebung halbiert) |
| Der Grüne Kreis | `green_circle` | Dornwald | Druidenzirkel; Stätten: Herzhain der Wurzelmutter | Baumschutz: Abholzung über 30 % der Provinz verärgert den Kreis (−5 Zufriedenheit je 10 %); +20 % Ernte; Wachstumsmagie verehrt | Druidenhain, Veyl-Einheiten |
| Das Haus der Gezeiten | `tide_house` | Saelith-Inseln | Gezeitenkonklave (gewählt); Stätten: Gezeitentempel der Hauptinsel | Pragmatismus: Andersgläubige ohne Malus; +15 % Handel; Gezeitenmagie verehrt | Hafentempel, Seegarde |
| Die Leere Liturgie | `hollow_liturgy` | Geheim, überall; offen in der Schleiernarbe | Die Stimme aus dem Tief; Stätten: jeder Hohlschrein | Verboten; Anhänger erhalten Leermagie, Hohlung +1/Jahr; siehe Hohlungspfad | `hollow_priest`, `hollow_shrine` |
| Die Sonnenhäresie | `sun_heresy` | Aurethwüste, Unterschichten Valemarks | Keine Hierarchie, Wanderprediger; Stätte: Ruinen von Solmere | Erlösungshoffnung: +10 Zufriedenheit der Leibeigenen; Varkhan Aurelith gilt als kommender Erlöser; −30 Meinung zur Ewigen Flamme; Anfällig für das Erwachen des Hohlen Königs | Sonnenprediger (Aufwiegeln von Provinzen) |

### 13.2 Werte

| Wert | Ebene | Bedeutung |
|---|---|---|
| Frömmigkeit | Figur | Währung für religiöse Handlungen (Pilgerfahrt, Heiliger Krieg 500, Bitten an das Oberhaupt) |
| Glaubensstärke | Provinz (0–1) | Anteil der Bevölkerung mit Staatsreligion × Tempelversorgung; senkt Schleierstärke |
| Eifer | Religion (0–100) | Globaler Wert jeder Religion: steigt durch Siege, Bekehrungen, heilige Stätten; erhöht Bekehrungsdruck |
| Haltung des Oberhaupts | Reich | Meinung des Erzflammenwahrers zum Herrscher (Exkommunikation unter −50) |

### 13.3 Bekehrung

```
Bekehrungsfortschritt pro Monat (in %) = (Missionarsstärke − Widerstand) × 0,5
Missionarsstärke = Gelehrsamkeit_Hohepriester × 0,5 + Tempelstufe × 2 + Eifer_Religion / 10 + 5 bei Glaubensgesetz Staatskirche
Widerstand       = Glaubensstärke_alt × 10 + 5 bei heiliger Stätte + Zufriedenheit_Schicht / 20
```

Bei 100 % wechselt die Provinz die Religion. Zwangsbekehrung (Glaubensgesetz Staatskirche + Aufgabe „Häresie
verfolgen“) verdoppelt den Fortschritt, erzeugt aber Spannung +3/Monat auf der Achse Einheimische ↔ Krone.

### 13.4 Kirche und Krone (Ewige Flamme)

- Der **Erzflammenwahrer in Kaldris** ist eine KI-Figur mit eigener Meinung zu allen Herrschern der Flamme.
- Er kann: Heilige Kriege gegen Hohlland und Häretiker ausrufen (Teilnehmer erhalten Frömmigkeit und
  Prestige), Kredite gewähren (8.7), Ehen annullieren, Herrscher exkommunizieren (Vasallen dürfen ohne Malus
  rebellieren, −30 Meinung aller Gläubigen) und Herrscher **weihen** (Eigenschaft Flammengeweiht, +10 Legitimität).
- Investitur-Konflikt: Wer den Hohepriester selbst ernennt statt ihn von Kaldris bestätigen zu lassen, erhält
  +20 % Wirkung des Amtes, aber −20 Meinung des Erzflammenwahrers.

### 13.5 Häresien und die Sonnenhäresie

Häresien entstehen emergent: Jede Provinz der Ewigen Flamme mit Glaubensstärke < 0,4 und Zufriedenheit der
Leibeigenen < 40 hat pro Jahr `(0,4 − Glaubensstärke) × 10 %` Chance, zur Sonnenhäresie zu wechseln, verstärkt
durch Sonnenprediger und Nähe zur Aurethwüste. Die Sonnenhäresie ist kein reines Problem: Sie bietet Zugang zu
Wissen über Varkhan Aurelith und ist in der Kampagne ein möglicher Weg zu einem der Enden.

### 13.6 Die alten Götter

Ithren, Morvaine, Calyx, Thal und Oruun verstummten mit dem Aschenfall. Ihre **Schreine** liegen als Ruinen auf der
Karte. Ab Zeitalter VI der Kampagne (oder im Endgame des Sandbox-Modus) können Schreine wiederhergestellt werden
(Kosten: 5.000 Gold, 200 `ashglass`, eine Heldenquest). Jeder wiederhergestellte Schrein gibt einen globalen
Segen (z. B. Oruun: alle Befestigungen +20 % HP; Calyx: Ernte +15 %; Morvaine: Hohle erheben sich nicht aus
Gefallenen) und senkt den globalen Schleierdruck um 10 %. Alle fünf Schreine sind Voraussetzung für das Ende
„Die Wiederkehr der Stimmen“ (siehe Kampagne) bzw. den Siegtyp „Siegel des Schleiers“.

---

## 14. Monster, neutrale Fraktionen und Bosse

### 14.1 Spielrolle

Neutrale Kräfte erfüllen drei Aufgaben: **Druck** (Bedrohung, die Verteidigung erzwingt und Expansion bremst),
**Gelegenheit** (Beute, Erfahrung, Aschglas, Verbündete) und **Weltgefühl** (die Welt ist gefährlich und lebendig,
auch ohne den Spieler).

### 14.2 Die Hohlen als Weltsystem

Die Hohlen (`hollowed`) sind keine Fraktion mit Diplomatie, sondern eine **Ausbreitungssimulation**:

- **Hohlschreine** entstehen in Provinzen mit Schleierstärke ≥ 3 (Chance 5 % pro Jahr je Provinz) oder werden von
  Leerpriestern errichtet.
- Jeder Schrein erzeugt pro Monat Hohle im Wert von `5 + Schleierstärke × 3` Bevölkerungspunkten, bis zum
  Limit `40 × Schleierstärke` je Provinz.
- **Raubzüge**: Überschreitet die Hohlenstärke einer Provinz 50 % ihres Limits, ziehen Gruppen in benachbarte
  lebende Provinzen (bevorzugt nachts, bevorzugt zu Friedhöfen, Schlachtfeldern und schwach verteidigten Dörfern).
- In der RTS-Stadt erscheinen Hohle über **Spawnpunkte** am Kartenrand und an Hohlungsflächen.
- Getötete Lebende in Hohlland können als `hollow_thrall` aufstehen (25 % ohne Bestattung).

### 14.3 Neutrale Fraktionen und Monster

| Kraft | Vorkommen | Verhalten | Spielrolle | Belohnung |
|---|---|---|---|---|
| **Die Aschenlegion** (`ashen_legion`) | Aurethwüste, um Solmere | Patrouilliert festgelegte Routen, greift alles an, was Solmere näherkommt; erwacht im Endgame (16.2) | Wächter des Endgegners, Hochrisiko-Zone | Aureth-Artefakte, Aschglas in großen Mengen |
| **Aschwölfe** | Aschmark, Aurethwüste | Rudel jagen nachts Herden und Arbeiter | Frühe Bedrohung, Störung der Wirtschaft | Häute, Erfahrung |
| **Grabwürmer** | Friedhöfe, Schlachtfelder | Graben sich unter Gebäude, fressen Leichen | Belohnen das Bestatten der Toten | Seltene Reagenzien |
| **Glutsalamander** | Glutweite | Feuerresistent, setzen Gebäude in Brand | Bremsen Expansion in die Glutweite | `sulfur`, Glutschuppen (Rüstungsmaterial) |
| **Moorlichter** | Sümpfe Valemarks, Dornwaldränder | Locken Einheiten vom Weg (Kontrollverlust 3 s) | Gefahr für Karawanen | Aschglas-Splitter |
| **Raubritter und Banditen** | Grenzprovinzen mit Unruhe > 40 | Überfallen Karawanen, erpressen Dörfer | Folge schlechter Ordnung | Lösegeld, Beute, anwerbbar als Söldner |
| **Söldnerkompanien** | Saelith-Häfen, Tavernen | Anwerbbar gegen Sold; wechseln die Seite bei 3 Monaten Soldverzug | Militärische Flexibilität gegen Gold | – |
| **Aschglas-Sucher** | Ruinen, Grenzgebiete | Neutrale Lager, handeln mit Rohaschglas | Handelspartner, Spione der Hohlen (Risiko 10 %) | Aschglas zu Marktpreis −20 % |
| **Ungebundene Veyl-Sippen** | Ränder des Dornwalds | Neutral; können für den Dornenbund oder den Spieler gewonnen werden (Meinung ≥ 40) | Diplomatischer Nebenpfad | `veyl_stalker` ohne Dornenbund-Beitritt |
| **Flüchtlingszüge** | Aus Provinzen mit Hohlenüberfällen | Ziehen in die nächste lebende Provinz | Aufnehmen (Bevölkerung, Spannung) oder abweisen (Meinung, Chronik) | Bevölkerung |
| **Ruinen und Gewölbe** | Überall, gehäuft in der Aurethwüste | Heldenquest-Schauplätze (RTS-Mini-Missionen) | RPG-Fortschritt | Gegenstände, Wissen, Ansprüche |

**Monsterhorte**: Jeder Hort hat einen **Bedrohungsradius** (Provinz plus Nachbarn). Raubzüge pro Jahr = `Hortstufe × (1 + Bevölkerungsdichte der Umgebung/1.000)`. Wird ein Hort zerstört, entsteht frühestens nach 10 Jahren
ein neuer in derselben Provinz.

### 14.4 Bosse

| Boss | ID | Ort | Spielrolle | Mechanische Signatur |
|---|---|---|---|---|
| **Der Fahle Vogt** | `boss_pale_reeve` | Hohlschrein von Vael Dorn (`hollow_shrine`), nordöstlich von Brennhollow | Mini-Boss der ersten Mission; lehrt die Bindungsmechanik und den Wert von Feuer und Fokusangriffen | Seelenentzug (heilt sich), Hohlruf (beschwört Knechte), stärker bei Nacht; Details in `campaign.md` |
| **Die Wurzelmutter** | `boss_root_mother` | Herzhain im Dornwald | Naturentität, weder gut noch böse; reagiert auf Abholzung und Hohlung. Kann Verbündete, Prüfung oder Feindin sein | Kampf über drei Areale des Hains; regeneriert aus Wurzelknoten, die zerstört oder (friedlich) geheilt werden können; friedliche Lösung möglich bei Meinung des Grünen Kreises ≥ 60 |
| **Der Aschdrache Vyrmathax** | `boss_ashen_dragon` | Glutweite | Territorialer Weltboss. Fliegt periodisch (alle 3–7 Jahre) über benachbarte Provinzen und verbrennt Ernten; kann bekämpft, bestochen (Tribut in Gold und Aschglas) oder gegen Feinde gelenkt werden | Flugphasen (nur Fernkampf und Magie wirksam), Landephasen (Nahkampf), Glutatem in Linien, Lavafelder; 40.000 HP auf Normal |
| **Der Hohle König** | `boss_hollow_king` | Festung Solmere, Aurethwüste | Endgegner: Varkhan Aurelith selbst, gefangen zwischen Leben und Tod. Seine Befreiung oder Vernichtung entscheidet das Zeitalter | Mehrphasige Belagerungs- und Bossschlacht: Aschenlegion schützt die Mauern, Sonnenhof (Licht/Feuer-Phase), Schleierphase (Ebenenwechsel), Kernphase im Thronsaal; Dialogoptionen je nach Hohlung und Sonnenhäresie |

**Bossregeln**: Bosse sind immun gegen Betäubung über 1 s, Moraleffekte und sofortige Tötungen
(*Seelengeleit* wirkt auf Bosse als 300 Arkanschaden). Jeder Boss hat **Phasen** mit klar angekündigten Wechseln
(Bildschirmrand verfärbt sich, Sprachzeile, Musikwechsel) und mindestens eine Mechanik, die Positionierung statt
reiner Stärke belohnt.

---

## 15. Dynamische Ereignisse und Kausalketten

### 15.1 Grundsatz: Simulation statt Skript

Ereignisse in ASHEN CROWNS sind **Zustandsänderungen gekoppelter Simulationsvariablen**. Es gibt kein Skript
„Dürre → Aufstand“. Es gibt eine Niederschlagsvariable, die eine Ernte bestimmt, die ein Lager füllt, das einen
Preis erzeugt, der eine Zufriedenheit verändert, die eine Unruhe speist. Ketten entstehen, weil die Systeme
verbunden sind – und sie können an jeder Stelle unterbrochen werden.

**Ereignisfenster** (die sichtbaren „Ereignisse“ mit Text und Entscheidungen) sind nur die **Interventionspunkte**:
Sie erscheinen, wenn eine Variable einen Schwellenwert überschreitet, und bieten Optionen, die wiederum Variablen
verändern. Sie lösen nichts aus, was die Simulation nicht ohnehin getan hätte – sie machen es sichtbar und
beeinflussbar.

### 15.2 Simulationsvariablen (Auswahl)

| Variable | Ebene | Bereich | Aktualisierung |
|---|---|---|---|
| Niederschlag | Provinz | 0–200 % des Klimamittels | Monatlich, Zufall mit Autokorrelation (Dürren dauern) |
| Temperatur | Provinz | Klimaabhängig | Monatlich |
| Bodenfruchtbarkeit | Provinz | 0–1 | Jährlich; sinkt bei Übernutzung, steigt bei Brache |
| Ernteertrag | Provinz | 0–150 % | Erntezeit: `Basis × Niederschlagsfaktor × Fruchtbarkeit × Arbeiterbesetzung × (1 − verbrannte Erde)` |
| Lagerbestände | Stadt | Einheiten | Täglich |
| Preise | Marktgebiet | Gold | Monatlich (8.6) |
| Zufriedenheit | Stadt/Schicht | 0–100 | Monatlich (9.4) |
| Unruhe | Provinz | 0–100 | Monatlich: `+ (50 − Ø Zufriedenheit)/10 + Spannungen/20 − Garnison/Bevölkerung × 100` |
| Hygiene | Stadt | 0–100 | Brunnen, Badehäuser, Bevölkerungsdichte, Bestattung |
| Seuchendruck | Provinz | 0–100 | Hygiene, Handelsverkehr aus Seuchengebieten, Hunger, Belagerungen |
| Schleierstärke | Provinz | 0–5 | Monatlich (12.7) |
| Kriegsmüdigkeit | Reich | 0–100 | Monatlich (7.6) |
| Legitimität | Herrscher | 0–100 | Ereignisgesteuert, jährlich |

### 15.3 Beispielkette: Dürre → Ernte → Preise → Unruhe → Aufstand → Krieg

Ausgangslage: Valemarks Herzogtum Nordmark, 6 Provinzen, Herzogin Mireille Corvane, Vasallin der Glutkrone.

| Monat | Simulation | Zahlen | Interventionspunkte (Ereignisfenster) |
|---|---|---|---|
| März | Niederschlag fällt auf 45 % (Autokorrelation: 70 % Wahrscheinlichkeit, dass der April ebenfalls trocken ist) | Vorzeichen: „Die Brunnen in Nordmark sinken“ | Optional: Bewässerungsgräben bauen (Gold, Arbeiter) |
| Juni | Dritter trockener Monat in Folge → Zustand „Dürre“ | Prognose Ernte 55 % | **„Dürre in Nordmark“**: Kornspeicher öffnen / Getreide importieren / Ernteritual (Wachstumsmagie) / nichts tun |
| September | Ernte 52 % | Getreideangebot im Marktgebiet Valemark −30 % | – |
| Oktober | Preis Getreide: `2 × (1,0/0,7)^0,7 = 2,57`, Brot 3,85 | Leibeigene und Freibauern: Nahrung nur zu 60 % gedeckt → G sinkt, Z −15 | **„Brotpreis steigt“**: Preisdeckel (Krone zahlt Differenz), Export verbieten (Meinung Saelith −10), Steuern senken |
| Dezember | Wintervorräte erschöpft in 2 von 6 Provinzen | Nahrungskonto leer → Hunger: Z −40 zusätzlich; Sterblichkeit ×1,5 | **„Hunger“**: Adelsvorräte beschlagnahmen (Adelsloyalität −15) / Kirche um Spenden bitten (Frömmigkeit −100) |
| Januar | Unruhe +6/Monat; Spannung Leibeigene ↔ Adel 78 | Unruhen: Lager in Nordmark-Stadt geplündert (in der RTS-Stadt sichtbar: brennende Speicher) | **„Brotaufruhr“**: Garnison einsetzen (Unruhe −20, Spannung +10, Tote) / Forderungen erfüllen |
| Februar | Spannung ≥ 90 → **Aufstand**; Rebellenarmee 400 Bauern | Mireille kann ihn nicht niederschlagen ohne Hilfe der Krone | **„Bauernaufstand in Nordmark“**: Krone schickt Truppen / Mireille muss es allein schaffen |
| April | Mireille bittet König Aldric III. um Hilfe; KI-Bewertung des Königs: Hilfe verweigert (Meinung zu Mireille −20, eigene Kriegsmüdigkeit 35) | Meinung Mireille → Krone −25 (Bündnisaufruf abgelehnt) | – |
| Juni | Mireille schlägt den Aufstand mit Söldnern nieder (Kredit bei den Saelith-Banken) und gründet die Fraktion „Anderer Thronerbe“ | Fraktionsstärke 1,25 × Kronstärke | **Ultimatum an die Krone** → Bürgerkrieg |

Jeder Schritt hätte anders verlaufen können: Bewässerung im März, Importe im Juni, ein Ernteritual oder eine
kluge Preispolitik im Oktober. Für den Spieler ist die Kette zugleich Bedrohung (eigene Provinzen) und
Gelegenheit (der Bürgerkrieg schwächt Rivalen; wer Mireille hilft, gewinnt eine Verbündete).

### 15.4 Weitere Kettenmuster

| Kette | Verlauf | Unterbrechungspunkte |
|---|---|---|
| **Seuche** | Belagerung/Handel → Seuchendruck > 60 → Ausbruch (Aschenfieber, Grauhusten) → Sterblichkeit, Arbeitskräftemangel → Produktion sinkt → Preise steigen → Flucht aus Städten → Ausbreitung entlang der Handelsrouten | Quarantäne (Routen sperren), Hospitäler, Heilmittel, Bestattung |
| **Schleier** | Aschglasabbau → Schleierstärke steigt → Ascheregen, nächtliche Hohle → Flüchtlinge → Überfüllung, Spannung Einheimische ↔ Flüchtlinge → Unruhe | Abbau begrenzen, Wachfeuer, Tempel, Flüchtlingssiedlungen |
| **Erbfolge** | Herrscher erkrankt → Thronansprüche werden laut → Adelshäuser wählen Kandidaten → Fraktionen → Erbfolgekrieg nach Tod | Erbe früh bestimmen, Ehen, Ämterverteilung, Komplotte |
| **Handel** | Krieg in Saelith-Gewässern → Seerouten unsicher → Gewürz-/Seidenpreise steigen → Adel unzufrieden (Luxusbedarf) → Adelsloyalität sinkt | Landrouten, Ersatzluxus, Flotte |
| **Drache** | Vyrmathax' Flug → Ernte in 3 Provinzen −80 % → Hunger → Tribut an den Drachen oder Feldzug | Tribut, Fernkampfstellungen, Glutschutz-Ritual |
| **Wald** | Holzbedarf (Flotten, Belagerung) → Abholzung > 30 % am Dornwaldrand → Grüner Kreis verärgert → Wurzelmutter erwacht → Dornenbund erklärt Krieg | Förstereien, Holzimport, Verträge mit dem Dornenbund |

### 15.5 Vorzeichen-System

Jede Variable, die einen kritischen Schwellenwert erreichen kann, erzeugt bei 70 % des Schwellenwerts ein
**Vorzeichen**: ein sichtbares Signal auf beiden Ebenen (Weltkarte: Symbol an der Provinz; RTS: optische Hinweise
wie welke Felder, Ratten in Gassen, Asche im Wind, Bettler am Markt) und einen Eintrag im „Berater“-Panel mit
Prognose („Bei gleichbleibender Lage: Hunger in 2 Monaten“).

### 15.6 Ereignisarchitektur (Daten)

```ron
EventTrigger(
    id: "drought_province",
    scope: Province,
    condition: All([
        VarBelow("rainfall_ratio", 0.6),
        ConsecutiveMonths("rainfall_ratio_below_0_6", 3),
        Season([Spring, Summer]),
    ]),
    mtth_months: 1,                    // Mittlere Zeit bis zum Auslösen, sobald die Bedingung gilt
    cooldown_months: 12,
    window: "evt_drought",             // Ereignisfenster mit Optionen
    sets_state: Some("drought"),       // Simulationszustand, den andere Systeme lesen
    chronicle_weight: 30,
)
```

Optionen eines Ereignisfensters verändern ausschließlich Variablen (Gold, Lager, Gesetze, Meinungen,
Zustände). Damit bleibt jedes Ereignis Teil der Simulation und jede Folge erklärbar.

### 15.7 Ereignisdichte

Pro Spieljahr erscheinen im Mittel 4–8 Ereignisfenster für den Spieler (einstellbar: Wenige / Normal / Viele).
Kleinere Veränderungen werden nur als Benachrichtigung gemeldet. Ereignisfenster pausieren die Weltkarte
optional (Einstellung, Standard: an bei wichtigen Ereignissen).

---

## 16. Endgame

Das Endgame beginnt nicht zu einem festen Datum, sondern wenn einer der folgenden Zustände eintritt:
ein Reich erreicht Großmachtstatus, der globale Schleierdruck überschreitet 60, oder das Jahr 1000 n.F. beginnt.
Ziel ist, das Problem vieler Strategiespiele zu vermeiden, in denen die letzte Stunde nur Aufräumarbeit ist.

### 16.1 Großmacht-Reaktion

**Großmachtstatus** erhält ein Reich, das **30 % aller Provinzen** kontrolliert oder dessen Stärke das **Doppelte**
des zweitstärksten Reiches beträgt.

| Folge | Wirkung |
|---|---|
| Pakt der Kronen | Alle Reiche mit Bedrohung ≥ 40 gegenüber der Großmacht bilden automatisch eine Koalition (unabhängig von Meinung); Koalition erhält +10 % Kampfkraft und gemeinsame Sicht |
| Innere Last | Verwaltungslimit wird um 25 % gesenkt; Korruption +5 Prozentpunkte; Adelshäuser fordern Rechte (Loyalität −10, solange keine neuen Rechte gewährt werden) |
| Separatismus | Provinzen anderer Kultur haben Unabhängigkeitsbestrebungen: Unruhe +1/Monat |
| Handelsboykott | Saelith-Pakt erhebt Zölle von 20 % auf Waren der Großmacht, sofern Meinung < 20 |
| Kirchliche Wachsamkeit | Bei Hohlungsstufe ≥ 2 der Großmacht: automatischer Heiliger Krieg des Erzflammenwahrers |

Ein Spieler kann eine Großmacht friedlich halten: Rechte verleihen, Vasallen statt direkter Herrschaft,
Ruf des Friedens, Handelsabkommen. Großmacht zu sein ist eine Prüfung, kein Siegeszug.

### 16.2 Uralte Bedrohungen

Der **globale Schleierdruck** (0–100) steigt um +1 pro 5 Jahre (Grundwert), +0,5 je Provinz mit Schleierstärke ≥ 4,
+5 je Leermagie-Erzritual, und sinkt um −10 je wiederhergestelltem Götterschrein sowie −5 je zerstörtem
Hohlschrein mit Bosswächter.

| Schwelle | Bedrohung | Verlauf |
|---|---|---|
| 40 | **Ruf aus Solmere** | Die Aschenlegion verlässt ihre Patrouillenrouten und rückt in die Aschmark und Aurethwüste-Randprovinzen vor; Sonnenhäresie +20 Eifer |
| 60 | **Das Erwachen des Hohlen Königs** | `boss_hollow_king` wird zur aktiven Macht: Jährlich marschiert eine Legionsarmee (Stärke 300 + 10 × Jahr seit Erwachen) gegen das stärkste lebende Reich; Hohlschreine entstehen doppelt so oft |
| 70 | **Vyrmathax' Zorn** | Der Aschdrache verlässt die Glutweite häufiger (jährlich), seine Flüge zielen auf Aschglasminen (er frisst Aschglas) |
| 80 | **Das Welken** | Die Wurzelmutter schließt den Dornwald: Alle Handelsrouten durch den Dornwald brechen ab; Abholzung löst sofort Angriffe aus, sofern der Grüne Kreis nicht verbündet ist |
| 100 | **Der Schleierbruch** | Jedes Jahr steigt in 3 zufälligen Provinzen die Schleierstärke um 1; ohne Gegenmaßnahmen fällt Eldraeth innerhalb von etwa 30 Jahren ins Hohle Tief (Niederlage aller lebenden Reiche, außer bei Hohlungspfad-Sieg) |

Gegenmaßnahmen: Götterschreine wiederherstellen, Hohlschreine vernichten, Erzrituale der Glut-, Ordnungs- und
Übergangsschule, Feldzug nach Solmere.

### 16.3 Gemeinsame Bedrohung als Diplomatie-Motor

Ab dem Erwachen des Hohlen Königs gilt: Reiche, die gegen die Legion kämpfen, erhalten untereinander +2 Meinung
pro Monat (max. +40) und können den **„Bund der Letzten Flamme“** gründen (Koalition gegen die Hohlen mit
gemeinsamer Kriegskasse: Mitglieder zahlen 5 % ihres Einkommens). Wer im Endgame Kriege gegen andere Lebende führt,
erhält die Erinnerung „Verrat am Licht“ (−30 Meinung bei allen Bundmitgliedern, 50 Jahre).

### 16.4 Siegbedingungen

Siegbedingungen werden zu Beginn einer Partie gewählt (Sandbox) oder ergeben sich aus der Kampagne. Mehrere
können gleichzeitig aktiv sein; die erste erfüllte beendet die Partie (Weiterspielen danach möglich).

| Sieg | Bedingung | Friedlich möglich |
|---|---|---|
| **Krone von Eldraeth** (Herrschaft) | 60 % aller Provinzen direkt oder durch Vasallen kontrollieren und den Titel „Hochkönig“ (Rang 5) halten | Nein |
| **Der Hohle König fällt** (Erlösung) | `boss_hollow_king` in Solmere besiegen und den Hohlschrein in Solmere zerstören | Militärisch, aber im Bund möglich |
| **Siegel des Schleiers** | Alle fünf Götterschreine wiederherstellen und das Erzritual „Siegel“ (gemeinsam von Glut-, Ordnungs- und Übergangsschule Grad IV) in Solmere wirken; globaler Schleierdruck < 20 | Ja (mit Verbündeten) |
| **Handelsherrschaft** | 50 % des gesamten Handelsvolumens Eldraeths über eigene Routen und Kontore, Reichsschatz ≥ 100.000 Gold, 10 Jahre gehalten | Ja |
| **Glaubenssieg** | Eigene Religion in 65 % aller Provinzen, alle heiligen Stätten der eigenen Religion kontrolliert | Ja |
| **Dynastische Legende** | Dynastieprestige ≥ 25.000 und Mitglieder der eigenen Dynastie auf den Thronen von mindestens 4 der 6 Fraktionen (Hohle ausgenommen; Aschenlegion zählt nicht) | Ja |
| **Die Hohle Krone** (Hohlungspfad) | Hohlungsstufe 5, Solmere kontrollieren und den Hohlen König entweder ersetzen (Duell) oder sich ihm unterwerfen (alternatives dunkles Ende) | Nein |
| **Überleben** | Nur in Survival und bestimmten Herausforderungen: festgelegte Zeitspanne überstehen | – |

---

## 17. Das Chroniksystem

### 17.1 Zweck

Die Chronik ist das **Gedächtnis der Welt** und das Erzählmedium des Spiels. Sie zeichnet alle bedeutsamen
Ereignisse auf, verdichtet sie zu lesbarer Geschichte im Stil eines Chronisten (Stimme: `narrator`) und macht
jede Partie zu einer eigenen Saga. Sie ist im Spiel jederzeit lesbar (Taste `C`), wird beim Generationswechsel
zusammengefasst und kann am Ende als Text- oder illustrierte HTML-Datei exportiert werden.

### 17.2 Datenmodell

Jedes chronikrelevante Ereignis wird als `ChronicleEvent` gespeichert:

```ron
ChronicleEvent(
    id: 48213,                               // fortlaufend, eindeutig pro Partie
    date: (year: 897, month: 10, day: 3),    // n.F.
    kind: Battle,                            // Battle, Siege, Death, Birth, Marriage, Succession, Coronation,
                                             // Plot, War, Peace, Disaster, Plague, Famine, Revolt, Founding,
                                             // Construction, Discovery, Religion, Hollowing, BossDefeated, Legend
    actors: [
        Actor(id: "char_isolde_raventhorn", role: Commander, side: A),
        Actor(id: "boss_pale_reeve", role: Commander, side: B),
    ],
    location: Location(province: "ashmarch_vael_dorn", site: Some("hollow_shrine")),
    magnitude: 72,                           // 0–100 Bedeutsamkeit
    causes: [48190, 48201],                  // IDs der auslösenden Ereignisse (Kausalkette)
    numbers: { "strength_a": 41, "strength_b": 96, "losses_a": 13, "losses_b": 96 },
    outcome: Victory(side: A),
    tags: ["first_victory", "hollowed", "night", "boss"],
    template: "battle_boss_victory",
    epithet_candidates: ["die Unbeugsame", "Vogtbezwingerin"],
    visibility: Public,                      // Public, Secret (erst bei Aufdeckung in der Chronik), Rumor
)
```

**Bedeutsamkeit** (`magnitude`) wird berechnet aus: Rang der Beteiligten, Größenordnung (Tote, Gold, Provinzen),
Seltenheit (erster Sieg gegen einen Gegnertyp, erste Krönung), Verkettung (Ereignisse mit vielen Folgeereignissen
werden nachträglich aufgewertet: +5 je Folgeereignis, max. +30).

### 17.3 Erzeugung der Texte

1. **Filtern**: Einträge mit magnitude ≥ 25 erscheinen in der Jahreschronik, ≥ 50 in der Herrscherchronik,
   ≥ 75 in der Zeitalterchronik.
2. **Verdichten**: Gleichartige Ereignisse werden zusammengefasst („Im Sommer 905 fielen drei Dörfer der Aschmark
   den Hohlen zum Opfer“).
3. **Verknüpfen**: Über `causes` entstehen Kausalsätze („Weil die Ernte verdorrte, …“).
4. **Formulieren**: Textvorlagen mit Varianten (mindestens 6 je Ereignistyp), Satzbausteinen für Ton
   (nüchtern, ehrfürchtig, bitter – abhängig von Ergebnis und Meinung des Chronisten) und grammatischer
   Kongruenz (Genus, Numerus, Kasus) für deutsche und englische Ausgabe.
5. **Perspektive**: Der Chronist ist eine Figur der Welt. Standardmäßig schreibt er aus Sicht des Hofes des
   Spielers; ein Wechsel der Dynastie oder eine Eroberung kann den Chronisten wechseln – und damit den Ton.

### 17.4 Beinamen

Beim Tod eines Herrschers wird aus den `epithet_candidates` seiner wichtigsten Einträge ein Beiname gewählt
(gewichtete Häufigkeit, Bedeutsamkeit, Eigenschaften). Beispiele: „die Unbeugsame“ (viele Verteidigungssiege),
„der Kornherr“ (Hungersnöte abgewendet), „die Aschgezeichnete“ (Hohlung ≥ 40), „der Brudermörder“ (Sippenmord),
„die Brückenbauerin“ (Bündnisse), „der Kurze“ (Regierungszeit < 2 Jahre).

### 17.5 Beispiel-Einträge

> **Im Jahr 897 n.F., im Monat der Fahlen Ernte**, rief Isolde Raventhorn, Wächterin der Aschmark, die letzten
> dreißig Seelen von Brennhollow in den Hof des Bergfrieds. Bruder Hal Venn segnete die Klingen, und in jener Nacht
> kamen die Hohlen zum ersten Mal bis an die Palisade. Sie kamen nicht ein zweites Mal so weit.

> **897 n.F.** Noch vor dem ersten Schnee zog die Wächterin mit einundvierzig Bewaffneten gegen den
> Hohlschrein von Vael Dorn. Der Fahle Vogt, der dort seit Menschengedenken die Toten hütete, erhob sechsundneunzig
> Knechte gegen sie. Dreizehn Männer und Frauen aus Brennhollow kehrten nicht zurück. Der Vogt kehrte nie zurück.
> Man nennt sie seither die Unbeugsame.

> **912 n.F.** Weil der Regen über Nordmark drei Monde lang ausblieb, verdorrte das Korn, und der Preis für Brot
> stieg auf das Doppelte. König Aldric III. schloss seine Speicher. Herzogin Mireille Corvane öffnete die ihren –
> und mit ihnen die Herzen der Bauern. Im Winter darauf nannten sie die Bauern von Nordmark ihre Königin.

> **931 n.F.** Kassian Vale, der Spionagemeister der Krone, starb an einem Becher Wein, den niemand eingeschenkt
> haben wollte. Die Chronik verzeichnet seinen Tod; den Namen des Mörders verzeichnet sie nicht. Noch nicht.

> **944 n.F.** In Kaldris weigerte sich der Erzflammenwahrer, die Ehe zwischen dem Haus Raventhorn und dem Haus
> der Saelith-Varo zu segnen. Ophira Saelith-Varo lachte darüber, heißt es, und kaufte am selben Tag die Schuldscheine
> des Flammendoms.

> **967 n.F.** Der Himmel über der Glutweite brannte drei Tage lang. Als Vyrmathax zurück in seine Krater zog,
> lagen die Kornfelder von vier Grafschaften in Asche. Man zahlte ihm im folgenden Jahr, was er verlangte.

### 17.6 Chronik und Spielmechanik

- **Legenden**: Einträge mit magnitude ≥ 90 werden zu Legenden, die als Weltwissen weiterleben: Orte erhalten
  Namen („Vogtsfeld“), Gegenstände werden zu Artefakten, Lieder (+5 Moral für Einheiten der Region bei
  Verteidigung).
- **Erinnerung der KI**: Das Meinungsmodell liest Chronikeinträge (z. B. „Hinrichtung des Vaters“), wodurch auch
  Generationen später Groll entstehen kann.
- **Ahnenbuch** (Meta-Fortschritt, optional): Abgeschlossene Chroniken werden im Ahnenbuch gesammelt; sie schalten
  kosmetische Wappen, Startdynastien und historische Szenarien frei, aber keine spielmechanischen Vorteile.

---

## 18. Wiederspielwert

| Element | Variation |
|---|---|
| Weltgenerierung | Feste Kanon-Welt Eldraeth (Kampagne, Historische Szenarien) oder generierte Varianten mit gleicher Regionsstruktur, aber neuer Provinzaufteilung, Flüssen, Ressourcen und Schleierverteilung (Sandbox) |
| Startpositionen | Jede Fraktion außer den Hohlen und der Aschenlegion spielbar; Startrang von Grenzherr bis König |
| Dynastien | Vordefinierte Häuser (Raventhorn, Corvane, das Königshaus Valemarks, Eisenhand, Saelith-Varo u. a.) oder eigene Dynastie mit Wappeneditor |
| Simulation | Deterministischer Seed, aber Wetter, Erbfolgen, Charaktereigenschaften, Komplotte und KI-Ziele ergeben jedes Mal andere Ketten |
| Entwicklungspfade | Feldherr, Baumeister, Händler, Glaubensreich, Ränkeschmied, Hohlungspfad – jeder mit eigenem Spielgefühl und eigener Siegbedingung |
| Kampagne | Mindestens 12 Entscheidungspunkte, mehrere Enden, alternative Missionen (siehe `campaign.md`) |
| Herausforderungen | Wöchentliche Herausforderung mit festem Seed und Bestenliste (offline-fähig, Online-Bestenliste optional) |
| Erfolge | 80 Erfolge, davon die Hälfte für ungewöhnliche Geschichten („Eine Leibeigene auf dem Thron“, „Der Drache, der Steuern zahlte“) |
| Modding | Datengetriebene Inhalte (RON), Szenario-Editor, Ereignis- und Chronikvorlagen erweiterbar |

---

## 19. Spielmodi

| Modus | Ebenen | Ziel | Dauer | Besonderheiten |
|---|---|---|---|---|
| **Skirmish** | Nur RTS | Gegner vernichten oder Siegpunkte halten | 20–60 min | Klassisch (4 Ressourcen) oder Erweitert (volle Wirtschaft); 1–8 Spieler-/KI-Plätze; Karten aus allen Regionen; Hohle als neutrale Nachtbedrohung zuschaltbar |
| **Grand Campaign** | Beide | Story-Kampagne „Das Zeitalter der Gefallenen“ in 7 Zeitaltern | 40–60 h | Weltkarte mit Missionsknoten; Story-Missionen sind handgebaute RTS-Karten, dazwischen freies Spiel auf der Weltkarte; Entscheidungen verändern Folgezeitalter (siehe `campaign.md`) |
| **Sandbox** | Beide | Frei gewählte Siegbedingungen (16.4) | offen | Alle Einstellungen frei: Startjahr (897 n.F. Standard), Fraktion, Schleierdruck, KI-Aggressivität, Permadeath, Ereignisdichte, Siegbedingungen, Weltgenerierung |
| **Challenge** | Beide oder RTS | Vorgegebene Aufgabe unter Einschränkungen | 30–120 min | Beispiele: „Ein Winter in Brennhollow“ (ohne Kaserne überleben), „Kein Blut“ (Handelssieg ohne Krieg), „Die Leibeigene“ (Spiel als machtloses Kadettenhaus); feste Seeds, Bestenlisten |
| **Survival** | RTS mit Weltkarten-Elementen | Möglichst lange gegen eskalierende Hohlenwellen bestehen | 30 min – unbegrenzt | Jede Nacht stärker; zwischen den Nächten Wirtschaft und Forschung; Bosse in Nacht 10, 20, 30 (Fahler Vogt, Wurzelmutter-Abklatsch „Verdorrte Mutter“ als Hohlenvariante, Aschenlegion-Banner); Punktwertung nach überlebten Nächten und Bevölkerung |
| **Custom Scenarios** | Beide | Vom Spieler definiert | – | Szenario-Editor: Karten (RTS) und Weltkarten-Startzustände, Auslöser, Ziele, Dialoge, Chronik-Vorlagen; Teilen über Datei-Export |
| **Historical Scenarios** | Beide | Historische Lage Eldraeths nachspielen und verändern | 3–15 h | „Der Aschenfall“ (Jahr 0: Flucht aus Solmere, RTS-Mission mit anschließendem Neuanfang), „Die Grenzladys“ (612 n.F.: Erlass der gleichberechtigten Erbfolge im Bürgerkrieg), „Der Schwur von Skarnheim“ (Einigung der Clans), „Die Gewürzkriege“ (Saelith-Pakt gegen Valemarks Flotte), „Der lange Winter“ (Hungersnot und Aufstände in Valemark) |

**Mehrspieler**: Skirmish (bis 8 Spieler, Lockstep-Netzcode auf Basis der deterministischen Simulation) und
Sandbox-Koop (2–4 Spieler auf der Weltkarte, Schlachten werden vom beteiligten Spieler gespielt; unbeteiligte
Spieler können als Verbündete Truppen übernehmen).

---

## 20. UI/UX-Konzept

### 20.1 Gestaltungsprinzipien

1. **Diegetische Rahmung**: UI-Elemente sind als Gegenstände der Welt gestaltet (Pergament, Eisenbeschläge,
   Glut), aber nie auf Kosten der Lesbarkeit.
2. **Informationstiefe in drei Stufen**: sichtbar (Icon/Zahl) → Tooltip (Aufschlüsselung) → Enzyklopädie
   (Taste `F1`, vollständige Erklärung inklusive Formel).
3. **Keine Information ohne Handlung**: Jede Warnung bietet eine Sprungmöglichkeit zur Ursache (Klick auf
   „Hunger in Brennhollow“ öffnet den Stadtbericht mit Nahrungsbilanz).
4. **Pixelgenaue Skalierung**: UI-Skalierung in ganzzahligen Faktoren (1×–4×) plus Textskalierung unabhängig
   (80–150 %).

### 20.2 Hauptmenü

| Eintrag | Inhalt |
|---|---|
| Fortsetzen | Letzter Spielstand, mit Chronik-Einzeiler („Jahr 903, Isolde die Unbeugsame regiert Brennhollow“) |
| Grand Campaign | Neues Spiel / Laden / Zeitalterauswahl (bereits erreichte Zeitalter) |
| Sandbox | Einrichtungsbildschirm (Fraktion, Dynastie, Welt, Siegbedingungen, Regeln) |
| Skirmish | Lobby (Einzel- und Mehrspieler) |
| Herausforderungen & Survival | Wöchentliche Herausforderung, Liste, Bestenlisten |
| Szenarien | Historische Szenarien, eigene Szenarien, Editor |
| Ahnenbuch | Abgeschlossene Chroniken, Erfolge, freigeschaltete Wappen |
| Einstellungen | Grafik, Audio, Steuerung (Tastenbelegung), Barrierefreiheit, Spiel (Ereignisdichte, Pausenverhalten) |
| Beenden | – |

Hintergrund: Animierte Pixel-Art-Szene (Brennhollow in der Dämmerung, Ascheflocken, flackernde Wachfeuer);
Musik `main_menu`. Der Hintergrund verändert sich mit dem Kampagnenfortschritt (spätere Zeitalter zeigen
Kaldris, Solmere, den Dornwald).

### 20.3 Ingame-HUD (Echtzeitwelt)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ [Gold 200] [Holz 150] [Stein 50] [Nahrung 100] [Bev. 11/15]  ☀ Tag 05:12  ⚑ Ziele │  ← Ressourcenleiste
│                                                                    [Benachr.]│
│                                                                              │
│                         S P I E L F E L D                                    │
│                                                                              │
│ [Heldenportrait(s) mit HP/Glut]                                              │
├────────────┬─────────────────────────────────────────┬───────────────────────┤
│ Minikarte  │ Auswahlpanel: Portrait, Werte, Gruppe    │ Befehlsraster 4×3     │
│ (Tag/Nacht,│ (bis 24 Einheiten), Statuseffekte,       │ (Bau, Ausbildung,     │
│ Alarme)    │ Ausbildungswarteschlange                 │ Fähigkeiten, Haltung) │
└────────────┴─────────────────────────────────────────┴───────────────────────┘
```

- **Ressourcenleiste**: Bestand, Tendenz (Pfeil) und Einkommen pro Minute im Tooltip.
- **Tageszeitanzeige**: Sonne/Mond mit Restzeit bis zur nächsten Phase; 60 s vor Einbruch der Nacht pulsiert sie
  und `narrator`/Berater kündigt an.
- **Ziele** (`J`): Haupt-, Neben- und versteckte Ziele mit Fortschritt.
- **Benachrichtigungen**: Stapel rechts, farbcodiert (Rot: Angriff, Gelb: Wirtschaft, Blau: Politik,
  Grün: Fortschritt, Fahlgrün: Hohlung). Klick springt zum Ort; `Leertaste` springt zur letzten Meldung.
- **Befehlsraster**: Feste Positionen (Raster-Hotkeys), damit Muskelgedächtnis entsteht.
- **Gesundheitsbalken**: Nur bei Beschädigung oder `Alt` gedrückt; Moral als dünner Balken darunter (Farbe des
  Moralzustands); Ausdauer als Punktreihe bei Erschöpfung.

### 20.4 Weltkarte

- **Kartenmodi** (Tasten `F2`–`F9`): Politisch, Gelände, Wirtschaft (Ressourcen und Handelsrouten), Religion,
  Dynastie (Herrscherhäuser), Schleier (Schleierstärke und Hohlung), Meinung (zum ausgewählten Herrscher),
  Unruhe.
- **Obere Leiste**: Gold mit Monatsbilanz, Prestige, Frömmigkeit, Legitimität, Reichshohlung (nur ab Stufe 1
  sichtbar), Datum und Geschwindigkeit (`1`–`5`, `Leertaste` = Pause).
- **Linke Leiste**: Herrscherportrait (Klick: Charakterbildschirm), Rat (Ämter), Dynastie, Hof, Intrigen,
  Diplomatie, Militär, Wirtschaft, Religion, Forschung/Magie, Chronik.
- **Provinzpanel**: Bevölkerung nach Schichten, Zufriedenheit, Gebäudeübersicht, Lager, Statthalter, Schaltfläche
  „Stadt betreten“.
- **Armeepanel**: Regimenter mit Figurenanzahl, Helden, Versorgung, Moral, Bewegungsweg mit Ankunftsdatum.
- **Berater-Panel**: Prognosen aus dem Vorzeichen-System (15.5).

### 20.5 Lesbarkeit der Pixel-Art

| Regel | Umsetzung |
|---|---|
| Silhouette vor Detail | Jede Einheitenklasse hat eine eindeutige Silhouette (Speer überragt Kopf, Ritter breit und hoch, Hohle gebeugt) |
| Fraktionsfarbe | 20–30 % der Sprite-Fläche in Fraktionsfarbe (Wappenrock, Banner, Schild) |
| Kontrastzonen | Spielfiguren haben 1 px dunklen Umriss; bei Nacht wird der Umriss in Fraktionsfarbe leicht aufgehellt |
| Zustände | Brennend (Flammenpartikel), vergiftet/hohlberührt (fahlgrüne Partikel), Schild (goldener Rand), betäubt (kreisende Funken) |
| Gebäudezustand | Beschädigung in drei sichtbaren Stufen, Feuer, Bauzustand als Gerüst |
| Projektile | Mindestens 4 px lang, mit Spur bei Feuer- und Leerschaden |
| Bildschirmgewimmel | Ab 120 sichtbaren Einheiten werden Nebeneffekte (Staub, Asche) reduziert |

---

## 21. Tastenkürzel

Alle Tasten sind frei belegbar. Auf macOS wird `Cmd` statt `Strg` verwendet; Raster-Hotkeys orientieren sich am
physischen Layout (funktionieren auch auf deutscher QWERTZ-Tastatur an derselben Position).

| Bereich | Taste | Funktion |
|---|---|---|
| Allgemein | `Esc` | Menü / Abbrechen |
| Allgemein | `F1` | Enzyklopädie (kontextsensitiv) |
| Allgemein | `Cmd`+`S` / `Cmd`+`L` | Schnellspeichern / Schnellladen (nicht im Ironman) |
| Allgemein | `Pause` oder `P` (Weltkarte) / `F11` (RTS) | Pause |
| Allgemein | `+` / `−` | Spielgeschwindigkeit |
| Allgemein | `C` | Chronik |
| Allgemein | `Tab` | Zwischen Weltkarte und Stadt wechseln |
| Kamera | `W` `A` `S` `D` (RTS: nur bei aktivierter Option) / Pfeiltasten | Kamera bewegen |
| Kamera | Mausrad / Pinch | Zoom |
| Kamera | `Pos1` | Zum Hauptgebäude bzw. zur Hauptstadt |
| Kamera | `F5`–`F8` (RTS) | Kamerapositionen speichern (`Cmd`) und abrufen |
| Kamera | `Leertaste` (RTS) | Zur letzten Benachrichtigung |
| Auswahl | `1`–`0` | Kontrollgruppe abrufen (zweimal: zentrieren) |
| Auswahl | `Cmd`+`1`–`0` | Kontrollgruppe zuweisen |
| Auswahl | `Shift`+`1`–`0` | Zur Kontrollgruppe hinzufügen |
| Auswahl | `.` | Nächster untätiger Arbeiter |
| Auswahl | `^` (QWERTZ) bzw. `` ` `` (US-Layout), zweimal drücken | Held auswählen, beim zweiten Druck zentrieren |
| Auswahl | `Cmd`+`A` | Alle Militäreinheiten auf dem Bildschirm |
| Befehle | `A` | Angriffsbewegung |
| Befehle | `S` | Stopp |
| Befehle | `H` | Position halten |
| Befehle | `P` (RTS) | Patrouille |
| Befehle | `R` | Rückzug zum Sammelpunkt |
| Befehle | `G` | Formation wechseln (Linie → Keil → Kolonne → Quadrat → Lockere Ordnung) |
| Befehle | `V` | Haltung wechseln |
| Befehle | `F` | Spezialhaltung (Speerwall, Salvenfeuer) |
| Fähigkeiten | `Q` `W` `E` `T` | Heldenfähigkeiten (bei Isolde: Glutspalter, Sammelbanner, Aschenschild, Letzte Glut) |
| Bau | `B` | Baumenü (Arbeiter ausgewählt) |
| Bau | `B` → `F` / `B` / `T` / `K` | Gehöft / Kaserne / Wachturm / Bergfried-Ausbau |
| Bau | `Shift` beim Platzieren | Mehrere Gebäude platzieren |
| Bau | `,` / `Cmd`+Mausrad | Gebäude drehen |
| Ausbildung | `Q` `W` `E` `R` (Gebäude ausgewählt) | Einheiten im Befehlsraster |
| Ausbildung | `Shift`+Klick | 5 Einheiten in Auftrag |
| Ausbildung | Rechtsklick (Gebäude ausgewählt) | Sammelpunkt setzen |
| Weltkarte | `Leertaste` | Pause / Fortsetzen |
| Weltkarte | `1`–`5` | Geschwindigkeit |
| Weltkarte | `F2`–`F9` | Kartenmodi |
| Weltkarte | `Enter` | Ausgewählte Stadt betreten |
| Weltkarte | `M` | Militärübersicht |
| Weltkarte | `I` | Intrigen |
| Weltkarte | `O` | Diplomatie |
| Weltkarte | `K` | Rat und Ämter |
| Weltkarte | `L` | Dynastie (Stammbaum) |
| Schlacht | `F10` | Schlachtmenü (Rückzug, Kapitulation anbieten) |
| Anzeige | `Alt` (halten) | Lebens- und Moralbalken aller Einheiten |
| Anzeige | `Cmd`+`G` | Raster anzeigen (Bauen) |
| Anzeige | `Ctrl`+`H` | Hohlungs-/Bindungsradien anzeigen |

---

## 22. Schwierigkeitsgrade

| Stufe | Name | KI-Verhalten | Wirtschaft | Hohle | Sonstiges |
|---|---|---|---|---|---|
| 1 | **Pilger** | Passiv, verzeiht Fehler, greift selten an, verschenkt keine Bündnisse | Spieler +25 % Ertrag | −40 % Stärke, Nachtangriffe nur angekündigt | Taktische Pause mit Befehlen, Ereignisse mit Empfehlung |
| 2 | **Vasall** | Defensiv bis ausgeglichen, einfache Taktiken | Spieler +10 % | −20 % | Hinweise aktiv |
| 3 | **Lehnsherr** (Standard) | Volle Taktik, Flankierung, Fokusfeuer, nutzt Diplomatie und Intrigen | gleich | 100 % | Referenz für das Balancing; keine KI-Boni |
| 4 | **Hochkönig** | Wie Lehnsherr, plus Koordination zwischen KI-Reichen, aggressive Expansion | KI +15 % Ertrag | +25 % | Schnellere Bedrohungsbildung (+20 %) |
| 5 | **Aschenfall** | Optimale Taktik, nutzt jede Schwäche, Komplotte gegen den Spieler | KI +30 % | +50 %, zusätzliche Nachtwelle | Permadeath mindestens „Hart“, kein manuelles Speichern (Ironman) |
| – | **Benutzerdefiniert** | Jeder Regler einzeln: KI-Aggressivität, KI-Ertrag, Hohlenstärke, Schleierdruckrate, Ereignisdichte, Permadeath, Auto-Resolve-Vorteil | | | Erfolge nur mit Werten mindestens auf Stufe 3 |

**Kampagnenspezifisch**: In Story-Missionen skaliert die Schwierigkeit zusätzlich Wellengrößen, Bosswerte und
Zeitlimits (siehe Mission 1 in `campaign.md`). Der Schwierigkeitsgrad kann in der Kampagne zwischen Missionen
gesenkt, aber nur zu Beginn eines Zeitalters erhöht werden.

**Adaptive Hilfe** (abschaltbar, nur Pilger und Vasall): Nach zwei Niederlagen in derselben Mission bietet das
Spiel an, die nächste Nachtwelle um 20 % abzuschwächen.

---

## 23. Barrierefreiheit und Lesbarkeit

| Bereich | Maßnahme |
|---|---|
| Farbe | Farbfehlsichtigkeits-Modi (Protanopie, Deuteranopie, Tritanopie) mit angepassten Fraktionsfarben; alle Zustände zusätzlich über Form/Symbol kodiert |
| Text | Skalierung 80–150 %, Schriftwahl (Pixel-Schrift oder gut lesbare Serifenlose), Untertitel mit Sprecherfarbe und Hintergrund |
| Audio | Getrennte Regler für Musik, Stimmen, Effekte, Umgebung; akustische Warnungen (Horn bei Angriff) zusätzlich visuell |
| Tempo | Taktische Pause mit Befehlsvergabe im Einzelspieler; Spielgeschwindigkeit RTS 50–150 % |
| Steuerung | Vollständig frei belegbar; Einhandmodus-Profil; Klick-statt-Ziehen-Option; Halte-Aktionen als Umschalter |
| Bewegung | Bildschirmwackeln und Blitze abschaltbar; reduzierte Partikel |
| Kognition | Berater mit Empfehlungen, Zielübersicht, Enzyklopädie, Formeln im Tooltip |

---

## 24. Balancing-Grundsätze

1. **Referenzstufe**: Balancing erfolgt auf „Lehnsherr“ ohne KI-Boni. Alle anderen Stufen sind Multiplikatoren
   auf diese Basis.
2. **Kosten-Wert-Gleichung**: Für Militäreinheiten gilt als Ausgangspunkt
   `Wert = (DPS × HP_eff)^0,5`; Kosten in Gold-Äquivalent (Holz 0,8, Stein 1,0, Nahrung 0,6, Gold 1,0) sollen im
   Verhältnis `Wert / Kosten` innerhalb von ±15 % des Klassendurchschnitts liegen. Spezialfähigkeiten werden mit
   einem Aufschlag von 10–30 % bewertet.
3. **Konter-Dreieck**: Jede Einheit hat mindestens einen harten Konter (Matrix ≥ 150 % oder Spezialbonus) und
   kontert mindestens eine andere Klasse. Grundachse: Speer > Kavallerie > Fernkampf > Infanterie > Speer; Feuer und
   Wucht > Hohle; Magie > Ätherisch; Belagerung > Befestigung.
4. **Keine dominante Strategie**: Kein Bauauftrag darf in Testserien von 200 KI-gegen-KI-Partien mehr als 60 %
   Siegquote erreichen. Zielwert für Fraktionen: 45–55 % gegeneinander auf Lehnsherr.
5. **Zeit bis zur ersten Entscheidung**: In jedem Modus muss der Spieler innerhalb von 60 s eine bedeutsame
   Entscheidung treffen können; die erste Bedrohung (Mission 1: erste Nacht) erscheint nach 7–9 Minuten.
6. **Schneeballbremse**: Wachstum wird durch Verwaltungslimit, Korruption, Bedrohung, Großmacht-Reaktion und
   Adelsforderungen gebremst – nie durch willkürliche Strafen. Die Bremse soll ab etwa 25 % Kartenkontrolle
   spürbar werden.
7. **Aufholmechaniken**: Unterlegene Reiche erhalten keine geheimen Boni, aber systemische Hilfen: Koalitionen,
   Verteidigungsvorteile (Befestigungen, Heimatmoral +10 %), Söldner, Kirchenhilfe.
8. **Hohlung ist stark, aber teuer**: Der Hohlungspfad soll in seiner Spitze 10–20 % stärker sein als lebende Pfade,
   bezahlt durch diplomatische Isolation, Bevölkerungsschwund und Irreversibilität.
9. **Wirtschaftliche Richtwerte**: Eine voll besetzte Brotkette (2 Gehöfte, 1 Windmühle, 1,3 Bäckereien)
   versorgt ca. 400 Bewohner; Arbeitskräftebedarf der Kette: 15 Personen (≈ 4 % der Versorgten). Wertschöpfung
   je Verarbeitungsstufe: 20–60 %. Kein Fertiggut darf mehr als 3 Verarbeitungsstufen brauchen, außer Luxus.
10. **Heldenanteil**: Ein Held soll in der frühen Phase etwa 25 % der Kampfkraft einer Armee ausmachen, in der späten
    Phase 10–15 % (Helden bleiben wichtig, ersetzen aber keine Armee).
11. **Auto-Resolve-Abstand**: Gespielte Schlachten sollen bei geübten Spielern 20–40 % weniger Verluste bringen; bei
    größeren Abständen wird die Auto-Resolve-Formel angepasst, nicht die Taktik geschwächt.
12. **Telemetrie** (nur mit Einwilligung, lokal auswertbar): Siegquoten, Missionsdauer, Abbruchpunkte,
    Einheitennutzung; jede Patch-Änderung wird gegen diese Grundsätze dokumentiert.
13. **Datenhaltung**: Alle Zahlen leben in RON-Dateien; Änderungen erfolgen über Versionierung mit Änderungsprotokoll
    in den Balancing-Notizen, nie direkt im Code.

---

## 25. Anhang: Glossar und ID-Übersicht

### 25.1 Glossar

| Begriff | Bedeutung |
|---|---|
| Aschenfall | Katastrophe im Jahr 0, ausgelöst durch Varkhan Aureliths Ritual |
| Aschglas | Magischer Kristall, entsteht wo der Schleier dünn ist; Magiewährung |
| Bindung | Abhängigkeit der Hohlen von Bindungsquellen (Schreine, Türme, Priester, Bosse) |
| Bedrohung | Wert, der Koalitionen gegen expansive Reiche auslöst |
| Hohlung | Korruption durch das Hohle Tief (Figur, Provinz, Reich) |
| Hohles Tief | Uralte Macht unter der Erde, älter als die Götter |
| Schleier | Grenze zwischen Eldraeth und dem Hohlen Tief |
| Schleierstärke | Provinzwert 0–5, Durchlässigkeit des Schleiers |
| Schleierdruck | Globaler Wert 0–100, treibt die uralten Bedrohungen |
| Seelenasche | Ressource des Hohlungspfads |
| n.F. | „nach dem Fall“ – Zeitrechnung ab dem Aschenfall |
| Vorzeichen | Warnsignal vor kritischen Simulationsereignissen |

### 25.2 Kanonische IDs in diesem Dokument

| Kategorie | IDs |
|---|---|
| Regionen | `valemark`, `ashmarch`, `skarnheim`, `thornwild`, `saelith`, `aureth_wastes`, `veilscar`, `cinderreach` |
| Fraktionen | `ember_crown`, `skarn_clans`, `thorn_covenant`, `saelith_compact`, `hollowed`, `ashen_legion` |
| Religionen | `eternal_flame`, `ancestor_path`, `green_circle`, `tide_house`, `hollow_liturgy`, `sun_heresy` |
| Bosse | `boss_hollow_king`, `boss_ashen_dragon`, `boss_root_mother`, `boss_pale_reeve` |
| Held | `hero_isolde` mit `ember_cleave`, `rallying_banner`, `ashen_ward` |
| Figuren | `char_king_aldric`, `char_mireille`, `char_hal_venn`, `char_torvald`, `char_sylaen`, `char_ophira`, `char_kassian`, `char_deep_voice`, `narrator` |
| Einheiten (Vertical Slice) | `worker`, `swordsman`, `spearman`, `archer`, `knight`, `hollow_thrall`, `hollow_archer`, `grave_brute`, `boss_pale_reeve` |
| Gebäude (Vertical Slice) | `keep`, `farmstead`, `barracks`, `watchtower`, `hollow_shrine`, `bone_spire` |
| Ressourcen (Vertical Slice) | `gold`, `wood`, `stone`, `food`, `population`, `population_cap` |
| Mission | `m01_last_settlement` |

Alle weiteren in diesem Dokument eingeführten IDs (z. B. `crossbowman`, `windmill`, `school_ember`, `last_ember`,
`soul_ash`) sind **Design-IDs** und werden bei ihrer Implementierung in `docs/canon.md` aufgenommen, bevor sie in
Daten oder Audio verwendet werden.
