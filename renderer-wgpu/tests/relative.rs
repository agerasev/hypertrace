//! Camera-relative upload updates remain transactional and preserve scene data.
use ccgeom::{Geometry3, Hyperboloid3};
use hypertrace_wgpu::{Camera, Gpu, Renderer, Scene, wgsl::Transform};
use objects::Scene as _;

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn camera_updates_reprepare_objects_without_pipeline_changes() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    for definition in [
        scenes::hy::scene::<3>().wgsl_scene().unwrap(),
        scenes::sp::scene::<6>().wgsl_scene().unwrap(),
    ] {
        let scene = Scene::from_definition(&definition).unwrap();
        let canonical = bytemuck::cast_slice::<_, u8>(&scene.objects).to_vec();
        let mut renderer = Renderer::new(&gpu.device, &gpu.queue, (9, 7), scene, 19).unwrap();
        let revision = renderer.pipeline_revision();
        let mut camera = renderer.scene().camera;
        camera
            .move_local_with_radius(
                [0.02, 0.0, -0.03],
                [0.01, 0.02, 0.0],
                f64::from(renderer.scene().radius),
            )
            .unwrap();
        renderer.update_camera(camera, 0.95).unwrap();
        assert_eq!(renderer.pipeline_revision(), revision);
        assert_eq!(renderer.snapshot().unwrap(), vec![[0.0; 4]; 63]);
        assert_eq!(
            bytemuck::cast_slice::<_, u8>(&renderer.scene().objects),
            canonical
        );
        renderer.render();
        let rendered = renderer.snapshot().unwrap();
        assert!(rendered.iter().all(|pixel| pixel[3] == 1.0));
        let installed = renderer.scene().camera.transform().components().unwrap();

        // In hyperbolic space this exceeds the supported relative f32 range;
        // for a spherical scene it is also the wrong camera geometry.
        let invalid = Camera::Embedded(Transform::Hyperboloid(Hyperboloid3::shift_x(15.0)));
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
