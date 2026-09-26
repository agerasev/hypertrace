use super::*;
use ccgeom::{Euclidean3, Hyperbolic3};

#[derive(Clone, Default, Debug)]
pub struct Plane;

impl Shape<Euclidean3> for Plane {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Plane)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Ok(crate::wgsl::ShapeValue::plane())
    }
}

impl Shape<Hyperbolic3> for Plane {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Plane)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Ok(crate::wgsl::ShapeValue::plane())
    }
}
