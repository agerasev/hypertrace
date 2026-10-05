//! Compiled scene configuration and camera-relative GPU preparation.

use bytemuck::{Pod, Zeroable};
use scene_ir::{Geometry, Transform};

use crate::Result;

/// A camera pose whose geometry remains a compile-time property.
///
/// A camera from another geometry cannot be assigned to a scene:
/// ```compile_fail
/// use ccgeom::{Flat3, Spherical3};
/// use hypertrace_renderer::{Camera, Scene};
/// fn replace(scene: &mut Scene<Flat3>, camera: Camera<Spherical3>) {
///     scene.camera = camera;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Camera<G: Geometry> {
    map: Transform<G>,
}

impl<G: Geometry> Camera<G> {
    /// Canonical f64 transform, before preparing camera-relative GPU data.
    pub fn transform(self) -> Transform<G> {
        self.map
    }

    /// Compose a local motion in f64. Distances/angles are already integrated
    /// over the caller's elapsed frame time; there is no assumed frame rate.
    /// Translation uses physical distances and the scene's curvature radius.
    /// Invalid motion leaves the camera unchanged.
    pub fn move_local(
        &mut self,
        translation: [f64; 3],
        rotation: [f64; 3],
        radius: f64,
    ) -> Result<()> {
        let next = self.map.move_local(translation, rotation, radius)?;
        next.components()?;
        self.map = next;
        Ok(())
    }
}

impl<G: Geometry> From<Transform<G>> for Camera<G> {
    fn from(map: Transform<G>) -> Self {
        Self { map }
    }
}

pub use scene_ir::Background;

#[derive(Clone, Debug)]
pub struct Scene<G: Geometry> {
    pub camera: Camera<G>,
    pub fov: f32,
    pub bounces: u32,
    pub radius: f32,
    pub medium: scene_ir::Medium,
    pub background: Background<G>,
    pub(crate) compiled: std::sync::Arc<scene_ir::CompiledScene<G>>,
}

impl<G: Geometry> Scene<G> {
    /// Compile a generic Rust scene lowered through `objects::Scene::definition`.
    /// The compiled program owns its parameter data independently of the Rust
    /// builder and its Rust memory layout.
    pub fn from_definition(definition: &scene_ir::SceneDefinition<G>) -> Result<Self> {
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
    pub(crate) fn prepare_objects(&self, camera: Camera<G>) -> Result<Vec<scene_ir::GpuObject>> {
        let mut objects = self.compiled.objects.clone();
        let inverse = camera.transform().inverse()?;
        anyhow::ensure!(
            objects.len() == self.compiled.transforms.len()
                && objects.len() == self.compiled.sampling_transforms.len(),
            "compiled object/transform count differs"
        );
        for ((object, map), sampling_map) in objects
            .iter_mut()
            .zip(&self.compiled.transforms)
            .zip(&self.compiled.sampling_transforms)
        {
            let rows = inverse.chain(map)?.rows()?;
            object.map0 = rows[0];
            object.map1 = rows[1];
            let sampling_rows = inverse.chain(sampling_map)?.rows()?;
            object.sampling_map0 = sampling_rows[0];
            object.sampling_map1 = sampling_rows[1];
        }
        Ok(objects)
    }

    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(
            self.radius == self.compiled.radius,
            "curvature radius must be updated through SceneDefinition"
        );
        self.camera.transform().components()?;
        anyhow::ensure!(
            self.radius.is_finite() && self.radius > 0.0 && (G::SIGN != 0 || self.radius == 1.0),
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
        self.background.validate()?;
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
    pub fn new<G: Geometry>(scene: &Scene<G>, size: (u32, u32), samples: u32) -> Self {
        // The kernel starts rays at the camera-relative origin. Only a
        // Euclidean gradient needs the absolute camera rotation.
        let camera0 = if G::SIGN == 0 {
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
        let [c0, c1] = scene.background.colors();
        let axis = scene.background.axis();
        let power = scene.background.power();
        let pad = |a: [f32; 3]| [a[0], a[1], a[2], 0.0];
        Self {
            camera0,
            camera1,
            background0: pad(c0),
            background1: pad(c1),
            background_axis: pad(axis),
            info: [size.0, size.1, 0, scene.objects().len() as u32],
            options: [samples, scene.bounces, scene.compiled.light_count, 0],
            misc: [scene.fov, scene.radius, power, 0.0],
            medium: scene.medium.gpu_row(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ccgeom::{Embedded3, Spherical3};
    use objects::Scene as _;

    fn fixture<const K: i8>() -> scene_ir::SceneDefinition<Embedded3<f64, K>>
    where
        Embedded3<f64, K>: Geometry<Map = ccgeom::EmbeddedIsometry<f64, K>>,
    {
        use objects::{
            Mapped, SceneImpl, background::ConstBg, material::Absorbing, object::Covered,
            shape::GeodesicSphere, view::PointView,
        };
        type G<const K: i8> = Embedded3<f64, K>;
        let space = ccgeom::Space3::<f64, K>::unit();
        let objects: Vec<_> = [-0.5, 0.5]
            .into_iter()
            .map(|distance| {
                Mapped::<G<K>, _, _>::new(
                    Covered::new(GeodesicSphere::new(0.25), Absorbing),
                    space.translation([1.0, 0.0, 0.0].into(), distance).unwrap(),
                )
            })
            .collect();
        SceneImpl::<G<K>, _, _, _, 6>::new(
            PointView::new(1.0),
            objects,
            ConstBg::new([0.0; 3].into()),
        )
        .definition()
        .unwrap()
    }

    #[test]
    fn generated_params_keep_radius_medium_and_relative_camera_contract() {
        let mut definition = fixture::<1>();
        definition.radius = 2.5;
        definition.medium = scene_ir::Medium::homogeneous(0.125, [0.2, 0.4, 0.7]);
        let scene = Scene::from_definition(&definition).unwrap();
        let params = Params::new(&scene, (13, 7), 3);
        assert_eq!(params.camera0, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(params.camera1, [0.0; 4]);
        assert_eq!(params.misc, [1.0, 2.5, 1.0, 0.0]);
        assert_eq!(params.medium, [0.2, 0.4, 0.7, 0.125]);
        assert_eq!(params.info, [13, 7, 0, 2]);
        assert_eq!(std::mem::offset_of!(Params, medium), 128);
        assert_eq!(std::mem::align_of::<Params>(), 16);
    }

    fn translated<const K: i8>(distance: f64)
    where
        Embedded3<f64, K>: Geometry<Map = ccgeom::EmbeddedIsometry<f64, K>>,
    {
        let mut definition = fixture::<K>();
        use objects::light::{LightSampler, SphereBound};
        for object in &mut definition.objects {
            let mut sampling =
                <SphereBound as LightSampler<Embedded3<f64, K>>>::encode(&SphereBound::new(0.25))
                    .unwrap();
            sampling.map = object.map;
            object.sampling = Some(sampling);
        }
        let global = Transform::<Embedded3<f64, K>>::identity()
            .move_local([distance, 0.0, 0.0], [0.0; 3], 1.0)
            .unwrap();
        let baseline = Scene::from_definition(&definition).unwrap();
        let expected = baseline.prepare_objects(baseline.camera).unwrap();
        definition.view.map = global.chain(&definition.view.map).unwrap();
        for object in &mut definition.objects {
            object.map = global.chain(&object.map).unwrap();
            let sampling = object.sampling.as_mut().unwrap();
            sampling.map = global.chain(&sampling.map).unwrap();
        }
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
                .chain(actual.sampling_map0)
                .chain(actual.sampling_map1)
                .zip(
                    expected
                        .map0
                        .into_iter()
                        .chain(expected.map1)
                        .chain(expected.sampling_map0)
                        .chain(expected.sampling_map1),
                )
            {
                assert!((a - b).abs() < 2e-4, "relative component {a} != {b}");
            }
            assert_eq!(actual.info, expected.info);
            assert_eq!(actual.sampling, expected.sampling);
        }
        assert_eq!(
            bytemuck::cast_slice::<scene_ir::GpuObject, u8>(shifted.objects()),
            original_records
        );
    }

    #[test]
    fn relative_preparation_preserves_far_translated_scenes() {
        translated::<0>(1e9);
        translated::<-1>(12.0);
    }

    #[test]
    fn physical_camera_motion_respects_radius_and_rejects_bad_input() {
        let mut camera = Camera::<Spherical3>::from(Transform::identity());
        camera
            .move_local([std::f64::consts::PI, 0.0, 0.0], [0.0; 3], 2.0)
            .unwrap();
        let position = camera.transform().apply_vector([1.0, 0.0, 0.0, 0.0]);
        assert!(position[0].abs() < 1e-14);
        assert!((position[1] - 1.0).abs() < 1e-14);
        let original = camera.transform().components().unwrap();
        for (translation, rotation) in [
            ([f64::NAN, 0.0, 0.0], [0.0; 3]),
            ([0.2; 3], [0.0, f64::INFINITY, 0.0]),
        ] {
            assert!(camera.move_local(translation, rotation, 2.0).is_err());
            assert_eq!(camera.transform().components().unwrap(), original);
        }
    }

    #[test]
    fn generated_validation_keeps_radius_constraints_and_medium_finite() {
        let scene = Scene::from_definition(&fixture::<1>()).unwrap();
        let mut changed = scene.clone();
        changed.radius = 0.01;
        assert!(
            changed
                .validate()
                .unwrap_err()
                .to_string()
                .contains("SceneDefinition")
        );
        for (extinction, albedo) in [
            (-1.0, [0.5; 3]),
            (f32::INFINITY, [0.5; 3]),
            (f32::from_bits(1), [0.5; 3]),
            (0.1, [1.1; 3]),
            (0.1, [f32::NAN; 3]),
        ] {
            let mut changed = scene.clone();
            changed.medium = scene_ir::Medium::homogeneous(extinction, albedo);
            assert!(changed.validate().is_err());
        }
    }

    #[test]
    fn relative_upload_rejects_unrepresentable_camera_motion() {
        let mut scene = Scene::from_definition(&fixture::<-1>()).unwrap();
        scene.camera = Transform::identity()
            .move_local([18.0, 0.0, 0.0], [0.0; 3], 1.0)
            .unwrap()
            .into();
        assert!(scene.validate().is_err());
    }

    #[test]
    fn uniform_abi() {
        assert_eq!(std::mem::size_of::<Params>(), 144);
        assert_eq!(std::mem::align_of::<Params>(), 16);
    }
}
