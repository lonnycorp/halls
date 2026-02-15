use glam::{Mat4, Vec2, Vec3, Vec4};
use url::Url;

use super::state::LevelState;
use crate::graphics::pipeline::level::bind_level_constants;
use crate::graphics::pipeline::level::PipelineLevelBindGroupConfig;
use crate::graphics::pipeline::portal::{
    bind_portal_constants, PipelinePortalBindGroupConfig, PipelinePortalBindGroupTexture,
};
use crate::graphics::render_target::RenderTarget;
use crate::graphics::uniform::{UniformCamera, UniformCameraData};
use crate::level::cache::{LevelCache, LevelCacheResult};

const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

#[derive(Default)]
pub struct LevelRenderState {
    pub camera: u32,
    pub render_target: u32,
}

pub struct LevelRenderParams<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub camera: &'a UniformCamera,
    pub tick: u32,
    pub projection: Mat4,
    pub render_targets: &'a [RenderTarget],
    pub cache: &'a mut LevelCache,
    pub state: &'a mut LevelRenderState,
    pub pipeline_level: &'a wgpu::RenderPipeline,
    pub pipeline_portal: &'a wgpu::RenderPipeline,
    pub color_view: &'a wgpu::TextureView,
    pub depth_view: &'a wgpu::TextureView,
    pub eye: Vec3,
    pub player_rotation: Vec2,
    pub clip: Vec4,
    pub schema: LevelRenderSchema,
    pub skip_portal: Option<&'a str>,
}

pub enum LevelRenderSchema {
    Current {
        last_portal: Option<(Url, String)>,
        open_factor: f32,
    },
    Last {
        open_factor: f32,
    },
    Other,
}

pub fn level_render(level_state: &LevelState, params: LevelRenderParams) {
    let material_data = &level_state.material_data;
    let level_bind_group_config = PipelineLevelBindGroupConfig::new(
        params.device,
        params.camera,
        &material_data.material_index,
    );
    let portal_bind_group_config = PipelinePortalBindGroupConfig::new(params.device, params.camera);

    let mut camera_data = UniformCameraData::new();
    camera_data.projection = params.projection;
    camera_data.clip_plane = params.clip;
    camera_data.view_set(params.eye, params.player_rotation);
    let camera_offset = params
        .camera
        .write(params.queue, params.state.camera, &camera_data);
    params.state.camera += 1;

    {
        let mut rp = params
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: params.color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: params.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

        rp.set_pipeline(params.pipeline_level);
        material_data.texture_bind_group.bind(&mut rp);
        level_bind_group_config.bind(&mut rp, camera_offset);
        bind_level_constants(&mut rp, params.tick, material_data.lightmap_material_id);
        level_state.model.draw(&mut rp);
    }

    let rt = &params.render_targets[params.state.render_target as usize];
    let portal_texture_bind_group = PipelinePortalBindGroupTexture::new(params.device, rt);
    params.state.render_target += 1;

    for (name, src_portal) in level_state.portals.iter() {
        if params.skip_portal == Some(name.as_str()) {
            continue;
        }
        let link = src_portal.link(params.cache);

        let (open_factor, recurse_schema) = if let Some(link) = &link {
            let portal_matches = match &params.schema {
                LevelRenderSchema::Current {
                    last_portal: Some((url, portal)),
                    ..
                } => url == link.url() && portal == link.name(),
                _ => false,
            };

            match &params.schema {
                &LevelRenderSchema::Current { open_factor, .. } if portal_matches => (
                    1.0,
                    Some(LevelRenderSchema::Last {
                        open_factor: -open_factor,
                    }),
                ),
                &LevelRenderSchema::Current { open_factor, .. } if open_factor > 0.0 => {
                    (open_factor, Some(LevelRenderSchema::Other))
                }
                &LevelRenderSchema::Current { open_factor, .. } => (open_factor.min(0.0), None),
                &LevelRenderSchema::Last { open_factor } if open_factor > 0.0 => {
                    (open_factor, Some(LevelRenderSchema::Other))
                }
                &LevelRenderSchema::Last { open_factor } => (open_factor.min(0.0), None),
                LevelRenderSchema::Other => (0.0, None),
            }
        } else {
            (0.0, None)
        };

        let dst_level =
            link.as_ref()
                .zip(recurse_schema.as_ref())
                .and_then(|(link, _)| match params.cache.get(link.url()) {
                    LevelCacheResult::Ready(level) => Some(level),
                    _ => None,
                });

        if let (Some(next_schema), Some(dst_level)) = (recurse_schema, dst_level) {
            let link = link.unwrap();
            let src_geometry = src_portal.geometry();

            let yaw_delta = link.yaw_delta();
            let dst_normal = link.dst_normal();
            let eye_side = (params.eye - src_geometry.center())
                .dot(src_geometry.normal())
                .signum();
            let clip_normal = dst_normal * eye_side;

            level_render(
                &dst_level.state,
                LevelRenderParams {
                    device: params.device,
                    queue: params.queue,
                    encoder: &mut *params.encoder,
                    camera: params.camera,
                    tick: params.tick,
                    projection: params.projection,
                    render_targets: params.render_targets,
                    cache: &mut *params.cache,
                    state: &mut *params.state,
                    pipeline_level: params.pipeline_level,
                    pipeline_portal: params.pipeline_portal,
                    color_view: rt.color_view(),
                    depth_view: rt.depth_view(),
                    eye: link.position_transform(params.eye, false),
                    player_rotation: Vec2::new(
                        params.player_rotation.x,
                        params.player_rotation.y + yaw_delta,
                    ),
                    clip: Vec4::new(
                        clip_normal.x,
                        clip_normal.y,
                        clip_normal.z,
                        -clip_normal.dot(link.dst_center()),
                    ),
                    schema: next_schema,
                    skip_portal: Some(link.name()),
                },
            );

            let mut camera_data = UniformCameraData::new();
            camera_data.projection = params.projection;
            camera_data.clip_plane = params.clip;
            camera_data.view_set(params.eye, params.player_rotation);
            let camera_offset =
                params
                    .camera
                    .write(params.queue, params.state.camera, &camera_data);
            params.state.camera += 1;

            {
                let mut rp = params
                    .encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: params.color_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: params.depth_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None,
                        }),
                        ..Default::default()
                    });

                rp.set_pipeline(params.pipeline_portal);
                portal_texture_bind_group.bind(&mut rp);
                portal_bind_group_config.bind(&mut rp, camera_offset);
                bind_portal_constants(&mut rp, open_factor);
                src_portal.draw(&mut rp);
            }
        } else {
            let mut camera_data = UniformCameraData::new();
            camera_data.projection = params.projection;
            camera_data.clip_plane = params.clip;
            camera_data.view_set(params.eye, params.player_rotation);
            let camera_offset =
                params
                    .camera
                    .write(params.queue, params.state.camera, &camera_data);
            params.state.camera += 1;

            {
                let mut rp = params
                    .encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: params.color_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: params.depth_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None,
                        }),
                        ..Default::default()
                    });

                rp.set_pipeline(params.pipeline_portal);
                portal_texture_bind_group.bind(&mut rp);
                portal_bind_group_config.bind(&mut rp, camera_offset);
                bind_portal_constants(&mut rp, 0.0);
                src_portal.draw(&mut rp);
            }
        }
    }
}
