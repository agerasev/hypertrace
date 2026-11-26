use std::f64::consts::TAU;

use anyhow::Result;
use wgame::{Library, glam::Vec3, prelude::Transformable, shapes::Polygon};

use super::{Material, ecology::Species, mesh};

pub(super) struct Plants {
    deciduous: Polygon,
    pine: Polygon,
    grass: Polygon,
}

impl Plants {
    pub(super) fn new(library: &Library) -> Result<Self> {
        Ok(Self {
            deciduous: tree(library, 0.25, false)?,
            pine: tree(library, 0.22, true)?,
            grass: grass(library)?,
        })
    }

    pub(super) fn grown(&self, species: Species, growth: f64) -> (Polygon, Material) {
        let (shape, material) = match species {
            Species::Deciduous => (&self.deciduous, Material::Diffuse(Vec3::ONE)),
            Species::Pine => (&self.pine, Material::Diffuse(Vec3::ONE)),
            Species::Grass => (
                &self.grass,
                Material::Translucent {
                    color: Vec3::ONE,
                    transmission: 0.22,
                },
            ),
        };
        (shape.scale(growth as f32), material)
    }
}

#[derive(Default)]
struct Plant {
    vertices: Vec<([f64; 3], [f32; 3])>,
    indices: Vec<u32>,
}

impl Plant {
    // The root is (0,0,1) in local half-space. +y grows away from
    // the geodesic ground plane and z is an offset in chart height.
    fn vertex(&mut self, [x, y, z]: [f64; 3], rgb: [f32; 3]) -> u32 {
        const PLANT_SCALE: f64 = 0.25;
        let index = self.vertices.len() as u32;
        self.vertices.push((
            [x * PLANT_SCALE, y * PLANT_SCALE, 1.0 + z * PLANT_SCALE],
            rgb,
        ));
        index
    }

    // Stack polygonal rings, close the base, then taper to a single tip.
    fn tapered_rings(
        &mut self,
        height: f64,
        sides: u32,
        rings: &[(f64, f64)],
        tip: f64,
        color: [f32; 3],
    ) {
        let base = self.vertices.len() as u32;
        for (ring, &(level, radius)) in rings.iter().enumerate() {
            for side in 0..sides {
                let angle = TAU * f64::from(side) / f64::from(sides);
                let (sin, cos) = angle.sin_cos();
                let i = self.vertex(
                    [radius * height * cos, level * height, radius * height * sin],
                    color,
                );
                if ring > 0 {
                    let next = base + ring as u32 * sides + (side + 1) % sides;
                    self.indices
                        .extend([i - sides, i, next - sides, next - sides, i, next]);
                }
            }
        }
        let top = self.vertex([0.0, height * tip, 0.0], color);
        for side in 0..sides {
            self.indices
                .extend([top - sides + side, top, top - sides + (side + 1) % sides]);
        }
        for side in 1..sides - 1 {
            self.indices.extend([base, base + side, base + side + 1]);
        }
    }

    fn upload(&self, library: &Library) -> Result<Polygon> {
        mesh(library, &self.vertices, &self.indices)
    }
}

fn tree(library: &Library, height: f64, conifer: bool) -> Result<Polygon> {
    let mut plant = Plant::default();
    plant.tapered_rings(
        height,
        5,
        &[(0.0, 0.048), (0.50, 0.033)],
        0.56,
        [0.29, 0.14, 0.055],
    );
    if conifer {
        plant.tapered_rings(height, 7, &[(0.24, 0.29)], 0.73, [0.07, 0.32, 0.14]);
        plant.tapered_rings(height, 7, &[(0.44, 0.23)], 0.88, [0.095, 0.39, 0.15]);
        plant.tapered_rings(height, 7, &[(0.64, 0.16)], 1.0, [0.15, 0.47, 0.18]);
    } else {
        plant.tapered_rings(
            height,
            7,
            &[(0.34, 0.14), (0.47, 0.30), (0.71, 0.34), (0.89, 0.21)],
            1.0,
            [0.28, 0.51, 0.10],
        );
    }
    plant.upload(library)
}

fn grass(library: &Library) -> Result<Polygon> {
    let mut plant = Plant::default();
    for blade in 0..9 {
        let angle = TAU * f64::from(blade) / 9.0;
        let (sin, cos) = angle.sin_cos();
        let height = 0.016 + 0.003 * f64::from(blade * 5 % 7);
        // The blade bends outward near its tip, where it catches sunlight
        // even when the nearly vertical lower face is in shade.
        let mut strip = |radius: f64, width: f64, y, tint| {
            [-width, width].map(|offset| {
                plant.vertex(
                    [radius * cos - offset * sin, y, radius * sin + offset * cos],
                    tint,
                )
            })
        };
        let [a, b] = strip(0.007, 0.0045, 0.0, [0.18, 0.39, 0.06]);
        let [c, d] = strip(0.018, 0.0035, height * 0.80, [0.28, 0.53, 0.09]);
        let tip = plant.vertex([0.037 * cos, height, 0.037 * sin], [0.35, 0.61, 0.13]);
        plant.indices.extend([a, b, c, b, d, c, c, d, tip]);
    }
    plant.upload(library)
}
