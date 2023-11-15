struct DepthOfFieldUniforms {
    near_max: f32,
    near_min: f32,
    far_min: f32,
    far_max: f32,
}

@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;
@group(0) @binding(2) var<uniform> uniforms: DepthOfFieldUniforms;

@fragment
fn fragment(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(input_texture, input_sampler, uv);
}
