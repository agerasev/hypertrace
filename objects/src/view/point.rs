use ccgeom::Geometry3;
use types::{Named, Sourced, Config, source::{SourceTree, SourceBuilder}};
use std::{marker::PhantomData};
use type_macros::{Entity, SizedEntity};
use crate::View;


#[derive(Clone, Debug, Entity, SizedEntity)]
pub struct PointView<G: Geometry3 + Named> {
    pub fov: f64,
    phantom: PhantomData<G>,
}

impl<G: Geometry3 + Named> PointView<G> {
    pub fn new(fov: f64) -> Self {
        Self { fov, phantom: PhantomData }
    }
}

impl<G: Geometry3 + Named> Named for PointView<G> {
    fn type_name(cfg: &Config) -> String {
        format!("PointView{}", G::type_name(cfg))
    }
    fn type_prefix(cfg: &Config) -> String {
        format!("point_view_{}", G::type_prefix(cfg))
    }
}

impl<G: Geometry3 + Sourced> Sourced for PointView<G> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("view/point/{}.hh", &G::type_prefix(cfg)))
        .tree(G::source(cfg))
        .build()
    }
}

impl<G: Geometry3 + Sourced> View<G> for PointView<G> {}
