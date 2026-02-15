use ::parry3d::math::Point;
use ::parry3d::shape::TriMesh;

use crate::gltf::GLTFVertex;

pub fn trimesh_from_vertices(gltf_vertices: impl Iterator<Item = GLTFVertex>) -> TriMesh {
    let mut vertices = Vec::new();

    for v in gltf_vertices {
        vertices.push(Point::new(v.position.x, v.position.y, v.position.z));
    }

    let mut indices = Vec::new();
    let tri_count = vertices.len() / 3;
    for tri in 0..tri_count {
        let base = (tri * 3) as u32;
        indices.push([base, base + 1, base + 2]);
    }

    return TriMesh::new(vertices, indices);
}
