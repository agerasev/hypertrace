mod basic;

use types::{prelude::*, source::SourceTree, Config};

pub trait Background<G: Geometry>: SizedEntity {
    fn background_prefix(cfg: &Config) -> String {
        Self::type_prefix(cfg)
    }
    fn background_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use basic::*;
