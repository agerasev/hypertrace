use anyhow::{Context, Result, ensure};
use serde::Deserialize;

use super::validate_rgb;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StarField {
    pub density: f64,
    pub seed: u64,
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
    pub(crate) fn mean_per_tile(&self) -> f64 {
        // The area of one right-angled pentagon is π/2 in square curvature radii.
        self.density * std::f64::consts::FRAC_PI_2
    }

    pub fn validate(&self) -> Result<()> {
        ensure!(
            (0.0..=f64::from(u32::MAX)).contains(&self.mean_per_tile()),
            "density must give a finite, nonnegative star count per pentagon"
        );
        let [near, far] = self.gap;
        ensure!(
            near > 0.0 && far >= near && (far.sinh() as f32).is_finite(),
            "gap must have ordered positive endpoints within the shader coordinate range"
        );
        validate_rgb(self.colors.start).context("colors.start")?;
        validate_rgb(self.colors.end).context("colors.end")?;
        Ok(())
    }
}
