use super::*;

/// A Euclidean cube. Unsupported geometries are rejected by the type system.
/// ```compile_fail
/// use ccgeom::Spherical3;
/// use hypertrace_objects::{Shape, shape::Cube};
/// let _ = <Cube as Shape<Spherical3>>::shader();
/// ```
#[derive(Clone, Default, Debug)]
pub struct Cube;

pub fn cube_schema() -> ShapeModule<ccgeom::Flat3> {
    ShapeModule::new(
        "hypertrace.shape.cube",
        include_str!("shaders/cube.wgsl"),
        Some(1),
    )
}

impl Shape<ccgeom::Flat3> for Cube {
    fn shader() -> Result<ShapeModule<ccgeom::Flat3>> {
        Ok(cube_schema())
    }
    fn encode(&self) -> Result<ShapeValue<ccgeom::Flat3>> {
        ShapeValue::new(<Self as Shape<ccgeom::Flat3>>::shader()?, vec![0])
    }
}

pub fn cube() -> ShapeValue<ccgeom::Flat3> {
    <Cube as Shape<ccgeom::Flat3>>::encode(&Cube).expect("valid built-in cube")
}
