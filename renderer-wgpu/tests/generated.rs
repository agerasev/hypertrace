//! Generic Rust builders through generated WGSL, including GPU update semantics.
use ccgeom::{Euclidean3, Homogenous3};
use hypertrace_wgpu::{Camera, Gpu, Renderer, Scene};
use objects::{
    Mapped, Scene as _, SceneImpl,
    background::ConstBg,
    material::{Absorbing, Colored, Emissive},
    mixture,
    object::Covered,
    shape::{Cube, Plane},
    view::PointView,
};
use vecmat::{
    Transform as _,
    transform::{Rotation3, Shift},
};

#[path = "../examples/support/mod.rs"]
mod support;

#[test]
fn shared_builders_compile_with_original_cameras_and_bounce_limits() {
    for (name, camera, bounces) in [
        ("eu", Camera::Euclidean(examples::eu::camera()), 4),
        ("hy", Camera::Hyperbolic(examples::hy::camera()), 3),
        (
            "sp",
            Camera::Embedded(hypertrace_wgpu::wgsl::Transform::Spherical(
                examples::sp::camera(),
            )),
            6,
        ),
    ] {
        let scene = support::scene(name).unwrap();
        scene.validate().unwrap();
        for (actual, expected) in scene
            .camera
            .transform()
            .components()
            .unwrap()
            .into_iter()
            .flatten()
            .zip(
                camera
                    .transform()
                    .components()
                    .unwrap()
                    .into_iter()
                    .flatten(),
            )
        {
            assert!((actual - expected).abs() < 1e-12);
        }
        assert_eq!(scene.fov, 1.0);
        assert_eq!(scene.bounces, bounces);
    }
}

#[test]
fn vector_length_and_active_choice_do_not_change_registered_schemas() {
    let mut builder = examples::eu::scene::<4>();
    let original = builder.wgsl_scene().unwrap();
    let original_source = hypertrace_wgpu::wgsl::compile(&original).unwrap().source;
    builder.object[0].inner.shape = examples::eu::Choice::Cube(Cube);
    builder.object.truncate(1);
    let changed = builder.wgsl_scene().unwrap();
    builder.object.clear();
    let empty = builder.wgsl_scene().unwrap();
    for definition in [changed, empty] {
        assert_eq!(definition.shape_schemas, original.shape_schemas);
        assert_eq!(definition.material_schemas, original.material_schemas);
        assert_eq!(
            hypertrace_wgpu::wgsl::compile(&definition).unwrap().source,
            original_source
        );
        Scene::from_definition(&definition).unwrap();
    }

    let mut builder = examples::hy::scene::<3>();
    let original = builder.wgsl_scene().unwrap();
    let original_source = hypertrace_wgpu::wgsl::compile(&original).unwrap().source;
    builder.object.reverse();
    builder.object.truncate(1);
    let changed = builder.wgsl_scene().unwrap();
    builder.object.clear();
    for definition in [changed, builder.wgsl_scene().unwrap()] {
        assert_eq!(definition.shape_schemas, original.shape_schemas);
        assert_eq!(definition.material_schemas, original.material_schemas);
        assert_eq!(
            hypertrace_wgpu::wgsl::compile(&definition).unwrap().source,
            original_source
        );
        Scene::from_definition(&definition).unwrap();
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn all_shared_builders_render_through_generated_wgsl() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    for example in examples::EXAMPLES {
        let name = example.id;
        let mut renderer = Renderer::new(
            &gpu.device,
            &gpu.queue,
            (37, 29),
            support::scene(name).unwrap(),
            123,
        )
        .unwrap();
        renderer.set_samples_per_dispatch(4).unwrap();
        renderer.render();
        let pixels = renderer.snapshot().unwrap();
        assert!(pixels.iter().flatten().all(|x| x.is_finite()));
        assert!(pixels.iter().all(|pixel| pixel[3] == 1.0));
        assert!(
            pixels.iter().any(|pixel| pixel != &pixels[0]),
            "{name} should contain visible geometry"
        );
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn data_updates_choice_switches_and_empty_vectors_reuse_the_pipeline() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let mut builder = examples::eu::scene::<4>();
    let compile = |builder: &examples::eu::ExampleScene<4>| {
        Scene::from_definition(&builder.wgsl_scene().unwrap()).unwrap()
    };
    let mut renderer =
        Renderer::new(&gpu.device, &gpu.queue, (13, 11), compile(&builder), 123).unwrap();
    let revision = renderer.pipeline_revision();
    renderer.render();
    let original = renderer.snapshot().unwrap();

    builder.object[0].inner.shape = examples::eu::Choice::Cube(Cube);
    builder.object[1].inner.material.diffuse.material.color = [0.9, 0.1, 0.4].into();
    builder.object[1].map = Shift::from_vector([0.5, -1.0, 0.0].into());
    builder.view.inner.fov = 0.8;
    let duplicate = builder.object[1].clone();
    builder.object.push(duplicate);
    renderer.update_scene(compile(&builder)).unwrap();
    assert_eq!(renderer.pipeline_revision(), revision);
    assert!(renderer.snapshot().unwrap().iter().all(|&p| p == [0.0; 4]));
    renderer.render();
    let changed = renderer.snapshot().unwrap();
    assert_ne!(changed, original);
    assert!(changed.iter().all(|p| p[3] == 1.0));

    builder.object.clear();
    builder.background.colors = [[0.25, 0.5, 0.75].into(); 2];
    renderer.update_scene(compile(&builder)).unwrap();
    assert_eq!(renderer.pipeline_revision(), revision);
    renderer.render();
    // The equal-endpoint gradient still executes a floating-point mix.
    for pixel in renderer.snapshot().unwrap() {
        assert_eq!(pixel[3], 1.0);
        for (actual, expected) in pixel[..3].iter().zip([0.25, 0.5, 0.75]) {
            assert!((actual - expected).abs() <= 2.0 * f32::EPSILON);
        }
    }

    // A material composition change really does require another pipeline.
    renderer.update_scene(emission_scene(true)).unwrap();
    assert!(renderer.pipeline_revision() > revision);
}

mixture! {
    InnerMixture {
        first: Emissive<Absorbing>,
        second: Emissive<Absorbing>,
    }
}

mixture! {
    OuterMixture {
        nested: Colored<InnerMixture>,
        inactive: Emissive<Absorbing>,
    }
}

fn emission_scene(first: bool) -> Scene {
    let weight = if first { 1.0 } else { 0.0 };
    let inner = InnerMixture::new(
        (Emissive::new(Absorbing, [0.5, 0.25, 0.125].into()), weight).into(),
        (
            Emissive::new(Absorbing, [0.25, 0.5, 0.75].into()),
            1.0 - weight,
        )
            .into(),
    );
    let outer = OuterMixture::new(
        (Colored::new(inner, [0.5, 0.25, 1.0].into()), 1.0).into(),
        (Emissive::new(Absorbing, [10.0; 3].into()), 0.0).into(),
    );
    plane_scene(Emissive::new(outer, [0.125, 0.25, 0.5].into()))
}

fn plane_scene<M: objects::Material>(material: M) -> Scene {
    let builder = SceneImpl::<Euclidean3, _, _, _, 1>::new(
        Mapped::new(
            PointView::<Euclidean3>::new(1.0),
            Homogenous3::new(
                Shift::from_vector([0.0, 0.0, 1.0].into()),
                Rotation3::identity(),
            ),
        ),
        Covered::new(Plane, material),
        ConstBg::new([0.0; 3].into()),
    );
    Scene::from_definition(&builder.wgsl_scene().unwrap()).unwrap()
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn nested_mixture_emission_and_modifier_order_are_preserved() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let mut renderer =
        Renderer::new(&gpu.device, &gpu.queue, (11, 7), emission_scene(true), 123).unwrap();
    let revision = renderer.pipeline_revision();
    for (first, expected) in [
        (true, [0.375, 0.3125, 0.625, 1.0]),
        (false, [0.25, 0.375, 1.25, 1.0]),
    ] {
        renderer.update_scene(emission_scene(first)).unwrap();
        assert_eq!(renderer.pipeline_revision(), revision);
        renderer.set_samples_per_dispatch(4).unwrap();
        renderer.render();
        assert_eq!(renderer.snapshot().unwrap(), vec![expected; 77]);
    }
}

#[derive(Clone)]
pub struct CustomGlow {
    pub color: vecmat::Vector<f32, 3>,
}

impl CustomGlow {
    fn shader() -> objects::wgsl::ShaderLeaf {
        objects::wgsl::ShaderLeaf {
            key: "test.constant-glow.v1".into(),
            source: r#"
fn test_constant_glow(base: u32, context: MaterialContext,
                     sample: ptr<function, MaterialSample>, rng: ptr<function, u32>) {
    (*sample).emission += (*sample).attenuation * load_vec3(base);
    (*sample).alive = 0u;
}
"#
            .into(),
            entry_point: "test_constant_glow".into(),
            parameter_words: 3,
        }
    }
}

impl objects::Material for CustomGlow {
    fn wgsl_material_schema() -> objects::wgsl::Result<objects::wgsl::MaterialSchema> {
        Ok(objects::wgsl::MaterialSchema::Custom(Self::shader()))
    }

    fn wgsl_material(&self) -> objects::wgsl::Result<objects::wgsl::MaterialValue> {
        objects::wgsl::MaterialValue::custom(
            Self::shader(),
            self.color.into_array().map(f32::to_bits).into(),
        )
    }
}

mixture! {
    CustomMixture {
        glowing: CustomGlow,
        absorbing: Absorbing,
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn custom_material_hook_extends_a_generic_mixture_without_renderer_changes() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let scene = |color| {
        plane_scene(CustomMixture::new(
            (CustomGlow { color }, 1.0).into(),
            (Absorbing, 0.0).into(),
        ))
    };
    let mut renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (7, 5),
        scene([0.125, 0.25, 0.5].into()),
        123,
    )
    .unwrap();
    let revision = renderer.pipeline_revision();
    for color in [[0.125, 0.25, 0.5], [0.75, 0.5, 0.25]] {
        renderer.update_scene(scene(color.into())).unwrap();
        assert_eq!(renderer.pipeline_revision(), revision);
        renderer.render();
        assert_eq!(
            renderer.snapshot().unwrap(),
            vec![[color[0], color[1], color[2], 1.0]; 35]
        );
    }
}
