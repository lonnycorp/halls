use glam::{Vec2, Vec3};

use crate::gltf::GLTFMesh;

use super::portal::{PortalError, PortalKind};

#[derive(Debug)]
pub struct PortalSpec {
    pub center: Vec3,
    pub yaw: f32,
    pub kind: PortalKind,
    pub dimensions: Vec2,
}

struct MergedVertex {
    position: Vec3,
    uv: Vec2,
}

const EPSILON: f32 = 0.001;
const UV_EPSILON: f32 = 0.001;

fn verify_coplanar(vertices: &[Vec3], plane_point: Vec3, plane_normal: Vec3) -> bool {
    for &v in vertices {
        let dist = (v - plane_point).dot(plane_normal).abs();
        if dist > EPSILON {
            return false;
        }
    }
    return true;
}

fn verify_perpendicular_edges(corners: &[Vec3; 4]) -> bool {
    for i in 0..4 {
        let edge_a = corners[(i + 1) % 4] - corners[i];
        let edge_b = corners[(i + 2) % 4] - corners[(i + 1) % 4];
        if edge_a.dot(edge_b).abs() > EPSILON {
            return false;
        }
    }
    return true;
}

fn merge_vertices(mesh: &GLTFMesh) -> Result<Vec<MergedVertex>, PortalError> {
    let mut merged: Vec<MergedVertex> = Vec::new();

    for i in 0..mesh.vertex_count() {
        let vertex = mesh.vertex(i);
        let uv = vertex.diffuse_uv.ok_or(PortalError::MissingUV)?;

        // Check if we already have a vertex at this position
        let existing = merged
            .iter()
            .find(|m| (m.position - vertex.position).length() < EPSILON);

        if let Some(existing) = existing {
            // Same position must have same UV
            if (existing.uv - uv).length() > UV_EPSILON {
                return Err(PortalError::InconsistentUVs);
            }
        } else {
            merged.push(MergedVertex {
                position: vertex.position,
                uv,
            });
        }
    }

    return Ok(merged);
}

fn uv_matches(uv: Vec2, target: (f32, f32)) -> bool {
    return (uv.x - target.0).abs() < UV_EPSILON && (uv.y - target.1).abs() < UV_EPSILON;
}

fn find_vertex_by_uv(vertices: &[MergedVertex], target: (f32, f32)) -> Option<Vec3> {
    return vertices
        .iter()
        .find(|v| uv_matches(v.uv, target))
        .map(|v| v.position);
}

impl PortalSpec {
    pub fn from_gltf(mesh: &GLTFMesh) -> Result<PortalSpec, PortalError> {
        if mesh.vertex_count() < 3 {
            return Err(PortalError::InsufficientVertices);
        }

        // Merge vertices by position, validating UV consistency
        let merged = merge_vertices(mesh)?;

        // Require exactly 4 unique vertices (rectangular quad)
        if merged.len() != 4 {
            return Err(PortalError::NotRectangularQuad);
        }

        // Get corners by UV layout: (0,0), (1,0), (1,1), (0,1)
        let corners = [
            find_vertex_by_uv(&merged, (0.0, 0.0)).ok_or(PortalError::InvalidUVLayout)?,
            find_vertex_by_uv(&merged, (1.0, 0.0)).ok_or(PortalError::InvalidUVLayout)?,
            find_vertex_by_uv(&merged, (1.0, 1.0)).ok_or(PortalError::InvalidUVLayout)?,
            find_vertex_by_uv(&merged, (0.0, 1.0)).ok_or(PortalError::InvalidUVLayout)?,
        ];

        // Derive normal from UV axes: axis_u × axis_v
        let axis_u = corners[1] - corners[0];
        let axis_v = corners[3] - corners[0];
        let plane_normal = axis_u.cross(axis_v).normalize();

        if plane_normal.is_nan() {
            return Err(PortalError::DegenerateGeometry);
        }

        // Verify all vertices lie in plane
        if !verify_coplanar(&corners, corners[0], plane_normal) {
            return Err(PortalError::NotCoplanar);
        }

        // Verify rectangle: all edges perpendicular to neighbors
        if !verify_perpendicular_edges(&corners) {
            return Err(PortalError::NotRectangularQuad);
        }

        let center = corners.iter().fold(Vec3::ZERO, |a, &b| a + b) / corners.len() as f32;

        // Dimensions are just the axis lengths
        let width = axis_u.length();
        let height = axis_v.length();

        let axis_u_norm = axis_u.normalize();

        // Determine portal kind from plane normal orientation
        let kind = if plane_normal.y.abs() < EPSILON {
            // For walls, axis_u must be horizontal (no Y component)
            if axis_u_norm.y.abs() > EPSILON {
                return Err(PortalError::RolledWallPortal);
            }
            PortalKind::Wall
        } else if plane_normal.y > 1.0 - EPSILON {
            PortalKind::Floor
        } else if plane_normal.y < -1.0 + EPSILON {
            PortalKind::Ceiling
        } else {
            return Err(PortalError::TiltedPortal);
        };

        // Unified yaw from axis_u for all kinds
        let yaw = (-axis_u_norm.z).atan2(axis_u_norm.x);

        return Ok(PortalSpec {
            center,
            yaw,
            kind,
            dimensions: Vec2::new(width, height),
        });
    }
}
