use super::*;

#[derive(Clone, Default, Debug)]
pub struct Horosphere;

impl<G: Geometry> Shape<G> for Horosphere {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        if crate::wgsl::geometry::<G>()? != crate::wgsl::Geometry::Hyperbolic {
            return Err(crate::wgsl::unsupported::<(Self, G)>());
        }
        Ok(crate::wgsl::ShapeSchema::Horosphere)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        <Self as Shape<G>>::wgsl_shape_schema()?;
        Ok(crate::wgsl::ShapeValue::horosphere())
    }
}
