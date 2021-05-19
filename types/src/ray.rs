use crate::{
    include_template,
    source::{SourceBuilder, SourceTree},
    Config, Geometry, Map, EntityId, EntitySource,
};
use std::marker::PhantomData;

pub struct RayMap<G: Geometry, M: Map<G::Pos, G::Dir>>(PhantomData<(G, M)>);

impl<G: Geometry, M: Map<G::Pos, G::Dir>> EntityId for RayMap<G, M> {
    fn name() -> (String, String) {
        ("".into(), "".into())
    }
}

impl<G: Geometry, M: Map<G::Pos, G::Dir>> EntitySource for RayMap<G, M> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/ray_map_{}.hh", Self::tag()))
            .tree(G::source(cfg))
            .tree(M::source(cfg))
            .content(&include_template!(
                "geometry/ray_map.inl",
                ("Geo", "geo") => G::name(),
                ("Map", "map") => M::name(),
            ))
            .build()
    }
}
