use glam::Vec2;

use super::Sprite;
use crate::graphics::model::ModelBuffer;
use crate::SYSTEM_TEXTURE_INDEX;

const UV_POSITION: Vec2 = Vec2::new(0.0, 16.0);
const UV_SIZE: Vec2 = Vec2::splat(480.0);
pub struct SpriteLogo {
    pub center: Vec2,
    pub alpha: u8,
}

impl SpriteLogo {
    pub fn new(center: Vec2, alpha: u8) -> Self {
        return Self { center, alpha };
    }

    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer, resolution: Vec2) {
        let position = self.center - UV_SIZE / 2.0;
        Sprite {
            uv_position: UV_POSITION,
            uv_size: UV_SIZE,
            texture_ix: SYSTEM_TEXTURE_INDEX as u32,
            position,
            size: UV_SIZE,
            color: [255, 255, 255, self.alpha],
        }
        .write_to_model_buffer(buffer, resolution);
    }
}
