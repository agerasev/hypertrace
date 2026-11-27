use anyhow::{Context, Result, ensure};
use serde::Deserialize;

use crate::world::Material;

use super::validate_rgb;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Vegetation {
    pub seed: u64,
    /// Candidate sites for each species per square curvature radius of Earth.
    pub site_density_per_species: f64,
    pub maturity: [f64; 2],
    pub deciduous: Deciduous,
    pub pine: Pine,
    pub grass: Grass,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LightResponse {
    pub light_range: [f64; 2],
    pub occupancy: [f64; 2],
    pub minimum_size: f64,
    pub half_saturation: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Deciduous {
    pub height: f64,
    pub bark: [f32; 3],
    pub foliage: [f32; 3],
    pub material: Material,
    pub ecology: LightResponse,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PineFoliage {
    pub lower: [f32; 3],
    pub middle: [f32; 3],
    pub upper: [f32; 3],
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Pine {
    pub height: f64,
    pub bark: [f32; 3],
    #[serde(deserialize_with = "super::named_colors")]
    pub foliage: PineFoliage,
    pub material: Material,
    pub ecology: LightResponse,
    pub daylight_range: [f64; 2],
    pub daylight_suppression: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BladeColors {
    pub root: [f32; 3],
    pub middle: [f32; 3],
    pub tip: [f32; 3],
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Grass {
    pub blades: u16,
    pub blade_height: [f64; 2],
    pub blade_width: f64,
    pub spread_radius: f64,
    #[serde(deserialize_with = "super::named_colors")]
    pub colors: BladeColors,
    pub material: Material,
    pub ecology: LightResponse,
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn fraction(value: f64) -> bool {
    (0.0..=1.0).contains(&value)
}

fn light_interval([low, high]: [f64; 2]) -> bool {
    fraction(low) && fraction(high) && low < high
}

impl LightResponse {
    fn validate(self) -> Result<()> {
        ensure!(
            light_interval(self.light_range),
            "light_range must increase within [0, 1]"
        );
        ensure!(
            self.occupancy.into_iter().all(fraction),
            "occupancy endpoints must be probabilities"
        );
        ensure!(
            fraction(self.minimum_size),
            "minimum_size must be in [0, 1]"
        );
        ensure!(
            positive(self.half_saturation),
            "half_saturation must be finite and positive"
        );
        Ok(())
    }
}

impl Vegetation {
    pub(crate) fn mean_sites_per_tile(&self) -> f64 {
        // Angle defect of a right-angled geodesic pentagon, in square curvature radii.
        self.site_density_per_species * std::f64::consts::FRAC_PI_2
    }

    pub(crate) fn validate(&self) -> Result<()> {
        ensure!(
            (0.0..=f64::from(u32::MAX)).contains(&self.mean_sites_per_tile()),
            "site_density_per_species must give a finite, nonnegative candidate count per pentagon"
        );
        let [min, max] = self.maturity;
        ensure!(
            fraction(min) && fraction(max) && min <= max,
            "maturity must be an ordered interval in [0, 1]"
        );
        for (name, value) in [
            ("deciduous.height", self.deciduous.height),
            ("pine.height", self.pine.height),
            ("grass.blade_width", self.grass.blade_width),
            ("grass.spread_radius", self.grass.spread_radius),
        ] {
            ensure!(positive(value), "{name} must be finite and positive");
        }

        for (name, material, ecology) in [
            (
                "deciduous",
                &self.deciduous.material,
                self.deciduous.ecology,
            ),
            ("pine", &self.pine.material, self.pine.ecology),
            ("grass", &self.grass.material, self.grass.ecology),
        ] {
            material
                .validate()
                .with_context(|| format!("{name}.material"))?;
            ecology
                .validate()
                .with_context(|| format!("{name}.ecology"))?;
        }
        validate_rgb(self.deciduous.bark).context("deciduous.bark")?;
        validate_rgb(self.deciduous.foliage).context("deciduous.foliage")?;
        validate_rgb(self.pine.bark).context("pine.bark")?;
        validate_rgb(self.pine.foliage.lower).context("pine.foliage.lower")?;
        validate_rgb(self.pine.foliage.middle).context("pine.foliage.middle")?;
        validate_rgb(self.pine.foliage.upper).context("pine.foliage.upper")?;
        ensure!(
            light_interval(self.pine.daylight_range),
            "pine.daylight_range must increase within [0, 1]"
        );
        ensure!(
            fraction(self.pine.daylight_suppression),
            "pine.daylight_suppression must be in [0, 1]"
        );
        let [short, tall] = self.grass.blade_height;
        ensure!(
            positive(short) && positive(tall) && short <= tall,
            "grass.blade_height must be an ordered positive finite interval"
        );
        ensure!(self.grass.blades > 0, "grass.blades must be positive");
        validate_rgb(self.grass.colors.root).context("grass.colors.root")?;
        validate_rgb(self.grass.colors.middle).context("grass.colors.middle")?;
        validate_rgb(self.grass.colors.tip).context("grass.colors.tip")?;
        Ok(())
    }
}
