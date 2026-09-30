use crate::shader::*;
use crate::{
    background::{ConstBg, GradBg},
    material::{Absorbing, Colored, Emissive, Lambertian, Refractive, Specular, Transparent},
    object::Covered,
    shape::{Cube, GeodesicSphere, Horosphere, Plane, Sphere},
    view::PointView,
    Mapped, Material, Object, Scene, SceneImpl, Shape, View as _,
};
use ccgeom::{Embedded3, EmbeddedIsometry};
use ccgeom::{Flat3, Hyperboloid3, Space3, Spherical3};

#[test]
fn specialized_shapes_and_gradient_enforce_geometry_requirements() {
    assert!(<Cube as Shape<Flat3>>::encode(&Cube::new(1.0)).is_ok());
    assert!(<Horosphere as Shape<Hyperboloid3>>::encode(&Horosphere).is_ok());
    let gradient = GradBg::new(
        [0.0, 1.0, 0.0].into(),
        [[1.0; 3].into(), [0.0; 3].into()],
        2.4,
    );
    let background = <GradBg as crate::Background<Flat3>>::background(&gradient).unwrap();
    assert_eq!(background.colors(), [[1.0; 3], [0.0; 3]]);
    assert_eq!(background.axis(), [0.0, 1.0, 0.0]);
    assert_eq!(background.power(), 2.4);
}

fn embedded_lowering<const K: i8>()
where
    Embedded3<f64, K>: Geometry<Map = EmbeddedIsometry<f64, K>>,
{
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
    scene.medium = Medium {
        extinction: 0.05,
        albedo: [0.8; 3],
    };
    let lowered = scene.definition().unwrap();
    assert_eq!(
        lowered.view.map.components().unwrap(),
        transform::<G<K>, _>(&map).unwrap().components().unwrap()
    );
    assert!(lowered
        .modules
        .shapes
        .iter()
        .any(|module| module.same_implementation(&crate::shape::geodesic_sphere_schema())));
    assert_eq!(lowered.radius, scene.radius);
    assert!(matches!(
        lowered.medium,
        Medium {
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
}

#[test]
fn geodesic_sphere_rejects_invalid_physical_radius() {
    for radius in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(
            <GeodesicSphere as Shape<Spherical3>>::encode(&GeodesicSphere::new(radius)).is_err()
        );
    }
    let shape = <GeodesicSphere as Shape<Spherical3>>::encode(&GeodesicSphere::new(0.3)).unwrap();
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
    let value = <OuterMixture as Material<Flat3>>::encode(&mixture).unwrap();
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
    assert!(<OuterMixture as Material<Flat3>>::encode(&mixture)
        .unwrap()
        .schema
        .same_implementation(&value.schema));
    assert!(<OuterMixture as Material<Flat3>>::shader()
        .unwrap()
        .same_implementation(&value.schema));
}

#[test]
fn empty_object_vectors_preserve_statically_known_implementations() {
    type Diffuse = Covered<Flat3, Sphere, Lambertian>;
    type Emitting = Covered<Flat3, Cube, Emissive<Absorbing>>;
    let mut scene = SceneImpl::<Flat3, _, _, _, 3>::new(
        PointView::new(1.0),
        (Vec::<Diffuse>::new(), Vec::<Emitting>::new()),
        ConstBg::new([0.0; 3].into()),
    );
    let empty = scene.definition().unwrap();
    assert_eq!(empty.modules.shapes.len(), 2);
    assert_eq!(empty.modules.materials.len(), 2);
    let empty_source = compile(&empty).unwrap().source;
    scene.object.0.push(Covered::new(Sphere, Lambertian));
    let populated = scene.definition().unwrap();
    assert_eq!(empty_source, compile(&populated).unwrap().source);
    scene.object.0.clear();
    scene.object.1.push(Covered::new(
        Cube::new(1.0),
        Emissive::new(Absorbing, [1.0; 3].into()),
    ));
    assert_eq!(
        empty_source,
        compile(&scene.definition().unwrap()).unwrap().source
    );
}

#[test]
fn nested_camera_maps_keep_outer_inner_order_in_f64() {
    let shifted = Mapped::<Flat3, _, _>::new(
        PointView::new(0.7),
        Space3::<f64, 0>::unit()
            .translation([1.0, 0.0, 0.0].into(), 1.0)
            .unwrap(),
    );
    let rotated = Mapped::new(
        shifted,
        EmbeddedIsometry::<f64, 0>::rotation([0.0, 0.0, 1.0].into(), std::f64::consts::FRAC_PI_2)
            .unwrap(),
    );
    let view = rotated.view().unwrap();
    assert_eq!(view.fov, 0.7);
    let position = view.map.apply_vector([1.0, 0.0, 0.0, 0.0]);
    assert_eq!(position[0], 1.0);
    assert!(position[1].abs() < 1e-14);
    assert!((position[2] - 1.0).abs() < 1e-14);
    assert_eq!(position[3], 0.0);
}

#[test]
fn mapped_shapes_and_mapped_objects_keep_distinct_material_frames() {
    let offset = Space3::<f64, 0>::unit()
        .translation([1.0, 0.0, 0.0].into(), 2.0)
        .unwrap();
    let shape = Mapped::<Flat3, _, _>::new(Plane, offset);
    let mut covered_shape = Vec::new();
    Covered::new(shape, Lambertian)
        .encode_objects(Transform::identity(), &mut covered_shape)
        .unwrap();
    assert!(covered_shape[0]
        .shape
        .schema
        .key
        .contains("hypertrace.shape.mapped"));
    assert_eq!(
        covered_shape[0].map.components().unwrap(),
        Transform::<Flat3>::identity().components().unwrap()
    );
    let covered = Covered::<Flat3, _, _>::new(Plane, Lambertian);
    let mut mapped_object = Vec::new();
    Mapped::new(covered, offset)
        .encode_objects(Transform::identity(), &mut mapped_object)
        .unwrap();
    assert_eq!(mapped_object[0].shape.schema.key, "hypertrace.shape.plane");
    assert_eq!(
        mapped_object[0].map.components().unwrap(),
        Transform::<Flat3>::from_isometry(offset)
            .unwrap()
            .components()
            .unwrap()
    );
}

#[test]
fn nested_shape_leaves_cannot_omit_their_identity_word() -> Result<()> {
    let mut definition = SceneImpl::<Flat3, _, _, _, 1>::new(
        PointView::new(1.0),
        Covered::new(Plane, Absorbing),
        ConstBg::new([0.0; 3].into()),
    )
    .definition()?;
    for parameter_words in [Some(0), None] {
        let module = ShapeModule::new(
            "test.empty-shape",
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
        let tuple = crate::shape::tuple(vec![malformed])?;
        let mapped = crate::shape::mapped(tuple.clone(), Transform::identity())?;
        let vector = crate::shape::vector(tuple.schema.clone(), vec![tuple.clone()])?;
        for shape in [tuple, mapped, vector] {
            definition.objects[0].shape = shape;
            let error = compile(&definition).unwrap_err();
            assert!(format!("{error:#}").contains("must reserve an identity word"));
        }
    }
    Ok(())
}

#[derive(Clone)]
struct CustomShift(vecmat::Vector<f64, 3>);
impl ccgeom::Map<vecmat::Vector<f64, 4>> for CustomShift {
    fn identity() -> Self {
        Self([0.0; 3].into())
    }
    fn apply_pos(&self, pos: vecmat::Vector<f64, 4>) -> vecmat::Vector<f64, 4> {
        pos + vecmat::Vector::from([0.0, self.0[0], self.0[1], self.0[2]])
    }
    fn apply_dir(
        &self,
        _: vecmat::Vector<f64, 4>,
        dir: vecmat::Vector<f64, 4>,
    ) -> vecmat::Vector<f64, 4> {
        dir
    }
    fn apply_normal(
        &self,
        _: vecmat::Vector<f64, 4>,
        normal: vecmat::Vector<f64, 4>,
    ) -> vecmat::Vector<f64, 4> {
        normal
    }
    fn chain(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
    fn inv(self) -> Self {
        Self(-self.0)
    }
}
impl RenderMap<Flat3> for CustomShift {
    fn render_transform(&self) -> Result<Transform<Flat3>> {
        let map = if self.0.length() == 0.0 {
            EmbeddedIsometry::identity()
        } else {
            Space3::<f64, 0>::unit()
                .translation(self.0, self.0.length())
                .unwrap()
        };
        Transform::from_isometry(map)
    }
}
#[test]
fn downstream_map_adapter_lowers_without_type_registration() {
    let map = CustomShift([1.25, -2.0, 3.0].into());
    let view = Mapped::<Flat3, _, _>::new(PointView::new(1.0), map.clone())
        .view()
        .unwrap();
    let position = view.map.apply_vector([1.0, 0.0, 0.0, 0.0]);
    for (actual, expected) in position.iter().copied().zip([1.0, 1.25, -2.0, 3.0]) {
        assert!((actual - expected).abs() < 1e-14);
    }
    let scene = SceneImpl::<Flat3, _, _, _, 2>::new(
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
