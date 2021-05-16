use crate::{
    include_template,
    source::{SourceBuilder, SourceTree},
    Config, Geometry, Map, EntityId, EntitySource,
};
use std::marker::PhantomData;

pub struct RayMap<G: Geometry, M: Map<G::Pos, G::Dir>>(PhantomData<(G, M)>);

impl<G: Geometry, M: Map<G::Pos, G::Dir>> EntityId for RayMap<G, M> {
    fn name() -> String {
        "".into()
    }
    fn data_prefix() -> String {
        "".into()
    }
}

impl<G: Geometry, M: Map<G::Pos, G::Dir>> EntitySource for RayMap<G, M> {
    fn data_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/ray_map_{}.hh", Self::tag()))
            .tree(G::data_source(cfg))
            .tree(M::data_source(cfg))
            .content(&include_template!(
                "geometry/ray_map.inl",
                "Geo": &G::name(),
                "geo": &G::data_prefix(),
                "Map": &M::name(),
                "map": &M::data_prefix(),
            ))
            .build()
    }
}
