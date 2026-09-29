use crate::shader::Geometry;
mod basic;

pub trait Background<G: Geometry>: Sized {
    fn background(&self) -> crate::shader::Result<crate::shader::Background<G>>;
}

pub use basic::*;
