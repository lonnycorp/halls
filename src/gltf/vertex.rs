use glam::{Vec2, Vec3};

use crate::color::Color;
use crate::graphics::model::ModelVertex;

pub struct GLTFVertex {
    pub position: Vec3,
    pub diffuse_uv: Option<Vec2>,
    pub lightmap_uv: Option<Vec2>,
    pub material_ix: Option<u32>,
    pub color: Option<Color>,
}

impl GLTFVertex {
    pub fn to_model_vertex(&self) -> ModelVertex {
        return ModelVertex {
            position: self.position,
            diffuse_uv: self.diffuse_uv.unwrap_or(Vec2::ZERO),
            lightmap_uv: self.lightmap_uv.unwrap_or(Vec2::ZERO),
            material_ix: self.material_ix.unwrap_or(0),
        };
    }
}
