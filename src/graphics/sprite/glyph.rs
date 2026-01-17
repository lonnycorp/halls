use glam::Vec2;

use super::Sprite;
use crate::graphics::model::ModelBuffer;
use crate::SYSTEM_TEXTURE_INDEX;

const GLYPH_WIDTH: f32 = 8.0;
const GLYPH_HEIGHT: f32 = 16.0;

pub const GLYPH_SIZE: Vec2 = Vec2::new(GLYPH_WIDTH, GLYPH_HEIGHT);

pub enum Glyph {
    Selector,
}

impl Glyph {
    fn uv_position(&self) -> Vec2 {
        return match self {
            Glyph::Selector => Vec2::new(0.0, 0.0),
        };
    }
}

pub struct SpriteGlyph {
    pub glyph: Glyph,
    pub position: Vec2,
    pub color: [u8; 4],
}

impl SpriteGlyph {
    pub fn new(glyph: Glyph, position: Vec2, color: [u8; 4]) -> Self {
        return Self {
            glyph,
            position,
            color,
        };
    }

    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer, resolution: Vec2) {
        Sprite {
            uv_position: self.glyph.uv_position(),
            uv_size: GLYPH_SIZE,
            texture_ix: SYSTEM_TEXTURE_INDEX as u32,
            position: self.position,
            size: GLYPH_SIZE,
            color: self.color,
        }
        .write_to_model_buffer(buffer, resolution);
    }
}
