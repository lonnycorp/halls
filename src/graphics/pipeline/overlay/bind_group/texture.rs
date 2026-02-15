use crate::graphics::sprite::{SYSTEM_TEXTURE_REF, TEXT_TEXTURE_REF};
use crate::graphics::texture::{
    sampler_bind_group_layout_entry, texture_array_bind_group_layout_entry, Sampler, TextureArray,
};
use crate::ASSET;

const BIND_GROUP_INDEX: u32 = 0;
const TEXT_TEXTURE_PATH: &str = "texture/text.png";
const SYSTEM_TEXTURE_PATH: &str = "texture/system.png";
const OVERLAY_TEXTURE_SIZE: (u32, u32) = (512, 512);
const OVERLAY_TEXTURE_LAYERS: usize = 2;

pub fn texture_bind_group_layout_create(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Overlay Texture Bind Group Layout"),
        entries: &[
            sampler_bind_group_layout_entry(0),
            texture_array_bind_group_layout_entry(1),
        ],
    });
}

pub struct PipelineOverlayBindGroupTexture {
    bind_group: wgpu::BindGroup,
}

impl PipelineOverlayBindGroupTexture {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let text_image =
            image::load_from_memory(ASSET.get_file(TEXT_TEXTURE_PATH).unwrap().contents())
                .unwrap()
                .to_rgba8();
        let system_image =
            image::load_from_memory(ASSET.get_file(SYSTEM_TEXTURE_PATH).unwrap().contents())
                .unwrap()
                .to_rgba8();

        let diffuse = TextureArray::new(device, OVERLAY_TEXTURE_SIZE, OVERLAY_TEXTURE_LAYERS);
        diffuse
            .write(queue, TEXT_TEXTURE_REF.layer as usize, &text_image)
            .unwrap();
        diffuse
            .write(queue, SYSTEM_TEXTURE_REF.layer as usize, &system_image)
            .unwrap();

        let layout = texture_bind_group_layout_create(device);

        let diffuse_sampler = Sampler::new(
            device,
            (wgpu::AddressMode::Repeat, wgpu::AddressMode::Repeat),
            wgpu::FilterMode::Nearest,
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
