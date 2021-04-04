use crate::{Canvas};

pub struct Render {
    kernel: ocl::Kernel,
}

impl Render {
    pub fn new(context: &ocl::Context) -> base::Result<Self> {
        let src = r#"
            __kernel void render(
                const uint2 shape,
                __global float4 *canvas
            ) {
                uint2 pos = (uint2)(
                    get_global_id(0),
                    get_global_id(1)
                );
                uint idx = pos.x + shape.x * pos.y;
                float2 fpos = convert_float2(pos) / convert_float2(shape);
                float4 color = (float4)(
                    1.0f - 0.5f * (fpos.x + fpos.y),
                    fpos.x,
                    fpos.y,
                    1.0f
                );
                canvas[idx] += color;
            }
        "#;
        let program = ocl::Program::builder().source(src).build(context)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("render")
            .arg_named("shape", &ocl::prm::Uint2::new(0, 0))
            .arg_named("canvas", None::<&ocl::Buffer<ocl::prm::Float4>>)
            .build()?;
        Ok(Self { kernel })
    }

    pub fn render(&self, queue: &ocl::Queue, canvas: &mut Canvas) -> base::Result<()> {
        self.kernel.set_arg("shape", ocl::prm::Uint2::new(canvas.width() as u32, canvas.height() as u32))?;
        self.kernel.set_arg("canvas", canvas.image().buffer())?;
        let cmd = self.kernel
            .cmd()
            .queue(queue)
            .global_work_size(canvas.shape());
        unsafe { cmd.enq()?; }

        queue.finish()?;
        canvas.add_pass();
        Ok(())
    }
}
