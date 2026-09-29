//! A shared list for native tools and the browser's example selector.
use objects::{
    shader::{Result, SceneDefinition},
    Scene as _,
};

pub struct Example {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub group: &'static str,
    build: fn() -> Result<SceneDefinition>,
}

impl Example {
    pub fn definition(&self) -> Result<SceneDefinition> {
        (self.build)()
    }
}

pub fn find(id: &str) -> Option<&'static Example> {
    EXAMPLES.iter().find(|example| example.id == id)
}

pub const EXAMPLES: &[Example] = &[
    Example {
        id: "hy", title: "Hyperbolic tilings", group: "Materials and lighting",
        description: "Explore pentagonal and horosphere tilings in hyperbolic space.",
        build: || crate::hy::scene::<3>().definition(),
    },
    Example {
        id: "eu", title: "Euclidean glass", group: "Materials and lighting",
        description: "Glass, diffuse surfaces and a directional background in flat space.",
        build: || crate::eu::scene::<4>().definition(),
    },
    Example {
        id: "sp", title: "Spherical studio", group: "Materials and lighting",
        description: "Diffuse, glass and mirror spheres lit by emissive objects; the background is black.",
        build: || crate::sp::scene::<6>().definition(),
    },
    Example {
        id: "sp-fog", title: "Spherical studio with fog", group: "Materials and lighting",
        description: "The same studio with isotropic scattering. Compare with Spherical studio and let it accumulate samples.",
        build: || crate::sp::fog_scene::<12>().definition(),
    },
    Example {
        id: "compare-eu", title: "Perspective: flat", group: "Equal physical layouts",
        description: "Equal-size glowing spheres at matching physical distances. Compare their apparent sizes across the three spaces.",
        build: || crate::comparison::scene::<0, 1>(1.0)?.definition(),
    },
    Example {
        id: "compare-hy", title: "Perspective: hyperbolic", group: "Equal physical layouts",
        description: "The same spheres and distances at curvature -1: distant objects shrink faster than in flat space.",
        build: || crate::comparison::scene::<-1, 1>(1.0)?.definition(),
    },
    Example {
        id: "compare-sp", title: "Perspective: spherical", group: "Equal physical layouts",
        description: "The same spheres and distances at curvature +1: apparent size grows again beyond a quarter circuit.",
        build: || crate::comparison::scene::<1, 1>(1.0)?.definition(),
    },
    Example {
        id: "compare-hy-flat", title: "Perspective: gentler hyperbolic", group: "Equal physical layouts",
        description: "Curvature radius 3 with unchanged physical sphere sizes and distances. Compare with radius 1 and flat space.",
        build: || crate::comparison::scene::<-1, 1>(3.0)?.definition(),
    },
    Example {
        id: "compare-sp-flat", title: "Perspective: gentler spherical", group: "Equal physical layouts",
        description: "Curvature radius 3 with unchanged physical sphere sizes and distances. Compare with radius 1 and flat space.",
        build: || crate::comparison::scene::<1, 1>(3.0)?.definition(),
    },
    Example {
        id: "sp-loop", title: "Spherical long route", group: "Light around a closed space",
        description: "The central cyan light is behind you. You see it ahead along the long route around the sphere; turn around for the short route.",
        build: || crate::recurrence::scene::<1>(false).definition(),
    },
    Example {
        id: "sp-loop-fog", title: "Spherical long route with fog", group: "Light around a closed space",
        description: "Fog scatters and dims the long-route light. Rays missing every sphere can scatter after multiple circuits; there is no floor to stop them.",
        build: || crate::recurrence::scene::<12>(true).definition(),
    },
];

#[cfg(test)]
mod tests {
    #[test]
    fn every_selectable_example_compiles_without_a_device() {
        for example in super::EXAMPLES {
            objects::shader::compile(&example.definition().unwrap())
                .unwrap_or_else(|error| panic!("{}: {error:#}", example.id));
        }
    }
}
