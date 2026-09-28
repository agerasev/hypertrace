use super::*;

#[derive(Clone, Default, Debug)]
pub struct Plane;

impl<G: Geometry> Shape<G> for Plane {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        crate::wgsl::geometry::<G>()?;
        Ok(crate::wgsl::ShapeSchema::Plane)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        crate::wgsl::geometry::<G>()?;
        Ok(crate::wgsl::ShapeValue::plane())
    }
}
