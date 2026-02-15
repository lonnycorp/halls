use glam::{Mat3, Vec3};

use crate::level::cache::LevelCache;
use crate::level::SurfaceKind;

use super::constant::GROUND_NORMAL_Y_MIN;
use super::state::{PlayerMovementMode, PlayerState};
use super::sweep::player_sweep;

const GROUND_CHECK_DISTANCE: f32 = 0.04;
const LADDER_CONTACT_DISTANCE: f32 = 0.1;

pub fn player_movement_mode_update(state: &mut PlayerState, cache: &mut LevelCache) {
    state.prev_movement_mode = state.movement_mode;

    let forward_axis = Mat3::from_rotation_y(state.rotation.y) * Vec3::NEG_Z;
    let forward_intent = state.wish_direction.dot(forward_axis);
    let wish_ladder_hit =
        match player_sweep(state, cache, state.wish_direction, LADDER_CONTACT_DISTANCE) {
            Some(hit) if matches!(hit.surface_kind, SurfaceKind::Ladder) => Some(hit),
            _ => None,
        };

    let attach_ladder_hit = match state.movement_mode {
        PlayerMovementMode::Ladder { normal } => {
            let attach_hit = player_sweep(state, cache, -normal, LADDER_CONTACT_DISTANCE);
            match attach_hit {
                Some(hit) if matches!(hit.surface_kind, SurfaceKind::Ladder) => Some(hit),
                _ => None,
            }
        }
        _ => None,
    };

    let grounded = matches!(
        player_sweep(state, cache, Vec3::NEG_Y, GROUND_CHECK_DISTANCE),
        Some(hit) if hit.normal.y >= GROUND_NORMAL_Y_MIN
    );

    let resolved_mode = if let Some(hit) = attach_ladder_hit {
        PlayerMovementMode::Ladder { normal: hit.normal }
    } else if let Some(hit) = wish_ladder_hit {
        if forward_intent > 0.0 {
            PlayerMovementMode::Ladder { normal: hit.normal }
        } else if grounded {
            PlayerMovementMode::Grounded
        } else {
            PlayerMovementMode::Airborne
        }
    } else if grounded {
        PlayerMovementMode::Grounded
    } else {
        PlayerMovementMode::Airborne
    };

    state.movement_mode = match resolved_mode {
        PlayerMovementMode::Grounded | PlayerMovementMode::Ladder { .. } if state.wish_jumping => {
            PlayerMovementMode::Airborne
        }
        _ => resolved_mode,
    };
}
