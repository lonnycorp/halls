use std::collections::HashMap;

use url::Url;

use crate::color::Color;
use crate::graphics::pipeline::level::{PipelineLevelBindGroupTexture, TEXTURE_BUCKETS};
use crate::graphics::storage::{
    MaterialIndexStorageBuffer, MaterialIndexStorageBufferData, MaterialTextureRef,
};
use crate::graphics::texture::TextureArray;
use crate::level::fetch::fetch;

use super::manifest::LevelManifestSurface;

const STATIC_ANIMATION_SPEED: f32 = 0.0;

pub struct MaterialData {
    pub texture_bind_group: PipelineLevelBindGroupTexture,
    pub material_index: MaterialIndexStorageBuffer,
    pub lightmap_material_id: u32,
}

#[derive(Debug)]
pub enum LevelMaterialLoadImageError {
    URLJoin,
    Fetch,
}

#[derive(Debug)]
pub enum LevelMaterialLoadError {
    Image,
    TextureArrayWrite,
    MaterialIndex,
}

fn find_texture_bucket(w: u32, h: u32) -> Option<usize> {
    return TEXTURE_BUCKETS
        .iter()
        .position(|b| b.width == w && b.height == h);
}

fn load_image(base_url: &Url, href: &str) -> Result<Vec<u8>, LevelMaterialLoadImageError> {
    let url = base_url
        .join(href)
        .map_err(|_| LevelMaterialLoadImageError::URLJoin)?;
    return fetch(&url).map_err(|_| LevelMaterialLoadImageError::Fetch);
}

fn surface_frame_refs_load(
    queue: &wgpu::Queue,
    base_url: &Url,
    frame_paths: &[String],
    diffuse: &mut [TextureArray],
    next_free: &mut [usize; TEXTURE_BUCKETS.len()],
    texture_ref_cache: &mut HashMap<String, MaterialTextureRef>,
) -> Result<Vec<MaterialTextureRef>, LevelMaterialLoadError> {
    let mut frames: Vec<MaterialTextureRef> = Vec::with_capacity(frame_paths.len());

    for frame_path in frame_paths {
        if let Some(&cached_ref) = texture_ref_cache.get(frame_path) {
            frames.push(cached_ref);
            continue;
        }

        let frame_data =
            load_image(base_url, frame_path).map_err(|_| LevelMaterialLoadError::Image)?;
        let img = image::load_from_memory(&frame_data)
            .map_err(|_| LevelMaterialLoadError::Image)?
            .to_rgba8();
        let (w, h) = img.dimensions();

        let bucket_ix = find_texture_bucket(w, h).ok_or(LevelMaterialLoadError::Image)?;
        let layer = next_free[bucket_ix];
        diffuse[bucket_ix]
            .write(queue, layer, &img)
            .map_err(|_| LevelMaterialLoadError::TextureArrayWrite)?;
        next_free[bucket_ix] += 1;

        let texture_ref = MaterialTextureRef {
            bucket: bucket_ix as u16,
            layer: layer as u16,
        };
        texture_ref_cache.insert(frame_path.clone(), texture_ref);
        frames.push(texture_ref);
    }

    return Ok(frames);
}

impl MaterialData {
    pub fn load(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        base_url: &Url,
        surfaces: &[Option<&LevelManifestSurface>],
        lightmap_path: Option<&str>,
    ) -> Result<Self, LevelMaterialLoadError> {
        let mut diffuse =
            TEXTURE_BUCKETS.map(|b| TextureArray::new(device, (b.width, b.height), b.layers));
        let mut material_index_data = MaterialIndexStorageBufferData::new();
        let mut next_free: [usize; TEXTURE_BUCKETS.len()] = [0; TEXTURE_BUCKETS.len()];
        let mut texture_ref_cache: HashMap<String, MaterialTextureRef> = HashMap::new();

        for (ix, surface) in surfaces.iter().enumerate() {
            let surface = match surface {
                Some(surface) => *surface,
                None => continue,
            };
            match surface {
                LevelManifestSurface::TextureSingle {
                    frame,
                    color,
                    unlit,
                    ..
                } => {
                    let color = (*color).unwrap_or(Color::WHITE);
                    let unlit = (*unlit).unwrap_or(false);
                    let frame_paths = std::slice::from_ref::<String>(frame);
                    let frames = surface_frame_refs_load(
                        queue,
                        base_url,
                        frame_paths,
                        &mut diffuse,
                        &mut next_free,
                        &mut texture_ref_cache,
                    )?;

                    material_index_data
                        .write(ix as u32, STATIC_ANIMATION_SPEED, &frames, color, unlit)
                        .map_err(|_| LevelMaterialLoadError::MaterialIndex)?;
                }
                LevelManifestSurface::TextureMulti {
                    frames: frame_paths,
                    animation_speed,
                    color,
                    unlit,
                    ..
                } => {
                    let color = (*color).unwrap_or(Color::WHITE);
                    let unlit = (*unlit).unwrap_or(false);
                    let frames = surface_frame_refs_load(
                        queue,
                        base_url,
                        frame_paths,
                        &mut diffuse,
                        &mut next_free,
                        &mut texture_ref_cache,
                    )?;

                    material_index_data
                        .write(ix as u32, *animation_speed, &frames, color, unlit)
                        .map_err(|_| LevelMaterialLoadError::MaterialIndex)?;
                }
                LevelManifestSurface::Untextured { color, unlit, .. } => {
                    let unlit = (*unlit).unwrap_or(false);
                    material_index_data
                        .write(ix as u32, STATIC_ANIMATION_SPEED, &[], *color, unlit)
                        .map_err(|_| LevelMaterialLoadError::MaterialIndex)?;
                }
                LevelManifestSurface::Invisible { .. } => {}
            }
        }

        let lightmap_material_id = surfaces.len() as u32;
        match lightmap_path {
            Some(path) => {
                let frame_path = path.to_string();
                let frame_paths = std::slice::from_ref::<String>(&frame_path);
                let frames = surface_frame_refs_load(
                    queue,
                    base_url,
                    frame_paths,
                    &mut diffuse,
                    &mut next_free,
                    &mut texture_ref_cache,
                )?;
                material_index_data
                    .write(
                        lightmap_material_id,
                        STATIC_ANIMATION_SPEED,
                        &frames,
                        Color::WHITE,
                        false,
                    )
                    .map_err(|_| LevelMaterialLoadError::MaterialIndex)?;
            }
            None => {
                material_index_data
                    .write(
                        lightmap_material_id,
                        STATIC_ANIMATION_SPEED,
                        &[],
                        Color::WHITE,
                        false,
                    )
                    .map_err(|_| LevelMaterialLoadError::MaterialIndex)?;
            }
        }

        let texture_bind_group = PipelineLevelBindGroupTexture::new(device, &diffuse);
        let material_index = MaterialIndexStorageBuffer::new(device);
        material_index.write(queue, &material_index_data);

        return Ok(Self {
            texture_bind_group,
            material_index,
            lightmap_material_id,
        });
    }
}
