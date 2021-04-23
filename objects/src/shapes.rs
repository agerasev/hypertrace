pub mod eu {
    use types::{Shape, Config, source::{Sourced, SourceInfo}};
    use type_macros::{SizedEntity};
    use ccgeom::{Euclidean3};

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
