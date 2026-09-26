use crate::View;
use ccgeom::Geometry;
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
    fn wgsl_view(&self) -> crate::wgsl::Result<crate::wgsl::View> {
        Ok(crate::wgsl::View {
            map: crate::wgsl::Transform::identity(crate::wgsl::geometry::<G>()?),
            fov: self.fov,
        })
    }
}
