use crate::{
    source::{SourceTree, Sourced},
    Config, Named,
};
use ccgeom::{Euclidean3, Hyperbolic3};

impl Named for Euclidean3 {
    fn type_name(_: &Config) -> String {
        "Eu".into()
    }
    fn type_prefix(_: &Config) -> String {
        "eu".into()
    }
}
impl Sourced for Euclidean3 {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("geometry/euclidean.hh")
    }
}

impl Named for Hyperbolic3 {
    fn type_name(_: &Config) -> String {
        "Hy".into()
    }
    fn type_prefix(_: &Config) -> String {
        "hy".into()
    }
}
impl Sourced for Hyperbolic3 {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("geometry/hyperbolic.hh")
    }
}
