//! Compiled scene configuration and camera-relative GPU preparation.

use bytemuck::{Pod, Zeroable};
use ccgeom::{Euclidean3, Geometry3, Homogenous3, Hyperbolic3};
use vecmat::{Complex, Transform, transform::Moebius};

use crate::Result;

#[derive(Clone, Copy, Debug)]
pub enum Camera {
    Euclidean(Homogenous3<f64>),
    Hyperbolic(Moebius<Complex<f64>>),
    /// Shared embedded coordinates. The geometry is part of the transform.
    Embedded(scene_ir::Transform),
}

impl Camera {
    pub fn geometry(&self) -> u32 {
        match self {
            Self::Euclidean(_) => 0,
            Self::Hyperbolic(_) => 1,
            Self::Embedded(map) => map.geometry().tag(),
        }
    }

    /// Canonical f64 transform, before preparing camera-relative GPU data.
    pub fn transform(self) -> scene_ir::Transform {
        match self {
            Self::Euclidean(map) => scene_ir::Transform::Euclidean(map),
            Self::Hyperbolic(map) => scene_ir::Transform::Hyperbolic(map),
            Self::Embedded(map) => map,
        }
    }

    /// Compose a local motion in f64. Distances/angles are already integrated
    /// over the caller's elapsed frame time; there is no assumed frame rate.
    pub fn move_local(&mut self, translation: [f64; 3], rotation: [f64; 3]) -> Result<()> {
        self.move_local_with_radius(translation, rotation, 1.0)
    }

    /// Move by physical distances in a space with the supplied curvature radius.
    /// Invalid motion leaves the camera unchanged.
    pub fn move_local_with_radius(
        &mut self,
        translation: [f64; 3],
        rotation: [f64; 3],
        radius: f64,
    ) -> Result<()> {
        anyhow::ensure!(
            translation.into_iter().chain(rotation).all(f64::is_finite),
            "camera motion must be finite"
        );
        anyhow::ensure!(
            radius.is_finite() && radius > 0.0 && (self.geometry() != 0 || radius == 1.0),
            "invalid curvature radius"
        );
        let [x, y, z] = translation.map(|distance| distance / radius);
        let [pitch, yaw, roll] = rotation;
        let next = match *self {
            Self::Euclidean(map) => Self::Euclidean(
                map.chain(Euclidean3::shift_x(x))
                    .chain(Euclidean3::shift_y(y))
                    .chain(Euclidean3::shift_z(z))
                    .chain(Euclidean3::rotate_x(pitch))
                    .chain(Euclidean3::rotate_y(yaw))
                    .chain(Euclidean3::rotate_z(roll)),
            ),
            Self::Hyperbolic(map) => Self::Hyperbolic(
                map.chain(Hyperbolic3::shift_x(x))
                    .chain(Hyperbolic3::shift_y(y))
                    .chain(Hyperbolic3::shift_z(z))
                    .chain(Hyperbolic3::rotate_x(pitch))
                    .chain(Hyperbolic3::rotate_y(yaw))
                    .chain(Hyperbolic3::rotate_z(roll)),
            ),
            Self::Embedded(map) => Self::Embedded(map.move_local(translation, rotation, radius)?),
        };
        next.transform().components()?;
        *self = next;
        Ok(())
    }
}

impl From<scene_ir::Transform> for Camera {
    fn from(map: scene_ir::Transform) -> Self {
        match map {
            scene_ir::Transform::Euclidean(map) => Self::Euclidean(map),
            scene_ir::Transform::Hyperbolic(map) => Self::Hyperbolic(map),
            map => Self::Embedded(map),
        }
    }
}

pub use scene_ir::Background;

#[derive(Clone, Debug)]
pub struct Scene {
    pub camera: Camera,
    pub fov: f32,
    pub bounces: u32,
    pub radius: f32,
    pub medium: scene_ir::Medium,
    pub background: Background,
    pub(crate) compiled: std::sync::Arc<scene_ir::CompiledScene>,
}

impl Scene {
    /// Compile a generic Rust scene lowered through `objects::Scene::definition`.
    /// The compiled program owns its parameter data independently of the Rust
    /// builder and its Rust memory layout.
    pub fn from_definition(definition: &scene_ir::SceneDefinition) -> Result<Self> {
        let compiled = scene_ir::compile(definition)?;
        let scene = Self {
            camera: definition.view.map.into(),
            fov: scene_ir::finite_f32(definition.view.fov)?,
            bounces: definition.bounces,
            radius: scene_ir::finite_f32(definition.radius)?,
            medium: definition.medium,
            background: definition.background.clone(),
            compiled: std::sync::Arc::new(compiled),
        };
        scene.validate()?;
        Ok(scene)
    }

    /// Validated canonical records. Camera updates prepare separate upload data.
    pub fn objects(&self) -> &[scene_ir::GpuObject] {
        &self.compiled.objects
    }

    pub(crate) fn material_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.compiled.materials)
    }

    pub(crate) fn words(&self) -> &[u32] {
        &self.compiled.words
    }

    /// Build upload records without mutating canonical scene data. Composition
    /// happens in f64; only the relative transforms are converted to f32.
    pub(crate) fn prepare_objects(&self, camera: Camera) -> Result<Vec<scene_ir::GpuObject>> {
        anyhow::ensure!(
            camera.geometry() == self.compiled.geometry.tag(),
            "camera geometry must match the scene"
        );
        let mut objects = self.compiled.objects.clone();
        let inverse = camera.transform().inverse()?;
        anyhow::ensure!(
            objects.len() == self.compiled.transforms.len(),
            "compiled object/transform count differs"
        );
        for (object, map) in objects.iter_mut().zip(&self.compiled.transforms) {
            let rows = inverse.chain(map)?.rows()?;
            object.map0 = rows[0];
            object.map1 = rows[1];
        }
        Ok(objects)
    }

    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(
            self.camera.geometry() == self.compiled.geometry.tag(),
            "camera geometry differs from the compiled scene; compile a new SceneDefinition"
        );
        anyhow::ensure!(
            self.radius == self.compiled.radius,
            "curvature radius must be updated through SceneDefinition"
        );
        self.camera.transform().components()?;
        anyhow::ensure!(
            self.radius.is_finite()
                && self.radius > 0.0
                && (self.camera.geometry() != 0 || self.radius == 1.0),
            "invalid curvature radius"
        );
        self.medium.validate_for_radius(self.radius)?;
        anyhow::ensure!(
            self.fov.is_finite() && self.fov > 0.0,
            "fov must be finite and positive"
        );
        anyhow::ensure!(
            (1..=64).contains(&self.bounces),
            "bounce limit must be 1..=64"
        );
        let colors: Vec<f32> = match self.background {
            Background::Constant(c) => c.into(),
            Background::Gradient {
                colors,
                axis,
                power,
            } => {
                anyhow::ensure!(
                    axis.into_iter().all(f32::is_finite) && power.is_finite() && power > 0.0,
                    "invalid gradient"
                );
                colors.into_iter().flatten().collect()
            }
        };
        anyhow::ensure!(
            colors.into_iter().all(|x| x.is_finite() && x >= 0.0),
            "invalid background color"
        );
        self.prepare_objects(self.camera)?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(crate) struct Params {
    pub camera0: [f32; 4],
    pub camera1: [f32; 4],
    pub background0: [f32; 4],
    pub background1: [f32; 4],
    pub background_axis: [f32; 4],
    pub info: [u32; 4],
    pub options: [u32; 4],
    pub misc: [f32; 4],
    pub medium: [f32; 4],
}

impl Params {
    pub fn new(scene: &Scene, size: (u32, u32), samples: u32) -> Self {
        // The kernel starts rays at the camera-relative origin. Only a
        // Euclidean gradient needs the absolute camera rotation.
        let camera0 = if scene.camera.geometry() == 0 {
            scene
                .camera
                .transform()
                .components()
                .expect("validated camera")[0]
                .map(|value| value as f32)
        } else {
            [1.0, 0.0, 0.0, 0.0]
        };
        let camera1 = [0.0; 4];
        let (c0, c1, axis, power, mode) = match scene.background {
            Background::Constant(c) => (c, c, [0.0; 3], 1.0, 0),
            Background::Gradient {
                colors,
                axis,
                power,
            } => (colors[0], colors[1], axis, power, 1),
        };
        let pad = |a: [f32; 3]| [a[0], a[1], a[2], 0.0];
        Self {
            camera0,
            camera1,
            background0: pad(c0),
            background1: pad(c1),
            background_axis: pad(axis),
            info: [
                size.0,
                size.1,
                scene.camera.geometry(),
                scene.objects().len() as u32,
            ],
            options: [samples, scene.bounces, mode, 0],
            misc: [scene.fov, scene.radius, power, 0.0],
            medium: scene.medium.gpu_row(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use objects::Scene as _;

    #[test]
    fn generated_params_keep_radius_medium_and_relative_camera_contract() {
        let mut definition = examples::sp::scene::<6>().definition().unwrap();
        definition.radius = 2.5;
        definition.medium = scene_ir::Medium::Homogeneous {
            extinction: 0.125,
            albedo: [0.2, 0.4, 0.7],
        };
        let scene = Scene::from_definition(&definition).unwrap();
        let params = Params::new(&scene, (13, 7), 3);
        assert_eq!(params.camera0, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(params.camera1, [0.0; 4]);
        assert_eq!(params.misc, [1.0, 2.5, 1.0, 0.0]);
        assert_eq!(params.medium, [0.2, 0.4, 0.7, 0.125]);
        assert_eq!(params.info, [13, 7, 2, 6]);
        assert_eq!(std::mem::offset_of!(Params, medium), 128);
        assert_eq!(std::mem::align_of::<Params>(), 16);
    }

    #[test]
    fn relative_preparation_preserves_far_translated_scenes() {
        let cases = [
            (
                examples::eu::scene::<4>().definition().unwrap(),
                scene_ir::Transform::Euclidean(Euclidean3::shift_x(1e9)),
            ),
            (
                examples::hy::scene::<3>().definition().unwrap(),
                scene_ir::Transform::Hyperboloid(
                    ccgeom::Space3::<f64, -1>::unit()
                        .translation([1.0, 0.0, 0.0].into(), 12.0)
                        .unwrap(),
                ),
            ),
        ];
        for (mut definition, global) in cases {
            let baseline = Scene::from_definition(&definition).unwrap();
            let expected = baseline.prepare_objects(baseline.camera).unwrap();
            definition.view.map = global.chain(&definition.view.map).unwrap();
            definition.object = scene_ir::ObjectNode::Mapped {
                map: global,
                inner: Box::new(definition.object),
            };
            let shifted = Scene::from_definition(&definition).unwrap();
            let original_records =
                bytemuck::cast_slice::<scene_ir::GpuObject, u8>(shifted.objects()).to_vec();
            let actual = shifted.prepare_objects(shifted.camera).unwrap();
            assert_eq!(actual.len(), expected.len());
            for (actual, expected) in actual.iter().zip(&expected) {
                for (a, b) in actual
                    .map0
                    .into_iter()
                    .chain(actual.map1)
                    .zip(expected.map0.into_iter().chain(expected.map1))
                {
                    assert!((a - b).abs() < 2e-4, "relative component {a} != {b}");
                }
                assert_eq!(actual.info, expected.info);
            }
            assert_eq!(
                bytemuck::cast_slice::<scene_ir::GpuObject, u8>(shifted.objects()),
                original_records
            );
        }
    }

    #[test]
    fn physical_camera_motion_respects_radius_and_rejects_bad_input() {
        let mut camera = Camera::Embedded(scene_ir::Transform::Spherical(
            ccgeom::EmbeddedIsometry::identity(),
        ));
        camera
            .move_local_with_radius([std::f64::consts::PI, 0.0, 0.0], [0.0; 3], 2.0)
            .unwrap();
        let scene_ir::Transform::Spherical(map) = camera.transform() else {
            unreachable!()
        };
        let position = map.apply_vector([1.0, 0.0, 0.0, 0.0].into());
        assert!(position[0].abs() < 1e-14);
        assert!((position[1] - 1.0).abs() < 1e-14);
        let original = camera.transform().components().unwrap();
        assert!(
            camera
                .move_local_with_radius([f64::NAN, 0.0, 0.0], [0.0; 3], 2.0)
                .is_err()
        );
        assert_eq!(camera.transform().components().unwrap(), original);
    }

    #[test]
    fn generated_validation_keeps_radius_constraints_and_medium_finite() {
        let scene =
            Scene::from_definition(&examples::sp::scene::<6>().definition().unwrap()).unwrap();
        let mut changed = scene.clone();
        changed.radius = 0.01;
        assert!(
            changed
                .validate()
                .unwrap_err()
                .to_string()
                .contains("SceneDefinition")
        );
        for medium in [
            scene_ir::Medium::Homogeneous {
                extinction: -1.0,
                albedo: [0.5; 3],
            },
            scene_ir::Medium::Homogeneous {
                extinction: f32::INFINITY,
                albedo: [0.5; 3],
            },
            scene_ir::Medium::Homogeneous {
                extinction: f32::from_bits(1),
                albedo: [0.5; 3],
            },
            scene_ir::Medium::Homogeneous {
                extinction: 0.1,
                albedo: [1.1; 3],
            },
            scene_ir::Medium::Homogeneous {
                extinction: 0.1,
                albedo: [f32::NAN; 3],
            },
        ] {
            let mut changed = scene.clone();
            changed.medium = medium;
            assert!(changed.validate().is_err());
        }
    }
    #[test]
    fn relative_upload_rejects_unrepresentable_camera_motion() {
        let mut scene =
            Scene::from_definition(&examples::hy::scene::<3>().definition().unwrap()).unwrap();
        scene.camera = Camera::Hyperbolic(Hyperbolic3::shift_x(18.0));
        assert!(scene.validate().is_err());
    }

    #[test]
    fn uniform_abi() {
        assert_eq!(std::mem::size_of::<Params>(), 144);
        assert_eq!(std::mem::align_of::<Params>(), 16);
    }
}
