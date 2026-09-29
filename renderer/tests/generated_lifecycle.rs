//! Composition regressions that require actual WGSL execution.
use ccgeom::{Euclidean3, Geometry3};
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::*};
use objects::Material as _;

fn planes(count: usize) -> SceneDefinition {
    let shapes: Vec<_> = (0..count)
        .map(|index| {
            objects::Mapped::<Euclidean3, _, _>::new(
                objects::shape::Plane,
                Euclidean3::shift_z(-(index as f64)),
            )
        })
        .collect();
    let shape = <Vec<_> as objects::Shape<Euclidean3>>::encode(&shapes).unwrap();
    SceneDefinition {
        view: View {
            map: Transform::Euclidean(Euclidean3::shift_z(3.0)),
            fov: 0.1,
        },
        background: Background::Constant([0.0; 3]),
        bounces: 4,
        radius: 1.0,
        medium: Default::default(),
        object: ObjectNode::Covered {
            shape,
            material: objects::material::Emissive::new(
                objects::material::Transparent,
                [0.25, 0.5, 0.75].into(),
            )
            .encode()
            .unwrap(),
        },
        modules: vec![],
    }
}

#[test]
fn compiled_geometry_stays_coupled_to_validated_payload() {
    let mut scene = Scene::from_definition(&planes(1)).unwrap();
    scene.camera = Transform::Hyperboloid(ccgeom::EmbeddedIsometry::identity()).into();
    assert!(
        scene
            .validate()
            .unwrap_err()
            .to_string()
            .contains("geometry")
    );
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
    if let ObjectNode::Covered { material, .. } = &mut invalid.object {
        *material = MaterialValue::new(ShaderModule::new(
            "tests::invalid", ShaderKind::Material,
            "fn {{self}}(base:u32, ctx:GeoMaterialContext, sample:ptr<function,MaterialSample>, rng:ptr<function,u32>) { missing_function(); }",
            Some(0),
        ), vec![]).unwrap();
    }
    let error = renderer
        .update_scene(Scene::from_definition(&invalid).unwrap())
        .unwrap_err();
    assert!(error.to_string().contains("WGSL pipeline"));
    assert_eq!(renderer.pipeline_revision(), 1);
    renderer.render();
    assert_eq!(renderer.snapshot().unwrap(), before);
}
