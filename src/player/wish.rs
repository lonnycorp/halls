use glam::{Mat3, Vec3};

use super::state::PlayerState;
use crate::config::Config;
use crate::config::ConfigControl;
use crate::window::{InputController, KeyState};

fn key_down(input: &InputController<'_>, config: &Config, control: ConfigControl) -> bool {
    return matches!(
        input.key(config.keycode_get(control)),
        KeyState::Pressed | KeyState::Down
    );
}

pub fn player_wish_update(state: &mut PlayerState, input: &InputController<'_>, config: &Config) {
    let mut dir = Vec3::ZERO;

    if key_down(input, config, ConfigControl::Forward) {
        dir.z -= 1.0;
    }
    if key_down(input, config, ConfigControl::Back) {
        dir.z += 1.0;
    }
    if key_down(input, config, ConfigControl::StrafeRight) {
        dir.x += 1.0;
    }
    if key_down(input, config, ConfigControl::StrafeLeft) {
        dir.x -= 1.0;
    }

    if dir != Vec3::ZERO {
        state.wish_direction = Mat3::from_rotation_y(state.rotation.y) * dir.normalize();
    } else {
        state.wish_direction = Vec3::ZERO;
    }

    state.wish_jumping = key_down(input, config, ConfigControl::Jump);
    state.wish_crouching = key_down(input, config, ConfigControl::Crouch);
}
