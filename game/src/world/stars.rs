//! Each ground-grid node owns a deterministic catalogue of distant geodesic
//! planes. Walking blends the four surrounding catalogues. A star's plane
//! stays at the same world pose throughout every transition.

use std::{collections::BTreeMap, f64::consts::TAU};

use anyhow::Result;
use ccgeom::{Geometry3, Hyperboloid3};
use wgame::glam::Vec3;

use super::{Cell, InfinitePlane, Material, Object, Pose, Samples};

struct Star {
    pose: Pose,
    radiance: Vec3,
    threshold: f64,
}

fn catalogue(node: Cell) -> Result<Vec<Star>> {
    const STARS: usize = 512;
    let anchor = node.root(0.0, 0.0)?;
    let mut samples = Samples::for_cell(node);
    samples.0 ^= 0x33cd_831e_6010_9289;
    (0..STARS)
        .map(|i| {
            // At the anchor the apparent angular radius satisfies sin(a)=sech(d).
            let distance = 6.0 + 0.9 * samples.next();
            // An upward plane with this normal has minimum Earth separation
            // acosh(cosh(distance)*vertical). Keep every plane above Earth.
            let min_vertical = 2.0_f64.cosh() / distance.cosh();
            // Uniform azimuth and vertical direction cosine give uniform solid angle.
            let vertical = min_vertical + (1.0 - min_vertical) * samples.next();
            let azimuth = TAU * samples.next();
            let local = Hyperboloid3::rotate_y(azimuth)
                .chain(Hyperboloid3::rotate_z(vertical.acos()))
                .chain(Hyperboloid3::shift_y(distance));
            let temperature = samples.next() as f32;
            let color = Vec3::new(0.72, 0.83, 1.0).lerp(Vec3::new(1.0, 0.87, 0.67), temperature);
            let intensity = 0.005 + 0.12 * (samples.next() as f32).powi(3);
            Ok(Star {
                pose: anchor.chain(&Pose::from_isometry(local)?)?,
                radiance: color * intensity,
                threshold: (i as f64 + samples.next()) / STARS as f64,
            })
        })
        .collect()
}

impl Star {
    fn visibility(&self, weight: f64) -> f32 {
        if weight <= 0.0 || weight >= 1.0 {
            return weight.clamp(0.0, 1.0) as f32;
        }
        // A fraction 'weight' of the uniformly spaced thresholds is active.
        // Centering the smooth transition on each threshold preserves that
        // fraction in expectation as well as fading individual stars gently.
        let width = 0.55 * weight * (1.0 - weight);
        let t = (0.5 + 0.5 * (weight - self.threshold) / width).clamp(0.0, 1.0);
        (t * t * (3.0 - 2.0 * t)) as f32
    }
}

#[derive(Default)]
pub(super) struct Stars {
    catalogues: BTreeMap<Cell, Vec<Star>>,
}

impl Stars {
    pub fn append_visible(
        &mut self,
        neighbors: [(Cell, f64); 4],
        planes: &mut Vec<Object<InfinitePlane>>,
    ) -> Result<()> {
        self.catalogues
            .retain(|cell, _| neighbors.iter().any(|(node, _)| node == cell));
        for (node, weight) in neighbors {
            if weight <= 0.0 {
                continue;
            }
            let stars = match self.catalogues.entry(node) {
                std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
                std::collections::btree_map::Entry::Vacant(entry) => entry.insert(catalogue(node)?),
            };
            for star in stars.iter() {
                let visibility = star.visibility(weight);
                if visibility > 0.0 {
                    planes.push(Object {
                        shape: InfinitePlane,
                        pose: star.pose,
                        material: Material::Emissive(star.radiance * visibility),
                    });
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::ground_cells::Location;

    #[test]
    fn catalogues_reproduce_distant_earth_disjoint_planes() -> Result<()> {
        let node = Cell {
            band: -3,
            column: 6,
        };
        let a = catalogue(node)?;
        let b = catalogue(node)?;
        for (star, same) in a.into_iter().zip(b) {
            assert_eq!(star.pose.components()?, same.pose.components()?);
            assert_eq!(star.radiance, same.radiance);
            let normal = star.pose.apply_vector([0.0, 0.0, 1.0, 0.0]);
            // Unit spacelike plane normals with mutual product > 1 are ultraparallel.
            assert!(normal[2] > 1.0);
            assert_eq!(star.visibility(0.0), 0.0);
            assert_eq!(star.visibility(1.0), 1.0);
        }
        Ok(())
    }

    #[test]
    fn crossing_grid_boundaries_preserves_catalogue_weights() -> Result<()> {
        let node = Cell {
            band: 2,
            column: -5,
        };
        let anchor = node.root(0.0, 0.0)?;
        // Approach the node from opposite sides of both irregular rows.
        for sign in [-1.0, 1.0] {
            let near = anchor.move_local([sign * 1e-7, 0.0, sign * 2e-7], [0.0; 3], 1.0)?;
            let eye = near.move_local([0.0, 0.1, 0.0], [0.5, -0.7, 0.0], 1.0)?;
            let neighbors = Location::at(eye)?.neighbors()?;
            assert!((neighbors.iter().map(|entry| entry.1).sum::<f64>() - 1.0).abs() < 1e-12);
            assert!(
                neighbors
                    .iter()
                    .any(|&(key, weight)| key == node && weight > 0.99999)
            );
        }
        Ok(())
    }
}
