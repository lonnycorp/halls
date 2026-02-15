struct CameraUniform {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
    clip_plane: vec4<f32>,
}

// Group 0: Textures
@group(0) @binding(0)
var texture_sampler: sampler;
@group(0) @binding(1)
var render_target: texture_2d<f32>;

// Group 1: Config
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct PushConstants {
    open_factor: f32,
}
var<push_constant> pc: PushConstants;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) diffuse_uv: vec2<f32>,
    @location(2) lightmap_uv: vec2<f32>,
    @location(3) material_ix: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = vec4<f32>(in.position, 1.0);
    let view_position = camera.view * world_position;

    out.clip_position = camera.projection * view_position;
    out.world_position = world_position.xyz;
    out.uv = in.diffuse_uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Clip plane check (skip if clip_plane is zero)
    if (length(camera.clip_plane.xyz) > 0.0) {
        let dist = dot(in.world_position, camera.clip_plane.xyz) + camera.clip_plane.w;
        if (dist < 0.0) {
            discard;
        }
    }

    // Sample using screen coordinates
    let screen_size = vec2<f32>(textureDimensions(render_target));
    let sample_coord = in.clip_position.xy / screen_size;
    let sampled = textureSample(render_target, texture_sampler, sample_coord);

    let black = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    return mix(black, sampled, pc.open_factor);
}
