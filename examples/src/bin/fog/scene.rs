//! One bright light surrounded by diffuse, reflective and refractive spheres.
//! Euclidean scattering fog reveals the illumination and shadows; there is no
//! floor, ambient light or emission from the fog itself.

use ccgeom::{EmbeddedIsometry, Flat3, Space3};
use objects::{
    Mapped, Material, SceneImpl,
    background::ConstBg,
    light::SphereBound,
    material::{Absorbing, Colored, Emissive, Lambertian, Refractive, Specular},
    object::{Covered, Sampled},
    shader::Medium,
    shape::GeodesicSphere,
    view::PointView,
};

type Map = EmbeddedIsometry<f64, 0>;
type Sphere<M> = Mapped<Flat3, Covered<Flat3, GeodesicSphere, M>, Map>;
type Light =
    Mapped<Flat3, Sampled<Covered<Flat3, GeodesicSphere, Emissive<Absorbing>>, SphereBound>, Map>;
pub type ExampleScene<const H: usize> = SceneImpl<
    Flat3,
    Mapped<Flat3, PointView<Flat3>, Map>,
    (
        Light,
        Sphere<Colored<Lambertian>>,
        Sphere<Colored<Specular>>,
        Sphere<Colored<Refractive>>,
    ),
    ConstBg,
    H,
>;

pub fn camera() -> Map {
    Map::identity()
}

fn sphere<M: Material<Flat3>>(position: [f64; 3], radius: f64, material: M) -> Sphere<M> {
    let distance = position.iter().map(|v| v * v).sum::<f64>().sqrt();
    let map = Space3::<f64, 0>::unit()
        .translation(position.into(), distance)
        .expect("finite example position");
    Mapped::new(Covered::new(GeodesicSphere::new(radius), material), map)
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let light = sphere(
        [0.0, 0.0, -2.65],
        0.12,
        Emissive::new(Absorbing, [24.0; 3].into()),
    );
    // The wrapper sits inside the map so its bound follows the emitter.
    let light = Mapped::new(Sampled::new(light.inner, SphereBound::new(0.12)), light.map);
    let red = sphere(
        [-0.7, -0.1, -2.9],
        0.28,
        Colored::new(Lambertian, [0.85, 0.06, 0.025].into()),
    );
    let green = sphere(
        [0.5, 0.6, -3.0],
        0.28,
        Colored::new(Specular, [0.12, 0.9, 0.2].into()),
    );
    // Slightly toward the camera so refracted light crosses visible fog.
    let blue = sphere(
        [0.45, -0.6, -2.5],
        0.28,
        Colored::new(Refractive::new(1.45), [0.35, 0.6, 0.98].into()),
    );
    let mut scene = SceneImpl::new(
        Mapped::new(PointView::new(0.45), camera()),
        (light, red, green, blue),
        ConstBg::new([0.0; 3].into()),
    );
    // Extinction is per physical world unit; its reciprocal is the mean free flight.
    scene.medium = Medium::homogeneous(0.35, [0.95; 3]);
    scene
}
