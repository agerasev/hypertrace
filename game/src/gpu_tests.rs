use crate::{
    config::AmbientLight,
    projection::{self, Drawing, Pose},
    world::{self, InfinitePlane, Material, Object, Objects, Sun},
};
use anyhow::Result;
use futures::executor::block_on;
use std::f64::consts::{FRAC_PI_2, PI};
use wgame::{
    Library,
    gfx::{
        Camera, Graphics, Target,
        types::{Color, color},
    },
    glam::Vec3,
    half::f16,
    image::{Image, ImageRead},
    prelude::Transformable,
    rgb::Rgba,
    texture::RenderTexture,
};

const NO_AMBIENT: AmbientLight = AmbientLight {
    night: [0.0; 3],
    day: [0.0; 3],
    response_factor: 1.0,
    response_exponent: 1.0,
};

fn headless_library() -> Result<Library> {
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&Default::default()))?;
    eprintln!("Adapter: {:?}", adapter.get_info());
    let (device, queue) = block_on(adapter.request_device(&Default::default()))?;
    // Wgame decodes sRGB readback to linear RGB, so the probes can compare radiance directly.
    let graphics = Graphics::new(adapter, device, queue, wgpu::TextureFormat::Rgba8UnormSrgb);
    Ok(Library::new(&graphics))
}

fn snapshot(
    target: &mut RenderTexture,
    camera: &Camera,
    drawing: &Drawing,
) -> Result<Image<Rgba<f16>>> {
    target.clear(color::BLACK);
    drawing.draw(target, camera);
    let submission = target.submit();
    target.state().device().poll(wgpu::PollType::Wait {
        submission_index: Some(submission),
        timeout: Some(std::time::Duration::from_secs(30)),
    })?;
    Ok(block_on(target.readback())?)
}

#[test]
#[ignore = "requires a graphics adapter"]
fn moved_camera_sees_depth_ordered_triangles_in_a_distant_world_frame() -> Result<()> {
    let library = headless_library()?;
    let size = (64, 64);
    let mut target = library
        .texturing()
        .render_texture(size, Default::default())?;
    let projection_camera =
        Camera::new(library.state(), projection::camera_matrix(size, 1.0, 0.005));
    let shader = projection::shader(
        &library,
        &Sun {
            pose: Pose::identity(),
            radiance: [1.0; 3],
        },
        NO_AMBIENT,
        None,
    )?;

    // At local chart height 1, all three corners have the same horizontal
    // radius, hence the same embedded Z. Translating along -Z scales all
    // half-space coordinates by 1/2 (near) or 1/3 (far).
    let triangle = |radius, tint| {
        world::mesh(
            &library,
            &[
                [-0.8 * radius, -0.6 * radius, 1.0],
                [0.8 * radius, -0.6 * radius, 1.0],
                [0.0, radius, 1.0],
            ]
            .map(|point| (point, tint)),
            &[0, 1, 2],
        )
    };
    let near = triangle(0.3, [1.0, 0.0, 0.0])?;
    let far = triangle(1.0, [0.0, 0.0, 1.0])?;
    let identity = Pose::identity();
    for world_offset in [0.0, 20.0] {
        let frame = identity.move_local([world_offset, 0.0, 0.0], [0.0; 3], 1.0)?;
        // Draw the red foreground first so a painter's-order renderer would
        // incorrectly show the blue background triangle at the red probe.
        let mut objects = Objects {
            meshes: vec![
                Object {
                    shape: near.clone(),
                    material: Material::Emissive(Vec3::ONE),
                    pose: frame.move_local([0.0, 0.0, -2_f64.ln()], [0.0; 3], 1.0)?,
                },
                Object {
                    shape: far.clone(),
                    material: Material::Emissive(Vec3::ONE),
                    pose: frame.move_local([0.0, 0.0, -3_f64.ln()], [0.0; 3], 1.0)?,
                },
            ],
            ..Objects::default()
        };

        // With tan(half-FOV)=1, screen x is 32*(1+X/-Z). Moving the
        // camera by d replaces X with cosh(d)*X-sinh(d)*W. The chosen
        // pixel centres lie well inside the red, blue and uncovered regions.
        // Rolling by a quarter turn maps pixel (x, y) to (63-y, x).
        for (dx, roll, red, blue, black) in [
            (0.0, 0.0, (32, 32), (32, 12), (16, 30)),
            (0.28, 0.0, (16, 30), (18, 10), (36, 30)),
            (
                0.28,
                std::f64::consts::FRAC_PI_2,
                (33, 16),
                (53, 18),
                (33, 36),
            ),
        ] {
            let camera = frame.move_local([dx, 0.0, 0.0], [0.0, 0.0, roll], 1.0)?;
            let drawing = projection::prepare(&objects, camera, &shader)?;
            let image = snapshot(&mut target, &projection_camera, &drawing)?;
            for ((x, y), expected) in [
                (red, color::RED),
                (blue, color::BLUE),
                (black, color::BLACK),
            ] {
                assert_eq!(
                    image.data()[y * size.0 as usize + x],
                    expected.to_rgba_f16(),
                    "pixel ({x}, {y}) with world offset {world_offset}, camera shift {dx}, roll {roll}"
                );
            }
        }

        // The triangle faces cross the optical axis at about 0.65 and 0.75
        // curvature radii. The intermediate plane must occlude only the far face.
        objects.planes.push(Object {
            shape: InfinitePlane,
            pose: frame.move_local([0.0, 0.0, -0.7], [FRAC_PI_2, 0.0, 0.0], 1.0)?,
            material: Material::Emissive(Vec3::Y),
        });
        let drawing = projection::prepare(&objects, frame, &shader)?;
        let image = snapshot(&mut target, &projection_camera, &drawing)?;
        for (y, expected) in [(32, color::RED), (12, color::GREEN)] {
            assert_eq!(
                image.data()[y * size.0 as usize + 32],
                expected.to_rgba_f16(),
                "plane between triangles at pixel (32, {y}) with world offset {world_offset}"
            );
        }
        // Growth scales the local mesh about its root before the hyperbolic pose.
        objects.meshes[0].shape = near.scale(0.5);
        let smaller = projection::prepare(&objects, frame, &shader)?;
        let smaller = snapshot(&mut target, &projection_camera, &smaller)?;
        let red_area = |frame: &Image<Rgba<f16>>| {
            frame
                .data()
                .iter()
                .filter(|&&pixel| pixel == color::RED.to_rgba_f16())
                .count()
        };
        assert_eq!(smaller.data()[32 * 64 + 32], color::RED.to_rgba_f16());
        assert!(red_area(&smaller) * 2 < red_area(&image));
    }
    Ok(())
}

#[test]
#[ignore = "requires a graphics adapter"]
fn geodesic_plane_light_has_a_finite_disc_and_lights_diffuse_faces() -> Result<()> {
    let library = headless_library()?;
    // Odd dimensions put the central pixel exactly on the camera's optical axis.
    const WIDTH: usize = 65;
    const CENTER: usize = WIDTH / 2;
    let size = (WIDTH as u32, WIDTH as u32);
    let mut target = library
        .texturing()
        .render_texture(size, Default::default())?;
    let camera = Camera::new(library.state(), projection::camera_matrix(size, 1.0, 0.005));
    let mut render = |objects: &Objects, eye, light: &projection::Shader| {
        let drawing = projection::prepare(objects, eye, light)?;
        snapshot(&mut target, &camera, &drawing)
    };
    let plane = |pose, material| Objects {
        planes: vec![Object {
            shape: InfinitePlane,
            pose,
            material,
        }],
        ..Objects::default()
    };
    let earth = plane(Pose::identity(), Material::Diffuse(Vec3::ONE));
    let above = |foot: Pose, height: f64| {
        foot.move_local(
            [0.0, height, 0.0],
            [-height.signum() * FRAC_PI_2, 0.0, 0.0],
            1.0,
        )
    };
    let illuminated =
        |pose, radiance| projection::shader(&library, &Sun { pose, radiance }, NO_AMBIENT, None);
    let check_center = |image: &Image<Rgba<f16>>, expected: f64| {
        let p = image.data()[CENTER * WIDTH + CENTER];
        for value in [p.r, p.g, p.b] {
            let actual = f64::from(value.to_f32());
            assert!(
                (actual - expected).abs() < 0.0004 + 0.007 * expected,
                "expected linear radiance {expected}, got {actual}"
            );
        }
    };

    let gap = 1.1_f64;
    for translation in [0.0, 12.0] {
        // Translate the light and observer along an infinite receiver. Its stored
        // pose stays fixed, even when the bright spot is 12 radii away.
        let center = Pose::identity().move_local([translation, 0.0, 0.0], [0.0; 3], 1.0)?;
        let sun = center.move_local([0.0, gap, 0.0], [0.0; 3], 1.0)?;
        let light = illuminated(sun, [1.0; 3])?;
        for radius in [0.0_f64, 1.8, 3.0] {
            let foot = center.move_local([radius, 0.0, 0.0], [0.0; 3], 1.0)?;
            let frame = render(&earth, above(foot, 0.2)?, &light)?;
            // Angular radius a of the solar disc satisfies sin(a)=sech(d),
            // sinh(d)=sinh(gap)*cosh(radius). Its cosine-weighted solid angle
            // divided by pi is cos(incidence)*sin(a)^2 for this plane.
            let cosh_d_squared = 1.0 + (gap.sinh() * radius.cosh()).powi(2);
            check_center(&frame, gap.cosh() / cosh_d_squared.powf(1.5));
        }
    }

    let sun = Pose::identity().move_local([0.0, gap, 0.0], [0.0; 3], 1.0)?;
    let light = illuminated(sun, [1.0; 3])?;
    let moon_height = 0.35;
    let moon_pose = Pose::identity().move_local([0.0, moon_height, 0.0], [0.0; 3], 1.0)?;
    let moon = plane(moon_pose, Material::Diffuse(Vec3::ONE));
    check_center(
        &render(&moon, above(moon_pose, 0.2)?, &light)?,
        (gap - moon_height).cosh().recip().powi(2),
    );
    check_center(&render(&moon, above(moon_pose, -0.2)?, &light)?, 0.0);

    // A finite triangle on the same plane divides the incident radiance between its two sides.
    let leaf = Objects {
        meshes: vec![Object {
            shape: world::mesh(
                &library,
                &[[-0.2, 0.0, 0.8], [0.2, 0.0, 0.8], [0.0, 0.0, 1.2]]
                    .map(|point| (point, [1.0; 3])),
                &[0, 1, 2],
            )?,
            pose: moon_pose,
            material: Material::Translucent {
                color: Vec3::ONE,
                transmission: 0.25,
            },
        }],
        ..Objects::default()
    };
    let leaf_front = render(&leaf, above(moon_pose, 0.2)?, &light)?;
    let leaf_back = render(&leaf, above(moon_pose, -0.2)?, &light)?;
    let red = |frame: &Image<Rgba<f16>>| frame.data()[CENTER * WIDTH + CENTER].r.to_f32();
    assert!(red(&leaf_back) > 0.0 && red(&leaf_back) < red(&leaf_front));
    let total = f64::from(red(&leaf_front) + red(&leaf_back));
    assert!((total - (gap - moon_height).cosh().recip().powi(2)).abs() < 0.006);

    // Tilt the vertex normals of a flat receiver. Their spatial components
    // interpolate to (0.5,1,0) on the axis; the analytic disk is above its
    // shading horizon, so irradiance scales with the inclination cosine.
    let tilted = Objects {
        meshes: vec![Object {
            shape: world::mesh_with_normals(
                &library,
                &[[-0.2, 0.0, 0.8], [0.2, 0.0, 0.8], [0.0, 0.0, 1.2]]
                    .map(|point| (point, [1.0; 3])),
                &[0, 1, 2],
                |[w, x, _, _]| Vec3::new((0.5 + 2.0 * x / w) as f32, 1.0, 0.0),
            )?,
            pose: Pose::identity(),
            material: Material::Diffuse(Vec3::ONE),
        }],
        ..Objects::default()
    };
    check_center(
        &render(&tilted, above(Pose::identity(), 0.2)?, &light)?,
        gap.cosh().recip().powi(2) / 1.25_f64.sqrt(),
    );
    check_center(
        &render(&tilted, above(Pose::identity(), -0.2)?, &light)?,
        0.0,
    );

    // Tilt the light to put its disc centre on the receiver's horizon.
    // Integrating the visible half-disc gives (a - sin(a)*cos(a))/pi.
    let grazing_sun = Pose::identity().move_local([gap, 0.0, 0.0], [0.0, 0.0, FRAC_PI_2], 1.0)?;
    let grazing = illuminated(grazing_sun, [1.0; 3])?;
    let a = gap.tanh().acos();
    check_center(
        &render(&earth, above(Pose::identity(), 0.2)?, &grazing)?,
        (a - a.sin() * a.cos()) / PI,
    );

    let emitter = plane(sun, Material::Emissive(Vec3::ONE));
    let looking_up = Pose::identity().move_local([0.0; 3], [FRAC_PI_2, 0.0, 0.0], 1.0)?;
    let frame = render(&emitter, looking_up, &light)?;
    // At this distance the disc's image radius is 32.5/sinh(gap) pixels.
    // The diagonal probe is outside the disc but inside its two-triangle scaffold.
    assert_eq!(
        frame.data()[CENTER * WIDTH + CENTER + 20],
        color::WHITE.to_rgba_f16()
    );
    assert_eq!(
        frame.data()[(CENTER + 20) * WIDTH + CENTER + 20],
        color::BLACK.to_rgba_f16()
    );

    let extinguished = illuminated(sun, [0.0; 3])?;
    check_center(&render(&moon, above(moon_pose, 0.2)?, &extinguished)?, 0.0);
    check_center(&render(&emitter, looking_up, &extinguished)?, 1.0);

    // On the unlit side of a white receiver, only local ambient light remains.
    let fill = AmbientLight {
        night: [0.005; 3],
        day: [0.15; 3],
        response_factor: 1.5,
        response_exponent: 2.0,
    };
    let with_fill = projection::shader(
        &library,
        &Sun {
            pose: sun,
            radiance: [1.0; 3],
        },
        fill,
        None,
    )?;
    for radius in [0.0_f64, 1.8, 6.0] {
        let foot = Pose::identity().move_local([radius, 0.0, 0.0], [0.0; 3], 1.0)?;
        let weight = (1.5 / (1.0 + (gap.sinh() * radius.cosh()).powi(2)))
            .min(1.0)
            .powi(2);
        let expected = 0.005 + 0.145 * weight;
        check_center(&render(&earth, above(foot, -0.2)?, &with_fill)?, expected);
    }
    Ok(())
}

#[test]
#[ignore = "requires a graphics adapter"]
fn atmosphere_matches_rayleigh_quadrature_to_surfaces_and_open_sky() -> Result<()> {
    use crate::atmosphere::Atmosphere;
    let library = headless_library()?;
    let size = (65, 65);
    let mut target = library
        .texturing()
        .render_texture(size, Default::default())?;
    let camera = Camera::new(library.state(), projection::camera_matrix(size, 1.0, 0.005));
    let gap = 1.3_f64;
    let sun = Sun {
        pose: Pose::identity().move_local([0.0, gap, 0.0], [0.0; 3], 1.0)?,
        radiance: [1.0; 3],
    };
    let air = Atmosphere {
        ground: Pose::identity(),
        height: 0.65,
        background: [0.012; 3],
        scattering: [0.5, 0.85, 1.20],
    };
    let shader = projection::shader(&library, &sun, NO_AMBIENT, Some(air))?;
    let height = 0.05_f64;
    let horizon_pitch = -height.tanh().asin();
    let mean = |p: [f64; 4]| -gap.sinh() * p[0] + gap.cosh() * p[2];
    // Use direct numerical quadrature of the finite solar disk's Rayleigh phase
    // moment at each position. This reference never solves a layer intersection.
    let reference = |eye: Pose, limit: f64| {
        let start = eye.apply_vector([1.0, 0.0, 0.0, 0.0]);
        let forward = eye.apply_vector([0.0, 0.0, 0.0, -1.0]);
        let step = limit / 16384.0;
        let (a, b) = (mean(start), mean(forward));
        let mut sum = 0.0;
        for i in 0..16384 {
            let distance = (i as f64 + 0.5) * step;
            let (s, c) = (distance.sinh(), distance.cosh());
            let altitude = start[2] * c + forward[2] * s;
            if !(0.0..=air.height.sinh()).contains(&altitude) {
                continue;
            }
            let q = a * c + b * s;
            let cosh_d = q.hypot(1.0);
            let cap_cos = q.abs() / cosh_d;
            let mu = (a * s + b * c) / cosh_d;
            let moment = cap_cos * (1.0 - cap_cos.powi(2)) * (3.0 * mu * mu - 1.0) / 16.0;
            sum += step * (0.5 * (1.0 - cap_cos) + moment);
        }
        sum
    };
    for (label, ground_offset, eye_height, pitch, hit) in [
        ("noon zenith", 0.0, height, FRAC_PI_2, None),
        ("night zenith", 4.0, height, FRAC_PI_2, None),
        ("ideal ground horizon", 0.0, height, horizon_pitch, None),
        ("dayward horizon", 1.8, height, horizon_pitch, None),
        ("outside outward", 0.0, 1.0, FRAC_PI_2, None),
        ("outside inward", 0.0, 1.0, -FRAC_PI_2, None),
        ("near emitter", 0.0, height, FRAC_PI_2, Some(0.025)),
        ("distant emitter", 0.0, height, FRAC_PI_2, Some(0.85)),
    ] {
        let foot = Pose::identity().move_local([0.0, 0.0, ground_offset], [0.0; 3], 1.0)?;
        let eye = foot.move_local([0.0, eye_height, 0.0], [pitch, 0.0, 0.0], 1.0)?;
        let mut objects = Objects::default();
        let emission = if let Some(distance) = hit {
            objects.planes.push(Object {
                shape: InfinitePlane,
                pose: foot.move_local([0.0, eye_height + distance, 0.0], [0.0; 3], 1.0)?,
                material: Material::Emissive(Vec3::splat(0.08)),
            });
            0.08
        } else {
            f64::from(air.background[0])
        };
        let drawing = projection::prepare(&objects, eye, &shader)?;
        let image = snapshot(&mut target, &camera, &drawing)?;
        let pixel = image.data()[32 * 65 + 32];
        let integral = reference(eye, hit.unwrap_or(24.0));
        for (value, beta) in [pixel.r, pixel.g, pixel.b].into_iter().zip(air.scattering) {
            let expected = emission + integral * f64::from(beta);
            let actual = f64::from(value.to_f32());
            assert!(
                (actual - expected).abs() < 0.001 + 0.009 * expected,
                "{label}: {actual} vs {expected}"
            );
        }
    }
    Ok(())
}
