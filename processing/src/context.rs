use types::Config;

pub struct Context {
    pub ocl: ocl::Context,
    pub config: Config,
}
