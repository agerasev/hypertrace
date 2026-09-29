//! Independent analytic checks of the shared production kernel. Input values
//! arrive through storage buffers so the driver evaluates the actual runtime
//! path. No ccgeom implementation is used to construct the references.
use hypertrace_renderer::{Gpu, read_buffer};
use wgpu::util::DeviceExt;

fn dispatch(k: i32, input: &[[f32; 4]], body: &str, count: usize) -> Vec<[f32; 4]> {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("GPU adapter required");
    // Probe the production intersection helpers directly; component wrappers
    // additionally depend on renderer bindings and are exercised by render tests.
    let primitives = [
        (
            "plane",
            include_str!("../../objects/src/shape/shaders/plane.wgsl"),
        ),
        (
            "sphere",
            include_str!("../../objects/src/shape/shaders/sphere.wgsl"),
        ),
        (
            "cube",
            include_str!("../../objects/src/shape/shaders/cube.wgsl"),
        ),
        (
            "horosphere",
            include_str!("../../objects/src/shape/shaders/horosphere.wgsl"),
        ),
    ]
    .map(|(name, source)| {
        source
            .split("\nfn {{self}}(")
            .next()
            .unwrap()
            .replace("{{self}}_intersect", &format!("geo_{name}"))
    })
    .join("\n");
    let source = format!(
        "{}\nconst GEO_K:f32={k}.0;\n{}\n{}\n{primitives}\n\
        @group(0) @binding(0) var<storage,read> input:array<vec4<f32>>;\n\
        @group(0) @binding(1) var<storage,read_write> output:array<vec4<f32>>;\n\
        @compute @workgroup_size(1) fn verify() {{ {body} }}",
        include_str!("../src/shaders/math.wgsl"),
        include_str!("../src/shaders/embedded.wgsl"),
        include_str!("../src/shaders/transport.wgsl"),
    );
    let module = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("embedded geometry references"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("embedded geometry references"),
            layout: None,
            module: &module,
            entry_point: Some("verify"),
            compilation_options: Default::default(),
            cache: None,
        });
    let input_buffer = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("runtime inputs"),
            contents: bytemuck::cast_slice(input),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let output_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("reference results"),
        size: (count * 16) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("embedded reference data"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
            },
        ],
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    gpu.queue.submit([encoder.finish()]);
    read_buffer(&gpu.device, &gpu.queue, &output_buffer, (count * 16) as u64)
        .expect("GPU readback")
        .as_chunks::<16>()
        .0
        .iter()
        .map(|row| {
            std::array::from_fn(|i| f32::from_le_bytes(row[4 * i..4 * i + 4].try_into().unwrap()))
        })
        .collect()
}

fn cs(k: i32, t: f64) -> (f64, f64) {
    match k {
        -1 => (t.cosh(), t.sinh()),
        0 => (1.0, t),
        1 => (t.cos(), t.sin()),
        _ => unreachable!(),
    }
}

fn row(actual: [f32; 4], expected: [f64; 4], tolerance: f64) {
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!(
            actual.is_finite() && (f64::from(actual) - expected).abs() <= tolerance,
            "actual {actual}, expected {expected}, tolerance {tolerance}"
        );
    }
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn embedded_action_matches_independent_matrices_and_frames() {
    for k in [-1, 0, 1] {
        let (c, s) = cs(k, 0.4);
        let input = [
            [1.2, 0.3, -0.4, 0.5],
            [c as f32, 0.0, 0.0, s as f32],
            [0.0, 0.6, 0.8, 0.0],
            [-(k as f64 * s) as f32, 0.0, 0.0, c as f32],
            [0.7, 0.6, 0.0, 0.0],
        ];
        let rows = dispatch(
            k,
            &input,
            r#"
            let boost = geo_translation(vec3<f32>(1,0,0),input[4].x);
            let rotation = geo_rotation(vec3<f32>(0,0,1),input[4].y);
            let map = geo_chain(boost,rotation);
            let mapped = geo_map_apply(map,input[0]);
            output[0] = mapped;
            output[1] = geo_map_apply(geo_inverse(map),mapped);
            output[2] = geo_map_apply(boost,geo_map_apply(rotation,input[0]));
            let tangent = geo_from_local(input[1],input[2].yzw);
            output[3] = tangent;
            output[4] = vec4<f32>(0,geo_to_local(input[1],tangent));
            let advanced = geo_advance(GeoRay(input[1],input[3]),1.5,2.5);
            output[5] = advanced.position;
            output[6] = advanced.tangent;
            output[7] = geo_map_apply(geo_chain(geo_inverse(map),map),input[0]);
        "#,
            8,
        );

        // Independent spatial rotation followed by the (w,x) boost/rotation
        // block, or homogeneous Euclidean translation when k=0.
        let x = 0.3 * 0.6_f64.cos() + 0.4 * 0.6_f64.sin();
        let y = 0.3 * 0.6_f64.sin() - 0.4 * 0.6_f64.cos();
        let (bc, bs) = cs(k, 0.7);
        let expected = [bc * 1.2 - f64::from(k) * bs * x, bs * 1.2 + bc * x, y, 0.5];
        row(rows[0], expected, 2e-6);
        row(rows[1], [1.2, 0.3, -0.4, 0.5], 2e-6);
        row(rows[2], expected, 2e-6);
        row(rows[7], [1.2, 0.3, -0.4, 0.5], 2e-6);
        let frame = if k == 1 {
            [0.0, 0.6 * c - 0.8 * s, 0.8 * c + 0.6 * s, 0.0]
        } else {
            [0.0, 0.6, 0.8, 0.0]
        };
        row(rows[3], frame, 2e-6);
        row(rows[4], [0.0, 0.6, 0.8, 0.0], 2e-6);
        let travel = if k == 0 { 1.5 } else { 0.6 };
        let (ac, as_) = cs(k, 0.4 + travel);
        row(rows[5], [ac, 0.0, 0.0, as_], 2e-6);
        row(rows[6], [-f64::from(k) * as_, 0.0, 0.0, ac], 2e-6);
    }
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn embedded_primitives_respect_physical_intervals_and_normals() {
    for k in [-1, 0, 1] {
        let (c, s) = cs(k, 1.2);
        let radius = if k == 0 { 1.0 } else { 2.5 };
        let input = [
            [c as f32, 0.0, 0.0, s as f32],
            [(f64::from(k) * s) as f32, 0.0, 0.0, -c as f32],
            [radius as f32, (0.7 * radius) as f32, 0.0, 0.0],
        ];
        let rows = dispatch(
            k,
            &input,
            r#"
            let ray = GeoRay(input[0],input[1]);
            let radius = input[2].x;
            let sphere = geo_sphere(ray,0,100,radius,input[2].y);
            output[0] = vec4<f32>(f32(sphere.valid),sphere.distance,0,0);
            output[1] = sphere.position;
            output[2] = sphere.tangent;
            output[3] = sphere.normal;
            let plane = geo_plane(ray,0,100,radius);
            output[4] = vec4<f32>(f32(plane.valid),plane.distance,0,0);
            output[5] = plane.position;
            output[6] = plane.normal;
            let far_sphere = geo_sphere(ray,0.6*radius,100,radius,input[2].y);
            output[7] = vec4<f32>(f32(far_sphere.valid),far_sphere.distance,0,0);
            let missed = geo_sphere(ray,0,0.4*radius,radius,input[2].y);
            output[8] = vec4<f32>(f32(missed.valid),0,0,0);
            let origin = vec4<f32>(1,0,0,0);
            let along = GeoRay(origin,vec4<f32>(0,0,0,1));
            let start = geo_plane(along,0,100,radius);
            let coplanar = geo_plane(GeoRay(origin,vec4<f32>(0,1,0,0)),0,100,radius);
            output[9] = vec4<f32>(f32(start.valid),start.distance,f32(coplanar.valid),0);
            let special = geo_horosphere(ray,0,100,radius);
            let cube = geo_cube(ray,0,100,radius);
            output[10] = vec4<f32>(f32(special.valid),special.distance,f32(cube.valid),cube.distance);
        "#,
            11,
        );
        row(rows[0], [1.0, 0.5 * radius, 0.0, 0.0], 2e-6);
        let (hc, hs) = cs(k, 0.7);
        row(rows[1], [hc, 0.0, 0.0, hs], 2e-6);
        row(rows[2], [f64::from(k) * hs, 0.0, 0.0, -hc], 2e-6);
        row(rows[3], [-f64::from(k) * hs, 0.0, 0.0, hc], 2e-6);
        row(rows[4], [1.0, 1.2 * radius, 0.0, 0.0], 2e-6);
        row(rows[5], [1.0, 0.0, 0.0, 0.0], 2e-6);
        row(rows[6], [0.0, 0.0, 0.0, -1.0], 0.0);
        row(rows[7], [1.0, 1.9 * radius, 0.0, 0.0], 3e-6);
        row(rows[8], [0.0; 4], 0.0);
        row(rows[9], [1.0, 0.0, 0.0, 0.0], 0.0);
        let special = match k {
            -1 => [1.0, 1.2 * radius, 0.0, 0.0],
            0 => [0.0, 0.0, 1.0, 0.2],
            _ => [0.0; 4],
        };
        row(rows[10], special, 2e-6);
    }
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn horosphere_normal_matches_independent_half_space_derivative() {
    let (x, y) = (0.4_f64, -0.3_f64);
    let distance = 0.7_f64;
    let radius = 2.5_f64;
    let z = distance.exp();
    let transverse = x * x + y * y;
    // The vertical half-space geodesic z(s)=exp(distance-s) has normalized
    // speed one and reaches z=1 after `distance`. Differentiate its analytic
    // embedding with respect to s; no production map/frame code builds inputs.
    let position = [
        (transverse + z * z + 1.0) / (2.0 * z),
        x / z,
        y / z,
        (transverse + z * z - 1.0) / (2.0 * z),
    ];
    let tangent = [
        (transverse + 1.0 - z * z) / (2.0 * z),
        x / z,
        y / z,
        (transverse - 1.0 - z * z) / (2.0 * z),
    ];
    let input = [
        position.map(|value| value as f32),
        tangent.map(|value| value as f32),
        [radius as f32, 0.0, 0.0, 0.0],
    ];
    let rows = dispatch(
        -1,
        &input,
        r#"
        let hit=geo_horosphere(GeoRay(input[0],input[1]),0,10,input[2].x);
        output[0]=vec4<f32>(f32(hit.valid),hit.distance,0,0);
        output[1]=hit.position;
        output[2]=hit.tangent;
        output[3]=hit.normal;
        output[4]=vec4<f32>(geo_metric(hit.position,hit.normal),
            geo_metric(hit.normal,hit.normal),geo_metric(hit.tangent,hit.normal),0);
        "#,
        5,
    );
    let normal = [transverse / 2.0, x, y, transverse / 2.0 - 1.0];
    row(rows[0], [1.0, distance * radius, 0.0, 0.0], 2e-6);
    row(
        rows[1],
        [1.0 + transverse / 2.0, x, y, transverse / 2.0],
        2e-6,
    );
    row(rows[2], normal, 2e-6);
    row(rows[3], normal, 2e-6);
    row(rows[4], [0.0, -1.0, -1.0, 0.0], 2e-6);
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn spherical_hits_and_advancement_keep_multiple_circuits() {
    let radius = 2.5_f64;
    let phase = 0.4_f64;
    let distance = (8.0 * std::f64::consts::PI + phase) * radius;
    let min = (6.0 * std::f64::consts::PI + 0.2) * radius;
    let input = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
        [
            distance as f32,
            min as f32,
            radius as f32,
            0.7 * radius as f32,
        ],
    ];
    let rows = dispatch(
        1,
        &input,
        r#"
        let ray = GeoRay(input[0],input[1]);
        let radius = input[2].z;
        let advanced = geo_advance(ray,input[2].x,radius);
        output[0] = advanced.position;
        output[1] = advanced.tangent;
        let plane = geo_plane(ray,input[2].y,100,radius);
        output[2] = vec4<f32>(f32(plane.valid),plane.distance,0,0);
        output[3] = plane.position;
        let sphere = geo_sphere(ray,input[2].y,100,radius,input[2].w);
        output[4] = vec4<f32>(f32(sphere.valid),sphere.distance,0,0);
        output[5] = sphere.position;
        let far = geo_sphere(ray,0,100,radius,2.1*radius);
        output[6] = vec4<f32>(f32(far.valid),far.distance,0,0);
        output[7] = far.normal;
        let repeat = geo_plane(ray,0.0001,100,radius);
        output[8] = vec4<f32>(f32(repeat.valid),repeat.distance,0,0);
        let antipodal = geo_advance(ray,PI*radius,radius);
        let tangent = geo_from_local(antipodal.position,vec3<f32>(0.6,0.8,0));
        output[9] = vec4<f32>(0,geo_to_local(antipodal.position,tangent));
        let same = geo_sphere(ray,sphere.distance,100,radius,input[2].w);
        let excluded = geo_sphere(ray,input[2].y,sphere.distance,radius,input[2].w);
        output[10] = vec4<f32>(f32(same.valid),same.distance-sphere.distance,
            f32(excluded.valid),0);
    "#,
        11,
    );
    row(rows[0], [phase.cos(), 0.0, 0.0, phase.sin()], 5e-6);
    row(rows[1], [-phase.sin(), 0.0, 0.0, phase.cos()], 5e-6);
    row(
        rows[2],
        [1.0, 7.0 * std::f64::consts::PI * radius, 0.0, 0.0],
        1e-5,
    );
    row(rows[3], [-1.0, 0.0, 0.0, 0.0], 5e-6);
    row(
        rows[4],
        [1.0, (6.0 * std::f64::consts::PI + 0.7) * radius, 0.0, 0.0],
        1e-5,
    );
    row(rows[5], [0.7_f64.cos(), 0.0, 0.0, 0.7_f64.sin()], 5e-6);
    row(rows[6], [1.0, 2.1 * radius, 0.0, 0.0], 2e-6);
    row(rows[7], [-2.1_f64.sin(), 0.0, 0.0, 2.1_f64.cos()], 2e-6);
    row(
        rows[8],
        [1.0, std::f64::consts::PI * radius, 0.0, 0.0],
        2e-6,
    );
    row(rows[9], [0.0, 0.6, 0.8, 0.0], 2e-6);
    row(rows[10], [1.0, 0.0, 0.0, 0.0], 0.0);
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn euclidean_tangent_contacts_and_half_open_endpoints() {
    let input = [[1.0, 1.0, 0.0, 2.0], [0.0, 0.0, 0.0, -1.0]];
    let rows = dispatch(
        0,
        &input,
        r#"
        let ray = GeoRay(input[0],input[1]);
        let tangent = geo_sphere(ray,0,3,1,1);
        let excluded = geo_sphere(ray,0,2,1,1);
        let included = geo_sphere(ray,2,3,1,1);
        let behind = geo_sphere(ray,2.001,3,1,1);
        output[0] = vec4<f32>(f32(tangent.valid),tangent.distance,
            f32(excluded.valid),f32(included.valid));
        output[1] = vec4<f32>(f32(behind.valid),included.distance,0,0);
        output[2] = tangent.normal;
    "#,
        3,
    );
    row(rows[0], [1.0, 2.0, 0.0, 1.0], 0.0);
    row(rows[1], [0.0, 2.0, 0.0, 0.0], 0.0);
    row(rows[2], [0.0, 1.0, 0.0, 0.0], 0.0);
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn curved_sphere_tangencies_have_an_isolated_forward_root() {
    for k in [-1, 1] {
        // Build a geodesic tangent to the sphere at (C(r),S(r),0,0), then
        // evaluate its state one known segment before that tangent point.
        let r = if k == -1 {
            2.0_f64.ln()
        } else {
            0.8_f64.acos()
        };
        let (c, s) = cs(k, r);
        let input = [
            [(c * c) as f32, (c * s) as f32, -s as f32, 0.0],
            [
                (f64::from(k) * s * c) as f32,
                (f64::from(k) * s * s) as f32,
                c as f32,
                0.0,
            ],
            [r as f32, 0.0, 0.0, 0.0],
        ];
        let rows = dispatch(
            k,
            &input,
            r#"
            let ray = GeoRay(input[0],input[1]);
            let hit = geo_sphere(ray,0,5,1,input[2].x);
            output[0] = vec4<f32>(f32(hit.valid),hit.distance,0,0);
            output[1] = hit.position;
            output[2] = hit.normal;
        "#,
            3,
        );
        row(rows[0], [1.0, r, 0.0, 0.0], 2e-6);
        row(rows[1], [c, s, 0.0, 0.0], 2e-6);
        row(rows[2], [-f64::from(k) * s, c, 0.0, 0.0], 2e-6);
    }
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn transport_retains_winding_and_competes_in_physical_distance() {
    let radius = 2.5_f64;
    let short = 0.4 * radius;
    let long = (6.0 * std::f64::consts::PI + 0.4) * radius;
    let extinction = 2.0_f64.ln() / long;
    let input = [[radius as f32, short as f32, long as f32, extinction as f32]];
    let rows = dispatch(
        1,
        &input,
        r#"
        let radius = input[0].x;
        let ray = GeoRay(vec4<f32>(1,0,0,0),vec4<f32>(0,0,0,1));
        let start = GeoPath(ray,0,0);
        let short = geo_travel(start,input[0].y,radius);
        let long = geo_travel(start,input[0].z,radius);
        output[0] = long.ray.position-short.ray.position;
        output[1] = long.ray.tangent-short.ray.tangent;
        let sample = geo_free_flight(input[0].w,0.5);
        output[2] = vec4<f32>(geo_path_distance(short),geo_path_distance(long),sample,
            geo_transmittance(input[0].w,geo_path_distance(long)));
        let sphere = geo_sphere(ray,0,geo_infinity(),radius,0.7*radius);
        output[3] = vec4<f32>(
            select(0.0,1.0,geo_medium_precedes(sample,geo_miss())),
            select(0.0,1.0,geo_medium_precedes(sample,sphere)),
            select(0.0,1.0,geo_medium_precedes(sphere.distance,sphere)),
            select(0.0,1.0,geo_medium_precedes(0.5*sphere.distance,sphere)));
        let vacuum = geo_free_flight(0,0.5);
        output[4] = vec4<f32>(select(0.0,1.0,vacuum==geo_infinity()),
            select(0.0,1.0,geo_medium_precedes(vacuum,geo_miss())),
            geo_transmittance(input[0].w,geo_path_distance(short)),
            geo_transmittance(0,geo_infinity()));
        var accumulated = GeoPath(ray,16777216.0,0);
        for (var i=0u; i<16u; i+=1u) {
            accumulated = geo_travel(accumulated,0.25,radius);
        }
        output[5] = vec4<f32>(geo_path_distance(accumulated)-16777216.0,
            accumulated.remainder,accumulated.travelled-16777216.0,0);
        output[6] = accumulated.ray.position;
        // Stratified inverse-CDF samples give a deterministic survival check
        // without adding Monte Carlo noise to the numerical regression.
        var survived = 0u;
        for (var i=0u; i<4096u; i+=1u) {
            let distance = geo_free_flight(0.6,(f32(i)+0.5)/4096.0);
            if distance >= 0.7 { survived += 1u; }
        }
        output[7] = vec4<f32>(f32(survived)/4096.0,
            geo_transmittance(0.6,0.7),0,0);
        let carry = geo_travel(GeoPath(ray,16777216.0,1023.75),0.5,radius);
        let last = geo_travel(GeoPath(ray,17179868160.0,1023.75),0.25,radius);
        output[8] = vec4<f32>(carry.travelled-16777216.0,carry.remainder,
            last.travelled-17179868160.0,last.remainder);
    "#,
        9,
    );
    row(rows[0], [0.0; 4], 5e-6);
    row(rows[1], [0.0; 4], 5e-6);
    row(rows[2], [short, long, long, 0.5], 1e-5);
    row(rows[3], [1.0, 0.0, 0.0, 1.0], 0.0);
    row(rows[4], [1.0, 0.0, (-extinction * short).exp(), 1.0], 2e-6);
    assert!(
        rows[4][2] > rows[2][3],
        "extra circuits must reduce survival"
    );
    assert_eq!(rows[5], [4.0, 4.0, 0.0, 0.0]);
    row(
        rows[6],
        [(4.0 / radius).cos(), 0.0, 0.0, (4.0 / radius).sin()],
        2e-6,
    );
    row(
        rows[7],
        [(-0.42_f64).exp(), (-0.42_f64).exp(), 0.0, 0.0],
        1.0 / 4096.0,
    );
    row(rows[8], [1024.0, 0.25, 1024.0, 0.0], 0.0);
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn numerical_guards_distinguish_failure_from_miss_and_keep_spherical_circuits() {
    for k in [-1, 0, 1] {
        let result = dispatch(
            k,
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [f32::INFINITY, f32::NAN, 0.0, 0.0],
                [12.0 * std::f32::consts::TAU + 0.4, 1000.0, -0.2, 0.0],
            ],
            r#"
            let ray = GeoRay(input[0],input[1]);
            output[0] = vec4<f32>(
                select(0.0,1.0,geo_ray_supported(ray)),
                select(0.0,1.0,geo_ray_supported(GeoRay(ray.position,2*ray.tangent))),
                select(0.0,1.0,geo_ray_supported(GeoRay(vec4<f32>(input[2].x,0,0,0),ray.tangent))),
                select(0.0,1.0,geo_ray_supported(GeoRay(vec4<f32>(input[2].y,0,0,0),ray.tangent))));
            output[1] = vec4<f32>(
                select(0.0,1.0,geo_advance_supported(ray,0.4,1)),
                select(0.0,1.0,geo_advance_supported(ray,input[2].x,1)),
                select(0.0,1.0,geo_advance_supported(ray,input[2].y,1)),
                select(0.0,1.0,geo_advance_supported(ray,input[3].z,1)));
            output[2] = vec4<f32>(
                select(0.0,1.0,geo_advance_supported(ray,input[3].y,1)),
                select(0.0,1.0,geo_advance_supported(ray,input[3].x,1)),
                f32(geo_failure().valid),f32(geo_miss().valid));
            output[3] = geo_advance(ray,input[3].x,1).position;
            "#,
            4,
        );
        assert_eq!(result[0], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(result[1], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            result[2],
            if k < 0 {
                [0.0, 0.0, 2.0, 0.0]
            } else {
                [1.0, 1.0, 2.0, 0.0]
            }
        );
        if k == 1 {
            row(result[3], [0.4_f64.cos(), 0.4_f64.sin(), 0.0, 0.0], 1e-5);
        }
    }
}

#[test]
#[ignore = "requires a working WGPU compute adapter"]
fn curved_section_tangency_band_preserves_resolved_near_hits_and_misses() {
    for k in [-1, 1] {
        let (a, b, c): (f32, f32, f32) = if k < 0 {
            (1.5625, -0.9375, 1.25)
        } else {
            (0.64, 0.48, 0.8)
        };
        let lower = f32::from_bits(c.to_bits() - 1);
        let upper = f32::from_bits(c.to_bits() + 1);
        let miss = c * (1.0 + 0.001 * k as f32);
        let crossing = c * (1.0 - 0.001 * k as f32);
        let result = dispatch(
            k,
            &[[a, b, c, lower], [upper, miss, crossing, 0.0]],
            r#"
            output[0]=vec4<f32>(
                geo_section_root(input[0].x,input[0].y,input[0].z,0,5,1),
                geo_section_root(input[0].x,input[0].y,input[0].w,0,5,1),
                geo_section_root(input[0].x,input[0].y,input[1].x,0,5,1),
                geo_section_root(input[0].x,input[0].y,input[1].y,0,5,1));
            output[1]=vec4<f32>(geo_section_root(input[0].x,input[0].y,input[1].z,0,5,1),0,0,0);
        "#,
            2,
        );
        let (a, b, crossing) = (f64::from(a), f64::from(b), f64::from(crossing));
        // Independent f64 roots for the quantized coefficients. Perturbations
        // of +/-one ULP are intentionally within the defined backward-error
        // band; the separated near crossing/miss must retain their topology.
        let tangent = if k < 0 {
            0.5 * ((a - b) / (a + b)).ln()
        } else {
            b.atan2(a)
        };
        row(result[0], [tangent, tangent, tangent, -1.0], 2e-6);
        let first = if k < 0 {
            let d = crossing * crossing - (a + b) * (a - b);
            ((a - b) / (crossing + d.sqrt())).ln()
        } else {
            b.atan2(a) - (crossing / a.hypot(b)).acos()
        };
        row(result[1], [first, 0.0, 0.0, 0.0], 2e-6);
        assert!((f64::from(result[1][0]) - tangent).abs() > 0.01);
    }
}
