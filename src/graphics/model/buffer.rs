use super::vertex::ModelVertex;

pub struct ModelBuffer {
    vertices: Vec<ModelVertex>,
}

impl ModelBuffer {
    pub fn new() -> Self {
        return Self {
            vertices: Vec::new(),
        };
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    pub fn push(&mut self, vertex: ModelVertex) {
        self.vertices.push(vertex);
    }

    pub(super) fn vertex_count(&self) -> usize {
        return self.vertices.len();
    }

    pub fn vertices(&self) -> &[ModelVertex] {
        return &self.vertices;
    }
}
