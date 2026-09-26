//! CPU-only lowering of existing scene builders for the WGSL renderer.
//!
//! The semantic traits expose fallible hooks with unsupported defaults. Existing
//! OpenCL-only implementations continue to compile; they need a WGSL hook only
//! when a caller requests WGSL lowering. Custom materials and shapes can return
//! a custom shader leaf without changing renderer dispatch code.
//!
//! ```
//! use base::ccgeom::Euclidean3;
//! use hypertrace_objects::{
//!     background::ConstBg, material::Lambertian, object::Covered,
//!     shape::Sphere, view::PointView, Scene, SceneImpl,
//! };
//! # fn main() -> hypertrace_objects::wgsl::Result<()> {
//! let scene = SceneImpl::<Euclidean3, _, _, _, 4>::new(
//!     PointView::new(1.0),
//!     vec![Covered::new(Sphere, Lambertian)],
//!     ConstBg::new([0.1, 0.2, 0.3].into()),
//! );
//! let definition = scene.wgsl_scene()?;
//! assert_eq!(definition.bounces, 4);
//! # Ok(())
//! # }
//! ```
//!
//! Schemas describe shader implementations and stay independent of parameter
//! values. Choice macros register every variant, and vectors register their
//! element type even when empty. Consequently every possible choice variant
//! needs WGSL support before the scene can be lowered, while changing a current
//! variant or vector length does not change the set of shader implementations.
//!
//! Supported maps are Euclidean shifts, rotations, and homogeneous rigid maps,
//! plus complex Möbius maps in hyperbolic space. Other legacy maps return an
//! explicit unsupported error. Nested shape maps remain inside their covering
//! material; object maps retain the object's local material frame.

pub use ::scene::*;

use base::{
    ccgeom::{Euclidean3, Homogenous3, Hyperbolic3},
    vecmat::{
        transform::{Moebius, Rotation3, Shift},
        Complex, Transform as _,
    },
};
use std::any::{Any, TypeId};

/// Identify the geometry supported by the current WGSL compiler.
pub fn geometry<G: types::Geometry>() -> Result<Geometry> {
    if TypeId::of::<G>() == TypeId::of::<Euclidean3>() {
        Ok(Geometry::Euclidean)
    } else if TypeId::of::<G>() == TypeId::of::<Hyperbolic3>() {
        Ok(Geometry::Hyperbolic)
    } else {
        Err(unsupported::<G>())
    }
}

/// Lower supported rigid maps without imposing new bounds on OpenCL builders.
/// Unsupported legacy maps return an error only when WGSL lowering is requested.
pub fn transform<G, M>(map: &M) -> Result<Transform>
where
    G: types::Geometry,
    M: types::Map<G::Pos, G::Dir>,
{
    let map = map as &dyn Any;
    match geometry::<G>()? {
        Geometry::Euclidean => {
            if let Some(map) = map.downcast_ref::<Homogenous3<f64>>() {
                return Ok(Transform::Euclidean(*map));
            }
            if let Some(map) = map.downcast_ref::<Shift<f64, 3>>() {
                return Ok(Transform::Euclidean(Homogenous3::new(
                    *map,
                    Rotation3::identity(),
                )));
            }
            if let Some(map) = map.downcast_ref::<Rotation3<f64>>() {
                return Ok(Transform::Euclidean(Homogenous3::new(
                    Shift::identity(),
                    *map,
                )));
            }
        }
        Geometry::Hyperbolic => {
            if let Some(map) = map.downcast_ref::<Moebius<Complex<f64>>>() {
                return Ok(Transform::Hyperbolic(*map));
            }
        }
    }
    Err(unsupported::<M>())
}

/// Lower an existing scene builder to a device-independent WGSL definition.
pub fn lower<G, S>(scene: &S) -> Result<SceneDefinition>
where
    G: types::Geometry,
    S: crate::Scene<G>,
{
    scene.wgsl_scene()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        background::ConstBg,
        material::{Absorbing, Colored, Emissive, Lambertian, Refractive, Specular, Transparent},
        object::Covered,
        shape::{Cube, Plane, Sphere},
        view::PointView,
        Mapped, Material, Object, Scene, SceneImpl, Shape, View as _,
    };
    use base::ccgeom::Geometry3;
    use type_macros::{Entity, EntityId, EntitySource, SizedEntity};

    crate::mixture! {
        InnerMixture {
            reflected: Specular,
            refracted: Refractive,
        }
    }
    crate::mixture! {
        OuterMixture {
            modified: Colored<Emissive<InnerMixture>>,
            transmitted: Transparent,
        }
    }

    #[test]
    fn nested_material_lowering_preserves_order_and_draw_boundaries() {
        let inner = InnerMixture::new((Specular, 0.25).into(), (Refractive::new(1.5), 0.75).into());
        let modified = Colored::new(
            Emissive::new(inner, [2.0, 3.0, 4.0].into()),
            [0.25, 0.5, 0.75].into(),
        );
        let mut mixture = OuterMixture::new((modified, 0.6).into(), (Transparent, 0.4).into());
        let value = mixture.wgsl_material().unwrap();
        assert_eq!(
            value.schema,
            MaterialSchema::Mixture(vec![
                MaterialSchema::Colored(Box::new(MaterialSchema::Emissive(Box::new(
                    MaterialSchema::Mixture(vec![
                        MaterialSchema::Specular,
                        MaterialSchema::Refractive
                    ]),
                )))),
                MaterialSchema::Transparent,
            ])
        );
        let actual: Vec<f32> = value.words.into_iter().map(f32::from_bits).collect();
        assert_eq!(
            actual,
            vec![0.6, 0.4, 0.25, 0.5, 0.75, 2.0, 3.0, 4.0, 0.25, 0.75, 1.5]
        );
        // A zero-weight branch still exists in the schema: flattening it would
        // change nested RNG behavior when values change on an existing pipeline.
        mixture.modified.portion = 0.0;
        mixture.transmitted.portion = 1.0;
        assert_eq!(mixture.wgsl_material().unwrap().schema, value.schema);
        assert_eq!(OuterMixture::wgsl_material_schema().unwrap(), value.schema);
    }

    crate::object_choice! {
        ObjectChoices(ObjectChoicesCache) {
            Diffuse(Covered<Euclidean3, Sphere, Lambertian>),
            Emitting(Covered<Euclidean3, Cube, Emissive<Absorbing>>),
        }
    }

    #[test]
    fn empty_object_vectors_register_inactive_choice_implementations() {
        let mut scene = SceneImpl::<Euclidean3, _, _, _, 3>::new(
            PointView::new(1.0),
            Vec::<ObjectChoices>::new(),
            ConstBg::new([0.0; 3].into()),
        );
        let empty = scene.wgsl_scene().unwrap();
        assert_eq!(
            empty.shape_schemas,
            vec![ShapeSchema::Sphere, ShapeSchema::Cube]
        );
        assert_eq!(
            empty.material_schemas,
            vec![
                MaterialSchema::Lambertian,
                MaterialSchema::Emissive(Box::new(MaterialSchema::Absorbing))
            ]
        );
        scene
            .object
            .push(ObjectChoices::Diffuse(Covered::new(Sphere, Lambertian)));
        let populated = scene.wgsl_scene().unwrap();
        assert_eq!(empty.shape_schemas, populated.shape_schemas);
        assert_eq!(empty.material_schemas, populated.material_schemas);
        scene.object[0] = ObjectChoices::Emitting(Covered::new(
            Cube,
            Emissive::new(Absorbing, [1.0; 3].into()),
        ));
        let changed = scene.wgsl_scene().unwrap();
        assert_eq!(empty.shape_schemas, changed.shape_schemas);
        assert_eq!(empty.material_schemas, changed.material_schemas);
    }

    #[test]
    fn nested_camera_maps_keep_outer_inner_order_in_f64() {
        let shifted = Mapped::<Euclidean3, _, _>::new(
            PointView::new(0.7),
            Shift::from_vector([1.0, 0.0, 0.0].into()),
        );
        let rotated = Mapped::new(shifted, Euclidean3::rotate_z(std::f64::consts::FRAC_PI_2));
        let view = rotated.wgsl_view().unwrap();
        assert_eq!(view.fov, 0.7);
        let map = match view.map {
            Transform::Euclidean(map) => map,
            _ => panic!("wrong geometry"),
        };
        let position = map.apply([0.0; 3].into());
        assert!(position[0].abs() < 1e-14);
        assert!((position[1] - 1.0).abs() < 1e-14);
        assert_eq!(position[2], 0.0);
    }

    #[test]
    fn mapped_shapes_and_mapped_objects_keep_distinct_material_frames() {
        let offset = Shift::from_vector([2.0, 0.0, 0.0].into());
        let shape = Mapped::<Euclidean3, _, _>::new(Plane, offset);
        let covered_shape = Covered::new(shape, Lambertian).wgsl_object().unwrap();
        match covered_shape {
            ObjectNode::Covered { shape, .. } => {
                assert!(matches!(shape.schema, ShapeSchema::Mapped { .. }))
            }
            _ => panic!("shape transform was moved across its material"),
        }
        let covered = Covered::<Euclidean3, _, _>::new(Plane, Lambertian);
        let mapped_object = Mapped::new(covered, offset).wgsl_object().unwrap();
        assert!(matches!(mapped_object, ObjectNode::Mapped { .. }));
    }

    // These implementations deliberately have no WGSL hooks. Macro expansion
    // must still compile, and OpenCL source generation must remain callable.
    #[derive(Clone, EntityId, Entity, SizedEntity, EntitySource)]
    pub struct LegacyMaterial;
    impl Material for LegacyMaterial {
        fn material_name() -> (String, String) {
            Absorbing::material_name()
        }
        fn material_source(config: &types::Config) -> types::source::SourceTree {
            Absorbing::material_source(config)
        }
    }
    #[derive(Clone, EntityId, Entity, SizedEntity, EntitySource)]
    pub struct LegacyShape;
    impl Shape<Euclidean3> for LegacyShape {
        fn shape_name() -> (String, String) {
            <Plane as Shape<Euclidean3>>::shape_name()
        }
        fn shape_source(config: &types::Config) -> types::source::SourceTree {
            <Plane as Shape<Euclidean3>>::shape_source(config)
        }
    }
    crate::mixture! { LegacyMixture { old: LegacyMaterial, diffuse: Lambertian } }
    crate::shape_choice! { LegacyShapes { Old(LegacyShape), Sphere(Sphere) } }
    crate::object_choice! {
        LegacyObjects(LegacyObjectsCache) {
            Old(Covered<Euclidean3, LegacyShapes, LegacyMixture>),
        }
    }

    #[test]
    fn legacy_only_custom_types_still_work_inside_generated_macros() {
        let material = LegacyMixture::new((LegacyMaterial, 0.5).into(), (Lambertian, 0.5).into());
        let object = LegacyObjects::Old(Covered::new(LegacyShapes::Old(LegacyShape), material));
        let config = types::Config {
            endian: types::config::Endian::Little,
            address_width: types::config::AddressWidth::X32,
            double_support: false,
        };
        let _source = <LegacyObjects as Object<Euclidean3>>::object_source(&config);
        assert!(object
            .wgsl_object()
            .unwrap_err()
            .to_string()
            .contains("LegacyShape"));
        assert!(LegacyMixture::wgsl_material_schema()
            .unwrap_err()
            .to_string()
            .contains("LegacyMaterial"));
        assert!(
            <LegacyObjects as Object<Euclidean3>>::wgsl_register(&mut Registry::default()).is_err()
        );
    }
}
