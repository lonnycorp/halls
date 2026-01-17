use std::f32::consts::TAU;

use super::state::PlayerState;
use crate::config::Config;
use crate::window::InputController;

const PITCH_LIMIT: f32 = 1.53589;
const BASE_MOUSE_SENSITIVITY: f32 = 0.002;

pub fn player_rotation_update(
    state: &mut PlayerState,
    input: &InputController<'_>,
    config: &Config,
) {
    let sensitivity = config.mouse_sensitivity * BASE_MOUSE_SENSITIVITY;
    state.rotation.x =
        (state.rotation.x - input.mouse_delta().y * sensitivity).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    state.rotation.y = (state.rotation.y - input.mouse_delta().x * sensitivity).rem_euclid(TAU);
}
