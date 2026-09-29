use super::*;
#[derive(Clone, Copy, Debug)]
pub struct Lambertian;
impl<G: Geometry> Material<G> for Lambertian {
    fn shader() -> Result<MaterialModule<G>> {
        Ok(MaterialModule::new(
            "hypertrace.material.lambertian",
            include_str!("shaders/lambertian.wgsl"),
            Some(0),
        ))
    }
    fn encode(&self) -> Result<MaterialValue<G>> {
        MaterialValue::new(<Self as Material<G>>::shader()?, vec![])
    }
}
pub fn lambertian<G: Geometry>() -> MaterialValue<G> {
    <Lambertian as Material<G>>::encode(&Lambertian).expect("valid built-in material")
}
