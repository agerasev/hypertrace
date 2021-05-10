mod basic;

use types::prelude::*;

pub trait Background<G: Geometry>: SizedEntity {}

pub use basic::*;
