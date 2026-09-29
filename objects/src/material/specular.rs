use super::*;
#[derive(Clone, Copy, Debug)]
pub struct Specular;
impl<G: Geometry> Material<G> for Specular {
    fn shader() -> Result<MaterialModule<G>> {
        Ok(MaterialModule::new(
            "hypertrace.material.specular",
            include_str!("shaders/specular.wgsl"),
            Some(0),
        ))
    }
    fn encode(&self) -> Result<MaterialValue<G>> {
        MaterialValue::new(<Self as Material<G>>::shader()?, vec![])
    }
}
pub fn specular<G: Geometry>() -> MaterialValue<G> {
    <Specular as Material<G>>::encode(&Specular).expect("valid built-in material")
}
