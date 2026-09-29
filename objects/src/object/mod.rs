use ccgeom::Geometry;
mod choice;
mod covered;
mod mapped;
pub mod tiled_horosphere;
pub mod tiled_plane;
pub mod tiling;
mod vector;

pub trait Object<G: Geometry>: Sized {
    /// Describe all possible implementations, including inactive variants.
    fn shader_modules() -> crate::shader::Result<Vec<crate::shader::ShaderModule>>;
    fn object_node(&self) -> crate::shader::Result<crate::shader::ObjectNode>;
}

pub use covered::Covered;
pub use tiled_horosphere::TiledHorosphere;
pub use tiled_plane::TiledPlane;
pub use tiling::Tiling;
