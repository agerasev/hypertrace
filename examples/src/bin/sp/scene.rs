//! A sun and two spheres resting on a great-sphere plane in unit-radius S³.
//! The balls touch opposite poles of the floor, with the sun between their
//! centers. Turn around to see the other ball. The miss background is black.

use ccgeom::{EmbeddedIsometry, Geometry3, Space3, Spherical3};
use objects::{
    Mapped, Material, SceneImpl,
    background::ConstBg,
    material::{Absorbing, Colored, Emissive, Lambertian, Refractive, Specular, Transparent},
    mixture,
    object::Covered,
    shape::{GeodesicSphere, Plane},
    view::PointView,
};

mixture! {
    Floor {
        diffuse: Colored<Lambertian>,
        reflective: Specular,
        transparent: Transparent,
    }
}

type Map = EmbeddedIsometry<f64, 1>;
type Object<S, M> = Mapped<Spherical3, Covered<Spherical3, S, M>, Map>;
pub type ExampleScene<const H: usize> = SceneImpl<
    Spherical3,
    Mapped<Spherical3, PointView<Spherical3>, Map>,
    (
        Object<GeodesicSphere, Colored<Lambertian>>,
        Object<GeodesicSphere, Refractive>,
        Object<GeodesicSphere, Emissive<Absorbing>>,
        Object<Plane, Floor>,
    ),
    ConstBg,
    H,
>;

pub fn camera() -> Map {
    Spherical3::rotate_x(0.85)
}

/// Move along the plane first, then lift its sphere center along the normal by
/// exactly one sphere radius. In spherical space a world-y offset is not enough
/// to keep a ball tangent to the floor at different positions.
fn resting_sphere<M: Material<Spherical3>>(
    floor: Map,
    position: [f64; 2],
    radius: f64,
    material: M,
) -> Object<GeodesicSphere, M> {
    let foot = Space3::<f64, 1>::unit()
        .translation(
            [position[0], position[1], 0.0].into(),
            position[0].hypot(position[1]),
        )
        .expect("finite floor position");
    // The floor's local -z normal points into the camera's upper hemisphere.
    let center = floor.chain(foot).chain(Spherical3::shift_z(-radius));
    Mapped::new(Covered::new(GeodesicSphere::new(radius), material), center)
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let floor = Spherical3::shift_y(-0.55).chain(Spherical3::rotate_x(std::f64::consts::FRAC_PI_2));
    let diffuse = resting_sphere(
        floor,
        [0.0, -std::f64::consts::FRAC_PI_2],
        0.25,
        Colored::new(Lambertian, [0.8, 0.18, 0.08].into()),
    );
    let glass = resting_sphere(
        floor,
        [0.0, std::f64::consts::FRAC_PI_2],
        0.25,
        Refractive::new(1.4),
    );
    let sun = Mapped::new(
        Covered::new(
            GeodesicSphere::new(0.05),
            Emissive::new(Absorbing, [5.5, 4.95, 4.0].into()),
        ),
        // The upper pole of the floor is halfway along the shortest geodesic
        // joining these equal-radius balls above antipodal contact points.
        floor.chain(Spherical3::shift_z(-std::f64::consts::FRAC_PI_2)),
    );
    let ground = Mapped::new(
        Covered::new(
            Plane,
            Floor::new(
                (Colored::new(Lambertian, [0.65, 0.68, 0.72].into()), 0.85).into(),
                (Specular, 0.05).into(),
                (Transparent, 0.10).into(),
            ),
        ),
        floor,
    );
    SceneImpl::new(
        Mapped::new(PointView::new(1.5), camera()),
        (diffuse, glass, sun, ground),
        ConstBg::new([0.0; 3].into()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use objects::{Scene as _, shader};

    #[test]
    fn spherical_studio_has_two_balls_a_sun_and_a_plane() {
        let definition = scene::<6>().definition().unwrap();
        assert_eq!(definition.background.colors(), [[0.0; 3]; 2]);
        assert_eq!(definition.medium, shader::Medium::vacuum());
        assert_eq!(definition.radius, 1.0);
        assert_eq!(definition.bounces, 6);
        assert_eq!(shader::compile(&definition).unwrap().objects.len(), 4);
    }

    #[test]
    fn both_balls_touch_the_top_of_the_spherical_plane() {
        let scene = scene::<6>();
        let space = Space3::<f64, 1>::unit();
        for (map, radius) in [
            (scene.object.0.map, scene.object.0.inner.shape.radius),
            (scene.object.1.map, scene.object.1.inner.shape.radius),
        ] {
            let center = scene
                .object
                .3
                .map
                .inv()
                .apply_vector(map.apply_vector(space.origin()));
            // Signed geodesic distance to the local z=0 great sphere.
            assert!(((-center[3]).asin() - radius).abs() < 1e-12);
            let foot = vecmat::Vector::from([center[0], center[1], center[2], 0.0]).normalize();
            assert!((space.distance(center, foot) - radius).abs() < 1e-12);
        }
    }

    #[test]
    fn balls_rest_at_opposite_poles_with_the_sun_halfway_between() {
        let scene = scene::<6>();
        let space = Space3::<f64, 1>::unit();
        let diffuse = scene.object.0.map.apply_vector(space.origin());
        let glass = scene.object.1.map.apply_vector(space.origin());
        let sun = scene.object.2.map.apply_vector(space.origin());
        let separation = std::f64::consts::PI - 2.0 * scene.object.0.inner.shape.radius;
        assert!((space.distance(diffuse, glass) - separation).abs() < 1e-12);
        assert!((space.distance(diffuse, sun) - separation / 2.0).abs() < 1e-12);
        assert!((space.distance(glass, sun) - separation / 2.0).abs() < 1e-12);
    }
}
