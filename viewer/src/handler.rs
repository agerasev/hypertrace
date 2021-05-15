use sdl2::{
    self,
    event::Event,
    keyboard::{KeyboardState, Keycode, Scancode},
    mouse::{MouseState, RelativeMouseState},
    EventPump, Sdl,
};
use std::rc::Rc;

use crate::{Controller, Motion};

pub struct Handler {
    context: Rc<Sdl>,
    event_pump: EventPump,
    size: (usize, usize),

    capture: bool,
    drop_move: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MouseMode {
    Capture,
    Drag,
}

pub enum PollStatus {
    Continue,
    Exit,
}

impl Handler {
    pub fn new(context: Rc<Sdl>, size: (usize, usize)) -> Result<Self, String> {
        let event_pump = context.event_pump()?;

        let mut self_ = Self {
            context,
            event_pump,
            size,

            capture: false,
            drop_move: false,
        };

        self_.set_mouse_mode(MouseMode::Drag);

        Ok(self_)
    }

    pub fn set_mouse_mode(&mut self, mode: MouseMode) {
        self.capture = mode == MouseMode::Capture;
        self.context.mouse().set_relative_mouse_mode(self.capture);
        if self.capture {
            self.drop_move = true;
        }
    }

    pub fn poll<C: Controller>(&mut self, controller: &mut C) -> Result<PollStatus, String> {
        let mut toggle_mouse_mode = false;
        let mut mouse_wheel = 0;

        'event_loop: loop {
            let event = match self.event_pump.poll_event() {
                Some(evt) => evt,
                None => break 'event_loop,
            };

            match event {
                Event::Quit { .. } => {
                    return Ok(PollStatus::Exit);
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => {
                        return Ok(PollStatus::Exit);
                    }
                    Keycode::Tab => {
                        toggle_mouse_mode = true;
                    }
                    _ => (),
                },
                Event::MouseWheel { y, .. } => {
                    mouse_wheel += y;
                }
                _ => (),
            }
        }

        // Keyboard

        self.handle_keys(controller, self.event_pump.keyboard_state());

        // Mouse

        if toggle_mouse_mode {
            self.set_mouse_mode(if self.capture {
                MouseMode::Drag
            } else {
                MouseMode::Capture
            });
        }

        let relative_mouse_state = self.event_pump.relative_mouse_state();
        if !self.drop_move {
            self.handle_mouse(
                controller,
                self.event_pump.keyboard_state(),
                self.event_pump.mouse_state(),
                relative_mouse_state,
                mouse_wheel,
            );
        } else {
            self.drop_move = false;
        }

        Ok(PollStatus::Continue)
    }

    fn handle_mouse<C: Controller>(
        &self,
        controller: &mut C,
        key_state: KeyboardState,
        state: MouseState,
        relative_state: RelativeMouseState,
        wheel: i32,
    ) {
        let shift = key_state.is_scancode_pressed(Scancode::LShift)
            || key_state.is_scancode_pressed(Scancode::RShift);

        if self.capture || state.left() {
            if relative_state.x() != 0 {
                controller.rotate_yaw(Motion::Position(
                    2.0 * relative_state.x() as f64 / self.size.0 as f64,
                ));
            }
            if relative_state.y() != 0 {
                controller.rotate_pitch(Motion::Position(
                    2.0 * relative_state.y() as f64 / self.size.1 as f64,
                ));
            }
        }

        if wheel != 0 {
            if !shift {
                controller.zoom(Motion::Position(wheel as f64));
            } else {
                controller.zoom_alt(Motion::Position(wheel as f64));
            }
        }
    }

    fn handle_keys<C: Controller>(&self, controller: &mut C, key_state: KeyboardState) {
        let key = |code: Scancode| key_state.is_scancode_pressed(code);

        if key(Scancode::W) || key(Scancode::Up) {
            controller.move_forward(Motion::Velocity(1.0));
        }
        if key(Scancode::A) || key(Scancode::Left) {
            controller.move_right(Motion::Velocity(-1.0));
        }
        if key(Scancode::S) || key(Scancode::Down) {
            controller.move_forward(Motion::Velocity(-1.0));
        }
        if key(Scancode::D) || key(Scancode::Right) {
            controller.move_right(Motion::Velocity(1.0));
        }
        if key(Scancode::Space) {
            controller.move_up(Motion::Velocity(1.0));
        }
        if key(Scancode::C) {
            controller.move_up(Motion::Velocity(-1.0));
        }
        if key(Scancode::E) {
            controller.rotate_roll(Motion::Velocity(1.0));
        }
        if key(Scancode::Q) {
            controller.rotate_roll(Motion::Velocity(-1.0));
        }
    }
}
