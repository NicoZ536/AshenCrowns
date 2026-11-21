# ASHEN CROWNS – Kampagnendesign
## „Das Zeitalter der Gefallenen“ (Grand Campaign)

| Feld | Wert |
|---|---|
| Dokument | `docs/design/campaign.md` |
| Status | Verbindliche Designgrundlage, Version 1.0 |
| Bezug | `docs/canon.md` (Namen und IDs), `docs/design/game_design_document.md` (Systeme, Werte) |
| Umfang | Prolog, 7 Zeitalter, 29 Missionen, 16 Entscheidungspunkte, 6 Enden |
| Spieldauer | 40–60 Stunden (Story-Missionen ca. 22 h, freies Weltkartenspiel zwischen den Missionen ca. 20–35 h) |

---

## 1. Überblick

### 1.1 Erzählerische Prämisse

Im Jahr 897 n.F. ist die Aschmark ein sterbendes Grenzland. Brennhollow ist die letzte bewohnte Siedlung zwischen
den fruchtbaren Tälern Valemarks und den verfluchten Ebenen, über die nachts die Hohlen ziehen. Isolde Raventhorn,
Wächterin der Aschmark, Tochter eines gefallenen Grenzlords, hält mit dreißig Seelen und einem alten
Flammenpriester die Stellung. Die Kampagne begleitet ihre Dynastie über rund 80 Jahre und mindestens zwei
Generationen: vom Überleben einer einzelnen Siedlung bis zur Entscheidung über das Schicksal Eldraeths in den
Ruinen von Solmere, wo Varkhan Aurelith, der Letzte Sonnenkönig, als Hohler König auf seinem Thron wartet.

### 1.2 Themen

| Thema | Ausprägung |
|---|---|
| **Pflicht gegen Überleben** | Isolde muss wieder und wieder zwischen dem Richtigen und dem Notwendigen wählen |
| **Erbe** | Was hinterlässt man denen, die nach einem kommen? Die zweite Hälfte der Kampagne spielt mit Isoldes Erben |
| **Die Versuchung der Unsterblichkeit** | Varkhan wollte den Tod überwinden und vernichtete dabei sein Reich; die Hohlung bietet dem Spieler dieselbe Versuchung |
| **Verstummte Götter** | Ist die Welt verlassen, oder haben die Götter nur aufgehört zu sprechen, weil niemand mehr zuhört? |

### 1.3 Struktur

- Jedes Zeitalter besteht aus **3–5 Story-Missionen** (handgebaute RTS-Karten) und einer **Weltkartenphase**
  zwischen den Missionen, in der der Spieler frei wirtschaftet, diplomatiert und Kriege führt.
- Missionsknoten erscheinen auf der Weltkarte als leuchtende Marker. Manche Missionen sind zeitkritisch
  (Ablaufdatum in Spieljahren), manche haben Voraussetzungen (Bündnisse, Gebäude, Variablen).
- **Fortschrittsbedingung** eines Zeitalters: Alle Pflichtmissionen abgeschlossen. Optionale Missionen verändern
  Variablen und Belohnungen.
- Die Weltkarte des folgenden Zeitalters übernimmt den vollständigen Zustand (Provinzen, Dynastie, Armeen,
  Meinungen, Chronik). Es gibt keinen Neustart zwischen Zeitaltern.
- Zu Beginn jedes Zeitalters zeigt eine Zwischensequenz in Pixel-Art-Standbildern mit Erzählerstimme (`narrator`)
  die Zusammenfassung der bisherigen Chronik – mit den tatsächlichen Ereignissen der eigenen Partie.

### 1.4 Zeitstrahl

| Zeitalter | Titel | Jahre (n.F.) | Herrscherin/Herrscher (Standardverlauf) | Kernkonflikt |
|---|---|---|---|---|
| Prolog | – | 0 und 893 | Varkhan Aurelith; Markgraf Edran Raventhorn | Der Aschenfall; der Fall von Grauwall |
| I | *The Last Settlement* – Die letzte Siedlung | 897–898 | Isolde Raventhorn (34) | Überleben gegen die Hohlen |
| II | *The Rising Crown* – Die aufsteigende Krone | 899–905 | Isolde | Aufbau, Anerkennung, Heirat |
| III | *The Broken Borders* – Die gebrochenen Grenzen | 906–915 | Isolde | Bündnisse gegen den Durchbruch der Hohlen |
| IV | *The Age of Conquest* – Das Zeitalter der Eroberung | 916–930 | Isolde, Herzogin der Aschmark | Expansion, Aschglas, Drache |
| V | *The Divided Throne* – Der geteilte Thron | 931–945 | Isolde, dann ihr Erbe | Tod König Aldrics III., Erbfolgekrieg, Generationswechsel |
| VI | *The Fallen Gods* – Die gefallenen Götter | 946–960 | Erbe/Erbin Raventhorn | Wiederherstellung der Götterschreine, Wahrheit über Varkhan |
| VII | *The Last Crown* – Die letzte Krone | 961–975 | Erbe/Erbin Raventhorn oder die Enkelgeneration | Erwachen des Hohlen Königs, Feldzug nach Solmere |

Der Begriff **„der Erbe“** bezeichnet in den Zeitaltern VI und VII den jeweils regierenden Raventhorn. Der Erbe aus
Zeitalter V wird 901 n.F. geboren (die Heirat aus E6 findet im Jahr 900 statt); um 960 übernimmt in der Regel die
dritte Generation, deren Name und Eigenschaften die Dynastiesimulation erzeugt. Story-Missionen sind für beide Fälle
vertont (Anrede über Titel statt Vornamen).

### 1.5 Kampagnenvariablen

Entscheidungen setzen **Kampagnenvariablen**, die in späteren Missionen, auf der Weltkarte und in den Enden
gelesen werden. Variablen werden in `data/campaign/variables.ron` definiert; ihre Namen sind verbindlich.

| Variable | Typ | Werte | Gesetzt in | Gelesen in |
|---|---|---|---|---|
| `flag_m01_dead_burned` | bool | true/false | m01 (E1) | m01 Nacht 2, m24 |
| `var_refugees` | enum | `all`, `healthy`, `none` | m02 (E2) | Weltkarte II, m11, m19, m25 |
| `flag_nel_sheltered` | bool | – | m02 (E2) | m11, m24, Ende „Erlösung“ |
| `var_bandits` | enum | `hanged`, `recruited`, `extradited` | m03 (E3) | m13, m19 |
| `var_allegiance` | enum | `crown`, `independent`, `corvane` | m04 (E4) | gesamte Zeitalter II–V |
| `var_kassian_secret` | enum | `warned_king`, `leverage`, `silent` | m06 (E5) | m14, m17, m18, m27 |
| `var_marriage` | enum | `skarn`, `corvane`, `saelith`, `none` | m08 (E6) | Erbe in V, Verbündete in III/VII |
| `var_root_mother` | enum | `pact`, `slain`, `truce` | m10 (E7) | m23, m27, Ende „Wiederkehr“ |
| `var_rift` | enum | `harvested`, `sealed` | m11 (E8) | Hohlung, m24, m29 |
| `var_council_lead` | enum | `raventhorn`, `torvald`, `shared` | m12 (E9) | m27, Bundstärke VII |
| `var_age4_war` | enum | `against_mireille`, `with_mireille` | m14 (E10) | V, m18, m19 |
| `var_dragon` | enum | `tribute`, `slain`, `redirected` | m15 (E11) | Weltkarte IV–VII, m28 |
| `var_regicide_blame` | enum | `kassian`, `mireille`, `heretics`, `truth` | m17 (E12) | m18, m27, m25 |
| `var_throne` | enum | `mireille`, `regency`, `raventhorn` | m18 (E13) | V–VII, Ende „Thron aus Glut“ |
| `var_isolde_fate` | enum | `fallen`, `abdicated`, `ashglass_heart` | m20 (E14) | VI–VII, m29 |
| `var_sun_heresy` | enum | `truth`, `faith`, `used` | m25 (E15) | m28, m29, Ende „Erlösung“ |
| `var_final_choice` | enum | `destroy`, `redeem`, `seal`, `crown` | m29 (E16) | Ende |
| `rep_crown`, `rep_flame`, `rep_skarn`, `rep_thorn`, `rep_saelith`, `rep_corvane` | int | −100…+100 | viele | Startmeinung der jeweiligen Fraktion auf der Weltkarte |
| `var_shrines_restored` | int | 0–5 | VI | Ende „Wiederkehr der Stimmen“ |
| `var_isolde_hollowing` | int | 0–100 | m11, m20 | Persönliche Hohlung Isoldes (= Figurenwert) |

### 1.6 Tod von Kernfiguren

Die Kampagne ist so gebaut, dass sie den Tod von Kernfiguren (bei Permadeath „Standard“ oder höher) übersteht:

| Figur | Wenn sie stirbt | Ersatz |
|---|---|---|
| Isolde Raventhorn (`hero_isolde`) | Vor Zeitalter V: Die Dynastie geht an ihr ältestes Kind über (falls minderjährig: Regentschaft durch Bruder Hal Venn oder den Ehepartner); Missionen mit Isolde verwenden das Kind bzw. ihren Ehepartner als Heerführer. In m01–m04 kann Isolde nicht endgültig sterben, außer auf Permadeath „Aschenfall“ – dann übernimmt ihr Knappe Tamsin Hale (Ersatzheld mit eigener Vertonung) | Erbe / Tamsin Hale |
| Bruder Hal Venn (`char_hal_venn`) | Seine Mentorrolle geht an eine Flammenpriesterin aus Kaldris über (Schwester Odile Marrant); Weisheitsdialoge entfallen, Glutschul-Forschung −10 % | Odile Marrant |
| König Aldric III. (`char_king_aldric`) | Stirbt laut Story 931 (m17); früherer Tod durch Spieleraktionen verschiebt Zeitalter V entsprechend nach vorn | – |
| Herzogin Mireille Corvane (`char_mireille`) | Ihr Haus wird von ihrem Bruder Lucan Corvane geführt (schwächer, aber rachsüchtig) | Lucan Corvane |
| Torvald Eisenhand, Sylaen, Ophira, Kassian | Nachfolger der jeweiligen Fraktion übernehmen Rolle und Meinung mit −20 Abschlag | Fraktionsnachfolger |

---

## 2. Prolog

Der Prolog besteht aus zwei kurzen spielbaren Sequenzen (zusammen 20–25 Minuten), die Steuerung und Ton einführen.

### 2.1 Prolog A – „Asche über Solmere“ (`p01_ashfall`), Jahr 0

| Aspekt | Inhalt |
|---|---|
| Spielfigur | Ein namenloser Hauptmann der Sonnengarde (Einheitensteuerung ohne Wirtschaft) |
| Ziel | Mit 20 Gardisten und einem Wagen voller Flüchtlinge durch das brennende Solmere zum Westtor fliehen |
| Lehrinhalt | Auswählen, Bewegen, Angriffsbewegung, Formation, Kamerasteuerung |
| Besonderheit | Der Himmel zerreißt (Schleierriss als Sequenz), Bürger verwandeln sich vor den Augen des Spielers in die ersten Hohlen; das Ende ist unabwendbar: Der Hauptmann bleibt am Tor zurück und hält es, damit der Wagen entkommt |
| Story-Beat | Im Thronsaal über der Stadt erhebt sich Varkhan Aurelith vom Ritualkreis. Er sieht, was er getan hat, und schweigt. Die Stimme aus dem Tief spricht zum ersten Mal: „Du wolltest den Tod verschließen. Du hast mir die Tür geöffnet.“ |

### 2.2 Prolog B – „Der Fall von Grauwall“ (`p02_grauwall`), Jahr 893 n.F.

| Aspekt | Inhalt |
|---|---|
| Spielfigur | Die 22-jährige Isolde Raventhorn (Stufe 1, nur Glutspalter) und ihr Vater Markgraf Edran Raventhorn, Grenzlord der Aschmark |
| Ziel | Die Grenzfeste Grauwall eine Nacht lang halten, dann die Bewohner nach Brennhollow evakuieren |
| Lehrinhalt | Heldensteuerung, Fähigkeiten, Tag/Nacht, Moral, Rückzug |
| Besonderheit | Die Mission kann nicht gewonnen werden: Um Mitternacht fällt die Feste. Markgraf Edran befiehlt den Rückzug und bleibt mit dem Vogt der Feste, Aldous Meringer, zurück. Isolde erkrankt auf der Flucht am Aschenfieber (daher ihre Narbe) |
| Story-Beat | Letzte Einstellung: Vogt Aldous Meringer führt die Überlebenden aus dem brennenden Grauwall nach Osten. Ein Jahr später sucht er mit ihnen Schutz in den Klosterruinen von Vael Dorn – und kehrt als Fahler Vogt (`boss_pale_reeve`) zurück, Hüter des dortigen Hohlschreins |

---

## 3. Die sieben Zeitalter

### 3.1 Zeitalter I – *The Last Settlement* (897–898)

**Ziel des Zeitalters**: Brennhollow vor dem Untergang bewahren, die unmittelbare Hohlenbedrohung in der Aschmark
brechen und die Verbindung nach Valemark wiederherstellen.

| Kategorie | Neu eingeführt |
|---|---|
| Mechaniken | RTS-Grundlagen, Ressourcen (`gold`, `wood`, `stone`, `food`), Bevölkerung (`population`/`population_cap`), Tag-Nacht-Zyklus, Bindung der Hohlen, Heldenfähigkeiten, Veteranenstufen, erste Entscheidungen |
| Einheiten | `worker`, `swordsman`, `spearman`, `archer`, `knight`, Heldin `hero_isolde`; Gegner `hollow_thrall`, `hollow_archer`, `grave_brute`, `boss_pale_reeve` |
| Gebäude | `keep`, `farmstead`, `barracks`, `watchtower`, Palisade; Gegner `hollow_shrine`, `bone_spire` |

| # | Mission | ID | Ziel | Besonderheit |
|---|---|---|---|---|
| 1 | **Die letzte Siedlung** | `m01_last_settlement` | Wirtschaft aufbauen, Armee ausbilden, drei Nächte überstehen, den Hohlschrein von Vael Dorn zerstören | Vertical Slice; siehe Abschnitt 6 im Detail |
| 2 | **Die Flüchtlinge von Harrowmere** | `m02_harrowmere` | Einen Flüchtlingszug (60 Menschen, 6 Wagen) aus dem gefallenen Dorf Harrowmere durch das Aschmoor nach Brennhollow geleiten | Eskortmission mit Ascheregen; die Wagen sind langsam und müssen nachts von Fackelträgern beleuchtet werden; Entscheidung E2 am Ende |
| 3 | **Die Graue Straße** | `m03_grey_road` | Die alte Handelsstraße nach Valemark von Raubrittern und Aschwölfen befreien, drei Straßenposten errichten | Erste Mission mit mehreren Lagern; Banditenhauptmann „Ser Wendel der Graue“ kann gefangen genommen werden; Entscheidung E3 |
| 4 | **Die Lange Nacht** | `m04_long_night` | Brennhollow eine 20-minütige Nacht lang gegen die größte Hohlenwelle des Zeitalters verteidigen, bis die Boten des Königs eintreffen | Survival-Mission mit vorbereiteter Stadt aus m01 (übernommener Ausbau); am Morgen trifft der Gesandte König Aldrics III. ein; Entscheidung E4 |

**Story-Beats**

1. Brennhollow steht kurz vor dem Ende; Isolde entscheidet sich zu bleiben statt zu fliehen.
2. Der Fahle Vogt wird als Aldous Meringer erkannt, der einstige königliche Vogt der Aschmark. Isolde erfährt in seinem Tod, dass
   ihr Vater nicht als Hohler weiterexistiert – er hat sich selbst verbrannt, bevor die Hohlung ihn nehmen konnte.
3. Die Flüchtlinge aus Harrowmere bringen das Mädchen Nel mit, das im Schlaf mit der Stimme aus dem Tief spricht.
4. Auf der Grauen Straße findet Isolde Briefe, die belegen, dass Raubritter mit Gold aus Nordmark bezahlt werden.
5. Nach der Langen Nacht trifft der königliche Gesandte ein: Kassian Vale. Er überbringt die Aufforderung, in
   Kaldris den Treueeid zu leisten – und ein Lächeln, das mehr weiß, als es sagt.

**Weltkartenphase I** (nach m03): Einführung der Weltkarte mit nur der Aschmark (5 Provinzen), Statthalter,
Provinzausbau, erste Handelsroute nach Valemark.

---

### 3.2 Zeitalter II – *The Rising Crown* (899–905)

**Ziel des Zeitalters**: Brennhollow zum Marktflecken (≥ 200 Einwohner) ausbauen, die Aschmark wirtschaftlich
tragfähig machen, Isoldes Stellung am Hof von Valemark sichern und die Dynastie durch Heirat begründen.

| Kategorie | Neu eingeführt |
|---|---|
| Mechaniken | Volle Weltkarte Valemarks, Produktionsketten (Brot, Holz, Metall), Markt und Preise, Steuern, erste Ämter (Kanzler, Marschall, Schatzmeister, Statthalter), Dynastie und Heirat, Meinungsmodell, Intrigen (passiv: Komplotte gegen den Spieler werden sichtbar) |
| Einheiten | `crossbowman`, `scout_rider`, `flame_priest`; Ritterschwur-Linie ausgebaut (`knight` mit Veteranen-Upgrades) |
| Gebäude | `windmill`, `bakery`, `sawmill`, `quarry`, `smelter`, `weaponsmith`, `marketplace`, `storehouse`, Flammentempel, Steinmauer, Bergfried Stufe II und III |

| # | Mission | ID | Ziel | Besonderheit |
|---|---|---|---|---|
| 5 | **Ein Markt aus Asche** | `m05_market_of_ash` | Brennhollow auf 200 Einwohner bringen, einen Marktplatz errichten und den ersten Saelith-Handelszug empfangen | Aufbaumission mit Anno-artigen Bedürfnissen; Ophira Saelith-Varo besucht persönlich und bietet einen Kredit an (annehmbar, mit Folgen für `rep_saelith`) |
| 6 | **Das Turnier von Kaldris** | `m06_tourney_kaldris` | Im Turnier der Glutkrone bestehen (Heldenduelle und Gruppenkämpfe in einer Arena) und gleichzeitig einen Anschlag auf König Aldric aufklären | Zweigeteilte Mission: Arena-Kämpfe (Mikro-Herausforderung) und Erkundung des nächtlichen Kaldris; Kassian Vale bietet Wissen an; Entscheidung E5 |
| 7 | **Eisen in den Hügeln** | `m07_iron_hills` | Die Eisenmine von Kargsted einnehmen, die ein Raubritter mit einer Steinfeste hält | Erste echte Belagerung (Rammbock, Sturmleitern, Untergraben); Weltkarten-Belagerungsfortschritt der Vorphase öffnet Breschen |
| 8 | **Der Brautzug** | `m08_bridal_procession` | Den Brautzug (je nach Wahl nach Skarnheim, Nordmark, an die Küste oder – bei Ablehnung aller Werber – eine Adoptionsfeier in Brennhollow) gegen einen Überfall verteidigen | Missionskarte hängt von E6 ab (vier Varianten); der Überfall wird von Hohlen oder von gedungenen Mördern verübt (abhängig von `var_kassian_secret`) |

**Story-Beats**

1. Brennhollow wächst; zum ersten Mal seit Jahrzehnten wird in der Aschmark ein Kind auf einem Markt geboren statt
   in einem Keller.
2. In Kaldris begegnet Isolde König Aldric III. – müde, klug, misstrauisch gegen seine eigenen Herzöge.
3. Herzogin Mireille Corvane behandelt Isolde als Gleichgestellte und Rivalin zugleich.
4. Kassian Vale enthüllt ein Komplott Mireilles gegen den König und lässt Isolde wählen, was sie damit tut.
5. Die Heirat (oder ihre Verweigerung) legt fest, welche Fraktion die Dynastie Raventhorn als Familie betrachtet.

---

### 3.3 Zeitalter III – *The Broken Borders* (906–915)

**Ziel des Zeitalters**: Den Durchbruch der Hohlen an drei Fronten (Aschmark, Skarnheim, Dornwaldrand) aufhalten
und dafür Bündnisse mit Völkern schmieden, die Valemark seit Jahrhunderten misstrauen.

| Kategorie | Neu eingeführt |
|---|---|
| Mechaniken | Diplomatie mit allen Fraktionen, Bündnisse, Bedrohung und Koalitionen, Schleierstärke als sichtbarer Kartenmodus, Magieforschung (Glutschule Grad I–II, je nach Bündnis Stein- oder Wachstumsschule), Rituale, Wetter (Schnee) |
| Einheiten | `ember_adept`, `ember_mage`, `halberdier`, `mangonel`; Bündniseinheiten `skarn_ironguard`/`skarn_rune_thrower` oder `veyl_stalker`/`thorn_warden`; neue Gegner `veil_wraith`, `hollow_priest` |
| Gebäude | Magierturm, Bibliothek, Skriptorium, Zeughaus, Wehrhof, Wachfeuer-Ritualplatz |

| # | Mission | ID | Ziel | Besonderheit |
|---|---|---|---|---|
| 9 | **Der Schwarze Winter** | `m09_black_winter` | Torvald Eisenhand helfen, die Bergfeste Hrimgard gegen eine Hohlenarmee im Schneesturm zu halten | Schnee und Kälte (Lebende ohne Winterausrüstung verlieren HP), Steinkunst-Runenwälle als Verbündetenfähigkeit; Skarn-Einheiten steuerbar |
| 10 | **Wurzel und Asche** | `m10_root_and_ash` | Sylaen der Veyl im Dornwald treffen und die Ursache des Welkens am Waldrand finden | Erkundungsmission mit Heldentrupp (Isolde + 6 Einheiten); Begegnung mit der Wurzelmutter (`boss_root_mother`) als Prüfung: Kampf oder Heilung der Wurzelknoten; Entscheidung E7 |
| 11 | **Der Riss von Marrowfen** | `m11_marrowfen_rift` | Einen offenen Schleierriss im Moor von Marrowfen schließen, aus dem Schleiergeister strömen | Erste Mission mit ätherischen Gegnern (Magie erforderlich); Nel (falls aufgenommen) kann den Riss „hören“ und zeigt den Weg; reiche Aschglasadern am Riss; Entscheidung E8 |
| 12 | **Der Graue Rat** | `m12_grey_council` | Vertreter der Glutkrone, der Skarn-Clans, des Dornenbunds und des Saelith-Pakts auf der neutralen Insel Grauholm zusammenbringen und den Rat gegen einen Hohlenangriff verteidigen | Politische Mission: Gespräche (Dialogwahlen mit Meinungsauswirkung) wechseln mit Verteidigungsphasen; Entscheidung E9 |

**Story-Beats**

1. Die Hohlen greifen erstmals koordiniert an – jemand führt sie. Leerpriester tauchen auf.
2. Torvald Eisenhand verlangt Ehre vor Bündnis; Sylaen verlangt Wahrheit vor Vertrauen; Ophira verlangt Zinsen.
3. Im Riss von Marrowfen spricht die Stimme aus dem Tief direkt zu Isolde und bietet ihr Kraft an.
4. Der Graue Rat gründet den Vorläufer des späteren Bundes – oder zerbricht an Valemarks Stolz.

---

### 3.4 Zeitalter IV – *The Age of Conquest* (916–930)

**Ziel des Zeitalters**: Die Aschmark zum Herzogtum erheben, die Aschglasfelder am Rand der Aurethwüste sichern
und sich gegen innere Rivalen und den Aschdrachen behaupten.

| Kategorie | Neu eingeführt |
|---|---|
| Mechaniken | Kriegsgründe, Kriegspunkte, Friedensverhandlungen, Vasallen und Adelshäuser, Adelsrat, Belagerungen über beide Ebenen, Aschglaswirtschaft, Großmacht-Vorstufen (Bedrohung), Weltbosse |
| Einheiten | `trebuchet`, Belagerungsturm, Ritter-Elitelinie; Gegner `ashen_legionnaire`, `cinder_knight`, `boss_ashen_dragon` |
| Gebäude | `ashglass_mine`, `ashglass_cutter`, `stud_farm`, `siege_workshop`, Zitadelle, Pflasterstraßen |

| # | Mission | ID | Ziel | Besonderheit |
|---|---|---|---|---|
| 13 | **Die Aschglasfelder** | `m13_ashglass_fields` | Drei Außenposten der Aschenlegion am Rand der Aurethwüste erobern und Aschglasschächte errichten | Erste Begegnung mit der Aschenlegion (Unsterbliche Legion: nur Feuer tötet endgültig); Hitze; „Graue Kompanie“ als Verbündete, falls `var_bandits = recruited` |
| 14a | **Die Belagerung von Nordwacht** | `m14a_siege_nordwacht` | Mireilles Festung Nordwacht belagern und einnehmen | Nur bei E10 = gegen Mireille; große Belagerung mit Tribok; Mireille kann gefangen werden |
| 14b | **Die Kaperflotte** | `m14b_corsair_fleet` | Gemeinsam mit Mireille die Küstenstädte Valemarks gegen Kaperer verteidigen, die von Saelith-Abtrünnigen finanziert werden | Nur bei E10 = mit Mireille; Küstenkarte mit Schiffen (Seegefechte vereinfacht als Fernkampfplattformen) |
| 15 | **Der Drachenzehnt** | `m15_dragon_tithe` | Die Aschglasfelder gegen Vyrmathax schützen | Weltboss-Begegnung (Flug- und Landephasen); Entscheidung E11 kann vor oder während des Kampfes getroffen werden |
| 16 | **Die Krone des Südens** | `m16_crown_of_the_south` | Isoldes Erhebung zur Herzogin der Aschmark in Brennhollow feiern und die Zeremonie gegen einen Angriff der Aschenlegion verteidigen | Städtische Verteidigung in der nun großen Stadt Brennhollow (Stadtkarte des Spielers); der Erzflammenwahrer aus Kaldris ist anwesend (muss überleben) |

**Story-Beats**

1. Isolde wird von der Verteidigerin zur Eroberin – und spürt, wie leicht das geht.
2. Die Aschenlegion trägt die Wappen Aureths; ihre Glutritter sprechen von ihrem König, der „bald erwacht“.
3. Vyrmathax ist kein Tier, sondern ein uralter, sprechender Herrscher, der Aschglas frisst und Menschen verachtet.
4. Bei der Krönungszeremonie erleidet König Aldric einen Schwächeanfall – der Hof beginnt, über die Nachfolge zu flüstern.

---

### 3.5 Zeitalter V – *The Divided Throne* (931–945)

**Ziel des Zeitalters**: Den Erbfolgekrieg nach dem Tod König Aldrics III. überstehen, über die Zukunft des Throns
von Valemark entscheiden und die Herrschaft der Dynastie Raventhorn an die nächste Generation übergeben.

| Kategorie | Neu eingeführt |
|---|---|
| Mechaniken | Aktive Intrigen (eigene Komplotte), Geheimnisse und Druckmittel, Fraktionen, Bürgerkrieg, Erbgesetze, Regentschaft, Generationswechsel, alle sieben Ämter (Diplomat, Spionagemeister, Hohepriester kommen hinzu) |
| Einheiten | Erbe/Erbin als neuer Held (eigene Talentbäume, abhängig von `var_marriage`); `saelith_arbalest`-Söldner; Rebellenheere |
| Gebäude | Herrenhaus, Kerker, Gesandtschaft, Spionagenetz-Gebäude (Schattenkontor) |

**Der Erbe** (abhängig von `var_marriage`):

| `var_marriage` | Erbe/Erbin | Prägung | Eigene Heldenausrichtung |
|---|---|---|---|
| `skarn` | Bjarna Raventhorn, Tochter Isoldes und Torvald Eisenhands | Tapfer, Jähzornig | Schwere Infanterie und Runenkunst |
| `corvane` | Corwin Raventhorn-Corvane, Sohn Isoldes und Lucan Corvanes | Ehrgeizig, Verschlagen | Diplomatie und Intrige, Feldherr |
| `saelith` | Teodor Saelith-Raventhorn, Sohn Isoldes und Davian Saelith-Varos | Gierig, Geduldig | Handel, Gezeitenmagie, Söldnerführung |
| `none` | Wren Raventhorn, Waise aus Brennhollow (geb. 896), von Isolde adoptiert | Gerecht, Fromm | Glutklinge, Flammenmagie (Legitimität −10, kompensiert durch +20 Meinung beim Volk) |

| # | Mission | ID | Ziel | Besonderheit |
|---|---|---|---|---|
| 17 | **Tod eines Königs** | `m17_death_of_a_king` | Im Flammendom von Kaldris die Umstände von König Aldrics Tod untersuchen, Beweise sammeln und den Hof vor einer Panik bewahren | Ermittlungsmission (Gespräche, Spuren, nächtliche Verfolgungen in den Gassen); Beweisstücke beeinflussen die verfügbaren Optionen von E12 |
| 18 | **Der Lange Rat** | `m18_long_council` | Im Adelsrat von Valemark über die Nachfolge abstimmen lassen | Weltkarten-Mission (Politik und Komplotte über 6 Spielmonate); Mehrheiten müssen über Gefallen, Druckmittel und Heirat gesichert werden; Entscheidung E13 |
| 19 | **Die Furt von Kaldris** | `m19_kaldris_ford` | Die Entscheidungsschlacht des Erbfolgekriegs an der Furt vor Kaldris schlagen | Größte Feldschlacht der Kampagne bis dahin (Frontbreite, Reserven); Gegner abhängig von E13; Sonnenhäretiker erheben sich im Rücken, falls `var_refugees = none` |
| 20 | **Die Erbin der Glut** | `m20_ember_heir` | Brennhollow ein letztes Mal unter Isoldes Befehl gegen einen Hohlenangriff verteidigen und die Herrschaft übergeben | Generationswechsel als spielbare Mission im Jahr 934: Isolde (71 Jahre, Werte durch Alter gemindert) und der Erbe kämpfen gemeinsam; Entscheidung E14 |

**Story-Beats**

1. König Aldric III. stirbt im Winter 931 – vergiftet, wie sich zeigt.
2. Kassian Vale verschwindet in derselben Nacht; jede Fraktion beschuldigt eine andere.
3. Mireille Corvane erhebt Anspruch auf den Thron; Aldrics Enkel, Prinz Aldren, ist erst elf Jahre alt.
4. Isolde muss entscheiden, ob sie Königsmacherin, Königin oder Regentin wird.
5. Am Ende des Zeitalters übergibt Isolde das Schwert der Raventhorns. Wie sie es tut, prägt den Rest der Kampagne.

---

### 3.6 Zeitalter VI – *The Fallen Gods* (946–960)

**Ziel des Zeitalters**: Den steigenden Schleierdruck brechen, indem die fünf Schreine der alten Götter
wiederhergestellt werden, und die Wahrheit über Varkhan Aurelith und den Aschenfall aufdecken.

| Kategorie | Neu eingeführt |
|---|---|
| Mechaniken | Globaler Schleierdruck, Götterschreine, Erzrituale (Grad IV), Religion aktiv (Bekehrung, Häresie, Erzflammenwahrer-Politik), Heldenquests in Ruinen, Seefahrt (Saelith) |
| Einheiten | Kampfmagier aller Schulen (je nach Bündnis), Götterschrein-Wächter, `saelith_marine`; Gegner: Hohle Hohepriester, Aschenlegions-Banner |
| Gebäude | Götterschreine (5), Magierakademie, Flammendom-Erweiterung, Hafen und Werft |

| # | Mission | ID | Ziel | Besonderheit |
|---|---|---|---|---|
| 21 | **Der Schrein Oruuns** | `m21_shrine_oruun` | Den Steinschrein unter Skarnheims Gipfeln aus einem Hohlennest befreien und neu weihen | Mehrstöckige Karte (Stollen und Hallen); Runenmeister-Verbündete; Erdbeben als Umwelteffekt |
| 22 | **Die Gezeiten Thals** | `m22_tide_of_thal` | Den Gezeitenschrein auf einer versunkenen Insel der Saelith bei Ebbe erreichen und vor der Flut weihen | Zeitdruck durch Gezeiten (Karte verändert sich alle 4 Minuten); Ophira oder ihre Nachfolgerin stellt Schiffe |
| 23 | **Der Herzhain** | `m23_heart_grove` | Den Schrein Calyx' im Herzhain wiederherstellen | Verlauf abhängig von `var_root_mother`: bei `pact` friedliche Ritualmission mit Verteidigung gegen Hohle; bei `slain` muss ein verdorrter Hain mit Wachstumsmagie neu belebt werden (deutlich schwerer); bei `truce` Prüfung durch die Wurzelmutter |
| 24 | **Morvaines Tor** | `m24_morvaines_gate` | Am Rand der Schleiernarbe Morvaines Schrein – ein Tor zwischen Leben und Tod – öffnen und wieder schließen | Halb in der Welt der Toten: Gefallene früherer Missionen erscheinen (aus der Chronik der eigenen Partie generiert); `flag_m01_dead_burned` und `flag_nel_sheltered` verändern Ablauf und Hilfe |
| 25 | **Ithrens letztes Licht** | `m25_ithren_light` | In den Außenbezirken von Solmere den Schrein Ithrens finden, der unter dem Sonnenhof begraben liegt | Erste Mission in Solmere; Sonnenhäretiker kontrollieren die Ruinen; Aufdeckung der Wahrheit über Varkhan; Entscheidung E15 |

**Story-Beats**

1. Mit jedem Schrein kehrt ein Flüstern der Götter zurück – nicht als Worte, sondern als Veränderung der Welt.
2. In Morvaines Tor erfährt der Erbe, dass die Toten des Aschenfalls nie hinübergehen konnten: Varkhans Ritual
   verschloss den Übergang, und das Hohle Tief füllte die Lücke.
3. Unter dem Sonnenhof findet der Erbe Varkhans Tagebuch: Er wollte nicht sich selbst unsterblich machen, sondern
   seine sterbende Tochter retten. Das macht seine Tat nicht kleiner – aber menschlicher.
4. Die Sonnenhäretiker glauben, Varkhan werde als Erlöser zurückkehren. Sie haben auf eine Weise recht, die sie nicht
   verstehen: Er wird zurückkehren.

---

### 3.7 Zeitalter VII – *The Last Crown* (961–975)

**Ziel des Zeitalters**: Den Hohlen König in Solmere stellen, den Bund der Letzten Flamme anführen oder ihm dienen
und über das Schicksal Eldraeths entscheiden.

| Kategorie | Neu eingeführt |
|---|---|
| Mechaniken | Endgame-Bedrohungen (Erwachen des Hohlen Königs), Bund der Letzten Flamme mit gemeinsamer Kriegskasse, Großmacht-Reaktion (falls zutreffend), Mehrfrontenkrieg, Bossschlacht in Phasen über beide Ebenen |
| Einheiten | Elite- und Legendärversionen aller Linien, Verbündete aller Fraktionen (abhängig von Variablen), Wiederkehrende Helden (Tamsin Hale, Nel, Sylaen, Torvald oder deren Nachfolger) |
| Gebäude | Bundeslager (Feldbasis auf der Weltkarte), Glutfeuer-Leuchtfeuer (Gegenmaßnahme gegen Schleierbruch) |

| # | Mission | ID | Ziel | Besonderheit |
|---|---|---|---|---|
| 26 | **Der Aschenmarsch** | `m26_ashen_march` | Den ersten Marsch der erwachten Aschenlegion an der Grauen Straße aufhalten und Brennhollow evakuieren oder halten | Rückkehr zu den Orten aus Zeitalter I – Brennhollow ist nun eine Großstadt; Wahl zwischen Halten (schwer) und Evakuieren (Stadt verloren, Bevölkerung gerettet) |
| 27 | **Der Bund der Letzten Flamme** | `m27_last_flame_pact` | Die Fraktionen Eldraeths zum Bund vereinen | Weltkarten-Mission; Startmeinungen aus allen `rep_*`-Variablen, `var_council_lead`, `var_kassian_secret`, `var_root_mother`; jede gewonnene Fraktion stellt Truppen für m28 |
| 28 | **Die Tore von Solmere** | `m28_gates_of_solmere` | Die äußeren Mauern von Solmere stürmen | Größte Belagerung der Kampagne; Vyrmathax greift ein (für den Spieler bei `redirected` oder `tribute`, gegen ihn bei Provokation, gar nicht bei `slain` – dafür steht das Drachenherz-Artefakt zur Verfügung); Sonnenhäretiker öffnen ein Tor bei `var_sun_heresy = truth` |
| 29 | **Der Hohle Thron** | `m29_hollow_throne` | Varkhan Aurelith, den Hohlen König (`boss_hollow_king`), im Thronsaal stellen | Vierphasige Bossschlacht (Mauerhof, Sonnenhof, Schleierphase, Thronsaal); Entscheidung E16 im Thronsaal; verfügbare Optionen hängen von allen vorherigen Variablen ab |

**Story-Beats**

1. Das Hohle Tief erwacht durch Varkhan, nicht umgekehrt – er ist seine Tür, sein Gefangener und sein Werkzeug.
2. Alte Feinde stehen nebeneinander, oder sie tun es nicht – und der Spieler erntet, was die Dynastie gesät hat.
3. Im Thronsaal spricht Varkhan zum ersten Mal seit fast tausend Jahren mit einer menschlichen Stimme.
4. Die Stimme aus dem Tief bietet dem Erben die Krone an. Der Erbe muss wählen, was eine Krone wert ist.

---

## 4. Entscheidungspunkte

Jeder Entscheidungspunkt nennt die Wahlmöglichkeiten, die gesetzten Variablen sowie kurz- und langfristige
Konsequenzen.

### E1 – Die Toten von Brennhollow (`m01_last_settlement`)

Nach der ersten Nacht liegen gefallene Milizionäre und Bewohner im Hof. Bruder Hal Venn mahnt, die Toten zu verbrennen,
bevor die Hohlen sie holen; die Familien wollen sie nach altem Brauch auf dem Friedhof begraben.

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Verbrennen (Brauch der Flamme) | `flag_m01_dead_burned = true`, `rep_flame +5` | Kostet 40 `wood`; −5 Zufriedenheit für einen Tag; in Nacht 2 erheben sich keine Toten vom Friedhof | Brennhollow übernimmt die Feuerbestattung (Glaubensstärke +0,1 in der Aschmark); in m24 verlangt Morvaines Torwächter „ein Grab für die Verbrannten“ (zusätzliches Ziel), dafür keine Wiedergänger aus Brennhollow |
| Begraben mit Hal Venns Segen | `flag_m01_dead_burned = false` | Hal Venn ist 90 s mit der Weihe beschäftigt (keine Heilung am Bergfried); in Nacht 2 erheben sich 4 `hollow_thrall` vom Friedhof innerhalb der Palisade, weil die Weihe unvollständig blieb | In m24 erscheinen die Begrabenen von Brennhollow als hilfreiche Seelen und gewähren kostenlos *Morvaines Schwelle*; Zufriedenheit Brennhollow +5 dauerhaft (Ehre der Ahnen) |

### E2 – Die Flüchtlinge von Harrowmere (`m02_harrowmere`)

Unter den Flüchtlingen sind Menschen mit Zeichen der Hohlung, darunter das Mädchen Nel.

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Alle aufnehmen | `var_refugees = all`, `flag_nel_sheltered = true` | +60 Bevölkerung, Nahrungsdruck, Spannung Einheimische ↔ Flüchtlinge +20, Provinz-Hohlung +3 | Nel wird Seherin: in m11 Wegweiser, in m24 Mittlerin, Voraussetzung für das Ende „Erlösung des Sonnenkönigs“; `rep_flame −5` |
| Nur die Gesunden aufnehmen | `var_refugees = healthy` | +45 Bevölkerung, keine Hohlung | Die Abgewiesenen ziehen zur Sonnenhäresie; in m19 erscheinen kleinere Häretikergruppen; Nel verschwindet (taucht in m25 als Predigerin der Häresie auf) |
| Niemanden aufnehmen | `var_refugees = none` | Nahrung geschont, −10 Zufriedenheit (Scham), `rep_flame −10` | Starke Sonnenhäresie in der Aschmark (Weltkarte II: jährliche Häresieprüfung +10 %); in m19 erhebt sich ein Häretikeraufstand im Rücken |

### E3 – Ser Wendel der Graue (`m03_grey_road`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Hängen | `var_bandits = hanged` | Straßensicherheit +30 % sofort; Unruhe in der Aschmark −10 | Mireille Corvane (die die Banditen heimlich bezahlte) verliert ihre Werkzeuge: `rep_corvane −10`, aber Ruf „Gerechte Wächterin“ (+10 Meinung beim Volk) |
| Als Miliz anwerben | `var_bandits = recruited` | +12 Einheiten (Armbrustschützen-Veteranen), −10 Meinung bei Kaufleuten | „Die Graue Kompanie“ steht in m13 und m19 als loyale Söldnertruppe bereit; 15 % Chance, dass Ser Wendel in Zeitalter V von Mireille zurückgekauft wird (Verrat in m19, falls `var_throne = raventhorn`) |
| An Kaldris ausliefern | `var_bandits = extradited` | `rep_crown +10`, Belohnung 200 Gold | Wendel nennt vor Gericht Mireille als Geldgeberin → `rep_corvane −20`; Aldric vertraut Isolde (in m06 zusätzliche Dialogoptionen) |

### E4 – Der Ruf des Königs (`m04_long_night`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Treueeid in Kaldris leisten | `var_allegiance = crown`, `rep_crown +20` | Königliche Hilfslieferung (500 `food`, 300 `gold`, 10 Ritter) | Zeitalter II–IV: Isolde ist Vasallin der Krone (Abgaben 20 %, Heerfolge), Zugang zum Turnier mit Vorteil; in V natürliche Verteidigerin von Prinz Aldren |
| Unabhängigkeit bewahren | `var_allegiance = independent` | Keine Hilfe; `rep_crown −15`; Prestige +100 | Freie Grenzmark: keine Abgaben, aber keine königliche Hilfe; ab IV Kriegsgrund der Krone gegen die Aschmark möglich; in V eigene Thronkandidatur leichter (+15 Stimmen im Adelsrat der Grenzhäuser) |
| Schutz bei Mireille Corvane suchen | `var_allegiance = corvane`, `rep_corvane +25` | Corvane-Söldner (8 `crossbowman`), Kredit 400 Gold | Isolde wird Lehnsfrau von Nordmark; Mireille erwartet Unterstützung bei ihrem Griff nach dem Thron; `rep_crown −25`; in V ist die Seite praktisch vorgezeichnet, kann aber verraten werden (Verrat: Erinnerung „Gebrochener Vertrag“ −50) |

### E5 – Kassians Geheimnis (`m06_tourney_kaldris`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Den König warnen | `var_kassian_secret = warned_king`, `rep_crown +15`, `rep_corvane −30` | Mireille wird vom Hof verbannt (5 Jahre); Isolde erhält das Amt „Wächterin des Südens“ (+1 Provinz) | Mireille wird erbitterte Feindin; in m08 überfallen gedungene Mörder den Brautzug; in m17 gilt Mireille als Hauptverdächtige |
| Mireille erpressen | `var_kassian_secret = leverage` | Geheimnis „Verschwörung gegen den König“ als Druckmittel gegen Mireille (Haken) | Mireille muss in V eine Ratsabstimmung im Sinne des Spielers stimmen; Kassian weiß, dass Isolde erpressen kann (Meinung Kassian +15 – er respektiert das) |
| Schweigen | `var_kassian_secret = silent`, `rep_corvane +10` | Kein unmittelbarer Effekt | Mireille schuldet Isolde einen Gefallen (einlösbar einmal auf der Weltkarte); das Komplott läuft weiter und trägt in m17 zu Aldrics Tod bei (Indiz) – der Spieler trägt eine stille Mitschuld (Chronik verzeichnet sie, sobald aufgedeckt) |

### E6 – Die Heirat (`m08_bridal_procession`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Torvald Eisenhand (verwitwet) | `var_marriage = skarn`, `rep_skarn +30` | Familienbund mit den Skarn-Clans, 6 `skarn_ironguard` | Erbin Bjarna (V); in III und VII stehen die Clans automatisch an Isoldes Seite; `rep_crown −5` (fremde Heirat) |
| Lucan Corvane | `var_marriage = corvane`, `rep_corvane +30` | Frieden mit Nordmark; Mireille wird Schwägerin | Erbe Corwin mit Thronanspruch über Corvane (V); E10-Option „mit Mireille“ wird wahrscheinlicher (KI-Meinung) |
| Davian Saelith-Varo | `var_marriage = saelith`, `rep_saelith +30` | Mitgift 2.000 Gold, Handelsabkommen | Erbe Teodor; Saelith-Flotte in VI/VII; `rep_flame −10` (die Kirche misstraut den Gezeiten) |
| Keine Heirat; Adoption Wrens | `var_marriage = none`, `rep_flame +10` | Keine Allianz; Volk von Brennhollow +10 Zufriedenheit | Erbin Wren (V); Legitimität −10, aber unabhängig von Fremdhäusern; in VII Bonus „Kind der Asche“ (+15 Moral der Aschmark-Truppen) |

### E7 – Die Wurzelmutter (`m10_root_and_ash`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Die Wurzelknoten heilen und einen Pakt schließen | `var_root_mother = pact`, `rep_thorn +30` | Abholzung am Dornwaldrand muss auf 20 % begrenzt werden (sonst Bruch) | Wachstumsschule zugänglich; m23 friedlich; Wurzelmutter erscheint in m28 als Verbündete (wandelnde Bäume) |
| Die Wurzelmutter bekämpfen | `var_root_mother = slain`, `rep_thorn −50` | Großer Holz- und Aschglasgewinn (1.500 `wood`, 30 `ashglass_raw`) | Dornenbund wird feindlich (Krieg ab Zeitalter IV wahrscheinlich); m23 deutlich schwerer; Ende „Wiederkehr der Stimmen“ nur noch mit Zusatzquest erreichbar |
| Waffenstillstand: Rückzug ohne Pakt | `var_root_mother = truce` | Keine Kosten, keine Belohnung | Prüfung in m23; Dornenbund neutral |

### E8 – Der Riss von Marrowfen (`m11_marrowfen_rift`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Aschglas bergen, bevor der Riss geschlossen wird | `var_rift = harvested`, `var_isolde_hollowing +10` | 60 `ashglass` (Magieforschung beschleunigt um ein Zeitalter) | Isolde erhält „Hohlberührt“; Stimme aus dem Tief kann sie später direkt ansprechen (Option E14 „Aschglasherz“); in m29 zusätzliche Dialogoption „Krone nehmen“ leichter erreichbar |
| Den Riss sofort versiegeln | `var_rift = sealed`, `rep_flame +10` | Kein Aschglas; Schleierstärke der Provinz −1 | Schleierdruck −5; in m24 öffnet sich Morvaines Tor leichter (eine Phase weniger) |

### E9 – Der Graue Rat (`m12_grey_council`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Führung für Raventhorn fordern | `var_council_lead = raventhorn` | Prestige +200; `rep_skarn −10`, `rep_thorn −10` | Im Bund (VII) bestimmt der Spieler die Kriegsziele, aber zwei Fraktionen treten nur bei Meinung ≥ 40 bei |
| Torvald die Führung überlassen | `var_council_lead = torvald` | `rep_skarn +20` | Skarn-Clans tragen die Hauptlast in VII; der Spieler erhält weniger Kontrolle, aber +20 % Bundtruppen |
| Gleichberechtigter Rat | `var_council_lead = shared` | Meinung aller Ratsfraktionen +10 | Bund in VII entsteht automatisch, Entscheidungen per Abstimmung (KI-Fraktionen können Vorschläge ablehnen) |

### E10 – Wem gilt der Krieg? (`m14a`/`m14b`, Weltkarte IV)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Krieg gegen Mireille Corvane | `var_age4_war = against_mireille` | Mission m14a; bei Sieg Gewinn von 2 Provinzen Nordmarks | Corvane bleibt Feind in V; Mireille (oder Lucan) führt die Gegenseite in m19; Bedrohung +10 bei anderen Herzögen |
| Bündnis mit Mireille gegen die Kaperer | `var_age4_war = with_mireille` | Mission m14b; Handelsrouten gesichert | `rep_corvane +20`; in V ist Mireille Verbündete oder zumindest verhandlungsbereit; `rep_saelith −10` |

### E11 – Der Drachenzehnt (`m15_dragon_tithe`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Tribut zahlen | `var_dragon = tribute` | Jährlich 1.000 Gold und 20 `ashglass` an Vyrmathax | Drache verschont eigene Provinzen; in m28 greift er die Aschenlegion an (er will kein Hohles Reich neben sich) |
| Den Drachen töten | `var_dragon = slain` | Schwere Bossschlacht; Beute: Drachenherz (`Relikt`: +30 % Feuerschaden, Glutfelder passierbar) | Glutweite wird erschließbar (Schwefel, Aschglas); in m28 kein Drache, aber das Drachenherz macht den Träger gegen den Glutatem des Hohlen Königs immun |
| Den Drachen auf einen Feind lenken (Schattenkunst oder Köder aus Aschglas) | `var_dragon = redirected` | Vyrmathax verwüstet die Provinzen des gewählten Gegners (Hohle oder Rivale) | Drache betrachtet den Spieler als „nützlich“; in m28 erscheint er unberechenbar (70 % auf Seiten des Spielers); bei Ziel „Rivale“ Bedrohung +15 |

### E12 – Wer tötete den König? (`m17_death_of_a_king`)

Optionen hängen von gefundenen Beweisen ab (mind. 3 von 5 Beweisstücken für „Wahrheit“).

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Kassian Vale beschuldigen | `var_regicide_blame = kassian` | Kassian wird geächtet; Spionagenetz der Krone zerfällt | Kassian taucht in VII als Agent der Stimme aus dem Tief auf (oder, falls unschuldig behandelt und später rehabilitiert, als reuiger Verbündeter) |
| Mireille Corvane beschuldigen | `var_regicide_blame = mireille` | Bürgerkrieg mit Corvane unausweichlich | Starker Stimmenvorteil im Langen Rat; Corvane-Haus wird zerschlagen oder zum Todfeind |
| Die Sonnenhäretiker beschuldigen | `var_regicide_blame = heretics` | Verfolgung der Häresie; Einigkeit des Adels | Häresie wird radikal (m25 feindlicher, `var_sun_heresy = truth` schwerer erreichbar) |
| Die Wahrheit enthüllen | `var_regicide_blame = truth` | Der Mörder war ein Leerpriester im Gewand eines Flammenpriesters – eingeschleust über Kassians Netz, ohne sein Wissen | Kassian wird verpflichteter Verbündeter (Spionagemeister ohne Gehalt); Kirche reformiert sich (`rep_flame +15`); Hohle als gemeinsamer Feind: Bund in VII +10 Meinung |

### E13 – Der Thron von Valemark (`m18_long_council`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Mireille unterstützen | `var_throne = mireille` | Frieden mit Corvane; Isolde wird Großmarschallin | Starkes, aber ehrgeiziges Königtum; Mireille fordert in VI Gehorsam; Ende „Thron aus Glut“ ausgeschlossen |
| Prinz Aldren unter Regentschaft Raventhorns | `var_throne = regency` | Isolde (bzw. der Erbe) ist Regent bis 940; Usurpationsversuche anderer Häuser | Aldren wird König (ab 940) mit Meinung +60 zu Raventhorn; Bund in VII erhält die gesamte Glutkrone |
| Den Thron für Raventhorn beanspruchen | `var_throne = raventhorn` | Krieg gegen alle anderen Thronkandidaten (m19 gegen Koalition) | Raventhorn wird Königshaus Valemarks; Legitimität niedrig (Usurpation); Bedrohung +20; Ende „Thron aus Glut“ möglich |

### E14 – Isoldes letzte Wahl (`m20_ember_heir`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Den letzten Kampf führen | `var_isolde_fate = fallen` | Isolde fällt im Kampf (gesichert, unabhängig von Permadeath); Moral +20 für alle Aschmark-Truppen für 10 Jahre | Legende „Die Unbeugsame“ (Chronik); ihr Schwert wird zum Artefakt des Erben (+15 % Schaden gegen Hohle); Isolde erscheint in m24 unter den Toten |
| Abdanken und Wächterin bleiben | `var_isolde_fate = abdicated` | Erbe herrscht; Isolde bleibt als Beraterin (Kanzlerin, Gelehrsamkeit +3 durch Erfahrung) | Isolde stirbt um 940 eines natürlichen Todes; in VII ist sie nicht mehr dabei, aber ihre Briefe (Dialoge) begleiten den Erben |
| Das Aschglasherz annehmen (nur bei `var_isolde_hollowing ≥ 10`) | `var_isolde_fate = ashglass_heart`, `var_isolde_hollowing +40` | Isolde verjüngt sich auf die Werte einer 40-Jährigen, kämpft weiter als Heldin | Isolde wird in VII zur Prüfung: Sie kann vom Erben gerettet (m24, Ritual) oder vom Hohlen König gegen ihn gewendet werden (Bossbegleiterin in m29) |

### E15 – Die Wahrheit über den Sonnenkönig (`m25_ithren_light`)

| Option | Variable | Kurzfristig | Langfristig |
|---|---|---|---|
| Den Sonnenhäretikern die Wahrheit sagen | `var_sun_heresy = truth` | Häresie spaltet sich; ein Teil wendet sich ab, ein Teil verbündet sich | In m28 öffnen Häretiker ein Tor Solmeres; Voraussetzung für Ende „Erlösung des Sonnenkönigs“ |
| Ihren Glauben unangetastet lassen | `var_sun_heresy = faith` | Häresie bleibt stabil und neutral | In m29 kämpfen Häretiker auf keiner Seite; kein Torbonus |
| Die Häretiker als Werkzeug benutzen | `var_sun_heresy = used` | Häretiker greifen Hohle in Solmere an (Ablenkung, viele Tote) | Häresie zerbricht verbittert; `rep_flame +10`; in m29 entfällt die Option „Erlösen“ |

### E16 – Der Hohle Thron (`m29_hollow_throne`)

| Option | Voraussetzung | Variable | Ergebnis |
|---|---|---|---|
| Den Hohlen König vernichten | immer | `var_final_choice = destroy` | Ende „Die Letzte Flamme“ oder „Der Thron aus Glut“ |
| Den Schleier versiegeln | `var_shrines_restored = 5` und Bund mit mindestens 4 Fraktionen | `var_final_choice = seal` | Ende „Die Wiederkehr der Stimmen“ |
| Varkhan erlösen | `flag_nel_sheltered`, `var_sun_heresy = truth`, m24 vollständig abgeschlossen | `var_final_choice = redeem` | Ende „Erlösung des Sonnenkönigs“ |
| Die Krone nehmen | Persönliche Hohlung des Herrschers ≥ 45 oder `var_isolde_fate = ashglass_heart` | `var_final_choice = crown` | Ende „Die Hohle Krone“ |

---

## 5. Enden

| # | Ende | Bedingungen | Ausgang |
|---|---|---|---|
| 1 | **Die Letzte Flamme** | `var_final_choice = destroy`, `var_throne ≠ raventhorn` oder Bund aktiv | Varkhan Aurelith wird vernichtet, die Aschenlegion zerfällt zu Asche. Der Schleier bleibt dünn, aber das Tief hat seine Tür verloren. Die Reiche Eldraeths, vereint im Bund, beginnen den langen Wiederaufbau. Die Chronik endet mit dem Ausblick auf ein Zeitalter, das „nicht golden, aber warm“ sein wird |
| 2 | **Die Wiederkehr der Stimmen** | `var_final_choice = seal` | Mit den fünf Schreinen und dem Erzritual schließt sich der Schleier. Zum ersten Mal seit fast tausend Jahren fällt in der Aschmark kein Ascheregen. In der letzten Einstellung hört der Erbe eine Stimme, die nicht aus dem Tief kommt. Bestes Ende; schaltet den Sandbox-Startzustand „Zeitalter der Stimmen“ frei |
| 3 | **Erlösung des Sonnenkönigs** | `var_final_choice = redeem` | Der Erbe führt Varkhan durch Morvaines Tor. Der Letzte Sonnenkönig geht hinüber, und mit ihm die Toten des Aschenfalls. Nel bleibt als Hüterin des Tores zurück. Das Hohle Tief verliert seine Gestalt in der Welt, bleibt aber als Macht bestehen – Hoffnung mit offenem Ausgang |
| 4 | **Die Hohle Krone** | `var_final_choice = crown` | Der Herrscher setzt die Krone Aureths auf. Die Hohlen knien. Eldraeth hat einen neuen, ewigen König – oder eine ewige Königin, wenn Isolde das Aschglasherz trug. Die Chronik bricht ab; die letzte Seite schreibt die Stimme aus dem Tief selbst. Dunkles Ende; schaltet den Sandbox-Start „Hohles Reich“ frei |
| 5 | **Der Thron aus Glut** | `var_final_choice = destroy` und `var_throne = raventhorn` und Herrschaft über ≥ 50 % der Provinzen Valemarks | Der Hohle König fällt, und Raventhorn herrscht als Hochkönigshaus über ein Eldraeth, das es mit Feuer geeint hat. Ein Sieg – und ein Echo von Aureth: Die Schlussszene zeigt den Erben auf einem Thron, über dem sich eine Sonnenscheibe aus Glut formt |
| 6 | **Asche über Eldraeth** | Niederlage in m29 (Gesamtverlust aller Helden) oder globaler Schleierdruck erreicht 100 vor Abschluss von m29 | Der Schleier bricht. Die Überlebenden fliehen auf die Saelith-Inseln. Die Chronik endet mit der Liste derer, die standhielten – mit den echten Namen aus der Partie. Die Kampagne kann ab m26 erneut versucht werden (Spielstand bleibt erhalten) |

Jedes Ende wird durch einen **Epilog** ergänzt, der für jede Fraktion, jede überlebende Kernfigur und Brennhollow einen
Absatz aus den Variablen erzeugt (z. B. `var_root_mother = pact` → „Im Dornwald wuchsen die Bäume über die alten
Grenzsteine hinweg, und niemand fällte sie.“).

---

## 6. Mission 1 im Detail: „Die letzte Siedlung“ (`m01_last_settlement`)

### 6.1 Steckbrief

| Feld | Wert |
|---|---|
| ID | `m01_last_settlement` |
| Zeitalter | I – *The Last Settlement* |
| Datum | Herbst 897 n.F. (Monat der Fahlen Ernte), drei Tage und Nächte |
| Ort | Brennhollow und Umland, Aschmark (`ashmarch`) |
| Spielerfraktion | Glutkrone von Valemark (`ember_crown`), Haus Raventhorn |
| Gegner | Die Hohlen (`hollowed`), Basis: Hohlschrein von Vael Dorn (`hollow_shrine`) in den Ruinen nordöstlich |
| Heldin | Isolde Raventhorn, Wächterin der Aschmark (`hero_isolde`), Stufe 3 |
| Nebenfigur | Bruder Hal Venn (`char_hal_venn`), Flammenpriester und Mentor (nicht steuerbar) |
| Boss | Der Fahle Vogt (`boss_pale_reeve`) |
| Kartengröße | 160 × 160 Felder |
| Dauer (Ziel) | 35–50 Minuten auf „Lehnsherr“ |
| Lernziele | Kamera, Auswahl, Bewegen, Bauen, Sammeln, Ausbilden, Tag/Nacht, Bindung, Heldenfähigkeiten, Fokusangriff auf Boss, erste Entscheidung |
| Musik | `build_calm` (Tag), `build_tension` (Dämmerung, Vorbereitung), `battle_small` (Nacht 1 und 2), `battle_large` (Nacht 3, Angriff auf Vael Dorn), `boss_pale_reeve` (Bosskampf), `victory_sting` / `defeat_sting` |
| Ambience | `ambience_village` + `ambience_day` (Brennhollow tagsüber), `ambience_night`, `ambience_ash_wind` (Aschfelder, Ascheregen), `ambience_ruins` (Vael Dorn) |

### 6.2 Erzählerischer Rahmen

Brennhollow ist alles, was von der Aschmark übrig ist: ein Bergfried aus der Zeit der Grenzlords, ein Gehöft, ein
paar Katen hinter einer halb verfallenen Palisade, dreißig Menschen. Seit Wochen kommen die Hohlen jede Nacht ein
Stück näher. Bruder Hal Venn hat ausgerechnet, dass die Vorräte bis zum Neumond reichen. Isolde Raventhorn hat
entschieden, dass niemand flieht.

Im Nordosten liegen die Ruinen des aurethischen Klosters Vael Dorn, in die Vogt Aldous Meringer 894 n.F. die Flüchtlinge aus dem gefallenen Grauwall führte (Prolog B).
Dort haben die Hohlen einen Schrein errichtet. Sein Hüter ist der Fahle Vogt – einst Aldous Meringer, Vogt der Feste
und Freund ihres Vaters. Solange der Schrein steht, sind die Hohlen in der Gegend gebunden und kehren jede Nacht
zurück.

### 6.3 Karte

```
        0         20        40        60        80        100       120       140      160
    0  ┌──────────────────────────────────────────────────────────────────────────────────┐
       │ WWWWWWWWWWWWWWWWWW              ║                    RRRRRRRRRRRRRRRRRRRR         │
       │ WWWWWWWWWWWWWWW    Nordwald     ║  Graue Straße       RR   Ruinen von    RR       │
   20  │ WWWWWWWWWWWW                    ║            (K2)     RR   Vael Dorn [S] RR       │
       │ WWWWWWWWWW      ~~              ║                     RR      (Krypta)  RR       │
       │ WWWWWWWW       ~~               ║                     RRRRRRRRRRRRRRRRRRRR       │
   40  │ WWWWWWW       ~~    Brennbach   ║                          ▲▲▲▲▲                 │
       │ WWWWWW       ~~                 ║                  (K1)  ▲▲▲▲▲▲▲▲   Steinkamm   │
       │ WWWWW  (M)  ~~       ┌──────────╨────────┐             ▲▲▲ Gold ▲▲▲            │
   60  │ WWWW       ~~        │   Palisade        │             ▲▲▲▲▲▲▲▲▲▲   ◄── Ost    │
       │ WWW       ~~         │  Katen   [B]      │                ▲▲▲▲▲                 │
       │ WWW      ~~          │       Bergfried   │   Felder                              │
   80  │ WW      ~~           │  [G]  Gehöft      │   (Baugrund)                          │
       │ WW     ~~            └───────────────────┘                                       │
       │ W     ~~          Furt                                                          │
  100  │ W    ~~                                        ░░ Hohlungsfläche ░░             │
       │     ~~       † Alter Friedhof †                                                 │
       │    ~~                                                                           │
  120  │   ~~                                                                            │
       │  ~~     ░░░░░░░░░░░░░░░░░░░░░░░ Aschfelder ░░░░░░░░░░░░░░░░░░░░░░░░░░░          │
  140  │ ~~      ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░          │
       │~~               ▲ Süd (Spawn)              ▲ Süd (Spawn)                        │
  160  └──────────────────────────────────────────────────────────────────────────────────┘
```

| Zeichen | Bedeutung | Koordinaten (Mitte) |
|---|---|---|
| [B] | Bergfried (`keep`), Startgebäude | (58, 64) |
| [G] | Gehöft (`farmstead`), Startgebäude | (50, 80) |
| W | Wald (Holz, 120 Bäume à 100 `wood`) | West- und Nordrand |
| ▲ | Felsen des Steinkamms (Stein 6 Vorkommen à 800 `stone`), darin die alte Goldader (1.500 `gold`) | (110–130, 40–70) |
| ~~ | Brennbach, Furt bei (40, 92) | – |
| (M) | Holzfällerlager der Familie Marrow (Nebenziel) | (22, 52) |
| † | Alter Friedhof | (44, 104) |
| ░ | Aschfelder (Hohlland, Spawnzone Süd), Hohlungsfläche (schwache Bindungsquelle, Radius 6) und eine kleine Hohlungsfläche im Westwald, die an Tag 2 aufbricht | (20–140, 128–150); (100, 100); (20, 56) |
| (K1), (K2) | Knochentürme (`bone_spire`) an Ost- und Nordzugang zu Vael Dorn | (104, 46); (86, 20) |
| [S] | Hohlschrein von Vael Dorn (`hollow_shrine`) | (128, 20) |
| Krypta | Verschütteter Gang hinter dem Schrein (verstecktes Ziel) | (140, 26) |
| ║ | Graue Straße, im Norden verschüttet | x = 70 |

Baugrund: innerhalb der Palisade Platz für Bergfried, 2 Gebäude und Katen; östlich der Palisade (66–96, 60–90) Felder
für Gehöfte und Kaserne. Wachtürme sind überall auf freiem Grund baubar.

### 6.4 Startzustand

| Kategorie | Wert |
|---|---|
| Einwohner | **30**: 6 Arbeiter (`worker`, steuerbar), 5 Milizionäre (steuerbar), 19 Bewohner (nicht steuerbar; besetzen das Start-Gehöft mit 4 Personen, zahlen Abgaben, laufen als Passanten durch das Dorf) |
| Miliz | 3 `spearman`, 2 `archer`, Status „Bürgermiliz“ (Rekrut, −10 Grundmoral, nach der ersten überlebten Nacht entfällt der Malus) |
| Heldin | `hero_isolde`, Stufe 3 (383 XP), 620 HP, Glut 110, alle drei Fähigkeiten auf Stufe 1 |
| Gebäude | `keep` (Stufe I, 2.000 HP), `farmstead` (voll besetzt), Palisade mit zwei Lücken (Nord, Süd), 4 Katen (Kulisse, brennbar) |
| Ressourcen | `gold` 200, `wood` 150, `stone` 50, `food` 100 |
| Bevölkerung | `population` 11 (6 Arbeiter + 5 Miliz; Isolde zählt 0), `population_cap` 15 (Bergfried 10 + Gehöft 5) |
| Einkommen | Bergfried: Abgaben der Bewohner +1 `gold` je 3 s (+10 % je weiteres besetztes Gehöft); Start-Gehöft: 1,6 `food`/s (4 Bewohner × 0,4) |
| Sichtbarkeit | Brennhollow und Umgebung (Radius 20) aufgedeckt, Rest Kriegsnebel; Vael Dorn ist als Silhouette am Horizont markiert |
| Tageszeit | Tag 1, 06:00 Uhr Spielzeit (Morgen) |

### 6.5 Mechanische Sonderregeln der Mission

| Regel | Beschreibung |
|---|---|
| Tageszyklus | Missionsspezifisch: Tag 1 dauert 8 min (erste Bedrohung nach 9 min, siehe Balancing-Grundsatz 5 im GDD) |
| Weitere Gehöfte | Neue Gehöfte werden mit `worker`-Einheiten besetzt (Rechtsklick auf das Gehöft, max. 4); jeder Arbeiter liefert 0,4 `food`/s |
| Sammeln | Holz: 10 je Gang; Stein: 8 je Gang; Gold: 5 je Gang (Goldader); Abgabe am Bergfried |
| Ritterschwur | Bergfried Stufe II (150 `gold`, 100 `wood`, 200 `stone`, 90 s) schaltet in der Kaserne die Forschung „Ritterschwur“ frei (100 `gold`, 50 `food`, 45 s), danach `knight` ausbildbar |
| Hal Venn | Steht am Bergfried; heilt alle eigenen Einheiten im Radius 6 um 5 HP/s außerhalb des Kampfes; kann nicht sterben, kann während E1 (Option Begraben) 90 s nicht heilen |
| Isolde | Wird sie niedergestreckt, erwacht sie nach 60 s am Bergfried (Wiederbelebung kostenlos). Ausnahme: Permadeath „Aschenfall“ – dann übernimmt Knappe Tamsin Hale (Kampagnenregel 1.6) |
| Bindung | Hohle außerhalb einer Bindungsquelle verlieren 1 % HP/s; bei Nacht verdoppeln sich die Radien: Schrein 20→40, Knochentürme 12→24, Hohlungsfläche 6→12, Fahler Vogt 15→30. Tagsüber sind die Hohlen deshalb nur in Vael Dorn und an der Hohlungsfläche gefährlich |
| Licht | Bergfried Lichtradius 10, Wachturm 6; Feuerstellen (10 `wood`, Lichtradius 4) sind ab der ersten Dämmerung baubar |
| Zerstörung des Schreins | Beendet die Mission sofort mit Sieg, auch vor Nacht 3 (belohnt Aggression); alle Hohlen im Bindungsbereich zerfallen |

### 6.6 Ziele

**Primärziele** (alle erforderlich, erscheinen nacheinander)

| ID | Ziel | Auslöser | Abschluss | Dialog/Audio |
|---|---|---|---|---|
| `obj_m01_farm` | „Baut ein weiteres Gehöft, damit Brennhollow den Winter übersteht.“ | Missionsstart (nach Intro) | Zweites `farmstead` fertiggestellt | `char_hal_venn.m01_first_farm` |
| `obj_m01_barracks` | „Errichtet eine Kaserne.“ | Missionsstart (parallel) | `barracks` fertiggestellt | `char_hal_venn.m01_barracks` |
| `obj_m01_army` | „Bildet eine Armee aus: mindestens 4 Nahkämpfer und 3 Bogenschützen zusätzlich zur Miliz.“ | Kaserne fertig | 4 × (`swordsman` oder `spearman`) und 3 × `archer` ausgebildet (kumuliert) | Isolde: `voice.hero_isolde.greet` |
| `obj_m01_nights` | „Übersteht die Nächte. Der Bergfried darf nicht fallen.“ | Erste Dämmerung (08:00) | Morgengrauen nach Nacht 2 erreicht (Nacht 3 entfällt, wenn der Schrein vorher fällt) | `char_hal_venn.m01_wave_warning`, `narrator.m01_night_falls` |
| `obj_m01_shrine` | „Zerstört den Hohlschrein von Vael Dorn. Der Fahle Vogt bewacht ihn.“ | Sichtkontakt zu Vael Dorn oder spätestens 18:00 (Tag 2) | `boss_pale_reeve` besiegt und `hollow_shrine` zerstört | `char_hal_venn.m01_shrine_found`, `voice.boss_pale_reeve.taunt` |

**Nebenziele** (optional, mit Belohnung)

| ID | Ziel | Belohnung |
|---|---|---|
| `obj_m01_towers` | Errichtet zwei Wachtürme | +50 `stone`; Wachtürme erhalten +1 Reichweite für den Rest der Mission („Grenzerwissen“) |
| `obj_m01_marrow` | Rettet die Familie Marrow (Tag 2, 15:00: Hohle aus der frisch aufgebrochenen Hohlungsfläche im Westwald greifen das Holzfällerlager an; Hohlungsfläche mit einer Feuerstelle oder *Glutspalter* auf das Zentrum versiegeln) | +3 `worker` (Bevölkerung +3), +100 `wood`; Marrows erscheinen in m02 als Bewohner mit Namen |
| `obj_m01_spires` | Zerstört beide Knochentürme, bevor Ihr den Schrein angreift | Jeder Turm: Nachtwellen −15 %; beide: Fahler Schleier am Schrein entfällt schon vor dem Tod des Vogts zur Hälfte (−37,5 % statt −75 %) |
| `obj_m01_oath` | Erforscht den Ritterschwur und bildet einen Ritter aus | +150 XP für Isolde, Ritter starten als „Erfahren“ |

**Versteckte Ziele**

| ID | Ziel | Belohnung |
|---|---|---|
| `obj_m01_blade` | Findet die Krypta hinter dem Schrein (nach dem Sieg begehbar, oder vorher mit Isolde allein bei Tag) | Gegenstand „Raventhorns Glutklinge“ (Waffe: +6 Schaden, 10 % Brandchance); Chronik-Eintrag über Markgraf Edran |
| `obj_m01_no_losses` | Kein Gebäude geht verloren | Erfolg „Kein Stein fällt“; +10 Zufriedenheit in Brennhollow zu Beginn von Zeitalter II |
| `obj_m01_dawn_strike` | Zerstört den Schrein, bevor Nacht 2 beginnt | Erfolg „Vor dem zweiten Mond“; Isolde +200 XP |

### 6.7 Ablauf und Zeitleiste (Lehnsherr)

| Zeit | Phase | Ereignis | Audio |
|---|---|---|---|
| 00:00 | Tag 1 | Intro: Kamerafahrt über die Aschfelder zur Palisade; Erzähler | `narrator.m01_opening`, Musik `build_calm` |
| 00:25 | Tag 1 | Hal Venn erklärt die Lage; Ziele `obj_m01_farm` und `obj_m01_barracks` erscheinen | `char_hal_venn.m01_intro`, `sfx.objective_new` |
| ca. 02:30 | Tag 1 | Zweites Gehöft fertig (Richtwert) | `char_hal_venn.m01_first_farm` |
| ca. 04:30 | Tag 1 | Kaserne fertig; `obj_m01_army` erscheint | `char_hal_venn.m01_barracks` |
| 07:30 | Tag 1 | Warnung: Wind dreht, Asche aus Süden; Feuerstellen werden baubar | `char_hal_venn.m01_wave_warning`, Musik `build_tension` |
| 08:00 | Dämmerung | Ziel `obj_m01_nights` erscheint; Sicht −20 % | `sfx.horn_alarm` |
| 08:45 | Dämmerung | Erzähler kündigt die Nacht an | `narrator.m01_night_falls` |
| 09:00 | Nacht 1 | Nachtphase beginnt | Musik `battle_small`, `ambience_night` |
| 09:30 | Nacht 1 | Welle 1a (Süd) | – |
| 10:45 | Nacht 1 | Welle 1b (Südost, Friedhofsweg) | – |
| 12:00 | Morgengrauen | „Erster Strahl“: Hohle −10 % Schaden/Tempo für 60 s; Miliz-Malus entfällt | Musik `build_calm` |
| 12:30 | Tag 2 | **Entscheidung E1**: Die Toten von Brennhollow (siehe 4) | – |
| 13:00 | Tag 2 | Tag; Erkundung und Ausbau | – |
| 15:00 | Tag 2 | Nebenziel Familie Marrow (die kleine Hohlungsfläche im Westwald bricht bei (20, 56) auf: 3 `hollow_thrall`, Bindung tagsüber Radius 6 – das Lager liegt knapp darin) | `sfx.ui_notify` |
| bis 18:00 | Tag 2 | Sichtkontakt zu Vael Dorn oder Zeitauslöser → `obj_m01_shrine` | `char_hal_venn.m01_shrine_found` |
| 19:00 | Dämmerung | Warnsignal | `sfx.horn_alarm`, Musik `build_tension` |
| 20:00 | Nacht 2 | Wellen 2a–2d (siehe 6.8) | Musik `battle_small` |
| 23:00 | Morgengrauen | – | – |
| 24:00 | Tag 3 | Zwei Ritter („die letzten Reiter von Grauwall“, `knight`, Erfahren) erreichen Brennhollow über die Graue Straße | Isolde: `voice.hero_isolde.greet` |
| 30:00 | Dämmerung | Letzte Warnung: „Die Lange Nacht“ | `sfx.horn_alarm` |
| 31:00 | Nacht 3 | Lange Nacht (4 min), Wellen 3a–3c; der Fahle Vogt führt Welle 3c persönlich, sofern der Schrein noch steht | Musik `battle_large` |
| 35:00 | Morgengrauen | Ab jetzt: Tageszyklen wiederholen sich mit +25 % Wellenstärke je weiterer Nacht, bis der Schrein fällt | – |
| variabel | Angriff | Angriff auf Vael Dorn → Bosskampf | Musik `boss_pale_reeve`, `ambience_ruins` |
| Sieg | – | Schrein zerstört | `char_hal_venn.m01_victory`, `narrator.m01_victory`, `victory_sting` |

### 6.8 Gegnerwellen (Lehnsherr)

| Welle | Zeit | Herkunft | Zusammensetzung | Verhalten |
|---|---|---|---|---|
| 1a | 09:30 | Süd (Aschfelder) | 6 `hollow_thrall`, 2 `hollow_archer` | Direkter Angriff auf die Südlücke der Palisade, dann auf das Gehöft |
| 1b | 10:45 | Südost (Friedhofsweg) | 4 `hollow_thrall` | Zielt auf Arbeiter außerhalb der Palisade |
| 2a | 20:15 | Süd | 8 `hollow_thrall`, 3 `hollow_archer` | Angriff auf die Südlücke |
| 2b | 21:00 | Ost (Steinkamm) | 4 `hollow_thrall`, 1 `grave_brute` | Grabkoloss greift bevorzugt Gebäude an (doppelter Gebäudeschaden) |
| 2c | 21:30 | Friedhof (innerhalb der Reichweite der Palisade) | 4 `hollow_thrall` – **nur wenn `flag_m01_dead_burned = false`** | Erheben sich aus den Gräbern, greifen von innen an |
| 2d | 22:15 | Süd | 4 `hollow_thrall`, 2 `hollow_archer` | Nachzügler, die geschwächte Verteidiger ausnutzen |
| 3a | 31:15 | Süd | 10 `hollow_thrall`, 4 `hollow_archer`, 1 `grave_brute` | Breiter Angriff über Süd- und Nordlücke |
| 3b | 32:00 | Ost | 6 `hollow_thrall`, 1 `grave_brute` | Angriff auf Felder und Kaserne |
| 3c | 33:00 | Nordost (von Vael Dorn) | `boss_pale_reeve` + 6 `hollow_thrall` – nur wenn der Schrein steht | Der Vogt kämpft bis 60 % HP, zieht sich dann per *Fahler Ernte* zum Schrein zurück und heilt dort über 120 s vollständig, wenn er nicht verfolgt wird |

Jeder zerstörte Knochenturm reduziert alle späteren Wellen um 15 % (abgerundet, mindestens 1 Einheit je Typ).

**Verteidigung des Schreins** (statisch): 8 `hollow_thrall`, 4 `hollow_archer`, 1 `grave_brute`, 2 `bone_spire`,
`boss_pale_reeve`. Der Schrein erzeugt alle 45 s (nachts 30 s) 2 `hollow_thrall`, bis 12 Knechte im Umkreis stehen.

### 6.9 Bosskampf: Der Fahle Vogt (`boss_pale_reeve`)

**Werte (Lehnsherr)**

| Wert | Betrag |
|---|---|
| HP | 3.200 |
| Rüstung | Untot 4 |
| Angriff | Sensenschwung: Leere 40, Kegel 90°, Reichweite 1,5, Intervall 2,0 s (Phase 3: 1,4 s) |
| Tempo | 2,4 |
| Sicht | 12 |
| Bindungsradius | 15 (nachts 30) |
| Immunitäten | Betäubungen über 1 s, Moraleffekte, sofortige Tötung |
| Schwäche | Feuer (140 % durch Untot-Rüstung), Wucht (150 %); Rückstoß durch *Glutspalter* unterbricht kanalisierte Fähigkeiten |
| Nachtbonus | +15 % Schaden |

**Fähigkeiten**

| Fähigkeit | Audio | Abklingzeit | Wirkung | Konter |
|---|---|---|---|---|
| Seelenentzug | `sfx.reeve_soul_drain`, `voice.boss_pale_reeve.ability` | 18 s | Kanalisiert 4 s auf ein Ziel in 6 Feldern (bevorzugt Isolde): 15 Leere/s, heilt den Vogt um 200 % des verursachten Schadens | Ziel aus 8 Feldern bewegen, *Aschenschild* auf das Ziel (blockiert vollständig), *Glutspalter*-Rückstoß unterbricht |
| Hohlruf | `sfx.hollow_summon` | 25 s (nachts 18 s) | 3 `hollow_thrall` erheben sich aus den Gräbern um den Schrein (max. 9 beschworene gleichzeitig) | Fernkämpfer auf Beschworene, Flächenschaden (*Glutspalter*) |
| Fahle Ernte | `voice.boss_pale_reeve.taunt` | Phasenwechsel bei 60 % | Vogt wird 6 s ätherisch und unangreifbar, gleitet zum Schreinzentrum; 4 **Seelenlaternen** (300 HP, Befestigt 2) erscheinen um den Schrein. Solange mindestens eine steht: Vogt −50 % erlittener Schaden und +10 HP/s je Laterne | Truppen aufteilen: Laternen zerstören (Ritter und Schwertkämpfer), Vogt mit Speeren binden |
| Totenglocke | `voice.boss_pale_reeve.ability` | 15 s (ab Phase 3) | Angekündigt durch 1,5 s Glockenklang und Bodenring (Radius 4): 1,5 s Betäubung, −15 Moral | Aus dem Ring treten; *Aschenschild* verhindert die Betäubung; *Sammelbanner* hebt den Moralverlust auf |
| Raserei | – | Phase 3 (unter 30 %) | Angriffsintervall 1,4 s, Schreckensaura −15 Moral (Radius 5) | *Sammelbanner*, Fokusangriff |

**Phasen**

| Phase | HP | Ablauf |
|---|---|---|
| 1 – Hüter | 100–60 % | Sensenschwung, Seelenentzug, Hohlruf. Der Fahle Schleier schützt den Schrein (−75 % Schaden) |
| 2 – Fahle Ernte | 60–30 % | Laternenmechanik; der Vogt kämpft vom Schreinzentrum aus und zieht sich nach je 20 s Kampf kurz zurück |
| 3 – Totenglocke | 30–0 % | Totenglocke, Raserei; der Vogt verfolgt Isolde bevorzugt |

**Tod des Vogts**: Aldous Meringer erkennt Isolde im Sterben (Untertitelzeile: „Edrans Tochter … er hat sich verbrannt,
Mädchen. Er ist nicht hier. Er war nie hier.“). Der Fahle Schleier fällt; der Schrein (3.000 HP, Befestigt 7) kann nun
voll beschädigt werden. Wird der Schrein zerstört, zerfallen alle Hohlen im Bindungsbereich zu Asche.

### 6.10 Dialoge und Texte

| Audio-Event | Sprecher | Text (Deutsch) |
|---|---|---|
| `narrator.m01_opening` | Erzähler | „Neunhundert Jahre nach dem Fall Aureths brannte in der Aschmark nur noch ein Licht. Man nannte es Brennhollow. Dreißig Seelen hüteten es – und eine Frau, die nicht wusste, wie man aufgibt.“ |
| `char_hal_venn.m01_intro` | Hal Venn | „Die Speicher reichen bis zum Neumond, Wächterin. Danach essen wir Asche. Wenn wir den Winter sehen wollen, brauchen wir Felder – und Klingen, die sie schützen.“ |
| `voice.hero_isolde.greet` | Isolde | „Dann pflügen wir am Tag und wachen in der Nacht. Wie immer, Bruder.“ |
| `char_hal_venn.m01_first_farm` | Hal Venn | „Ein zweites Gehöft. Euer Vater pflegte zu sagen: Brot ist die erste Mauer. Die zweite müsst Ihr aus Eisen bauen.“ |
| `char_hal_venn.m01_barracks` | Hal Venn | „Die Kaserne steht. Es sind Bauern, Wächterin, keine Krieger. Aber Bauern, die wissen, wofür sie kämpfen, halten länger als jeder Söldner.“ |
| `char_hal_venn.m01_wave_warning` | Hal Venn | „Spürt Ihr, wie der Wind dreht? Die Asche kommt von Süden. Sie kommen heute Nacht. Holt alle hinter die Palisade und zündet die Feuer an!“ |
| `narrator.m01_night_falls` | Erzähler | „Und die Nacht kam über die Aschmark, und mit ihr jene, die nicht mehr schlafen können.“ |
| `char_hal_venn.m01_shrine_found` | Hal Venn | „Vael Dorn … das alte Kloster, in das Meringer die Flüchtlinge von Grauwall führte. Sie haben dort einen Schrein errichtet, aus den Knochen derer, die sie verteidigten. Solange er steht, werden die Nächte nicht enden.“ |
| `voice.boss_pale_reeve.taunt` (Variante 01) | Fahler Vogt | „Kleine Raventhorn. Ich habe die Tore deines Vaters gehütet. Nun hüte ich seine Asche.“ |
| `voice.boss_pale_reeve.taunt` (Variante 02) | Fahler Vogt | „Das Tief vergisst niemanden. Es wartet nur.“ |
| `voice.boss_pale_reeve.taunt` (Variante 03) | Fahler Vogt | „Bleib. Es tut nur einen Atemzug lang weh.“ |
| `char_hal_venn.m01_victory` | Hal Venn | „Es ist vorbei. Für heute. Aldous Meringer war ein guter Mann, bevor das Tief ihn nahm. Möge die Flamme ihm den Weg zeigen, den er selbst nicht mehr fand.“ |
| `narrator.m01_victory` | Erzähler | „In jener Nacht erlosch das fahle Licht über Vael Dorn. Brennhollow brannte weiter. Und in der Chronik der Aschmark stand zum ersten Mal seit vier Jahren ein Sieg.“ |
| `narrator.m01_defeat` | Erzähler | „So fiel Brennhollow, das letzte Licht der Aschmark. Die Asche deckte es zu, wie sie alles zudeckt. Doch die Chronik ist noch nicht zu Ende geschrieben.“ |

**Entscheidung E1 im Spiel** (Ereignisfenster, Untertitel ohne Vertonung in der Vertical Slice):

> *Im Morgengrauen liegen die Gefallenen der Nacht im Hof des Bergfrieds. Bruder Hal Venn steht mit einer Fackel daneben.*
> **Hal Venn**: „Verbrennt sie, Wächterin. Was die Flamme nimmt, kann das Tief nicht mehr holen.“
> **Witwe Marga**: „Mein Mann hat diesen Friedhof selbst umgegraben. Er gehört in diese Erde, nicht in den Rauch.“
>
> - **„Verbrennt sie. Ich will keinen von ihnen heute Nacht wiedersehen.“** → 40 Holz, `flag_m01_dead_burned = true`
> - **„Begrabt sie. Bruder, segnet die Gräber, so gut Ihr könnt.“** → Hal Venn 90 s gebunden, `flag_m01_dead_burned = false`

### 6.11 Siegs- und Niederlagebedingungen

| Bedingung | Ergebnis |
|---|---|
| `boss_pale_reeve` besiegt **und** `hollow_shrine` zerstört | **Sieg** |
| `keep` zerstört | **Niederlage** |
| Alle `worker` und alle Militäreinheiten tot (Isolde allein reicht nicht) | **Niederlage** |
| Isolde endgültig gefallen (nur Permadeath „Aschenfall“ und Tamsin Hale ebenfalls gefallen) | **Niederlage** |

Bei Niederlage: Neustart der Mission oder Neustart ab dem letzten Morgengrauen (automatischer Speicherpunkt bei jedem
Morgengrauen; im Ironman nur Neustart der Mission).

### 6.12 Schwierigkeitsskalierung

| Parameter | Pilger | Vasall | Lehnsherr | Hochkönig | Aschenfall |
|---|---|---|---|---|---|
| Startressourcen | ×1,5 | ×1,2 | ×1,0 | ×1,0 | ×0,8 |
| Wellenstärke | ×0,6 | ×0,8 | ×1,0 | ×1,25 | ×1,5 + Zusatzwelle 1c/2e/3d (je 4 `hollow_thrall`, 2 `hollow_archer`) |
| HP Fahler Vogt | 2.000 | 2.600 | 3.200 | 4.000 | 4.800 |
| Abklingzeit Hohlruf | 35 s | 30 s | 25 s | 22 s | 18 s |
| Länge Tag 1 | 10 min | 9 min | 8 min | 7 min | 6 min |
| Schreingarnison | 5/2/0 | 6/3/1 | 8/4/1 | 10/5/1 | 12/6/2 (Knechte/Schützen/Kolosse) |
| Hinweise | alle, mit Pfeilen | wichtige | nur Ziele | nur Ziele | keine |
| Isolde-Wiederbelebung | 30 s | 45 s | 60 s | 90 s | Permadeath-Einstellung (mindestens „Hart“) |

### 6.13 Tutorialführung (Pilger und Vasall; abschaltbar)

| Auslöser | Hinweis |
|---|---|
| Missionsstart | „Ziehe einen Rahmen um Einheiten, um sie auszuwählen. Rechtsklick bewegt sie.“ |
| Arbeiter ausgewählt | „Drücke `B`, um zu bauen. Gehöfte brauchen Platz für Felder.“ |
| Holz < 60 | „Schicke Arbeiter in den Westwald (Rechtsklick auf Bäume).“ |
| Erste Dämmerung | „Hohle sind an Schreine und Türme gebunden. Bei Nacht reicht ihr Einfluss doppelt so weit. Licht stärkt die Moral deiner Truppen.“ |
| Isolde im Kampf | „`Q` – Glutspalter trifft alle Gegner vor Isolde. Feuer ist gegen Hohle besonders wirksam.“ |
| Erste Moralflucht | „Fliehende Einheiten sammeln sich am Banner: `W` – Sammelbanner.“ |
| Seelenentzug des Vogts | „Der Vogt saugt Leben ab! Schütze das Ziel mit `E` – Aschenschild oder unterbrich ihn mit Glutspalter.“ |
| Laternen erscheinen | „Zerstöre die Seelenlaternen, solange sie den Vogt stärken.“ |

### 6.14 Auslöser (Implementierungsübersicht)

| Auslöser-ID | Bedingung | Aktion |
|---|---|---|
| `trg_m01_intro` | Missionsstart | Kamerafahrt, `narrator.m01_opening`, danach `char_hal_venn.m01_intro`, Ziele `obj_m01_farm` + `obj_m01_barracks` |
| `trg_m01_farm_done` | Anzahl `farmstead` ≥ 2 (fertig) | Ziel abschließen, `char_hal_venn.m01_first_farm` |
| `trg_m01_barracks_done` | `barracks` fertig | Ziel abschließen, `char_hal_venn.m01_barracks`, `obj_m01_army` aktivieren |
| `trg_m01_warn_n1` | Spielzeit 07:30 | `char_hal_venn.m01_wave_warning`, Feuerstellen freischalten, Musik `build_tension` |
| `trg_m01_dusk` | Phasenwechsel Dämmerung | `obj_m01_nights`, `sfx.horn_alarm`; 45 s später `narrator.m01_night_falls` |
| `trg_m01_wave_*` | Zeitstempel laut 6.8 | Wellen erzeugen (Skalierung nach Schwierigkeit und zerstörten Knochentürmen) |
| `trg_m01_e1` | Morgengrauen nach Nacht 1 + 30 s | Ereignisfenster E1 |
| `trg_m01_graves` | Spielzeit 21:30 und `flag_m01_dead_burned = false` | Welle 2c am Friedhof |
| `trg_m01_marrow` | Spielzeit 15:00 | Nebenziel `obj_m01_marrow`, Angriff auf Lager (M) |
| `trg_m01_shrine_seen` | Eigene Einheit sieht Feld (128, 20) oder Spielzeit 18:00 | `char_hal_venn.m01_shrine_found`, `obj_m01_shrine` |
| `trg_m01_knights` | Spielzeit 24:00 | 2 `knight` (Erfahren) am Nordrand der Grauen Straße |
| `trg_m01_boss_aggro` | Eigene Einheit im Radius 14 um den Schrein | Musik `boss_pale_reeve`, `voice.boss_pale_reeve.taunt` |
| `trg_m01_phase2` | Vogt-HP ≤ 60 % | Fahle Ernte, Laternen erzeugen |
| `trg_m01_phase3` | Vogt-HP ≤ 30 % | Totenglocke und Raserei aktivieren |
| `trg_m01_reeve_dead` | Vogt tot | Sterbezeile, Fahler Schleier aus, Krypta-Eingang sichtbar |
| `trg_m01_victory` | `hollow_shrine` zerstört | Hohle im Bindungsbereich zerfallen, `char_hal_venn.m01_victory`, `narrator.m01_victory`, `victory_sting`, Missionsauswertung |
| `trg_m01_defeat` | Niederlagebedingung | `narrator.m01_defeat`, `defeat_sting` |

### 6.15 Balancing-Richtwerte (Lehnsherr)

| Messpunkt | Richtwert |
|---|---|
| Zweites Gehöft fertig | 02:00–03:00 |
| Kaserne fertig | 04:00–05:30 |
| Militär bei Beginn Nacht 1 | Miliz (5) + Isolde + 3–5 neue Einheiten + optional 1 Wachturm |
| Verluste Nacht 1 | 0–3 Einheiten |
| `population_cap` beim Angriff auf Vael Dorn | 30–35 (Bergfried II + 3–4 Gehöfte) |
| Typische Angriffsarmee | 5–6 `swordsman`, 4–6 `spearman`, 5–6 `archer`, 2–3 `knight`, Isolde |
| Bosskampfdauer | 3–5 Minuten |
| Gesamtdauer | 35–50 Minuten (schnelle Spieler: Sieg vor Nacht 2 in ca. 20 Minuten möglich) |
| Verlustquote gesamt | 25–40 % der ausgebildeten Einheiten |

### 6.16 Ergebnisübernahme in die Kampagne

| Ergebnis | Übernahme |
|---|---|
| Überlebende Militäreinheiten | Bis zu 12 Einheiten (höchste Erfahrung zuerst) begleiten Isolde in `m02_harrowmere` |
| Isolde | Erfahrung, Gegenstände (ggf. Raventhorns Glutklinge), Verletzungen |
| Stadtzustand Brennhollow | Gebäude und Ausbau werden als Stadtkarte gespeichert und in `m04_long_night` sowie auf der Weltkarte verwendet |
| Variablen | `flag_m01_dead_burned`; `rep_flame` (+5 bei Verbrennung); Erfolge |
| Bevölkerung | Überlebende Einwohner + Familie Marrow (falls gerettet) bilden die Startbevölkerung der Provinz Brennhollow |
| Chronik | Eintrag „Die letzte Siedlung“ (siehe Beispiel unten) |

**Beispiel des generierten Chronik-Eintrags** (bei Sieg in Nacht 3, Verbrennung der Toten, Familie Marrow gerettet):

> **897 n.F., im Monat der Fahlen Ernte.** Drei Nächte lang kamen die Hohlen nach Brennhollow, und drei Nächte lang
> brannten die Feuer auf der Palisade. Die Toten der ersten Nacht übergab Bruder Hal Venn der Flamme, damit das Tief sie
> nicht hole. Am dritten Tag ritten die letzten Reiter von Grauwall heim, und in der Langen Nacht stellte Isolde
> Raventhorn den Fahlen Vogt in den Ruinen von Vael Dorn. Der Vogt, der einst Aldous Meringer hieß, fiel unter
> ihrer Klinge, und mit ihm der Schrein, den er hütete. Am Morgen danach zählte Brennhollow mehr Seelen als vor
> den drei Nächten, denn die Familie Marrow war aus dem Westwald heimgekehrt, und mit ihr die Hoffnung.

---

## 7. Querschnitt: Kampagnenweite Systeme

### 7.1 Wiederkehrende Orte

| Ort | Zeitalter | Entwicklung |
|---|---|---|
| Brennhollow | I, II, IV, V, VII | Von 30 Seelen zum Marktflecken, zur Herzogsstadt, zur Großstadt; in m26 möglicherweise aufgegeben |
| Vael Dorn | Prolog B, I, V | Ruinen; kann ab Zeitalter II als Grenzfeste wiederaufgebaut werden (Weltkarten-Projekt), in m20 Schauplatz von Isoldes letzter Wahl |
| Kaldris | II, V, VI | Sitz des Erzflammenwahrers; Turnier, Königsmord, Kirchenreform |
| Solmere | Prolog A, VI, VII | Vom brennenden Reich zur Ruine zur Festung des Hohlen Königs |

### 7.2 Wiederkehrende Musik- und Bildmotive

- **Das Glutmotiv** (Isolde, später der Erbe): Wird mit jedem Zeitalter voller instrumentiert; in m29 erklingt es in
  der Fassung, die dem gewählten Ende entspricht.
- **Die fahle Glocke** (Totenglocke des Vogts): Kehrt in der Schleierphase des Hohlen Königs wieder – als Hinweis, dass
  alle Vögte Varkhans Diener waren.
- **Die zerbrochene Sonnenscheibe**: Wappen Aureths; im Ende „Der Thron aus Glut“ setzt sie sich aus Glut neu zusammen.

### 7.3 Schwierigkeit und Fortschritt über die Kampagne

| Zeitalter | Typische Armeegröße (Bevölkerungspunkte) | Typische Missionsdauer | Weltkartenumfang |
|---|---|---|---|
| I | 15–40 | 30–50 min | Aschmark (5 Provinzen) |
| II | 40–70 | 40–60 min | Valemark (24 Provinzen) |
| III | 60–100 | 40–70 min | Valemark, Skarnheim, Dornwaldrand, Grauholm |
| IV | 80–140 | 45–75 min | + Aurethwüste-Rand, Glutweite |
| V | 100–160 | 45–90 min | Gesamtes Valemark mit allen Häusern |
| VI | 100–160 + Reserven | 45–80 min | Ganz Eldraeth außer Schleiernarbe |
| VII | 160 + mehrere Reservewellen | 60–120 min | Ganz Eldraeth |

### 7.4 Wiederspielwert der Kampagne

- 16 Entscheidungspunkte mit insgesamt 48 Optionen, 2 alternative Missionen (m14a/m14b), 4 Erbenvarianten,
  6 Enden.
- Die Weltkartenphasen erzeugen bei jedem Durchgang andere Ereignisketten und damit andere Chroniken.
- Nach dem ersten Abschluss: „Zeitalter wählen“ mit vorgefertigten Variablenkombinationen, um gezielt andere Wege
  zu erkunden; die eigene Chronik bleibt im Ahnenbuch erhalten.
