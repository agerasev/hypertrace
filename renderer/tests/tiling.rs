//! Runtime input fixtures are important here: an Intel Arc/Mesa 23.2.1 run
//! diverged in full tile reduction while constant/early-return probes agreed.
use hypertrace_renderer::{
    Gpu, Renderer, Scene, read_buffer,
    shader::{
        Background, Geometry, LibraryModule, Medium, Modules, SceneDefinition, Transform, View,
    },
};
use objects::{
    object::tiling::{self, Pentagonal, Pentastar},
    shape,
};
use wgpu::util::DeviceExt;

fn definition<G: Geometry>(radius: f64) -> SceneDefinition<G> {
    SceneDefinition {
        view: View {
            map: Transform::identity(),
            fov: 1.0,
        },
        background: Background::constant([0.0; 3]),
        bounces: 1,
        radius,
        medium: Medium::vacuum(),
        objects: vec![],
        modules: Modules::default(),
    }
}

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
    let points: Vec<[f32; 4]> = FIXTURES
        .iter()
        .cycle()
        .take(FIXTURES.len() * 2)
        .map(|([x, y], _)| {
            let z = (1.0 - x * x - y * y).sqrt();
            let squared = x * x + y * y + z * z;
            [
                (squared + 1.0) / (2.0 * z),
                x / z,
                y / z,
                (squared - 1.0) / (2.0 * z),
            ]
        })
        .collect();
    let output = probe(
        &gpu,
        1.0,
        vec![
            tiling::selector::<ccgeom::Hyperboloid3, shape::Plane, Pentastar>().unwrap(),
            tiling::selector::<ccgeom::Hyperboloid3, shape::Plane, Pentagonal>().unwrap(),
        ],
        r#"
        var selected=0u;
        if index<12u {selected={{dep0}}(point,1.0,0.01,2u);}
        else {selected={{dep1}}(point,1.0,0.02,2u);}
        accumulation[index].w=f32(selected);
    "#,
        &points,
    );
    for (index, row) in output.iter().enumerate() {
        let fixture = index % FIXTURES.len();
        let tiling = index / FIXTURES.len();
        assert_eq!(
            row[3],
            FIXTURES[fixture].1[tiling] as f32,
            "tiling {tiling}, point {:?}, adapter {:?}",
            points[index],
            gpu.adapter.get_info()
        );
    }
}

/// Runtime inputs prevent constant folding from hiding driver-specific errors.
fn probe<G: Geometry>(
    gpu: &Gpu,
    radius: f64,
    selectors: Vec<LibraryModule<G>>,
    body: &str,
    points: &[[f32; 4]],
) -> Vec<[f32; 4]> {
    let source = format!(
        "fn {{{{self}}}}(index:u32) {{\nif index>={}u {{return;}}\nlet point=accumulation[index];\n{body}\n}}\n@compute @workgroup_size(64) fn tiling_probe(@builtin(global_invocation_id) gid:vec3<u32>) {{{{{{self}}}}(gid.x);}}",
        points.len(),
    );
    let mut module = LibraryModule::<G>::new("tests.tiling-probe", source, Some(0));
    module.dependencies = selectors
        .into_iter()
        .map(LibraryModule::into_source)
        .collect();
    let mut definition = definition::<G>(radius);
    definition.modules.libraries.push(module);
    let renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (1, 1),
        Scene::from_definition(&definition).unwrap(),
        1,
    )
    .unwrap();
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("tiling selector probe"),
            source: wgpu::ShaderSource::Wgsl(renderer.shader_source().into()),
        });
    let layout = gpu
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("tiling selector inputs"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
    let pipeline_layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("tiling_probe"),
            compilation_options: Default::default(),
            cache: None,
        });
    // Independent encoding of the documented 9-row renderer uniform ABI.
    let mut params = [0u32; 36];
    params[29] = (radius as f32).to_bits();
    let params = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("probe curvature radius"),
            contents: bytemuck::cast_slice(&params),
            usage: wgpu::BufferUsages::UNIFORM,
        });
    let results = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("runtime tiling points"),
            contents: bytemuck::cast_slice(points),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: params.as_entire_binding(),
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
        pass.dispatch_workgroups((points.len() as u32).div_ceil(64), 1, 1);
    }
    gpu.queue.submit([encoder.finish()]);
    let bytes = read_buffer(&gpu.device, &gpu.queue, &results, points.len() as u64 * 16).unwrap();
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
#[ignore = "requires a native WGPU compute adapter"]
fn flat_plane_and_horosphere_share_physical_square_and_hexagonal_patterns() {
    use ccgeom::{Flat3, Hyperboloid3};
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    eprintln!("tiling adapter: {:?}", gpu.adapter.get_info());
    // Known cell interiors, translated centers, and edges. The two selectors
    // intentionally have different material counts.
    let fixtures = [
        ([0.25, 0.25], 0),
        ([1.25, 0.25], 1),
        ([1.25, 1.25], 2),
        ([0.25, 1.25], 3),
        ([-0.25, -0.25], 2),
        ([1.001, 0.25], 4),
        ([0.75, 3.0f32.sqrt() / 4.0], 0),
        ([2.25, 3.0f32.sqrt() / 4.0], 2),
        ([1.5, 3.0f32.sqrt()], 1),
        ([-0.75, 3.0f32.sqrt() / 4.0], 1),
        ([0.0, 0.0], 3),
        ([1.481, 3.0f32.sqrt() / 4.0], 3),
        ([1.475, 3.0f32.sqrt() / 4.0], 0),
        ([1.519, 3.0f32.sqrt() / 4.0], 3),
        ([1.525, 3.0f32.sqrt() / 4.0], 2),
    ];
    for scale in [0.5, 1.0, 2.0] {
        let body = format!(
            "var selected=0u; if index<6u {{selected={{{{dep0}}}}(point,{scale},0.02*{scale},4u);}} else {{selected={{{{dep1}}}}(point,{scale},0.02*{scale},3u);}} accumulation[index].w=f32(selected);"
        );
        let flat: Vec<_> = fixtures
            .iter()
            .map(|([x, y], _)| [1.0, x * scale, y * scale, 0.0])
            .collect();
        let output = probe(
            &gpu,
            1.0,
            vec![
                tiling::selector::<Flat3, shape::Plane, tiling::Square>().unwrap(),
                tiling::selector::<Flat3, shape::Plane, tiling::Hexagonal>().unwrap(),
            ],
            &body,
            &flat,
        );
        for (actual, (_, expected)) in output.iter().zip(fixtures) {
            assert_eq!(
                actual[3], expected as f32,
                "flat scale={scale}, point={actual:?}"
            );
        }
        for radius in [1.0, 3.0] {
            let hyperbolic: Vec<_> = fixtures
                .iter()
                .map(|([x, y], _)| {
                    let (x, y) = (x * scale / radius, y * scale / radius);
                    let z = 0.5 * (x * x + y * y);
                    [1.0 + z, x, y, z]
                })
                .collect();
            let output = probe(
                &gpu,
                f64::from(radius),
                vec![
                    tiling::selector::<Hyperboloid3, shape::Horosphere, tiling::Square>().unwrap(),
                    tiling::selector::<Hyperboloid3, shape::Horosphere, tiling::Hexagonal>()
                        .unwrap(),
                ],
                &body,
                &hyperbolic,
            );
            for (actual, (_, expected)) in output.iter().zip(fixtures) {
                assert_eq!(
                    actual[3], expected as f32,
                    "horosphere R={radius}, scale={scale}, point={actual:?}"
                );
            }
        }
    }
    let output = probe(
        &gpu,
        1.0,
        vec![
            tiling::selector::<Flat3, shape::Plane, tiling::Square>().unwrap(),
            tiling::selector::<Flat3, shape::Plane, tiling::Hexagonal>().unwrap(),
        ],
        r#"
        accumulation[index]=vec4<f32>(
            f32({{dep0}}(point,1.0,0.02,4u)),
            f32({{dep1}}(point,1.0,0.02,3u)),0,0);
    "#,
        &[
            [1.0, 16777216.0, 0.25, 0.0],
            [1.0, -16777216.0, 0.25, 0.0],
            [1.0, 0.25, 1e30, 0.0],
        ],
    );
    assert!(
        output.iter().all(|row| row[..2] == [5.0, 4.0]),
        "unresolvable cells must stop the sample"
    );
}

fn unit(v: [f64; 3]) -> [f64; 3] {
    let length = v.into_iter().map(|x| x * x).sum::<f64>().sqrt();
    v.map(|x| x / length)
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}

/// Face normals derived from each Platonic solid's dual, independently of the
/// production selector and its arbitrary face numbering.
fn spherical_centers() -> Vec<Vec<[f64; 3]>> {
    let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
    let tetra = vec![
        [1.0, 1.0, 1.0],
        [1.0, -1.0, -1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, 1.0],
    ];
    let cube = vec![
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
    ];
    let mut oct = vec![];
    let mut dodec = vec![];
    let mut icos = vec![];
    for a in [-1.0, 1.0] {
        for b in [-1.0, 1.0] {
            for c in [-1.0, 1.0] {
                oct.push([a, b, c]);
            }
            for v in [[0.0, a, b * phi], [a, b * phi, 0.0], [b * phi, 0.0, a]] {
                dodec.push(v);
            }
            for v in [
                [0.0, a / phi, b * phi],
                [a / phi, b * phi, 0.0],
                [b * phi, 0.0, a / phi],
            ] {
                icos.push(v);
            }
        }
    }
    icos.extend(oct.iter().copied());
    let lunes = (0..7)
        .map(|i| {
            let angle = std::f64::consts::TAU * f64::from(i) / 7.0;
            [angle.cos(), angle.sin(), 0.0]
        })
        .collect();
    let dihedron = vec![[0.0, 0.0, 1.0], [0.0, 0.0, -1.0]];
    vec![tetra, cube, oct, dodec, icos, lunes, dihedron]
        .into_iter()
        .map(|centers| centers.into_iter().map(unit).collect())
        .collect()
}

fn spherical_selectors<G, S>() -> Vec<LibraryModule<G>>
where
    G: Geometry,
    S: tiling::TileSurface<G, Domain = tiling::Spherical>,
{
    use tiling::RegularSpherical as Regular;
    vec![
        tiling::selector::<G, S, Regular<3, 3>>().unwrap(),
        tiling::selector::<G, S, Regular<4, 3>>().unwrap(),
        tiling::selector::<G, S, Regular<3, 4>>().unwrap(),
        tiling::selector::<G, S, Regular<5, 3>>().unwrap(),
        tiling::selector::<G, S, Regular<3, 5>>().unwrap(),
        tiling::selector::<G, S, Regular<2, 7>>().unwrap(),
        tiling::selector::<G, S, Regular<7, 2>>().unwrap(),
    ]
}

fn spherical_patterns<G, S>(gpu: &Gpu, radius: f64, surface_radius: f64, plane: bool)
where
    G: Geometry,
    S: tiling::TileSurface<G, Domain = tiling::Spherical>,
{
    let families = spherical_centers();
    let mut points = vec![];
    let mut references = vec![];
    let mut ranges = vec![];
    let mut body = String::from("var selected=0u;\n");
    for (family, centers) in families.iter().enumerate() {
        let start = points.len();
        let mut directions = centers.clone();
        // Spiral samples reach both hemispheres without depending on a UV seam.
        for i in 0..160 {
            let z = 1.0 - 2.0 * (f64::from(i) + 0.5) / 160.0;
            let azimuth = f64::from(i) * std::f64::consts::PI * (3.0 - 5.0f64.sqrt());
            let radial = (1.0 - z * z).sqrt();
            directions.push([radial * azimuth.cos(), radial * azimuth.sin(), z]);
        }
        directions.extend([
            unit([1e-5, 0.0, 1.0]),
            unit([-1e-5, 0.0, 1.0]),
            unit([1e-5, 0.0, -1.0]),
            unit([-1e-5, 0.0, -1.0]),
        ]);
        for direction in directions {
            let (face, _) = centers
                .iter()
                .enumerate()
                .map(|(i, &n)| (i, dot(direction, n)))
                .max_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap();
            let closest = centers[face];
            let edge = centers
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != face)
                .map(|(_, &other)| {
                    let normal = unit(std::array::from_fn(|axis| closest[axis] - other[axis]));
                    dot(direction, normal).clamp(-1.0, 1.0).asin()
                })
                .fold(f64::INFINITY, f64::min);
            references.push((face, edge));
            let [x, y, z] = direction;
            let point = if plane {
                [x, y, z, 0.0]
            } else {
                let angular = surface_radius / radius;
                let (w, s) = match G::SIGN {
                    -1 => (angular.cosh(), angular.sinh()),
                    0 => (1.0, surface_radius),
                    1 => (angular.cos(), angular.sin()),
                    _ => unreachable!(),
                };
                [w, s * x, s * y, s * z]
            };
            points.push(point.map(|x| x as f32));
        }
        let end = points.len();
        body.push_str(&format!("if index>={start}u && index<{end}u {{selected={{{{dep{family}}}}}(point,1.0,0.03,{}u);}}\n",centers.len()));
        ranges.push(start..end);
    }
    body.push_str("accumulation[index].w=f32(selected);");
    let output = probe(gpu, radius, spherical_selectors::<G, S>(), &body, &points);
    for (family, (centers, range)) in families.iter().zip(ranges).enumerate() {
        let labels: Vec<_> = output[range.start..range.start + centers.len()]
            .iter()
            .map(|p| p[3] as usize)
            .collect();
        let mut sorted = labels.clone();
        sorted.sort();
        assert_eq!(
            sorted,
            (0..centers.len()).collect::<Vec<_>>(),
            "distinct face centers, family {family}, K={}",
            G::SIGN
        );
        for index in range {
            let (face, edge) = references[index];
            // The independent f64 oracle avoids exact boundary choices.
            if (edge - 0.03).abs() < 1e-5 {
                continue;
            }
            let expected = if edge < 0.03 {
                centers.len()
            } else {
                labels[face]
            };
            assert_eq!(
                output[index][3],
                expected as f32,
                "family={family}, K={}, R={radius}, r={surface_radius}, plane={plane}, point={:?}, edge={edge}",
                G::SIGN,
                points[index]
            );
        }
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn every_regular_spherical_family_matches_intrinsic_reference_in_every_ambient_geometry() {
    use ccgeom::{Flat3, Hyperboloid3, Spherical3};
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    eprintln!("spherical tiling adapter: {:?}", gpu.adapter.get_info());
    for (radius, surface_radius) in [(1.0, 0.4), (3.0, 2.0)] {
        spherical_patterns::<Flat3, shape::GeodesicSphere>(&gpu, 1.0, surface_radius, false);
        spherical_patterns::<Hyperboloid3, shape::GeodesicSphere>(
            &gpu,
            radius,
            surface_radius,
            false,
        );
        spherical_patterns::<Spherical3, shape::GeodesicSphere>(
            &gpu,
            radius,
            surface_radius,
            false,
        );
        spherical_patterns::<Spherical3, shape::Plane>(&gpu, radius, 0.0, true);
    }
}

fn mapped_pattern<G: Geometry>(gpu: &Gpu) {
    use objects::{
        Object,
        material::{Absorbing, Emissive},
        object::Tiled,
    };
    let tile = Tiled::new(
        shape::GeodesicSphere::new(0.7),
        tiling::RegularSpherical::<3, 4>::new(0.03),
        [
            Emissive::new(Absorbing, [1.0, 0.0, 0.0].into()),
            Emissive::new(Absorbing, [0.0, 1.0, 0.0].into()),
            Emissive::new(Absorbing, [0.0, 0.0, 1.0].into()),
        ],
        Absorbing,
    );
    let radius = if G::SIGN == 0 { 1.0 } else { 2.0 };
    let mut definition = definition::<G>(radius);
    tile.encode_objects(Transform::identity(), &mut definition.objects)
        .unwrap();
    let mut renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (31, 23),
        Scene::from_definition(&definition).unwrap(),
        43,
    )
    .unwrap();
    renderer.render();
    let original = renderer.snapshot().unwrap();
    assert!(
        original.iter().any(|pixel| pixel != &original[0]),
        "fixture must visit multiple cells"
    );
    let revision = renderer.pipeline_revision();
    let global = Transform::<G>::identity()
        .move_local([0.37, -0.19, 0.23], [0.3, -0.4, 0.2], radius)
        .unwrap();
    definition.view.map = global;
    definition.objects[0].map = global;
    renderer
        .update_scene(Scene::from_definition(&definition).unwrap())
        .unwrap();
    renderer.render();
    assert_eq!(
        renderer.snapshot().unwrap(),
        original,
        "global camera/object motion changed intrinsic tiling for K={}",
        G::SIGN
    );
    assert_eq!(renderer.pipeline_revision(), revision);
    // Moving only the material-bearing object must instead rotate the pattern
    // in the view, even though the underlying centered sphere is unchanged.
    definition.view.map = Transform::identity();
    definition.objects[0].map = Transform::identity()
        .move_local([0.0; 3], [0.31, 0.43, 0.17], radius)
        .unwrap();
    renderer
        .update_scene(Scene::from_definition(&definition).unwrap())
        .unwrap();
    renderer.render();
    assert_ne!(renderer.snapshot().unwrap(), original);
    assert_eq!(renderer.pipeline_revision(), revision);
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn mapped_tiled_objects_carry_material_frames_and_reuse_pipelines() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    mapped_pattern::<ccgeom::Flat3>(&gpu);
    mapped_pattern::<ccgeom::Hyperboloid3>(&gpu);
    mapped_pattern::<ccgeom::Spherical3>(&gpu);
}
