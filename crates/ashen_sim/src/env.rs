//! Tageszeit und Wetter. Wirken auf Simulation (Sichtweite, Fernkampf) und Darstellung.

use rand::RngExt;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Overcast,
    Fog,
    Rain,
    Storm,
    Ashfall,
}

impl Weather {
    pub fn label(self) -> &'static str {
        match self {
            Weather::Clear => "Klar",
            Weather::Overcast => "Bewölkt",
            Weather::Fog => "Nebel",
            Weather::Rain => "Regen",
            Weather::Storm => "Gewitter",
            Weather::Ashfall => "Aschefall",
        }
    }

    pub fn is_rainy(self) -> bool {
        matches!(self, Weather::Rain | Weather::Storm)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayPhase {
    Dawn,
    Day,
    Dusk,
    Night,
}

impl DayPhase {
    pub fn label(self) -> &'static str {
        match self {
            DayPhase::Dawn => "Morgengrauen",
            DayPhase::Day => "Tag",
            DayPhase::Dusk => "Abenddämmerung",
            DayPhase::Night => "Nacht",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    /// Tageszeit 0..1 (0.25 Sonnenaufgang, 0.5 Mittag, 0.75 Sonnenuntergang).
    pub time_of_day: f32,
    pub day: u32,
    pub night: u32,
    pub weather: Weather,
    pub weather_timer: f32,
    /// Weicher Übergang 0..1 der Wetterintensität.
    pub weather_intensity: f32,
    pub lightning_timer: f32,
    pub forced_weather: Option<Weather>,
    pub was_night: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            time_of_day: 0.30,
            day: 1,
            night: 0,
            weather: Weather::Clear,
            weather_timer: 120.0,
            weather_intensity: 1.0,
            lightning_timer: 8.0,
            forced_weather: None,
            was_night: false,
        }
    }
}

/// Ergebnis eines Umwelt-Ticks.
#[derive(Default)]
pub struct EnvTransitions {
    pub night_began: bool,
    pub day_began: bool,
    pub weather_changed: bool,
    pub lightning: bool,
}

impl Environment {
    pub fn phase(&self) -> DayPhase {
        let t = self.time_of_day;
        if (0.21..0.30).contains(&t) {
            DayPhase::Dawn
        } else if (0.30..0.72).contains(&t) {
            DayPhase::Day
        } else if (0.72..0.80).contains(&t) {
            DayPhase::Dusk
        } else {
            DayPhase::Night
        }
    }

    pub fn is_night(&self) -> bool {
        self.phase() == DayPhase::Night
    }

    /// Sonnenhöhe −1..1 (positiv = Tag).
    pub fn sun_elevation(&self) -> f32 {
        ((self.time_of_day - 0.25) * std::f32::consts::TAU).sin()
    }

    pub fn tick(&mut self, dt: f32, day_length: f32, rng: &mut ChaCha8Rng) -> EnvTransitions {
        let mut tr = EnvTransitions::default();
        self.time_of_day += dt / day_length;
        if self.time_of_day >= 1.0 {
            self.time_of_day -= 1.0;
        }
        let night = self.is_night();
        if night && !self.was_night {
            self.night += 1;
            tr.night_began = true;
        }
        if !night && self.was_night {
            self.day += 1;
            tr.day_began = true;
        }
        self.was_night = night;

        self.weather_timer -= dt;
        if self.weather_timer <= 0.0 {
            let next = self.forced_weather.unwrap_or_else(|| {
                let r: f32 = rng.random();
                match r {
                    x if x < 0.36 => Weather::Clear,
                    x if x < 0.56 => Weather::Overcast,
                    x if x < 0.70 => Weather::Fog,
                    x if x < 0.84 => Weather::Rain,
                    x if x < 0.91 => Weather::Storm,
                    _ => Weather::Ashfall,
                }
            });
            self.weather_timer = 70.0 + rng.random::<f32>() * 110.0;
            if next != self.weather {
                self.weather = next;
                self.weather_intensity = 0.0;
                tr.weather_changed = true;
            }
        }
        self.weather_intensity = (self.weather_intensity + dt / 8.0).min(1.0);
        if self.weather == Weather::Storm {
            self.lightning_timer -= dt;
            if self.lightning_timer <= 0.0 {
                self.lightning_timer = 5.0 + rng.random::<f32>() * 12.0;
                tr.lightning = true;
            }
        }
        tr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn day_night_cycle_counts() {
        let mut e = Environment::default();
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        let mut nights = 0;
        let mut days = 0;
        for _ in 0..(360 * 20 * 2) {
            let t = e.tick(0.05, 360.0, &mut rng);
            nights += t.night_began as u32;
            days += t.day_began as u32;
        }
        assert_eq!(nights, 2);
        assert_eq!(days, 2);
        assert_eq!(e.day, 3);
    }
}
