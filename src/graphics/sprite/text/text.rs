use glam::Vec2;

use crate::graphics::color::Color;
use crate::graphics::model::ModelBuffer;
use crate::graphics::sprite::Sprite;
use crate::FONT_TEXTURE_INDEX;

const TEXT_WIDTH: f32 = 8.0;
const TEXT_HEIGHT: f32 = 16.0;
const CHARS_PER_ROW: usize = 16;
const FIRST_CHAR: usize = 32; // space
const BOLD_ROW_OFFSET: usize = 8;

pub const TEXT_SIZE: Vec2 = Vec2::new(TEXT_WIDTH, TEXT_HEIGHT);

pub struct SpriteText {
    pub c: char,
    pub bold: bool,
    pub position: Vec2,
    pub color: Color,
}

impl SpriteText {
    pub fn new(c: char, bold: bool, position: Vec2, color: Color) -> Self {
        return Self {
            c,
            bold,
            position,
            color,
        };
    }

    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer, resolution: Vec2) {
        let code = (self.c as usize).wrapping_sub(FIRST_CHAR);
        let code = if code >= 96 { 0 } else { code };
        let row_offset = if self.bold { BOLD_ROW_OFFSET } else { 0 };
        let col = (FIRST_CHAR + code) % CHARS_PER_ROW;
        let row = (FIRST_CHAR + code) / CHARS_PER_ROW + row_offset;

        let uv_position = Vec2::new(col as f32 * TEXT_WIDTH, row as f32 * TEXT_HEIGHT);
        let uv_size = Vec2::new(TEXT_WIDTH, TEXT_HEIGHT);

        Sprite {
            uv_position,
            uv_size,
            texture_ix: FONT_TEXTURE_INDEX as u32,
            position: self.position,
            size: TEXT_SIZE,
            color: self.color,
        }
        .write_to_model_buffer(buffer, resolution);
    }
}
