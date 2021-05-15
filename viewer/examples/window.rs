use hypertrace_viewer::{Controller, Motion, Window, PollStatus};
use std::{rc::Rc, thread::sleep, time::Duration};

pub struct DummyController;

impl Controller for DummyController {
    fn move_forward(&mut self, motion: Motion) {
        println!("move_forward: {:?}", motion);
    }
    fn move_right(&mut self, motion: Motion) {
        println!("move_right: {:?}", motion);
    }
    fn move_up(&mut self, motion: Motion) {
        println!("move_up: {:?}", motion);
    }

    fn rotate_yaw(&mut self, motion: Motion) {
        println!("rotate_yaw: {:?}", motion);
    }
    fn rotate_pitch(&mut self, motion: Motion) {
        println!("rotate_pitch: {:?}", motion);
    }
    fn rotate_roll(&mut self, motion: Motion) {
        println!("rotate_roll: {:?}", motion);
    }

    fn zoom(&mut self, motion: Motion) {
        println!("zoom: {:?}", motion);
    }
    fn zoom_alt(&mut self, motion: Motion) {
        println!("zoom_alt: {:?}", motion);
    }

    fn step(&mut self, _: f64) {}
}

fn main() -> Result<(), String> {
    let shape = (800, 600);
    let context = Rc::new(sdl2::init()?);
    let mut window = Window::new(context, shape, "Press ESC to close")?;
    let mut image = base::Image::new(shape);
    for iy in 0..shape.1 {
        for ix in 0..shape.0 {
            let (x, y) = (ix as f64 / shape.0 as f64, iy as f64 / shape.1 as f64);
            let color = &mut image.data_mut()[(4 * (ix + shape.0 * iy))..][..4];
            color[0] = (255.0 * (1.0 - 0.5 * (x + y))) as u8;
            color[1] = (255.0 * x) as u8;
            color[2] = (255.0 * y) as u8;
            color[3] = 255;
        }
    }

    let delay = 0.04;
    let mut controller = DummyController;
    loop {
        if let PollStatus::Exit = window.poll(&mut controller)? {
            break Ok(());
        }
        window.draw(&image)?;
        controller.step(delay);
        sleep(Duration::from_secs_f64(delay));
    }
}
