use crate::{Canvas, Context, EntityBuffer};
use objects::Scene;
use ocl_include::{source, Index, Parser};
use regex::{Captures, Regex};
use std::{fs, marker::PhantomData};
use types::{prelude::*, Config};
use uni_path::PathBuf;

pub struct Render<G: Geometry, S: Scene<G>> {
    kernel: ocl::Kernel,
    phantom: PhantomData<(G, S)>,
}

impl<G: Geometry, S: Scene<G>> Render<G, S> {
    fn data_source(config: &Config) -> crate::Result<(String, Index)> {
        let source = S::scene_source(config);
        let parser_builder = Parser::builder().add_source(&*kernel::SOURCE);

        let include = PathBuf::from(source.root());
        let mut memfs = source::Mem::builder();
        for (k, v) in source.into_iter() {
            memfs = memfs.add_file(&k, v)?;
        }

        let root = PathBuf::from("render.ocl");
        memfs = memfs.add_file(
            &root,
            format!(
                r#"
                #define ADDRESS_WIDTH {}

                #include <{}>

                typedef {} Scene;
                #define scene_sample {}_sample

                #include <render/render.cl>
            "#,
                config.address_width.num_value(),
                include,
                S::name(),
                S::scene_prefix(),
            ),
        )?;

        let parser = parser_builder
            .add_source(memfs.build())
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
            Regex::new(r#"([\w.:_\-/<>]*):([\d]+):([\d]+)"#)
                .unwrap()
                .replace_all(&String::from(err), |cap: &Captures| {
                    let origin = cap[2].parse::<usize>().unwrap() - 2; // `ocl` workaround
                    let (file, line) = index
                        .search(origin)
                        .map(|(f, l)| (String::from(f), l))
                        .unwrap_or((cap[1].into(), origin));
                    format!("{}:{}:{}", file, line, &cap[3])
                })
                .into_owned(),
        ))
    }

    pub fn new(context: &Context) -> crate::Result<Self> {
        let (text, index) = Self::data_source(&context.config)?;
        fs::write(
            "render.ocl",
            Regex::new(r#"\n\n+"#)
                .unwrap()
                .replace_all(&text, "\n\n")
                .into_owned(),
        )?;

        let program = ocl::Program::builder()
            .source(text)
            .build(&context.backend.context)
            .map_err(|e| Self::map_error(&index, e))?;
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("render")
            .arg_named("shape", &ocl::prm::Uint2::new(0, 0))
            .arg_named("scene", None::<&ocl::Buffer<u8>>)
            .arg_named("canvas", None::<&ocl::Buffer<f32>>)
            .arg_named("seeds", None::<&ocl::Buffer<u32>>)
            .queue(context.backend.queue.clone())
            .build()?;

        Ok(Self {
            kernel,
            phantom: PhantomData,
        })
    }

    pub fn render(&self, scene_buffer: &EntityBuffer<S>, canvas: &mut Canvas) -> crate::Result<()> {
        self.kernel.set_arg(
            "shape",
            ocl::prm::Uint2::new(canvas.width() as u32, canvas.height() as u32),
        )?;
        self.kernel.set_arg("scene", scene_buffer.raw())?;
        self.kernel.set_arg("canvas", canvas.raw_image().raw())?;
        self.kernel.set_arg("seeds", canvas.seeds_mut().raw_mut())?;
        let cmd = self.kernel.cmd().global_work_size(canvas.shape());
        unsafe {
            cmd.enq()?;
        }

        self.kernel.default_queue().unwrap().finish()?;
        canvas.add_pass();
        Ok(())
    }
}
