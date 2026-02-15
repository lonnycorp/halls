use glam::Vec2;

use crate::graphics::sprite::SpriteVertex;

use super::text::{SpriteText, TEXT_SIZE};
use super::TextColor;

const TEXT_COLOR: TextColor = TextColor::White;
const BLINK_PERIOD: u32 = 30;

pub struct SpriteTextInput<'a> {
    position: Vec2,
    max_len: usize,
    text: &'a str,
    active: bool,
    clock: u32,
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

    pub fn vertices(&self) -> impl Iterator<Item = SpriteVertex> + 'a {
        let color = TEXT_COLOR;
        let max_len_zero = self.max_len == 0;

        let visible: &str = if max_len_zero {
            ""
        } else if self.active {
            let start = self.text.len().saturating_sub(self.max_len - 1);
            &self.text[start..]
        } else {
            let end = self.text.len().min(self.max_len);
            &self.text[..end]
        };

        let start_x = self.position.x;
        let y = self.position.y;
        let visible_len = visible.chars().count();
        let text_vertices = visible.chars().enumerate().flat_map(move |(i, c)| {
            let position = Vec2::new(start_x + i as f32 * TEXT_SIZE.x, y);
            return SpriteText::new(c, false, position, color).vertices();
        });

        let cursor_visible =
            !max_len_zero && self.active && (self.clock / BLINK_PERIOD).is_multiple_of(2);
        let cursor_x = start_x + visible_len as f32 * TEXT_SIZE.x;
        let cursor_pos = Vec2::new(cursor_x, y);
        let cursor_vertices = cursor_visible
            .then_some('_')
            .into_iter()
            .flat_map(move |_| SpriteText::new('_', false, cursor_pos, color).vertices());

        return text_vertices.chain(cursor_vertices);
    }
}
