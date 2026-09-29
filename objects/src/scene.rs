use crate::shader::Geometry;
use crate::{Background, Object, View};
use std::{convert::TryFrom, marker::PhantomData};

pub trait Scene<G: Geometry>: Sized {
    /// Lower a scene without requiring a GPU device.
    fn definition(&self) -> crate::shader::Result<crate::shader::SceneDefinition<G>>;
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
    pub medium: crate::shader::Medium,
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
            medium: crate::shader::Medium::vacuum(),
            geometry: PhantomData,
        }
    }
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize> Scene<G>
    for SceneImpl<G, V, T, B, H>
{
    fn definition(&self) -> crate::shader::Result<crate::shader::SceneDefinition<G>> {
        let modules = T::shader_modules()?;
        let mut objects = Vec::new();
        self.object
            .encode_objects(crate::shader::Transform::identity(), &mut objects)?;
        Ok(crate::shader::SceneDefinition {
            view: self.view.view()?,
            background: self.background.background()?,
            bounces: u32::try_from(H)?,
            radius: self.radius,
            medium: self.medium,
            objects,
            modules,
        })
    }
}
