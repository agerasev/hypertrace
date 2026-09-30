//! Euclidean light in fog: a complete application built directly from workspace crates.
use hypertrace_renderer::{Presenter, Renderer, Scene, fit_render_size};
use objects::Scene as _;
use wgame::{
    WindowConfig,
    app::time::Instant,
    canvas::{Button, Event, Key},
    gfx::Target,
};

mod scene;

// Cursor capture is application policy. The guard also releases it on errors.
struct MouseCapture<'a> {
    window: &'a wgame::app::RawWindow,
    active: bool,
}

impl<'a> MouseCapture<'a> {
    fn new(window: &'a wgame::app::RawWindow) -> Self {
        Self {
            window,
            active: false,
        }
    }

    fn set(&mut self, active: bool) -> wgame::Result<()> {
        use winit::window::CursorGrabMode;
        if active {
            self.window
                .set_cursor_grab(CursorGrabMode::Locked)
                .or_else(|_| self.window.set_cursor_grab(CursorGrabMode::Confined))?;
        } else {
            self.window.set_cursor_grab(CursorGrabMode::None)?;
        }
        self.window.set_cursor_visible(!active);
        self.active = active;
        Ok(())
    }
}

impl Drop for MouseCapture<'_> {
    fn drop(&mut self) {
        if let Err(error) = self.set(false) {
            eprintln!("Unable to release mouse: {error:#}");
        }
    }
}

fn main() -> wgame::Result<()> {
    let mut smoke = false;
    for argument in std::env::args().skip(1) {
        match argument.as_str() {
            "--smoke" => smoke = true,
            "--help" | "-h" => {
                println!(
                    "fog [--smoke]\nWASD/arrows: move; Space/C: up/down; Q/E: roll; left-drag: look; Tab: toggle mouse lock; scroll: zoom; R: reset; Esc: close"
                );
                return Ok(());
            }
            _ => anyhow::bail!("unknown option {argument}; use --help"),
        }
    }

    // Scene construction and CPU validation do not require a window or GPU.
    let source = scene::scene::<12>();
    let scene = Scene::from_definition(&source.definition()?)?;
    let config = WindowConfig::default()
        .title("Hypertrace · Euclidean light in fog")
        .size(if smoke { (320, 240) } else { (960, 720) })
        .required_limits(wgpu::Limits {
            max_storage_buffer_binding_size: if smoke {
                2 << 20
            } else {
                wgpu::Limits::default().max_storage_buffer_binding_size
            },
            ..Default::default()
        })
        .use_adapter_buffer_limits(!smoke);

    wgame::app::entry(async move || {
        wgame::within_window(config, async move |mut window| {
            let graphics = window.graphics().clone();
            eprintln!("Adapter: {:?}", graphics.adapter().get_info());
            let limits = graphics.device().limits();
            let raw = window.raw();
            let mut capture = MouseCapture::new(raw);
            let size = raw.inner_size();
            let size = fit_render_size(&limits, (size.width.max(1), size.height.max(1)))?;
            let (initial_camera, initial_fov) = (scene.camera, scene.fov);
            let radius = f64::from(scene.radius);
            let mut renderer = Renderer::new_async(
                graphics.device(), graphics.queue(), size, scene, 1,
            ).await?;
            let mut presenter = Presenter::new(graphics.device(), graphics.format(), &renderer);
            let (mut camera, mut fov) = (initial_camera, initial_fov);
            let mut previous = Instant::now();
            let mut frames = 0;
            let (mut smoke_scaled, mut smoke_restored) = (false, false);
            let mut motion_blocked = false;

            while let Some(mut frame) = window.next_frame().await? {
                let now = Instant::now();
                let dt = (now - previous).as_secs_f64().min(0.1);
                previous = now;
                let size = frame.size();
                if size.0 == 0 || size.1 == 0 {
                    frame.discard();
                    continue;
                }
                let render_size = fit_render_size(&limits, size)?;
                if smoke {
                    smoke_scaled |= render_size != size;
                    smoke_restored |= smoke_scaled && render_size == size;
                }
                if renderer.resize(render_size)? {
                    presenter.rebind(graphics.device(), &renderer);
                }

                let input = frame.input();
                let mut reset = false;
                let mut zoom = 0.0;
                let mut suppress_look = false;
                for event in &input.events {
                    match event {
                        Event::Key {
                            key: Key::Escape, pressed: true, repeat: false,
                        } => {
                            frame.discard();
                            return Ok(());
                        }
                        Event::Key {
                            key: Key::Character('r'), pressed: true, repeat: false,
                        } => reset = true,
                        Event::Key {
                            key: Key::Tab, pressed: true, repeat: false,
                        } => {
                            if let Err(error) = capture.set(!capture.active) {
                                eprintln!("Unable to change mouse lock: {error:#}");
                            }
                            suppress_look = true;
                        }
                        Event::Focused(false) => {
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
                    renderer.update_camera(initial_camera, initial_fov)?;
                    (camera, fov) = (initial_camera, initial_fov);
                    motion_blocked = false;
                }
                let key = |letter| input.key_down(Key::Character(letter));
                let key_or = |letter, arrow| key(letter) || input.key_down(arrow);
                let axis = |positive: bool, negative: bool| f64::from(positive) - f64::from(negative);
                let mut translation = [
                    axis(key_or('d', Key::ArrowRight), key_or('a', Key::ArrowLeft)) * dt,
                    axis(input.key_down(Key::Space), key('c')) * dt,
                    axis(key_or('s', Key::ArrowDown), key_or('w', Key::ArrowUp)) * dt,
                ];
                let mut rotation = [0.0, 0.0, axis(key('q'), key('e')) * dt];
                if !suppress_look && input.focused
                    && (capture.active || input.button_down(Button::Primary)) {
                    rotation[0] -= 2.0 * f64::from(input.relative_motion.y) / f64::from(size.1);
                    rotation[1] -= 2.0 * f64::from(input.relative_motion.x) / f64::from(size.0);
                }
                if smoke && (frames == 5 || frames == 7) {
                    translation[0] += 0.02;
                    rotation[1] += 0.01;
                }
                let moving = translation.into_iter().chain(rotation).any(|value| value != 0.0);
                if moving || zoom != 0.0 {
                    let mut next_camera = camera;
                    let next_fov = (fov * (-zoom * 0.002).exp()).clamp(0.05, 10.0);
                    match next_camera.move_local(translation, rotation, radius)
                        .and_then(|()| renderer.update_camera(next_camera, next_fov)) {
                        Ok(()) => {
                            (camera, fov) = (next_camera, next_fov);
                            motion_blocked = false;
                        }
                        Err(error) => {
                            if !motion_blocked {
                                eprintln!("Camera movement stopped: {error:#}; move back or reset");
                            }
                            motion_blocked = true;
                        }
                    }
                }

                // Parameter writes precede encoding; presentation uses the same submission.
                renderer.encode(frame.encoder());
                let view = frame.view().clone();
                presenter.draw(frame.encoder(), &view);
                frame.present();
                frames += 1;
                if smoke && frames == 2 {
                    capture.set(true)?;
                    anyhow::ensure!(capture.active, "smoke test failed to lock mouse");
                }
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
                        smoke_scaled && smoke_restored,
                        "smoke test missed resize across the buffer limit and back"
                    );
                    eprintln!("Euclidean light in fog smoke passed: 12 frames, mouse lock/unlock, camera movement, resize and GPU presentation");
                    break;
                }
            }
            Ok(())
        }).await
    });
    Ok(())
}
