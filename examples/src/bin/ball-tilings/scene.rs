//! Seven regular spherical tilings on equal-radius balls in Euclidean space.
//! Top row: tetrahedron, cube, octahedron. Bottom row: dodecahedron, icosahedron,
//! eight lunes, and a dihedron (two hemispheres). Move around to see every face.

use ccgeom::{EmbeddedIsometry, Flat3, Geometry3};
use objects::{
    Mapped, SceneImpl,
    background::GradBg,
    material::{Colored, Lambertian},
    object::{Tiled, tiling::RegularSpherical},
    shape::GeodesicSphere,
    view::PointView,
};

type Map = EmbeddedIsometry<f64, 0>;
// Match the main spherical balls; layout and camera distances scale with them.
const BALL_RADIUS: f64 = 0.25;
type Matte = Colored<Lambertian>;
type Ball<const P: usize, const Q: usize> =
    Mapped<Flat3, Tiled<GeodesicSphere, RegularSpherical<P, Q>, Matte, Matte, 4>, Map>;

// Different patterns remain concrete types, composed through an ordinary tuple.
pub type ExampleScene<const H: usize> = SceneImpl<
    Flat3,
    Mapped<Flat3, PointView<Flat3>, Map>,
    (
        Ball<3, 3>,
        Ball<4, 3>,
        Ball<3, 4>,
        Ball<5, 3>,
        Ball<3, 5>,
        Ball<2, 8>,
        Ball<6, 2>,
    ),
    GradBg,
    H,
>;

pub fn camera() -> Map {
    Flat3::shift_z(12.0 * BALL_RADIUS)
}

fn ball<const P: usize, const Q: usize>(position: [f64; 2], rotation: Map) -> Ball<P, Q> {
    let matte = |color: [f32; 3]| Colored::new(Lambertian, color.into());
    Mapped::new(
        Tiled::new(
            GeodesicSphere::new(BALL_RADIUS),
            // Border half-width is an angle on the sphere, in radians.
            RegularSpherical::<P, Q>::new(0.025),
            [
                [0.12, 0.48, 0.72],
                [0.80, 0.20, 0.10],
                [0.95, 0.64, 0.14],
                [0.58, 0.78, 0.68],
            ]
            .map(matte),
            matte([0.015, 0.020, 0.025]),
        ),
        // Map the complete object so the tiling rotates with its surface.
        Flat3::shift_x(position[0] * BALL_RADIUS)
            .chain(Flat3::shift_y(position[1] * BALL_RADIUS))
            .chain(rotation),
    )
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let rotation = Flat3::rotate_y(0.55).chain(Flat3::rotate_x(0.35));
    // Tilt the polar axis to reveal the lunes and both hemispheres of a dihedron.
    let polar_rotation = Flat3::rotate_y(0.25).chain(Flat3::rotate_x(1.25));
    let balls = (
        ball::<3, 3>([-2.8, 1.5], rotation),
        ball::<4, 3>([0.0, 1.5], rotation),
        ball::<3, 4>([2.8, 1.5], rotation),
        ball::<5, 3>([-4.2, -1.5], rotation),
        ball::<3, 5>([-1.4, -1.5], rotation),
        ball::<2, 8>([1.4, -1.5], polar_rotation),
        // Degree-two vertices divide the equator without adding face borders.
        ball::<6, 2>([4.2, -1.5], polar_rotation),
    );
    // Broad environment lighting gives every ball the same soft shading.
    let background = GradBg::new(
        [-0.48, 0.64, 0.60].into(),
        [[1.8, 1.7, 1.5].into(), [0.06, 0.08, 0.12].into()],
        2.0,
    );
    SceneImpl::new(
        Mapped::new(PointView::new(0.36), camera()),
        balls,
        background,
    )
}
