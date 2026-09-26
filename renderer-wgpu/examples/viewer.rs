//! Native and web viewer. WASD/arrows move, Space/C move vertically, Q/E roll,
//! left-drag looks around, scroll changes field of view, R resets, Esc closes.
//! `--scene eu|hy --smoke` processes twelve frames with a small binding limit,
//! resizing across that limit and back, camera updates and one discarded frame.

use hypertrace_wgpu::{Presenter, Renderer, Scene, fit_render_size};
use wgame::{
    Window, WindowConfig,
    app::time::Instant,
    canvas::{Button, Event, Key},
    gfx::Target,
};

mod support;
#[cfg(target_arch = "wasm32")]
#[path = "support/web.rs"]
mod web;

const SEED: u32 = 1;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    let result = start().await;
    #[cfg(target_arch = "wasm32")]
    {
        if let Err(error) = result {
            web::set_status(&format!("Unable to render: {error:#}"), true);
        }
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    result
}

#[cfg(not(target_arch = "wasm32"))]
fn options() -> wgame::Result<(String, bool)> {
    let mut scene = "hy".to_owned();
    let mut smoke = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--scene" => {
                scene = args
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("--scene needs eu or hy"))?
            }
            "--smoke" => smoke = true,
            "--help" | "-h" => {
                println!(
                    "viewer [--scene eu|hy] [--smoke]\nWASD/arrows: move; Space/C: up/down; Q/E: roll; left-drag: look; scroll: zoom; R: reset; Esc: close"
                );
                return Ok((String::new(), false));
            }
            _ => anyhow::bail!("unknown option {arg}; use --help"),
        }
    }
    Ok((scene, smoke))
}

async fn start() -> wgame::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    let (scene, smoke) = options()?;
    #[cfg(target_arch = "wasm32")]
    let (scene, smoke) = {
        anyhow::ensure!(
            web::supported(),
            "WebGPU is unavailable. Use a WebGPU-capable browser on HTTPS or localhost."
        );
        web::set_status("Preparing scene…", false);
        (web::scene_name(), false)
    };
    if scene.is_empty() {
        return Ok(());
    }
    let initial_scene = support::scene(&scene)?;
    let config = WindowConfig::default()
        .title(&format!("Hypertrace · WGPU · {scene}"))
        .size(if smoke { (320, 240) } else { (960, 720) })
        .required_limits(wgpu::Limits {
            // Exercise the hardware-limit fallback with small window sizes.
            max_storage_buffer_binding_size: if smoke {
                2 << 20
            } else {
                wgpu::Limits::default().max_storage_buffer_binding_size
            },
            ..Default::default()
        })
        .use_adapter_buffer_limits(!smoke);
    wgame::within_window(config, async move |window| {
        run(window, initial_scene, scene, smoke).await
    })
    .await
}

async fn run(
    mut window: Window<'_>,
    initial_scene: Scene,
    _scene_name: String,
    smoke: bool,
) -> wgame::Result<()> {
    #[cfg(target_arch = "wasm32")]
    let (mut scene_name, mut initial_scene) = (_scene_name, initial_scene);
    let graphics = window.graphics().clone();
    eprintln!("Adapter: {:?}", graphics.adapter().get_info());
    let raw = window.raw();
    let initial_size = raw.inner_size();
    let limits = graphics.device().limits();
    eprintln!(
        "Buffer limits: storage binding {} MiB, allocation {} MiB",
        limits.max_storage_buffer_binding_size / (1 << 20),
        limits.max_buffer_size / (1 << 20)
    );
    let initial_size = (initial_size.width.max(1), initial_size.height.max(1));
    #[cfg(target_arch = "wasm32")]
    anyhow::ensure!(
        graphics.adapter().get_info().backend == wgpu::Backend::BrowserWebGpu,
        "this browser did not provide a WebGPU adapter"
    );
    let render_size = viewer_render_size(&limits, initial_size)?;
    let mut was_scaled = render_size != initial_size;
    if was_scaled {
        report_scaled_size(initial_size, render_size);
    }
    let mut renderer = Renderer::new_async(
        graphics.device(),
        graphics.queue(),
        render_size,
        initial_scene.clone(),
        SEED,
    )
    .await?;
    let mut presenter = Presenter::new(graphics.device(), graphics.format(), &renderer);
    let mut camera = initial_scene.camera;
    let mut fov = initial_scene.fov;
    let mut previous = Instant::now();
    let mut frames = 0;
    let mut smoke_resized = false;
    let mut smoke_scaled = false;
    let mut smoke_restored = false;
    #[cfg(target_arch = "wasm32")]
    let mut samples = 0u64;
    #[cfg(target_arch = "wasm32")]
    web::set_status("Ready · click the scene to explore", false);
    while let Some(mut frame) = window.next_frame().await? {
        #[cfg(target_arch = "wasm32")]
        if web::scene_name() != scene_name {
            frame.discard();
            web::set_status("Preparing scene…", false);
            scene_name = web::scene_name();
            initial_scene = support::scene(&scene_name)?;
            renderer.update_scene_async(initial_scene.clone()).await?;
            presenter.rebind(graphics.device(), &renderer);
            camera = initial_scene.camera;
            fov = initial_scene.fov;
            samples = 0;
            web::set_status("Ready · click the scene to explore", false);
            continue;
        }
        let now = Instant::now();
        // Integrate elapsed time, bounding a stalled/minimized window's first
        // movement update so resuming cannot cause a large camera jump.
        let dt = (now - previous).as_secs_f64().min(0.1);
        previous = now;
        let size = frame.size();
        if size.0 == 0 || size.1 == 0 {
            frame.discard();
            continue;
        }
        let render_size = viewer_render_size(&limits, size)?;
        let scaled = render_size != size;
        if scaled && !was_scaled {
            report_scaled_size(size, render_size);
        }
        if !scaled && was_scaled {
            eprintln!("Rendering at the window's full resolution again");
        }
        was_scaled = scaled;
        if smoke {
            smoke_scaled |= scaled;
            smoke_restored |= smoke_scaled && !scaled;
        }
        if renderer.resize(render_size)? {
            #[cfg(target_arch = "wasm32")]
            {
                samples = 0;
            }
            presenter.rebind(graphics.device(), &renderer);
            if frames > 0 {
                smoke_resized = true;
            }
        }
        let input = frame.input();
        let mut reset = false;
        #[cfg(target_arch = "wasm32")]
        {
            reset |= web::take_reset();
        }
        let mut zoom = 0.0;
        for event in &input.events {
            match event {
                Event::Key {
                    key: Key::Escape,
                    pressed: true,
                    repeat: false,
                } => {
                    #[cfg(target_arch = "wasm32")]
                    web::toggle_pause();
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        frame.discard();
                        return Ok(());
                    }
                }
                Event::Key {
                    key: Key::Character('r'),
                    pressed: true,
                    repeat: false,
                } => reset = true,
                Event::Scroll(delta) => zoom += delta.y,
                _ => {}
            }
        }
        if reset {
            camera = initial_scene.camera;
            fov = initial_scene.fov;
            renderer.update_scene_async(initial_scene.clone()).await?;
            #[cfg(target_arch = "wasm32")]
            {
                samples = 0;
            }
        }
        let key = |letter| input.key_down(Key::Character(letter));
        let axis = |positive: bool, negative: bool| f64::from(positive) - f64::from(negative);
        let mut translation = [
            axis(
                key('d') || input.key_down(Key::ArrowRight),
                key('a') || input.key_down(Key::ArrowLeft),
            ) * dt,
            axis(input.key_down(Key::Space), key('c')) * dt,
            axis(
                key('s') || input.key_down(Key::ArrowDown),
                key('w') || input.key_down(Key::ArrowUp),
            ) * dt,
        ];
        let mut rotation = [0.0, 0.0, axis(key('q'), key('e')) * dt];
        if input.button_down(Button::Primary) {
            rotation[0] -= 2.0 * f64::from(input.relative_motion.y) / f64::from(size.1);
            rotation[1] -= 2.0 * f64::from(input.relative_motion.x) / f64::from(size.0);
        }
        if smoke && (frames == 5 || frames == 7) {
            translation[0] += 0.02;
            rotation[1] += 0.01;
        }
        if translation
            .into_iter()
            .chain(rotation)
            .any(|value| value != 0.0)
            || zoom != 0.0
        {
            camera.move_local(translation, rotation);
            fov = (fov * (-zoom * 0.002).exp()).clamp(0.05, 10.0);
            renderer.update_camera(camera, fov)?;
            #[cfg(target_arch = "wasm32")]
            {
                samples = 0;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let paused = false;
        #[cfg(target_arch = "wasm32")]
        let paused = web::paused();
        // Keep a visible preview after resetting/changing a paused scene.
        #[cfg(not(target_arch = "wasm32"))]
        let needs_preview = false;
        #[cfg(target_arch = "wasm32")]
        let needs_preview = samples == 0;
        if !paused || needs_preview {
            renderer.encode(frame.encoder());
            #[cfg(target_arch = "wasm32")]
            {
                samples += 1;
            }
        }
        #[cfg(target_arch = "wasm32")]
        web::set_stats(render_size.0, render_size.1, samples as f64);
        let view = frame.view().clone();
        presenter.draw(frame.encoder(), &view);
        if smoke && frames == 7 {
            // Camera/reset writes are queue operations and must remain valid
            // even when this frame's compute and presentation are discarded.
            frame.discard();
        } else {
            frame.present();
        }
        frames += 1;
        if smoke && frames == 3 {
            let _ = raw.request_inner_size(wgame::app::Size::new(640, 480));
        }
        if smoke && frames == 8 {
            let _ = raw.request_inner_size(wgame::app::Size::new(400, 300));
        }
        if smoke && frames == 12 {
            anyhow::ensure!(
                smoke_resized && smoke_scaled && smoke_restored,
                "smoke test did not observe resize across the binding limit and back"
            );
            eprintln!(
                "Viewer smoke passed: {frames} frames (one discarded), resize across buffer limit and back, camera updates, GPU presentation"
            );
            break;
        }
    }
    Ok(())
}

fn viewer_render_size(limits: &wgpu::Limits, size: (u32, u32)) -> wgame::Result<(u32, u32)> {
    #[cfg(target_arch = "wasm32")]
    let size = {
        let cap = web::resolution();
        let longest = size.0.max(size.1);
        if cap > 0 && longest > cap {
            (
                ((u64::from(size.0) * u64::from(cap) / u64::from(longest)) as u32).max(1),
                ((u64::from(size.1) * u64::from(cap) / u64::from(longest)) as u32).max(1),
            )
        } else {
            size
        }
    };
    fit_render_size(limits, size)
}

fn report_scaled_size(window: (u32, u32), render: (u32, u32)) {
    eprintln!(
        "Window {}x{} exceeds this device's full-resolution render capacity; rendering at {}x{} and scaling to the window",
        window.0, window.1, render.0, render.1
    );
}
