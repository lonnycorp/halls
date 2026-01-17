use super::buffer::ModelBuffer;
use super::vertex::ModelVertex;

pub struct Model {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
}

impl Model {
    pub fn new(device: &wgpu::Device, capacity: usize) -> Self {
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Model Vertex Buffer"),
            size: (capacity * std::mem::size_of::<ModelVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self {
            vertex_buffer,
            vertex_count: 0,
        };
    }

    pub fn upload(&mut self, queue: &wgpu::Queue, buffer: &ModelBuffer) {
        queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(buffer.vertices()),
        );
        self.vertex_count = buffer.vertex_count() as u32;
    }

    pub fn draw<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rp.draw(0..self.vertex_count, 0..1);
    }
}
