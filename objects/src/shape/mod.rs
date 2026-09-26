use ccgeom::Geometry;
mod choice;
mod mapped;
mod vector;

mod cube;
mod horosphere;
mod plane;
mod sphere;

pub trait Shape<G: Geometry>: Sized {
    /// Describe the WGSL implementation independently of the current value.
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Err(crate::wgsl::unsupported::<Self>())
    }

    /// Lower this shape's parameters to the portable scene representation.
    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}

pub use cube::*;
pub use horosphere::*;
pub use plane::*;
pub use sphere::*;
