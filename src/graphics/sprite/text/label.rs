use glam::Vec2;

use crate::graphics::sprite::SpriteVertex;

use super::text::{SpriteText, TEXT_SIZE};
use super::TextColor;

#[derive(Clone, Copy)]
pub enum SpriteLabelAlignment {
    Left,
    Right,
}

pub struct SpriteLabel<'a> {
    position: Vec2,
    max_len: usize,
    color: TextColor,
    bold: bool,
    alignment: SpriteLabelAlignment,
    text: &'a str,
}

impl<'a> SpriteLabel<'a> {
    pub fn new(
        position: Vec2,
        max_len: usize,
        color: TextColor,
        bold: bool,
        alignment: SpriteLabelAlignment,
        text: &'a str,
    ) -> Self {
        return Self {
            position,
            max_len,
            color,
            bold,
            alignment,
            text,
        };
    }

    pub fn vertices(&self) -> impl Iterator<Item = SpriteVertex> + 'a {
        let len = self.text.len().min(self.max_len);
        let visible = &self.text[..len];
        let position = self.position;
        let visible_len = visible.chars().count();
        let bold = self.bold;
        let color = self.color;
        let start_x = match self.alignment {
            SpriteLabelAlignment::Left => position.x,
            SpriteLabelAlignment::Right => {
                position.x + (self.max_len - visible_len) as f32 * TEXT_SIZE.x
            }
        };
        return visible.chars().enumerate().flat_map(move |(i, c)| {
            let char_position = Vec2::new(start_x + i as f32 * TEXT_SIZE.x, position.y);
            return SpriteText::new(c, bold, char_position, color).vertices();
        });
    }
}
