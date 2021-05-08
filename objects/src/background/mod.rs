mod basic;

use types::{Named, Entity, Sourced};
use ccgeom::Geometry3;

pub trait Background<G: Geometry3>: Named + Entity + Sourced {}

pub use basic::*;
