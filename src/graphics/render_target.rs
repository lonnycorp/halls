pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    return wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    };
}

pub struct RenderTarget {
    color_view: wgpu::TextureView,
    depth_view: wgpu::TextureView,
}

impl RenderTarget {
    pub fn new(device: &wgpu::Device, size: (u32, u32), format: wgpu::TextureFormat) -> Self {
        let extent = wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let color_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("RenderTarget Color"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("RenderTarget Depth"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let color_view = color_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        return Self {
            color_view,
            depth_view,
        };
    }

    pub fn color_view(&self) -> &wgpu::TextureView {
        return &self.color_view;
    }

    pub fn depth_view(&self) -> &wgpu::TextureView {
        return &self.depth_view;
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        return wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(&self.color_view),
        };
    }
}
