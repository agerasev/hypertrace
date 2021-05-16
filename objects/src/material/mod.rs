mod basic;
mod modifier;

use types::{prelude::*, source::SourceTree, Config};

pub trait Material: SizedEntity {
    fn material_prefix() -> String {
        Self::data_prefix()
    }
    fn material_source(cfg: &Config) -> SourceTree {
        Self::data_source(cfg)
    }
}

pub use basic::*;
pub use modifier::*;
