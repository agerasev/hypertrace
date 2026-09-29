mod absorbing;
mod lambertian;
mod mixture;
mod modifier;
mod refractive;
mod specular;
mod transparent;

use crate::shader::{Geometry, MaterialModule, MaterialValue, Result};
pub trait Material<G: Geometry>: Sized {
    fn shader() -> Result<MaterialModule<G>>;
    fn encode(&self) -> Result<MaterialValue<G>>;
}

pub use absorbing::*;
pub use lambertian::*;
pub use mixture::*;
pub use modifier::*;
pub use refractive::*;
pub use specular::*;
pub use transparent::*;

pub trait MaterialValueExt<G: Geometry> {
    fn colored(self, rgb: [f32; 3]) -> Result<MaterialValue<G>>;
    fn emissive(self, rgb: [f32; 3]) -> Result<MaterialValue<G>>;
}
impl<G: Geometry> MaterialValueExt<G> for MaterialValue<G> {
    fn colored(self, rgb: [f32; 3]) -> Result<MaterialValue<G>> {
        colored(self, rgb)
    }
    fn emissive(self, rgb: [f32; 3]) -> Result<MaterialValue<G>> {
        emissive(self, rgb)
    }
}
