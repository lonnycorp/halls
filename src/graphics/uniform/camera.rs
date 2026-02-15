use std::mem::size_of;
use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct UniformCameraData {
    pub projection: Mat4,
    pub view: Mat4,
    pub clip_plane: Vec4,
}

impl UniformCameraData {
    pub fn new() -> Self {
        return Self {
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            clip_plane: Vec4::ZERO,
        };
    }

    pub fn view_set(&mut self, position: Vec3, rotation: Vec2) {
        self.view = Mat4::from_rotation_x(-rotation.x)
            * Mat4::from_rotation_y(-rotation.y)
            * Mat4::from_translation(-position);
    }
}

pub fn camera_bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    return wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: true,
            min_binding_size: NonZeroU64::new(size_of::<UniformCameraData>() as u64),
        },
        count: None,
    };
}

pub struct UniformCamera {
    buffer: wgpu::Buffer,
    aligned_size: u64,
    capacity: u32,
}

impl UniformCamera {
    pub fn new(device: &wgpu::Device, capacity: u32) -> Self {
        let min_alignment = device.limits().min_uniform_buffer_offset_alignment as u64;
        let data_size = size_of::<UniformCameraData>() as u64;
        let aligned_size = data_size.div_ceil(min_alignment) * min_alignment;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: aligned_size * capacity as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self {
            buffer,
            aligned_size,
            capacity,
        };
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        return wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: NonZeroU64::new(size_of::<UniformCameraData>() as u64),
            }),
        };
    }

    pub fn write(&self, queue: &wgpu::Queue, index: u32, data: &UniformCameraData) -> u32 {
        assert!(index < self.capacity, "camera uniform buffer overflow");
        let byte_offset = self.aligned_size * index as u64;
        queue.write_buffer(&self.buffer, byte_offset, bytemuck::bytes_of(data));
        return byte_offset as u32;
    }
}
