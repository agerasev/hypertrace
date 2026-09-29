use super::*;
#[derive(Clone, Copy, Debug)]
pub struct Lambertian;
impl Material for Lambertian {
    fn shader() -> Result<ShaderModule> {
        Ok(ShaderModule::new(
            "hypertrace.material.lambertian",
            ShaderKind::Material,
            include_str!("shaders/lambertian.wgsl"),
            Some(0),
        ))
    }
    fn encode(&self) -> Result<MaterialValue> {
        MaterialValue::new(Self::shader()?, vec![])
    }
}
pub fn lambertian() -> MaterialValue {
    Lambertian.encode().expect("valid built-in material")
}
