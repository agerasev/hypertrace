//! Walking on the geodesic Earth plane (embedded Y = 0), in curvature-radius units.
use anyhow::{Context, Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3, Space3};

use crate::{config::observer::Observer, projection::Pose};

#[derive(Clone, Copy)]
pub(crate) struct Walker {
    observer: Observer,
    /// Maps the origin to the feet and local +Y to the Earth normal.
    ground: Pose,
    pitch: f64,
    /// Seconds since takeoff, or None while standing on the surface.
    jump_time: Option<f64>,
}

impl Walker {
    pub(crate) fn new(observer: Observer) -> Result<Self> {
        observer.validate()?;
        let [x, z] = observer.spawn_ground;
        let ground = Pose::identity().move_local(
            [x, 0.0, z],
            [0.0, observer.yaw_degrees.to_radians(), 0.0],
            1.0,
        )?;
        Ok(Self {
            observer,
            ground,
            pitch: observer.pitch_degrees.to_radians(),
            jump_time: None,
        })
    }

    pub(crate) fn eye_pose(&self) -> Result<Pose> {
        // The constant-gravity parabola peaks at jump_height halfway through flight.
        let jump = self.jump_time.map_or(0.0, |t| {
            let phase = t / self.observer.flight_time();
            self.observer.jump_height * (4.0 * phase * (1.0 - phase))
        });
        self.ground.move_local(
            [0.0, self.observer.height + jump, 0.0],
            [self.pitch, 0.0, 0.0],
            1.0,
        )
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
        let limit = self.observer.pitch_limit_degrees.to_radians();
        let pitch = (self.pitch + pitch).clamp(-limit, limit);
        let jump_time = self.jump_time.or(jump.then_some(0.0)).and_then(|t| {
            let elapsed = t + dt;
            (elapsed < self.observer.flight_time()).then_some(elapsed)
        });
        let magnitude = strafe.hypot(forward);
        let distance = self.observer.walk_speed * dt * magnitude.min(1.0);
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
            ..*self
        };
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    const ORIGIN: [f64; 4] = [1.0, 0.0, 0.0, 0.0];

    fn observer(pitch: f64) -> Observer {
        Observer {
            spawn_ground: [0.0; 2],
            yaw_degrees: 0.0,
            pitch_degrees: pitch.to_degrees(),
            pitch_limit_degrees: 87.5,
            height: 0.047,
            walk_speed: 0.18,
            jump_height: 0.012,
            gravity: 0.39,
        }
    }

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
                let settings = observer(pitch);
                let mut walker = Walker::new(settings)?;
                walker.step(strafe, forward, 0.0, 0.0, false, 1.0)?;
                let [w, x, y, z] = walker.ground.apply_vector(ORIGIN);
                let magnitude = strafe.hypot(forward);
                close(w.acosh(), settings.walk_speed);
                close(x, settings.walk_speed.sinh() * strafe / magnitude);
                close(y, 0.0);
                close(z, -settings.walk_speed.sinh() * forward / magnitude);
                close(height(walker)?, settings.height);
            }
        }
        Ok(())
    }

    #[test]
    fn yaw_turns_the_ground_heading_and_pitch_keeps_the_horizon_level() -> Result<()> {
        let settings = Observer {
            spawn_ground: [0.16, -0.24],
            yaw_degrees: 35.0,
            ..observer(0.0)
        };
        let mut walker = Walker::new(settings)?;
        let [dx, dz] = settings.spawn_ground;
        let [w, x, y, z] = walker.ground.apply_vector(ORIGIN);
        close(w, dx.cosh() * dz.cosh());
        close(x, dx.sinh() * dz.cosh());
        close(y, 0.0);
        close(z, dz.sinh());

        let walk_frame = Pose::identity().move_local([dx, 0.0, dz], [0.0; 3], 1.0)?;
        let turn = FRAC_PI_2 - settings.yaw_degrees.to_radians();
        walker.step(0.0, 0.0, turn, 0.0, false, 0.0)?;
        walker.step(0.0, 1.0, 0.0, 0.0, false, 0.8)?;
        let local_ground = walk_frame.inverse()?.chain(&walker.ground)?;
        close(
            local_ground.apply_vector(ORIGIN)[1],
            -(settings.walk_speed * 0.8).sinh(),
        );

        for pitch in [0.5, 4.0, -8.0] {
            walker.step(0.4, 0.7, -0.3, pitch, false, 0.2)?;
            let eye = walker.eye_pose()?;
            let right = eye.apply_vector([0.0, 1.0, 0.0, 0.0]);
            let up = eye.apply_vector([0.0, 0.0, 1.0, 0.0]);
            // A tangent's Y component divided by cosh(h) is its dot product
            // with the local upward unit normal at height h.
            close(right[2], 0.0);
            close(up[2] / settings.height.cosh(), walker.pitch.cos());
            assert!(up[2] > 0.0 && walker.pitch.abs() < FRAC_PI_2);
        }
        Ok(())
    }

    #[test]
    fn looking_down_keeps_the_ground_beneath_the_walker_in_view() -> Result<()> {
        let walker = Walker::new(observer(-85.0_f64.to_radians()))?;
        let [w, x, y, z] = walker.eye_pose()?.inverse()?.apply_vector(ORIGIN);
        let ground = wgame::glam::Vec4::new(x as f32, y as f32, z as f32, w as f32);
        let clip = crate::projection::camera_matrix((640, 480), 0.66, 0.006) * ground;
        assert!(clip.z > 0.0 && clip.z < clip.w);
        assert!(clip.x.abs() < clip.w && clip.y.abs() < clip.w);
        Ok(())
    }

    #[test]
    fn jump_follows_normal_ballistics_and_lands_under_variable_steps() -> Result<()> {
        let settings = observer(0.3);
        let launch_speed = (2.0 * settings.gravity * settings.jump_height).sqrt();
        let flight_time = 2.0 * launch_speed / settings.gravity;
        let mut walker = Walker::new(settings)?;
        assert!(!walker.step(0.0, 0.0, 0.0, 0.0, false, 0.1)?);
        walker.step(0.0, 1.0, 0.0, 0.0, true, flight_time / 4.0)?;
        let t = flight_time / 4.0;
        close(
            height(walker)?,
            settings.height + launch_speed * t - settings.gravity * t * t / 2.0,
        );
        // Another press in the air does not restart the jump.
        walker.step(0.0, 1.0, 0.0, 0.0, true, flight_time / 4.0)?;
        close(height(walker)?, settings.height + settings.jump_height);
        close(
            walker.ground.apply_vector(ORIGIN)[0].acosh(),
            settings.walk_speed * flight_time / 2.0,
        );
        assert!(walker.step(0.0, 0.0, 0.0, 0.0, false, flight_time)?);
        close(height(walker)?, settings.height);
        assert!(!walker.step(0.0, 0.0, 0.0, 0.0, false, 1.0)?);
        assert!(walker.step(0.0, 0.0, 0.0, 0.0, true, 0.01)?);
        assert!(height(walker)? > settings.height);

        let mut grounded = Walker::new(Observer {
            jump_height: 0.0,
            ..settings
        })?;
        assert!(!grounded.step(0.0, 0.0, 0.0, 0.0, true, 0.0)?);
        assert!(!grounded.is_airborne());
        close(height(grounded)?, settings.height);
        Ok(())
    }
}
