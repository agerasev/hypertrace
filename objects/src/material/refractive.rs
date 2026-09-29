use crate::Material;

#[derive(Clone, Copy, Debug)]
pub struct Refractive {
    pub index: f64,
}

impl Default for Refractive {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Refractive {
    pub fn new(index: f64) -> Self {
        Self { index }
    }
}

impl Material for Refractive {
    fn shader() -> crate::shader::Result<crate::shader::ShaderModule> {
        use crate::shader::{ShaderKind, ShaderModule};
        let mut module = ShaderModule::new(
            "hypertrace.material.refractive",
            ShaderKind::Material,
            include_str!("shaders/refractive.wgsl"),
            Some(1),
        );
        module.validate_words = |_, _, words| {
            let index = f32::from_bits(words[0]);
            anyhow::ensure!(
                index.is_finite() && index > 0.0,
                "refractive index must be finite and positive"
            );
            Ok(())
        };
        Ok(module)
    }
    fn encode(&self) -> crate::shader::Result<crate::shader::MaterialValue> {
        let index = crate::shader::finite_f32(self.index)?;
        anyhow::ensure!(index > 0.0, "refractive index must be positive");
        crate::shader::MaterialValue::new(Self::shader()?, vec![index.to_bits()])
    }
}
pub fn refractive(index: f64) -> crate::shader::Result<crate::shader::MaterialValue> {
    Refractive::new(index).encode()
}
