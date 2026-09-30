//! Square backdrop tiles and an octahedral tiling on a refractive sphere.
use ccgeom::{EmbeddedIsometry, Flat3, Geometry3};
use objects::{
    Mapped, SceneImpl,
    background::GradBg,
    material::{Colored, Lambertian, Refractive, Specular},
    mixture,
    object::{
        Covered, Tiled,
        tiling::{RegularSpherical, Square},
    },
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

type Map = EmbeddedIsometry<f64, 0>;
pub type Object<S> = Mapped<Flat3, Covered<Flat3, S, Mixture>, Map>;
type TiledObject<S, P, const N: usize> = Mapped<Flat3, Tiled<S, P, Mixture, Mixture, N>, Map>;

pub type ExampleScene<const H: usize> = SceneImpl<
    Flat3,
    Mapped<Flat3, PointView<Flat3>, Map>,
    (
        Vec<TiledObject<Sphere, RegularSpherical<3, 4>, 3>>,
        Vec<Object<Cube>>,
        Vec<TiledObject<Plane, Square, 4>>,
    ),
    GradBg,
    H,
>;

pub fn camera() -> Map {
    Flat3::shift_y(0.5).chain(Flat3::shift_z(2.0))
}

fn material(color: [f32; 3], diffuse: f64, specular: f64) -> Mixture {
    Mixture::new(
        (Colored::new(Lambertian, color.into()), diffuse).into(),
        (Specular, specular).into(),
        (
            Colored::new(Refractive::new(1.2), color.into()),
            1.0 - (diffuse + specular),
        )
            .into(),
    )
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let view = Mapped::new(PointView::new(1.0), camera());
    let objects = (
        vec![Mapped::new(
            Tiled::new(
                Sphere,
                RegularSpherical::<3, 4>::new(0.025),
                [[1.0, 0.9, 0.3], [0.45, 0.8, 1.0], [1.0, 0.45, 0.25]]
                    .map(|color| material(color, 0.08, 0.1)),
                material([0.08, 0.1, 0.12], 0.9, 0.1),
            ),
            Flat3::shift_y(1.0),
        )],
        vec![Mapped::new(
            Covered::new(Cube, material([0.2, 0.8, 0.8], 1.0, 0.0)),
            Flat3::shift_y(-1.0),
        )],
        vec![Mapped::new(
            Tiled::new(
                Plane,
                Square::new(0.5, 0.012),
                [
                    [0.85, 0.85, 0.85],
                    [0.48, 0.55, 0.65],
                    [0.7, 0.75, 0.8],
                    [0.35, 0.42, 0.5],
                ]
                .map(|color| material(color, 0.9, 0.1)),
                material([0.1, 0.12, 0.15], 1.0, 0.0),
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
