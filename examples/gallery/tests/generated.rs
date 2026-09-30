//! Generic Rust builders through generated WGSL, including GPU update semantics.
use ccgeom::{Flat3, Geometry3};
use hypertrace_gallery as examples;
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::Geometry};
use objects::{
    Mapped, Scene as _, SceneImpl,
    background::ConstBg,
    material::{Absorbing, Colored, Emissive},
    mixture,
    object::Covered,
    shape::Plane,
    view::PointView,
};

#[test]
fn static_groups_and_vector_lengths_do_not_change_shader_dependencies() {
    let mut builder = examples::euclidean::scene::<4>();
    let original = builder.definition().unwrap();
    let original_source = hypertrace_renderer::shader::compile(&original)
        .unwrap()
        .source;
    builder.object.0.clear();
    builder.object.1.push(builder.object.1[0].clone());
    let changed = builder.definition().unwrap();
    builder.object.0.clear();
    builder.object.1.clear();
    builder.object.2.clear();
    let empty = builder.definition().unwrap();
    for definition in [changed, empty] {
        assert_eq!(
            hypertrace_renderer::shader::compile(&definition)
                .unwrap()
                .source,
            original_source
        );
        Scene::from_definition(&definition).unwrap();
    }

    let mut builder = examples::hyperbolic::scene::<3>();
    let original = builder.definition().unwrap();
    let original_source = hypertrace_renderer::shader::compile(&original)
        .unwrap()
        .source;
    builder.object.0.push(builder.object.0[0].clone());
    builder.object.0.reverse();
    builder.object.1.clear();
    let changed = builder.definition().unwrap();
    builder.object.0.clear();
    builder.object.1.clear();
    builder.object.2.clear();
    builder.object.3.clear();
    builder.object.4.clear();
    for definition in [changed, builder.definition().unwrap()] {
        assert_eq!(
            hypertrace_renderer::shader::compile(&definition)
                .unwrap()
                .source,
            original_source
        );
        Scene::from_definition(&definition).unwrap();
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn all_standalone_scenes_render() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    for example in examples::EXAMPLES {
        examples::with_example!(example.id, |metadata, factory| {
            let name = metadata.id;
            let mut renderer = Renderer::new(
                &gpu.device,
                &gpu.queue,
                (37, 29),
                Scene::from_definition(&factory().unwrap()).unwrap(),
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
            Ok::<(), anyhow::Error>(())
        })
        .unwrap();
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn data_updates_and_empty_static_groups_reuse_the_pipeline() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let mut builder = examples::euclidean::scene::<4>();
    let compile = |builder: &examples::euclidean::ExampleScene<4>| {
        Scene::from_definition(&builder.definition().unwrap()).unwrap()
    };
    let mut renderer =
        Renderer::new(&gpu.device, &gpu.queue, (13, 11), compile(&builder), 123).unwrap();
    let revision = renderer.pipeline_revision();
    renderer.render();
    let original = renderer.snapshot().unwrap();

    builder.object.0.clear();
    builder.object.1[0].inner.material.diffuse.material.color = [0.9, 0.1, 0.4].into();
    builder.object.1[0].map = Flat3::shift_x(0.5).chain(Flat3::shift_y(-1.0));
    builder.view.inner.fov = 0.8;
    let duplicate = builder.object.1[0].clone();
    builder.object.1.push(duplicate);
    renderer.update_scene(compile(&builder)).unwrap();
    assert_eq!(renderer.pipeline_revision(), revision);
    assert!(renderer.snapshot().unwrap().iter().all(|&p| p == [0.0; 4]));
    renderer.render();
    let changed = renderer.snapshot().unwrap();
    assert_ne!(changed, original);
    assert!(changed.iter().all(|p| p[3] == 1.0));

    builder.object.0.clear();
    builder.object.1.clear();
    builder.object.2.clear();
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

fn emission_scene(first: bool) -> Scene<Flat3> {
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

fn plane_scene<M: objects::Material<Flat3>>(material: M) -> Scene<Flat3> {
    let builder = SceneImpl::<Flat3, _, _, _, 1>::new(
        Mapped::new(PointView::<Flat3>::new(1.0), Flat3::shift_z(1.0)),
        Covered::new(Plane, material),
        ConstBg::new([0.0; 3].into()),
    );
    Scene::from_definition(&builder.definition().unwrap()).unwrap()
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
    fn shader<G: Geometry>() -> objects::shader::MaterialModule<G> {
        objects::shader::MaterialModule::new(
            "test.constant-glow",
            r#"
fn {{self}}(base: u32, context: GeoMaterialContext,
            sample: ptr<function, MaterialSample>, rng: ptr<function, u32>) {
    (*sample).emission += (*sample).attenuation * load_vec3(base);
    (*sample).alive = 0u;
}
"#,
            Some(3),
        )
    }
}

impl<G: Geometry> objects::Material<G> for CustomGlow {
    fn shader() -> objects::shader::Result<objects::shader::MaterialModule<G>> {
        Ok(Self::shader())
    }

    fn encode(&self) -> objects::shader::Result<objects::shader::MaterialValue<G>> {
        objects::shader::MaterialValue::new(
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
