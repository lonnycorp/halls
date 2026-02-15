use glam::Vec3;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

use crate::color::Color;
use crate::level::fetch::fetch;

const MANIFEST_VERSION: &str = "coco";
const MAX_PORTALS: usize = 4;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum LevelManifestColliderType {
    Wall,
    Ladder,
    Null,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum LevelManifestSurface {
    TextureSingle {
        collider: Option<LevelManifestColliderType>,
        frame: String,
        color: Option<Color>,
        unlit: Option<bool>,
    },
    TextureMulti {
        collider: Option<LevelManifestColliderType>,
        frames: Vec<String>,
        animation_speed: f32,
        color: Option<Color>,
        unlit: Option<bool>,
    },
    Untextured {
        collider: Option<LevelManifestColliderType>,
        color: Color,
        unlit: Option<bool>,
    },
    Invisible {
        collider: Option<LevelManifestColliderType>,
    },
}

#[derive(Debug, Deserialize)]
pub struct LevelManifestPortal {
    mesh: String,
    link: String,
}

impl LevelManifestPortal {
    pub fn mesh(&self) -> &str {
        return &self.mesh;
    }

    pub fn link_href(&self) -> &str {
        return &self.link;
    }
}

#[derive(Debug, Deserialize)]
pub struct LevelManifestLevel {
    mesh: String,
    lightmap: Option<String>,
    track: Option<String>,
    spawn: Option<Vec3>,
    surface: HashMap<String, LevelManifestSurface>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelManifestMeta {
    name: String,
    author: Option<String>,
    track: Option<String>,
}

impl LevelManifestMeta {
    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn author(&self) -> Option<&str> {
        return self.author.as_deref();
    }

    pub fn track(&self) -> Option<&str> {
        return self.track.as_deref();
    }
}

#[derive(Debug, Deserialize)]
pub struct LevelManifest {
    #[serde(rename = "_version")]
    version: String,
    meta: LevelManifestMeta,
    level: LevelManifestLevel,
    portal: HashMap<String, LevelManifestPortal>,
}

#[derive(Debug)]
pub enum LevelManifestLoadError {
    Fetch,
    FromBytes,
}

#[derive(Debug)]
pub enum LevelManifestFromBytesError {
    UTF8,
    Decode,
    TooManyPortals,
    InvalidVersion,
    EmptySurfaceFrameArray,
}

impl LevelManifest {
    pub fn meta(&self) -> &LevelManifestMeta {
        return &self.meta;
    }

    pub fn level(&self) -> &LevelManifestLevel {
        return &self.level;
    }

    #[cfg(test)]
    pub fn portal(&self, name: &str) -> Option<&LevelManifestPortal> {
        return self.portal.get(name);
    }

    pub fn portal_iter(&self) -> impl Iterator<Item = (&String, &LevelManifestPortal)> {
        return self.portal.iter();
    }

    #[cfg(test)]
    pub fn portal_len(&self) -> usize {
        return self.portal.len();
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, LevelManifestFromBytesError> {
        let contents = std::str::from_utf8(data).map_err(|_| LevelManifestFromBytesError::UTF8)?;

        let manifest: LevelManifest =
            serde_json::from_str(contents).map_err(|_| LevelManifestFromBytesError::Decode)?;

        if manifest.version != MANIFEST_VERSION {
            return Err(LevelManifestFromBytesError::InvalidVersion);
        }

        if manifest.portal.len() > MAX_PORTALS {
            return Err(LevelManifestFromBytesError::TooManyPortals);
        }

        for (_, surface) in manifest.level.surface_iter() {
            if let LevelManifestSurface::TextureMulti { frames, .. } = surface {
                if frames.is_empty() {
                    return Err(LevelManifestFromBytesError::EmptySurfaceFrameArray);
                }
            }
        }

        return Ok(manifest);
    }

    pub fn load(url: &Url) -> Result<Self, LevelManifestLoadError> {
        let data = fetch(url).map_err(|_| LevelManifestLoadError::Fetch)?;
        return Self::from_bytes(&data).map_err(|_| LevelManifestLoadError::FromBytes);
    }
}

impl LevelManifestLevel {
    pub fn mesh(&self) -> &str {
        return &self.mesh;
    }

    pub fn lightmap(&self) -> Option<&str> {
        return self.lightmap.as_deref();
    }

    pub fn track(&self) -> Option<&str> {
        return self.track.as_deref();
    }

    pub fn spawn(&self) -> Vec3 {
        return self.spawn.unwrap_or(Vec3::ZERO);
    }

    pub fn surface(&self, name: &str) -> Option<&LevelManifestSurface> {
        return self.surface.get(name);
    }

    pub fn surface_iter(&self) -> impl Iterator<Item = (&String, &LevelManifestSurface)> {
        return self.surface.iter();
    }
}
