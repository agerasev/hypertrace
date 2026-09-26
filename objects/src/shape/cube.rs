use super::*;
use base::ccgeom::Euclidean3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, EntityId, Entity, SizedEntity)]
pub struct Cube;

impl EntitySource for Cube {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/primitive.hh")
    }
}

impl Shape<Euclidean3> for Cube {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Cube)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Ok(crate::wgsl::ShapeValue::cube())
    }

    fn shape_name() -> (String, String) {
        ("CubeEu".into(), "cube_eu".into())
    }
    fn shape_source(_: &Config) -> SourceTree {
        SourceTree::new("shape/eu/cube.hh")
    }
}
