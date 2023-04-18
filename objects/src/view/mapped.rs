use crate::{Mapped, View};
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config, Map,
};

impl<G: Geometry, T: View<G>, M: Map<G::Pos, G::Dir>> View<G> for Mapped<G, T, M> {
    fn view_name() -> (String, String) {
        (
            format!("View{}", Self::name().0),
            format!("view_{}", Self::name().1),
        )
    }
    fn view_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::view_name().1))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(M::map_source(cfg))
            .tree(T::view_source(cfg))
            .content(&include(format!(
                "geometry/ray_{}.hh",
                &G::geometry_name().1
            )))
            .content(&include_template!(
                "view/mapped.inl",
                ("Self", "self") => Self::view_name(),
                ("Base", "base") => Self::name(),
                ("Geo", "geo") => G::geometry_name(),
                ("Map", "map") => M::map_name(),
                ("View", "view") => T::view_name(),
            ))
            .build()
    }
}
