use glam::Vec2;

use super::SystemColor;
use crate::graphics::sprite::{Sprite, SpriteVertex};

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
    pub color: SystemColor,
}

impl SpriteGlyph {
    pub fn new(glyph: Glyph, position: Vec2, color: SystemColor) -> Self {
        return Self {
            glyph,
            position,
            color,
        };
    }

    pub fn vertices(&self) -> impl Iterator<Item = SpriteVertex> {
        return Sprite::new(
            self.glyph.uv_position(),
            GLYPH_SIZE,
            self.color.material(),
            self.position,
            GLYPH_SIZE,
        )
        .vertices();
    }
}
