use super::*;
#[derive(Clone, Copy, Debug)]
pub struct Specular;
impl Material for Specular {
    fn shader() -> Result<ShaderModule> {
        Ok(ShaderModule::new(
            "hypertrace.material.specular",
            ShaderKind::Material,
            include_str!("shaders/specular.wgsl"),
            Some(0),
        ))
    }
    fn encode(&self) -> Result<MaterialValue> {
        MaterialValue::new(Self::shader()?, vec![])
    }
}
pub fn specular() -> MaterialValue {
    Specular.encode().expect("valid built-in material")
}
