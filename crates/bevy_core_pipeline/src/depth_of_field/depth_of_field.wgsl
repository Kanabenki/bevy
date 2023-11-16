#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

struct DepthOfFieldUniforms {
    near_max: f32,
    near_min: f32,
    far_min: f32,
    far_max: f32,
}

@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;
//@group(0) @binding(2) var<uniform> uniforms: DepthOfFieldUniforms;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(input_texture, input_sampler, in.uv);
    return vec4<f32>(1.0 - color.r, 1.0 - color.g, 1.0 - color.b, 1.0 - color.a);
}
