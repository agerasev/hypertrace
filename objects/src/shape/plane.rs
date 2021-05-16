use super::*;
use ccgeom::Hyperbolic3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, EntityId, Entity, SizedEntity)]
pub struct Plane;

impl EntitySource for Plane {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/basic.hh")
    }
}

impl Shape<Hyperbolic3> for Plane {
    fn shape_prefix() -> String {
        "plane_hy".into()
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/hy/plane.hh")
    }
}
