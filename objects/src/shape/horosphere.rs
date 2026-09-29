use super::*;

#[derive(Clone, Default, Debug)]
pub struct Horosphere;

pub fn horosphere_schema() -> ShaderModule {
    #[allow(unused_mut)]
    let mut module = ShaderModule::new(
        "hypertrace.shape.horosphere",
        ShaderKind::Shape,
        include_str!("shaders/horosphere.wgsl"),
        Some(1),
    );
    module.validate_context = validate_context;
    module
}

fn validate_context(ctx: crate::shader::GeometryContext) -> crate::shader::Result<()> {
    anyhow::ensure!(
        ctx.geometry == crate::shader::Geometry::Hyperbolic,
        "horosphere requires hyperbolic geometry"
    );
    Ok(())
}

impl<G: crate::shader::RenderGeometry> Shape<G> for Horosphere {
    fn shader() -> Result<ShaderModule> {
        validate_context(crate::shader::GeometryContext {
            geometry: crate::shader::geometry::<G>()?,
            radius: 1.0,
        })?;
        Ok(horosphere_schema())
    }
    fn encode(&self) -> Result<ShapeValue> {
        ShapeValue::new(<Self as Shape<G>>::shader()?, vec![0])
    }
}

pub fn horosphere() -> ShapeValue {
    <Horosphere as Shape<ccgeom::Hyperbolic3>>::encode(&Horosphere)
        .expect("valid built-in horosphere")
}
