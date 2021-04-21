use hy_viewer as viewer;
use hy_base as base;

use std::{
    rc::Rc,
    thread::sleep,
    time::Duration,
};
use viewer::Window;

fn main() -> base::Result<()> {
    let shape = (800, 600);
    let context = Rc::new(sdl2::init()?);
    let mut window = Window::new(context, shape, "Empty window")?;

    window.draw(&base::Image::new(shape))?;
    sleep(Duration::from_millis(1000));

    Ok(())
}
