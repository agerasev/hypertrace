mod absorbing;
mod lambertian;
mod mixture;
mod modifier;
mod refractive;
mod specular;
mod transparent;

pub trait Material: Sized {
    /// Describe this material for WGSL compilation without depending on values.
    /// The default reports an unsupported material until a hook is supplied.
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Err(crate::wgsl::unsupported::<Self>())
    }

    /// Lower values while preserving modifier order and nested mixture draws.
    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}

pub use absorbing::*;
pub use lambertian::*;
pub use mixture::*;
pub use modifier::*;
pub use refractive::*;
pub use specular::*;
pub use transparent::*;
