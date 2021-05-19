mod mapped;
mod point;

use types::{prelude::*, source::SourceTree, Config};

pub trait View<G: Geometry>: SizedEntity + EntitySource {
    fn view_name() -> (String, String) {
        Self::name()
    }
    fn view_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use mapped::*;
pub use point::*;
