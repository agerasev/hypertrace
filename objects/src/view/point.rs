use crate::shader::Geometry;
use crate::View;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
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

impl<G: Geometry> View<G> for PointView<G> {
    fn view(&self) -> crate::shader::Result<crate::shader::View<G>> {
        Ok(crate::shader::View {
            map: crate::shader::Transform::<G>::identity(),
            fov: self.fov,
        })
    }
}
