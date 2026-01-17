use glam::Vec2;

use super::Sprite;
use crate::graphics::model::ModelBuffer;
use crate::SYSTEM_TEXTURE_INDEX;

const UV_POSITION: Vec2 = Vec2::new(32.0, 0.0);
const UV_SIZE: Vec2 = Vec2::splat(16.0);

pub struct SpriteSolid {
    pub position: Vec2,
    pub size: Vec2,
    pub color: [u8; 4],
}

impl SpriteSolid {
    pub fn new(position: Vec2, size: Vec2, color: [u8; 4]) -> Self {
        return Self {
            position,
            size,
            color,
        };
    }

    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer, resolution: Vec2) {
        Sprite {
            uv_position: UV_POSITION,
            uv_size: UV_SIZE,
            texture_ix: SYSTEM_TEXTURE_INDEX as u32,
            position: self.position,
            size: self.size,
            color: self.color,
        }
        .write_to_model_buffer(buffer, resolution);
    }
}
