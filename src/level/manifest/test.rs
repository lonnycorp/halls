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
            "collider": "level.glb"
        },
        "portal": {
            "portal_a": {
                "model": "portal_a.glb",
                "link": "other.json#portal_b",
                "spawn": true
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
    assert_eq!(manifest.spawn, "portal_a");
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
            "p1": { "model": "p1.glb", "link": "a.json#x", "spawn": true }
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
fn test_manifest_without_portals() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb"
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(matches!(result, Err(LevelManifestError::NoSpawnPortal)));
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
            "p1": { "model": "p1.glb", "link": "a.json#x", "spawn": true },
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
fn test_no_spawn_portal_returns_error() {
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
    assert!(matches!(result, Err(LevelManifestError::NoSpawnPortal)));
}

#[test]
fn test_multiple_spawn_portals_returns_error() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb"
        },
        "portal": {
            "portal_a": { "model": "p1.glb", "link": "a.json#x", "spawn": true },
            "portal_b": { "model": "p2.glb", "link": "b.json#x", "spawn": true }
        }
    }"#;

    let result = LevelManifest::load(json.as_bytes());
    assert!(matches!(
        result,
        Err(LevelManifestError::MultipleSpawnPortals)
    ));
}
