use crate::graphics::storage::MaterialIndexStorageBuffer;
use crate::graphics::uniform::{camera_bind_group_layout_entry, UniformCamera};

const BIND_GROUP_INDEX: u32 = 1;

pub fn config_bind_group_layout_create(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Level Config Bind Group Layout"),
        entries: &[
            camera_bind_group_layout_entry(0),
            MaterialIndexStorageBuffer::bind_group_layout_entry(1),
        ],
    });
}

pub struct PipelineLevelBindGroupConfig {
    bind_group: wgpu::BindGroup,
}

impl PipelineLevelBindGroupConfig {
    pub fn new(
        device: &wgpu::Device,
        camera: &UniformCamera,
        material_index: &MaterialIndexStorageBuffer,
    ) -> Self {
        let layout = config_bind_group_layout_create(device);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Level Config Bind Group"),
            layout: &layout,
            entries: &[
                camera.bind_group_entry(0),
                material_index.bind_group_entry(1),
            ],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>, camera_offset: u32) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[camera_offset]);
    }
}
