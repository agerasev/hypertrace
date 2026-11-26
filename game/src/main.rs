//! First-person navigation through a world of hyperbolic triangle meshes.
use anyhow::{Result, bail, ensure};
use wgame::{
    Library, Window, WindowConfig,
    app::time::Instant,
    canvas::{Button, CanvasInput, Event, Key},
    gfx::{Camera, Target, types::color},
};
use winit::window::CursorGrabMode;

#[cfg(test)]
mod gpu_tests;
mod projection;
mod walker;
mod world;

use walker::Walker;

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

fn motion(input: &CanvasInput, size: (u32, u32), look: bool) -> ([f64; 2], [f64; 2]) {
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
            -2.0 * f64::from(input.relative_motion.x) / f64::from(size.0),
            -2.0 * f64::from(input.relative_motion.y) / f64::from(size.1),
        ]
    } else {
        [0.0; 2]
    };
    (axes, angles)
}

async fn run(mut window: Window<'_>, smoke: bool) -> Result<()> {
    let graphics = window.graphics().clone();
    eprintln!("Adapter: {:?}", graphics.adapter().get_info());
    let library = Library::new(&graphics);
    let world = world::world(&library)?;
    let shader = projection::shader(&library, &world.sun)?;
    let start = Walker::new(world::spawn_frame()?, 0.14)?;
    let mut walker = start;
    let mut drawing = projection::prepare(&world.objects, walker.eye_pose()?, &shader)?.bake();
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
    let mut smoke_jumped = false;

    while let Some(mut frame) = window.next_frame().await? {
        let now = Instant::now();
        let dt = if smoke {
            0.05
        } else {
            (now - previous).as_secs_f64().min(0.1)
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
        let ([mut strafe, mut forward], [mut yaw, mut pitch]) = motion(input, frame_size, look);
        jump &= input.focused;
        if smoke && smoke_frames == 2 {
            strafe = 1.0;
            forward = 1.0;
            yaw = 0.12;
            pitch = -0.15;
            jump = true;
        }
        reset |= smoke && smoke_frames == 15;
        let mut next = walker;
        let update = (|| -> Result<_> {
            if reset {
                next = start;
            } else if !next.step(strafe, forward, yaw, pitch, jump, dt)? {
                return Ok(None);
            }
            Ok(Some(
                projection::prepare(&world.objects, next.eye_pose()?, &shader)?.bake(),
            ))
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
            INITIAL_FOV
        } else {
            (fov * (-0.002 * scroll).exp()).clamp(0.08, 4.0)
        };
        if frame_size != size || next_fov != fov {
            size = frame_size;
            fov = next_fov;
            camera = Camera::new(&graphics, projection::camera_matrix(size, fov));
        }
        frame.clear(color::BLACK);
        frame.render(&camera, &drawing);
        frame.present();

        if smoke {
            smoke_frames += 1;
            smoke_jumped |= walker.is_airborne();
            if smoke_frames == 15 {
                ensure!(
                    smoke_jumped && !walker.is_airborne(),
                    "jump did not complete before reset"
                );
            }
            if smoke_frames == 3 {
                let _ = raw.request_inner_size(wgame::app::Size::new(480, 270));
            }
            smoke_resized |= size == (480, 270);
            if smoke_frames == 16 {
                ensure!(smoke_resized, "window resize was not observed");
                eprintln!(
                    "Rendered 16 frames with walking, looking, jumping, landing, reset and resize."
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
    for argument in std::env::args().skip(1) {
        match argument.as_str() {
            "--smoke" => smoke = true,
            "--help" | "-h" => {
                println!(
                    "hypergame [--smoke]\nWASD/arrows to walk, Space to jump, left-drag to look.\nTab toggles mouse capture, scroll changes field of view, R resets, Esc closes."
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
