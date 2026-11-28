//! Walking on the geodesic Earth plane (embedded Z = 0), in curvature-radius units.
use std::f64::consts::FRAC_PI_2;

use anyhow::{Context, Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3, Space3};

use crate::{config::observer::Observer, projection::Pose};

#[derive(Clone, Copy)]
pub(crate) struct Walker {
    observer: Observer,
    /// Maps the origin to the ground beneath the feet and local +Z to the Earth normal.
    ground: Pose,
    pitch: f64,
    altitude: f64,
    /// Local [right, forward, up] speed components, in curvature radii per second.
    velocity: [f64; 3],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ObserverCoordinates {
    /// Fermi ground coordinates in spawn order, followed by eye height, in curvature radii.
    pub position: [f64; 3],
    /// Unit viewing direction in the local orthonormal frame of the position axes.
    pub direction: [f64; 3],
}

impl Walker {
    pub(crate) fn new(observer: Observer) -> Result<Self> {
        observer.validate()?;
        let [x, y] = observer.spawn_ground;
        let ground = Pose::identity().move_local(
            [x, y, 0.0],
            [0.0, 0.0, observer.yaw_degrees.to_radians()],
            1.0,
        )?;
        Ok(Self {
            observer,
            ground,
            pitch: observer.pitch_degrees.to_radians(),
            altitude: 0.0,
            velocity: [0.0; 3],
        })
    }

    pub(crate) fn reconfigured(self, observer: Observer) -> Result<Self> {
        observer.validate()?;
        let limit = observer.pitch_limit_degrees.to_radians();
        Ok(Self {
            observer,
            pitch: self.pitch.clamp(-limit, limit),
            ..self
        })
    }

    fn eye_height(&self) -> f64 {
        self.observer.height + self.altitude
    }

    /// Highest eye height remaining in the current flight under the current gravity.
    pub(crate) fn peak_eye_height(&self) -> f64 {
        let rise = self.velocity[2].max(0.0) / self.observer.gravity.sqrt();
        (0.5 * rise).mul_add(rise, self.eye_height())
    }

    pub(crate) fn coordinates(&self) -> ObserverCoordinates {
        let [_, x, y, _] = self.ground.apply_vector([1.0, 0.0, 0.0, 0.0]);
        // Bx(a) By(b) maps the origin to (cosh(a)cosh(b), sinh(a)cosh(b), sinh(b), 0).
        let position = [(x / y.hypot(1.0)).asinh(), y.asinh(), self.eye_height()];
        // In Bx(a) By(b) Rz(yaw), the Y components of right and forward
        // are sin(yaw) and cos(yaw) multiplied by the same cosh(b).
        let right = self.ground.apply_vector([0.0, 1.0, 0.0, 0.0]);
        let forward = self.ground.apply_vector([0.0, 0.0, 1.0, 0.0]);
        let (sin_yaw, cos_yaw) = right[2].atan2(forward[2]).sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        ObserverCoordinates {
            position,
            direction: [-sin_yaw * cos_pitch, cos_yaw * cos_pitch, sin_pitch],
        }
    }

    pub(crate) fn velocity(&self) -> [f64; 3] {
        self.velocity
    }

    /// Maps renderer camera coordinates (+Y up, −Z forward) into the Z-up world.
    pub(crate) fn eye_pose(&self) -> Result<Pose> {
        self.ground.move_local(
            [0.0, 0.0, self.eye_height()],
            [FRAC_PI_2 + self.pitch, 0.0, 0.0],
            1.0,
        )
    }

    pub(crate) fn is_airborne(&self) -> bool {
        self.altitude > 0.0 || self.velocity[2] > 0.0
    }

    /// Horizontal axes are fractions of full walking speed. Look deltas are
    /// radians, and jump is a fresh press. Returns whether the eye pose changed.
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
        let mut altitude = self.altitude;
        let mut up = self.velocity[2];
        if jump && !self.is_airborne() {
            up = self.observer.launch_speed();
        }
        if dt > 0.0 && (altitude > 0.0 || up > 0.0) {
            let end_speed = (-self.observer.gravity).mul_add(dt, up);
            // Under constant gravity the mean endpoint velocity is exact.
            altitude = (0.5 * (up + end_speed)).mul_add(dt, altitude);
            if altitude <= 0.0 {
                altitude = 0.0;
                up = 0.0;
            } else {
                ensure!(
                    altitude.is_finite() && end_speed.is_finite(),
                    "vertical movement exceeds f64 range"
                );
                up = end_speed;
            }
        }
        let magnitude = strafe.hypot(forward);
        ensure!(
            magnitude.is_finite(),
            "walking input magnitude exceeds f64 range"
        );
        let distance = (self.observer.walk_speed * magnitude.min(1.0)) * dt;
        ensure!(
            distance.is_finite(),
            "ground displacement exceeds f64 range"
        );
        let scale = magnitude.max(1.0);
        let right = self.observer.walk_speed * (strafe / scale);
        let forward_speed = self.observer.walk_speed * (forward / scale);

        let mut ground = self.ground;
        if distance > 0.0 {
            // A single in-plane boost parallel-transports the walking frame
            // along the geodesic, including when moving diagonally.
            let axis = [strafe / magnitude, forward / magnitude, 0.0];
            let step = Space3::<f64, -1>::unit()
                .translation(axis.into(), distance)
                .context("invalid ground displacement")?;
            ground = ground.chain(&Pose::from_isometry(step)?)?;
        }
        if yaw != 0.0 {
            ground = ground.chain(&Pose::from_isometry(Hyperboloid3::rotate_z(yaw))?)?;
        }
        // Ground motion precedes yaw. Express its transported velocity in the final heading.
        let (sin_yaw, cos_yaw) = yaw.sin_cos();
        let velocity = [
            right * cos_yaw + forward_speed * sin_yaw,
            forward_speed * cos_yaw - right * sin_yaw,
            up,
        ];
        ensure!(
            velocity.into_iter().all(f64::is_finite),
            "observer velocity exceeds f64 range"
        );
        let moved =
            distance > 0.0 || yaw != 0.0 || pitch != self.pitch || altitude != self.altitude;
        *self = Self {
            ground,
            pitch,
            altitude,
            velocity,
            ..*self
        };
        Ok(moved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        Ok(walker.eye_pose()?.apply_vector(ORIGIN)[3].asinh())
    }

    #[test]
    fn level_camera_maps_right_forward_and_up_into_the_world_basis() -> Result<()> {
        let settings = observer(0.0);
        let walker = Walker::new(settings)?;
        let eye = walker.eye_pose()?;
        let h = settings.height;
        for (input, expected) in [
            (ORIGIN, [h.cosh(), 0.0, 0.0, h.sinh()]),
            ([0.0, 1.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]),
            ([0.0, 0.0, 0.0, -1.0], [0.0, 0.0, 1.0, 0.0]),
            ([0.0, 0.0, 1.0, 0.0], [h.sinh(), 0.0, 0.0, h.cosh()]),
        ] {
            for (actual, expected) in eye.apply_vector(input).into_iter().zip(expected) {
                close(actual, expected);
            }
        }
        Ok(())
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
                close(y, settings.walk_speed.sinh() * forward / magnitude);
                close(z, 0.0);
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
        let [dx, dy] = settings.spawn_ground;
        let [w, x, y, z] = walker.ground.apply_vector(ORIGIN);
        close(w, dx.cosh() * dy.cosh());
        close(x, dx.sinh() * dy.cosh());
        close(y, dy.sinh());
        close(z, 0.0);

        let walk_frame = Pose::identity().move_local([dx, dy, 0.0], [0.0; 3], 1.0)?;
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
            // A tangent's Z component divided by cosh(h) is its dot product
            // with the local upward unit normal at height h.
            close(right[3], 0.0);
            close(up[3] / settings.height.cosh(), walker.pitch.cos());
            assert!(up[3] > 0.0 && walker.pitch.abs() < FRAC_PI_2);
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
    fn coordinates_recover_spawn_and_reconstruct_a_transported_walking_frame() -> Result<()> {
        for (spawn_ground, yaw_degrees) in [
            ([-3.2, 0.0], 0.0),
            ([1.2, -2.4], 134.0),
            ([-0.7, 1.6], -123.0),
        ] {
            let settings = Observer {
                spawn_ground,
                yaw_degrees,
                ..observer(0.27)
            };
            let mut walker = Walker::new(settings)?;
            let initial = walker.coordinates();
            let [x, y, z] = initial.position;
            close(x, spawn_ground[0]);
            close(y, spawn_ground[1]);
            close(z, settings.height);
            let pitch = settings.pitch_degrees.to_radians();
            let yaw = yaw_degrees.to_radians();
            close(initial.direction[0], -yaw.sin() * pitch.cos());
            close(initial.direction[1], yaw.cos() * pitch.cos());
            close(initial.direction[2], pitch.sin());

            walker.step(0.6, 0.8, -0.8, 0.3, false, 1.4)?;
            walker.step(-0.4, 0.7, 0.35, -0.02, true, 0.09)?;
            assert!(walker.is_airborne());
            let location = walker.coordinates();
            let [x, y, z] = location.position;
            let [dx, dy, dz] = location.direction;
            close(dx.hypot(dy).hypot(dz), 1.0);
            let recreated = Walker::new(Observer {
                spawn_ground: [x, y],
                height: z,
                yaw_degrees: (-dx).atan2(dy).to_degrees(),
                pitch_degrees: dz.asin().to_degrees(),
                ..settings
            })?;
            for axis in [ORIGIN, [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0]] {
                let expected = walker.eye_pose()?.apply_vector(axis);
                let actual = recreated.eye_pose()?.apply_vector(axis);
                for (a, b) in actual.into_iter().zip(expected) {
                    close(a, b);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn ground_velocity_is_relative_to_final_heading_and_clears_on_release() -> Result<()> {
        let settings = observer(0.0);
        let mut walker = Walker::new(settings)?;
        walker.step(1.0, 1.0, 0.0, 0.0, false, 0.5)?;
        let [right, forward, up] = walker.velocity();
        let speed = settings.walk_speed / 2.0_f64.sqrt();
        close(right, speed);
        close(up, 0.0);
        close(forward, speed);
        walker.step(0.0, 0.5, FRAC_PI_2, 0.0, false, 0.5)?;
        let [right, forward, _] = walker.velocity();
        close(right, 0.5 * settings.walk_speed);
        close(forward, 0.0);
        assert!(!walker.step(0.0, 0.0, 0.0, 0.0, false, 0.5)?);
        assert_eq!(walker.velocity(), [0.0; 3]);
        Ok(())
    }

    #[test]
    fn reload_preserves_walked_pose_and_applies_new_movement_settings() -> Result<()> {
        let settings = observer(0.1);
        let mut walker = Walker::new(settings)?;
        walker.step(0.4, 0.8, 0.6, 0.5, false, 0.7)?;
        let edited = Observer {
            spawn_ground: [-1.2, 2.0],
            yaw_degrees: -55.0,
            pitch_degrees: 0.0,
            walk_speed: 2.0 * settings.walk_speed,
            ..settings
        };
        let restored = walker.reconfigured(edited)?;
        assert_eq!(
            restored.eye_pose()?.components()?,
            walker.eye_pose()?.components()?
        );

        let edited = Observer {
            height: 0.12,
            pitch_limit_degrees: 20.0,
            ..edited
        };
        let mut limited = restored.reconfigured(edited)?;
        assert_eq!(limited.ground.components()?, walker.ground.components()?);
        close(limited.pitch, 20_f64.to_radians());
        close(height(limited)?, edited.height);
        limited.step(0.0, 1.0, 0.0, 0.0, false, 1.0)?;
        let travel = walker.ground.inverse()?.chain(&limited.ground)?;
        close(travel.apply_vector(ORIGIN)[2], edited.walk_speed.sinh());
        Ok(())
    }

    #[test]
    fn reload_during_jump_preserves_height_and_velocity_and_applies_new_gravity() -> Result<()> {
        let settings = observer(0.0);
        let mut walker = Walker::new(settings)?;
        let quarter = settings.flight_time() / 4.0;
        walker.step(0.0, 0.0, 0.0, 0.0, true, quarter)?;
        let edited = Observer {
            jump_height: 0.0,
            gravity: 2.0 * settings.gravity,
            ..settings
        };
        let mut restored = walker.reconfigured(edited)?;
        assert_eq!(
            restored.eye_pose()?.components()?,
            walker.eye_pose()?.components()?
        );
        let initial_height = walker.eye_height();
        let initial_velocity = walker.velocity()[2];
        assert_eq!(restored.velocity(), walker.velocity());
        close(
            walker.peak_eye_height(),
            settings.height + settings.jump_height,
        );
        close(
            restored.peak_eye_height(),
            initial_height + initial_velocity.powi(2) / (2.0 * edited.gravity),
        );
        restored.step(0.0, 0.0, 0.0, 0.0, false, quarter)?;
        close(
            restored.velocity()[2],
            initial_velocity - edited.gravity * quarter,
        );
        close(
            restored.eye_height(),
            initial_height + initial_velocity * quarter - 0.5 * edited.gravity * quarter.powi(2),
        );
        close(restored.peak_eye_height(), restored.eye_height());
        restored.step(0.0, 0.0, 0.0, 0.0, false, 2.0 * quarter)?;
        assert!(!restored.is_airborne());
        close(restored.velocity()[2], 0.0);
        assert!(!restored.step(0.0, 0.0, 0.0, 0.0, true, quarter)?);
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
        assert!(walker.step(0.0, 0.0, 0.0, 0.0, false, f64::MAX)?);
        assert_eq!(walker.eye_height(), settings.height);
        assert!(!walker.is_airborne());

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
