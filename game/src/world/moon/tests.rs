use super::HexGrid;
use anyhow::Result;
use std::collections::BTreeMap;
use wgame::glam::DVec2;

#[test]
fn regular_lunar_faces_share_edges_across_the_visible_disk() -> Result<()> {
    for subdivisions in [1, 16] {
        let grid = HexGrid::new(subdivisions)?;
        let xy = |index| {
            let [u, v, h] = grid.vertex(index);
            assert_eq!(h, 1.0);
            DVec2::new(u, v)
        };
        let tolerance = 1e-11;
        let mut edges = BTreeMap::new();
        let mut contains_origin = false;
        for row in 0..grid.rows {
            for column in 0..grid.columns {
                let face = grid.face(column, row);
                let center = face.map(xy).into_iter().sum::<DVec2>() / 6.0;
                let mut encloses_origin = true;
                for (&i, &j) in face.iter().zip(face.iter().cycle().skip(1)) {
                    let a = xy(i);
                    let side = xy(j) - a;
                    // Equal chart radii at h=1 put all corners on one hemisphere
                    // orthogonal to h=0, hence in the same geodesic plane.
                    assert!((a.distance(center) - grid.radius).abs() < tolerance);
                    assert!((side.length() - grid.radius).abs() < tolerance);
                    assert!(side.perp_dot(center - a) > 0.0);
                    encloses_origin &= side.perp_dot(-a) >= -tolerance;
                    let (count, winding) = edges.entry((i.min(j), i.max(j))).or_insert((0, 0));
                    *count += 1;
                    *winding += if i < j { 1 } else { -1 };
                }
                contains_origin |= encloses_origin;
            }
        }
        assert!(contains_origin);
        let visible_radius = 1.0 + 1.0_f64.hypot(grid.radius) + grid.radius;
        for ((i, j), (count, winding)) in edges {
            if count == 2 {
                assert_eq!(winding, 0, "shared edges must have opposite winding");
            } else {
                assert_eq!(count, 1);
                let a = xy(i);
                let side = xy(j) - a;
                let t = (-a.dot(side) / side.length_squared()).clamp(0.0, 1.0);
                assert!((a + t * side).length() >= visible_radius - tolerance);
            }
        }
    }
    Ok(())
}
