//! Physical invariants shared by independently owned example sources.
use ccgeom::{Embedded3, EmbeddedIsometry, EmbeddedRay, Space3};
use hypertrace_gallery::{
    compare_eu, compare_hy, compare_hy_flat, compare_sp, compare_sp_flat, sp_loop, sp_loop_fog,
};
use objects::{
    Scene as _,
    shader::{self, Geometry, Medium, Result, SceneDefinition},
};
use vecmat::Vector;

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
        // independently owned builder, rather than accidentally using chart units.
        let expected_direction =
            Vector::from([[-0.65, 0.0, 0.65][i % 3], [0.45, 0.0, -0.45][i / 3], -1.0]).normalize();
        let direction = Vector::from([position[1], position[2], position[3]]).normalize();
        assert!((direction - expected_direction).length() < 1e-12);
    }
    compiled.words
}

#[test]
fn comparison_preserves_physical_layout_and_payloads_across_curvatures() -> Result<()> {
    let euclidean = check_physical_layout::<0>(compare_eu::scene()?.definition()?, 1.0);
    assert_eq!(
        check_physical_layout::<-1>(compare_hy::scene()?.definition()?, 1.0),
        euclidean
    );
    assert_eq!(
        check_physical_layout::<1>(compare_sp::scene()?.definition()?, 1.0),
        euclidean
    );
    assert_eq!(
        check_physical_layout::<-1>(compare_hy_flat::scene()?.definition()?, 3.0),
        euclidean
    );
    assert_eq!(
        check_physical_layout::<1>(compare_sp_flat::scene()?.definition()?, 3.0),
        euclidean
    );
    Ok(())
}

#[test]
fn behind_camera_beacon_has_a_forward_long_route() -> Result<()> {
    let scene = sp_loop::scene()?;
    let space = Space3::<f64, 1>::unit();
    let beacon = &scene.object[0];
    let center = beacon.map.apply_vector(space.origin());
    let radius = beacon.inner.shape.radius;
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
fn vacuum_and_fog_share_only_emissive_spheres_and_a_black_background() -> Result<()> {
    let vacuum = sp_loop::scene()?;
    let fog = sp_loop_fog::scene()?;
    let clear = vacuum.definition()?;
    let hazy = fog.definition()?;
    assert_eq!(clear.bounces, 1);
    assert_eq!(hazy.bounces, 12);
    for definition in [&clear, &hazy] {
        assert_eq!(definition.radius, 1.0);
        assert_eq!(definition.objects.len(), 4);
        assert_eq!(shader::compile(definition)?.objects.len(), 4);
        assert_eq!(definition.background.colors(), [[0.0; 3]; 2]);
    }
    assert_eq!(vacuum.medium, Medium::vacuum());
    assert_eq!(fog.medium, Medium::homogeneous(0.1, [0.9; 3]));
    for (clear, hazy) in vacuum.object.iter().zip(&fog.object) {
        assert_eq!(clear.map.pair(), hazy.map.pair());
        assert_eq!(clear.inner.shape.radius, hazy.inner.shape.radius);
        assert_eq!(clear.inner.material.emission, hazy.inner.material.emission);
    }
    Ok(())
}
