use crate::View;
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config, Map,
};

#[derive(Clone, Debug, Named, Entity, SizedEntity)]
pub struct MappedView<G: Geometry, V: View<G>, M: Map<G::Pos, G::Dir>> {
    pub inner: V,
    pub map: M,
    geometry: PhantomData<G>,
}

impl<G: Geometry, V: View<G>, M: Map<G::Pos, G::Dir>> MappedView<G, V, M> {
    pub fn new(inner: V, map: M) -> Self {
        Self {
            inner,
            map,
            geometry: PhantomData,
        }
    }
}

impl<G: Geometry, V: View<G>, M: Map<G::Pos, G::Dir>> Sourced for MappedView<G, V, M> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/mapped_view_{}.hh", Self::type_tag()))
            .tree(Self::type_source(cfg))
            .tree(G::source(cfg))
            .tree(M::source(cfg))
            .tree(V::source(cfg))
            .content(&include(&format!(
                "geometry/ray_{}.hh",
                &G::type_prefix(cfg)
            )))
            .content(&include_template!(
                "view/mapped.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
                "Map": &M::type_name(cfg),
                "map": &M::type_prefix(cfg),
                "View": &V::type_name(cfg),
                "view": &V::type_prefix(cfg),
            ))
            .build()
    }
}

impl<G: Geometry, V: View<G>, M: Map<G::Pos, G::Dir>> View<G> for MappedView<G, V, M> {}
