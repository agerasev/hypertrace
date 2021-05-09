use types::{Config, Named, Entity, SizedEntity, Sourced, source::{SourceTree, SourceBuilder}, include_template};
use type_macros::{Named, SizedEntity};
use ccgeom::Geometry3;
use crate::{View, Object, Background};
use std::marker::PhantomData;

pub trait Scene<G: Geometry3>: Named + Entity + Sourced {}

#[derive(Clone, Debug, Named, SizedEntity)]
pub struct SceneImpl<
    G: Geometry3 + Sourced,
    V: View<G>,
    T: Object<G>,
    B: Background<G>,
> {
    #[skip]geometry: PhantomData<G>,
    pub view: V,
    pub background: B,
    pub object: T,
}

impl<
    G: Geometry3 + Sourced,
    V: View<G>,
    T: Object<G>,
    B: Background<G>,
> SceneImpl<G, V, T, B> {
    pub fn new(view: V, object: T, background: B) -> Self {
        Self { view, background, object, geometry: PhantomData }
    }
}

impl<
    G: Geometry3 + Sourced,
    V: View<G>,
    T: Object<G>,
    B: Background<G>,
> Sourced for SceneImpl<G, V, T, B> where Self: Entity, T: SizedEntity {
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

impl<
    G: Geometry3 + Sourced,
    V: View<G>,
    T: Object<G>,
    B: Background<G>,
> Scene<G> for SceneImpl<G, V, T, B> where Self: Entity, T: SizedEntity {}
