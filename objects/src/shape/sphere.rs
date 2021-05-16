use super::*;
use ccgeom::Euclidean3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, EntityId, Entity, SizedEntity)]
pub struct Sphere;

impl EntitySource for Sphere {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/basic.hh")
    }
}

impl Shape<Euclidean3> for Sphere {
    fn shape_prefix() -> String {
        "sphere_eu".into()
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/eu/sphere.hh")
    }
}
