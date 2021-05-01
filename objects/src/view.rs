use ccgeom::Geometry3;
use types::{Entity, SizedEntity, Sourced, Config, source::{SourceInfo, SourceTree}};
use std::{marker::PhantomData, iter};
use type_macros::SizedEntity;

pub trait View<G: Geometry3>: SizedEntity + Sourced {}

#[derive(Clone, Debug, SizedEntity)]
pub struct PointView<G: Geometry3> {
    fov: f64,
    phantom: PhantomData<G>,
}

impl<G: Geometry3> PointView<G> {
    pub fn new(fov: f64) -> Self {
        Self { fov, phantom: PhantomData }
    }
}

impl<G: Geometry3 + Sourced> Sourced for PointView<G> {
    fn source(cfg: &Config) -> SourceInfo {
        let geo_src = G::source(cfg);
        SourceInfo::with_trees(
            format!("PointView{}", &geo_src.name),
            format!("point_view_{}", &geo_src.prefix),
            "view/point.hh",
            iter::once(geo_src.tree),
        ).unwrap()
    }
}

impl<G: Geometry3 + Sourced> View<G> for PointView<G> {}

#[derive(Clone, Debug, SizedEntity)]
pub struct MappedView<G: Geometry3, V: View<G>> {
    map: G::Map,
    inner: V,
}

impl<G: Geometry3 + Sourced, V: View<G> + Sourced> Sourced for MappedView<G, V> where G::Map: SizedEntity {
    fn source(cfg: &Config) -> SourceInfo {
        let geo_src = G::source(cfg);
        let view_src = V::source(cfg);
        let file = format!("generated/mapped_view_{}.hh", Self::type_tag());
        let name = format!("MappedView{}{}", &geo_src.name, Self::type_tag());
        let prefix = format!("mapped_view_{}_{}", &geo_src.prefix, Self::type_tag());
        let mut tree = SourceTree::new(file.clone().into());
        tree.insert(file.into(), format!(
            r#"
                #pragma once

                #include <{geo_root}>
                #include <geometry/ray.hh>
                #include <{view_root}>

                #define $Self {name}
                #define $self {prefix}
                #define $View {View}
                #define $view {view}
                #define $Geo {Geo}
                #define $geo {geo}
                #include "mapped.inl"
                #undef $Self
                #undef $self
                #undef $View
                #undef $view
                #undef $Geo
                #undef $geo
            "#,
            geo_root=geo_src.tree.root(),
            view_root=view_src.tree.root(),
            name=&name,
            prefix=&prefix,
            Geo=&geo_src.name,
            geo=&geo_src.prefix,
            View=&view_src.name,
            view=&view_src.prefix,
        )).unwrap();
        tree.append(geo_src.tree).unwrap();
        tree.append(view_src.tree).unwrap();
        
        SourceInfo::new(
            name,
            prefix,
            tree,
        )
    }
}

impl<G: Geometry3 + Sourced, V: View<G> + Sourced> View<G> for MappedView<G, V> where G::Map: SizedEntity {}
