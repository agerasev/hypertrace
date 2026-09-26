use crate::Material;
use vecmat::Vector;

// Colored

#[derive(Clone, Copy, Debug)]
pub struct Colored<M: Material> {
    pub color: Vector<f32, 3>,
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

impl<M: Material> Material for Colored<M> {
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Ok(crate::wgsl::MaterialSchema::Colored(Box::new(
            M::wgsl_material_schema()?,
        )))
    }

    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        self.inner.wgsl_material()?.colored(self.color.into_array())
    }
}

// Emissive

#[derive(Clone, Copy, Debug)]
pub struct Emissive<M: Material> {
    pub emission: Vector<f32, 3>,
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

impl<M: Material> Material for Emissive<M> {
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
}
