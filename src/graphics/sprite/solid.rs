use glam::Vec2;

use super::{Sprite, SpriteVertex};
use crate::graphics::sprite::SpriteMaterial;

const UV_POSITION: Vec2 = Vec2::new(32.0, 0.0);
const UV_SIZE: Vec2 = Vec2::splat(16.0);

pub struct SpriteSolid {
    position: Vec2,
    size: Vec2,
    material: SpriteMaterial,
}

impl SpriteSolid {
    pub fn new(position: Vec2, size: Vec2, material: SpriteMaterial) -> Self {
        return Self {
            position,
            size,
            material,
        };
    }

    pub fn vertices(&self) -> impl Iterator<Item = SpriteVertex> {
        return Sprite::new(
            UV_POSITION,
            UV_SIZE,
            self.material,
            self.position,
            self.size,
        )
        .vertices();
    }
}
