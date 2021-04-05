use crate::{Canvas, Buffer};
use types::{Object, ObjectType};

pub struct Render<T: ObjectType> where T::Value: Object {
    kernel: ocl::Kernel,
    object_type: T,
}

impl<T: ObjectType> Render<T> where T::Value: Object {
    pub fn new(context: &ocl::Context, object_type: T) -> base::Result<Self> {
        let src = format!(r#"
            {}

            typedef {} Object;
            float4 object_render(__global const Object *object, float2 pos) {{
                return {}_render(object, pos);
            }}

            __kernel void render(
                const uint2 shape,
                __global const Object *object,
                __global float4 *canvas
            ) {{
                uint2 pos = (uint2)(
                    get_global_id(0),
                    get_global_id(1)
                );
                uint idx = pos.x + shape.x * pos.y;
                float2 fpos = convert_float2(pos) / convert_float2(shape);
                float4 color = object_render(object, fpos);
                (float4)(
                    1.0f - 0.5f * (fpos.x + fpos.y),
                    fpos.x,
                    fpos.y,
                    1.0f
                );
                canvas[idx] += color;
            }}
        "#, object_type.source(), object_type.name(), object_type.name());
        let program = ocl::Program::builder().source(src).build(context)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("render")
            .arg_named("shape", &ocl::prm::Uint2::new(0, 0))
            .arg_named("object", None::<&ocl::Buffer<u8>>)
            .arg_named("canvas", None::<&ocl::Buffer<f32>>)
            .build()?;
        Ok(Self { kernel, object_type })
    }

    pub fn render(&self, queue: &ocl::Queue, object_buffer: &Buffer<T>, canvas: &mut Canvas) -> base::Result<()> {
        assert_eq!(self.object_type.id(), object_buffer.type_().id());
        self.kernel.set_arg("shape", ocl::prm::Uint2::new(canvas.width() as u32, canvas.height() as u32))?;
        self.kernel.set_arg("object", object_buffer.buffer())?;
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
