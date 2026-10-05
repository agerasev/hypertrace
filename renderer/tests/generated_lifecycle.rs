//! Composition regressions that require actual WGSL execution.
use ccgeom::{Flat3, Geometry3};
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::*};
use objects::Material as _;

fn planes(count: usize) -> SceneDefinition<Flat3> {
    let shapes: Vec<_> = (0..count)
        .map(|index| {
            objects::Mapped::<Flat3, _, _>::new(
                objects::shape::Plane,
                Flat3::shift_z(-(index as f64)),
            )
        })
        .collect();
    let shape = <Vec<_> as objects::Shape<Flat3>>::encode(&shapes).unwrap();
    SceneDefinition {
        view: View {
            map: Transform::from_isometry(Flat3::shift_z(3.0)).unwrap(),
            fov: 0.1,
        },
        background: Background::constant([0.0; 3]),
        bounces: 4,
        radius: 1.0,
        medium: Default::default(),
        objects: vec![EncodedObject {
            sampling: None,
            map: Transform::identity(),
            shape,
            material: objects::material::Emissive::new(
                objects::material::Transparent,
                [0.25, 0.5, 0.75].into(),
            )
            .encode()
            .unwrap(),
        }],
        modules: Modules::default(),
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn shape_vector_hits_distinct_children_and_survives_empty_updates() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let mut renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (7, 5),
        Scene::from_definition(&planes(2)).unwrap(),
        37,
    )
    .unwrap();
    renderer.render();
    assert_eq!(renderer.snapshot().unwrap(), vec![[0.5, 1.0, 1.5, 1.0]; 35]);
    for count in [1, 0, 3] {
        renderer
            .update_scene(Scene::from_definition(&planes(count)).unwrap())
            .unwrap();
        assert_eq!(
            renderer.pipeline_revision(),
            1,
            "shape vector length is data, not shader structure"
        );
        renderer.render();
        let n = count as f32;
        assert_eq!(
            renderer.snapshot().unwrap(),
            vec![[0.25 * n, 0.5 * n, 0.75 * n, 1.0]; 35]
        );
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn rejected_custom_shader_keeps_previous_pipeline_and_data() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let mut renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (3, 3),
        Scene::from_definition(&planes(1)).unwrap(),
        19,
    )
    .unwrap();
    renderer.render();
    let before = renderer.snapshot().unwrap();
    let mut invalid = planes(1);
    invalid.objects[0].material = MaterialValue::new(MaterialModule::new(
            "tests::invalid",
            "fn {{self}}(base:u32, ctx:GeoMaterialContext, sample:ptr<function,MaterialSample>, rng:ptr<function,u32>) { missing_function(); }",
            Some(0),
        ), vec![]).unwrap();
    let error = renderer
        .update_scene(Scene::from_definition(&invalid).unwrap())
        .unwrap_err();
    assert!(error.to_string().contains("WGSL pipeline"));
    assert_eq!(renderer.pipeline_revision(), 1);
    renderer.render();
    assert_eq!(renderer.snapshot().unwrap(), before);
}
