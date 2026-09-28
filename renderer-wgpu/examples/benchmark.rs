//! Completed-render timings, with resets, startup and readback measured separately.
use anyhow::{Context, bail, ensure};
use hypertrace_wgpu::{Gpu, Renderer, Result};
use std::time::{Duration, Instant};

mod support;

fn finish(gpu: &Gpu) -> Result<()> {
    gpu.device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: Some(Duration::from_secs(60)),
    })?;
    Ok(())
}

fn reset(gpu: &Gpu, renderer: &mut Renderer, seed: u32, batch: u32) -> Result<()> {
    renderer.reset(seed);
    renderer.set_samples_per_dispatch(batch)?;
    // write_buffer transfers start on submission; poll alone cannot flush them.
    gpu.queue.submit([]);
    finish(gpu)
}

fn render_samples(gpu: &Gpu, renderer: &mut Renderer, samples: u32, batch: u32) -> Result<()> {
    let mut remaining = samples;
    while remaining > 0 {
        let count = remaining.min(batch);
        if count != batch {
            renderer.set_samples_per_dispatch(count)?;
        }
        renderer.render();
        // Measure completed work after every batch. Batch=1 measures individual
        // sample submissions; larger batches amortize submission and state I/O.
        finish(gpu)?;
        remaining -= count;
    }
    Ok(())
}

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c < ' ' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn main() -> Result<()> {
    let mut scene_name = String::from("hy");
    let (mut width, mut height, mut samples) = (640u32, 480u32, 16u32);
    let (mut trials, mut warmup, mut batch, mut seed) = (5u32, 2u32, 1u32, 3735928559u32);
    let mut bounces = None;
    let mut output = String::from("benchmark-wgpu.json");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--help" || arg == "-h" {
            println!(
                "benchmark [--scene eu|hy|sp] [--width 640] [--height 480] [--samples 16] [--trials 5] [--warmup 2] [--batch 1] [--seed 3735928559] [--bounces N] [--output benchmark-wgpu.json]\nUse --release. Timings wait for completed work after each batch, exclude reset/upload, and report readback separately. --warmup counts samples. WGPU_BACKEND and Vulkan ICD selection control the device."
            );
            return Ok(());
        }
        let value = args
            .next()
            .with_context(|| format!("missing value for {arg}"))?;
        match arg.as_str() {
            "--scene" => scene_name = value,
            "--width" => width = value.parse()?,
            "--height" => height = value.parse()?,
            "--samples" => samples = value.parse()?,
            "--trials" => trials = value.parse()?,
            "--warmup" => warmup = value.parse()?,
            "--batch" => batch = value.parse()?,
            "--seed" => seed = value.parse()?,
            "--bounces" => bounces = Some(value.parse()?),
            "--output" => output = value,
            _ => bail!("unknown option {arg}"),
        }
    }
    ensure!(
        samples > 0 && trials > 0 && warmup > 0,
        "samples, trials and warmup must be positive"
    );
    ensure!((1..=1024).contains(&batch), "batch must be 1..=1024");
    let device_start = Instant::now();
    let gpu = futures::executor::block_on(Gpu::headless())?;
    let device_setup_ms = device_start.elapsed().as_secs_f64() * 1000.0;
    let adapter = gpu.adapter.get_info();
    eprintln!("WGPU benchmark adapter: {adapter:?}");
    let setup_start = Instant::now();
    let mut scene = support::scene(&scene_name)?;
    if let Some(value) = bounces {
        scene.bounces = value;
    }
    let bounces = scene.bounces;
    let mut renderer = Renderer::new(&gpu.device, &gpu.queue, (width, height), scene, seed)?;
    gpu.queue.submit([]);
    finish(&gpu)?;
    let setup_ms = setup_start.elapsed().as_secs_f64() * 1000.0;
    reset(&gpu, &mut renderer, seed, 1)?;
    let start = Instant::now();
    render_samples(&gpu, &mut renderer, warmup, 1)?;
    let warmup_ms = start.elapsed().as_secs_f64() * 1000.0;
    let mut render_ms = Vec::new();
    let mut readback_ms = Vec::new();
    let mut checksum = [0.0f64; 3];
    for trial in 0..trials {
        reset(&gpu, &mut renderer, seed, batch)?;
        let start = Instant::now();
        render_samples(&gpu, &mut renderer, samples, batch)?;
        render_ms.push(start.elapsed().as_secs_f64() * 1000.0);
        let start = Instant::now();
        let pixels = renderer.snapshot()?;
        ensure!(pixels.iter().all(|p| p[3] == 1.0), "incomplete GPU frame");
        readback_ms.push(start.elapsed().as_secs_f64() * 1000.0);
        checksum = [0.0; 3];
        for pixel in pixels {
            for channel in 0..3 {
                checksum[channel] += f64::from(pixel[channel]);
            }
        }
        println!(
            "{scene_name} {width}x{height}, {samples} spp, batch {batch}, trial {}: render {:.3} ms, readback {:.3} ms",
            trial + 1,
            render_ms[trial as usize],
            readback_ms[trial as usize]
        );
    }
    // Check the actual counter outside all measured sections. The normalized
    // snapshot's alpha is one for any nonzero count, including a partial frame.
    let raw = hypertrace_wgpu::read_buffer(
        &gpu.device,
        &gpu.queue,
        renderer.accumulation_buffer(),
        renderer.accumulation_buffer().size(),
    )?;
    ensure!(
        raw.as_chunks::<16>()
            .0
            .iter()
            .all(|bytes| { bytemuck::pod_read_unaligned::<[f32; 4]>(bytes)[3] == samples as f32 }),
        "GPU accumulation count differs from requested samples"
    );
    let mut sorted = render_ms.clone();
    sorted.sort_by(f64::total_cmp);
    let median = if sorted.len() % 2 == 1 {
        sorted[sorted.len() / 2]
    } else {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) * 0.5
    };
    println!(
        "Median: {median:.3} ms, {:.3} ms/sample, {:.3} M pixel-samples/s",
        median / f64::from(samples),
        f64::from(width) * f64::from(height) * f64::from(samples) / (median * 1000.0)
    );
    let device_type = format!("{:?}", adapter.device_type);
    let backend_api = format!("{:?}", adapter.backend);
    let driver = format!("{} {}", adapter.driver, adapter.driver_info);
    let report = format!(
        "{{\n  \"schema_version\":1,\n  \"backend\":\"wgpu\",\n  \"device\":{},\n  \"device_type\":{},\n  \"driver\":{},\n  \"backend_api\":{},\n  \"scene\":{},\n  \"width\":{width},\"height\":{height},\"samples\":{samples},\"bounces\":{bounces},\"seed\":{seed},\n  \"batch_size\":{batch},\"sync_per_batch\":true,\"warmup_samples\":{warmup},\"trials\":{trials},\n  \"device_setup_ms\":{device_setup_ms},\"setup_ms\":{setup_ms},\"warmup_ms\":{warmup_ms},\n  \"render_ms\":{render_ms:?},\"readback_ms\":{readback_ms:?},\"checksum\":{checksum:?}\n}}\n",
        json_string(&adapter.name),
        json_string(&device_type),
        json_string(&driver),
        json_string(&backend_api),
        json_string(&scene_name)
    );
    std::fs::write(&output, report)?;
    Ok(())
}
