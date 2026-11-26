//! Walking on the geodesic Earth plane (embedded Y = 0), in curvature-radius units.
use anyhow::{Context, Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3, Space3};

use crate::projection::Pose;

const EYE_HEIGHT: f64 = 0.01875;
const WALK_SPEED: f64 = 0.1;
const MAX_PITCH: f64 = 89.0_f64.to_radians();
// At a human eye height of about 1.65 m, six eye heights/s² is close to Earth's gravity.
const GRAVITY: f64 = 6.0 * EYE_HEIGHT;
const JUMP_SPEED: f64 = 1.75 * EYE_HEIGHT;
const FLIGHT_TIME: f64 = 2.0 * JUMP_SPEED / GRAVITY;

#[derive(Clone, Copy)]
pub(crate) struct Walker {
    /// Maps the origin to the feet and local +Y to the Earth normal.
    ground: Pose,
    pitch: f64,
    /// Seconds since takeoff, or None while standing on the surface.
    jump_time: Option<f64>,
}

impl Walker {
    pub(crate) fn new(ground: Pose, pitch: f64) -> Result<Self> {
        let earth_normal = [0.0, 0.0, 1.0, 0.0];
        let normal = ground.apply_vector(earth_normal);
        ensure!(
            normal
                .iter()
                .zip(earth_normal)
                .all(|(a, b)| (a - b).abs() < 1e-8),
            "initial ground frame must preserve Earth's oriented plane"
        );
        ensure!(pitch.is_finite(), "invalid initial pitch");
        Ok(Self {
            ground,
            pitch: pitch.clamp(-MAX_PITCH, MAX_PITCH),
            jump_time: None,
        })
    }

    pub(crate) fn eye_pose(&self) -> Result<Pose> {
        // h(t) = v₀t − gt²/2 is physical distance along the normal geodesic.
        let jump = self
            .jump_time
            .map_or(0.0, |t| t * (JUMP_SPEED - 0.5 * GRAVITY * t));
        self.ground
            .move_local([0.0, EYE_HEIGHT + jump, 0.0], [self.pitch, 0.0, 0.0], 1.0)
    }

    pub(crate) fn is_airborne(&self) -> bool {
        self.jump_time.is_some()
    }

    /// Horizontal axes are fractions of full walking speed. Look deltas are
    /// radians, and jump is a fresh press. Returns whether the state changed.
    pub(crate) fn step(
        &mut self,
        strafe: f64,
        forward: f64,
        yaw: f64,
        pitch: f64,
        jump: bool,
        dt: f64,
    ) -> Result<bool> {
        ensure!(
            [strafe, forward, yaw, pitch, dt]
                .into_iter()
                .all(f64::is_finite)
                && dt >= 0.0,
            "invalid walking input or elapsed time"
        );
        let pitch = (self.pitch + pitch).clamp(-MAX_PITCH, MAX_PITCH);
        let jump_time = self.jump_time.or(jump.then_some(0.0)).and_then(|t| {
            let elapsed = t + dt;
            (elapsed < FLIGHT_TIME).then_some(elapsed)
        });
        let magnitude = strafe.hypot(forward);
        let distance = WALK_SPEED * dt * magnitude.min(1.0);
        if distance == 0.0 && yaw == 0.0 && pitch == self.pitch && jump_time == self.jump_time {
            return Ok(false);
        }

        let mut ground = self.ground;
        if distance > 0.0 {
            // A single in-plane boost parallel-transports the walking frame
            // along the geodesic, including when moving diagonally.
            let axis = [strafe / magnitude, 0.0, -forward / magnitude];
            let step = Space3::<f64, -1>::unit()
                .translation(axis.into(), distance)
                .context("invalid ground displacement")?;
            ground = ground.chain(&Pose::from_isometry(step)?)?;
        }
        if yaw != 0.0 {
            ground = ground.chain(&Pose::from_isometry(Hyperboloid3::rotate_y(yaw))?)?;
        }
        *self = Self {
            ground,
            pitch,
            jump_time,
        };
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    const ORIGIN: [f64; 4] = [1.0, 0.0, 0.0, 0.0];

    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 2e-11, "{actual} != {expected}");
    }

    fn height(walker: Walker) -> Result<f64> {
        Ok(walker.eye_pose()?.apply_vector(ORIGIN)[2].asinh())
    }

    #[test]
    fn diagonal_walk_is_one_ground_geodesic_and_independent_of_pitch() -> Result<()> {
        for pitch in [-1.3, 0.0, 1.2] {
            for (strafe, forward) in [(0.0_f64, 1.0_f64), (1.0, 1.0), (-1.0, -1.0)] {
                let mut walker = Walker::new(Pose::identity(), pitch)?;
                walker.step(strafe, forward, 0.0, 0.0, false, 1.0)?;
                let [w, x, y, z] = walker.ground.apply_vector(ORIGIN);
                let magnitude = strafe.hypot(forward);
                close(w.acosh(), WALK_SPEED);
                close(x, WALK_SPEED.sinh() * strafe / magnitude);
                close(y, 0.0);
                close(z, -WALK_SPEED.sinh() * forward / magnitude);
                close(height(walker)?, EYE_HEIGHT);
            }
        }
        Ok(())
    }

    #[test]
    fn yaw_turns_the_ground_heading_and_pitch_keeps_the_horizon_level() -> Result<()> {
        let mut walker = Walker::new(Pose::identity(), 0.0)?;
        walker.step(0.0, 0.0, FRAC_PI_2, 0.0, false, 0.0)?;
        walker.step(0.0, 1.0, 0.0, 0.0, false, 0.8)?;
        close(
            walker.ground.apply_vector(ORIGIN)[1],
            -(WALK_SPEED * 0.8).sinh(),
        );

        for pitch in [0.5, 4.0, -8.0] {
            walker.step(0.4, 0.7, -0.3, pitch, false, 0.2)?;
            let eye = walker.eye_pose()?;
            let right = eye.apply_vector([0.0, 1.0, 0.0, 0.0]);
            let up = eye.apply_vector([0.0, 0.0, 1.0, 0.0]);
            // A tangent's Y component divided by cosh(h) is its dot product
            // with the local upward unit normal at height h.
            close(right[2], 0.0);
            close(up[2] / EYE_HEIGHT.cosh(), walker.pitch.cos());
            assert!(up[2] > 0.0 && walker.pitch.abs() < FRAC_PI_2);
        }
        Ok(())
    }

    #[test]
    fn looking_down_keeps_the_ground_beneath_the_walker_in_view() -> Result<()> {
        let walker = Walker::new(Pose::identity(), -FRAC_PI_2)?;
        let [w, x, y, z] = walker.eye_pose()?.inverse()?.apply_vector(ORIGIN);
        let ground = wgame::glam::Vec4::new(x as f32, y as f32, z as f32, w as f32);
        let clip = crate::projection::camera_matrix((640, 480), 0.66) * ground;
        assert!(clip.z > 0.0 && clip.z < clip.w);
        assert!(clip.x.abs() < clip.w && clip.y.abs() < clip.w);
        Ok(())
    }

    #[test]
    fn jump_follows_normal_ballistics_and_lands_under_variable_steps() -> Result<()> {
        let mut walker = Walker::new(Pose::identity(), 0.3)?;
        assert!(!walker.step(0.0, 0.0, 0.0, 0.0, false, 0.1)?);
        walker.step(0.0, 1.0, 0.0, 0.0, true, FLIGHT_TIME / 4.0)?;
        // Another press in the air does not restart the jump.
        walker.step(0.0, 1.0, 0.0, 0.0, true, FLIGHT_TIME / 4.0)?;
        close(
            height(walker)?,
            EYE_HEIGHT + JUMP_SPEED.powi(2) / (2.0 * GRAVITY),
        );
        close(
            walker.ground.apply_vector(ORIGIN)[0].acosh(),
            WALK_SPEED * FLIGHT_TIME / 2.0,
        );
        assert!(walker.step(0.0, 0.0, 0.0, 0.0, false, FLIGHT_TIME)?);
        close(height(walker)?, EYE_HEIGHT);
        assert!(!walker.step(0.0, 0.0, 0.0, 0.0, false, 1.0)?);
        assert!(walker.step(0.0, 0.0, 0.0, 0.0, true, 0.01)?);
        assert!(height(walker)? > EYE_HEIGHT);
        Ok(())
    }
}
