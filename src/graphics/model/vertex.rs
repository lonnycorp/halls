use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

const MODEL_VERTEX_SHADER_LOCATION_POSITION: u32 = 0;
const MODEL_VERTEX_SHADER_LOCATION_DIFFUSE_UV: u32 = 1;
const MODEL_VERTEX_SHADER_LOCATION_LIGHTMAP_UV: u32 = 2;
const MODEL_VERTEX_SHADER_LOCATION_MATERIAL_IX: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: Vec3,
    pub diffuse_uv: Vec2,
    pub lightmap_uv: Vec2,
    pub material_ix: u32,
}

pub fn model_layout() -> wgpu::VertexBufferLayout<'static> {
    return wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<ModelVertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::offset_of!(ModelVertex, position) as u64,
                shader_location: MODEL_VERTEX_SHADER_LOCATION_POSITION,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: std::mem::offset_of!(ModelVertex, diffuse_uv) as u64,
                shader_location: MODEL_VERTEX_SHADER_LOCATION_DIFFUSE_UV,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: std::mem::offset_of!(ModelVertex, lightmap_uv) as u64,
                shader_location: MODEL_VERTEX_SHADER_LOCATION_LIGHTMAP_UV,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Uint32,
                offset: std::mem::offset_of!(ModelVertex, material_ix) as u64,
                shader_location: MODEL_VERTEX_SHADER_LOCATION_MATERIAL_IX,
            },
        ],
    };
}
