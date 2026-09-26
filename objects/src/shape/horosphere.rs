use super::*;
use ccgeom::Hyperbolic3;

#[derive(Clone, Default, Debug)]
pub struct Horosphere;

impl Shape<Hyperbolic3> for Horosphere {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Horosphere)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Ok(crate::wgsl::ShapeValue::horosphere())
    }
}
