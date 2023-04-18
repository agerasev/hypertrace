use crate::{Mapped, Object};
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config, Map, RayMap,
};

impl<G: Geometry, T: Object<G>, M: Map<G::Pos, G::Dir>> Object<G> for Mapped<G, T, M> {
    type Cache = T::Cache;
    fn object_name() -> (String, String) {
        (
            format!("Object{}", Self::name().0),
            format!("object_{}", Self::name().1),
        )
    }
    fn object_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::object_name().1))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(M::map_source(cfg))
            .tree(T::object_source(cfg))
            .content(&include(format!(
                "geometry/ray_{}.hh",
                &G::geometry_name().1
            )))
            .tree(RayMap::<G, M>::source(cfg))
            .content(&include_template!(
                "object/mapped.inl",
                ("Self", "self") => Self::object_name(),
                ("Base", "base") => Self::name(),
                ("Geo", "geo") => G::geometry_name(),
                ("Map", "map") => M::map_name(),
                ("Object", "object") => T::object_name(),
                ("Cache", "cache") => T::Cache::name(),
            ))
            .build()
    }
}
