use ccgeom::Geometry;
mod choice;
mod mapped;
mod vector;

mod cube;
mod horosphere;
mod plane;
mod sphere;

use crate::shader::{
    Geometry as RenderGeometry, Result, ShaderKind, ShaderModule, ShapeValue, Transform,
};

pub trait Shape<G: Geometry>: Sized {
    /// Describe this implementation and its dependencies independently of values.
    fn shader() -> Result<ShaderModule>;
    /// Encode this instance's parameters, independently of a GPU device.
    fn encode(&self) -> Result<ShapeValue>;
}

pub use cube::*;
pub use horosphere::*;
pub use plane::*;
pub use sphere::*;

pub use choice::{choice, choice_schema};
pub use mapped::{mapped, mapped_schema};
pub use vector::{vector, vector_schema};

pub trait ShapeValueExt {
    fn mapped(self, map: Transform) -> Result<ShapeValue>;
}
impl ShapeValueExt for ShapeValue {
    fn mapped(self, map: Transform) -> Result<ShapeValue> {
        mapped(self, map)
    }
}
