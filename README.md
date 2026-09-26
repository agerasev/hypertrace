# Hypertrace

Physically based non-Euclidean path tracer using WGPU compute shaders.
Rust scene builders compile to WGSL, with headless rendering and an interactive
viewer powered by [`wgame`](https://github.com/agerasev/wgame).

This project is primarily educational. Euclidean and hyperbolic geometries are
supported; spherical geometry is planned.

[Scene gallery and web demos](https://agerasev.github.io/hypertrace/) ·
[Theory](https://agerasev.github.io/hypertrace/theory.html) ·
[Original video](https://www.youtube.com/watch?v=LGWusRNcJ6A)

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

## Web viewer

The same viewer and scene builders run in WebAssembly, using Wgame's web runtime
and WebGPU compute shaders. From the repository root:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked  # if not already installed; use Trunk 0.21 or newer
NO_COLOR=true trunk serve --release
```

Open <http://127.0.0.1:8080>. Use the Space menu to switch between hyperbolic and
Euclidean scenes, or start with `?scene=hy` / `?scene=eu`. Click the canvas to use
the camera controls above. Escape toggles pause in the browser. Quality caps the
longest render dimension (Fast: 640, Balanced: 960, Sharp: 1440 pixels); Full
resolution follows the canvas size, subject to device limits. The default is
Balanced to bound the work on large or high-DPI displays.

```sh
NO_COLOR=true trunk build --release
```

The resulting `dist/` directory is a static site: serve it over HTTPS, or HTTP on
localhost for development. Opening the HTML as a local file is unsupported.
A browser and GPU with WebGPU support are required; WebGL cannot run these compute
shaders. Unsupported browsers show a message in the page. No Wgame changes are
needed: Hypertrace enables WGPU's WebGPU feature alongside Wgame's web runtime.
Trunk downloads a matching `wasm-bindgen` tool on the first build if needed.

Browser validation was performed in headed Chromium on the local Intel GPU,
with `--enable-unsafe-webgpu` to enable WebGPU on that Linux setup. Whether a
browser exposes WebGPU without flags depends on its platform and GPU support.

To reproduce the high-quality previews and publish the gallery, video link,
theory page, and viewer together, see [site/README.md](site/README.md).

## Development

```sh
cargo test --workspace
cargo clippy --workspace --all-targets --features viewer -- -D warnings
cargo clippy -p hypertrace-wgpu --target wasm32-unknown-unknown --features web --example viewer -- -D warnings
WGPU_BACKEND=vulkan cargo test -p hypertrace-wgpu -- --ignored --test-threads=1
```

GPU tests are opt-in and require a working compute adapter. See [ABOUT.md](ABOUT.md)
for the workspace structure and [TODO.md](TODO.md) for planned rendering features.
