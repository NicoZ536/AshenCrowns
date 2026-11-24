# ASHEN CROWNS – Echtzeit-Militär

Dieses Dokument beschreibt, was von GDD Kapitel 10 (Militär, Moral, Belagerung), 11 (Helden), 12 (Gegenstände), 13 (Magie) und
der KI-Kapitel **tatsächlich im Spiel läuft**: Einheiten und Gebäude des Echtzeit-Gefechts, Konter-Matrix, Schmiede-Upgrades,
Helden mit Talenten und Ausrüstung, Formationen sowie den KI-Spieler der Hohlen. Alle Zahlen stammen aus
`assets/data/{units,buildings,abilities,items,talents,upgrades,rules}.ron`; Datenfelder und Typen stehen in
`docs/tech/data_model.md` (Abschnitt 1.3, 1.5, 1.8–1.10). Ausrüstungskosten (Waren) folgen `docs/design/rts_economy.md` Abschnitt 6.

## 1. Überblick

```
Kaserne ─ Schwertkämpfer, Speerträger, Bogenschütze, Ritter │ ab Burg: Hellebardier, Schildträger
Schießstand ─ Bogenschütze, Armbrustschütze (Burg) │ Stall ─ Spähreiter, Berittener Bogenschütze (Burg)
Kapelle ─ Priester │ Magierturm (Burg, braucht Kapelle) ─ Kampfmagier
Belagerungswerkstatt (Burg, braucht Kaserne + Schmiede) ─ Rammbock, Katapult
Altar der Flamme (braucht Kapelle) ─ Hal Venn, Torvald │ Wiederbeleben gefallener Helden (auch im Bergfried)
Schmiede ─ Schärfere Klingen │ Rüstkammer ─ Härtere Rüstung │ Schießstand ─ Bessere Geschosse   (je Stufe 1–3)
```

* **Ausrüstung aus dem Lager**: Waffen, Rüstung, Tuch, Kräuter, Bretter, Eisen und Werkzeug werden bei der Ausbildung verbraucht
  (Spalte *Kosten*). Fehlt etwas, meldet das Spiel „Nicht genug Waffen.“ usw.
* **Burg** (`era: 1`): Einheiten und Gebäude mit „Burg“ sind erst nach dem Ausbau Bergfried → Burg verfügbar. Das Spiel meldet
  „… erfordert die Ausbaustufe Burg.“; im Baumenü steht „gesperrt“.
* **Bevölkerung**: Soldaten zählen je `pop` (Maschinen 3, Reiter/Magier 2); Helden 2.

## 2. Einheiten

Spalten: Klasse · Fraktion · LP · Rüstung (Typ) · Schaden je Treffer und Typ · Reichweite · Tempo (Kacheln/s) · Angriffsintervall (s) ·
Kosten · Freischaltung · Ausbildungszeit (s) · Klassenboni (Faktor auf den Schaden).

| ID | Name | Klasse | Fraktion | LP | Rüstung | Schaden | Reichweite | Tempo | Intervall | Kosten | Frei | Zeit | Boni |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `worker` | Leibeigener | Arbeiter | Glutkrone | 160 | 0 (Medium) | 4–6 Normal | 0.5 | 2.4 | 2.00 | 50 Gold | – | 12 |  |
| `swordsman` | Schwertkämpfer | Infanterie | Glutkrone | 300 | 3 (Heavy) | 14–17 Normal | 0.6 | 2.5 | 1.30 | 80 Gold, 10 Holz, 1 Waffen, 1 Rüstung | – | 16 |  |
| `spearman` | Speerträger | Infanterie | Glutkrone | 260 | 2 (Medium) | 11–14 Pierce | 1.0 | 2.6 | 1.40 | 60 Gold, 30 Holz, 1 Waffen | – | 14 | Kavallerie×2, Boss×1.3 |
| `archer` | Bogenschütze | Fernkämpfer | Glutkrone | 180 | 0 (Light) | 12–15 Pierce | 5.5 | 2.7 | 1.60 | 70 Gold, 40 Holz, 1 Waffen | – | 15 |  |
| `knight` | Ritter | Kavallerie | Glutkrone | 520 | 4 (Heavy) | 22–28 Normal | 0.7 | 3.8 | 1.50 | 170 Gold, 40 Holz, 20 Stein, 1 Waffen, 2 Rüstung | – | 24 | Fernkämpfer×1.5 |
| `hero_isolde` | Isolde Raventhorn | Held | Glutkrone | 650 | 4 (Hero) | 26–32 Hero | 0.7 | 3.0 | 1.40 |  | – | 60 |  |
| `crossbowman` | Armbrustschütze | Fernkämpfer | Glutkrone | 190 | 1 (Medium) | 26–32 Pierce | 6.2 | 2.4 | 2.70 | 90 Gold, 40 Holz, 1 Waffen, 1 Bretter | Burg | 20 | Kavallerie×1.6, Monster×1.2, Boss×1.2, Flieger×1.4 |
| `halberdier` | Hellebardier | Infanterie | Glutkrone | 320 | 4 (Heavy) | 17–21 Pierce | 1.2 | 2.5 | 1.40 | 75 Gold, 25 Holz, 1 Waffen, 1 Rüstung | Burg | 18 | Kavallerie×3.2, Boss×1.2, Monster×1.15 |
| `shieldbearer` | Schildträger | Infanterie | Glutkrone | 340 | 5 (Heavy) | 10–13 Normal | 0.6 | 2.3 | 1.30 | 70 Gold, 40 Holz, 1 Waffen, 1 Rüstung | – | 17 |  |
| `scout_rider` | Spähreiter | Kavallerie | Glutkrone | 210 | 1 (Light) | 9–12 Normal | 0.6 | 4.8 | 1.10 | 90 Gold, 30 Holz, 1 Waffen | – | 16 | Fernkämpfer×1.3, Unterstützung×1.3, Arbeiter×1.4 |
| `horse_archer` | Berittener Bogenschütze | Kavallerie | Glutkrone | 220 | 1 (Light) | 10–13 Pierce | 5.0 | 4.3 | 1.50 | 130 Gold, 50 Holz, 1 Waffen | Burg | 22 |  |
| `battle_mage` | Kampfmagier | Fernkämpfer | Glutkrone | 150 | 0 (Unarmored) | 18–22 Fire | 6.0 | 2.4 | 2.20 | 120 Gold, 20 Holz, 2 Tuch, 1 Kräuter | Burg | 24 |  |
| `priest` | Priester der Flamme | Unterstützung | Glutkrone | 140 | 0 (Unarmored) | 5–7 Normal | 0.9 | 2.5 | 1.60 | 80 Gold, 20 Holz, 1 Tuch, 2 Kräuter | – | 20 |  |
| `battering_ram` | Rammbock | Belagerung | Glutkrone | 480 | 4 (Fortified) | 30–38 Siege | 0.9 | 1.8 | 2.40 | 140 Gold, 160 Holz, 6 Bretter, 2 Eisenbarren | Burg | 30 |  |
| `catapult` | Katapult | Belagerung | Glutkrone | 260 | 1 (Light) | 70–90 Siege | 10.5 (min. 3.5) | 1.6 | 4.50 | 160 Gold, 180 Holz, 8 Bretter, 2 Werkzeug | Burg | 36 |  |
| `hero_hal_venn` | Bruder Hal Venn | Held | Glutkrone | 520 | 2 (Hero) | 18–24 Hero | 0.8 | 2.8 | 1.50 | 350 Gold, 40 Holz, 2 Tuch, 3 Kräuter | – | 45 |  |
| `hero_torvald` | Torvald Eisenhand | Held | Glutkrone | 820 | 6 (Hero) | 34–42 Hero | 0.9 | 2.7 | 1.70 | 420 Gold, 40 Holz, 2 Waffen, 2 Rüstung | – | 50 |  |
| `hollow_thrall` | Hohlknecht | Monster | Hohle | 200 | 1 (Medium) | 10–13 Normal | 0.6 | 2.3 | 1.50 | 50 Gold, 15 Holz | – | 10 |  |
| `hollow_archer` | Knochenschütze | Fernkämpfer | Hohle | 150 | 0 (Light) | 10–13 Pierce | 5.0 | 2.4 | 1.70 | 70 Gold, 30 Holz | – | 14 |  |
| `grave_brute` | Grabkoloss | Monster | Hohle | 700 | 2 (Heavy) | 28–36 Siege | 0.8 | 1.9 | 2.20 | 190 Gold, 80 Holz | – | 30 |  |
| `boss_pale_reeve` | Der Fahle Vogt | Boss | Hohle | 2400 | 5 (Hero) | 34–42 Hero | 0.9 | 2.6 | 1.60 |  | – | 15 |  |
| `hollow_digger` | Grabgräber | Arbeiter | Hohle | 120 | 0 (Medium) | 4–6 Normal | 0.5 | 2.3 | 2.00 | 45 Gold | – | 12 |  |
| `ghoul` | Ghul | Monster | Hohle | 170 | 0 (Light) | 11–14 Normal | 0.6 | 3.6 | 0.90 | 60 Gold, 20 Holz | – | 12 | Fernkämpfer×1.5, Unterstützung×1.5, Belagerung×1.3 |
| `ash_witch` | Aschhexe | Fernkämpfer | Hohle | 160 | 0 (Unarmored) | 14–18 Magic | 5.5 | 2.4 | 2.00 | 130 Gold, 40 Holz | – | 22 |  |
| `bat_swarm` | Fledermausschwarm | Flieger | Hohle | 90 | 0 (Unarmored) | 5–7 Normal | 0.5 | 4.8 | 0.60 | 55 Gold | – | 10 | Fernkämpfer×1.3, Unterstützung×1.5 |

Besonderheiten (Datenfelder `UnitDef`):

| Einheit | Merkmal |
|---|---|
| Schildträger | `ranged_resist 0,4`: fängt 40 % des Pierce-Schadens ab; Fähigkeit *Schildwall* |
| Hellebardier, Speerträger | `range ≥ 1,0`: erreichen als einzige Nahkämpfer **Flieger** |
| Armbrustschütze | Bolzen (`Bolt`, Tempo 14); Boni gegen Kavallerie, Monster, Boss, Flieger |
| Katapult | `min_range 3,5`, `splash 1,8` (Flächenschaden, zum Rand −50 %), `building_mult 1,6`, Projektilbogen (`Boulder`) |
| Rammbock | `building_mult 3,2`, furchtlos, Rüstungstyp *Fortified* (Pfeile prallen ab) |
| Kampfmagier | Schaden `Fire`, Feuerball mit Flächenradius 1,2; Fähigkeiten *Feuerball* (Zielzauber) und *Flammenwoge* |
| Priester | Klasse *Unterstützung*; `autocast: holy_heal` – heilt Verwundete im Umkreis selbstständig |
| Aschhexe | `autocast: raise_dead`; Diener (`master`) vergehen mit ihr, höchstens 4 gleichzeitig |
| Fledermausschwarm | `flying`: fliegt über Mauern/Wasser/Gebäude, Separation nur von Fliegern; Nahkämpfer ohne Stangenwaffe können ihn weder wählen noch angreifen |
| Grabkoloss | `building_mult 2,2` (Ramme der Hohlen) |
| Grabgräber | Arbeiter der Hohlen (sammelt, baut); nur die KI bildet sie aus |

## 3. Schaden, Rüstung und Konter-Matrix

### 3.1 Schadensart gegen Rüstungsart

Erweiterte Tabelle (neu: **Fire**, Feuerschule). Formel und weitere Faktoren siehe `data_model.md` Abschnitt 1.7.

| | Unarmored | Light | Medium | Heavy | Fortified | Hero |
|---|---|---|---|---|---|---|
| **Normal** | 1,0 | 1,0 | 1,5 | 1,0 | 0,7 | 1,0 |
| **Pierce** | 1,5 | 2,0 | 0,75 | 0,8 | 0,35 | 0,5 |
| **Siege** | 1,5 | 1,0 | 0,5 | 1,0 | 1,5 | 0,5 |
| **Magic** | 1,0 | 1,25 | 0,75 | 2,0 | 0,35 | 0,5 |
| **Fire** | 1,5 | 1,25 | 1,0 | 0,75 | 0,6 | 0,75 |
| **Hero** | 1,0 | 1,0 | 1,0 | 1,0 | 0,5 | 1,0 |

Weitere Modifikatoren des Schadens (alle multiplikativ): Klassenbonus der Waffe (`bonus_vs`), Gebäudefaktor
(`building_mult`, nur gegen Gebäude), Talent-/Ausrüstungsbonus (`damage_pct`, `damage`), Upgrades (+10 % je Waffenstufe …),
Formation (Abschnitt 8), Segen/Buffs, Regen bei Fernkämpfern (×0,9). Erlittener Schaden: Rüstung (Basis + Stufe·0,5 + Ausrüstung +
Upgrades + Formation + Buffs), `ranged_resist` und Formation gegen Pierce, Schild.

### 3.2 Konter-Matrix (Einheit gegen Einheit)

Lesart: Zeile schlägt Spalte (++ harter Konter ≥ ×1,5 wirksam, + Vorteil, 0 ausgeglichen, − Nachteil, −− harter Nachteil).
Das Konter-Dreieck des GDD (Speer > Kavallerie > Fernkampf > Infanterie > Speer; Feuer und Wucht > Hohle; Belagerung >
Befestigung) bleibt erhalten und wird um Magie und Flieger erweitert.

| ↓ schlägt → | Schwert | Speer/Hellebarde | Schild | Bogen | Armbrust | Reiter | Magier | Priester | Belagerung | Hohle Masse | Ghul | Hexe | Flieger | Koloss | Gebäude |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Schwertkämpfer | 0 | − | 0 | + | + | − | + | + | 0 | 0 | 0 | + | (kein Ziel) | −− | − |
| Speerträger | 0 | 0 | 0 | + | + | ++ | + | + | − | + | 0 | + | + | − | − |
| Hellebardier | 0 | + | 0 | + | + | ++ (×3,2) | + | + | − | + | 0 | + | + | 0 | − |
| Schildträger | 0 | 0 | 0 | ++ (−40 % Pfeile) | ++ | − | 0 | 0 | − | 0 | + | − | (kein Ziel) | −− | − |
| Bogenschütze | − | − | −− | 0 | 0 | −− | + | + | + | 0 | − | 0 | ++ | −− | −− |
| Armbrustschütze | 0 | − | − | + | 0 | ++ (×1,6) | + | + | + | 0 | − | 0 | ++ (×1,4) | + | −− |
| Ritter | + | −− | − | ++ (×1,5) | ++ | 0 | ++ | ++ | + | 0 | − | + | (kein Ziel) | 0 | − |
| Spähreiter | − | −− | −− | + (×1,3) | + | − | ++ | ++ | + | − | − | + | (kein Ziel) | −− | −− |
| Berittener Bogenschütze | − | −− | − | 0 | 0 | −− | + | + | + | 0 | − | 0 | ++ | − | −− |
| Kampfmagier (Feuer) | ++ (Heavy ×0,75) | + | ++ | ++ (Light) | ++ | + | + | ++ | 0 | + | + | ++ | ++ | + | − |
| Katapult | + (Fläche) | + | 0 | + | + | − | + | + | 0 | ++ (Fläche) | + | + | (kein Ziel) | + | ++ |
| Rammbock | −− | −− | −− | −− | −− | −− | −− | −− | − | −− | −− | −− | (kein Ziel) | − | ++ (×3,2) |
| Priester | Unterstützung: heilt, segnet, kämpft kaum | | | | | | | | | | | | | | |

Auf der Gegenseite (die KI wählt nach genau diesen Gewichten, Abschnitt 10.2):

| Hohle Einheit | Starke Seite | Schwäche |
|---|---|---|
| Hohlknecht | billige Masse, Normal ×1,5 gegen Medium (Speer-/Hellebardenträger) | Feuer, Flächenschaden, Schildwall |
| Knochenschütze | Pierce ×2 gegen Light/Unarmored (Bogen, Magier, Priester) | Reiter, Ghule |
| Ghul | ×1,5 gegen Fernkämpfer, Priester; ×1,3 gegen Belagerung; schnell (3,6) | Schildträger, Speere, Katapult-Fläche |
| Aschhexe | Magic ×2 gegen Heavy (Ritter, Schwertkämpfer, Schildträger); beschwört | zerbrechlich (160 LP), Reiter |
| Grabkoloss | Siege ×1,5, `building_mult 2,2`: reißt Mauern und Tore ein | Armbrust/Magie, Hellebarde, Fernkämpfer-Massen |
| Fledermausschwarm | fliegt über Mauern, ×1,5 gegen Priester, ×1,3 gegen Fernkämpfer | Bogen-/Armbrustschützen (×1,4), Speere |

## 4. Fähigkeiten

Fähigkeiten setzen sich aus Effekten zusammen (`Damage`, `Heal`, `Shield`, `Rally`, `Drain`, `Summon`, `Terror`); `Summon` kennt ein
`max` (gleichzeitig lebende Diener). **Zielzauber** (`targeting: Point`) wirken am Klickpunkt: Fähigkeit wählen, dann Linksklick auf den
Boden (Reichweite siehe Tabelle; außerhalb läuft der Zauberer hin). Alle Fähigkeitswerte skalieren mit der Stufe (+8 % je Stufe).

| ID | Name | Mana | Abklingzeit s | Reichweite | Wirkung |
|---|---|---|---|---|---|
| `shield_wall` | Schildwall | 0 | 25 | – | Verbündete im Radius 3: +4 Rüstung, −15 % Schaden, 10 s |
| `fireball` | Feuerball | 45 | 8 | 7,5 | 85 Feuerschaden im Radius 2 |
| `flame_nova` | Flammenwoge | 70 | 16 | – | 70 Feuerschaden im Radius 3,2 |
| `holy_heal` | Heilendes Licht | 40 | 6 | – | heilt Verbündete im Radius 4,5 um 90; Autocast des Priesters |
| `blessing` | Segen der Flamme | 70 | 25 | – | Radius 6: +20 % Schaden, +2 Rüstung, Moral, 15 s |
| `raise_dead` | Tote erwecken | 70 | 20 | – | 2 Hohlknechte, höchstens 4; Autocast der Hexe |
| `flame_mend` / `sacred_flame` / `beacon_of_faith` | Hal Venn: Flammenheilung / Heilige Flamme / Glaubensfeuer | 70 / 85 / 120 | 9 / 12 / 40 | – / 7 / – | Heilung 150 (R 5) / 120 Feuer (R 3, Zielzauber) / +15 % Schaden, +3 Rüstung, Heilung 60 (R 8) |
| `clan_roar` / `ground_slam` / `iron_vow` | Torvald: Ahnenruf / Erdspalter / Eiserner Schwur | 80 / 70 / 60 | 25 / 12 / 22 | – | Rally R 6 + Furcht / 120 Schaden R 2,8 / Schild 300 für 12 s |
| `phoenix_cut`, `war_cry`, `last_stand` | Isolde (Talente) | 100 / 110 / 90 | 18 / 45 / 40 | – | 160 Feuer R 3,4 / +35 % Schaden, +3 Rüstung R 8 / Schild 400 + Rüstung für Umfeld |
| `pillar_of_fire`, `resurgence`, `aegis_of_flame` | Hal Venn (Talente) | 140 / 150 / 100 | 30 / 50 / 35 | 8 / – / – | 220 Feuer R 4 / Heilung 260 R 9 / Schild 350 + Rüstung |
| `ancestral_fury`, `stone_skin`, `war_horns` | Torvald (Talente) | 100 / 80 / 120 | 25 / 40 / 45 | – | 200 Schaden R 3,5 / Schild 450 für 15 s / +25 % Schaden R 9, Furcht |

Selbstständiges Wirken (`autocast`): alle 0,5 s prüft jede Einheit mit Autocast-Liste, ob die Lage passt – Heilen bei Verwundeten (Fehlbetrag
≥ 60 % der Heilmenge oder ein Verbündeter unter 45 %), Beschwören bei Feindkontakt unter der Dienergrenze, Flächenzauber ab zwei Zielen.
Bosse folgen weiter dem Skript des Wellen-Direktors.

## 5. Gebäude

| ID | Name | Größe | LP | Kosten | Bauzeit | Frei | Bildet aus / Funktion |
|---|---|---|---|---|---|---|---|
| `barracks` | Kaserne | 3×2 | 1300 | 160 Gold, 120 Holz, 40 Stein | 45 | – | swordsman, spearman, archer, knight, halberdier, shieldbearer |
| `bone_spire` | Knochenturm | 1×1 | 900 | 120 Gold, 60 Holz | 30 | – |  |
| `shooting_range` | Schießstand | 3×2 | 1000 | 140 Gold, 120 Holz, 20 Stein | 40 | – | archer, crossbowman |
| `stable` | Stall | 3×2 | 1000 | 150 Gold, 160 Holz, 10 Stein | 42 | – | scout_rider, horse_archer |
| `chapel` | Kapelle der Flamme | 2×2 | 900 | 130 Gold, 80 Holz, 90 Stein | 38 | – | priest |
| `mage_tower` | Magierturm | 2×2 | 1100 | 220 Gold, 100 Holz, 160 Stein, 10 Ziegel | 50 | Burg | battle_mage |
| `siege_workshop` | Belagerungswerkstatt | 3×3 | 1300 | 200 Gold, 220 Holz, 60 Stein, 20 Bretter, 4 Werkzeug | 55 | Burg | battering_ram, catapult |
| `altar` | Altar der Flamme | 2×2 | 1000 | 200 Gold, 100 Holz, 100 Stein | 45 | – | hero_hal_venn, hero_torvald |
| `bone_pit` | Knochengrube | 3×2 | 900 | 90 Gold, 70 Holz | 35 | – | hollow_thrall, hollow_archer |
| `ghoul_den` | Ghulbau | 2×2 | 750 | 110 Gold, 80 Holz | 40 | – | ghoul, bat_swarm |
| `ash_altar` | Aschaltar | 3×2 | 800 | 170 Gold, 90 Holz | 50 | – | ash_witch |
| `grave_forge` | Grabschmiede | 3×3 | 1400 | 220 Gold, 140 Holz | 60 | – | grave_brute |
| `bone_cache` | Knochenlager | 2×2 | 500 | 40 Gold, 30 Holz | 20 | – |  |

Voraussetzungen (`requires`): Schießstand, Stall → Kaserne; Magierturm → Kapelle; Altar → Kapelle; Belagerungswerkstatt → Kaserne + Schmiede.
Die Gebäude der Hohlen (Knochengrube bis Knochenlager, Knochenturm als Wehrturm) sind `faction: Hollowed` und erscheinen nicht im Baumenü
des Spielers; die KI baut sie nach den Regeln aus Abschnitt 10.

3D-Modelle (`crates/ashen_game/src/art3d/military.rs`): Schießstand mit Zielscheiben, Stall mit Boxen und Heuballen, Kapelle mit Glockenturm,
Magierturm mit schwebender Feuerkugel, Belagerungswerkstatt mit halbem Rammbock und Katapultarm, Altar mit Flammenschale; Knochengrube mit Rippenbögen
und grünem Glühen, Ghulbau als Felsmound, Aschaltar mit Aschglas-Pfeilern, Grabschmiede mit Esse, Knochenlager als Zelt.

## 6. Schmiede-Upgrades

Befehlskarte des Gebäudes: je Upgrade eine Schaltfläche mit Kosten, Forschungszeit und Fortschrittsanzeige (Klick: abbrechen, volle Erstattung).
Es läuft je Gebäude eine Forschung; Wirkung gilt sofort für alle Einheiten der Klassen.

| Upgrade | Gebäude | Klassen | Wirkung je Stufe | Stufe 1 | Stufe 2 (Burg) | Stufe 3 (Burg) |
|---|---|---|---|---|---|---|
| Schärfere Klingen (`forge_weapons`) | Schmiede | Infanterie, Kavallerie, Belagerung, Monster, Flieger | +10 % Schaden | 120 Gold, 4 Eisenbarren · 40 s (ohne Burg) | 220 Gold, 8 Eisenbarren, 4 Waffen · 60 s | 380 Gold, 14 Eisenbarren, 8 Waffen · 90 s |
| Härtere Rüstung (`forge_armor`) | Rüstkammer (Burg) | alle Soldaten | +1 Rüstung | 150 Gold, 6 Eisenbarren, 2 Rüstung · 50 s | 260 Gold, 10 Eisenbarren, 4 Rüstung · 75 s | 420 Gold, 16 Eisenbarren, 8 Rüstung · 100 s |
| Bessere Geschosse (`fletching`) | Schießstand | Fernkämpfer | +8 % Schaden | 100 Gold, 6 Bretter · 35 s (ohne Burg) | 180 Gold, 10 Bretter, 2 Eisenbarren · 55 s | 300 Gold, 16 Bretter, 5 Eisenbarren · 80 s |

## 7. Belagerung

* **Mindestreichweite** (`min_range`): Ziele näher als 3,5 Kacheln werden nicht beschossen; ein Katapult mit solchem Ziel weicht zurück, bis der
  Abstand reicht. Der Schuss prüft die Reichweite erneut beim Abfeuern.
* **Projektilbogen und Flächenschaden**: `Boulder` fliegt mit hohem Bogen (Höhe ≈ 0,45 · Entfernung) und schlägt **immer am Zielpunkt** ein; alle Feinde im
  Radius erhalten bis zu 100 % (Mitte) bzw. 50 % (Rand) des Schadens, Gebäude zusätzlich ×`building_mult`.
* **Gebäudeschaden**: Ramme ×3,2, Katapult ×1,6, Grabkoloss ×2,2 – zusätzlich zur Siege-Tabelle (Fortified ×1,5). Beispiel: Ein Katapulttreffer
  (70–90) richtet gegen eine Mauer (Rüstung 6) ca. 140 Schaden an; 700-LP-Mauern fallen in fünf Treffern.
* **Wegfindung**: Findet ein Angreifer keinen Weg, brechen Einheiten die nächste Befestigung auf (siehe `rts_economy.md` Abschnitt 8).
* **Flieger** ignorieren Mauern, Wasser und Gelände (gerade Linie zum Ziel).

## 8. Formationen

Befehlskarte: Schaltfläche *Formation* (Taste `F`) wechselt Block → Linie → Keil → Kolonne → Karree für die ausgewählten Kampfeinheiten
(`Command::SetFormation`). Sie wirkt auf **Bewegungsbefehle** (die Plätze werden nach der häufigsten Formation der Gruppe berechnet) und auf **Kampfwerte**.
Rang der Plätze: Kavallerie, Nahkämpfer/Monster, Helden, Fernkämpfer/Belagerung, Unterstützung, Arbeiter (vorne nach hinten).

| Formation | Aufstellung | Wirkung |
|---|---|---|
| Block | lockeres Raster (Grundverhalten) | – |
| Linie | eine breite Reihe, bis 12 Plätze breit | Fernkämpfer +10 % Schaden; −1 Rüstung |
| Keil | Dreieck mit Spitze voran | +15 % Schaden; −1 Rüstung |
| Kolonne | zwei bis drei Mann breit | +15 % Tempo; −15 % Schaden; −1 Rüstung |
| Karree (Schildwall) | hohles Quadrat, Nahkämpfer außen, Fernkämpfer/Heiler innen | +3 Rüstung; −25 % erlittener Pierce-Schaden; −30 % Moralverlust; −20 % Tempo; −10 % Schaden |

Mindestabstand der Plätze 0,85 Kacheln (Test `formation_keeps_spacing_and_has_effects`). **Moral und Flucht** bleiben unverändert: Fällt die Moral unter
15 %, flieht die Einheit (ignoriert Formationsbefehle) und kehrt mit 40 % Moral zurück; das Karree senkt nur den Verlust (Faktor 0,7).

## 9. Helden

### 9.1 Stufen, Punkte, Wiederbelebung

* **Stufe 1–10** (`rules.ron → xp_levels`: 0, 100, 250, 450, 700, 1000, 1400, 1900, 2500, 3200 Erfahrung). Je Stufe + Leben (`hp_per_level`), + Schaden
  (`damage_per_level`), +15 Mana, +0,5 Rüstung; Fähigkeitswerte +8 % je Stufe.
* **Talentpunkte**: einer je Stufe über 1 (insgesamt 9). Drei Pfade, je drei Reihen; Reihe 2 verlangt 2, Reihe 3 verlangt 4 Punkte im selben Pfad. Reihe 3 schaltet
  jeweils eine **Fähigkeit** frei (zusätzlicher Befehlsplatz, Taste `R`/`T`/`Y`).
* **Spielbare Helden**: *Isolde Raventhorn* (`hero_isolde`, Mission 1), *Bruder Hal Venn* (`hero_hal_venn`, Flammenpriester, Heiler) und *Torvald Eisenhand*
  (`hero_torvald`, Clanfürst von Skarnheim, Bollwerk). Hal und Torvald werden am **Altar der Flamme** angeworben (je Held nur einmal).
* **Sterben und Wiederbeleben**: Ein gefallener Held bleibt mit Stufe, Talenten und Ausrüstung gespeichert (`SimWorld::fallen`). Er kehrt nach 40 s kostenlos im
  Bergfried zurück (50 % Leben) oder sofort gegen Gold (`150 + 60 · (Stufe−1)`) am Bergfried (50 %) bzw. Altar (80 %). Befehlskarte des Gebäudes: *… wiederbeleben*.
  Mit „Dauerhafter Heldentod“ gibt es keine Rückkehr.

### 9.2 Talentbäume

**hero_isolde** – Pfade: Glutklinge, Bannerträgerin, Aschenwacht

| Pfad | Reihe | Talent | Ränge | Wirkung je Rang |
|---|---|---|---|---|
| Glutklinge | 1 | Glühende Schneide (`edge_of_embers`) | 3 | +6 % Schaden je Rang. |
| Glutklinge | 2 | Sengende Hiebe (`searing_strikes`) | 2 | +5 Schaden je Treffer je Rang. |
| Glutklinge | 3 | Phönixhieb (`phoenix_cut`) | 1 | Schaltet die Fähigkeit Phönixhieb frei (160 Feuerschaden im Umkreis) und gibt +8 % Schaden. |
| Bannerträgerin | 1 | Standhaftes Banner (`steadfast_banner`) | 3 | +40 Leben je Rang. |
| Bannerträgerin | 2 | Karmesinroter Wille (`crimson_resolve`) | 2 | Fähigkeiten laden 8 % schneller je Rang. |
| Bannerträgerin | 3 | Kriegsstandarte (`war_standard`) | 1 | Schaltet den Schlachtruf frei (+35 % Schaden, +3 Rüstung für das Heer) und gibt +30 Mana. |
| Aschenwacht | 1 | Aschenplatte (`ashen_plate`) | 3 | +1 Rüstung je Rang. |
| Aschenwacht | 2 | Glutkraft (`ember_vigor`) | 2 | +0,8 Leben pro Sekunde je Rang. |
| Aschenwacht | 3 | Unbeugsam (`unyielding`) | 1 | Schaltet das Letzte Aufgebot frei (400 Schutz) und gibt +80 Leben. |

**hero_hal_venn** – Pfade: Flammenlicht, Heilende Hände, Glaubenswall

| Pfad | Reihe | Talent | Ränge | Wirkung je Rang |
|---|---|---|---|---|
| Flammenlicht | 1 | Entfachte Flamme (`kindled_flame`) | 3 | +7 % Schaden je Rang. |
| Flammenlicht | 2 | Läuterndes Feuer (`purifying_fire`) | 2 | +25 Mana je Rang und +4 Schaden je Treffer. |
| Flammenlicht | 3 | Feuersäule (`pillar_of_fire`) | 1 | Schaltet die Feuersäule frei (220 Feuerschaden im Radius 4). |
| Heilende Hände | 1 | Sanfte Hände (`gentle_hands`) | 3 | +0,5 Mana pro Sekunde je Rang. |
| Heilende Hände | 2 | Göttliche Gnade (`divine_grace`) | 2 | Fähigkeiten laden 8 % schneller je Rang. |
| Heilende Hände | 3 | Neuerwachen (`resurgence`) | 1 | Schaltet Neuerwachen frei (260 Leben für alle im Umkreis 9). |
| Glaubenswall | 1 | Gesegnete Gewänder (`blessed_vestments`) | 3 | +1 Rüstung je Rang. |
| Glaubenswall | 2 | Unerschütterlicher Glaube (`unshaken_faith`) | 2 | +50 Leben je Rang. |
| Glaubenswall | 3 | Flammenaegis (`aegis_of_flame`) | 1 | Schaltet die Flammenaegis frei (350 Schutz, +3 Rüstung für das Umfeld). |

**hero_torvald** – Pfade: Axt der Ahnen, Eisenhaut, Clanruf

| Pfad | Reihe | Talent | Ränge | Wirkung je Rang |
|---|---|---|---|---|
| Axt der Ahnen | 1 | Ahnenschneide (`ancestral_edge`) | 3 | +6 % Schaden je Rang. |
| Axt der Ahnen | 2 | Spaltende Hiebe (`cleaving_blows`) | 2 | +7 Schaden je Treffer je Rang. |
| Axt der Ahnen | 3 | Zorn der Ahnen (`ancestral_fury`) | 1 | Schaltet den Zorn der Ahnen frei (200 Schaden im Radius 3,5). |
| Eisenhaut | 1 | Eisenhaut (`iron_hide`) | 3 | +1 Rüstung je Rang. |
| Eisenhaut | 2 | Herz des Berges (`mountain_heart`) | 2 | +70 Leben je Rang. |
| Eisenhaut | 3 | Steinhaut (`stone_skin`) | 1 | Schaltet die Steinhaut frei (450 Schutz für 15 s). |
| Clanruf | 1 | Flinke Clans (`swift_clans`) | 3 | +5 % Geschwindigkeit je Rang. |
| Clanruf | 2 | Erfahrener Kriegsherr (`seasoned_warlord`) | 2 | +15 % Erfahrung je Rang. |
| Clanruf | 3 | Kriegshörner (`war_horns`) | 1 | Schaltet die Kriegshörner frei (+25 % Schaden für das Heer, Feinde verlieren Moral). |


### 9.3 Ausrüstung und Gegenstände

Plätze: **Waffe, Rüstung, Amulett, zwei Ringe**; dazu ein **Inventar** mit 12 Plätzen. Ausrüstung ändert die Werte sofort (`StatMods`: Leben, Mana, Schaden fest und
in %, Rüstung, Tempo, Regeneration, Erfahrung, Abklingzeiten). Tränke wirken beim Trinken (Abklingzeit 8 s). Seltenheiten: Gewöhnlich, Ungewöhnlich, Selten, Episch,
**Artefakt** (einzigartig, Kanon).

| ID | Name | Platz | Seltenheit | Wirkung | Wert (Gold) |
|---|---|---|---|---|---|
| `rusty_sword` | Rostiges Schwert | Waffe | Gewöhnlich | +3 Schaden | 12 |
| `steel_sword` | Stahlschwert | Waffe | Ungewöhnlich | +6 Schaden, +3 % Schaden | 45 |
| `pilgrim_staff` | Pilgerstab | Waffe | Ungewöhnlich | +3 Schaden, +30 Mana, +0,4 Mana/s | 50 |
| `embersteel_blade` | Glutstahlklinge | Waffe | Selten | +10 Schaden, +6 % Schaden | 140 |
| `skarn_greataxe` | Skarn-Streitaxt | Waffe | Selten | +14 Schaden, +30 Leben | 150 |
| `edran_blade` | Edrans Grenzklinge | Waffe | Artefakt | +16 Schaden, +10 % Schaden, +60 Leben, +10 % schnellere Fähigkeiten | 600 |
| `padded_jerkin` | Wattierter Wams | Rüstung | Gewöhnlich | +1 Rüstung, +20 Leben | 14 |
| `chain_hauberk` | Kettenhemd | Rüstung | Ungewöhnlich | +2 Rüstung, +50 Leben | 55 |
| `ember_plate` | Glutkronen-Plattenpanzer | Rüstung | Selten | +4 Rüstung, +90 Leben | 170 |
| `grauwall_aegis` | Aegis von Grauwall | Rüstung | Artefakt | +5 Rüstung, +140 Leben, +1,5 Leben/s | 650 |
| `bone_charm` | Knochenamulett | Amulett | Gewöhnlich | +0,5 Leben/s | 15 |
| `flamekeeper_pendant` | Flammenwahrer-Anhänger | Amulett | Ungewöhnlich | +40 Mana, +0,6 Mana/s | 60 |
| `oath_medal` | Eidmedaille | Amulett | Selten | +1 Rüstung, +5 % Schaden, +10 % Erfahrung | 130 |
| `ashglass_heart` | Aschglas-Herz | Amulett | Artefakt | +80 Mana, +1,2 Mana/s, +12 % schnellere Fähigkeiten, +8 % Schaden | 700 |
| `copper_ring` | Kupferring | Ring | Gewöhnlich | +25 Leben | 10 |
| `iron_band` | Eisenband | Ring | Ungewöhnlich | +1 Rüstung | 40 |
| `wolf_signet` | Wolfssiegel | Ring | Ungewöhnlich | +6 % Tempo, +2 Schaden | 45 |
| `ember_ring` | Glutring | Ring | Selten | +7 % Schaden, +40 Leben | 120 |
| `varkhan_signet` | Siegelring Varkhans | Ring | Artefakt | +12 % Schaden, +70 Leben, +20 % Erfahrung | 750 |
| `vael_dorn_seal` | Siegel von Vael Dorn | Ring | Artefakt | +1 Mana/s, +1 Leben/s, +2 Rüstung | 700 |
| `potion_heal` | Heiltrank | Trank | Gewöhnlich | 220 Leben | 25 |
| `potion_heal_greater` | Großer Heiltrank | Trank | Ungewöhnlich | 450 Leben | 60 |
| `potion_mana` | Manatrank | Trank | Gewöhnlich | 150 Mana | 25 |
| `potion_ember` | Glutelixier | Trank | Selten | +30 % Schaden, 20 s | 80 |
| `potion_ward` | Aschenfestigkeit | Trank | Ungewöhnlich | +4 Rüstung, 20 s | 55 |

Artefakte des Kanons: *Edrans Grenzklinge* (Markgraf Edran Raventhorn, Grauwall), *Aegis von Grauwall*, *Aschglas-Herz*, *Siegelring Varkhans* (der Letzte Sonnenkönig),
*Siegel von Vael Dorn* (Vogt Aldous Meringer).

### 9.4 Beute

* **Einheiten** der Hohlen tragen eine Beutetabelle (`loot`: Gegenstand, Wahrscheinlichkeit), z. B. Hohlknecht 4 % Kupferring / 6 % Heiltrank, Grabkoloss 18 % Kettenhemd.
* **Bosse**: Der Fahle Vogt lässt *Siegel von Vael Dorn*, *Glutkronen-Plattenpanzer* und *Großen Heiltrank* fallen (je 100 %).
* **Lager** (`Placement.camp`): fällt die letzte Einheit eines Lagers, lässt es zwei gewichtete Würfe aus `rules.ron → camp_loot` fallen (auch Artefakte, Gewicht 1 von 46).
  Mission 1 hat die Lager *Ruinenmine* und *Nördliche Furt*.
* Beute liegt als funkelndes Symbol (Farbe = Seltenheit) am Boden; Helden nehmen sie im Umkreis 1,3 auf, untätige Helden laufen im Umkreis 7 selbst hin. Volles Inventar: Hinweis, Gegenstand bleibt liegen.

### 9.5 Heldenfenster

`H` (ohne Auswahl, oder nur Helden gewählt) oder Klick auf das Heldenporträt (Auswahlfeld oder Heldenleiste links unter der Hauptleiste). Inhalt: Werte (inklusive Ausrüstungs-/Talentboni), Fähigkeiten,
Talentbaum (Klick auf ein Talent setzt einen Punkt; Tooltip nennt Voraussetzung), Ausrüstungsplätze (Klick: ablegen), Inventar (Klick: anlegen bzw. Trank trinken, Rechtsklick: wegwerfen).
Die Heldenleiste zeigt alle Helden mit Leben, Stufe und freien Talentpunkten (`+`); gefallene Helden mit Rückkehrzeit.

## 10. Gegner-KI (Gefecht)

Im **Gefecht** (Skirmish) baut und befehligt `systems/ai_player.rs` die Hohlen; der Wellen-Direktor ruht. In **Kampagne** und **Überleben** gilt weiter die bisherige Wellenlogik
(`systems/ai.rs`). Die KI nutzt dieselben Befehle und Regeln wie der Spieler (Kosten, Bauzeit, Warteschlange); jede Entscheidung fällt alle `think` Sekunden, Zufall
stammt ausschließlich aus dem Spielzufall (deterministisch, Test `enemy_ai_is_deterministic_and_levels_differ`).

### 10.1 Wirtschaft und Basisbau

* **Sammeln**: Grabgräber (`hollow_digger`, 45 Gold) aus dem Hohlschrein (dieser ist Abgabestelle). Untätige Arbeiter sammeln Gold bzw. Holz nach Bedarf (Holzanteil 70 % bei Holzmangel, sonst 30–50 %); bei Holzmangel und Goldüberschuss
  werden Goldsammler umgeschichtet. Einnahmen × `income_mult` der Stufe.
* **Bauen** (immer ein Bauauftrag nach dem anderen, ein freier Streifen Abstand, Plätze in Richtung Gegner): Knochenlager an Mine/Wald (≥ 9 Kacheln vom nächsten Lager), dann
  Knochengrube → Ghulbau → 2. Knochengrube → Aschaltar → Grabschmiede → weitere Ghulbauten/Altäre/Schmieden; Knochentürme (Stufe-abhängig); bei Überschuss
  (Gold > 600) zusätzliche Produktionsgebäude bis vier je Art. Fällt ein Bauarbeiter, übernimmt ein anderer die Baustelle.
* **Startvorrat** 260 Gold, 200 Holz; Schrein und Wachen der Mission bleiben als Besatzung (Wache `Guard`).
* **Schmiede**: Die KI erhält zeitgesteuert Upgrades (Waffen, Rüstung, Geschosse im Wechsel, höchstens 9 Schritte; `tech_interval`).

### 10.2 Armeezusammenstellung nach Konterlogik

Die KI misst die Spielerarmee (Anteile Kavallerie, Fernkämpfer, Heavy, Medium, Light, Belagerung, Unterstützung, Befestigungen) und gewichtet ihre Einheiten:

| Einheit | Grundgewicht | Faktor |
|---|---|---|
| Hohlknecht | 1,0 | × (1 + 1,5 · Medium) |
| Ghul | 0,8 | × (1 + 2 · Fernkämpfer + Belagerung + Unterstützung) |
| Knochenschütze | 0,8 | × (1 + 1,5 · Light) |
| Aschhexe | 0,5 | × (1 + 2 · Heavy + 1,5 · Kavallerie) |
| Grabkoloss | 0,35 | × (1 + 4 · Befestigungen + Heavy) |
| Fledermausschwarm | 0,45 | × (1 + 1,5 · Belagerung + Unterstützung) · max(0,2; 1,2 − Fernkämpfer) |

Mit Wahrscheinlichkeit `counter_skill` wird proportional zum **Quadrat** der Gewichte gewürfelt, sonst gleichverteilt unter den bezahlbaren Einheiten. Die Armeegröße
wächst mit dem nächsten Angriff: Zielgröße `(8 + 3 · bisherige Angriffe) · army_mult`, Ausbildung bis 150 % davon (höchstens `max_army`).

### 10.3 Angriff, Verteidigung, Rückzug

* **Angriff**: Ab `first_attack` und genügender Armeegröße marschiert die gesamte Armee (Formation *Keil*, Leicht: *Block*) per Angriffsbewegung auf das nächste Spielergebäude (Mauern/Tore zählen nicht
  als Ziel; auf *Albtraum* sendet die KI zusätzlich ihre schnellen Einheiten (Tempo ≥ 3,2) zuerst gegen Arbeiter). Meldung: Horn und Chronik „… marschierten mit N Kriegern gegen Brennhollow“.
* **Weiterziehen**: Sind mindestens die Hälfte der Angreifer untätig, wählt die KI das nächste Ziel; unter drei Überlebenden endet der Angriff (nächster nach `attack_interval`).
* **Rückzug**: Ist die eigene Stärke (Σ √(LP · DPS)) in 12 Kacheln Umkreis kleiner als `retreat_ratio` × Gegnerstärke, zieht die KI 25 s zur Basis zurück („Die Hohlen wichen vor der Übermacht zurück.“).
* **Verteidigung**: Spielereinheiten in 16 Kacheln um den Schrein werden von allen Soldaten angegriffen (Vorrang vor Angriffen); danach kehren sie zum Sammelpunkt zurück.
* Diener der Hexen folgen ihrer Herrin.

### 10.4 Schwierigkeitsgrade (`rules.ron → ai_levels`)

| | Leicht | Normal | Schwer | Albtraum |
|---|---|---|---|---|
| Entscheidung alle | 3,0 s | 2,0 s | 1,5 s | 1,0 s |
| Arbeiter | 5 | 8 | 10 | 12 |
| Einnahmenfaktor | 0,7 | 0,9 | 1,25 | 1,6 |
| Armeegröße (Faktor / Maximum) | 0,55 / 12 | 0,85 / 24 | 1,3 / 44 | 1,6 / 60 |
| Erster Angriff | 11:00 | 8:00 | 5:30 | 4:20 |
| Angriffsabstand | 260 s | 190 s | 120 s | 90 s |
| Schmiede-Stufe alle | 15:00 | 8:00 | 5:00 | 3:30 |
| Konterlogik (`counter_skill`) | 15 % | 50 % | 85 % | 100 % |
| Rückzug bei Stärke unter | 30 % | 50 % | 65 % | 75 % |
| Türme | 0 | 1 | 3 | 4 |
| Belästigung (Arbeiter zuerst) | – | – | – | ja |

Die Stufen sind auch Wellen-Multiplikatoren im Wellenmodus (Leicht ×0,65, Normal ×1,0, Schwer ×1,4, Albtraum ×1,8). Gemessen mit passivem Spieler (Test `enemy_ai_builds_a_base_and_attacks`, Schwer):
innerhalb von neun Minuten ≥ 6 Arbeiter, ≥ 4 fertige Gebäude, ≥ 8 ausgebildete Soldaten, mindestens ein Angriff erreicht Brennhollow.

### 10.5 Balancemessung gegen einen aktiven Testspieler

Werkzeug: `cargo run -p ashen_tools -- rts-balance [--seeds 12] [--minutes 30]` (Code: `ashen_sim::balance`, Test `tests/balance.rs`). Ein skriptgesteuerter
Testspieler der Glutkrone spielt Mission 1 im Gefechtsmodus gegen die KI: Arbeiter (10–14) sammeln Gold, Holz und Stein, Gehöfte, Kasernen, Schießplatz und
Stall werden gebaut, eine gemischte Armee wird ausgebildet, die Basis verteidigt, in Wellen (ab 10–16 Soldaten, je Seed ein anderer Stil) angegriffen. Die
Waffen- und Rüstungskette ist durch eine feste Lieferung ersetzt (1,3 Waffen und 0,8 Rüstung je 10 s ab Minute 1), der Testspieler nutzt keine Taktik-KI und keine Heldenfähigkeiten.
Punktquote = (Siege + ½ · Zeitlimit) / Läufe des Testspielers; Zeitlimit 30 Minuten.

| Stufe | Vorher (6 Seeds: Siege / Niederlagen / Zeitlimit) | Nachher (12 Seeds) | Soll |
|---|---|---|---|
| Leicht | 4 / 0 / 2 (67 %) | 11 / 0 / 1 (**96 %**) | Spieler gewinnt meist |
| Normal | 0 / 3 / 3 (0 %, zu hart) | 3 / 4 / 5 (**46 %**) | ausgeglichen |
| Schwer | 1 / 5 / 0 (17 %) | 1 / 11 / 0 (**8 %**) | KI gewinnt meist |
| Albtraum | 0 / 6 / 0 (0 %) | 0 / 12 / 0 (**0 %**) | KI gewinnt meist |

Angepasst wurde nur in `rules.ron → ai_levels`: Leicht (Einnahmen 0,8 → 0,7, Armee 0,7/16 → 0,55/12, erster Angriff 10:00 → 11:00, Abstand 240 → 260 s) und Normal
(Einnahmen 1,0 → 0,9, Armee 1,0/28 → 0,85/24, erster Angriff 7:00 → 8:00, Abstand 160 → 190 s, Schmiede 7:00 → 8:00, Konterlogik 60 → 50 %); Schwer und Albtraum blieben unverändert.
Der Test `difficulty_levels_scale_against_an_active_player` sichert die Spannen (Leicht ≥ 66 %, Normal 25–75 %, Schwer ≤ 40 %, Albtraum ≤ 25 %, monoton).

**Konter-Matrix (1v1-Gruppen gleichen Gold-/Holz-/Steinwerts, 800 Wert, Angriffsbewegung, Restlebenspunkte):** Schwertkämpfer ×9 gegen Hohlknechte ×12 69 % : 0 %, Bogenschützen ×7 gegen Hohlschützen ×8 39 % : 0 %,
Speerkämpfer ×9 gegen Hohlknechte ×12 0 % : 63 %, Ritter ×3 gegen Ghule ×10 0 % : 69 %, Hellebardiere ×8 gegen Grabkolosse ×3 77 % : 0 %, Kampfmagier ×6 gegen Aschehexen ×5 0 % : 25 %,
Schwertkämpfer ×9 gegen Ghule ×10 40 % : 0 %, Bogenschützen ×7 gegen Ghule ×10 0 % : 57 %, Ritter ×3 gegen Hohlknechte ×12 0 % : 41 %. Die Ergebnisse folgen der Auslegung (Ghule schlagen Fernkämpfer und Reiter,
Hellebarden schlagen Wucht, Infanterie schlägt Ghule/Knechte); kein Paar vernichtet das andere nahezu verlustfrei (Test `counter_matrix_has_no_extreme_outliers_at_equal_cost`: Differenz der Restanteile < 85 %). Offensichtliche Ausreißer wurden nicht gefunden, daher keine Wertänderungen an Einheiten.

### 10.6 Taktische Führung in Weltkarten-Schlachten

Schlachten der Großen Kampagne laufen ohne Wirtschaft; der Gegner wird vom Modul `systems/tactics.rs` geführt (`worldmap::battle::drive_enemy`, alle 1,5 s; zustandslos, deterministisch):

* **Fokusfeuer**: Jede Einheit wählt unter Gegnern in Reichweite + 7 Kacheln das Ziel mit dem höchsten Wert (Heiler 3,4 > Belagerung 3,0 > Schützen 2,8 > Helden > Reiter > Infanterie > Arbeiter), verstärkt durch fehlende Lebenspunkte (bis ×3), geteilt durch Entfernung, mit Bonus für Ziele, die Kameraden schon angreifen. Das Ziel wird nur gewechselt, wenn ein anderes um 30 % besser ist.
* **Fernkämpfer hinter dem Nahkampf**: Schützen und Heiler stellen sich 2–3 Kacheln hinter den Schwerpunkt der eigenen Nahkämpfer und weichen Nahkämpfern aus, solange mindestens zwei eigene Nahkämpfer decken.
* **Rückzug**: Einheiten unter 30 % Leben (Helden 25 %) ziehen sich 9 Kacheln hinter die Front zurück und halten dort Stellung, solange mindestens vier gesunde Kameraden (≥ 50 %) kämpfen; sonst wird bis zum Ende gekämpft.
* **Belagerungswaffen** greifen Tore, dann Türme, dann Mauern, dann übrige Gebäude an.
* **Verteidiger einer Wehranlage** bemannen die Türme und Tore auf der Feindseite (Fernkämpfer, Befehl „Halten“), Nahkämpfer halten den Einfallspunkt (Wache mit Radius 7); bei einem Durchbruch (zwei Feinde im Inneren) greifen alle an.

Tests: `crates/ashen_sim/tests/tactics.rs` (Fokusfeuer, Fernkämpfer hinten, Rückzug, Belagerung, Turmbesatzung, Determinismus sowie Duelle gleicher Heere: die geführte Seite gewinnt mindestens 5 von 6 Duellen gegen reine Angriffsbewegung).

## 11. Bedienung

Ausbildungs-, Bau- und Fähigkeitskürzel stammen aus den Daten (`hotkey` an Einheiten und Gebäuden, Buchstaben A–Z; Fähigkeiten `Q W E R T Y` der Reihe nach). `validate-data` und der Test `all_data_hotkeys_are_mapped_and_conflict_free` melden doppelte Kürzel je Gebäude (Ausbildungsliste, zusätzlich zu den festen Tasten `R` Sammelpunkt, `L` Lager, `C` Stadt) und doppelte Baukürzel je Fraktion.

| Aktion | Eingabe |
|---|---|
| Fähigkeiten (Held bzw. erste Einheit mit Fähigkeiten, wirkt für alle gleichartigen) | `Q` `W` `E` `R` `T` `Y` oder Befehlskarte; Zielzauber: danach Linksklick |
| Formation wechseln | `F` oder Schaltfläche |
| Heldenfenster | `H` (nur Helden gewählt oder nichts gewählt) · Klick auf Heldenporträt · Heldenleiste |
| Position halten | `H` (sonst) |
| Heldenleiste: Held wählen / Fenster | Klick / erneuter Klick |
| Ausbilden | Kürzel der Einheit (Schießstand: A, X · Stall: V, Z · Kapelle: Y · Magierturm: M · Werkstatt: N, O · Altar: H, T · Kaserne: S, P, A, K, D, G) |
| Forschen / Wiederbeleben | Befehlskarte des Gebäudes |
| Gefecht-Schwierigkeit | Menü → Neues Spiel → Gefecht |

## 12. Technische Anmerkungen

* Logik: `crates/ashen_sim/src/systems/{combat,units,abilities,casters,heroes,formation,ai_player,movement,economy,commands}.rs`, Daten `rpg.rs`
  (Gegenstände, Talente, Upgrades, Formationen), Befehle in `command.rs`, Ereignisse in `events.rs`.
* Spielstandformat **4** (neu: Upgrades je Spieler, Einheiten mit Talenten/Ausrüstung/Inventar/Formation/Diener, gefallene Helden, Beute am Boden, KI-Zustand, Forschung je Gebäude);
  die Typtabelle enthält zusätzlich Gegenstände (`i:`) und Upgrades (`p:`).
* Tests: `crates/ashen_sim/tests/military.rs` (Konter, Katapult gegen Gebäude/Fläche, Mindestreichweite, Rammbock, Flieger, Heilung, Beschwörung, Feuerball, Talente, Ausrüstung,
  Tränke, Beute/Aufnahme, Wiederbeleben am Altar, Altarheld, Speichern/Laden mit Inventar, Formationen, Schmiede-Upgrades und Burg-Sperre, Ausrüstungskosten, KI-Aufbau/Angriff,
  Determinismus und Schwierigkeitsunterschiede, Konterlogik, KI nur im Gefecht), `systems/formation.rs` (Abstände).
* Kontaktbogen und Screenshots: `ASHEN_DUMP_SHEET=/tmp/sheet.png ashen_crowns --screenshot x.png`; `--demo army|siege|hero` (Sandbox/Mission) zeigt alle neuen Einheiten und Gebäude, eine Belagerungsschlacht
  und das Heldenfenster; `--difficulty` setzt die Gefechtsstufe.
