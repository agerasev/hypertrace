use ccgeom::{EmbeddedIsometry, Geometry3, Hyperboloid3};
use objects::{
    Mapped, SceneImpl,
    background::ConstBg,
    material::*,
    mixture,
    object::{TiledHorosphere, TiledPlane, tiling},
    object_choice,
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

object_choice! {
    Choice {
        PlaneStar(TiledPlane<MyMaterial, tiling::Pentastar, 2>),
        PlanePenta(TiledPlane<MyMaterial, tiling::Pentagonal, 2>),
        HoroHexa(TiledHorosphere<MyMaterial, tiling::Hexagonal, 3>),
        HoroSquare(TiledHorosphere<MyMaterial, tiling::Square, 4>),
    }
}

pub type ExampleScene<const H: usize> = SceneImpl<
    Hyperboloid3,
    Mapped<Hyperboloid3, PointView<Hyperboloid3>, EmbeddedIsometry<f64, -1>>,
    Vec<Mapped<Hyperboloid3, Choice, EmbeddedIsometry<f64, -1>>>,
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
            EmbeddedIsometry::identity(),
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
            horosphere_translation([2.0f64.sqrt(), 0.0]).chain(Hyperboloid3::rotate_x(PI)),
        ),
        Mapped::new(
            Choice::PlaneStar(TiledPlane::new(
                [
                    make_material(unpack_color(0xfe7401), 0.1, 0.0, None),
                    make_material(unpack_color(0x35adae), 0.1, 0.0, None),
                ],
                0.01,
                border_material.clone(),
            )),
            EmbeddedIsometry::identity(),
        ),
        Mapped::new(
            Choice::PlanePenta(TiledPlane::new(
                [
                    make_material(unpack_color(0xfe0000), 0.1, 0.0, None),
                    make_material(unpack_color(0xfed601), 0.1, 0.0, None),
                ],
                0.02,
                border_material,
            )),
            horosphere_translation([0.0, 2.0]),
        ),
    ];

    SceneImpl::<Hyperboloid3, _, _, _, H>::new(view, objects, background)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ccgeom::Space3;

    #[test]
    fn horosphere_placements_preserve_points_and_material_directions() {
        let space = Space3::<f64, -1>::unit();
        for offset in [[2.0f64.sqrt(), 0.0], [0.0, 2.0]] {
            let map = horosphere_translation(offset);
            for point in [[0.0, 0.0, 1.0], [0.2, -0.4, 0.7], [-0.3, 0.8, 2.0]] {
                let embedded = space.point_from_half_space(point.into()).unwrap();
                let mapped = map.apply_vector(embedded);
                let expected = [point[0] + offset[0], point[1] + offset[1], point[2]];
                for (actual, expected) in
                    space.point_to_half_space(mapped).into_iter().zip(expected)
                {
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

    #[test]
    fn camera_position_and_forward_follow_the_declared_rotations() {
        let map = camera();
        let (sine_x, cosine_x) = (2.0 * PI / 5.0).sin_cos();
        let (sine_z, cosine_z) = (5.0 * PI / 6.0).sin_cos();
        let (sinh, cosh) = (2.0f64.sinh(), 2.0f64.cosh());
        for (basis, expected) in [
            (
                [1.0, 0.0, 0.0, 0.0],
                [
                    cosh,
                    sine_z * sine_x * sinh,
                    -cosine_z * sine_x * sinh,
                    cosine_x * sinh,
                ],
            ),
            (
                [0.0, 0.0, 0.0, -1.0],
                [
                    -sinh,
                    -sine_z * sine_x * cosh,
                    cosine_z * sine_x * cosh,
                    -cosine_x * cosh,
                ],
            ),
        ] {
            for (actual, expected) in map.apply_vector(basis.into()).into_iter().zip(expected) {
                assert!((actual - expected).abs() < 1e-12);
            }
        }
    }
}
