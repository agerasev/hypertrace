mod basic;
mod modifier;

use types::{prelude::*, source::SourceTree, Config};

pub trait Material: SizedEntity {
    fn material_prefix(cfg: &Config) -> String {
        Self::type_prefix(cfg)
    }
    fn material_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use basic::*;
pub use modifier::*;
