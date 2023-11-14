struct DofUniforms {
    focal_distance: f32
};

@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var s: sampler;

@group(0) @binding(2) var<uniform> uniforms: DofUniforms;

@fragment
fn fragment(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0);
}
