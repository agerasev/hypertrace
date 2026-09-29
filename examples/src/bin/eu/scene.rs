use ccgeom::{EmbeddedIsometry, Flat3, Geometry3};
use objects::{
    Mapped, SceneImpl,
    background::GradBg,
    material::{Colored, Lambertian, Refractive, Specular},
    mixture,
    object::Covered,
    shape::{Cube, Plane, Sphere},
    view::PointView,
};

mixture! {
    Mixture {
        diffuse: Colored<Lambertian>,
        specular: Specular,
        refractive: Colored<Refractive>,
    }
}

pub type Object<S> = Mapped<Flat3, Covered<Flat3, S, Mixture>, EmbeddedIsometry<f64, 0>>;

pub type ExampleScene<const H: usize> = SceneImpl<
    Flat3,
    Mapped<Flat3, PointView<Flat3>, EmbeddedIsometry<f64, 0>>,
    (Vec<Object<Sphere>>, Vec<Object<Cube>>, Vec<Object<Plane>>),
    GradBg,
    H,
>;

pub fn camera() -> EmbeddedIsometry<f64, 0> {
    Flat3::shift_y(0.5).chain(Flat3::shift_z(2.0))
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let view = Mapped::new(PointView::new(1.0), camera());
    let objects = (
        vec![Mapped::new(
            Covered::new(
                Sphere,
                Mixture::new(
                    (Colored::new(Lambertian, [1.0, 0.2, 0.2].into()), 0.0).into(),
                    (Specular, 0.1).into(),
                    (
                        Colored::new(Refractive::new(1.2), [1.0, 1.0, 0.2].into()),
                        0.9,
                    )
                        .into(),
                ),
            ),
            Flat3::shift_y(1.0),
        )],
        vec![Mapped::new(
            Covered::new(
                Cube,
                Mixture::new(
                    (Colored::new(Lambertian, [0.2, 0.8, 0.8].into()), 1.0).into(),
                    (Specular, 0.0).into(),
                    (
                        Colored::new(Refractive::new(1.0), [1.0, 1.0, 1.0].into()),
                        0.0,
                    )
                        .into(),
                ),
            ),
            Flat3::shift_y(-1.0),
        )],
        vec![Mapped::new(
            Covered::new(
                Plane,
                Mixture::new(
                    (Colored::new(Lambertian, [1.0, 1.0, 1.0].into()), 0.9).into(),
                    (Specular, 0.1).into(),
                    (
                        Colored::new(Refractive::new(1.0), [1.0, 1.0, 1.0].into()),
                        0.0,
                    )
                        .into(),
                ),
            ),
            Flat3::shift_z(-1.0),
        )],
    );
    let background = GradBg::new(
        [0.0, 1.0, 0.0].into(),
        [[1.0, 1.0, 1.0].into(), [0.0, 0.0, 0.0].into()],
        2.4,
    );
    SceneImpl::<_, _, _, _, H>::new(view, objects, background)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_placements_use_physical_flat_coordinates() {
        let example = scene::<4>();
        let origin = [1.0, 0.0, 0.0, 0.0].into();
        assert_eq!(
            example.view.map.apply_vector(origin).into_array(),
            [1.0, 0.0, 0.5, 2.0]
        );
        for (map, expected) in [
            example.object.0[0].map,
            example.object.1[0].map,
            example.object.2[0].map,
        ]
        .into_iter()
        .zip([
            [1.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, -1.0, 0.0],
            [1.0, 0.0, 0.0, -1.0],
        ]) {
            assert_eq!(map.apply_vector(origin).into_array(), expected);
        }
    }
}
