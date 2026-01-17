use std::mem::size_of;
use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};

const MAX_MATERIALS: usize = 512;
const MAX_FRAMES: usize = 4096;

pub enum MaterialIndexWriteError {
    TooManyMaterials,
    TooManyFrames,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct MaterialEntry {
    num_frames: u32,
    speed: f32,
    offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct MaterialIndexStorageBufferData {
    entries: [MaterialEntry; MAX_MATERIALS],
    frames: [u32; MAX_FRAMES],
    next_free_frame: u32,
}

impl MaterialIndexStorageBufferData {
    pub fn new() -> Self {
        return Zeroable::zeroed();
    }

    pub fn write(
        &mut self,
        material_id: usize,
        speed: f32,
        texture_ids: &[u32],
    ) -> Result<(), MaterialIndexWriteError> {
        if material_id >= MAX_MATERIALS {
            return Err(MaterialIndexWriteError::TooManyMaterials);
        }
        if self.next_free_frame as usize + texture_ids.len() > MAX_FRAMES {
            return Err(MaterialIndexWriteError::TooManyFrames);
        }
        let offset = self.next_free_frame;
        self.entries[material_id] = MaterialEntry {
            num_frames: texture_ids.len() as u32,
            speed,
            offset,
        };
        for (i, &tid) in texture_ids.iter().enumerate() {
            self.frames[offset as usize + i] = tid;
        }
        self.next_free_frame += texture_ids.len() as u32;
        return Ok(());
    }
}

pub struct MaterialIndexStorageBuffer {
    buffer: wgpu::Buffer,
}

impl MaterialIndexStorageBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Material Index Storage Buffer"),
            size: size_of::<MaterialIndexStorageBufferData>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self { buffer };
    }

    pub fn write(&self, queue: &wgpu::Queue, data: &MaterialIndexStorageBufferData) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(data));
    }

    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(
                    size_of::<MaterialIndexStorageBufferData>() as u64
                ),
            },
            count: None,
        }
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: NonZeroU64::new(size_of::<MaterialIndexStorageBufferData>() as u64),
            }),
        }
    }
}
