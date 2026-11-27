use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer, de::Error};

use crate::config::validate_rgb;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Material {
    #[serde(deserialize_with = "weighted_color")]
    pub reflection: [f32; 3],
    #[serde(deserialize_with = "weighted_color")]
    pub transmission: [f32; 3],
    #[serde(deserialize_with = "weighted_color")]
    pub emission: [f32; 3],
}

fn weighted_color<'de, D: Deserializer<'de>>(deserializer: D) -> Result<[f32; 3], D::Error> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Term {
        weight: f32,
        color: Option<[f32; 3]>,
    }

    let Term { weight, color } = Term::deserialize(deserializer)?;
    if !weight.is_finite() || weight < 0.0 {
        return Err(D::Error::custom("weight must be finite and nonnegative"));
    }
    let color = color.unwrap_or([1.0; 3]);
    validate_rgb(color).map_err(D::Error::custom)?;
    let weighted = color.map(|channel| weight * channel);
    if !weighted.iter().all(|v| v.is_finite()) {
        return Err(D::Error::custom(
            "weight times color exceeds the shader range",
        ));
    }
    Ok(weighted)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terms_have_independent_weighted_colors_and_omitted_terms_are_black() -> Result<()> {
        let mixed: Material = toml::from_str(
            "reflection = { weight = 0.5, color = [0.25, 1, 0] }
             emission = { weight = 2, color = [2, 0, 8] }",
        )?;
        assert_eq!(mixed.reflection, [0.125, 0.5, 0.0]);
        assert_eq!(mixed.transmission, [0.0; 3]);
        assert_eq!(mixed.emission, [4.0, 0.0, 16.0]);

        let black: Material = toml::from_str("")?;
        assert_eq!(black.reflection, [0.0; 3]);
        assert_eq!(black.transmission, [0.0; 3]);
        assert_eq!(black.emission, [0.0; 3]);
        Ok(())
    }

    #[test]
    fn malformed_terms_and_unknown_keys_are_rejected() {
        for invalid in [
            "transmission = 0.22",
            "emission = [1, 1, 1]",
            "reflection = { color = [1, 1, 1] }",
            "reflection = {}",
            "reflection = { weight = 1, color = [1, 1, 1], intensity = 2 }",
            "emisson = { weight = 1, color = [0, 1, 1] }",
            "kind = 'emissive'",
        ] {
            assert!(toml::from_str::<Material>(invalid).is_err(), "{invalid}");
        }
    }

    #[test]
    fn omitted_color_has_unit_response_in_every_channel() -> Result<()> {
        let implicit: Material = toml::from_str("transmission = { weight = 0.22 }")?;
        let white: Material = toml::from_str("transmission = { weight = 0.22, color = [1,1,1] }")?;
        assert_eq!(implicit.transmission, white.transmission);
        assert_eq!(implicit.transmission, [0.22; 3]);
        Ok(())
    }

    #[test]
    fn zero_weights_and_colors_do_not_mask_invalid_inputs() {
        for term in [
            "{ weight = -1, color = [0, 0, 0] }",
            "{ weight = nan, color = [0, 0, 0] }",
            "{ weight = 0, color = [-1, 0, 0] }",
            "{ weight = 0, color = [inf, 0, 0] }",
            "{ weight = 2, color = [2e38, 0, 0] }",
        ] {
            let error = toml::from_str::<Material>(&format!("transmission = {term}")).unwrap_err();
            assert!(error.to_string().contains("transmission"), "{error}");
        }
    }

    #[test]
    fn invalid_runtime_coefficients_identify_the_term() {
        let rgb = Material {
            transmission: [0.0, f32::NAN, 0.0],
            ..Material::default()
        };
        assert_eq!(rgb.validate().unwrap_err().to_string(), "transmission");
    }
}
