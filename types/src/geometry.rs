use ccgeom::{Euclidean3, Hyperbolic3};
use crate::{Config, source::{Sourced, SourceInfo}};

impl Sourced for Euclidean3 {
    fn source(_: &Config) -> SourceInfo {
        SourceInfo::with_root(
            "Eu",
            "eu",
            "geometry/euclidean.hh",
        )
    }
}

impl Sourced for Hyperbolic3 {
    fn source(_: &Config) -> SourceInfo {
        SourceInfo::with_root(
            "Hy",
            "hy",
            "geometry/hyperbolic.hh",
        )
    }
}
