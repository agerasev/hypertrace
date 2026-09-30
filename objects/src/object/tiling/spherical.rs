use super::{pattern, Geometry, LibraryModule, Result, Spherical, Tiling};
use std::{convert::TryFrom, fmt::Write};

/// The regular spherical tiling `{P,Q}`: P edges per face, Q faces per vertex.
///
/// Supports `{3,3}`, `{4,3}`, `{3,4}`, `{5,3}`, `{3,5}`, and the infinite
/// hosohedron `{2,Q}` and dihedron `{P,2}` families (P,Q >= 2). A dihedron's
/// two faces are hemispheres; its degree-two vertices only subdivide their
/// common great circle and do not add visible material boundaries. Hosohedron
/// sectors must remain wider than eight f32 epsilon units across a full turn;
/// finer divisions cannot reliably resolve their boundaries in shader arithmetic.
///
/// Width is the geodesic border half-width in radians on the intrinsic unit
/// sphere, independent of the surface's physical radius. Face colours follow
/// stable face indices modulo the supplied material count.
#[derive(Clone, Copy, Debug)]
pub struct RegularSpherical<const P: usize, const Q: usize> {
    pub border_width: f64,
}
impl<const P: usize, const Q: usize> RegularSpherical<P, Q> {
    pub fn new(border_width: f64) -> Self {
        Self { border_width }
    }
}
impl<const P: usize, const Q: usize> Tiling<Spherical> for RegularSpherical<P, Q> {
    fn shader<G: Geometry>() -> Result<LibraryModule<G>> {
        anyhow::ensure!(
            P >= 2 && Q >= 2,
            "a spherical face and vertex need degree at least two"
        );
        let source = if Q == 2 {
            // Every dihedron has two hemispherical faces, regardless of how
            // many degree-two vertices subdivide its equator.
            "fn {{self}}(position:vec4<f32>,cell:f32,width:f32,count:u32)->u32 {
                if abs(position.z)<=sin(width) {return count;}
                return select(0u,1u,position.z<0)%count;
            }"
            .to_owned()
        } else if P == 2 {
            let n = u32::try_from(Q)?;
            let step = std::f32::consts::TAU / n as f32;
            anyhow::ensure!(
                step > 8.0 * f32::EPSILON * std::f32::consts::TAU,
                "spherical lune width is outside the f32 angular resolver range"
            );
            format!(
                "fn {{{{self}}}}(position:vec4<f32>,cell:f32,width:f32,count:u32)->u32 {{
                if all(position.xy==vec2<f32>(0)) {{return count;}}
                let longitude=atan2(position.y,position.x)+PI;
                let step=2*PI/{n}.0;
                let face=min(u32(floor(longitude/step)),{last}u);
                let left=-PI+f32(face)*step;
                let right=left+step;
                let edge0=abs(dot(position.xy,vec2<f32>(-sin(left),cos(left))));
                let edge1=abs(dot(position.xy,vec2<f32>(-sin(right),cos(right))));
                if min(edge0,edge1)<=sin(width) {{return count;}}
                return face%count;
            }}",
                last = n - 1
            )
        } else {
            let centres = face_centres::<P, Q>()?;
            let mut source = format!(
                "const {{{{self}}}}_centres=array<vec3<f32>,{}>(\n",
                centres.len()
            );
            for centre in &centres {
                let norm = centre.iter().map(|v| v * v).sum::<f64>().sqrt();
                writeln!(
                    source,
                    "vec3<f32>({:.9},{:.9},{:.9}),",
                    centre[0] / norm,
                    centre[1] / norm,
                    centre[2] / norm
                )?;
            }
            source.push_str(
                ");\nfn {{self}}(position:vec4<f32>,cell:f32,width:f32,count:u32)->u32 {\n",
            );
            source.push_str("let direction=position.xyz;\nvar face=0u;\nvar nearest=-2.0;\n");
            writeln!(source, "for(var i=0u;i<{}u;i+=1u) {{", centres.len())?;
            source.push_str("let score=dot(direction,{{self}}_centres[i]);\nif score>nearest {nearest=score;face=i;}\n}\n");
            source.push_str("let centre={{self}}_centres[face];\nlet border=sin(width);\n");
            writeln!(source, "for(var i=0u;i<{}u;i+=1u) {{", centres.len())?;
            source.push_str("if i==face {continue;}\nlet normal=normalize(centre-{{self}}_centres[i]);\nif dot(direction,normal)<=border {return count;}\n}\nreturn face%count;\n}\n");
            source
        };
        let mut module =
            pattern::<G, false>(&format!("hypertrace.tiling.spherical.{P}.{Q}"), &source);
        module.validate_words = |_, _, words| {
            super::validate_width(words)?;
            anyhow::ensure!(
                f32::from_bits(words[1]) <= std::f32::consts::FRAC_PI_2,
                "spherical border width must not exceed pi/2 radians"
            );
            Ok(())
        };
        Ok(module)
    }
    fn parameters(&self) -> (f64, f64) {
        (1.0, self.border_width)
    }
}

// A regular polyhedron's face centres are the vertices of its dual. Their
// spherical Voronoi regions are the required regular faces. Great-circle
// bisectors give angular border distances without a latitude/longitude seam.
fn face_centres<const P: usize, const Q: usize>() -> Result<Vec<[f64; 3]>> {
    let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
    match (P, Q) {
        (3, 3) => Ok(vec![
            [1.0, 1.0, 1.0],
            [1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
        ]),
        (4, 3) => Ok(vec![
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ]),
        (3, 4) => Ok(cube_vertices()),
        (5, 3) => Ok(golden_vertices(1.0, phi)),
        (3, 5) => {
            let mut centres = cube_vertices();
            centres.extend(golden_vertices(1.0 / phi, phi));
            Ok(centres)
        }
        _ => anyhow::bail!(
            "unsupported spherical regular tiling {{{P},{Q}}}: require 1/P + 1/Q > 1/2"
        ),
    }
}
fn cube_vertices() -> Vec<[f64; 3]> {
    let mut result = Vec::new();
    for x in [-1.0, 1.0] {
        for y in [-1.0, 1.0] {
            for z in [-1.0, 1.0] {
                result.push([x, y, z]);
            }
        }
    }
    result
}
fn golden_vertices(a: f64, b: f64) -> Vec<[f64; 3]> {
    let mut result = Vec::new();
    for x in [-a, a] {
        for y in [-b, b] {
            result.extend([[0.0, x, y], [x, y, 0.0], [y, 0.0, x]]);
        }
    }
    result
}
