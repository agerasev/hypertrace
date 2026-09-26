use std::{sync::mpsc, time::Duration};

use anyhow::Context;

use crate::Result;

/// A compute-capable headless device. Software adapters are allowed, but adapter
/// selection failure is an error rather than a silently skipped render/test.
pub struct Gpu {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Gpu {
    pub async fn headless() -> Result<Self> {
        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .context("no WGPU compute adapter available")?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("hypertrace compute device"),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            })
            .await
            .context("requesting a device with WebGPU compute limits")?;
        Ok(Self {
            adapter,
            device,
            queue,
        })
    }
}

/// Explicit, blocking snapshot for native tools/tests. Interactive rendering
/// never calls this: it presents directly from GPU accumulation storage.
pub fn read_buffer(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    source: &wgpu::Buffer,
    byte_len: u64,
) -> Result<Vec<u8>> {
    anyhow::ensure!(
        byte_len > 0 && byte_len.is_multiple_of(4),
        "invalid readback length"
    );
    // Finish producer submissions before scheduling the snapshot copy. Without
    // this boundary, first-frame readbacks on Intel MTL / Mesa 23.2.1 returned
    // partially updated data despite waiting for the copy and mapping callback.
    // Keep this synchronization confined to explicitly blocking CPU snapshots.
    device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: Some(Duration::from_secs(60)),
    })?;
    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("hypertrace snapshot staging"),
        size: byte_len,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(source, 0, &staging, 0, byte_len);
    let submission = queue.submit([encoder.finish()]);
    let (tx, rx) = mpsc::channel();
    staging.map_async(wgpu::MapMode::Read, .., move |result| {
        let _ = tx.send(result);
    });
    device.poll(wgpu::PollType::Wait {
        submission_index: Some(submission),
        timeout: Some(Duration::from_secs(60)),
    })?;
    rx.recv_timeout(Duration::from_secs(5))
        .context("readback callback missing")??;
    let bytes = staging.get_mapped_range(..)?.to_vec();
    staging.unmap();
    Ok(bytes)
}
