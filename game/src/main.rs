//! First-person navigation through a world of hyperbolic triangle meshes.
use std::path::PathBuf;

use anyhow::{Context, Result, bail, ensure};
use wgame::{
    Library, Window, WindowConfig,
    app::time::Instant,
    canvas::{Button, CanvasInput, Event, Key},
    gfx::{Camera, Target, types::color},
};
use winit::window::CursorGrabMode;

mod atmosphere;
mod config;
#[cfg(test)]
mod gpu_tests;
mod projection;
mod walker;
mod world;

use config::{Config, observer::View};
use walker::Walker;

struct MouseCapture<'a> {
    window: &'a wgame::app::RawWindow,
    active: bool,
}

impl MouseCapture<'_> {
    fn set(&mut self, active: bool) -> Result<()> {
        if active == self.active {
            return Ok(());
        }
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
            eprintln!("Unable to release mouse capture: {error}");
        }
    }
}

fn motion(
    input: &CanvasInput,
    size: (u32, u32),
    look: bool,
    sensitivity: f64,
) -> ([f64; 2], [f64; 2]) {
    if !input.focused {
        return ([0.0; 2], [0.0; 2]);
    }
    let held = |c| input.key_down(Key::Character(c));
    let axis = |positive, negative| f64::from(positive) - f64::from(negative);
    let axes = [
        axis(
            held('d') || input.key_down(Key::ArrowRight),
            held('a') || input.key_down(Key::ArrowLeft),
        ),
        axis(
            held('w') || input.key_down(Key::ArrowUp),
            held('s') || input.key_down(Key::ArrowDown),
        ),
    ];
    let angles = if look {
        [
            -sensitivity * f64::from(input.relative_motion.x) / f64::from(size.0),
            -sensitivity * f64::from(input.relative_motion.y) / f64::from(size.1),
        ]
    } else {
        [0.0; 2]
    };
    (axes, angles)
}

async fn run(
    mut window: Window<'_>,
    config: Config,
    patch: world::Patch,
    smoke: bool,
) -> Result<()> {
    let graphics = window.graphics().clone();
    eprintln!("Adapter: {:?}", graphics.adapter().get_info());
    let library = Library::new(&graphics);
    let mut world = world::world(&library, &config, patch, [])?;
    let shader = projection::shader(&library, &world.suns, world.ambient, Some(world.atmosphere))?;
    let start = Walker::new(config.observer)?;
    let mut walker = start;
    let mut prepare = |walker: &Walker| {
        let eye = walker.eye_pose()?;
        projection::prepare(world.objects_near(eye)?, eye, &shader)
    };
    let mut drawing = prepare(&walker)?;
    let view = config.view;
    let initial_fov = View::tan_half_fov(view.fov_degrees);
    let min_fov = View::tan_half_fov(view.min_fov_degrees);
    let max_fov = View::tan_half_fov(view.max_fov_degrees);
    let mut fov = initial_fov;
    let raw = window.raw();
    let mut capture = MouseCapture {
        window: raw,
        active: false,
    };
    let initial_size = raw.inner_size();
    let mut size = (initial_size.width.max(1), initial_size.height.max(1));
    let mut camera = Camera::new(
        &graphics,
        projection::camera_matrix(size, fov, view.near_distance),
    );
    let mut previous = Instant::now();
    let mut blocked = false;
    let mut smoke_frames = 0;
    let mut smoke_resized = false;
    let mut smoke_jumped = false;

    while let Some(mut frame) = window.next_frame().await? {
        let now = Instant::now();
        let dt = if smoke {
            // Exercise a whole ballistic flight in the fixed smoke frame budget.
            if smoke_frames >= 2 {
                config.observer.flight_time() / 8.0
            } else {
                0.05
            }
        } else {
            (now - previous).as_secs_f64().min(view.max_frame_seconds)
        };
        previous = now;
        let frame_size = frame.size();
        if frame_size.0 == 0 || frame_size.1 == 0 {
            frame.discard();
            continue;
        }
        let input = frame.input();
        let mut suppress_look = false;
        let mut reset = false;
        let mut jump = false;
        let mut scroll = 0.0;
        for event in &input.events {
            match event {
                Event::Key {
                    key: Key::Escape,
                    pressed: true,
                    repeat: false,
                } => {
                    frame.discard();
                    ensure!(!smoke, "smoke test interrupted after {smoke_frames} frames");
                    return Ok(());
                }
                Event::Key {
                    key: Key::Character('r'),
                    pressed: true,
                    repeat: false,
                } => reset = true,
                Event::Key {
                    key: Key::Space,
                    pressed: true,
                    repeat: false,
                } => jump = true,
                Event::Key {
                    key: Key::Tab,
                    pressed: true,
                    repeat: false,
                } => {
                    if let Err(error) = capture.set(!capture.active) {
                        eprintln!("Unable to toggle mouse capture: {error}");
                    }
                    suppress_look = true;
                }
                Event::Focused(false) => {
                    capture.set(false)?;
                    suppress_look = true;
                    jump = false;
                }
                Event::Cancelled => {
                    suppress_look = true;
                    jump = false;
                }
                Event::Scroll(delta) => scroll += f64::from(delta.y),
                _ => {}
            }
        }
        let look = !suppress_look && (capture.active || input.button_down(Button::Primary));
        let ([mut strafe, mut forward], [mut yaw, mut pitch]) =
            motion(input, frame_size, look, view.look_sensitivity);
        jump &= input.focused;
        if smoke && smoke_frames == 1 {
            strafe = 1.0;
            forward = 1.0;
            yaw = 0.12;
            pitch = -0.15;
        }
        jump |= smoke && smoke_frames == 2;
        reset |= smoke && smoke_frames == 15;
        let mut next = walker;
        let update = (|| -> Result<_> {
            if reset {
                next = start;
            } else if !next.step(strafe, forward, yaw, pitch, jump, dt)? {
                return Ok(None);
            }
            Ok(Some(prepare(&next)?))
        })();
        match update {
            Ok(drawing_update) => {
                walker = next;
                if let Some(scene) = drawing_update {
                    drawing = scene;
                    blocked = false;
                }
            }
            Err(error) if smoke => return Err(error),
            Err(error) => {
                if !blocked {
                    eprintln!("Camera movement stopped: {error:#}. Move back or press R to reset.");
                }
                blocked = true;
            }
        }
        let next_fov = if reset {
            initial_fov
        } else {
            (fov * (-view.zoom_sensitivity * scroll).exp()).clamp(min_fov, max_fov)
        };
        if frame_size != size || next_fov != fov {
            size = frame_size;
            fov = next_fov;
            camera = Camera::new(
                &graphics,
                projection::camera_matrix(size, fov, view.near_distance),
            );
        }
        frame.clear(color::BLACK);
        drawing.draw(&mut frame, &camera);
        frame.present();

        if smoke {
            smoke_frames += 1;
            smoke_jumped |= walker.is_airborne();
            if smoke_frames == 15 {
                ensure!(
                    smoke_jumped == (config.observer.jump_height > 0.0) && !walker.is_airborne(),
                    "unexpected jump state before reset"
                );
            }
            if smoke_frames == 3 {
                let _ = raw.request_inner_size(wgame::app::Size::new(480, 270));
            }
            smoke_resized |= size == (480, 270);
            if smoke_frames == 16 {
                ensure!(smoke_resized, "window resize was not observed");
                eprintln!(
                    "Rendered 16 frames with walking, looking, jump input, reset and resize."
                );
                return Ok(());
            }
        }
    }
    ensure!(!smoke, "window closed after {smoke_frames} smoke frames");
    Ok(())
}

fn main() -> Result<()> {
    let mut smoke = false;
    let mut path = None;
    let mut args = std::env::args_os().skip(1);
    while let Some(argument) = args.next() {
        match argument.to_str() {
            Some("--config") => {
                ensure!(path.is_none(), "--config was provided more than once");
                path = Some(PathBuf::from(
                    args.next().context("--config requires a TOML file path")?,
                ));
            }
            Some("--smoke") => smoke = true,
            Some("--help" | "-h") => {
                println!(
                    "hypergame --config PATH [--smoke]\nWASD/arrows to walk, Space to jump, left-drag to look.\nTab toggles mouse capture, scroll changes field of view, R resets, Esc closes."
                );
                return Ok(());
            }
            _ => bail!("unknown option {argument:?}; use --help"),
        }
    }
    let path = path.context("missing required --config PATH; use --help")?;
    let config = Config::load(&path)?;
    let patch = world::Patch::new(&config.earth.grid, config.max_sites_per_tile())
        .with_context(|| format!("invalid earth.grid in {}", path.display()))?;
    let window_config = WindowConfig::default().title("Hypergame").size(if smoke {
        (320, 240)
    } else {
        config.view.window_size.into()
    });
    wgame::app::entry(async move || {
        wgame::within_window(window_config, async move |window| {
            run(window, config, patch, smoke).await
        })
        .await
    });
    Ok(())
}
