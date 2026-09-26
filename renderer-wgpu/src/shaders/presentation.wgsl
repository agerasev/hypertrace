// Shared 128-byte Params ABI from scene.rs and trace.wgsl.
struct Params {
    camera0: vec4<f32>, camera1: vec4<f32>,
    background0: vec4<f32>, background1: vec4<f32>, background_axis: vec4<f32>,
    info: vec4<u32>, options: vec4<u32>, misc: vec4<f32>,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> accumulation: array<vec4<f32>>;
override ATTACHMENT_SRGB: bool = false;

@vertex
fn fullscreen(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let corner = vec2<f32>(f32((index << 1u) & 2u), f32(index & 2u));
    return vec4<f32>(corner * 2.0 - 1.0, 0.0, 1.0);
}

@fragment
fn display(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let pixel = vec2<u32>(position.xy);
    let sum = accumulation[pixel.y * params.info.x + pixel.x];
    var display_rgb = vec3<f32>(0.0);
    if sum.w > 0.0 {
        display_rgb = pow(max(sum.xyz / sum.w, vec3<f32>(0.0)), vec3<f32>(1.0 / 2.2));
    }
    if ATTACHMENT_SRGB {
        // The attachment then encodes sRGB, restoring the legacy display gamma.
        display_rgb = select(
            display_rgb / 12.92,
            pow((display_rgb + 0.055) / 1.055, vec3<f32>(2.4)),
            display_rgb > vec3<f32>(0.04045),
        );
    }
    return vec4<f32>(display_rgb, 1.0);
}
