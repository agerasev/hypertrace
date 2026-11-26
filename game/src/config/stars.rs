use anyhow::{Context, Result, ensure};
use serde::Deserialize;

use super::validate_rgb;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StarField {
    pub count: u32,
    pub seed: u64,
    pub ground_radius: f64,
    pub gap: [f64; 2],
    pub cool_color: [f32; 3],
    pub warm_color: [f32; 3],
    pub intensity: [f32; 2],
    pub intensity_exponent: f32,
}

impl StarField {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.ground_radius.is_finite() && self.ground_radius >= 0.0,
            "ground_radius must be finite and nonnegative"
        );
        let [near, far] = self.gap;
        ensure!(
            near > 0.0 && far >= near && ((self.ground_radius + far).sinh() as f32).is_finite(),
            "gap must have ordered positive endpoints within the shader coordinate range"
        );
        validate_rgb(self.cool_color).context("cool_color")?;
        validate_rgb(self.warm_color).context("warm_color")?;
        let [low, high] = self.intensity;
        ensure!(
            low >= 0.0 && high >= low && high.is_finite(),
            "intensity must have ordered finite nonnegative endpoints"
        );
        ensure!(
            self.cool_color
                .into_iter()
                .chain(self.warm_color)
                .all(|c| (c * high).is_finite()),
            "color at maximum intensity exceeds the shader coordinate range"
        );
        ensure!(
            self.intensity_exponent.is_finite() && self.intensity_exponent > 0.0,
            "intensity_exponent must be finite and positive"
        );
        Ok(())
    }
}
