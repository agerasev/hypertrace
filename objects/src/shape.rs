use types::{Entity, source::Sourced};
use ccgeom::Geometry3;

pub trait Shape<G: Geometry3>: Entity + Sourced {}

pub mod eu {
    use types::{Config, Named, source::{Sourced, SourceTree}};
    use type_macros::{SizedEntity};
    use ccgeom::{Euclidean3};
    use super::*;

    // Sphere

    #[derive(Clone, Default, Debug, SizedEntity)]
    pub struct Sphere;

    impl Named for Sphere {
        fn type_name(_: &Config) -> String { "SphereEu".into() }
        fn type_prefix(_: &Config) -> String { "sphere_eu".into() }
    }

    impl Sourced for Sphere where Self: Entity {
        fn source(_: &Config) -> SourceTree {
            SourceTree::new("shape/eu/shapes.hh")
        }
    }

    impl Shape<Euclidean3> for Sphere {}

    // Cube

    #[derive(Clone, Default, Debug, SizedEntity)]
    pub struct Cube;

    impl Named for Cube {
        fn type_name(_: &Config) -> String { "CubeEu".into() }
        fn type_prefix(_: &Config) -> String { "cube_eu".into() }
    }

    impl Sourced for Cube where Self: Entity {
        fn source(_: &Config) -> SourceTree {
            SourceTree::new("shape/eu/shapes.hh")
        }
    }

    impl Shape<Euclidean3> for Cube {}
}
