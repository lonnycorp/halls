use glam::{Vec2, Vec3};

use crate::graphics::color::Color;
use crate::graphics::model::{ModelBuffer, ModelVertex};

pub struct GLTFVertex {
    pub position: Vec3,
    pub diffuse_uv: Option<Vec2>,
    pub lightmap_uv: Option<Vec2>,
    pub material_ix: Option<u32>,
    pub color: Option<Color>,
}

impl GLTFVertex {
    pub fn write_to_model_buffer(&self, buffer: &mut ModelBuffer) {
        buffer.push(ModelVertex {
            position: self.position.into(),
            diffuse_uv: self.diffuse_uv.unwrap_or(Vec2::ZERO).into(),
            lightmap_uv: self.lightmap_uv.unwrap_or(Vec2::ZERO).into(),
            texture_ix: self.material_ix.unwrap_or(0),
            color: self.color.unwrap_or(Color::WHITE),
        });
    }
}
