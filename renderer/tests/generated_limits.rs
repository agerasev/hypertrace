//! Scene allocation limits must fail before changing a working renderer.
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::*};
use objects::Material as _;

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn scene_update_exceeding_allocation_limit_preserves_previous_renderer() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let (device, queue) =
        futures::executor::block_on(gpu.adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("scene allocation limit regression"),
            required_limits: wgpu::Limits {
                max_buffer_size: 256,
                ..Default::default()
            },
            ..Default::default()
        }))
        .unwrap();
    assert_eq!(device.limits().max_buffer_size, 256);
    assert!(device.limits().max_storage_buffer_binding_size > 256);

    let mut definition = SceneDefinition {
        view: View {
            map: Transform::identity(Geometry::Euclidean),
            fov: 1.0,
        },
        background: Background::Constant([0.125, 0.25, 0.5]),
        bounces: 1,
        radius: 1.0,
        medium: Default::default(),
        object: ObjectNode::Vector(vec![]),
        modules: vec![
            objects::material::Absorbing::shader().unwrap(),
            <objects::shape::Plane as objects::Shape<ccgeom::Euclidean3>>::shader().unwrap(),
        ],
    };
    let mut renderer = Renderer::new(
        &device,
        &queue,
        (1, 1),
        Scene::from_definition(&definition).unwrap(),
        123,
    )
    .unwrap();
    renderer.render();
    let expected = vec![[0.125, 0.25, 0.5, 1.0]];
    assert_eq!(renderer.snapshot().unwrap(), expected);
    let revision = renderer.pipeline_revision();
    let source = renderer.shader_source().to_owned();

    // Six 48-byte records exceed allocation size while remaining well below
    // the device's independent storage-binding limit. The schema is unchanged.
    definition.object = ObjectNode::Vector(vec![
        ObjectNode::Covered {
            shape: <objects::shape::Plane as objects::Shape<ccgeom::Euclidean3>>::encode(
                &objects::shape::Plane
            )
            .unwrap(),
            material: objects::material::Absorbing.encode().unwrap(),
        };
        6
    ]);
    let large_scene = Scene::from_definition(&definition).unwrap();
    let error = renderer.update_scene(large_scene).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("scene exceeds device buffer limit")
    );
    assert_eq!(renderer.pipeline_revision(), revision);
    assert_eq!(renderer.shader_source(), source);
    assert!(renderer.scene().objects().is_empty());
    assert_eq!(renderer.snapshot().unwrap(), expected);
    renderer.render();
    assert_eq!(renderer.snapshot().unwrap(), expected);
}
