//! Audio-Backends: kira (macOS CoreAudio / Windows WASAPI / Linux ALSA) oder stumm.

use std::path::Path;

/// Mischpult-Busse.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Bus {
    Music,
    Sfx,
    Voice,
    Ambience,
    Ui,
}

impl Bus {
    pub const ALL: [Bus; 5] = [Bus::Music, Bus::Sfx, Bus::Voice, Bus::Ambience, Bus::Ui];
}

pub type HandleId = u64;

/// Abstraktion über die Audioausgabe, damit Logik ohne Gerät testbar bleibt.
pub trait AudioBackend {
    /// Kurzer Klang, vollständig in den Speicher geladen.
    fn play_oneshot(&mut self, path: &Path, bus: Bus, volume_db: f32, pan: f32) -> Option<HandleId>;
    /// Gestreamter Klang (Musik, Ambience), optional geloopt, mit Einblendung.
    fn play_stream(&mut self, path: &Path, bus: Bus, looped: bool, volume_db: f32, fade_in: f32) -> Option<HandleId>;
    fn set_volume(&mut self, id: HandleId, volume_db: f32, fade: f32);
    fn stop(&mut self, id: HandleId, fade: f32);
    fn is_playing(&self, id: HandleId) -> bool;
    fn set_bus_volume(&mut self, bus: Bus, volume_db: f32);
    fn name(&self) -> &'static str;
}

/// Stummes Backend: protokolliert Aufrufe (Tests, Systeme ohne Audiogerät).
#[derive(Default)]
pub struct NullBackend {
    next: HandleId,
    pub log: Vec<String>,
    playing: Vec<(HandleId, f32)>,
    clock: f32,
}

impl NullBackend {
    /// Simuliert Zeitablauf: One-Shots gelten nach 1 s als beendet.
    pub fn advance(&mut self, dt: f32) {
        self.clock += dt;
        let now = self.clock;
        self.playing.retain(|(_, end)| *end > now);
    }
}

impl AudioBackend for NullBackend {
    fn play_oneshot(&mut self, path: &Path, bus: Bus, volume_db: f32, _pan: f32) -> Option<HandleId> {
        self.next += 1;
        self.log.push(format!("oneshot {:?} {} {volume_db:.1}", bus, path.display()));
        self.playing.push((self.next, self.clock + 1.0));
        Some(self.next)
    }
    fn play_stream(&mut self, path: &Path, bus: Bus, looped: bool, volume_db: f32, _fade_in: f32) -> Option<HandleId> {
        self.next += 1;
        self.log.push(format!("stream {:?} {} loop={looped} {volume_db:.1}", bus, path.display()));
        self.playing.push((self.next, if looped { f32::MAX } else { self.clock + 30.0 }));
        Some(self.next)
    }
    fn set_volume(&mut self, id: HandleId, volume_db: f32, _fade: f32) {
        self.log.push(format!("volume {id} {volume_db:.1}"));
    }
    fn stop(&mut self, id: HandleId, _fade: f32) {
        self.log.push(format!("stop {id}"));
        self.playing.retain(|(h, _)| *h != id);
    }
    fn is_playing(&self, id: HandleId) -> bool {
        self.playing.iter().any(|(h, _)| *h == id)
    }
    fn set_bus_volume(&mut self, bus: Bus, volume_db: f32) {
        self.log.push(format!("bus {bus:?} {volume_db:.1}"));
    }
    fn name(&self) -> &'static str {
        "stumm"
    }
}

#[cfg(feature = "audio")]
pub use kira_backend::KiraBackend;

#[cfg(feature = "audio")]
mod kira_backend {
    use super::*;
    use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
    use kira::sound::streaming::{StreamingSoundData, StreamingSoundHandle};
    use kira::sound::{FromFileError, PlaybackState};
    use kira::track::{TrackBuilder, TrackHandle};
    use kira::{AudioManager, AudioManagerSettings, Decibels, DefaultBackend, Panning, Tween};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::time::Duration;

    enum Handle {
        Static(StaticSoundHandle),
        Stream(StreamingSoundHandle<FromFileError>),
    }

    pub struct KiraBackend {
        _manager: AudioManager<DefaultBackend>,
        tracks: HashMap<Bus, TrackHandle>,
        cache: HashMap<PathBuf, StaticSoundData>,
        handles: HashMap<HandleId, Handle>,
        next: HandleId,
    }

    fn tween(secs: f32) -> Tween {
        Tween { duration: Duration::from_secs_f32(secs.max(0.0)), ..Default::default() }
    }

    impl KiraBackend {
        pub fn new() -> Result<KiraBackend, String> {
            let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).map_err(|e| e.to_string())?;
            let mut tracks = HashMap::new();
            for b in Bus::ALL {
                let t = manager.add_sub_track(TrackBuilder::new()).map_err(|e| e.to_string())?;
                tracks.insert(b, t);
            }
            Ok(KiraBackend { _manager: manager, tracks, cache: HashMap::new(), handles: HashMap::new(), next: 0 })
        }

        fn gc(&mut self) {
            if self.handles.len() > 256 {
                self.handles.retain(|_, h| match h {
                    Handle::Static(s) => s.state() != PlaybackState::Stopped,
                    Handle::Stream(s) => s.state() != PlaybackState::Stopped,
                });
            }
        }
    }

    impl AudioBackend for KiraBackend {
        fn play_oneshot(&mut self, path: &Path, bus: Bus, volume_db: f32, pan: f32) -> Option<HandleId> {
            self.gc();
            let data = match self.cache.get(path) {
                Some(d) => d.clone(),
                None => match StaticSoundData::from_file(path) {
                    Ok(d) => {
                        self.cache.insert(path.to_path_buf(), d.clone());
                        d
                    }
                    Err(e) => {
                        log::warn!("Audio {} nicht ladbar: {e}", path.display());
                        return None;
                    }
                },
            };
            let data = data.volume(Decibels(volume_db)).panning(Panning(pan.clamp(-1.0, 1.0)));
            let track = self.tracks.get_mut(&bus)?;
            match track.play(data) {
                Ok(h) => {
                    self.next += 1;
                    self.handles.insert(self.next, Handle::Static(h));
                    Some(self.next)
                }
                Err(e) => {
                    log::warn!("Wiedergabe fehlgeschlagen: {e}");
                    None
                }
            }
        }

        fn play_stream(&mut self, path: &Path, bus: Bus, looped: bool, volume_db: f32, fade_in: f32) -> Option<HandleId> {
            self.gc();
            let data = match StreamingSoundData::from_file(path) {
                Ok(d) => d,
                Err(e) => {
                    log::warn!("Stream {} nicht ladbar: {e}", path.display());
                    return None;
                }
            };
            let data = if looped { data.loop_region(..) } else { data };
            let data = data.volume(Decibels(if fade_in > 0.0 { Decibels::SILENCE.0 } else { volume_db }));
            let track = self.tracks.get_mut(&bus)?;
            match track.play(data) {
                Ok(mut h) => {
                    if fade_in > 0.0 {
                        h.set_volume(Decibels(volume_db), tween(fade_in));
                    }
                    self.next += 1;
                    self.handles.insert(self.next, Handle::Stream(h));
                    Some(self.next)
                }
                Err(e) => {
                    log::warn!("Stream-Wiedergabe fehlgeschlagen: {e}");
                    None
                }
            }
        }

        fn set_volume(&mut self, id: HandleId, volume_db: f32, fade: f32) {
            match self.handles.get_mut(&id) {
                Some(Handle::Static(h)) => h.set_volume(Decibels(volume_db), tween(fade)),
                Some(Handle::Stream(h)) => h.set_volume(Decibels(volume_db), tween(fade)),
                None => {}
            }
        }

        fn stop(&mut self, id: HandleId, fade: f32) {
            match self.handles.get_mut(&id) {
                Some(Handle::Static(h)) => h.stop(tween(fade)),
                Some(Handle::Stream(h)) => h.stop(tween(fade)),
                None => {}
            }
        }

        fn is_playing(&self, id: HandleId) -> bool {
            match self.handles.get(&id) {
                Some(Handle::Static(h)) => !matches!(h.state(), PlaybackState::Stopped | PlaybackState::Stopping),
                Some(Handle::Stream(h)) => !matches!(h.state(), PlaybackState::Stopped | PlaybackState::Stopping),
                None => false,
            }
        }

        fn set_bus_volume(&mut self, bus: Bus, volume_db: f32) {
            if let Some(t) = self.tracks.get_mut(&bus) {
                t.set_volume(Decibels(volume_db), tween(0.05));
            }
        }

        fn name(&self) -> &'static str {
            "kira"
        }
    }
}
