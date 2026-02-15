use winit::keyboard::{Key, NamedKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::EnumIter, strum::EnumCount)]
pub enum ConfigControl {
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    Jump,
    Crouch,
}

impl ConfigControl {
    pub fn key_default(self) -> Key {
        return match self {
            ConfigControl::Forward => Key::Character("w".into()),
            ConfigControl::Back => Key::Character("s".into()),
            ConfigControl::StrafeLeft => Key::Character("a".into()),
            ConfigControl::StrafeRight => Key::Character("d".into()),
            ConfigControl::Jump => Key::Named(NamedKey::Space),
            ConfigControl::Crouch => Key::Named(NamedKey::Control),
        };
    }
}
