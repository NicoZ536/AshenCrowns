//! Farbpalette der Art Direction: gedämpft, düster, mit warmen Glut- und kalten Hohl-Akzenten.

use ashen_core::color::{Ramp, Rgba};

pub fn grass() -> Ramp {
    Ramp::from_hex(&[0x1a2214, 0x232d19, 0x2c3a1e, 0x364724, 0x41532a, 0x4e6031])
}
pub fn dark_grass() -> Ramp {
    Ramp::from_hex(&[0x141c12, 0x1a2617, 0x22311c, 0x2b3d22, 0x354a28])
}
pub fn dirt() -> Ramp {
    Ramp::from_hex(&[0x221e18, 0x2d2820, 0x393228, 0x463d31, 0x54493b])
}
pub fn stone() -> Ramp {
    Ramp::from_hex(&[0x1a1a1d, 0x26262b, 0x34343a, 0x45454b, 0x5a5a60, 0x75757a])
}
pub fn rock() -> Ramp {
    Ramp::from_hex(&[0x1d1c1a, 0x2a2926, 0x3a3833, 0x4c4a43, 0x625f56, 0x7c786d])
}
pub fn sand() -> Ramp {
    Ramp::from_hex(&[0x342c21, 0x463b2c, 0x5a4d39, 0x6f6048, 0x857456])
}
pub fn ash() -> Ramp {
    Ramp::from_hex(&[0x232127, 0x2d2a31, 0x38343c, 0x444048, 0x524c56])
}
pub fn wood() -> Ramp {
    Ramp::from_hex(&[0x1e130d, 0x2e1d13, 0x41291a, 0x573823, 0x6e4a2e])
}
pub fn dark_wood() -> Ramp {
    Ramp::from_hex(&[0x140d09, 0x1f140e, 0x2c1c13, 0x3a261a])
}
pub fn thatch() -> Ramp {
    Ramp::from_hex(&[0x2e2412, 0x433519, 0x5a4721, 0x725b2b, 0x8a6f36])
}
pub fn steel() -> Ramp {
    Ramp::from_hex(&[0x1e2228, 0x2f353d, 0x47505a, 0x657079, 0x8b969e, 0xb4bec4])
}
pub fn crimson() -> Ramp {
    Ramp::from_hex(&[0x2c0a0b, 0x4a1012, 0x6c1717, 0x8f2220, 0xab3329])
}
pub fn gold() -> Ramp {
    Ramp::from_hex(&[0x4a3210, 0x7a5418, 0xa77a26, 0xcfa23f, 0xeccb6c])
}
pub fn skin() -> Ramp {
    Ramp::from_hex(&[0x3d241b, 0x63402f, 0x8a5d44, 0xad7c5c, 0xc99c79])
}
pub fn cloth_brown() -> Ramp {
    Ramp::from_hex(&[0x221911, 0x34261a, 0x483625, 0x5d4630, 0x72583c])
}
pub fn bone() -> Ramp {
    Ramp::from_hex(&[0x2e2c24, 0x4a4739, 0x6b6752, 0x8f8a6e, 0xb5ae8d, 0xd2cbaa])
}
pub fn bark() -> Ramp {
    Ramp::from_hex(&[0x150f0b, 0x221811, 0x302218, 0x3f2e21])
}
pub fn water_tex() -> Ramp {
    Ramp::from_hex(&[0x3c4448, 0x454f53, 0x50595e, 0x5c666b])
}

pub const EMBER: Rgba = Rgba::hex(0xf08a2c);
pub const EMBER_HOT: Rgba = Rgba::hex(0xffd36a);
pub const FIRE_DEEP: Rgba = Rgba::hex(0xb3361a);
pub const HOLLOW_GLOW: Rgba = Rgba::hex(0x86f2a0);
pub const OUTLINE: f32 = 0.42;
