//! These regressions execute the production WGSL, rather than a CPU translation.
//! Run explicitly with `cargo test -p hypertrace-wgpu --test math -- --ignored`.
//! An unavailable adapter is a failure, never an implicit skip.

use hypertrace_wgpu::{Gpu, read_buffer};

const CASES: usize = 19;

const REGRESSION_SHADER: &str = r#"
@group(0) @binding(0) var<storage,read_write> results: array<vec4<f32>>;

@compute @workgroup_size(1)
fn math_regression() {
    let origin = vec3<f32>(0,0,1);
    results[0] = vec4<f32>(
        hy_distance(origin,vec3<f32>(0.0001,0,1)),
        hy_distance(origin,origin),
        hy_distance(origin,vec3<f32>(0,0,2)),
        hy_distance(vec3<f32>(0,0,1e-20),vec3<f32>(1e-24,0,1e-20)));

    let down = hy_horosphere(Ray(vec3<f32>(0,0,2),vec3<f32>(0,0,-1)),false);
    results[1] = vec4<f32>(f32(down.valid),down.distance,down.position.z,down.direction.z);
    let up = hy_horosphere(Ray(vec3<f32>(0,0,0.5),vec3<f32>(0,0,1)),false);
    results[2] = vec4<f32>(f32(up.valid),up.distance,up.position.z,up.direction.z);
    let away = hy_horosphere(Ray(vec3<f32>(0,0,2),vec3<f32>(0,0,1)),false);
    let again = hy_horosphere(Ray(origin,vec3<f32>(0,0,-1)),true);
    let below = hy_horosphere(Ray(vec3<f32>(0,0,0.5),vec3<f32>(0,0,-1)),false);
    results[3] = vec4<f32>(f32(away.valid),f32(again.valid),f32(below.valid),0);
    let near = hy_horosphere(Ray(vec3<f32>(0,0,2),normalize(vec3<f32>(0.0001,0,-1))),false);
    results[4] = vec4<f32>(f32(near.valid),near.distance,near.position.x,near.position.z);
    results[5] = vec4<f32>(near.direction,length(near.normal));

    let transform = hy_chain(hy_xshift(0.6),hy_zrotate(0.8));
    let a = vec3<f32>(0.2,-0.3,1.1);
    let b = vec3<f32>(0.5,0.4,0.8);
    let mapped_a = hy_apply_pos(transform,a);
    let mapped_b = hy_apply_pos(transform,b);
    results[6] = vec4<f32>(hy_distance(a,b),hy_distance(mapped_a,mapped_b),
        length(hy_apply_pos(hy_inverse(transform),mapped_a)-a),mapped_a.z);
    let direction = normalize(vec3<f32>(0.3,0.4,-0.8));
    let tangent = hy_apply_dir(transform,a,direction);
    let difference = normalize(hy_apply_pos(transform,a+0.001*direction)
        -hy_apply_pos(transform,a-0.001*direction));
    results[7] = vec4<f32>(length(tangent),length(tangent-difference),
        dot(tangent,difference),0);

    let plane = hy_plane(Ray(vec3<f32>(0,0,2),vec3<f32>(0,0,-1)),false);
    results[8] = vec4<f32>(f32(plane.valid),plane.distance,plane.position.z,plane.normal.z);
    let sphere = eu_sphere(Ray(vec3<f32>(0,0,3),vec3<f32>(0,0,-1)),false);
    results[9] = vec4<f32>(f32(sphere.valid),sphere.distance,sphere.position.z,sphere.normal.z);
    let cube = eu_cube(Ray(vec3<f32>(0,0,3),vec3<f32>(0,0,-1)),false);
    results[10] = vec4<f32>(f32(cube.valid),cube.distance,cube.position.z,cube.normal.z);
    let parallel = eu_plane(Ray(vec3<f32>(0,0,1),vec3<f32>(1,0,0)),false);
    let outside = eu_cube(Ray(vec3<f32>(2,0,3),vec3<f32>(0,0,-1)),false);
    let inside = eu_cube(Ray(vec3<f32>(0),vec3<f32>(0,1,0)),false);
    results[11] = vec4<f32>(f32(parallel.valid),f32(outside.valid),inside.distance,inside.normal.y);

    let translate = HyMap(vec4<f32>(1,0,2,3),vec4<f32>(0,0,1,0));
    results[12] = vec4<f32>(hy_apply_pos(translate,origin),0);
    let quarter = hy_zrotate(PI/2);
    results[13] = vec4<f32>(hy_apply_pos(quarter,vec3<f32>(1,0,1)),0);
    let on_surface = hy_horosphere(Ray(origin,vec3<f32>(0,0,-1)),false);
    results[14] = vec4<f32>(f32(on_surface.valid),on_surface.distance,on_surface.position.z,0);
    let near_absolute = hy_plane(Ray(vec3<f32>(0,0,1e-8),vec3<f32>(0,0,1)),false);
    results[15] = vec4<f32>(f32(near_absolute.valid),near_absolute.distance,
        near_absolute.position.z,near_absolute.direction.z);
    let near_away = hy_plane(Ray(vec3<f32>(0,0,1e-8),vec3<f32>(0,0,-1)),false);
    let tangent_plane = hy_plane(Ray(vec3<f32>(0,0,2),vec3<f32>(1,0,0)),false);
    results[16] = vec4<f32>(f32(near_away.valid),f32(tangent_plane.valid),0,0);
    let off_axis = hy_plane(Ray(vec3<f32>(0,0,1e-8),normalize(vec3<f32>(1e-9,0,1))),false);
    results[17] = vec4<f32>(f32(off_axis.valid),off_axis.position.x,
        off_axis.position.z,length(off_axis.direction));
    let ideal = hy_plane(Ray(vec3<f32>(0,0,0.5),vec3<f32>(0.8,0,0.6)),false);
    results[18] = vec4<f32>(f32(ideal.valid),0,0,0);
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
                    "{}\n{}",
                    include_str!("../src/shaders/math.wgsl"),
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

fn close(actual: f32, expected: f32, tolerance: f32) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual {actual}, expected {expected}, tolerance {tolerance}"
    );
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn production_wgsl_geometry_regressions() {
    let rows = dispatch_math();
    assert!(
        rows.iter().flatten().all(|x| x.is_finite()),
        "non-finite GPU result: {rows:?}"
    );
    close(rows[0][0], 1e-4, 2e-10);
    close(rows[0][1], 0.0, 0.0);
    close(rows[0][2], std::f32::consts::LN_2, 2e-6);
    close(rows[0][3], 1e-4, 2e-10);
    for index in [1, 2, 4, 8] {
        close(rows[index][0], 1.0, 0.0);
        close(rows[index][1], std::f32::consts::LN_2, 2e-6);
    }
    assert_eq!(rows[1][2..], [1.0, -1.0]);
    assert_eq!(rows[2][2..], [1.0, 1.0]);
    assert_eq!(rows[3], [0.0; 4]);
    close(rows[4][2], 7.5e-5, 2e-9);
    close(rows[4][3], 1.0, 0.0);
    close(rows[5][0], 5e-5, 2e-9);
    close(rows[5][2], -1.0, 2e-7);
    close(rows[5][3], 1.0, 0.0);

    // Independent f64 distance for the untransformed pair.
    let a = [0.2_f64, -0.3, 1.1];
    let b = [0.5_f64, 0.4, 0.8];
    let separation = (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f64>().sqrt();
    let reference = (2.0 * (separation / (2.0 * (a[2] * b[2]).sqrt())).asinh()) as f32;
    close(rows[6][0], reference, 2e-6);
    close(rows[6][1], reference, 3e-6);
    assert!(
        rows[6][2] < 2e-6 && rows[6][3] > 0.0,
        "transform round trip {:?}",
        rows[6]
    );
    close(rows[7][0], 1.0, 2e-6);
    assert!(
        rows[7][1] < 2e-4,
        "direction derivative error {:?}",
        rows[7]
    );
    close(rows[7][2], 1.0, 2e-6);
    assert_eq!(rows[8][2..], [1.0, -1.0]);
    assert_eq!(rows[9], [1.0, 2.0, 1.0, 1.0]);
    assert_eq!(rows[10], [1.0, 2.0, 1.0, 1.0]);
    assert_eq!(rows[11], [0.0, 0.0, 1.0, 1.0]);
    assert_eq!(rows[12], [2.0, 3.0, 1.0, 0.0]);
    close(rows[13][0], 0.0, 2e-6);
    close(rows[13][1], 1.0, 2e-6);
    close(rows[13][2], 1.0, 2e-6);
    assert_eq!(rows[14], [1.0, 0.0, 1.0, 0.0]);
    close(rows[15][0], 1.0, 0.0);
    close(rows[15][1], 1e8_f32.ln(), 4e-6);
    assert_eq!(rows[15][2..], [1.0, 1.0]);
    assert_eq!(rows[16], [0.0; 4]);
    close(rows[17][0], 1.0, 0.0);
    close(rows[17][1], 0.05, 2e-7);
    close(rows[17][2], 0.9975_f32.sqrt(), 2e-7);
    close(rows[17][3], 1.0, 2e-7);
    assert_eq!(rows[18], [0.0; 4]);
}
