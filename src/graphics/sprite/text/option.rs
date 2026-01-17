use glam::Vec2;

use super::label::SpriteLabel;
use super::text::TEXT_SIZE;
use crate::graphics::model::ModelBuffer;
use crate::graphics::sprite::{Glyph, SpriteGlyph};

const INDENT: f32 = TEXT_SIZE.x + 2.0;
const GREY: [u8; 4] = [128, 128, 128, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];
const CYAN: [u8; 4] = [0, 255, 255, 255];

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
            OptionState::Disabled => GREY,
            OptionState::Unselected => WHITE,
            OptionState::Selected => CYAN,
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
