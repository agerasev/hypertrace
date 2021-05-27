use crate::{source::SourceTree, Config, EntityId, EntitySource, SizedEntity, Map};
use ccgeom::{Euclidean3, Hyperbolic3};

pub trait Geometry: EntitySource + Clone {
    type Pos: SizedEntity + Clone;
    type Dir: SizedEntity + Clone;
    type Map: Map<Self::Pos, Self::Dir> + SizedEntity + Clone;

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

// Geometry

impl<G: ccgeom::Geometry> Geometry for G where Self: EntitySource, G::Pos: SizedEntity, G::Dir: SizedEntity, G::Map: SizedEntity {
    type Pos = G::Pos;
    type Dir = G::Dir;
    type Map = G::Map;

    fn geometry_name() -> (String, String) {
        Self::name()
    }
    fn geometry_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}
