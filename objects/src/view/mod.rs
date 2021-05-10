mod mapped;
mod point;

use ccgeom::Geometry3;
use types::{SizedEntity, Sourced};

pub trait View<G: Geometry3>: SizedEntity + Sourced {}

pub use mapped::*;
pub use point::*;
