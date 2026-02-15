use glam::Vec3;
use std::collections::HashMap;
use url::Url;

use parry3d::math::{Isometry, Vector};
use parry3d::query::{cast_shapes, ShapeCastHit, ShapeCastOptions};
use parry3d::shape::Cuboid;

use crate::audio::TrackData;
use crate::gltf::{GLTFMesh, GLTFVertex};
use crate::graphics::model::{Model, ModelUploadError, ModelVertex};

use super::fetch::fetch;
use super::manifest::{
    LevelManifest, LevelManifestColliderType, LevelManifestMeta, LevelManifestSurface,
};
use super::material::MaterialData as LevelMaterialData;
use super::portal::LevelPortal;
use super::render::LevelRenderParams;
use super::state::{LevelColliderData, LevelState};
use super::trimesh::trimesh_from_vertices;

#[derive(Debug)]
pub enum LevelMeshLoadError {
    URLJoin,
    Fetch,
    GLTF,
}

#[derive(Debug)]
pub enum LevelTrackLoadError {
    URLJoin,
    Fetch,
    Decode,
}

#[derive(Debug)]
pub enum LevelLoadError {
    Manifest,
    Mesh,
    Material,
    Portal,
    Track,
    ModelUpload,
}

impl std::fmt::Display for LevelLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            LevelLoadError::Manifest => write!(f, "failed to load level manifest"),
            LevelLoadError::Mesh => write!(f, "failed to load level mesh"),
            LevelLoadError::Material => write!(f, "failed to load level materials"),
            LevelLoadError::Portal => write!(f, "failed to load level portals"),
            LevelLoadError::Track => write!(f, "failed to load level track"),
            LevelLoadError::ModelUpload => write!(f, "failed to upload level model"),
        };
    }
}

pub enum SurfaceKind {
    Wall,
    Ladder,
}

pub struct LevelHit {
    pub hit: ShapeCastHit,
    pub kind: SurfaceKind,
}

pub struct Level {
    pub state: LevelState,
}

impl Level {
    fn surface_collider(surface: &LevelManifestSurface) -> LevelManifestColliderType {
        let collider = match surface {
            LevelManifestSurface::TextureSingle { collider, .. } => collider,
            LevelManifestSurface::TextureMulti { collider, .. } => collider,
            LevelManifestSurface::Untextured { collider, .. } => collider,
            LevelManifestSurface::Invisible { collider } => collider,
        };

        return collider.unwrap_or(LevelManifestColliderType::Wall);
    }

    fn surface_index_build<'a>(
        manifest: &'a LevelManifest,
        mesh: &GLTFMesh,
    ) -> Vec<Option<&'a LevelManifestSurface>> {
        let mut mapped: Vec<Option<&LevelManifestSurface>> =
            Vec::with_capacity(mesh.materials().len());

        for material_name in mesh.materials() {
            let surface = match material_name {
                Some(name) => manifest.level().surface(name),
                None => None,
            };
            mapped.push(surface);
        }
        return mapped;
    }

    fn mesh_load(base_url: &Url, mesh_href: &str) -> Result<GLTFMesh, LevelMeshLoadError> {
        let mesh_url = base_url
            .join(mesh_href)
            .map_err(|_| LevelMeshLoadError::URLJoin)?;
        let mesh_data = fetch(&mesh_url).map_err(|_| LevelMeshLoadError::Fetch)?;
        return GLTFMesh::from_bytes(&mesh_data).map_err(|_| LevelMeshLoadError::GLTF);
    }

    fn model_build(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mesh: &GLTFMesh,
        surfaces: &[Option<&LevelManifestSurface>],
    ) -> Result<Model, ModelUploadError> {
        let mut vertices: Vec<ModelVertex> = Vec::new();
        for vertex in mesh.vertices() {
            let material_ix = match vertex.material_ix {
                Some(material_ix) => material_ix,
                None => continue,
            };
            let surface = match surfaces.get(material_ix as usize) {
                Some(Some(surface)) => surface,
                _ => continue,
            };
            if let LevelManifestSurface::Invisible { .. } = surface {
                continue;
            }
            vertices.push(vertex.to_model_vertex());
        }

        let mut model = Model::new(device, mesh.vertex_count());
        model.upload(queue, &vertices)?;
        return Ok(model);
    }

    fn track_load(base_url: &Url, track_href: &str) -> Result<TrackData, LevelTrackLoadError> {
        let track_url = base_url
            .join(track_href)
            .map_err(|_| LevelTrackLoadError::URLJoin)?;
        let track_data = fetch(&track_url).map_err(|_| LevelTrackLoadError::Fetch)?;
        return TrackData::new(&track_data, true).map_err(|_| LevelTrackLoadError::Decode);
    }

    fn collider_build(
        mesh: &GLTFMesh,
        surfaces: &[Option<&LevelManifestSurface>],
    ) -> LevelColliderData {
        let mut wall_vertices: Vec<GLTFVertex> = Vec::new();
        let mut ladder_vertices: Vec<GLTFVertex> = Vec::new();

        for vertex in mesh.vertices() {
            let material_ix = match vertex.material_ix {
                Some(material_ix) => material_ix,
                None => continue,
            };
            let surface = match surfaces.get(material_ix as usize) {
                Some(Some(surface)) => surface,
                _ => continue,
            };
            let collider = Self::surface_collider(surface);

            match collider {
                LevelManifestColliderType::Wall => {
                    wall_vertices.push(vertex);
                }
                LevelManifestColliderType::Ladder => {
                    ladder_vertices.push(vertex);
                }
                LevelManifestColliderType::Null => {}
            }
        }

        return LevelColliderData {
            wall: trimesh_from_vertices(wall_vertices.into_iter()),
            ladder: trimesh_from_vertices(ladder_vertices.into_iter()),
        };
    }

    pub fn url(&self) -> &Url {
        return &self.state.url;
    }

    pub fn meta(&self) -> &LevelManifestMeta {
        return &self.state.meta;
    }

    pub fn sweep(
        &self,
        pos: &Isometry<f32>,
        vel: &Vector<f32>,
        shape: &Cuboid,
        max_toi: f32,
    ) -> Option<LevelHit> {
        let wall_hit = cast_shapes(
            pos,
            vel,
            shape,
            &Isometry::identity(),
            &Vector::zeros(),
            &self.state.collider_data.wall,
            ShapeCastOptions::with_max_time_of_impact(max_toi),
        )
        .unwrap();

        let ladder_hit = cast_shapes(
            pos,
            vel,
            shape,
            &Isometry::identity(),
            &Vector::zeros(),
            &self.state.collider_data.ladder,
            ShapeCastOptions::with_max_time_of_impact(max_toi),
        )
        .unwrap();

        return match (wall_hit, ladder_hit) {
            (Some(wall), Some(ladder)) => {
                if wall.time_of_impact <= ladder.time_of_impact {
                    Some(LevelHit {
                        hit: ladder,
                        kind: SurfaceKind::Ladder,
                    })
                } else {
                    Some(LevelHit {
                        hit: wall,
                        kind: SurfaceKind::Wall,
                    })
                }
            }
            (Some(wall), None) => Some(LevelHit {
                hit: wall,
                kind: SurfaceKind::Wall,
            }),
            (None, Some(ladder)) => Some(LevelHit {
                hit: ladder,
                kind: SurfaceKind::Ladder,
            }),
            (None, None) => None,
        };
    }

    pub fn track(&self) -> Option<&TrackData> {
        return self.state.track.as_ref();
    }

    pub fn portal(&self, name: &str) -> Option<&LevelPortal> {
        return self.state.portals.get(name);
    }

    pub fn portals(&self) -> impl Iterator<Item = (&String, &LevelPortal)> {
        return self.state.portals.iter();
    }

    pub fn spawn_position(&self) -> Vec3 {
        return self.state.spawn;
    }

    pub fn render(&self, params: LevelRenderParams) {
        super::render::level_render(&self.state, params);
    }

    pub fn load(
        url: Url,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self, LevelLoadError> {
        let manifest = LevelManifest::load(&url).map_err(|_| LevelLoadError::Manifest)?;

        let level_mesh =
            Self::mesh_load(&url, manifest.level().mesh()).map_err(|_| LevelLoadError::Mesh)?;
        let surface_index = Self::surface_index_build(&manifest, &level_mesh);

        let material_data = LevelMaterialData::load(
            device,
            queue,
            &url,
            &surface_index,
            manifest.level().lightmap(),
        )
        .map_err(|_| LevelLoadError::Material)?;
        let model = Self::model_build(device, queue, &level_mesh, &surface_index)
            .map_err(|_| LevelLoadError::ModelUpload)?;

        let collider_data = Self::collider_build(&level_mesh, &surface_index);

        let mut portals = HashMap::new();
        for (name, manifest_portal) in manifest.portal_iter() {
            let portal = LevelPortal::load(
                &url,
                manifest_portal.mesh(),
                manifest_portal.link_href(),
                device,
                queue,
            )
            .map_err(|_| LevelLoadError::Portal)?;
            portals.insert(name.clone(), portal);
        }

        let track = match manifest.level().track() {
            Some(track_href) => {
                Some(Self::track_load(&url, track_href).map_err(|_| LevelLoadError::Track)?)
            }
            None => None,
        };

        return Ok(Self {
            state: LevelState {
                url,
                meta: manifest.meta().clone(),
                spawn: manifest.level().spawn(),
                collider_data,
                model,
                material_data,
                portals,
                track,
            },
        });
    }
}
