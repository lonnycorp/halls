use glam::Vec2;

use crate::graphics::model::ModelVertex;

use super::SpriteMaterial;

pub struct SpriteVertex {
    pub position: Vec2,
    pub uv_position: Vec2,
    pub material: SpriteMaterial,
}

impl SpriteVertex {
    pub fn to_model_vertex(&self, resolution: Vec2) -> ModelVertex {
        // NDC: x maps [0,w] -> [-1,1], y maps [0,h] -> [1,-1] (inverted)
        let ndc = self.position / resolution * 2.0 - Vec2::ONE;
        let ndc = Vec2::new(ndc.x, -ndc.y);

        return ModelVertex {
            position: ndc.extend(0.0),
            diffuse_uv: self.uv_position,
            lightmap_uv: Vec2::ZERO,
            material_ix: self.material.data().material_ix,
        };
    }
}
