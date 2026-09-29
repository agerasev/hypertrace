use super::*;

#[derive(Clone, Default, Debug)]
pub struct Plane;

pub fn plane_schema<G: Geometry>() -> ShapeModule<G> {
    ShapeModule::new(
        "hypertrace.shape.plane",
        include_str!("shaders/plane.wgsl"),
        Some(1),
    )
}

impl<G: Geometry> Shape<G> for Plane {
    fn shader() -> Result<ShapeModule<G>> {
        Ok(plane_schema())
    }
    fn encode(&self) -> Result<ShapeValue<G>> {
        ShapeValue::new(<Self as Shape<G>>::shader()?, vec![0])
    }
}

pub fn plane<G: Geometry>() -> ShapeValue<G> {
    <Plane as Shape<G>>::encode(&Plane).expect("valid built-in plane")
}
