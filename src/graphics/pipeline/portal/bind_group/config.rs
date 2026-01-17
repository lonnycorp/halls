use crate::graphics::uniform::{camera_bind_group_layout_entry, UniformCamera};

const BIND_GROUP_INDEX: u32 = 1;

pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Portal Config Bind Group Layout"),
        entries: &[camera_bind_group_layout_entry(0)],
    });
}

pub struct PipelinePortalBindGroupConfig {
    bind_group: wgpu::BindGroup,
}

impl PipelinePortalBindGroupConfig {
    pub fn new(device: &wgpu::Device, camera: &UniformCamera) -> Self {
        let layout = create_bind_group_layout(device);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Portal Config Bind Group"),
            layout: &layout,
            entries: &[camera.bind_group_entry(0)],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>, camera_offset: u32) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[camera_offset]);
    }
}
