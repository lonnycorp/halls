pub struct TextureDepth {
    view: wgpu::TextureView,
}

impl TextureDepth {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        return Self {
            view: texture.create_view(&Default::default()),
        };
    }

    pub fn view(&self) -> &wgpu::TextureView {
        return &self.view;
    }
}
