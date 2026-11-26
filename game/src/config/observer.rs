use anyhow::{Result, ensure};
use serde::Deserialize;

/// Distances are in curvature radii and times are in seconds.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Observer {
    pub spawn_ground: [f64; 2],
    pub yaw_degrees: f64,
    pub pitch_degrees: f64,
    pub pitch_limit_degrees: f64,
    pub height: f64,
    pub walk_speed: f64,
    pub jump_height: f64,
    pub gravity: f64,
}

impl Observer {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.spawn_ground.into_iter().all(f64::is_finite) && self.yaw_degrees.is_finite(),
            "observer.spawn_ground and yaw_degrees must be finite"
        );
        ensure!(
            (0.0..90.0).contains(&self.pitch_limit_degrees)
                && self.pitch_degrees.abs() <= self.pitch_limit_degrees,
            "observer.pitch_limit_degrees must be in [0, 90) and include the initial pitch"
        );
        ensure!(
            self.height.is_finite() && self.height > 0.0,
            "observer.height must be finite and positive"
        );
        ensure!(
            self.walk_speed.is_finite() && self.walk_speed >= 0.0,
            "observer.walk_speed must be finite and nonnegative"
        );
        ensure!(
            self.gravity.is_finite() && self.gravity > 0.0,
            "observer.gravity must be finite and positive"
        );
        ensure!(
            self.jump_height.is_finite() && self.jump_height >= 0.0,
            "observer.jump_height must be finite and nonnegative"
        );
        let [x, z] = self.spawn_ground;
        ensure!(
            (x.cosh() * z.cosh() * (self.height + self.jump_height).cosh()).is_finite(),
            "observer spawn and peak eye position exceed f64 hyperboloid coordinates"
        );
        ensure!(
            self.flight_time().is_finite() && (self.jump_height == 0.0 || self.flight_time() > 0.0),
            "observer.jump_height and gravity give an unrepresentable flight time"
        );
        Ok(())
    }

    pub fn flight_time(&self) -> f64 {
        // T = 2 sqrt(2 h / g). Taking the roots first keeps h/g in range.
        2.0 * std::f64::consts::SQRT_2 * self.jump_height.sqrt() / self.gravity.sqrt()
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct View {
    pub window_size: [u32; 2],
    pub fov_degrees: f64,
    pub min_fov_degrees: f64,
    pub max_fov_degrees: f64,
    pub near_distance: f64,
    pub look_sensitivity: f64,
    pub zoom_sensitivity: f64,
    pub max_frame_seconds: f64,
}

impl View {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.window_size.into_iter().all(|size| size > 0),
            "view.window_size must have positive dimensions"
        );
        ensure!(
            self.min_fov_degrees > 0.0
                && self.max_fov_degrees < 180.0
                && self.min_fov_degrees <= self.fov_degrees
                && self.fov_degrees <= self.max_fov_degrees,
            "view fields of view must satisfy 0 < min_fov_degrees <= fov_degrees <= max_fov_degrees < 180"
        );
        let near = self.near_distance.tanh() as f32;
        ensure!(
            near > 0.0 && near < 1.0,
            "view.near_distance must give a positive clipping distance inside the f32 Klein ball"
        );
        for degrees in [self.min_fov_degrees, self.max_fov_degrees] {
            let projection = crate::projection::camera_matrix(
                self.window_size.into(),
                Self::tan_half_fov(degrees),
                self.near_distance,
            );
            ensure!(
                projection.is_finite() && projection.x_axis.x > 0.0 && projection.y_axis.y > 0.0,
                "view field of view is outside f32 projection range"
            );
        }
        ensure!(
            self.look_sensitivity.is_finite() && self.look_sensitivity >= 0.0,
            "view.look_sensitivity must be finite and nonnegative"
        );
        ensure!(
            self.zoom_sensitivity.is_finite() && self.zoom_sensitivity >= 0.0,
            "view.zoom_sensitivity must be finite and nonnegative"
        );
        ensure!(
            self.max_frame_seconds.is_finite() && self.max_frame_seconds > 0.0,
            "view.max_frame_seconds must be finite and positive"
        );
        Ok(())
    }

    pub fn tan_half_fov(degrees: f64) -> f64 {
        (0.5 * degrees.to_radians()).tan()
    }
}
