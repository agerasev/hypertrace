use crate::Object;
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config,
};

#[derive(Clone, Debug, Default, Entity)]
pub struct ObjectVector<G: Geometry, T: Object<G>> {
    geometry: PhantomData<G>,
    pub objects: Vec<T>,
}

impl<G: Geometry, T: Object<G>> ObjectVector<G, T> {
    pub fn new() -> Self {
        Self {
            geometry: PhantomData,
            objects: Vec::new(),
        }
    }
    pub fn from_objects(objects: Vec<T>) -> Self {
        Self {
            geometry: PhantomData,
            objects,
        }
    }
}

impl<G: Geometry, T: Object<G>> Named for ObjectVector<G, T> {
    fn type_name(_: &Config) -> String {
        format!("ObjectVector{}", Self::type_tag())
    }
    fn type_prefix(_: &Config) -> String {
        format!("object_vector_{}", Self::type_tag())
    }
}

impl<G: Geometry, T: Object<G>> Sourced for ObjectVector<G, T>
where
    Self: Entity,
{
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::type_prefix(cfg)))
            .tree(Self::type_source(cfg))
            .tree(G::source(cfg))
            .tree(T::source(cfg))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::type_prefix(cfg)
            )))
            .content(&include_template!(
                "object/vector.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
                "Object": &T::type_name(cfg),
                "object": &T::type_prefix(cfg),
            ))
            .build()
    }
}

impl<G: Geometry, T: Object<G>> Object<G> for ObjectVector<G, T> where Self: Entity {}
