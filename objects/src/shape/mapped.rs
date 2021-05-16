use crate::{Mapped, Shape};
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config, Map, RayMap,
};

impl<G: Geometry, T: Shape<G>, M: Map<G::Pos, G::Dir>> Shape<G> for Mapped<G, T, M> {
    fn shape_prefix(cfg: &Config) -> String {
        format!("shape_{}", Self::type_prefix(cfg))
    }
    fn shape_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::shape_prefix(cfg)))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(M::map_source(cfg))
            .tree(T::shape_source(cfg))
            .content(&include(&format!(
                "geometry/ray_{}.hh",
                &G::geometry_prefix(cfg)
            )))
            .tree(RayMap::<G, M>::source(cfg))
            .content(&include_template!(
                "shape/mapped.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::shape_prefix(cfg),
                "self_data": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::geometry_prefix(cfg),
                "Map": &M::type_name(cfg),
                "map": &M::map_prefix(cfg),
                "Shape": &T::type_name(cfg),
                "shape": &T::shape_prefix(cfg),
            ))
            .build()
    }
}
