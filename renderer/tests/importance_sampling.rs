use ccgeom::{Embedded3, EmbeddedIsometry, Space3};
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::*};
use objects::{
    light::{LightSampler, SphereBound},
    material::{self, MaterialValueExt},
    shape,
};

type G<const K: i8> = Embedded3<f64, K>;
type Map<const K: i8> = EmbeddedIsometry<f64, K>;

fn definition<const K: i8>() -> SceneDefinition<G<K>>
where
    G<K>: Geometry<Map = Map<K>>,
{
    let space = Space3::<f64, K>::unit();
    let light_map =
        Transform::from_isometry(space.translation([0.0, 0.0, 1.0].into(), 0.8).unwrap()).unwrap();
    let mut sampling =
        <SphereBound as LightSampler<G<K>>>::encode(&SphereBound::new(0.08)).unwrap();
    sampling.map = light_map;
    SceneDefinition {
        view: View {
            map: Transform::from_isometry(space.translation([0.0, 0.0, 1.0].into(), 0.3).unwrap())
                .unwrap(),
            fov: 1e-7,
        },
        background: Background::constant([0.0; 3]),
        bounces: 2,
        radius: 1.0,
        medium: Medium::vacuum(),
        objects: vec![
            EncodedObject {
                map: Transform::identity(),
                shape: shape::plane(),
                material: material::lambertian(),
                sampling: None,
            },
            EncodedObject {
                map: light_map,
                shape: shape::geodesic_sphere(0.08).unwrap(),
                material: material::absorbing().emissive([1.0; 3]).unwrap(),
                sampling: Some(sampling),
            },
        ],
        modules: Modules::default(),
    }
}

fn render<T: Geometry>(gpu: &Gpu, definition: &SceneDefinition<T>, samples: u32) -> Vec<f32> {
    let mut renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (128, 1),
        Scene::from_definition(definition).unwrap(),
        37,
    )
    .unwrap();
    renderer.set_samples_per_dispatch(samples.min(64)).unwrap();
    for _ in 0..samples.div_ceil(64) {
        renderer.render();
    }
    renderer.snapshot().unwrap().iter().map(|p| p[0]).collect()
}
fn mean(values: &[f32]) -> f32 {
    values.iter().sum::<f32>() / values.len() as f32
}

fn analytic<const K: i8>(gpu: &Gpu)
where
    G<K>: Geometry<Map = Map<K>>,
{
    let mut scene = definition::<K>();
    let sine = |x: f64| match K {
        -1 => x.sinh(),
        0 => x,
        _ => x.sin(),
    };
    let expected = (sine(0.08) / sine(0.8)).powi(2) as f32;
    let mis = render(gpu, &scene, 128);
    scene.objects[1].sampling = None;
    let ordinary = render(gpu, &scene, 128);
    let mse = |values: &[f32]| {
        values.iter().map(|x| (x - expected).powi(2)).sum::<f32>() / values.len() as f32
    };
    eprintln!(
        "K={K}: analytic={expected}, MIS mean={} MSE={}, ordinary mean={} MSE={}",
        mean(&mis),
        mse(&mis),
        mean(&ordinary),
        mse(&ordinary)
    );
    assert!((mean(&mis) - expected).abs() < 0.025 * expected);
    assert!((mean(&ordinary) - expected).abs() < 0.4 * expected);
    assert!(mse(&mis) < 0.05 * mse(&ordinary));
    // The source is outside the camera's backward-facing primary rays; one
    // interaction cannot connect the diffuse receiver to a second surface.
    scene.objects[1].sampling = definition::<K>().objects[1].sampling.clone();
    scene.bounces = 1;
    assert_eq!(render(gpu, &scene, 1), vec![0.0; 128]);
    // Preserve ideal reflection, including within a diffuse/delta mixture.
    scene.bounces = 2;
    scene.objects[0].material = material::mixture(vec![
        (0.5, material::lambertian()),
        (0.5, material::specular()),
    ])
    .unwrap();
    assert!((mean(&render(gpu, &scene, 512)) - (0.5 + 0.5 * expected)).abs() < 0.015);
    // An opaque object blocks both the explicit connection and continuation.
    let space = Space3::<f64, K>::unit();
    scene.objects.push(EncodedObject {
        map: Transform::from_isometry(space.translation([0.0, 0.0, 1.0].into(), 0.5).unwrap())
            .unwrap(),
        shape: shape::geodesic_sphere(0.15).unwrap(),
        material: material::absorbing(),
        sampling: None,
    });
    assert_eq!(render(gpu, &scene, 64), vec![0.0; 128]);
}

#[test]
fn light_validation_and_structure_are_independent_of_parameters() {
    fn check<const K: i8>()
    where
        G<K>: Geometry<Map = Map<K>>,
    {
        let a = definition::<K>();
        let compiled = compile(&a).unwrap();
        assert_eq!(compiled.light_count, 1);
        let mut b = a.clone();
        b.objects[1].sampling.as_mut().unwrap().words[0] = 0.2f32.to_bits();
        assert_eq!(compiled.source, compile(&b).unwrap().source);
        for radius in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::MIN_POSITIVE / 2.0] {
            b.objects[1].sampling.as_mut().unwrap().words[0] = radius.to_bits();
            assert!(compile(&b).is_err());
        }
    }
    check::<-1>();
    check::<0>();
    check::<1>();
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn direct_sphere_illumination_matches_analytic_integrals_and_reduces_variance() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    eprintln!("importance sampling adapter: {:?}", gpu.adapter.get_info());
    analytic::<-1>(&gpu);
    analytic::<0>(&gpu);
    analytic::<1>(&gpu);
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn emission_weights_do_not_attenuate_transmitted_continuation() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let mut scene = definition::<0>();
    scene.bounces = 3;
    scene.background = Background::constant([2.0; 3]);
    scene.objects[0].material = material::lambertian().colored([0.5; 3]).unwrap();
    scene.objects[1].shape = shape::plane();
    scene.objects[1].material = material::transparent().emissive([1.0; 3]).unwrap();
    let value = mean(&render(&gpu, &scene, 512));
    assert!(
        (value - 1.5).abs() < 0.01,
        "transmitted background plus emission: {value}"
    );
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn off_axis_volume_lighting_matches_independent_single_scattering_integral() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let mut scene = definition::<0>();
    scene.view.map = Transform::identity();
    scene.objects.remove(0);
    let position = [0.6f64, 0.0, -1.2];
    let map = Transform::from_isometry(
        Space3::<f64, 0>::unit()
            .translation(position.into(), position[0].hypot(position[2]))
            .unwrap(),
    )
    .unwrap();
    scene.objects[0].map = map;
    scene.objects[0].shape = shape::geodesic_sphere(0.15).unwrap();
    scene.objects[0].sampling.as_mut().unwrap().map = map;
    scene.objects[0].sampling.as_mut().unwrap().words[0] = 0.15f32.to_bits();
    scene.objects.push(EncodedObject {
        map: Transform::identity(),
        shape: shape::geodesic_sphere(2.5).unwrap(),
        material: material::absorbing(),
        sampling: None,
    });
    scene.medium = Medium::homogeneous(0.6, [0.8; 3]);
    // Deterministic quadrature over primary physical distance and source solid
    // angle, independent of the shader's free-flight and cone samplers.
    let mut expected = 0.0f64;
    for i in 0..2048 {
        let distance = 2.5 * (i as f64 + 0.5) / 2048.0;
        let d = 0.6f64.hypot(distance - 1.2);
        let cosine = (1.0 - (0.15 / d).powi(2)).sqrt();
        let mut angular = 0.0;
        for j in 0..128 {
            let mu = cosine + (1.0 - cosine) * (j as f64 + 0.5) / 128.0;
            let flight = d * mu - (0.15f64.powi(2) - d * d * (1.0 - mu * mu)).max(0.0).sqrt();
            angular += (-0.6 * flight).exp();
        }
        expected += 0.6 * (-0.6 * distance).exp() * 0.8 * 0.5 * angular * (1.0 - cosine) / 128.0;
    }
    expected *= 2.5 / 2048.0;
    let mis = mean(&render(&gpu, &scene, 1024));
    let sampling = scene.objects[0].sampling.take();
    let ordinary = mean(&render(&gpu, &scene, 1024));
    eprintln!("single scattering: quadrature={expected}, MIS={mis}, ordinary={ordinary}");
    assert!((f64::from(mis) - expected).abs() < 0.035 * expected);
    assert!((f64::from(ordinary) - expected).abs() < 0.2 * expected);
    scene.objects[0].sampling = sampling;
    scene.medium.albedo = [0.0; 3];
    assert_eq!(render(&gpu, &scene, 64), vec![0.0; 128]);
    scene.medium.albedo = [0.8; 3];
    scene.objects[0].material = material::absorbing();
    assert_eq!(render(&gpu, &scene, 64), vec![0.0; 128]);
}
