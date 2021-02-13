use hypertrace_viewer as viewer;

use std::{
    rc::Rc,
    thread::sleep,
    time::Duration,
};
use viewer::Window;

fn main() -> base::Result<()> {
    let context = Rc::new(sdl2::init()?);
    let mut window = Window::new(context, (800, 600), "Empty window")?;

    window.draw()?;
    sleep(Duration::from_millis(1000));

    Ok(())
}
