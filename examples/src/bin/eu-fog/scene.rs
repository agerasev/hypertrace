//! Two small lights with opaque companions and a glass sphere in spherical fog.
//! There is no floor or ambient light. Small emitters leave most of the scene
//! dark; scattered illumination and shadows need time to accumulate.

use ccgeom::{EmbeddedIsometry, Space3, Spherical3};
use objects::{
    Mapped, Material, SceneImpl,
    background::ConstBg,
    material::{Absorbing, Colored, Emissive, Lambertian, Refractive},
    object::Covered,
    shader::Medium,
    shape::GeodesicSphere,
    view::PointView,
};

type Map = EmbeddedIsometry<f64, 1>;
type Sphere<M> = Mapped<Spherical3, Covered<Spherical3, GeodesicSphere, M>, Map>;
pub type ExampleScene<const H: usize> = SceneImpl<
    Spherical3,
    Mapped<Spherical3, PointView<Spherical3>, Map>,
    (
        Vec<Sphere<Emissive<Absorbing>>>,
        Vec<Sphere<Colored<Lambertian>>>,
        Sphere<Refractive>,
    ),
    ConstBg,
    H,
>;

pub fn camera() -> Map {
    Map::identity()
}

fn displacement(position: [f64; 3]) -> Map {
    let distance = position.iter().map(|v| v * v).sum::<f64>().sqrt();
    Space3::<f64, 1>::unit()
        .translation(position.into(), distance)
        .expect("finite example position")
}

fn sphere<M: Material<Spherical3>>(map: Map, radius: f64, material: M) -> Sphere<M> {
    Mapped::new(Covered::new(GeodesicSphere::new(radius), material), map)
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let mut lights = Vec::new();
    let mut companions = Vec::new();
    for (position, emission) in [
        ([-0.65, 0.25, -1.8], [0.8, 0.7, 0.55]),
        ([0.7, -0.15, -2.0], [0.2, 0.35, 0.6]),
    ] {
        let center = displacement(position);
        lights.push(sphere(
            center,
            0.055,
            Emissive::new(Absorbing, emission.into()),
        ));
        // Small opaque balls on the camera-facing side cast shadows into the
        // fog. Place them in each light's own frame, preserving physical sizes.
        for offset in [
            [-0.13, 0.0, 0.22],
            [0.13, 0.0, 0.22],
            [0.0, -0.13, 0.22],
            [0.0, 0.13, 0.22],
        ] {
            companions.push(sphere(
                center.chain(displacement(offset)),
                0.06,
                Colored::new(Lambertian, [0.2; 3].into()),
            ));
        }
    }
    let glass = sphere(
        displacement([0.32, -0.12, -1.25]),
        0.18,
        Refractive::new(1.4),
    );
    let mut scene = SceneImpl::new(
        Mapped::new(PointView::new(1.0), camera()),
        (lights, companions, glass),
        ConstBg::new([0.0; 3].into()),
    );
    // Extinction is per physical world unit; its reciprocal is the mean free flight.
    scene.medium = Medium::homogeneous(0.65, [0.95; 3]);
    scene
}
