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
    #[serde(deserialize_with = "super::named_colors")]
    pub colors: StarColors,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StarColors {
    pub start: [f32; 3],
    pub end: [f32; 3],
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
        validate_rgb(self.colors.start).context("colors.start")?;
        validate_rgb(self.colors.end).context("colors.end")?;
        Ok(())
    }
}
