//! Runtime scene and viewer settings. Every field comes from the requested TOML file.
use std::{fs, path::Path};

use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use wgame::glam::Vec3;

use crate::world::{AmbientLight, Material};

pub(crate) mod observer;
pub(crate) mod stars;
pub(crate) mod vegetation;

use observer::{Observer, View};
use stars::StarField;
use vegetation::Vegetation;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    pub earth: Earth,
    pub sun: Sun,
    pub moon: Moon,
    pub ambient: Ambient,
    pub atmosphere: Air,
    pub stars: StarField,
    pub observer: Observer,
    pub view: View,
    pub vegetation: Vegetation,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Light {
    pub color: [f32; 3],
    pub intensity: f32,
}

impl Light {
    pub fn rgb(self) -> Result<[f32; 3]> {
        validate_rgb(self.color)?;
        ensure!(
            self.intensity.is_finite() && self.intensity >= 0.0,
            "intensity must be finite and nonnegative"
        );
        let rgb = self.color.map(|v| v * self.intensity);
        validate_rgb(rgb)?;
        Ok(rgb)
    }
}

pub(crate) fn validate_rgb(rgb: [f32; 3]) -> Result<()> {
    ensure!(
        rgb.iter().all(|&v| v.is_finite() && v >= 0.0),
        "RGB components must be finite and nonnegative"
    );
    Ok(())
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum MaterialConfig {
    Diffuse { color: [f32; 3] },
    Translucent { color: [f32; 3], transmission: f32 },
    Emissive { color: [f32; 3], intensity: f32 },
}

impl MaterialConfig {
    pub fn build(&self) -> Result<Material> {
        let material = match *self {
            Self::Diffuse { color } => Material::Diffuse(Vec3::from_array(color)),
            Self::Translucent {
                color,
                transmission,
            } => {
                ensure!(
                    (0.0..=1.0).contains(&transmission),
                    "transmission must be in [0, 1]"
                );
                Material::Translucent {
                    color: Vec3::from_array(color),
                    transmission,
                }
            }
            Self::Emissive { color, intensity } => {
                Material::Emissive(Vec3::from_array(Light { color, intensity }.rgb()?))
            }
        };
        validate_rgb(material.color().to_array())?;
        Ok(material)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Earth {
    pub material: MaterialConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Sun {
    pub distance: f64,
    pub emission: Light,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Moon {
    pub ground_offset: [f64; 2],
    pub distance: f64,
    pub subdivisions: u32,
    pub material: MaterialConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Ambient {
    pub night: Light,
    pub day: Light,
    pub response_factor: f32,
    pub response_exponent: f32,
}

impl Ambient {
    pub fn build(&self) -> Result<AmbientLight> {
        ensure!(
            self.response_factor.is_finite() && self.response_factor >= 0.0,
            "response_factor must be finite and nonnegative"
        );
        ensure!(
            self.response_exponent.is_finite() && self.response_exponent > 0.0,
            "response_exponent must be positive and finite"
        );
        Ok(AmbientLight {
            day: self.day.rgb().context("day")?,
            night: self.night.rgb().context("night")?,
            response_factor: self.response_factor,
            response_exponent: self.response_exponent,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Air {
    pub height: f64,
    pub scattering: Light,
    pub background: [f32; 3],
}

impl Config {
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
        self.earth.material.build().context("earth.material")?;
        let sun = self.sun.emission.rgb().context("sun.emission")?;
        ensure!(
            self.sun.distance > 0.0
                && self.sun.distance.cosh() > 1.0
                && (self.sun.distance.sinh() as f32).is_finite(),
            "sun.distance must be positive and representable"
        );
        ensure!(
            self.atmosphere.height > 0.0
                && self.atmosphere.height.cosh() < self.sun.distance.cosh()
                && (self.atmosphere.height.sinh() as f32) > 0.0,
            "atmosphere.height must be positive and below sun.distance"
        );
        validate_rgb(self.atmosphere.background).context("atmosphere.background")?;
        let scattering = self
            .atmosphere
            .scattering
            .rgb()
            .context("atmosphere.scattering")?;
        ensure!(
            sun.into_iter()
                .zip(scattering)
                .all(|(light, beta)| (light * beta).is_finite()),
            "sun.emission times atmosphere.scattering exceeds the shader range"
        );
        self.ambient.build().context("ambient")?;
        self.moon.material.build().context("moon.material")?;
        ensure!(
            self.moon.distance > 0.0
                && (self.moon.distance.cosh()
                    * self.moon.ground_offset[0].cosh()
                    * self.moon.ground_offset[1].cosh())
                .is_finite(),
            "invalid moon.distance or moon.ground_offset"
        );
        ensure!(
            // An n by n grid needs 6*n² indices and (n+1)² vertices.
            (1..=(u32::MAX / 6).isqrt()).contains(&self.moon.subdivisions),
            "moon.subdivisions must fit indexed triangle buffers"
        );
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

    const WORLD: &str = include_str!("../hearth.toml");

    #[test]
    fn every_setting_in_the_supplied_world_is_required() -> Result<()> {
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
                path.push(key.clone());
                assert!(
                    Config::parse(&toml::to_string(&incomplete)?).is_err(),
                    "omitting {} should fail",
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
    fn unknown_settings_and_incompatible_values_report_the_field() -> Result<()> {
        let mut table: toml::Table = toml::from_str(WORLD)?;
        table["sun"]
            .as_table_mut()
            .unwrap()
            .insert("distence".into(), 3.0.into());
        let error = Config::parse(&toml::to_string(&table)?).unwrap_err();
        assert!(error.to_string().contains("distence"));

        table["sun"].as_table_mut().unwrap().remove("distence");
        table["atmosphere"]["height"] = table["sun"]["distance"].clone();
        let error = Config::parse(&toml::to_string(&table)?).unwrap_err();
        assert!(format!("{error:#}").contains("atmosphere.height"));
        Ok(())
    }
}
