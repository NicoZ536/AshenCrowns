# 3D→Pixel-Pipeline (Dead-Cells-Prinzip)

Einheiten, Gebäude, Ressourcenquellen, Bäume und größere Dekorationen von ASHEN CROWNS sind **3D-Modelle**,
die beim Spielstart von der Engine in niedriger Auflösung **ohne Kantenglättung** zu Pixel-Sprites gerastert
werden. So entsteht – wie bei Dead Cells – flüssige, plastische Animation mit der Härte handgepixelter Sprites.
Albedo, Normalen und Leuchtkraft werden getrennt gebacken; der Laufzeit-Renderer beleuchtet die Sprites danach
dynamisch (Fackeln, Zauber, Tag/Nacht), dadurch wirken sie räumlich.

Code: `crates/ashen_engine/src/bake/` (spielunabhängig) und `crates/ashen_game/src/art3d/` (Modelle, Animationen).

## 1. Ablauf

1. **Modellieren** (`art3d/rig.rs`, `art3d/units3d.rs`, `art3d/structures.rs`): Low-Poly-Meshes aus Grundkörpern
   (`Mesh::cuboid`, `frustum`, `ellipsoid`, `prism`, `quad`, `tri`) mit Vertexfarbe, Leuchtkraft und Materialkennung.
2. **Skelett & Pose** (`bake/skeleton.rs`): Starres Skinning – jeder Vertex hängt an genau einem Knochen.
   `Skeleton::world(&Pose)` liefert Weltmatrizen; Posen werden prozedural je Animation und Zeitpunkt erzeugt.
3. **Projektion** (`bake::project`): Isometrische Orthokamera, Neigung 30° (ergibt exakt das 2:1-Kachelraster),
   Drehung um die Hochachse je Blickrichtung. 1 Welteinheit = 1 Kachel = 22,63 px (32/√2). Figuren werden mit
   `UNIT_SCALE = 0.8` gebacken (leicht überhöht gegenüber Gebäuden, aber kleiner als diese).
4. **GPU-Raster** (`bake::bake`, `bake/bake.wgsl`): Alle Zellen werden seitenweise (2048×2048) mit Tiefenpuffer
   gerastert, ohne MSAA. Der Fragment-Shader erzeugt prozedurale Materialmuster und dreistufiges Cel-Shading.
5. **Nachbearbeitung** (`art3d/mod.rs`): Rückgelesene Pixel → `Canvas::from_baked`, selektive dunkle Kontur
   (`outline(0.32)`), Zuschnitt auf den Inhalt (`Canvas::trimmed`), Ablage im Atlas mit korrigiertem Anker.
6. **Laufzeit**: `ArtLibrary` stellt je Einheit und Richtung `AnimationSet`s bereit; `view.rs` wählt die Richtung
   aus `Unit::facing` (`art3d::dir_for`).

Dauer: ca. 2 s für 1330 Einheiten-Frames und 40 Bauwerke (gemessen mit Software-Vulkan/llvmpipe; auf einer
echten GPU deutlich schneller). Der Atlas wächst dadurch auf 4096×1024.

## 2. Blickrichtungen

Gebacken werden 5 Richtungen in 45°-Schritten: 0 = zum Betrachter, 1 = rechts unten, 2 = rechts, 3 = rechts oben,
4 = vom Betrachter weg. Die Richtungen 5–7 entstehen durch horizontale Spiegelung von 3, 2, 1 (die Normalen werden
dabei im Shader mitgespiegelt). Die Simulation führt dafür `Unit::facing` (Bogenmaß, 0 = zum Betrachter,
+π/2 = rechts), berechnet aus der Bewegungs- bzw. Zielrichtung im Kachelraum (`Unit::face`).

## 3. Materialien (`bake/mesh.rs` ↔ `bake.wgsl`)

| Kennung | Material | Muster |
|---|---|---|
| 0 | Plain | einfarbig |
| 1 | StoneBrick | versetzte Steinreihen mit Fugen, Tonvariation je Stein |
| 2 | WoodPlank | Bretter mit Maserung und Fugen |
| 3 | RoofTile | Ziegelreihen mit Kanten |
| 4 | Thatch | Strohfasern |
| 5 | Cloth | feines Gewebe-Rauschen |
| 6 / 17 | Metal / Gold | Glanzband (Spekularanteil), feines Rauschen |
| 7 | Skin | einfarbig |
| 8 | Leaves | Laubklumpen, Lichtverschiebung |
| 9 | Bark | Rindenstreifen |
| 10 | Bone | Knochenstruktur |
| 11 | Glow | volle Leuchtkraft (Fenster, Glut, Runen, Augen) |
| 12 | Rock | Felsrauschen mit Rissen |
| 13 | Plaster | Putz |
| 14 | Hair | Haarsträhnen |
| 15 | Crops | Feldreihen |
| 16 | Leather | Leder |

Gemusterte Koordinaten sind Knochenraum-Positionen – Texturen „schwimmen“ bei Animation daher nicht.

## 4. Rig und Animationen

Humanoides Skelett (`rig::build_skeleton`): root, pelvis, chest, head, arm/fore/hand links und rechts,
thigh/shin links und rechts, cape, weapon (an der rechten Hand), offhand (linke Hand). Konfiguration über `Look`
(Größe, Masse, Farben/Materialien, Helmtyp, Haare, Umhang, Schild, Waffe, Schulterplatten, untot, Buckel,
Beine oder Robe, Rucksack).

| Animation | Bilder | FPS | Schleife | Inhalt |
|---|---|---|---|---|
| idle | 4 | 4 | ja | Atmen, Kopf- und Umhangbewegung |
| walk | 8 | 12 | ja | Gangzyklus mit Bein-/Armschwung, Beckenhub |
| attack | 6 | 12 | nein | Hieb (ausholen–schlagen–durchziehen), Stoß (Speer/Lanze), Bogen spannen, Stab heben |
| work | 6 | 9 | ja | beidhändiges Hacken (Arbeiter) |
| carry | 8 | 12 | ja | Gehen mit Last (Arbeiter) |
| cast | 6 | 10 | nein | Arme heben (Heldin, Boss) |
| death | 6 | 10 | nein | Rückwärts fallen |

Weitere Rig-Elemente (Echtzeit-Militär): Waffen *Armbrust*, *Hellebarde*, *Feuerstab*, *Priesterstab*, *Streitkolben*, *Streitaxt*, *Krallen*, *Spitzhacke*; Helme *Spitzhut*, *Kettenhaube*,
*Nordhelm mit Hörnern*, *Mitra*; Körperform *Gown* (langes Gewand über animierten Beinen; Rock und Robe hängen an einem eigenen Knochen `skirt`, sind unten offen
(`Mesh::frustum_open`, dunkleres Futter) und sinken beim Fallen zusammen – vorher zeigte der geschlossene Saumdeckel in der Todesanimation
eine Kreisscheibe), Turmschild (`big_shield`).
Berittene (Ritter, Spähreiter, Berittene Bogenschützen) teilen das Pferdeskelett (`mounted()`, Fell- und Schabrackenfarbe als Parameter).
**Belagerungsmaschinen** (`Kind::Machine`): Körper, beweglicher Teil (Rammbaum als Pendel unter dem Dach, Wurfarm des Katapults) und vier Räder; Lauf = Raddrehung,
Angriff = Rückschwung–Stoß bzw. Spannen–Auslösen, Tod = Einsturz. **Fledermausschwarm** (`Kind::Swarm`): vier Tiere mit dunklem Körper, leuchtenden Augen und Flügeln aus Hautfächern mit gezackter Hinterkante
und Fingerknochen (helle Haut, damit sie sich vom Körper abhebt); 12° V-Grundstellung ±32° Flügelschlag, hoch über dem Anker schwebend (die
Blob-Schatten bleiben am Boden zurück, `shadow_l`). Angriff = Sturzflug, Tod = Fallen.

Sonderformen: **Ritter** = Pferdeskelett (Körper, Hals, Kopf, 4 zweigliedrige Beine, Schweif) mit eingehängtem
Reiter; Galopp mit versetzten Beinpaaren. **Grabkoloss** = humanoid mit `bulk 1.75` und Buckel. **Fahler Vogt** =
schwebende Robe (`Body::Robe`), Dornenkrone, Seelenstab.

## 5. Gebäude

Gebäude liegen im Kachelraum (Modell-X = Kachel-x, Modell-Z = Kachel-y, Ursprung = obere Ecke der Grundfläche) und
werden mit −45° Drehung gebacken, sodass sie exakt auf dem Raster stehen. Effektpunkte (Lichter, Rauch, Feuer,
Hohlglühen) werden als 3D-Punkte modelliert und beim Backen in Pixelversätze projiziert (`BuildingMeta`).
Baugerüste und Fundamente bleiben 2D-Overlays.

## 6. Werkzeuge für die Art-Iteration

```bash
# Kontaktbogen aller Einheiten (Idle, Lauf, Angriff, Tod × 5 Richtungen) als PNG
ASHEN_DUMP_SHEET=/tmp/sheet.png cargo run --release -- --screenshot /tmp/x.png --frames 2
# Gesamten Atlas als PNG exportieren
ASHEN_DUMP_ATLAS=/tmp/atlas.png cargo run --release -- --screenshot /tmp/x.png --frames 2
```

## 7. Ausbau (geplant)

- **glTF-Import** für Modelle aus Blender o. Ä. (gleiche Bake-Pipeline, Skinning mit Gewichten statt starrer Knochen).
- Paletten-Quantisierung je Fraktion für noch geschlosseneren Look.
- Weitere Animationen (Sieg, Treffer-Reaktion, Belagerung), Fraktionsvarianten, Ausrüstungswechsel sichtbar am Modell.
- Bake-Cache auf der Festplatte (nur bei geänderten Modellen neu backen).
