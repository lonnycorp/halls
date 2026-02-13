use super::manifest::LevelManifestMaterial;
use super::*;

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
            "model": "level.glb",
            "collider": "level.glb",
            "spawn": [1.0, 2.0, 3.0]
        },
        "portal": {
            "portal_a": {
                "model": "portal_a.glb",
                "link": "other.json#portal_b"
            }
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert_eq!(manifest.meta.name, "Test Level");
    assert_eq!(manifest.meta.author.as_deref(), Some("Test Author"));
    assert_eq!(manifest.meta.track.as_deref(), Some("Test Track"));
    assert_eq!(manifest.portal.len(), 1);
    assert!(manifest.portal.contains_key("portal_a"));
    assert_eq!(manifest.level.spawn, [1.0, 2.0, 3.0]);
}

#[test]
fn test_valid_manifest_without_optional_meta_fields() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level"
        },
        "level": {
            "model": "level.glb"
        },
        "portal": {
            "p1": { "model": "p1.glb", "link": "a.json#x" }
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert_eq!(manifest.meta.name, "Test Level");
    assert!(manifest.meta.author.is_none());
    assert!(manifest.meta.track.is_none());
}

#[test]
fn test_manifest_without_material_defaults_to_empty_map() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level"
        },
        "level": {
            "model": "level.glb"
        },
        "portal": {
            "p1": { "model": "p1.glb", "link": "a.json#x" }
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert!(manifest.level.material.is_empty());
}

#[test]
fn test_manifest_without_portals() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb"
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(result.is_ok());
}

#[test]
fn test_invalid_json_returns_decode_error() {
    let result = LevelManifest::load(b"{ invalid json }");
    assert!(matches!(result, Err(LevelManifestError::Load)));
}

#[test]
fn test_too_many_portals_returns_error() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb"
        },
        "portal": {
            "p1": { "model": "p1.glb", "link": "a.json#x" },
            "p2": { "model": "p2.glb", "link": "a.json#x" },
            "p3": { "model": "p3.glb", "link": "a.json#x" },
            "p4": { "model": "p4.glb", "link": "a.json#x" },
            "p5": { "model": "p5.glb", "link": "a.json#x" }
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(matches!(result, Err(LevelManifestError::TooManyPortals)));
}

#[test]
fn test_invalid_version_returns_error() {
    let json = r#"{
        "_version": "wrong",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb"
        },
        "portal": {}
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(matches!(result, Err(LevelManifestError::InvalidVersion)));
}

#[test]
fn test_spawn_defaults_to_origin() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb"
        },
        "portal": {
            "portal_a": { "model": "p1.glb", "link": "a.json#x" }
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(result.is_ok());
    let manifest = result.unwrap();
    assert_eq!(manifest.level.spawn, [0.0, 0.0, 0.0]);
}

#[test]
fn test_spawn_accepts_coordinates() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [-12.5, 0.0, 3.75]
        },
        "portal": {
            "portal_a": { "model": "p1.glb", "link": "a.json#x" },
            "portal_b": { "model": "p2.glb", "link": "b.json#x" }
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(result.is_ok());
    let manifest = result.unwrap();
    assert_eq!(manifest.level.spawn, [-12.5, 0.0, 3.75]);
}

#[test]
fn test_material_tint_defaults_to_white() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "material": {
                "wall": {
                    "image": "wall.png"
                }
            }
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(result.is_ok());
    let manifest = result.unwrap();

    let material = manifest.level.material.get("wall").unwrap();
    match material {
        LevelManifestMaterial::Static { tint, .. } => {
            assert_eq!(*tint, [255, 255, 255]);
        }
        _ => panic!("expected static material"),
    }
}

#[test]
fn test_material_tint_parses_for_static_and_animated() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "material": {
                "wall": {
                    "image": "wall.png",
                    "tint": [1, 2, 3]
                },
                "lava": {
                    "images": ["lava_0.png", "lava_1.png"],
                    "animation_speed": 2.5,
                    "tint": [10, 20, 30]
                }
            }
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(result.is_ok());
    let manifest = result.unwrap();

    let wall = manifest.level.material.get("wall").unwrap();
    match wall {
        LevelManifestMaterial::Static { tint, .. } => {
            assert_eq!(*tint, [1, 2, 3]);
        }
        _ => panic!("expected static material"),
    }

    let lava = manifest.level.material.get("lava").unwrap();
    match lava {
        LevelManifestMaterial::Animated { tint, .. } => {
            assert_eq!(*tint, [10, 20, 30]);
        }
        _ => panic!("expected animated material"),
    }
}
