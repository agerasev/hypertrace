//! Runtime input fixtures are important here: an Intel Arc/Mesa 23.2.1 run
//! diverged in full tile reduction while constant/early-return probes agreed.
use hypertrace_wgpu::{Gpu, Scene, read_buffer, shader_source};
use wgpu::util::DeviceExt;

// Canonical hemisphere points and material IDs measured with the original
// reference renderer, using Scene::hy's pentastar/pentagonal materials.
// The first eight exercise the native regression; remaining points cover borders
// and the distinction between the star pattern and ordinary pentagons.
const FIXTURES: [([f32; 2], [u32; 2]); 12] = [
    ([-0.171875, -0.984375], [8, 10]),
    ([0.140625, -0.953125], [9, 11]),
    ([-0.421875, -0.796875], [8, 10]),
    ([0.390625, -0.796875], [9, 11]),
    ([0.328125, 0.796875], [8, 10]),
    ([-0.359375, 0.921875], [8, 11]),
    ([-0.234375, 0.953125], [9, 11]),
    ([0.078125, 0.984375], [8, 11]),
    ([-0.015625, -0.015625], [0, 0]),
    ([0.015625, -0.015625], [0, 0]),
    ([0.015625, 0.015625], [0, 0]),
    ([0.265625, -0.515625], [0, 10]),
];

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn production_tiling_matches_reference_fixtures() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let count = FIXTURES.len() as u32;
    let source = format!(
        "{}\n@compute @workgroup_size(64)\n\
        fn tiling_regression(@builtin(global_invocation_id) gid: vec3<u32>) {{\n\
            if gid.x >= {count}u || gid.y >= 2u {{ return; }}\n\
            let index = gid.x + {count}u * gid.y;\n\
            let position = accumulation[index].xyz;\n\
            accumulation[index].w = f32(tiled_material(objects[gid.y], position));\n\
        }}",
        shader_source()
    );
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
    let scene = Scene::hy();
    let objects = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("original hyperbolic tiling records"),
            contents: bytemuck::cast_slice(&scene.objects[2..4]),
            usage: wgpu::BufferUsages::STORAGE,
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
        entries: &[
            wgpu::BindGroupEntry {
                binding: 1,
                resource: objects.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: results.as_entire_binding(),
            },
        ],
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
