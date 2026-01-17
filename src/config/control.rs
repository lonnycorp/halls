use winit::keyboard::KeyCode;

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
    pub fn default_key_code(self) -> KeyCode {
        return match self {
            ConfigControl::Forward => KeyCode::KeyW,
            ConfigControl::Back => KeyCode::KeyS,
            ConfigControl::StrafeLeft => KeyCode::KeyA,
            ConfigControl::StrafeRight => KeyCode::KeyD,
            ConfigControl::Jump => KeyCode::Space,
            ConfigControl::Crouch => KeyCode::ControlLeft,
        };
    }
}
