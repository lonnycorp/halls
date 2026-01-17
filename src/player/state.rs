use glam::{Vec2, Vec3};
use parry3d::math::Vector;
use parry3d::shape::Cuboid;
use url::Url;

pub const WIDTH: f32 = 0.6;
pub const HEIGHT: f32 = 1.4;
pub const CROUCH_HEIGHT: f32 = 0.7;

pub struct PlayerState {
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
    pub grounded: bool,
}

impl PlayerState {
    pub fn new(position: Vec3) -> Self {
        return Self {
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
            grounded: false,
        };
    }
}
