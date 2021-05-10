use super::*;
use ccgeom::Euclidean3;
use type_macros::{Entity, SizedEntity};
use types::{
    source::{SourceTree, Sourced},
    Config, Named,
};

// Sphere

#[derive(Clone, Default, Debug, Entity, SizedEntity)]
pub struct Sphere;

impl Named for Sphere {
    fn type_name(_: &Config) -> String {
        "SphereEu".into()
    }
    fn type_prefix(_: &Config) -> String {
        "sphere_eu".into()
    }
}

impl Sourced for Sphere
where
    Self: Entity,
{
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/eu/sphere.hh")
    }
}

impl Shape<Euclidean3> for Sphere {}

// Cube

#[derive(Clone, Default, Debug, Entity, SizedEntity)]
pub struct Cube;

impl Named for Cube {
    fn type_name(_: &Config) -> String {
        "CubeEu".into()
    }
    fn type_prefix(_: &Config) -> String {
        "cube_eu".into()
    }
}

impl Sourced for Cube
where
    Self: Entity,
{
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/eu/cube.hh")
    }
}

impl Shape<Euclidean3> for Cube {}
