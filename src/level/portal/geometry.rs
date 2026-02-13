use glam::Vec3;

use crate::gltf::GLTFMesh;
use crate::graphics::color::Color;

use super::kind::PortalKind;
use super::portal::PortalError;

#[derive(Debug, Clone)]
pub struct PortalGeometry {
    center: Vec3,
    normal: Vec3,
    yaw: f32,
    roll: f32,
    kind: PortalKind,
    fingerprint_points: Vec<(f32, f32)>,
}

struct MergedVertex {
    position: Vec3,
    color: Option<Color>,
}

const EPSILON: f32 = 0.001;
const ROLL_EPSILON: f32 = 0.001;
const ANGLE_EPSILON: f32 = 0.001;
const LENGTH_EPSILON: f32 = 0.001;
const ANCHOR_COLOR: Color = Color::MAGENTA;

impl PortalGeometry {
    pub fn new(
        center: Vec3,
        normal: Vec3,
        yaw: f32,
        roll: f32,
        kind: PortalKind,
        fingerprint_points: Vec<(f32, f32)>,
    ) -> PortalGeometry {
        return PortalGeometry {
            center,
            normal,
            yaw,
            roll,
            kind,
            fingerprint_points,
        };
    }

    pub fn from_gltf(mesh: &GLTFMesh) -> Result<PortalGeometry, PortalError> {
        let mut merged: Vec<MergedVertex> = Vec::new();
        for i in 0..mesh.vertex_count() {
            let vertex = mesh.vertex(i);
            let existing = merged
                .iter()
                .find(|m| (m.position - vertex.position).length() < EPSILON);

            if let Some(existing) = existing {
                if existing.color != vertex.color {
                    return Err(PortalError::InconsistentColors);
                }
            } else {
                merged.push(MergedVertex {
                    position: vertex.position,
                    color: vertex.color,
                });
            }
        }

        if merged.len() < 3 {
            return Err(PortalError::InsufficientVertices);
        }

        let mut normal = None;
        for tri in 0..(mesh.vertex_count() / 3) {
            let a = mesh.vertex(tri * 3).position;
            let b = mesh.vertex(tri * 3 + 1).position;
            let c = mesh.vertex(tri * 3 + 2).position;
            let tri_normal = (b - a).cross(c - a);
            if tri_normal.length() > EPSILON {
                normal = Some(tri_normal.normalize());
                break;
            }
        }
        let normal = normal.ok_or(PortalError::DegenerateGeometry)?;

        if normal.is_nan() {
            return Err(PortalError::DegenerateGeometry);
        }

        let plane_point = merged[0].position;
        for v in &merged {
            let dist = (v.position - plane_point).dot(normal).abs();
            if dist > EPSILON {
                return Err(PortalError::NotCoplanar);
            }
        }

        let mut center = Vec3::ZERO;
        for v in &merged {
            center += v.position;
        }
        center /= merged.len() as f32;

        let mut anchor = None;
        for v in &merged {
            if v.color == Some(ANCHOR_COLOR) {
                match anchor {
                    None => {
                        anchor = Some(v.position);
                    }
                    Some(existing) => {
                        if (existing - v.position).length() > EPSILON {
                            return Err(PortalError::AmbiguousAnchorColor);
                        }
                    }
                }
            }
        }
        let anchor = anchor.ok_or(PortalError::MissingAnchorColor)?;
        let anchor_to_center = center - anchor;
        if anchor_to_center.length() < EPSILON {
            return Err(PortalError::UnstableAnchor);
        }
        let anchor_to_center = anchor_to_center.normalize();

        let kind = if normal.y.abs() < EPSILON {
            PortalKind::Wall
        } else if normal.y > 1.0 - EPSILON {
            PortalKind::Floor
        } else if normal.y < -1.0 + EPSILON {
            PortalKind::Ceiling
        } else {
            return Err(PortalError::TiltedPortal);
        };

        let reference_axis = kind.reference_axis();
        let reference_in_plane = reference_axis - normal * reference_axis.dot(normal);
        if reference_in_plane.length() < EPSILON {
            return Err(PortalError::DegenerateGeometry);
        }
        let reference_in_plane = reference_in_plane.normalize();
        let center_to_anchor = anchor - center;
        let center_to_anchor = center_to_anchor.normalize();

        let roll = normal
            .dot(reference_in_plane.cross(center_to_anchor))
            .atan2(reference_in_plane.dot(center_to_anchor));

        let yaw = if kind == PortalKind::Wall {
            normal.x.atan2(normal.z)
        } else {
            0.0
        };

        let mut fingerprint_points: Vec<(f32, f32)> = Vec::with_capacity(merged.len());
        for v in &merged {
            let point_to_center = center - v.position;
            let point_length = point_to_center.length();
            let point_angle = if point_length < EPSILON {
                0.0
            } else {
                let point_to_center = point_to_center / point_length;
                anchor_to_center
                    .dot(point_to_center)
                    .clamp(-1.0, 1.0)
                    .acos()
            };
            fingerprint_points.push((point_angle, point_length));
        }
        fingerprint_points.sort_by(|a, b| {
            let angle_order = a.0.total_cmp(&b.0);
            if angle_order == std::cmp::Ordering::Equal {
                return a.1.total_cmp(&b.1);
            }
            return angle_order;
        });

        return Ok(PortalGeometry::new(
            center,
            normal,
            yaw,
            roll,
            kind,
            fingerprint_points,
        ));
    }

    pub fn matches(&self, other: &PortalGeometry) -> bool {
        let kind_compatible = matches!(
            (self.kind, other.kind),
            (PortalKind::Wall, PortalKind::Wall)
                | (PortalKind::Floor, PortalKind::Ceiling)
                | (PortalKind::Ceiling, PortalKind::Floor)
        );
        if !kind_compatible {
            return false;
        }

        if self.kind == PortalKind::Wall && (self.roll - other.roll).abs() > ROLL_EPSILON {
            return false;
        }

        if self.fingerprint_points.len() != other.fingerprint_points.len() {
            return false;
        }

        for (left, right) in self
            .fingerprint_points
            .iter()
            .zip(other.fingerprint_points.iter())
        {
            let (left_angle, left_length) = left;
            let (right_angle, right_length) = right;
            if (*left_angle - *right_angle).abs() > ANGLE_EPSILON {
                return false;
            }
            if (*left_length - *right_length).abs() > LENGTH_EPSILON {
                return false;
            }
        }

        return true;
    }

    pub fn center(&self) -> Vec3 {
        return self.center;
    }

    pub fn normal(&self) -> Vec3 {
        return self.normal;
    }

    pub fn yaw(&self) -> f32 {
        return self.yaw;
    }

    pub fn roll(&self) -> f32 {
        return self.roll;
    }

    pub fn kind(&self) -> PortalKind {
        return self.kind;
    }
}
