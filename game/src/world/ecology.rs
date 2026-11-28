use anyhow::Result;

use crate::{
    config::vegetation::{BiomeProfile, Ecology, SunVegetation},
    projection::Pose,
};

pub(crate) use crate::config::vegetation::Species;

#[derive(Clone, Copy)]
struct Biome {
    // Maps points from the sampling frame to a ground frame centered at noon.
    to_noon: Pose,
    vegetation: SunVegetation,
}

impl Biome {
    fn radius(&self, root: [f64; 4]) -> f64 {
        let [_, x, y, _] = self.to_noon.apply_vector(root);
        x.hypot(y).asinh()
    }
}

pub(crate) struct Biomes {
    biomes: Vec<Biome>,
}

impl Biomes {
    pub fn new(suns: impl IntoIterator<Item = (Pose, SunVegetation)>) -> Result<Self> {
        let mut biomes = Vec::new();
        for (noon_pose, vegetation) in suns {
            biomes.push(Biome {
                to_noon: noon_pose.inverse()?,
                vegetation,
            });
        }
        Ok(Self { biomes })
    }

    /// Compose tile-to-noon frames once before sampling the tile's local roots.
    pub fn in_frame(&self, tile: Pose) -> Result<Self> {
        let mut biomes = Vec::with_capacity(self.biomes.len());
        for biome in &self.biomes {
            biomes.push(Biome {
                to_noon: biome.to_noon.chain(&tile)?,
                ..*biome
            });
        }
        Ok(Self { biomes })
    }

    /// Expected plants per R² and the uniform size range at a candidate root.
    pub fn at(&self, root: [f64; 4], species: Species, floor: Ecology) -> (f64, [f64; 2]) {
        let mut density = floor.night_density;
        let mut size: Option<[f64; 2]> = None;
        for biome in &self.biomes {
            if let Some(profile) = species.profile(&biome.vegetation) {
                let (local_density, local_size) = profile.at(biome.radius(root), floor);
                density = density.max(local_density);
                size = Some(match size {
                    Some([lo, hi]) => [lo.max(local_size[0]), hi.max(local_size[1])],
                    None => local_size,
                });
            }
        }
        (density, size.unwrap_or(floor.night_size))
    }
}

fn gaussian(radius: f64, peak_radius: f64, half_width: f64) -> f64 {
    let offset = (radius - peak_radius) / half_width;
    (-offset.powi(2)).exp2()
}

impl BiomeProfile {
    fn at(self, radius: f64, floor: Ecology) -> (f64, [f64; 2]) {
        let density_weight = gaussian(radius, self.peak_radius, self.half_width);
        let density =
            floor.night_density + (self.peak_density - floor.night_density) * density_weight;
        let size_weight = gaussian(radius, self.size.peak_radius, self.size.half_width);
        let size = std::array::from_fn(|i| {
            (1.0 - size_weight) * floor.night_size[i] + size_weight * self.size.peak[i]
        });
        (density, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::vegetation::SizeProfile;

    const EMPTY: SunVegetation = SunVegetation {
        deciduous: None,
        pine: None,
        grass: None,
    };
    const RING: BiomeProfile = BiomeProfile {
        peak_density: 25.0,
        peak_radius: 2.0,
        half_width: 1.0,
        size: SizeProfile {
            peak: [0.625, 1.0],
            peak_radius: 2.5,
            half_width: 0.5,
        },
    };
    const FLOOR: Ecology = Ecology {
        night_density: 1.0,
        night_size: [0.125, 0.25],
    };

    fn pines(profile: BiomeProfile) -> SunVegetation {
        SunVegetation {
            pine: Some(profile),
            ..EMPTY
        }
    }

    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-10, "{actual} vs {expected}");
    }

    #[test]
    fn density_and_size_follow_their_own_rings_in_every_direction() -> Result<()> {
        let shifted_noon = Pose::identity().move_local([1.2, 0.8, 0.0], [0.0; 3], 1.0)?;
        let tile = shifted_noon.move_local([0.6, 0.3, 0.0], [0.0, 0.0, 0.4], 1.0)?;
        let noon_to_tile = tile.inverse()?.chain(&shifted_noon)?;
        let biomes = Biomes::new([(shifted_noon, pines(RING))])?.in_frame(tile)?;
        for (r, density, size_weight) in [
            (0.0_f64, 2.5, 2.0_f64.powi(-25)),
            (1.0, 13.0, 1.0 / 512.0),
            (2.0, 25.0, 0.5),
            (2.5, 1.0 + 24.0 * 2.0_f64.powf(-0.25), 1.0),
            (3.0, 13.0, 0.5),
        ] {
            for bearing in [0.0_f64, 0.9, 2.7] {
                let root = noon_to_tile.apply_vector([
                    r.cosh(),
                    r.sinh() * bearing.cos(),
                    -r.sinh() * bearing.sin(),
                    0.0,
                ]);
                let (d, [lo, hi]) = biomes.at(root, Species::Pine, FLOOR);
                close(d, density);
                close(lo, 0.125 + 0.5 * size_weight);
                close(hi, 0.25 + 0.75 * size_weight);
            }
        }
        Ok(())
    }

    #[test]
    fn tile_local_sampling_resolves_a_narrow_belt_at_a_distant_noon() -> Result<()> {
        let noon = Pose::identity().move_local([12.0, 0.0, 0.0], [0.0; 3], 1.0)?;
        let narrow = pines(BiomeProfile {
            peak_radius: 2e-8,
            half_width: 1e-8,
            ..RING
        });
        let biomes = Biomes::new([(noon, narrow)])?.in_frame(noon)?;
        let r = 1e-8_f64;
        let root = [r.cosh(), 0.0, -r.sinh(), 0.0];
        close(biomes.at(root, Species::Pine, FLOOR).0, 13.0);
        Ok(())
    }

    #[test]
    fn overlapping_profiles_take_each_maximum_and_preserve_night_ranges() -> Result<()> {
        let narrow = pines(BiomeProfile {
            size: SizeProfile {
                peak_radius: 0.0,
                peak: [0.75, 0.75],
                ..RING.size
            },
            ..RING
        });
        let wide = pines(BiomeProfile {
            peak_radius: 0.0,
            size: SizeProfile {
                peak_radius: 0.0,
                ..RING.size
            },
            ..RING
        });
        let noon = Pose::identity();
        let root = [1.0, 0.0, 0.0, 0.0];
        let biomes = Biomes::new([(noon, narrow), (noon, wide)])?;
        assert_eq!(biomes.at(root, Species::Pine, FLOOR), (25.0, [0.75, 1.0]));
        let night = (FLOOR.night_density, FLOOR.night_size);
        assert_eq!(biomes.at(root, Species::Grass, FLOOR), night);
        let distant = [40.0_f64.cosh(), 40.0_f64.sinh(), 0.0, 0.0];
        assert_eq!(biomes.at(distant, Species::Pine, FLOOR), night);
        assert_eq!(Biomes::new([])?.at(root, Species::Pine, FLOOR), night);
        // A biome can explicitly grow smaller plants at its center than in the night.
        let dwarf = pines(BiomeProfile {
            size: SizeProfile {
                peak: [0.0625; 2],
                peak_radius: 0.0,
                ..RING.size
            },
            ..RING
        });
        assert_eq!(
            Biomes::new([(noon, dwarf)])?
                .at(root, Species::Pine, FLOOR)
                .1,
            [0.0625; 2]
        );
        Ok(())
    }
}
