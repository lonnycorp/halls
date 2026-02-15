use glam::{Vec2, Vec3};
use parry3d::math::Vector;
use parry3d::shape::Cuboid;
use url::Url;

use super::constant::{HEIGHT, WIDTH};
use super::crouching::player_crouching_update;
use super::movement_mode::player_movement_mode_update;
use super::position::player_position_update;
use super::rotation::player_rotation_update;
use super::state::{PlayerMovementMode, PlayerState};
use super::teleport::player_try_teleport;
use super::velocity::player_velocity_update;
use super::wish::player_wish_update;
use crate::config::Config;
use crate::level::cache::LevelCache;
use crate::window::WindowContext;

const OPEN_FACTOR_STEP: f32 = 0.05;

pub struct Player {
    state: PlayerState,
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        return Self {
            state: PlayerState {
                prev_position: position,
                position,
                rotation: Vec2::ZERO,
                velocity: Vec3::ZERO,
                collider: Cuboid::new(Vector::new(WIDTH / 2.0, HEIGHT / 2.0, WIDTH / 2.0)),
                level_url: None,
                last_portal: None,
                open_factor: 0.0,
                wish_direction: Vec3::ZERO,
                wish_jumping: false,
                wish_crouching: false,
                crouching: false,
                prev_movement_mode: PlayerMovementMode::Airborne,
                movement_mode: PlayerMovementMode::Airborne,
            },
        };
    }

    pub fn update(&mut self, window: &WindowContext<'_>, cache: &mut LevelCache, config: &Config) {
        if self.state.level_url.is_none() {
            self.state.open_factor = (self.state.open_factor + OPEN_FACTOR_STEP).min(1.0);
            return;
        }

        player_rotation_update(&mut self.state, window, config);
        player_wish_update(&mut self.state, window, config);
        player_crouching_update(&mut self.state, cache);
        player_movement_mode_update(&mut self.state, cache);
        player_velocity_update(&mut self.state);
        player_position_update(&mut self.state, cache);
        player_try_teleport(&mut self.state, cache);
        self.state.open_factor = (self.state.open_factor + OPEN_FACTOR_STEP).min(1.0);
    }

    pub fn rotation(&self) -> Vec2 {
        return self.state.rotation;
    }

    pub fn level_url(&self) -> Option<&Url> {
        return self.state.level_url.as_ref();
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.state.position = position;
    }

    pub fn set_level_url(&mut self, url: Url) {
        self.state.level_url = Some(url);
    }

    pub fn last_portal(&self) -> Option<&(Url, String)> {
        return self.state.last_portal.as_ref();
    }

    pub fn open_factor(&self) -> f32 {
        return self.state.open_factor;
    }

    pub fn is_walking(&self) -> bool {
        return self.state.movement_mode == PlayerMovementMode::Grounded
            && self.state.wish_direction != Vec3::ZERO;
    }

    pub fn eye_position(&self) -> Vec3 {
        return self.state.position + Vec3::Y * HEIGHT / 2.0;
    }
}
