use super::portal::{PortalError, PortalKind};
use super::spec::PortalSpec;
use crate::gltf::GLTFMesh;

fn make_mesh(positions: Vec<f32>, indices: Vec<u32>) -> GLTFMesh {
    GLTFMesh::from_raw(positions, indices)
}

fn make_mesh_with_uvs(positions: Vec<f32>, uvs: Vec<f32>, indices: Vec<u32>) -> GLTFMesh {
    GLTFMesh::from_raw_with_uvs(positions, uvs, indices)
}

// Standard UV layout for a quad: (0,0), (1,0), (1,1), (0,1)
fn standard_uvs() -> Vec<f32> {
    vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0]
}

// Creates a wall portal (vertical face, normal in XZ plane)
// UV layout: vertex 0 = (0,0), vertex 1 = (1,0), vertex 2 = (1,1), vertex 3 = (0,1)
fn make_wall_portal(w: f32, h: f32, yaw: f32) -> GLTFMesh {
    let (sin_yaw, cos_yaw) = yaw.sin_cos();
    // Direction from UV(0,0) to UV(1,0) defines the local_u axis
    // For yaw=0, local_u should point along +X, so portal faces +Z
    let right_x = cos_yaw;
    let right_z = -sin_yaw;

    // Vertices: bottom-left, bottom-right, top-right, top-left
    let positions = vec![
        0.0,
        0.0,
        0.0, // UV (0,0)
        right_x * w,
        0.0,
        right_z * w, // UV (1,0)
        right_x * w,
        h,
        right_z * w, // UV (1,1)
        0.0,
        h,
        0.0, // UV (0,1)
    ];
    let uvs = standard_uvs();
    let indices = vec![0, 1, 2, 0, 2, 3];

    make_mesh_with_uvs(positions, uvs, indices)
}

#[test]
fn rejects_degenerate_line() {
    let mesh = make_mesh(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], vec![0, 1]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::InsufficientVertices)));
}

#[test]
fn accepts_floor_portal() {
    // Floor portal facing up (normal = +Y)
    // axis_u = (0,0,3), axis_v = (3,0,0), U × V = (0,+9,0) -> Floor
    let positions = vec![
        0.0, 0.0, 0.0, // UV (0,0)
        0.0, 0.0, 3.0, // UV (1,0)
        3.0, 0.0, 3.0, // UV (1,1)
        3.0, 0.0, 0.0, // UV (0,1)
    ];
    let uvs = standard_uvs();
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(result.is_ok(), "Expected Ok but got {:?}", result);

    let spec = result.unwrap();
    assert!(matches!(spec.kind, PortalKind::Floor));
}

#[test]
fn accepts_ceiling_portal() {
    // Ceiling portal facing down (normal = -Y)
    // For -Y normal: need CW winding from above
    // Vertices: 0=(0,0,0), 1=(3,0,0), 2=(3,0,3), 3=(0,0,3)
    let positions = vec![
        0.0, 0.0, 0.0, // UV (0,0)
        3.0, 0.0, 0.0, // UV (1,0) - local_u points +X
        3.0, 0.0, 3.0, // UV (1,1)
        0.0, 0.0, 3.0, // UV (0,1)
    ];
    let uvs = standard_uvs();
    // Winding: 0->1->2, 0->2->3 gives -Y normal
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(result.is_ok(), "Expected Ok but got {:?}", result);

    let spec = result.unwrap();
    assert!(matches!(spec.kind, PortalKind::Ceiling));
}

#[test]
fn rejects_non_coplanar_vertices() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.5, 0.0, 1.0, 0.0];
    let uvs = standard_uvs();
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::NotCoplanar)));
}

#[test]
fn accepts_valid_front_portal() {
    let mesh = make_wall_portal(2.0, 3.0, 0.0);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(result.is_ok(), "Expected Ok but got {:?}", result);

    let spec = result.unwrap();
    assert!(matches!(spec.kind, PortalKind::Wall));
    assert!(spec.yaw.abs() < 0.001);
}

#[test]
fn accepts_diagonal_wall_portal() {
    use std::f32::consts::FRAC_PI_4;

    let mesh = make_wall_portal(2.0, 3.0, FRAC_PI_4);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(result.is_ok(), "Expected Ok but got {:?}", result);

    let spec = result.unwrap();
    assert!(matches!(spec.kind, PortalKind::Wall));
    assert!((spec.yaw - FRAC_PI_4).abs() < 0.001);
}

#[test]
fn accepts_shifted_winding_order() {
    // Same rectangle, different triangle winding
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 3.0, 0.0, 0.0, 3.0, 0.0];
    let uvs = standard_uvs();
    let mesh = make_mesh_with_uvs(positions, uvs, vec![1, 2, 3, 1, 3, 0]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(result.is_ok(), "Expected Ok but got {:?}", result);
}

#[test]
fn accepts_reversed_triangle_order() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 3.0, 0.0, 0.0, 3.0, 0.0];
    let uvs = standard_uvs();
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 2, 3, 0, 1, 2]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(result.is_ok(), "Expected Ok but got {:?}", result);
}

#[test]
fn rejects_hexagon_portal() {
    let half_w = 2.0;
    let quarter_w = 1.0;
    let h = 2.0;

    let positions = vec![
        -quarter_w,
        0.0,
        0.0,
        quarter_w,
        0.0,
        0.0,
        half_w,
        h / 2.0,
        0.0,
        quarter_w,
        h,
        0.0,
        -quarter_w,
        h,
        0.0,
        -half_w,
        h / 2.0,
        0.0,
    ];
    // 6 vertices, can't have standard 4-corner UV layout
    let uvs = vec![0.0, 0.0, 0.5, 0.0, 1.0, 0.5, 0.5, 1.0, 0.0, 1.0, 0.0, 0.5];
    let indices = vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5];

    let mesh = make_mesh_with_uvs(positions, uvs, indices);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::NotRectangularQuad)));
}

#[test]
fn rejects_triangle_portal() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0];
    let uvs = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::NotRectangularQuad)));
}

#[test]
fn rejects_parallelogram_portal() {
    // Skewed quad - not symmetric around center
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.5, 1.0, 0.0, 0.5, 1.0, 0.0];
    let uvs = standard_uvs();
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::NotRectangularQuad)));
}

#[test]
fn rejects_trapezoid_portal() {
    // Asymmetric widths - wider at bottom than top
    let positions = vec![-1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0, -0.5, 1.0, 0.0];
    let uvs = standard_uvs();
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::NotRectangularQuad)));
}

#[test]
fn floor_portal_yaw_from_uv_layout() {
    // Floor portal where local_u (UV 0,0 -> 1,0) points along +Z
    // axis_u_norm = (0,0,1)
    // yaw = atan2(-1, 0) = -π/2
    use std::f32::consts::FRAC_PI_2;

    let positions = vec![
        0.0, 0.0, 0.0, // UV (0,0)
        0.0, 0.0, 3.0, // UV (1,0) - local_u points +Z
        3.0, 0.0, 3.0, // UV (1,1)
        3.0, 0.0, 0.0, // UV (0,1)
    ];
    let uvs = standard_uvs();
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(result.is_ok(), "Expected Ok but got {:?}", result);

    let spec = result.unwrap();
    assert!(matches!(spec.kind, PortalKind::Floor));
    // local_u = (0,0,3) - (0,0,0) = (0,0,3), normalized = (0,0,1)
    // yaw = atan2(-1, 0) = -π/2
    assert!(
        (spec.yaw - (-FRAC_PI_2)).abs() < 0.001,
        "Expected yaw ~-π/2, got {}",
        spec.yaw
    );
}

#[test]
fn ceiling_portal_yaw_from_uv_layout() {
    // Ceiling portal where local_u points along +X
    // axis_u_norm = (1,0,0)
    // yaw = atan2(0, 1) = 0
    let positions = vec![
        0.0, 0.0, 0.0, // UV (0,0)
        3.0, 0.0, 0.0, // UV (1,0) - local_u points +X
        3.0, 0.0, 3.0, // UV (1,1)
        0.0, 0.0, 3.0, // UV (0,1)
    ];
    let uvs = standard_uvs();
    // Winding for ceiling (normal -Y): [0,1,2, 0,2,3]
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(result.is_ok(), "Expected Ok but got {:?}", result);

    let spec = result.unwrap();
    assert!(matches!(spec.kind, PortalKind::Ceiling));
    // local_u = (3,0,0) - (0,0,0) = (3,0,0), normalized = (1,0,0)
    // yaw = atan2(0, 1) = 0
    assert!(spec.yaw.abs() < 0.001, "Expected yaw ~0, got {}", spec.yaw);
}

#[test]
fn rejects_portal_without_uvs() {
    // Portal without any UV data
    let positions = vec![0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0, 0.0, 3.0, 0.0, 0.0, 3.0];
    let mesh = make_mesh(positions, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::MissingUV)));
}

#[test]
fn rejects_portal_with_invalid_uv_layout() {
    // Portal with UVs but not the expected corner layout
    let positions = vec![0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0, 0.0, 3.0, 0.0, 0.0, 3.0];
    let uvs = vec![
        0.5, 0.5, // Not (0,0)
        1.0, 0.5, 1.0, 1.0, 0.5, 1.0,
    ];
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::InvalidUVLayout)));
}

#[test]
fn rejects_tilted_portal() {
    // Portal tilted at 45 degrees - neither wall nor floor/ceiling
    let positions = vec![
        0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0, 2.12, 2.12, 0.0, 2.12, 2.12,
    ];
    let uvs = standard_uvs();
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::TiltedPortal)));
}

#[test]
fn rejects_inconsistent_uvs() {
    // Same position referenced with different UVs
    let positions = vec![
        0.0, 0.0, 0.0, // First occurrence
        3.0, 0.0, 0.0, 3.0, 3.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0,
        0.0, // Same position as vertex 0
        3.0, 3.0, 0.0, // Same position as vertex 2
    ];
    let uvs = vec![
        0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.5, 0.5, // Different UV for same position!
        0.5, 0.5, // Different UV for same position!
    ];
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 3, 4, 5]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(matches!(result, Err(PortalError::InconsistentUVs)));
}

#[test]
fn accepts_non_square_floor_rectangle() {
    // 3x4 rectangle
    // axis_u = (0,0,3), axis_v = (4,0,0), U × V = (0,+12,0) -> Floor
    let positions = vec![
        0.0, 0.0, 0.0, // UV (0,0)
        0.0, 0.0, 3.0, // UV (1,0)
        4.0, 0.0, 3.0, // UV (1,1)
        4.0, 0.0, 0.0, // UV (0,1)
    ];
    let uvs = standard_uvs();
    let mesh = make_mesh_with_uvs(positions, uvs, vec![0, 1, 2, 0, 2, 3]);
    let result = PortalSpec::from_gltf(&mesh);
    assert!(result.is_ok(), "Expected Ok but got {:?}", result);

    let spec = result.unwrap();
    assert!(matches!(spec.kind, PortalKind::Floor));
}
