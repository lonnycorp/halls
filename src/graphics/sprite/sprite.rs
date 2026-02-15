use glam::Vec2;

use crate::graphics::sprite::SpriteMaterial;

use super::SpriteVertex;

// CCW winding: TL(0), BL(1), BR(2), TL(0), BR(2), TR(3)
const WINDING: [usize; 6] = [0, 1, 2, 0, 2, 3];

pub struct Sprite {
    uv_position: Vec2,
    uv_size: Vec2,
    material: SpriteMaterial,
    position: Vec2,
    size: Vec2,
}

impl Sprite {
    pub fn new(
        uv_position: Vec2,
        uv_size: Vec2,
        material: SpriteMaterial,
        position: Vec2,
        size: Vec2,
    ) -> Self {
        return Self {
            uv_position,
            uv_size,
            material,
            position,
            size,
        };
    }

    pub fn vertices(&self) -> impl Iterator<Item = SpriteVertex> {
        let min = self.position;
        let max = self.position + self.size;

        let uv_min = self.uv_position;
        let uv_max = self.uv_position + self.uv_size;

        // Corners: TL, BL, BR, TR
        let positions = [min, Vec2::new(min.x, max.y), max, Vec2::new(max.x, min.y)];
        let uvs = [
            uv_min,
            Vec2::new(uv_min.x, uv_max.y),
            uv_max,
            Vec2::new(uv_max.x, uv_min.y),
        ];

        return WINDING
            .map(|i| {
                return SpriteVertex {
                    position: positions[i],
                    uv_position: uvs[i],
                    material: self.material,
                };
            })
            .into_iter();
    }
}
