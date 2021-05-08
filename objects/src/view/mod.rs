mod point;
mod mapped;

use ccgeom::Geometry3;
use types::{SizedEntity, Sourced};

pub trait View<G: Geometry3>: SizedEntity + Sourced {}

pub use point::*;
pub use mapped::*;
