use crate::View;
use std::marker::PhantomData;
use type_macros::*;
use types::{
    prelude::*,
    source::{SourceBuilder, SourceTree},
    Config,
};

#[derive(Clone, Debug, Entity, SizedEntity)]
pub struct PointView<G: Geometry> {
    pub fov: f64,
    phantom: PhantomData<G>,
}

impl<G: Geometry> PointView<G> {
    pub fn new(fov: f64) -> Self {
        Self {
            fov,
            phantom: PhantomData,
        }
    }
}

impl<G: Geometry> EntityId for PointView<G> {
    fn name() -> (String, String) {
        let gname = G::name();
        (
            format!("PointView{}", gname.0),
            format!("point_view_{}", gname.1),
        )
    }
}

impl<G: Geometry> EntitySource for PointView<G> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("view/point/{}.hh", &G::geometry_name().1))
            .tree(G::geometry_source(cfg))
            .build()
    }
}

impl<G: Geometry> View<G> for PointView<G> {}
