use crate::graphics::model::{Model, ModelBuffer, ModelVertex};

const VERTICES: [ModelVertex; 6] = [
    ModelVertex {
        position: [-0.5, -0.5, 0.0],
        diffuse_uv: [0.0, 0.0],
        lightmap_uv: [0.0, 0.0],
        texture_ix: 0,
        color: [255, 255, 255, 255],
    },
    ModelVertex {
        position: [0.5, -0.5, 0.0],
        diffuse_uv: [1.0, 0.0],
        lightmap_uv: [0.0, 0.0],
        texture_ix: 0,
        color: [255, 255, 255, 255],
    },
    ModelVertex {
        position: [0.5, 0.5, 0.0],
        diffuse_uv: [1.0, 1.0],
        lightmap_uv: [0.0, 0.0],
        texture_ix: 0,
        color: [255, 255, 255, 255],
    },
    ModelVertex {
        position: [-0.5, -0.5, 0.0],
        diffuse_uv: [0.0, 0.0],
        lightmap_uv: [0.0, 0.0],
        texture_ix: 0,
        color: [255, 255, 255, 255],
    },
    ModelVertex {
        position: [0.5, 0.5, 0.0],
        diffuse_uv: [1.0, 1.0],
        lightmap_uv: [0.0, 0.0],
        texture_ix: 0,
        color: [255, 255, 255, 255],
    },
    ModelVertex {
        position: [-0.5, 0.5, 0.0],
        diffuse_uv: [0.0, 1.0],
        lightmap_uv: [0.0, 0.0],
        texture_ix: 0,
        color: [255, 255, 255, 255],
    },
];

pub struct PortalModel {
    model: Model,
}

impl PortalModel {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mut buffer = ModelBuffer::new();
        for v in &VERTICES {
            buffer.push(*v);
        }

        let mut model = Model::new(device, 6);
        model.upload(queue, &buffer);

        return Self { model };
    }

    pub fn draw<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        self.model.draw(rp);
    }
}
