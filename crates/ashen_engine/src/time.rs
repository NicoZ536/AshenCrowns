//! Zeitmessung: Frame-Zeit, geglättete Statistik und fester Simulationstakt.

use std::time::Instant;

pub struct FrameTimer {
    last: Instant,
    pub dt: f32,
    pub elapsed: f64,
    pub frame: u64,
    history: [f32; 120],
    idx: usize,
}

impl Default for FrameTimer {
    fn default() -> Self {
        FrameTimer { last: Instant::now(), dt: 1.0 / 60.0, elapsed: 0.0, frame: 0, history: [1.0 / 60.0; 120], idx: 0 }
    }
}

impl FrameTimer {
    /// Neuer Frame; Frame-Zeit wird auf 100 ms begrenzt (Debugger, Fensterverschiebung).
    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        self.dt = (now - self.last).as_secs_f32().min(0.1);
        self.last = now;
        self.elapsed += self.dt as f64;
        self.frame += 1;
        self.history[self.idx] = self.dt;
        self.idx = (self.idx + 1) % self.history.len();
        self.dt
    }

    /// Feste Zeit vorgeben (Headless-Rendering).
    pub fn tick_fixed(&mut self, dt: f32) -> f32 {
        self.dt = dt;
        self.elapsed += dt as f64;
        self.frame += 1;
        dt
    }

    pub fn avg_ms(&self) -> f32 {
        self.history.iter().sum::<f32>() / self.history.len() as f32 * 1000.0
    }

    pub fn max_ms(&self) -> f32 {
        self.history.iter().copied().fold(0.0, f32::max) * 1000.0
    }

    pub fn fps(&self) -> f32 {
        1000.0 / self.avg_ms().max(0.001)
    }
}

/// Akkumulator für einen festen Simulationstakt mit Interpolationsfaktor.
#[derive(Clone, Copy, Debug)]
pub struct FixedStep {
    pub step: f32,
    acc: f32,
    pub max_steps: u32,
}

impl FixedStep {
    pub fn new(hz: u32) -> FixedStep {
        FixedStep { step: 1.0 / hz as f32, acc: 0.0, max_steps: 8 }
    }

    /// Anzahl der in diesem Frame auszuführenden Schritte.
    pub fn advance(&mut self, dt: f32) -> u32 {
        self.acc += dt;
        let mut n = 0;
        while self.acc >= self.step && n < self.max_steps {
            self.acc -= self.step;
            n += 1;
        }
        if n == self.max_steps {
            self.acc = 0.0;
        }
        n
    }

    /// Interpolationsfaktor 0..1 zwischen letztem und nächstem Schritt.
    pub fn alpha(&self) -> f32 {
        (self.acc / self.step).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_step_accumulates() {
        let mut f = FixedStep::new(20);
        assert_eq!(f.advance(0.04), 0);
        assert_eq!(f.advance(0.02), 1);
        assert!(f.alpha() > 0.15 && f.alpha() < 0.25);
        assert_eq!(f.advance(10.0), 8, "spiral of death capped");
    }
}
