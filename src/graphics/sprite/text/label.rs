use glam::Vec2;

use crate::graphics::color::Color;
use crate::graphics::model::ModelBuffer;

use super::text::{SpriteText, TEXT_SIZE};

pub struct SpriteLabel<'a> {
    pub position: Vec2,
    pub max_len: usize,
    pub color: Color,
    pub bold: bool,
    pub text: &'a str,
}

impl<'a> SpriteLabel<'a> {
    pub fn new(position: Vec2, max_len: usize, color: Color, bold: bool, text: &'a str) -> Self {
        return Self {
            position,
            max_len,
            color,
            bold,
            text,
        };
    }

    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer, resolution: Vec2) {
        let len = self.text.len().min(self.max_len);
        let visible = &self.text[..len];

        for (i, c) in visible.chars().enumerate() {
            let position = Vec2::new(self.position.x + i as f32 * TEXT_SIZE.x, self.position.y);
            SpriteText::new(c, self.bold, position, self.color)
                .write_to_model_buffer(buffer, resolution);
        }
    }
}
