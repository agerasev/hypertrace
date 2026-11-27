//! Runtime scene and viewer settings loaded from the requested TOML file.
use std::{fs, path::Path};

use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};

use crate::world::{MAX_SUNS, Material};

pub(crate) mod observer;
pub(crate) mod stars;
pub(crate) mod vegetation;

use observer::{Observer, View};
use stars::StarField;
use vegetation::{Species, SunVegetation, Vegetation};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    pub earth: Earth,
    pub suns: Vec<Sun>,
    pub moon: Moon,
    pub ambient: AmbientLight,
    pub atmosphere: Air,
    pub stars: StarField,
    pub observer: Observer,
    pub view: View,
    pub vegetation: Vegetation,
}

pub(crate) fn validate_rgb(rgb: [f32; 3]) -> Result<()> {
    ensure!(
        rgb.iter().all(|&v| v.is_finite() && v >= 0.0),
        "RGB components must be finite and nonnegative"
    );
    Ok(())
}

// A derived struct can also deserialize a sequence by field order. Palette
// entries must be keyed by name so changing the order cannot recolor a part.
fn named_colors<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    toml::Table::deserialize(deserializer)?
        .try_into()
        .map_err(serde::de::Error::custom)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Earth {
    pub material: Material,
    pub grid: GroundGrid,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GroundGrid {
    pub view_radius: f64,
    pub max_patch_sites: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Sun {
    pub ground_offset: [f64; 2],
    pub distance: f64,
    pub disk_radiance: [f32; 3],
    pub surface_radiance: [f32; 3],
    pub scattering_radiance: [f32; 3],
    pub ambient: SunAmbient,
    pub vegetation: SunVegetation,
}

impl Sun {
    fn validate(&self, air_height: f64) -> Result<()> {
        let origin_w =
            self.distance.cosh() * self.ground_offset[0].cosh() * self.ground_offset[1].cosh();
        ensure!(
            self.distance.cosh() > 1.0 && self.distance > 0.0 && (origin_w as f32).is_finite(),
            "distance must be positive and the ground_offset and distance must be representable"
        );
        validate_rgb(self.disk_radiance).context("disk_radiance")?;
        validate_rgb(self.surface_radiance).context("surface_radiance")?;
        validate_rgb(self.scattering_radiance).context("scattering_radiance")?;
        ensure!(
            self.scattering_radiance == [0.0; 3] || self.distance.cosh() > air_height.cosh(),
            "a scattering Sun must have distance above atmosphere.height"
        );
        self.ambient.validate().context("ambient")?;
        self.vegetation.validate().context("vegetation")
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Moon {
    pub ground_offset: [f64; 2],
    pub distance: f64,
    pub subdivisions: u32,
    pub material: Material,
}

/// Diffuse fill on a white receiver, in linear RGB.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AmbientLight {
    pub night: [f32; 3],
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SunAmbient {
    pub color: [f32; 3],
    pub response_factor: f32,
    pub response_exponent: f32,
}

impl SunAmbient {
    fn validate(&self) -> Result<()> {
        validate_rgb(self.color).context("color")?;
        ensure!(
            self.response_factor.is_finite() && self.response_factor >= 0.0,
            "response_factor must be finite and nonnegative"
        );
        ensure!(
            self.response_exponent.is_finite() && self.response_exponent > 0.0,
            "response_exponent must be positive and finite"
        );
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Air {
    pub height: f64,
    pub background: [f32; 3],
}

impl Config {
    pub(crate) fn candidate_densities(&self) -> [f64; 3] {
        Species::ALL.map(|species| {
            self.suns
                .iter()
                .filter_map(|sun| species.profile(&sun.vegetation))
                .map(|profile| profile.peak_density)
                .fold(species.ecology(&self.vegetation).night_density, f64::max)
        })
    }

    /// Upper bound before stochastic rounding and plant occupancy sampling.
    pub(crate) fn max_sites_per_tile(&self) -> u64 {
        let plants: u64 = self
            .candidate_densities()
            .into_iter()
            .map(|density| (density * std::f64::consts::FRAC_PI_2).ceil() as u64)
            .sum();
        plants + self.stars.mean_per_tile().ceil() as u64
    }

    pub fn load(path: &Path) -> Result<Self> {
        let source = fs::read_to_string(path)
            .with_context(|| format!("reading configuration {}", path.display()))?;
        Self::parse(&source).with_context(|| format!("invalid configuration {}", path.display()))
    }

    fn parse(source: &str) -> Result<Self> {
        let config: Self = toml::from_str(source)?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        self.earth.material.validate().context("earth.material")?;
        let air_height = self.atmosphere.height.sinh() as f32;
        ensure!(
            air_height > 0.0 && air_height.is_finite(),
            "atmosphere.height must be positive and representable"
        );
        validate_rgb(self.atmosphere.background).context("atmosphere.background")?;
        validate_rgb(self.ambient.night).context("ambient.night")?;
        ensure!(
            self.suns.len() <= MAX_SUNS,
            "suns supports up to {MAX_SUNS} entries"
        );
        for (i, sun) in self.suns.iter().enumerate() {
            sun.validate(self.atmosphere.height)
                .with_context(|| format!("suns[{i}]"))?;
        }
        self.moon.material.validate().context("moon.material")?;
        ensure!(
            self.moon.distance > 0.0
                && (self.moon.distance.cosh()
                    * self.moon.ground_offset[0].cosh()
                    * self.moon.ground_offset[1].cosh())
                .is_finite(),
            "invalid moon.distance or moon.ground_offset"
        );
        crate::world::moon::HexGrid::new(self.moon.subdivisions)?;
        self.observer.validate().context("observer")?;
        ensure!(
            self.observer.height + self.observer.jump_height < self.moon.distance,
            "moon.distance must exceed observer.height + observer.jump_height to keep the lunar mesh edge out of sight"
        );
        self.view.validate().context("view")?;
        self.vegetation.validate().context("vegetation")?;
        self.stars.validate().context("stars")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORLD: &str = include_str!("../config.toml");

    #[test]
    fn shared_world_settings_require_all_fields_except_material_terms_and_colors() -> Result<()> {
        Config::parse(WORLD)?;
        let table: toml::Table = toml::from_str(WORLD)?;
        fn check_fields(
            table: &toml::Table,
            whole: &toml::Table,
            path: &mut Vec<String>,
        ) -> Result<()> {
            for (key, value) in table {
                let mut incomplete = whole.clone();
                let mut parent = &mut incomplete;
                for component in path.iter() {
                    parent = parent[component].as_table_mut().unwrap();
                }
                parent.remove(key);
                let optional = match path.as_slice() {
                    [.., table] if table == "material" => true,
                    [.., table, _term] if table == "material" => key == "color",
                    _ => false,
                };
                path.push(key.clone());
                assert_eq!(
                    Config::parse(&toml::to_string(&incomplete)?).is_ok(),
                    optional,
                    "unexpected result when omitting {}",
                    path.join(".")
                );
                if let Some(children) = value.as_table() {
                    check_fields(children, whole, path)?;
                }
                path.pop();
            }
            Ok(())
        }
        check_fields(&table, &table, &mut Vec::new())
    }

    #[test]
    fn palettes_require_named_colors() -> Result<()> {
        let world: toml::Value = toml::from_str(WORLD)?;
        for path in [
            &["vegetation", "pine", "foliage"][..],
            &["vegetation", "grass", "colors"],
            &["stars", "colors"],
        ] {
            let mut document = world.clone();
            let palette = path
                .iter()
                .fold(&mut document, |value, &key| &mut value[key]);
            let positional = palette.as_table().unwrap().values().cloned().collect();
            *palette = toml::Value::Array(positional);
            assert!(
                Config::parse(&toml::to_string(&document)?).is_err(),
                "{} accepted a positional palette",
                path.join(".")
            );
        }
        Ok(())
    }

    #[test]
    fn unknown_settings_and_incompatible_values_report_the_field() -> Result<()> {
        let mut table: toml::Table = toml::from_str(WORLD)?;
        table["suns"][0]
            .as_table_mut()
            .unwrap()
            .insert("distence".into(), 3.0.into());
        let error = Config::parse(&toml::to_string(&table)?).unwrap_err();
        assert!(error.to_string().contains("distence"));

        table["suns"][0].as_table_mut().unwrap().remove("distence");
        table["atmosphere"]["height"] = table["suns"][0]["distance"].clone();
        let error = Config::parse(&toml::to_string(&table)?).unwrap_err();
        assert!(format!("{error:#}").contains("atmosphere.height"));
        Ok(())
    }

    #[test]
    fn sun_list_allows_dark_worlds_and_checks_each_source_independently() -> Result<()> {
        let mut document: toml::Value = toml::from_str(WORLD)?;
        let source = document["suns"][0].clone();
        document["suns"] = toml::Value::Array(Vec::new());
        Config::parse(&toml::to_string(&document)?)?;
        document["suns"] = toml::Value::Array(vec![source; MAX_SUNS + 1]);
        let error = Config::parse(&toml::to_string(&document)?).unwrap_err();
        assert!(format!("{error:#}").contains("up to"));
        document["suns"].as_array_mut().unwrap().pop();
        for field in ["disk_radiance", "surface_radiance", "scattering_radiance"] {
            let mut invalid = document.clone();
            invalid["suns"][1][field][2] = (-1.0).into();
            let error = Config::parse(&toml::to_string(&invalid)?).unwrap_err();
            let message = format!("{error:#}");
            assert!(message.contains("suns[1]") && message.contains(field));
        }
        document["suns"][1]
            .as_table_mut()
            .unwrap()
            .remove("disk_radiance");
        let error = Config::parse(&toml::to_string(&document)?).unwrap_err();
        assert!(error.to_string().contains("disk_radiance"));
        Ok(())
    }

    #[test]
    fn a_sun_lists_only_the_species_it_supports() -> Result<()> {
        let mut document: toml::Value = toml::from_str(WORLD)?;
        let biome = &mut document["suns"][0]["vegetation"];
        let mut pine = biome["pine"].clone();
        *biome = toml::Value::Table(toml::Table::new());
        let config = Config::parse(&toml::to_string(&document)?)?;
        assert!(config.suns[0].vegetation.pine.is_none());
        pine.as_table_mut().unwrap().remove("max_size");
        document["suns"][0]["vegetation"]
            .as_table_mut()
            .unwrap()
            .insert("pine".into(), pine);
        let error = Config::parse(&toml::to_string(&document)?).unwrap_err();
        assert!(format!("{error:#}").contains("max_size"));
        Ok(())
    }
}
