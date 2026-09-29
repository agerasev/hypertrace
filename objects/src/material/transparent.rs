use super::*;
#[derive(Clone, Copy, Debug)]
pub struct Transparent;
impl<G: Geometry> Material<G> for Transparent {
    fn shader() -> Result<MaterialModule<G>> {
        Ok(MaterialModule::new(
            "hypertrace.material.transparent",
            include_str!("shaders/transparent.wgsl"),
            Some(0),
        ))
    }
    fn encode(&self) -> Result<MaterialValue<G>> {
        MaterialValue::new(<Self as Material<G>>::shader()?, vec![])
    }
}
pub fn transparent<G: Geometry>() -> MaterialValue<G> {
    <Transparent as Material<G>>::encode(&Transparent).expect("valid built-in material")
}
