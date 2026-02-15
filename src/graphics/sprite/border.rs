use glam::Vec2;

use super::{Sprite, SpriteVertex};
use crate::graphics::sprite::SpriteMaterial;

const BORDER: f32 = 3.0;

const BOX_TL: Vec2 = Vec2::new(16.0, 0.0);
const BOX_T: Vec2 = Vec2::new(19.0, 0.0);
const BOX_TR: Vec2 = Vec2::new(29.0, 0.0);
const BOX_L: Vec2 = Vec2::new(16.0, 3.0);
const BOX_C: Vec2 = Vec2::new(19.0, 3.0);
const BOX_R: Vec2 = Vec2::new(29.0, 3.0);
const BOX_BL: Vec2 = Vec2::new(16.0, 13.0);
const BOX_B: Vec2 = Vec2::new(19.0, 13.0);
const BOX_BR: Vec2 = Vec2::new(29.0, 13.0);

const BOX_CORNER_SIZE: Vec2 = Vec2::new(3.0, 3.0);
const BOX_EDGE_H_SIZE: Vec2 = Vec2::new(10.0, 3.0);
const BOX_EDGE_V_SIZE: Vec2 = Vec2::new(3.0, 10.0);
const BOX_CENTER_SIZE: Vec2 = Vec2::new(10.0, 10.0);
const MATERIAL: SpriteMaterial = SpriteMaterial::SystemWhite;

#[derive(Clone, Copy)]
enum BorderCell {
    Start,
    Middle,
    End,
}

#[derive(Clone, Copy)]
struct BorderMask {
    x: BorderCell,
    y: BorderCell,
}

const BORDER_MASKS: [BorderMask; 9] = [
    BorderMask {
        x: BorderCell::Start,
        y: BorderCell::Start,
    },
    BorderMask {
        x: BorderCell::Middle,
        y: BorderCell::Start,
    },
    BorderMask {
        x: BorderCell::End,
        y: BorderCell::Start,
    },
    BorderMask {
        x: BorderCell::Start,
        y: BorderCell::Middle,
    },
    BorderMask {
        x: BorderCell::Middle,
        y: BorderCell::Middle,
    },
    BorderMask {
        x: BorderCell::End,
        y: BorderCell::Middle,
    },
    BorderMask {
        x: BorderCell::Start,
        y: BorderCell::End,
    },
    BorderMask {
        x: BorderCell::Middle,
        y: BorderCell::End,
    },
    BorderMask {
        x: BorderCell::End,
        y: BorderCell::End,
    },
];

pub struct SpriteBorder {
    position: Vec2,
    size: Vec2,
}

impl BorderCell {
    fn position(&self, position: f32, size: f32) -> f32 {
        return match self {
            BorderCell::Start => position,
            BorderCell::Middle => position + BORDER,
            BorderCell::End => position + size - BORDER,
        };
    }

    fn size(&self, size: f32) -> f32 {
        return match self {
            BorderCell::Middle => size - BORDER * 2.0,
            _ => BORDER,
        };
    }
}

impl BorderMask {
    fn position(&self, position: Vec2, size: Vec2) -> Vec2 {
        return Vec2::new(
            self.x.position(position.x, size.x),
            self.y.position(position.y, size.y),
        );
    }

    fn size(&self, size: Vec2) -> Vec2 {
        return Vec2::new(self.x.size(size.x), self.y.size(size.y));
    }

    fn uv_position(&self) -> Vec2 {
        return match (self.x, self.y) {
            (BorderCell::Start, BorderCell::Start) => BOX_TL,
            (BorderCell::Middle, BorderCell::Start) => BOX_T,
            (BorderCell::End, BorderCell::Start) => BOX_TR,
            (BorderCell::Start, BorderCell::Middle) => BOX_L,
            (BorderCell::Middle, BorderCell::Middle) => BOX_C,
            (BorderCell::End, BorderCell::Middle) => BOX_R,
            (BorderCell::Start, BorderCell::End) => BOX_BL,
            (BorderCell::Middle, BorderCell::End) => BOX_B,
            (BorderCell::End, BorderCell::End) => BOX_BR,
        };
    }

    fn uv_size(&self) -> Vec2 {
        return match (self.x, self.y) {
            (BorderCell::Middle, BorderCell::Middle) => BOX_CENTER_SIZE,
            (BorderCell::Middle, _) => BOX_EDGE_H_SIZE,
            (_, BorderCell::Middle) => BOX_EDGE_V_SIZE,
            _ => BOX_CORNER_SIZE,
        };
    }
}

impl SpriteBorder {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        return Self { position, size };
    }

    pub fn vertices(&self) -> impl Iterator<Item = SpriteVertex> {
        let position = self.position;
        let size = self.size;

        return BORDER_MASKS.into_iter().flat_map(move |mask| {
            return Sprite::new(
                mask.uv_position(),
                mask.uv_size(),
                MATERIAL,
                mask.position(position, size),
                mask.size(size),
            )
            .vertices();
        });
    }
}
