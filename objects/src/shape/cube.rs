use super::*;
use ccgeom::Euclidean3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, Named, Entity, SizedEntity)]
pub struct Cube;

impl Sourced for Cube {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/basic.hh")
    }
}

impl Shape<Euclidean3> for Cube {
    fn shape_prefix(_: &Config) -> String {
        "cube_eu".into()
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/eu/cube.hh")
    }
}
