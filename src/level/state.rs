use std::collections::HashMap;
use url::Url;

use parry3d::shape::TriMesh;

use crate::audio::TrackData;
use crate::graphics::model::Model;
use crate::graphics::pipeline::level::PipelineLevelBindGroupTexture;
use crate::graphics::storage::{MaterialIndexStorageBuffer, TextureIndexStorageBuffer};

use super::manifest::LevelManifestMeta;
use super::portal::LevelPortal;

pub struct LevelState {
    pub url: Url,
    pub meta: LevelManifestMeta,
    pub spawn: String,
    pub trimesh: TriMesh,
    pub model: Model,
    pub texture_index: TextureIndexStorageBuffer,
    pub material_index: MaterialIndexStorageBuffer,
    pub texture_bind_group: PipelineLevelBindGroupTexture,
    pub lightmap_texture_id: u32,
    pub portals: HashMap<String, LevelPortal>,
    pub track: Option<TrackData>,
}
