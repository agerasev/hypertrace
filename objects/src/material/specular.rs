use crate::Material;

#[derive(Clone, Copy, Debug)]
pub struct Specular;

impl Material for Specular {
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Ok(crate::wgsl::MaterialSchema::Specular)
    }

    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        Ok(crate::wgsl::MaterialValue::specular())
    }
}
