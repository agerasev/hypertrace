use crate::Context;
use std::marker::PhantomData;
use types::{
    io::{CntWrapper, EntityWriter},
    Entity,
};

pub struct EntityBuffer<T: Entity> {
    buffer: ocl::Buffer<u8>,
    phantom: PhantomData<T>,
}

impl<T: Entity> EntityBuffer<T> {
    pub fn new(context: &Context, value: &T) -> crate::Result<Self> {
        let size = value.size(&context.config);
        let mut buffer = Vec::with_capacity(size);
        if size == 0 {
            buffer.push(0);
        }
        CntWrapper::new(&mut buffer).write_entity::<T>(&context.config, value)?;
        Ok(Self {
            buffer: ocl::Buffer::builder()
                .queue(context.backend.queue.clone())
                .copy_host_slice(&buffer)
                .len(buffer.len())
                .build()?,
            phantom: PhantomData,
        })
    }

    pub fn raw(&self) -> &ocl::Buffer<u8> {
        &self.buffer
    }
}
