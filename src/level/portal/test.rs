use std::f32::consts::PI;

use glam::Vec3;
use url::Url;

use super::geometry::PortalGeometry;
use super::kind::PortalKind;
use super::link::PortalLink;
use super::portal::PortalError;
use crate::gltf::GLTFMesh;

const WHITE_COLOR: [u8; 4] = [255, 255, 255, 255];
const ANCHOR_COLOR: [u8; 4] = [255, 0, 255, 255];

fn make_mesh_with_colors(positions: Vec<f32>, colors: Vec<u8>, indices: Vec<u32>) -> GLTFMesh {
    return GLTFMesh::new(positions, indices, None, None, Some(colors));
}

#[test]
fn rejects_insufficient_vertices() {
    let mesh = make_mesh_with_colors(
        vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        vec![
            ANCHOR_COLOR[0],
            ANCHOR_COLOR[1],
            ANCHOR_COLOR[2],
            ANCHOR_COLOR[3],
            WHITE_COLOR[0],
            WHITE_COLOR[1],
            WHITE_COLOR[2],
            WHITE_COLOR[3],
        ],
        vec![0, 1],
    );
    let result = PortalGeometry::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::InsufficientVertices)));
}

#[test]
fn rejects_non_coplanar_vertices() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, 1.0, 1.0, 0.5, // off plane
        0.0, 1.0, 0.0,
    ];
    let colors = vec![
        ANCHOR_COLOR[0],
        ANCHOR_COLOR[1],
        ANCHOR_COLOR[2],
        ANCHOR_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
    ];
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let result = PortalGeometry::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::NotCoplanar)));
}

#[test]
fn accepts_arbitrary_polygon_floor_portal() {
    let positions = vec![
        1.0, 0.0, 0.0, // anchor
        0.5, 0.0, 0.8, -0.5, 0.0, 0.8, -1.0, 0.0, 0.0, -0.5, 0.0, -0.8, 0.5, 0.0, -0.8,
    ];
    let colors = vec![
        ANCHOR_COLOR[0],
        ANCHOR_COLOR[1],
        ANCHOR_COLOR[2],
        ANCHOR_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
    ];
    let indices = vec![0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4];
    let mesh = make_mesh_with_colors(positions, colors, indices);

    let spec = PortalGeometry::from_gltf(&mesh).unwrap();
    assert!(matches!(spec.kind(), PortalKind::Floor));
    assert!(spec.yaw().abs() < 0.001);
}

#[test]
fn rejects_missing_anchor_color() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0];
    let colors = vec![
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
    ];
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let result = PortalGeometry::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::MissingAnchorColor)));
}

#[test]
fn rejects_ambiguous_anchor_color() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor 1
        1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0,
        0.0, // anchor 2 (different position)
    ];
    let colors = vec![
        ANCHOR_COLOR[0],
        ANCHOR_COLOR[1],
        ANCHOR_COLOR[2],
        ANCHOR_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        ANCHOR_COLOR[0],
        ANCHOR_COLOR[1],
        ANCHOR_COLOR[2],
        ANCHOR_COLOR[3],
    ];
    let indices = vec![0, 1, 2, 0, 2, 3, 0, 4, 1];
    let mesh = make_mesh_with_colors(positions, colors, indices);

    let result = PortalGeometry::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::AmbiguousAnchorColor)));
}

#[test]
fn rejects_unstable_anchor() {
    // Anchor at the centroid of all unique vertices.
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0,
    ];
    let colors = vec![
        ANCHOR_COLOR[0],
        ANCHOR_COLOR[1],
        ANCHOR_COLOR[2],
        ANCHOR_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
    ];
    let indices = vec![0, 1, 3, 0, 3, 2, 0, 2, 4, 0, 4, 1];
    let mesh = make_mesh_with_colors(positions, colors, indices);

    let result = PortalGeometry::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::UnstableAnchor)));
}

#[test]
fn wall_portal_computes_yaw_and_roll() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
    ];
    let colors = vec![
        ANCHOR_COLOR[0],
        ANCHOR_COLOR[1],
        ANCHOR_COLOR[2],
        ANCHOR_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
    ];
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let spec = PortalGeometry::from_gltf(&mesh).unwrap();
    assert!(matches!(spec.kind(), PortalKind::Wall));
    assert!(spec.yaw().abs() < 0.001);
    assert!((spec.roll() - 2.3561945).abs() < 0.001);
}

#[test]
fn rejects_tilted_portal() {
    let positions = vec![
        0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0, 2.12, 2.12, 0.0, 2.12, 2.12,
    ];
    let colors = vec![
        ANCHOR_COLOR[0],
        ANCHOR_COLOR[1],
        ANCHOR_COLOR[2],
        ANCHOR_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
        WHITE_COLOR[0],
        WHITE_COLOR[1],
        WHITE_COLOR[2],
        WHITE_COLOR[3],
    ];
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let result = PortalGeometry::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::TiltedPortal)));
}

#[test]
fn wall_link_yaw_delta_uses_yaw() {
    let link = PortalLink::new(
        Url::parse("https://example.com/level.json").unwrap(),
        "dst".to_string(),
        PortalGeometry::new(Vec3::ZERO, Vec3::Z, 0.25, 0.0, PortalKind::Wall, vec![]),
        PortalGeometry::new(Vec3::ZERO, Vec3::Z, 1.0, 0.5, PortalKind::Wall, vec![]),
    );

    assert!((link.yaw_delta() - (1.0 - 0.25 + PI)).abs() < 0.001);
}

#[test]
fn floor_ceiling_link_yaw_delta_uses_roll_plus_pi() {
    let link = PortalLink::new(
        Url::parse("https://example.com/level.json").unwrap(),
        "dst".to_string(),
        PortalGeometry::new(Vec3::ZERO, Vec3::Y, 1.2, 0.25, PortalKind::Floor, vec![]),
        PortalGeometry::new(
            Vec3::ZERO,
            Vec3::NEG_Y,
            -0.7,
            1.0,
            PortalKind::Ceiling,
            vec![],
        ),
    );

    assert!((link.yaw_delta() - (1.0 - 0.25 + PI)).abs() < 0.001);
}

#[test]
fn geometry_matches_within_epsilon() {
    let a = PortalGeometry::new(
        Vec3::ZERO,
        Vec3::Z,
        0.0,
        0.2,
        PortalKind::Wall,
        vec![(0.4, 1.2), (1.1, 2.5), (2.7, 0.8)],
    );
    let b = PortalGeometry::new(
        Vec3::new(10.0, 2.0, -3.0),
        Vec3::Z,
        1.0,
        0.2005,
        PortalKind::Wall,
        vec![(0.4005, 1.2005), (1.1005, 2.5005), (2.7005, 0.8005)],
    );

    assert!(a.matches(&b));
}

#[test]
fn geometry_rejects_incompatible_kinds() {
    let a = PortalGeometry::new(
        Vec3::ZERO,
        Vec3::Y,
        0.0,
        0.0,
        PortalKind::Floor,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );
    let b = PortalGeometry::new(
        Vec3::ZERO,
        Vec3::Y,
        0.0,
        0.0,
        PortalKind::Floor,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );

    assert!(!a.matches(&b));
}

#[test]
fn geometry_rejects_wall_roll_mismatch() {
    let a = PortalGeometry::new(
        Vec3::ZERO,
        Vec3::Z,
        0.0,
        0.0,
        PortalKind::Wall,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );
    let b = PortalGeometry::new(
        Vec3::ZERO,
        Vec3::Z,
        0.0,
        0.1,
        PortalKind::Wall,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );

    assert!(!a.matches(&b));
}

#[test]
fn geometry_rejects_fingerprint_mismatch() {
    let a = PortalGeometry::new(
        Vec3::ZERO,
        Vec3::Y,
        0.0,
        0.0,
        PortalKind::Floor,
        vec![(0.5, 1.0), (1.5, 1.0), (2.5, 1.0)],
    );
    let b = PortalGeometry::new(
        Vec3::ZERO,
        Vec3::NEG_Y,
        0.0,
        0.0,
        PortalKind::Ceiling,
        vec![(0.5, 1.0), (1.5, 1.0), (2.8, 1.0)],
    );

    assert!(!a.matches(&b));
}
