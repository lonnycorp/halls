use image::RgbaImage;

pub fn texture_array_bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2Array,
            multisampled: false,
        },
        count: None,
    }
}

pub fn texture_array_binding_array_bind_group_layout_entry(
    binding: u32,
    count: u32,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2Array,
            multisampled: false,
        },
        count: std::num::NonZeroU32::new(count),
    }
}

pub fn texture_array_binding_array_bind_group_entry<'a>(
    binding: u32,
    views: &'a [&'a wgpu::TextureView],
) -> wgpu::BindGroupEntry<'a> {
    wgpu::BindGroupEntry {
        binding,
        resource: wgpu::BindingResource::TextureViewArray(views),
    }
}

pub struct TextureArray {
    texture: wgpu::Texture,
    dims: (u32, u32),
    layers: usize,
    view: wgpu::TextureView,
}

#[derive(Debug, Clone, Copy)]
pub enum TextureArrayWriteError {
    DimensionsMismatch,
    LayerOutOfBounds,
}

impl TextureArray {
    pub fn new(device: &wgpu::Device, dims: (u32, u32), size: usize) -> Self {
        let (width, height) = dims;

        let extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: size as u32,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("TextureArray"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        Self {
            texture,
            dims: (width, height),
            layers: size,
            view,
        }
    }

    pub fn view(&self) -> &wgpu::TextureView {
        return &self.view;
    }

    pub fn write(
        &self,
        queue: &wgpu::Queue,
        index: usize,
        image: &RgbaImage,
    ) -> Result<(), TextureArrayWriteError> {
        let (width, height) = self.dims;
        let (w, h) = image.dimensions();
        if w != width || h != height {
            return Err(TextureArrayWriteError::DimensionsMismatch);
        }

        if index >= self.layers {
            return Err(TextureArrayWriteError::LayerOutOfBounds);
        }

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: index as u32,
                },
                aspect: wgpu::TextureAspect::All,
            },
            image,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        return Ok(());
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(&self.view),
        }
    }
}
