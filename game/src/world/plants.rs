use std::f64::consts::TAU;

use anyhow::Result;
use wgame::{
    Library,
    glam::{Mat3, Vec3},
    prelude::Transformable,
    shapes::Polygon,
};

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

// The plant's half-space patch has geodesic ground v=0. Rotate its embedded
// frame so +v is world +Z and increasing chart height points along world -Y at the root.
const CHART_TO_PLANT: Mat3 = Mat3::from_cols(Vec3::X, Vec3::Z, Vec3::NEG_Y);

impl Plant {
    // Author in a Z-up frame with the root at zero; store the chart's (u, v, h).
    fn vertex(&mut self, [x, y, z]: [f64; 3], rgb: [f32; 3]) -> u32 {
        let index = self.vertices.len() as u32;
        self.vertices.push(([x, z, 1.0 - y], rgb));
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
                    [
                        radius * height * cos,
                        -radius * height * sin,
                        level * height,
                    ],
                    color,
                );
                if ring > 0 {
                    let next = base + ring as u32 * sides + (side + 1) % sides;
                    self.indices
                        .extend([i - sides, i, next - sides, next - sides, i, next]);
                }
            }
        }
        let top = self.vertex([0.0, 0.0, height * tip], color);
        for side in 0..sides {
            self.indices
                .extend([top - sides + side, top, top - sides + (side + 1) % sides]);
        }
        for side in 1..sides - 1 {
            self.indices.extend([base, base + side, base + side + 1]);
        }
    }

    fn upload(&self, library: &Library) -> Result<Polygon> {
        Ok(mesh(library, &self.vertices, &self.indices)?.transform(CHART_TO_PLANT))
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
        let mut strip = |radius: f64, width: f64, z, tint| {
            [-width, width].map(|offset| {
                plant.vertex(
                    [radius * cos - offset * sin, -radius * sin - offset * cos, z],
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
            [tuft.spread_radius * cos, -tuft.spread_radius * sin, height],
            tuft.colors.tip,
        );
        plant.indices.extend([a, b, c, b, d, c, c, d, tip]);
    }
    plant.upload(library)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ccgeom::Space3;

    #[test]
    fn a_tapered_plant_stands_on_z_zero_with_its_base_facing_down() {
        let mut plant = Plant::default();
        let sides = 6;
        plant.tapered_rings(0.14, sides, &[(0.0, 0.1), (0.5, 0.08)], 1.0, [1.0; 3]);
        let space = Space3::<f64, -1>::unit();
        let vertices: Vec<_> = plant
            .vertices
            .iter()
            .map(|&(chart, _)| {
                let p = space.point_from_half_space(chart.into()).unwrap();
                CHART_TO_PLANT * (Vec3::new(p[1] as f32, p[2] as f32, p[3] as f32) / p[0] as f32)
            })
            .collect();
        let (faces, remainder) = plant.indices.as_chunks::<3>();
        assert!(remainder.is_empty());
        for face in faces {
            let [a, b, c] = face.map(|index| vertices[index as usize]);
            assert!(a.z >= 0.0 && b.z >= 0.0 && c.z >= 0.0);
            if a.z == 0.0 && b.z == 0.0 && c.z == 0.0 {
                assert!((b - a).cross(c - a).z < 0.0);
            }
        }
        assert!(vertices[..sides as usize].iter().all(|v| v.z == 0.0));
        assert!(vertices.last().unwrap().z > 0.0);
    }
}
