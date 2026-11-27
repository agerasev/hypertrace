use anyhow::{Result, ensure};

use crate::{
    config::vegetation::{Ecology, LightResponse, SunVegetation, Vegetation},
    projection::Pose,
    world::Sun,
};

struct SolarExposure {
    // Unit spacelike Sun plane normal in Earth's frame (W,X,Y,Z).
    normal: [f64; 4],
    noon: f64,
    vegetation: SunVegetation,
}

impl SolarExposure {
    fn at(&self, [w, x, _, z]: [f64; 4]) -> f64 {
        let [nw, nx, ny, nz] = self.normal;
        let sinh_distance = -nw * w + nx * x + nz * z;
        // Lambertian irradiance scales as cosh(gap)/cosh(distance)^3 on
        // Earth. Its fraction of this Sun's own noon is the cubed ratio.
        self.noon * (ny.abs() / sinh_distance.hypot(1.0)).min(1.0).powi(3)
    }
}

/// Surface illumination on Earth, relative to the brightest individual noon.
pub struct Daylight {
    sources: Vec<SolarExposure>,
}

impl Daylight {
    pub fn new<'a>(
        earth: Pose,
        suns: impl IntoIterator<Item = (&'a Sun, &'a SunVegetation)>,
    ) -> Result<Self> {
        let world_to_earth = earth.inverse()?;
        let mut sources = Vec::new();
        let mut brightest = 0.0_f64;
        for (sun, vegetation) in suns {
            let normal = world_to_earth
                .chain(&sun.pose)?
                .apply_vector([0.0, 0.0, 1.0, 0.0]);
            ensure!(
                normal[2].abs() > 1.0,
                "permanent noon requires separated Earth and Sun planes"
            );
            let peak = sun.surface_radiance.into_iter().fold(0.0_f32, f32::max);
            let noon = f64::from(peak) / normal[2].powi(2);
            if normal[0].is_sign_positive() == normal[2].is_sign_positive() && noon > 0.0 {
                brightest = brightest.max(noon);
                sources.push(SolarExposure {
                    normal,
                    noon,
                    vegetation: *vegetation,
                });
            }
        }
        for sun in &mut sources {
            sun.noon /= brightest;
        }
        Ok(Self { sources })
    }

    /// Occupancy and maximum size at a candidate root on Earth (Y=0).
    pub fn at(&self, root: [f64; 4], species: Species, settings: &Vegetation) -> (f64, f64) {
        let ecology = match species {
            Species::Deciduous => settings.deciduous.ecology,
            Species::Pine => settings.pine.ecology,
            Species::Grass => settings.grass.ecology,
        };
        ecology.response(
            species,
            self.sources
                .iter()
                .map(|sun| (&sun.vegetation, sun.at(root))),
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Species {
    Deciduous,
    Pine,
    Grass,
}

fn smoothstep([low, high]: [f64; 2], value: f64) -> f64 {
    let t = ((value - low) / (high - low)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl LightResponse {
    fn at(self, light: f64, ecology: Ecology) -> (f64, f64) {
        let occupancy = ecology.night_occupancy
            + (self.max_occupancy - ecology.night_occupancy) * smoothstep(self.light_range, light);
        let vigor = light * (1.0 + self.half_saturation) / (light + self.half_saturation);
        let size = ecology.minimum_size + (self.max_size - ecology.minimum_size) * vigor;
        (occupancy, size)
    }
}

impl Ecology {
    fn response<'a>(
        self,
        species: Species,
        exposures: impl Iterator<Item = (&'a SunVegetation, f64)>,
    ) -> (f64, f64) {
        let mut density = self.night_occupancy;
        let mut growth = self.minimum_size;
        for (vegetation, light) in exposures {
            let response = match species {
                Species::Deciduous => vegetation.deciduous.map(|curve| curve.at(light, self)),
                Species::Grass => vegetation.grass.map(|curve| curve.at(light, self)),
                Species::Pine => vegetation.pine.map(|pine| {
                    let (occupancy, size) = pine.response().at(light, self);
                    let suppress =
                        pine.daylight_suppression * smoothstep(pine.daylight_range, light);
                    (occupancy * (1.0 - suppress), size)
                }),
            };
            if let Some((occupancy, size)) = response {
                density = density.max(occupancy);
                growth = growth.max(size);
            }
        }
        (density, growth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::vegetation::PineResponse;

    const EMPTY: SunVegetation = SunVegetation {
        deciduous: None,
        pine: None,
        grass: None,
    };
    const CURVE: LightResponse = LightResponse {
        light_range: [0.25, 0.75],
        half_saturation: 0.5,
        max_occupancy: 0.875,
        max_size: 1.0,
    };
    const FLOOR: Ecology = Ecology {
        night_occupancy: 0.125,
        minimum_size: 0.25,
    };

    #[test]
    fn daylight_follows_the_sun_across_the_ground() -> Result<()> {
        let gap: f64 = 1.2;
        let noon = Pose::identity().move_local([0.8, 0.0, 0.0], [0.0; 3], 1.0)?;
        let earth = Pose::identity().move_local([0.1, -0.4, 0.2], [0.3, 0.1, -0.4], 1.0)?;
        let sun = Sun {
            pose: earth
                .chain(&noon)?
                .move_local([0.0, gap, 0.0], [0.0; 3], 1.0)?,
            surface_radiance: [1.0; 3],
            scattering_radiance: [0.0; 3],
            ambient: crate::config::SunAmbient {
                color: [0.0; 3],
                response_factor: 1.0,
                response_exponent: 1.0,
            },
        };
        let source = Daylight::new(earth, [(&sun, &EMPTY)])?;
        for radius in [0.0_f64, 0.8, 2.2] {
            let root = noon.move_local([0.0, 0.0, radius], [0.0; 3], 1.0)?;
            let cosh_distance = (gap.sinh() * radius.cosh()).hypot(1.0);
            let expected = (gap.cosh() / cosh_distance).powi(3);
            assert!(
                (source.sources[0].at(root.apply_vector([1.0, 0.0, 0.0, 0.0])) - expected).abs()
                    < 1e-12
            );
        }
        let dim = Sun {
            surface_radiance: [0.25, 0.0, 0.0],
            ..sun
        };
        let scatter_only = Sun {
            surface_radiance: [0.0; 3],
            scattering_radiance: [10.0; 3],
            ..sun
        };
        let daylight = Daylight::new(
            earth,
            [(&sun, &EMPTY), (&dim, &EMPTY), (&scatter_only, &EMPTY)],
        )?;
        let root = noon.apply_vector([1.0, 0.0, 0.0, 0.0]);
        assert_eq!(daylight.sources.len(), 2);
        assert!((daylight.sources[1].at(root) - 0.25).abs() < 1e-12);
        Ok(())
    }

    #[test]
    fn each_species_uses_its_strongest_sun_response_and_its_night_floors() {
        let warm = SunVegetation {
            grass: Some(CURVE),
            ..EMPTY
        };
        let cool = SunVegetation {
            grass: Some(LightResponse {
                max_occupancy: 0.75,
                max_size: 0.5,
                ..CURVE
            }),
            ..EMPTY
        };
        let red = SunVegetation {
            pine: Some(PineResponse {
                light_range: CURVE.light_range,
                half_saturation: CURVE.half_saturation,
                max_occupancy: CURVE.max_occupancy,
                max_size: CURVE.max_size,
                daylight_range: [0.5, 1.0],
                daylight_suppression: 0.75,
            }),
            ..EMPTY
        };
        let exposures = [(&warm, 0.5), (&red, 1.0), (&cool, 1.0)];
        assert_eq!(
            FLOOR.response(Species::Grass, [(&warm, 0.5)].into_iter()),
            (0.5, 0.8125)
        );
        assert_eq!(
            FLOOR.response(Species::Grass, exposures.into_iter()),
            (0.75, 0.8125)
        );
        assert_eq!(
            FLOOR.response(Species::Pine, exposures.into_iter()),
            (0.875 * 0.25, 1.0)
        );
        assert_eq!(
            FLOOR.response(Species::Deciduous, exposures.into_iter()),
            (0.125, 0.25)
        );
        assert_eq!(
            FLOOR.response(Species::Grass, [].into_iter()),
            (0.125, 0.25)
        );
    }
}
