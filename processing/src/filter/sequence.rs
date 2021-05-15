use crate::{Filter, ImageBuffer};
use std::mem::swap;

pub trait FilterSequence {
    fn process_sequence(
        &self,
        queue: &ocl::Queue,
        current: &mut ImageBuffer<f32, 4>,
        next: &mut ImageBuffer<f32, 4>,
    ) -> base::Result<()>;
}

impl<F: Filter> FilterSequence for F {
    fn process_sequence(
        &self,
        queue: &ocl::Queue,
        current: &mut ImageBuffer<f32, 4>,
        next: &mut ImageBuffer<f32, 4>,
    ) -> base::Result<()> {
        self.process(queue, current, next)
            .map(|_| swap(current, next))
    }
}
