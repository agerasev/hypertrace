use crate::{Background, Object, View};
use ccgeom::Geometry;
use std::{convert::TryFrom, marker::PhantomData};

pub trait Scene<G: Geometry>: Sized {
    /// Lower a scene without requiring a GPU device.
    fn wgsl_scene(&self) -> crate::wgsl::Result<crate::wgsl::SceneDefinition> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}

#[derive(Clone, Debug)]
pub struct SceneImpl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize> {
    geometry: PhantomData<G>,
    pub view: V,
    pub background: B,
    pub object: T,
    /// Curvature radius in physical units; Euclidean scenes use one.
    pub radius: f64,
    /// Medium sampled before resolving a surface miss to the background.
    pub medium: crate::wgsl::Medium,
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize>
    SceneImpl<G, V, T, B, H>
{
    pub fn new(view: V, object: T, background: B) -> Self {
        Self {
            view,
            background,
            object,
            radius: 1.0,
            medium: crate::wgsl::Medium::Vacuum,
            geometry: PhantomData,
        }
    }
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize> Scene<G>
    for SceneImpl<G, V, T, B, H>
{
    fn wgsl_scene(&self) -> crate::wgsl::Result<crate::wgsl::SceneDefinition> {
        let mut registry = crate::wgsl::Registry::default();
        T::wgsl_register(&mut registry)?;
        Ok(crate::wgsl::SceneDefinition {
            view: self.view.wgsl_view()?,
            background: self.background.wgsl_background()?,
            bounces: u32::try_from(H)?,
            radius: self.radius,
            medium: self.medium,
            object: self.object.wgsl_object()?,
            material_schemas: registry.material_schemas,
            shape_schemas: registry.shape_schemas,
        })
    }
}
