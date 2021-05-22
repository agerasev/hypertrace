mod covered;
mod vector;
mod choice;
mod mapped;

use types::{prelude::*, source::SourceTree, Config};

pub trait Object<G: Geometry>: Entity {
    type Cache: SizedEntity;

    fn object_name() -> (String, String) {
        Self::name()
    }
    fn object_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use covered::Covered;
