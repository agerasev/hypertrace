mod covered;

use ccgeom::Geometry3;
use types::{Entity, Sourced};

pub trait Object<G: Geometry3>: Entity + Sourced {}

pub use covered::*;
