struct CameraUniform {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
    clip_plane: vec4<f32>,
}

struct TextureEntry {
    bucket: u32,
    layer: u32,
}

struct TextureIndex {
    entries: array<TextureEntry, 1024>,
}

struct MaterialEntry {
    num_frames: u32,
    speed: f32,
    offset: u32,
}

struct MaterialIndex {
    entries: array<MaterialEntry, 512>,
    frames: array<u32, 4096>,
}

// Group 0: Textures
@group(0) @binding(0)
var diffuse_sampler: sampler;
@group(0) @binding(1)
var diffuse: binding_array<texture_2d_array<f32>, 6>;

// Group 1: Config
@group(1) @binding(0)
var<uniform> camera: CameraUniform;
@group(1) @binding(1)
var<storage, read> texture_index: TextureIndex;
@group(1) @binding(2)
var<storage, read> material_index: MaterialIndex;

struct PushConstants {
    clock: u32,
    lightmap_texture_id: u32,
}

var<push_constant> pc: PushConstants;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) diffuse_uv: vec2<f32>,
    @location(2) lightmap_uv: vec2<f32>,
    @location(3) texture_ix: u32,
    @location(4) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) diffuse_uv: vec2<f32>,
    @location(1) lightmap_uv: vec2<f32>,
    @location(2) world_position: vec3<f32>,
    @location(3) @interpolate(flat) texture_ix: u32,
    @location(4) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = vec4<f32>(in.position, 1.0);
    let view_position = camera.view * world_position;

    out.clip_position = camera.projection * view_position;
    out.diffuse_uv = in.diffuse_uv;
    out.lightmap_uv = in.lightmap_uv;
    out.world_position = world_position.xyz;
    out.texture_ix = in.texture_ix;
    out.color = in.color;
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

    // Look up material entry to get animation frame info
    let mat = material_index.entries[in.texture_ix];
    let t = pc.clock % (100u * mat.num_frames);
    let speed = clamp(mat.speed, 0.0, 1.0);
    let frame_offset = u32(floor(f32(t) * speed)) % mat.num_frames;
    let texture_id = material_index.frames[mat.offset + frame_offset];

    // Look up texture bucket and layer from texture_index
    let entry = texture_index.entries[texture_id];
    let array_ix = entry.bucket;
    let layer_ix = entry.layer;

    let diffuse_color = textureSample(diffuse[array_ix], diffuse_sampler, in.diffuse_uv, layer_ix);

    let lm_entry = texture_index.entries[pc.lightmap_texture_id];
    let lm_array_ix = lm_entry.bucket;
    let lm_layer_ix = lm_entry.layer;
    let light = textureSample(diffuse[lm_array_ix], diffuse_sampler, in.lightmap_uv, lm_layer_ix);

    return diffuse_color * light * in.color;
}
