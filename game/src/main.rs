//! First-person navigation through a world of hyperbolic triangle meshes.
use std::path::{Path, PathBuf};

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

fn load_config(path: &Path) -> Result<(Config, world::Patch)> {
    let config = Config::load(path)?;
    let patch = world::Patch::new(&config.earth.grid, config.max_sites_per_tile())
        .with_context(|| format!("invalid earth.grid in {}", path.display()))?;
    Ok((config, patch))
}

struct Game {
    world: world::World,
    shader: projection::Shader,
    drawing: projection::Drawing,
    walker: Walker,
    view: View,
}

impl Game {
    fn new(
        library: &Library,
        config: &Config,
        patch: world::Patch,
        current: Option<Walker>,
    ) -> Result<Self> {
        let walker = match current {
            Some(current) => current.reconfigured(config.observer)?,
            None => Walker::new(config.observer)?,
        };
        ensure!(
            walker.peak_eye_height() < config.moon.distance,
            "moon.distance must exceed the observer's peak eye height during the current jump"
        );
        let mut world = world::world(library, config, patch, [])?;
        let shader =
            projection::shader(library, &world.suns, world.ambient, Some(world.atmosphere))?;
        let drawing = Self::drawing_at(&mut world, &shader, &walker)?;
        Ok(Self {
            world,
            shader,
            drawing,
            walker,
            view: config.view,
        })
    }

    fn drawing_at(
        world: &mut world::World,
        shader: &projection::Shader,
        walker: &Walker,
    ) -> Result<projection::Drawing> {
        let eye = walker.eye_pose()?;
        projection::prepare(world.objects_near(eye)?, eye, shader)
    }

    fn reload(&mut self, library: &Library, path: &Path) -> Result<()> {
        let (config, patch) = load_config(path)?;
        *self = Self::new(library, &config, patch, Some(self.walker))?;
        Ok(())
    }
}

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
    path: PathBuf,
    config: Config,
    patch: world::Patch,
    smoke: bool,
) -> Result<()> {
    let graphics = window.graphics().clone();
    eprintln!("Adapter: {:?}", graphics.adapter().get_info());
    let library = Library::new(&graphics);
    let mut game = Game::new(&library, &config, patch, None)?;
    let mut fov = View::tan_half_fov(game.view.fov_degrees);
    let raw = window.raw();
    let mut capture = MouseCapture {
        window: raw,
        active: false,
    };
    let initial_size = raw.inner_size();
    let mut size = (initial_size.width.max(1), initial_size.height.max(1));
    let mut camera = Camera::new(
        &graphics,
        projection::camera_matrix(size, fov, game.view.near_distance),
    );
    let mut previous = Instant::now();
    let mut blocked = false;
    let mut smoke_frames = 0;
    let mut smoke_resized = false;
    let mut smoke_jumped = false;

    while let Some(mut frame) = window.next_frame().await? {
        let now = Instant::now();
        let elapsed = (now - previous).as_secs_f64();
        previous = now;
        let frame_size = frame.size();
        if frame_size.0 == 0 || frame_size.1 == 0 {
            frame.discard();
            continue;
        }
        let input = frame.input();
        let mut suppress_look = false;
        let mut reload = false;
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
                } => reload = true,
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
        let old_view = game.view;
        if reload || (smoke && smoke_frames == 5) {
            let before = game.walker;
            match game.reload(&library, &path) {
                Ok(()) => {
                    if smoke {
                        ensure!(
                            before.eye_pose()?.components()?
                                == game.walker.eye_pose()?.components()?
                                && before.velocity() == game.walker.velocity(),
                            "reloading the smoke configuration changed the observer's pose or velocity"
                        );
                    }
                    eprintln!("Reloaded {}", path.display());
                    blocked = false;
                    if game.view.window_size != old_view.window_size {
                        let [w, h] = game.view.window_size;
                        let _ = raw.request_inner_size(wgame::app::Size::new(w, h));
                    }
                }
                Err(error) if smoke => return Err(error),
                Err(error) => eprintln!("Unable to reload {}: {error:#}", path.display()),
            }
            // Time spent reading the file and rebuilding the world is not a movement step.
            previous = Instant::now();
        }
        let view = game.view;
        let dt = if smoke {
            // Exercise a whole ballistic flight in the fixed smoke frame budget.
            if smoke_frames >= 2 {
                config.observer.flight_time() / 8.0
            } else {
                0.05
            }
        } else {
            elapsed.min(view.max_frame_seconds)
        };
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
        let mut next = game.walker;
        let update = (|| -> Result<_> {
            if !next.step(strafe, forward, yaw, pitch, jump, dt)? {
                return Ok(None);
            }
            Ok(Some(Game::drawing_at(
                &mut game.world,
                &game.shader,
                &next,
            )?))
        })();
        match update {
            Ok(drawing_update) => {
                game.walker = next;
                if let Some(scene) = drawing_update {
                    game.drawing = scene;
                    blocked = false;
                }
            }
            Err(error) if smoke => return Err(error),
            Err(error) => {
                if !blocked {
                    eprintln!("Camera movement stopped: {error:#}. Try moving back.");
                }
                blocked = true;
            }
        }
        let next_fov = if view.fov_degrees != old_view.fov_degrees {
            View::tan_half_fov(view.fov_degrees)
        } else {
            (fov * (-view.zoom_sensitivity * scroll).exp()).clamp(
                View::tan_half_fov(view.min_fov_degrees),
                View::tan_half_fov(view.max_fov_degrees),
            )
        };
        if frame_size != size || next_fov != fov || view.near_distance != old_view.near_distance {
            size = frame_size;
            fov = next_fov;
            camera = Camera::new(
                &graphics,
                projection::camera_matrix(size, fov, view.near_distance),
            );
        }
        frame.clear(color::BLACK);
        game.drawing.draw(&mut frame, &camera);
        frame.present();

        if smoke {
            smoke_frames += 1;
            smoke_jumped |= game.walker.is_airborne();
            if smoke_frames == 3 {
                let _ = raw.request_inner_size(wgame::app::Size::new(480, 270));
            }
            smoke_resized |= size == (480, 270);
            if smoke_frames == 16 {
                ensure!(
                    smoke_jumped == (config.observer.jump_height > 0.0)
                        && !game.walker.is_airborne(),
                    "unexpected jump state at the end of the smoke check"
                );
                ensure!(smoke_resized, "window resize was not observed");
                eprintln!(
                    "Rendered 16 frames with walking, looking, jump input, config reload and resize."
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
                    "hypergame --config PATH [--smoke]\nWASD/arrows to walk, Space to jump, left-drag to look.\nTab toggles mouse capture, scroll changes field of view, R reloads the config, Esc closes."
                );
                return Ok(());
            }
            _ => bail!("unknown option {argument:?}; use --help"),
        }
    }
    let path = path.context("missing required --config PATH; use --help")?;
    let (config, patch) = load_config(&path)?;
    let window_config = WindowConfig::default().title("Hypergame").size(if smoke {
        (320, 240)
    } else {
        config.view.window_size.into()
    });
    wgame::app::entry(async move || {
        wgame::within_window(window_config, async move |window| {
            run(window, path, config, patch, smoke).await
        })
        .await
    });
    Ok(())
}
