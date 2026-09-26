use ccgeom::Geometry;
mod choice;
mod covered;
mod mapped;
pub mod tiled_horosphere;
pub mod tiled_plane;
pub mod tiling;
mod vector;

pub trait Object<G: Geometry>: Sized {
    /// Register every shape/material type, including inactive choice variants.
    fn wgsl_register(_registry: &mut crate::wgsl::Registry) -> crate::wgsl::Result<()> {
        Err(crate::wgsl::unsupported::<Self>())
    }

    /// Lower object values without coupling builders to a GPU device.
    fn wgsl_object(&self) -> crate::wgsl::Result<crate::wgsl::ObjectNode> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}

pub use covered::Covered;
pub use tiled_horosphere::TiledHorosphere;
pub use tiled_plane::TiledPlane;
pub use tiling::Tiling;
