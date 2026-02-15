use crate::graphics::render_target::{render_target_bind_group_layout_entry, RenderTarget};
use crate::graphics::texture::{sampler_bind_group_layout_entry, Sampler};

const BIND_GROUP_INDEX: u32 = 0;

pub fn texture_bind_group_layout_create(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Portal Texture Bind Group Layout"),
        entries: &[
            sampler_bind_group_layout_entry(0),
            render_target_bind_group_layout_entry(1),
        ],
    });
}

pub struct PipelinePortalBindGroupTexture {
    bind_group: wgpu::BindGroup,
}

impl PipelinePortalBindGroupTexture {
    pub fn new(device: &wgpu::Device, render_target: &RenderTarget) -> Self {
        let layout = texture_bind_group_layout_create(device);
        let sampler = Sampler::new(
            device,
            (
                wgpu::AddressMode::ClampToEdge,
                wgpu::AddressMode::ClampToEdge,
            ),
            wgpu::FilterMode::Linear,
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Portal Texture Bind Group"),
            layout: &layout,
            entries: &[
                sampler.bind_group_entry(0),
                render_target.bind_group_entry(1),
            ],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[]);
    }
}
