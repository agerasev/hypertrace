use super::*;
#[derive(Clone, Copy, Debug)]
pub struct Absorbing;
impl<G: Geometry> Material<G> for Absorbing {
    fn shader() -> Result<MaterialModule<G>> {
        Ok(MaterialModule::new(
            "hypertrace.material.absorbing",
            include_str!("shaders/absorbing.wgsl"),
            Some(0),
        ))
    }
    fn encode(&self) -> Result<MaterialValue<G>> {
        MaterialValue::new(<Self as Material<G>>::shader()?, vec![])
    }
}
pub fn absorbing<G: Geometry>() -> MaterialValue<G> {
    <Absorbing as Material<G>>::encode(&Absorbing).expect("valid built-in material")
}
