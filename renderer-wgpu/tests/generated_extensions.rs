//! Execute extension leaves through the same compiler/runtime as scene builders.
use ccgeom::{Euclidean3, Geometry3};
use hypertrace_wgpu::{Gpu, Renderer, Scene, pixel_seed, wgsl::*};

fn definition(shape: ShapeValue, material: MaterialValue) -> SceneDefinition {
    SceneDefinition {
        view: View {
            map: Transform::Euclidean(Euclidean3::shift_z(3.0)),
            fov: 0.1,
        },
        background: Background::Constant([0.0; 3]),
        bounces: 1,
        object: ObjectNode::Covered { shape, material },
        material_schemas: vec![],
        shape_schemas: vec![],
    }
}

fn custom_material(key: &str, entry_point: &str, source: &str) -> MaterialValue {
    MaterialValue::custom(
        ShaderLeaf {
            key: key.into(),
            entry_point: entry_point.into(),
            source: source.into(),
            parameter_words: 0,
        },
        vec![],
    )
    .unwrap()
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn nested_single_component_mixtures_each_advance_rng_once() {
    let probe = custom_material(
        "tests.rng-state",
        "test_rng_state",
        r#"
fn test_rng_state(base:u32, ctx:MaterialContext,
                  sample:ptr<function,MaterialSample>, rng:ptr<function,u32>) {
    (*sample).emission=vec3<f32>(f32(*rng)*(1.0/4294967296.0));
    (*sample).alive=0u;
}
"#,
    );
    // Even deterministic, single-component mixtures draw independently. Removing
    // either draw changes the RNG seen by this leaf and all later path events.
    let inner = MaterialValue::mixture(vec![(1.0, probe)]).unwrap();
    let outer = MaterialValue::mixture(vec![(1.0, inner)]).unwrap();
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let seed = 37;
    let renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (5, 3),
        Scene::from_definition(&definition(ShapeValue::plane(), outer)).unwrap(),
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
    let plane = ShapeValue::custom(
        ShaderLeaf {
            key: "tests.plane-alias".into(),
            entry_point: "test_plane_alias".into(),
            parameter_words: 0,
            source: r#"
fn test_plane_alias(base:u32,ray:Ray,previous_identity:u32)->TaggedHit {
    return TaggedHit(eu_plane(ray,previous_identity==base),base);
}
"#
            .into(),
        },
        vec![],
    )
    .unwrap();
    let material = custom_material(
        "tests.local-height",
        "test_local_height",
        r#"
fn test_local_height(base:u32,ctx:MaterialContext,
                     sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    (*sample).emission=vec3<f32>(ctx.position.z);
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
        assert_eq!(
            renderer.snapshot().unwrap(),
            vec![[height, height, height, 1.0]; 9]
        );
    }
}
