pub mod eu {
    use types::{Shape, Config, source::{Sourced, SourceInfo, SourceTree}};
    use type_macros::{SizedEntity};
    use ccgeom::{Euclidean3};
    use std::collections::HashMap;

    #[derive(Clone, Default, SizedEntity, Debug)]
    pub struct Sphere;

    impl Sourced for Sphere {
        fn source(_: &Config) -> SourceInfo {
            let mut si = SourceInfo::new(
                "SphereEu".into(),
                "sphere_eu".into(),
            );
            si.source_tree = Some(SourceTree {
                root: String::from("shape/eu/shapes.hh"),
                tree: HashMap::new(),
            });
            si
        }
    }

    impl Shape<Euclidean3<f64>> for Sphere {}
}
