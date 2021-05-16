use super::*;
use ccgeom::Hyperbolic3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, Named, Entity, SizedEntity)]
pub struct Horosphere;

impl Sourced for Horosphere {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/basic.hh")
    }
}

impl Shape<Hyperbolic3> for Horosphere {
    fn shape_prefix(cfg: &Config) -> String {
        Self::type_prefix(cfg)
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/hy/horosphere.hh")
    }
}
