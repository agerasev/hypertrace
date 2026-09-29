//! Unit-radius spherical space lit by emissive geodesic spheres.
//!
//! The miss background starts black and remains configurable through the scene's
//! `background` field. Surface emission supplies all illumination in this demo.

use ccgeom::{EmbeddedIsometry, Geometry3, Space3, Spherical3};
use objects::{
    background::ConstBg,
    material::{Colored, Emissive, Lambertian, Refractive, Specular},
    mixture,
    object::Covered,
    shape::{GeodesicSphere, Plane},
    shape_choice,
    view::PointView,
    Mapped, SceneImpl,
};

shape_choice! {
    Choice {
        Plane(Plane),
        Sphere(GeodesicSphere),
    }
}

mixture! {
    Mixture {
        diffuse: Colored<Lambertian>,
        specular: Specular,
        refractive: Colored<Refractive>,
    }
}

type Material = Emissive<Mixture>;
type Object = Mapped<Spherical3, Covered<Spherical3, Choice, Material>, EmbeddedIsometry<f64, 1>>;

pub type ExampleScene<const H: usize> = SceneImpl<
    Spherical3,
    Mapped<Spherical3, PointView<Spherical3>, EmbeddedIsometry<f64, 1>>,
    Vec<Object>,
    ConstBg,
    H,
>;

pub fn camera() -> EmbeddedIsometry<f64, 1> {
    EmbeddedIsometry::identity()
}

fn material(color: [f32; 3], specular: f64, refractive: f64, emission: [f32; 3]) -> Material {
    Emissive::new(
        Mixture::new(
            (
                Colored::new(Lambertian, color.into()),
                1.0 - specular - refractive,
            )
                .into(),
            (Specular, specular).into(),
            (Colored::new(Refractive::new(1.3), color.into()), refractive).into(),
        ),
        emission.into(),
    )
}

fn sphere(position: [f64; 3], radius: f64, material: Material) -> Object {
    let distance = position.iter().map(|v| v * v).sum::<f64>().sqrt();
    let map = Space3::<f64, 1>::unit()
        .translation(position.into(), distance)
        .expect("finite demo position");
    Mapped::new(
        Covered::new(GeodesicSphere::new(radius).into(), material),
        map,
    )
}

pub fn scene<const H: usize>() -> ExampleScene<H> {
    let objects = vec![
        sphere(
            [-0.46, -0.12, -1.05],
            0.3,
            material([0.9, 0.2, 0.12], 0.1, 0.0, [0.0; 3]),
        ),
        sphere(
            [0.43, -0.17, -1.1],
            0.27,
            material([0.85, 0.95, 1.0], 0.1, 0.85, [0.0; 3]),
        ),
        sphere(
            [0.03, 0.32, -1.6],
            0.24,
            material([0.2, 0.65, 0.85], 0.8, 0.0, [0.0; 3]),
        ),
        sphere(
            [-0.25, 0.92, -0.65],
            0.34,
            material([0.6; 3], 0.0, 0.0, [0.8, 0.7, 0.55]),
        ),
        sphere(
            [0.65, 0.3, 1.25],
            0.4,
            material([0.6; 3], 0.0, 0.0, [0.2, 0.35, 0.6]),
        ),
        Mapped::new(
            Covered::new(
                Plane.into(),
                material([0.52, 0.55, 0.6], 0.05, 0.0, [0.0; 3]),
            ),
            Spherical3::shift_y(-0.55).chain(Spherical3::rotate_x(std::f64::consts::FRAC_PI_2)),
        ),
    ];
    SceneImpl::new(
        Mapped::new(PointView::new(1.0), camera()),
        objects,
        ConstBg::new([0.0; 3].into()),
    )
}

/// The spherical demo with a homogeneous scattering medium.
///
/// Extinction is inverse physical distance. Its mean free flight is 12.5 world
/// units, almost two complete circuits of the unit sphere. Surface events still
/// interrupt a sampled free flight whenever they occur first.
///
/// ```
/// use hypertrace_examples::sp;
/// use objects::Scene;
/// let scene = sp::fog_scene::<8>();
/// let definition = scene.wgsl_scene().unwrap();
/// ```
pub fn fog_scene<const H: usize>() -> ExampleScene<H> {
    let mut scene = scene();
    scene.medium = objects::wgsl::Medium::Homogeneous {
        extinction: 0.08,
        albedo: [0.85, 0.9, 0.95],
    };
    scene
}

#[cfg(test)]
mod tests {
    use super::*;
    use objects::{wgsl, Scene as _};

    #[test]
    fn spherical_demo_lowers_with_black_background_and_emissive_objects() {
        let scene = scene::<6>();
        let definition = scene.wgsl_scene().unwrap();
        assert_eq!(definition.view.map.geometry(), wgsl::Geometry::Spherical);
        assert!(matches!(
            definition.background,
            wgsl::Background::Constant([0.0, 0.0, 0.0])
        ));
        assert!(matches!(definition.medium, wgsl::Medium::Vacuum));
        assert_eq!(definition.radius, 1.0);
        assert_eq!(definition.bounces, 6);
        assert_eq!(scene.object.len(), 6);
        assert!(definition
            .material_schemas
            .iter()
            .any(|schema| matches!(schema, wgsl::MaterialSchema::Emissive(_))));
        assert!(definition.shape_schemas.iter().any(|schema| matches!(schema, wgsl::ShapeSchema::Choice(shapes) if shapes.contains(&wgsl::ShapeSchema::GeodesicSphere))));
    }
}
