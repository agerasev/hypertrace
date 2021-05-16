mod mapped;
mod vector;

mod cube;
mod horosphere;
mod plane;
mod sphere;

use types::{prelude::*, source::SourceTree, Config};

pub trait Shape<G: Geometry>: Entity {
    fn shape_prefix() -> String {
        Self::data_prefix()
    }
    fn shape_source(cfg: &Config) -> SourceTree {
        Self::data_source(cfg)
    }
}

pub use cube::*;
pub use horosphere::*;
pub use plane::*;
pub use sphere::*;
