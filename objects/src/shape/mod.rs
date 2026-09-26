mod choice;
mod mapped;
mod vector;

mod cube;
mod horosphere;
mod plane;
mod sphere;

use types::{prelude::*, source::SourceTree, Config};

pub trait Shape<G: Geometry>: Entity {
    /// Describe the WGSL implementation independently of the current value.
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Err(crate::wgsl::unsupported::<Self>())
    }

    /// Lower this shape's parameters to the portable scene representation.
    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Err(crate::wgsl::unsupported::<Self>())
    }

    fn shape_name() -> (String, String) {
        Self::name()
    }
    fn shape_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use cube::*;
pub use horosphere::*;
pub use plane::*;
pub use sphere::*;
