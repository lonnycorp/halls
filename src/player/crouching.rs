use parry3d::math::Vector;
use parry3d::shape::Cuboid;

use crate::level::cache::LevelCache;

use super::constant::{HEIGHT, WIDTH};
use super::state::PlayerState;
use super::sweep::player_sweep;

const CROUCH_HEIGHT: f32 = 0.7;

pub fn player_crouching_update(state: &mut PlayerState, cache: &mut LevelCache) {
    let old_height = if state.crouching {
        CROUCH_HEIGHT
    } else {
        HEIGHT
    };

    if state.wish_crouching {
        state.crouching = true;
    } else if state.crouching {
        let clear = player_sweep(state, cache, glam::Vec3::Y, HEIGHT - CROUCH_HEIGHT).is_none();
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
}
