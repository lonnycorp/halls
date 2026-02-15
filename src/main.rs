// Avoid spawning a console window on Windows builds.
#![windows_subsystem = "windows"]

mod audio;
mod color;
mod config;
mod gltf;
mod graphics;
mod level;
mod overlay;
mod player;
mod window;

use std::sync::Arc;
use std::time::{Duration, Instant};

use include_dir::include_dir;
use rodio::{OutputStream, Sink};

pub static ASSET: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/asset");
pub const WINDOW_TITLE: &str = "Halls";

use glam::{Mat4, Vec2, Vec4};
use graphics::pipeline::level::pipeline_level_create;
use graphics::pipeline::overlay::bind_group::{
    PipelineOverlayBindGroupConfig, PipelineOverlayBindGroupTexture,
};
use graphics::pipeline::overlay::pipeline_overlay_create;
use graphics::pipeline::portal::pipeline_portal_create;
use graphics::uniform::UniformCamera;

use config::Config;
use graphics::model::{Model, ModelVertex};
use graphics::render_target::RenderTarget;
use level::cache::{LevelCache, LevelCacheResult};
use level::{LevelRenderParams, LevelRenderSchema, LevelRenderState};
use overlay::MenuHomeUpdateParams;
use player::Player;
use window::WindowKeyState;
use window::{Window, WindowContext, WindowHandler, WindowHandlerEvent};
use winit::keyboard::{Key, NamedKey};

pub const SIM_STEP: Duration = Duration::from_millis(10);
pub const TARGET_WIDTH: f32 = 1280.0;

const JINGLE_AUDIO_PATH: &str = "audio/jingle.wav";
const SELECT_AUDIO_PATH: &str = "audio/select.wav";
const MOVE_AUDIO_PATH: &str = "audio/move.wav";
const WALK_AUDIO_PATH: &str = "audio/walk.wav";

#[derive(PartialEq)]
pub(crate) enum Status {
    Intro,
    MenuHome,
    MenuVisit,
    MenuSettings,
    Simulation,
}

pub(crate) struct StatusBuffer {
    current: Status,
    next: Option<Status>,
}

impl StatusBuffer {
    fn new(status: Status) -> Self {
        return Self {
            current: status,
            next: None,
        };
    }

    pub fn get(&self) -> &Status {
        return &self.current;
    }

    pub fn set(&mut self, status: Status) {
        self.next = Some(status);
    }

    fn swap(&mut self) {
        if let Some(next) = self.next.take() {
            self.current = next;
        }
    }
}

struct State {
    config: Config,
    status: StatusBuffer,
    pipeline_level: wgpu::RenderPipeline,
    pipeline_portal: wgpu::RenderPipeline,
    pipeline_overlay: wgpu::RenderPipeline,
    depth_texture: graphics::texture::TextureDepth,
    overlay_bind_group_texture: PipelineOverlayBindGroupTexture,
    overlay_bind_group_config: PipelineOverlayBindGroupConfig,
    overlay_buffer: Vec<ModelVertex>,
    overlay_model: Model,
    intro: overlay::Intro,
    menu: overlay::MenuHome,
    menu_settings: overlay::MenuSettings,
    menu_visit: overlay::MenuVisit,
    camera: UniformCamera,
    tick: u32,
    projection: Mat4,
    render_targets: Vec<RenderTarget>,
    cache: LevelCache,
    _audio_stream: OutputStream,
    master_sink: Sink,
    cross_fader: audio::CrossFader,
    jingle_track: audio::Track,
    select_track: audio::Track,
    move_track: audio::Track,
    walk_track: audio::Track,
    player: Player,
    last_update: Instant,
}

struct Halls {
    state: Option<State>,
}

fn create_render_targets(
    device: &wgpu::Device,
    size: Vec2,
    format: wgpu::TextureFormat,
    count: usize,
) -> Vec<RenderTarget> {
    let width = size.x as u32;
    let height = size.y as u32;

    return (0..count)
        .map(|_| RenderTarget::new(device, (width, height), format))
        .collect();
}

fn update(state: &mut State, ctx: &mut WindowContext<'_>) {
    state.master_sink.set_volume(state.config.volume);
    state.status.swap();
    state.cache.update();
    state.cross_fader.update(&state.player, &mut state.cache);

    let size = ctx.size();
    let scale = (size.x / TARGET_WIDTH).floor().max(1.0);
    let resolution = size / scale;

    state.overlay_buffer.clear();
    state.intro.update(&mut overlay::IntroUpdateParams {
        buffer: &mut state.overlay_buffer,
        resolution,
        window: ctx,
        status: &mut state.status,
        jingle_track: &state.jingle_track,
    });
    state.menu.update(&mut MenuHomeUpdateParams {
        buffer: &mut state.overlay_buffer,
        resolution,
        window: ctx,
        status: &mut state.status,
        select_track: &state.select_track,
        move_track: &state.move_track,
    });
    state
        .menu_settings
        .update(&mut overlay::MenuSettingsUpdateParams {
            buffer: &mut state.overlay_buffer,
            resolution,
            window: ctx,
            status: &mut state.status,
            config: &mut state.config,
            select_track: &state.select_track,
            move_track: &state.move_track,
        });
    state
        .menu_visit
        .update(&mut overlay::MenuVisitUpdateParams {
            buffer: &mut state.overlay_buffer,
            resolution,
            window: ctx,
            status: &mut state.status,
            player: &mut state.player,
            cache: &mut state.cache,
            select_track: &state.select_track,
            move_track: &state.move_track,
        });
    match state.status.get() {
        Status::MenuHome | Status::MenuVisit | Status::MenuSettings => {
            overlay::update_banner(
                &mut state.overlay_buffer,
                resolution,
                &state.player,
                &mut state.cache,
            );
        }
        _ => {}
    }

    state
        .overlay_model
        .upload(ctx.queue(), &state.overlay_buffer)
        .unwrap();
    if matches!(state.status.get(), Status::Simulation) {
        if let WindowKeyState::Pressed = ctx.key(&Key::Named(NamedKey::Escape)) {
            state.move_track.reset();
            state.move_track.play();
            state.walk_track.pause();
            state.status.set(Status::MenuHome);
        } else {
            state.player.update(ctx, &mut state.cache, &state.config);
            if state.player.is_walking() {
                state.walk_track.play();
            }
        }
    }

    if !state.player.is_walking() {
        state.walk_track.pause();
    }
    ctx.input_reset();
    state.tick = state.tick.wrapping_add(1);
}

fn render(state: &mut State, ctx: &WindowContext<'_>) {
    let output = ctx.current_texture();
    let color_view = output.texture.create_view(&Default::default());
    let mut encoder = ctx.device().create_command_encoder(&Default::default());

    {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &color_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });
    }

    if let Some(level_url) = state.player.level_url() {
        if let LevelCacheResult::Ready(level) = state.cache.get(level_url) {
            let eye = state.player.eye_position();
            let rot = state.player.rotation();
            let mut render_ctx_state = LevelRenderState::default();

            level.render(LevelRenderParams {
                device: ctx.device(),
                queue: ctx.queue(),
                encoder: &mut encoder,
                camera: &state.camera,
                tick: state.tick,
                projection: state.projection,
                render_targets: &state.render_targets,
                cache: &mut state.cache,
                state: &mut render_ctx_state,
                pipeline_level: &state.pipeline_level,
                pipeline_portal: &state.pipeline_portal,
                color_view: &color_view,
                depth_view: state.depth_texture.view(),
                eye,
                player_rotation: rot,
                clip: Vec4::ZERO,
                schema: LevelRenderSchema::Current {
                    last_portal: state.player.last_portal().cloned(),
                    open_factor: state.player.open_factor(),
                },
                skip_portal: None,
            });
        }
    }

    {
        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Banner Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &color_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });

        rp.set_pipeline(&state.pipeline_overlay);
        state.overlay_bind_group_texture.bind(&mut rp);
        state.overlay_bind_group_config.bind(&mut rp);
        state.overlay_model.draw(&mut rp);
    }

    ctx.queue().submit([encoder.finish()]);
    output.present();
}

impl Halls {
    fn new() -> Self {
        return Self { state: None };
    }
}

impl WindowHandler for Halls {
    fn on_event(&mut self, ctx: &mut WindowContext<'_>, event: WindowHandlerEvent) {
        match event {
            WindowHandlerEvent::Resume => {
                let config = Config::load();
                let size = ctx.size();

                let pipeline_level = pipeline_level_create(ctx.device(), ctx.format());
                let pipeline_portal = pipeline_portal_create(ctx.device(), ctx.format());
                let pipeline_overlay = pipeline_overlay_create(ctx.device(), ctx.format());

                let depth_texture = graphics::texture::TextureDepth::new(
                    ctx.device(),
                    size.x as u32,
                    size.y as u32,
                );

                let overlay_bind_group_texture =
                    PipelineOverlayBindGroupTexture::new(ctx.device(), ctx.queue());
                let overlay_bind_group_config =
                    PipelineOverlayBindGroupConfig::new(ctx.device(), ctx.queue());
                let overlay_buffer: Vec<ModelVertex> = Vec::new();
                let overlay_model = Model::new(ctx.device(), 50_000);
                let intro = overlay::Intro::new();
                let menu = overlay::MenuHome::new();
                let menu_settings = overlay::MenuSettings::new(&config);
                let menu_visit = overlay::MenuVisit::new(&config);

                let camera = UniformCamera::new(ctx.device(), 64);
                let projection =
                    Mat4::perspective_rh(75f32.to_radians(), size.x / size.y, 0.05, 1000.0);

                let render_targets = create_render_targets(ctx.device(), size, ctx.format(), 6);

                let mut cache =
                    LevelCache::new(Arc::clone(ctx.device()), Arc::clone(ctx.queue()), 8);
                cache.get(&config.default_url);

                let (_audio_stream, audio) = OutputStream::try_default().unwrap();
                let (mixer_ctrl, mixer_src) = rodio::dynamic_mixer::mixer::<f32>(2, 44100);
                let master_sink = Sink::try_new(&audio).unwrap();
                master_sink.append(mixer_src);
                let mut cross_fader = audio::CrossFader::new();
                mixer_ctrl.add(cross_fader.source());
                let jingle_track_data = audio::TrackData::new(
                    ASSET.get_file(JINGLE_AUDIO_PATH).unwrap().contents(),
                    false,
                )
                .unwrap();
                let jingle_track = audio::Track::new(jingle_track_data);
                mixer_ctrl.add(jingle_track.source());
                let select_track_data = audio::TrackData::new(
                    ASSET.get_file(SELECT_AUDIO_PATH).unwrap().contents(),
                    false,
                )
                .unwrap();
                let select_track = audio::Track::new(select_track_data);
                mixer_ctrl.add(select_track.source());
                let move_track_data = audio::TrackData::new(
                    ASSET.get_file(MOVE_AUDIO_PATH).unwrap().contents(),
                    false,
                )
                .unwrap();
                let move_track = audio::Track::new(move_track_data);
                mixer_ctrl.add(move_track.source());
                let walk_track_data = audio::TrackData::new(
                    ASSET.get_file(WALK_AUDIO_PATH).unwrap().contents(),
                    true,
                )
                .unwrap();
                let walk_track = audio::Track::new(walk_track_data);
                mixer_ctrl.add(walk_track.source());
                let player = Player::new(glam::Vec3::ZERO);
                let last_update = Instant::now();

                self.state = Some(State {
                    config,
                    status: StatusBuffer::new(Status::Intro),
                    pipeline_level,
                    pipeline_portal,
                    pipeline_overlay,
                    depth_texture,
                    overlay_bind_group_texture,
                    overlay_bind_group_config,
                    overlay_buffer,
                    overlay_model,
                    intro,
                    menu,
                    menu_settings,
                    menu_visit,
                    camera,
                    tick: 0,
                    projection,
                    render_targets,
                    cache,
                    _audio_stream,
                    master_sink,
                    cross_fader,
                    jingle_track,
                    select_track,
                    move_track,
                    walk_track,
                    player,
                    last_update,
                });
            }
            WindowHandlerEvent::Resize => {
                let Some(ref mut state) = self.state else {
                    return;
                };

                let size = ctx.size();
                if size.x > 0.0 && size.y > 0.0 {
                    state.depth_texture = graphics::texture::TextureDepth::new(
                        ctx.device(),
                        size.x as u32,
                        size.y as u32,
                    );
                    state.render_targets =
                        create_render_targets(ctx.device(), size, ctx.format(), 6);
                    state.projection =
                        Mat4::perspective_rh(75f32.to_radians(), size.x / size.y, 0.05, 1000.0);
                }
            }
            WindowHandlerEvent::Redraw => {
                let Some(ref mut state) = self.state else {
                    return;
                };
                let now = Instant::now();
                if now - state.last_update > Duration::from_millis(200) {
                    state.last_update = now;
                }
                while state.last_update.elapsed() >= SIM_STEP {
                    update(state, ctx);
                    state.last_update += SIM_STEP;
                }
                render(state, ctx);
            }
        }
    }
}

fn main() {
    Window::new(Halls::new()).run();
}
