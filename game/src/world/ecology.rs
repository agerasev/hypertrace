use anyhow::{Result, ensure};

use crate::{projection::Pose, world::Sun};

/// Direct light on the upper face of Earth, as a fraction of permanent noon.
pub struct Daylight {
    // Unit spacelike normal to the disjoint Sun plane in Earth's frame (W,X,Y,Z).
    sun_normal: [f64; 4],
    above_and_emitting: bool,
}

impl Daylight {
    pub fn new(earth: Pose, sun: &Sun) -> Result<Self> {
        let relative = earth.inverse()?.chain(&sun.pose)?;
        let sun_normal = relative.apply_vector([0.0, 0.0, 1.0, 0.0]);
        ensure!(
            sun_normal[2].abs() > 1.0,
            "permanent noon requires separated Earth and Sun planes"
        );
        let above = sun_normal[0].is_sign_positive() == sun_normal[2].is_sign_positive();
        let above_and_emitting = above && sun.radiance.iter().any(|&channel| channel > 0.0);
        Ok(Self {
            sun_normal,
            above_and_emitting,
        })
    }

    /// Sample a unit hyperboloid point on the local Earth plane (Y=0).
    pub fn at(&self, [w, x, _, z]: [f64; 4]) -> f64 {
        if !self.above_and_emitting {
            return 0.0;
        }
        let [nw, nx, ny, nz] = self.sun_normal;
        let sinh_distance = -nw * w + nx * x + nz * z;
        // For the full solar disk, Lambertian irradiance is proportional to
        // cosh(gap)/cosh(distance)^3. At noon distance=gap and |Ny|=cosh(gap).
        (ny.abs() / sinh_distance.hypot(1.0)).min(1.0).powi(3)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Species {
    Deciduous,
    Pine,
    Grass,
}

fn smoothstep(low: f64, high: f64, value: f64) -> f64 {
    let t = ((value - low) / (high - low)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl Species {
    /// Occupancy probability at one candidate site for this species.
    pub fn density(self, light: f64) -> f64 {
        let day = smoothstep(0.055, 0.42, light);
        match self {
            Self::Deciduous => 0.40 * day,
            Self::Pine => {
                let twilight = smoothstep(0.003, 0.10, light);
                (0.0003 + (0.30 - 0.0003) * twilight) * (1.0 - 0.55 * day)
            }
            Self::Grass => 0.006 + (0.78 - 0.006) * smoothstep(0.001, 0.19, light),
        }
    }

    /// Maximum local size as a fraction of the full-grown plant at noon.
    pub fn growth(self, light: f64) -> f64 {
        let (minimum, half_saturation) = match self {
            Self::Deciduous => (0.35, 0.24),
            Self::Pine => (0.22, 0.10),
            Self::Grass => (0.25, 0.06),
        };
        // Saturating response normalized to one at noon.
        let light = light.clamp(0.0, 1.0);
        let vigor = light * (1.0 + half_saturation) / (light + half_saturation);
        minimum + (1.0 - minimum) * vigor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daylight_follows_the_sun_across_the_ground() -> Result<()> {
        let gap: f64 = 1.2;
        let noon = Pose::identity().move_local([0.8, 0.0, 0.0], [0.0; 3], 1.0)?;
        let earth = Pose::identity().move_local([0.1, -0.4, 0.2], [0.3, 0.1, -0.4], 1.0)?;
        let sun = Sun {
            pose: earth
                .chain(&noon)?
                .move_local([0.0, gap, 0.0], [0.0; 3], 1.0)?,
            radiance: [1.0; 3],
        };
        let daylight = Daylight::new(earth, &sun)?;
        for radius in [0.0_f64, 0.8, 2.2] {
            let root = noon.move_local([0.0, 0.0, radius], [0.0; 3], 1.0)?;
            let cosh_distance = (gap.sinh() * radius.cosh()).hypot(1.0);
            let expected = (gap.cosh() / cosh_distance).powi(3);
            let actual = daylight.at(root.apply_vector([1.0, 0.0, 0.0, 0.0]));
            assert!((actual - expected).abs() < 1e-12);
        }
        let noon_point = noon.apply_vector([1.0, 0.0, 0.0, 0.0]);
        let dark = Sun {
            radiance: [0.0; 3],
            ..sun
        };
        assert_eq!(Daylight::new(earth, &dark)?.at(noon_point), 0.0);
        Ok(())
    }

    #[test]
    fn shade_favors_conifers_then_sparse_dwarf_growth() {
        use Species::*;
        assert!(Deciduous.density(1.0) > Pine.density(1.0));
        assert!(Pine.density(0.1) > Deciduous.density(0.1));
        assert_eq!(Deciduous.density(0.0), 0.0);
        assert!(Pine.density(0.0) > 0.0 && Grass.density(0.0) > Pine.density(0.0));
        assert!(Grass.density(0.0) < Grass.density(0.1));
        for species in [Deciduous, Pine, Grass] {
            let night = species.growth(0.0);
            let twilight = species.growth(0.1);
            let day = species.growth(1.0);
            assert!(0.0 < night && night < twilight && twilight < day);
            assert!((day - 1.0).abs() < 1e-12);
        }
    }
}
