# ASHEN CROWNS – Asset-Produktionsplan

Wie Sprites heute entstehen, wie handgezeichnete Ersatz-Assets eingebunden werden, welche Namen und Größen gelten und
in welcher Reihenfolge Kunst für die kommenden Phasen produziert werden soll. Grundlage ist der **tatsächliche Code**
(`crates/ashen_game/src/art/*`, `crates/ashen_engine/src/assets/*`). Kunstrichtung und Shader:
[`graphics_and_shaders.md`](graphics_and_shaders.md). Roadmap-Phasen: [`../tech/roadmap.md`](../tech/roadmap.md).

---

## 1. Die prozedurale Pipeline

Alle Sprites des Vertical Slice werden **beim Start** von `ArtLibrary::build` (`art/mod.rs`) im Speicher erzeugt;
es sind keine Bilddateien nötig. Das Ergebnis ist ein einziger Atlas (Albedo plus Normalen/Emission).

```text
AtlasBuilder::new()
  terrain::build    →  tile_*, edge_*, bridge, foundation_*
  units::build      →  <unit>_<anim>_<n>, <unit>_portrait
  buildings::build  →  bld_*, scaffold_*, banner_*  (+ BuildingMeta)
  props::build      →  tree_*, node_*, decor_*
  fx::build         →  fx_*, proj_*, shadow_*, ring_*, marker_*, rally_flag
  icons::build      →  icon_*
  PNG-Overrides     →  assets/sprites/*.png (ersetzen gleichnamige Sprites, siehe Abschnitt 2)
AtlasBuilder::build →  Atlas (Regal-Packer)  →  Renderer::set_atlas (GPU)
```

Beim Start protokolliert das Spiel (Log-Ebene `info`):
`Atlas <B>×<H> mit <n> Sprites (<k> Overrides) in <ms> ms erzeugt`. Das ist die schnellste Kontrolle, ob Overrides
geladen wurden.

### 1.1 `Canvas` (`ashen_engine::assets::pixel`)

Eine `Canvas` ist ein Pixelraster mit zwei Ebenen je Pixel: **Farbe** (`Rgba`) und **Normale/Emission** (`NormalPx`:
`nx`, `ny`, `emissive`, `fixed`). Wichtige Primitive:

| Funktion | Zweck |
|---|---|
| `px`, `pxn` | Pixel setzen (ohne bzw. mit Normale); Alpha 0 wird ignoriert, Alpha < 255 geblendet |
| `glow(x, y, e)` | Emission eines Pixels (0..1) |
| `rect`, `rect_n`, `line` | Flächen und Bresenham-Linien |
| `ellipse_shaded` | Gefüllte Ellipse mit Kugelschattierung aus einer `Ramp` (Licht links oben) und passenden Normalen |
| `polygon` | Scanline-Polygon mit Shader-Callback `(x, y) → (Farbe, Normale)` (Fassaden, Dächer) |
| `outline(darkness)` | Selektive Kontur (dunkle Variante der Nachbarfarbe) |
| `pillow_normals(strength)` | Rundet Normalen aus einer Distanztransformation der Silhouette plus Helligkeitsfeinstruktur |
| `grain(amount, seed)` | Materialrauschen |
| `blit`, `rotated` | Kopieren (optional gespiegelt), Drehen um einen Pivot (genutzt für Todesanimationen) |
| `from_png(path, normal_path)` | Laden aus PNG (Override-Pfad) |

Farbe entsteht fast immer über **Rampen** (`ashen_core::color::Ramp`, Palette in `art/palette.rs`) mit
`Ramp::dither(t, x, y)` (4×4-Bayer). Variation ohne Zufallsgenerator: `ashen_core::noise::{rand2, fbm2, hash2}` mit festen
Seeds, daher ist die Atlas-Erzeugung deterministisch.

### 1.2 Kissen-Normalen (Pillow)

`finish(canvas)` in `art/units.rs` und `art/props.rs` führt aus: `outline(0.42)`, dann `pillow_normals(0.9)`. Die
Silhouette wird per Chamfer-Distanztransformation vermessen; die Höhe `sqrt(min(Distanz, 4))` ergibt Gradienten, die
zusammen mit der Helligkeit benachbarter Pixel (dunkle Pixel = Vertiefung) die Normale bilden. Explizit gesetzte
Normalen (`fixed = 1`, z. B. Gebäudeflächen mit `normals::LEFT/RIGHT/UP`) bleiben erhalten. PNG-Overrides ohne
Normalen-Datei erhalten `pillow_normals(0.8)`.

### 1.3 Atlas-Packer

`AtlasBuilder::build`: Sprites absteigend nach Höhe und Breite sortieren, in Regalen packen, **1 Pixel Abstand**
(gegen Ausbluten bei Nearest-Sampling), Breite ab 512 px in Zweierpotenzen wachsend (bis 8192 px), Höhe auf die
nächste Zweierpotenz. Zusätzlich wird das Sprite `__missing` (8×8 magenta Schachbrett) eingefügt. `Atlas::get(name)`
liefert bei unbekanntem Namen dieses Ersatz-Sprite und protokolliert `Sprite '<name>' fehlt im Atlas`.
`Atlas::save_png` existiert als Debug-Methode, wird aber von keinem Befehl aufgerufen.

### 1.4 Umfang (rechnerisch aus den `add`-Aufrufen)

| Kategorie | Anzahl | Inhalt |
|---|---|---|
| Gelände | 77 | 40 Kacheln, 32 Übergänge, 1 Brücke, 4 Fundamente |
| Einheiten | 184 | 10 Einheiten mit Animationen und Porträts |
| Gebäude | 13 | 6 Gebäude, 4 Gerüste, 3 Banner |
| Props | 43 | 11 Bäume, 2 Quellen, 30 Dekorationen |
| Effekte | 24 | Partikel, Schatten, Ringe, Projektile, Marker |
| Icons | 17 | 16×16 |
| **Summe** | **≈ 358** | plus `__missing` |

---

## 2. PNG-Override-Konvention

Handgezeichnete Sprites ersetzen prozedurale **schrittweise und ohne Codeänderung**.

### 2.1 Regeln

| Regel | Beschreibung |
|---|---|
| Ort | `assets/sprites/<name>.png` (Verzeichnis anlegen, falls nicht vorhanden) |
| Name | Dateiname ohne `.png` = Atlas-Name des zu ersetzenden Sprites (Abschnitt 3). Ein Name, den es im Atlas noch nicht gibt, wird als **neues** Sprite hinzugefügt. |
| Normalen | Optional `assets/sprites/<name>_n.png`, **gleiche Pixelgröße** wie das Albedo. Stimmt die Größe nicht, wird die Normalen-Datei **stillschweigend ignoriert** (flache Normalen, keine Kissen-Normalen). Fehlt die Datei, werden Kissen-Normalen (`pillow_normals(0.8)`) erzeugt. |
| Dateien mit `_n` im Namen | Dateien, deren Name auf `_n` endet, werden nie als Albedo geladen. |
| Format | PNG (die `image`-Crate ist nur mit dem Feature `png` eingebunden – andere Formate werden nicht gelesen). Gelesen wird als 8-Bit-RGBA. |
| Fehler | Nicht ladbare PNGs erzeugen eine Warnung (`Sprite-Override <name> nicht ladbar`); das prozedurale Sprite bleibt. |
| Zeitpunkt | Beim Start; Overrides werden nach der prozeduralen Erzeugung angewandt und ersetzen deren Eintrag. Neustart des Spiels nötig. |
| Kontrolle | Log `Atlas … (<k> Overrides)`; Headless-Screenshot (`--screenshot`) zum Vergleichen. |

Albedo: Farben in sRGB, Alpha **nicht vormultipliziert**. Der Shader dithert Alpha (Bayer 4×4) statt zu blenden; saubere
Sprites verwenden deshalb nur die Alphawerte 0 und 255.

Normalen-PNG: `R` = Normale x, `G` = Normale y, `B` = Emission, `A` wird ignoriert (die Abdeckung stammt aus dem
Albedo-Alpha). Kodierung `Kanal = (n · 0,5 + 0,5) · 255`; `127/128` bzw. `128/128` gelten als flach (die Engine nutzt 128 als „flach“ für unbelegte Pixel). Bildschirmraum: **y zeigt nach unten**.
Referenzwerte der Engine (`normals::*`, z. B. für Gebäudeflächen):

| Fläche | Normale `(x, y, z)` | `R`, `G` |
|---|---|---|
| Flach / frontal | `(0, 0, 1)` | 127–128, 127–128 |
| Nach oben geneigt (Dachfirst, Oberseite) | `(0; −0,75; 0,66)` | 127, 31 |
| Linke Fassade | `(−0,8; 0,15; 0,58)` | 25, 146 |
| Rechte Fassade | `(0,8; 0,15; 0,58)` | 229, 146 |
| Dach links / rechts | `(∓0,55; −0,55; 0,63)` | 57 bzw. 197, 57 |

Emission (`B`): 0 = keine. Die Emission des Atlas wirkt immer und wird durch die Instanz-Emission verstärkt
(`emission = B · (1 + p0.y)`). Eingemalte Leuchtpixel (Fenster, Glut, Runen, Augen der Hohlen) leuchten damit von selbst
und speisen den Bloom; Punktlichter (`BuildingMeta`) sorgen zusätzlich für den Lichtschein auf der Umgebung.

### 2.2 Anker

Ein Override **ersetzt** das gleichnamige prozedurale Sprite (`AtlasBuilder::replace`). Hat das PNG **dieselbe Größe**
wie das prozedurale Original, wird dessen Ankerpunkt übernommen – das gilt für alle Kategorien (Figuren, Gelände-Kacheln,
Übergänge, Gebäude, Gerüste, Fundamente, Minen, Effekte). Weicht die Größe ab, gilt der Ersatzanker
`(Breite / 2, Höhe − 2)` (unten Mitte), der für Figuren, Bäume und Dekoration passt.

Empfehlung: Overrides in exakt der Größe des prozeduralen Sprites anlegen (Größen siehe Abschnitt 3 bzw. Atlas-Export).
Für Gebäude mit abweichender Größe ist eine Ankerangabe pro Datei (z. B. `<name>.anchor.ron`) geplant (Phase 15).

## 3. Namenskonventionen der vorhandenen Sprites

Alle Namen klein, `snake_case`. `<n>` ist eine Frame-Nummer ab 0 ohne führende Null.

### 3.1 Einheiten: `<unit>_<anim>_<n>`

`<unit>` ist die Einheiten-ID aus `units.ron`. Sprites existieren für die zehn Einheiten in `UNIT_ART`:
`worker`, `swordsman`, `spearman`, `archer`, `knight`, `hero_isolde`, `hollow_thrall`, `hollow_archer`, `grave_brute`,
`boss_pale_reeve`. Mod-Einheiten ohne eigene Sprites verwenden `swordsman` (Glutkrone/neutral) oder `hollow_thrall`
(Hohle) als Basisfigur (`ArtLibrary::unit_anims`).

| Animation (`<anim>`) | Frames | Tempo | Wiederholung | Zuständigkeit |
|---|---|---|---|---|
| `idle` | 2 (`_0`, `_1`) | 1,6 fps | Schleife | alle Einheiten |
| `walk` | 4 | 8 fps | Schleife | alle Einheiten |
| `attack` | 4 | 9 fps | einmalig | alle Einheiten |
| `death` | 5 | 9 fps | einmalig (letztes Bild bleibt) | alle Einheiten |
| `work` | 4 | 6 fps | Schleife | nur `worker` |
| `carry` | 4 | 8 fps | Schleife | nur `worker` (Gehen mit Last) |
| `cast` | 4 | 8 fps | einmalig | `hero_isolde`, `knight`, `grave_brute`, `boss_pale_reeve` |
| `<unit>_portrait` | 1 | – | – | alle (14×14, Kopf-/Schulterausschnitt für das HUD) |

Wiedergabe-Logik (`WorldView::render`): `Walk`/`Flee` → `walk` (mit Last `carry`); `Attack` → `attack`; `Work` → `work`;
`Cast` → `cast` (sonst `attack`); sonst `idle` (oder `walk`, solange die Einheit einen Weg verfolgt). Fehlt ein Clip,
fällt `AnimationSet::get` auf `idle` zurück. Einmalige Clips (`attack`, `cast`) springen nach Ende auf `idle`.
Der Tod wird als „Leiche“ gezeichnet (`death`-Clip, danach letztes Bild, 22 s sichtbar, ab 16 s ausblendend).
Alle Frames sind für „blickt nach rechts“ gezeichnet; links entsteht durch Spiegelung (Normale x wird im Shader invertiert).

Frame-Bestand je Einheit:

| Einheit | Rahmen (B×H) | Frames |
|---|---|---|
| `worker` | 26×30 | idle 2, walk 4, attack 4, work 4, carry 4, death 5, portrait 1 = 24 |
| `swordsman`, `spearman`, `archer`, `hollow_thrall`, `hollow_archer` | 26×30 | idle 2, walk 4, attack 4, death 5, portrait 1 = 16 |
| `hero_isolde` | 26×30 | wie oben plus cast 4 = 20 |
| `knight` | 36×34 | idle 2, walk 4, attack 4, cast 4, death 5, portrait 1 = 20 |
| `grave_brute` | 34×34 | wie `knight` = 20 |
| `boss_pale_reeve` | 36×48 | wie `knight` = 20 |

### 3.2 Gebäude und Zubehör

| Name | Inhalt | Größe (B×H) | Anker (prozedural) |
|---|---|---|---|
| `bld_<id>` | Gebäude, `<id>` aus `buildings.ron`: `keep`, `farmstead`, `barracks`, `watchtower`, `hollow_shrine`, `bone_spire` | siehe 5.2 | Obere Ecke der Grundfläche `(Höhe_Kacheln · 16, oy)` |
| `scaffold_<w>x<h>` | Gerüst für Baustellen: `1x1`, `2x2`, `3x2`, `3x3` | `((w+h)·16, (w+h)·8 + 37)` | wie Gebäude (`oy` = 36) |
| `foundation_<w>x<h>` | Fundamentrahmen am Boden (gleiche vier Größen) | `((w+h)·16, (w+h)·8)` | `(h · 16, 0)` |
| `banner_<0..2>` | Banner-Animation (3 Phasen) | 10×20 | `(1, 18)` |

Mod-Gebäude ohne `bld_<id>`-Sprite werden als magentafarbenes Ersatz-Sprite gezeichnet.

### 3.3 Gelände

| Name | Inhalt | Größe | Anker |
|---|---|---|---|
| `tile_<terrain>_<0..3>` | Bodenkachel, vier Varianten; `<terrain>` ∈ `grass`, `darkgrass`, `dirt`, `road`, `stone`, `sand`, `ash`, `ford`, `water`, `deepwater` | 32×16 | `(16, 8)` |
| `edge_<terrain>_<0..3>` | Übergangsrand; `<terrain>` ∈ `sand`, `ash`, `dirt`, `stone`, `grass`, `darkgrass`, `road`, `ford`; Seite `0` = NE, `1` = SE, `2` = SW, `3` = NW | 32×16 | `(16, 8)` |
| `bridge` | Holzbrücke (über Straßenkacheln neben Wasser) | 32×22 | `(16, 10)` |

### 3.4 Props

| Name | Größe (B×H) | Anker |
|---|---|---|
| `tree_pine_<0..3>` | 26×46 | `(13, 43)` |
| `tree_oak_<0..3>` | 32×40 | `(16, 37)` |
| `tree_dead_<0..2>` | 28×40 | `(14, 37)` |
| `node_gold_mine` | 66×50 | `(33, 18)` (obere Ecke der 2×2-Fläche) |
| `node_quarry` | 64×44 | `(32, 12)` |
| `decor_rock_<0..2>` | 10×7 | `(5, 5)` |
| `decor_boulder_<0..2>` | 26×20 | `(13, 16)` |
| `decor_bush_<0..2>` | 16×12 | `(8, 10)` |
| `decor_tuft_<0..2>` | 8×6 | `(4, 5)` |
| `decor_flowers_<0..2>` | 8×5 | `(4, 4)` |
| `decor_reeds_<0..2>` | 10×12 | `(5, 11)` |
| `decor_grave_<0..2>` | 10×14 | `(5, 12)` |
| `decor_bones_<0..2>` | 12×6 | `(6, 4)` |
| `decor_ruin` | 26×30 | `(13, 24)` |
| `decor_pillar` | 14×34 | `(7, 31)` |
| `decor_brazier` | 10×16 | `(5, 14)` |
| `decor_lantern` | 8×22 | `(3, 20)` |
| `decor_fence` | 18×12 | `(9, 9)` |
| `decor_stump` | 10×8 | `(5, 6)` |

### 3.5 Effekte, Marker, Icons

| Name | Größe | Verwendung |
|---|---|---|
| `fx_smoke`, `fx_smoke_big`, `fx_dust`, `fx_heat`, `fx_glow` | 9×9, 13×13, 7×7, 11×11, 7×7 | Weiche Partikel; `fx_heat` als Material *Heat* |
| `fx_pixel`, `fx_dot`, `fx_spark` | 2×2, 1×1, 5×5 | Funken, Splitter |
| `fx_flame_<0..3>` | 8×12 | Flammenanimation (emissiv) |
| `proj_arrow`, `proj_bone`, `proj_soul` | 8×3, 6×3, 7×7 (Seelenblitz) | Projektile |
| `shadow_s`, `shadow_m`, `shadow_l` | 14×7, 20×9, 30×12 | Weiche Blob-Schatten (Alpha-Verlauf) |
| `ring_s`, `ring_m`, `ring_l` | 17×10, 23×13, 35×19 | Auswahlringe (weiß, werden getönt) |
| `marker_move`, `marker_attack` | 11×7 | Befehlsmarker |
| `rally_flag` | 8×14 | Sammelpunkt-Fahne |
| `icon_<name>` | 16×16 | `gold`, `wood`, `stone`, `food`, `pop`, `attack`, `move`, `stop`, `hold`, `patrol`, `build`, `rally`, `cancel`, `demolish`, `ember_cleave`, `rallying_banner`, `ashen_ward` |

Fähigkeiten und Gebäude aus Mods ohne `icon_<ability-id>` erscheinen mit dem Ersatz-Sprite.

---

## 4. Spezifikationen für handgezeichnete Ersatz-Assets

### 4.1 Allgemein

* **Stil:** Dunkle Pixel-Art laut [`graphics_and_shaders.md`](graphics_and_shaders.md) Abschnitt 1: gedämpfte
  Rampen, 4–6 Stufen je Material, Dithering statt weicher Verläufe, **kein Anti-Aliasing**, kein Skalieren oder Drehen
  im Bildprogramm.
* **Licht:** Von links oben (entspricht Konvention der prozeduralen Sprites). Eingemalte Schatten nur schwach; der
  Renderer addiert Sonne, Schatten und Punktlichter selbst.
* **Kontur:** 1 Pixel, dunkle Variante der angrenzenden Farbe (Faktor ≈ 0,42), keine reine Schwarzkontur.
* **Palette:** Farben aus `art/palette.rs` verwenden; Akzente (Glut-Orange, Hohl-Grün) nur an Lichtquellen und
  Magie. Fraktionsfarben nach `docs/canon.md`.
* **Blickrichtung:** Alle Einheiten blicken **nach rechts** (Bildschirm), Spiegelung übernimmt der Renderer.
* **Fußpunkt:** Unten Mitte; Figuren stehen mit den Füßen auf Pixelzeile `Höhe − 2` (Anker 2 px über der Unterkante).
* **Alpha:** Nur 0 oder 255; Hintergrund vollständig transparent.
* **Normalen (`_n.png`):** gleiche Größe wie das Albedo; Referenzwerte siehe Abschnitt 2.1. Sprites ohne `_n`-Datei
  erhalten automatisch Kissen-Normalen – für Figuren und organische Formen in der Regel ausreichend. Für harte
  Flächen (Gebäude, Kisten, Mauern) Normalen explizit malen.

### 4.2 Maße je Kategorie

| Kategorie | Maße | Anker | Hinweise |
|---|---|---|---|
| Infanterie / Arbeiter / Held | 26×30 | `(13, 28)` | Figurenhöhe etwa 20–24 px inkl. Helm; Waffe darf in die Breite ragen |
| Ritter (Kavallerie) | 36×34 | `(18, 32)` | Pferd und Reiter in einem Sprite |
| Grabkoloss | 34×34 | `(17, 32)` | Große Silhouette, Keule |
| Boss (Fahler Vogt) | 36×48 | `(18, 46)` | Höchste Figur; Lichtquelle (grün) wird vom Spiel gesetzt |
| Porträt | 14×14 | – | Kopf- und Schulterbereich, Ausschnitt aus dem Idle-Frame |
| Gelände-Kachel | 32×16 | `(16, 8)` | **Exakte Rautenform** (`art::terrain::in_diamond`: Zeilen 0–7 Halbbreite `(y+1)·2`, Zeilen 8–15 Halbbreite `(16−y)·2` Pixel; Mitte x = 16); nahtlos kachelbar; 4 Varianten; Wasserkacheln nur mittelgraue Struktur (wird vom Wasser-Shader eingefärbt, Referenz `water_tex()` 3c4448–5c666b) |
| Übergangsrand | 32×16 | `(16, 8)` | Nur der Randstreifen an der jeweiligen Seite, Rest transparent |
| Gebäude | `((w+h)·16, (w+h)·8 + oy + 1)` | `(h·16, oy)` | `oy` je Gebäude: Bergfried 100, Gehöft 40, Kaserne 54, Wachturm 66, Hohlschrein 92, Knochenturm 66 |
| Baum | 26×46 (Kiefer), 32×40 (Eiche), 28×40 (tot) | Fuß `(B/2, H−3)` | Krone mit Platz für Windscherung (bis ±0,03 · Höhe) |
| Dekoration | siehe 3.4 | Fuß | Kleine Dekos ohne Kontur zulässig |
| Icon | 16×16 | – | Klar lesbar bei Anzeige 28–32 px; Gold-/Karmesin-Akzente |

Gebäudegrößen im Einzelnen: Bergfried 3×3 → 96×149; Gehöft 2×2 → 64×73; Kaserne 3×2 → 80×95; Wachturm 1×1 → 32×83;
Hohlschrein 3×3 → 96×141; Knochenturm 1×1 → 32×83. Gerüste: `scaffold_1x1` 32×53, `2x2` 64×69, `3x2` 80×77,
`3x3` 96×85 (Höhe = `(w+h)·8 + 37`).

### 4.3 Animationen neuer Einheiten

Ein vollständiger Einheitensatz hat **16 Sprites** (idle 2, walk 4, attack 4, death 5, portrait 1), dazu optional `cast 4`
(Zauberer, Helden) bzw. `work 4` + `carry 4` (Arbeiter). Frame-Namen exakt nach 3.1. Alle Frames eines Satzes haben
**dieselbe Leinwandgröße und denselben Fußpunkt**, damit Wechsel zwischen Clips nicht springen.

### 4.4 Qualitätskriterien (Abnahme)

1. Name und Größe entsprechen den Tabellen; Albedo und `_n` haben gleiche Größe.
2. Lesbar in Originalgröße **und** bei Skalierung 2 und 3 (typische Bildschirmskalierung bei 1600×900, 330 Zeilen: 3).
3. Silhouette unterscheidet sich von allen anderen Einheiten derselben Fraktion.
4. Funktioniert bei Tag, Nacht, Nebel und im Kriegsnebel (Headless-Screenshots mit `--time`, `--weather`, siehe
   `project_structure.md`).
5. Keine halbtransparenten Pixel außer bewusst gewählten Rändern; keine Pixel mit `A` zwischen 1 und 254 in Konturen.
6. Palette konform (keine neuen Akzentfarben ohne Abstimmung).
7. Log zeigt den Override in der Zählung `(<k> Overrides)`; keine Warnung `nicht ladbar`.

---

## 5. Animationsliste je Einheit (Bestand und Lücken)

Bestand der Vertical Slice (alle prozedural, Frames siehe 3.1):

| Einheit | Fraktion | idle | walk | attack | death | work/carry | cast | Besonderheit der Figur |
|---|---|---|---|---|---|---|---|---|
| `worker` | Glutkrone | ✓ | ✓ | ✓ (Axt) | ✓ | ✓ | – | Leinen, Kappe |
| `swordsman` | Glutkrone | ✓ | ✓ | ✓ (Schwert) | ✓ | – | – | Stahl, Schild, Kesselhaube |
| `spearman` | Glutkrone | ✓ | ✓ | ✓ (Speer) | ✓ | – | – | Leder, Kesselhaube |
| `archer` | Glutkrone | ✓ | ✓ | ✓ (Bogen) | ✓ | – | – | Kapuze, Umhang |
| `knight` | Glutkrone | ✓ | ✓ | ✓ (Lanze) | ✓ | – | ✓ | Pferd |
| `hero_isolde` | Glutkrone | ✓ | ✓ | ✓ (Glutklinge) | ✓ | – | ✓ | Stahlrüstung, Kronreif, roter Umhang |
| `hollow_thrall` | Hohle | ✓ | ✓ | ✓ (rostige Klinge) | ✓ (Knochenstaub) | – | – | Totenschädel |
| `hollow_archer` | Hohle | ✓ | ✓ | ✓ (Knochenbogen) | ✓ | – | – | Kapuze |
| `grave_brute` | Hohle | ✓ | ✓ | ✓ (Keule) | ✓ | – | ✓ | Koloss |
| `boss_pale_reeve` | Hohle | ✓ | ✓ | ✓ (Stab) | ✓ | – | ✓ | Größte Figur |

Bekannte Lücken (nicht vorhanden, siehe Abschnitt 6 für die Planung):

| Fehlt | Betrifft | Geplant in Phase |
|---|---|---|
| Eigene Animationen für Fähigkeiten über `cast` hinaus (z. B. Beschwören, Kanalisieren) | Helden, Zauberer | 6, 7 |
| Reit-/Fahrzeug- und Belagerungsgerät-Animationen | Neue Militäreinheiten | 5 |
| Gebäude-Animationen über Banner und Feuer hinaus (Tore, Mühlenräder, Rauchzustände) | Neue Wirtschaftsgebäude | 4 |
| Weitere Fraktionssätze | Skarn-Clans, Dornenbund, Saelith-Pakt, Aschenlegion | 11 |
| Zusätzliche Tod-/Treffer-Varianten | alle | 15 |

---

## 6. Prioritäten und Produktionsreihenfolge

Kriterien: **Sichtbare Wirkung** (Bildschirmzeit), **technische Machbarkeit mit der aktuellen Override-Pipeline**
(Abschnitt 2.2), **Abhängigkeit von Roadmap-Phasen**.

### Stufe 0 – Voraussetzung: Pipeline-Erweiterung (vor Gelände-/Gebäudekunst)

Override-Pipeline um eine **Ankerangabe pro Sprite** erweitern (derzeit nicht implementiert). Ohne diese Erweiterung
können Gelände und Gebäude nicht per PNG ersetzt werden. Empfohlen vor Beginn der Biom-Arbeit in Phase 2 und spätestens in Phase 15.

### Stufe 1 – Hohe Wirkung, sofort möglich (Vertical-Slice-Politur)

| Reihenfolge | Asset | Grund | Umfang |
|---|---|---|---|
| 1 | `hero_isolde` (idle, walk, attack, cast, death, portrait) | Hauptfigur, ständig im Bild und im HUD | 20 Sprites |
| 2 | `worker` (inkl. work/carry) | Meiste Bildschirmzeit | 24 |
| 3 | `swordsman`, `spearman`, `archer` | Kernarmee | 48 |
| 4 | `hollow_thrall`, `hollow_archer` | Häufigste Gegner | 32 |
| 5 | `knight`, `grave_brute`, `boss_pale_reeve` | Große Silhouetten, Höhepunkte der Mission | 60 |
| 6 | Icons (`icon_*`) und Porträts | Dauerhaft im HUD sichtbar | 17 + 10 |
| 7 | Partikel (`fx_*`), Projektile | Kampfgefühl | je nach Bedarf |

### Stufe 2 – Gelände und Gebäude (nach Stufe 0)

| Reihenfolge | Asset | Umfang |
|---|---|---|
| 8 | Bodenkacheln Gras, Dunkelgras, Erde, Straße, Stein, Sand, Asche (je 4 Varianten) | 28 |
| 9 | Wasser-Kacheln, Furt, Brücke | 9 |
| 10 | Übergangsränder | 32 |
| 11 | Gebäude: Bergfried, Gehöft, Kaserne, Wachturm, Hohlschrein, Knochenturm (+ Normalen) | 6 (+ 6 `_n`) |
| 12 | Gerüste, Fundamente, Banner | 11 |
| 13 | Props: Bäume, Minen, Dekoration | 43 |

### Stufe 3 – Inhalte für kommende Phasen

| Phase | Kunst | Geschätzter Umfang |
|---|---|---|
| 2 (Echtzeitwelt) | Terrain-/Dekor-Sets weiterer Biome der Regionen aus `canon.md` (Valemark-Täler, Skarnheim-Schnee, Dornwald, Aurethwüste, Schleiernarbe, Glutweite); Karten-Sprites für Minen/Kamps | je Biom ≈ 70–90 Sprites |
| 3 (Weltkarte) | Weltkartenhintergrund, Provinzmarker, Armeemarker, Missionsknoten (leuchtend), Wegeverbindungen, Weltkarten-Icons | ≈ 60–100 |
| 4 (Wirtschaft) | Wirtschaftsgebäude (Mühle, Markt, Lager, Schmiede u. a. laut GDD Kapitel 8), Ressourcen-Icons erweitern | ≈ 8–15 Gebäude, 20 Icons |
| 5 (Militär) | Neue Einheiten (je 16–24 Sprites), Belagerungsgerät, Mauern/Tore (Gelände- und Hindernis-Sprites) | je Einheit 16–24 |
| 6 (Helden) | Weitere Helden (je 20 Sprites + Porträt in höherer Auflösung für Charakterbogen), Ausrüstungs-Icons | je Held ≈ 20 + 1 |
| 7 (Magie/Hohlung) | Zauber-Effekte (Zonen, Beschwörung, Verderbnis-Bodentextur), Hohlungs-Icons | ≈ 30–50 |
| 8–10 (Dynastien, Politik, Diplomatie) | Wappen, Herrscherporträts, Hof-/Diplomatie-Oberflächenelemente, Fraktionsbanner | ≈ 100+ (UI-lastig) |
| 11 (Fraktionen/Bosse) | Komplette Einheiten-/Gebäudesätze je Fraktion (Skarn-Clans, Dornenbund, Saelith-Pakt, Aschenlegion), Bosse (Hohler König, Aschdrache, Wurzelmutter) | je Fraktion ≈ 150–250 |
| 12–13 (Ereignisse, Kampagne) | Ereignisillustrationen, Zwischensequenz-Bilder, Missionsbildschirme | nach Kampagnenplan |
| 15 (Politur) | Zusätzliche Animationsphasen, Treffer-/Todesvarianten, UI-Feinschliff | nach Bedarf |

Die Schätzungen sind Planungswerte, keine Zusagen; sie orientieren sich an den Mengen der Vertical Slice
(z. B. 24 Sprites je Arbeiter, 16–20 je Kampfeinheit, 6 Gebäude).

### Produktionsablauf je Asset

1. Spezifikation aus Abschnitt 3 und 4 festlegen (Name, Größe, Anker, Animationsbestand).
2. Albedo (und bei harten Flächen `_n`) zeichnen; Dateien nach `assets/sprites/` legen.
3. Spiel starten, Log prüfen (`Overrides`-Zähler), Screenshot bei Tag, Nacht und Nebel erzeugen
   (`cargo run --release -- --screenshot … --time … --weather …`).
4. Abnahme nach 4.4; bei Gelände/Gebäuden erst nach Umsetzung von Stufe 0.
5. Wenn ein Satz vollständig ist, kann der prozedurale Generator für diese Einheit stillgelegt werden (Phase 15,
   Aufräumen); bis dahin schadet der prozedurale Fallback nicht (er wird einfach überschrieben).

Audio-Produktion: siehe [`../audio/audio_integration.md`](../audio/audio_integration.md).
