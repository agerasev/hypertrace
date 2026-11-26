use anyhow::{Result, ensure};

use crate::{
    config::vegetation::{LightResponse, Vegetation},
    projection::Pose,
    world::Sun,
};

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

impl LightResponse {
    fn occupancy(self, light: f64) -> f64 {
        let [dark, bright] = self.occupancy;
        let [start, end] = self.light_range;
        dark + (bright - dark) * smoothstep(start, end, light)
    }

    /// Saturating size response normalized to one at noon.
    fn growth(self, light: f64) -> f64 {
        let light = light.clamp(0.0, 1.0);
        let vigor = light * (1.0 + self.half_saturation) / (light + self.half_saturation);
        self.minimum_size + (1.0 - self.minimum_size) * vigor
    }
}

impl Species {
    fn ecology(self, settings: &Vegetation) -> LightResponse {
        match self {
            Self::Deciduous => settings.deciduous.ecology,
            Self::Pine => settings.pine.ecology,
            Self::Grass => settings.grass.ecology,
        }
    }

    /// Occupancy probability at one candidate site for this species.
    pub fn density(self, light: f64, settings: &Vegetation) -> f64 {
        let occupancy = self.ecology(settings).occupancy(light);
        if let Self::Pine = self {
            let [start, end] = settings.pine.daylight_range;
            return occupancy
                * (1.0 - settings.pine.daylight_suppression * smoothstep(start, end, light));
        }
        occupancy
    }

    /// Maximum local size as a fraction of the full-grown plant at noon.
    pub fn growth(self, light: f64, settings: &Vegetation) -> f64 {
        self.ecology(settings).growth(light)
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
    fn custom_light_response_matches_its_limits_and_smooth_midpoint() {
        let response = LightResponse {
            light_range: [0.25, 0.75],
            occupancy: [0.125, 0.875],
            minimum_size: 0.25,
            half_saturation: 0.5,
        };
        assert_eq!(response.occupancy(0.0), 0.125);
        assert_eq!(response.occupancy(0.5), 0.5);
        assert_eq!(response.occupancy(1.0), 0.875);
        assert_eq!(response.growth(0.0), 0.25);
        assert_eq!(response.growth(0.5), 0.8125);
        assert_eq!(response.growth(1.0), 1.0);
    }
}
