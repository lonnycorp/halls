// Avoid spawning a console window on Windows builds.
#![windows_subsystem = "windows"]

mod audio;
mod config;
mod gltf;
mod graphics;
mod level;
mod overlay;
mod parry3d;
mod player;
mod window;

use std::sync::Arc;
use std::time::{Duration, Instant};

use include_dir::include_dir;
use rodio::{OutputStream, Sink};

pub static ASSET: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/asset");
pub const WINDOW_TITLE: &str = "Halls";

use glam::{Mat4, Vec2, Vec4};
use graphics::pipeline::level::create_pipeline_level;
use graphics::pipeline::overlay::bind_group::PipelineOverlayBindGroupTexture;
use graphics::pipeline::overlay::create_pipeline_overlay;
use graphics::pipeline::portal::create_pipeline_portal;
use graphics::uniform::UniformCamera;

use config::Config;
use graphics::model::{Model, ModelBuffer};
use graphics::render_target::RenderTarget;
use level::cache::{LevelCache, LevelCacheResult};
use level::render::{LevelRenderContext, LevelRenderContextState, LevelRenderSchema};
use overlay::MenuHomeUpdateContext;
use player::Player;
use window::{Event, GPUContext, Window, WindowHandler, WindowOnEventContext};
use window::{InputController, KeyState};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

pub const SIM_STEP: Duration = Duration::from_millis(10);
pub const FONT_TEXTURE_INDEX: usize = 0;
pub const SYSTEM_TEXTURE_INDEX: usize = 1;
pub const TARGET_WIDTH: f32 = 1280.0;

const TEXT_TEXTURE_PATH: &str = "texture/text.png";
const SYSTEM_TEXTURE_PATH: &str = "texture/system.png";
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
    overlay_bind_group: PipelineOverlayBindGroupTexture,
    overlay_buffer: ModelBuffer,
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
    jukebox: audio::Jukebox,
    jingle_effect: audio::Effect,
    select_effect: audio::Effect,
    move_effect: audio::Effect,
    walk_effect: audio::Effect,
    player: Player,
    last_update: Instant,
}

struct Halls {
    state: Option<State>,
}

fn create_render_targets(
    device: &wgpu::Device,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    count: usize,
) -> Vec<RenderTarget> {
    return (0..count)
        .map(|_| RenderTarget::new(device, size, format))
        .collect();
}

fn update<'a>(
    state: &mut State,
    gpu: &mut GPUContext,
    input: &mut InputController<'a>,
    event_loop: &ActiveEventLoop,
) {
    let input_ref: &InputController<'a> = input;

    state.master_sink.set_volume(state.config.volume);
    state.status.swap();
    state.cache.update();
    state.jukebox.update(&state.player, &mut state.cache);

    let (width, height) = gpu.size();
    let scale = (width as f32 / TARGET_WIDTH).floor().max(1.0);
    let resolution = Vec2::new(width as f32, height as f32) / scale;
    state.overlay_buffer.clear();
    state.intro.update(&mut overlay::IntroUpdateContext {
        buffer: &mut state.overlay_buffer,
        resolution,
        input: input_ref,
        status: &mut state.status,
        jingle_effect: &state.jingle_effect,
    });
    state.menu.update(&mut MenuHomeUpdateContext {
        event_loop,
        buffer: &mut state.overlay_buffer,
        resolution,
        input: input_ref,
        status: &mut state.status,
        select_effect: &state.select_effect,
        move_effect: &state.move_effect,
    });
    state
        .menu_settings
        .update(&mut overlay::MenuSettingsUpdateContext {
            buffer: &mut state.overlay_buffer,
            resolution,
            input: input_ref,
            status: &mut state.status,
            config: &mut state.config,
            select_effect: &state.select_effect,
            move_effect: &state.move_effect,
        });
    state
        .menu_visit
        .update(&mut overlay::MenuVisitUpdateContext {
            buffer: &mut state.overlay_buffer,
            resolution,
            input: input_ref,
            status: &mut state.status,
            player: &mut state.player,
            cache: &mut state.cache,
            select_effect: &state.select_effect,
            move_effect: &state.move_effect,
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
        .upload(&gpu.queue, &state.overlay_buffer);
    if matches!(state.status.get(), Status::Simulation) {
        if let KeyState::Pressed = input_ref.key(KeyCode::Escape) {
            state.move_effect.reset();
            state.move_effect.play();
            state.walk_effect.pause();
            state.status.set(Status::MenuHome);
        } else {
            state
                .player
                .update(input_ref, &mut state.cache, &state.config);
            if state.player.is_walking() {
                state.walk_effect.play();
            }
        }
    }

    if !state.player.is_walking() {
        state.walk_effect.pause();
    }
    input.reset();
    state.tick = state.tick.wrapping_add(1);
}

fn render(state: &mut State, gpu: &GPUContext) {
    let output = gpu.current_texture();
    let color_view = output.texture.create_view(&Default::default());
    let mut encoder = gpu.device.create_command_encoder(&Default::default());

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
            let mut render_ctx_state = LevelRenderContextState::default();

            level.render(LevelRenderContext {
                device: &gpu.device,
                queue: &gpu.queue,
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

        state.overlay_bind_group.bind(&mut rp);
        rp.set_pipeline(&state.pipeline_overlay);
        state.overlay_model.draw(&mut rp);
    }

    gpu.queue.submit([encoder.finish()]);
    output.present();
}

impl Halls {
    fn new() -> Self {
        return Self { state: None };
    }
}

impl WindowHandler for Halls {
    fn on_event(&mut self, ctx: &mut WindowOnEventContext<'_>, event: Event) {
        match event {
            Event::Resume => {
                let config = Config::load();

                let pipeline_level = create_pipeline_level(&ctx.gpu.device, ctx.gpu.format());
                let pipeline_portal = create_pipeline_portal(&ctx.gpu.device, ctx.gpu.format());
                let pipeline_overlay = create_pipeline_overlay(&ctx.gpu.device, ctx.gpu.format());

                let (width, height) = ctx.gpu.size();
                let depth_texture =
                    graphics::texture::TextureDepth::new(&ctx.gpu.device, width, height);

                let text_image =
                    image::load_from_memory(ASSET.get_file(TEXT_TEXTURE_PATH).unwrap().contents())
                        .unwrap()
                        .to_rgba8();
                let system_image = image::load_from_memory(
                    ASSET.get_file(SYSTEM_TEXTURE_PATH).unwrap().contents(),
                )
                .unwrap()
                .to_rgba8();
                let overlay_texture =
                    graphics::texture::TextureArray::new(&ctx.gpu.device, (512, 512), 2);
                overlay_texture.write_texture(&ctx.gpu.queue, FONT_TEXTURE_INDEX, &text_image);
                overlay_texture.write_texture(&ctx.gpu.queue, SYSTEM_TEXTURE_INDEX, &system_image);
                let overlay_bind_group = PipelineOverlayBindGroupTexture::new(
                    &ctx.gpu.device,
                    &overlay_texture,
                    wgpu::FilterMode::Nearest,
                );
                let overlay_buffer = ModelBuffer::new();
                let overlay_model = Model::new(&ctx.gpu.device, 50_000);
                let intro = overlay::Intro::new();
                let menu = overlay::MenuHome::new();
                let menu_settings = overlay::MenuSettings::new(&config);
                let menu_visit = overlay::MenuVisit::new(&config);

                let camera = UniformCamera::new(&ctx.gpu.device, 64);
                let projection =
                    Mat4::perspective_rh(75f32.to_radians(), ctx.gpu.aspect(), 0.05, 1000.0);

                let render_targets =
                    create_render_targets(&ctx.gpu.device, ctx.gpu.size(), ctx.gpu.format(), 6);

                let mut cache =
                    LevelCache::new(Arc::clone(&ctx.gpu.device), Arc::clone(&ctx.gpu.queue), 8);
                cache.get(&config.default_url);

                let (_audio_stream, audio) = OutputStream::try_default().unwrap();
                let (mixer_ctrl, mixer_src) = rodio::dynamic_mixer::mixer::<f32>(2, 44100);
                let master_sink = Sink::try_new(&audio).unwrap();
                master_sink.append(mixer_src);
                let jukebox = audio::Jukebox::new(&mixer_ctrl);
                let jingle_track = audio::TrackData::new(
                    ASSET.get_file(JINGLE_AUDIO_PATH).unwrap().contents(),
                    false,
                )
                .unwrap();
                let jingle_effect = audio::Effect::new(&mixer_ctrl, jingle_track);
                let select_track = audio::TrackData::new(
                    ASSET.get_file(SELECT_AUDIO_PATH).unwrap().contents(),
                    false,
                )
                .unwrap();
                let select_effect = audio::Effect::new(&mixer_ctrl, select_track);
                let move_track = audio::TrackData::new(
                    ASSET.get_file(MOVE_AUDIO_PATH).unwrap().contents(),
                    false,
                )
                .unwrap();
                let move_effect = audio::Effect::new(&mixer_ctrl, move_track);
                let walk_track = audio::TrackData::new(
                    ASSET.get_file(WALK_AUDIO_PATH).unwrap().contents(),
                    true,
                )
                .unwrap();
                let walk_effect = audio::Effect::new(&mixer_ctrl, walk_track);
                let player = Player::new(glam::Vec3::ZERO);
                let last_update = Instant::now();

                self.state = Some(State {
                    config,
                    status: StatusBuffer::new(Status::Intro),
                    pipeline_level,
                    pipeline_portal,
                    pipeline_overlay,
                    depth_texture,
                    overlay_bind_group,
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
                    jukebox,
                    jingle_effect,
                    select_effect,
                    move_effect,
                    walk_effect,
                    player,
                    last_update,
                });
            }
            Event::Resize { width, height } => {
                let Some(ref mut state) = self.state else {
                    return;
                };
                if width > 0 && height > 0 {
                    state.depth_texture =
                        graphics::texture::TextureDepth::new(&ctx.gpu.device, width, height);
                    state.render_targets = create_render_targets(
                        &ctx.gpu.device,
                        (width, height),
                        ctx.gpu.format(),
                        6,
                    );
                    state.projection =
                        Mat4::perspective_rh(75f32.to_radians(), ctx.gpu.aspect(), 0.05, 1000.0);
                }
            }
            Event::Redraw => {
                let Some(ref mut state) = self.state else {
                    return;
                };
                let now = Instant::now();
                if now - state.last_update > Duration::from_millis(200) {
                    state.last_update = now;
                }
                while state.last_update.elapsed() >= SIM_STEP {
                    update(state, ctx.gpu, &mut ctx.input, ctx.event_loop);
                    state.last_update += SIM_STEP;
                }
                render(state, ctx.gpu);
            }
        }
    }
}

fn main() {
    Window::new(Halls::new()).run();
}
