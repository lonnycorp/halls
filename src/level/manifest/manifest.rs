use serde::Deserialize;
use std::collections::HashMap;

const MANIFEST_VERSION: &str = "coco";
const MAX_PORTALS: usize = 4;

#[derive(Debug, Deserialize)]
struct LevelManifestPortalRaw {
    pub model: String,
    pub link: String,
    #[serde(default)]
    pub spawn: bool,
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
    },
    Animated {
        images: Vec<String>,
        animation_speed: f32,
    },
}

impl LevelManifestMaterial {
    pub fn frame_data(&self) -> (&[String], f32) {
        match self {
            LevelManifestMaterial::Static { image } => return (std::slice::from_ref(image), 0.0),
            LevelManifestMaterial::Animated {
                images,
                animation_speed,
            } => return (images, *animation_speed),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LevelManifestLevel {
    pub model: String,
    pub collider: Option<String>,
    pub lightmap: Option<String>,
    pub track: Option<String>,
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
    pub spawn: String,
}

#[derive(Debug, Clone)]
pub enum LevelManifestError {
    Load,
    TooManyPortals,
    InvalidVersion,
    NoSpawnPortal,
    MultipleSpawnPortals,
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
        let mut spawn: Option<String> = None;

        for (name, entry) in raw.portal {
            if entry.spawn {
                if spawn.is_some() {
                    return Err(LevelManifestError::MultipleSpawnPortals);
                }
                spawn = Some(name.clone());
            }

            portal.insert(
                name,
                LevelManifestPortal {
                    model: entry.model,
                    link: entry.link,
                },
            );
        }

        let spawn = spawn.ok_or(LevelManifestError::NoSpawnPortal)?;

        return Ok(Self {
            meta: raw.meta,
            level: raw.level,
            portal,
            spawn,
        });
    }
}
