use glam::{Vec2, Vec3};
use parry3d::shape::Cuboid;
use url::Url;

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerMovementMode {
    Grounded,
    Airborne,
    Ladder { normal: Vec3 },
}

pub struct PlayerState {
    pub prev_position: Vec3,
    pub position: Vec3,
    pub rotation: Vec2,
    pub velocity: Vec3,
    pub collider: Cuboid,
    pub level_url: Option<Url>,
    pub last_portal: Option<(Url, String)>,
    pub open_factor: f32,
    pub wish_direction: Vec3,
    pub wish_jumping: bool,
    pub wish_crouching: bool,
    pub crouching: bool,
    pub prev_movement_mode: PlayerMovementMode,
    pub movement_mode: PlayerMovementMode,
}
