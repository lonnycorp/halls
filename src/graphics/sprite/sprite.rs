use glam::Vec2;

use crate::graphics::model::{ModelBuffer, ModelVertex};

// CCW winding: TL(0), BL(1), BR(2), TL(0), BR(2), TR(3)
const WINDING: [usize; 6] = [0, 1, 2, 0, 2, 3];

pub struct Sprite {
    pub uv_position: Vec2,
    pub uv_size: Vec2,
    pub texture_ix: u32,
    pub position: Vec2,
    pub size: Vec2,
    pub color: [u8; 4],
}

impl Sprite {
    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer, resolution: Vec2) {
        let min = self.position;
        let max = self.position + self.size;

        // NDC: x maps [0,w] -> [-1,1], y maps [0,h] -> [1,-1] (inverted)
        let ndc_min = min / resolution * 2.0 - Vec2::ONE;
        let ndc_max = max / resolution * 2.0 - Vec2::ONE;
        let ndc_min = Vec2::new(ndc_min.x, -ndc_min.y);
        let ndc_max = Vec2::new(ndc_max.x, -ndc_max.y);

        let uv_min = self.uv_position;
        let uv_max = self.uv_position + self.uv_size;

        // Corners: TL, BL, BR, TR
        let positions = [
            ndc_min,
            Vec2::new(ndc_min.x, ndc_max.y),
            ndc_max,
            Vec2::new(ndc_max.x, ndc_min.y),
        ];
        let uvs = [
            uv_min,
            Vec2::new(uv_min.x, uv_max.y),
            uv_max,
            Vec2::new(uv_max.x, uv_min.y),
        ];

        for &i in &WINDING {
            buffer.push(ModelVertex {
                position: positions[i].extend(0.0).into(),
                diffuse_uv: uvs[i].into(),
                lightmap_uv: [0.0, 0.0],
                texture_ix: self.texture_ix,
                color: self.color,
            });
        }
    }
}
