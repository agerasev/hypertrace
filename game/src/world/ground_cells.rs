//! Equal-area cells on the Earth plane in upper-half-space coordinates (u, 0, h).
use anyhow::{Context, Result, ensure};
use ccgeom::Space3;

use crate::projection::Pose;

const LOG_HEIGHT: f64 = 0.3;
const WIDTH: f64 = 0.3; // Chart width as a fraction of the band's lower height.
const VIEW_RADIUS: f64 = 1.5; // Curvature radii.

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

pub(super) struct Location {
    pub cell: Cell,
    column: f64,
    band_fraction: f64,
}

impl Location {
    /// The cell under the eye. A normal displacement (eye height or jump) and
    /// any change of view direction leave the perpendicular ground foot fixed.
    pub fn at(eye: Pose) -> Result<Self> {
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
        let band = (log_h / LOG_HEIGHT).floor() as i32;
        // x is u/h. Work with the height ratio to avoid forming a large absolute u.
        let column = x * (log_h - f64::from(band) * LOG_HEIGHT).exp() / WIDTH;
        Ok(Self {
            cell: Cell {
                band,
                column: column_index(column)?,
            },
            column,
            band_fraction: log_h / LOG_HEIGHT - f64::from(band),
        })
    }

    /// Bilinear interpolation between grid nodes. The upper band's columns
    /// have different chart widths, so determine their indices independently.
    pub fn neighbors(&self) -> Result<[(Cell, f64); 4]> {
        let row = |band, column: f64, weight| -> Result<_> {
            let left = column_index(column)?;
            let fraction = column - left as f64;
            Ok([
                (Cell { band, column: left }, weight * (1.0 - fraction)),
                (
                    Cell {
                        band,
                        column: left + 1,
                    },
                    weight * fraction,
                ),
            ])
        };
        let [a, b] = row(self.cell.band, self.column, 1.0 - self.band_fraction)?;
        let [c, d] = row(
            self.cell.band + 1,
            self.column * (-LOG_HEIGHT).exp(),
            self.band_fraction,
        )?;
        Ok([a, b, c, d])
    }
}

impl Cell {
    /// All cells within VIEW_RADIUS of any point in this cell, plus a margin.
    /// The result is independent of where the observer stands inside the cell.
    pub fn neighborhood(self) -> Result<Vec<Self>> {
        // In units of this band's lower height, the center has h=exp(LOG_HEIGHT/2).
        // The farthest corner is at most half a band and half a width away by
        // walking vertically then horizontally (intrinsic metric ds=|d(u,h)|/h).
        let center_u = (self.column as f64 + 0.5) * WIDTH;
        let center_h = (0.5 * LOG_HEIGHT).exp();
        let radius = VIEW_RADIUS + 0.5 * (LOG_HEIGHT + WIDTH);
        // A hyperbolic disk maps to a chart circle centered directly above its eye.
        let circle_h = center_h * radius.cosh();
        let circle_r = center_h * radius.sinh();
        let first = (0.5 - radius / LOG_HEIGHT).floor() as i32;
        let last = (0.5 + radius / LOG_HEIGHT).floor() as i32;
        let mut cells = Vec::new();
        for offset in first..=last {
            let low = (f64::from(offset) * LOG_HEIGHT).exp();
            let high = (f64::from(offset + 1) * LOG_HEIGHT).exp();
            let dy = circle_h - circle_h.clamp(low, high);
            let dx = ((circle_r - dy) * (circle_r + dy)).max(0.0).sqrt();
            let width = low * WIDTH;
            let left = column_index((center_u - dx) / width)?;
            let right = column_index((center_u + dx) / width)?;
            let band = self
                .band
                .checked_add(offset)
                .context("ground band exceeds range")?;
            cells.extend((left..=right).map(|column| Self { band, column }));
        }
        Ok(cells)
    }

    /// Sample a root from two independent uniform fractions in [0,1).
    /// Each cell has intrinsic area WIDTH*(1-exp(-LOG_HEIGHT)). Since
    /// dA=du dh/h², sampling u and reciprocal height uniformly is area-uniform.
    pub fn root(self, across: f64, area: f64) -> Result<Pose> {
        ensure!(
            (0.0..1.0).contains(&across) && (0.0..1.0).contains(&area),
            "invalid ground sample"
        );
        let low = (f64::from(self.band) * LOG_HEIGHT).exp();
        let u = (self.column as f64 + across) * WIDTH * low;
        let h = low / (1.0 + area * (-LOG_HEIGHT).exp_m1());
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

    #[test]
    fn cell_lookup_resolves_distant_positive_height() -> Result<()> {
        let space = Space3::<f64, -1>::unit();
        let eye = Pose::from_isometry(space.translation([0.0, 0.0, 1.0].into(), 20.0).unwrap())?;
        let cell = Location::at(eye)?.cell;
        assert_eq!(cell.band, (20.0 / LOG_HEIGHT).floor() as i32);
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
            let low = (f64::from(cell.band) * LOG_HEIGHT).exp();
            for (across, area) in [(0.1, 0.9), (0.8, 0.2), (0.5, 0.5)] {
                let root = cell.root(across, area)?;
                let normal = root.apply_vector([0.0, 0.0, 1.0, 0.0]);
                assert!((normal[2] - 1.0).abs() < 1e-12);
                assert_eq!([normal[0], normal[1], normal[3]], [0.0; 3]);
                let point = root.apply_vector([1.0, 0.0, 0.0, 0.0]);
                let chart = space.point_to_half_space(point.into());
                assert!((chart[0] / (WIDTH * low) - cell.column as f64 - across).abs() < 1e-9);
                assert!(((1.0 - low / chart[2]) / (-(-LOG_HEIGHT).exp_m1()) - area).abs() < 1e-9);
                assert_eq!(Location::at(root)?.cell, cell);
                let eye = root.move_local([0.0, 0.2, 0.0], [0.2, 1.3, 0.0], 1.0)?;
                assert_eq!(Location::at(eye)?.cell, cell);
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
            let cells = cell.neighborhood()?;
            assert!(cells.windows(2).all(|w| w[0] < w[1]));
            for across in [0.001, 0.999] {
                for area in [0.001, 0.999] {
                    let start = cell.root(across, area)?;
                    for i in 0..32 {
                        let theta = f64::from(i) * std::f64::consts::TAU / 32.0;
                        let axis = [theta.cos(), 0.0, theta.sin()];
                        let step = Pose::from_isometry(
                            space.translation(axis.into(), VIEW_RADIUS).unwrap(),
                        )?;
                        let end = Location::at(start.chain(&step)?)?.cell;
                        assert!(cells.binary_search(&end).is_ok(), "{cell:?} -> {end:?}");
                    }
                }
            }
        }
        Ok(())
    }
}
