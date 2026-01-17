pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    return wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        count: None,
    };
}

pub struct Sampler {
    sampler: wgpu::Sampler,
}

impl Sampler {
    pub fn new(
        device: &wgpu::Device,
        address_mode: (wgpu::AddressMode, wgpu::AddressMode),
        filter: wgpu::FilterMode,
    ) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: address_mode.0,
            address_mode_v: address_mode.1,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter,
            min_filter: filter,
            mipmap_filter: filter,
            ..Default::default()
        });

        return Self { sampler };
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        return wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Sampler(&self.sampler),
        };
    }
}
