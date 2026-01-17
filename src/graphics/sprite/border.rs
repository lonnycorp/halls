use glam::Vec2;

use super::Sprite;
use crate::graphics::model::ModelBuffer;
use crate::SYSTEM_TEXTURE_INDEX;

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
const COLOR: [u8; 4] = [255, 255, 255, 255];

pub struct SpriteBorder {
    pub position: Vec2,
    pub size: Vec2,
}

impl SpriteBorder {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        return Self { position, size };
    }

    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer, resolution: Vec2) {
        let p = self.position;
        let s = self.size;
        let color = COLOR;
        let texture_ix = SYSTEM_TEXTURE_INDEX as u32;
        let inner_w = s.x - BORDER * 2.0;
        let inner_h = s.y - BORDER * 2.0;

        let parts: [(Vec2, Vec2, Vec2); 9] = [
            // Top-left corner
            (BOX_TL, BOX_CORNER_SIZE, Vec2::new(BORDER, BORDER)),
            // Top edge
            (BOX_T, BOX_EDGE_H_SIZE, Vec2::new(inner_w, BORDER)),
            // Top-right corner
            (BOX_TR, BOX_CORNER_SIZE, Vec2::new(BORDER, BORDER)),
            // Left edge
            (BOX_L, BOX_EDGE_V_SIZE, Vec2::new(BORDER, inner_h)),
            // Center
            (BOX_C, BOX_CENTER_SIZE, Vec2::new(inner_w, inner_h)),
            // Right edge
            (BOX_R, BOX_EDGE_V_SIZE, Vec2::new(BORDER, inner_h)),
            // Bottom-left corner
            (BOX_BL, BOX_CORNER_SIZE, Vec2::new(BORDER, BORDER)),
            // Bottom edge
            (BOX_B, BOX_EDGE_H_SIZE, Vec2::new(inner_w, BORDER)),
            // Bottom-right corner
            (BOX_BR, BOX_CORNER_SIZE, Vec2::new(BORDER, BORDER)),
        ];

        let positions = [
            p,
            Vec2::new(p.x + BORDER, p.y),
            Vec2::new(p.x + s.x - BORDER, p.y),
            Vec2::new(p.x, p.y + BORDER),
            Vec2::new(p.x + BORDER, p.y + BORDER),
            Vec2::new(p.x + s.x - BORDER, p.y + BORDER),
            Vec2::new(p.x, p.y + s.y - BORDER),
            Vec2::new(p.x + BORDER, p.y + s.y - BORDER),
            Vec2::new(p.x + s.x - BORDER, p.y + s.y - BORDER),
        ];

        for i in 0..9 {
            let (uv_position, uv_size, size) = parts[i];
            Sprite {
                uv_position,
                uv_size,
                texture_ix,
                position: positions[i],
                size,
                color,
            }
            .write_to_model_buffer(buffer, resolution);
        }
    }
}
