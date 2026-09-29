use super::*;
#[derive(Clone, Copy, Debug)]
pub struct Transparent;
impl Material for Transparent {
    fn shader() -> Result<ShaderModule> {
        Ok(ShaderModule::new(
            "hypertrace.material.transparent",
            ShaderKind::Material,
            include_str!("shaders/transparent.wgsl"),
            Some(0),
        ))
    }
    fn encode(&self) -> Result<MaterialValue> {
        MaterialValue::new(Self::shader()?, vec![])
    }
}
pub fn transparent() -> MaterialValue {
    Transparent.encode().expect("valid built-in material")
}
