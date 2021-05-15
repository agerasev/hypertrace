use crate::{Buffer, Canvas, Context, Extractor, FilterSequence, ImageBuffer, Packer, Render};
use base::Image;
use objects::Scene;
use std::marker::PhantomData;
use types::Geometry;

pub struct Pipeline<G: Geometry, S: Scene<G>, F: FilterSequence> {
    context: Context,
    geometry: PhantomData<G>,
    scene_buffer: Buffer<S>,
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
        context: Context,
        canvas_size: (usize, usize),
        scene: &S,
        filter: F,
    ) -> base::Result<Self> {
        Ok(Self {
            geometry: PhantomData,
            scene_buffer: Buffer::new(&context, scene)?,
            render: Render::new(context.clone())?,
            extractor: Extractor::new(&context.context)?,
            packer: Packer::new(&context.context)?,
            filter,
            canvas: Canvas::new(&context.context, canvas_size)?,
            image_buf_src: ImageBuffer::new(&context.context, canvas_size)?,
            image_buf_dst: ImageBuffer::new(&context.context, canvas_size)?,
            image_buf_packed: ImageBuffer::new(&context.context, canvas_size)?,
            image: Image::new(canvas_size),
            context,
        })
    }

    pub fn set_scene(&mut self, scene: &S) -> base::Result<()> {
        self.scene_buffer = Buffer::new(&self.context, scene)?;
        Ok(())
    }

    pub fn render(&mut self) -> base::Result<()> {
        self.render.render(&self.scene_buffer, &mut self.canvas)
    }

    pub fn get_image(&mut self) -> base::Result<&Image<u8, 4>> {
        self.extractor
            .process(&self.context.queue, &self.canvas, &mut self.image_buf_src)?;
        self.filter.process_sequence(
            &self.context.queue,
            &mut self.image_buf_src,
            &mut self.image_buf_dst,
        )?;
        self.packer.process(
            &self.context.queue,
            &self.image_buf_src,
            &mut self.image_buf_packed,
        )?;
        self.image_buf_packed
            .load(&self.context.queue, &mut self.image)?;
        Ok(&self.image)
    }

    pub fn clean(&mut self) -> base::Result<()> {
        self.canvas.clean(&self.context.queue)
    }
}
