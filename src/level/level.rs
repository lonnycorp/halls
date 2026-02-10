use std::collections::HashMap;
use url::Url;

use parry3d::math::{Isometry, Vector};
use parry3d::query::{cast_shapes, ShapeCastHit, ShapeCastOptions};
use parry3d::shape::{Cuboid, TriMesh};

use super::fetch::FetchedData;
use crate::audio::TrackData;
use crate::gltf::GLTFMesh;
use crate::graphics::model::{Model, ModelBuffer};
use crate::graphics::pipeline::level::{PipelineLevelBindGroupTexture, TEXTURE_BUCKETS};
use crate::graphics::storage::{
    MaterialIndexStorageBuffer, MaterialIndexStorageBufferData, TextureIndexStorageBuffer,
    TextureIndexStorageBufferData,
};
use crate::graphics::texture::TextureArray;

use super::error::LevelLoadError;
use super::manifest::{LevelManifest, LevelManifestMeta};
use super::portal::{LevelPortal, PortalSpec};
use super::render::LevelRenderContext;
use super::state::LevelState;

fn find_texture_bucket(w: u32, h: u32) -> Option<usize> {
    return TEXTURE_BUCKETS
        .iter()
        .position(|b| b.width == w && b.height == h);
}

const FALLBACK_TEXTURE_SIZE: u32 = 64;
const WHITE_RGBA: [u8; 4] = [255, 255, 255, 255];

pub struct Level {
    pub(super) state: LevelState,
}

impl Level {
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
    ) -> Option<ShapeCastHit> {
        return cast_shapes(
            pos,
            vel,
            shape,
            &Isometry::identity(),
            &Vector::zeros(),
            &self.state.trimesh,
            ShapeCastOptions::with_max_time_of_impact(max_toi),
        )
        .unwrap();
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

    pub fn spawn(&self) -> &LevelPortal {
        return &self.state.portals[&self.state.spawn];
    }

    pub fn render(&self, ctx: LevelRenderContext) {
        super::render::level_render(&self.state, ctx);
    }

    pub fn new(
        url: Url,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self, LevelLoadError> {
        let manifest_file = FetchedData::new(&url).map_err(|e| LevelLoadError::Fetch {
            asset: url.as_str().to_string(),
            error: e,
        })?;
        let manifest =
            LevelManifest::load(manifest_file.data()).map_err(LevelLoadError::Manifest)?;

        let base_url = url.join(".").unwrap();

        let model_url = base_url
            .join(&manifest.level.model)
            .map_err(|_| LevelLoadError::URLInvalid(manifest.level.model.clone()))?;
        let model_file = FetchedData::new(&model_url).map_err(|e| LevelLoadError::Fetch {
            asset: manifest.level.model.clone(),
            error: e,
        })?;
        let mesh = GLTFMesh::new(model_file.data()).map_err(|e| LevelLoadError::Mesh {
            asset: manifest.level.model.clone(),
            error: e,
        })?;

        let mut buffer = ModelBuffer::new();
        for v in mesh.model_vertices() {
            v.write_to_model_buffer(&mut buffer);
        }
        let mut model = Model::new(device, mesh.vertex_count());
        model.upload(queue, &buffer);

        let trimesh = match &manifest.level.collider {
            Some(collider_path) => {
                let collider_url = base_url
                    .join(collider_path)
                    .map_err(|_| LevelLoadError::URLInvalid(collider_path.clone()))?;
                let collider_file =
                    FetchedData::new(&collider_url).map_err(|e| LevelLoadError::Fetch {
                        asset: collider_path.clone(),
                        error: e,
                    })?;
                let collider_mesh =
                    GLTFMesh::new(collider_file.data()).map_err(|e| LevelLoadError::Mesh {
                        asset: collider_path.clone(),
                        error: e,
                    })?;
                TriMesh::from(&collider_mesh)
            }
            None => TriMesh::from(&mesh),
        };

        let diffuse =
            TEXTURE_BUCKETS.map(|b| TextureArray::new(device, (b.width, b.height), b.layers));
        let mut texture_index_data = TextureIndexStorageBufferData::new();
        let mut material_index_data = MaterialIndexStorageBufferData::new();
        let mut next_free: [usize; TEXTURE_BUCKETS.len()] = [0; TEXTURE_BUCKETS.len()];
        let mut path_to_texture_id: HashMap<String, u32> = HashMap::new();
        let fallback_bucket_ix =
            find_texture_bucket(FALLBACK_TEXTURE_SIZE, FALLBACK_TEXTURE_SIZE).unwrap();

        if mesh.materials().is_empty() {
            return Err(LevelLoadError::NoMaterials);
        }

        if mesh.materials().len() > 256 {
            return Err(LevelLoadError::TooManyMaterials);
        }

        for (ix, material_info) in mesh.materials().iter().enumerate() {
            if let Some(material) = manifest.level.material.get(&material_info.name) {
                let (frame_paths, animation_speed) = material.frame_data();
                let mut frames: Vec<u32> = Vec::with_capacity(frame_paths.len());

                for image_path in frame_paths {
                    if let Some(&cached_id) = path_to_texture_id.get(image_path) {
                        frames.push(cached_id);
                        continue;
                    }

                    let material_url = base_url
                        .join(image_path)
                        .map_err(|_| LevelLoadError::URLInvalid(image_path.to_string()))?;
                    let material_file =
                        FetchedData::new(&material_url).map_err(|e| LevelLoadError::Fetch {
                            asset: image_path.to_string(),
                            error: e,
                        })?;
                    let img = image::load_from_memory(material_file.data())
                        .map_err(|_| LevelLoadError::ImageLoadFailed {
                            asset: image_path.to_string(),
                        })?
                        .to_rgba8();
                    let (w, h) = img.dimensions();

                    let bucket_ix = find_texture_bucket(w, h).ok_or_else(|| {
                        LevelLoadError::InvalidMaterialTextureDimensions(material_info.name.clone())
                    })?;

                    let layer = next_free[bucket_ix];
                    if layer >= TEXTURE_BUCKETS[bucket_ix].layers {
                        return Err(LevelLoadError::TextureBucketExhausted(
                            material_info.name.clone(),
                        ));
                    }

                    diffuse[bucket_ix].write_texture(queue, layer, &img);
                    next_free[bucket_ix] += 1;

                    let texture_id = texture_index_data
                        .write(bucket_ix as u32, layer as u32)
                        .map_err(|_| {
                            LevelLoadError::TextureIndexFull(material_info.name.clone())
                        })?;
                    path_to_texture_id.insert(image_path.to_string(), texture_id);
                    frames.push(texture_id);
                }

                material_index_data
                    .write(ix, animation_speed, &frames)
                    .map_err(|_| LevelLoadError::MaterialIndexFull(material_info.name.clone()))?;
                continue;
            }

            let img = image::RgbaImage::from_pixel(
                FALLBACK_TEXTURE_SIZE,
                FALLBACK_TEXTURE_SIZE,
                image::Rgba(material_info.color),
            );
            let layer = next_free[fallback_bucket_ix];
            if layer >= TEXTURE_BUCKETS[fallback_bucket_ix].layers {
                return Err(LevelLoadError::TextureBucketExhausted(
                    material_info.name.clone(),
                ));
            }

            diffuse[fallback_bucket_ix].write_texture(queue, layer, &img);
            next_free[fallback_bucket_ix] += 1;

            let texture_id = texture_index_data
                .write(fallback_bucket_ix as u32, layer as u32)
                .map_err(|_| LevelLoadError::TextureIndexFull(material_info.name.clone()))?;
            material_index_data
                .write(ix, 0.0, &[texture_id])
                .map_err(|_| LevelLoadError::MaterialIndexFull(material_info.name.clone()))?;
        }

        let lightmap_texture_id = match &manifest.level.lightmap {
            Some(lightmap_path) => {
                let lightmap_url = base_url
                    .join(lightmap_path)
                    .map_err(|_| LevelLoadError::URLInvalid(lightmap_path.clone()))?;
                let lightmap_file =
                    FetchedData::new(&lightmap_url).map_err(|e| LevelLoadError::Fetch {
                        asset: lightmap_path.clone(),
                        error: e,
                    })?;
                let img = image::load_from_memory(lightmap_file.data())
                    .map_err(|_| LevelLoadError::ImageLoadFailed {
                        asset: lightmap_path.clone(),
                    })?
                    .to_rgba8();
                let (w, h) = img.dimensions();
                let bucket_ix = find_texture_bucket(w, h).ok_or_else(|| {
                    LevelLoadError::InvalidMaterialTextureDimensions(lightmap_path.clone())
                })?;
                let layer = next_free[bucket_ix];
                if layer >= TEXTURE_BUCKETS[bucket_ix].layers {
                    return Err(LevelLoadError::TextureBucketExhausted(
                        lightmap_path.clone(),
                    ));
                }
                diffuse[bucket_ix].write_texture(queue, layer, &img);
                texture_index_data
                    .write(bucket_ix as u32, layer as u32)
                    .map_err(|_| LevelLoadError::TextureIndexFull(lightmap_path.clone()))?
            }
            None => {
                let img = image::RgbaImage::from_pixel(
                    FALLBACK_TEXTURE_SIZE,
                    FALLBACK_TEXTURE_SIZE,
                    image::Rgba(WHITE_RGBA),
                );
                let layer = next_free[fallback_bucket_ix];
                if layer >= TEXTURE_BUCKETS[fallback_bucket_ix].layers {
                    return Err(LevelLoadError::TextureBucketExhausted(
                        "lightmap".to_string(),
                    ));
                }
                diffuse[fallback_bucket_ix].write_texture(queue, layer, &img);
                texture_index_data
                    .write(fallback_bucket_ix as u32, layer as u32)
                    .map_err(|_| LevelLoadError::TextureIndexFull("lightmap".to_string()))?
            }
        };

        let texture_index = TextureIndexStorageBuffer::new(device);
        texture_index.write(queue, &texture_index_data);
        let material_index = MaterialIndexStorageBuffer::new(device);
        material_index.write(queue, &material_index_data);

        let texture_bind_group = PipelineLevelBindGroupTexture::new(device, &diffuse);

        let mut portals = HashMap::new();
        for (name, manifest_portal) in &manifest.portal {
            let model_url = base_url
                .join(&manifest_portal.model)
                .map_err(|_| LevelLoadError::URLInvalid(manifest_portal.model.clone()))?;
            let model_file = FetchedData::new(&model_url).map_err(|e| LevelLoadError::Fetch {
                asset: manifest_portal.model.clone(),
                error: e,
            })?;
            let portal_mesh =
                GLTFMesh::new(model_file.data()).map_err(|e| LevelLoadError::Mesh {
                    asset: manifest_portal.model.clone(),
                    error: e,
                })?;

            let link = base_url
                .join(&manifest_portal.link)
                .map_err(|_| LevelLoadError::URLInvalid(manifest_portal.link.clone()))?;
            let spec = PortalSpec::from_gltf(&portal_mesh).map_err(|e| LevelLoadError::Portal {
                asset: manifest_portal.model.clone(),
                error: e,
            })?;
            let portal = LevelPortal::new(name.clone(), &spec, link);
            portals.insert(name.clone(), portal);
        }

        let track = match &manifest.level.track {
            Some(track_path) => {
                let track_url = base_url
                    .join(track_path)
                    .map_err(|_| LevelLoadError::URLInvalid(track_path.clone()))?;
                let track_file =
                    FetchedData::new(&track_url).map_err(|e| LevelLoadError::Fetch {
                        asset: track_path.clone(),
                        error: e,
                    })?;
                Some(TrackData::new(track_file.data(), true).map_err(|e| {
                    LevelLoadError::Track {
                        asset: track_path.clone(),
                        error: e,
                    }
                })?)
            }
            None => None,
        };

        return Ok(Self {
            state: LevelState {
                url,
                meta: manifest.meta,
                spawn: manifest.spawn,
                trimesh,
                model,
                texture_index,
                material_index,
                texture_bind_group,
                lightmap_texture_id,
                portals,
                track,
            },
        });
    }
}
