mod covered;

use types::{Entity, Sourced};
use ccgeom::Geometry3;

pub trait Object<G: Geometry3>: Entity + Sourced {}

pub use covered::*;
