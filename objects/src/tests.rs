//! Component validation and stable linking behavior, independent of WGPU.
use crate::object::tiling::{self, Tiling};
use crate::shader::*;
use crate::{material, material::MaterialValueExt, shape, shape::ShapeValueExt};
use ccgeom::{Flat3, Hyperboloid3, Spherical3};
fn scene(shape: ShapeValue<Flat3>, material: MaterialValue<Flat3>) -> SceneDefinition<Flat3> {
    definition(shape, material, 1.0)
}
fn definition<G: Geometry>(
    shape: ShapeValue<G>,
    material: MaterialValue<G>,
    radius: f64,
) -> SceneDefinition<G> {
    SceneDefinition {
        view: View {
            map: Transform::identity(),
            fov: 1.0,
        },
        background: Background::constant([0.0; 3]),
        bounces: 3,
        radius,
        medium: Medium::vacuum(),
        objects: vec![EncodedObject {
            map: Transform::identity(),
            shape,
            material,
        }],
        modules: Modules::default(),
    }
}
fn curved_scene<G: Geometry>(radius: f64, shape: ShapeValue<G>) -> SceneDefinition<G> {
    definition(shape, material::transparent(), radius)
}
#[test]
fn values_and_vector_lengths_do_not_change_shader_source() -> Result<()> {
    let a = scene(
        shape::vector(shape::plane_schema(), vec![])?,
        material::absorbing().emissive([1.0; 3])?,
    );
    let b = scene(
        shape::vector(shape::plane_schema(), vec![shape::plane(), shape::plane()])?,
        material::absorbing().emissive([2.0, 3.0, 4.0])?,
    );
    let mut c = b.clone();
    c.objects = vec![b.objects[0].clone(), b.objects[0].clone()];
    let (a, b, c) = (compile(&a)?, compile(&b)?, compile(&c)?);
    assert_eq!(a.source, b.source);
    assert_eq!(b.source, c.source);
    assert_ne!(a.words, b.words);
    assert_eq!(c.objects.len(), 2);
    assert_ne!(c.objects[0].info[2], c.objects[1].info[2]);
    Ok(())
}

#[test]
fn malformed_public_payloads_are_rejected() -> Result<()> {
    let mut value = scene(
        shape::vector(shape::plane_schema(), vec![shape::plane()])?,
        material::transparent(),
    );
    let shape = &mut value.objects[0].shape;
    shape.words[1] = u32::MAX;
    assert!(compile(&value).is_err());
    let invalid = MaterialValue {
        schema: material::colored_schema(material::absorbing().schema),
        words: vec![0, 0],
    };
    assert!(compile(&scene(shape::plane(), invalid)).is_err());
    let mut mapped = shape::plane().mapped(Transform::<Flat3>::identity())?;
    mapped.words[0] = 0;
    assert!(compile(&scene(mapped, material::transparent())).is_err());
    let hyperbolic = definition(
        ShapeValue {
            schema: shape::mapped_schema::<Hyperboloid3>(shape::plane_schema()),
            // Equal real/unreal quaternion norms make this embedded map singular.
            words: [0.0f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]
                .map(f32::to_bits)
                .into(),
        },
        material::transparent(),
        1.0,
    );
    assert!(compile(&hyperbolic).is_err());
    Ok(())
}

#[test]
fn spherical_sphere_radii_are_validated_through_nested_payloads() -> Result<()> {
    let radius = 2.0;
    let valid = shape::geodesic_sphere(1.8 * radius)?;
    let nested = shape::vector(valid.schema.clone(), vec![valid])?
        .mapped(Transform::<Spherical3>::identity())?;
    compile(&curved_scene::<Spherical3>(radius, nested))?;
    for shape_radius in [std::f64::consts::PI * radius, 4.0 * radius] {
        let shape = shape::geodesic_sphere(shape_radius)?;
        let nested = shape::tuple(vec![shape])?;
        let error = compile(&curved_scene::<Spherical3>(radius, nested)).unwrap_err();
        assert!(format!("{error:#}").contains("strictly below pi"));
    }
    assert!(compile(&curved_scene::<Spherical3>(0.25, shape::sphere())).is_err());
    // Parameterless sphere semantics also apply to inactive registered leaves.
    let mut inactive = curved_scene::<Spherical3>(0.25, shape::plane());
    inactive.modules.shapes.push(shape::sphere_schema());
    assert!(compile(&inactive).is_err());
    let mut malformed = shape::geodesic_sphere(1.0)?;
    malformed.words[0] = f32::NAN.to_bits();
    assert!(compile(&curved_scene::<Spherical3>(radius, malformed)).is_err());
    Ok(())
}

#[test]
fn spherical_uniform_tiling_keeps_shared_material_semantics() -> Result<()> {
    let material = tiling::tiled(
        <tiling::Uniform as Tiling<Spherical3>>::shader(),
        vec![material::transparent()],
        material::absorbing(),
        1.0,
        0.01,
    )?;
    compile(&definition(shape::plane(), material, 1.0))?;
    Ok(())
}

#[test]
fn physical_parameters_update_payloads_without_changing_shader_structure() -> Result<()> {
    let mut a = curved_scene::<Spherical3>(1.0, shape::geodesic_sphere(0.3)?);
    let mut b = curved_scene::<Spherical3>(2.0, shape::geodesic_sphere(0.7)?);
    a.medium = Medium {
        extinction: 0.1,
        albedo: [0.7; 3],
    };
    b.medium = Medium {
        extinction: 0.3,
        albedo: [0.2; 3],
    };
    let (a, b) = (compile(&a)?, compile(&b)?);
    assert_eq!(a.source, b.source);
    assert_ne!(a.radius, b.radius);
    assert_ne!(a.words, b.words);
    Ok(())
}
#[test]
fn curved_spheres_reject_unresolvable_f32_sections() -> Result<()> {
    fn check<G: Geometry>(radii: &[f64]) -> Result<()> {
        for &radius in radii {
            let shape = shape::geodesic_sphere(radius)?;
            let error = compile(&curved_scene::<G>(1.0, shape)).unwrap_err();
            assert!(
                format!("{error:#}").contains("f32 section resolver range"),
                "{:#}",
                error
            );
        }
        let error = compile(&curved_scene::<G>(1e6, shape::sphere())).unwrap_err();
        assert!(format!("{error:#}").contains("f32 section resolver range"));
        compile(&curved_scene::<G>(1.0, shape::geodesic_sphere(0.002)?))?;
        Ok(())
    }
    check::<Spherical3>(&[1e-5, f64::from(std::f32::consts::PI - f32::EPSILON), 0.001])?;
    check::<Hyperboloid3>(&[1e-5, 0.001, 100.0])
}

#[test]
fn tiling_components_validate_parameters_and_children() -> Result<()> {
    let context = GeometryContext::<Hyperboloid3>::new(1.0);
    for selector in [tiling::Square::shader(), tiling::Hexagonal::shader()] {
        for cell in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let value = tiling::tiled(
                selector.clone(),
                vec![material::transparent()],
                material::absorbing(),
                cell,
                0.01,
            );
            assert!(value
                .and_then(|v| v.schema.validate(context, &v.words))
                .is_err());
        }
    }
    for width in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(tiling::tiled(
            <tiling::Uniform as Tiling<Flat3>>::shader(),
            vec![material::transparent()],
            material::absorbing(),
            1.0,
            width
        )
        .is_err());
    }
    assert!(tiling::tiled(
        <tiling::Uniform as Tiling<Flat3>>::shader(),
        vec![],
        material::absorbing(),
        1.0,
        0.0
    )
    .is_err());
    let mut value = tiling::tiled(
        tiling::Square::shader(),
        vec![material::refractive(1.5)?],
        material::absorbing(),
        1.0,
        0.0,
    )?;
    let child_offset = value.words[2] as usize;
    value.words[child_offset] = f32::NAN.to_bits();
    assert!(
        value.schema.validate(context, &value.words).is_err(),
        "selector must validate nested material data"
    );
    Ok(())
}

fn variable_material<G: Geometry>(values: &[f32]) -> Result<MaterialValue<G>> {
    let mut module = MaterialModule::new(
        "tests.variable-material",
        "fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) { for(var i=0u;i<load_u32(base);i+=1u) { (*sample).emission+=vec3<f32>(load_f32(base+1u+i)); } (*sample).alive=0u; }",
        None,
    );
    module.validate_words = |_, _, words| {
        let count = *words
            .first()
            .ok_or_else(|| anyhow::anyhow!("missing variable material length"))?
            as usize;
        anyhow::ensure!(
            count == words.len() - 1,
            "variable material length mismatch"
        );
        anyhow::ensure!(
            words[1..]
                .iter()
                .all(|&word| f32::from_bits(word).is_finite()),
            "invalid variable material value"
        );
        Ok(())
    };
    let mut words = vec![values.len() as u32];
    words.extend(values.iter().map(|value| value.to_bits()));
    MaterialValue::new(module, words)
}

#[test]
fn variable_material_lengths_are_data_through_nested_combinators() -> Result<()> {
    let build = |values: &[f32], border: &[f32]| -> Result<SceneDefinition<Flat3>> {
        let mixture = material::mixture(vec![
            (0.25, material::transparent()),
            (0.75, variable_material(values)?.colored([0.5; 3])?),
        ])?;
        let tiled = tiling::tiled(
            <tiling::Uniform as Tiling<Flat3>>::shader(),
            vec![mixture],
            material::mixture(vec![(1.0, variable_material(border)?)])?,
            1.0,
            0.0,
        )?;
        assert_eq!(tiled.schema.parameter_words, None);
        Ok(scene(shape::plane(), tiled))
    };
    let empty = compile(&build(&[], &[])?)?;
    let populated = compile(&build(&[0.25, 0.5, 0.75], &[1.0, 2.0])?)?;
    assert_eq!(empty.source, populated.source);
    assert_ne!(empty.words, populated.words);
    assert_eq!(empty.materials.len(), populated.materials.len());
    Ok(())
}

#[test]
fn offset_tables_reject_malformed_ranges_and_validate_inactive_children() -> Result<()> {
    let context = GeometryContext::<Flat3>::new(1.0);
    // Both containers have a two-word prefix and two children; the first child
    // is variable-sized, and the last child has an empty parameter payload.
    let containers = [
        material::mixture(vec![
            (0.0, variable_material(&[0.25, 0.5])?),
            (1.0, material::absorbing()),
        ])?,
        tiling::tiled(
            <tiling::Uniform as Tiling<Flat3>>::shader(),
            vec![variable_material(&[0.25, 0.5])?],
            material::absorbing(),
            1.0,
            0.0,
        )?,
    ];
    for value in containers {
        value.schema.validate(context, &value.words)?;
        let mut corruptions = Vec::new();
        let mut truncated = value.words.clone();
        truncated.truncate(4);
        corruptions.push(truncated);
        for (slot, offset) in [(2, 4), (2, 6), (3, 4), (3, u32::MAX), (4, 7)] {
            let mut invalid = value.words.clone();
            invalid[slot] = offset;
            corruptions.push(invalid);
        }
        let mut trailing = value.words.clone();
        trailing.push(0);
        corruptions.push(trailing);
        let mut invalid_child = value.words.clone();
        let child = invalid_child[2] as usize;
        invalid_child[child] = u32::MAX;
        corruptions.push(invalid_child);
        for invalid in corruptions {
            assert!(
                value.schema.validate(context, &invalid).is_err(),
                "accepted malformed offset table or child payload: {:?}",
                invalid
            );
        }
    }
    Ok(())
}

#[test]
fn fixed_and_empty_materials_share_offset_layouts_with_variable_children() -> Result<()> {
    let context = GeometryContext::<Flat3>::new(1.0);
    let empty = material::mixture(vec![])?;
    assert_eq!(empty.schema.parameter_words, Some(1));
    empty.schema.validate(context, &empty.words)?;
    let fixed = material::mixture(vec![
        (0.5, material::transparent()),
        (0.5, material::absorbing()),
    ])?;
    assert_eq!(fixed.schema.parameter_words, Some(5));
    fixed.schema.validate(context, &fixed.words)?;
    assert_eq!(fixed.words[2], fixed.words[3]);
    assert_eq!(fixed.words[3], fixed.words[4]);
    let mut overflow = material::absorbing().schema;
    overflow.parameter_words = Some(u32::MAX);
    assert!(
        material::mixture_schema(vec![variable_material(&[])?.schema, overflow.clone()]).is_err()
    );
    assert!(tiling::tiled_schema(
        <tiling::Uniform as Tiling<Flat3>>::shader(),
        vec![overflow],
        material::absorbing().schema
    )
    .is_err());
    Ok(())
}
