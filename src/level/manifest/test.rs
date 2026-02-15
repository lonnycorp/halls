use super::manifest::{LevelManifestColliderType, LevelManifestSurface};
use super::*;

fn load_manifest_bytes(data: &[u8]) -> Result<LevelManifest, LevelManifestFromBytesError> {
    return LevelManifest::from_bytes(data);
}

fn load_manifest_json(json: &str) -> Result<LevelManifest, LevelManifestFromBytesError> {
    return load_manifest_bytes(json.as_bytes());
}

#[test]
fn test_valid_manifest_parsing() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level",
            "author": "Test Author",
            "track": "Test Track"
        },
        "level": {
            "mesh": "level.glb",
            "spawn": [1.0, 2.0, 3.0],
            "surface": {}
        },
        "portal": {
            "portal_a": {
                "mesh": "portal_a.glb",
                "link": "other.json#portal_b"
            }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert_eq!(manifest.meta().name(), "Test Level");
    assert_eq!(manifest.meta().author(), Some("Test Author"));
    assert_eq!(manifest.meta().track(), Some("Test Track"));
    assert_eq!(manifest.portal_len(), 1);
    assert!(manifest.portal("portal_a").is_some());
    assert_eq!(manifest.level().spawn(), glam::Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_valid_manifest_without_optional_meta_fields() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level"
        },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {}
        },
        "portal": {
            "p1": { "mesh": "p1.glb", "link": "a.json#x" }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert_eq!(manifest.meta().name(), "Test Level");
    assert!(manifest.meta().author().is_none());
    assert!(manifest.meta().track().is_none());
}

#[test]
fn test_manifest_without_surface_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level"
        },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0]
        },
        "portal": {
            "p1": { "mesh": "p1.glb", "link": "a.json#x" }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestFromBytesError::Decode)));
}

#[test]
fn test_manifest_without_portals_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {}
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestFromBytesError::Decode)));
}

#[test]
fn test_old_level_model_and_collider_fields_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "collider": "level.glb"
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestFromBytesError::Decode)));
}

#[test]
fn test_old_portal_model_field_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {}
        },
        "portal": {
            "p1": { "model": "p1.glb", "link": "a.json#x" }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestFromBytesError::Decode)));
}

#[test]
fn test_invalid_json_returns_decode_error() {
    let result = load_manifest_bytes(b"{ invalid json }");
    assert!(matches!(result, Err(LevelManifestFromBytesError::Decode)));
}

#[test]
fn test_invalid_utf8_returns_utf8_error() {
    let result = load_manifest_bytes(&[0xff, 0xfe, 0xfd]);
    assert!(matches!(result, Err(LevelManifestFromBytesError::UTF8)));
}

#[test]
fn test_too_many_portals_returns_error() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {}
        },
        "portal": {
            "p1": { "mesh": "p1.glb", "link": "a.json#x" },
            "p2": { "mesh": "p2.glb", "link": "a.json#x" },
            "p3": { "mesh": "p3.glb", "link": "a.json#x" },
            "p4": { "mesh": "p4.glb", "link": "a.json#x" },
            "p5": { "mesh": "p5.glb", "link": "a.json#x" }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(
        result,
        Err(LevelManifestFromBytesError::TooManyPortals)
    ));
}

#[test]
fn test_invalid_version_returns_error() {
    let json = r#"{
        "_version": "wrong",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {}
        },
        "portal": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(
        result,
        Err(LevelManifestFromBytesError::InvalidVersion)
    ));
}

#[test]
fn test_spawn_missing_defaults_to_origin() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "surface": {}
        },
        "portal": {
            "portal_a": { "mesh": "p1.glb", "link": "a.json#x" }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();
    assert_eq!(manifest.level().spawn(), glam::Vec3::new(0.0, 0.0, 0.0));
}

#[test]
fn test_spawn_accepts_coordinates() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [-12.5, 0.0, 3.75]
            ,
            "surface": {}
        },
        "portal": {
            "portal_a": { "mesh": "p1.glb", "link": "a.json#x" },
            "portal_b": { "mesh": "p2.glb", "link": "b.json#x" }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();
    assert_eq!(manifest.level().spawn(), glam::Vec3::new(-12.5, 0.0, 3.75));
}

#[test]
fn test_surface_texture_single_parses_optional_fields() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {
                "wall": {
                    "collider": "Null",
                    "type": "TextureSingle",
                    "frame": "wall.png",
                    "color": [255, 255, 255, 255],
                    "unlit": true
                }
            }
        },
        "portal": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();

    let surface = manifest.level().surface("wall").unwrap();
    match surface {
        LevelManifestSurface::TextureSingle {
            frame,
            color,
            collider,
            unlit,
        } => {
            assert_eq!(frame, "wall.png");
            assert_eq!(*color, Some(crate::color::Color::WHITE));
            assert_eq!(*collider, Some(LevelManifestColliderType::Null));
            assert_eq!(*unlit, Some(true));
        }
        _ => panic!("expected single texture"),
    }
}

#[test]
fn test_surface_texture_single_missing_color_is_accepted() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {
                "wall": {
                    "collider": "Null",
                    "type": "TextureSingle",
                    "frame": "wall.png"
                }
            }
        },
        "portal": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();
    let surface = manifest.level().surface("wall").unwrap();

    match surface {
        LevelManifestSurface::TextureSingle {
            color,
            collider,
            unlit,
            ..
        } => {
            assert_eq!(*color, None);
            assert_eq!(*collider, Some(LevelManifestColliderType::Null));
            assert_eq!(*unlit, None);
        }
        _ => panic!("expected single texture"),
    }
}

#[test]
fn test_surface_missing_collider_is_accepted() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {
                "wall": {
                    "type": "TextureSingle",
                    "frame": "wall.png",
                    "color": [255, 255, 255, 255]
                }
            }
        },
        "portal": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();
    let surface = manifest.level().surface("wall").unwrap();

    match surface {
        LevelManifestSurface::TextureSingle {
            color,
            collider,
            unlit,
            ..
        } => {
            assert_eq!(*color, Some(crate::color::Color::WHITE));
            assert_eq!(*collider, None);
            assert_eq!(*unlit, None);
        }
        _ => panic!("expected single texture"),
    }
}

#[test]
fn test_surface_appearance_variants_parse() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {
                "wall": {
                    "collider": "Wall",
                    "type": "TextureSingle",
                    "frame": "wall.png",
                    "color": [1, 2, 3, 4],
                    "unlit": false
                },
                "lava": {
                    "collider": "Null",
                    "type": "TextureMulti",
                    "frames": ["lava_0.png", "lava_1.png"],
                    "animation_speed": 2.5,
                    "color": [10, 20, 30, 40],
                    "unlit": true
                },
                "paint": {
                    "collider": "Ladder",
                    "type": "Untextured",
                    "color": [7, 8, 9, 10],
                    "unlit": true
                },
                "ghost": {
                    "collider": "Null",
                    "type": "Invisible"
                }
            }
        },
        "portal": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();

    match manifest.level().surface("wall").unwrap() {
        LevelManifestSurface::TextureSingle {
            color,
            collider,
            unlit,
            ..
        } => {
            assert_eq!(*color, Some(crate::color::Color::new(1, 2, 3, 4)));
            assert_eq!(*collider, Some(LevelManifestColliderType::Wall));
            assert_eq!(*unlit, Some(false));
        }
        _ => panic!("expected single texture appearance"),
    }

    match manifest.level().surface("lava").unwrap() {
        LevelManifestSurface::TextureMulti {
            color,
            collider,
            unlit,
            ..
        } => {
            assert_eq!(*color, Some(crate::color::Color::new(10, 20, 30, 40)));
            assert_eq!(*collider, Some(LevelManifestColliderType::Null));
            assert_eq!(*unlit, Some(true));
        }
        _ => panic!("expected multi texture appearance"),
    }

    match manifest.level().surface("paint").unwrap() {
        LevelManifestSurface::Untextured {
            color,
            collider,
            unlit,
            ..
        } => {
            assert_eq!(*color, crate::color::Color::new(7, 8, 9, 10));
            assert_eq!(*collider, Some(LevelManifestColliderType::Ladder));
            assert_eq!(*unlit, Some(true));
        }
        _ => panic!("expected untextured appearance"),
    }

    match manifest.level().surface("ghost").unwrap() {
        LevelManifestSurface::Invisible { collider } => {
            assert_eq!(*collider, Some(LevelManifestColliderType::Null));
        }
        _ => panic!("expected invisible appearance"),
    }
}

#[test]
fn test_surface_multi_texture_must_have_frames() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {
                "lava": {
                    "collider": "Null",
                    "type": "TextureMulti",
                    "frames": [],
                    "animation_speed": 1.0,
                    "color": [255, 255, 255, 255]
                }
            }
        },
        "portal": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(
        result,
        Err(LevelManifestFromBytesError::EmptySurfaceFrameArray)
    ));
}

#[test]
fn test_old_surface_schema_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {
                "wall": {
                    "image": "wall.png",
                    "collider_type": "wall"
                }
            }
        },
        "portal": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestFromBytesError::Decode)));
}

#[test]
fn test_manifest_load_accepts_reused_asset_paths() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "lightmap": "lm.png",
            "track": "music.ogg",
            "spawn": [0.0, 0.0, 0.0],
            "surface": {
                "wall": {
                    "collider": "Null",
                    "type": "TextureSingle",
                    "frame": "shared.png",
                    "color": [255, 255, 255, 255]
                },
                "lava": {
                    "collider": "Null",
                    "type": "TextureMulti",
                    "frames": ["shared.png", "lava_1.png"],
                    "animation_speed": 2.5,
                    "color": [255, 255, 255, 255]
                }
            }
        },
        "portal": {
            "p1": { "mesh": "portal_a.glb", "link": "a.json#x" },
            "p2": { "mesh": "portal_a.glb", "link": "b.json#x" }
        }
    }"#;

    let manifest = load_manifest_json(json).unwrap();
    assert_eq!(manifest.level().mesh(), "level.glb");
    assert_eq!(manifest.portal_len(), 2);
}
