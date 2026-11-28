use anyhow::{Context, Result, ensure};
use serde::Deserialize;

use crate::world::Material;

use super::validate_rgb;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Vegetation {
    pub seed: u64,
    pub deciduous: Deciduous,
    pub pine: Pine,
    pub grass: Grass,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Ecology {
    pub night_density: f64,
    pub night_size: [f64; 2],
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BiomeProfile {
    pub peak_density: f64,
    pub peak_radius: f64,
    pub half_width: f64,
    pub size: SizeProfile,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SizeProfile {
    pub peak: [f64; 2],
    pub peak_radius: f64,
    pub half_width: f64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SunVegetation {
    pub deciduous: Option<BiomeProfile>,
    pub pine: Option<BiomeProfile>,
    pub grass: Option<BiomeProfile>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Species {
    Deciduous,
    Pine,
    Grass,
}

impl Species {
    pub const ALL: [Self; 3] = [Self::Deciduous, Self::Pine, Self::Grass];

    pub fn profile(self, settings: &SunVegetation) -> Option<BiomeProfile> {
        match self {
            Self::Deciduous => settings.deciduous,
            Self::Pine => settings.pine,
            Self::Grass => settings.grass,
        }
    }

    pub fn ecology(self, settings: &Vegetation) -> Ecology {
        match self {
            Self::Deciduous => settings.deciduous.ecology,
            Self::Pine => settings.pine.ecology,
            Self::Grass => settings.grass.ecology,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Deciduous {
    pub height: f64,
    pub bark: [f32; 3],
    pub foliage: [f32; 3],
    pub material: Material,
    pub ecology: Ecology,
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
    pub ecology: Ecology,
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
    pub ecology: Ecology,
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn valid_size_range([low, high]: [f64; 2]) -> bool {
    0.0 <= low && low <= high && high <= 1.0
}

fn validate_gaussian(peak_radius: f64, half_width: f64) -> Result<()> {
    ensure!(
        peak_radius.is_finite() && peak_radius >= 0.0,
        "peak_radius must be finite and nonnegative"
    );
    ensure!(
        positive(half_width),
        "half_width must be finite and positive"
    );
    Ok(())
}

fn valid_density(density: f64) -> bool {
    (0.0..=f64::from(u32::MAX)).contains(&(density * std::f64::consts::FRAC_PI_2))
}

impl Ecology {
    fn validate(self) -> Result<()> {
        ensure!(
            valid_density(self.night_density),
            "night_density must give a finite, nonnegative count that fits a pentagon"
        );
        ensure!(
            valid_size_range(self.night_size),
            "night_size must be an ordered interval in [0, 1]"
        );
        Ok(())
    }
}

impl BiomeProfile {
    fn validate(self) -> Result<()> {
        ensure!(
            valid_density(self.peak_density),
            "peak_density must give a finite, nonnegative count that fits a pentagon"
        );
        validate_gaussian(self.peak_radius, self.half_width)?;
        ensure!(
            valid_size_range(self.size.peak),
            "size.peak must be an ordered interval in [0, 1]"
        );
        validate_gaussian(self.size.peak_radius, self.size.half_width).context("size")
    }
}

impl SunVegetation {
    pub(crate) fn validate(&self) -> Result<()> {
        for (name, profile) in [
            ("deciduous", self.deciduous),
            ("pine", self.pine),
            ("grass", self.grass),
        ] {
            if let Some(profile) = profile {
                profile.validate().context(name)?;
            }
        }
        Ok(())
    }
}

impl Vegetation {
    pub(crate) fn validate(&self) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_profiles_validate_their_own_range_radius_and_width() -> Result<()> {
        let mut biome = BiomeProfile {
            peak_density: 2.0,
            peak_radius: 1.0,
            half_width: 0.5,
            size: SizeProfile {
                peak: [0.6, 0.6],
                peak_radius: 0.0,
                half_width: 2.0,
            },
        };
        biome.validate()?;
        for range in [[0.7, 0.3], [-0.1, 0.5], [0.0, 1.1], [0.0, f64::NAN]] {
            biome.size.peak = range;
            assert!(biome.validate().is_err());
            assert!(
                Ecology {
                    night_density: 0.0,
                    night_size: range
                }
                .validate()
                .is_err()
            );
        }
        biome.size.peak = [0.0, 1.0];
        for (radius, width) in [
            (-0.1, 1.0),
            (f64::INFINITY, 1.0),
            (1.0, 0.0),
            (1.0, f64::NAN),
        ] {
            biome.size.peak_radius = radius;
            biome.size.half_width = width;
            assert!(biome.validate().is_err());
        }
        Ok(())
    }
}
