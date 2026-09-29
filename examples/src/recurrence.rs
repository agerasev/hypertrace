//! Light arriving from behind the camera after taking the long way around S³.
//!
//! The identity camera looks along `-z`. Every beacon lies behind it, toward
//! `+z`, yet appears in the forward view because spherical geodesics close after
//! `2πR` physical units. The central cyan beacon is one unit away with radius
//! `0.24`, so its first forward surface is `2π - 1 - 0.24` units away. Turning
//! around reveals the same beacon by the short route, only `1 - 0.24` units away.
//!
//! There is deliberately no floor: a great plane would interrupt the long
//! route. The absorbing emitters terminate surface paths without introducing
//! reflections. Passing `true` to [`scene`] adds homogeneous fog to the exact
//! same layout, demonstrating the effect of physical path length on scattering.
//! A sampled fog event can occur after several complete circuits along a ray
//! that misses every beacon; its distance must never be reduced modulo `2πR`.

use ccgeom::{EmbeddedIsometry, Space3, Spherical3};
use objects::{
    background::ConstBg,
    material::{Absorbing, Emissive},
    object::Covered,
    shader::Medium,
    shape::GeodesicSphere,
    view::PointView,
    Mapped, SceneImpl,
};

type Object = Mapped<
    Spherical3,
    Covered<Spherical3, GeodesicSphere, Emissive<Absorbing>>,
    EmbeddedIsometry<f64, 1>,
>;

pub type ExampleScene<const H: usize> = SceneImpl<
    Spherical3,
    Mapped<Spherical3, PointView<Spherical3>, EmbeddedIsometry<f64, 1>>,
    Vec<Object>,
    ConstBg,
    H,
>;

fn beacon(position: [f64; 3], radius: f64, emission: [f32; 3]) -> Object {
    let distance = position.iter().map(|v| v * v).sum::<f64>().sqrt();
    let map = Space3::<f64, 1>::unit()
        .translation(position.into(), distance)
        .expect("finite beacon position");
    Mapped::new(
        Covered::new(
            GeodesicSphere::new(radius),
            Emissive::new(Absorbing, emission.into()),
        ),
        map,
    )
}

/// A black, unit-radius spherical space containing four emissive beacons.
///
/// `fog` enables a scattering medium with mean free flight ten physical units,
/// longer than one complete circuit. The vacuum and fog versions share all
/// geometry, materials, and camera parameters. Twelve path events are a useful
/// starting point for the fog version; vacuum needs only one surface event.
pub fn scene<const H: usize>(fog: bool) -> ExampleScene<H> {
    let objects = vec![
        beacon([0.0, 0.0, 1.0], 0.24, [0.25, 1.15, 1.6]),
        beacon([-0.7, 0.35, 1.25], 0.15, [1.5, 0.45, 0.12]),
        beacon([0.65, 0.22, 1.35], 0.12, [0.75, 0.3, 1.4]),
        beacon([0.05, -0.65, 1.3], 0.14, [1.4, 1.05, 0.25]),
    ];
    let mut scene = SceneImpl::new(
        Mapped::new(PointView::new(1.0), EmbeddedIsometry::identity()),
        objects,
        ConstBg::new([0.0; 3].into()),
    );
    if fog {
        scene.medium = Medium::Homogeneous {
            extinction: 0.1,
            albedo: [0.9; 3],
        };
    }
    scene
}

#[cfg(test)]
mod tests {
    use super::*;
    use ccgeom::EmbeddedRay;
    use objects::{shader, Scene as _};

    #[test]
    fn behind_camera_beacon_has_a_forward_long_route() {
        let scene = scene::<12>(false);
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

        // Just before the far-side surface, the forward ray is still outside.
        let before = space.advance(forward, long_distance - 0.01);
        assert!(space.distance(before.position, center) > radius);
        let reverse = EmbeddedRay {
            tangent: -forward.tangent,
            ..forward
        };
        let short_hit = space.advance(reverse, 1.0 - radius);
        assert!((space.distance(short_hit.position, center) - radius).abs() < 1e-12);
    }

    #[test]
    fn vacuum_and_fog_share_only_emissive_spheres_and_a_black_background() {
        let vacuum = scene::<12>(false);
        let fog = scene::<12>(true);
        for scene in [&vacuum, &fog] {
            let definition = scene.definition().unwrap();
            assert_eq!(definition.view.map.geometry(), shader::Geometry::Spherical);
            assert_eq!(definition.radius, 1.0);
            assert_eq!(definition.bounces, 12);
            assert_eq!(scene.object.len(), 4);
            let compiled = shader::compile(&definition).unwrap();
            assert_eq!(compiled.objects.len(), 4);
            assert!(matches!(
                definition.background,
                shader::Background::Constant([0.0, 0.0, 0.0])
            ));
        }
        assert!(matches!(vacuum.medium, Medium::Vacuum));
        assert!(matches!(
            fog.medium,
            Medium::Homogeneous {
                extinction: 0.1,
                albedo: [0.9, 0.9, 0.9]
            }
        ));
        for (clear, hazy) in vacuum.object.iter().zip(&fog.object) {
            assert_eq!(clear.map.pair(), hazy.map.pair());
            assert_eq!(clear.inner.shape.radius, hazy.inner.shape.radius);
            assert_eq!(clear.inner.material.emission, hazy.inner.material.emission);
        }
    }
}
