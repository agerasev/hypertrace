use types::{Config, Named, Entity, SizedEntity, Sourced, source::{SourceTree, SourceBuilder}, include_template};
use type_macros::{Named, SizedEntity};
use ccgeom::Geometry3;
use crate::{View, Object};
use std::marker::PhantomData;

#[derive(Clone, Debug, Named, SizedEntity)]
pub struct Scene<G: Geometry3 + Sourced, V: View<G>, T: Object<G>> {
    #[getter] pub view: V,
    #[getter] pub object: T,
    phantom: PhantomData<G>,
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Object<G>> Scene<G, V, T> {
    pub fn new(view: V, object: T) -> Self {
        Self { view, object, phantom: PhantomData }
    }
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Object<G>> Sourced for Scene<G, V, T> where Self: Entity, T: SizedEntity {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/scene_{}.hh", Self::type_tag()))
            .tree(Self::type_source(cfg))
            .tree(G::source(cfg))
            .tree(V::source(cfg))
            .tree(T::source(cfg))
            .content(&include_template!(
                "render/scene.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
                "View": &V::type_name(cfg),
                "view": &V::type_prefix(cfg),
                "Object": &T::type_name(cfg),
                "object": &T::type_prefix(cfg),
            ))
            .build()
    }
}
