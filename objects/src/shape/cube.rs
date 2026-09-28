use super::*;

#[derive(Clone, Default, Debug)]
pub struct Cube;

impl<G: Geometry> Shape<G> for Cube {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        if crate::wgsl::geometry::<G>()? != crate::wgsl::Geometry::Euclidean {
            return Err(crate::wgsl::unsupported::<(Self, G)>());
        }
        Ok(crate::wgsl::ShapeSchema::Cube)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        <Self as Shape<G>>::wgsl_shape_schema()?;
        Ok(crate::wgsl::ShapeValue::cube())
    }
}
