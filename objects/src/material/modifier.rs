use crate::Material;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{SourceBuilder, SourceTree},
    Config,
};
use vecmat::Vector;

// Colored

#[derive(Clone, Copy, Debug, Named, Entity, SizedEntity)]
pub struct Colored<M: Material> {
    pub color: Vector<f32, 3>,
    #[getter]
    pub inner: M,
}

impl<M: Material> Colored<M> {
    pub fn new(material: M, color: Vector<f32, 3>) -> Self {
        Self {
            inner: material,
            color,
        }
    }
}

impl<M: Material> Sourced for Colored<M>
where
    Self: Entity,
{
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/colored_{}.hh", Self::type_tag()))
            .tree(Self::type_source(cfg))
            .tree(M::source(cfg))
            .content(&include_template!(
                "material/colored.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Material": &M::type_name(cfg),
                "material": &M::type_prefix(cfg),
            ))
            .build()
    }
}

impl<M: Material> Material for Colored<M> {}

// Emissive

#[derive(Clone, Copy, Debug, Named, Entity, SizedEntity)]
pub struct Emissive<M: Material> {
    pub emission: Vector<f32, 3>,
    #[getter]
    pub inner: M,
}

impl<M: Material> Emissive<M> {
    pub fn new(material: M, emission: Vector<f32, 3>) -> Self {
        Self {
            inner: material,
            emission,
        }
    }
}

impl<M: Material> Sourced for Emissive<M>
where
    Self: Entity,
{
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/emissive_{}.hh", Self::type_tag()))
            .tree(Self::type_source(cfg))
            .tree(M::source(cfg))
            .content(&include_template!(
                "material/emissive.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Material": &M::type_name(cfg),
                "material": &M::type_prefix(cfg),
            ))
            .build()
    }
}

impl<M: Material> Material for Emissive<M> {}
