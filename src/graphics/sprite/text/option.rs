use glam::Vec2;

use super::label::SpriteLabel;
use super::text::TEXT_SIZE;
use crate::graphics::color::Color;
use crate::graphics::model::ModelBuffer;
use crate::graphics::sprite::{Glyph, SpriteGlyph};

const INDENT: f32 = TEXT_SIZE.x + 2.0;

pub enum OptionState {
    Disabled,
    Unselected,
    Selected,
}

pub struct SpriteTextOption<'a> {
    pub position: Vec2,
    pub max_len: usize,
    pub hovered: bool,
    pub state: OptionState,
    pub text: &'a str,
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

    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer, resolution: Vec2) {
        let color = match self.state {
            OptionState::Disabled => Color::GRAY,
            OptionState::Unselected => Color::WHITE,
            OptionState::Selected => Color::CYAN,
        };

        if self.hovered {
            SpriteGlyph::new(Glyph::Selector, self.position, color)
                .write_to_model_buffer(buffer, resolution);
        }

        let text_pos = Vec2::new(self.position.x + INDENT, self.position.y);
        SpriteLabel::new(text_pos, self.max_len, color, false, self.text)
            .write_to_model_buffer(buffer, resolution);
    }
}
