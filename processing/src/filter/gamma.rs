use crate::{Filter, ImageBuffer, OclContext};

pub struct GammaFilter {
    kernel: ocl::Kernel,
    pub gamma: f32,
}

impl GammaFilter {
    pub fn new(context: &OclContext, gamma: f32) -> crate::Result<Self> {
        let src = r#"
            __kernel void filter(
                uint width,
                float gamma,
                __global const float4 *input,
                __global float4 *output
            ) {
                uint idx = get_global_id(0) + width * get_global_id(1);
                output[idx] = pow(max(input[idx], 0.0f), gamma);
            }
        "#;
        let program = ocl::Program::builder()
            .source(src)
            .build(&context.context)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("filter")
            .arg_named("width", &0u32)
            .arg_named("gamma", &0.0f32)
            .arg_named("input", None::<&ocl::Buffer<f32>>)
            .arg_named("output", None::<&ocl::Buffer<f32>>)
            .queue(context.queue.clone())
            .build()?;

        Ok(Self { kernel, gamma })
    }
}

impl Filter for GammaFilter {
    fn process(
        &self,
        input: &ImageBuffer<f32, 4>,
        output: &mut ImageBuffer<f32, 4>,
    ) -> crate::Result<()> {
        assert_eq!(input.shape(), output.shape());

        self.kernel.set_arg("width", input.width() as u32)?;
        self.kernel.set_arg("gamma", self.gamma)?;
        self.kernel.set_arg("input", input.raw())?;
        self.kernel.set_arg("output", output.raw_mut())?;
        let cmd = self.kernel.cmd().global_work_size(input.shape());
        unsafe {
            cmd.enq()?;
        }

        self.kernel.default_queue().unwrap().flush()?;
        Ok(())
    }
}
