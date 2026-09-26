mod absorbing;
mod lambertian;
mod mixture;
mod modifier;
mod refractive;
mod specular;
mod transparent;

use types::{prelude::*, source::SourceTree, Config};

pub trait Material: SizedEntity {
    /// Describe this material for WGSL compilation without depending on values.
    /// Existing OpenCL-only materials remain valid and report an error here.
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Err(crate::wgsl::unsupported::<Self>())
    }

    /// Lower values while preserving modifier order and nested mixture draws.
    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        Err(crate::wgsl::unsupported::<Self>())
    }

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
