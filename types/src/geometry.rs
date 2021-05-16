use crate::{source::SourceTree, Config, EntityId, EntitySource};
use ccgeom::{Euclidean3, Hyperbolic3};

pub trait Geometry: ccgeom::Geometry + EntitySource {
    fn geometry_prefix() -> String;
    fn geometry_source(cfg: &Config) -> SourceTree;
}

// Euclidean

impl EntityId for Euclidean3 {
    fn name() -> String {
        "Eu".into()
    }
    fn data_prefix() -> String {
        "eu".into()
    }
}

impl EntitySource for Euclidean3 {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("geometry/euclidean.hh")
    }
}

impl Geometry for Euclidean3 {
    fn geometry_prefix() -> String {
        Self::data_prefix()
    }
    fn geometry_source(cfg: &Config) -> SourceTree {
        Self::data_source(cfg)
    }
}

// Hyperbolic

impl EntityId for Hyperbolic3 {
    fn name() -> String {
        "Hy".into()
    }
    fn data_prefix() -> String {
        "hy".into()
    }
}

impl EntitySource for Hyperbolic3 {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("geometry/hyperbolic.hh")
    }
}

impl Geometry for Hyperbolic3 {
    fn geometry_prefix() -> String {
        Self::data_prefix()
    }
    fn geometry_source(cfg: &Config) -> SourceTree {
        Self::data_source(cfg)
    }
}
