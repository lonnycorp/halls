use serde::Deserialize;
use std::collections::HashMap;

const MANIFEST_VERSION: &str = "coco";
const MAX_PORTALS: usize = 4;

fn default_spawn() -> [f32; 3] {
    return [0.0, 0.0, 0.0];
}

fn default_material_tint() -> [u8; 3] {
    return [255, 255, 255];
}

#[derive(Debug, Deserialize)]
struct LevelManifestPortalRaw {
    pub model: String,
    pub link: String,
}

#[derive(Debug)]
pub struct LevelManifestPortal {
    pub model: String,
    pub link: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LevelManifestMaterial {
    Static {
        image: String,
        #[serde(default = "default_material_tint")]
        tint: [u8; 3],
    },
    Animated {
        images: Vec<String>,
        animation_speed: f32,
        #[serde(default = "default_material_tint")]
        tint: [u8; 3],
    },
}

impl LevelManifestMaterial {
    pub fn frame_data(&self) -> (&[String], f32) {
        match *self {
            LevelManifestMaterial::Static { ref image, .. } => {
                return (std::slice::from_ref(image), 0.0);
            }
            LevelManifestMaterial::Animated {
                ref images,
                animation_speed,
                ..
            } => return (images, animation_speed),
        }
    }

    pub fn tint(&self) -> [u8; 3] {
        match *self {
            LevelManifestMaterial::Static { tint, .. } => {
                return tint;
            }
            LevelManifestMaterial::Animated { tint, .. } => {
                return tint;
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LevelManifestLevel {
    pub model: String,
    pub collider: Option<String>,
    pub lightmap: Option<String>,
    pub track: Option<String>,
    #[serde(default = "default_spawn")]
    pub spawn: [f32; 3],
    #[serde(default)]
    pub material: HashMap<String, LevelManifestMaterial>,
}

#[derive(Debug, Deserialize)]
pub struct LevelManifestMeta {
    pub name: String,
    pub author: Option<String>,
    pub track: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LevelManifestRaw {
    #[serde(rename = "_version")]
    version: String,
    meta: LevelManifestMeta,
    level: LevelManifestLevel,
    #[serde(default)]
    portal: HashMap<String, LevelManifestPortalRaw>,
}

#[derive(Debug)]
pub struct LevelManifest {
    pub meta: LevelManifestMeta,
    pub level: LevelManifestLevel,
    pub portal: HashMap<String, LevelManifestPortal>,
}

#[derive(Debug, Clone)]
pub enum LevelManifestError {
    Load,
    TooManyPortals,
    InvalidVersion,
}

impl LevelManifest {
    pub fn load(data: &[u8]) -> Result<Self, LevelManifestError> {
        let contents = std::str::from_utf8(data).map_err(|_| LevelManifestError::Load)?;

        let raw: LevelManifestRaw =
            serde_json::from_str(contents).map_err(|_| LevelManifestError::Load)?;

        if raw.version != MANIFEST_VERSION {
            return Err(LevelManifestError::InvalidVersion);
        }

        if raw.portal.len() > MAX_PORTALS {
            return Err(LevelManifestError::TooManyPortals);
        }

        let mut portal = HashMap::new();

        for (name, entry) in raw.portal {
            portal.insert(
                name,
                LevelManifestPortal {
                    model: entry.model,
                    link: entry.link,
                },
            );
        }

        return Ok(Self {
            meta: raw.meta,
            level: raw.level,
            portal,
        });
    }
}
