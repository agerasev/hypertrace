//! A finite world-fixed catalogue of emitting geodesic planes.
use std::f64::consts::TAU;

use anyhow::Result;
use ccgeom::{Geometry3, Hyperboloid3};
use wgame::glam::Vec3;

use super::{InfinitePlane, Material, Object, Pose, Samples};
use crate::config::stars::StarField;

pub(super) fn catalogue(settings: &StarField) -> Result<Vec<Object<InfinitePlane>>> {
    let mut samples = Samples(settings.seed);
    let disk_scale = (0.5 * settings.ground_radius).sinh();
    let [near, far] = settings.gap;
    let cool = Vec3::from_array(settings.cool_color);
    let warm = Vec3::from_array(settings.warm_color);
    let [dim, bright] = settings.intensity;
    (0..settings.count)
        .map(|_| {
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
            let color = cool.lerp(warm, samples.next() as f32);
            let intensity =
                dim + (bright - dim) * (samples.next() as f32).powf(settings.intensity_exponent);
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
        let settings = StarField {
            count: 32,
            seed: 11,
            ground_radius: 4.0,
            gap: [3.0, 6.0],
            cool_color: [0.4, 0.7, 1.0],
            warm_color: [1.0, 0.9, 0.5],
            intensity: [0.01, 0.1],
            intensity_exponent: 1.5,
        };
        let first = catalogue(&settings)?;
        let second = catalogue(&settings)?;
        assert_eq!(first.len(), settings.count as usize);
        for (star, same) in first.iter().zip(second) {
            assert_eq!(star.pose.components()?, same.pose.components()?);
            assert_eq!(star.material.color(), same.material.color());

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
}
