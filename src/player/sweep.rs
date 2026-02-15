use glam::{Mat3, Vec3};
use parry3d::math::{Isometry, Vector};

use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::level::{LevelHit, SurfaceKind};

use super::constant::EPSILON;
use super::state::PlayerState;

pub struct PlayerSweepHit {
    pub time: f32,
    pub normal: Vec3,
    pub point: Vec3,
    pub surface_kind: SurfaceKind,
}

pub fn player_sweep(
    state: &PlayerState,
    cache: &mut LevelCache,
    velocity: Vec3,
    max_toi: f32,
) -> Option<PlayerSweepHit> {
    let level_url = state.level_url.as_ref().unwrap();
    let LevelCacheResult::Ready(level) = cache.get(level_url) else {
        return None;
    };
    let shape_pos = Isometry::translation(state.position.x, state.position.y, state.position.z);
    let shape_vel = Vector::new(velocity.x, velocity.y, velocity.z);

    let mut best_hit: Option<PlayerSweepHit> = level
        .sweep(&shape_pos, &shape_vel, &state.collider, max_toi)
        .map(|r: LevelHit| {
            let surface_kind = r.kind;
            let hit = r.hit;
            return PlayerSweepHit {
                time: hit.time_of_impact,
                normal: Vec3::new(hit.normal2.x, hit.normal2.y, hit.normal2.z),
                point: Vec3::new(hit.witness2.x, hit.witness2.y, hit.witness2.z),
                surface_kind,
            };
        });

    for (name, src_portal) in level.portals() {
        let Some(portal_hit) = src_portal
            .sweep(&shape_pos, &shape_vel, &state.collider, max_toi)
            .map(|r| PlayerSweepHit {
                time: r.time_of_impact,
                normal: Vec3::new(r.normal2.x, r.normal2.y, r.normal2.z),
                point: Vec3::new(r.witness2.x, r.witness2.y, r.witness2.z),
                surface_kind: SurfaceKind::Wall,
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

        let LevelCacheResult::Ready(dst_level) = cache.get(link.url()) else {
            panic!("linked level not ready")
        };

        // Validate destination portal links back to this level + source portal
        let Some(dst_portal) = dst_level.portal(link.name()) else {
            continue;
        };
        let mut dst_back_url = dst_portal.link_url().clone();
        let dst_back_fragment = dst_back_url.fragment().map(str::to_string);
        dst_back_url.set_fragment(None);
        if dst_back_url != *level_url || dst_back_fragment.as_deref() != Some(name.as_str()) {
            match &best_hit {
                Some(best) if best.time <= portal_hit.time => {}
                _ => best_hit = Some(portal_hit),
            }
            continue;
        }

        let transformed_pos = link.position_transform(state.position, true);
        let transformed_vel = link.velocity_transform(velocity);

        let shape_pos =
            Isometry::translation(transformed_pos.x, transformed_pos.y, transformed_pos.z);
        let shape_vel = Vector::new(transformed_vel.x, transformed_vel.y, transformed_vel.z);
        let Some(result) = dst_level.sweep(&shape_pos, &shape_vel, &state.collider, max_toi) else {
            continue;
        };
        let result_hit = result.hit;

        // t=0 collision at destination - treat portal as solid
        if result_hit.time_of_impact < EPSILON {
            match &best_hit {
                Some(best) if best.time <= portal_hit.time => {}
                _ => best_hit = Some(portal_hit),
            }
            continue;
        }

        // Normal through-portal hit - rotate collision info back to source frame
        let rot_back = Mat3::from_rotation_y(-link.yaw_delta());
        let through_portal_hit = PlayerSweepHit {
            time: result_hit.time_of_impact,
            normal: rot_back
                * Vec3::new(
                    result_hit.normal2.x,
                    result_hit.normal2.y,
                    result_hit.normal2.z,
                ),
            point: src_portal.geometry().center()
                + rot_back
                    * (Vec3::new(
                        result_hit.witness2.x,
                        result_hit.witness2.y,
                        result_hit.witness2.z,
                    ) - link.dst_center()),
            surface_kind: SurfaceKind::Wall,
        };

        match &best_hit {
            Some(best) if best.time <= through_portal_hit.time => {}
            _ => best_hit = Some(through_portal_hit),
        }
    }

    return best_hit;
}
