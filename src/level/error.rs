use super::fetch::FetchError;
use crate::audio::TrackDataError;
use crate::gltf::GLTFMeshError;

use super::manifest::LevelManifestError;
use super::portal::PortalError;

#[derive(Debug, Clone)]
pub enum LevelLoadError {
    Fetch {
        asset: String,
        error: FetchError,
    },
    Manifest(LevelManifestError),
    Mesh {
        asset: String,
        error: GLTFMeshError,
    },
    ImageLoadFailed {
        asset: String,
    },
    Portal {
        asset: String,
        error: PortalError,
    },
    Track {
        asset: String,
        error: TrackDataError,
    },
    URLInvalid(String),
    MissingMaterial(String),
    InvalidMaterialTextureDimensions(String),
    TextureBucketExhausted(String),
    TextureIndexFull(String),
    MaterialIndexFull(String),
    NoMaterials,
    TooManyMaterials,
}

impl std::fmt::Display for LevelLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LevelLoadError::Fetch { asset, error } => {
                return write!(f, "failed to fetch {asset}: {error:?}")
            }
            LevelLoadError::Manifest(error) => return write!(f, "invalid manifest: {error:?}"),
            LevelLoadError::Mesh { asset, error } => {
                return write!(f, "failed to load mesh {asset}: {error:?}")
            }
            LevelLoadError::ImageLoadFailed { asset } => {
                return write!(f, "failed to load image {asset}")
            }
            LevelLoadError::Portal { asset, error } => {
                return write!(f, "invalid portal {asset}: {error:?}")
            }
            LevelLoadError::Track { asset, error } => {
                return write!(f, "failed to load track {asset}: {error:?}")
            }
            LevelLoadError::URLInvalid(href) => return write!(f, "invalid url resolution: {href}"),
            LevelLoadError::MissingMaterial(material) => {
                return write!(f, "material {material} not found in manifest")
            }
            LevelLoadError::InvalidMaterialTextureDimensions(material) => {
                return write!(f, "invalid texture dimensions for material {material}")
            }
            LevelLoadError::TextureBucketExhausted(material) => {
                return write!(f, "texture bucket exhausted for material {material}")
            }
            LevelLoadError::TextureIndexFull(material) => {
                return write!(f, "texture index full for material {material}")
            }
            LevelLoadError::MaterialIndexFull(material) => {
                return write!(f, "material index full for material {material}")
            }
            LevelLoadError::NoMaterials => return write!(f, "no materials"),
            LevelLoadError::TooManyMaterials => return write!(f, "too many materials (max 256)"),
        }
    }
}
