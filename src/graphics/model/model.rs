use super::vertex::ModelVertex;

const MODEL_VERTEX_BUFFER_SLOT: u32 = 0;
const MODEL_VERTEX_START: u32 = 0;
const MODEL_INSTANCE_START: u32 = 0;
const MODEL_INSTANCE_COUNT: u32 = 1;

#[derive(Debug)]
pub enum ModelUploadError {
    VerticesExceedCapacity,
}

pub struct Model {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    vertex_capacity: usize,
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
            vertex_capacity: capacity,
        };
    }

    pub fn upload(
        &mut self,
        queue: &wgpu::Queue,
        vertices: &[ModelVertex],
    ) -> Result<(), ModelUploadError> {
        if vertices.len() > self.vertex_capacity {
            return Err(ModelUploadError::VerticesExceedCapacity);
        }
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertices));
        self.vertex_count = vertices.len() as u32;
        return Ok(());
    }

    pub fn draw<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_vertex_buffer(MODEL_VERTEX_BUFFER_SLOT, self.vertex_buffer.slice(..));
        rp.draw(
            MODEL_VERTEX_START..self.vertex_count,
            MODEL_INSTANCE_START..MODEL_INSTANCE_COUNT,
        );
    }
}
