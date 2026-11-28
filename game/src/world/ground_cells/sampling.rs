use std::f64::consts::TAU;

use anyhow::{Context, Result};
use ccgeom::Space3;

use crate::{projection::Pose, world::Samples};

use super::tile::{circumradius, inradius, side_direction};

pub(in crate::world) struct RootSampler {
    sides: [[f64; 2]; 5],
    klein_inradius: f64,
    disk_area_factor: f64,
}

impl RootSampler {
    pub(in crate::world) fn new() -> Self {
        Self {
            sides: std::array::from_fn(|side| side_direction(side as u8)),
            klein_inradius: inradius().tanh(),
            disk_area_factor: circumradius().cosh() - 1.0,
        }
    }

    /// Sample in the bounded coordinates of the upright tile frame.
    pub(in crate::world) fn local_root(&self, samples: &mut Samples) -> Result<Pose> {
        loop {
            // A disk of radius r has area 2π(cosh(r) - 1) in curvature units.
            let w = 1.0 + self.disk_area_factor * samples.next();
            // Seeded azimuths run clockwise in the tile's local XY plane.
            let (y, x) = (-TAU * samples.next()).sin_cos();
            let sinh_r = ((w - 1.0) * (w + 1.0)).sqrt();
            // In the local Klein disk the five sides are straight lines.
            if self
                .sides
                .iter()
                .all(|[nx, ny]| (nx * x + ny * y) * sinh_r <= self.klein_inradius * w)
            {
                let shift = Space3::<f64, -1>::unit()
                    .translation([x, y, 0.0].into(), w.acosh())
                    .context("invalid pentagon site")?;
                return Pose::from_isometry(shift);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::TileId;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn tile_addresses_and_seeded_root_bearings_run_clockwise_from_x() -> Result<()> {
        for side in 0..5 {
            let tile = TileId::default().neighbor(side);
            let [_, x, y, _] = tile.frame()?.apply_vector([1.0, 0.0, 0.0, 0.0]);
            let bearing = y.atan2(x);
            assert!((bearing + f64::from(side) * TAU / 5.0).cos() > 1.0 - 1e-12);
        }

        let mut samples = Samples(3);
        let mut expected = Samples(samples.0);
        let w = 1.0 + (circumradius().cosh() - 1.0) * expected.next();
        // A candidate inside the inscribed disk always consumes just these two draws.
        assert!(w < inradius().cosh());
        let bearing = -TAU * expected.next();
        let root = RootSampler::new().local_root(&mut samples)?;
        let [_, x, y, _] = root.apply_vector([1.0, 0.0, 0.0, 0.0]);
        assert!((y.atan2(x) - bearing).cos() > 1.0 - 1e-12);
        assert_eq!(samples.0, expected.0);
        Ok(())
    }

    #[test]
    fn roots_are_area_uniform_inside_the_pentagon() -> Result<()> {
        let tile = TileId::default();
        let sampler = RootSampler::new();
        let mut samples = Samples(29);
        let probe_radius: f64 = 0.45;
        let mut inside_probe = 0;
        const SITES: usize = 16384;
        for _ in 0..SITES {
            let root = sampler.local_root(&mut samples)?;
            let [w, _, _, _] = root.apply_vector([1.0, 0.0, 0.0, 0.0]);
            inside_probe += usize::from(w < probe_radius.cosh());
            assert_eq!(TileId::at(root)?, tile);
        }
        // The probe disk lies inside all five sides. The area of the
        // pentagon is its angle defect, 3π - 5π/2 = π/2.
        let expected = TAU * (probe_radius.cosh() - 1.0) / FRAC_PI_2;
        let fraction = inside_probe as f64 / SITES as f64;
        assert!(
            (fraction - expected).abs() < 0.015,
            "{fraction} vs {expected}"
        );
        Ok(())
    }

    #[test]
    fn odd_and_even_tiles_keep_roots_on_earth_and_plants_upright() -> Result<()> {
        let odd = TileId::default().neighbor(1);
        let even = odd.neighbor(3);
        for tile in [odd, even] {
            let sampler = RootSampler::new();
            let frame = tile.upright_frame()?;
            let mut samples = Samples(tile.seed(17));
            for _ in 0..32 {
                let root = frame.chain(&sampler.local_root(&mut samples)?)?;
                assert_eq!(TileId::at(root)?, tile);
                assert!(root.apply_vector([1.0, 0.0, 0.0, 0.0])[3].abs() < 1e-12);
                let [w, x, y, z] = root.apply_vector([0.0, 0.0, 0.0, 1.0]);
                assert!(w.abs() + x.abs() + y.abs() + (z - 1.0).abs() < 1e-12);
            }
        }
        Ok(())
    }
}
