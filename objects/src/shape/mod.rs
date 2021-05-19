mod mapped;
mod vector;

mod cube;
mod horosphere;
mod plane;
mod sphere;

use types::{prelude::*, source::SourceTree, Config};

pub trait Shape<G: Geometry>: Entity {
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
