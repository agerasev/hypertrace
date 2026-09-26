use crate::Material;

#[derive(Clone, Copy, Debug)]
pub struct Transparent;

impl Material for Transparent {
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Ok(crate::wgsl::MaterialSchema::Transparent)
    }

    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        Ok(crate::wgsl::MaterialValue::transparent())
    }
}
