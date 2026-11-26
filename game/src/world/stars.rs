//! A finite world-fixed catalogue of emitting geodesic planes.
use std::f64::consts::TAU;

use anyhow::Result;
use ccgeom::{Geometry3, Hyperboloid3};
use wgame::glam::Vec3;

use super::{InfinitePlane, Material, Object, Pose, Samples};

pub(super) fn catalogue() -> Result<Vec<Object<InfinitePlane>>> {
    let mut samples = Samples(0x33cd_831e_6010_9289);
    let ground_radius = 5.0_f64;
    (0..1536)
        .map(|_| {
            // Place the foot of the common perpendicular uniformly by area on
            // Earth. A disk of radius r has area 2π(cosh(r)−1) in unit curvature.
            let radius = (1.0 + samples.next() * (ground_radius.cosh() - 1.0)).acosh();
            let azimuth = TAU * samples.next();
            let gap = 4.9 + samples.next();
            let pose = Pose::from_isometry(
                Hyperboloid3::rotate_y(azimuth)
                    .chain(Hyperboloid3::shift_z(radius))
                    .chain(Hyperboloid3::shift_y(gap)),
            )?;
            let color =
                Vec3::new(0.72, 0.83, 1.0).lerp(Vec3::new(1.0, 0.87, 0.67), samples.next() as f32);
            let intensity = 0.005 + 0.12 * (samples.next() as f32).powi(3);
            Ok(Object {
                shape: InfinitePlane,
                pose,
                material: Material::Emissive(color * intensity),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_catalogue_reproduces_distant_earth_disjoint_planes() -> Result<()> {
        let first = catalogue()?;
        let second = catalogue()?;
        for (star, same) in first.iter().zip(second) {
            assert_eq!(star.pose.components()?, same.pose.components()?);
            assert_eq!(star.material.color(), same.material.color());

            let [w, x, y, z] = star.pose.apply_vector([0.0, 0.0, 1.0, 0.0]);
            // The two unit spacelike plane normals have inner product cosh(gap).
            assert!(y > 1.0);
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
}
