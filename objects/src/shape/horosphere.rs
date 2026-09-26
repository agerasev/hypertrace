use super::*;
use base::ccgeom::Hyperbolic3;
use type_macros::*;
use types::{source::SourceTree, Config};

#[derive(Clone, Default, Debug, EntityId, Entity, SizedEntity)]
pub struct Horosphere;

impl EntitySource for Horosphere {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("shape/hy/horosphere.hh")
    }
}

impl Shape<Hyperbolic3> for Horosphere {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Horosphere)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        Ok(crate::wgsl::ShapeValue::horosphere())
    }
}
