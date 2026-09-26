use super::*;
use base::ccgeom::Euclidean3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, EntityId, Entity, SizedEntity)]
pub struct Sphere;

impl EntitySource for Sphere {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/primitive.hh")
    }
}

impl Shape<Euclidean3> for Sphere {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Sphere)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Ok(crate::wgsl::ShapeValue::sphere())
    }

    fn shape_name() -> (String, String) {
        ("SphereEu".into(), "sphere_eu".into())
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/eu/sphere.hh")
    }
}
