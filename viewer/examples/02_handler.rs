use hy_viewer as viewer;
use hy_base as base;

use std::{
    rc::Rc,
    cell::RefCell,
    thread::sleep,
    time::Duration,
};
use viewer::{Window, Handler, DummyController};

fn main() -> base::Result<()> {
    let shape = (800, 600);
    let context = Rc::new(sdl2::init()?);
    let mut window = Window::new(context.clone(), shape, "Press ESC to close")?;
    let mut handler = Handler::new(context, Rc::new(RefCell::new(DummyController)))?;
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

    loop {
        if handler.poll()? {
            break Ok(());
        }
        window.draw(&image)?;
        sleep(Duration::from_millis(40));
    }
}
