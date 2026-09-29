use crate::shader::Geometry;
mod covered;
mod mapped;
pub mod tiled_horosphere;
pub mod tiled_plane;
pub mod tiling;
mod tuple;
mod vector;

pub trait Object<G: Geometry>: Sized {
    /// Describe element implementations even when homogeneous vectors are empty.
    fn shader_modules() -> crate::shader::Result<crate::shader::Modules<G>>;
    /// Statically traverse the object tree into canonical GPU input records.
    fn encode_objects(
        &self,
        outer: crate::shader::Transform<G>,
        output: &mut Vec<crate::shader::EncodedObject<G>>,
    ) -> crate::shader::Result<()>;
}

pub use covered::Covered;
pub use tiled_horosphere::TiledHorosphere;
pub use tiled_plane::TiledPlane;
pub use tiling::Tiling;
