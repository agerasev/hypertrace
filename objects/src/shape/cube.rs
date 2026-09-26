use super::*;
use ccgeom::Euclidean3;

#[derive(Clone, Default, Debug)]
pub struct Cube;

impl Shape<Euclidean3> for Cube {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Cube)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Ok(crate::wgsl::ShapeValue::cube())
    }
}
