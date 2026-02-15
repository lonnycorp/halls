use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct LevelPushConstants {
    pub clock: u32,
    pub lightmap_material_id: u32,
}

pub const PUSH_CONSTANT_RANGE: wgpu::PushConstantRange = wgpu::PushConstantRange {
    stages: wgpu::ShaderStages::FRAGMENT,
    range: 0..8,
};

pub fn bind(rp: &mut wgpu::RenderPass, clock: u32, lightmap_material_id: u32) {
    let pc = LevelPushConstants {
        clock,
        lightmap_material_id,
    };
    rp.set_push_constants(wgpu::ShaderStages::FRAGMENT, 0, bytemuck::bytes_of(&pc));
}
