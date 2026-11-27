use anyhow::{Context, Result};
use serde::Deserialize;

use crate::config::validate_rgb;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Material<C = [f32; 3]> {
    pub reflection: C,
    pub transmission: C,
    pub emission: C,
}

impl Material {
    pub(crate) fn emissive(emission: [f32; 3]) -> Self {
        Self {
            emission,
            ..Self::default()
        }
    }

    pub(crate) fn validate(&self) -> Result<()> {
        validate_rgb(self.reflection).context("reflection")?;
        validate_rgb(self.transmission).context("transmission")?;
        validate_rgb(self.emission).context("emission")
    }
}

impl Material<f32> {
    pub(crate) fn achromatic(&self) -> Material {
        Material {
            reflection: [self.reflection; 3],
            transmission: [self.transmission; 3],
            emission: [self.emission; 3],
        }
    }

    pub(crate) fn validate(&self) -> Result<()> {
        self.achromatic().validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_material_preserves_independent_rgb_coefficients() -> Result<()> {
        let definition = "reflection = [0.75, 0.5, 0.25]
                          transmission = [0.5, 0.25, 0.0]
                          emission = [0.125, 0, 8]";
        let material: Material = toml::from_str(definition)?;
        material.validate()?;
        assert_eq!(material.reflection, [0.75, 0.5, 0.25]);
        assert_eq!(material.transmission, [0.5, 0.25, 0.0]);
        assert_eq!(material.emission, [0.125, 0.0, 8.0]);
        for extra in ["intensity = 2.0", "kind = 'emissive'"] {
            assert!(toml::from_str::<Material>(&format!("{definition}\n{extra}")).is_err());
        }
        Ok(())
    }

    #[test]
    fn invalid_coefficients_identify_the_term() {
        let rgb = Material {
            transmission: [0.0, f32::NAN, 0.0],
            ..Material::default()
        };
        assert_eq!(rgb.validate().unwrap_err().to_string(), "transmission");
        let scalar = Material {
            reflection: -0.1_f32,
            ..Material::default()
        };
        assert_eq!(scalar.validate().unwrap_err().to_string(), "reflection");
    }
}
