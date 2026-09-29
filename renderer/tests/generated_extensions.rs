//! Execute extension leaves through the same compiler/runtime as scene builders.
use ccgeom::{Euclidean3, Geometry3};
use hypertrace_renderer::{Gpu, Renderer, Scene, pixel_seed, shader::*};
use objects::{
    material,
    shape::{self, ShapeValueExt as _},
};

fn definition(shape: ShapeValue, material: MaterialValue) -> SceneDefinition {
    SceneDefinition {
        view: View {
            map: Transform::Euclidean(Euclidean3::shift_z(3.0)),
            fov: 0.1,
        },
        background: Background::Constant([0.0; 3]),
        bounces: 1,
        radius: 1.0,
        medium: Default::default(),
        object: ObjectNode::Covered { shape, material },
        modules: vec![],
    }
}

fn custom_material(key: &str, source: &str) -> MaterialValue {
    MaterialValue::new(
        ShaderModule::new(key, ShaderKind::Material, source, Some(0)),
        vec![],
    )
    .unwrap()
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn nested_single_component_mixtures_each_advance_rng_once() {
    let probe = custom_material(
        "tests.rng-state",
        r#"
fn {{self}}(base:u32, ctx:GeoMaterialContext,
                  sample:ptr<function,MaterialSample>, rng:ptr<function,u32>) {
    (*sample).emission=vec3<f32>(f32(*rng)*(1.0/4294967296.0));
    (*sample).alive=0u;
}
"#,
    );
    // Even deterministic, single-component mixtures draw independently. Removing
    // either draw changes the RNG seen by this leaf and all later path events.
    let inner = material::mixture(vec![(1.0, probe)]).unwrap();
    let outer = material::mixture(vec![(1.0, inner)]).unwrap();
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let seed = 37;
    let renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (5, 3),
        Scene::from_definition(&definition(shape::plane(), outer)).unwrap(),
        seed,
    )
    .unwrap();
    renderer.render();
    let expected: Vec<_> = (0..15)
        .map(|index| {
            let mut state = pixel_seed(seed, index);
            // Two camera jitter draws, then one per nested mixture.
            for _ in 0..4 {
                state = state.wrapping_mul(1103515245).wrapping_add(12345);
            }
            let value = state as f32 * (1.0 / 4294967296.0);
            [value, value, value, 1.0]
        })
        .collect();
    assert_eq!(renderer.snapshot().unwrap(), expected);
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn custom_shape_keeps_mapped_shape_and_object_material_frames_distinct() {
    // This downstream wrapper imports its primitive explicitly. The renderer
    // has no knowledge of this implementation or of the imported plane module.
    let mut module = ShaderModule::new(
        "tests.plane-alias",
        ShaderKind::Shape,
        "fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit { return {{dep0}}(base,ray,previous); }",
        Some(1),
    );
    module.dependencies.push(shape::plane_schema());
    let plane = ShapeValue::new(module, vec![0]).unwrap();
    let material = custom_material(
        "tests.local-height",
        r#"
fn {{self}}(base:u32,ctx:GeoMaterialContext,
                     sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    (*sample).emission=vec3<f32>(ctx.position.w);
    (*sample).alive=0u;
}
"#,
    );
    let shift = Transform::Euclidean(Euclidean3::shift_z(1.0));
    let mapped_shape = definition(plane.clone().mapped(shift).unwrap(), material.clone());
    let mut mapped_object = definition(plane, material);
    mapped_object.object = ObjectNode::Mapped {
        map: shift,
        inner: Box::new(mapped_object.object),
    };
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    for (definition, height) in [(mapped_shape, 1.0), (mapped_object, 0.0)] {
        let renderer = Renderer::new(
            &gpu.device,
            &gpu.queue,
            (3, 3),
            Scene::from_definition(&definition).unwrap(),
            19,
        )
        .unwrap();
        renderer.render();
        for pixel in renderer.snapshot().unwrap() {
            assert_eq!(pixel[3], 1.0);
            // Embedded advancement and the two frame transforms introduce
            // ordinary f32 coordinate roundoff; the frame separation is 1.0.
            for value in &pixel[..3] {
                assert!((value - height).abs() <= 8.0 * f32::EPSILON);
            }
        }
    }
}
