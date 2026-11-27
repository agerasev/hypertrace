//! Emitting geodesic planes rooted in the pentagonal tiling of Earth.
use anyhow::Result;
use ccgeom::{Geometry3, Hyperboloid3};
use wgame::glam::Vec3;

use super::{InfinitePlane, Material, Object, Pose, RootSampler, Samples, TileId};
use crate::config::stars::StarField;

pub(super) fn in_tile(settings: &StarField, id: &TileId) -> Result<Vec<Object<InfinitePlane>>> {
    let mut samples = Samples(id.seed(settings.seed));
    let count = samples.site_count(settings.mean_per_tile());
    let sampler = RootSampler::new();
    let [near, far] = settings.gap;
    let start = Vec3::from_array(settings.colors.start).as_dvec3();
    let end = Vec3::from_array(settings.colors.end).as_dvec3();
    (0..count)
        .map(|_| {
            let foot = sampler.local_root(&mut samples)?;
            let gap = near + (far - near) * samples.next();
            let pose = foot.chain(&Pose::from_isometry(Hyperboloid3::shift_y(gap))?)?;
            let color = start.lerp(end, samples.next()).as_vec3();
            Ok(Object {
                shape: InfinitePlane,
                pose,
                material: Material::emissive(color.to_array()),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::stars::StarColors;
    use std::f64::consts::FRAC_PI_2;

    fn field() -> StarField {
        StarField {
            density: 20.5,
            seed: 11,
            gap: [3.0, 6.0],
            colors: StarColors {
                start: [0.0, 1.0, 2.0],
                end: [1.0, 0.0, 2.0],
            },
        }
    }

    #[test]
    fn sparse_tiles_preserve_the_requested_ground_density() -> Result<()> {
        let tile = TileId::default();
        let mut occupied = 0;
        for seed in 0..1024 {
            let settings = StarField {
                density: 0.25 / FRAC_PI_2,
                seed,
                ..field()
            };
            let count = in_tile(&settings, &tile)?.len();
            assert!(count <= 1);
            occupied += count;
        }
        assert!((220..292).contains(&occupied), "{occupied}");
        Ok(())
    }

    #[test]
    fn returning_to_a_tile_reproduces_its_earth_disjoint_planes() -> Result<()> {
        let settings = field();
        let tile = TileId::default().neighbor(0);
        let frame = tile.upright_frame()?;
        let first = in_tile(&settings, &tile)?;
        let neighbor = in_tile(&settings, &tile.neighbor(2))?;
        assert_ne!(first[0].pose.components()?, neighbor[0].pose.components()?);
        let second = in_tile(&settings, &tile.neighbor(2).neighbor(2))?;
        let mean = settings.density * FRAC_PI_2;
        assert!((mean.floor() as usize..=mean.ceil() as usize).contains(&first.len()));
        assert_eq!(first.len(), second.len());
        for (star, same) in first.iter().zip(second) {
            assert_eq!(star.pose.components()?, same.pose.components()?);
            assert_eq!(star.material.emission, same.material.emission);

            let world = frame.chain(&star.pose)?;
            assert_eq!(TileId::at(world)?, tile);
            let [w, x, y, z] = world.apply_vector([0.0, 0.0, 1.0, 0.0]);
            // The two unit spacelike plane normals have inner product cosh(gap).
            assert!((settings.gap[0]..settings.gap[1]).contains(&y.acosh()));
            let sinh_gap = ((y - 1.0) * (y + 1.0)).sqrt();
            // The projection of the star normal onto Earth is proportional to
            // the foot. Walk halfway there along the geodesic from noon.
            let half_w = ((1.0 + w / sinh_gap) * 0.5).sqrt();
            let half_sinh_d = w * half_w - (x * x + z * z) / (2.0 * half_w * sinh_gap);
            let radius_sine = |sinh_d: f64| 1.0 / sinh_d.hypot(1.0);
            assert!(radius_sine(sinh_gap) >= radius_sine(half_sinh_d));
            assert!(radius_sine(half_sinh_d) >= radius_sine(w));
        }
        Ok(())
    }

    #[test]
    fn stars_sample_one_uniform_color_mix_independently_of_placement() -> Result<()> {
        let settings = StarField {
            density: 1024.0 / FRAC_PI_2,
            ..field()
        };
        let tile = TileId::default().neighbor(4).neighbor(2);
        let stars = in_tile(&settings, &tile)?;
        let flat = in_tile(
            &StarField {
                colors: StarColors {
                    start: [0.17, 1.3, f32::MAX],
                    end: [0.17, 1.3, f32::MAX],
                },
                ..settings
            },
            &tile,
        )?;
        let mut bins = [0; 4];
        let mut nearest = f64::INFINITY;
        let mut farthest = 0.0_f64;
        for (star, plain) in stars.iter().zip(flat) {
            assert_eq!(star.pose.components()?, plain.pose.components()?);
            assert_eq!(plain.material.emission, [0.17, 1.3, f32::MAX]);
            let gap = star.pose.apply_vector([0.0, 0.0, 1.0, 0.0])[2].acosh();
            nearest = nearest.min(gap);
            farthest = farthest.max(gap);

            let color = Vec3::from_array(star.material.emission);
            let t = color.x;
            assert!((0.0..=1.0).contains(&t));
            assert!(color.abs_diff_eq(Vec3::new(t, 1.0 - t, 2.0), 1e-6));
            bins[((t * 4.0) as usize).min(3)] += 1;
        }
        assert!(farthest - nearest > 0.5 * (settings.gap[1] - settings.gap[0]));
        assert!(
            bins.iter().all(|&count| (200..320).contains(&count)),
            "{bins:?}"
        );
        Ok(())
    }
}
