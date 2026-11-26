use crate::{
    projection::{self, Pose},
    world::{self, Material, Object, Sun},
};
use anyhow::Result;
use futures::executor::block_on;
use std::f64::consts::{FRAC_PI_2, PI};
use wgame::{
    Library,
    gfx::{
        Camera, Graphics, Scene, Target,
        types::{Color, color},
    },
    half::f16,
    image::{Image, ImageRead},
    rgb::Rgba,
    texture::RenderTexture,
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
    scene: &Scene,
) -> Result<Image<Rgba<f16>>> {
    target.clear(color::BLACK);
    target.render(camera, &scene.bake());
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
    let projection_camera = Camera::new(library.state(), projection::camera_matrix(size, 1.0));
    let shader = projection::shader(
        &library,
        &Sun {
            pose: Pose::identity(),
            radiance: [1.0; 3],
        },
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
        let objects = [
            Object {
                material: Material::Emissive,
                infinite_plane: false,
                mesh: near.clone(),
                pose: frame.move_local([0.0, 0.0, -2_f64.ln()], [0.0; 3], 1.0)?,
            },
            Object {
                material: Material::Emissive,
                infinite_plane: false,
                mesh: far.clone(),
                pose: frame.move_local([0.0, 0.0, -3_f64.ln()], [0.0; 3], 1.0)?,
            },
        ];

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
            let scene = projection::prepare(&objects, camera, &shader)?;
            let image = snapshot(&mut target, &projection_camera, &scene)?;
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
    let camera = Camera::new(library.state(), projection::camera_matrix(size, 1.0));
    let mut render = |object: &Object, eye, light: &projection::Shader| {
        let scene = projection::prepare(std::slice::from_ref(object), eye, light)?;
        snapshot(&mut target, &camera, &scene)
    };
    let plane = world::plane(&library);
    let earth = Object {
        mesh: plane.clone(),
        pose: Pose::identity(),
        material: Material::Diffuse,
        infinite_plane: true,
    };
    let above = |foot: Pose, height: f64| {
        foot.move_local(
            [0.0, height, 0.0],
            [-height.signum() * FRAC_PI_2, 0.0, 0.0],
            1.0,
        )
    };
    let illuminated = |pose, radiance| projection::shader(&library, &Sun { pose, radiance });
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
        // mesh origin stays fixed, even when the bright spot is 12 radii away.
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
    let moon = Object {
        mesh: plane.clone(),
        pose: Pose::identity().move_local([0.0, moon_height, 0.0], [0.0; 3], 1.0)?,
        material: Material::Diffuse,
        infinite_plane: true,
    };
    check_center(
        &render(&moon, above(moon.pose, 0.2)?, &light)?,
        (gap - moon_height).cosh().recip().powi(2),
    );
    check_center(&render(&moon, above(moon.pose, -0.2)?, &light)?, 0.0);

    // A thin leaf divides the same incident radiance between its two sides.
    let leaf = Object {
        mesh: plane.clone(),
        material: Material::Translucent { transmission: 0.25 },
        ..moon
    };
    let leaf_front = render(&leaf, above(leaf.pose, 0.2)?, &light)?;
    let leaf_back = render(&leaf, above(leaf.pose, -0.2)?, &light)?;
    let red = |frame: &Image<Rgba<f16>>| frame.data()[CENTER * WIDTH + CENTER].r.to_f32();
    assert!(red(&leaf_back) > 0.0 && red(&leaf_back) < red(&leaf_front));
    let total = f64::from(red(&leaf_front) + red(&leaf_back));
    assert!((total - (gap - moon_height).cosh().recip().powi(2)).abs() < 0.006);

    // Tilt the light to put its disc centre on the receiver's horizon.
    // Integrating the visible half-disc gives (a - sin(a)*cos(a))/pi.
    let grazing_sun = Pose::identity().move_local([gap, 0.0, 0.0], [0.0, 0.0, FRAC_PI_2], 1.0)?;
    let grazing = illuminated(grazing_sun, [1.0; 3])?;
    let a = gap.tanh().acos();
    check_center(
        &render(&earth, above(earth.pose, 0.2)?, &grazing)?,
        (a - a.sin() * a.cos()) / PI,
    );

    let emitter = Object {
        mesh: plane,
        pose: sun,
        material: Material::Emissive,
        infinite_plane: true,
    };
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
    for (object, eye) in [(&moon, above(moon.pose, 0.2)?), (&emitter, looking_up)] {
        assert!(
            render(object, eye, &extinguished)?
                .data()
                .iter()
                .all(|p| *p == color::BLACK.to_rgba_f16())
        );
    }
    Ok(())
}
