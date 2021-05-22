use super::*;
use ccgeom::Hyperbolic3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, EntityId, Entity, SizedEntity)]
pub struct Horosphere;

impl EntitySource for Horosphere {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/hy/horosphere.hh")
    }
}

impl Shape<Hyperbolic3> for Horosphere {}
