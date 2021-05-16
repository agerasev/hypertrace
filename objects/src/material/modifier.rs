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

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity, EntitySource)]
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

impl<M: Material> Material for Colored<M>
where
    Self: Entity,
{
    fn material_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/colored_{}.hh", Self::tag()))
            .tree(Self::data_source(cfg))
            .tree(M::material_source(cfg))
            .content(&include_template!(
                "material/colored.inl",
                "Self": &Self::name(),
                "self": &Self::material_prefix(),
                "Material": &M::name(),
                "material": &M::material_prefix(),
            ))
            .build()
    }
}

// Emissive

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity, EntitySource)]
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

impl<M: Material> Material for Emissive<M>
where
    Self: Entity,
{
    fn material_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/emissive_{}.hh", Self::tag()))
            .tree(Self::data_source(cfg))
            .tree(M::material_source(cfg))
            .content(&include_template!(
                "material/emissive.inl",
                "Self": &Self::name(),
                "self": &Self::material_prefix(),
                "Material": &M::name(),
                "material": &M::material_prefix(),
            ))
            .build()
    }
}
