mod covered;
mod vector;

use types::{prelude::*, source::SourceTree, Config};

pub trait Object<G: Geometry>: Entity {
    fn object_name() -> (String, String) {
        Self::name()
    }
    fn object_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use covered::*;
pub use vector::*;
