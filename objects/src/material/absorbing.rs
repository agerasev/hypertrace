use super::*;
#[derive(Clone, Copy, Debug)]
pub struct Absorbing;
impl Material for Absorbing {
    fn shader() -> Result<ShaderModule> {
        Ok(ShaderModule::new(
            "hypertrace.material.absorbing",
            ShaderKind::Material,
            include_str!("shaders/absorbing.wgsl"),
            Some(0),
        ))
    }
    fn encode(&self) -> Result<MaterialValue> {
        MaterialValue::new(Self::shader()?, vec![])
    }
}
pub fn absorbing() -> MaterialValue {
    Absorbing.encode().expect("valid built-in material")
}
