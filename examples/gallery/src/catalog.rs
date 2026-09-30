//! A shared list for native tools and the browser's example selector.

#[derive(Clone, Copy)]
pub struct Example {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub group: &'static str,
}

pub fn find(id: &str) -> Option<&'static Example> {
    EXAMPLES.iter().find(|example| example.id == id)
}

pub const EXAMPLES: &[Example] = &[
    Example {
        id: "hy",
        title: "Hyperbolic tilings",
        group: "Materials and lighting",
        description: "Pentagonal planes, square and hexagonal horospheres, and a dodecahedrally tiled sphere in hyperbolic space.",
    },
    Example {
        id: "eu",
        title: "Euclidean glass",
        group: "Materials and lighting",
        description: "A glass sphere, a diffuse cube and a square-tiled backdrop in flat space.",
    },
    Example {
        id: "sp",
        title: "Spherical studio",
        group: "Materials and lighting",
        description: "An off-center sun lights diffuse red and refractive blue balls on a dodecahedrally tiled floor. Explore the opposite pole and glowing landmark.",
    },
    Example {
        id: "eu-fog",
        title: "Euclidean light in fog",
        group: "Materials and lighting",
        description: "One bright light surrounded by red diffuse, green mirror and blue glass spheres in Euclidean fog. No floor or ambient light; let scattering accumulate.",
    },
    Example {
        id: "ball-tilings",
        title: "Ball tilings",
        group: "Materials and lighting",
        description: "Equal-sized matte balls. Top: tetrahedron, cube, octahedron. Bottom: dodecahedron, icosahedron, eight lunes, two hemispheres. Move around to see every face.",
    },
];

/// Statically typed factories used only by the optional gallery applications.
pub mod factories {
    use ccgeom::{Flat3, Hyperboloid3, Spherical3};
    use objects::{
        Scene as _,
        shader::{Result, SceneDefinition},
    };
    pub fn eu() -> Result<SceneDefinition<Flat3>> {
        crate::eu::scene::<4>().definition()
    }
    pub fn hy() -> Result<SceneDefinition<Hyperboloid3>> {
        crate::hy::scene::<3>().definition()
    }
    pub fn sp() -> Result<SceneDefinition<Spherical3>> {
        crate::sp::scene::<6>().definition()
    }
    pub fn eu_fog() -> Result<SceneDefinition<Flat3>> {
        crate::eu_fog::scene::<12>().definition()
    }
    pub fn ball_tilings() -> Result<SceneDefinition<Flat3>> {
        crate::ball_tilings::scene::<4>().definition()
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
            "eu-fog" => {
                let $metadata = *$crate::find("eu-fog").expect("catalogue entry");
                let $factory = $crate::factories::eu_fog;
                $body
            }
            "ball-tilings" => {
                let $metadata = *$crate::find("ball-tilings").expect("catalogue entry");
                let $factory = $crate::factories::ball_tilings;
                $body
            }
            unknown => Err(anyhow::anyhow!(
                "unknown example {unknown:?}; use --list-scenes"
            )),
        }
    }};
}
