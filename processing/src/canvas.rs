use base::Image;
use ocl::OclPrm;
use rand::{Rng, SeedableRng, rngs::{SmallRng}};

pub struct ImageBuffer<T: Copy + Default + OclPrm, const N: usize> {
    buffer: ocl::Buffer<T>,
    shape: (usize, usize),
}

pub struct Canvas {
    image: ImageBuffer<f32, 4>,
    seeds: ImageBuffer<u32, 1>,
    clear: Image<f32, 4>,
    passes: usize,
}

impl<T: Copy + Default + OclPrm, const N: usize> ImageBuffer<T, N> {
    pub fn new(context: &ocl::Context, (width, height): (usize, usize)) -> base::Result<Self> {
        Ok(Self {
            buffer: ocl::Buffer::builder()
                .context(context)
                .len(N * width * height)
                .build()?,
            shape: (width, height),
        })
    }

    pub fn from_image(context: &ocl::Context, image: &Image<T, N>) -> base::Result<Self> {
        let shape = image.shape();
        Ok(Self {
            buffer: ocl::Buffer::builder()
                .context(context)
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

    pub fn buffer(&self) -> &ocl::Buffer<T> {
        &self.buffer
    }
    pub fn buffer_mut(&mut self) -> &mut ocl::Buffer<T> {
        &mut self.buffer
    }

    pub fn store(&mut self, queue: &ocl::Queue, image: &Image<T, N>) -> base::Result<()> {
        assert_eq!(self.shape(), image.shape());
        self.buffer_mut().write(image.data()).queue(queue).enq()?;
        queue.finish()?;
        Ok(())
    }
    pub fn load(&self, queue: &ocl::Queue, image: &mut Image<T, N>) -> base::Result<()> {
        assert_eq!(self.shape(), image.shape());
        self.buffer().read(image.data_mut()).queue(queue).enq()?;
        queue.finish()?;
        Ok(())
    }
}

impl Canvas {
    pub fn new(context: &ocl::Context, shape: (usize, usize)) -> base::Result<Self> {
        let mut seeds = Image::new(shape);
        SmallRng::seed_from_u64(0xdeadbeefdeadbeef).fill(seeds.data_mut());

        let clear = Image::new(shape);
        Ok(Self {
            image: ImageBuffer::from_image(context, &clear)?,
            seeds: ImageBuffer::from_image(context, &seeds)?,
            clear,
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
    pub fn clean(&mut self, queue: &ocl::Queue) -> base::Result<()> {
        self.image.store(queue, &self.clear)?;
        self.passes = 0;
        Ok(())
    }

    pub fn raw_image(&self) -> &ImageBuffer<f32, 4> {
        &self.image
    }

    pub fn seeds(&self) -> &ImageBuffer<u32, 1> {
        &self.seeds
    }
}

pub struct Extractor {
    kernel: ocl::Kernel,
}

impl Extractor {
    pub fn new(context: &ocl::Context) -> base::Result<Self> {
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
        let program = ocl::Program::builder().source(src).build(context)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("extract")
            .arg_named("width", &0u32)
            .arg_named("passes", &0u32)
            .arg_named("canvas", None::<&ocl::Buffer<f32>>)
            .arg_named("image", None::<&ocl::Buffer<f32>>)
            .build()?;

        Ok(Self { kernel })
    }

    pub fn process(&self, queue: &ocl::Queue, canvas: &Canvas, image: &mut ImageBuffer<f32, 4>) -> base::Result<()> {
        assert_eq!(canvas.shape(), image.shape());

        self.kernel.set_arg("width", image.width() as u32)?;
        self.kernel.set_arg("passes", canvas.passes() as u32)?;
        self.kernel.set_arg("canvas", canvas.raw_image().buffer())?;
        self.kernel.set_arg("image", image.buffer())?;
        let cmd = self.kernel
            .cmd()
            .queue(&queue)
            .global_work_size(image.shape());
        unsafe { cmd.enq()?; }

        queue.flush()?;
        Ok(())
    }
}

pub struct Packer {
    kernel: ocl::Kernel,
}

impl Packer {
    pub fn new(context: &ocl::Context) -> base::Result<Self> {
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
        let program = ocl::Program::builder().source(src).build(context)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("pack")
            .arg_named("width", &0u32)
            .arg_named("image", None::<&ocl::Buffer<f32>>)
            .arg_named("packed", None::<&ocl::Buffer<u8>>)
            .build()?;

        Ok(Self { kernel })
    }

    pub fn process(&self, queue: &ocl::Queue, image: &ImageBuffer<f32, 4>, packed: &mut ImageBuffer<u8, 4>) -> base::Result<()> {
        assert_eq!(image.shape(), packed.shape());

        self.kernel.set_arg("width", image.width() as u32)?;
        self.kernel.set_arg("image", image.buffer())?;
        self.kernel.set_arg("packed", packed.buffer())?;
        let cmd = self.kernel
            .cmd()
            .queue(&queue)
            .global_work_size(image.shape());
        unsafe { cmd.enq()?; }

        queue.flush()?;
        Ok(())
    }
}
