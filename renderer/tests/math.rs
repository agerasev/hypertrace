//! Active half-space chart helpers used by material patterns.
use hypertrace_renderer::{Gpu, read_buffer};
const CASES: usize = 4;
const REGRESSION_SHADER: &str = r#"
@group(0) @binding(0) var<storage,read_write> results: array<vec4<f32>>;
@compute @workgroup_size(1) fn math_regression() {
    let origin=vec4<f32>(1,0,0,0);
    let shift=geo_translation(vec3<f32>(1,0,0),0.6);
    results[0]=vec4<f32>(geo_to_half_space_pos(geo_map_apply(shift,origin)),0);
    let quarter=geo_rotation(vec3<f32>(0,0,1),PI/2);
    // Half-space (1,0,1) embedded analytically.
    results[1]=vec4<f32>(geo_to_half_space_pos(geo_map_apply(quarter,vec4<f32>(1.5,1,0,0.5))),0);
    let rotation=geo_rotation(vec3<f32>(0,0,1),0.8);
    let p=vec4<f32>(1.25,0.75,0,0);
    results[2]=geo_map_apply(geo_chain(shift,rotation),p)-geo_map_apply(shift,geo_map_apply(rotation,p));
    // Parabolic x/y translation encoded directly in the canonical pair.
    let translate=GeoMap(vec4<f32>(1,1.5,-1,0),vec4<f32>(0,1,1.5,0));
    results[3]=vec4<f32>(geo_to_half_space_pos(geo_map_apply(translate,origin)),0);

}
"#;

fn dispatch_math() -> Vec<[f32; 4]> {
    let gpu =
        futures::executor::block_on(Gpu::headless()).expect("GPU math tests require an adapter");
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("production hyperbolic math regressions"),
            source: wgpu::ShaderSource::Wgsl(
                format!(
                    "const GEO_K:f32=-1.0;\n{}\n{}\n{}",
                    include_str!("../src/shaders/math.wgsl"),
                    include_str!("../src/shaders/embedded.wgsl"),
                    REGRESSION_SHADER
                )
                .into(),
            ),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("math regressions"),
            layout: None,
            module: &shader,
            entry_point: Some("math_regression"),
            compilation_options: Default::default(),
            cache: None,
        });
    let byte_len = (CASES * 16) as u64;
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("math regression results"),
        size: byte_len,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("math regression results"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: output.as_entire_binding(),
        }],
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    gpu.queue.submit([encoder.finish()]);
    let bytes = read_buffer(&gpu.device, &gpu.queue, &output, byte_len).expect("GPU readback");
    bytes
        .as_chunks::<16>()
        .0
        .iter()
        .map(|row| {
            std::array::from_fn(|i| f32::from_le_bytes(row[i * 4..i * 4 + 4].try_into().unwrap()))
        })
        .collect()
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn production_chart_helpers_preserve_composition_and_coordinates() {
    let rows = dispatch_math();
    let expected = [
        [0.6f64.tanh() as f32, 0.0, (1.0 / 0.6f64.cosh()) as f32, 0.0],
        [0.0, 1.0, 1.0, 0.0],
        [0.0; 4],
        [2.0, 3.0, 1.0, 0.0],
    ];
    for (actual, expected) in rows.iter().zip(expected) {
        for (&a, b) in actual.iter().zip(expected) {
            assert!(
                (a - b).abs() < 2e-6,
                "actual {actual:?}, expected {expected:?}"
            );
        }
    }
}
