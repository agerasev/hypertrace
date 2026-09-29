use crate::shader::Geometry;
mod mapped;
mod point;

pub trait View<G: Geometry>: Sized {
    /// Lower the camera in f64; GPU conversion occurs when uploading the scene.
    fn view(&self) -> crate::shader::Result<crate::shader::View<G>>;
}

pub use point::*;
