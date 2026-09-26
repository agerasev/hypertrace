use ccgeom::Geometry;
mod basic;

pub trait Background<G: Geometry>: Sized {
    fn wgsl_background(&self) -> crate::wgsl::Result<crate::wgsl::Background> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}

pub use basic::*;
