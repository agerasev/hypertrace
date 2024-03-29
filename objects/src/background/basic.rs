use crate::Background;
use base::{ccgeom::Euclidean3, vecmat::Vector};
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config,
};

/// Constant color background.
#[derive(Clone, Debug, EntityId, Entity, SizedEntity)]
pub struct ConstBg {
    pub color: Vector<f32, 3>,
}

impl ConstBg {
    pub fn new(color: Vector<f32, 3>) -> Self {
        Self { color }
    }
}

impl EntitySource for ConstBg {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("background/constant.hh")
    }
}

impl<G: Geometry> Background<G> for ConstBg {
    fn background_name() -> (String, String) {
        (
            format!("ConstBg{}", G::geometry_name().0),
            format!("const_bg_{}", G::geometry_name().1),
        )
    }
    fn background_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/const_bg_{}.hh", &G::geometry_name().1,))
            .tree(G::geometry_source(cfg))
            .content(&include(format!(
                "render/light/{}.hh",
                &G::geometry_name().1,
            )))
            .content(&include_template!(
                "background/constant.inl",
                ("Geo", "geo") => G::geometry_name(),
            ))
            .build()
    }
}

/// Gradient background.
/// Available only for euclidean space because only that space preserves direction.
#[derive(Clone, Debug, EntityId, Entity, SizedEntity)]
pub struct GradBg {
    pub direction: Vector<f64, 3>,
    pub colors: [Vector<f32, 3>; 2],
    pub power: f32,
}

impl GradBg {
    pub fn new(direction: Vector<f64, 3>, colors: [Vector<f32, 3>; 2], power: f32) -> Self {
        Self {
            direction,
            colors,
            power,
        }
    }
}

impl EntitySource for GradBg {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("background/gradient.hh")
    }
}

impl Background<Euclidean3> for GradBg {}
