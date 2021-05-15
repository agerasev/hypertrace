mod gamma;
mod sequence;

pub use gamma::*;
pub use sequence::*;

use crate::ImageBuffer;

pub trait Filter {
    fn process(
        &self,
        input: &ImageBuffer<f32, 4>,
        output: &mut ImageBuffer<f32, 4>,
    ) -> crate::Result<()>;
}
