//! A shared list for native tools and the browser's example selector.

#[derive(Clone, Copy)]
pub struct Example {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub group: &'static str,
}

impl Example {
    /// Describe an independent example without registering it in the gallery.
    pub const fn new(id: &'static str, title: &'static str) -> Self {
        Self {
            id,
            title,
            description: "",
            group: "Standalone",
        }
    }
}

pub fn find(id: &str) -> Option<&'static Example> {
    EXAMPLES.iter().find(|example| example.id == id)
}

pub const EXAMPLES: &[Example] = &[
    Example {
        id: "hy",
        title: "Hyperbolic tilings",
        group: "Materials and lighting",
        description: "Explore pentagonal and horosphere tilings in hyperbolic space.",
    },
    Example {
        id: "eu",
        title: "Euclidean glass",
        group: "Materials and lighting",
        description: "Glass, diffuse surfaces and a directional background in flat space.",
    },
    Example {
        id: "sp",
        title: "Spherical studio",
        group: "Materials and lighting",
        description: "Diffuse, glass and mirror spheres lit by emissive objects; the background is black.",
    },
    Example {
        id: "sp-fog",
        title: "Spherical studio with fog",
        group: "Materials and lighting",
        description: "The same studio with isotropic scattering. Compare with Spherical studio and let it accumulate samples.",
    },
    Example {
        id: "compare-eu",
        title: "Perspective: flat",
        group: "Equal physical layouts",
        description: "Equal-size glowing spheres at matching physical distances. Compare their apparent sizes across the three spaces.",
    },
    Example {
        id: "compare-hy",
        title: "Perspective: hyperbolic",
        group: "Equal physical layouts",
        description: "The same spheres and distances at curvature -1: distant objects shrink faster than in flat space.",
    },
    Example {
        id: "compare-sp",
        title: "Perspective: spherical",
        group: "Equal physical layouts",
        description: "The same spheres and distances at curvature +1: apparent size grows again beyond a quarter circuit.",
    },
    Example {
        id: "compare-hy-flat",
        title: "Perspective: gentler hyperbolic",
        group: "Equal physical layouts",
        description: "Curvature radius 3 with unchanged physical sphere sizes and distances. Compare with radius 1 and flat space.",
    },
    Example {
        id: "compare-sp-flat",
        title: "Perspective: gentler spherical",
        group: "Equal physical layouts",
        description: "Curvature radius 3 with unchanged physical sphere sizes and distances. Compare with radius 1 and flat space.",
    },
    Example {
        id: "sp-loop",
        title: "Spherical long route",
        group: "Light around a closed space",
        description: "The central cyan light is behind you. You see it ahead along the long route around the sphere; turn around for the short route.",
    },
    Example {
        id: "sp-loop-fog",
        title: "Spherical long route with fog",
        group: "Light around a closed space",
        description: "Fog scatters and dims the long-route light. Rays missing every sphere can scatter after multiple circuits; there is no floor to stop them.",
    },
];

/// Statically typed factories used only by the optional gallery applications.
pub mod factories {
    use ccgeom::{Flat3, Hyperboloid3, Spherical3};
    use objects::{
        Scene as _,
        shader::{Result, SceneDefinition},
    };
    pub fn hy() -> Result<SceneDefinition<Hyperboloid3>> {
        crate::hy::scene::<3>().definition()
    }
    pub fn eu() -> Result<SceneDefinition<Flat3>> {
        crate::eu::scene::<4>().definition()
    }
    pub fn sp() -> Result<SceneDefinition<Spherical3>> {
        crate::sp::scene::<6>().definition()
    }
    pub fn sp_fog() -> Result<SceneDefinition<Spherical3>> {
        crate::sp::fog_scene::<12>().definition()
    }
    pub fn compare_eu() -> Result<SceneDefinition<Flat3>> {
        crate::comparison::scene::<0, 1>(1.0)?.definition()
    }
    pub fn compare_hy() -> Result<SceneDefinition<Hyperboloid3>> {
        crate::comparison::scene::<-1, 1>(1.0)?.definition()
    }
    pub fn compare_sp() -> Result<SceneDefinition<Spherical3>> {
        crate::comparison::scene::<1, 1>(1.0)?.definition()
    }
    pub fn compare_hy_flat() -> Result<SceneDefinition<Hyperboloid3>> {
        crate::comparison::scene::<-1, 1>(3.0)?.definition()
    }
    pub fn compare_sp_flat() -> Result<SceneDefinition<Spherical3>> {
        crate::comparison::scene::<1, 1>(3.0)?.definition()
    }
    pub fn sp_loop() -> Result<SceneDefinition<Spherical3>> {
        crate::recurrence::scene::<1>(false).definition()
    }
    pub fn sp_loop_fog() -> Result<SceneDefinition<Spherical3>> {
        crate::recurrence::scene::<12>(true).definition()
    }
}

/// Choose a monomorphized application at startup without erasing scene types.
/// The body must return a Result. Its factory retains its concrete geometry.
#[macro_export]
macro_rules! with_example {
    ($id:expr, |$metadata:ident, $factory:ident| $body:expr) => {{
        match $id {
            "hy" => {
                let $metadata = *$crate::find("hy").expect("catalogue entry");
                let $factory = $crate::factories::hy;
                $body
            }
            "eu" => {
                let $metadata = *$crate::find("eu").expect("catalogue entry");
                let $factory = $crate::factories::eu;
                $body
            }
            "sp" => {
                let $metadata = *$crate::find("sp").expect("catalogue entry");
                let $factory = $crate::factories::sp;
                $body
            }
            "sp-fog" => {
                let $metadata = *$crate::find("sp-fog").expect("catalogue entry");
                let $factory = $crate::factories::sp_fog;
                $body
            }
            "compare-eu" => {
                let $metadata = *$crate::find("compare-eu").expect("catalogue entry");
                let $factory = $crate::factories::compare_eu;
                $body
            }
            "compare-hy" => {
                let $metadata = *$crate::find("compare-hy").expect("catalogue entry");
                let $factory = $crate::factories::compare_hy;
                $body
            }
            "compare-sp" => {
                let $metadata = *$crate::find("compare-sp").expect("catalogue entry");
                let $factory = $crate::factories::compare_sp;
                $body
            }
            "compare-hy-flat" => {
                let $metadata = *$crate::find("compare-hy-flat").expect("catalogue entry");
                let $factory = $crate::factories::compare_hy_flat;
                $body
            }
            "compare-sp-flat" => {
                let $metadata = *$crate::find("compare-sp-flat").expect("catalogue entry");
                let $factory = $crate::factories::compare_sp_flat;
                $body
            }
            "sp-loop" => {
                let $metadata = *$crate::find("sp-loop").expect("catalogue entry");
                let $factory = $crate::factories::sp_loop;
                $body
            }
            "sp-loop-fog" => {
                let $metadata = *$crate::find("sp-loop-fog").expect("catalogue entry");
                let $factory = $crate::factories::sp_loop_fog;
                $body
            }
            unknown => Err(anyhow::anyhow!(
                "unknown example {unknown:?}; use --list-scenes"
            )),
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_selectable_example_compiles_without_a_device() {
        for example in super::EXAMPLES {
            crate::with_example!(example.id, |_metadata, factory| {
                objects::shader::compile(&factory().unwrap()).map(|_| ())
            })
            .unwrap_or_else(|error| panic!("{}: {error:#}", example.id));
        }
    }
}
