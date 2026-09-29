//! Explicit CPU conversions for WGSL scene construction.
//!
//! Scene components own their shader modules and parameter encoding. Geometry
//! and map conversions are traits: downstream adapters need no type whitelist.
//!
//! ```
//! use ccgeom::Euclidean3;
//! use hypertrace_objects::{background::ConstBg, material::Lambertian,
//!     object::Covered, shape::Sphere, view::PointView, Scene, SceneImpl};
//! # fn main() -> hypertrace_objects::shader::Result<()> {
//! let scene = SceneImpl::<Euclidean3, _, _, _, 4>::new(
//!     PointView::new(1.0), vec![Covered::new(Sphere, Lambertian)],
//!     ConstBg::new([0.1, 0.2, 0.3].into()));
//! assert_eq!(scene.definition()?.bounces, 4);
//! # Ok(())
//! # }
//! ```

pub use ::scene::*;
use ccgeom::{Embedded3, EmbeddedIsometry, Euclidean3, Homogenous3, Hyperbolic3};
use vecmat::{
    transform::{Moebius, Rotation3, Shift},
    Complex, QuaternionPair, Transform as _,
};

/// Construction coordinates supported by a geometry adapter.
/// The renderer itself always receives embedded constant-curvature geometry.
pub trait RenderGeometry: ccgeom::Geometry {
    fn render_geometry() -> Result<Geometry>;
    fn render_identity() -> Result<Transform>;
}

/// Convert a construction map to a checked, canonical f64 isometry.
pub trait RenderMap<G: ccgeom::Geometry>: ccgeom::Map<G::Pos, G::Dir> {
    fn render_transform(&self) -> Result<Transform>;
}

impl RenderGeometry for Euclidean3 {
    fn render_geometry() -> Result<Geometry> {
        Ok(Geometry::Euclidean)
    }
    fn render_identity() -> Result<Transform> {
        Ok(Transform::Euclidean(Homogenous3::identity()))
    }
}
impl RenderGeometry for Hyperbolic3 {
    fn render_geometry() -> Result<Geometry> {
        Ok(Geometry::Hyperbolic)
    }
    fn render_identity() -> Result<Transform> {
        Ok(Transform::Hyperbolic(Moebius::identity()))
    }
}
impl<const K: i8> RenderGeometry for Embedded3<f64, K> {
    fn render_geometry() -> Result<Geometry> {
        match K {
            -1 => Ok(Geometry::Hyperbolic),
            0 => Ok(Geometry::Euclidean),
            1 => Ok(Geometry::Spherical),
            _ => Err(unsupported::<Self>()),
        }
    }
    fn render_identity() -> Result<Transform> {
        embedded_transform(&EmbeddedIsometry::<f64, K>::identity())
    }
}
fn embedded_transform<const K: i8>(map: &EmbeddedIsometry<f64, K>) -> Result<Transform> {
    let values = map.pair().into_array();
    // Reconstruct across const parameters without runtime type inspection. Each
    // constructor validates the same pair in its explicitly chosen algebra.
    let invalid = || unsupported::<EmbeddedIsometry<f64, K>>();
    Ok(match K {
        0 => Transform::Flat(
            EmbeddedIsometry::from_pair(QuaternionPair::from_array(values)).ok_or_else(invalid)?,
        ),
        -1 => Transform::Hyperboloid(
            EmbeddedIsometry::from_pair(QuaternionPair::from_array(values)).ok_or_else(invalid)?,
        ),
        1 => Transform::Spherical(
            EmbeddedIsometry::from_pair(QuaternionPair::from_array(values)).ok_or_else(invalid)?,
        ),
        _ => return Err(invalid()),
    })
}
impl<const K: i8> RenderMap<Embedded3<f64, K>> for EmbeddedIsometry<f64, K> {
    fn render_transform(&self) -> Result<Transform> {
        embedded_transform(self)
    }
}
impl RenderMap<Euclidean3> for Homogenous3<f64> {
    fn render_transform(&self) -> Result<Transform> {
        Ok(Transform::Euclidean(*self))
    }
}
impl RenderMap<Euclidean3> for Shift<f64, 3> {
    fn render_transform(&self) -> Result<Transform> {
        Ok(Transform::Euclidean(Homogenous3::new(
            *self,
            Rotation3::identity(),
        )))
    }
}
impl RenderMap<Euclidean3> for Rotation3<f64> {
    fn render_transform(&self) -> Result<Transform> {
        Ok(Transform::Euclidean(Homogenous3::new(
            Shift::identity(),
            *self,
        )))
    }
}
impl RenderMap<Hyperbolic3> for Moebius<Complex<f64>> {
    fn render_transform(&self) -> Result<Transform> {
        Ok(Transform::Hyperbolic(*self))
    }
}
pub fn geometry<G: RenderGeometry>() -> Result<Geometry> {
    G::render_geometry()
}
pub fn identity<G: RenderGeometry>() -> Result<Transform> {
    G::render_identity()
}
pub fn transform<G: ccgeom::Geometry, M: RenderMap<G>>(map: &M) -> Result<Transform> {
    map.render_transform()
}
pub fn lower<G: ccgeom::Geometry, S: crate::Scene<G>>(scene: &S) -> Result<SceneDefinition> {
    scene.definition()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        background::{ConstBg, GradBg},
        material::{Absorbing, Colored, Emissive, Lambertian, Refractive, Specular, Transparent},
        object::Covered,
        shape::{Cube, GeodesicSphere, Horosphere, Plane, Sphere},
        view::PointView,
        Mapped, Material, Object, Scene, SceneImpl, Shape, View as _,
    };
    use ccgeom::{Flat3, Geometry3, Hyperboloid3, Spherical3};

    #[test]
    fn specialized_shapes_and_gradient_share_geometry_families() {
        let legacy_cube = <Cube as Shape<Euclidean3>>::encode(&Cube).unwrap();
        let embedded_cube = <Cube as Shape<Flat3>>::encode(&Cube).unwrap();
        assert!(legacy_cube
            .schema
            .same_implementation(&embedded_cube.schema));
        assert_eq!(legacy_cube.words, embedded_cube.words);
        let legacy_horosphere = <Horosphere as Shape<Hyperbolic3>>::encode(&Horosphere).unwrap();
        let embedded_horosphere = <Horosphere as Shape<Hyperboloid3>>::encode(&Horosphere).unwrap();
        assert!(legacy_horosphere
            .schema
            .same_implementation(&embedded_horosphere.schema));
        assert_eq!(legacy_horosphere.words, embedded_horosphere.words);
        let gradient = GradBg::new(
            [0.0, 1.0, 0.0].into(),
            [[1.0; 3].into(), [0.0; 3].into()],
            2.4,
        );
        let legacy = <GradBg as crate::Background<Euclidean3>>::background(&gradient).unwrap();
        let embedded = <GradBg as crate::Background<Flat3>>::background(&gradient).unwrap();
        match (legacy, embedded) {
            (
                Background::Gradient {
                    colors: a,
                    axis: b,
                    power: c,
                },
                Background::Gradient {
                    colors: x,
                    axis: y,
                    power: z,
                },
            ) => {
                assert_eq!((a, b, c), (x, y, z));
            }
            _ => panic!("gradient lowering changed its kind"),
        }
        assert!(<Cube as Shape<Hyperboloid3>>::encode(&Cube).is_err());
        assert!(<Cube as Shape<Spherical3>>::shader().is_err());
        assert!(<Horosphere as Shape<Flat3>>::encode(&Horosphere).is_err());
        assert!(<Horosphere as Shape<Spherical3>>::shader().is_err());
        assert!(<GradBg as crate::Background<Hyperboloid3>>::background(&gradient).is_err());
        assert!(<GradBg as crate::Background<Spherical3>>::background(&gradient).is_err());
    }

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
        let lowered = scene.definition().unwrap();
        assert_eq!(lowered.view.map.geometry().sign(), K);
        assert_eq!(
            lowered.view.map.components().unwrap(),
            transform::<G<K>, _>(&map).unwrap().components().unwrap()
        );
        assert!(lowered
            .modules
            .iter()
            .any(|module| module.same_implementation(&crate::shape::geodesic_sphere_schema())));
        assert_eq!(lowered.radius, scene.radius);
        assert!(matches!(
            lowered.medium,
            Medium::Homogeneous {
                extinction: 0.05,
                albedo: [0.8, 0.8, 0.8]
            }
        ));
        assert!(<Plane as Shape<G<K>>>::shader()
            .unwrap()
            .same_implementation(&crate::shape::plane_schema()));
        assert!(<Sphere as Shape<G<K>>>::shader()
            .unwrap()
            .same_implementation(&crate::shape::sphere_schema()));
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
                <GeodesicSphere as Shape<Spherical3>>::encode(&GeodesicSphere::new(radius))
                    .is_err()
            );
        }
        let shape =
            <GeodesicSphere as Shape<Spherical3>>::encode(&GeodesicSphere::new(0.3)).unwrap();
        assert!(shape
            .schema
            .same_implementation(&crate::shape::geodesic_sphere_schema()));
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
        let value = mixture.encode().unwrap();
        let outer = &value.schema;
        assert!(outer.source.contains("var choice=uniform_random(rng)"));
        let colored = &outer.dependencies[0];
        assert!(colored.source.contains("(*sample).attenuation*="));
        let emissive = &colored.dependencies[0];
        assert!(emissive.source.contains("(*sample).emission+="));
        let nested = &emissive.dependencies[0];
        assert!(nested.source.contains("var choice=uniform_random(rng)"));
        assert_eq!(nested.dependencies[0].key, "hypertrace.material.specular");
        assert_eq!(nested.dependencies[1].key, "hypertrace.material.refractive");
        let floats = |words: &[u32]| {
            words
                .iter()
                .copied()
                .map(f32::from_bits)
                .collect::<Vec<_>>()
        };
        assert_eq!(floats(&value.words[..2]), [0.6, 0.4]);
        let branches = crate::parameters::slices(&value.words, 2, 2).unwrap();
        assert_eq!(floats(&branches[0][..6]), [0.25, 0.5, 0.75, 2.0, 3.0, 4.0]);
        assert!(branches[1].is_empty());
        let nested = &branches[0][6..];
        assert_eq!(floats(&nested[..2]), [0.25, 0.75]);
        let branches = crate::parameters::slices(nested, 2, 2).unwrap();
        assert!(branches[0].is_empty());
        assert_eq!(floats(branches[1]), [1.5]);
        // A zero-weight branch still exists in the schema: flattening it would
        // change nested RNG behavior when values change on an existing pipeline.
        mixture.modified.portion = 0.0;
        mixture.transmitted.portion = 1.0;
        assert!(mixture
            .encode()
            .unwrap()
            .schema
            .same_implementation(&value.schema));
        assert!(OuterMixture::shader()
            .unwrap()
            .same_implementation(&value.schema));
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
        let empty = scene.definition().unwrap();
        assert_eq!(empty.modules.len(), 4);
        let empty_source = compile(&empty).unwrap().source;
        scene
            .object
            .push(ObjectChoices::Diffuse(Covered::new(Sphere, Lambertian)));
        let populated = scene.definition().unwrap();
        assert_eq!(empty_source, compile(&populated).unwrap().source);
        scene.object[0] = ObjectChoices::Emitting(Covered::new(
            Cube,
            Emissive::new(Absorbing, [1.0; 3].into()),
        ));
        let changed = scene.definition().unwrap();
        assert_eq!(empty_source, compile(&changed).unwrap().source);
    }

    #[test]
    fn nested_camera_maps_keep_outer_inner_order_in_f64() {
        let shifted = Mapped::<Euclidean3, _, _>::new(
            PointView::new(0.7),
            Shift::from_vector([1.0, 0.0, 0.0].into()),
        );
        let rotated = Mapped::new(shifted, Euclidean3::rotate_z(std::f64::consts::FRAC_PI_2));
        let view = rotated.view().unwrap();
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
        let covered_shape = Covered::new(shape, Lambertian).object_node().unwrap();
        match covered_shape {
            ObjectNode::Covered { shape, .. } => {
                assert!(shape.schema.key.contains("hypertrace.shape.mapped"))
            }
            _ => panic!("shape transform was moved across its material"),
        }
        let covered = Covered::<Euclidean3, _, _>::new(Plane, Lambertian);
        let mapped_object = Mapped::new(covered, offset).object_node().unwrap();
        assert!(matches!(mapped_object, ObjectNode::Mapped { .. }));
    }

    #[test]
    fn nested_shape_leaves_cannot_omit_their_identity_word() -> Result<()> {
        let mut definition = SceneImpl::<Euclidean3, _, _, _, 1>::new(
            PointView::new(1.0),
            Covered::new(Plane, Absorbing),
            ConstBg::new([0.0; 3].into()),
        )
        .definition()?;
        for parameter_words in [Some(0), None] {
            let module = ShaderModule::new(
                "test.empty-shape",
                ShaderKind::Shape,
                "fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit {return GeoTaggedHit(geo_miss(),base);}",
                parameter_words,
            );
            // Public value fields can bypass the constructor's allocation check.
            // Wrappers must still reject a leaf whose identity would point past
            // its own payload, even when the wrapper itself has parameter words.
            let malformed = ShapeValue {
                schema: module.clone(),
                words: vec![],
            };
            let choice = crate::shape::choice(vec![module], 0, malformed)?;
            let mapped =
                crate::shape::mapped(choice.clone(), Transform::identity(Geometry::Euclidean))?;
            let vector = crate::shape::vector(choice.schema.clone(), vec![choice.clone()])?;
            for shape in [choice, mapped, vector] {
                definition.object = ObjectNode::Covered {
                    shape,
                    material: crate::material::absorbing(),
                };
                let error = compile(&definition).unwrap_err();
                assert!(format!("{error:#}").contains("shapes must reserve an identity word"));
            }
        }
        Ok(())
    }

    #[derive(Clone)]
    struct CustomShift(vecmat::Vector<f64, 3>);
    impl ccgeom::Map<vecmat::Vector<f64, 3>> for CustomShift {
        fn identity() -> Self {
            Self([0.0; 3].into())
        }
        fn apply_pos(&self, pos: vecmat::Vector<f64, 3>) -> vecmat::Vector<f64, 3> {
            pos + self.0
        }
        fn apply_dir(
            &self,
            _pos: vecmat::Vector<f64, 3>,
            dir: vecmat::Vector<f64, 3>,
        ) -> vecmat::Vector<f64, 3> {
            dir
        }
        fn apply_normal(
            &self,
            _pos: vecmat::Vector<f64, 3>,
            normal: vecmat::Vector<f64, 3>,
        ) -> vecmat::Vector<f64, 3> {
            normal
        }
        fn chain(self, other: Self) -> Self {
            Self(self.0 + other.0)
        }
        fn inv(self) -> Self {
            Self(-self.0)
        }
    }
    impl RenderMap<Euclidean3> for CustomShift {
        fn render_transform(&self) -> Result<Transform> {
            Ok(Transform::Euclidean(Homogenous3::new(
                Shift::from_vector(self.0),
                Rotation3::identity(),
            )))
        }
    }
    #[test]
    fn downstream_map_adapter_lowers_without_type_registration() {
        let map = CustomShift([1.25, -2.0, 3.0].into());
        let view = Mapped::<Euclidean3, _, _>::new(PointView::new(1.0), map.clone())
            .view()
            .unwrap();
        let position = match view.map.embedded().unwrap() {
            Transform::Flat(map) => map.apply_vector([1.0, 0.0, 0.0, 0.0].into()),
            _ => unreachable!(),
        };
        assert_eq!(position.into_array(), [1.0, 1.25, -2.0, 3.0]);
        let scene = SceneImpl::<Euclidean3, _, _, _, 2>::new(
            PointView::new(1.0),
            Mapped::new(Covered::new(Plane, Lambertian), map.clone()),
            ConstBg::new([0.0; 3].into()),
        );
        let compiled = compile(&scene.definition().unwrap()).unwrap();
        assert_eq!(
            compiled.transforms[0].components().unwrap(),
            map.render_transform().unwrap().components().unwrap()
        );
    }
}
