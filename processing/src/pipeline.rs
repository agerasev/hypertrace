use crate::{
    Canvas, Context, EntityBuffer, Extractor, FilterSequence, ImageBuffer, Packer, Render,
};
use base::Image;
use objects::Scene;
use types::Geometry;

pub struct Pipeline<G: Geometry, S: Scene<G>, F: FilterSequence> {
    context: Context,
    scene_buffer: EntityBuffer<S>,
    render: Render<G, S>,
    canvas: Canvas,
    extractor: Extractor,
    filter: F,
    packer: Packer,
    image_buf_src: ImageBuffer<f32, 4>,
    image_buf_dst: ImageBuffer<f32, 4>,
    image_buf_packed: ImageBuffer<u8, 4>,
    image: Image<u8, 4>,
}

impl<G: Geometry, S: Scene<G>, F: FilterSequence> Pipeline<G, S, F> {
    pub fn new(
        context: &Context,
        canvas_size: (usize, usize),
        scene: &S,
        filter: F,
    ) -> crate::Result<Self> {
        Ok(Self {
            context: context.clone(),
            scene_buffer: EntityBuffer::new(context, scene)?,
            render: Render::new(context)?,
            extractor: Extractor::new(&context.backend)?,
            packer: Packer::new(&context.backend)?,
            filter,
            canvas: Canvas::new(&context.backend, canvas_size)?,
            image_buf_src: ImageBuffer::new(&context.backend, canvas_size)?,
            image_buf_dst: ImageBuffer::new(&context.backend, canvas_size)?,
            image_buf_packed: ImageBuffer::new(&context.backend, canvas_size)?,
            image: Image::new(canvas_size),
        })
    }

    pub fn set_scene(&mut self, scene: &S) -> crate::Result<()> {
        self.scene_buffer = EntityBuffer::new(&self.context, scene)?;
        Ok(())
    }

    pub fn render(&mut self) -> crate::Result<()> {
        self.render.render(&self.scene_buffer, &mut self.canvas)
    }

    pub fn get_image(&mut self) -> crate::Result<&Image<u8, 4>> {
        self.extractor
            .process(&self.canvas, &mut self.image_buf_src)?;
        self.filter
            .process_sequence(&mut self.image_buf_src, &mut self.image_buf_dst)?;
        self.packer
            .process(&self.image_buf_src, &mut self.image_buf_packed)?;
        self.image_buf_packed.load(&mut self.image)?;
        Ok(&self.image)
    }

    pub fn clean(&mut self) -> crate::Result<()> {
        self.canvas.clean()
    }
}
