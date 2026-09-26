# Hypertrace

Physically based non-Euclidean path tracer using WGPU compute shaders.
Rust scene builders compile to WGSL, with headless rendering and an interactive
viewer powered by [`wgame`](https://github.com/agerasev/wgame).

This project is primarily educational. Euclidean and hyperbolic geometries are
supported; spherical geometry is planned.

## Requirements

- Current stable Rust and Cargo.
- A native WGPU adapter with compute support. Software Vulkan can run the tests.
- A sibling `../wgame` checkout with `WindowConfig::required_limits` and
  `use_adapter_buffer_limits`. Cargo resolves this optional path dependency even
  for headless builds.

## Run

From the repository root:

```sh
cargo run --release -p hypertrace-wgpu --features viewer --example viewer -- --scene hy
cargo run --release -p hypertrace-wgpu --example headless -- \
  --scene eu --width 640 --height 480 --samples 64 --output /tmp/eu
```

Use `--scene eu` or `--scene hy`. Headless output includes linear RGBA floats, a
PPM preview, and JSON settings. The [renderer guide](renderer-wgpu/README.md)
covers the rendering API, scene extensions, validation, and benchmarking.

### Controls

- Left-drag to look; scroll to zoom.
- WASD or arrow keys to move; Space/C to move up/down; Q/E to roll.
- R to restore the initial camera; Escape to exit.

Camera movement, scene changes, and resizing restart progressive accumulation.
Oversized windows render at a supported resolution and scale to the window.

## Development

```sh
cargo test --workspace
cargo clippy --workspace --all-targets --features viewer -- -D warnings
WGPU_BACKEND=vulkan cargo test -p hypertrace-wgpu -- --ignored --test-threads=1
```

GPU tests are opt-in and require a working compute adapter. See [ABOUT.md](ABOUT.md)
for the workspace structure and [TODO.md](TODO.md) for planned rendering features.
