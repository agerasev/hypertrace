use crate::{Canvas, Buffer, Context};
use types::{Config, Named, SizedEntity, Sourced};
use objects::{View, Shape, Scene};
use ccgeom::Geometry3;
use std::{marker::PhantomData, fs};
use ocl_include::{Parser, Index, source};
use uni_path::PathBuf;
use regex::{Regex, Captures};

pub struct Render<G: Geometry3, V: View<G>, T: Shape<G> + SizedEntity> {
    kernel: ocl::Kernel,
    phantom: PhantomData<(G, V, T)>,
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Shape<G> + SizedEntity> Render<G, V, T> {
    fn source(config: &Config) -> base::Result<(String, Index)> {
        let source = <Scene::<G, V, T> as Sourced>::source(config);
        let parser_builder = Parser::builder().add_source(&*kernel::SOURCE);

        let include = PathBuf::from(source.root());
        let mut memfs = source::Mem::builder();
        for (k, v) in source.into_iter() {
            memfs = memfs.add_file(&k, v)?;
        }

        let root = PathBuf::from("render.ocl");
        memfs = memfs.add_file(&root, format!(
            r#"
                #define ADDRESS_WIDTH {}

                #include <{}>

                typedef {} Scene;
                #define scene_sample {}_sample

                #include <render/render.cc>
            "#,
            config.address_width.num_value(),
            include,
            <Scene::<G, V, T> as Named>::type_name(config),
            <Scene::<G, V, T> as Named>::type_prefix(config),
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
                    let origin = cap[2].parse::<usize>().unwrap() - 2; // `ocl` workaround
                    let (file, line) = index.search(origin).map(|(f, l)| (String::from(f), l)).unwrap_or((cap[1].into(), origin));
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
            .arg_named("scene", None::<&ocl::Buffer<u8>>)
            .arg_named("canvas", None::<&ocl::Buffer<f32>>)
            .build()?;

        Ok(Self { kernel, phantom: PhantomData })
    }

    pub fn render(&self, queue: &ocl::Queue, scene_buffer: &Buffer<Scene<G, V, T>>, canvas: &mut Canvas) -> base::Result<()> {
        self.kernel.set_arg("shape", ocl::prm::Uint2::new(canvas.width() as u32, canvas.height() as u32))?;
        self.kernel.set_arg("scene", scene_buffer.buffer())?;
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
