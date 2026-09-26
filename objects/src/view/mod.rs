mod mapped;
mod point;

use types::{prelude::*, source::SourceTree, Config};

pub trait View<G: Geometry>: SizedEntity + EntitySource {
    /// Lower the camera in f64; GPU conversion occurs when uploading the scene.
    fn wgsl_view(&self) -> crate::wgsl::Result<crate::wgsl::View> {
        Err(crate::wgsl::unsupported::<Self>())
    }

    fn view_name() -> (String, String) {
        Self::name()
    }
    fn view_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

pub use mapped::*;
pub use point::*;
