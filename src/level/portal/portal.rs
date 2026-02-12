use parry3d::math::{Isometry, Vector};
use parry3d::query::{cast_shapes, ShapeCastHit, ShapeCastOptions};
use parry3d::shape::{Cuboid, TriMesh};
use url::Url;

use crate::graphics::model::Model;
use crate::level::cache::{LevelCache, LevelCacheResult};

use super::geometry::PortalGeometry;
use super::PortalLink;

#[derive(Debug, Clone)]
pub enum PortalError {
    InsufficientVertices,
    DegenerateGeometry,
    NotCoplanar,
    TiltedPortal,
    InconsistentColors,
    MissingAnchorColor,
    AmbiguousAnchorColor,
    UnstableAnchor,
}

pub struct LevelPortal {
    name: String,
    geometry: PortalGeometry,
    model: Model,
    collider: TriMesh,
    link: Url,
}

impl LevelPortal {
    pub fn new(
        name: String,
        geometry: PortalGeometry,
        model: Model,
        collider: TriMesh,
        link: Url,
    ) -> Self {
        return Self {
            name,
            geometry,
            model,
            collider,
            link,
        };
    }

    pub fn geometry(&self) -> &PortalGeometry {
        return &self.geometry;
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn link_url(&self) -> &Url {
        return &self.link;
    }

    pub fn draw<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        self.model.draw(rp);
    }

    pub fn sweep(
        &self,
        pos: &Isometry<f32>,
        vel: &Vector<f32>,
        shape: &Cuboid,
        max_toi: f32,
    ) -> Option<ShapeCastHit> {
        return cast_shapes(
            pos,
            vel,
            shape,
            &Isometry::identity(),
            &Vector::zeros(),
            &self.collider,
            ShapeCastOptions::with_max_time_of_impact(max_toi),
        )
        .unwrap();
    }

    pub fn link(&self, cache: &mut LevelCache) -> Option<PortalLink> {
        let fragment = self.link.fragment()?;
        let mut url = self.link.clone();
        url.set_fragment(None);

        let LevelCacheResult::Ready(level) = cache.get(&url) else {
            return None;
        };
        let dst_portal = level.portal(fragment)?;

        if !self.geometry.matches(&dst_portal.geometry) {
            return None;
        }

        return Some(PortalLink::new(
            url,
            fragment.to_string(),
            self.geometry.clone(),
            dst_portal.geometry.clone(),
        ));
    }
}
