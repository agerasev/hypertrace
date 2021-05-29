mod absorbing;
mod transparent;
mod specular;
mod lambertian;
mod refractive;
mod modifier;
mod mixture;

use types::{prelude::*, source::SourceTree, Config};

pub trait Material: SizedEntity {
    fn material_name() -> (String, String) {
        Self::name()
    }
    fn material_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use absorbing::*;
pub use transparent::*;
pub use specular::*;
pub use lambertian::*;
pub use refractive::*;
pub use modifier::*;
pub use mixture::*;
