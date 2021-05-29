use uni_path::Path;
use lazy_static::lazy_static;
use ocl_include::source::Mem as MemSource;

include!(concat!(env!("OUT_DIR"), "/kernel_source.rs"));

lazy_static! {
    /// Kernel source.
    /// Contains files from `ocl`.
    pub static ref SOURCE: MemSource = {
        let mut builder = MemSource::builder();
        for (name, content) in FILES_STATIC.iter() {
            builder = builder.add_file(&Path::new(name), String::from(*content)).unwrap();
        }
        builder.build()
    };
}
