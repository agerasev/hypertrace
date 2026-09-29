//! Runtime input fixtures are important here: an Intel Arc/Mesa 23.2.1 run
//! diverged in full tile reduction while constant/early-return probes agreed.
use hypertrace_examples as examples;
use hypertrace_renderer::{
    Gpu, Renderer, Scene, read_buffer,
    shader::{ObjectNode, ShaderKind, ShaderModule},
};
use objects::object::tiling::{Pentagonal, Pentastar, Tiling as _};
use wgpu::util::DeviceExt;

// Independent reference fixtures on the canonical hemisphere. Expected values
// select material zero or one, with two denoting the border.
// The first eight exercise the native regression; remaining points cover borders
// and the distinction between the star pattern and ordinary pentagons.
const FIXTURES: [([f32; 2], [u32; 2]); 12] = [
    ([-0.171875, -0.984375], [0, 0]),
    ([0.140625, -0.953125], [1, 1]),
    ([-0.421875, -0.796875], [0, 0]),
    ([0.390625, -0.796875], [1, 1]),
    ([0.328125, 0.796875], [0, 0]),
    ([-0.359375, 0.921875], [0, 1]),
    ([-0.234375, 0.953125], [1, 1]),
    ([0.078125, 0.984375], [0, 1]),
    ([-0.015625, -0.015625], [2, 2]),
    ([0.015625, -0.015625], [2, 2]),
    ([0.015625, 0.015625], [2, 2]),
    ([0.265625, -0.515625], [2, 0]),
];

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn production_tiling_matches_reference_fixtures() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let count = FIXTURES.len() as u32;
    let mut probe = ShaderModule::new(
        "tests::tiling_probe",
        ShaderKind::Library,
        r#"
fn {{self}}(gid: vec3<u32>) {
    if gid.x >= 12u || gid.y >= 2u { return; }
    let index = gid.x + 12u * gid.y;
    let chart = accumulation[index].xyz;
    // Independent half-space -> hyperboloid conversion for this fixture.
    let squared = dot(chart,chart);
    let position = vec4<f32>((squared+1)/(2*chart.z),chart.xy/chart.z,(squared-1)/(2*chart.z));
    var selected = 0u;
    if gid.y == 0u { selected = {{dep0}}(position,1.0,0.01,2u); }
    else { selected = {{dep1}}(position,1.0,0.02,2u); }
    accumulation[index].w = f32(selected);
}
@compute @workgroup_size(64)
fn tiling_regression(@builtin(global_invocation_id) gid: vec3<u32>) { {{self}}(gid); }
"#,
        Some(0),
    );
    probe.dependencies = vec![Pentastar::shader(), Pentagonal::shader()];
    let mut definition = examples::find("hy").unwrap().definition().unwrap();
    definition.object = ObjectNode::Vector(vec![]);
    definition.modules = vec![probe];
    let scene = Scene::from_definition(&definition).unwrap();
    let renderer = Renderer::new(&gpu.device, &gpu.queue, (1, 1), scene, 1).unwrap();
    let source = renderer.shader_source();
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("production tiling regression"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("production tiling regression"),
            layout: None,
            module: &shader,
            entry_point: Some("tiling_regression"),
            compilation_options: Default::default(),
            cache: None,
        });
    let points: Vec<[f32; 4]> = FIXTURES
        .iter()
        .cycle()
        .take(FIXTURES.len() * 2)
        .map(|([x, y], _)| [*x, *y, (1.0 - x * x - y * y).sqrt(), -1.0])
        .collect();
    let results = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("runtime hemisphere points and selected materials"),
            contents: bytemuck::cast_slice(&points),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("tiling regression buffers"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 3,
            resource: results.as_entire_binding(),
        }],
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(count.div_ceil(64), 2, 1);
    }
    gpu.queue.submit([encoder.finish()]);
    let bytes = read_buffer(&gpu.device, &gpu.queue, &results, points.len() as u64 * 16)
        .expect("tiling result readback");
    for (index, row) in bytes.as_chunks::<16>().0.iter().enumerate() {
        let actual = f32::from_le_bytes(row[12..16].try_into().unwrap());
        let fixture = index % FIXTURES.len();
        let tiling = index / FIXTURES.len();
        let expected = FIXTURES[fixture].1[tiling] as f32;
        assert_eq!(
            actual,
            expected,
            "tiling {tiling}, point {:?}, adapter {:?}",
            points[index],
            gpu.adapter.get_info()
        );
    }
}
