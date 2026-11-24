# ASHEN CROWNS – Grafik- und Shader-Konzept

Art Direction, Rasterung, Rendering-Pipeline und Shader im **tatsächlichen Stand** der Vertical Slice. Quellen:
`crates/ashen_engine/src/render/` (Renderer, WGSL), `crates/ashen_game/src/art/` (Sprites, Palette),
`crates/ashen_game/src/view.rs` (Weltdarstellung, Atmosphäre). Architektur-Überblick:
[`../tech/architecture.md`](../tech/architecture.md); Sprite-Produktion: [`asset_production_plan.md`](asset_production_plan.md).

---

## 1. Art Direction

### 1.1 Leitbild

Düstere, gedämpfte Pixel-Art für ein sterbendes Grenzland (Aschmark, Jahr 897 n.F.): erdige, entsättigte Grundtöne,
wenige kräftige Akzente. Licht erzählt die Geschichte: **warmes Glut-Orange** steht für die Glutkrone und menschliche
Zuflucht, **kaltes Fahlgrün** für die Hohlen. Die Palette ist in `crates/ashen_game/src/art/palette.rs` festgelegt;
alle prozedural erzeugten Sprites beziehen ihre Farben ausschließlich von dort.

| Prinzip | Umsetzung im Code |
|---|---|
| Gedämpfte Grundpalette | Rampen für Gras, Erde, Stein, Holz, Stoff, Metall mit niedriger Sättigung und dunklem Fuß (z. B. `grass()` von `#171c13` bis `#4a5233`). |
| Akzente sparsam | Nur sechs Akzentfarben: `EMBER` `#f08a2c`, `EMBER_HOT` `#ffd36a`, `FIRE_DEEP` `#b3361a`, `WINDOW_WARM` `#f2b25a`, `HOLLOW_GLOW` `#86f2a0`, `HOLLOW_DEEP` `#2f9a5a`. |
| Fraktionslesbarkeit | Glutkrone: Karmesin (`crimson()`), Stahl, Gold, Leinen. Hohle: Knochen (`bone()`), Aschgrau (`hollow_cloth()`), Fahlgrün-Glühen. Farbvorgaben nach `docs/canon.md`. |
| Selektive Kontur | `Canvas::outline(OUTLINE = 0.42)`: transparente Nachbarn opaker Pixel werden mit einer **dunklen Variante der Nachbarfarbe** gefüllt statt mit Schwarz; wirkt weicher und hält Silhouetten lesbar. |
| Rampen mit Dithering | `Ramp::dither(t, x, y)` mischt zwischen zwei Rampenstufen mit einer 4×4-Bayer-Matrix (`ashen_core::noise::bayer`); keine weichen Verläufe, sondern Pixelmuster. |
| Licht von links oben | `Canvas::ellipse_shaded` und die Kachel-/Gebäudeflächen schattieren mit Licht von links oben; die Flächennormalen (`normals::UP`, `LEFT`, `RIGHT`, `ROOF_L`, `ROOF_R`) folgen derselben Konvention. |
| Pseudo-3D durch Normalen | Jeder Sprite besitzt eine Normalenkarte. Dynamische Lichter modellieren dadurch Volumen, ohne die 2D-Optik zu verlassen. |
| Quantisierte Beleuchtung | Lichtstufen werden im Shader auf 8 Stufen gerastert (`light.wgsl`), Wasserbänder auf 4, Nebel auf 16 – Pixel-Art-typisches Banding statt glatter Verläufe. |

### 1.2 Lesbarkeit

Lesbarkeit hat Vorrang vor Atmosphäre. Umgesetzte Maßnahmen:

* **Silhouetten:** Jede Einheit hat Kontur und eigene Helm-/Waffenform (Kettenhaube, Kapuze, Totenschädel, Krone,
  Lanze, Keule, Stab). Der Ritter und der Fahle Vogt sind größer (36×34 bzw. 36×48 px) als Infanterie (26×30 px).
* **Bodenkontakt:** Einheiten werfen einen weichen Blob-Schatten (`shadow_s`, `shadow_l` für Kavallerie, Bosse
  und große Monster).
* **Auswahl:** Bodenring (`ring_s`/`ring_m`/`ring_l`) im Material *Unlit*, grün für eigene Einheiten, rot für
  Gegner, Gebäude mit großem Ring und Sammelpunkt-Fahne.
* **Lebensbalken:** Einfarbige Rechtecke (Material *Solid*, unbeleuchtet); eigene Balken laufen von Grün nach Rot, gegnerische
  sind rot; Mana blau, Bau- und Ausbildungsfortschritt eigene Farben. Sichtbar bei Auswahl, Mausüberfahrt, Kampf
  und bei gehaltener `Alt`-Taste.
* **Unbeleuchtete Overlays:** Ringe, Marker, Balken und die Bauvorschau (grün = gültig, rot = ungültig) werden vom
  Lichtmodell, Nebel und Wetter ausgenommen und bleiben bei Nacht und Sturm lesbar.
* **Tonemapping:** Die Kurve ist bis 0,6 **linear**, damit gemalte Farben unverändert bleiben; nur Lichter darüber
  werden weich komprimiert.
* **Kriegsnebel:** Erforschtes Gelände wird entsättigt und abgedunkelt (Faktor 0,6), unerforschtes fast schwarz;
  Gegner im Nebel werden nicht gezeichnet.
* **Treffer-Feedback:** Getroffene Einheiten blitzen kurz auf (Tönung 1,6 / Emission); Flucht färbt Einheiten bläulich.
* **Leichen** werden unter lebenden Einheiten gezeichnet (Tiefenschlüssel −0,3) und blenden nach 16–22 s aus.

### 1.3 Palette (`art/palette.rs`)

Rampen von dunkel nach hell (Hex, ohne `#`). Anzahl der Stufen in Klammern.

| Rampe | Stufen | Dunkelste → hellste | Verwendung |
|---|---|---|---|
| `grass()` | 6 | 171c13 → 4a5233 | Gras-Kacheln |
| `dark_grass()` | 5 | 121610 → 313b27 | Dunkelgras (Wald, Hohlnähe) |
| `dirt()` | 5 | 221e18 → 54493b | Erde, Fundamente |
| `stone()` | 6 | 1a1a1d → 75757a | Stein, Mauern |
| `rock()` | 6 | 1d1c1a → 7c786d | Felsen, Minen |
| `sand()` | 5 | 342c21 → 857456 | Sand |
| `ash()` | 5 | 181719 → 433e40 | Asche-Kacheln |
| `wood()` | 5 | 1e130d → 6e4a2e | Holz, Brücke, Gerüst |
| `dark_wood()` | 4 | 140d09 → 3a261a | Balken, Schatten |
| `plaster()` | 5 | 4a4236 → 9e927a | Putz |
| `roof_red()` | 5 | 2a1213 → 843b30 | Dächer |
| `thatch()` | 5 | 2e2412 → 8a6f36 | Stroh |
| `steel()` | 6 | 1e2228 → b4bec4 | Rüstung, Waffen |
| `iron_dark()` | 4 | 141518 → 3c4046 | Eisen, Helme, Beinschutz |
| `crimson()` | 5 | 2c0a0b → ab3329 | Glutkrone: Tuch, Schilde, Umhänge |
| `gold()` | 5 | 4a3210 → eccb6c | Münzen, Kronreif |
| `skin()` | 5 | 3d241b → c99c79 | Haut |
| `leather()` | 4 | 1f140e → 5f432c | Leder |
| `linen()` | 5 | 3b3122 → ae9a63 | Leinen (Arbeiter) |
| `cloth_brown()` | 5 | 221911 → 72583c | Braunes Tuch |
| `cloth_green()` | 4 | 151a12 → 3b492d | Grünes Tuch (Bogenschützen) |
| `bone()` | 6 | 2e2c24 → d2cbaa | Hohle: Knochen |
| `rot()` | 5 | 1a1e18 → 5b6b56 | Verwesung |
| `hollow_cloth()` | 4 | 0f1012 → 2c3035 | Hohle: Lumpen |
| `pine()` | 6 | 0b130f → 38553b | Kiefern |
| `oak()` | 6 | 10170b → 4d622d | Eichen |
| `bark()` | 4 | 150f0b → 3f2e21 | Rinde |
| `horse()` | 5 | 1a110c → 684a33 | Pferd (Ritter) |
| `water_tex()` | 4 | 3c4448 → 5c666b | Wasser-Basistextur |

Kachelvariation: `WorldView::prepare` multipliziert jede Bodenkachel mit einem Rauschfaktor zwischen 0,82 und 1,18
(`fbm2`), damit große Flächen nicht gekachelt wirken.

---

### 1.4 3D-gebackene Pixel-Art

Figuren und Bauwerke entstehen aus 3D-Modellen, die beim Start ohne Kantenglättung in Sprites gerastert werden
(Albedo, Normalen, Leuchtkraft getrennt). Details: [`3d_pixel_pipeline.md`](3d_pixel_pipeline.md).

## 2. Isometrisches 32×16-Raster

* Kachel = **2:1-Diamant**, 32 Pixel breit, 16 Pixel hoch (`ashen_core::iso::TILE_W` / `TILE_H`).
* Projektion: `welt = ((x − y) · 16, (x + y) · 8)`; Umkehrung `world_to_tile`. Kachel-x läuft auf dem Bildschirm nach
  rechts unten, Kachel-y nach links unten. Die Kachelmitte liegt bei `(x + 0,5, y + 0,5)`.
* **Tiefensortierung:** Schlüssel `x + y` (`depth_of`); größere Werte werden später (weiter vorn) gezeichnet.
  Ein Tiefenpuffer existiert nicht; Weltobjekte werden vor dem Zeichnen sortiert.
* **Ankerpunkte:** Figuren, Bäume und Dekoration sind am **Fußpunkt** verankert; Gebäude, Minen und Fundamente an
  der **oberen Ecke** ihrer Grundfläche (`Iso::anchor`: `(h · 16, oy)`). `SpriteInstance::new` rundet die
  Position auf ganze Pixel – Sprites stehen immer pixelgenau.
* **Kacheln:** `tile_<terrain>_<0..3>` (32×16, `in_diamond` gibt die exakte kachelbare Form vor). Zehn Terrains:
  `grass`, `darkgrass`, `dirt`, `road`, `stone`, `sand`, `ash`, `ford`, `water`, `deepwater`.
* **Übergänge:** Höher priorisierte Nachbarn ragen über `edge_<terrain>_<0..3>` (Seiten 0 = NE, 1 = SE, 2 = SW,
  3 = NW) in niedriger priorisierte Kacheln hinein. Priorität (niedrig → hoch): Tiefes Wasser, Wasser, Furt, Sand,
  Asche, Erde, Stein, Gras, Dunkelgras, Straße. Kantensprites existieren für Sand und höher sowie Furt.
* **Brücke:** Straßenkacheln neben Wasser werden als Wasser mit Brücken-Sprite (`bridge`, 32×22) gezeichnet.
* **Gebäudegrundflächen:** Gebäude belegen 1×1 bis 3×3 Kacheln; Fundament- und Gerüst-Sprites existieren für
  1×1, 2×2, 3×2 und 3×3.

---

## 3. Low-Res-Rendering

Alle Szenen-Passes laufen in einer **niedrigen internen Auflösung**; der letzte Pass vergrößert ganzzahlig.

```text
lines  = max(camera.target_lines, profile.min_lines)
scale  = clamp(round(Ausgabehöhe / lines), 1, 16)
intern = (ceil(B / scale) + 2, ceil(H / scale) + 2)
```

* Startwert `target_lines` der Mission: 330; Zoom per Mausrad 220–480. Das Grafikprofil begrenzt das
  Heranzoomen (`min_lines` 300 / 260 / 220).
* **Nearest-Sampling** beim Hochskalieren: Jeder interne Pixel wird zu einem `scale × scale`-Block, keine Unschärfe.
* **Subpixel-Scrolling:** Die Kamera behält ihren Bruchteil (`ViewParams::subpixel`); der Ausgabe-Shader verschiebt
  die Abtastung damit in Schritten von 1/`scale` interner Pixel. Das Scrollen wirkt flüssig, obwohl das Bild
  pixelgenau bleibt. Zwei Pixel Rand (je 1 Pixel) verhindern Kantenartefakte.
* Beispiel: 1600×900, 330 Zeilen → Skalierung 3, intern 536×302.

---

## 4. Renderpipeline im Überblick

```text
FrameData (CPU) ──► 1 G-Buffer ──► 2 Licht ──► 3 Komposition ──► 4 Bloom ──► 5 Ausgabe ──► 6 egui
```

| Pass | Shader (Einstieg) | Ziel | Inhalt |
|---|---|---|---|
| G-Buffer | `sprite.wgsl` (`vs_main`, `fs_gbuffer` / `fs_shadow` / `fs_heat`) | `g-albedo` `Rgba8UnormSrgb`, `g-normal` `Rgba8Unorm` | Boden, Schatten, Weltobjekte, Hitze, Overlays |
| Licht | `light.wgsl` (`vs_main`, `fs_main`) | `light` `Rgba16Float` | Additiv akkumulierte Punktlichter |
| Komposition | `composite.wgsl` (`fs_main`) | `scene` `Rgba16Float` | Beleuchtung, Emission, Nebel, Wetter |
| Bloom | `bloom.wgsl` (`fs_prefilter`, `fs_down`, `fs_up`) | `bloom0..n` `Rgba16Float` | Helligkeitsfilter, Mip-Kette |
| Ausgabe | `final.wgsl` (`fs_main`) | Fenster-/Headless-Ziel | Skalierung, Tonemapping, Grading |
| UI | egui (`egui-wgpu`) | gleiches Ziel | HUD in voller Auflösung |

`common.wgsl` wird vor jede Shaderdatei gestellt (`ShaderManager::source`) und enthält das gemeinsame
`Globals`-Uniform sowie Hilfsfunktionen: `hash12`, `noise2`, `fbm2` (4 Oktaven), `bayer4`, `world_to_tile` und das
Vollbilddreieck `vs_fullscreen`.

### 4.1 `Globals`-Uniform (`common.wgsl`, `render/types.rs`)

Ein Uniform-Block pro Frame (16 Felder):

| Feld | Inhalt |
|---|---|
| `view_proj` | Weltpixel → Clip-Raum |
| `screen` | interne Breite, Höhe, 1/Breite, 1/Höhe |
| `time` | Zeit (s), Frame-dt, Framezähler, Pixelskalierung |
| `camera` | Weltpixel der linken oberen Ecke, Subpixel-Versatz |
| `ambient` | Umgebungslicht rgb, Intensität |
| `sun_color` | Sonnen-/Mondfarbe rgb, Intensität |
| `sun_dir` | Lichtrichtung xyz (Bildschirmraum), Schattenstärke |
| `fog` | Dichte (0 bei ausgeschaltetem Profil-Nebel), Bodennebel, Wind x, y |
| `fog_color` | rgb, Lichtstreuung |
| `weather` | Regen, Asche, Blitz, Nässe |
| `fow` | Kartenbreite, Kartenhöhe, aktiv, Dunkelheit unerforscht (1,0) |
| `grade` | Belichtung, Kontrast, Sättigung, Vignette |
| `grade_tint` | Tönung rgb, Filmkorn |
| `bloom` | Intensität, Schwelle, Knie, aktiv |
| `output` | Ausgabe-Breite, -Höhe, sRGB-Ziel (1/0), Pixelskalierung |
| `tile` | halbe Kachelbreite, halbe Kachelhöhe, Normalmapping aktiv, Qualität |

---

## 5. Sprite-Instanzen und G-Buffer-Kodierung

### 5.1 `SpriteInstance` (80 Byte)

| Feld | Inhalt |
|---|---|
| `pos`, `size` | Linke obere Ecke und Größe in Weltpixeln |
| `uv` | Atlas-Rechteck (u0, v0, u1, v1) |
| `color` | Tönung rgb und Alpha (Alpha wird dithered, nicht geblendet) |
| `p0` | `x` Material-ID, `y` Emission, `z` Spiegelung (0/1), `w` reserviert |
| `p1` | `x` Scherung, `y` Stauchung, `z` Wassertiefe, `w` reserviert |

Alle Sprites sind Instanzen eines Quads mit 6 Vertices; es gibt keine Vertexpuffer pro Objekt.
Hinweis: Die Kommentare in `sprite.wgsl` beschreiben `p0.w` als „Lichtstärke-Multiplikator“ und `p1.w` als
„Animationsphase“; diese Komponenten werden im Rust-Code nicht gesetzt und im Shader nicht gelesen.

### 5.2 Kodierung

**Ziel 0 – `g-albedo` (`Rgba8UnormSrgb`):**

| Kanal | Inhalt |
|---|---|
| rgb | Albedo (Hardware-sRGB-Dekodierung beim Lesen) |
| a | Hitzeverzerrungsstärke (nur vom Pass *Heat* geschrieben, sonst 0) |

Löschwert: `(0,004; 0,004; 0,008; 0)` (fast schwarz).

**Ziel 1 – `g-normal` (`Rgba8Unorm`):**

| Kanal | Inhalt |
|---|---|
| r, g | Normale x, y: `n.xy · 0,5 + 0,5`; z wird rekonstruiert (`sqrt(1 − x² − y²)`) |
| b | Emission 0..1 |
| a | Flags: `0,0` = beleuchtet, kein Schatten; `0,0…0,49` = beleuchtet, Schattenanteil (`a / 0,49`); `0,6` = **unbeleuchtet** (Unlit, Solid) |

Löschwert: `(0,5; 0,5; 0; 0)` (flache Normale, keine Emission).

Der Schattenanteil wird ausschließlich in den Alphakanal von Ziel 1 geschrieben (Schreibmaske `ALPHA`); die
Hitzestärke in den Alphakanal von Ziel 0. So bleiben Schatten- und Hitzemaske ohne zusätzliche Ziele in den beiden
vorhandenen Texturen untergebracht.

**Alpha-Behandlung:** Sprites werden nicht geblendet. `dither_discard` verwirft Pixel mit `alpha < 0,02` oder
`alpha < bayer4(Pixelposition)` – halbtransparente Sprites (ausblendende Leichen, Geisterbild der Bauvorschau)
erscheinen als Bayer-Raster statt als weicher Verlauf.

**Spiegelung:** `p0.z = 1` spiegelt die U-Koordinate und **negiert `n.x`**, damit die Beleuchtung gespiegelter
Figuren stimmt. Alle Figuren sind nur für „blickt nach rechts“ gezeichnet.

### 5.3 Normalenkarte im Atlas

Der Normalen-Atlas (`Rgba8Unorm`): `r` = Normale x, `g` = Normale y, `b` = Emission, `a` = Abdeckung. Die Atlas-Emission wirkt immer und wird durch die Instanz-Emission `p0.y` verstärkt (`emission = b · (1 + p0.y)`). Sprites mit eingemalter Emission (`Canvas::glow`, z. B. Fenster, Fackeln, Runen) leuchten daher von selbst; `SpriteInstance::emissive(…)` verstärkt sie zusätzlich. Normalen
entstehen entweder explizit (`Canvas::pxn`, Flächen mit `normals::*`) oder per „Kissen-Normalen“
(`Canvas::pillow_normals`) aus einer Distanztransformation der Silhouette plus Helligkeitsfeinstruktur
(dunkle Pixel = Vertiefung). Deaktiviert ein Grafikprofil Normalmapping (`Performance`), wird in Licht- und
Kompositionspass eine flache Normale `(0, 0, 1)` verwendet.

---

## 6. Materialien

Sieben Materialien (`render::Material`, `u8` in `p0.x`). *Shadow* und *Heat* werden nicht über die ID im
Fragment-Shader unterschieden, sondern über eigene Pipelines (Listen `frame.shadows` / `frame.heat`).

| ID | Material | Pipeline / Einstieg | Funktionsweise | Typische Verwendung |
|---|---|---|---|---|
| 0 | **Lit** | `fs_gbuffer` | Standard: Albedo × Tönung, Normale aus dem Atlas, voll beleuchtet (Ambient, Sonne, Punktlichter), Nebel, Wetter. Flags 0. | Gelände, Figuren, Gebäude, Bäume, Dekoration |
| 1 | **Water** | `fs_gbuffer` | Ruhiges Pixel-Art-Wasser, vollständig im Shader: Das Ufer-Distanzfeld liegt als 4-Bit-Wert je Kachelecke gepackt in `p1.w` (`SpriteInstance::water_corners`, vom `WorldView` aus dem Gelände berechnet), wird im Fragment bilinear gelesen und per Bayer-Raster auf 5 Tiefenbänder (Flachwasser-Türkis bis Tiefsee-Blau) abgestuft; Brücken zählen als Wasser, Furten und Land als Ufer. Darüber: dünne, langsam wandernde Wellenlinien (Striche je Zeile, leicht geschwungen), ein weicher Brandungssaum bei geringer Tiefe, seltene Glanzpunkte, flache animierte Normale. Das Kachel-Sprite liefert nur die Form. **Furt:** `p1.z < 0` – sandiger Grund (Kachel-Textur mit Kieseln) scheint durch türkises Flachwasser. Zusätzlich zeichnet der `WorldView` je Uferkante einen Schaumsaum-Streifen (`foam_<terrain>_<seite>`, Alpha pulsiert je Kachel versetzt) unter die Land-Übergänge. | Wasser, Tiefes Wasser, Furt, Brückenkachel |
| 2 | **Unlit** | `fs_gbuffer`, Flags 0,6 | Albedo wird unverändert ausgegeben, **ohne** Beleuchtung, Nebel, Wetter und Kriegsnebel. | Auswahlringe, Befehlsmarker, Bauvorschau |
| 3 | **Emissive** | `fs_gbuffer` | Wie Lit, aber die Emission beträgt mindestens `p0.y` (Instanz-Emission). In der Komposition wird `albedo · emission · 3,2` addiert → HDR-Werte, die den Bloom speisen. | Flammen, Glut, Funken, Seelenblitze, Schildring |
| 4 | **Shadow** | `fs_shadow` | Schreibt nur den Schattenanteil (`alpha · fade · Schattenstärke · 0,49`) in `g-normal.a`; `fade` nimmt von 1,0 (unten) auf 0,55 (oben) ab. Schattenform ist die Alphamaske des Sprites selbst. | Schatten von Bäumen, Gebäuden, hohen Dekorationen; weiche Blob-Schatten der Einheiten |
| 5 | **Heat** | `fs_heat` | Schreibt `alpha · p0.y` in `g-albedo.a`. Die Komposition verschiebt die Abtastung an diesen Pixeln wellenförmig (bis 1,5 interne Pixel). | Hitzeflimmern über brennenden Gebäuden (`fx_heat`) |
| 6 | **Solid** | `fs_gbuffer`, Flags 0,6 | Einfarbiges Rechteck, Textur wird ignoriert, Alpha wird dithered; unbeleuchtet. Aufruf: `SpriteInstance::solid(pos, size, color)`. | Lebens-, Mana- und Fortschrittsbalken |

Zeichenreihenfolge im G-Buffer: **Boden → Schatten → Weltobjekte → Hitze → Overlays**. Da Weltobjekte nach den
Schatten gezeichnet werden und die Schattenmaske mit `flags = 0` überschreiben, fällt Schatten ausschließlich auf den
Boden, nie über Objekte hinweg.

---

## 7. WGSL-Dateien

Alle Quellen liegen in `crates/ashen_engine/src/render/shaders/` und sind per `include_str!` in die Binärdatei
eingebettet.

| Datei | Zeilen | Inhalt |
|---|---|---|
| `common.wgsl` | 80 | `Globals`-Struktur, `@group(0) @binding(0)`, `hash12`, `noise2`, `fbm2`, `bayer4`, `world_to_tile`, `vs_fullscreen` |
| `sprite.wgsl` | 161 | Instanz-Vertex-Shader (Scherung, Stauchung, Spiegelung), `fs_gbuffer` (Lit/Water/Unlit/Emissive/Solid), `fs_shadow`, `fs_heat` |
| `light.wgsl` | 64 | Lichtquads als Ellipsen, Normalen-Beleuchtung, Quantisierung, Flackern |
| `composite.wgsl` | 106 | Hitzeverzerrung, Beleuchtungsmodell, Emission, Nässe, Kriegsnebel, Nebel, Regen, Asche |
| `bloom.wgsl` | 63 | Prefilter (13 Taps + Soft-Knee), Downsample (13 Taps), Upsample (Tent-Filter) |
| `final.wgsl` | 57 | Skalierung mit Subpixel, Bloom-Addition, Belichtung, Tonemapping, Kontrast, Sättigung, Split-Toning, Vignette, Filmkorn, Gamma |

### 7.1 Vertex-Stufe der Sprites

```text
local = ecke · size
local.x += (1 − ecke.y) · scherung · size.y          // nur die Oberkante verschiebt sich
local.y  = size.y − (size.y − local.y) · (1 − stauchung)   // Höhe schrumpft auf (1 − stauchung)
welt    = pos + local
```

**Scherung** wird für Windwiegen (Bäume, Gras, Schilf) und für den Schattenwurf verwendet. **Stauchung** legt
Schattensprites flach auf den Boden. Beides wirkt nur in der Vertex-Stufe; der Atlas bleibt unverändert.

### 7.2 Lichtpass (`light.wgsl`)

* Jedes Licht ist ein Quad mit Radius `r` in x und `0,6 · r` in y (Boden ist isometrisch auf die Hälfte gestaucht).
* Entfernung: `d = (dx, 2 · dy)`; außerhalb von `r` wird verworfen.
* Abfall: `att = (1 − |d| / r)²`.
* Normalenbeleuchtung: Richtung zum Licht `(−d.x, −d.y · 0,5, Lichthöhe)` (mit bereits verdoppeltem `d.y`); `wrap = 0,35 + 0,65 · max(n · l, 0)`
  (Wrap-Beleuchtung, damit abgewandte Flächen nicht ganz schwarz werden). Ohne Normalmapping `diffuse = 1`.
* **Quantisierung:** `q = round(att · wrap · 8) / 8` – 8 harte Lichtstufen.
* Flackern: `1 − flicker · (0,5 + 0,5 · sin(13 t + φ) · sin(7,3 t + 2,1 φ))`.
* Ergebnis: `farbe.rgb · intensität · q · flackern`, additiv in HDR (`Rgba16Float`). Lichtstandardhöhe 24 px
  (`Light::new`); Lichter lesen nur die Normale des G-Buffers.

### 7.3 Komposition (`composite.wgsl`)

1. **Hitze:** `heat = g-albedo.a`; bei `heat > 0,01` wird die UV um `sin/cos`-Wellen (Ortsfrequenz 0,9/0,7, Zeitfrequenz
   9/7) verschoben.
2. **Unbeleuchtet:** `flags ≥ 0,5` → Albedo wird sofort zurückgegeben (Unlit/Solid).
3. **Beleuchtung:**
   ```text
   shadow   = clamp(flags / 0,49, 0, 1)
   sun_term = sun.rgb · sun.intensität · (0,45 + 0,55 · max(n · sonne, 0)) · (1 − shadow · 0,85)
   amb      = ambient.rgb · ambient.intensität · (1 − shadow · 0,35)
   farbe    = albedo · (amb + sun_term + lichter + blitz · 0,9) + albedo · emission · 3,2
   ```
4. **Nässe:** `farbe · (1 − nässe · 0,18)`.
5. **Kriegsnebel** (siehe Abschnitt 10).
6. **Atmosphärischer Nebel** (siehe Abschnitt 9.3).
7. **Regen / Asche** (siehe Abschnitt 9.4).

### 7.4 Bloom (`bloom.wgsl`)

* **Prefilter:** 13-Tap-Filter der `scene`-Textur, dann Soft-Knee: `soft = clamp(br − thr + knee, 0, 2·knee)² / (4·knee)`,
  `Beitrag = max(soft, br − thr) / br` mit `br = max(r, g, b)`. Standard: Schwelle 0,85, Knee 0,5.
* **Downsample:** 13-Tap-Filter je Stufe; Stufe 0 hat die halbe interne Auflösung, jede weitere wieder die Hälfte.
* **Upsample:** Tent-Filter (3×3, Gewichte 4-2-1) und **additives** Blending in die nächstgrößere Stufe.
* Die Ausgabe verwendet nur die größte Stufe (`bloom0`, linear gefiltert) mit Intensität 0,65 (Standard).
* Stufenzahl je Profil: 0 (aus) / 3 / 4.

### 7.5 Ausgabe (`final.wgsl`)

```text
ip   = ausgabepixel / skalierung + subpixel            // Subpixel-Scrolling
farbe = scene(ip)  [+ bloom · intensität]
farbe *= belichtung
farbe  = tonemap(farbe)                                // linear bis 0,6, darüber weiche Schulter 1 − exp(−x)
farbe  = (farbe − 0,5) · kontrast + 0,5
farbe  = mix(luma, farbe, sättigung)
farbe += tönung · (1 − luma) · 0,12                    // Split-Toning: Schatten zur Tönung
farbe *= 1 − |uv − 0,5|² · vignette · 1,6
farbe += (hash(pixel, frame) − 0,5) · korn             // Filmkorn, nur wenn Profil-Korn > 0
ausgabe = sRGB-Ziel ? linear : to_srgb(farbe)
```

Das Surface-Format wird bevorzugt **ohne** sRGB gewählt (`!format.is_srgb()`), der Shader kodiert Gamma selbst
(`to_srgb`); egui nutzt dasselbe Format. Im Headless-Betrieb ist das Ziel `Rgba8Unorm`.

---

## 8. Schattenwurf nach Sonnenstand

Schatten sind **keine** Echtzeit-Schatten, sondern Sprite-Silhouetten, die nach dem Sonnenstand geschert und gestaucht
werden (`WorldView::atmosphere`):

```text
shear  = clamp(−sun_dir.x · 1,1 / max(sun_dir.z, 0,25), −2,2, 2,2)
squash = 0,55 + 0,2 · (1 − low_sun)
```

* `sun_dir.x` bei Tag: `−cos((t − 0,25) · 2π) · 0,85` → morgens (t = 0,25) −0,85 (Licht von links, Schatten fallen
  nach rechts), mittags 0 (Schatten fallen senkrecht nach unten), abends +0,85 (Schatten fallen nach links). Nachts
  ist die Richtung fest (`0,55`, Mond).
* `low_sun = 1 − smooth(0, 0,5, |Sonnenhöhe|)`: bei tiefstehender Sonne sind Schatten länger (kleinere Stauchung).
* Die Schattenstärke (`shadow_strength`) beträgt bei Tag `0,55 + 0,35 · day`, nachts 0,35; Bewölkung, Nebel und
  Regen reduzieren sie (siehe Abschnitt 9).
* Schatten werfen: Bäume (inkl. Windschwingung), Gebäude (fertig), hohe Dekoration (Findlinge, tote Bäume, Ruinen,
  Säulen, Feuerschalen, Laternen, Gräber). Einheiten haben stattdessen weiche, nicht geschwenkte Blob-Schatten.
* Der Schatten reduziert im Komposit Sonne um bis zu 85 % und Ambient um bis zu 35 % – Punktlichter und Emission
  bleiben vom Schatten unberührt.

---

## 9. Tag, Nacht, Wetter und Atmosphäre

### 9.1 Tageszeit

`Environment::time_of_day` läuft von 0 bis 1 (`Rules::day_length` = 360 s je Zyklus; Start bei 0,30). Phasen:
Morgengrauen 0,21–0,30, Tag 0,30–0,72, Abenddämmerung 0,72–0,80, sonst Nacht. Die Sonnenhöhe ist
`sin((t − 0,25) · 2π)`.

`WorldView::atmosphere` leitet daraus ab (`day = smooth(−0,15; 0,35; Sonnenhöhe)`):

| Größe | Formel / Werte |
|---|---|
| Ambient-Farbe | Nacht `(0,36; 0,44; 0,78)` → Tag `(0,56; 0,58; 0,66)`, bei tiefer Sonne zusätzlich Richtung warm (morgens `(0,72; 0,52; 0,55)`, abends `(0,66; 0,42; 0,52)`) |
| Ambient-Intensität | `0,44 + 0,16 · day` |
| Sonnenfarbe | Tag `(1,0; 0,93; 0,82)`, tief morgens `(1,0; 0,62; 0,34)`, tief abends `(1,0; 0,45; 0,28)`, Mond `(0,55; 0,66; 0,95)` |
| Sonnenintensität | Tag `0,35 + 0,65 · day`, Nacht 0,32 |
| Schattenstärke | Tag `0,55 + 0,35 · day`, Nacht 0,35 |
| Lichter-Intensität (Gebäude) | `0,8 + 0,9 · Nacht` – Fensterlichter, Lagerfeuer und Laternen leuchten nachts stärker |
| Standard-Grading | Belichtung 1,3, Kontrast 1,08, Sättigung 0,92 (zur Hohlnähe überschrieben), Vignette 0,35, Tönung `(0,05; 0,02; 0,10)` (kühle Schatten) |

### 9.2 Wetter

Sechs Wetterlagen (`Weather`): `Clear`, `Overcast`, `Fog`, `Rain`, `Storm`, `Ashfall`. Wechsel zufällig alle 70–180 s
(Wahrscheinlichkeiten 36 % / 20 % / 14 % / 14 % / 7 % / 9 %), Übergang über 8 s (`weather_intensity`).

| Wetter | Wirkung auf die Atmosphäre | Simulationswirkung |
|---|---|---|
| Klar | – | – |
| Bedeckt | Sonne −45 %, Ambient +25 %, Schatten −50 % | – |
| Nebel | Nebeldichte +0,9, Sonne −40 %, Schatten halbiert | Sichtweite des Spielers × 0,75 |
| Regen | Regen 0,7, Nässe, Nebel +0,3, Sonne −55 %, Schatten −55 % | Fernkampf-Faktor 0,9 (`rain_ranged_mult`) |
| Sturm | Wie Regen mit Stärke 1,0; Blitze alle 5–17 s | wie Regen |
| Aschenfall | Aschepartikel, Nebel +0,35 (aschgrau), Sonne −30 % | – |

Zusätzlich: Im Morgengrauen liegt Nebel (+0,45) und Bodennebel (0,3). Mission-Aktion `SetWeather` und die
Kommandozeile (`--weather`) können das Wetter erzwingen.

### 9.3 Nebel

Im Komposit werden zwei Rauschfelder (`fbm2`) mit Wind (`atmosphere.wind`) verschoben:

```text
dichte = smoothstep(0,25; 0,85; f1·0,65 + f2·0,5) · nebeldichte · 0,75 + bodennebel · 0,18   (max. 0,72)
nebelfarbe = fog_color · (ambient · 0,75 + sonne · 0,35) + lichter · lichtstreuung
q = floor(dichte · 16 + bayer · 0,6) / 16            // 16 Stufen, Bayer-Dither
farbe = mix(farbe, nebelfarbe, q)
```

Punktlichter färben den Nebel (Lichtstreuung): Lagerfeuer und Fenster erzeugen sichtbare Lichthöfe im Dunst.

### 9.4 Regen, Asche, Blitz, Nässe

* **Regen:** Diagonale Schlieren im Bildschirmraum (Zellen 5×13 px, Geschwindigkeit 160 px/s, Dichte 9 % × Intensität),
  zusätzlich 12 Spritzer-Partikel alle 0,12 s im Sichtbereich.
* **Asche/Schnee:** Treibende Flocken (7-px-Zellen, Dichte 5 %), seitlich pendelnd.
* **Blitz:** `Lightning`-Ereignis setzt `lightning = 1`, klingt mit 3 pro Sekunde ab; im Komposit wird
  `blitz · 0,9` zum Umgebungslicht addiert (der ganze Bildschirm flackert auf). Der Donner kommt über das Audio-Event
  `sfx.thunder`.
* **Nässe:** Dunkelt Materialien um bis zu 18 % ab.

### 9.5 Verderbnis nahe dem Hohlschrein

`WorldView::atmosphere` berechnet `hollow = 1 − smooth(8, 26, Abstand der Bildmitte zu ENEMY_BASE)`:

* Sättigung `0,95 − 0,3 · hollow`, Kontrast `1,08 + 0,06 · hollow`.
* Tönung wechselt von `(0,05; 0,02; 0,10)` (kühles Violett) zu `(−0,02; 0,07; 0,03)` (fahles Grün).

So wird beim Kameraflug zum Hohlschrein die Welt sichtbar kränklicher – ohne Gameplay-Eingriff, rein über die Position.
Die Position `ENEMY_BASE` ist derzeit eine Konstante in `ashen_sim::mapgen` (siehe Roadmap Phase 7: datengetriebene Verderbnis).

---

## 10. Kriegsnebel

* Simulation: `FogOfWar` hält pro Kachel `0` unerforscht, `1` erforscht, `2` sichtbar; Aktualisierung mit 5 Hz aus den
  Sichtweiten aller eigenen Einheiten und Gebäude (nachts × 0,7, bei Nebelwetter × 0,75).
* Upload: Eine `R8Unorm`-Textur in Kartengröße (72×72) mit den Werten 0 / 128 / 255; nur bei neuer `revision`.
* Shader (`composite.wgsl`): Bildpixel → Kachelkoordinate → Textur (linear gefiltert → weiche Ränder).
  * `explored = smoothstep(0,15; 0,5; v)`, `visible = smoothstep(0,55; 0,95; v)`.
  * **Erforscht, nicht sichtbar:** `mix(grau, farbe, 0,35) · 0,6` mit kühl getöntem Grau (`luma · (0,75; 0,8; 0,95)`).
  * **Unerforscht:** fast schwarz `(0,012; 0,012; 0,02)` mit leisem, langsamem Rauschen (`fbm2`).
* Darstellung im Spiel: Gegner nur wenn sichtbar; Gebäude nur wenn erforscht; Bauplätze nur auf erforschtem Gelände.
* Im Sandbox-Modus und im Hauptmenü ist der Nebel aus (`FogOfWar::disabled`).

---

## 11. Bloom und Tonemapping

| Aspekt | Wert |
|---|---|
| Bloom-Quelle | HDR-`scene` (`Rgba16Float`) |
| Schwelle / Knie / Intensität | 0,85 / 0,5 / 0,65 (Standard der `Atmosphere`) |
| Stufen | Leistung 0, Ausgewogen 3, Qualität 4 |
| Speisende Quellen | Emission (`albedo · emission · 3,2`), stark beleuchtete Flächen, Lichter über der Schwelle |
| Tonemapping | Linear bis 0,6; darüber `0,6 + 0,4 · (1 − exp(−(x − 0,6) / 0,4))` je Kanal (maximal 1,0) |
| Gamma | sRGB-Kodierung im Shader, falls das Ziel nicht sRGB ist |

---

## 12. Grafikprofile

Aus `render/profile.rs` (Zielhardware: Apple A18 Pro, 5-Kern-GPU, 8 GB Unified Memory):

| Einstellung | Leistung | Ausgewogen (Standard) | Qualität |
|---|---|---|---|
| Bloom-Stufen | 0 | 3 | 4 |
| Max. Lichter | 48 | 128 | 256 |
| Max. Partikel | 600 | 1 600 | 3 200 |
| Normalmapping | aus | an | an |
| Atmosphärischer Nebel | an | an | an |
| Filmkorn | 0,0 | 0,006 | 0,009 |
| Mindest-Bildzeilen (Zoom-Grenze) | 300 | 260 | 220 |

Profil wählen: Einstellungen im Spiel, `--profile <performance|balanced|quality>` oder `settings.ron`. Der Kommentar in
`profile.rs` nennt für „Leistung“ eine „halbe Lichtauflösung“; sie ist im Renderer **nicht** implementiert
(der Lichtpuffer hat in allen Profilen die interne Auflösung).

---

## 13. Hot-Reload der Shader

Mit der Umgebungsvariable `ASHEN_SHADER_DIR` (Verzeichnis mit den sechs `.wgsl`-Dateien) lädt der
`ShaderManager` die Quellen aus dem Dateisystem statt aus der Binärdatei:

```text
ASHEN_SHADER_DIR=crates/ashen_engine/src/render/shaders cargo run --release
```

* Alle 0,5 s werden die Änderungszeitpunkte geprüft (`common.wgsl` plus fünf Shader).
* Bei Änderung werden **alle** Pipelines neu erstellt (`create_pipelines`); Fehler werden per Error-Scope abgefangen,
  protokolliert, und die alte Version bleibt aktiv.
* Fehlende Dateien im Verzeichnis fallen auf die eingebetteten Quellen zurück.
* Ein Fehler beim **ersten** Start verhindert den Start des Renderers (`Err` aus `Renderer::new`).

---

## 14. Gezielte visuelle Momente und wie sie entstehen

Die Welt soll auf Spielereignisse visuell reagieren. Diese Momente entstehen aus **Gameplay-Ereignissen**
(`SimEvent`, verarbeitet in `WorldView::on_event`) oder aus Zuständen (`WorldView::render`):

| Moment | Auslöser im Gameplay | Visuelle Umsetzung |
|---|---|---|
| Die erste Nacht senkt sich | Tageszeit erreicht Nachtphase (`NightBegins`) | Ambient wechselt zu Blau, Sonne zu Mondlicht (Intensität 0,32), Gebäudelichter steigen auf `0,8 + 0,9`; Gebäude-, Feuerschalen- und Laternenlicht dominieren das Bild |
| Der Horizont brennt | Dämmerung (t ≈ 0,72–0,80) | Tiefstehende Sonne in Orange/Rot, lange Schatten, warmer Ambient-Anteil |
| Nebel im Morgengrauen | Phase `Dawn` | Nebeldichte +0,45, Bodennebel; Lichthöfe um Lichtquellen |
| Gewitter über Brennhollow | Wetter `Storm`, `Lightning`-Ereignis | Ganzbildblitz (Ambient +0,9, klingt ab), Regenschlieren, nasse Materialien, Donner |
| Der Hohlschrein pulsiert | Gegnerisches Gebäude mit Hohlglut | Grünes Pulslicht (Radius 60, Intensität 0,6–1,0), aufsteigende grüne Funken; Kamera nahe `ENEMY_BASE` → Entsättigung und grünliche Tönung |
| Der Fahle Vogt | Boss-Einheit sichtbar | Grünes, flackerndes Bosslicht (Radius 60, Höhe 30), größerer Lebensbalken |
| Hohle sterben | `UnitDied` (Besitzer Gegner) | Grüner Glühpartikel-Burst (`fx_glow`, 10 Stück, emissiv), Leiche mit Todesanimation, ausblendend nach 16–22 s |
| Eigene Einheiten fallen | `UnitDied` (Spieler) | Staubwolke, Leiche |
| Treffer | `Hit` | Weißer Trefferblitz auf der Einheit, 3 Funken (orange bei Blockierung, grünlich bei Magie) |
| Seelenblitz schlägt ein | `ProjectileImpact` (`SoulBolt`) | Grüner Burst plus Lichtblitz (Radius 40, 0,3 s) |
| Glutspalter | `AbilityCast` `ember_cleave` | Ring aus 36 Glutpartikeln, Funkenexplosion, orangefarbener Lichtblitz (Radius 110) |
| Sammelbanner | `AbilityCast` `rallying_banner` | Goldener Funkenregen (30), Lichtblitz Radius 140 (1,0 s); Verbündete erhalten Aufwärtsfunken |
| Aschenschild | `AbilityCast` `ashen_ward` | Aschrauch, Lichtblitz; leuchtender Ring um die Einheit, solange das Schild hält |
| Heldin steigt auf | `LevelUp` | Goldener Burst (26) und Lichtblitz (Radius 80) |
| Heldin kehrt zurück | `HeroRespawned` | Warmer Lichtblitz am Bergfried (Radius 90, 1,2 s) |
| Gebäude brennt | Schaden > 45 % (> 75 % → drei Feuer) | Flammensprites (`fx_flame_0..3`), Rauch, Hitzeflimmern, Flammenlicht (Radius 70) |
| Gebäude stürzt ein | `BuildingDestroyed` | Rauchwolke (24), Glutfunken (16), großer orangefarbener Lichtblitz (Radius 120, 0,8 s) |
| Bau schreitet voran | Baustelle mit `progress` | Fundament am Boden, Gerüst, Gebäude-Sprite von unten nach oben freigelegt (`crop_top`) |
| Bau fertig | `BuildingCompleted` | Staubburst |
| Bäume werden gefällt | `NodeDepleted` (Baum) | Blattpartikel, Baumstumpf bleibt (bis 400 Stümpfe) |
| Arbeit | `WorkStroke` | Holzsplitter, Goldfunken (emissiv), Steinstaub |
| Goldmine glimmt | Mine im Sichtfeld | Flackerndes Goldlicht, nachts stärker |
| Welle rückt an | `WaveIncoming` (Vorwarnung 20 s vorher, dann Spawn) | Hinweis im HUD, Horn-Audio; Hohle erscheinen im Nebel erst bei Sichtkontakt |
| Reiterei galoppiert | Ritter im Zustand `Walk` | Staubwölkchen unter den Hufen |
| Wind | Dauerzustand | Bäume, Gras, Schilf und Büsche schwingen per Scherung |

Alle Effekte sind rein visuell und beeinflussen die Simulation nicht; sie unterliegen den Partikel- und Lichtbudgets
des gewählten Profils.

---

## 15. Nachweis per Screenshot

Mit den Headless-Optionen (`--screenshot`, `--time`, `--weather`, `--battle`, …; siehe
[`../tech/project_structure.md`](../tech/project_structure.md)) lassen sich alle genannten Zustände reproduzierbar
aufnehmen, z. B. Tag/Nacht/Nebel/Sturm, Kampf vor der Basis, Hohlschrein. Im Verzeichnis `screenshots/` liegen
Beispielaufnahmen (`test_day.png`, `test_night.png`, `test_mission.png`, `test_shrine.png`, `test_menu.png`,
`dbg_fog.png`).
