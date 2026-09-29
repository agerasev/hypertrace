use crate::{Mapped, View};
use ccgeom::Geometry;

impl<G: Geometry, T: View<G>, M: crate::shader::RenderMap<G>> View<G> for Mapped<G, T, M> {
    fn view(&self) -> crate::shader::Result<crate::shader::View> {
        let mut view = self.inner.view()?;
        view.map = crate::shader::transform::<G, M>(&self.map)?.chain(&view.map)?;
        Ok(view)
    }
}
