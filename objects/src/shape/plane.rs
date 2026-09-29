use super::*;

#[derive(Clone, Default, Debug)]
pub struct Plane;

pub fn plane_schema() -> ShaderModule {
    #[allow(unused_mut)]
    let mut module = ShaderModule::new(
        "hypertrace.shape.plane",
        ShaderKind::Shape,
        include_str!("shaders/plane.wgsl"),
        Some(1),
    );
    module
}

impl<G: crate::shader::RenderGeometry> Shape<G> for Plane {
    fn shader() -> Result<ShaderModule> {
        crate::shader::geometry::<G>()?;
        Ok(plane_schema())
    }
    fn encode(&self) -> Result<ShapeValue> {
        ShapeValue::new(<Self as Shape<G>>::shader()?, vec![0])
    }
}

pub fn plane() -> ShapeValue {
    <Plane as Shape<ccgeom::Euclidean3>>::encode(&Plane).expect("valid built-in plane")
}
