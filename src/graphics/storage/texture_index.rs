use std::mem::size_of;
use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};

const MAX_TEXTURES: usize = 1024;

pub struct TextureIndexFull;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct TextureEntry {
    pub bucket: u32,
    pub layer: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct TextureIndexStorageBufferData {
    entries: [TextureEntry; MAX_TEXTURES],
    next_index: u32,
}

impl TextureIndexStorageBufferData {
    pub fn new() -> Self {
        return Zeroable::zeroed();
    }

    pub fn write(&mut self, bucket: u32, layer: u32) -> Result<u32, TextureIndexFull> {
        if self.next_index as usize >= MAX_TEXTURES {
            return Err(TextureIndexFull);
        }
        let texture_id = self.next_index;
        self.entries[texture_id as usize] = TextureEntry { bucket, layer };
        self.next_index += 1;
        return Ok(texture_id);
    }
}

pub struct TextureIndexStorageBuffer {
    buffer: wgpu::Buffer,
}

impl TextureIndexStorageBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Texture Index Storage Buffer"),
            size: size_of::<TextureIndexStorageBufferData>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self { buffer };
    }

    pub fn write(&self, queue: &wgpu::Queue, data: &TextureIndexStorageBufferData) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(data));
    }

    pub fn bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(size_of::<TextureIndexStorageBufferData>() as u64),
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
                size: NonZeroU64::new(size_of::<TextureIndexStorageBufferData>() as u64),
            }),
        }
    }
}
