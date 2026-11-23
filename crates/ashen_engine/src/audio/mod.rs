//! Audiosystem: Manifest, Asset-Scanner, Event-Registry, Musiksteuerung mit Crossfades,
//! Ambience-Schichten, Dialogsequenzen mit Ducking und Lautstärkeregelung.

pub mod backend;
pub mod manifest;
pub mod scanner;

pub use backend::{AudioBackend, Bus, HandleId, NullBackend};
pub use manifest::{AudioManifest, Category, Intensity, Playback};
pub use scanner::ScanReport;

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};

/// Lautstärken (linear 0..1).
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Volumes {
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
    pub voice: f32,
    pub ambience: f32,
    pub ui: f32,
}

impl Default for Volumes {
    fn default() -> Self {
        Volumes { master: 0.85, music: 0.7, sfx: 0.85, voice: 1.0, ambience: 0.6, ui: 0.7 }
    }
}

fn lin_to_db(v: f32) -> f32 {
    if v <= 0.001 { -60.0 } else { (20.0 * v.log10()).max(-60.0) }
}

const CROSSFADE: f32 = 2.5;

struct MusicPlayer {
    current: Option<(String, HandleId)>,
    /// Laufendes Intro, nach dessen Ende der Loop startet.
    intro: Option<(String, HandleId, PathBuf)>,
    intensity: Intensity,
    pending_down: Option<(Intensity, f32)>,
    duck_db: f32,
    sting: Option<HandleId>,
    /// Bevorzugte Stücke (z. B. das Boss-Thema der Mission); gilt je Intensität vor dem Standardstück.
    preferred: Vec<String>,
}

struct SeqItem {
    path: PathBuf,
    volume_db: f32,
}

pub struct AudioSystem {
    backend: Box<dyn AudioBackend>,
    pub manifest: AudioManifest,
    pub report: ScanReport,
    volumes: Volumes,
    time: f32,
    rng: u32,
    last_play: HashMap<String, f32>,
    last_variant: HashMap<String, usize>,
    active: HashMap<String, Vec<HandleId>>,
    music: MusicPlayer,
    ambience: HashMap<String, HandleId>,
    sequence: VecDeque<SeqItem>,
    seq_handle: Option<HandleId>,
    /// Event-IDs, die angefragt wurden, aber keine Dateien haben (einmalige Logmeldung).
    warned: std::collections::HashSet<String>,
}

impl AudioSystem {
    /// Erstellt das System: lädt das Manifest, scannt Dateien und wählt das Backend.
    pub fn new(audio_root: Option<&Path>, enable_output: bool) -> AudioSystem {
        let (manifest, report) = match audio_root {
            Some(root) => {
                let manifest = AudioManifest::load(&root.join("manifest.ron")).unwrap_or_else(|e| {
                    log::warn!("Audio-Manifest nicht geladen: {e}");
                    AudioManifest::default()
                });
                let report = scanner::scan(root, &manifest);
                (manifest, report)
            }
            None => (AudioManifest::default(), ScanReport::default()),
        };
        log::info!("Audio: {} Dateien zugeordnet, {} Muster ohne Datei", report.found_count(), report.missing.len());
        let backend: Box<dyn AudioBackend> = if enable_output { Self::default_backend() } else { Box::new(NullBackend::default()) };
        Self::with_backend(backend, manifest, report)
    }

    #[cfg(feature = "audio")]
    fn default_backend() -> Box<dyn AudioBackend> {
        match backend::KiraBackend::new() {
            Ok(b) => Box::new(b),
            Err(e) => {
                log::warn!("Kein Audiogerät verfügbar ({e}) – Audio stumm");
                Box::new(NullBackend::default())
            }
        }
    }

    #[cfg(not(feature = "audio"))]
    fn default_backend() -> Box<dyn AudioBackend> {
        Box::new(NullBackend::default())
    }

    pub fn with_backend(backend: Box<dyn AudioBackend>, manifest: AudioManifest, report: ScanReport) -> AudioSystem {
        let mut s = AudioSystem {
            backend,
            manifest,
            report,
            volumes: Volumes::default(),
            time: 0.0,
            rng: 0x1234_5678,
            last_play: HashMap::new(),
            last_variant: HashMap::new(),
            active: HashMap::new(),
            music: MusicPlayer { current: None, intro: None, intensity: Intensity::Menu, pending_down: None, duck_db: 0.0, sting: None, preferred: Vec::new() },
            ambience: HashMap::new(),
            sequence: VecDeque::new(),
            seq_handle: None,
            warned: Default::default(),
        };
        s.apply_volumes();
        s
    }

    pub fn backend_name(&self) -> &'static str {
        self.backend.name()
    }

    pub fn volumes(&self) -> Volumes {
        self.volumes
    }

    pub fn set_volumes(&mut self, v: Volumes) {
        self.volumes = v;
        self.apply_volumes();
    }

    fn apply_volumes(&mut self) {
        let v = self.volumes;
        for (bus, lv) in [(Bus::Music, v.music), (Bus::Sfx, v.sfx), (Bus::Voice, v.voice), (Bus::Ambience, v.ambience), (Bus::Ui, v.ui)] {
            self.backend.set_bus_volume(bus, lin_to_db(lv * v.master));
        }
    }

    fn rand(&mut self) -> u32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        self.rng
    }

    /// Spielt ein Event. `pan` −1..1, `attenuation` 0..1 (Entfernung zur Kamera).
    pub fn play_at(&mut self, event_id: &str, pan: f32, attenuation: f32) -> bool {
        let Some(ev) = self.manifest.event(event_id).cloned() else {
            if self.warned.insert(event_id.to_string()) {
                log::debug!("Audio-Event '{event_id}' nicht im Manifest");
            }
            return false;
        };
        if let Some(&t) = self.last_play.get(event_id)
            && (self.time - t) * 1000.0 < ev.cooldown_ms as f32
        {
            return false;
        }
        let active = self.active.entry(event_id.to_string()).or_default();
        let backend = &self.backend;
        active.retain(|h| backend.is_playing(*h));
        if ev.max_instances > 0 && active.len() as u32 >= ev.max_instances {
            return false;
        }
        let files: Vec<PathBuf> = ev.files.iter().flat_map(|p| self.report.files_for(p).to_vec()).collect();
        if files.is_empty() {
            if self.warned.insert(event_id.to_string()) {
                log::debug!("Audio-Event '{event_id}': keine Dateien vorhanden");
            }
            return false;
        }
        let bus = match ev.category {
            Category::Voice | Category::Narration | Category::Dialogue => Bus::Voice,
            Category::Sfx => Bus::Sfx,
            Category::Ui => Bus::Ui,
        };
        let vol = ev.volume_db + lin_to_db(1.0 - attenuation.clamp(0.0, 0.95));
        self.last_play.insert(event_id.to_string(), self.time);
        if ev.playback == Playback::Sequential {
            for f in files {
                self.sequence.push_back(SeqItem { path: f, volume_db: vol });
            }
            return true;
        }
        let idx = if files.len() == 1 {
            0
        } else {
            let last = self.last_variant.get(event_id).copied();
            let mut i = (self.rand() as usize) % files.len();
            if Some(i) == last {
                i = (i + 1) % files.len();
            }
            i
        };
        self.last_variant.insert(event_id.to_string(), idx);
        let narration = matches!(ev.category, Category::Narration | Category::Dialogue);
        if narration {
            self.sequence.push_back(SeqItem { path: files[idx].clone(), volume_db: vol });
            return true;
        }
        if let Some(h) = self.backend.play_oneshot(&files[idx], bus, vol, pan) {
            self.active.entry(event_id.to_string()).or_default().push(h);
            true
        } else {
            false
        }
    }

    pub fn play(&mut self, event_id: &str) -> bool {
        self.play_at(event_id, 0.0, 0.0)
    }

    /// Bevorzugte Musikstücke der laufenden Mission (Track-IDs aus dem Manifest); leer = Standard.
    pub fn set_preferred_music(&mut self, ids: Vec<String>) {
        if self.music.preferred == ids {
            return;
        }
        self.music.preferred = ids;
        // Läuft schon Musik, wechselt ein bevorzugtes Stück der aktuellen Intensität sofort ein.
        if self.music.current.is_some() {
            let intensity = self.music.intensity;
            self.switch_music(intensity);
        }
    }

    /// Gewünschte Musikintensität. Steigerungen wirken sofort, Absenkungen nach 8 s Ruhe.
    pub fn set_music_intensity(&mut self, intensity: Intensity) {
        if intensity == self.music.intensity && self.music.current.is_some() {
            self.music.pending_down = None;
            return;
        }
        if intensity > self.music.intensity || self.music.current.is_none() || self.music.intensity == Intensity::Menu || intensity == Intensity::Menu {
            self.music.pending_down = None;
            self.switch_music(intensity);
        } else if self.music.pending_down.is_none_or(|(i, _)| i != intensity) {
            self.music.pending_down = Some((intensity, 8.0));
        }
    }

    fn switch_music(&mut self, intensity: Intensity) {
        self.music.intensity = intensity;
        let preferred = self.manifest.music.iter().find(|m| m.intensity == intensity && self.music.preferred.contains(&m.id));
        let Some(track) = preferred.or_else(|| self.manifest.music.iter().find(|m| m.intensity == intensity)).cloned() else { return };
        if self.music.current.as_ref().is_some_and(|(id, _)| *id == track.id) {
            return;
        }
        let from_silence = self.music.current.is_none();
        if let Some((_, h)) = self.music.current.take() {
            self.backend.stop(h, CROSSFADE);
        }
        if let Some((_, h, _)) = self.music.intro.take() {
            self.backend.stop(h, CROSSFADE);
        }
        let loop_file = self.report.files_for(&track.file).first().cloned();
        let intro_file = track.intro.as_ref().and_then(|i| self.report.files_for(i).first().cloned());
        let Some(loop_file) = loop_file else { return };
        let vol = self.music.duck_db;
        if let (true, Some(intro)) = (from_silence || intensity == Intensity::Menu, intro_file)
            && let Some(h) = self.backend.play_stream(&intro, Bus::Music, false, vol, 0.0) {
                self.music.intro = Some((track.id.clone(), h, loop_file));
                self.music.current = Some((track.id, h));
                return;
            }
        if let Some(h) = self.backend.play_stream(&loop_file, Bus::Music, track.looped, vol, CROSSFADE) {
            self.music.current = Some((track.id, h));
        }
    }

    /// Kurzer musikalischer Akzent (Sieg/Niederlage); die laufende Musik wird abgesenkt.
    pub fn play_sting(&mut self, track_id: &str) {
        let Some(track) = self.manifest.track(track_id).cloned() else { return };
        let Some(file) = self.report.files_for(&track.file).first().cloned() else { return };
        if let Some((_, h)) = &self.music.current {
            self.backend.set_volume(*h, -18.0, 0.8);
        }
        self.music.sting = self.backend.play_stream(&file, Bus::Music, false, 0.0, 0.0);
    }

    pub fn stop_music(&mut self, fade: f32) {
        if let Some((_, h)) = self.music.current.take() {
            self.backend.stop(h, fade);
        }
        self.music.intro = None;
    }

    /// Setzt die aktiven Ambience-Schichten mit relativer Lautstärke (0..1). Nicht genannte blenden aus.
    pub fn set_ambience(&mut self, layers: &[(&str, f32)]) {
        let wanted: HashMap<&str, f32> = layers.iter().copied().collect();
        let ids: Vec<String> = self.ambience.keys().cloned().collect();
        for id in ids {
            if !wanted.contains_key(id.as_str())
                && let Some(h) = self.ambience.remove(&id)
            {
                self.backend.stop(h, 3.0);
            }
        }
        for (id, level) in layers {
            let Some(layer) = self.manifest.ambience.iter().find(|a| a.id == *id).cloned() else { continue };
            let db = layer.volume_db + lin_to_db(*level);
            match self.ambience.get(*id) {
                Some(h) => self.backend.set_volume(*h, db, 2.0),
                None => {
                    if let Some(f) = self.report.files_for(&layer.file).first().cloned()
                        && let Some(h) = self.backend.play_stream(&f, Bus::Ambience, true, db, 3.0)
                    {
                        self.ambience.insert(id.to_string(), h);
                    }
                }
            }
        }
    }

    pub fn voice_busy(&self) -> bool {
        self.seq_handle.is_some_and(|h| self.backend.is_playing(h)) || !self.sequence.is_empty()
    }

    /// Pro Frame aufrufen.
    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        // Intro beendet → Loop starten.
        if let Some((id, h, loop_file)) = self.music.intro.clone()
            && !self.backend.is_playing(h)
        {
            self.music.intro = None;
            let vol = self.music.duck_db;
            if let Some(nh) = self.backend.play_stream(&loop_file, Bus::Music, true, vol, 0.0) {
                self.music.current = Some((id, nh));
            }
        }
        if let Some((i, t)) = self.music.pending_down {
            let t = t - dt;
            if t <= 0.0 {
                self.music.pending_down = None;
                self.switch_music(i);
            } else {
                self.music.pending_down = Some((i, t));
            }
        }
        if let Some(s) = self.music.sting
            && !self.backend.is_playing(s)
        {
            self.music.sting = None;
            if let Some((_, h)) = &self.music.current {
                self.backend.set_volume(*h, self.music.duck_db, 2.0);
            }
        }
        // Dialogsequenzen mit Ducking der Musik.
        let playing = self.seq_handle.is_some_and(|h| self.backend.is_playing(h));
        if !playing {
            self.seq_handle = None;
            if let Some(item) = self.sequence.pop_front() {
                self.seq_handle = self.backend.play_oneshot(&item.path, Bus::Voice, item.volume_db, 0.0);
            }
        }
        let want_duck = if self.seq_handle.is_some() { -8.0 } else { 0.0 };
        if (want_duck - self.music.duck_db).abs() > 0.1 && self.music.sting.is_none() {
            self.music.duck_db = want_duck;
            if let Some((_, h)) = &self.music.current {
                self.backend.set_volume(*h, want_duck, 0.6);
            }
        }
    }

    pub fn current_track(&self) -> Option<&str> {
        self.music.current.as_ref().map(|(id, _)| id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use manifest::*;

    fn fixture() -> (AudioManifest, ScanReport) {
        let m = AudioManifest {
            version: 1,
            music: vec![
                MusicTrack { id: "calm".into(), intensity: Intensity::Calm, file: "music/calm_loop".into(), intro: None, looped: true },
                MusicTrack { id: "battle".into(), intensity: Intensity::Battle, file: "music/battle_loop".into(), intro: None, looped: true },
            ],
            events: vec![
                SoundEvent { id: "hit".into(), category: Category::Sfx, files: vec!["sfx/hit_*".into()], volume_db: 0.0, cooldown_ms: 100, max_instances: 2, playback: Playback::Random },
                SoundEvent { id: "story".into(), category: Category::Narration, files: vec!["n/story_*".into()], volume_db: 0.0, cooldown_ms: 0, max_instances: 1, playback: Playback::Sequential },
            ],
            ambience: vec![],
        };
        let mut r = ScanReport::default();
        let mut add = |p: &str, files: &[&str]| {
            r.resolved.insert(p.into(), scanner::Resolved { owner: p.into(), files: files.iter().map(PathBuf::from).collect() });
        };
        add("music/calm_loop", &["calm.ogg"]);
        add("music/battle_loop", &["battle.ogg"]);
        add("sfx/hit_*", &["hit1.ogg", "hit2.ogg"]);
        add("n/story_*", &["s1.ogg", "s2.ogg"]);
        (m, r)
    }

    #[test]
    fn cooldown_and_instances() {
        let (m, r) = fixture();
        let mut a = AudioSystem::with_backend(Box::new(NullBackend::default()), m, r);
        assert!(a.play("hit"));
        assert!(!a.play("hit"), "cooldown blocks");
        a.update(0.2);
        assert!(a.play("hit"));
        a.update(0.2);
        assert!(!a.play("hit"), "max instances");
        assert!(!a.play("unknown"));
    }

    #[test]
    fn music_crossfade_with_hysteresis() {
        let (m, r) = fixture();
        let mut a = AudioSystem::with_backend(Box::new(NullBackend::default()), m, r);
        a.set_music_intensity(Intensity::Calm);
        assert_eq!(a.current_track(), Some("calm"));
        a.set_music_intensity(Intensity::Battle);
        assert_eq!(a.current_track(), Some("battle"));
        a.set_music_intensity(Intensity::Calm);
        a.update(4.0);
        assert_eq!(a.current_track(), Some("battle"), "hold before calming down");
        a.update(5.0);
        assert_eq!(a.current_track(), Some("calm"));
    }

    #[test]
    fn sequential_narration_queues_parts() {
        let (m, r) = fixture();
        let mut a = AudioSystem::with_backend(Box::new(NullBackend::default()), m, r);
        assert!(a.play("story"));
        assert!(a.voice_busy());
        a.update(0.016);
        assert!(a.voice_busy());
    }
}
