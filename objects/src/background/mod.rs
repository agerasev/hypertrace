mod basic;

use types::{prelude::*, source::SourceTree, Config};

pub trait Background<G: Geometry>: SizedEntity {
    fn wgsl_background(&self) -> crate::wgsl::Result<crate::wgsl::Background> {
        Err(crate::wgsl::unsupported::<Self>())
    }

    fn background_name() -> (String, String) {
        Self::name()
    }
    fn background_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use basic::*;
