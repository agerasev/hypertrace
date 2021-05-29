mod basic;

use types::{prelude::*, source::SourceTree, Config};

pub trait Background<G: Geometry>: SizedEntity {
    fn background_name() -> (String, String) {
        Self::name()
    }
    fn background_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use basic::*;
