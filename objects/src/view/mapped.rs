use crate::{Mapped, View};
use ccgeom::{Geometry, Map};

impl<G: Geometry, T: View<G>, M: Map<G::Pos, G::Dir> + 'static> View<G> for Mapped<G, T, M> {
    fn wgsl_view(&self) -> crate::wgsl::Result<crate::wgsl::View> {
        let mut view = self.inner.wgsl_view()?;
        view.map = crate::wgsl::transform::<G, M>(&self.map)?.chain(&view.map)?;
        Ok(view)
    }
}
