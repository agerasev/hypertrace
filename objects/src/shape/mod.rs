use crate::shader::Geometry;
mod mapped;
mod tuple;
mod vector;

mod cube;
mod horosphere;
mod plane;
mod sphere;

use crate::shader::{Result, ShapeModule, ShapeValue, Transform};

pub trait Shape<G: Geometry>: Sized {
    /// Describe this implementation and its dependencies independently of values.
    fn shader() -> Result<ShapeModule<G>>;
    /// Encode this instance's parameters, independently of a GPU device.
    fn encode(&self) -> Result<ShapeValue<G>>;
}

pub use cube::*;
pub use horosphere::*;
pub use plane::*;
pub use sphere::*;

pub use mapped::{mapped, mapped_schema};
pub use tuple::{tuple, tuple_schema};
pub use vector::{vector, vector_schema};

pub trait ShapeValueExt<G: Geometry> {
    fn mapped(self, map: Transform<G>) -> Result<ShapeValue<G>>;
}
impl<G: Geometry> ShapeValueExt<G> for ShapeValue<G> {
    fn mapped(self, map: Transform<G>) -> Result<ShapeValue<G>> {
        mapped(self, map)
    }
}
