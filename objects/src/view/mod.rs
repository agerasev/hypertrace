use ccgeom::Geometry;
mod mapped;
mod point;

pub trait View<G: Geometry>: Sized {
    /// Lower the camera in f64; GPU conversion occurs when uploading the scene.
    fn wgsl_view(&self) -> crate::wgsl::Result<crate::wgsl::View> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}

pub use point::*;
