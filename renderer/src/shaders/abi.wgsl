// Shared compute storage and uniform declarations.
struct Params {
    camera0: vec4<f32>, camera1: vec4<f32>,
    background0: vec4<f32>, background1: vec4<f32>, background_axis: vec4<f32>,
    info: vec4<u32>, options: vec4<u32>, misc: vec4<f32>, medium: vec4<f32>,
}
struct Object {
    map0: vec4<f32>, map1: vec4<f32>,
    info: vec4<u32>,
}
struct MaterialRecord { data: vec4<u32> }
