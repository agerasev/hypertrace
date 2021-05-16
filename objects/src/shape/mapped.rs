use crate::{Mapped, Shape};
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config, Map, RayMap,
};

impl<G: Geometry, T: Shape<G>, M: Map<G::Pos, G::Dir>> Shape<G> for Mapped<G, T, M> {
    fn shape_prefix() -> String {
        format!("shape_{}", Self::data_prefix())
    }
    fn shape_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::shape_prefix()))
            .tree(Self::data_source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(M::map_source(cfg))
            .tree(T::shape_source(cfg))
            .content(&include(&format!(
                "geometry/ray_{}.hh",
                &G::geometry_prefix()
            )))
            .tree(RayMap::<G, M>::data_source(cfg))
            .content(&include_template!(
                "shape/mapped.inl",
                "Self": &Self::name(),
                "self": &Self::shape_prefix(),
                "self_data": &Self::data_prefix(),
                "Geo": &G::name(),
                "geo": &G::geometry_prefix(),
                "Map": &M::name(),
                "map": &M::map_prefix(),
                "Shape": &T::name(),
                "shape": &T::shape_prefix(),
            ))
            .build()
    }
}
