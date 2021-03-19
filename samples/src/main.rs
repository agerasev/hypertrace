use std::{
    rc::Rc,
    cell::RefCell,
    thread::sleep,
    time::Duration,
};
use viewer::{Window, Handler, DummyController};

fn main() -> base::Result<()> {
    let context = Rc::new(sdl2::init()?);
    let mut window = Window::new(context.clone(), (800, 600), "Press ESC to close")?;
    let mut handler = Handler::new(context, Rc::new(RefCell::new(DummyController)))?;

    loop {
        if handler.poll()? {
            break Ok(());
        }
        window.draw()?;
        sleep(Duration::from_millis(40));
    }
}
