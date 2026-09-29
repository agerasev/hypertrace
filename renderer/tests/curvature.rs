//! End-to-end event transport through the generated renderer.
use ccgeom::{Flat3, Hyperboloid3, Spherical3};
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::*};
use objects::{
    material::{self, MaterialValueExt as _},
    shape::{self, ShapeValueExt as _},
};

fn definition<G: Geometry>(objects: Vec<EncodedObject<G>>) -> SceneDefinition<G> {
    SceneDefinition {
        view: View {
            map: Transform::identity(),
            fov: 1.0,
        },
        background: Background::constant([0.25, 0.5, 0.75]),
        bounces: 4,
        radius: 1.0,
        medium: Medium::vacuum(),
        objects,
        modules: Modules::default(),
    }
}

fn render<G: Geometry>(
    gpu: &Gpu,
    definition: &SceneDefinition<G>,
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
    vacuum_case::<Flat3>(&gpu);
    vacuum_case::<Hyperboloid3>(&gpu);
    vacuum_case::<Spherical3>(&gpu);
}

fn vacuum_case<G: Geometry>(gpu: &Gpu) {
    let mut scene = definition::<G>(vec![]);
    assert_eq!(
        render(gpu, &scene, (3, 2), 4),
        vec![[0.25, 0.5, 0.75, 1.0]; 6]
    );
    scene.medium = Medium {
        extinction: 0.0,
        albedo: [0.0; 3],
    };
    assert_eq!(
        render(gpu, &scene, (3, 2), 4),
        vec![[0.25, 0.5, 0.75, 1.0]; 6]
    );
    // Low extinction often samples many spherical circuits before the first
    // event. A surface miss must not emit the background through that fog.
    scene.medium = Medium {
        extinction: 0.01,
        albedo: [1.0; 3],
    };
    assert_eq!(
        render(gpu, &scene, (3, 2), 4),
        vec![[0.0, 0.0, 0.0, 1.0]; 6]
    );
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn transparent_spherical_surface_is_encountered_again_after_each_circuit() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let object = EncodedObject::<Spherical3> {
        map: Transform::identity(),
        shape: shape::geodesic_sphere(0.7).unwrap(),
        material: material::transparent()
            .emissive([0.125, 0.25, 0.5])
            .unwrap(),
    };
    let mut scene = definition(vec![object]);
    scene.radius = 2.0;
    scene.bounces = 6;
    scene.background = Background::constant([0.0; 3]);
    assert_eq!(
        render(&gpu, &scene, (7, 5), 4),
        vec![[0.75, 1.5, 3.0, 1.0]; 35]
    );
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn absorbing_fog_matches_physical_surface_transmittance_for_every_curvature() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    absorbing_case::<Flat3>(&gpu);
    absorbing_case::<Hyperboloid3>(&gpu);
    absorbing_case::<Spherical3>(&gpu);
}

fn absorbing_case<G: Geometry>(gpu: &Gpu) {
    let mut scene = definition::<G>(vec![EncodedObject {
        map: Transform::identity(),
        shape: shape::geodesic_sphere(0.8).unwrap(),
        material: material::absorbing().emissive([1.0; 3]).unwrap(),
    }]);
    scene.bounces = 1;
    scene.radius = if G::SIGN == 0 { 1.0 } else { 2.0 };
    scene.medium = Medium {
        extinction: 0.7,
        albedo: [0.0; 3],
    };
    let pixels = render(gpu, &scene, (256, 1), 32);
    let actual = pixels.iter().map(|p| p[0] as f64).sum::<f64>() / 256.0;
    let expected = (-0.7f64 * 0.8).exp();
    // 8192 Bernoulli paths; fixed six-sigma bound against the independent
    // analytic expectation, including physical radius conversion.
    let tolerance = 6.0 * (expected * (1.0 - expected) / 8192.0).sqrt();
    assert!(
        (actual - expected).abs() < tolerance,
        "K={}: {actual} vs {expected}",
        G::SIGN
    );
    assert!(
        pixels
            .iter()
            .all(|p| p[0] == p[1] && p[1] == p[2] && p[3] == 1.0)
    );
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn embedded_custom_leaves_use_spherical_positions_and_tangent_frames() {
    let mut shape_module = ShapeModule::<Spherical3>::new(
        "test.embedded-sphere",
        "fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit { return {{dep0}}(base,ray,previous); }",
        Some(1),
    );
    shape_module
        .dependencies
        .push(shape::geodesic_sphere_schema::<Spherical3>().into_source());
    let shape = ShapeValue::new(shape_module, vec![0.7f32.to_bits()]).unwrap();
    let material = MaterialValue::new(MaterialModule::<Spherical3>::new(
        "test.embedded-frame",
        r#"fn {{self}}(base:u32,ctx:GeoMaterialContext,
            sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
            (*sample).emission=vec3<f32>(ctx.position.x,dot(ctx.normal,ctx.normal),dot(ctx.normal,(*sample).direction));
            (*sample).alive=0u;
        }"#, Some(0)),vec![]).unwrap();
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let scene = definition(vec![EncodedObject::<Spherical3> {
        map: Transform::identity(),
        shape,
        material,
    }]);
    for pixel in render(&gpu, &scene, (7, 5), 4) {
        for (actual, expected) in pixel.into_iter().zip([0.7f32.cos(), 1.0, 1.0, 1.0]) {
            assert!((actual - expected).abs() < 2e-5, "{actual} vs {expected}");
        }
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn numerical_failure_preserves_prior_emission_without_evaluating_background() {
    let mut module = ShapeModule::<Spherical3>::new(
        "test.failure-after-hit",
        r#"fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit {
            if previous==base { return GeoTaggedHit(geo_failure(),base); }
            return {{dep0}}(base,ray,previous);
        }"#,
        Some(1),
    );
    module
        .dependencies
        .push(shape::geodesic_sphere_schema::<Spherical3>().into_source());
    let shape = ShapeValue::new(module, vec![0.7f32.to_bits()]).unwrap();
    // Nest a vector and shape map so failure must propagate through both levels.
    let shape = shape.mapped(Transform::<Spherical3>::identity()).unwrap();
    let shape = shape::vector(shape.schema.clone(), vec![shape]).unwrap();
    let scene = definition::<Spherical3>(vec![EncodedObject {
        map: Transform::identity(),
        shape,
        material: material::transparent()
            .emissive([0.125, 0.25, 0.5])
            .unwrap(),
    }]);
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    assert_eq!(
        render(&gpu, &scene, (3, 2), 4),
        vec![[0.125, 0.25, 0.5, 1.0]; 6]
    );
}
