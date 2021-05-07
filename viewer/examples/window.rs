use std::{
    rc::Rc,
    thread::sleep,
    time::Duration,
};
use hypertrace_viewer::{Window, Controller};


pub struct DummyController;

impl Controller for DummyController {
    fn move_forward(&mut self, value: f64) {
        println!("move_forward: {}", value);
    }
    fn move_right(&mut self, value: f64) {
        println!("move_right: {}", value);
    }
    fn move_up(&mut self, value: f64) {
        println!("move_up: {}", value);
    }

    fn rotate_yaw(&mut self, value: f64) {
        println!("rotate_yaw: {}", value);
    }
    fn rotate_pitch(&mut self, value: f64) {
        println!("rotate_pitch: {}", value);
    }
    fn rotate_roll(&mut self, value: f64) {
        println!("rotate_roll: {}", value);
    }

    fn zoom(&mut self, value: f64) {
        println!("zoom: {}", value);
    }
    fn zoom_alt(&mut self, value: f64) {
        println!("zoom_alt: {}", value);
    }
}


fn main() -> base::Result<()> {
    let shape = (800, 600);
    let context = Rc::new(sdl2::init()?);
    let mut window = Window::new(context.clone(), shape, "Press ESC to close")?;
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

    let mut controller = DummyController;
    loop {
        if window.poll(&mut controller)? {
            break Ok(());
        }
        window.draw(&image)?;
        sleep(Duration::from_millis(40));
    }
}
