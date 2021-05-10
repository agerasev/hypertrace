use crate::{handler::Handler, Controller};
use base::Image;
use sdl2::{
    self,
    pixels::PixelFormatEnum,
    render::{Texture, TextureAccess, WindowCanvas},
    Sdl,
};
use std::rc::Rc;

pub struct Window {
    canvas: Canvas,
    handler: Handler,
}

pub struct Canvas {
    size: (usize, usize),
    canvas: WindowCanvas,
    texture: Texture,
}

impl Window {
    pub fn new(context: Rc<Sdl>, size: (usize, usize), title: &str) -> base::Result<Self> {
        let video = context.video()?;

        let window = video
            .window(title, size.0 as u32, size.1 as u32)
            .position_centered() /*.resizable()*/
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = Canvas::new(
            window.into_canvas().build().map_err(|e| e.to_string())?,
            size,
        )?;

        let handler = Handler::new(context, size)?;

        Ok(Self { canvas, handler })
    }

    pub fn size(&self) -> (usize, usize) {
        self.canvas.size()
    }

    pub fn draw(&mut self, image: &Image<u8, 4>) -> base::Result<()> {
        self.canvas.draw(image)
    }

    pub fn poll<C: Controller>(&mut self, controller: &mut C) -> base::Result<bool> {
        self.handler.poll(controller)
    }
}

impl Canvas {
    fn new(canvas: WindowCanvas, size: (usize, usize)) -> base::Result<Self> {
        let texture = canvas
            .texture_creator()
            .create_texture(
                PixelFormatEnum::RGBA32,
                TextureAccess::Streaming,
                size.0 as u32,
                size.1 as u32,
            )
            .map_err(|e| e.to_string())?;

        Ok(Self {
            size,
            canvas,
            texture,
        })
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn draw(&mut self, image: &Image<u8, 4>) -> base::Result<()> {
        let query = self.texture.query();
        assert_eq!((query.width as usize, query.height as usize), image.shape());

        self.texture
            .update(None, image.data(), 4 * image.width())
            .map_err(|e| e.to_string())?;

        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None)?;

        self.canvas.present();

        Ok(())
    }
}
