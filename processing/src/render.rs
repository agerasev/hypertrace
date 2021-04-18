use crate::{Canvas, Buffer, Context};
use types::{Shape, Config};
use ccgeom::Geometry3;
use std::{marker::PhantomData, fs};
use ocl_include::{Parser, Index, source};
use uni_path::PathBuf;
use regex::{Regex, Captures};

pub struct Render<G: Geometry3, T: Shape<G>> {
    kernel: ocl::Kernel,
    phantom: PhantomData<(G, T)>,
}

impl<G: Geometry3, T: Shape<G>> Render<G, T> {
    fn source(config: &Config) -> base::Result<(String, Index)> {
        let source_info = T::source(config);
        let parser_builder = Parser::builder().add_source(&*kernel::SOURCE);
        
        let object_link = if let Some(root) = source_info.tree.root() {
            format!("#include <{}>", root)
        } else {
            String::new()
        };
        let mut memfs = source::Mem::builder();
        for (k, v) in source_info.tree.into_iter() {
            memfs = memfs.add_file(&k, v)?;
        }

        let root = PathBuf::from("render.ocl");
        memfs = memfs.add_file(&root, format!(
            r#"
                #define ADDRESS_WIDTH {}

                {}

                typedef {} Shape;
                #define shape_detect {}_detect

                #include <render/render.cc>
            "#,
            config.address_width.num_value(),
            object_link,
            source_info.name,
            source_info.prefix,
        ))?;

        let parser = parser_builder.add_source(memfs.build())
            .add_flag("HOST".into(), false)
            .add_flag("UNITTEST".into(), false)
            .add_flag("VARIADIC_MACROS".into(), false)
            .add_flag("DOUBLE_PRECISION".into(), config.double_support)
            .build();
        let node = parser.parse(&root)?;
        Ok(node.collect())
    }

    fn map_error(index: &Index, err: ocl::Error) -> ocl::Error {
        ocl::Error::from(ocl::OclCoreError::from(
            Regex::new(r#"([\w.:_\-/<>]*):([\d]+):([\d]+)"#).unwrap().replace_all(
                &String::from(err),
                |cap: &Captures| {
                    let line = cap[2].parse::<usize>().unwrap();
                    let (file, line) = index.search(line).map(|(f, l)| (String::from(f), l)).unwrap_or((cap[1].into(), line));
                    format!("{}:{}:{}", file, line, &cap[3])
                }
            ).into_owned()
        ))
    }

    pub fn new(context: &Context) -> base::Result<Self> {
        let (text, index) = Self::source(&context.config)?;
        fs::write("render.ocl", Regex::new(r#"\n\n+"#).unwrap().replace_all(&text, "\n\n").into_owned())?;

        let program = ocl::Program::builder().source(text).build(&context.ocl)
            .map_err(|e| Self::map_error(&index, e))?;
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
