//! Explicit GPU checks: `cargo test -p hypertrace-examples --test renderer -- --ignored`.
use hypertrace_renderer::{Gpu, Renderer};
mod support;

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn resize_fits_storage_binding_limit_and_recovers_full_resolution() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let (device, queue) =
        futures::executor::block_on(gpu.adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("small image binding limit regression"),
            required_limits: wgpu::Limits {
                max_storage_buffer_binding_size: 1024,
                ..Default::default()
            },
            ..Default::default()
        }))
        .unwrap();
    let scene = support::background([0.25, 0.5, 0.75]);
    let mut renderer = Renderer::new(&device, &queue, (8, 8), scene, 37).unwrap();
    renderer.render();
    let before = renderer.snapshot().unwrap();
    assert!(
        renderer
            .resize((40, 30))
            .unwrap_err()
            .to_string()
            .contains("19200 bytes")
    );
    assert_eq!(renderer.size(), (8, 8));
    assert_eq!(renderer.snapshot().unwrap(), before);
    for requested in [(40, 30), (200, 100), (8, 8), (2, 3)] {
        let fitted = hypertrace_renderer::fit_render_size(&device.limits(), requested).unwrap();
        renderer.resize(fitted).unwrap();
        assert!(u64::from(fitted.0) * u64::from(fitted.1) * 16 <= 1024);
        if requested.0 <= 8 && requested.1 <= 8 {
            assert_eq!(fitted, requested);
        }
        renderer.render();
        assert_eq!(
            renderer.snapshot().unwrap(),
            vec![[0.25, 0.5, 0.75, 1.0]; (fitted.0 * fitted.1) as usize]
        );
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn accumulation_reset_resize_and_scene_upload() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let scene = support::background([0.25, 0.5, 0.75]);
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
    // Grow from an empty scene, then shrink back to an empty scene.
    r.update_scene(support::scene(hypertrace_examples::factories::eu))
        .unwrap();
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
    for name in ["eu", "hy", "sp"] {
        hypertrace_examples::with_example!(name, |_metadata, factory| {
            let scene = support::scene(factory);
            let mut r =
                Renderer::new(&gpu.device, &gpu.queue, (24, 18), scene, 3735928559).unwrap();
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
            Ok::<(), anyhow::Error>(())
        })
        .unwrap();
    }
}
