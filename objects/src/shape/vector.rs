use crate::Shape;
use ccgeom::Geometry;

impl<G: Geometry, T: Shape<G>> Shape<G> for Vec<T> {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Vector(Box::new(
            T::wgsl_shape_schema()?,
        )))
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        crate::wgsl::ShapeValue::vector(
            T::wgsl_shape_schema()?,
            self.iter()
                .map(T::wgsl_shape)
                .collect::<crate::wgsl::Result<Vec<_>>>()?,
        )
    }
}
