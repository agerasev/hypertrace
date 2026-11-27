use std::f64::consts::TAU;

use anyhow::Result;
use wgame::{Library, prelude::Transformable, shapes::Polygon};

use crate::config::vegetation::{Deciduous, Grass, Pine, Vegetation};

use super::{Material, ecology::Species, mesh};

pub(super) struct Plants {
    deciduous: (Polygon, Material),
    pine: (Polygon, Material),
    grass: (Polygon, Material),
}

impl Plants {
    pub(super) fn new(library: &Library, settings: &Vegetation) -> Result<Self> {
        Ok(Self {
            deciduous: (
                deciduous(library, &settings.deciduous)?,
                settings.deciduous.material,
            ),
            pine: (pine(library, &settings.pine)?, settings.pine.material),
            grass: (grass(library, &settings.grass)?, settings.grass.material),
        })
    }

    pub(super) fn grown(&self, species: Species, growth: f64) -> (Polygon, Material) {
        let (shape, material) = match species {
            Species::Deciduous => &self.deciduous,
            Species::Pine => &self.pine,
            Species::Grass => &self.grass,
        };
        (shape.scale(growth as f32), *material)
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
        let index = self.vertices.len() as u32;
        self.vertices.push(([x, y, 1.0 + z], rgb));
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

fn trunk(height: f64, bark: [f32; 3]) -> Plant {
    let mut plant = Plant::default();
    plant.tapered_rings(height, 5, &[(0.0, 0.048), (0.50, 0.033)], 0.56, bark);
    plant
}

fn deciduous(library: &Library, tree: &Deciduous) -> Result<Polygon> {
    let mut plant = trunk(tree.height, tree.bark);
    plant.tapered_rings(
        tree.height,
        7,
        &[(0.34, 0.14), (0.47, 0.30), (0.71, 0.34), (0.89, 0.21)],
        1.0,
        tree.foliage,
    );
    plant.upload(library)
}

fn pine(library: &Library, tree: &Pine) -> Result<Polygon> {
    let mut plant = trunk(tree.height, tree.bark);
    plant.tapered_rings(tree.height, 7, &[(0.24, 0.29)], 0.73, tree.foliage.lower);
    plant.tapered_rings(tree.height, 7, &[(0.44, 0.23)], 0.88, tree.foliage.middle);
    plant.tapered_rings(tree.height, 7, &[(0.64, 0.16)], 1.0, tree.foliage.upper);
    plant.upload(library)
}

fn grass(library: &Library, tuft: &Grass) -> Result<Polygon> {
    let mut plant = Plant::default();
    let [low, high] = tuft.blade_height;
    for blade in 0..u32::from(tuft.blades) {
        let angle = TAU * f64::from(blade) / f64::from(tuft.blades);
        let (sin, cos) = angle.sin_cos();
        let height = low + (high - low) * f64::from(blade * 5 % 7) / 6.0;
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
        let [a, b] = strip(
            tuft.spread_radius * (7.0 / 37.0),
            0.5 * tuft.blade_width,
            0.0,
            tuft.colors.root,
        );
        let [c, d] = strip(
            tuft.spread_radius * (18.0 / 37.0),
            tuft.blade_width * (3.5 / 9.0),
            height * 0.80,
            tuft.colors.middle,
        );
        let tip = plant.vertex(
            [tuft.spread_radius * cos, height, tuft.spread_radius * sin],
            tuft.colors.tip,
        );
        plant.indices.extend([a, b, c, b, d, c, c, d, tip]);
    }
    plant.upload(library)
}
