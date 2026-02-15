use glam::Vec3;
use parry3d::math::Isometry;
use parry3d::query::PointQuery;

use crate::level::cache::LevelCache;

use super::constant::{EPSILON, GROUND_NORMAL_Y_MIN};
use super::state::{PlayerMovementMode, PlayerState};
use super::sweep::player_sweep;
use crate::SIM_STEP;

const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();
const MAX_ITERATIONS: usize = 4;
const STOP_EPSILON: f32 = 0.1;
const SKIN_THICKNESS: f32 = 0.01;
const GROUND_SNAP_DISTANCE: f32 = 0.1;
const STEP_HEIGHT: f32 = 0.5;

fn clip_velocity(velocity: Vec3, planes: &[Vec3]) -> Vec3 {
    // Try clipping against each plane, validating against all others
    for (i, &plane) in planes.iter().enumerate() {
        let backoff = velocity.dot(plane);
        let mut clipped = velocity - plane * backoff;

        // Clamp tiny values to zero (prevents drift)
        if clipped.x.abs() < STOP_EPSILON {
            clipped.x = 0.0;
        }
        if clipped.y.abs() < STOP_EPSILON {
            clipped.y = 0.0;
        }
        if clipped.z.abs() < STOP_EPSILON {
            clipped.z = 0.0;
        }

        // Check if valid against all OTHER planes
        let valid = planes
            .iter()
            .enumerate()
            .all(|(j, &p)| i == j || clipped.dot(p) >= 0.0);

        if valid {
            return clipped;
        }
    }

    // No single-plane clip worked - try 2-plane crease
    if planes.len() == 2 {
        let dir = planes[0].cross(planes[1]).normalize_or_zero();
        return dir * dir.dot(velocity);
    }

    return Vec3::ZERO;
}

fn slide_move(state: &mut PlayerState, cache: &mut LevelCache) {
    let primal_vel = state.velocity;
    let mut remaining_time = SIM_STEP_SECS;
    let mut planes: Vec<Vec3> = Vec::new();

    for _ in 0..MAX_ITERATIONS {
        if remaining_time < EPSILON {
            break;
        }

        if let Some(hit) = player_sweep(state, cache, state.velocity, remaining_time) {
            let speed = state.velocity.length();
            let epsilon_time = if speed > 0.0 {
                SKIN_THICKNESS / speed
            } else {
                0.0
            };
            let safe_time = (hit.time - epsilon_time).max(0.0);

            state.position += state.velocity * safe_time;
            remaining_time -= safe_time;

            let shape_pos =
                Isometry::translation(state.position.x, state.position.y, state.position.z);
            let hit_point = parry3d::math::Point::new(hit.point.x, hit.point.y, hit.point.z);
            let dist = state
                .collider
                .distance_to_point(&shape_pos, &hit_point, true);
            if dist < EPSILON {
                state.position += hit.normal * EPSILON;
            }

            planes.push(hit.normal);
            state.velocity = clip_velocity(primal_vel, &planes);

            if state.velocity.dot(primal_vel) <= 0.0 {
                state.velocity = Vec3::ZERO;
                break;
            }
            continue;
        }

        state.position += state.velocity * remaining_time;
        break;
    }
}

fn ground_snap(state: &mut PlayerState, cache: &mut LevelCache) {
    let hit = player_sweep(state, cache, Vec3::NEG_Y, GROUND_SNAP_DISTANCE);
    match hit {
        Some(hit) if hit.normal.y >= GROUND_NORMAL_Y_MIN => {
            let snap_dist = (hit.time - SKIN_THICKNESS).max(0.0);
            state.position.y -= snap_dist;
        }
        _ => {}
    }
}

pub fn player_position_update(state: &mut PlayerState, cache: &mut LevelCache) {
    state.prev_position = state.position;

    let movement_mode = state.movement_mode;

    let step_enabled = movement_mode != PlayerMovementMode::Airborne;
    let snap_enabled = movement_mode == PlayerMovementMode::Grounded;

    let original_vel = state.velocity;

    // Normal slide move
    slide_move(state, cache);

    // Step-up attempt when grounded or on ladder
    if step_enabled {
        let normal_pos = state.position;
        let normal_vel = state.velocity;

        // Reset to start and sweep up to check ceiling clearance
        state.position = state.prev_position;
        let up_hit = player_sweep(state, cache, glam::Vec3::Y, STEP_HEIGHT);
        let step_up = match up_hit {
            Some(hit) => (hit.time - SKIN_THICKNESS).max(0.0),
            None => STEP_HEIGHT,
        };

        // Raise position and re-run slide move with original velocity
        state.position = state.prev_position;
        state.position.y += step_up;
        state.velocity = original_vel;
        slide_move(state, cache);

        // Sweep down to find ground
        let down_dist = step_up + GROUND_SNAP_DISTANCE;
        let down_hit = player_sweep(state, cache, glam::Vec3::NEG_Y, down_dist);
        match down_hit {
            Some(hit) if hit.normal.y >= GROUND_NORMAL_Y_MIN => {
                let snap_dist = (hit.time - SKIN_THICKNESS).max(0.0);
                state.position.y -= snap_dist;
            }
            _ => {
                // No ground found — revert to normal pass
                state.position = normal_pos;
                state.velocity = normal_vel;
            }
        }

        // Compare horizontal distance: keep whichever went further
        let normal_horiz = glam::Vec3::new(
            normal_pos.x - state.prev_position.x,
            0.0,
            normal_pos.z - state.prev_position.z,
        );
        let stepped_horiz = glam::Vec3::new(
            state.position.x - state.prev_position.x,
            0.0,
            state.position.z - state.prev_position.z,
        );
        let normal_dist_sq = normal_horiz.length_squared();
        let stepped_dist_sq = stepped_horiz.length_squared();
        if normal_dist_sq >= stepped_dist_sq {
            state.position = normal_pos;
            state.velocity = normal_vel;
        }
    }

    if snap_enabled {
        ground_snap(state, cache);
    }
}
