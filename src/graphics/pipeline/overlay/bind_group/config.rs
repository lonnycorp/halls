use strum::IntoEnumIterator;

use crate::graphics::sprite::SpriteMaterial;
use crate::graphics::storage::{MaterialIndexStorageBuffer, MaterialIndexStorageBufferData};

const BIND_GROUP_INDEX: u32 = 1;

pub fn config_bind_group_layout_create(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Overlay Config Bind Group Layout"),
        entries: &[MaterialIndexStorageBuffer::bind_group_layout_entry(0)],
    });
}

pub struct PipelineOverlayBindGroupConfig {
    bind_group: wgpu::BindGroup,
}

impl PipelineOverlayBindGroupConfig {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mut data = MaterialIndexStorageBufferData::new();
        for material in SpriteMaterial::iter() {
            let material_data = material.data();
            data.write(
                material_data.material_ix,
                material_data.speed,
                material_data.texture_refs,
                material_data.color,
                false,
            )
            .unwrap();
        }

        let material_index = MaterialIndexStorageBuffer::new(device);
        material_index.write(queue, &data);

        let layout = config_bind_group_layout_create(device);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Overlay Config Bind Group"),
            layout: &layout,
            entries: &[material_index.bind_group_entry(0)],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[]);
    }
}
