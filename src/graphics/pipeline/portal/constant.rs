pub const PUSH_CONSTANT_RANGE: wgpu::PushConstantRange = wgpu::PushConstantRange {
    stages: wgpu::ShaderStages::FRAGMENT,
    range: 0..4,
};

pub fn bind(rp: &mut wgpu::RenderPass, open_factor: f32) {
    rp.set_push_constants(
        wgpu::ShaderStages::FRAGMENT,
        0,
        bytemuck::bytes_of(&open_factor),
    );
}
