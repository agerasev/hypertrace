use types::{Config, Type, Value, CountingWrapper};

pub struct Buffer<T: Type> {
    type_: T,
    buffer: ocl::Buffer<u8>,
}

impl<T: Type> Buffer<T> {
    pub fn new(context: &ocl::Context, cfg: &Config, value: T::Value) -> base::Result<Self> {
        let size = value.size(cfg);
        let mut buffer = Vec::with_capacity(size);
        value.store(cfg, &mut CountingWrapper::new(&mut buffer))?;
        Ok(Self {
            type_: value.type_(),
            buffer: ocl::Buffer::builder()
                .context(context)
                .copy_host_slice(&buffer)
                .len(size)
                .build()?,
        })
    }

    pub fn type_(&self) -> &T {
        &self.type_
    }
    pub fn buffer(&self) -> &ocl::Buffer<u8> {
        &self.buffer
    }
}
