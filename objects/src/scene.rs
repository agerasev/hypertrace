use types::{Config, Entity, SizedEntity, Sourced, source::{SourceInfo, SourceBuilder}};
use type_macros::SizedEntity;
use ccgeom::Geometry3;
use crate::{View, Shape};
use std::marker::PhantomData;

#[derive(Clone, Debug, SizedEntity)]
pub struct Scene<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> {
    pub view: V,
    pub object: T,
    phantom: PhantomData<G>,
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> Scene<G, V, T> {
    pub fn new(view: V, shape: T) -> Self {
        Self { view, object: shape, phantom: PhantomData }
    }
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> Sourced for Scene<G, V, T> where T: SizedEntity {
    fn source(cfg: &Config) -> SourceInfo {
        let src = Self::entity_source(cfg);
        let name = format!("Scene{}", Self::type_tag());
        let prefix = format!("scene_{}", Self::type_tag());
        SourceBuilder::new(format!("generated/scene_{}.hh", Self::type_tag()))
            .tree(src.tree)
            .tree(G::source(cfg).tree)
            .tree(V::source(cfg).tree)
            .tree(T::source(cfg).tree)
            .content(&format!("typedef {} {};", &src.name, &name))
            .build(name, prefix)
    }
}
