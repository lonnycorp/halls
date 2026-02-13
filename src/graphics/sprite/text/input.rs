use glam::Vec2;

use crate::graphics::color::Color;
use crate::graphics::model::ModelBuffer;

use super::text::{SpriteText, TEXT_SIZE};

const COLOR: Color = Color::WHITE;
const BLINK_PERIOD: u32 = 30;

pub struct SpriteTextInput<'a> {
    pub position: Vec2,
    pub max_len: usize,
    pub text: &'a str,
    pub active: bool,
    pub clock: u32,
}

impl<'a> SpriteTextInput<'a> {
    pub fn new(position: Vec2, max_len: usize, text: &'a str, active: bool, clock: u32) -> Self {
        return Self {
            position,
            max_len,
            text,
            active,
            clock,
        };
    }

    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer, resolution: Vec2) {
        let color = COLOR;

        let visible: &str = if self.active {
            let start = self.text.len().saturating_sub(self.max_len - 1);
            &self.text[start..]
        } else {
            let end = self.text.len().min(self.max_len);
            &self.text[..end]
        };

        let start_x = self.position.x;
        let y = self.position.y;

        for (i, c) in visible.chars().enumerate() {
            let position = Vec2::new(start_x + i as f32 * TEXT_SIZE.x, y);
            SpriteText::new(c, false, position, color).write_to_model_buffer(buffer, resolution);
        }

        let cursor_visible = self.active && (self.clock / BLINK_PERIOD).is_multiple_of(2);
        if cursor_visible {
            let cursor_x = start_x + visible.len() as f32 * TEXT_SIZE.x;
            let cursor_pos = Vec2::new(cursor_x, y);
            SpriteText::new('_', false, cursor_pos, color)
                .write_to_model_buffer(buffer, resolution);
        }
    }
}
