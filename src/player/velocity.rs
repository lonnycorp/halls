use glam::{Mat3, Vec3};

use super::state::{PlayerMovementMode, PlayerState};
use crate::SIM_STEP;

const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();
const SPEED: f32 = 8.0;
const CROUCH_SPEED: f32 = 4.0;
const GRAVITY: f32 = 20.0;
const LADDER_CLIMB_SPEED: f32 = 4.0;
const JUMP_SPEED: f32 = 7.0;

pub fn player_velocity_update(state: &mut PlayerState) {
    let forward_axis = Mat3::from_rotation_y(state.rotation.y) * Vec3::NEG_Z;
    let forward_intent = state.wish_direction.dot(forward_axis);
    let movement_mode = state.movement_mode;

    let speed = if state.crouching { CROUCH_SPEED } else { SPEED };
    state.velocity.x = state.wish_direction.x * speed;
    state.velocity.z = state.wish_direction.z * speed;

    match movement_mode {
        PlayerMovementMode::Grounded => state.velocity.y = 0.0,
        PlayerMovementMode::Airborne => {
            if !matches!(state.prev_movement_mode, PlayerMovementMode::Airborne)
                && state.wish_jumping
            {
                state.velocity.y = JUMP_SPEED;
            } else {
                state.velocity.y -= GRAVITY * SIM_STEP_SECS;
            }
        }
        PlayerMovementMode::Ladder { .. } => {
            if forward_intent > 0.0 {
                state.velocity.y = LADDER_CLIMB_SPEED;
            } else if forward_intent < 0.0 {
                state.velocity.y = -LADDER_CLIMB_SPEED;
            } else {
                state.velocity.y = 0.0;
            }
        }
    }
}
