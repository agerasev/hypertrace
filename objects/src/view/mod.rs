mod mapped;
mod point;

use types::{prelude::*, source::SourceTree, Config};

pub trait View<G: Geometry>: SizedEntity + EntitySource {
    fn view_prefix() -> String {
        Self::data_prefix()
    }
    fn view_source(cfg: &Config) -> SourceTree {
        Self::data_source(cfg)
    }
}

pub use mapped::*;
pub use point::*;
