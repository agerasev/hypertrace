mod absorbing;
mod lambertian;
mod mixture;
mod modifier;
mod refractive;
mod specular;
mod transparent;

use crate::shader::{MaterialValue, Result, ShaderKind, ShaderModule};
pub trait Material: Sized {
    fn shader() -> Result<ShaderModule>;
    fn encode(&self) -> Result<MaterialValue>;
}

pub use absorbing::*;
pub use lambertian::*;
pub use mixture::*;
pub use modifier::*;
pub use refractive::*;
pub use specular::*;
pub use transparent::*;

pub trait MaterialValueExt {
    fn colored(self, rgb: [f32; 3]) -> Result<MaterialValue>;
    fn emissive(self, rgb: [f32; 3]) -> Result<MaterialValue>;
}
impl MaterialValueExt for MaterialValue {
    fn colored(self, rgb: [f32; 3]) -> Result<MaterialValue> {
        colored(self, rgb)
    }
    fn emissive(self, rgb: [f32; 3]) -> Result<MaterialValue> {
        emissive(self, rgb)
    }
}
