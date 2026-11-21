//! Farbpalette der Art Direction: gedämpft, düster, mit warmen Glut- und kalten Hohl-Akzenten.

use ashen_core::color::{Ramp, Rgba};

pub fn grass() -> Ramp {
    Ramp::from_hex(&[0x171c13, 0x1f2619, 0x283120, 0x323c26, 0x3d462c, 0x4a5233])
}
pub fn dark_grass() -> Ramp {
    Ramp::from_hex(&[0x121610, 0x181e15, 0x1f271b, 0x283121, 0x313b27])
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
    Ramp::from_hex(&[0x181719, 0x211f22, 0x2b282b, 0x363235, 0x433e40])
}
pub fn wood() -> Ramp {
    Ramp::from_hex(&[0x1e130d, 0x2e1d13, 0x41291a, 0x573823, 0x6e4a2e])
}
pub fn dark_wood() -> Ramp {
    Ramp::from_hex(&[0x140d09, 0x1f140e, 0x2c1c13, 0x3a261a])
}
pub fn plaster() -> Ramp {
    Ramp::from_hex(&[0x4a4236, 0x5f5545, 0x756a57, 0x8a7e68, 0x9e927a])
}
pub fn roof_red() -> Ramp {
    Ramp::from_hex(&[0x2a1213, 0x3f1a19, 0x56231f, 0x6d2e27, 0x843b30])
}
pub fn thatch() -> Ramp {
    Ramp::from_hex(&[0x2e2412, 0x433519, 0x5a4721, 0x725b2b, 0x8a6f36])
}
pub fn steel() -> Ramp {
    Ramp::from_hex(&[0x1e2228, 0x2f353d, 0x47505a, 0x657079, 0x8b969e, 0xb4bec4])
}
pub fn iron_dark() -> Ramp {
    Ramp::from_hex(&[0x141518, 0x1f2125, 0x2c2f34, 0x3c4046])
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
pub fn leather() -> Ramp {
    Ramp::from_hex(&[0x1f140e, 0x332216, 0x4a3220, 0x5f432c])
}
pub fn linen() -> Ramp {
    Ramp::from_hex(&[0x3b3122, 0x56472f, 0x75633f, 0x93804f, 0xae9a63])
}
pub fn cloth_brown() -> Ramp {
    Ramp::from_hex(&[0x221911, 0x34261a, 0x483625, 0x5d4630, 0x72583c])
}
pub fn cloth_green() -> Ramp {
    Ramp::from_hex(&[0x151a12, 0x20281a, 0x2d3823, 0x3b492d])
}
pub fn bone() -> Ramp {
    Ramp::from_hex(&[0x2e2c24, 0x4a4739, 0x6b6752, 0x8f8a6e, 0xb5ae8d, 0xd2cbaa])
}
pub fn rot() -> Ramp {
    Ramp::from_hex(&[0x1a1e18, 0x283024, 0x374233, 0x485644, 0x5b6b56])
}
pub fn hollow_cloth() -> Ramp {
    Ramp::from_hex(&[0x0f1012, 0x17191c, 0x212428, 0x2c3035])
}
pub fn pine() -> Ramp {
    Ramp::from_hex(&[0x0b130f, 0x111d16, 0x18281e, 0x213526, 0x2b4430, 0x38553b])
}
pub fn oak() -> Ramp {
    Ramp::from_hex(&[0x10170b, 0x18220f, 0x223015, 0x2e3f1b, 0x3c5023, 0x4d622d])
}
pub fn bark() -> Ramp {
    Ramp::from_hex(&[0x150f0b, 0x221811, 0x302218, 0x3f2e21])
}
pub fn horse() -> Ramp {
    Ramp::from_hex(&[0x1a110c, 0x2a1c13, 0x3d2a1c, 0x523927, 0x684a33])
}
pub fn water_tex() -> Ramp {
    Ramp::from_hex(&[0x3c4448, 0x454f53, 0x50595e, 0x5c666b])
}

pub const EMBER: Rgba = Rgba::hex(0xf08a2c);
pub const EMBER_HOT: Rgba = Rgba::hex(0xffd36a);
pub const FIRE_DEEP: Rgba = Rgba::hex(0xb3361a);
pub const HOLLOW_GLOW: Rgba = Rgba::hex(0x86f2a0);
pub const HOLLOW_DEEP: Rgba = Rgba::hex(0x2f9a5a);
pub const WINDOW_WARM: Rgba = Rgba::hex(0xf2b25a);
pub const OUTLINE: f32 = 0.42;
