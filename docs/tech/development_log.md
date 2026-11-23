# Entwicklungsprotokoll (kompakt)

Ziel: alle Roadmap-Phasen außer Sounddesign umsetzen; am Ende vollständige ElevenLabs- und Suno-Prompts liefern,
danach die gelieferten Audiodateien integrieren.

## Stand vor dem Lauf (Commit 52bfa69)
Vertical Slice: eigene Engine, deterministische RTS-Simulation, Mission 1, 3D→Pixel-Bake, neue UI.

## Runde 1 (parallel, Worktrees)
- S: Crate `ashen_strategy` – Grand-Campaign-Simulation (Welt, Reiche, Dynastien, Politik, Diplomatie, Wirtschaft,
  Bevölkerung, Religion, Magie/Hohlung, Ereignisse, Endgame, Chronik).
- R1: RTS-Wirtschaft/Städtebau – Waren, Produktionsketten, neue Gebäude, Wohnstufen, Mauern/Tore, Belagerung.

## Runde 2 (geplant)
- R2: RTS-Militär – neue Einheiten inkl. Belagerung/Magie, Helden-Talente/Ausrüstung, Formationen, aufbauende Gegner-KI.
- C: Kampagne Zeitalter I–VII (Missionen, Dialoge, Entscheidungen).
- E: Karteneditor, eigene/historische Szenarien, Herausforderungsmodus.
- I: Integration Weltkarte ↔ Echtzeitschlachten im Spiel (Hauptsession).

## Runde 3 (geplant)
- Audio-Manifest für alle Inhalte, ElevenLabs-/Suno-Prompts vollständig, Balancing, Performance, Doku.
