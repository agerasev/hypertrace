//! Explicit GPU checks: `cargo test -p hypertrace-wgpu --test renderer -- --ignored`.
use hypertrace_wgpu::{Background, Gpu, Renderer, Scene};

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn accumulation_reset_resize_and_scene_upload() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let mut scene = Scene::eu();
    scene.objects.clear();
    scene.materials.clear();
    scene.background = Background::Constant([0.25, 0.5, 0.75]);
    let mut r = Renderer::new(&gpu.device, &gpu.queue, (13, 7), scene.clone(), 123).unwrap();
    assert!(r.snapshot().unwrap().iter().all(|&p| p == [0.0; 4]));
    r.set_samples_per_dispatch(3).unwrap();
    r.render();
    assert_eq!(r.snapshot().unwrap(), vec![[0.25, 0.5, 0.75, 1.0]; 91]);
    r.reset(123);
    assert!(r.snapshot().unwrap().iter().all(|&p| p == [0.0; 4]));
    assert!(r.resize((9, 11)).unwrap());
    assert!(!r.resize((9, 11)).unwrap());
    assert!(r.resize((0, 10)).is_err());
    assert!(r.resize((u32::MAX, u32::MAX)).is_err());
    r.render();
    assert_eq!(r.snapshot().unwrap(), vec![[0.25, 0.5, 0.75, 1.0]; 99]);
    // Grow from an empty scene, then shrink without compiling new pipelines.
    r.update_scene(Scene::hy()).unwrap();
    r.render();
    let pixels = r.snapshot().unwrap();
    assert!(pixels.iter().flatten().all(|x| x.is_finite()));
    assert!(pixels.iter().all(|p| p[3] == 1.0));
    r.update_scene(scene).unwrap();
    r.render();
    assert_eq!(r.snapshot().unwrap(), vec![[0.25, 0.5, 0.75, 1.0]; 99]);
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn fixed_seed_batching_and_repeated_renders_agree() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    for scene in [Scene::eu(), Scene::hy()] {
        let mut r = Renderer::new(&gpu.device, &gpu.queue, (24, 18), scene, 3735928559).unwrap();
        r.set_samples_per_dispatch(4).unwrap();
        r.render();
        let batch = r.snapshot().unwrap();
        assert!(
            batch.iter().all(|p| p[3] == 1.0),
            "first frame must contain every pixel"
        );
        r.reset(3735928559);
        r.set_samples_per_dispatch(1).unwrap();
        for _ in 0..4 {
            r.render();
        }
        assert_eq!(batch, r.snapshot().unwrap());
        let camera = r.scene().camera;
        r.update_camera(camera, 1.0).unwrap();
        assert!(r.snapshot().unwrap().iter().all(|&p| p == [0.0; 4]));
    }
}
