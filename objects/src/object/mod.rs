mod covered;

use types::{Named, Entity, Sourced};
use ccgeom::Geometry3;

pub trait Object<G: Geometry3>: Named + Entity + Sourced {}

pub use covered::*;
