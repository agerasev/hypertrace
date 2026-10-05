use super::*;
use crate::{
    background::ConstBg,
    material::{Absorbing, Emissive},
    object::{Covered, Sampled},
    shader::{compile, Transform},
    shape::GeodesicSphere,
    view::PointView,
    Mapped, Object, Scene as _, SceneImpl,
};
use ccgeom::{Embedded3, EmbeddedIsometry, Space3};

fn check<const K: i8>()
where
    Embedded3<f64, K>: Geometry<Map = EmbeddedIsometry<f64, K>>,
{
    type G<const K: i8> = Embedded3<f64, K>;
    let map = Space3::<f64, K>::unit()
        .translation([0.0, 0.0, 1.0].into(), 0.7)
        .unwrap();
    let object = Mapped::<G<K>, _, _>::new(
        Sampled::new(
            Covered::new(
                GeodesicSphere::new(0.2),
                Emissive::new(Absorbing, [1.0; 3].into()),
            ),
            SphereBound::new(0.2),
        ),
        map,
    );
    let mut builder = SceneImpl::<G<K>, _, _, _, 3>::new(
        PointView::new(1.0),
        vec![object],
        ConstBg::new([0.0; 3].into()),
    );
    let full = compile(&builder.definition().unwrap()).unwrap();
    assert_eq!(full.light_count, 1);
    assert_eq!(
        full.transforms[0].components().unwrap(),
        full.sampling_transforms[0].components().unwrap()
    );
    builder.object[0].inner.sampler.radius = 0.3;
    assert_eq!(
        full.source,
        compile(&builder.definition().unwrap()).unwrap().source
    );
    builder.object.clear();
    let empty = compile(&builder.definition().unwrap()).unwrap();
    assert_eq!(
        full.source, empty.source,
        "empty vectors retain light sampler dependencies"
    );
    assert_eq!(empty.light_count, 0);
    let nested = Sampled::new(
        Sampled::new(
            Covered::<G<K>, _, _>::new(GeodesicSphere::new(0.2), Absorbing),
            SphereBound::new(0.2),
        ),
        SphereBound::new(0.3),
    );
    let mut records = vec![];
    assert!(nested
        .encode_objects(Transform::identity(), &mut records)
        .is_err());
    assert!(
        records.is_empty(),
        "failed lowering must not append partial objects"
    );
}

#[test]
fn sampled_objects_keep_frames_static_dependencies_and_validate_nesting() {
    check::<-1>();
    check::<0>();
    check::<1>();
}
