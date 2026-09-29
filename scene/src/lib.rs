//! CPU-only scene descriptions and compositional WGSL generation.
//! Explicit storage records and parameter words define the GPU ABI.
#![forbid(unsafe_code)]

use vecmat::{Complex, transform::Moebius};

pub type Result<T> = anyhow::Result<T>;

pub fn unsupported<T: ?Sized>() -> anyhow::Error {
    anyhow::anyhow!("{} has no WGSL implementation", std::any::type_name::<T>())
}

mod transform;
pub use transform::{Geometry, Transform, validate_embedded_rows};

/// Geometry-dependent validation context, in physical world units.
#[derive(Clone, Copy, Debug)]
pub struct GeometryContext {
    pub geometry: Geometry,
    pub radius: f32,
}

/// The calling convention of a linked module. Libraries provide shared helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShaderKind {
    Shape,
    Material,
    Library,
}

/// A component-owned WGSL implementation, independent of instance values.
///
/// `{{self}}` expands to this module's unique function prefix and `{{dep0}}`
/// to its first dependency's prefix, etc. Helpers use `{{self}}_helper` names.
/// Keys identify implementations, including their structural specialization;
/// the linker rejects conflicting definitions and recursive dependencies.
/// Callbacks belong to the component, so compilation needs no built-in catalogue.
#[derive(Clone, Debug)]
pub struct ShaderModule {
    pub key: String,
    pub source: String,
    pub kind: ShaderKind,
    /// Fixed parameter size, or None for a component-defined variable layout.
    pub parameter_words: Option<u32>,
    pub dependencies: Vec<Self>,
    pub validate_context: fn(GeometryContext) -> Result<()>,
    pub validate_words: fn(&Self, GeometryContext, &[u32]) -> Result<()>,
}

impl ShaderModule {
    pub fn new(
        key: impl Into<String>,
        kind: ShaderKind,
        source: impl Into<String>,
        parameter_words: Option<u32>,
    ) -> Self {
        Self {
            key: key.into(),
            source: source.into(),
            kind,
            parameter_words,
            dependencies: Vec::new(),
            validate_context: |_| Ok(()),
            validate_words: |_, _, _| Ok(()),
        }
    }
    /// An unambiguous structural key for a combinator's ordered dependencies.
    pub fn specialized_key(template: &str, dependencies: &[Self]) -> String {
        let mut key = format!("{}:{template}[", template.len());
        for child in dependencies {
            key.push_str(&format!("{}:{};", child.key.len(), child.key));
        }
        key.push(']');
        key
    }
    pub fn validate_geometry(&self, context: GeometryContext) -> Result<()> {
        (self.validate_context)(context)
    }
    pub fn validate(&self, context: GeometryContext, words: &[u32]) -> Result<()> {
        anyhow::ensure!(
            self.kind != ShaderKind::Shape || !words.is_empty(),
            "shapes must reserve an identity word"
        );
        self.validate_geometry(context)?;
        self.validate_length(words)?;
        (self.validate_words)(self, context, words)
    }
    pub fn validate_length(&self, words: &[u32]) -> Result<()> {
        if let Some(length) = self.parameter_words {
            anyhow::ensure!(
                words.len() == length as usize,
                "module {} parameter length mismatch: expected {}, got {}",
                self.key,
                length,
                words.len()
            );
        }
        Ok(())
    }
    /// Compare shader structure, never parameter values or callback addresses.
    pub fn same_implementation(&self, other: &Self) -> bool {
        self.key == other.key
            && self.kind == other.kind
            && self.source == other.source
            && self.parameter_words == other.parameter_words
            && self.dependencies.len() == other.dependencies.len()
            && self
                .dependencies
                .iter()
                .zip(&other.dependencies)
                .all(|(a, b)| a.same_implementation(b))
    }
}

#[derive(Clone, Debug)]
pub struct ShapeValue {
    pub schema: ShaderModule,
    pub words: Vec<u32>,
}
impl ShapeValue {
    pub fn new(schema: ShaderModule, words: Vec<u32>) -> Result<Self> {
        anyhow::ensure!(schema.kind == ShaderKind::Shape, "expected a shape module");
        anyhow::ensure!(!words.is_empty(), "shapes must reserve an identity word");
        schema.validate_length(&words)?;
        Ok(Self { schema, words })
    }
}

#[derive(Clone, Debug)]
pub struct MaterialValue {
    pub schema: ShaderModule,
    pub words: Vec<u32>,
}
impl MaterialValue {
    pub fn new(schema: ShaderModule, words: Vec<u32>) -> Result<Self> {
        anyhow::ensure!(
            schema.kind == ShaderKind::Material,
            "expected a material module"
        );
        schema.validate_length(&words)?;
        Ok(Self { schema, words })
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
#[derive(Clone, Debug)]
pub struct SceneDefinition {
    pub view: View,
    pub background: Background,
    pub bounces: u32,
    /// Curvature radius in physical world units; Euclidean space requires one.
    pub radius: f64,
    pub medium: Medium,
    pub object: ObjectNode,
    /// Type dependencies, including empty vectors and inactive choice variants.
    pub modules: Vec<ShaderModule>,
}

/// Homogeneous analog transport: scalar extinction per world unit and RGB
/// scattering albedo. Zero extinction is vacuum; albedo zero is pure absorption.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Medium {
    #[default]
    Vacuum,
    Homogeneous {
        extinction: f32,
        albedo: [f32; 3],
    },
}
impl Medium {
    pub fn validate(self) -> Result<()> {
        if let Self::Homogeneous { extinction, albedo } = self {
            anyhow::ensure!(
                extinction.is_finite() && extinction >= 0.0,
                "medium extinction must be finite and nonnegative"
            );
            anyhow::ensure!(
                albedo
                    .iter()
                    .all(|v| v.is_finite() && (0.0..=1.0).contains(v)),
                "medium scattering albedo must be between zero and one"
            );
        }
        Ok(())
    }
    /// Validate the maximum sampled physical flight and normalized phase for
    /// the shader's open 23-bit random samples (maximum optical depth 24 ln 2).
    /// This guards overflow; it does not certify long-path spatial precision.
    pub fn validate_for_radius(self, radius: f32) -> Result<()> {
        self.validate()?;
        anyhow::ensure!(
            radius.is_finite() && radius > 0.0,
            "invalid medium curvature radius"
        );
        if let Self::Homogeneous { extinction, .. } = self
            && extinction > 0.0
        {
            let maximum_flight = 24.0 * std::f64::consts::LN_2 / f64::from(extinction);
            anyhow::ensure!(
                (maximum_flight as f32).is_finite(),
                "medium free-flight distance is outside f32 range"
            );
            anyhow::ensure!(
                ((maximum_flight / f64::from(radius)) as f32).is_finite(),
                "medium free-flight phase is outside f32 range"
            );
        }
        Ok(())
    }
    pub fn gpu_row(self) -> [f32; 4] {
        match self {
            Self::Vacuum => [0.0; 4],
            Self::Homogeneous {
                extinction,
                albedo: [r, g, b],
            } => [r, g, b, extinction],
        }
    }
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
