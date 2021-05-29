use crate::{BackendContext, OclContext};
use base::Image;
use ocl::OclPrm;
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub struct ImageBuffer<T: Copy + Default + OclPrm, const N: usize> {
    buffer: ocl::Buffer<T>,
    shape: (usize, usize),
}

pub struct Canvas {
    image: ImageBuffer<f32, 4>,
    seeds: ImageBuffer<u32, 1>,
    initial: Image<f32, 4>,
    passes: usize,
}

impl<T: Copy + Default + OclPrm, const N: usize> ImageBuffer<T, N> {
    pub fn new(context: &OclContext, (width, height): (usize, usize)) -> crate::Result<Self> {
        Ok(Self {
            buffer: ocl::Buffer::builder()
                .queue(context.queue.clone())
                .len(N * width * height)
                .build()?,
            shape: (width, height),
        })
    }

    pub fn from_image(context: &OclContext, image: &Image<T, N>) -> crate::Result<Self> {
        let shape = image.shape();
        Ok(Self {
            buffer: ocl::Buffer::builder()
                .queue(context.queue.clone())
                .len(N * shape.0 * shape.1)
                .copy_host_slice(image.data())
                .build()?,
            shape,
        })
    }

    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }
    pub fn width(&self) -> usize {
        self.shape.0
    }
    pub fn height(&self) -> usize {
        self.shape.1
    }

    pub fn raw(&self) -> &ocl::Buffer<T> {
        &self.buffer
    }
    pub fn raw_mut(&mut self) -> &mut ocl::Buffer<T> {
        &mut self.buffer
    }

    pub fn store(&mut self, image: &Image<T, N>) -> crate::Result<()> {
        assert_eq!(self.shape(), image.shape());
        self.raw_mut().write(image.data()).enq()?;
        self.raw().default_queue().unwrap().finish()?;
        Ok(())
    }
    pub fn load(&self, image: &mut Image<T, N>) -> crate::Result<()> {
        assert_eq!(self.shape(), image.shape());
        self.raw().read(image.data_mut()).enq()?;
        self.raw().default_queue().unwrap().finish()?;
        Ok(())
    }
}

impl Canvas {
    pub fn new(context: &BackendContext, shape: (usize, usize)) -> crate::Result<Self> {
        let mut seeds = Image::new(shape);
        SmallRng::seed_from_u64(0xdeadbeefdeadbeef).fill(seeds.data_mut());

        let initial = Image::new(shape);
        Ok(Self {
            image: ImageBuffer::from_image(context, &initial)?,
            seeds: ImageBuffer::from_image(context, &seeds)?,
            initial,
            passes: 0,
        })
    }

    pub fn shape(&self) -> (usize, usize) {
        self.image.shape()
    }
    pub fn width(&self) -> usize {
        self.image.width()
    }
    pub fn height(&self) -> usize {
        self.image.height()
    }

    pub fn passes(&self) -> usize {
        self.passes
    }
    pub fn add_pass(&mut self) {
        self.passes += 1;
    }
    pub fn clean(&mut self) -> crate::Result<()> {
        self.image.store(&self.initial)?;
        self.passes = 0;
        Ok(())
    }

    pub fn raw_image(&self) -> &ImageBuffer<f32, 4> {
        &self.image
    }

    pub fn seeds(&self) -> &ImageBuffer<u32, 1> {
        &self.seeds
    }
    pub fn seeds_mut(&mut self) -> &mut ImageBuffer<u32, 1> {
        &mut self.seeds
    }
}

pub struct Extractor {
    kernel: ocl::Kernel,
}

impl Extractor {
    pub fn new(context: &OclContext) -> crate::Result<Self> {
        let src = r#"
            __kernel void extract(
                uint width,
                uint passes,
                __global const float4 *canvas,
                __global float4 *image
            ) {
                uint idx = get_global_id(0) + width * get_global_id(1);
                image[idx] = canvas[idx] / (float)passes;
            }
        "#;
        let program = ocl::Program::builder()
            .source(src)
            .build(&context.context)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("extract")
            .arg_named("width", &0u32)
            .arg_named("passes", &0u32)
            .arg_named("canvas", None::<&ocl::Buffer<f32>>)
            .arg_named("image", None::<&ocl::Buffer<f32>>)
            .queue(context.queue.clone())
            .build()?;

        Ok(Self { kernel })
    }

    pub fn process(&self, canvas: &Canvas, image: &mut ImageBuffer<f32, 4>) -> crate::Result<()> {
        assert_eq!(canvas.shape(), image.shape());

        self.kernel.set_arg("width", image.width() as u32)?;
        self.kernel.set_arg("passes", canvas.passes() as u32)?;
        self.kernel.set_arg("canvas", canvas.raw_image().raw())?;
        self.kernel.set_arg("image", image.raw_mut())?;
        let cmd = self.kernel.cmd().global_work_size(image.shape());
        unsafe {
            cmd.enq()?;
        }

        self.kernel.default_queue().unwrap().flush()?;
        Ok(())
    }
}

pub struct Packer {
    kernel: ocl::Kernel,
}

impl Packer {
    pub fn new(context: &OclContext) -> crate::Result<Self> {
        let src = r#"
            __kernel void pack(
                uint width,
                __global const float4 *image,
                __global uchar4 *packed
            ) {
                uint idx = get_global_id(0) + width * get_global_id(1);
                float4 fcolor = image[idx];
                uchar4 icolor = convert_uchar4(0xFF * clamp(fcolor, 0.0f, 1.0f));
                packed[idx] = icolor;
            }
        "#;
        let program = ocl::Program::builder()
            .source(src)
            .build(&context.context)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("pack")
            .arg_named("width", &0u32)
            .arg_named("image", None::<&ocl::Buffer<f32>>)
            .arg_named("packed", None::<&ocl::Buffer<u8>>)
            .queue(context.queue.clone())
            .build()?;

        Ok(Self { kernel })
    }

    pub fn process(
        &self,
        image: &ImageBuffer<f32, 4>,
        packed: &mut ImageBuffer<u8, 4>,
    ) -> crate::Result<()> {
        assert_eq!(image.shape(), packed.shape());

        self.kernel.set_arg("width", image.width() as u32)?;
        self.kernel.set_arg("image", image.raw())?;
        self.kernel.set_arg("packed", packed.raw_mut())?;
        let cmd = self.kernel.cmd().global_work_size(image.shape());
        unsafe {
            cmd.enq()?;
        }

        self.kernel.default_queue().unwrap().flush()?;
        Ok(())
    }
}
