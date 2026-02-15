use glam::Vec2;

use super::text::SpriteText;
use super::text::TEXT_SIZE;
use crate::graphics::sprite::{Glyph, SpriteGlyph, SpriteVertex, SystemColor};

use super::TextColor;

const INDENT: f32 = TEXT_SIZE.x + 2.0;

pub enum OptionState {
    Disabled,
    Unselected,
    Selected,
}

pub struct SpriteTextOption<'a> {
    position: Vec2,
    max_len: usize,
    hovered: bool,
    state: OptionState,
    text: &'a str,
}

impl<'a> SpriteTextOption<'a> {
    pub fn new(
        position: Vec2,
        max_len: usize,
        hovered: bool,
        state: OptionState,
        text: &'a str,
    ) -> Self {
        return Self {
            position,
            max_len,
            hovered,
            state,
            text,
        };
    }

    pub fn vertices(&self) -> impl Iterator<Item = SpriteVertex> + 'a {
        let color = match self.state {
            OptionState::Disabled => TextColor::Gray,
            OptionState::Unselected => TextColor::White,
            OptionState::Selected => TextColor::Cyan,
        };

        let selector_color = match self.state {
            OptionState::Disabled => SystemColor::Gray,
            OptionState::Unselected => SystemColor::White,
            OptionState::Selected => SystemColor::Cyan,
        };

        let selector_vertices = self
            .hovered
            .then_some(SpriteGlyph::new(
                Glyph::Selector,
                self.position,
                selector_color,
            ))
            .into_iter()
            .flat_map(|glyph| glyph.vertices());

        let len = self.text.len().min(self.max_len);
        let visible = &self.text[..len];
        let text_position = Vec2::new(self.position.x + INDENT, self.position.y);
        let text_vertices = visible.chars().enumerate().flat_map(move |(i, c)| {
            let char_position =
                Vec2::new(text_position.x + i as f32 * TEXT_SIZE.x, text_position.y);
            return SpriteText::new(c, false, char_position, color).vertices();
        });

        return selector_vertices.chain(text_vertices);
    }
}
