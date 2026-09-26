use crate::Material;

#[derive(Clone, Copy, Debug)]
pub struct Lambertian;

impl Material for Lambertian {
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Ok(crate::wgsl::MaterialSchema::Lambertian)
    }

    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        Ok(crate::wgsl::MaterialValue::lambertian())
    }
}
