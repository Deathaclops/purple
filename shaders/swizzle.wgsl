@group(0) @binding(0)
var input_tex: texture_2d<f32>;
@group(0) @binding(1)
var input_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}; // end VertexOutput

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2(-1.0, -3.0),
        vec2(3.0, 1.0),
        vec2(-1.0, 1.0),
    );
    let pos = positions[index];
    var out: VertexOutput;
    out.position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = vec2(0.5 * (pos.x + 1.0), 1.0 - 0.5 * (pos.y + 1.0));
    return out;
} // end vertex main

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let rgba = textureSample(input_tex, input_sampler, uv);
    return vec4(rgba.r, rgba.g, rgba.b, rgba.a);
} // end fragment main