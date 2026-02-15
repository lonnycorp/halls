use glam::Vec2;

use crate::graphics::model::ModelVertex;
use crate::graphics::sprite::{Sprite, SpriteMaterial, SpriteVertex};

const UV_POSITION: Vec2 = Vec2::new(0.0, 16.0);
const UV_SIZE: Vec2 = Vec2::splat(480.0);

pub struct SpriteLogo {
    pub center: Vec2,
}

impl SpriteLogo {
    pub fn new(center: Vec2) -> Self {
        return Self { center };
    }

    pub fn vertices(&self) -> impl Iterator<Item = SpriteVertex> {
        let position = self.center - UV_SIZE / 2.0;
        return Sprite::new(
            UV_POSITION,
            UV_SIZE,
            SpriteMaterial::SystemWhite,
            position,
            UV_SIZE,
        )
        .vertices();
    }

    pub fn write_to_model_buffer(&self, buffer: &mut Vec<ModelVertex>, resolution: Vec2) {
        for vertex in self.vertices() {
            buffer.push(vertex.to_model_vertex(resolution));
        }
    }
}
