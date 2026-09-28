//! Reproducible native WGPU rendering without a window.
use anyhow::{Context, bail, ensure};
use hypertrace_wgpu::{Gpu, Renderer, Result};
use std::{
    fs::File,
    io::{BufWriter, Write},
    time::{Duration, Instant},
};

mod support;

fn main() -> Result<()> {
    let mut name = String::from("hy");
    let (mut width, mut height, mut samples, mut seed) = (320u32, 240u32, 64u32, 3735928559u32);
    let mut bounces = None;
    let mut output = String::from("wgpu");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--list-scenes" {
            println!("{}", support::catalog());
            return Ok(());
        }
        if arg == "--help" || arg == "-h" {
            println!(
                "headless [--scene NAME] [--width 320] [--height 240] [--samples 64] [--seed 3735928559] [--bounces 1..64] [--output PREFIX]\nWrites PREFIX.{{rgba32f,ppm,json}}; Use --list-scenes to see examples. WGPU_BACKEND selects a native backend."
            );
            return Ok(());
        }
        let value = args
            .next()
            .with_context(|| format!("missing value for {arg}"))?;
        match arg.as_str() {
            "--scene" => name = value,
            "--width" => width = value.parse()?,
            "--height" => height = value.parse()?,
            "--samples" => samples = value.parse()?,
            "--seed" => seed = value.parse()?,
            "--bounces" => bounces = Some(value.parse()?),
            "--output" => output = value,
            _ => bail!("unknown option {arg}"),
        }
    }
    ensure!(samples > 0, "samples must be positive");
    let mut scene = support::scene(&name)?;
    if let Some(b) = bounces {
        scene.bounces = b;
    }
    let bounces = scene.bounces;
    let curvature = scene.camera.transform().geometry().sign();
    let radius = scene.radius;
    let [red, green, blue, extinction] = scene.medium.gpu_row();
    let gpu = futures::executor::block_on(Gpu::headless())?;
    let adapter = gpu.adapter.get_info();
    println!(
        "WGPU adapter: {} ({:?}, {:?})",
        adapter.name, adapter.backend, adapter.device_type
    );
    let mut renderer = Renderer::new(&gpu.device, &gpu.queue, (width, height), scene, seed)?;
    let start = Instant::now();
    let mut remaining = samples;
    while remaining > 0 {
        let batch = remaining.min(16);
        renderer.set_samples_per_dispatch(batch)?;
        renderer.render();
        remaining -= batch;
        // Bound queued work for high-sample offline renders. Otherwise the
        // final snapshot's completion timeout includes the entire image.
        if (samples - remaining).is_multiple_of(128) {
            gpu.device.poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(Duration::from_secs(60)),
            })?;
            eprintln!("{name}: {}/{} samples", samples - remaining, samples);
        }
    }
    let pixels = renderer.snapshot()?;
    ensure!(
        pixels.iter().all(|p| p[3] == 1.0),
        "incomplete GPU frame: some pixels have no samples"
    );
    let elapsed = start.elapsed();
    let mut raw = BufWriter::new(File::create(format!("{output}.rgba32f"))?);
    let mut ppm = BufWriter::new(File::create(format!("{output}.ppm"))?);
    write!(ppm, "P6\n{width} {height}\n255\n")?;
    for pixel in pixels {
        for channel in pixel {
            raw.write_all(&channel.to_le_bytes())?;
        }
        for channel in &pixel[..3] {
            ppm.write_all(&[(channel.max(0.0).powf(1.0 / 2.2).min(1.0) * 255.0) as u8])?;
        }
    }
    raw.flush()?;
    ppm.flush()?;
    std::fs::write(
        format!("{output}.json"),
        format!(
            "{{\n  \"backend\": \"wgpu\",\n  \"adapter\": {:?},\n  \"scene\": {:?},\n  \"width\": {width},\n  \"height\": {height},\n  \"samples\": {samples},\n  \"seed\": {seed},\n  \"bounces\": {bounces},\n  \"curvature_sign\": {curvature},\n  \"curvature_radius\": {radius},\n  \"medium\": {{\"extinction\": {extinction}, \"albedo\": [{red}, {green}, {blue}]}},\n  \"linear_format\": \"little-endian rgba32f, row-major, top row first\",\n  \"display_gamma\": 2.2\n}}\n",
            adapter.name, name
        ),
    )?;
    println!(
        "WGPU {name}: {width}x{height}, {samples} samples, {bounces} bounces in {:.3}s; wrote {output}.{{rgba32f,ppm,json}}",
        elapsed.as_secs_f64()
    );
    Ok(())
}
