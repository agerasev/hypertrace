mod basic;

use ccgeom::Geometry3;
use types::{Entity, Named, Sourced};

pub trait Background<G: Geometry3>: Named + Entity + Sourced {}

pub use basic::*;
