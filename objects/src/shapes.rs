pub mod eu {
    use hy_types::{Shape, Config, source::{Sourced, SourceInfo}};
    use hy_type_macros::{SizedEntity};
    use ccgeom::{Euclidean3};

    #[derive(Clone, Default, SizedEntity, Debug)]
    pub struct Sphere;
    impl Sourced for Sphere {
        fn source(_: &Config) -> SourceInfo {
            let mut si = SourceInfo::new(
                "SphereEu".into(),
                "sphere_eu".into(),
            );
            si.tree.set_root(Some("shape/eu/shapes.hh".into()));
            si
        }
    }
    impl Shape<Euclidean3> for Sphere {}

    #[derive(Clone, Default, SizedEntity, Debug)]
    pub struct Cube;
    impl Sourced for Cube {
        fn source(_: &Config) -> SourceInfo {
            let mut si = SourceInfo::new(
                "CubeEu".into(),
                "cube_eu".into(),
            );
            si.tree.set_root(Some("shape/eu/shapes.hh".into()));
            si
        }
    }
    impl Shape<Euclidean3> for Cube {}
}
