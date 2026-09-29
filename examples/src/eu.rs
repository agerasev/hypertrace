use ccgeom::{Euclidean3, Homogenous3};
use objects::{
    Mapped, SceneImpl,
    background::GradBg,
    material::{Colored, Lambertian, Refractive, Specular},
    mixture,
    object::Covered,
    shape::{Cube, Plane, Sphere},
    shape_choice,
    view::PointView,
};
use vecmat::{
    Transform, Vector,
    transform::{Rotation3, Shift},
};

shape_choice! {
    Choice {
        Plane(Plane),
        Sphere(Sphere),
        Cube(Cube),
    }
}

mixture! {
    Mixture {
        diffuse: Colored<Lambertian>,
        specular: Specular,
        refractive: Colored<Refractive>,
    }
}

pub type ExampleScene<const H: usize> = SceneImpl<
    Euclidean3,
    Mapped<Euclidean3, PointView<Euclidean3>, Homogenous3<f64>>,
    Vec<Mapped<Euclidean3, Covered<Euclidean3, Choice, Mixture>, Shift<f64, 3>>>,
    GradBg,
    H,
>;

pub fn camera() -> Homogenous3<f64> {
    Homogenous3::new(
        Shift::from(Vector::from([0.0, 0.5, 2.0])),
        Rotation3::identity(),
    )
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let view = Mapped::new(PointView::new(1.0), camera());
    let objects = vec![
        Mapped::new(
            Covered::new(
                Choice::from(Sphere),
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
            Shift::from_vector([0.0, 1.0, 0.0].into()),
        ),
        Mapped::new(
            Covered::new(
                Choice::from(Cube),
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
            Shift::from_vector([0.0, -1.0, 0.0].into()),
        ),
        Mapped::new(
            Covered::new(
                Choice::from(Plane),
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
            Shift::from_vector([0.0, 0.0, -1.0].into()),
        ),
    ];
    let background = GradBg::new(
        [0.0, 1.0, 0.0].into(),
        [[1.0, 1.0, 1.0].into(), [0.0, 0.0, 0.0].into()],
        2.4,
    );
    SceneImpl::<_, _, _, _, H>::new(view, objects, background)
}
