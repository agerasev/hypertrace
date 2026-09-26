use crate::Material;
use base::vecmat::Vector;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{SourceBuilder, SourceTree},
    Config,
};

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
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Ok(crate::wgsl::MaterialSchema::Colored(Box::new(
            M::wgsl_material_schema()?,
        )))
    }

    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        self.inner.wgsl_material()?.colored(self.color.into_array())
    }

    fn material_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::material_name().1))
            .tree(Self::source(cfg))
            .tree(M::material_source(cfg))
            .content(&include_template!(
                "material/colored.inl",
                ("Self", "self") => Self::material_name(),
                ("Material", "material") => M::material_name(),
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
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Ok(crate::wgsl::MaterialSchema::Emissive(Box::new(
            M::wgsl_material_schema()?,
        )))
    }

    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        self.inner
            .wgsl_material()?
            .emissive(self.emission.into_array())
    }

    fn material_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::material_name().1))
            .tree(Self::source(cfg))
            .tree(M::material_source(cfg))
            .content(&include_template!(
                "material/emissive.inl",
                ("Self", "self") => Self::material_name(),
                ("Material", "material") => M::material_name(),
            ))
            .build()
    }
}
