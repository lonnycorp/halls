use glam::{Mat4, Vec2, Vec3};

use super::material::GLTFMaterial;
use super::vertex::GLTFVertex;

pub struct GLTFMesh {
    positions: Vec<f32>,
    diffuse_uvs: Vec<f32>,
    lightmap_uvs: Vec<f32>,
    indices: Vec<u32>,
    material_indices: Vec<Option<u32>>,
    materials: Vec<GLTFMaterial>,
}

#[derive(Debug, Clone)]
pub enum GLTFMeshError {
    Load,
    NoScene,
    MultipleScenes,
    InconsistentDiffuseUVs,
    InconsistentLightmapUVs,
}

impl GLTFMesh {
    pub fn new(data: &[u8]) -> Result<Self, GLTFMeshError> {
        let (document, buffers, _) = ::gltf::import_slice(data).map_err(|_| GLTFMeshError::Load)?;

        let scenes: Vec<_> = document.scenes().collect();
        let scene = match scenes.len() {
            0 => return Err(GLTFMeshError::NoScene),
            1 => &scenes[0],
            _ => return Err(GLTFMeshError::MultipleScenes),
        };

        let materials: Vec<GLTFMaterial> = document
            .materials()
            .map(|material| {
                let base_color = material.pbr_metallic_roughness().base_color_factor();
                let color = [
                    (base_color[0].clamp(0.0, 1.0) * 255.0).round() as u8,
                    (base_color[1].clamp(0.0, 1.0) * 255.0).round() as u8,
                    (base_color[2].clamp(0.0, 1.0) * 255.0).round() as u8,
                    (base_color[3].clamp(0.0, 1.0) * 255.0).round() as u8,
                ];
                return GLTFMaterial {
                    name: material.name().unwrap_or("unnamed").to_string(),
                    color,
                };
            })
            .collect();

        let mut mesh = GLTFMesh {
            positions: Vec::new(),
            diffuse_uvs: Vec::new(),
            lightmap_uvs: Vec::new(),
            indices: Vec::new(),
            material_indices: Vec::new(),
            materials,
        };

        for node in scene.nodes() {
            process_node_recursive(&node, &buffers, Mat4::IDENTITY, &mut mesh)?;
        }

        let vertex_count = mesh.positions.len() / 3;

        if !mesh.diffuse_uvs.is_empty() && mesh.diffuse_uvs.len() != vertex_count * 2 {
            return Err(GLTFMeshError::InconsistentDiffuseUVs);
        }
        if !mesh.lightmap_uvs.is_empty() && mesh.lightmap_uvs.len() != vertex_count * 2 {
            return Err(GLTFMeshError::InconsistentLightmapUVs);
        }

        return Ok(mesh);
    }

    pub fn vertex_count(&self) -> usize {
        return self.indices.len();
    }

    pub fn materials(&self) -> &[GLTFMaterial] {
        return &self.materials;
    }

    #[cfg(test)]
    pub fn from_raw(positions: Vec<f32>, indices: Vec<u32>) -> Self {
        let vertex_count = positions.len() / 3;
        return Self {
            positions,
            diffuse_uvs: Vec::new(),
            lightmap_uvs: Vec::new(),
            indices,
            material_indices: vec![None; vertex_count],
            materials: Vec::new(),
        };
    }

    #[cfg(test)]
    pub fn from_raw_with_uvs(
        positions: Vec<f32>,
        diffuse_uvs: Vec<f32>,
        indices: Vec<u32>,
    ) -> Self {
        let vertex_count = positions.len() / 3;
        return Self {
            positions,
            diffuse_uvs,
            lightmap_uvs: Vec::new(),
            indices,
            material_indices: vec![None; vertex_count],
            materials: Vec::new(),
        };
    }

    pub fn vertex(&self, index: usize) -> GLTFVertex {
        let idx = self.indices[index] as usize;
        let pos_start = idx * 3;

        let position = Vec3::new(
            self.positions[pos_start],
            self.positions[pos_start + 1],
            self.positions[pos_start + 2],
        );

        let diffuse_uv = if self.diffuse_uvs.is_empty() {
            None
        } else {
            Some(Vec2::new(
                self.diffuse_uvs[idx * 2],
                self.diffuse_uvs[idx * 2 + 1],
            ))
        };

        let lightmap_uv = if self.lightmap_uvs.is_empty() {
            None
        } else {
            Some(Vec2::new(
                self.lightmap_uvs[idx * 2],
                self.lightmap_uvs[idx * 2 + 1],
            ))
        };

        let material_ix = self.material_indices[idx];

        return GLTFVertex {
            position,
            diffuse_uv,
            lightmap_uv,
            material_ix,
        };
    }

    pub fn model_vertices(&self) -> impl Iterator<Item = GLTFVertex> + '_ {
        return (0..self.vertex_count()).map(|i| self.vertex(i));
    }
}

fn process_node_recursive(
    node: &::gltf::Node,
    buffers: &[::gltf::buffer::Data],
    parent_transform: Mat4,
    mesh: &mut GLTFMesh,
) -> Result<(), GLTFMeshError> {
    let local = Mat4::from_cols_array_2d(&node.transform().matrix());
    let global = parent_transform * local;

    if let Some(node_mesh) = node.mesh() {
        for primitive in node_mesh.primitives() {
            let material_ix = primitive.material().index().map(|i| i as u32);

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let vertex_offset = (mesh.positions.len() / 3) as u32;

            let primitive_vertex_count;
            if let Some(pos_iter) = reader.read_positions() {
                let pos_vec: Vec<_> = pos_iter.collect();
                primitive_vertex_count = pos_vec.len();
                for pos in pos_vec {
                    let p = global.transform_point3(Vec3::from_array(pos));
                    mesh.positions.extend_from_slice(&[p.x, p.y, p.z]);
                }
            } else {
                continue;
            }

            for _ in 0..primitive_vertex_count {
                mesh.material_indices.push(material_ix);
            }

            if let Some(tex_iter) = reader.read_tex_coords(0) {
                for tex in tex_iter.into_f32() {
                    mesh.diffuse_uvs.extend_from_slice(&tex);
                }
            }

            if let Some(tex_iter) = reader.read_tex_coords(1) {
                for tex in tex_iter.into_f32() {
                    mesh.lightmap_uvs.extend_from_slice(&tex);
                }
            }

            if let Some(idx_iter) = reader.read_indices() {
                for idx in idx_iter.into_u32() {
                    mesh.indices.push(idx + vertex_offset);
                }
            } else {
                for idx in 0..primitive_vertex_count as u32 {
                    mesh.indices.push(idx + vertex_offset);
                }
            }
        }
    }

    for child in node.children() {
        process_node_recursive(&child, buffers, global, mesh)?;
    }

    return Ok(());
}
