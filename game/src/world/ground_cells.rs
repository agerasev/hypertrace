//! Equal-area cells on the Earth plane in upper-half-space coordinates (u, 0, h).
use anyhow::{Context, Result, ensure};
use ccgeom::Space3;

use crate::{config::vegetation::GroundGrid, projection::Pose};

fn band_index(band: f64) -> Result<i32> {
    let index = band.floor();
    ensure!(
        (f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&index),
        "ground band exceeds range"
    );
    Ok(index as i32)
}

impl GroundGrid {
    pub(crate) fn validate(&self, sites_per_species: u32) -> Result<()> {
        ensure!(
            [self.view_radius, self.log_height, self.width]
                .into_iter()
                .all(|v| v.is_finite() && v > 0.0),
            "view_radius, log_height and width must be finite and positive"
        );
        // Each band intersects at most ceil(diameter / cell width) + 1 columns,
        // irrespective of the position of the observer within its column.
        let sites_per_cell = (3.0 * f64::from(sites_per_species)).max(1.0);
        ensure!(self.max_patch_sites > 0, "max_patch_sites must be positive");
        let mut budget = f64::from(self.max_patch_sites);
        self.visit_bands(|_, half_width, cell_width| {
            budget -= ((2.0 * half_width / cell_width).ceil() + 1.0) * sites_per_cell;
            ensure!(
                budget >= 0.0,
                "vegetation patch exceeds max_patch_sites ({})",
                self.max_patch_sites
            );
            Ok(())
        })
    }

    fn visit_bands(&self, mut visit: impl FnMut(i32, f64, f64) -> Result<()>) -> Result<()> {
        // In units of this band's lower height, the center is at exp(log_height/2).
        // Any corner is at most half a band and half a width away by walking
        // vertically then horizontally. A hyperbolic disk maps to a chart circle.
        let log_eye = 0.5 * self.log_height;
        let radius = self.view_radius + 0.5 * (self.log_height + self.width);
        let log_bottom = log_eye - radius;
        let log_top = log_eye + radius;
        let log_center = log_eye + radius.cosh().ln();
        ensure!(
            log_top.exp().is_finite(),
            "ground neighborhood exceeds chart range"
        );
        let first = band_index(0.5 - radius / self.log_height)?;
        let last = band_index(0.5 + radius / self.log_height)?;
        ensure!(
            i64::from(last) - i64::from(first) < i64::from(self.max_patch_sites),
            "ground band count exceeds max_patch_sites ({})",
            self.max_patch_sites
        );
        for offset in first..=last {
            let log_low = f64::from(offset) * self.log_height;
            let log_high = (f64::from(offset) + 1.0) * self.log_height;
            let width = log_low.exp() * self.width;
            ensure!(
                width.is_finite() && width > 0.0,
                "ground band exceeds chart range"
            );
            // At height h, the chord has half-width sqrt((h - bottom) * (top - h)).
            // Use log ratios so a small lower height survives even for a large circle.
            let log_h = log_center
                .clamp(log_low, log_high)
                .clamp(log_bottom, log_top);
            let lower = -(log_bottom - log_h).exp_m1();
            let upper = -(log_h - log_top).exp_m1();
            let dx = (0.5 * (log_h + log_top)).exp() * lower.sqrt() * upper.sqrt();
            visit(offset, dx, width)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Cell {
    pub band: i32,
    pub column: i64,
}

fn column_index(column: f64) -> Result<i64> {
    // Retain at least 12 fraction bits in the cell for subcell positions.
    ensure!(
        column.abs() < (1_u64 << 40) as f64,
        "ground cell exceeds f64 sampling precision"
    );
    Ok(column.floor() as i64)
}

impl Cell {
    /// The cell under the eye. A normal displacement (eye height or jump) and
    /// any change of view direction leave the perpendicular ground foot fixed.
    pub fn at(eye: Pose, grid: &GroundGrid) -> Result<Self> {
        let [w, x, y, z] = eye.apply_vector([1.0, 0.0, 0.0, 0.0]);
        let scale = y.hypot(1.0);
        let (w, x, z) = (w / scale, x / scale, z / scale);
        // The foot is (w,x,0,z), with w²-z² = 1+x². Rationalize 1/(w-z)
        // on the +z side where the two components can round to the same value.
        let h = if z > 0.0 {
            let norm = x.hypot(1.0);
            (w / norm + z / norm) / norm
        } else {
            1.0 / (w - z)
        };
        ensure!(
            h.is_finite() && h > 0.0,
            "ground height exceeds chart range"
        );
        let log_h = h.ln();
        let band = band_index(log_h / grid.log_height)?;
        // x is u/h. Work with the height ratio to avoid forming a large absolute u.
        let column =
            column_index(x * (log_h - f64::from(band) * grid.log_height).exp() / grid.width)?;
        Ok(Self { band, column })
    }

    /// All cells within the configured radius of any point in this cell, plus a margin.
    /// The result is independent of where the observer stands inside the cell.
    pub fn neighborhood(self, grid: &GroundGrid) -> Result<Vec<Self>> {
        let center_u = (self.column as f64 + 0.5) * grid.width;
        let mut cells = Vec::new();
        grid.visit_bands(|offset, dx, width| {
            let left = column_index((center_u - dx) / width)?;
            let right = column_index((center_u + dx) / width)?;
            let band = self
                .band
                .checked_add(offset)
                .context("ground band exceeds range")?;
            let count = usize::try_from(right - left + 1)?;
            ensure!(
                count <= grid.max_patch_sites as usize - cells.len(),
                "ground cell count exceeds max_patch_sites ({})",
                grid.max_patch_sites
            );
            cells.try_reserve(count)?;
            cells.extend((left..=right).map(|column| Self { band, column }));
            Ok(())
        })?;
        Ok(cells)
    }

    /// Sample a root from two independent uniform fractions in [0,1).
    /// Each cell has intrinsic area width*(1-exp(-log_height)). Since
    /// dA=du dh/h², sampling u and reciprocal height uniformly is area-uniform.
    pub fn root(self, across: f64, area: f64, grid: &GroundGrid) -> Result<Pose> {
        ensure!(
            (0.0..1.0).contains(&across) && (0.0..1.0).contains(&area),
            "invalid ground sample"
        );
        let low = (f64::from(self.band) * grid.log_height).exp();
        let u = (self.column as f64 + across) * grid.width * low;
        let h = low / (1.0 + area * (-grid.log_height).exp_m1());
        let space = Space3::<f64, -1>::unit();
        let point = space
            .point_from_half_space([u, 0.0, h].into())
            .context("invalid ground site")?;
        let (x, z) = (point[1], point[3]);
        let radial = x.hypot(z);
        if radial == 0.0 {
            return Ok(Pose::identity());
        }
        // The origin-to-site boost is tangent to Earth and fixes its +Y normal.
        let map = space
            .translation([x / radial, 0.0, z / radial].into(), radial.asinh())
            .context("ground site exceeds isometry range")?;
        Pose::from_isometry(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRID: GroundGrid = GroundGrid {
        log_height: 0.27,
        width: 0.33,
        view_radius: 1.4,
        max_patch_sites: 4096,
    };

    #[test]
    fn invalid_patch_sizes_fail_before_allocation() {
        for grid in [
            GroundGrid {
                view_radius: 1e5,
                ..GRID
            },
            GroundGrid {
                log_height: 1e-30,
                ..GRID
            },
            GroundGrid {
                width: 1e-30,
                ..GRID
            },
            GroundGrid {
                width: f64::NAN,
                ..GRID
            },
        ] {
            assert!(grid.validate(1).is_err());
        }
        assert!(
            GroundGrid {
                max_patch_sites: 0,
                ..GRID
            }
            .validate(0)
            .is_err()
        );
        assert!(GRID.validate(u32::MAX).is_err());
        assert!(GRID.validate(0).is_ok());
    }

    #[test]
    fn raising_the_budget_accepts_a_denser_patch() {
        assert!(GRID.validate(100).is_err());
        let larger_budget = GroundGrid {
            max_patch_sites: 100_000,
            ..GRID
        };
        assert!(larger_budget.validate(100).is_ok());
    }

    #[test]
    fn distant_low_band_reaches_the_intrinsic_disk_boundary() -> Result<()> {
        let grid = GroundGrid {
            view_radius: 19.35,
            log_height: 0.3,
            width: 1.0,
            ..GRID
        };
        let eye_h = (0.5 * grid.log_height).exp();
        let radius = grid.view_radius + 0.5 * (grid.log_height + grid.width);
        let mut lowest = None;
        grid.visit_bands(|offset, dx, _| {
            lowest.get_or_insert((offset, dx));
            Ok(())
        })?;
        let (offset, dx) = lowest.unwrap();
        let h = ((f64::from(offset) + 1.0) * grid.log_height).exp();
        let cosh_distance = 1.0 + (dx * dx + (eye_h - h).powi(2)) / (2.0 * h * eye_h);
        assert!((cosh_distance / radius.cosh() - 1.0).abs() < 1e-12);
        Ok(())
    }

    #[test]
    fn cell_lookup_resolves_distant_positive_height() -> Result<()> {
        let space = Space3::<f64, -1>::unit();
        let eye = Pose::from_isometry(space.translation([0.0, 0.0, 1.0].into(), 20.0).unwrap())?;
        let cell = Cell::at(eye, &GRID)?;
        assert_eq!(cell.band, (20.0 / GRID.log_height).floor() as i32);
        assert_eq!(cell.column, 0);
        Ok(())
    }

    #[test]
    fn roots_sample_intrinsic_area_and_fix_the_earth_normal() -> Result<()> {
        let space = Space3::<f64, -1>::unit();
        for cell in [
            Cell {
                band: -7,
                column: -17,
            },
            Cell {
                band: 9,
                column: 13,
            },
        ] {
            let low = (f64::from(cell.band) * GRID.log_height).exp();
            for (across, area) in [(0.1, 0.9), (0.8, 0.2), (0.5, 0.5)] {
                let root = cell.root(across, area, &GRID)?;
                let normal = root.apply_vector([0.0, 0.0, 1.0, 0.0]);
                assert!((normal[2] - 1.0).abs() < 1e-12);
                assert_eq!([normal[0], normal[1], normal[3]], [0.0; 3]);
                let point = root.apply_vector([1.0, 0.0, 0.0, 0.0]);
                let chart = space.point_to_half_space(point.into());
                assert!((chart[0] / (GRID.width * low) - cell.column as f64 - across).abs() < 1e-9);
                assert!(
                    ((1.0 - low / chart[2]) / (-(-GRID.log_height).exp_m1()) - area).abs() < 1e-9
                );
                assert_eq!(Cell::at(root, &GRID)?, cell);
                let eye = root.move_local([0.0, 0.2, 0.0], [0.2, 1.3, 0.0], 1.0)?;
                assert_eq!(Cell::at(eye, &GRID)?, cell);
            }
        }
        Ok(())
    }

    #[test]
    fn neighborhood_covers_ground_travel_from_every_corner() -> Result<()> {
        let space = Space3::<f64, -1>::unit();
        for cell in [
            Cell {
                band: -12,
                column: -20,
            },
            Cell {
                band: 7,
                column: 14,
            },
        ] {
            let cells = cell.neighborhood(&GRID)?;
            assert!(cells.windows(2).all(|w| w[0] < w[1]));
            for across in [0.001, 0.999] {
                for area in [0.001, 0.999] {
                    let start = cell.root(across, area, &GRID)?;
                    for i in 0..32 {
                        let theta = f64::from(i) * std::f64::consts::TAU / 32.0;
                        let axis = [theta.cos(), 0.0, theta.sin()];
                        let step = Pose::from_isometry(
                            space.translation(axis.into(), GRID.view_radius).unwrap(),
                        )?;
                        let end = Cell::at(start.chain(&step)?, &GRID)?;
                        assert!(cells.binary_search(&end).is_ok(), "{cell:?} -> {end:?}");
                    }
                }
            }
        }
        Ok(())
    }
}
