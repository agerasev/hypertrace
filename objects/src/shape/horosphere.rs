use super::*;
use ccgeom::Hyperbolic3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, EntityId, Entity, SizedEntity)]
pub struct Horosphere;

impl EntitySource for Horosphere {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/basic.hh")
    }
}

impl Shape<Hyperbolic3> for Horosphere {
    fn shape_prefix() -> String {
        Self::data_prefix()
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/hy/horosphere.hh")
    }
}
