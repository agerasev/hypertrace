//! Native and web viewer. WASD/arrows move, Space/C move vertically, Q/E roll,
//! left-drag looks around, Tab toggles mouse lock, scroll zooms, R resets, Esc closes.
//! `--scene NAME --smoke` processes twelve frames with a small binding limit,
//! resizing across that limit and back, camera updates and one discarded frame.

use hypertrace_renderer::{Presenter, Renderer, Scene, fit_render_size};
use wgame::{
    Window, WindowConfig,
    app::time::Instant,
    canvas::{Button, Event, Key},
    gfx::Target,
};

use crate::Example;
use objects::shader::{Geometry, Result, SceneDefinition};
#[cfg(not(target_arch = "wasm32"))]
mod cursor;
#[cfg(target_arch = "wasm32")]
mod web;

const SEED: u32 = 1;

/// Select a concrete application at startup. Browser selection reloads the page.
pub async fn run_gallery() -> wgame::Result<()> {
    finish(
        async {
            let (id, smoke) = selection(crate::EXAMPLES)?;
            if id.is_empty() {
                return Ok(());
            }
            crate::with_example!(id.as_str(), |example, factory| start(
                example, factory, smoke
            )
            .await)
        }
        .await,
    )
}

fn finish(result: wgame::Result<()>) -> wgame::Result<()> {
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

fn selection(examples: &[Example]) -> wgame::Result<(String, bool)> {
    anyhow::ensure!(!examples.is_empty(), "provide at least one example");
    #[cfg(not(target_arch = "wasm32"))]
    {
        options(examples)
    }
    #[cfg(target_arch = "wasm32")]
    {
        anyhow::ensure!(
            web::supported(),
            "WebGPU is unavailable. Use a WebGPU-capable browser on HTTPS or localhost."
        );
        for example in examples {
            web::add_example(
                example.id,
                example.title,
                example.description,
                example.group,
            );
        }
        web::set_status("Preparing scene…", false);
        Ok((web::scene_name(), false))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn options(examples: &[Example]) -> wgame::Result<(String, bool)> {
    let mut scene = examples[0].id.to_owned();
    let mut smoke = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--list-scenes" => {
                println!("{}", catalog(examples));
                return Ok((String::new(), false));
            }
            "--scene" => {
                scene = args.next().ok_or_else(|| {
                    anyhow::anyhow!("--scene needs an example name; use --list-scenes")
                })?
            }
            "--smoke" => smoke = true,
            "--help" | "-h" => {
                println!(
                    "viewer [--scene NAME] [--smoke] [--list-scenes]\nWASD/arrows: move; Space/C: up/down; Q/E: roll; left-drag: look; Tab: toggle mouse lock; scroll: zoom; R: reset; Esc: close"
                );
                return Ok((String::new(), false));
            }
            _ => anyhow::bail!("unknown option {arg}; use --help"),
        }
    }
    Ok((scene, smoke))
}

async fn start<G: Geometry>(
    example: Example,
    factory: fn() -> Result<SceneDefinition<G>>,
    smoke: bool,
) -> wgame::Result<()> {
    let initial_scene = Scene::from_definition(&factory()?)?;
    let config = WindowConfig::default()
        .title(&format!("Hypertrace · {}", example.title))
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
        render_loop(window, initial_scene, smoke).await
    })
    .await
}

async fn render_loop<G: Geometry>(
    mut window: Window<'_>,
    initial_scene: Scene<G>,
    smoke: bool,
) -> wgame::Result<()> {
    let graphics = window.graphics().clone();
    eprintln!("Adapter: {:?}", graphics.adapter().get_info());
    let raw = window.raw();
    #[cfg(not(target_arch = "wasm32"))]
    let mut capture = cursor::MouseCapture::new(raw);
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
    let mut motion_blocked = false;
    #[cfg(target_arch = "wasm32")]
    let mut samples = 0u64;
    #[cfg(target_arch = "wasm32")]
    web::set_ready();
    while let Some(mut frame) = window.next_frame().await? {
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
        #[cfg(not(target_arch = "wasm32"))]
        let mut suppress_look = false;
        #[cfg(target_arch = "wasm32")]
        let mut suppress_look = web::take_mouse_lock_change();
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
                #[cfg(not(target_arch = "wasm32"))]
                Event::Key {
                    key: Key::Tab,
                    pressed: true,
                    repeat: false,
                } => {
                    if let Err(error) = capture.set(!capture.active) {
                        eprintln!("Unable to change mouse lock: {error:#}");
                    }
                    suppress_look = true;
                }
                Event::Focused(false) => {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Err(error) = capture.set(false) {
                        eprintln!("Unable to release mouse: {error:#}");
                    }
                    suppress_look = true;
                }
                Event::Cancelled => suppress_look = true,
                Event::Scroll(delta) => zoom += delta.y,
                _ => {}
            }
        }
        if reset {
            camera = initial_scene.camera;
            fov = initial_scene.fov;
            renderer.update_scene_async(initial_scene.clone()).await?;
            motion_blocked = false;
            #[cfg(target_arch = "wasm32")]
            {
                samples = 0;
                web::set_ready();
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
        #[cfg(not(target_arch = "wasm32"))]
        let captured = capture.active;
        #[cfg(target_arch = "wasm32")]
        let captured = web::mouse_locked();
        if !suppress_look && input.focused && (captured || input.button_down(Button::Primary)) {
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
            let mut next_camera = camera;
            let next_fov = (fov * (-zoom * 0.002).exp()).clamp(0.05, 10.0);
            let updated = next_camera
                .move_local(translation, rotation, f64::from(renderer.scene().radius))
                .and_then(|()| renderer.update_camera(next_camera, next_fov));
            match updated {
                Ok(()) => {
                    camera = next_camera;
                    fov = next_fov;
                    #[cfg(target_arch = "wasm32")]
                    {
                        samples = 0;
                        if motion_blocked {
                            web::set_ready();
                        }
                    }
                    motion_blocked = false;
                }
                Err(_error) => {
                    // Retain both controller and renderer state so the user can
                    // move back from a numerical limit instead of exiting.
                    if !motion_blocked {
                        #[cfg(not(target_arch = "wasm32"))]
                        eprintln!("Camera movement stopped: {_error:#}; move back or reset");
                        #[cfg(target_arch = "wasm32")]
                        web::set_status("Movement limit reached · move back or reset", true);
                    }
                    motion_blocked = true;
                }
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
        #[cfg(not(target_arch = "wasm32"))]
        if smoke && frames == 2 {
            capture.set(true)?;
            anyhow::ensure!(capture.active, "smoke test failed to lock mouse");
        }
        #[cfg(not(target_arch = "wasm32"))]
        if smoke && frames == 4 {
            capture.set(false)?;
            anyhow::ensure!(!capture.active, "smoke test failed to release mouse");
        }
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

#[cfg(not(target_arch = "wasm32"))]
fn catalog(examples: &[Example]) -> String {
    examples
        .iter()
        .map(|example| {
            format!(
                "  {:20} {}\n    {}",
                example.id, example.title, example.description
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
