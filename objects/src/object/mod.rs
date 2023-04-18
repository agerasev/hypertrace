mod choice;
mod covered;
mod mapped;
pub mod tiled_horosphere;
pub mod tiled_plane;
pub mod tiling;
mod vector;

use types::{prelude::*, source::SourceTree, Config};

pub trait Object<G: Geometry>: Entity {
    type Cache: SizedEntity + Clone;

    fn object_name() -> (String, String) {
        Self::name()
    }
    fn object_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use covered::Covered;
pub use tiled_horosphere::TiledHorosphere;
pub use tiled_plane::TiledPlane;
pub use tiling::Tiling;
