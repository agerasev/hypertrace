//! End-to-end event transport through the generated renderer.
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::*};
use objects::{
    material::{self, MaterialValueExt as _},
    shape::{self, ShapeValueExt as _},
};

fn definition(geometry: Geometry, object: ObjectNode) -> SceneDefinition {
    SceneDefinition {
        view: View {
            map: Transform::identity(geometry),
            fov: 1.0,
        },
        background: Background::Constant([0.25, 0.5, 0.75]),
        bounces: 4,
        radius: 1.0,
        medium: Medium::Vacuum,
        object,
        modules: vec![],
    }
}

fn render(
    gpu: &Gpu,
    definition: &SceneDefinition,
    size: (u32, u32),
    samples: u32,
) -> Vec<[f32; 4]> {
    let mut renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        size,
        Scene::from_definition(definition).unwrap(),
        0xdeadbeef,
    )
    .unwrap();
    renderer.set_samples_per_dispatch(samples).unwrap();
    renderer.render();
    renderer.snapshot().unwrap()
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn vacuum_background_and_unbounded_fog_misses_have_distinct_outcomes() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    for geometry in [
        Geometry::Euclidean,
        Geometry::Hyperbolic,
        Geometry::Spherical,
    ] {
        let mut scene = definition(geometry, ObjectNode::Vector(vec![]));
        assert_eq!(
            render(&gpu, &scene, (3, 2), 4),
            vec![[0.25, 0.5, 0.75, 1.0]; 6]
        );
        scene.medium = Medium::Homogeneous {
            extinction: 0.0,
            albedo: [0.0; 3],
        };
        assert_eq!(
            render(&gpu, &scene, (3, 2), 4),
            vec![[0.25, 0.5, 0.75, 1.0]; 6]
        );
        // Low extinction often samples many spherical circuits before the first
        // event. A surface miss must not emit the background through that fog.
        scene.medium = Medium::Homogeneous {
            extinction: 0.01,
            albedo: [1.0; 3],
        };
        assert_eq!(
            render(&gpu, &scene, (3, 2), 4),
            vec![[0.0, 0.0, 0.0, 1.0]; 6]
        );
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn transparent_spherical_surface_is_encountered_again_after_each_circuit() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let object = ObjectNode::Covered {
        shape: shape::geodesic_sphere(0.7).unwrap(),
        material: material::transparent()
            .emissive([0.125, 0.25, 0.5])
            .unwrap(),
    };
    let mut scene = definition(Geometry::Spherical, object);
    scene.radius = 2.0;
    scene.bounces = 6;
    scene.background = Background::Constant([0.0; 3]);
    assert_eq!(
        render(&gpu, &scene, (7, 5), 4),
        vec![[0.75, 1.5, 3.0, 1.0]; 35]
    );
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn absorbing_fog_matches_physical_surface_transmittance_for_every_curvature() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    for geometry in [
        Geometry::Euclidean,
        Geometry::Hyperbolic,
        Geometry::Spherical,
    ] {
        let mut scene = definition(
            geometry,
            ObjectNode::Covered {
                shape: shape::geodesic_sphere(0.8).unwrap(),
                material: material::absorbing().emissive([1.0; 3]).unwrap(),
            },
        );
        scene.bounces = 1;
        scene.radius = if geometry == Geometry::Euclidean {
            1.0
        } else {
            2.0
        };
        scene.medium = Medium::Homogeneous {
            extinction: 0.7,
            albedo: [0.0; 3],
        };
        let pixels = render(&gpu, &scene, (256, 1), 32);
        let actual = pixels.iter().map(|p| p[0] as f64).sum::<f64>() / 256.0;
        let expected = (-0.7f64 * 0.8).exp();
        // 8192 Bernoulli paths; fixed six-sigma bound against the independent
        // analytic expectation, including physical radius conversion.
        let tolerance = 6.0 * (expected * (1.0 - expected) / 8192.0).sqrt();
        assert!(
            (actual - expected).abs() < tolerance,
            "{geometry:?}: {actual} vs {expected}"
        );
        assert!(
            pixels
                .iter()
                .all(|p| p[0] == p[1] && p[1] == p[2] && p[3] == 1.0)
        );
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn embedded_custom_leaves_use_spherical_positions_and_tangent_frames() {
    let mut shape_module = ShaderModule::new(
        "test.embedded-sphere",
        ShaderKind::Shape,
        "fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit { return {{dep0}}(base,ray,previous); }",
        Some(1),
    );
    shape_module
        .dependencies
        .push(shape::geodesic_sphere_schema());
    let shape = ShapeValue::new(shape_module, vec![0.7f32.to_bits()]).unwrap();
    let material = MaterialValue::new(ShaderModule::new(
        "test.embedded-frame", ShaderKind::Material,
        r#"fn {{self}}(base:u32,ctx:GeoMaterialContext,
            sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
            (*sample).emission=vec3<f32>(ctx.position.x,dot(ctx.normal,ctx.normal),dot(ctx.normal,(*sample).direction));
            (*sample).alive=0u;
        }"#, Some(0)),vec![]).unwrap();
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let scene = definition(Geometry::Spherical, ObjectNode::Covered { shape, material });
    for pixel in render(&gpu, &scene, (7, 5), 4) {
        for (actual, expected) in pixel.into_iter().zip([0.7f32.cos(), 1.0, 1.0, 1.0]) {
            assert!((actual - expected).abs() < 2e-5, "{actual} vs {expected}");
        }
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn numerical_failure_preserves_prior_emission_without_evaluating_background() {
    let mut module = ShaderModule::new(
        "test.failure-after-hit",
        ShaderKind::Shape,
        r#"fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit {
            if previous==base { return GeoTaggedHit(geo_failure(),base); }
            return {{dep0}}(base,ray,previous);
        }"#,
        Some(1),
    );
    module.dependencies.push(shape::geodesic_sphere_schema());
    let shape = ShapeValue::new(module, vec![0.7f32.to_bits()]).unwrap();
    // Nest a vector and shape map so failure must propagate through both levels.
    let shape = shape
        .mapped(Transform::identity(Geometry::Spherical))
        .unwrap();
    let shape = shape::vector(shape.schema.clone(), vec![shape]).unwrap();
    let scene = definition(
        Geometry::Spherical,
        ObjectNode::Covered {
            shape,
            material: material::transparent()
                .emissive([0.125, 0.25, 0.5])
                .unwrap(),
        },
    );
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    assert_eq!(
        render(&gpu, &scene, (3, 2), 4),
        vec![[0.125, 0.25, 0.5, 1.0]; 6]
    );
}
