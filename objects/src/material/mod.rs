mod absorbing;
mod lambertian;
mod mixture;
mod modifier;
mod refractive;
mod specular;
mod transparent;

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
pub use lambertian::*;
pub use mixture::*;
pub use modifier::*;
pub use refractive::*;
pub use specular::*;
pub use transparent::*;
