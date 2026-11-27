//! A finite world-fixed catalogue of emitting geodesic planes.
use std::f64::consts::TAU;

use anyhow::Result;
use ccgeom::{Geometry3, Hyperboloid3};
use wgame::glam::Vec3;

use super::{InfinitePlane, Material, Object, Pose, Samples};
use crate::config::stars::StarField;

pub(super) fn catalogue(settings: &StarField) -> Result<Vec<Object<InfinitePlane>>> {
    let disk_scale = (0.5 * settings.ground_radius).sinh();
    let [near, far] = settings.gap;
    let [start, end] = settings.colors.map(|rgb| Vec3::from_array(rgb).as_dvec3());
    (0..settings.count)
        .map(|index| {
            // The catalogue assigns five sample slots per star. Addressing its
            // entry directly keeps the seeded constellation stable as colors change.
            let mut samples = Samples::at_offset(settings.seed, u64::from(index) * 5);
            // Place the foot of the common perpendicular uniformly by area on
            // Earth. A disk of radius r has area 4π sinh²(r/2) in unit curvature.
            let radius = 2.0 * (samples.next().sqrt() * disk_scale).asinh();
            let azimuth = TAU * samples.next();
            let gap = near + (far - near) * samples.next();
            let pose = Pose::from_isometry(
                Hyperboloid3::rotate_y(azimuth)
                    .chain(Hyperboloid3::shift_z(radius))
                    .chain(Hyperboloid3::shift_y(gap)),
            )?;
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

    fn field() -> StarField {
        StarField {
            count: 32,
            seed: 11,
            ground_radius: 4.0,
            gap: [3.0, 6.0],
            colors: [[0.0, 1.0, 2.0], [1.0, 0.0, 2.0]],
        }
    }

    #[test]
    fn fixed_catalogue_reproduces_distant_earth_disjoint_planes() -> Result<()> {
        let settings = field();
        let first = catalogue(&settings)?;
        let second = catalogue(&settings)?;
        assert_eq!(first.len(), settings.count as usize);
        for (star, same) in first.iter().zip(second) {
            assert_eq!(star.pose.components()?, same.pose.components()?);
            assert_eq!(star.material.emission, same.material.emission);

            let [w, x, y, z] = star.pose.apply_vector([0.0, 0.0, 1.0, 0.0]);
            // The two unit spacelike plane normals have inner product cosh(gap).
            assert!((settings.gap[0]..settings.gap[1]).contains(&y.acosh()));
            let sinh_gap = ((y - 1.0) * (y + 1.0)).sqrt();
            assert!((w / sinh_gap).acosh() <= settings.ground_radius);
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
            count: 1024,
            ..field()
        };
        let stars = catalogue(&settings)?;
        let flat = catalogue(&StarField {
            colors: [[0.17, 1.3, f32::MAX]; 2],
            ..settings
        })?;
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
