use std::f32::consts::PI;

use glam::Vec3;
use url::Url;

use super::geometry::{LevelPortalGeometry, LevelPortalGeometryFromGLTFError};
use super::kind::LevelPortalKind;
use super::link::LevelPortalLink;
use crate::color::Color;
use crate::gltf::GLTFMesh;

const WHITE_COLOR: Color = Color::WHITE;
const ANCHOR_COLOR: Color = Color::new(255, 0, 255, 255);

fn push_color(bytes: &mut Vec<u8>, color: Color) {
    bytes.push(color.r);
    bytes.push(color.g);
    bytes.push(color.b);
    bytes.push(color.a);
}

fn make_mesh_with_colors(positions: Vec<f32>, colors: Vec<u8>, indices: Vec<u32>) -> GLTFMesh {
    return GLTFMesh::new(positions, indices, None, None, Some(colors));
}

#[test]
fn rejects_insufficient_vertices() {
    let mesh = make_mesh_with_colors(
        vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        {
            let mut bytes = Vec::new();
            push_color(&mut bytes, ANCHOR_COLOR);
            push_color(&mut bytes, WHITE_COLOR);
            bytes
        },
        vec![0, 1],
    );
    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::InsufficientVertices)
    ));
}

#[test]
fn rejects_non_coplanar_vertices() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, 1.0, 1.0, 0.5, // off plane
        0.0, 1.0, 0.0,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::NotCoplanar)
    ));
}

#[test]
fn accepts_arbitrary_polygon_floor_portal() {
    let positions = vec![
        1.0, 0.0, 0.0, // anchor
        0.5, 0.0, 0.8, -0.5, 0.0, 0.8, -1.0, 0.0, 0.0, -0.5, 0.0, -0.8, 0.5, 0.0, -0.8,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let indices = vec![0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4];
    let mesh = make_mesh_with_colors(positions, colors, indices);

    let spec = LevelPortalGeometry::from_gltf(mesh.vertices()).unwrap();
    assert!(matches!(spec.kind(), LevelPortalKind::Floor));
    assert!(spec.yaw().abs() < 0.001);
}

#[test]
fn rejects_missing_anchor_color() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0];
    let mut colors = Vec::new();
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::MissingAnchorColor)
    ));
}

#[test]
fn rejects_ambiguous_anchor_color() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor 1
        1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0,
        0.0, // anchor 2 (different position)
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, ANCHOR_COLOR);
    let indices = vec![0, 1, 2, 0, 2, 3, 0, 4, 1];
    let mesh = make_mesh_with_colors(positions, colors, indices);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::AmbiguousAnchorColor)
    ));
}

#[test]
fn rejects_unstable_anchor() {
    // Anchor at the centroid of all unique vertices.
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let indices = vec![0, 1, 3, 0, 3, 2, 0, 2, 4, 0, 4, 1];
    let mesh = make_mesh_with_colors(positions, colors, indices);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::UnstableAnchor)
    ));
}

#[test]
fn wall_portal_computes_yaw_and_roll() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let spec = LevelPortalGeometry::from_gltf(mesh.vertices()).unwrap();
    assert!(matches!(spec.kind(), LevelPortalKind::Wall));
    assert!(spec.yaw().abs() < 0.001);
    assert!((spec.roll() - 2.3561945).abs() < 0.001);
}

#[test]
fn rejects_tilted_portal() {
    let positions = vec![
        0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0, 2.12, 2.12, 0.0, 2.12, 2.12,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::TiltedPortal)
    ));
}

#[test]
fn wall_link_yaw_delta_uses_yaw() {
    let link = LevelPortalLink::from_geometry_pair(
        Url::parse("https://example.com/level.json").unwrap(),
        "dst".to_string(),
        LevelPortalGeometry::new(
            Vec3::ZERO,
            Vec3::Z,
            0.25,
            0.0,
            LevelPortalKind::Wall,
            vec![],
        ),
        LevelPortalGeometry::new(Vec3::ZERO, Vec3::Z, 1.0, 0.5, LevelPortalKind::Wall, vec![]),
    );

    assert!((link.yaw_delta() - (1.0 - 0.25 + PI)).abs() < 0.001);
}

#[test]
fn floor_ceiling_link_yaw_delta_uses_roll_plus_pi() {
    let link = LevelPortalLink::from_geometry_pair(
        Url::parse("https://example.com/level.json").unwrap(),
        "dst".to_string(),
        LevelPortalGeometry::new(
            Vec3::ZERO,
            Vec3::Y,
            1.2,
            0.25,
            LevelPortalKind::Floor,
            vec![],
        ),
        LevelPortalGeometry::new(
            Vec3::ZERO,
            Vec3::NEG_Y,
            -0.7,
            1.0,
            LevelPortalKind::Ceiling,
            vec![],
        ),
    );

    assert!((link.yaw_delta() - (1.0 - 0.25 + PI)).abs() < 0.001);
}

#[test]
fn geometry_matches_within_epsilon() {
    let a = LevelPortalGeometry::new(
        Vec3::ZERO,
        Vec3::Z,
        0.0,
        0.2,
        LevelPortalKind::Wall,
        vec![(0.4, 1.2), (1.1, 2.5), (2.7, 0.8)],
    );
    let b = LevelPortalGeometry::new(
        Vec3::new(10.0, 2.0, -3.0),
        Vec3::Z,
        1.0,
        0.2005,
        LevelPortalKind::Wall,
        vec![(0.4005, 1.2005), (1.1005, 2.5005), (2.7005, 0.8005)],
    );

    assert!(a.matches(&b));
}

#[test]
fn geometry_rejects_incompatible_kinds() {
    let a = LevelPortalGeometry::new(
        Vec3::ZERO,
        Vec3::Y,
        0.0,
        0.0,
        LevelPortalKind::Floor,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );
    let b = LevelPortalGeometry::new(
        Vec3::ZERO,
        Vec3::Y,
        0.0,
        0.0,
        LevelPortalKind::Floor,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );

    assert!(!a.matches(&b));
}

#[test]
fn geometry_rejects_wall_roll_mismatch() {
    let a = LevelPortalGeometry::new(
        Vec3::ZERO,
        Vec3::Z,
        0.0,
        0.0,
        LevelPortalKind::Wall,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );
    let b = LevelPortalGeometry::new(
        Vec3::ZERO,
        Vec3::Z,
        0.0,
        0.1,
        LevelPortalKind::Wall,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );

    assert!(!a.matches(&b));
}

#[test]
fn geometry_rejects_fingerprint_mismatch() {
    let a = LevelPortalGeometry::new(
        Vec3::ZERO,
        Vec3::Y,
        0.0,
        0.0,
        LevelPortalKind::Floor,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );
    let b = LevelPortalGeometry::new(
        Vec3::ZERO,
        Vec3::NEG_Y,
        0.0,
        0.0,
        LevelPortalKind::Ceiling,
        vec![(0.5, 1.0), (1.5, 1.0), (2.8, 1.0)],
    );

    assert!(!a.matches(&b));
}
