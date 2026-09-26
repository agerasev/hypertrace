use ccgeom::{Geometry3, Hyperbolic3};
use objects::{
    background::ConstBg,
    material::*,
    mixture,
    object::{tiling, TiledHorosphere, TiledPlane},
    object_choice,
    view::PointView,
    Mapped, SceneImpl,
};
use std::f64::consts::PI;
use vecmat::{transform::Moebius, Complex, Matrix, Vector};

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

object_choice! {
    Choice {
        PlaneStar(TiledPlane<MyMaterial, tiling::Pentastar, 2>),
        PlanePenta(TiledPlane<MyMaterial, tiling::Pentagonal, 2>),
        HoroHexa(TiledHorosphere<MyMaterial, tiling::Hexagonal, 3>),
        HoroSquare(TiledHorosphere<MyMaterial, tiling::Square, 4>),
    }
}

pub type ExampleScene<const H: usize> = SceneImpl<
    Hyperbolic3,
    Mapped<Hyperbolic3, PointView<Hyperbolic3>, Moebius<Complex<f64>>>,
    Vec<Mapped<Hyperbolic3, Choice, Moebius<Complex<f64>>>>,
    ConstBg,
    H,
>;

pub fn camera() -> Moebius<Complex<f64>> {
    Hyperbolic3::rotate_z(5.0 * PI / 6.0)
        .chain(Hyperbolic3::rotate_x(2.0 * PI / 5.0))
        .chain(Hyperbolic3::shift_z(2.0))
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let view = Mapped::new(PointView::new(1.0), camera());

    let background = ConstBg::new(unpack_color(0xeeeeee));
    let border_material = make_material(
        unpack_color(0xe4e4e4),
        0.0,
        0.0,
        Some(make_color(Vector::fill(1.0))),
    );
    let objects = vec![
        Mapped::new(
            Choice::HoroHexa(TiledHorosphere::new(
                [
                    make_material(unpack_color(0xfe0000), 0.1, 0.1, None),
                    make_material(unpack_color(0xffaa01), 0.1, 0.1, None),
                    make_material(unpack_color(0x35adae), 0.1, 0.1, None),
                ],
                0.5,
                0.02,
                border_material.clone(),
            )),
            Moebius::identity(),
        ),
        Mapped::new(
            Choice::HoroSquare(TiledHorosphere::new(
                [
                    make_material(unpack_color(0xfe7401), 0.1, 0.1, None),
                    make_material(unpack_color(0xfe0000), 0.1, 0.1, None),
                    make_material(unpack_color(0xffaa01), 0.1, 0.1, None),
                    make_material(unpack_color(0xfed601), 0.1, 0.1, None),
                ],
                0.5,
                0.02,
                border_material.clone(),
            )),
            Moebius::from(Matrix::from([
                [Complex::new(1.0, 0.0), Complex::new(2.0f64.sqrt(), 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ]))
            .chain(Hyperbolic3::rotate_x(PI)),
        ),
        Mapped::new(
            Choice::PlaneStar(TiledPlane::new(
                [
                    make_material(unpack_color(0xfe7401), 0.1, 0.0, None),
                    make_material(unpack_color(0x35adae), 0.1, 0.0, None),
                ],
                f64::NAN,
                0.01,
                border_material.clone(),
            )),
            Moebius::identity(),
        ),
        Mapped::new(
            Choice::PlanePenta(TiledPlane::new(
                [
                    make_material(unpack_color(0xfe0000), 0.1, 0.0, None),
                    make_material(unpack_color(0xfed601), 0.1, 0.0, None),
                ],
                f64::NAN,
                0.02,
                border_material,
            )),
            Moebius::from(Matrix::from([
                [Complex::new(1.0, 0.0), Complex::new(0.0, 2.0)],
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ])),
        ),
    ];

    SceneImpl::<Hyperbolic3, _, _, _, H>::new(view, objects, background)
}
