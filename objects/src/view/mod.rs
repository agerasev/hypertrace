mod mapped;
mod point;

use types::{prelude::*, source::SourceTree, Config};

pub trait View<G: Geometry>: SizedEntity + Sourced {
    fn view_prefix(cfg: &Config) -> String {
        Self::type_prefix(cfg)
    }
    fn view_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use mapped::*;
pub use point::*;
