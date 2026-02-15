use glam::Vec3;
use std::collections::HashMap;
use url::Url;

use parry3d::shape::TriMesh;

use crate::audio::TrackData;
use crate::graphics::model::Model;

use super::manifest::LevelManifestMeta;
use super::material::MaterialData;
use super::portal::LevelPortal;

pub struct LevelColliderData {
    pub wall: TriMesh,
    pub ladder: TriMesh,
}

pub struct LevelState {
    pub url: Url,
    pub meta: LevelManifestMeta,
    pub spawn: Vec3,
    pub collider_data: LevelColliderData,
    pub model: Model,
    pub material_data: MaterialData,
    pub portals: HashMap<String, LevelPortal>,
    pub track: Option<TrackData>,
}
