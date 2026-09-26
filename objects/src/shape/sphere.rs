use super::*;
use ccgeom::Euclidean3;

#[derive(Clone, Default, Debug)]
pub struct Sphere;

impl Shape<Euclidean3> for Sphere {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Sphere)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Ok(crate::wgsl::ShapeValue::sphere())
    }
}
