use crate::{Background, Object, View};
use ccgeom::Geometry3;
use std::marker::PhantomData;
use type_macros::{Entity, Named, SizedEntity};
use types::{
    include_template,
    source::{SourceBuilder, SourceTree},
    Config, Entity, Named, Sourced,
};

pub trait Scene<G: Geometry3>: Named + Entity + Sourced {}

#[derive(Clone, Debug, Named, Entity, SizedEntity)]
pub struct SceneImpl<G: Geometry3 + Sourced, V: View<G>, T: Object<G>, B: Background<G>> {
    geometry: PhantomData<G>,
    #[getter]
    pub view: V,
    #[getter]
    pub background: B,
    #[getter]
    pub object: T,
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Object<G>, B: Background<G>> SceneImpl<G, V, T, B> {
    pub fn new(view: V, object: T, background: B) -> Self {
        Self {
            view,
            background,
            object,
            geometry: PhantomData,
        }
    }
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Object<G>, B: Background<G>> Sourced
    for SceneImpl<G, V, T, B>
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

impl<G: Geometry3 + Sourced, V: View<G>, T: Object<G>, B: Background<G>> Scene<G>
    for SceneImpl<G, V, T, B>
where
    Self: Entity,
{
}
