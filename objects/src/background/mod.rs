use ccgeom::Geometry;
mod basic;

pub trait Background<G: Geometry>: Sized {
    fn background(&self) -> crate::shader::Result<crate::shader::Background>;
}

pub use basic::*;
