use crate::Material;

#[derive(Clone, Copy, Debug)]
pub struct Refractive {
    pub index: f64,
}

impl Default for Refractive {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Refractive {
    pub fn new(index: f64) -> Self {
        Self { index }
    }
}

impl Material for Refractive {
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Ok(crate::wgsl::MaterialSchema::Refractive)
    }

    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        crate::wgsl::MaterialValue::refractive(self.index)
    }
}
