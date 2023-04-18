use crate::{Mapped, Shape};
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config, Map, RayMap,
};

impl<G: Geometry, T: Shape<G>, M: Map<G::Pos, G::Dir>> Shape<G> for Mapped<G, T, M> {
    fn shape_name() -> (String, String) {
        (
            format!("Shape{}", Self::name().0),
            format!("shape_{}", Self::name().1),
        )
    }
    fn shape_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::shape_name().1))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(M::map_source(cfg))
            .tree(T::shape_source(cfg))
            .content(&include(format!(
                "geometry/ray_{}.hh",
                &G::geometry_name().1
            )))
            .tree(RayMap::<G, M>::source(cfg))
            .content(&include_template!(
                "shape/mapped.inl",
                ("Self", "self") => Self::shape_name(),
                ("Base", "base") => Self::name(),
                ("Geo", "geo") => G::geometry_name(),
                ("Map", "map") => M::map_name(),
                ("Shape", "shape") => T::shape_name(),
            ))
            .build()
    }
}
