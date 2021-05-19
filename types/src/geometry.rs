use crate::{source::SourceTree, Config, EntityId, EntitySource};
use ccgeom::{Euclidean3, Hyperbolic3};

pub trait Geometry: ccgeom::Geometry + EntitySource {
    fn geometry_name() -> (String, String);
    fn geometry_source(cfg: &Config) -> SourceTree;
}

// Euclidean

impl EntityId for Euclidean3 {
    fn name() -> (String, String) {
        ("Eu".into(), "eu".into())
    }
}

impl EntitySource for Euclidean3 {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("geometry/euclidean.hh")
    }
}

impl Geometry for Euclidean3 {
    fn geometry_name() -> (String, String) {
        Self::name()
    }
    fn geometry_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

// Hyperbolic

impl EntityId for Hyperbolic3 {
    fn name() -> (String, String) {
        ("Hy".into(), "hy".into())
    }
}

impl EntitySource for Hyperbolic3 {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("geometry/hyperbolic.hh")
    }
}

impl Geometry for Hyperbolic3 {
    fn geometry_name() -> (String, String) {
        Self::name()
    }
    fn geometry_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}
