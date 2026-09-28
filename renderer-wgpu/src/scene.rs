//! Scene configuration and explicit, version-local storage ABI.
//!
//! Record fields and alignments match the WGSL storage layouts. `Scene::from_definition` accepts the generic
//! scene compiler's output; fixed `eu`/`hy` records remain comparison fixtures.

use bytemuck::{Pod, Zeroable};
use ccgeom::{Euclidean3, Geometry3, Homogenous3, Hyperbolic3};
use vecmat::{
    Complex, Transform,
    transform::{Moebius, Rotation3, Shift},
};

use crate::Result;

#[derive(Clone, Copy, Debug)]
pub enum Camera {
    Euclidean(Homogenous3<f64>),
    Hyperbolic(Moebius<Complex<f64>>),
    /// Shared embedded coordinates. The geometry is part of the transform.
    Embedded(scene_ir::Transform),
}

impl Camera {
    pub(crate) fn validate(&self) -> Result<()> {
        if let Self::Embedded(map) = self {
            map.rows()?;
            return Ok(());
        }
        if let Self::Hyperbolic(map) = self {
            scene_ir::validate_moebius(*map)?;
        }
        validate_map(self.gpu_rows(), self.geometry())
    }

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

    pub fn gpu_rows(&self) -> [[f32; 4]; 2] {
        match self {
            Self::Hyperbolic(map) => moebius_rows(*map),
            Self::Euclidean(map) => {
                let p = map.apply([0.0, 0.0, 0.0].into());
                let q = map.inner().into_quaternion();
                [
                    [p[0] as f32, p[1] as f32, p[2] as f32, 0.0],
                    q.into_array().map(|x| x as f32),
                ]
            }
            Self::Embedded(map) => map
                .components()
                .unwrap_or([[f64::NAN; 4]; 2])
                .map(|row| row.map(|value| value as f32)),
        }
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

pub fn moebius_rows(map: Moebius<Complex<f64>>) -> [[f32; 4]; 2] {
    let (a, b, c, d) = map.into_tuple();
    [
        [a.re() as f32, a.im() as f32, b.re() as f32, b.im() as f32],
        [c.re() as f32, c.im() as f32, d.re() as f32, d.im() as f32],
    ]
}

// Check after conversion: a valid f64 boost can become singular when cosh(t/2)
// and sinh(t/2) round to the same f32 value. This detects complete loss of the
// transform, not every source of error in ill-conditioned maps.
fn validate_map(rows: [[f32; 4]; 2], geometry: u32) -> Result<()> {
    anyhow::ensure!(
        rows.into_iter().flatten().all(f32::is_finite),
        "transform is outside f32 range"
    );
    if geometry == 0 {
        let norm = rows[1]
            .map(|x| f64::from(x).powi(2))
            .into_iter()
            .sum::<f64>();
        anyhow::ensure!(
            (norm - 1.0).abs() < 1e-4,
            "Euclidean rotation must be a unit quaternion"
        );
    } else {
        let [ab, cd] = rows.map(|row| row.map(f64::from));
        let a = Complex::new(ab[0], ab[1]);
        let b = Complex::new(ab[2], ab[3]);
        let c = Complex::new(cd[0], cd[1]);
        let d = Complex::new(cd[2], cd[3]);
        let determinant = a * d - b * c;
        anyhow::ensure!(
            determinant.re() != 0.0 || determinant.im() != 0.0,
            "Möbius transform becomes singular in f32; camera-relative coordinate frames are needed for this position"
        );
    }
    Ok(())
}

/// Four aligned vectors; mixtures are sampled in diffuse/specular/transparent/
/// refractive order. Unassigned probability is absorption.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Material {
    /// Diffuse RGB and its mixture probability.
    pub diffuse: [f32; 4],
    /// Emitted RGB and the specular reflection probability.
    pub emission: [f32; 4],
    /// Refracted RGB multiplier and the uncolored transparency probability.
    pub transmission: [f32; 4],
    /// Refraction probability, refractive index, and two reserved zeros.
    pub properties: [f32; 4],
}

impl Material {
    pub fn diffuse(color: [f32; 3]) -> Self {
        Self {
            diffuse: [color[0], color[1], color[2], 1.0],
            emission: [0.0; 4],
            transmission: [1.0, 1.0, 1.0, 0.0],
            properties: [0.0, 1.0, 0.0, 0.0],
        }
    }
}

/// Shape tags: plane=0, sphere=1, cube=2, horosphere=3. Tiling tags:
/// none=0, square=1, hexagonal=2, pentagonal=3, pentastar=4.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Object {
    /// Euclidean translation, or the complex Möbius coefficients a and b.
    pub map0: [f32; 4],
    /// Euclidean unit quaternion (real first), or complex coefficients c and d.
    pub map1: [f32; 4],
    /// Shape tag, first material, material count, and tiling tag.
    pub info: [u32; 4],
    /// Border material index followed by three reserved zeros.
    pub extra: [u32; 4],
    /// Cell size, border width, and two reserved zeros.
    pub props: [f32; 4],
}

impl Object {
    pub fn new(shape: u32, material: u32, map: [[f32; 4]; 2]) -> Self {
        Self {
            map0: map[0],
            map1: map[1],
            info: [shape, material, 1, 0],
            extra: [material, 0, 0, 0],
            props: [1.0, 0.0, 0.0, 0.0],
        }
    }
}

#[derive(Clone, Debug)]
pub enum Background {
    Constant([f32; 3]),
    Gradient {
        colors: [[f32; 3]; 2],
        axis: [f32; 3],
        power: f32,
    },
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub camera: Camera,
    pub fov: f32,
    pub bounces: u32,
    pub radius: f32,
    pub medium: scene_ir::Medium,
    pub background: Background,
    pub objects: Vec<Object>,
    pub materials: Vec<Material>,
    pub(crate) generated: Option<std::sync::Arc<scene_ir::CompiledScene>>,
}

impl Scene {
    /// Compile a generic Rust scene lowered through `objects::Scene::wgsl_scene`.
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
            background: match definition.background {
                scene_ir::Background::Constant(color) => Background::Constant(color),
                scene_ir::Background::Gradient {
                    colors,
                    axis,
                    power,
                } => Background::Gradient {
                    colors,
                    axis,
                    power,
                },
            },
            objects: compiled
                .objects
                .iter()
                .map(|o| Object {
                    map0: o.map0,
                    map1: o.map1,
                    info: o.info,
                    extra: o.extra,
                    props: o.props,
                })
                .collect(),
            materials: vec![],
            generated: Some(std::sync::Arc::new(compiled)),
        };
        scene.validate()?;
        Ok(scene)
    }

    pub(crate) fn material_bytes(&self) -> &[u8] {
        if let Some(compiled) = &self.generated {
            bytemuck::cast_slice(&compiled.materials)
        } else {
            bytemuck::cast_slice(&self.materials)
        }
    }

    pub(crate) fn words(&self) -> &[u32] {
        self.generated
            .as_ref()
            .map_or(&[0], |compiled| compiled.words.as_slice())
    }

    /// Build upload records without mutating canonical scene data. Composition
    /// happens in f64; only the relative transforms are converted to f32.
    pub(crate) fn prepare_objects(&self, camera: Camera) -> Result<Vec<Object>> {
        anyhow::ensure!(
            camera.geometry() == self.camera.geometry(),
            "camera geometry must match the scene"
        );
        let mut objects = self.objects.clone();
        if let Some(compiled) = &self.generated {
            let inverse = camera.transform().inverse()?;
            anyhow::ensure!(
                objects.len() == compiled.transforms.len(),
                "compiled object/transform count differs"
            );
            for (object, map) in objects.iter_mut().zip(&compiled.transforms) {
                let rows = inverse.chain(map)?.rows()?;
                object.map0 = rows[0];
                object.map1 = rows[1];
            }
        } else {
            anyhow::ensure!(
                !matches!(camera, Camera::Embedded(_)),
                "embedded cameras require a generated scene"
            );
            camera.validate()?;
        }
        Ok(objects)
    }

    pub fn validate(&self) -> Result<()> {
        if let Some(compiled) = &self.generated {
            let geometry = compiled.geometry.tag();
            anyhow::ensure!(
                self.camera.geometry() == geometry,
                "camera geometry differs from the compiled scene; compile a new SceneDefinition"
            );
            anyhow::ensure!(
                self.radius == compiled.radius,
                "curvature radius must be updated through SceneDefinition"
            );
            // Raw legacy records are public for the baseline ABI experiments.
            // Generated records contain word-arena offsets and function IDs;
            // keep them coupled to the data validated by the scene compiler.
            anyhow::ensure!(
                self.materials.is_empty()
                    && bytemuck::cast_slice::<Object, u8>(&self.objects)
                        == bytemuck::cast_slice::<scene_ir::GpuObject, u8>(&compiled.objects),
                "generated object/material data must be updated through SceneDefinition"
            );
            self.camera.transform().components()?;
        } else {
            anyhow::ensure!(
                self.radius == 1.0 && self.medium == scene_ir::Medium::Vacuum,
                "radius and medium settings require a generated scene"
            );
            anyhow::ensure!(
                !matches!(self.camera, Camera::Embedded(_)),
                "embedded cameras require a generated scene"
            );
            self.camera.validate()?;
        }
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
        anyhow::ensure!(self.objects.len() <= u32::MAX as usize, "too many objects");
        for m in &self.materials {
            let floats: &[f32] = bytemuck::cast_slice(std::slice::from_ref(m));
            anyhow::ensure!(
                floats.iter().all(|x| x.is_finite() && *x >= 0.0),
                "material values must be finite and nonnegative"
            );
            anyhow::ensure!(m.properties[1] > 0.0, "refractive index must be positive");
            anyhow::ensure!(
                m.diffuse[3] + m.emission[3] + m.transmission[3] + m.properties[0] <= 1.00001,
                "material weights exceed one"
            );
        }
        for o in &self.objects {
            if self.generated.is_none() {
                validate_map([o.map0, o.map1], self.camera.geometry())?;
            }
            let [shape, base, count, tiling] = o.info;
            anyhow::ensure!(
                self.generated.is_some()
                    || (self.camera.geometry() == 0 && shape <= 2)
                    || (self.camera.geometry() == 1 && (shape == 0 || shape == 3)),
                "shape is unsupported by this geometry"
            );
            anyhow::ensure!(
                tiling <= 4 && (tiling == 0 || self.camera.geometry() == 1),
                "unsupported tiling"
            );
            let material_count = self
                .generated
                .as_ref()
                .map_or(self.materials.len(), |c| c.materials.len());
            anyhow::ensure!(
                count > 0
                    && u64::from(base) + u64::from(count) <= material_count as u64
                    && (o.extra[0] as usize) < material_count,
                "invalid object material indices"
            );
            anyhow::ensure!(
                o.map0
                    .into_iter()
                    .chain(o.map1)
                    .chain(o.props)
                    .all(f32::is_finite),
                "object data must be finite"
            );
            anyhow::ensure!(
                o.props[0] > 0.0 && o.props[1] >= 0.0,
                "invalid tiling dimensions"
            );
        }
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

    /// The existing Euclidean example, with its initial controller pose.
    pub fn eu() -> Self {
        let mut sphere = Material::diffuse([1.0, 0.2, 0.2]);
        sphere.diffuse[3] = 0.0;
        sphere.emission[3] = 0.1;
        sphere.transmission = [1.0, 1.0, 0.2, 0.0];
        sphere.properties = [0.9, 1.2, 0.0, 0.0];
        let cube = Material::diffuse([0.2, 0.8, 0.8]);
        let mut plane = Material::diffuse([1.0; 3]);
        plane.diffuse[3] = 0.9;
        plane.emission[3] = 0.1;
        Self {
            camera: Camera::Euclidean(Homogenous3::new(
                Shift::from_vector([0.0, 0.5, 2.0].into()),
                Rotation3::identity(),
            )),
            fov: 1.0,
            bounces: 4,
            radius: 1.0,
            medium: scene_ir::Medium::Vacuum,
            generated: None,
            background: Background::Gradient {
                colors: [[1.0; 3], [0.0; 3]],
                axis: [0.0, 1.0, 0.0],
                power: 2.4,
            },
            materials: vec![sphere, cube, plane],
            objects: vec![
                Object::new(1, 0, [[0.0, 1.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]]),
                Object::new(2, 1, [[0.0, -1.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]]),
                Object::new(0, 2, [[0.0, 0.0, -1.0, 0.0], [1.0, 0.0, 0.0, 0.0]]),
            ],
        }
    }

    /// The existing hyperbolic tiling example. Unused cell sizes use 1, avoiding
    /// the legacy NaN placeholder in GPU data.
    pub fn hy() -> Self {
        use std::f64::consts::PI;
        fn color(rgb: u32) -> [f32; 3] {
            [16, 8, 0].map(|shift| (((rgb >> shift) & 255) as f32 / 255.0).powf(2.2))
        }
        fn material(rgb: u32, transparent: f64) -> Material {
            let mut m = Material::diffuse(color(rgb));
            m.diffuse[3] = (1.0 - 0.1 - transparent) as f32;
            m.emission[3] = 0.1;
            m.transmission[3] = transparent as f32;
            m
        }
        let mut border = Material::diffuse(color(0xe4e4e4));
        border.emission = [1.0, 1.0, 1.0, 0.0];
        let materials = vec![
            border,
            material(0xfe0000, 0.1),
            material(0xffaa01, 0.1),
            material(0x35adae, 0.1),
            material(0xfe7401, 0.1),
            material(0xfe0000, 0.1),
            material(0xffaa01, 0.1),
            material(0xfed601, 0.1),
            material(0xfe7401, 0.0),
            material(0x35adae, 0.0),
            material(0xfe0000, 0.0),
            material(0xfed601, 0.0),
        ];
        let id = Moebius::identity();
        let shift = |x, y| {
            Moebius::new(
                Complex::new(1.0, 0.0),
                Complex::new(x, y),
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
            )
        };
        let mut objects = vec![
            Object::new(3, 1, moebius_rows(id)),
            Object::new(
                3,
                4,
                moebius_rows(shift(2.0f64.sqrt(), 0.0).chain(Hyperbolic3::rotate_x(PI))),
            ),
            Object::new(0, 8, moebius_rows(id)),
            Object::new(0, 10, moebius_rows(shift(0.0, 2.0))),
        ];
        for (o, (count, tiling, cell, border)) in objects.iter_mut().zip([
            (3, 2, 0.5, 0.02),
            (4, 1, 0.5, 0.02),
            (2, 4, 1.0, 0.01),
            (2, 3, 1.0, 0.02),
        ]) {
            o.info[2] = count;
            o.info[3] = tiling;
            o.extra[0] = 0;
            o.props = [cell, border, 0.0, 0.0];
        }
        Self {
            camera: Camera::Hyperbolic(
                Hyperbolic3::rotate_z(5.0 * PI / 6.0)
                    .chain(Hyperbolic3::rotate_x(2.0 * PI / 5.0))
                    .chain(Hyperbolic3::shift_z(2.0)),
            ),
            fov: 1.0,
            bounces: 3,
            radius: 1.0,
            medium: scene_ir::Medium::Vacuum,
            background: Background::Constant(color(0xeeeeee)),
            objects,
            materials,
            generated: None,
        }
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
        let [camera0, camera1] = if scene.generated.is_some() {
            // The shared kernel starts rays at the camera-relative origin.
            // Only a Euclidean gradient needs the absolute camera rotation.
            let rotation = if scene.camera.geometry() == 0 {
                scene
                    .camera
                    .transform()
                    .components()
                    .expect("validated camera")[0]
                    .map(|value| value as f32)
            } else {
                [1.0, 0.0, 0.0, 0.0]
            };
            [rotation, [0.0; 4]]
        } else {
            scene.camera.gpu_rows()
        };
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
                scene.objects.len() as u32,
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
        let mut definition = scenes::sp::scene::<6>().wgsl_scene().unwrap();
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
                scenes::eu::scene::<4>().wgsl_scene().unwrap(),
                scene_ir::Transform::Euclidean(Euclidean3::shift_x(1e9)),
            ),
            (
                scenes::hy::scene::<3>().wgsl_scene().unwrap(),
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
            let original_records = bytemuck::cast_slice::<Object, u8>(&shifted.objects).to_vec();
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
                bytemuck::cast_slice::<Object, u8>(&shifted.objects),
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
    fn raw_fixtures_reject_embedded_settings() {
        let mut scene = Scene::hy();
        scene.radius = 2.0;
        assert!(scene.validate().is_err());
        scene.radius = 1.0;
        scene.medium = scene_ir::Medium::Homogeneous {
            extinction: 0.0,
            albedo: [0.0; 3],
        };
        assert!(scene.validate().is_err());
        scene.medium = scene_ir::Medium::Vacuum;
        scene.camera = Camera::Embedded(scene.camera.transform().embedded().unwrap());
        assert!(scene.validate().is_err());
    }

    #[test]
    fn generated_validation_keeps_radius_constraints_and_medium_finite() {
        let scene =
            Scene::from_definition(&scenes::sp::scene::<6>().wgsl_scene().unwrap()).unwrap();
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
    fn reject_transform_destroyed_by_f32_conversion() {
        let mut scene = Scene::hy();
        scene.camera = Camera::Hyperbolic(Hyperbolic3::shift_x(18.0));
        assert!(
            scene
                .validate()
                .unwrap_err()
                .to_string()
                .contains("singular")
        );
        scene = Scene::hy();
        let [ab, cd] = moebius_rows(Hyperbolic3::shift_x(18.0));
        scene.objects[0].map0 = ab;
        scene.objects[0].map1 = cd;
        assert!(scene.validate().is_err());
        let mut eu = Scene::eu();
        eu.objects[0].map1 = [0.0; 4];
        assert!(eu.validate().is_err());
    }
    #[test]
    fn scene_abi_and_builtins() {
        assert_eq!(std::mem::size_of::<Material>(), 64);
        assert_eq!(std::mem::size_of::<Object>(), 80);
        assert_eq!(std::mem::size_of::<Params>(), 144);
        Scene::eu().validate().unwrap();
        Scene::hy().validate().unwrap();
        assert_eq!(
            Scene::eu().camera.gpu_rows(),
            [[0.0, 0.5, 2.0, 0.0], [1.0, 0.0, 0.0, 0.0]]
        );
    }
}
