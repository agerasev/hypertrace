use base::Image;
use ocl::OclPrm;

pub struct DeviceImage<T: Copy + Default + OclPrm, const N: usize> {
    buffer: ocl::Buffer<T>,
    shape: (usize, usize),
}

pub struct Canvas {
    image: DeviceImage<f32, 4>,
    passes: usize,
}

impl<T: Copy + Default + OclPrm, const N: usize> DeviceImage<T, N> {
    pub fn new(context: &ocl::Context, (width, height): (usize, usize)) -> base::Result<Self> {
        Ok(Self {
            buffer: ocl::Buffer::builder()
                .context(context)
                .len(N * width * height)
                .build()?,
            shape: (width, height),
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
        Ok(Self {
            image: DeviceImage::new(context, shape)?,
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
    pub fn clear(&mut self) {
        self.passes = 0;
    }

    pub fn image(&self) -> &DeviceImage<f32, 4> {
        &self.image
    }
    pub fn image_mut(&mut self) -> &mut DeviceImage<f32, 4> {
        &mut self.image
    }
}

pub struct Converter {
    kernel: ocl::Kernel,
    image: DeviceImage<u8, 4>,
}

impl Converter {
    pub fn new(context: &ocl::Context, shape: (usize, usize)) -> base::Result<Self> {
        let src = r#"
            __kernel void make_image(
                uint width,
                uint passes,
                __global const float4 *canvas,
                __global uchar4 *image
            ) {
                uint idx = get_global_id(0) + width * get_global_id(1);
                float4 fcolor = canvas[idx] / (float)passes;
                uchar4 icolor = convert_uchar4(0xFF * clamp(fcolor, 0.0f, 1.0f));
                image[idx] = icolor;
            }
        "#;
        let program = ocl::Program::builder().source(src).build(context)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("make_image")
            .arg_named("width", &0u32)
            .arg_named("passes", &0u32)
            .arg_named("canvas", None::<&ocl::Buffer<f32>>)
            .arg_named("image", None::<&ocl::Buffer<u8>>)
            .build()?;
        
        let image = DeviceImage::new(context, shape)?;

        Ok(Self {
            kernel,
            image,
        })
    }

    pub fn shape(&self) -> (usize, usize) {
        self.image.shape()
    }

    pub fn convert_canvas_to_image(&self, queue: &ocl::Queue, canvas: &Canvas, image: &mut Image<u8, 4>) -> base::Result<()> {
        assert_eq!(self.shape(), canvas.shape());
        assert_eq!(canvas.shape(), image.shape());

        self.kernel.set_arg("width", image.width() as u32)?;
        self.kernel.set_arg("passes", canvas.passes() as u32)?;
        self.kernel.set_arg("canvas", canvas.image().buffer())?;
        self.kernel.set_arg("image", self.image.buffer())?;
        let cmd = self.kernel
            .cmd()
            .queue(&queue)
            .global_work_size(image.shape());
        unsafe { cmd.enq()?; }
        
        self.image.load(queue, image)?;
        
        queue.finish()?;
        Ok(())
    }
}
