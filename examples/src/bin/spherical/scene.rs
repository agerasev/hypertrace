//! A spherical landscape with asymmetric sunlight and a local glowing landmark.
//! Red and clear balls rest at opposite floor poles; a small blue ball sits near
//! the red one. Turn around to explore. The miss background is black.

use ccgeom::{EmbeddedIsometry, Geometry3, Space3, Spherical3};
use objects::{
    Mapped, Material, SceneImpl,
    background::ConstBg,
    material::{Absorbing, Colored, Emissive, Lambertian, Refractive, Specular, Transparent},
    mixture,
    object::{Covered, Tiled, tiling::RegularSpherical},
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
type TiledObject<S, P, M, const N: usize> = Mapped<Spherical3, Tiled<S, P, M, M, N>, Map>;
pub type ExampleScene<const H: usize> = SceneImpl<
    Spherical3,
    Mapped<Spherical3, PointView<Spherical3>, Map>,
    (
        Object<GeodesicSphere, Colored<Lambertian>>,
        Object<GeodesicSphere, Refractive>,
        Object<GeodesicSphere, Emissive<Absorbing>>,
        TiledObject<Plane, RegularSpherical<5, 3>, Floor, 3>,
        Object<GeodesicSphere, Emissive<Absorbing>>,
        Object<GeodesicSphere, Colored<Refractive>>,
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
            GeodesicSphere::new(0.2),
            Emissive::new(Absorbing, [5.5, 4.95, 4.0].into()),
        ),
        floor
            .chain(
                Space3::<f64, 1>::unit()
                    .translation([-0.45, -0.25, 0.0].into(), 1.65)
                    .expect("finite sun footprint"),
            )
            .chain(Spherical3::shift_z(-0.55)),
    );
    let floor_material = |color: [f32; 3]| {
        Floor::new(
            (Colored::new(Lambertian, color.into()), 0.9).into(),
            (Specular, 0.05).into(),
            (Transparent, 0.05).into(),
        )
    };
    let ground = Mapped::new(
        Tiled::new(
            Plane,
            RegularSpherical::<5, 3>::new(0.012),
            [[0.65, 0.68, 0.72], [0.5, 0.57, 0.65], [0.72, 0.65, 0.55]].map(floor_material),
            floor_material([0.2, 0.23, 0.26]),
        ),
        floor,
    );
    // The weakest direct sunlight is a quarter circuit from the sun's floor
    // footprint; its antipode is bright again because spherical rays converge.
    // Place this opaque emitter in that dark band, partly below the floor.
    let beacon = Mapped::new(
        Covered::new(
            GeodesicSphere::new(0.16),
            Emissive::new(Absorbing, [0.25, 0.9, 0.5].into()),
        ),
        floor
            .chain(
                Space3::<f64, 1>::unit()
                    .translation(
                        [-0.45, -0.25, 0.0].into(),
                        1.65 + std::f64::consts::FRAC_PI_2,
                    )
                    .expect("finite dark floor position"),
            )
            .chain(Spherical3::shift_z(-0.04)),
    );
    let blue = resting_sphere(
        floor,
        [0.65, -1.5],
        0.11,
        Colored::new(Refractive::new(1.08), [0.4, 0.65, 1.0].into()),
    );
    SceneImpl::new(
        Mapped::new(PointView::new(1.5), camera()),
        (diffuse, glass, sun, ground, beacon, blue),
        ConstBg::new([0.0; 3].into()),
    )
}
