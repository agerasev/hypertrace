use types::Config;

#[derive(Clone)]
pub struct OclContext {
    pub context: ocl::Context,
    pub queue: ocl::Queue,
}

#[cfg(feature = "backend_ocl")]
pub type BackendContext = OclContext;

#[derive(Clone)]
pub struct Context {
    pub backend: BackendContext,
    pub config: Config,
}
