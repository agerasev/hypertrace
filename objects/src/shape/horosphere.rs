use super::*;

/// A hyperbolic horosphere.
/// ```compile_fail
/// use ccgeom::Flat3;
/// use hypertrace_objects::{Shape, shape::Horosphere};
/// let _ = <Horosphere as Shape<Flat3>>::shader();
/// ```
#[derive(Clone, Default, Debug)]
pub struct Horosphere;

pub fn horosphere_schema() -> ShapeModule<ccgeom::Hyperboloid3> {
    ShapeModule::new(
        "hypertrace.shape.horosphere",
        include_str!("shaders/horosphere.wgsl"),
        Some(1),
    )
}

impl Shape<ccgeom::Hyperboloid3> for Horosphere {
    fn shader() -> Result<ShapeModule<ccgeom::Hyperboloid3>> {
        Ok(horosphere_schema())
    }
    fn encode(&self) -> Result<ShapeValue<ccgeom::Hyperboloid3>> {
        ShapeValue::new(<Self as Shape<ccgeom::Hyperboloid3>>::shader()?, vec![0])
    }
}

pub fn horosphere() -> ShapeValue<ccgeom::Hyperboloid3> {
    <Horosphere as Shape<ccgeom::Hyperboloid3>>::encode(&Horosphere)
        .expect("valid built-in horosphere")
}
