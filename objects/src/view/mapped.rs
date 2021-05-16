use crate::{Mapped, View};
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config, Map,
};

impl<G: Geometry, T: View<G>, M: Map<G::Pos, G::Dir>> View<G> for Mapped<G, T, M>{
    fn view_prefix() -> String {
        format!("view_{}", Self::data_prefix())
    }
    fn view_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::data_prefix()))
            .tree(Self::data_source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(M::map_source(cfg))
            .tree(T::view_source(cfg))
            .content(&include(&format!(
                "geometry/ray_{}.hh",
                &G::geometry_prefix()
            )))
            .content(&include_template!(
                "view/mapped.inl",
                "Self": &Self::name(),
                "self": &Self::view_prefix(),
                "self_data": &Self::data_prefix(),
                "Geo": &G::name(),
                "geo": &G::geometry_prefix(),
                "Map": &M::name(),
                "map": &M::map_prefix(),
                "View": &T::name(),
                "view": &T::view_prefix(),
            ))
            .build()
    }
}
