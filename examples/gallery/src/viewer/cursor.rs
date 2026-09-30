// Cursor capture is application policy. The guard also releases it on errors.
pub(super) struct MouseCapture<'a> {
    window: &'a wgame::app::RawWindow,
    pub(super) active: bool,
}

impl<'a> MouseCapture<'a> {
    pub(super) fn new(window: &'a wgame::app::RawWindow) -> Self {
        Self {
            window,
            active: false,
        }
    }

    pub(super) fn set(&mut self, active: bool) -> wgame::Result<()> {
        use winit::window::CursorGrabMode;
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
            eprintln!("Unable to release mouse: {error:#}");
        }
    }
}
