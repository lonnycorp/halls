use glam::{Mat3, Vec3};
use parry3d::math::{Isometry, Vector};
use parry3d::query::PointQuery;
use parry3d::shape::Cuboid;

use crate::level::cache::{LevelCache, LevelCacheResult};

use super::state::{PlayerState, CROUCH_HEIGHT, HEIGHT, WIDTH};
use crate::SIM_STEP;

const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();
const MAX_ITERATIONS: usize = 4;
const STOP_EPSILON: f32 = 0.1;
const EPSILON: f32 = 0.001;
const SKIN_THICKNESS: f32 = 0.01;
const SPEED: f32 = 8.0;
const CROUCH_SPEED: f32 = 4.0;
const GRAVITY: f32 = 20.0;
const GROUND_CHECK_DISTANCE: f32 = 0.04;
const GROUND_SNAP_DISTANCE: f32 = 0.1;
const GROUND_NORMAL_Y_MIN: f32 = 0.7;
const STEP_HEIGHT: f32 = 0.5;
const JUMP_SPEED: f32 = 7.0;

struct SweepHit {
    time: f32,
    normal: Vec3,
    point: Vec3,
}

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

    Vec3::ZERO
}

fn player_sweep(
    state: &PlayerState,
    cache: &mut LevelCache,
    velocity: Vec3,
    max_toi: f32,
) -> Option<SweepHit> {
    let level_url = state.level_url.as_ref().unwrap();
    let LevelCacheResult::Ready(level) = cache.get(level_url) else {
        return None;
    };
    let shape_pos = Isometry::translation(state.position.x, state.position.y, state.position.z);
    let shape_vel = Vector::new(velocity.x, velocity.y, velocity.z);

    let mut best_hit: Option<SweepHit> = level
        .sweep(&shape_pos, &shape_vel, &state.collider, max_toi)
        .map(|r| SweepHit {
            time: r.time_of_impact,
            normal: Vec3::new(r.normal2.x, r.normal2.y, r.normal2.z),
            point: Vec3::new(r.witness2.x, r.witness2.y, r.witness2.z),
        });

    for (_, src_portal) in level.portals() {
        let Some(portal_hit) = src_portal
            .sweep(&shape_pos, &shape_vel, &state.collider, max_toi)
            .map(|r| SweepHit {
                time: r.time_of_impact,
                normal: Vec3::new(r.normal2.x, r.normal2.y, r.normal2.z),
                point: Vec3::new(r.witness2.x, r.witness2.y, r.witness2.z),
            })
        else {
            continue;
        };

        // Get linked level and destination portal
        let Some(link) = src_portal.link(cache) else {
            // Destination level not loaded or incompatible - treat as solid
            match &best_hit {
                Some(best) if best.time <= portal_hit.time => {}
                _ => best_hit = Some(portal_hit),
            }
            continue;
        };

        let LevelCacheResult::Ready(dst_level) = cache.get(&link.url) else {
            panic!("linked level not ready")
        };

        // Validate destination portal links back to this level + source portal
        let Some(dst_portal) = dst_level.portal(&link.portal_name) else {
            continue;
        };
        let mut dst_back_url = dst_portal.link.clone();
        let dst_back_fragment = dst_back_url.fragment().map(str::to_string);
        dst_back_url.set_fragment(None);
        if dst_back_url != *level_url || dst_back_fragment.as_ref() != Some(&src_portal.name) {
            match &best_hit {
                Some(best) if best.time <= portal_hit.time => {}
                _ => best_hit = Some(portal_hit),
            }
            continue;
        }

        let transformed_pos = link.transform_position(state.position, true);
        let transformed_vel = link.transform_velocity(velocity);

        let shape_pos =
            Isometry::translation(transformed_pos.x, transformed_pos.y, transformed_pos.z);
        let shape_vel = Vector::new(transformed_vel.x, transformed_vel.y, transformed_vel.z);
        let Some(result) = dst_level.sweep(&shape_pos, &shape_vel, &state.collider, max_toi) else {
            continue;
        };

        // t=0 collision at destination - treat portal as solid
        if result.time_of_impact < EPSILON {
            match &best_hit {
                Some(best) if best.time <= portal_hit.time => {}
                _ => best_hit = Some(portal_hit),
            }
            continue;
        }

        // Normal through-portal hit - rotate collision info back to source frame
        let rot_back = Mat3::from_rotation_y(-link.yaw_delta());
        let through_portal_hit = SweepHit {
            time: result.time_of_impact,
            normal: rot_back * Vec3::new(result.normal2.x, result.normal2.y, result.normal2.z),
            point: src_portal.center
                + rot_back
                    * (Vec3::new(result.witness2.x, result.witness2.y, result.witness2.z)
                        - link.dst_center),
        };

        match &best_hit {
            Some(best) if best.time <= through_portal_hit.time => {}
            _ => best_hit = Some(through_portal_hit),
        }
    }

    return best_hit;
}

fn try_teleport(state: &mut PlayerState, cache: &mut LevelCache, start_pos: Vec3) -> bool {
    let level_url = state.level_url.as_ref().unwrap();
    let LevelCacheResult::Ready(level) = cache.get(level_url) else {
        return false;
    };
    for (name, src_portal) in level.portals() {
        let Some(link) = src_portal.link(cache) else {
            continue;
        };

        let src_normal = src_portal.normal();
        let start_side = (start_pos - src_portal.center).dot(src_normal);
        let end_side = (state.position - src_portal.center).dot(src_normal);

        // Crossed from one side to the other and player fully contained within portal bounds
        let offset = state.position - src_portal.center;
        let (right, up) = src_portal.local_axes();
        let half = &state.collider.half_extents;
        let player_half = Vec3::new(half.x, half.y, half.z);
        let contained = offset.dot(right).abs() + right.abs().dot(player_half)
            <= src_portal.dimensions.x / 2.0
            && offset.dot(up).abs() + up.abs().dot(player_half) <= src_portal.dimensions.y / 2.0;

        if start_side.signum() != end_side.signum() && contained {
            let yaw_delta = link.yaw_delta();
            state.last_portal = Some((level.url().clone(), name.clone()));
            state.open_factor = -state.open_factor;
            state.position = link.transform_position(state.position, true);
            state.velocity = link.transform_velocity(state.velocity);
            state.rotation.y += yaw_delta;
            state.level_url = Some(link.url.clone());

            return true;
        }
    }
    return false;
}

fn check_grounded(state: &PlayerState, cache: &mut LevelCache) -> bool {
    let hit = player_sweep(state, cache, Vec3::NEG_Y, GROUND_CHECK_DISTANCE);
    return match hit {
        Some(hit) => hit.normal.y >= GROUND_NORMAL_Y_MIN,
        None => false,
    };
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

pub fn player_integrate(state: &mut PlayerState, cache: &mut LevelCache) {
    if state.level_url.is_none() {
        return;
    }

    let old_height = if state.crouching {
        CROUCH_HEIGHT
    } else {
        HEIGHT
    };

    if state.wish_crouching {
        state.crouching = true;
    } else if state.crouching {
        let clear = player_sweep(state, cache, Vec3::Y, HEIGHT - CROUCH_HEIGHT).is_none();
        if clear {
            state.crouching = false;
        }
    }

    let new_height = if state.crouching {
        CROUCH_HEIGHT
    } else {
        HEIGHT
    };
    state.position.y += (new_height - old_height) / 2.0;
    state.collider = Cuboid::new(Vector::new(WIDTH / 2.0, new_height / 2.0, WIDTH / 2.0));

    let speed = if state.crouching { CROUCH_SPEED } else { SPEED };
    state.velocity.x = state.wish_direction.x * speed;
    state.velocity.z = state.wish_direction.z * speed;

    let mut grounded = check_grounded(state, cache);
    state.grounded = grounded;
    if grounded {
        if state.wish_jumping {
            state.velocity.y = JUMP_SPEED;
            grounded = false;
        } else {
            state.velocity.y = 0.0;
        }
    } else {
        state.velocity.y -= GRAVITY * SIM_STEP_SECS;
    }

    let start_position = state.position;
    let original_vel = state.velocity;

    // Normal slide move
    slide_move(state, cache);

    // Step-up attempt when grounded
    if grounded {
        let normal_pos = state.position;
        let normal_vel = state.velocity;

        // Reset to start and sweep up to check ceiling clearance
        state.position = start_position;
        let up_hit = player_sweep(state, cache, Vec3::Y, STEP_HEIGHT);
        let step_up = match up_hit {
            Some(hit) => (hit.time - SKIN_THICKNESS).max(0.0),
            None => STEP_HEIGHT,
        };

        // Raise position and re-run slide move with original velocity
        state.position = start_position;
        state.position.y += step_up;
        state.velocity = original_vel;
        slide_move(state, cache);

        // Sweep down to find ground
        let down_dist = step_up + GROUND_SNAP_DISTANCE;
        let down_hit = player_sweep(state, cache, Vec3::NEG_Y, down_dist);
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
        let normal_horiz = Vec3::new(
            normal_pos.x - start_position.x,
            0.0,
            normal_pos.z - start_position.z,
        );
        let stepped_horiz = Vec3::new(
            state.position.x - start_position.x,
            0.0,
            state.position.z - start_position.z,
        );
        let normal_dist_sq = normal_horiz.length_squared();
        let stepped_dist_sq = stepped_horiz.length_squared();
        if normal_dist_sq >= stepped_dist_sq {
            state.position = normal_pos;
            state.velocity = normal_vel;
        }
    }

    if grounded {
        ground_snap(state, cache);
    }

    // Check for portal crossing
    try_teleport(state, cache, start_position);
}
