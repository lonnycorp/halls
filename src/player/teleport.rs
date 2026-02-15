use parry3d::math::{Isometry, Vector};

use crate::level::cache::{LevelCache, LevelCacheResult};

use super::constant::EPSILON;
use super::state::PlayerState;

pub fn player_try_teleport(state: &mut PlayerState, cache: &mut LevelCache) -> bool {
    let level_url = state.level_url.as_ref().unwrap();
    let LevelCacheResult::Ready(level) = cache.get(level_url) else {
        return false;
    };
    let start_pos = state.prev_position;
    for (name, src_portal) in level.portals() {
        let Some(link) = src_portal.link(cache) else {
            continue;
        };

        let src_geometry = src_portal.geometry();
        let src_normal = src_geometry.normal();
        let start_side = (start_pos - src_geometry.center()).dot(src_normal);
        let end_side = (state.position - src_geometry.center()).dot(src_normal);
        let crossed = start_side * end_side <= 0.0 && (start_side - end_side).abs() > EPSILON;

        if !crossed {
            continue;
        }

        let move_delta = state.position - start_pos;
        let shape_pos = Isometry::translation(start_pos.x, start_pos.y, start_pos.z);
        let shape_vel = Vector::new(move_delta.x, move_delta.y, move_delta.z);
        let in_contact = src_portal
            .sweep(&shape_pos, &shape_vel, &state.collider, 1.0)
            .is_some();

        if in_contact {
            let yaw_delta = link.yaw_delta();
            state.last_portal = Some((level.url().clone(), name.clone()));
            state.open_factor = -state.open_factor;
            state.position = link.position_transform(state.position, true);
            state.velocity = link.velocity_transform(state.velocity);
            state.rotation.y += yaw_delta;
            state.level_url = Some(link.url().clone());

            return true;
        }
    }
    return false;
}
