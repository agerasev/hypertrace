//! Native viewer. WASD/arrows move, Space/C move vertically, Q/E roll,
//! left-drag looks around, scroll changes field of view, R resets, Esc closes.
//! `--scene eu|hy --smoke` processes twelve frames, including a resize, camera
//! updates and one intentionally discarded frame.

use std::time::Instant;

use hypertrace_wgpu::{Presenter, Renderer, Scene};
use wgame::{
    Window, WindowConfig,
    canvas::{Button, Event, Key},
    gfx::Target,
};

mod support;

const SEED: u32 = 1;

#[wgame::app]
async fn main() -> wgame::Result<()> {
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
                return Ok(());
            }
            _ => anyhow::bail!("unknown option {arg}; use --help"),
        }
    }
    let initial_scene = support::scene(&scene)?;
    let config = WindowConfig::default()
        .title(&format!("Hypertrace · WGPU · {scene}"))
        .size(if smoke { (320, 240) } else { (960, 720) })
        .required_limits(wgpu::Limits::default());
    wgame::within_window(config, async move |window| {
        run(window, initial_scene, smoke).await
    })
    .await
}

async fn run(mut window: Window<'_>, initial_scene: Scene, smoke: bool) -> wgame::Result<()> {
    let graphics = window.graphics().clone();
    eprintln!("Adapter: {:?}", graphics.adapter().get_info());
    let raw = window.raw();
    let initial_size = raw.inner_size();
    let mut renderer = Renderer::new(
        graphics.device(),
        graphics.queue(),
        (initial_size.width.max(1), initial_size.height.max(1)),
        initial_scene.clone(),
        SEED,
    )?;
    let mut presenter = Presenter::new(graphics.device(), graphics.format(), &renderer);
    let mut camera = initial_scene.camera;
    let mut fov = initial_scene.fov;
    let mut previous = Instant::now();
    let mut frames = 0;
    let mut smoke_resized = false;
    while let Some(mut frame) = window.next_frame().await? {
        let now = Instant::now();
        // Integrate elapsed time, bounding a stalled/minimized window's first
        // movement update so resuming cannot cause a large camera jump.
        let dt = (now - previous).as_secs_f64().min(0.1);
        previous = now;
        let size = frame.size();
        if renderer.resize(size)? {
            presenter.rebind(graphics.device(), &renderer);
            if frames > 0 {
                smoke_resized = true;
            }
        }
        let input = frame.input();
        let mut reset = false;
        let mut zoom = 0.0;
        for event in &input.events {
            match event {
                Event::Key {
                    key: Key::Escape,
                    pressed: true,
                    ..
                } => {
                    frame.discard();
                    return Ok(());
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
            renderer.update_scene(initial_scene.clone())?;
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
        }
        renderer.encode(frame.encoder());
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
            let _ = raw.request_inner_size(wgame::app::Size::new(400, 300));
        }
        if smoke && frames == 12 {
            anyhow::ensure!(
                smoke_resized,
                "smoke test did not observe its requested window resize"
            );
            eprintln!(
                "Viewer smoke passed: {frames} frames (one discarded), resize, camera updates, GPU presentation"
            );
            break;
        }
    }
    Ok(())
}
