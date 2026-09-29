//! Component validation and stable linking behavior, independent of WGPU.
use crate::object::tiling::{self, Tiling};
use crate::shader::*;
use crate::{material, material::MaterialValueExt, shape, shape::ShapeValueExt};
fn scene(shape: ShapeValue, material: MaterialValue) -> SceneDefinition {
    SceneDefinition {
        view: View {
            map: Transform::identity(Geometry::Euclidean),
            fov: 1.0,
        },
        background: Background::Constant([0.0; 3]),
        bounces: 3,
        radius: 1.0,
        medium: Medium::Vacuum,
        object: ObjectNode::Covered { shape, material },
        modules: vec![],
    }
}
fn curved_scene(geometry: Geometry, radius: f64, shape: ShapeValue) -> SceneDefinition {
    let mut value = scene(shape, material::transparent());
    value.view.map = Transform::identity(geometry);
    value.radius = radius;
    value
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
    c.object = ObjectNode::Vector(vec![b.object.clone(), b.object.clone()]);
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
    let ObjectNode::Covered { shape, .. } = &mut value.object else {
        unreachable!()
    };
    shape.words[1] = u32::MAX;
    assert!(compile(&value).is_err());
    let invalid = MaterialValue {
        schema: material::colored_schema(material::absorbing().schema),
        words: vec![0, 0],
    };
    assert!(compile(&scene(shape::plane(), invalid)).is_err());
    let mut mapped = shape::plane().mapped(Transform::identity(Geometry::Euclidean))?;
    mapped.words[0] = 0;
    assert!(compile(&scene(mapped, material::transparent())).is_err());
    let mut hyperbolic = scene(shape::plane(), material::transparent());
    hyperbolic.view.map = Transform::identity(Geometry::Hyperbolic);
    hyperbolic.object = ObjectNode::Covered {
        shape: ShapeValue {
            schema: shape::mapped_schema(Geometry::Hyperbolic, shape::plane_schema()),
            // Equal real/unreal quaternion norms make this embedded
            // hyperbolic map singular, despite finite components.
            words: [0.0f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]
                .map(f32::to_bits)
                .into(),
        },
        material: material::transparent(),
    };
    assert!(compile(&hyperbolic).is_err());
    Ok(())
}

#[test]
fn spherical_sphere_radii_are_validated_through_nested_payloads() -> Result<()> {
    let radius = 2.0;
    let valid = shape::geodesic_sphere(1.8 * radius)?;
    let nested = shape::vector(valid.schema.clone(), vec![valid])?
        .mapped(Transform::identity(Geometry::Spherical))?;
    compile(&curved_scene(Geometry::Spherical, radius, nested))?;
    for shape_radius in [std::f64::consts::PI * radius, 4.0 * radius] {
        let shape = shape::geodesic_sphere(shape_radius)?;
        let nested = shape::choice(vec![shape.schema.clone()], 0, shape)?;
        let error = compile(&curved_scene(Geometry::Spherical, radius, nested)).unwrap_err();
        assert!(format!("{error:#}").contains("strictly below pi"));
    }
    assert!(compile(&curved_scene(Geometry::Spherical, 0.25, shape::sphere())).is_err());
    // Parameterless sphere semantics also apply to inactive registered leaves.
    let mut inactive = curved_scene(Geometry::Spherical, 0.25, shape::plane());
    inactive.modules.push(shape::sphere_schema());
    assert!(compile(&inactive).is_err());
    let mut malformed = shape::geodesic_sphere(1.0)?;
    malformed.words[0] = f32::NAN.to_bits();
    assert!(compile(&curved_scene(Geometry::Spherical, radius, malformed)).is_err());
    Ok(())
}

#[test]
fn spherical_tilings_require_explicitly_uniform_semantics() -> Result<()> {
    for (selector, allowed) in [
        (tiling::Uniform::shader(), true),
        (tiling::Square::shader(), false),
        (tiling::Hexagonal::shader(), false),
        (tiling::Pentagonal::shader(), false),
        (tiling::Pentastar::shader(), false),
    ] {
        let material = tiling::tiled(
            selector,
            vec![material::transparent()],
            material::absorbing(),
            1.0,
            0.01,
        )?;
        let mut value = scene(shape::plane(), material);
        value.view.map = Transform::identity(Geometry::Spherical);
        assert_eq!(compile(&value).is_ok(), allowed);
    }
    Ok(())
}

#[test]
fn physical_parameters_update_payloads_without_changing_shader_structure() -> Result<()> {
    let mut a = curved_scene(Geometry::Spherical, 1.0, shape::geodesic_sphere(0.3)?);
    let mut b = curved_scene(Geometry::Spherical, 2.0, shape::geodesic_sphere(0.7)?);
    a.medium = Medium::Homogeneous {
        extinction: 0.1,
        albedo: [0.7; 3],
    };
    b.medium = Medium::Homogeneous {
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
    for (geometry, radius) in [
        (Geometry::Spherical, 1e-5),
        (
            Geometry::Spherical,
            f64::from(std::f32::consts::PI - f32::EPSILON),
        ),
        (Geometry::Hyperbolic, 1e-5),
        (Geometry::Spherical, 0.001),
        (Geometry::Hyperbolic, 0.001),
        (Geometry::Hyperbolic, 100.0),
    ] {
        let shape = shape::geodesic_sphere(radius)?;
        let error = compile(&curved_scene(geometry, 1.0, shape)).unwrap_err();
        assert!(
            format!("{error:#}").contains("f32 section resolver range"),
            "{}",
            error
        );
    }
    for geometry in [Geometry::Spherical, Geometry::Hyperbolic] {
        let error = compile(&curved_scene(geometry, 1e6, shape::sphere())).unwrap_err();
        assert!(format!("{error:#}").contains("f32 section resolver range"));
        compile(&curved_scene(geometry, 1.0, shape::geodesic_sphere(0.002)?))?;
    }
    Ok(())
}
#[test]
fn component_combinators_reject_wrong_shader_kinds_on_cpu() -> Result<()> {
    let invalid = shape::vector_schema(material::absorbing().schema);
    let mut scene = scene(shape::plane(), material::absorbing());
    scene.modules.push(invalid);
    assert!(
        compile(&scene).is_err(),
        "inactive invalid descriptors must fail too"
    );
    assert!(shape::vector(material::absorbing().schema, vec![]).is_err());
    assert!(shape::choice(vec![material::absorbing().schema], 0, shape::plane()).is_err());
    assert!(material::mixture_schema(vec![shape::plane_schema()]).is_err());
    assert!(material::colored(
        MaterialValue {
            schema: shape::plane_schema(),
            words: vec![0]
        },
        [1.0; 3]
    )
    .is_err());
    scene.modules = vec![material::emissive_schema(shape::plane_schema())];
    assert!(compile(&scene).is_err());
    scene.modules = vec![shape::mapped_schema(
        Geometry::Euclidean,
        material::absorbing().schema,
    )];
    assert!(compile(&scene).is_err());
    Ok(())
}

#[test]
fn tiling_components_validate_parameters_and_children() -> Result<()> {
    let context = GeometryContext {
        geometry: Geometry::Hyperbolic,
        radius: 1.0,
    };
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
            tiling::Uniform::shader(),
            vec![material::transparent()],
            material::absorbing(),
            1.0,
            width
        )
        .is_err());
    }
    assert!(tiling::tiled(
        tiling::Uniform::shader(),
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
    value.words[2] = f32::NAN.to_bits();
    assert!(
        value.schema.validate(context, &value.words).is_err(),
        "selector must validate nested material data"
    );
    Ok(())
}
