use super::*;
use ccgeom::Hyperbolic3;
use type_macros::{Entity, SizedEntity};
use types::{source::SourceTree, Config, Named, Sourced};

// Horosphere

#[derive(Clone, Default, Debug, Entity, SizedEntity)]
pub struct Horosphere;

impl Named for Horosphere {
    fn type_name(_: &Config) -> String {
        "Horosphere".into()
    }
    fn type_prefix(_: &Config) -> String {
        "horosphere".into()
    }
}

impl Sourced for Horosphere
where
    Self: Entity,
{
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/hy/horosphere.hh")
    }
}

impl Shape<Hyperbolic3> for Horosphere {}

// Hyperbolic plane

#[derive(Clone, Default, Debug, Entity, SizedEntity)]
pub struct Plane;

impl Named for Plane {
    fn type_name(_: &Config) -> String {
        "PlaneHy".into()
    }
    fn type_prefix(_: &Config) -> String {
        "plane_hy".into()
    }
}

impl Sourced for Plane
where
    Self: Entity,
{
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/hy/plane.hh")
    }
}

impl Shape<Hyperbolic3> for Plane {}
