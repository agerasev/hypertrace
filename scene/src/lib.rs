//! CPU-only scene descriptions and compositional WGSL generation.
//! This ABI is independent of the legacy OpenCL entity serialization.
#![forbid(unsafe_code)]

use ccgeom::Homogenous3;
use vecmat::{Complex, Transform as _, transform::Moebius};

pub type Result<T> = anyhow::Result<T>;

pub fn unsupported<T: ?Sized>() -> anyhow::Error {
    anyhow::anyhow!("{} has no WGSL implementation", std::any::type_name::<T>())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Geometry {
    Euclidean,
    Hyperbolic,
}

#[derive(Clone, Copy, Debug)]
pub enum Transform {
    Euclidean(Homogenous3<f64>),
    Hyperbolic(Moebius<Complex<f64>>),
}

impl Transform {
    pub fn geometry(&self) -> Geometry {
        match self {
            Self::Euclidean(_) => Geometry::Euclidean,
            Self::Hyperbolic(_) => Geometry::Hyperbolic,
        }
    }
    pub fn identity(geometry: Geometry) -> Self {
        match geometry {
            Geometry::Euclidean => Self::Euclidean(Homogenous3::identity()),
            Geometry::Hyperbolic => Self::Hyperbolic(Moebius::identity()),
        }
    }
    /// Compose outer(self) with inner, retaining f64 until upload.
    pub fn chain(&self, inner: &Self) -> Result<Self> {
        match (*self, *inner) {
            (Self::Euclidean(a), Self::Euclidean(b)) => Ok(Self::Euclidean(a.chain(b))),
            (Self::Hyperbolic(a), Self::Hyperbolic(b)) => Ok(Self::Hyperbolic(a.chain(b))),
            _ => anyhow::bail!("cannot compose transforms from different geometries"),
        }
    }
    pub fn rows(&self) -> Result<[[f32; 4]; 2]> {
        let rows = match *self {
            Self::Euclidean(map) => {
                let p = map.apply([0.0, 0.0, 0.0].into());
                [
                    [p[0] as f32, p[1] as f32, p[2] as f32, 0.0],
                    map.inner().into_quaternion().into_array().map(|x| x as f32),
                ]
            }
            Self::Hyperbolic(map) => {
                validate_moebius(map)?;
                let (a, b, c, d) = map.into_tuple();
                [
                    [a.re() as f32, a.im() as f32, b.re() as f32, b.im() as f32],
                    [c.re() as f32, c.im() as f32, d.re() as f32, d.im() as f32],
                ]
            }
        };
        anyhow::ensure!(
            rows.iter().flatten().all(|x| x.is_finite()),
            "transform is outside f32 range"
        );
        match self.geometry() {
            Geometry::Euclidean => anyhow::ensure!(
                (rows[1].iter().map(|&x| f64::from(x).powi(2)).sum::<f64>() - 1.0).abs() < 1e-4,
                "Euclidean rotation must be a unit quaternion"
            ),
            Geometry::Hyperbolic => {
                let [ab, cd] = rows.map(|r| r.map(f64::from));
                let det = Complex::new(ab[0], ab[1]) * Complex::new(cd[2], cd[3])
                    - Complex::new(ab[2], ab[3]) * Complex::new(cd[0], cd[1]);
                anyhow::ensure!(
                    det.re() != 0.0 || det.im() != 0.0,
                    "Möbius transform becomes singular in f32; camera-relative coordinate frames are needed"
                );
            }
        }
        Ok(rows)
    }
    pub fn words(&self) -> Result<Vec<u32>> {
        Ok(self
            .rows()?
            .into_iter()
            .flatten()
            .map(f32::to_bits)
            .collect())
    }
}

/// A complete WGSL leaf function plus any uniquely named helpers.
/// `key` identifies its implementation; parameter words have a fixed length.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShaderLeaf {
    pub key: String,
    pub source: String,
    pub entry_point: String,
    pub parameter_words: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MaterialSchema {
    Absorbing,
    Lambertian,
    Specular,
    Transparent,
    Refractive,
    Colored(Box<Self>),
    Emissive(Box<Self>),
    Mixture(Vec<Self>),
    Custom(ShaderLeaf),
}
impl MaterialSchema {
    pub fn word_len(&self) -> usize {
        match self {
            Self::Absorbing | Self::Lambertian | Self::Specular | Self::Transparent => 0,
            Self::Refractive => 1,
            Self::Colored(inner) | Self::Emissive(inner) => 3 + inner.word_len(),
            Self::Mixture(children) => {
                children.len() + children.iter().map(Self::word_len).sum::<usize>()
            }
            Self::Custom(leaf) => leaf.parameter_words as usize,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MaterialValue {
    pub schema: MaterialSchema,
    pub words: Vec<u32>,
}
impl MaterialValue {
    pub fn absorbing() -> Self {
        Self {
            schema: MaterialSchema::Absorbing,
            words: vec![],
        }
    }
    pub fn lambertian() -> Self {
        Self {
            schema: MaterialSchema::Lambertian,
            words: vec![],
        }
    }
    pub fn specular() -> Self {
        Self {
            schema: MaterialSchema::Specular,
            words: vec![],
        }
    }
    pub fn transparent() -> Self {
        Self {
            schema: MaterialSchema::Transparent,
            words: vec![],
        }
    }
    pub fn refractive(index: f64) -> Result<Self> {
        let index = finite_f32(index)?;
        anyhow::ensure!(index > 0.0, "refractive index must be positive");
        Ok(Self {
            schema: MaterialSchema::Refractive,
            words: vec![index.to_bits()],
        })
    }
    pub fn colored(self, color: [f32; 3]) -> Result<Self> {
        self.modifier(color, false)
    }
    pub fn emissive(self, emission: [f32; 3]) -> Result<Self> {
        self.modifier(emission, true)
    }
    fn modifier(self, rgb: [f32; 3], emission: bool) -> Result<Self> {
        anyhow::ensure!(
            rgb.iter().all(|&x| x.is_finite() && x >= 0.0),
            "colors must be finite and nonnegative"
        );
        let mut words: Vec<_> = rgb.into_iter().map(f32::to_bits).collect();
        words.extend(self.words);
        let schema = if emission {
            MaterialSchema::Emissive(Box::new(self.schema))
        } else {
            MaterialSchema::Colored(Box::new(self.schema))
        };
        Ok(Self { schema, words })
    }
    /// Weights precede concatenated child payloads. Each mixture draws its own
    /// random number, including nested mixtures and one-component mixtures.
    pub fn mixture(components: Vec<(f64, Self)>) -> Result<Self> {
        let mut words = Vec::new();
        let mut total = 0.0;
        for (portion, _) in &components {
            let weight = finite_f32(*portion)?;
            anyhow::ensure!(weight >= 0.0, "mixture portions must be nonnegative");
            total += *portion;
            words.push(weight.to_bits());
        }
        anyhow::ensure!(total <= 1.00001, "mixture portions exceed one");
        let mut schemas = Vec::new();
        for (_, value) in components {
            schemas.push(value.schema);
            words.extend(value.words);
        }
        Ok(Self {
            schema: MaterialSchema::Mixture(schemas),
            words,
        })
    }
    pub fn custom(leaf: ShaderLeaf, words: Vec<u32>) -> Result<Self> {
        anyhow::ensure!(
            words.len() == leaf.parameter_words as usize,
            "custom material payload length mismatch"
        );
        Ok(Self {
            schema: MaterialSchema::Custom(leaf),
            words,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ShapeSchema {
    Plane,
    Sphere,
    Cube,
    Horosphere,
    Custom(ShaderLeaf),
    Mapped {
        geometry: Geometry,
        inner: Box<Self>,
    },
    Vector(Box<Self>),
    Choice(Vec<Self>),
}
#[derive(Clone, Debug)]
pub struct ShapeValue {
    pub schema: ShapeSchema,
    pub words: Vec<u32>,
}
impl ShapeValue {
    // Even a parameterless shape owns a word, used as its self-hit identity.
    pub fn plane() -> Self {
        Self {
            schema: ShapeSchema::Plane,
            words: vec![0],
        }
    }
    pub fn sphere() -> Self {
        Self {
            schema: ShapeSchema::Sphere,
            words: vec![0],
        }
    }
    pub fn cube() -> Self {
        Self {
            schema: ShapeSchema::Cube,
            words: vec![0],
        }
    }
    pub fn horosphere() -> Self {
        Self {
            schema: ShapeSchema::Horosphere,
            words: vec![0],
        }
    }
    pub fn mapped(self, map: Transform) -> Result<Self> {
        let mut words = map.words()?;
        words.extend(self.words);
        Ok(Self {
            schema: ShapeSchema::Mapped {
                geometry: map.geometry(),
                inner: Box::new(self.schema),
            },
            words,
        })
    }
    pub fn vector(element: ShapeSchema, values: Vec<Self>) -> Result<Self> {
        anyhow::ensure!(values.len() < u32::MAX as usize, "too many shapes");
        let mut words = vec![values.len() as u32];
        words.resize(1 + values.len(), 0);
        for (index, value) in values.into_iter().enumerate() {
            anyhow::ensure!(value.schema == element, "shape vector schema mismatch");
            words[index + 1] = u32::try_from(words.len())?;
            words.extend(value.words);
        }
        Ok(Self {
            schema: ShapeSchema::Vector(Box::new(element)),
            words,
        })
    }
    pub fn choice(variants: Vec<ShapeSchema>, index: usize, value: Self) -> Result<Self> {
        anyhow::ensure!(
            variants.get(index) == Some(&value.schema),
            "shape choice schema mismatch"
        );
        let mut words = vec![u32::try_from(index)?];
        words.extend(value.words);
        Ok(Self {
            schema: ShapeSchema::Choice(variants),
            words,
        })
    }
    pub fn custom(leaf: ShaderLeaf, mut words: Vec<u32>) -> Result<Self> {
        anyhow::ensure!(
            words.len() == leaf.parameter_words as usize,
            "custom shape payload length mismatch"
        );
        if words.is_empty() {
            words.push(0);
        }
        Ok(Self {
            schema: ShapeSchema::Custom(leaf),
            words,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tiling {
    Uniform,
    Square,
    Hexagonal,
    Pentagonal,
    Pentastar,
}
impl Tiling {
    pub fn tag(self) -> u32 {
        match self {
            Self::Uniform => 0,
            Self::Square => 1,
            Self::Hexagonal => 2,
            Self::Pentagonal => 3,
            Self::Pentastar => 4,
        }
    }
}
#[derive(Clone, Debug)]
pub enum ObjectNode {
    Covered {
        shape: ShapeValue,
        material: MaterialValue,
    },
    Mapped {
        map: Transform,
        inner: Box<Self>,
    },
    Vector(Vec<Self>),
    Tiled {
        shape: ShapeValue,
        materials: Vec<MaterialValue>,
        border_material: MaterialValue,
        tiling: Tiling,
        cell_size: f64,
        border_width: f64,
    },
}
#[derive(Clone, Debug)]
pub struct View {
    pub map: Transform,
    pub fov: f64,
}
#[derive(Clone, Debug)]
pub enum Background {
    Constant([f32; 3]),
    Gradient {
        colors: [[f32; 3]; 2],
        axis: [f32; 3],
        power: f32,
    },
}
#[derive(Clone, Debug, Default)]
pub struct Registry {
    pub material_schemas: Vec<MaterialSchema>,
    pub shape_schemas: Vec<ShapeSchema>,
}
impl Registry {
    pub fn material(&mut self, schema: MaterialSchema) {
        if !self.material_schemas.contains(&schema) {
            self.material_schemas.push(schema);
        }
    }
    pub fn shape(&mut self, schema: ShapeSchema) {
        if !self.shape_schemas.contains(&schema) {
            self.shape_schemas.push(schema);
        }
    }
}
#[derive(Clone, Debug)]
pub struct SceneDefinition {
    pub view: View,
    pub background: Background,
    pub bounces: u32,
    pub object: ObjectNode,
    pub material_schemas: Vec<MaterialSchema>,
    pub shape_schemas: Vec<ShapeSchema>,
}
pub fn finite_f32(value: f64) -> Result<f32> {
    let value = value as f32;
    anyhow::ensure!(value.is_finite(), "value is outside finite f32 range");
    Ok(value)
}

/// Stored hyperbolic maps use SL(2,C), not an arbitrary projective matrix.
/// A complex scalar cannot be discarded inside the quaternion action as it
/// can for a purely complex fractional linear transformation.
pub fn validate_moebius(map: Moebius<Complex<f64>>) -> Result<()> {
    let (a, b, c, d) = map.into_tuple();
    let det = a * d - b * c;
    anyhow::ensure!(
        det.re().is_finite()
            && det.im().is_finite()
            && (det.re() - 1.0).abs() < 1e-6
            && det.im().abs() < 1e-6,
        "hyperbolic Möbius map must have complex determinant one (SL(2,C))"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reject_non_sl2c_matrices_before_quaternion_action() {
        let i = Complex::new(0.0, 1.0);
        let zero = Complex::new(0.0, 0.0);
        let map = Moebius::new(i, zero, zero, i);
        // On the complex boundary this acts as the identity; on the interior
        // its unnormalized quaternion action would reverse the height sign.
        assert!(
            Transform::Hyperbolic(map)
                .rows()
                .unwrap_err()
                .to_string()
                .contains("determinant one")
        );
    }
}

mod compiler;
pub use compiler::*;
