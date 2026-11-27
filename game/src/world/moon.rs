use std::f64::consts::FRAC_PI_2;

use anyhow::{Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3};
use wgame::{
    Library,
    shader::Attribute,
    shapes::{Polygon, shader::Vertex},
};

use super::{LightingDomain, Object, Pose, mesh};
use crate::config::Moon;

pub(crate) struct HexGrid {
    radius: f64,
    width: f64,
    columns: u32,
    rows: u32,
}

impl HexGrid {
    pub(crate) fn new(subdivisions: u32) -> Result<Self> {
        ensure!(subdivisions > 0, "moon.subdivisions must be positive");
        let width = 4.0 / f64::from(subdivisions); // distance across opposite sides
        let radius = width / 3.0_f64.sqrt();

        // A facet lies on a hemisphere with heights from 1 to H = hypot(1, r)
        // across the hexagon. An eye below the lunar gap has chart height < 1
        // and horizontal radius at most exp(-gap) * cosh(altitude) < 1.
        // A ray reaching H must hit on ascent within horizontal travel H. A lower
        // ray can travel at most another sqrt(H²-1) = r past its apex at height ≥ 1.
        let extent = 1.0 + radius.hypot(1.0) + radius;
        let columns = 2 * (extent / width).ceil() as u64 + 1;
        let rows = 2 * ((extent + radius) / (1.5 * radius)).ceil() as u64 + 1;
        // Four indexed triangles per hexagon. The shared vertex count is smaller.
        ensure!(
            columns <= u64::from(u32::MAX) / 12 / rows,
            "moon.subdivisions must fit indexed triangle buffers"
        );
        Ok(Self {
            radius,
            width,
            columns: columns as u32,
            rows: rows as u32,
        })
    }

    // Each boundary between rows is one zigzag of shared vertices. Adjacent
    // rows of hexagons are offset horizontally by half a hexagon width.
    fn stride(&self) -> u32 {
        2 * self.columns + 2
    }

    fn stagger(&self, row: u32) -> u32 {
        (row + self.rows / 2) % 2
    }

    fn vertex(&self, index: u32) -> [f64; 3] {
        let row = index / self.stride();
        let lane = index % self.stride();
        let x = f64::from(lane) - f64::from(self.columns);
        let y = 3.0 * (f64::from(row) - f64::from(self.rows / 2));
        let drop = if lane % 2 == self.stagger(row) {
            1.0
        } else {
            2.0
        };
        [0.5 * self.width * x, 0.5 * self.radius * (y - drop), 1.0]
    }

    fn face(&self, column: u32, row: u32) -> [u32; 6] {
        let lower = row * self.stride() + 2 * column + self.stagger(row);
        let upper = lower + self.stride();
        [lower + 1, lower + 2, upper + 2, upper + 1, upper, lower]
    }
}

/// A finite patch of the horosphere h=1 in its own upper-half-space chart.
pub(super) fn moon(library: &Library, moon: &Moon) -> Result<Object<Polygon>> {
    // The +z chart axis points away from Earth.
    let pose = Pose::from_isometry(
        Hyperboloid3::shift_x(moon.ground_offset[0])
            .chain(Hyperboloid3::shift_z(moon.ground_offset[1]))
            .chain(Hyperboloid3::shift_y(moon.distance))
            .chain(Hyperboloid3::rotate_x(-FRAC_PI_2)),
    )?;

    let grid = HexGrid::new(moon.subdivisions)?;
    let vertex_count = grid.stride() * (grid.rows + 1);
    let index_count = 12 * grid.columns * grid.rows;
    let limit = library.state().device().limits().max_buffer_size;
    ensure!(
        u64::from(vertex_count) * Vertex::SIZE as u64 <= limit
            && u64::from(index_count) * size_of::<u32>() as u64 <= limit,
        "moon.subdivisions exceeds the device buffer limit"
    );
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    vertices.try_reserve_exact(vertex_count as usize)?;
    indices.try_reserve_exact(index_count as usize)?;
    vertices.extend((0..vertex_count).map(|index| (grid.vertex(index), [1.0; 3])));
    for row in 0..grid.rows {
        for column in 0..grid.columns {
            // The six corners lie on one geodesic plane. Triangulate from a
            // corner so that all four triangles retain the same face normal.
            let [a, b, c, d, e, f] = grid.face(column, row);
            indices.extend([a, b, c, a, c, d, a, d, e, a, e, f]);
        }
    }

    Ok(Object {
        shape: mesh(library, &vertices, &indices)?,
        pose,
        material: moon.material,
        domain: LightingDomain::Celestial,
    })
}

#[cfg(test)]
mod tests;
