//! Analytic checks executed by the actual OpenCL kernels (no C++/googletest).
use hypertrace_kernel::{
    includer::{source, Parser},
    SOURCE,
};
use std::path::Path;

fn program() -> ocl::ProQue {
    let root = Path::new("hyperbolic-regression.cl");
    let probes = r#"
        #define ADDRESS_WIDTH 32
        #include <geometry/hyperbolic.hh>
        #include <shape/hy/horosphere.hh>
        #include <shape/hy/plane.hh>
        __kernel void distances(__global const float4 *a, __global const float4 *b,
                                __global float4 *out) {
            uint i = get_global_id(0);
            out[i] = (float4)(hy_distance(a[i], b[i]), 0, 0, 0);
        }
        __kernel void intersections(__global const float4 *positions,
                                    __global const float4 *directions,
                                    __global float4 *out) {
            uint i = get_global_id(0);
            Context ctx = context_init();
            if (directions[i].w != 0) {
                ctx.prev_hash = hash_finish(&ctx.hasher);
            }
            LightHy light;
            light.ray.start = positions[i];
            light.ray.direction = (float4)(directions[i].xyz, 0);
            HyDir normal = (float4)(0);
            real distance = horosphere_detect(NULL, &ctx, &normal, &light);
            out[4*i] = (float4)(distance, 0, 0, 0);
            out[4*i+1] = light.ray.start;
            out[4*i+2] = light.ray.direction;
            out[4*i+3] = normal;
        }
        __kernel void plane_intersections(__global const float4 *positions,
                                          __global const float4 *directions,
                                          __global float4 *out) {
            uint i = get_global_id(0);
            Context ctx = context_init();
            if (directions[i].w != 0) {
                ctx.prev_hash = hash_finish(&ctx.hasher);
            }
            LightHy light;
            light.ray.start = positions[i];
            light.ray.direction = (float4)(directions[i].xyz, 0);
            HyDir normal = (float4)(0);
            real distance = plane_hy_detect(NULL, &ctx, &normal, &light);
            out[4*i] = (float4)(distance, 0, 0, 0);
            out[4*i+1] = light.ray.start;
            out[4*i+2] = light.ray.direction;
            out[4*i+3] = normal;
        }
    "#;
    let memory = source::Mem::builder()
        .add_file(&root, probes.into())
        .unwrap()
        .build();
    let parser = Parser::builder()
        .add_source(&*SOURCE)
        .add_source(memory)
        .add_flag("HOST".into(), false)
        .add_flag("UNITTEST".into(), false)
        .add_flag("VARIADIC_MACROS".into(), false)
        .add_flag("DOUBLE_PRECISION".into(), false)
        .build();
    let (text, _) = parser.parse(root).unwrap().collect();
    // A missing OpenCL platform/device is a failure when explicitly invoked.
    ocl::ProQue::builder().src(text).dims(1).build().unwrap()
}

fn evaluate(
    program: &ocl::ProQue,
    name: &str,
    a: &[[f32; 4]],
    b: &[[f32; 4]],
    stride: usize,
) -> Vec<f32> {
    let a: Vec<_> = a.iter().flatten().copied().collect();
    let b: Vec<_> = b.iter().flatten().copied().collect();
    let count = a.len() / 4;
    let input = |data: &[f32]| {
        ocl::Buffer::builder()
            .queue(program.queue().clone())
            .copy_host_slice(data)
            .len(data.len())
            .build()
            .unwrap()
    };
    let a = input(&a);
    let b = input(&b);
    let output = ocl::Buffer::<f32>::builder()
        .queue(program.queue().clone())
        .len(count * stride)
        .build()
        .unwrap();
    let kernel = program
        .kernel_builder(name)
        .arg(&a)
        .arg(&b)
        .arg(&output)
        .global_work_size(count)
        .build()
        .unwrap();
    // Every buffer contains enough records for the specified global work size.
    unsafe {
        kernel.enq().unwrap();
    }
    let mut values = vec![0.0; count * stride];
    output.read(&mut values).enq().unwrap();
    program.finish().unwrap();
    values
}

fn near(actual: f32, expected: f64, tolerance: f64) {
    assert!(
        actual.is_finite() && (actual as f64 - expected).abs() <= tolerance,
        "actual={}, expected={}, tolerance={}",
        actual,
        expected,
        tolerance
    );
}

#[test]
#[ignore = "requires an OpenCL device; run with --ignored"]
fn hyperbolic_distance_and_intersection_regressions() {
    let program = program();
    let a = [
        [0., 0., 1., 0.],
        [0., 0., 2., 0.],
        [0., 0., 1e-20, 0.],
        [0., 0., 1e20, 0.],
        [2., -3., 4., 0.],
    ];
    let b = [
        [1e-4, 0., 1., 0.],
        [0., 0., 1., 0.],
        [1e-21, 0., 1e-20, 0.],
        [1e19, 0., 1e20, 0.],
        [2., -3., 4., 0.],
    ];
    let values = evaluate(&program, "distances", &a, &b, 4);
    for ((a, b), result) in a.iter().zip(&b).zip(values.chunks_exact(4)) {
        let norm = (0..3)
            .map(|i| (a[i] as f64 - b[i] as f64).powi(2))
            .sum::<f64>()
            .sqrt();
        let expected = 2.0 * (norm / (2.0 * (a[2] as f64).sqrt() * (b[2] as f64).sqrt())).asinh();
        near(result[0], expected, expected.abs() * 3e-6 + 1e-10);
    }

    // Exact vertical hit/miss, almost vertical, repeat suppression, and tangency.
    let positions = [
        [0., 0., 2., 0.],
        [0., 0., 2., 0.],
        [0., 0., 0.5, 0.],
        [0., 0., 0.5, 0.],
        [0., 0., 2., 0.],
        [0., 0., 0.5, 0.],
        [0., 0., 1., 0.],
        [0., 0., 1., 0.],
        [0., 0., 1., 0.],
        [0., 0., 2., 0.],
    ];
    let directions = [
        [0., 0., -1., 0.],
        [0., 0., 1., 0.],
        [0., 0., 1., 0.],
        [0., 0., -1., 0.],
        [1e-6, 0., -1., 0.],
        [1e-6, 0., 1., 0.],
        [0., 0., -1., 1.],
        [0., 0., 1., 1.],
        [1., 0., 0., 0.],
        [0.6, 0., -0.8, 0.],
    ];
    let values = evaluate(&program, "intersections", &positions, &directions, 16);
    for (i, result) in values.chunks_exact(16).enumerate() {
        assert!(
            result.iter().all(|x| x.is_finite()),
            "case {}: {:?}",
            i,
            result
        );
        if [1, 3, 6, 7].contains(&i) {
            assert!(
                result[0] < -0.5,
                "expected miss for case {}: {:?}",
                i,
                result
            );
            continue;
        }
        if i < 6 {
            near(result[0], 2.0f64.ln(), 3e-6);
        }
        if i == 8 {
            near(result[0], 0.0, 1e-7);
        }
        near(result[6], 1.0, 1e-7);
        let direction_length = result[8..11].iter().map(|x| x * x).sum::<f32>().sqrt();
        near(direction_length, 1.0, 3e-6);
        assert_eq!(&result[12..16], &[0., 0., -1., 0.]);
    }

    // A small p dot d is not a parallel-ray test near the ideal boundary:
    // even an exactly vertical upward ray has p dot d = p.z.
    let positions = [
        [0., 0., 1e-8, 0.],
        [0., 0., 1e-8, 0.],
        [0., 0., 2., 0.],
        [0., 0., 0.5, 0.],
        [0., 0., 0.5, 0.],
        [0., 0., 1e-8, 0.],
        [0., 0., 1e-8, 0.],
    ];
    let directions = [
        [0., 0., 1., 0.],
        [0., 0., -1., 0.],
        [1., 0., 0., 0.],
        [1., 0., 1e-8, 0.],
        [0.8, 0., 0.6, 0.],
        [1e-9, 0., 1., 0.],
        [0., 0., 1., 1.],
    ];
    let values = evaluate(&program, "plane_intersections", &positions, &directions, 16);
    for (i, result) in values.chunks_exact(16).enumerate() {
        assert!(
            result.iter().all(|x| x.is_finite()),
            "plane case {}: {:?}",
            i,
            result
        );
        if [1, 2, 3, 4, 6].contains(&i) {
            assert!(
                result[0] < -0.5,
                "expected plane miss for case {}: {:?}",
                i,
                result
            );
            continue;
        }
        let z = positions[i][2] as f64;
        let x = directions[i][0] as f64 * (1.0 - z * z) / (2.0 * z);
        let hit_z = (1.0 - x * x).sqrt();
        let expected =
            2.0 * ((x * x + (hit_z - z).powi(2)).sqrt() / (2.0 * z.sqrt() * hit_z.sqrt())).asinh();
        near(result[0], expected, 4e-5);
        if i == 0 {
            near(result[0], -z.ln(), 4e-5);
        }
        near(result[4], x, 2e-7);
        near(result[6], hit_z, 2e-7);
        near(result[12], -x, 2e-7);
        near(result[14], -hit_z, 2e-7);
        near(
            result[8..11].iter().map(|x| x * x).sum::<f32>().sqrt(),
            1.0,
            3e-6,
        );
    }
}
