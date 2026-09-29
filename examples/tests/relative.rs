//! Camera-relative upload updates remain transactional and preserve scene data.
use ccgeom::{Geometry3, Hyperboloid3, Spherical3};
use hypertrace_examples as examples;
use hypertrace_renderer::{Camera, Gpu, Renderer, Scene, shader::Transform};
use objects::Scene as _;

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn spherical_camera_circuit_uses_physical_radius_without_recompilation() {
    use objects::{
        SceneImpl,
        background::ConstBg,
        material::{Absorbing, Emissive},
        object::Covered,
        shape::GeodesicSphere,
        view::PointView,
    };

    let radius = 2.0;
    let emission = [0.2, 0.4, 0.8];
    let mut builder = SceneImpl::<Spherical3, _, _, _, 1>::new(
        PointView::new(1.0),
        vec![Covered::new(
            GeodesicSphere::new(0.6),
            Emissive::new(Absorbing, emission.into()),
        )],
        ConstBg::new([0.0; 3].into()),
    );
    builder.radius = radius;
    let scene = Scene::from_definition(&builder.definition().unwrap()).unwrap();
    let canonical = bytemuck::cast_slice::<_, u8>(scene.objects()).to_vec();
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let mut renderer = Renderer::new(&gpu.device, &gpu.queue, (9, 7), scene, 41).unwrap();
    let revision = renderer.pipeline_revision();
    renderer.render();
    let initial = renderer.snapshot().unwrap();
    assert_eq!(
        initial,
        vec![[emission[0], emission[1], emission[2], 1.0]; 63]
    );

    let mut camera = renderer.scene().camera;
    let step = std::f64::consts::FRAC_PI_2 * radius;
    let mut travelled = 0.0;
    for quarter in 1..=4 {
        camera
            .move_local([step, 0.0, 0.0], [0.0; 3], radius)
            .unwrap();
        travelled += step;
        let map = camera.transform();
        let position = map.apply_vector([1.0, 0.0, 0.0, 0.0]);
        let phase = travelled / radius;
        for (actual, expected) in position
            .into_iter()
            .zip([phase.cos(), phase.sin(), 0.0, 0.0])
        {
            assert!((actual - expected).abs() < 1e-12);
        }
        if quarter == 2 {
            assert!(
                position[0] < -0.999999999999,
                "the antipode must remain distinct from the origin"
            );
        }

        renderer.update_camera(camera, 1.0).unwrap();
        assert_eq!(renderer.pipeline_revision(), revision);
        assert_eq!(renderer.snapshot().unwrap(), vec![[0.0; 4]; 63]);
        assert_eq!(
            bytemuck::cast_slice::<_, u8>(renderer.scene().objects()),
            canonical
        );
        renderer.render();
        let pixels = renderer.snapshot().unwrap();
        assert!(pixels.iter().flatten().all(|value| value.is_finite()));
        assert!(pixels.iter().all(|pixel| pixel[3] == 1.0));
        if quarter == 4 {
            assert_eq!(
                pixels, initial,
                "one full physical circumference returns the original view"
            );
            // Quaternion pairs form a double cover: compare their complete
            // point/tangent action instead of requiring identical storage sign.
            for basis in [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ] {
                for (actual, expected) in map.apply_vector(basis).into_iter().zip(basis) {
                    assert!((actual - expected).abs() < 1e-12);
                }
            }
        }
    }
    assert_eq!(travelled, std::f64::consts::TAU * radius);
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn camera_updates_reprepare_objects_without_pipeline_changes() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    for definition in [
        examples::hy::scene::<3>().definition().unwrap(),
        examples::sp::scene::<6>().definition().unwrap(),
    ] {
        let scene = Scene::from_definition(&definition).unwrap();
        let canonical = bytemuck::cast_slice::<_, u8>(scene.objects()).to_vec();
        let mut renderer = Renderer::new(&gpu.device, &gpu.queue, (9, 7), scene, 19).unwrap();
        let revision = renderer.pipeline_revision();
        let mut camera = renderer.scene().camera;
        camera
            .move_local(
                [0.02, 0.0, -0.03],
                [0.01, 0.02, 0.0],
                f64::from(renderer.scene().radius),
            )
            .unwrap();
        renderer.update_camera(camera, 0.95).unwrap();
        assert_eq!(renderer.pipeline_revision(), revision);
        assert_eq!(renderer.snapshot().unwrap(), vec![[0.0; 4]; 63]);
        assert_eq!(
            bytemuck::cast_slice::<_, u8>(renderer.scene().objects()),
            canonical
        );
        renderer.render();
        let rendered = renderer.snapshot().unwrap();
        assert!(rendered.iter().all(|pixel| pixel[3] == 1.0));
        let installed = renderer.scene().camera.transform().components().unwrap();

        // In hyperbolic space this exceeds the supported relative f32 range;
        // for a spherical scene it is also the wrong camera geometry.
        let invalid = Camera::from(Transform::from_isometry(Hyperboloid3::shift_x(15.0)).unwrap());
        assert!(renderer.update_camera(invalid, 1.0).is_err());
        assert_eq!(renderer.pipeline_revision(), revision);
        assert_eq!(
            renderer.scene().camera.transform().components().unwrap(),
            installed
        );
        assert_eq!(renderer.scene().fov, 0.95);
        assert_eq!(renderer.snapshot().unwrap(), rendered);
    }
}
