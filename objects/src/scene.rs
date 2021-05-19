use crate::{Background, Object, View};
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{SourceBuilder, SourceTree},
    Config,
};

pub trait Scene<G: Geometry>: EntityId + Entity + EntitySource {
    fn scene_name() -> (String, String) {
        Self::name()
    }
    fn scene_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

#[derive(Clone, Debug, EntityId, Entity, SizedEntity, EntitySource)]
pub struct SceneImpl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize> {
    geometry: PhantomData<G>,
    #[getter]
    pub view: V,
    #[getter]
    pub background: B,
    #[getter]
    pub object: T,
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize>
    SceneImpl<G, V, T, B, H>
{
    pub fn new(view: V, object: T, background: B) -> Self {
        Self {
            view,
            background,
            object,
            geometry: PhantomData,
        }
    }
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize> Scene<G>
    for SceneImpl<G, V, T, B, H>
where
    Self: Entity,
{
    fn scene_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::scene_name().1))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(V::view_source(cfg))
            .tree(T::object_source(cfg))
            .tree(B::background_source(cfg))
            .content(&include_template!(
                "render/scene.inl",
                ("Self", "self") => Self::scene_name(),
                ("Geo", "geo") => G::geometry_name(),
                ("View", "view") => V::view_name(),
                ("Object", "object") => T::object_name(),
                ("Background", "background") => B::background_name(),
                "light_hops" => format!("{}", H),
            ))
            .build()
    }
}
