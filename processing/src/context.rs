use types::Config;

#[derive(Clone)]
pub struct Context {
    pub context: ocl::Context,
    pub queue: ocl::Queue,
    pub config: Config,
}
