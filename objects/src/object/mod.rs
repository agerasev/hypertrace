mod covered;
mod vector;

use types::prelude::*;

pub trait Object<G: Geometry>: Entity {}

pub use covered::*;
pub use vector::*;
