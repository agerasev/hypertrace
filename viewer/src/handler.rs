use std::{rc::Rc, cell::RefCell};
use sdl2::{
    self,
    Sdl, EventPump,
    render::{WindowCanvas, TextureAccess},
    pixels::PixelFormatEnum,
    event::Event,
    keyboard::{Keycode, Scancode},
    mouse::{MouseState, RelativeMouseState},
};
use hy_base as base;

use crate::Controller;

pub struct Handler<C: Controller> {
    context: Rc<Sdl>,
    event_pump: Option<EventPump>,
    controller: Rc<RefCell<C>>,

    pub lock: bool,
    pub capture: bool,
    pub drop_mouse: bool,
}

impl<C: Controller> Handler<C> {
    pub fn new(context: Rc<Sdl>, controller: Rc<RefCell<C>>) -> base::Result<Self> {
        context.mouse().set_relative_mouse_mode(true);

        let event_pump = Some(context.event_pump()?);

        let mut self_ = Self {
            context,
            controller,
            event_pump,

            lock: false,
            capture: false,
            drop_mouse: false,
        };

        self_.set_capture_mode(false);
        self_.unlock();

        Ok(self_)
    }

    pub fn set_capture_mode(&mut self, state: bool) {
        self.capture = state;
        self.context.mouse().set_relative_mouse_mode(state);
    }

    pub fn lock(&mut self) {
        self.lock = true;
        self.context.mouse().set_relative_mouse_mode(false);
        //self.canvas.window_mut().set_title("Clay [LOCKED]").unwrap();
    }

    pub fn unlock(&mut self) {
        self.lock = false;
        self.context.mouse().set_relative_mouse_mode(self.capture);
        //self.canvas.window_mut().set_title("Clay").unwrap();
    }

    pub fn locked(&self) -> bool {
        self.lock
    }

    #[allow(clippy::unnecessary_wraps)]
    fn poll_pump(&mut self, event_pump: &mut EventPump) -> base::Result<bool> {
        'event_loop: loop {
            let event = match event_pump.poll_event() {
                Some(evt) => evt,
                None => break 'event_loop,
            };
            /*
            let kbs = event_pump.keyboard_state();
            let shift =
                kbs.is_scancode_pressed(Scancode::LShift) ||
                kbs.is_scancode_pressed(Scancode::RShift);
            */

            match event {
                Event::Quit {..} => { return Ok(true); },
                Event::KeyDown { keycode: Some(key), .. } => match key {
                    Keycode::Escape => { return Ok(true); },
                    /*
                    Keycode::Tab => {
                        if !self.locked() {
                            self.set_capture_mode(!self.state.capture);
                            if self.state.capture {
                                self.state.drop_mouse = true;
                            }
                        }
                    },
                    Keycode::P => {
                        self.state.screenshot = Some(shift);
                    },
                    Keycode::L => {
                        if !shift {
                            self.lock();
                        } else {
                            self.unlock();
                            if self.state.capture {
                                self.state.drop_mouse = true;
                            }
                        }
                    },
                    */
                    _ => (),
                },
                _ => (),
            }
        }
        /*
        if !self.locked() {
            if !self.state.drop_mouse {
                handler.handle_mouse(
                    &self.state,
                    &event_pump.mouse_state(),
                    &event_pump.relative_mouse_state(),
                )?;
            } else {
                event_pump.relative_mouse_state();
                self.state.drop_mouse = false;
            }
        }
        */
        Ok(false)
    }

    pub fn poll(&mut self) -> base::Result<bool> {
        let mut event_pump = self.event_pump.take().unwrap();
        let res = self.poll_pump(&mut event_pump);
        assert!(self.event_pump.replace(event_pump).is_none());
        res
    }
}
