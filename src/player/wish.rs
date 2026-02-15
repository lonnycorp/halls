use glam::{Mat3, Vec3};

use super::state::PlayerState;
use crate::config::Config;
use crate::config::ConfigControl;
use crate::window::{WindowContext, WindowKeyState};

fn key_down(window: &WindowContext<'_>, config: &Config, control: ConfigControl) -> bool {
    return matches!(
        window.key(config.key_get(control)),
        WindowKeyState::Pressed | WindowKeyState::Down
    );
}

pub fn player_wish_update(state: &mut PlayerState, window: &WindowContext<'_>, config: &Config) {
    let mut dir = Vec3::ZERO;

    if key_down(window, config, ConfigControl::Forward) {
        dir.z -= 1.0;
    }
    if key_down(window, config, ConfigControl::Back) {
        dir.z += 1.0;
    }
    if key_down(window, config, ConfigControl::StrafeRight) {
        dir.x += 1.0;
    }
    if key_down(window, config, ConfigControl::StrafeLeft) {
        dir.x -= 1.0;
    }

    if dir != Vec3::ZERO {
        state.wish_direction = Mat3::from_rotation_y(state.rotation.y) * dir.normalize();
    } else {
        state.wish_direction = Vec3::ZERO;
    }

    state.wish_jumping = key_down(window, config, ConfigControl::Jump);
    state.wish_crouching = key_down(window, config, ConfigControl::Crouch);
}
