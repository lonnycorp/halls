use ::parry3d::math::Point;
use ::parry3d::shape::TriMesh;

use crate::gltf::GLTFMesh;

impl From<&GLTFMesh> for TriMesh {
    fn from(mesh: &GLTFMesh) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let tri_count = mesh.vertex_count() / 3;
        for tri in 0..tri_count {
            let base = (tri * 3) as u32;
            indices.push([base, base + 1, base + 2]);
        }

        for i in 0..mesh.vertex_count() {
            let v = mesh.vertex(i);
            vertices.push(Point::new(v.position.x, v.position.y, v.position.z));
        }

        return TriMesh::new(vertices, indices);
    }
}
