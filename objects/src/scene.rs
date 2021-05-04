use types::{Config, Named, Entity, SizedEntity, Sourced, source::{SourceTree, SourceBuilder}, include_template};
use type_macros::{SizedEntity};
use ccgeom::Geometry3;
use crate::{View, Shape};
use std::marker::PhantomData;

#[derive(Clone, Debug, SizedEntity)]
pub struct Scene<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> {
    #[getter] pub view: V,
    #[getter] pub object: T,
    phantom: PhantomData<G>,
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> Scene<G, V, T> {
    pub fn new(view: V, shape: T) -> Self {
        Self { view, object: shape, phantom: PhantomData }
    }
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> Named for Scene<G, V, T> {
    fn type_name(_: &Config) -> String {
        format!("Scene{}", Self::type_tag())
    }
    fn type_prefix(_: &Config) -> String {
        format!("scene_{}", Self::type_tag())
    }
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> Sourced for Scene<G, V, T> where Self: Entity, T: SizedEntity {
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
