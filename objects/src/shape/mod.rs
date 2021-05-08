pub mod euclidean;
mod mapped;

use types::{Entity, source::Sourced};
use ccgeom::Geometry3;

pub trait Shape<G: Geometry3>: Entity + Sourced {}

pub use mapped::MappedShape;
