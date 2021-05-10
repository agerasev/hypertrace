pub mod euclidean;
mod mapped;
mod vector;

use types::prelude::*;

pub trait Shape<G: Geometry>: Entity {}

pub use mapped::MappedShape;
pub use vector::ShapeVector;
