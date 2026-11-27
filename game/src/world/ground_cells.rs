//! Vegetation paging on the regular right-angled pentagonal tiling of Earth.
use std::{collections::BTreeSet, f64::consts::PI};

use anyhow::{Result, ensure};

use crate::{config::vegetation::GroundGrid, projection::Pose};

mod sampling;
mod tile;

pub(super) use sampling::RootSampler;
pub(super) use tile::TileId;

type Vector = [f64; 4];

fn dot(a: Vector, b: Vector) -> f64 {
    -a[0] * b[0] + a[1] * b[1] + a[3] * b[3]
}

struct Pentagon {
    normals: [Vector; 5],
    vertices: [Vector; 5],
}

impl Pentagon {
    fn origin() -> Self {
        let r = tile::inradius();
        let c = tile::circumradius();
        Self {
            normals: std::array::from_fn(|i| {
                let [x, z] = tile::side_direction(i as u8);
                [r.sinh(), r.cosh() * x, 0.0, r.cosh() * z]
            }),
            vertices: std::array::from_fn(|i| {
                let angle = (2 * i + 1) as f64 * PI / 5.0;
                [
                    c.cosh(),
                    c.sinh() * angle.cos(),
                    0.0,
                    c.sinh() * angle.sin(),
                ]
            }),
        }
    }

    fn placed(&self, frame: Pose) -> Self {
        Self {
            normals: self.normals.map(|v| frame.apply_vector(v)),
            vertices: self.vertices.map(|v| frame.apply_vector(v)),
        }
    }

    // The two neighboring sides meet this one at right angles. Orthogonally
    // projecting onto its supporting line therefore preserves the signs of
    // both neighboring half-space tests. The foot lies between the endpoints
    // exactly when both tests accept it.
    fn projects_onto_side(&self, side: usize, v: Vector, sign: f64, tol: f64) -> bool {
        [self.normals[(side + 4) % 5], self.normals[(side + 1) % 5]]
            .into_iter()
            .all(|n| sign * dot(n, v) <= tol)
    }

    fn within(&self, other: &Self, cosh_limit: f64, sinh_limit: f64) -> bool {
        let tol = 128.0 * f64::EPSILON * cosh_limit;
        // A shortest segment joins two vertices, a vertex to a side, or the
        // interiors of two sides along their common perpendicular.
        for i in 0..5 {
            for j in 0..5 {
                let (a, b) = (self.vertices[i], other.vertices[j]);
                let (n, m) = (self.normals[i], other.normals[j]);
                if -dot(a, b) <= cosh_limit + tol
                    || (dot(a, m).abs() <= sinh_limit + tol
                        && other.projects_onto_side(j, a, 1.0, tol))
                    || (dot(b, n).abs() <= sinh_limit + tol
                        && self.projects_onto_side(i, b, 1.0, tol))
                {
                    return true;
                }
                // The outward normals at closest interior points face one
                // another with <n,m> = -cosh(d). The future feet are positive
                // multiples of -m + <n,m>n and -n + <n,m>m, respectively.
                let cosh_gap = -dot(n, m);
                if cosh_gap > 1.0 + tol
                    && cosh_gap <= cosh_limit + tol
                    && self.projects_onto_side(i, m, -1.0, tol)
                    && other.projects_onto_side(j, n, -1.0, tol)
                {
                    return true;
                }
            }
        }
        false
    }
}

/// Relative tiles touching the closed radius-neighborhood of a home pentagon.
/// The same finite list works at every home tile by symmetry.
pub(crate) struct Patch {
    offsets: Vec<TileId>,
}

impl Patch {
    pub(crate) fn new(grid: &GroundGrid, sites_per_tile: u32) -> Result<Self> {
        let radius = grid.view_radius;
        ensure!(
            radius.is_finite() && radius > 0.0,
            "view_radius must be finite and positive"
        );
        let sites = (3 * u64::from(sites_per_tile)).max(1);
        let tile_budget = u64::from(grid.max_patch_sites) / sites;
        // A pentagon has area π/2 and the patch must at least cover a disk of
        // area 4π sinh²(radius/2). Reject oversized requests before traversal.
        let min_tiles = 8.0 * (0.5 * radius).sinh().powi(2);
        let check_budget = |count: f64| -> Result<()> {
            ensure!(
                count <= tile_budget as f64,
                "vegetation patch exceeds max_patch_sites ({})",
                grid.max_patch_sites
            );
            Ok(())
        };
        check_budget(min_tiles.max(1.0))?;

        let home = Pentagon::origin();
        let cosh_limit = radius.cosh();
        let sinh_limit = radius.sinh();
        let max_center = (radius + 2.0 * tile::circumradius()).cosh();
        let mut offsets = vec![TileId::default()];
        let mut seen = BTreeSet::from([TileId::default()]);
        let mut i = 0;
        // The inflated pentagon is convex. Every intersecting tile is
        // reachable inside it by crossing sides, also at four-tile corners.
        while i < offsets.len() {
            for side in 0..5 {
                let next = offsets[i].neighbor(side);
                if !seen.insert(next.clone()) {
                    continue;
                }
                let frame = next.frame()?;
                let center = frame.apply_vector([1.0, 0.0, 0.0, 0.0]);
                if center[0] <= max_center * (1.0 + 128.0 * f64::EPSILON)
                    && (center[0] <= cosh_limit
                        || home.within(&home.placed(frame), cosh_limit, sinh_limit))
                {
                    check_budget((offsets.len() + 1) as f64)?;
                    offsets.push(next);
                }
            }
            i += 1;
        }
        Ok(Self { offsets })
    }

    pub(super) fn tiles<'a>(&'a self, center: &'a TileId) -> impl Iterator<Item = TileId> + 'a {
        self.offsets.iter().map(|offset| center.compose(offset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ccgeom::Space3;

    fn grid(radius: f64, budget: u32) -> GroundGrid {
        GroundGrid {
            view_radius: radius,
            max_patch_sites: budget,
        }
    }

    #[test]
    fn first_ring_includes_all_edge_and_corner_neighbors() -> Result<()> {
        let origin = TileId::default();
        let patch = Patch::new(&grid(1.0, 11), 0)?;
        let actual: BTreeSet<_> = patch.tiles(&origin).collect();
        let mut expected = BTreeSet::from([origin.clone()]);
        for side in 0..5 {
            let edge = origin.neighbor(side);
            expected.insert(edge.neighbor((side + 1) % 5));
            expected.insert(edge);
        }
        assert_eq!(actual.len(), 11);
        assert_eq!(actual, expected);
        assert!(Patch::new(&grid(1.1, 11), 0).is_err());
        Ok(())
    }

    #[test]
    fn neighborhood_covers_travel_from_sides_and_corners_of_moved_tile() -> Result<()> {
        let home = TileId::default().neighbor(0).neighbor(2).neighbor(4);
        let patch = Patch::new(&grid(1.5, 10_000), 1)?;
        let tiles: BTreeSet<_> = patch.tiles(&home).collect();
        let mut reached = BTreeSet::from([home.clone()]);
        assert_eq!(tiles.len(), patch.offsets.len());
        let geometry = Space3::<f64, -1>::unit();
        for radius in [0.0, tile::inradius(), tile::circumradius()] {
            for j in 0..5 {
                let azimuth = (2 * j) as f64 * PI / 5.0
                    + if radius == tile::circumradius() {
                        PI / 5.0
                    } else {
                        0.0
                    };
                let step = geometry
                    .translation([azimuth.cos(), 0.0, azimuth.sin()].into(), radius)
                    .unwrap();
                let start = home.frame()?.chain(&Pose::from_isometry(step)?)?;
                for k in 0..50 {
                    let angle = (k as f64) * 2.0 * PI / 50.0;
                    let walk = geometry
                        .translation([angle.cos(), 0.0, angle.sin()].into(), 1.5)
                        .unwrap();
                    let end = TileId::at(start.chain(&Pose::from_isometry(walk)?)?)?;
                    assert!(tiles.contains(&end));
                    reached.insert(end);
                }
            }
        }
        assert_eq!(reached, tiles);
        Ok(())
    }

    #[test]
    fn patch_budget_counts_all_species_and_empty_tiles() -> Result<()> {
        assert!(Patch::new(&grid(1.0, 10), 0).is_err());
        assert!(Patch::new(&grid(1.0, 32), 1).is_err());
        assert!(Patch::new(&grid(1e5, u32::MAX), 0).is_err());
        assert!(Patch::new(&grid(1.0, u32::MAX), u32::MAX).is_err());
        assert!(Patch::new(&grid(f64::NAN, 1000), 0).is_err());
        Patch::new(&grid(1.0, 33), 1)?;
        Ok(())
    }
}
