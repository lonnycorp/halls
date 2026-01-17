use crate::graphics::texture::{
    bind_group_entry_array, bind_group_layout_entry_array, sampler_bind_group_layout_entry,
    Sampler, TextureArray,
};

const BIND_GROUP_INDEX: u32 = 0;

#[derive(Copy, Clone)]
pub struct TextureBucket {
    pub width: u32,
    pub height: u32,
    pub layers: usize,
}

pub const TEXTURE_BUCKETS: [TextureBucket; 6] = [
    TextureBucket {
        width: 2048,
        height: 2048,
        layers: 1,
    },
    TextureBucket {
        width: 1024,
        height: 1024,
        layers: 4,
    },
    TextureBucket {
        width: 512,
        height: 512,
        layers: 8,
    },
    TextureBucket {
        width: 256,
        height: 256,
        layers: 32,
    },
    TextureBucket {
        width: 128,
        height: 128,
        layers: 64,
    },
    TextureBucket {
        width: 64,
        height: 64,
        layers: 256,
    },
];

pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Level Texture Bind Group Layout"),
        entries: &[
            sampler_bind_group_layout_entry(0),
            bind_group_layout_entry_array(1, TEXTURE_BUCKETS.len() as u32),
        ],
    });
}

pub struct PipelineLevelBindGroupTexture {
    bind_group: wgpu::BindGroup,
}

impl PipelineLevelBindGroupTexture {
    pub fn new(device: &wgpu::Device, diffuse: &[TextureArray; TEXTURE_BUCKETS.len()]) -> Self {
        let layout = create_bind_group_layout(device);

        let diffuse_sampler = Sampler::new(
            device,
            (wgpu::AddressMode::Repeat, wgpu::AddressMode::Repeat),
            wgpu::FilterMode::Linear,
        );

        let views: [&wgpu::TextureView; TEXTURE_BUCKETS.len()] =
            std::array::from_fn(|i| diffuse[i].view());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Level Texture Bind Group"),
            layout: &layout,
            entries: &[
                diffuse_sampler.bind_group_entry(0),
                bind_group_entry_array(1, &views),
            ],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[]);
    }
}
