use anyhow::Result;

use crate::{
    config::{
        Sun,
        vegetation::{BiomeProfile, Ecology, SunVegetation},
    },
    projection::Pose,
};

pub(crate) use crate::config::vegetation::Species;

#[derive(Clone, Copy)]
struct SolarExposure {
    // Maps points from the sampling frame to a ground frame centered at noon.
    to_noon: Pose,
    tanh_gap: f64,
    noon_irradiance: f64,
    vegetation: SunVegetation,
}

impl SolarExposure {
    fn at(&self, root: [f64; 4]) -> (f64, f64) {
        let [_, x, _, z] = self.to_noon.apply_vector(root);
        let sinh_radius = x.hypot(z);
        // For a point on Earth at ground radius r and a Sun at gap h, the
        // plane distance d satisfies sinh(d) = sinh(h) cosh(r). The ratio of
        // local to noon irradiance is (cosh(h)/cosh(d))³.
        let ratio = (self.tanh_gap * sinh_radius).hypot(1.0).recip();
        (sinh_radius.asinh(), self.noon_irradiance * ratio.powi(3))
    }
}

pub(crate) struct Daylight {
    sources: Vec<SolarExposure>,
}

impl Daylight {
    pub fn new<'a>(suns: impl IntoIterator<Item = (Pose, &'a Sun)>) -> Result<Self> {
        let mut sources = Vec::new();
        let mut brightest = 0.0_f64;
        for (noon_pose, sun) in suns {
            let peak = sun.surface_radiance.into_iter().fold(0.0_f32, f32::max);
            let irradiance = f64::from(peak) / sun.distance.cosh().powi(2);
            brightest = brightest.max(irradiance);
            sources.push(SolarExposure {
                to_noon: noon_pose.inverse()?,
                tanh_gap: sun.distance.tanh(),
                noon_irradiance: irradiance,
                vegetation: sun.vegetation,
            });
        }
        if brightest > 0.0 {
            for sun in &mut sources {
                sun.noon_irradiance /= brightest;
            }
        }
        Ok(Self { sources })
    }

    /// Compose tile-to-noon frames once before sampling the tile's local roots.
    pub fn in_frame(&self, tile: Pose) -> Result<Self> {
        let mut sources = Vec::with_capacity(self.sources.len());
        for sun in &self.sources {
            sources.push(SolarExposure {
                to_noon: sun.to_noon.chain(&tile)?,
                ..*sun
            });
        }
        Ok(Self { sources })
    }

    /// Expected plants per R² and maximum mature size at a candidate root.
    pub fn at(&self, root: [f64; 4], species: Species, floor: Ecology) -> (f64, f64) {
        let mut density = floor.night_density;
        let mut size = floor.minimum_size;
        for sun in &self.sources {
            if let Some(profile) = species.profile(&sun.vegetation) {
                let (radius, light) = sun.at(root);
                let (local_density, local_size) = profile.at(radius, light, floor);
                density = density.max(local_density);
                size = size.max(local_size);
            }
        }
        (density, size)
    }
}

impl BiomeProfile {
    fn at(self, radius: f64, light: f64, floor: Ecology) -> (f64, f64) {
        let offset = (radius - self.peak_radius) / self.half_width;
        let weight = (-offset.powi(2)).exp2();
        let density = floor.night_density + (self.peak_density - floor.night_density) * weight;
        let vigor = light * (1.0 + self.half_saturation) / (light + self.half_saturation);
        let size = floor.minimum_size + (self.max_size - floor.minimum_size) * vigor;
        (density, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SunAmbient;

    const EMPTY: SunVegetation = SunVegetation {
        deciduous: None,
        pine: None,
        grass: None,
    };
    const RING: BiomeProfile = BiomeProfile {
        peak_density: 25.0,
        peak_radius: 2.0,
        half_width: 1.0,
        half_saturation: 0.5,
        max_size: 1.0,
    };
    const FLOOR: Ecology = Ecology {
        night_density: 1.0,
        minimum_size: 0.25,
    };

    fn sun(radiance: f32, profile: BiomeProfile) -> Sun {
        Sun {
            ground_offset: [0.0; 2],
            distance: 1.2,
            disk_radiance: [1.0; 3],
            surface_radiance: [radiance; 3],
            scattering_radiance: [0.0; 3],
            ambient: SunAmbient {
                color: [0.0; 3],
                response_factor: 1.0,
                response_exponent: 1.0,
            },
            vegetation: SunVegetation {
                pine: Some(profile),
                ..EMPTY
            },
        }
    }

    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-10, "{actual} vs {expected}");
    }

    #[test]
    fn ring_has_equal_half_density_at_both_edges_in_every_direction() -> Result<()> {
        let shifted_noon = Pose::identity().move_local([1.2, 0.0, -0.8], [0.0; 3], 1.0)?;
        let tile = shifted_noon.move_local([0.6, 0.0, -0.3], [0.0, 0.4, 0.0], 1.0)?;
        let noon_to_tile = tile.inverse()?.chain(&shifted_noon)?;
        let source = sun(2.0, RING);
        let daylight = Daylight::new([(shifted_noon, &source)])?.in_frame(tile)?;
        for (r, density) in [(0.0_f64, 2.5), (1.0, 13.0), (2.0, 25.0), (3.0, 13.0)] {
            for bearing in [0.0_f64, 0.9, 2.7] {
                let root = noon_to_tile.apply_vector([
                    r.cosh(),
                    r.sinh() * bearing.cos(),
                    0.0,
                    r.sinh() * bearing.sin(),
                ]);
                let (d, size) = daylight.at(root, Species::Pine, FLOOR);
                close(d, density);
                let cosh_distance = (source.distance.sinh() * r.cosh()).hypot(1.0);
                let light = (source.distance.cosh() / cosh_distance).powi(3);
                let expected_size = 0.25 + 0.75 * light * 1.5 / (light + 0.5);
                close(size, expected_size);
            }
        }
        Ok(())
    }

    #[test]
    fn tile_local_sampling_resolves_a_narrow_belt_at_a_distant_noon() -> Result<()> {
        let noon = Pose::identity().move_local([12.0, 0.0, 0.0], [0.0; 3], 1.0)?;
        let source = sun(
            1.0,
            BiomeProfile {
                peak_radius: 2e-8,
                half_width: 1e-8,
                ..RING
            },
        );
        let daylight = Daylight::new([(noon, &source)])?.in_frame(noon)?;
        let r = 1e-8_f64;
        let root = [r.cosh(), 0.0, 0.0, r.sinh()];
        close(daylight.at(root, Species::Pine, FLOOR).0, 13.0);
        Ok(())
    }

    #[test]
    fn density_and_growth_choose_their_strongest_sources_independently() -> Result<()> {
        let dark = sun(
            0.0,
            BiomeProfile {
                peak_radius: 0.0,
                ..RING
            },
        );
        let dim = sun(
            1.0,
            BiomeProfile {
                peak_density: 5.0,
                ..RING
            },
        );
        let mut bright = sun(4.0, RING);
        bright.vegetation = EMPTY;
        let foot = Pose::identity();
        let root = [1.0, 0.0, 0.0, 0.0];
        let daylight = Daylight::new([(foot, &dark), (foot, &dim), (foot, &bright)])?;
        // A dark Sun defines a centered biome, while the dim Sun supplies
        // a quarter of the brightest individual noon for this species' growth.
        assert_eq!(daylight.at(root, Species::Pine, FLOOR), (25.0, 0.625));
        assert_eq!(daylight.at(root, Species::Grass, FLOOR), (1.0, 0.25));
        let unlit = Daylight::new([(foot, &dark)])?;
        assert_eq!(unlit.at(root, Species::Pine, FLOOR), (25.0, 0.25));
        let distant = [40.0_f64.cosh(), 40.0_f64.sinh(), 0.0, 0.0];
        assert_eq!(unlit.at(distant, Species::Pine, FLOOR), (1.0, 0.25));
        assert_eq!(
            Daylight::new([])?.at(root, Species::Pine, FLOOR),
            (1.0, 0.25)
        );
        Ok(())
    }
}
