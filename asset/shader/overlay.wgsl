// Group 0: Textures
@group(0) @binding(0)
var diffuse_sampler: sampler;
@group(0) @binding(1)
var diffuse: texture_2d_array<f32>;

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
    @location(1) @interpolate(flat) texture_ix: u32,
    @location(2) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    let tex_size = vec2<f32>(textureDimensions(diffuse));
    out.diffuse_uv = in.diffuse_uv / tex_size;
    out.texture_ix = in.texture_ix;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(diffuse, diffuse_sampler, in.diffuse_uv, in.texture_ix);
    return sampled * in.color;
}
