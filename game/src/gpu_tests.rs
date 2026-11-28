use crate::{
    config::{AmbientLight, SunAmbient},
    projection::{self, Drawing, Pose},
    world::{
        self, InfinitePlane,
        LightingDomain::{Celestial, Terrestrial},
        Material, Object, Objects, Sun,
    },
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

const NO_AMBIENT: AmbientLight = AmbientLight { night: [0.0; 3] };

fn plane_light(pose: Pose, surface_radiance: [f32; 3], scattering_radiance: [f32; 3]) -> Sun {
    Sun {
        pose,
        surface_radiance,
        scattering_radiance,
        ambient: SunAmbient {
            color: [0.0; 3],
            response_factor: 1.0,
            response_exponent: 1.0,
        },
    }
}

pub(crate) fn headless_library() -> Result<Library> {
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&Default::default()))?;
    eprintln!("Adapter: {:?}", adapter.get_info());
    let (device, queue) = block_on(adapter.request_device(&Default::default()))?;
    // Wgame decodes sRGB readback to linear RGB, so the probes can compare radiance directly.
    let graphics = Graphics::new(adapter, device, queue, wgpu::TextureFormat::Rgba8UnormSrgb);
    Ok(Library::new(&graphics))
}

// The renderer looks along local -Z. A level game observer looks along +Y with +Z up.
fn eye_at(foot: Pose, height: f64, pitch: f64) -> Result<Pose> {
    foot.move_local([0.0, 0.0, height], [FRAC_PI_2 + pitch, 0.0, 0.0], 1.0)
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
    let shader = projection::shader(&library, &[], NO_AMBIENT, None)?;

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
        let foot = identity.move_local([world_offset, 0.0, 0.0], [0.0; 3], 1.0)?;
        let frame = eye_at(foot, 0.0, 0.0)?;
        // Draw the red foreground first so a painter's-order renderer would
        // incorrectly show the blue background triangle at the red probe.
        let mut objects = Objects {
            meshes: vec![
                Object {
                    shape: near.clone(),
                    domain: Terrestrial,
                    material: Material::emissive([1.0; 3]),
                    pose: frame.move_local([0.0, 0.0, -2_f64.ln()], [0.0; 3], 1.0)?,
                },
                Object {
                    shape: far.clone(),
                    domain: Terrestrial,
                    material: Material::emissive([1.0; 3]),
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
            domain: Terrestrial,
            pose: frame.move_local([0.0, 0.0, -0.7], [0.0; 3], 1.0)?,
            material: Material::emissive([0.0, 1.0, 0.0]),
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
fn queued_tile_drawings_match_global_meshes_and_planes_from_each_camera() -> Result<()> {
    use crate::{atmosphere::Atmosphere, world::GroundTile};
    use std::rc::Rc;

    let library = headless_library()?;
    let size = (96, 96);
    let target = || library.texturing().render_texture(size, Default::default());
    let projection = projection::camera_matrix(size, 0.85, 0.005);
    let camera = Camera::new(library.state(), projection);
    let ground = Pose::identity().move_local([12.0, 0.0, 0.0], [0.0, 0.0, 0.3], 1.0)?;
    let shader = projection::shader(
        &library,
        &[
            plane_light(
                ground.move_local([0.0, 0.0, 1.1], [0.0; 3], 1.0)?,
                [0.7, 0.9, 0.6],
                [0.2, 0.5, 0.6],
            ),
            plane_light(
                ground.move_local([0.7, 0.3, 1.8], [0.0; 3], 1.0)?,
                [0.4, 0.15, 0.0],
                [0.1, 0.0, 0.02],
            ),
        ],
        AmbientLight { night: [0.15; 3] },
        Some(Atmosphere {
            ground,
            height: 0.6,
            background: [0.01; 3],
        }),
    )?;
    let triangle = world::mesh_with_normals(
        &library,
        &[
            ([-0.13, -0.11, 1.0], [0.9, 0.8, 0.7]),
            ([0.13, -0.11, 1.0], [0.8, 0.7, 0.9]),
            ([0.0, 0.16, 1.0], [0.7, 0.9, 0.8]),
        ],
        &[0, 1, 2],
        |[_, x, _, _]| Vec3::new(x as f32, 0.7, 1.0),
    )?;
    let diamond = world::mesh(
        &library,
        &[
            [-0.12, 0.0, 1.0],
            [0.0, -0.14, 1.0],
            [0.12, 0.0, 1.0],
            [0.0, 0.14, 1.0],
        ]
        .map(|point| (point, [1.0; 3])),
        &[0, 1, 2, 0, 2, 3],
    )?;
    let object = |shape: &wgame::shapes::Polygon, position, yaw, scale, emission| -> Result<_> {
        Ok(Object {
            shape: shape.scale(scale),
            domain: Terrestrial,
            // The raw chart triangles face their local Z axis. Stand them upright.
            pose: Pose::identity()
                .move_local(position, [0.0, 0.0, yaw], 1.0)?
                .move_local([0.0; 3], [FRAC_PI_2, 0.0, 0.0], 1.0)?,
            material: Material {
                reflection: [0.42, 0.33, 0.21],
                transmission: [0.03, 0.08, 0.16],
                emission,
            },
        })
    };
    let star = |yaw, emission| -> Result<_> {
        Ok(Object {
            shape: InfinitePlane,
            domain: Celestial,
            pose: Pose::identity()
                .move_local([0.0; 3], [0.48, 0.0, yaw], 1.0)?
                .move_local([0.0, 2.5, 0.0], [FRAC_PI_2, 0.0, 0.0], 1.0)?,
            // The reflective term exposes any ambient picked up by the tiled plane.
            material: Material {
                reflection: [0.3; 3],
                ..Material::emissive(emission)
            },
        })
    };
    let tiles = vec![
        Rc::new(GroundTile {
            pose: ground,
            planes: vec![star(-0.34, [0.8, 0.1, 0.0])?, star(0.37, [0.0, 0.7, 0.1])?],
            meshes: vec![
                object(&triangle, [-0.21, 0.65, 0.10], 0.15, 0.85, [0.32, 0.0, 0.0])?,
                object(&diamond, [0.0, 0.95, 0.23], -0.20, 0.95, [0.0, 0.32, 0.0])?,
                object(&triangle, [-0.14, 0.73, 0.42], 0.04, 0.8, [0.32, 0.12, 0.0])?,
            ],
        }),
        Rc::new(GroundTile {
            pose: ground.move_local([0.4, 0.1, 0.0], [0.0, 0.0, 0.25], 1.0)?,
            planes: vec![],
            meshes: vec![object(
                &triangle,
                [0.03, 0.62, 0.07],
                0.1,
                0.9,
                [0.0, 0.0, 0.32],
            )?],
        }),
    ];
    let mut global = Objects::default();
    for tile in &tiles {
        for mesh in &tile.meshes {
            global.meshes.push(Object {
                pose: tile.pose.chain(&mesh.pose)?,
                ..mesh.clone()
            });
        }
        for plane in &tile.planes {
            global.planes.push(Object {
                pose: tile.pose.chain(&plane.pose)?,
                ..*plane
            });
        }
    }
    let tiled = Objects {
        tiles,
        ..Objects::default()
    };
    let eyes = [
        eye_at(ground, 0.12, 0.0)?,
        ground
            .move_local([0.10, -0.04, 0.20], [0.0, 0.0, -0.1], 1.0)?
            .move_local([0.0; 3], [FRAC_PI_2 - 0.04, 0.0, 0.02], 1.0)?,
    ];
    let mut reference = target()?;
    let first = snapshot(
        &mut reference,
        &camera,
        &projection::prepare(&global, eyes[0], &shader)?,
    )?;
    let second = snapshot(
        &mut reference,
        &camera,
        &projection::prepare(&global, eyes[1], &shader)?,
    )?;

    // Both drawings share static tile buffers. Each must see its own eye and
    // atmosphere even when the queue receives both command buffers together.
    let drawings = [
        projection::prepare(&tiled, eyes[0], &shader)?,
        projection::prepare(&tiled, eyes[1], &shader)?,
    ];
    let mut outputs = [target()?, target()?];
    for (output, drawing) in outputs.iter_mut().zip(drawings.iter()) {
        // The first tile's triangle, diamond and triangle form two instanced
        // draws, and both stars batch into a third. The next tile draws one mesh.
        assert_eq!(drawing.ground_batch_count(), 4);
        output.clear(color::BLACK);
        drawing.draw(output, &camera);
    }
    let submission = library
        .state()
        .queue()
        .submit(outputs.each_mut().map(RenderTexture::finish));
    library.state().device().poll(wgpu::PollType::Wait {
        submission_index: Some(submission),
        timeout: Some(std::time::Duration::from_secs(30)),
    })?;
    for ((output, eye), expected) in outputs.iter_mut().zip(eyes).zip([first, second]) {
        let actual = block_on(output.readback())?;
        let rgb = |p: Rgba<f16>| Vec3::new(p.r.to_f32(), p.g.to_f32(), p.b.to_f32());
        let poses = global
            .meshes
            .iter()
            .map(|mesh| mesh.pose)
            .chain(global.planes.iter().map(|plane| plane.pose));
        for (pose, channel) in poses.zip([0, 1, 0, 2, 0, 1]) {
            // Compare interior pixels, clear of subpixel rounding at the edges.
            let [w, x, y, z] = eye
                .inverse()?
                .chain(&pose)?
                .apply_vector([1.0, 0.0, 0.0, 0.0]);
            let clip = projection * wgame::glam::Vec4::new(x as f32, y as f32, z as f32, w as f32);
            let px = (0.5 * (1.0 + clip.x / clip.w) * size.0 as f32) as usize;
            let py = (0.5 * (1.0 - clip.y / clip.w) * size.1 as f32) as usize;
            for index in [
                py * size.0 as usize + px,
                (py + 1) * size.0 as usize + px + 1,
            ] {
                let baseline = rgb(expected.data()[index]);
                assert!(
                    baseline[channel] > 0.20,
                    "missing reference surface at ({px}, {py})"
                );
                let pixel = rgb(actual.data()[index]);
                assert!(
                    pixel.abs_diff_eq(baseline, 0.009),
                    "tile {pixel:?} vs world {baseline:?} at ({px}, {py})"
                );
            }
        }
        // Include the sky around the two circular silhouettes. Omitting the
        // open-disk clip would leave bright corners on the tile's plane quads.
        let differing = actual
            .data()
            .iter()
            .zip(expected.data())
            .filter(|(a, b)| !rgb(**a).abs_diff_eq(rgb(**b), 0.04))
            .count();
        assert!(
            differing <= 4,
            "{differing} pixels differ around the surfaces"
        );
    }
    Ok(())
}

#[test]
#[ignore = "requires a graphics adapter"]
fn configured_sun_disk_and_reflected_sunlight_have_independent_colors() -> Result<()> {
    let library = headless_library()?;
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("config.toml");
    let mut config = crate::config::Config::load(&path)?;
    config.earth.material = Material {
        reflection: [1.0; 3],
        ..Material::default()
    };
    config.suns.truncate(1);
    let sun = &mut config.suns[0];
    sun.ground_offset = [0.0; 2];
    sun.distance = 1.0;
    sun.disk_radiance = [2.0, 0.5, 0.0];
    sun.surface_radiance = [0.0, 0.0, 1.0];
    sun.ambient.color = [0.0; 3];
    config.earth.grid.view_radius = 0.1;
    let patch = world::Patch::new(&config.earth.grid, config.max_sites_per_tile())?;
    let mut world = world::world(&library, &config, patch, [])?;
    let shader = projection::shader(&library, &world.suns, NO_AMBIENT, None)?;
    let surfaces = Objects {
        planes: world.objects_near(Pose::identity())?.planes.clone(),
        ..Objects::default()
    };
    let size = (33, 33);
    let mut target = library
        .texturing()
        .render_texture(size, Default::default())?;
    let camera = Camera::new(library.state(), projection::camera_matrix(size, 1.0, 0.005));
    // From just above noon the upward view sees the display-mapped disk.
    // The white ground below receives the blue source radiance times sech²(gap).
    for (pitch, expected) in [
        (FRAC_PI_2, Vec3::new(1.0, 0.25, 0.0)),
        (-FRAC_PI_2, Vec3::Z / 1.0_f32.cosh().powi(2)),
    ] {
        let eye = eye_at(Pose::identity(), 0.1, pitch)?;
        let drawing = projection::prepare(&surfaces, eye, &shader)?;
        let image = snapshot(&mut target, &camera, &drawing)?;
        let pixel = image.data()[image.data().len() / 2];
        let actual = Vec3::new(pixel.r.to_f32(), pixel.g.to_f32(), pixel.b.to_f32());
        assert!(
            actual.abs_diff_eq(expected, 0.006),
            "{actual} vs {expected}"
        );
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
    let plane = |pose, material, domain| Objects {
        planes: vec![Object {
            shape: InfinitePlane,
            pose,
            material,
            domain,
        }],
        ..Objects::default()
    };
    let white = Material {
        reflection: [1.0; 3],
        ..Material::default()
    };
    let earth = plane(Pose::identity(), white, Terrestrial);
    let above = |foot: Pose, height: f64| eye_at(foot, height, -height.signum() * FRAC_PI_2);
    let illuminated = |pose, surface_radiance| {
        projection::shader(
            &library,
            &[plane_light(pose, surface_radiance, [0.0; 3])],
            NO_AMBIENT,
            None,
        )
    };
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
        let sun = center.move_local([0.0, 0.0, gap], [0.0; 3], 1.0)?;
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

    let sun = Pose::identity().move_local([0.0, 0.0, gap], [0.0; 3], 1.0)?;
    let light = illuminated(sun, [1.0; 3])?;
    let moon_height = 0.35;
    let moon_pose = Pose::identity().move_local([0.0, 0.0, moon_height], [0.0; 3], 1.0)?;
    let moon = plane(moon_pose, white, Celestial);
    check_center(
        &render(&moon, above(moon_pose, 0.2)?, &light)?,
        (gap - moon_height).cosh().recip().powi(2),
    );
    check_center(&render(&moon, above(moon_pose, -0.2)?, &light)?, 0.0);

    // At normal incidence the entire solar disc lies on one side of the surface.
    // Moving the eye across it exchanges the reflection and transmission terms.
    let mixed = Material {
        reflection: [0.4, 0.12, 0.0],
        transmission: [0.0, 0.2, 0.55],
        emission: [0.05, 0.025, 0.04],
    };
    let tint = [0.7, 0.85, 0.6];
    // The unit hemisphere of the half-space chart maps to the Z=0 receiver.
    let h = (1.0_f64 - 0.2_f64.powi(2)).sqrt();
    let receiver = [[-0.16, -0.12, h], [0.16, -0.12, h], [0.0, 0.2, h]];
    let triangle = world::mesh(&library, &receiver.map(|point| (point, tint)), &[0, 1, 2])?;
    let mesh = |domain| Objects {
        meshes: vec![Object {
            shape: triangle.clone(),
            pose: moon_pose,
            material: mixed,
            domain,
        }],
        ..Objects::default()
    };
    let sunlight = [0.8, 0.45, 0.7];
    let night = [0.06, 0.09, 0.12];
    let day = [0.05, 0.07, 0.02];
    let mut source = plane_light(sun, sunlight, [0.0; 3]);
    // The daytime response saturates at the centre of the triangle.
    source.ambient.color = day;
    source.ambient.response_factor = 10.0;
    let colored = projection::shader(&library, &[source], AmbientLight { night }, None)?;
    let extinguished = illuminated(sun, [0.0; 3])?;
    let disc = (gap - moon_height).cosh().recip().powi(2) as f32;
    let ambient = std::array::from_fn(|i| night[i] + day[i]);
    for (surface, tint, ambient) in [
        (plane(moon_pose, mixed, Terrestrial), [1.0; 3], ambient),
        (mesh(Terrestrial), tint, ambient),
        (mesh(Celestial), tint, [0.0; 3]),
    ] {
        for (light, height, direct, fill) in [
            (&colored, 0.2, mixed.reflection, ambient),
            (&colored, -0.2, mixed.transmission, ambient),
            (&extinguished, 0.2, [0.0; 3], [0.0; 3]),
        ] {
            let frame = render(&surface, above(moon_pose, height)?, light)?;
            let p = frame.data()[CENTER * WIDTH + CENTER];
            for (i, value) in [p.r, p.g, p.b].into_iter().enumerate() {
                let expected = tint[i]
                    * (mixed.emission[i]
                        + (mixed.reflection[i] + mixed.transmission[i]) * fill[i]
                        + direct[i] * disc * sunlight[i]);
                let actual = value.to_f32();
                assert!(
                    (actual - expected).abs() < 0.001 + 0.009 * expected,
                    "surface tint {tint:?}, eye height {height}, channel {i}: {actual} vs {expected}"
                );
            }
        }
    }

    // Tilt the vertex normals of a flat receiver. Their spatial components
    // interpolate to (0.5,0,1) on the axis; the analytic disk is above its
    // shading horizon, so irradiance scales with the inclination cosine.
    let tilted = Objects {
        meshes: vec![Object {
            shape: world::mesh_with_normals(
                &library,
                &receiver.map(|point| (point, [1.0; 3])),
                &[0, 1, 2],
                |[w, x, _, _]| Vec3::new((0.5 + 2.0 * x / w) as f32, 0.0, 1.0),
            )?,
            domain: Terrestrial,
            pose: Pose::identity(),
            material: white,
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
    let grazing_sun = Pose::identity().move_local([gap, 0.0, 0.0], [0.0, FRAC_PI_2, 0.0], 1.0)?;
    let grazing = illuminated(grazing_sun, [1.0; 3])?;
    let a = gap.tanh().acos();
    check_center(
        &render(&earth, above(Pose::identity(), 0.2)?, &grazing)?,
        (a - a.sin() * a.cos()) / PI,
    );

    let emitter = plane(sun, Material::emissive([1.0; 3]), Celestial);
    let looking_up = eye_at(Pose::identity(), 0.0, FRAC_PI_2)?;
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

    check_center(&render(&moon, above(moon_pose, 0.2)?, &extinguished)?, 0.0);
    check_center(&render(&emitter, looking_up, &extinguished)?, 1.0);

    // On the unlit side of a white receiver, only local ambient light remains.
    let fill = AmbientLight { night: [0.005; 3] };
    let with_fill = projection::shader(
        &library,
        &[Sun {
            ambient: SunAmbient {
                color: [0.145; 3],
                response_factor: 1.5,
                response_exponent: 2.0,
            },
            ..plane_light(sun, [1.0; 3], [0.0; 3])
        }],
        fill,
        None,
    )?;
    check_center(&render(&moon, above(moon_pose, -0.2)?, &with_fill)?, 0.0);
    for radius in [0.0_f64, 1.8, 6.0] {
        let foot = Pose::identity().move_local([radius, 0.0, 0.0], [0.0; 3], 1.0)?;
        let weight = (1.5 / (1.0 + (gap.sinh() * radius.cosh()).powi(2)))
            .min(1.0)
            .powi(2);
        let expected = 0.005 + 0.145 * weight;
        check_center(&render(&earth, above(foot, -0.2)?, &with_fill)?, expected);
    }
    let dark = projection::shader(&library, &[], fill, None)?;
    check_center(
        &render(&earth, above(Pose::identity(), 0.2)?, &dark)?,
        f64::from(fill.night[0]),
    );

    // Add the projected irradiances of three distinct disks. The brightest
    // channel exceeds one, so channel ratios also test the final display mapping.
    let mut suns = Vec::new();
    let mut direct = Vec3::ZERO;
    let mut ambient = Vec3::from(fill.night);
    for (offset, gap, color) in [
        (0.0_f64, 1.1_f64, Vec3::new(3.1, 0.07, 0.3)),
        (0.5, 0.8, Vec3::new(1.6, 0.75, 0.09)),
        (-0.35, 0.55, Vec3::new(0.05, 0.2, 2.5)),
    ] {
        let pose = Pose::identity().move_local([offset, 0.0, gap], [0.0; 3], 1.0)?;
        let fill = SunAmbient {
            color: (color * 0.1).to_array(),
            response_factor: gap as f32,
            response_exponent: 1.0 + offset as f32,
        };
        suns.push(Sun {
            ambient: fill,
            ..plane_light(pose, color.to_array(), [0.0; 3])
        });
        let cosh_d = (gap.sinh() * offset.cosh()).hypot(1.0);
        direct += color * (gap.cosh() / cosh_d.powi(3)) as f32;
        let weight = (fill.response_factor * color.max_element() / cosh_d.powi(2) as f32)
            .min(1.0)
            .powf(fill.response_exponent);
        ambient += Vec3::from(fill.color) * weight;
    }
    // A surface-only Sun can illuminate the receiver from inside the air layer.
    let combined = projection::shader(
        &library,
        &suns,
        fill,
        Some(crate::atmosphere::Atmosphere {
            ground: Pose::identity(),
            height: 0.9,
            background: [0.0; 3],
        }),
    )?;
    for (height, direct) in [(0.2, direct), (-0.2, Vec3::ZERO)] {
        let linear = direct + ambient;
        let expected = linear / linear.max_element().max(1.0);
        let frame = render(&earth, above(Pose::identity(), height)?, &combined)?;
        let p = frame.data()[CENTER * WIDTH + CENTER];
        let actual = Vec3::new(p.r.to_f32(), p.g.to_f32(), p.b.to_f32());
        assert!(
            actual.abs_diff_eq(expected, 0.006),
            "{actual} vs {expected}"
        );
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
    let mut suns = Vec::new();
    for (foot, gap, scattering_radiance) in [
        (0.0, 1.3, [0.8, 0.1, 0.0]),
        (0.85, 0.95, [0.2, 0.7, 0.0]),
        (-0.45, 1.9, [0.0, 0.1, 1.0]),
    ] {
        let pose = Pose::identity().move_local([foot, 0.0, gap], [0.0; 3], 1.0)?;
        suns.push(plane_light(pose, [0.0; 3], scattering_radiance));
    }
    let air = Atmosphere {
        ground: Pose::identity(),
        height: 0.65,
        background: [0.012; 3],
    };
    let shader = projection::shader(&library, &suns, AmbientLight { night: [0.2; 3] }, Some(air))?;
    let height = 0.05_f64;
    let horizon_pitch = -height.tanh().asin();
    // Use direct numerical quadrature of the finite solar disk's Rayleigh phase
    // moment at each position. This reference never solves a layer intersection.
    let reference = |eye: Pose, limit: f64, sun: &Sun| {
        let start = eye.apply_vector([1.0, 0.0, 0.0, 0.0]);
        let forward = eye.apply_vector([0.0, 0.0, 0.0, -1.0]);
        let step = limit / 16384.0;
        let [w, x, y, z] = sun.pose.apply_vector([0.0, 0.0, 0.0, 1.0]);
        let signed_distance_sinh = |p: [f64; 4]| -w * p[0] + x * p[1] + y * p[2] + z * p[3];
        let (a, b) = (signed_distance_sinh(start), signed_distance_sinh(forward));
        let mut sum = 0.0;
        for i in 0..16384 {
            let distance = (i as f64 + 0.5) * step;
            let (s, c) = (distance.sinh(), distance.cosh());
            let altitude = start[3] * c + forward[3] * s;
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
        let foot = Pose::identity().move_local([0.0, -ground_offset, 0.0], [0.0; 3], 1.0)?;
        let eye = eye_at(foot, eye_height, pitch)?;
        let mut objects = Objects::default();
        let emission = if let Some(distance) = hit {
            objects.planes.push(Object {
                shape: InfinitePlane,
                domain: Celestial,
                pose: foot.move_local([0.0, 0.0, eye_height + distance], [0.0; 3], 1.0)?,
                material: Material {
                    reflection: [0.5; 3],
                    transmission: [0.5; 3],
                    emission: [0.08; 3],
                },
            });
            0.08
        } else {
            f64::from(air.background[0])
        };
        let drawing = projection::prepare(&objects, eye, &shader)?;
        let image = snapshot(&mut target, &camera, &drawing)?;
        let pixel = image.data()[32 * 65 + 32];
        let scattered: wgame::glam::DVec3 = suns
            .iter()
            .map(|sun| {
                Vec3::from(sun.scattering_radiance).as_dvec3()
                    * reference(eye, hit.unwrap_or(24.0), sun)
            })
            .sum();
        for (i, value) in [pixel.r, pixel.g, pixel.b].into_iter().enumerate() {
            let expected = emission + scattered[i];
            let actual = f64::from(value.to_f32());
            assert!(
                (actual - expected).abs() < 0.001 + 0.009 * expected,
                "{label}: {actual} vs {expected}"
            );
        }
    }
    Ok(())
}
