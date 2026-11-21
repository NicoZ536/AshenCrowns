//! Eingabezustand (Tastatur, Maus) pro Frame mit Flanken-Erkennung.

use glam::Vec2;
use std::collections::HashSet;
pub use winit::keyboard::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Default)]
pub struct Input {
    down: HashSet<KeyCode>,
    pressed: HashSet<KeyCode>,
    released: HashSet<KeyCode>,
    mouse_down: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,
    /// Mausposition in physischen Pixeln.
    pub mouse: Vec2,
    pub mouse_delta: Vec2,
    pub wheel: f32,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
    /// egui beansprucht Maus/Tastatur (Klick auf UI, Texteingabe).
    pub ui_wants_pointer: bool,
    pub ui_wants_keyboard: bool,
    pub focused: bool,
    pub cursor_inside: bool,
}

impl Input {
    pub fn new() -> Input {
        Input { focused: true, cursor_inside: false, ..Default::default() }
    }

    pub fn key_down(&self, k: KeyCode) -> bool {
        !self.ui_wants_keyboard && self.down.contains(&k)
    }

    pub fn key_pressed(&self, k: KeyCode) -> bool {
        !self.ui_wants_keyboard && self.pressed.contains(&k)
    }

    pub fn key_released(&self, k: KeyCode) -> bool {
        self.released.contains(&k)
    }

    pub fn mouse_down(&self, b: MouseButton) -> bool {
        self.mouse_down.contains(&b)
    }

    /// Klick in der Spielwelt (nicht auf UI).
    pub fn mouse_pressed(&self, b: MouseButton) -> bool {
        !self.ui_wants_pointer && self.mouse_pressed.contains(&b)
    }

    pub fn mouse_released(&self, b: MouseButton) -> bool {
        self.mouse_released.contains(&b)
    }

    /// Plattformübergreifender „Befehl“-Modifikator (Cmd auf macOS, Strg sonst).
    pub fn command_mod(&self) -> bool {
        if cfg!(target_os = "macos") { self.cmd } else { self.ctrl }
    }

    pub fn on_key(&mut self, k: KeyCode, pressed: bool, repeat: bool) {
        if pressed {
            if !repeat {
                self.pressed.insert(k);
            }
            self.down.insert(k);
        } else {
            self.down.remove(&k);
            self.released.insert(k);
        }
    }

    pub fn on_mouse(&mut self, b: MouseButton, pressed: bool) {
        if pressed {
            self.mouse_down.insert(b);
            self.mouse_pressed.insert(b);
        } else {
            self.mouse_down.remove(&b);
            self.mouse_released.insert(b);
        }
    }

    pub fn on_cursor(&mut self, p: Vec2) {
        self.cursor_inside = true;
        self.mouse_delta += p - self.mouse;
        self.mouse = p;
    }

    /// Am Frame-Ende aufrufen.
    pub fn end_frame(&mut self) {
        self.pressed.clear();
        self.released.clear();
        self.mouse_pressed.clear();
        self.mouse_released.clear();
        self.wheel = 0.0;
        self.mouse_delta = Vec2::ZERO;
    }

    /// Alle gehaltenen Tasten lösen (Fokusverlust).
    pub fn reset(&mut self) {
        self.down.clear();
        self.mouse_down.clear();
        self.shift = false;
        self.ctrl = false;
        self.alt = false;
        self.cmd = false;
    }
}
