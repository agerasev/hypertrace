use super::*;
use base::ccgeom::Euclidean3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, EntityId, Entity, SizedEntity)]
pub struct Sphere;

impl EntitySource for Sphere {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/primitive.hh")
    }
}

impl Shape<Euclidean3> for Sphere {
    fn shape_name() -> (String, String) {
        ("SphereEu".into(), "sphere_eu".into())
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/eu/sphere.hh")
    }
}
