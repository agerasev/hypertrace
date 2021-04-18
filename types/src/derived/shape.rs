use crate::{Entity, Sourced};
use ccgeom::Geometry3;

pub trait Shape<G: Geometry3<f64>>: Entity + Sourced {}
