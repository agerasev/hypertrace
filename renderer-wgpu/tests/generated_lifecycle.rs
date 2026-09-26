//! Composition regressions that require actual WGSL execution.
use ccgeom::{Euclidean3, Geometry3};
use hypertrace_wgpu::{Gpu, Renderer, Scene, wgsl::*};

fn planes(count: usize) -> SceneDefinition {
    let shape = ShapeValue::vector(
        ShapeSchema::Mapped {
            geometry: Geometry::Euclidean,
            inner: Box::new(ShapeSchema::Plane),
        },
        (0..count)
            .map(|index| {
                ShapeValue::plane()
                    .mapped(Transform::Euclidean(Euclidean3::shift_z(-(index as f64))))
                    .unwrap()
            })
            .collect(),
    )
    .unwrap();
    SceneDefinition {
        view: View {
            map: Transform::Euclidean(Euclidean3::shift_z(3.0)),
            fov: 0.1,
        },
        background: Background::Constant([0.0; 3]),
        bounces: 4,
        object: ObjectNode::Covered {
            shape,
            material: MaterialValue::transparent()
                .emissive([0.25, 0.5, 0.75])
                .unwrap(),
        },
        material_schemas: vec![],
        shape_schemas: vec![],
    }
}

#[test]
fn generated_records_and_geometry_stay_coupled_to_validated_payload() {
    let scene = Scene::from_definition(&planes(1)).unwrap();
    let mut invalid = scene.clone();
    invalid.objects[0].extra[1] = u32::MAX;
    assert!(
        invalid
            .validate()
            .unwrap_err()
            .to_string()
            .contains("SceneDefinition")
    );
    invalid = scene.clone();
    invalid.objects[0].info[0] = u32::MAX;
    assert!(invalid.validate().is_err());
    invalid = scene.clone();
    invalid
        .materials
        .push(hypertrace_wgpu::Material::diffuse([1.0; 3]));
    assert!(invalid.validate().is_err());
    invalid = scene;
    invalid.camera = hypertrace_wgpu::Scene::hy().camera;
    assert!(
        invalid
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
        *material=MaterialValue::custom(ShaderLeaf {
            key:"tests::invalid".into(),entry_point:"invalid_material".into(),parameter_words:0,
            source:"fn invalid_material(base:u32, ctx:MaterialContext, sample:ptr<function,MaterialSample>, rng:ptr<function,u32>) { missing_function(); }".into(),
        },vec![]).unwrap();
    }
    let error = renderer
        .update_scene(Scene::from_definition(&invalid).unwrap())
        .unwrap_err();
    assert!(error.to_string().contains("WGSL pipeline"));
    assert_eq!(renderer.pipeline_revision(), 1);
    renderer.render();
    assert_eq!(renderer.snapshot().unwrap(), before);
}
