use glam::{Mat4, Vec2, Vec3};

use super::vertex::GLTFVertex;
use crate::color::Color;

pub struct GLTFMesh {
    positions: Vec<f32>,
    diffuse_uvs: Vec<f32>,
    lightmap_uvs: Vec<f32>,
    colors: Vec<u8>,
    indices: Vec<u32>,
    material_indices: Vec<Option<u32>>,
    materials: Vec<Option<String>>,
}

#[derive(Debug)]
pub enum GLTFMeshError {
    GLTF,
    NoScene,
    MultipleScenes,
    InconsistentDiffuseUVs,
    InconsistentLightmapUVs,
    InconsistentColors,
}

fn node_process_recursive(
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

            if let Some(color_iter) = reader.read_colors(0) {
                for color in color_iter.into_rgba_f32() {
                    mesh.colors
                        .push((color[0].clamp(0.0, 1.0) * 255.0).round() as u8);
                    mesh.colors
                        .push((color[1].clamp(0.0, 1.0) * 255.0).round() as u8);
                    mesh.colors
                        .push((color[2].clamp(0.0, 1.0) * 255.0).round() as u8);
                    mesh.colors
                        .push((color[3].clamp(0.0, 1.0) * 255.0).round() as u8);
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
        node_process_recursive(&child, buffers, global, mesh)?;
    }

    return Ok(());
}

impl GLTFMesh {
    pub fn from_bytes(data: &[u8]) -> Result<Self, GLTFMeshError> {
        let (document, buffers, _) = ::gltf::import_slice(data).map_err(|_| GLTFMeshError::GLTF)?;

        let scenes: Vec<_> = document.scenes().collect();
        let scene = match scenes.len() {
            0 => return Err(GLTFMeshError::NoScene),
            1 => &scenes[0],
            _ => return Err(GLTFMeshError::MultipleScenes),
        };

        let materials: Vec<Option<String>> = document
            .materials()
            .map(|material| material.name().map(str::to_string))
            .collect();

        let mut mesh = GLTFMesh {
            positions: Vec::new(),
            diffuse_uvs: Vec::new(),
            lightmap_uvs: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
            material_indices: Vec::new(),
            materials,
        };

        for node in scene.nodes() {
            node_process_recursive(&node, &buffers, Mat4::IDENTITY, &mut mesh)?;
        }

        let vertex_count = mesh.positions.len() / 3;

        if !mesh.diffuse_uvs.is_empty() && mesh.diffuse_uvs.len() != vertex_count * 2 {
            return Err(GLTFMeshError::InconsistentDiffuseUVs);
        }
        if !mesh.lightmap_uvs.is_empty() && mesh.lightmap_uvs.len() != vertex_count * 2 {
            return Err(GLTFMeshError::InconsistentLightmapUVs);
        }
        if !mesh.colors.is_empty() && mesh.colors.len() != vertex_count * 4 {
            return Err(GLTFMeshError::InconsistentColors);
        }

        return Ok(mesh);
    }

    pub fn vertex_count(&self) -> usize {
        return self.indices.len();
    }

    pub fn materials(&self) -> &[Option<String>] {
        return &self.materials;
    }

    #[cfg(test)]
    pub fn new(
        positions: Vec<f32>,
        indices: Vec<u32>,
        diffuse_uvs: Option<Vec<f32>>,
        lightmap_uvs: Option<Vec<f32>>,
        colors: Option<Vec<u8>>,
    ) -> Self {
        let vertex_count = positions.len() / 3;
        return Self {
            positions,
            diffuse_uvs: diffuse_uvs.unwrap_or_default(),
            lightmap_uvs: lightmap_uvs.unwrap_or_default(),
            colors: colors.unwrap_or_default(),
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
        let color = if self.colors.is_empty() {
            None
        } else {
            Some(Color::new(
                self.colors[idx * 4],
                self.colors[idx * 4 + 1],
                self.colors[idx * 4 + 2],
                self.colors[idx * 4 + 3],
            ))
        };

        return GLTFVertex {
            position,
            diffuse_uv,
            lightmap_uv,
            material_ix,
            color,
        };
    }

    pub fn vertices(&self) -> impl Iterator<Item = GLTFVertex> + '_ {
        return (0..self.vertex_count()).map(|i| self.vertex(i));
    }
}
