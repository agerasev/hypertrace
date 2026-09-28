//! CPU-only lowering of existing scene builders for the WGSL renderer.
//!
//! The semantic traits expose fallible hooks with unsupported defaults. Custom
//! implementations report an error until their WGSL hooks are supplied. Materials
//! and shapes can return custom shader leaves without changing renderer dispatch.
//!
//! ```
//! use ccgeom::Euclidean3;
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
//! Supported maps are embedded quaternion-pair isometries in all three spaces,
//! Euclidean shifts, rotations, and homogeneous rigid maps, plus legacy complex
//! Möbius maps in hyperbolic space. Other maps return an
//! explicit unsupported error. Nested shape maps remain inside their covering
//! material; object maps retain the object's local material frame.

pub use ::scene::*;

use ccgeom::{
    EmbeddedIsometry, Euclidean3, Flat3, Homogenous3, Hyperbolic3, Hyperboloid3, Spherical3,
};
use std::any::{Any, TypeId};
use vecmat::{
    transform::{Moebius, Rotation3, Shift},
    Complex, Transform as _,
};

/// Identify the geometry supported by the current WGSL compiler.
pub fn geometry<G: ccgeom::Geometry>() -> Result<Geometry> {
    if TypeId::of::<G>() == TypeId::of::<Euclidean3>() || TypeId::of::<G>() == TypeId::of::<Flat3>()
    {
        Ok(Geometry::Euclidean)
    } else if TypeId::of::<G>() == TypeId::of::<Hyperbolic3>()
        || TypeId::of::<G>() == TypeId::of::<Hyperboloid3>()
    {
        Ok(Geometry::Hyperbolic)
    } else if TypeId::of::<G>() == TypeId::of::<Spherical3>() {
        Ok(Geometry::Spherical)
    } else {
        Err(unsupported::<G>())
    }
}

/// Identity in the coordinate representation chosen by the scene builder.
pub fn identity<G: ccgeom::Geometry>() -> Result<Transform> {
    if TypeId::of::<G>() == TypeId::of::<Flat3>() {
        Ok(Transform::Flat(EmbeddedIsometry::identity()))
    } else if TypeId::of::<G>() == TypeId::of::<Hyperboloid3>() {
        Ok(Transform::Hyperboloid(EmbeddedIsometry::identity()))
    } else {
        Ok(Transform::identity(geometry::<G>()?))
    }
}

/// Lower supported rigid maps; other map types return an unsupported error.
pub fn transform<G, M>(map: &M) -> Result<Transform>
where
    G: ccgeom::Geometry,
    M: ccgeom::Map<G::Pos, G::Dir> + 'static,
{
    let map = map as &dyn Any;
    match geometry::<G>()? {
        Geometry::Euclidean => {
            if let Some(map) = map.downcast_ref::<EmbeddedIsometry<f64, 0>>() {
                return Ok(Transform::Flat(*map));
            }
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
            if let Some(map) = map.downcast_ref::<EmbeddedIsometry<f64, -1>>() {
                return Ok(Transform::Hyperboloid(*map));
            }
            if let Some(map) = map.downcast_ref::<Moebius<Complex<f64>>>() {
                return Ok(Transform::Hyperbolic(*map));
            }
        }
        Geometry::Spherical => {
            if let Some(map) = map.downcast_ref::<EmbeddedIsometry<f64, 1>>() {
                return Ok(Transform::Spherical(*map));
            }
        }
    }
    Err(unsupported::<M>())
}

/// Lower an existing scene builder to a device-independent WGSL definition.
pub fn lower<G, S>(scene: &S) -> Result<SceneDefinition>
where
    G: ccgeom::Geometry,
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
        shape::{Cube, GeodesicSphere, Plane, Sphere},
        view::PointView,
        Mapped, Material, Object, Scene, SceneImpl, Shape, View as _,
    };
    use ccgeom::Geometry3;

    fn embedded_lowering<const K: i8>() {
        type G<const K: i8> = ccgeom::Embedded3<f64, K>;
        let map = ccgeom::Space3::<f64, K>::unit()
            .translation([0.0, 0.0, -1.0].into(), 0.6)
            .unwrap();
        let mut scene = SceneImpl::<G<K>, _, _, _, 5>::new(
            Mapped::new(PointView::new(0.8), map),
            vec![Mapped::new(
                Covered::new(
                    GeodesicSphere::new(0.25),
                    Emissive::new(Lambertian, [2.0; 3].into()),
                ),
                map,
            )],
            ConstBg::new([0.0; 3].into()),
        );
        scene.radius = if K == 0 { 1.0 } else { 2.0 };
        scene.medium = Medium::Homogeneous {
            extinction: 0.05,
            albedo: [0.8; 3],
        };
        let lowered = scene.wgsl_scene().unwrap();
        assert_eq!(lowered.view.map.geometry().sign(), K);
        assert_eq!(
            lowered.view.map.components().unwrap(),
            transform::<G<K>, _>(&map).unwrap().components().unwrap()
        );
        assert_eq!(lowered.shape_schemas, [ShapeSchema::GeodesicSphere]);
        assert_eq!(lowered.radius, scene.radius);
        assert!(matches!(
            lowered.medium,
            Medium::Homogeneous {
                extinction: 0.05,
                albedo: [0.8, 0.8, 0.8]
            }
        ));
        assert_eq!(
            <Plane as Shape<G<K>>>::wgsl_shape_schema().unwrap(),
            ShapeSchema::Plane
        );
        assert_eq!(
            <Sphere as Shape<G<K>>>::wgsl_shape_schema().unwrap(),
            ShapeSchema::Sphere
        );
    }

    #[test]
    fn all_embedded_geometries_lower_through_shared_shapes_and_materials() {
        embedded_lowering::<-1>();
        embedded_lowering::<0>();
        embedded_lowering::<1>();
        assert!(matches!(identity::<Flat3>().unwrap(), Transform::Flat(_)));
        assert!(matches!(
            identity::<Hyperboloid3>().unwrap(),
            Transform::Hyperboloid(_)
        ));
        assert!(matches!(
            identity::<Spherical3>().unwrap(),
            Transform::Spherical(_)
        ));
    }

    #[test]
    fn geodesic_sphere_rejects_invalid_physical_radius() {
        for radius in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(
                <GeodesicSphere as Shape<Spherical3>>::wgsl_shape(&GeodesicSphere::new(radius))
                    .is_err()
            );
        }
        let shape =
            <GeodesicSphere as Shape<Spherical3>>::wgsl_shape(&GeodesicSphere::new(0.3)).unwrap();
        assert_eq!(shape.schema, ShapeSchema::GeodesicSphere);
        assert_eq!(f32::from_bits(shape.words[0]), 0.3);
    }

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
        ObjectChoices {
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
        let map = match view.map.embedded().unwrap() {
            Transform::Flat(map) => map,
            _ => panic!("wrong geometry"),
        };
        let position = map.apply_vector([1.0, 0.0, 0.0, 0.0].into());
        assert_eq!(position[0], 1.0);
        assert!(position[1].abs() < 1e-14);
        assert!((position[2] - 1.0).abs() < 1e-14);
        assert_eq!(position[3], 0.0);
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

    // Unsupported leaves still compose, but lowering reports the missing hook.
    #[derive(Clone)]
    pub struct UnsupportedMaterial;
    impl Material for UnsupportedMaterial {}
    #[derive(Clone)]
    pub struct UnsupportedShape;
    impl Shape<Euclidean3> for UnsupportedShape {}
    crate::mixture! { UnsupportedMixture { old: UnsupportedMaterial, diffuse: Lambertian } }
    crate::shape_choice! { UnsupportedShapes { Old(UnsupportedShape), Sphere(Sphere) } }
    crate::object_choice! {
        UnsupportedObjects {
            Old(Covered<Euclidean3, UnsupportedShapes, UnsupportedMixture>),
        }
    }

    #[test]
    fn unsupported_custom_types_report_errors_inside_macros() {
        let material =
            UnsupportedMixture::new((UnsupportedMaterial, 0.5).into(), (Lambertian, 0.5).into());
        let object = UnsupportedObjects::Old(Covered::new(
            UnsupportedShapes::Old(UnsupportedShape),
            material,
        ));
        assert!(object
            .wgsl_object()
            .unwrap_err()
            .to_string()
            .contains("UnsupportedShape"));
        assert!(UnsupportedMixture::wgsl_material_schema()
            .unwrap_err()
            .to_string()
            .contains("UnsupportedMaterial"));
        assert!(
            <UnsupportedObjects as Object<Euclidean3>>::wgsl_register(&mut Registry::default())
                .is_err()
        );
    }
}
