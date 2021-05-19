mod basic;
mod modifier;

use types::{prelude::*, source::SourceTree, Config};

pub trait Material: SizedEntity {
    fn material_name() -> (String, String) {
        Self::name()
    }
    fn material_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use basic::*;
pub use modifier::*;
