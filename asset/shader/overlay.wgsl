// Group 0: Textures
@group(0) @binding(0)
var diffuse_sampler: sampler;
@group(0) @binding(1)
var diffuse: texture_2d_array<f32>;

struct MaterialEntry {
    num_frames: u32,
    speed: f32,
    offset: u32,
    color: u32,
    unlit: u32,
}

struct MaterialIndex {
    entries: array<MaterialEntry, 512>,
    frames: array<u32, 4096>,
    next_free: u32,
}

// Group 1: Config
@group(1) @binding(0)
var<storage, read> material_index: MaterialIndex;

fn unpack_layer(texture_ref: u32) -> u32 {
    return (texture_ref >> 16u) & 0xFFFFu;
}

fn unpack_color(color: u32) -> vec4<f32> {
    let r = f32(color & 0xFFu) / 255.0;
    let g = f32((color >> 8u) & 0xFFu) / 255.0;
    let b = f32((color >> 16u) & 0xFFu) / 255.0;
    let a = f32((color >> 24u) & 0xFFu) / 255.0;
    return vec4<f32>(r, g, b, a);
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) diffuse_uv: vec2<f32>,
    @location(2) lightmap_uv: vec2<f32>,
    @location(3) material_ix: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) diffuse_uv: vec2<f32>,
    @location(1) @interpolate(flat) material_ix: u32,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    let tex_size = vec2<f32>(textureDimensions(diffuse));
    out.diffuse_uv = in.diffuse_uv / tex_size;
    out.material_ix = in.material_ix;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let material = material_index.entries[in.material_ix];
    let texture_ref = material_index.frames[material.offset];
    let layer_ix = unpack_layer(texture_ref);
    let sampled = textureSample(diffuse, diffuse_sampler, in.diffuse_uv, layer_ix);
    return sampled * unpack_color(material.color);
}
