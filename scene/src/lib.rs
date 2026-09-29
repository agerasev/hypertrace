//! CPU-only, statically typed scene descriptions and compositional shader generation.
//! Explicit storage records and parameter words define the GPU ABI.
#![forbid(unsafe_code)]

use std::marker::PhantomData;
pub type Result<T> = anyhow::Result<T>;

mod transform;
pub use transform::{Geometry, Transform, validate_embedded_rows};

/// Geometry-dependent validation context, in physical world units.
#[derive(Clone, Copy, Debug)]
pub struct GeometryContext<G: Geometry> {
    pub radius: f32,
    geometry: PhantomData<G>,
}
impl<G: Geometry> GeometryContext<G> {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            geometry: PhantomData,
        }
    }
}

mod role {
    pub trait Sealed {}
}
/// A shader calling convention, selected by its Rust type.
pub trait ShaderRole: role::Sealed + Clone + std::fmt::Debug {
    #[doc(hidden)]
    const MINIMUM_WORDS: usize;
}
#[derive(Clone, Copy, Debug)]
pub struct ShapeRole;
#[derive(Clone, Copy, Debug)]
pub struct MaterialRole;
#[derive(Clone, Copy, Debug)]
pub struct LibraryRole;
impl role::Sealed for ShapeRole {}
impl role::Sealed for MaterialRole {}
impl role::Sealed for LibraryRole {}
impl ShaderRole for ShapeRole {
    const MINIMUM_WORDS: usize = 1;
}
impl ShaderRole for MaterialRole {
    const MINIMUM_WORDS: usize = 0;
}
impl ShaderRole for LibraryRole {
    const MINIMUM_WORDS: usize = 0;
}

/// Component-owned parameter validation with statically selected geometry.
pub type WordValidator<G> = fn(&SourceModule<G>, GeometryContext<G>, &[u32]) -> Result<()>;

/// Kindless source and validation data consumed by the shader linker.
///
/// Dependencies are link data: their calling conventions have already been
/// checked by the typed component constructors that compose them.
#[derive(Clone, Debug)]
pub struct SourceModule<G: Geometry> {
    pub key: String,
    pub source: String,
    /// Fixed parameter size, or None for a component-defined variable layout.
    pub parameter_words: Option<u32>,
    pub dependencies: Vec<Self>,
    pub validate_context: fn(GeometryContext<G>) -> Result<()>,
    pub validate_words: WordValidator<G>,
    minimum_words: usize,
}
impl<G: Geometry> SourceModule<G> {
    /// An unambiguous structural key for a combinator's ordered dependencies.
    pub fn specialized_key(template: &str, dependencies: &[Self]) -> String {
        let mut key = format!("{}:{template}[", template.len());
        for child in dependencies {
            key.push_str(&format!("{}:{};", child.key.len(), child.key));
        }
        key.push(']');
        key
    }
    pub fn validate_geometry(&self, context: GeometryContext<G>) -> Result<()> {
        (self.validate_context)(context)
    }
    pub fn validate(&self, context: GeometryContext<G>, words: &[u32]) -> Result<()> {
        self.validate_length(words)?;
        self.validate_geometry(context)?;
        (self.validate_words)(self, context, words)
    }
    pub fn validate_length(&self, words: &[u32]) -> Result<()> {
        anyhow::ensure!(
            words.len() >= self.minimum_words,
            "module {} must reserve an identity word",
            self.key
        );
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
            && self.source == other.source
            && self.parameter_words == other.parameter_words
            && self.minimum_words == other.minimum_words
            && self.dependencies.len() == other.dependencies.len()
            && self
                .dependencies
                .iter()
                .zip(&other.dependencies)
                .all(|(a, b)| a.same_implementation(b))
    }
}

/// A component-owned shader implementation, independent of instance values.
///
/// `{{self}}` expands to its unique function prefix and `{{dep0}}` to its first
/// dependency's prefix. Private helpers use `{{self}}_helper` names. Structural
/// keys and dependencies select shader source; payload values stay in buffers.
///
/// Calling conventions cannot be interchanged:
/// ```compile_fail
/// use hypertrace_scene::{MaterialModule, ShapeValue};
/// use ccgeom::Flat3;
/// let material = MaterialModule::<Flat3>::new("material", "fn {{self}}() {}", Some(1));
/// let _ = ShapeValue::new(material, vec![0]);
/// ```
/// Geometry is also preserved through dependencies:
/// ```compile_fail
/// use hypertrace_scene::{LibraryModule, ShapeModule};
/// use ccgeom::{Flat3, Spherical3};
/// let library = LibraryModule::<Spherical3>::new("library", "fn {{self}}() {}", None);
/// let mut shape = ShapeModule::<Flat3>::new("shape", "fn {{self}}() {}", Some(1));
/// shape.dependencies.push(library.into_source());
/// ```
#[derive(Clone, Debug)]
pub struct ShaderModule<G: Geometry, R: ShaderRole> {
    pub key: String,
    pub source: String,
    pub parameter_words: Option<u32>,
    pub dependencies: Vec<SourceModule<G>>,
    pub validate_context: fn(GeometryContext<G>) -> Result<()>,
    pub validate_words: WordValidator<G>,
    role: PhantomData<R>,
}
pub type ShapeModule<G> = ShaderModule<G, ShapeRole>;
pub type MaterialModule<G> = ShaderModule<G, MaterialRole>;
pub type LibraryModule<G> = ShaderModule<G, LibraryRole>;
impl<G: Geometry, R: ShaderRole> ShaderModule<G, R> {
    pub fn new(
        key: impl Into<String>,
        source: impl Into<String>,
        parameter_words: Option<u32>,
    ) -> Self {
        Self {
            key: key.into(),
            source: source.into(),
            parameter_words,
            dependencies: Vec::new(),
            validate_context: |_| Ok(()),
            validate_words: |_, _, _| Ok(()),
            role: PhantomData,
        }
    }
    pub fn specialized_key(template: &str, dependencies: &[SourceModule<G>]) -> String {
        SourceModule::specialized_key(template, dependencies)
    }
    pub fn dependency(&self) -> SourceModule<G> {
        self.clone().into_source()
    }
    pub fn into_source(self) -> SourceModule<G> {
        SourceModule {
            key: self.key,
            source: self.source,
            parameter_words: self.parameter_words,
            dependencies: self.dependencies,
            validate_context: self.validate_context,
            validate_words: self.validate_words,
            minimum_words: R::MINIMUM_WORDS,
        }
    }
    pub fn validate_geometry(&self, context: GeometryContext<G>) -> Result<()> {
        (self.validate_context)(context)
    }
    pub fn validate_length(&self, words: &[u32]) -> Result<()> {
        anyhow::ensure!(
            words.len() >= R::MINIMUM_WORDS,
            "module {} must reserve an identity word",
            self.key
        );
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
    pub fn validate(&self, context: GeometryContext<G>, words: &[u32]) -> Result<()> {
        self.validate_length(words)?;
        self.validate_geometry(context)?;
        (self.validate_words)(&self.dependency(), context, words)
    }
    pub fn same_implementation(&self, other: &Self) -> bool {
        self.dependency().same_implementation(&other.dependency())
    }
}

/// Typed shader roots, including dependencies of empty collections.
#[derive(Clone, Debug)]
pub struct Modules<G: Geometry> {
    pub shapes: Vec<ShapeModule<G>>,
    pub materials: Vec<MaterialModule<G>>,
    pub libraries: Vec<LibraryModule<G>>,
}
impl<G: Geometry> Default for Modules<G> {
    fn default() -> Self {
        Self {
            shapes: vec![],
            materials: vec![],
            libraries: vec![],
        }
    }
}
impl<G: Geometry> Modules<G> {
    pub fn extend(&mut self, other: Self) {
        self.shapes.extend(other.shapes);
        self.materials.extend(other.materials);
        self.libraries.extend(other.libraries);
    }
}

#[derive(Clone, Debug)]
pub struct ShapeValue<G: Geometry> {
    pub schema: ShapeModule<G>,
    pub words: Vec<u32>,
}
impl<G: Geometry> ShapeValue<G> {
    pub fn new(schema: ShapeModule<G>, words: Vec<u32>) -> Result<Self> {
        schema.validate_length(&words)?;
        Ok(Self { schema, words })
    }
}
#[derive(Clone, Debug)]
pub struct MaterialValue<G: Geometry> {
    pub schema: MaterialModule<G>,
    pub words: Vec<u32>,
}
impl<G: Geometry> MaterialValue<G> {
    pub fn new(schema: MaterialModule<G>, words: Vec<u32>) -> Result<Self> {
        schema.validate_length(&words)?;
        Ok(Self { schema, words })
    }
}

/// A flattened instance produced by statically typed object composition.
#[derive(Clone, Debug)]
pub struct EncodedObject<G: Geometry> {
    pub map: Transform<G>,
    pub shape: ShapeValue<G>,
    pub material: MaterialValue<G>,
}
#[derive(Clone, Debug)]
pub struct View<G: Geometry> {
    pub map: Transform<G>,
    pub fov: f64,
}
/// Background interpolation data. Constant backgrounds use equal endpoint colors.
#[derive(Clone, Debug)]
pub struct Background<G: Geometry> {
    colors: [[f32; 3]; 2],
    axis: [f32; 3],
    power: f32,
    geometry: PhantomData<G>,
}
impl<G: Geometry> Background<G> {
    pub fn constant(color: [f32; 3]) -> Self {
        Self {
            colors: [color; 2],
            axis: [0.0; 3],
            power: 1.0,
            geometry: PhantomData,
        }
    }
    pub fn colors(&self) -> [[f32; 3]; 2] {
        self.colors
    }
    pub fn axis(&self) -> [f32; 3] {
        self.axis
    }
    pub fn power(&self) -> f32 {
        self.power
    }
    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(
            self.colors
                .iter()
                .flatten()
                .all(|v| v.is_finite() && *v >= 0.0),
            "background colors must be finite and nonnegative"
        );
        anyhow::ensure!(
            self.axis.iter().all(|v| v.is_finite()) && self.power.is_finite() && self.power > 0.0,
            "invalid background gradient"
        );
        Ok(())
    }
}
impl Background<ccgeom::Flat3<f64>> {
    pub fn gradient(colors: [[f32; 3]; 2], axis: [f32; 3], power: f32) -> Self {
        Self {
            colors,
            axis,
            power,
            geometry: PhantomData,
        }
    }
}
#[derive(Clone, Debug)]
pub struct SceneDefinition<G: Geometry> {
    pub view: View<G>,
    pub background: Background<G>,
    pub bounces: u32,
    /// Curvature radius in physical world units; Euclidean space requires one.
    pub radius: f64,
    pub medium: Medium,
    pub objects: Vec<EncodedObject<G>>,
    pub modules: Modules<G>,
}

/// Homogeneous analog transport: scalar extinction per world unit and RGB
/// scattering albedo. Zero extinction is vacuum; albedo zero is pure absorption.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Medium {
    pub extinction: f32,
    pub albedo: [f32; 3],
}
impl Medium {
    pub fn vacuum() -> Self {
        Self::default()
    }
    pub fn homogeneous(extinction: f32, albedo: [f32; 3]) -> Self {
        Self { extinction, albedo }
    }
    pub fn validate(self) -> Result<()> {
        anyhow::ensure!(
            self.extinction.is_finite() && self.extinction >= 0.0,
            "medium extinction must be finite and nonnegative"
        );
        anyhow::ensure!(
            self.albedo
                .iter()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v)),
            "medium scattering albedo must be between zero and one"
        );
        Ok(())
    }
    /// Guard overflow for the shader's open 23-bit random samples, whose maximum
    /// optical depth is 24 ln 2. This does not certify long-path spatial precision.
    pub fn validate_for_radius(self, radius: f32) -> Result<()> {
        self.validate()?;
        anyhow::ensure!(
            radius.is_finite() && radius > 0.0,
            "invalid medium curvature radius"
        );
        if self.extinction > 0.0 {
            let maximum_flight = 24.0 * std::f64::consts::LN_2 / f64::from(self.extinction);
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
        [
            self.albedo[0],
            self.albedo[1],
            self.albedo[2],
            self.extinction,
        ]
    }
}
pub fn finite_f32(value: f64) -> Result<f32> {
    let value = value as f32;
    anyhow::ensure!(value.is_finite(), "value is outside finite f32 range");
    Ok(value)
}

mod compiler;
pub use compiler::*;
