use super::*;

#[derive(Clone, Default, Debug)]
pub struct Cube;

pub fn cube_schema() -> ShaderModule {
    #[allow(unused_mut)]
    let mut module = ShaderModule::new(
        "hypertrace.shape.cube",
        ShaderKind::Shape,
        include_str!("shaders/cube.wgsl"),
        Some(1),
    );
    module.validate_context = validate_context;
    module
}

fn validate_context(ctx: crate::shader::GeometryContext) -> crate::shader::Result<()> {
    anyhow::ensure!(
        ctx.geometry == crate::shader::Geometry::Euclidean,
        "cube requires euclidean geometry"
    );
    Ok(())
}

impl<G: crate::shader::RenderGeometry> Shape<G> for Cube {
    fn shader() -> Result<ShaderModule> {
        validate_context(crate::shader::GeometryContext {
            geometry: crate::shader::geometry::<G>()?,
            radius: 1.0,
        })?;
        Ok(cube_schema())
    }
    fn encode(&self) -> Result<ShapeValue> {
        ShapeValue::new(<Self as Shape<G>>::shader()?, vec![0])
    }
}

pub fn cube() -> ShapeValue {
    <Cube as Shape<ccgeom::Euclidean3>>::encode(&Cube).expect("valid built-in cube")
}
