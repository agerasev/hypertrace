use std::{rc::Rc};
use sdl2::{
    self,
    Sdl,
    render::{WindowCanvas, TextureAccess},
    pixels::PixelFormatEnum,
};
use base::Image;
use crate::{Controller, handler::Handler};

rental! { mod rent {
    use sdl2::{
        video::{WindowContext},
        render::{TextureCreator, Texture},
    };

    #[rental_mut]
    pub struct RentTexture {
        creator: Box<TextureCreator<WindowContext>>,
        texture: Texture<'creator>,
    }
}}
use rent::{RentTexture};

pub struct Window {
    canvas: Canvas,
    handler: Handler,
}

pub struct Canvas {
    size: (usize, usize),
    canvas: WindowCanvas,
    texture: Option<RentTexture>,
}

impl Window {
    pub fn new(context: Rc<Sdl>, size: (usize, usize), title: &str) -> base::Result<Self> {
        let video = context.video()?;
     
        let window = video.window(title, size.0 as u32, size.1 as u32)
        .position_centered()/*.resizable()*/.build()
        .map_err(|e| e.to_string())?;
     
        let canvas = Canvas::new(
            window.into_canvas().build().map_err(|e| e.to_string())?,
            size,
        )?;

        let handler = Handler::new(context, size)?;

        Ok(Self {
            canvas,
            handler,
        })
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
        let texture_creator = canvas.texture_creator();
        let texture = Some(RentTexture::try_new_or_drop(
            Box::new(texture_creator),
            |tc| {
                tc.create_texture(
                    PixelFormatEnum::RGBA32,
                    TextureAccess::Streaming,
                    size.0 as u32,
                    size.1 as u32,
                ).map_err(|e| e.to_string())
            }
        )?);

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
        let mut texture = self.texture.take().unwrap();
        texture.rent(|tex| {
            let query = tex.query();
            assert_eq!((query.width as usize, query.height as usize), image.shape());
        });

        texture.rent_mut(|tex| tex.update(None, image.data(), 4 * image.width()).map_err(|e| e.to_string()))?;

        self.canvas.clear();
        texture.rent(|tex| self.canvas.copy(tex, None, None))?;

        self.canvas.present();

        assert!(self.texture.replace(texture).is_none());

        Ok(())
    }
}
