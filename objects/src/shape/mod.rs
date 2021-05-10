pub mod euclidean;
mod mapped;

use ccgeom::Geometry3;
use types::{source::Sourced, Entity};

pub trait Shape<G: Geometry3>: Entity + Sourced {}

pub use mapped::MappedShape;
