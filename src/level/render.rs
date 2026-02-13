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
pub struct LevelRenderContextState {
    pub camera: u32,
    pub render_target: u32,
}

pub struct LevelRenderContext<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub camera: &'a UniformCamera,
    pub tick: u32,
    pub projection: Mat4,
    pub render_targets: &'a [RenderTarget],
    pub cache: &'a mut LevelCache,
    pub state: &'a mut LevelRenderContextState,
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

pub(super) fn level_render(state: &LevelState, ctx: LevelRenderContext) {
    let LevelRenderContext {
        device,
        queue,
        encoder,
        camera,
        tick,
        projection,
        render_targets,
        cache,
        state: render_state,
        pipeline_level,
        pipeline_portal,
        color_view,
        depth_view,
        eye,
        player_rotation,
        clip,
        schema,
        skip_portal,
    } = ctx;
    // Create bind groups for this frame
    let level_bind_group_config = PipelineLevelBindGroupConfig::new(
        device,
        camera,
        &state.texture_index,
        &state.material_index,
    );
    let portal_bind_group_config = PipelinePortalBindGroupConfig::new(device, camera);

    // Pass 1: Draw level geometry
    let mut camera_data = UniformCameraData::new();
    camera_data.projection = projection;
    camera_data.clip_plane = clip;
    camera_data.set_view(eye, player_rotation);
    let camera_offset = camera.write(queue, render_state.camera, &camera_data);
    render_state.camera += 1;

    {
        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        rp.set_pipeline(pipeline_level);
        state.texture_bind_group.bind(&mut rp);
        level_bind_group_config.bind(&mut rp, camera_offset);
        bind_level_constants(&mut rp, tick, state.lightmap_texture_id);
        state.model.draw(&mut rp);
    }

    let rt = &render_targets[render_state.render_target as usize];
    let portal_texture_bind_group = PipelinePortalBindGroupTexture::new(device, rt);
    render_state.render_target += 1;

    // Interleaved portal rendering
    for (name, src_portal) in state.portals.iter() {
        if skip_portal == Some(name.as_str()) {
            continue;
        }
        let link = src_portal.link(cache);

        let (open_factor, recurse_schema) = if let Some(link) = &link {
            let portal_matches = match &schema {
                LevelRenderSchema::Current {
                    last_portal: Some((url, portal)),
                    ..
                } => url == link.url() && portal == link.portal_name(),
                _ => false,
            };

            match &schema {
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
                .and_then(|(link, _)| match cache.get(link.url()) {
                    LevelCacheResult::Ready(level) => Some(level),
                    _ => None,
                });

        if let (Some(next_schema), Some(dst_level)) = (recurse_schema, dst_level) {
            // Render destination to render target from pool
            let link = link.unwrap();
            let src_geometry = src_portal.geometry();

            let yaw_delta = link.yaw_delta();
            let dst_normal = link.dst_normal();
            let eye_side = (eye - src_geometry.center())
                .dot(src_geometry.normal())
                .signum();
            let clip_normal = dst_normal * eye_side;

            level_render(
                &dst_level.state,
                LevelRenderContext {
                    device,
                    queue,
                    encoder,
                    camera,
                    tick,
                    projection,
                    render_targets,
                    cache,
                    state: render_state,
                    pipeline_level,
                    pipeline_portal,
                    color_view: rt.color_view(),
                    depth_view: rt.depth_view(),
                    eye: link.transform_position(eye, false),
                    player_rotation: Vec2::new(player_rotation.x, player_rotation.y + yaw_delta),
                    clip: Vec4::new(
                        clip_normal.x,
                        clip_normal.y,
                        clip_normal.z,
                        -clip_normal.dot(link.dst_center()),
                    ),
                    schema: next_schema,
                    skip_portal: Some(link.portal_name()),
                },
            );

            // Draw portal quad with RT texture
            let mut camera_data = UniformCameraData::new();
            camera_data.projection = projection;
            camera_data.clip_plane = clip;
            camera_data.set_view(eye, player_rotation);
            let camera_offset = camera.write(queue, render_state.camera, &camera_data);
            render_state.camera += 1;

            {
                let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: color_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });

                rp.set_pipeline(pipeline_portal);
                portal_texture_bind_group.bind(&mut rp);
                portal_bind_group_config.bind(&mut rp, camera_offset);
                bind_portal_constants(&mut rp, open_factor);
                src_portal.draw(&mut rp);
            }
        } else {
            // Draw portal as black (no recursion available)
            let mut camera_data = UniformCameraData::new();
            camera_data.projection = projection;
            camera_data.clip_plane = clip;
            camera_data.set_view(eye, player_rotation);
            let camera_offset = camera.write(queue, render_state.camera, &camera_data);
            render_state.camera += 1;

            {
                let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: color_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });

                rp.set_pipeline(pipeline_portal);
                portal_texture_bind_group.bind(&mut rp);
                portal_bind_group_config.bind(&mut rp, camera_offset);
                bind_portal_constants(&mut rp, 0.0);
                src_portal.draw(&mut rp);
            }
        }
    }
}
