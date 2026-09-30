//! Native GPU checks; run explicitly with `cargo test -p hypertrace-examples
//! --test presentation -- --ignored`. Missing adapters are failures.

use hypertrace_renderer::{Gpu, Presenter, Renderer, read_buffer};
mod support;
use std::time::Duration;

fn output_texture(gpu: &Gpu, size: (u32, u32), format: wgpu::TextureFormat) -> wgpu::Texture {
    gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("presentation test output"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}

fn pixels(
    gpu: &Gpu,
    presenter: &Presenter,
    size: (u32, u32),
    format: wgpu::TextureFormat,
) -> Vec<[u8; 4]> {
    let texture = output_texture(gpu, size, format);
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    presenter.draw(&mut encoder, &texture.create_view(&Default::default()));
    let submission = gpu.queue.submit([encoder.finish()]);
    texture_pixels(gpu, &texture, submission)
}

fn texture_pixels(
    gpu: &Gpu,
    texture: &wgpu::Texture,
    producer: wgpu::SubmissionIndex,
) -> Vec<[u8; 4]> {
    // This synchronization is only for the explicit CPU snapshot. Compute and
    // presentation have already executed together without an intervening wait.
    gpu.device
        .poll(wgpu::PollType::Wait {
            submission_index: Some(producer),
            timeout: Some(Duration::from_secs(30)),
        })
        .unwrap();
    let size = (texture.width(), texture.height());
    let stride = (size.0 * 4).div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT)
        * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let byte_len = u64::from(stride) * u64::from(size.1);
    let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("presentation test copy"),
        size: byte_len,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(stride),
                rows_per_image: Some(size.1),
            },
        },
        texture.size(),
    );
    gpu.queue.submit([encoder.finish()]);
    read_buffer(&gpu.device, &gpu.queue, &buffer, byte_len)
        .unwrap()
        .chunks(stride as usize)
        .flat_map(|row| {
            row[..size.0 as usize * 4]
                .as_chunks::<4>()
                .0
                .iter()
                .copied()
        })
        .collect()
}

#[test]
#[ignore = "requires a GPU adapter"]
fn real_scenes_present_current_compute_results_without_an_intermediate_wait() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    eprintln!("Presentation adapter: {:?}", gpu.adapter.get_info());
    let size = (37, 29);
    let format = wgpu::TextureFormat::Rgba8Unorm;
    for name in ["euclidean", "hyperbolic", "spherical"] {
        hypertrace_gallery::with_example!(name, |_metadata, factory| {
            let scene = support::scene(factory);
            let mut renderer = Renderer::new(&gpu.device, &gpu.queue, size, scene, 123).unwrap();
            renderer.set_samples_per_dispatch(4).unwrap();
            let presenter = Presenter::new(&gpu.device, format, &renderer);
            let texture = output_texture(&gpu, size, format);
            let mut encoder = gpu.device.create_command_encoder(&Default::default());
            renderer.encode(&mut encoder);
            presenter.draw(&mut encoder, &texture.create_view(&Default::default()));
            let submission = gpu.queue.submit([encoder.finish()]);
            let displayed = texture_pixels(&gpu, &texture, submission);
            let linear = renderer.snapshot().unwrap();
            assert!(
                linear.iter().all(|pixel| pixel[3] == 1.0),
                "{name}: all pixels must be sampled"
            );
            assert!(
                linear.windows(2).any(|pair| pair[0][..3] != pair[1][..3]),
                "{name}: exercise a nonuniform image, not only a constant background"
            );
            for (index, (actual, color)) in displayed.into_iter().zip(linear).enumerate() {
                let expected = expected([color[0], color[1], color[2]]);
                assert!(
                    actual
                        .into_iter()
                        .zip(expected)
                        .all(|(a, e)| a.abs_diff(e) <= 1),
                    "{name} pixel {index}: displayed {actual:?}, expected {expected:?}"
                );
            }
            Ok::<(), anyhow::Error>(())
        })
        .unwrap();
    }
}

fn expected(color: [f32; 3]) -> [u8; 4] {
    let rgb = color.map(|v| (v.max(0.0).powf(1.0 / 2.2).min(1.0) * 255.0).round() as u8);
    [rgb[0], rgb[1], rgb[2], 255]
}

fn assert_pixel(actual: [u8; 4], expected: [u8; 4]) {
    assert!(
        actual
            .into_iter()
            .zip(expected)
            .all(|(a, e)| a.abs_diff(e) <= 1),
        "pixel {actual:?}, expected {expected:?}"
    );
}

#[test]
#[ignore = "requires a GPU adapter"]
fn accumulation_normalization_orientation_and_gamma_match_attachment_formats() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (2, 2),
        support::scene(hypertrace_gallery::factories::euclidean),
        1,
    )
    .unwrap();
    let colors = [
        [0.04, 0.25, 1.0],
        [0.0; 3],
        [0.8, 0.002, 0.25],
        [1.5, 0.0001, 0.5],
    ];
    let sums: [[f32; 4]; 4] = std::array::from_fn(|index| {
        let count = [2.0, 0.0, 4.0, 1.0][index];
        let [r, g, b] = colors[index];
        [r * count, g * count, b * count, count]
    });
    gpu.queue.write_buffer(
        renderer.accumulation_buffer(),
        0,
        bytemuck::cast_slice(&sums),
    );
    for format in [
        wgpu::TextureFormat::Rgba8Unorm,
        wgpu::TextureFormat::Rgba8UnormSrgb,
    ] {
        let presenter = Presenter::new(&gpu.device, format, &renderer);
        for (actual, color) in pixels(&gpu, &presenter, (2, 2), format)
            .into_iter()
            .zip(colors)
        {
            assert_pixel(actual, expected(color));
        }
    }
}

#[test]
#[ignore = "requires a GPU adapter"]
fn resize_rebinding_and_reset_show_current_accumulation() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let scene = support::background([0.04, 0.25, 1.0]);
    let mut renderer = Renderer::new(&gpu.device, &gpu.queue, (2, 2), scene, 1).unwrap();
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let mut presenter = Presenter::new(&gpu.device, format, &renderer);
    assert!(renderer.resize((7, 5)).unwrap());
    presenter.rebind(&gpu.device, &renderer);
    assert!(
        pixels(&gpu, &presenter, (7, 5), format)
            .into_iter()
            .all(|pixel| pixel == [0, 0, 0, 255])
    );
    renderer.render();
    for actual in pixels(&gpu, &presenter, (7, 5), format) {
        assert_pixel(actual, expected([0.04, 0.25, 1.0]));
    }
    let mut discarded = gpu.device.create_command_encoder(&Default::default());
    renderer.encode(&mut discarded);
    renderer.reset(2);
    drop(discarded);
    assert!(
        pixels(&gpu, &presenter, (7, 5), format)
            .into_iter()
            .all(|pixel| pixel == [0, 0, 0, 255])
    );
}

#[test]
#[ignore = "requires a GPU adapter"]
fn scaled_targets_sample_nearest_pixels_and_rebind_after_renderer_resize() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let mut renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (2, 2),
        support::scene(hypertrace_gallery::factories::euclidean),
        1,
    )
    .unwrap();
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let mut presenter = Presenter::new(&gpu.device, format, &renderer);
    let colors = [
        [0.04, 0.25, 1.0],
        [1.0, 0.1, 0.04],
        [0.25, 1.0, 0.04],
        [0.8, 0.002, 0.25],
    ];
    let upload = |colors: &[[f32; 3]], renderer: &Renderer<ccgeom::Flat3>| {
        let sums: Vec<_> = colors.iter().map(|&[r, g, b]| [r, g, b, 1.0]).collect();
        gpu.queue.write_buffer(
            renderer.accumulation_buffer(),
            0,
            bytemuck::cast_slice(&sums),
        );
    };
    let check = |presenter: &Presenter,
                 source_size: (u32, u32),
                 target_size: (u32, u32),
                 colors: &[[f32; 3]]| {
        let actual = pixels(&gpu, presenter, target_size, format);
        for y in 0..target_size.1 {
            for x in 0..target_size.0 {
                // Target pixel centers mapped to source cells. Integer math
                // keeps this independent of the shader's UV interpolation.
                let sx = (2 * x + 1) * source_size.0 / (2 * target_size.0);
                let sy = (2 * y + 1) * source_size.1 / (2 * target_size.1);
                assert_pixel(
                    actual[(y * target_size.0 + x) as usize],
                    expected(colors[(sy * source_size.0 + sx) as usize]),
                );
            }
        }
    };
    upload(&colors, &renderer);
    // Every corner, edge and interior pixel must come from its expected cell.
    check(&presenter, (2, 2), (6, 4), &colors);
    check(&presenter, (2, 2), (1, 1), &colors);

    assert!(renderer.resize((3, 2)).unwrap());
    presenter.rebind(&gpu.device, &renderer);
    let resized_colors = [
        colors[0],
        [0.4, 0.2, 0.8],
        colors[1],
        colors[2],
        [0.2, 0.4, 0.6],
        colors[3],
    ];
    upload(&resized_colors, &renderer);
    check(&presenter, (3, 2), (2, 1), &resized_colors);
    // Uneven horizontal scaling without centers exactly between source rows.
    check(&presenter, (3, 2), (5, 4), &resized_colors);
}
