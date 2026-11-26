use crate::{projection, world};
use anyhow::Result;
use futures::executor::block_on;
use wgame::{
    Library,
    gfx::{
        Camera, Graphics, Target,
        types::{Color, color},
    },
    image::ImageRead,
};

#[test]
#[ignore = "requires a graphics adapter"]
fn moved_camera_sees_depth_ordered_triangles_in_a_distant_world_frame() -> Result<()> {
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = block_on(instance.request_adapter(&Default::default()))?;
    eprintln!("Adapter: {:?}", adapter.get_info());
    let (device, queue) = block_on(adapter.request_device(&Default::default()))?;
    let graphics = Graphics::new(adapter, device, queue, wgpu::TextureFormat::Rgba8Unorm);
    let library = Library::new(&graphics);
    let size = (64, 64);
    let mut target = library
        .texturing()
        .render_texture(size, Default::default())?;
    let projection_camera = Camera::new(&graphics, projection::camera_matrix(size, 1.0));
    let shader = projection::shader(&library)?;

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
    let identity = projection::Pose::identity();
    for world_offset in [0.0, 20.0] {
        let frame = identity.move_local([world_offset, 0.0, 0.0], [0.0; 3], 1.0)?;
        // Draw the red foreground first so a painter's-order renderer would
        // incorrectly show the blue background triangle at the red probe.
        let objects = [
            world::Object {
                mesh: near.clone(),
                pose: frame.move_local([0.0, 0.0, -2_f64.ln()], [0.0; 3], 1.0)?,
            },
            world::Object {
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
            target.clear(color::BLACK);
            target.render(&projection_camera, &scene.bake());
            let submission = target.submit();
            graphics.device().poll(wgpu::PollType::Wait {
                submission_index: Some(submission),
                timeout: Some(std::time::Duration::from_secs(30)),
            })?;
            let image = block_on(target.readback())?;
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
