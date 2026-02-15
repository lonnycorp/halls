use parry3d::math::{Isometry, Vector};
use parry3d::query::{cast_shapes, ShapeCastHit, ShapeCastOptions};
use parry3d::shape::{Cuboid, TriMesh};
use url::Url;

use crate::gltf::GLTFMesh;
use crate::graphics::model::Model;
use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::level::fetch::fetch;

use super::super::trimesh::trimesh_from_vertices;
use super::geometry::LevelPortalGeometry;
use super::LevelPortalLink;

#[derive(Debug)]
pub enum LevelPortalLoadError {
    URLJoin,
    Fetch,
    GLTF,
    GeometryFromGLTF,
    ModelUpload,
}

pub struct LevelPortal {
    geometry: LevelPortalGeometry,
    model: Model,
    collider: TriMesh,
    link: Url,
}

impl LevelPortal {
    pub fn new(geometry: LevelPortalGeometry, model: Model, collider: TriMesh, link: Url) -> Self {
        return Self {
            geometry,
            model,
            collider,
            link,
        };
    }

    pub fn load(
        base_url: &Url,
        mesh_href: &str,
        link_href: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self, LevelPortalLoadError> {
        let mesh_url = base_url
            .join(mesh_href)
            .map_err(|_| LevelPortalLoadError::URLJoin)?;
        let mesh_data = fetch(&mesh_url).map_err(|_| LevelPortalLoadError::Fetch)?;
        let portal_mesh =
            GLTFMesh::from_bytes(&mesh_data).map_err(|_| LevelPortalLoadError::GLTF)?;

        let link = base_url
            .join(link_href)
            .map_err(|_| LevelPortalLoadError::URLJoin)?;
        let geometry = LevelPortalGeometry::from_gltf(portal_mesh.vertices())
            .map_err(|_| LevelPortalLoadError::GeometryFromGLTF)?;

        let portal_vertices: Vec<_> = portal_mesh.vertices().collect();
        let portal_buffer: Vec<_> = portal_vertices
            .iter()
            .map(|vertex| vertex.to_model_vertex())
            .collect();
        let mut portal_model = Model::new(device, portal_mesh.vertex_count());
        portal_model
            .upload(queue, &portal_buffer)
            .map_err(|_| LevelPortalLoadError::ModelUpload)?;
        let portal_collider = trimesh_from_vertices(portal_vertices.into_iter());

        return Ok(Self::new(geometry, portal_model, portal_collider, link));
    }

    pub fn geometry(&self) -> &LevelPortalGeometry {
        return &self.geometry;
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

    pub fn link(&self, cache: &mut LevelCache) -> Option<LevelPortalLink> {
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

        return Some(LevelPortalLink::from_geometry_pair(
            url,
            fragment.to_string(),
            self.geometry.clone(),
            dst_portal.geometry.clone(),
        ));
    }
}
