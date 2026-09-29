//! Link component-owned shader modules and pack instance values for the GPU.
use crate::{
    EncodedObject, Geometry, GeometryContext, MaterialValue, Result, SceneDefinition, ShapeValue,
    SourceModule, Transform, finite_f32,
};
use anyhow::{Context as _, ensure};
use bytemuck::{Pod, Zeroable};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// Object transforms remain in the material's coordinate frame. Mapped shapes
/// are generated independently and transform their hit back into that frame.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GpuObject {
    pub map0: [f32; 4],
    pub map1: [f32; 4],
    /// Shape function, material record, shape word offset, reserved.
    pub info: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MaterialRecord {
    /// Material function, word offset, reserved, reserved.
    pub data: [u32; 4],
}

#[derive(Clone, Debug)]
pub struct CompiledScene<G: Geometry> {
    /// Validated physical radius used by primitive compilation.
    pub radius: f32,
    /// Canonical f64 isometries; GPU rows are prepared relative to the camera.
    pub transforms: Vec<Transform<G>>,
    pub objects: Vec<GpuObject>,
    pub materials: Vec<MaterialRecord>,
    pub words: Vec<u32>,
    /// Functions only. Concatenate after the shared math and generated runtime.
    /// Values, vector lengths and object counts do not occur in this string.
    pub source: String,
}

/// A linked module graph. All identifiers and ordering depend only on structure.
struct Linker<G: Geometry> {
    modules: BTreeMap<String, SourceModule<G>>,
    visiting: BTreeSet<String>,
    shapes: BTreeSet<String>,
    materials: BTreeSet<String>,
}
impl<G: Geometry> Linker<G> {
    fn collect(&mut self, module: &SourceModule<G>, context: GeometryContext<G>) -> Result<()> {
        ensure!(
            !module.key.is_empty(),
            "shader module key must not be empty"
        );
        ensure!(
            !self.visiting.contains(&module.key),
            "shader dependency cycle at {}",
            module.key
        );
        module
            .validate_geometry(context)
            .with_context(|| format!("shader module {}", module.key))?;
        if let Some(previous) = self.modules.get(&module.key) {
            ensure!(
                previous.same_implementation(module),
                "shader module key has conflicting implementations: {}",
                module.key
            );
        }
        // Equal shader structure does not imply equal validation callbacks.
        // Validate every dependency instance, including those of a duplicate
        // parent, so acceptance cannot depend on object/module traversal order.
        self.visiting.insert(module.key.clone());
        for child in &module.dependencies {
            self.collect(child, context)?;
        }
        self.visiting.remove(&module.key);
        self.modules
            .entry(module.key.clone())
            .or_insert_with(|| module.clone());
        Ok(())
    }
    fn object(&mut self, object: &EncodedObject<G>, context: GeometryContext<G>) -> Result<()> {
        self.collect(&object.shape.schema.dependency(), context)?;
        self.shapes.insert(object.shape.schema.key.clone());
        self.collect(&object.material.schema.dependency(), context)?;
        self.materials.insert(object.material.schema.key.clone());
        Ok(())
    }
    fn link(&self) -> Result<(String, BTreeMap<String, u32>)> {
        let ids = self
            .modules
            .keys()
            .enumerate()
            .map(|(i, key)| Ok((key.clone(), u32::try_from(i)?)))
            .collect::<Result<BTreeMap<_, _>>>()?;
        let mut source = format!(
            "// Component shader interface: embedded geometry.\nconst GEO_K:f32 = {}.0;\n",
            G::SIGN
        );
        for (key, module) in &self.modules {
            let prefix = format!("ht_module_{}", ids[key]);
            ensure!(
                module.source.contains("{{self}}"),
                "module {} source must use its {{{{self}}}} namespace",
                key
            );
            let mut body = module.source.replace("{{self}}", &prefix);
            for (i, dep) in module.dependencies.iter().enumerate() {
                body = body.replace(
                    &format!("{{{{dep{i}}}}}"),
                    &format!("ht_module_{}", ids[&dep.key]),
                );
            }
            ensure!(
                !body.contains("{{"),
                "module {} has an unresolved shader placeholder",
                key
            );
            // Debug formatting escapes newlines in user keys, preserving this comment.
            writeln!(source, "// module {key:?}\n{body}")?;
        }
        source.push_str("fn ht_shape_dispatch(kind:u32,base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {\nswitch kind {\n");
        for key in &self.shapes {
            {
                let id = ids[key];
                writeln!(
                    source,
                    "case {id}u: {{return ht_module_{id}(base,ray,previous_identity);}}"
                )?;
            }
        }
        source.push_str("default: {return GeoTaggedHit(geo_miss(),0xffffffffu);}\n}}\n");
        source.push_str("fn ht_material_dispatch(kind:u32,base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {\nswitch kind {\n");
        for key in &self.materials {
            {
                let id = ids[key];
                writeln!(
                    source,
                    "case {id}u: {{ht_module_{id}(base,ctx,sample,rng);}}"
                )?;
            }
        }
        source.push_str("default: {(*sample).alive=0u;}\n}}\n");
        Ok((source, ids))
    }
}

/// Compile type dependencies and current values without a device or global registry.
pub fn compile<G: Geometry>(scene: &SceneDefinition<G>) -> Result<CompiledScene<G>> {
    let radius = validate_scene_parameters(scene)?;
    let context = GeometryContext::new(radius);
    let mut linker = Linker {
        modules: BTreeMap::new(),
        visiting: BTreeSet::new(),
        shapes: BTreeSet::new(),
        materials: BTreeSet::new(),
    };
    for module in &scene.modules.shapes {
        linker.collect(&module.dependency(), context)?;
        linker.shapes.insert(module.key.clone());
    }
    for module in &scene.modules.materials {
        linker.collect(&module.dependency(), context)?;
        linker.materials.insert(module.key.clone());
    }
    for module in &scene.modules.libraries {
        linker.collect(&module.dependency(), context)?;
    }
    for object in &scene.objects {
        linker.object(object, context)?;
    }
    let (source, ids) = linker.link()?;
    let mut compiler = Compiler {
        context,
        ids,
        result: CompiledScene {
            radius,
            transforms: vec![],
            objects: vec![],
            materials: vec![],
            words: vec![],
            source,
        },
    };
    for object in &scene.objects {
        compiler.object(object)?;
    }
    let inverse_camera = scene.view.map.inverse()?;
    for map in &compiler.result.transforms {
        inverse_camera
            .chain(map)?
            .rows()
            .context("camera-relative object transform cannot be represented safely")?;
    }
    if compiler.result.words.is_empty() {
        compiler.result.words.push(0);
    }
    Ok(compiler.result)
}

fn validate_scene_parameters<G: Geometry>(scene: &SceneDefinition<G>) -> Result<f32> {
    let radius = finite_f32(scene.radius)?;
    ensure!(
        radius.is_normal() && radius > 0.0 && (G::SIGN != 0 || scene.radius == 1.0),
        "curvature radius must be positive and normal in f32; Euclidean radius must be one"
    );
    if G::SIGN == 1 {
        ensure!(
            (radius * std::f32::consts::TAU).is_finite(),
            "spherical circumference is outside finite f32 range"
        );
    }
    ensure!(
        finite_f32(scene.view.fov)? > 0.0,
        "camera field of view must be positive"
    );
    scene.medium.validate_for_radius(radius)?;
    scene.background.validate()?;
    scene.view.map.components()?;
    Ok(radius)
}

struct Compiler<G: Geometry> {
    context: GeometryContext<G>,
    ids: BTreeMap<String, u32>,
    result: CompiledScene<G>,
}
impl<G: Geometry> Compiler<G> {
    fn words(&mut self, words: &[u32]) -> Result<u32> {
        let base = u32::try_from(self.result.words.len())?;
        let length = self
            .result
            .words
            .len()
            .checked_add(words.len())
            .context("scene word count overflow")?;
        ensure!(
            length < u32::MAX as usize,
            "scene word arena exceeds u32 addressing"
        );
        self.result.words.extend_from_slice(words);
        Ok(base)
    }
    fn material(&mut self, value: &MaterialValue<G>) -> Result<u32> {
        value
            .schema
            .validate(self.context, &value.words)
            .with_context(|| format!("material {}", value.schema.key))?;
        let index = u32::try_from(self.result.materials.len())?;
        let base = self.words(&value.words)?;
        self.result.materials.push(MaterialRecord {
            data: [self.ids[&value.schema.key], base, 0, 0],
        });
        Ok(index)
    }
    fn object(&mut self, object: &EncodedObject<G>) -> Result<()> {
        let material = self.material(&object.material)?;
        self.push_object(&object.shape, object.map, material)
    }
    fn push_object(
        &mut self,
        shape: &ShapeValue<G>,
        map: Transform<G>,
        material: u32,
    ) -> Result<()> {
        ensure!(
            !shape.words.is_empty(),
            "shapes must reserve an identity word"
        );
        shape
            .schema
            .validate(self.context, &shape.words)
            .with_context(|| format!("shape {}", shape.schema.key))?;
        ensure!(
            self.result.objects.len() < u32::MAX as usize,
            "too many objects"
        );
        let base = self.words(&shape.words)?;
        map.components()?;
        self.result.transforms.push(map);
        let [map0, map1] = Transform::<G>::identity().rows()?;
        self.result.objects.push(GpuObject {
            map0,
            map1,
            info: [self.ids[&shape.schema.key], material, base, 0],
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Background, LibraryModule, MaterialModule, Modules, ShapeModule, View};
    use ccgeom::{Flat3, Hyperboloid3, Spherical3};
    fn shape<G: Geometry>() -> ShapeValue<G> {
        ShapeValue::new(ShapeModule::new("test.shape",
            "fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit {return GeoTaggedHit(geo_miss(),base);}", Some(1)), vec![0]).unwrap()
    }
    fn material<G: Geometry>() -> MaterialValue<G> {
        MaterialValue::new(MaterialModule::new("test.material",
            "fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {(*sample).alive=0u;}", Some(1)), vec![0]).unwrap()
    }
    fn scene<G: Geometry>() -> SceneDefinition<G> {
        SceneDefinition {
            view: View {
                map: Transform::identity(),
                fov: 1.0,
            },
            background: Background::constant([0.0; 3]),
            bounces: 4,
            radius: 1.0,
            medium: Default::default(),
            objects: vec![EncodedObject {
                map: Transform::identity(),
                shape: shape(),
                material: material(),
            }],
            modules: Modules::default(),
        }
    }
    #[test]
    fn storage_records_have_exact_word_layout() {
        assert_eq!(std::mem::size_of::<GpuObject>(), 48);
        assert_eq!(std::mem::size_of::<MaterialRecord>(), 16);
        assert_eq!(std::mem::offset_of!(GpuObject, info), 32);
    }
    #[test]
    fn linking_is_deterministic_and_values_are_buffer_only() -> Result<()> {
        let a = scene::<Flat3>();
        let mut b = a.clone();
        let EncodedObject {
            shape, material, ..
        } = &mut b.objects[0];
        shape.words[0] = 42;
        material.words[0] = 91;
        b.modules.materials.push(material.schema.clone());
        b.modules.shapes.push(shape.schema.clone());
        let x = compile(&a)?;
        let y = compile(&b)?;
        assert_eq!(x.source, y.source);
        assert_ne!(x.words, y.words);
        b.modules.materials.reverse();
        b.modules.shapes.reverse();
        assert_eq!(x.source, compile(&b)?.source);
        Ok(())
    }
    #[test]
    fn dependencies_are_namespaced_and_conflicts_cycles_missing_imports_fail() -> Result<()> {
        let mut a = scene::<Flat3>();
        let mut helper =
            LibraryModule::new("helpers.constant", "fn {{self}}()->f32{return 1.0;}", None);
        let mut root = LibraryModule::new(
            "helpers.user",
            "fn {{self}}()->f32{return {{dep0}}();}",
            None,
        );
        root.dependencies.push(helper.dependency());
        a.modules.libraries.push(root.clone());
        let source = compile(&a)?.source;
        assert!(source.contains("// module \"helpers.constant\""));
        assert!(!source.contains("{{"));
        helper.source.push_str("// changed");
        a.modules.libraries.push(helper);
        assert!(format!("{:#}", compile(&a).unwrap_err()).contains("conflicting implementations"));
        a.modules.libraries = vec![root.clone()];
        a.modules.libraries[0].dependencies[0]
            .dependencies
            .push(root.into_source());
        assert!(format!("{:#}", compile(&a).unwrap_err()).contains("dependency cycle"));
        a.modules.libraries = vec![LibraryModule::new(
            "bad",
            "fn {{self}}()->f32{return {{dep0}}();}",
            None,
        )];
        assert!(format!("{:#}", compile(&a).unwrap_err()).contains("unresolved"));
        Ok(())
    }
    #[test]
    fn duplicate_parent_dependencies_are_validated_independently_of_order() -> Result<()> {
        let child = LibraryModule::new(
            "test.context-child",
            "fn {{self}}()->f32{return 1.0;}",
            None,
        );
        let mut parent = LibraryModule::new(
            "test.context-parent",
            "fn {{self}}()->f32{return {{dep0}}();}",
            None,
        );
        parent.dependencies.push(child.into_source());
        let mut rejected = parent.clone();
        rejected.dependencies[0].validate_context =
            |_| anyhow::bail!("dependency rejects the current geometry");
        assert!(parent.same_implementation(&rejected));
        let mut definition = scene::<Flat3>();
        definition.modules.libraries = vec![parent.clone()];
        compile(&definition)?;
        for modules in [
            vec![parent.clone(), rejected.clone()],
            vec![rejected.clone(), parent.clone()],
        ] {
            definition.modules.libraries = modules;
            let error = compile(&definition).unwrap_err();
            assert!(format!("{error:#}").contains("dependency rejects the current geometry"));
        }
        Ok(())
    }
    #[test]
    fn malformed_values_and_component_validators_are_checked() {
        let mut a = scene::<Flat3>();
        let material = &mut a.objects[0].material;
        material.schema.validate_words = |_, _, words| {
            ensure!(words[0] == 0, "invalid component value");
            Ok(())
        };
        material.words[0] = 1;
        assert!(compile(&a).is_err());
        let shape = &mut a.objects[0].shape;
        shape.words.clear();
        assert!(compile(&a).is_err());
    }
    #[test]
    fn invalid_scene_context_and_relative_precision_fail() {
        fn check<G: Geometry>() {
            for radius in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MIN_POSITIVE] {
                let mut a = scene::<G>();
                a.radius = radius;
                assert!(compile(&a).is_err());
            }
            for fov in [0.0, -1.0, f64::NAN, f64::INFINITY] {
                let mut a = scene::<G>();
                a.view.fov = fov;
                assert!(compile(&a).is_err());
            }
        }
        check::<Flat3>();
        check::<Hyperboloid3>();
        check::<Spherical3>();
        use ccgeom::Geometry3;
        let mut a = scene::<Hyperboloid3>();
        a.view.map = Transform::from_isometry(Hyperboloid3::shift_z(12.0)).unwrap();
        a.objects[0].map = Transform::from_isometry(Hyperboloid3::shift_z(12.2)).unwrap();
        assert!(compile(&a).is_ok());
        a.view.map = Transform::identity();
        assert!(compile(&a).is_err());
    }
    #[test]
    fn nested_shape_identity_survives_source_erasure() {
        let mut module = ShapeModule::<Flat3>::new("nested", "fn {{self}}() {}", None);
        // Public layout changes cannot remove the shape's identity requirement.
        module.parameter_words = Some(0);
        assert!(module.validate_length(&[]).is_err());
        let source = module.into_source();
        assert!(source.validate(GeometryContext::new(1.0), &[]).is_err());
        let material = MaterialModule::<Flat3>::new("empty", "fn {{self}}() {}", Some(0));
        assert!(
            material
                .into_source()
                .validate(GeometryContext::new(1.0), &[])
                .is_ok()
        );
    }
    #[test]
    fn unused_typed_roots_keep_source_structure_stable() -> Result<()> {
        let mut a = scene::<Flat3>();
        a.modules.shapes.push(a.objects[0].shape.schema.clone());
        a.modules
            .materials
            .push(a.objects[0].material.schema.clone());
        let full = compile(&a)?;
        a.objects.clear();
        let empty = compile(&a)?;
        assert_eq!(full.source, empty.source);
        assert!(empty.objects.is_empty());
        Ok(())
    }
}
