use crate::Context;
use std::marker::PhantomData;
use types::{io::CntWrapper, Entity};

pub struct Buffer<T: Entity> {
    buffer: ocl::Buffer<u8>,
    phantom: PhantomData<T>,
}

impl<T: Entity> Buffer<T> {
    pub fn new(context: &Context, value: &T) -> base::Result<Self> {
        let size = value.size(&context.config);
        let mut buffer = Vec::with_capacity(size);
        if size == 0 {
            buffer.push(0);
        }
        value.store(&context.config, &mut CntWrapper::new(&mut buffer))?;
        Ok(Self {
            buffer: ocl::Buffer::builder()
                .context(&context.context)
                .copy_host_slice(&buffer)
                .len(buffer.len())
                .build()?,
            phantom: PhantomData,
        })
    }

    pub fn buffer(&self) -> &ocl::Buffer<u8> {
        &self.buffer
    }
}
