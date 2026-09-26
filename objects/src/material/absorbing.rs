use crate::Material;

#[derive(Clone, Copy, Debug)]
pub struct Absorbing;

impl Material for Absorbing {
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Ok(crate::wgsl::MaterialSchema::Absorbing)
    }

    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        Ok(crate::wgsl::MaterialValue::absorbing())
    }
}
