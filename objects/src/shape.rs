use types::{Entity, Sourced};
use ccgeom::Geometry3;

pub trait Shape<G: Geometry3>: Entity + Sourced {}

pub mod eu {
    use types::{Config, source::{Sourced, SourceInfo}};
    use type_macros::{SizedEntity};
    use ccgeom::{Euclidean3};
    use super::Shape;

    // Sphere

    #[derive(Clone, Default, SizedEntity, Debug)]
    pub struct Sphere;

    impl Sourced for Sphere {
        fn source(_: &Config) -> SourceInfo {
            SourceInfo::with_root(
                "SphereEu",
                "sphere_eu",
                "shape/eu/shapes.hh",
            )
        }
    }

    impl Shape<Euclidean3> for Sphere {}

    // Cube

    #[derive(Clone, Default, SizedEntity, Debug)]
    pub struct Cube;

    impl Sourced for Cube {
        fn source(_: &Config) -> SourceInfo {
            SourceInfo::with_root(
                "CubeEu",
                "cube_eu",
                "shape/eu/shapes.hh",
            )
        }
    }

    impl Shape<Euclidean3> for Cube {}
}
