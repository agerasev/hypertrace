use crate::{Filter, ImageBuffer};

pub struct GammaFilter {
    kernel: ocl::Kernel,
    pub gamma: f32,
}

impl GammaFilter {
    pub fn new(context: &ocl::Context, gamma: f32) -> base::Result<Self> {
        let src = r#"
            __kernel void filter(
                uint width,
                float gamma,
                __global const float4 *input,
                __global float4 *output
            ) {
                uint idx = get_global_id(0) + width * get_global_id(1);
                output[idx] = pow(input[idx], gamma);
            }
        "#;
        let program = ocl::Program::builder().source(src).build(context)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("filter")
            .arg_named("width", &0u32)
            .arg_named("gamma", &0.0f32)
            .arg_named("input", None::<&ocl::Buffer<f32>>)
            .arg_named("output", None::<&ocl::Buffer<f32>>)
            .build()?;

        Ok(Self { kernel, gamma })
    }
}

impl Filter for GammaFilter {
    fn process(
        &self,
        queue: &ocl::Queue,
        input: &ImageBuffer<f32, 4>,
        output: &mut ImageBuffer<f32, 4>,
    ) -> base::Result<()> {
        assert_eq!(input.shape(), output.shape());

        self.kernel.set_arg("width", input.width() as u32)?;
        self.kernel.set_arg("gamma", self.gamma)?;
        self.kernel.set_arg("input", input.buffer())?;
        self.kernel.set_arg("output", output.buffer())?;
        let cmd = self
            .kernel
            .cmd()
            .queue(&queue)
            .global_work_size(input.shape());
        unsafe {
            cmd.enq()?;
        }

        queue.flush()?;
        Ok(())
    }
}
