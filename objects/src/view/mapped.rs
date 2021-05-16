use crate::{Mapped, View};
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config, Map,
};

impl<G: Geometry, T: View<G>, M: Map<G::Pos, G::Dir>> View<G> for Mapped<G, T, M>{
    fn view_prefix(cfg: &Config) -> String {
        format!("view_{}", Self::type_prefix(cfg))
    }
    fn view_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::type_prefix(cfg)))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(M::map_source(cfg))
            .tree(T::view_source(cfg))
            .content(&include(&format!(
                "geometry/ray_{}.hh",
                &G::geometry_prefix(cfg)
            )))
            .content(&include_template!(
                "view/mapped.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::view_prefix(cfg),
                "self_data": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::geometry_prefix(cfg),
                "Map": &M::type_name(cfg),
                "map": &M::map_prefix(cfg),
                "View": &T::type_name(cfg),
                "view": &T::view_prefix(cfg),
            ))
            .build()
    }
}
