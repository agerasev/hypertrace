mod basic;

use types::{prelude::*, source::SourceTree, Config};

pub trait Background<G: Geometry>: SizedEntity {
    fn background_prefix() -> String {
        Self::data_prefix()
    }
    fn background_source(cfg: &Config) -> SourceTree {
        Self::data_source(cfg)
    }
}

pub use basic::*;
