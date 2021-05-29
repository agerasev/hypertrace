use super::*;
use base::ccgeom::{Euclidean3, Hyperbolic3};
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, EntityId, Entity, SizedEntity)]
pub struct Plane;

impl EntitySource for Plane {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/primitive.hh")
    }
}

impl Shape<Euclidean3> for Plane {
    fn shape_name() -> (String, String) {
        ("PlaneEu".into(), "plane_eu".into())
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/eu/plane.hh")
    }
}

impl Shape<Hyperbolic3> for Plane {
    fn shape_name() -> (String, String) {
        ("PlaneHy".into(), "plane_hy".into())
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/hy/plane.hh")
    }
}
