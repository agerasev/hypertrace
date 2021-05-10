mod mapped;
mod point;

use types::prelude::*;

pub trait View<G: Geometry>: SizedEntity + Sourced {}

pub use mapped::*;
pub use point::*;
