use bytemuck::{Pod, Zeroable};

use crate::graphics::color::Color;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub diffuse_uv: [f32; 2],
    pub lightmap_uv: [f32; 2],
    pub texture_ix: u32,
    pub color: Color,
}

pub fn layout() -> wgpu::VertexBufferLayout<'static> {
    return wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<ModelVertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 12,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 20,
                shader_location: 2,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Uint32,
                offset: 28,
                shader_location: 3,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Unorm8x4,
                offset: 32,
                shader_location: 4,
            },
        ],
    };
}
