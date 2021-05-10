use crate::{Background, Object, View};
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{SourceBuilder, SourceTree},
    Config,
};

pub trait Scene<G: Geometry>: Named + Entity + Sourced {}

#[derive(Clone, Debug, Named, Entity, SizedEntity)]
pub struct SceneImpl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>> {
    geometry: PhantomData<G>,
    #[getter]
    pub view: V,
    #[getter]
    pub background: B,
    #[getter]
    pub object: T,
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>> SceneImpl<G, V, T, B> {
    pub fn new(view: V, object: T, background: B) -> Self {
        Self {
            view,
            background,
            object,
            geometry: PhantomData,
        }
    }
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>> Sourced for SceneImpl<G, V, T, B>
where
    Self: Entity,
{
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/scene_{}.hh", Self::type_tag()))
            .tree(Self::type_source(cfg))
            .tree(G::source(cfg))
            .tree(V::source(cfg))
            .tree(T::source(cfg))
            .tree(B::source(cfg))
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
                "Background": &B::type_name(cfg),
                "background": &B::type_prefix(cfg),
            ))
            .build()
    }
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>> Scene<G> for SceneImpl<G, V, T, B> where
    Self: Entity
{
}
