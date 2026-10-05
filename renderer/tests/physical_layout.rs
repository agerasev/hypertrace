//! Independent physical-layout and long-route fixtures, without example dependencies.
use ccgeom::{Embedded3, EmbeddedIsometry, EmbeddedRay, Space3};
use objects::{
    material::{self, MaterialValueExt as _},
    shader::{
        self, Background, EncodedObject, Geometry, Medium, Modules, Result, SceneDefinition,
        Transform, View,
    },
    shape,
};

fn layout<const K: i8>(radius: f64) -> Result<SceneDefinition<Embedded3<f64, K>>>
where
    Embedded3<f64, K>: Geometry<Map = EmbeddedIsometry<f64, K>>,
{
    let space = Space3::<f64, K>::new(radius).unwrap();
    let mut objects = Vec::new();
    for (distance, y) in [(0.8, 0.45), (1.6, 0.0), (2.5, -0.45)] {
        for x in [-0.65, 0.0, 0.65] {
            objects.push(EncodedObject {
                sampling: None,
                map: Transform::from_isometry(
                    space.translation([x, y, -1.0].into(), distance).unwrap(),
                )?,
                shape: shape::geodesic_sphere(0.12)?,
                material: material::absorbing().emissive([1.0; 3])?,
            });
        }
    }
    Ok(SceneDefinition {
        view: View {
            map: Transform::identity(),
            fov: 1.0,
        },
        background: Background::constant([0.0; 3]),
        bounces: 1,
        radius,
        medium: Medium::vacuum(),
        objects,
        modules: Modules::default(),
    })
}

fn check_physical_layout<const K: i8>(
    definition: SceneDefinition<Embedded3<f64, K>>,
    radius: f64,
) -> Vec<u32>
where
    Embedded3<f64, K>: Geometry<Map = EmbeddedIsometry<f64, K>>,
{
    let compiled = shader::compile(&definition).unwrap();
    assert_eq!(compiled.transforms.len(), 9);
    assert_eq!(definition.bounces, 1);
    assert_eq!(definition.medium, Medium::vacuum());
    assert_eq!(definition.radius, radius);
    assert_eq!(definition.background.colors(), [[0.0; 3]; 2]);

    let space = Space3::<f64, K>::new(radius).unwrap();
    for (i, map) in compiled.transforms.iter().enumerate() {
        let position = map.apply_vector([1.0, 0.0, 0.0, 0.0]).into();
        let distance = space.distance(space.origin(), position);
        let expected = [0.8, 1.6, 2.5][i / 3];
        assert!((distance - expected).abs() < 1e-12);
        assert_eq!(definition.objects[i].shape.words, [0.12f32.to_bits()]);
        // The same camera directions and physical distances must survive each
        // geometry, rather than accidentally using chart units.
        let expected = [
            [-0.65, 0.0, 0.65][i % 3],
            [0.45, 0.0, -0.45][i / 3],
            -1.0_f64,
        ];
        let expected_norm = expected.iter().map(|x| x * x).sum::<f64>().sqrt();
        let actual_norm = [position[1], position[2], position[3]]
            .iter()
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt();
        for axis in 0..3 {
            assert!(
                (position[axis + 1] / actual_norm - expected[axis] / expected_norm).abs() < 1e-12
            );
        }
    }
    compiled.words
}

#[test]
fn compilation_preserves_physical_layout_and_payloads_across_curvatures() -> Result<()> {
    let euclidean = check_physical_layout::<0>(layout::<0>(1.0)?, 1.0);
    assert_eq!(
        check_physical_layout::<-1>(layout::<-1>(1.0)?, 1.0),
        euclidean
    );
    assert_eq!(
        check_physical_layout::<1>(layout::<1>(1.0)?, 1.0),
        euclidean
    );
    assert_eq!(
        check_physical_layout::<-1>(layout::<-1>(3.0)?, 3.0),
        euclidean
    );
    assert_eq!(
        check_physical_layout::<1>(layout::<1>(3.0)?, 3.0),
        euclidean
    );
    Ok(())
}

#[test]
fn behind_camera_beacon_has_a_forward_long_route() -> Result<()> {
    let space = Space3::<f64, 1>::unit();
    let center = space
        .translation([0.0, 0.0, 1.0].into(), 1.0)
        .unwrap()
        .apply_vector(space.origin());
    let radius = 0.24;
    assert!(center[3] > 0.0);
    assert!((space.distance(space.origin(), center) - 1.0).abs() < 1e-12);

    let forward = EmbeddedRay {
        position: space.origin(),
        tangent: [0.0, 0.0, 0.0, -1.0].into(),
    };
    let long_distance = std::f64::consts::TAU - 1.0 - radius;
    let hit = space.advance(forward, long_distance);
    assert!(long_distance > std::f64::consts::PI);
    assert!((space.distance(hit.position, center) - radius).abs() < 1e-12);

    // Just before the far-side surface, the forward ray remains outside.
    let before = space.advance(forward, long_distance - 0.01);
    assert!(space.distance(before.position, center) > radius);
    let reverse = EmbeddedRay {
        tangent: -forward.tangent,
        ..forward
    };
    let short_hit = space.advance(reverse, 1.0 - radius);
    assert!((space.distance(short_hit.position, center) - radius).abs() < 1e-12);
    Ok(())
}

#[test]
fn parabolic_horosphere_maps_preserve_height_and_material_directions() {
    let space = Space3::<f64, -1>::unit();
    for [x, y] in [[2.0f64.sqrt(), 0.0], [0.0, 2.0], [-0.7, 0.4]] {
        // A parabolic translation, rather than a geodesic displacement of the
        // same length: the entire horosphere keeps its half-space height.
        let map = <Embedded3<f64, -1> as Geometry>::map_from_components([
            1.0,
            y / 2.0,
            -x / 2.0,
            0.0,
            0.0,
            x / 2.0,
            y / 2.0,
            0.0,
        ])
        .unwrap();
        for point in [[0.0, 0.0, 1.0], [0.2, -0.4, 0.7], [-0.3, 0.8, 2.0]] {
            let embedded = space.point_from_half_space(point.into()).unwrap();
            let mapped = map.apply_vector(embedded);
            for (actual, expected) in space.point_to_half_space(mapped).into_iter().zip([
                point[0] + x,
                point[1] + y,
                point[2],
            ]) {
                assert!((actual - expected).abs() < 1e-12);
            }
            for direction in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] {
                let tangent = space
                    .tangent_from_half_space(point.into(), direction.into())
                    .unwrap();
                let actual = space.tangent_to_half_space(mapped, map.apply_vector(tangent));
                for (actual, expected) in actual.into_iter().zip(direction) {
                    assert!((actual - expected).abs() < 1e-12);
                }
            }
        }
    }
}
