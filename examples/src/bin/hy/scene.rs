use ccgeom::{EmbeddedIsometry, Geometry3, Hyperboloid3};
use objects::{
    Mapped, SceneImpl,
    background::ConstBg,
    material::*,
    mixture,
    object::{Tiled, tiling},
    shape::{GeodesicSphere, Horosphere, Plane},
    view::PointView,
};
use std::f64::consts::PI;
use vecmat::{QuaternionPair, Vector};

fn unpack_color(rgb: u32) -> Vector<f32, 3> {
    make_color(
        (Vector::from([rgb >> 16, rgb >> 8, rgb]) & Vector::fill(0xff)).map(|x| (x as f32) / 255.0),
    )
}

fn make_color(rgb: Vector<f32, 3>) -> Vector<f32, 3> {
    rgb.powf(2.2)
}

mixture! {
    Mixture {
        diffuse: Colored<Lambertian>,
        specular: Specular,
        transparent: Transparent,
    }
}

fn make_material(
    diffuse: Vector<f32, 3>,
    specularity: f64,
    transparency: f64,
    emission: Option<Vector<f32, 3>>,
) -> Emissive<Mixture> {
    Emissive::new(
        Mixture::new(
            (
                Colored::new(Lambertian, diffuse),
                1.0 - specularity - transparency,
            )
                .into(),
            (Specular, specularity).into(),
            (Transparent, transparency).into(),
        ),
        emission.unwrap_or_else(|| [0.0, 0.0, 0.0].into()),
    )
}

type MyMaterial = Emissive<Mixture>;

type Object<T> = Mapped<Hyperboloid3, T, EmbeddedIsometry<f64, -1>>;
type Objects = (
    Vec<Object<Tiled<Horosphere, tiling::Hexagonal, MyMaterial, MyMaterial, 3>>>,
    Vec<Object<Tiled<Horosphere, tiling::Square, MyMaterial, MyMaterial, 4>>>,
    Vec<Object<Tiled<Plane, tiling::Pentastar, MyMaterial, MyMaterial, 2>>>,
    Vec<Object<Tiled<Plane, tiling::Pentagonal, MyMaterial, MyMaterial, 2>>>,
    Vec<Object<Tiled<GeodesicSphere, tiling::RegularSpherical<3, 5>, MyMaterial, MyMaterial, 3>>>,
);

pub type ExampleScene<const H: usize> = SceneImpl<
    Hyperboloid3,
    Mapped<Hyperboloid3, PointView<Hyperboloid3>, EmbeddedIsometry<f64, -1>>,
    Objects,
    ConstBg,
    H,
>;

pub fn camera() -> EmbeddedIsometry<f64, -1> {
    Hyperboloid3::rotate_z(5.0 * PI / 6.0)
        .chain(Hyperboloid3::rotate_x(2.0 * PI / 5.0))
        .chain(Hyperboloid3::shift_z(2.0))
}

/// A parabolic isometry that translates a horosphere's half-space coordinates.
/// Unlike an origin-centered geodesic boost, it preserves the horosphere's
/// height and the chart-aligned tangent frame used by its material tiling.
fn horosphere_translation([x, y]: [f64; 2]) -> EmbeddedIsometry<f64, -1> {
    EmbeddedIsometry::from_pair(QuaternionPair::from_array([
        1.0,
        y / 2.0,
        -x / 2.0,
        0.0,
        0.0,
        x / 2.0,
        y / 2.0,
        0.0,
    ]))
    .expect("finite horosphere translation")
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let view = Mapped::new(PointView::new(1.0), camera());

    // In half-space coordinates the horospheres are z=1 and a radius-1/2
    // sphere centered at (sqrt(2), 0, 1/2); the star plane is the unit hemisphere.
    // A hyperbolic ball of radius r tangent below z=1 has chart center height
    // exp(-r), Euclidean radius s=(1-exp(-2r))/2 and Euclidean center height 1-s.
    // The other two tangencies give x=(1+s)/sqrt(2) and x*x+y*y=4s.
    // Choose negative y to put the ball behind the horospheres from this camera.
    let ball_radius = 0.36_f64;
    let chart_radius = -(-2.0 * ball_radius).exp_m1() / 2.0;
    let ball_x = (1.0 + chart_radius) / 2.0_f64.sqrt();
    let ball_y = -(4.0 * chart_radius - ball_x * ball_x).sqrt();
    let ball_map =
        horosphere_translation([ball_x, ball_y]).chain(Hyperboloid3::shift_z(-ball_radius));

    let background = ConstBg::new(unpack_color(0xeeeeee));
    let border_material = make_material(
        unpack_color(0xe4e4e4),
        0.0,
        0.0,
        Some(make_color(Vector::fill(1.0))),
    );
    let objects = (
        vec![Mapped::new(
            Tiled::new(
                Horosphere,
                tiling::Hexagonal::new(0.5, 0.02),
                [
                    make_material(unpack_color(0xfe0000), 0.1, 0.1, None),
                    make_material(unpack_color(0xffaa01), 0.1, 0.1, None),
                    make_material(unpack_color(0x35adae), 0.1, 0.1, None),
                ],
                border_material.clone(),
            ),
            EmbeddedIsometry::identity(),
        )],
        vec![Mapped::new(
            Tiled::new(
                Horosphere,
                tiling::Square::new(0.5, 0.02),
                [
                    make_material(unpack_color(0xfe7401), 0.1, 0.1, None),
                    make_material(unpack_color(0xfe0000), 0.1, 0.1, None),
                    make_material(unpack_color(0xffaa01), 0.1, 0.1, None),
                    make_material(unpack_color(0xfed601), 0.1, 0.1, None),
                ],
                border_material.clone(),
            ),
            horosphere_translation([2.0f64.sqrt(), 0.0]).chain(Hyperboloid3::rotate_x(PI)),
        )],
        vec![Mapped::new(
            Tiled::new(
                Plane,
                tiling::Pentastar::new(0.01),
                [
                    make_material(unpack_color(0xfe7401), 0.1, 0.0, None),
                    make_material(unpack_color(0x35adae), 0.1, 0.0, None),
                ],
                border_material.clone(),
            ),
            EmbeddedIsometry::identity(),
        )],
        vec![Mapped::new(
            Tiled::new(
                Plane,
                tiling::Pentagonal::new(0.02),
                [
                    make_material(unpack_color(0xfe0000), 0.1, 0.0, None),
                    make_material(unpack_color(0xfed601), 0.1, 0.0, None),
                ],
                border_material.clone(),
            ),
            horosphere_translation([0.0, 2.0]),
        )],
        vec![Mapped::new(
            Tiled::new(
                GeodesicSphere::new(ball_radius),
                tiling::RegularSpherical::<3, 5>::new(0.025),
                [0xfe7401, 0x35adae, 0xfed601]
                    .map(|color| make_material(unpack_color(color), 0.1, 0.0, None)),
                border_material,
            ),
            ball_map,
        )],
    );

    SceneImpl::<Hyperboloid3, _, _, _, H>::new(view, objects, background)
}
