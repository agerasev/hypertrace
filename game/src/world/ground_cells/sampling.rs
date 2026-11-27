use std::f64::consts::TAU;

use anyhow::{Context, Result};
use ccgeom::Space3;

use crate::{projection::Pose, world::Samples};

use super::tile::{TileId, circumradius, inradius, side_direction};

pub(in crate::world) struct RootSampler {
    frame: Pose,
    sides: [[f64; 2]; 5],
    klein_inradius: f64,
    disk_area_factor: f64,
}

impl RootSampler {
    pub(in crate::world) fn new(tile: &TileId) -> Result<Self> {
        let mut frame = tile.frame()?;
        if tile.flips_normal() {
            // A half-turn about X restores Earth-up and reflects Z. The X axis
            // bisects side 0, so this reflection preserves the local pentagon.
            let half_turn = Pose::from_rows([[0.0, 1.0, 0.0, 0.0], [0.0; 4]])?;
            frame = frame.chain(&half_turn)?;
        }
        Ok(Self {
            frame,
            sides: std::array::from_fn(|side| side_direction(side as u8)),
            klein_inradius: inradius().tanh(),
            disk_area_factor: circumradius().cosh() - 1.0,
        })
    }

    pub(in crate::world) fn root(&self, samples: &mut Samples) -> Result<Pose> {
        loop {
            // A disk of radius r has area 2π(cosh(r) - 1) in curvature units.
            let w = 1.0 + self.disk_area_factor * samples.next();
            let (z, x) = (TAU * samples.next()).sin_cos();
            let sinh_r = ((w - 1.0) * (w + 1.0)).sqrt();
            // In the local Klein disk the five sides are straight lines.
            if self
                .sides
                .iter()
                .all(|[nx, nz]| (nx * x + nz * z) * sinh_r <= self.klein_inradius * w)
            {
                let shift = Space3::<f64, -1>::unit()
                    .translation([x, 0.0, z].into(), w.acosh())
                    .context("invalid pentagon site")?;
                return self.frame.chain(&Pose::from_isometry(shift)?);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn roots_are_area_uniform_inside_the_pentagon() -> Result<()> {
        let tile = TileId::default();
        let sampler = RootSampler::new(&tile)?;
        let mut samples = Samples(29);
        let probe_radius: f64 = 0.45;
        let mut inside_probe = 0;
        const SITES: usize = 16384;
        for _ in 0..SITES {
            let root = sampler.root(&mut samples)?;
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
            let sampler = RootSampler::new(&tile)?;
            let mut samples = Samples(tile.seed(17));
            for _ in 0..32 {
                let root = sampler.root(&mut samples)?;
                assert_eq!(TileId::at(root)?, tile);
                assert!(root.apply_vector([1.0, 0.0, 0.0, 0.0])[2].abs() < 1e-12);
                let [w, x, y, z] = root.apply_vector([0.0, 0.0, 1.0, 0.0]);
                assert!(w.abs() + x.abs() + (y - 1.0).abs() + z.abs() < 1e-12);
            }
        }
        Ok(())
    }
}
