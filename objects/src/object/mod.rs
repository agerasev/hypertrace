mod covered;
mod vector;

use types::{prelude::*, source::SourceTree, Config};

pub trait Object<G: Geometry>: Entity {
    fn object_prefix() -> String {
        Self::data_prefix()
    }
    fn object_source(cfg: &Config) -> SourceTree {
        Self::data_source(cfg)
    }
}

pub use covered::*;
pub use vector::*;
