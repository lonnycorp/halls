use glam::{Vec2, Vec3};
use url::Url;

use super::integrate::player_integrate;
use super::rotation::player_rotation_update;
use super::state::{PlayerState, HEIGHT};
use super::wish::player_wish_update;
use crate::config::Config;
use crate::level::cache::LevelCache;
use crate::window::InputController;

pub struct Player {
    state: PlayerState,
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        return Self {
            state: PlayerState::new(position),
        };
    }

    pub fn update(&mut self, input: &InputController<'_>, cache: &mut LevelCache, config: &Config) {
        player_rotation_update(&mut self.state, input, config);
        player_wish_update(&mut self.state, input, config);
        player_integrate(&mut self.state, cache);
        self.state.open_factor = (self.state.open_factor + 0.05).min(1.0);
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
        return self.state.grounded && self.state.wish_direction != Vec3::ZERO;
    }

    pub fn eye_position(&self) -> Vec3 {
        return self.state.position + Vec3::Y * HEIGHT / 2.0;
    }
}
