use crate::Material;
use type_macros::*;
use types::{prelude::*, source::SourceTree, Config};

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Absorbing;

impl EntitySource for Absorbing {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/absorbing.hh")
    }
}

impl Material for Absorbing {
    fn wgsl_material_schema() -> crate::wgsl::Result<crate::wgsl::MaterialSchema> {
        Ok(crate::wgsl::MaterialSchema::Absorbing)
    }

    fn wgsl_material(&self) -> crate::wgsl::Result<crate::wgsl::MaterialValue> {
        Ok(crate::wgsl::MaterialValue::absorbing())
    }
}
