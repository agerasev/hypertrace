use crate::{Filter, ImageBuffer};
use std::mem::swap;

pub trait FilterSequence {
    fn process_sequence(
        &self,
        current: &mut ImageBuffer<f32, 4>,
        next: &mut ImageBuffer<f32, 4>,
    ) -> crate::Result<()>;
}

impl<F: Filter> FilterSequence for F {
    fn process_sequence(
        &self,
        current: &mut ImageBuffer<f32, 4>,
        next: &mut ImageBuffer<f32, 4>,
    ) -> crate::Result<()> {
        self.process(current, next).map(|_| swap(current, next))
    }
}
