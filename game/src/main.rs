//! First-person navigation through a world of hyperbolic triangle meshes.
use anyhow::{Result, bail, ensure};
use wgame::{
    Library, Window, WindowConfig,
    app::time::Instant,
    canvas::{Button, CanvasInput, Event, Key},
    gfx::{Camera, Target},
    rgb::Rgb,
};
use winit::window::CursorGrabMode;

#[cfg(test)]
mod gpu_tests;
mod projection;
mod world;

const RADIUS: f64 = 1.0;
const INITIAL_FOV: f64 = 0.66; // Tangent of half the vertical field of view.

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

fn motion(input: &CanvasInput, size: (u32, u32), dt: f64, look: bool) -> ([f64; 3], [f64; 3]) {
    let held = |c| input.key_down(Key::Character(c));
    let axis = |positive, negative| (f64::from(positive) - f64::from(negative)) * dt;
    let translation = [
        axis(
            held('d') || input.key_down(Key::ArrowRight),
            held('a') || input.key_down(Key::ArrowLeft),
        ),
        axis(input.key_down(Key::Space), held('c')),
        axis(
            held('s') || input.key_down(Key::ArrowDown),
            held('w') || input.key_down(Key::ArrowUp),
        ),
    ];
    let mut rotation = [0.0, 0.0, axis(held('q'), held('e'))];
    if look && input.focused {
        rotation[0] = -2.0 * f64::from(input.relative_motion.y) / f64::from(size.1);
        rotation[1] = -2.0 * f64::from(input.relative_motion.x) / f64::from(size.0);
    }
    (translation, rotation)
}

async fn run(mut window: Window<'_>, smoke: bool) -> Result<()> {
    let graphics = window.graphics().clone();
    eprintln!("Adapter: {:?}", graphics.adapter().get_info());
    let library = Library::new(&graphics);
    let objects = world::objects(&library)?;
    let shader = projection::shader(&library)?;
    let mut pose = projection::Pose::identity();
    let mut drawing = projection::prepare(&objects, pose, &shader)?.bake();
    let mut fov = INITIAL_FOV;
    let raw = window.raw();
    let mut capture = MouseCapture {
        window: raw,
        active: false,
    };
    let initial_size = raw.inner_size();
    let mut size = (initial_size.width.max(1), initial_size.height.max(1));
    let mut camera = Camera::new(&graphics, projection::camera_matrix(size, fov));
    let mut previous = Instant::now();
    let mut blocked = false;
    let mut smoke_frames = 0;
    let mut smoke_resized = false;

    while let Some(mut frame) = window.next_frame().await? {
        let now = Instant::now();
        let dt = (now - previous).as_secs_f64().min(0.1);
        previous = now;
        let frame_size = frame.size();
        if frame_size.0 == 0 || frame_size.1 == 0 {
            frame.discard();
            continue;
        }
        let input = frame.input();
        let mut suppress_look = false;
        let mut reset = false;
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
                }
                Event::Cancelled => suppress_look = true,
                Event::Scroll(delta) => scroll += f64::from(delta.y),
                _ => {}
            }
        }
        let look = !suppress_look && (capture.active || input.button_down(Button::Primary));
        let (mut translation, mut rotation) = motion(input, frame_size, dt, look);
        if smoke && smoke_frames == 2 {
            translation[0] += 0.15;
            rotation[1] += 0.12;
        }
        reset |= smoke && smoke_frames == 5;
        if reset || translation != [0.0; 3] || rotation != [0.0; 3] {
            let next = if reset {
                Ok(projection::Pose::identity())
            } else {
                pose.move_local(translation, rotation, RADIUS)
            };
            match next.and_then(|next| Ok((next, projection::prepare(&objects, next, &shader)?))) {
                Ok((next, scene)) => {
                    pose = next;
                    drawing = scene.bake();
                    blocked = false;
                }
                Err(error) if smoke => return Err(error),
                Err(error) => {
                    if !blocked {
                        eprintln!(
                            "Camera movement stopped: {error:#}. Move back or press R to reset."
                        );
                    }
                    blocked = true;
                }
            }
        }
        let next_fov = if reset {
            INITIAL_FOV
        } else {
            (fov * (-0.002 * scroll).exp()).clamp(0.08, 4.0)
        };
        if frame_size != size || next_fov != fov {
            size = frame_size;
            fov = next_fov;
            camera = Camera::new(&graphics, projection::camera_matrix(size, fov));
        }
        frame.clear(Rgb::new(0.015_f32, 0.024, 0.041));
        frame.render(&camera, &drawing);
        frame.present();

        if smoke {
            smoke_frames += 1;
            if smoke_frames == 3 {
                let _ = raw.request_inner_size(wgame::app::Size::new(480, 270));
            }
            smoke_resized |= size == (480, 270);
            if smoke_frames == 16 {
                ensure!(smoke_resized, "window resize was not observed");
                eprintln!("Rendered 16 frames with camera movement, reset and resize.");
                return Ok(());
            }
        }
    }
    ensure!(!smoke, "window closed after {smoke_frames} smoke frames");
    Ok(())
}

fn main() -> Result<()> {
    let mut smoke = false;
    for argument in std::env::args().skip(1) {
        match argument.as_str() {
            "--smoke" => smoke = true,
            "--help" | "-h" => {
                println!(
                    "hypergame [--smoke]\nWASD/arrows to move, Space/C for up/down, Q/E to roll, left-drag to look.\nTab toggles mouse capture, scroll changes field of view, R resets, Esc closes."
                );
                return Ok(());
            }
            _ => bail!("unknown option {argument}; use --help"),
        }
    }
    let config = WindowConfig::default().title("Hypergame").size(if smoke {
        (320, 240)
    } else {
        (960, 720)
    });
    wgame::app::entry(async move || {
        wgame::within_window(config, async move |window| run(window, smoke).await).await
    });
    Ok(())
}
