use crate::{Canvas, Buffer, Context};
use types::Shape;
use ccgeom::Geometry3;
use std::{marker::PhantomData};
use ocl_include::{Parser, Source, source};
use uni_path::PathBuf;

pub struct Render<G: Geometry3, T: Shape<G>> {
    kernel: ocl::Kernel,
    phantom: PhantomData<(G, T)>,
}

impl<G: Geometry3, T: Shape<G>> Render<G, T> {
    pub fn new(context: &Context, source: Option<Box<dyn Source>>) -> base::Result<Self> {
        let source_info = T::source(&context.config);
        let mut pb = Parser::builder();
        let text = if let Some(tree) = source_info.source_tree {
            if let Some(outer_src) = source {
                println!("Outer src");
                pb = pb.add_source(outer_src);
            }
            let mut msb = source::Mem::builder();
            for (k, v) in tree.tree {
                println!("Add file: {}, {}", k, v);
                msb = msb.add_file(&PathBuf::from(k), v)?;
            }
            let parser = pb.add_source(msb.build())
                .add_flag("HOST".into(), false)
                .add_flag("UNITTEST".into(), false)
                .add_flag("DOUBLE_SUPPORT".into(), context.config.double_support)
                .build();
            let node = parser.parse(&PathBuf::from(tree.root))?;
            let (text, _index) = node.collect();
            text
        } else {
            String::new()
        };

        let src = format!(r#"
            #define ADDRESS_WIDTH {}
            {}

            typedef {} Shape;
            float4 object_render(__global const Shape *object, float2 pos) {{
                return {}_render(object, pos);
            }}

            __kernel void render(
                const uint2 shape,
                __global const Shape *object,
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
        "#, context.config.address_width.num_value(), text, source_info.name, source_info.prefix);
        let program = ocl::Program::builder().source(src).build(&context.ocl)?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("render")
            .arg_named("shape", &ocl::prm::Uint2::new(0, 0))
            .arg_named("object", None::<&ocl::Buffer<u8>>)
            .arg_named("canvas", None::<&ocl::Buffer<f32>>)
            .build()?;
        Ok(Self { kernel, phantom: PhantomData })
    }

    pub fn render(&self, queue: &ocl::Queue, object_buffer: &Buffer<T>, canvas: &mut Canvas) -> base::Result<()> {
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
