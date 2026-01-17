use crate::graphics::texture::{
    bind_group_layout_entry, sampler_bind_group_layout_entry, Sampler, TextureArray,
};

const BIND_GROUP_INDEX: u32 = 0;

pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Overlay Texture Bind Group Layout"),
        entries: &[
            sampler_bind_group_layout_entry(0),
            bind_group_layout_entry(1),
        ],
    });
}

pub struct PipelineOverlayBindGroupTexture {
    bind_group: wgpu::BindGroup,
}

impl PipelineOverlayBindGroupTexture {
    pub fn new(
        device: &wgpu::Device,
        diffuse: &TextureArray,
        diffuse_filter: wgpu::FilterMode,
    ) -> Self {
        let layout = create_bind_group_layout(device);

        let diffuse_sampler = Sampler::new(
            device,
            (wgpu::AddressMode::Repeat, wgpu::AddressMode::Repeat),
            diffuse_filter,
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Overlay Texture Bind Group"),
            layout: &layout,
            entries: &[
                diffuse_sampler.bind_group_entry(0),
                diffuse.bind_group_entry(1),
            ],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[]);
    }
}
