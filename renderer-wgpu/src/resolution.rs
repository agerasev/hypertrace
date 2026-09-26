//! Resolution bounds for the per-pixel accumulation and random-state buffers.

use crate::Result;

const ACCUMULATION_BYTES_PER_PIXEL: u64 = 16;
const WORKGROUP_SIDE: u32 = 8;

fn pixel_capacity(limits: &wgpu::Limits) -> u64 {
    (limits
        .max_storage_buffer_binding_size
        .min(limits.max_buffer_size)
        / ACCUMULATION_BYTES_PER_PIXEL)
        // Shader indices and the seed initialization sequence use u32.
        .min(u64::from(u32::MAX))
}

/// Fit a window's physical dimensions to the renderer's device limits.
///
/// Returns the original dimensions when supported. Otherwise, scales both
/// dimensions down together, rounding to whole pixels and retaining at least
/// one pixel on each axis. The presenter can scale this image to the window.
/// This bounds individual resources; it is not a guarantee of available VRAM.
/// Zero dimensions should be skipped by the window host.
pub fn fit_render_size(limits: &wgpu::Limits, requested: (u32, u32)) -> Result<(u32, u32)> {
    anyhow::ensure!(
        requested.0 > 0 && requested.1 > 0,
        "image dimensions must be positive"
    );
    let capacity = pixel_capacity(limits);
    let max_axis = limits
        .max_compute_workgroups_per_dimension
        .saturating_mul(WORKGROUP_SIDE);
    anyhow::ensure!(
        capacity > 0 && max_axis > 0,
        "device cannot render a single pixel"
    );
    let pixels = u64::from(requested.0) * u64::from(requested.1);
    let scale = (capacity as f64 / pixels as f64)
        .sqrt()
        .min(f64::from(max_axis) / f64::from(requested.0))
        .min(f64::from(max_axis) / f64::from(requested.1))
        .min(1.0);
    let mut width = (f64::from(requested.0) * scale).floor().max(1.0) as u32;
    let mut height = (f64::from(requested.1) * scale).floor().max(1.0) as u32;
    // A very thin image may have a scaled axis below one pixel. Clamping that
    // axis to one can exceed the area budget, so constrain the other axis too.
    if u64::from(width) * u64::from(height) > capacity {
        if width >= height {
            width = (capacity / u64::from(height)) as u32;
        } else {
            height = (capacity / u64::from(width)) as u32;
        }
    }
    validate_render_size(limits, (width, height))?;
    Ok((width, height))
}

pub(crate) fn validate_render_size(limits: &wgpu::Limits, size: (u32, u32)) -> Result<()> {
    anyhow::ensure!(
        size.0 > 0 && size.1 > 0,
        "image dimensions must be positive"
    );
    let pixels = u64::from(size.0) * u64::from(size.1);
    let bytes = pixels
        .checked_mul(ACCUMULATION_BYTES_PER_PIXEL)
        .ok_or_else(|| anyhow::anyhow!("image dimensions overflow buffer size"))?;
    let limit = limits
        .max_storage_buffer_binding_size
        .min(limits.max_buffer_size);
    anyhow::ensure!(
        bytes <= limit,
        "image {}x{} requires {} bytes for accumulation, exceeding the device storage buffer limit of {} bytes (binding {}, allocation {}); request larger supported device limits or reduce the render resolution",
        size.0,
        size.1,
        bytes,
        limit,
        limits.max_storage_buffer_binding_size,
        limits.max_buffer_size
    );
    anyhow::ensure!(
        pixels <= u64::from(u32::MAX),
        "image exceeds u32 pixel indexing"
    );
    anyhow::ensure!(
        size.0.div_ceil(WORKGROUP_SIDE) <= limits.max_compute_workgroups_per_dimension
            && size.1.div_ceil(WORKGROUP_SIDE) <= limits.max_compute_workgroups_per_dimension,
        "image {}x{} exceeds the device compute dispatch limit of {} workgroups per axis",
        size.0,
        size.1,
        limits.max_compute_workgroups_per_dimension
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_limit_boundary_and_large_windows() {
        let limits = wgpu::Limits::default();
        for size in [(1, 1), (1920, 1080), (3840, 2160), (4096, 2048)] {
            assert_eq!(fit_render_size(&limits, size).unwrap(), size);
        }
        for size in [(4096, 2049), (3840, 2400), (5120, 2880), (7680, 4320)] {
            let fitted = fit_render_size(&limits, size).unwrap();
            assert!(fitted.0 < size.0 && fitted.1 < size.1);
            validate_render_size(&limits, fitted).unwrap();
            let scale_x = f64::from(fitted.0) / f64::from(size.0);
            let scale_y = f64::from(fitted.1) / f64::from(size.1);
            assert!((scale_x - scale_y).abs() <= 1.0 / f64::from(size.0.min(size.1)));
        }
        let error = validate_render_size(&limits, (3840, 2400))
            .unwrap_err()
            .to_string();
        assert!(error.contains("147456000") && error.contains("134217728"));
        let larger = wgpu::Limits {
            max_storage_buffer_binding_size: 512 << 20,
            max_buffer_size: 512 << 20,
            ..limits
        };
        assert_eq!(
            fit_render_size(&larger, (5120, 2880)).unwrap(),
            (5120, 2880)
        );
    }

    #[test]
    fn fitting_handles_allocation_dispatch_and_thin_images() {
        let limits = wgpu::Limits {
            max_buffer_size: 16 * 64,
            max_compute_workgroups_per_dimension: 2,
            ..Default::default()
        };
        for requested in [
            (u32::MAX, 1),
            (1, u32::MAX),
            (u32::MAX, u32::MAX),
            (900, 700),
            (16, 4),
            (1, 1),
        ] {
            let fitted = fit_render_size(&limits, requested).unwrap();
            validate_render_size(&limits, fitted).unwrap();
            assert!(fitted.0 <= requested.0 && fitted.1 <= requested.1);
        }
        assert_eq!(fit_render_size(&limits, (16, 4)).unwrap(), (16, 4));
        assert_eq!(fit_render_size(&limits, (u32::MAX, 1)).unwrap(), (16, 1));
        assert!(fit_render_size(&limits, (0, 20)).is_err());
        let unusable = wgpu::Limits {
            max_buffer_size: 15,
            ..limits
        };
        assert!(fit_render_size(&unusable, (1, 1)).is_err());
    }
}
