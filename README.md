# Hypertrace

Physically based non-Euclidean path tracer using WGPU compute shaders.
Rust scene builders compile to WGSL, with headless rendering and an interactive
viewer powered by [`wgame`](https://github.com/agerasev/wgame).

This project is primarily educational. Euclidean, hyperbolic, and spherical
geometries share a constant-curvature tracing kernel. Scene construction,
camera movement and GPU tracing all use embedded points and quaternion-pair
isometries. Hyperbolic points lie on the hyperboloid; half-space and Poincaré-ball
coordinates are explicit charts for construction and surface patterns.

[Scene gallery and web demos](https://agerasev.github.io/hypertrace/) ·
[Theory](https://agerasev.github.io/hypertrace/theory.html) ·
[Original video](https://www.youtube.com/watch?v=LGWusRNcJ6A)

## Requirements

- Current stable Rust and Cargo.
- Sibling `../vecmat-rs` and `../ccgeom` checkouts. Workspace patches use these
  sources for the shared geometry kernel; `ccgeom` also uses local `vecmat`.
  Compatible revisions and setup are recorded in [DEVELOPMENT.md](DEVELOPMENT.md).
- A native WGPU adapter with compute support. Software Vulkan can run the tests.
- A sibling `../wgame` checkout with `WindowConfig::required_limits` and
  `use_adapter_buffer_limits`. Cargo resolves this optional path dependency even
  for headless builds.

## Run

From the repository root:

```sh
cargo run --release -p hypertrace-examples --bin sp
cargo run --release -p hypertrace-gallery --bin headless -- --list-scenes
cargo run --release -p hypertrace-gallery --bin headless -- \
  --scene eu --width 640 --height 480 --samples 64 --output /tmp/eu
```

Each scene has its own binary: `eu`, `hy`, `sp`, and `sp-fog`. Each scene folder under
[`examples/src/bin`](examples/src/bin) owns its `scene.rs` and `main.rs`: scene
construction, renderer and presenter setup, and the Wgame event loop are visible
in that example. Start with [the spherical studio](examples/src/bin/sp) to copy
or adapt a complete application.

The separate [`hypertrace-gallery` package](examples/gallery) owns the optional
`viewer`, `headless`, and `benchmark` applications. Its `hypertrace_gallery`
library imports the example-owned scene files and supplies metadata and factories.
The `hypertrace-examples` package has no library target or gallery dependency;
standalone examples use `objects`, `ccgeom`, and `renderer` directly.

The gallery catalog includes Euclidean glass, hyperbolic tilings, a spherical
shadow studio, and one floorless spherical fog scene. Use `--list-scenes`
in a gallery tool to see the choices. Start with `sp` to explore a sun and two
balls touching a slightly transparent, mostly diffuse plane. The
[example guide](examples/README.md) explains what to observe and how the builders work.

Spherical scenes use emissive objects and a configurable black miss background.
Headless output includes linear RGBA floats, a PPM preview, and JSON settings.
The [renderer guide](renderer/README.md) covers the rendering API, scene
extensions, validation, and benchmarking.

### Controls

- Left-drag to look; scroll to zoom.
- WASD or arrow keys to move; Space/C to move up/down; Q/E to roll.
- R to restore the initial camera; Escape to exit.

Camera movement, scene changes, and resizing restart progressive accumulation.
Oversized windows render at a supported resolution and scale to the window.

## Web viewer

The gallery viewer runs the example-owned scenes in WebAssembly, using Wgame's
web runtime and WebGPU compute shaders. From the repository root:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked  # if not already installed; use Trunk 0.21 or newer
NO_COLOR=true trunk serve --release
```

Open <http://127.0.0.1:8080>. Use the grouped Example menu to select a scene,
or start with any catalog ID such as `?scene=sp` or
`?scene=sp-fog`. Selection reloads the page into the chosen typed application.
Each example includes a short description. Click the canvas to use
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

Whether a browser exposes WebGPU without flags depends on its platform and GPU
support. Native validation has covered software Vulkan and Intel Arc Vulkan; a
headed browser run of the shared renderer remains an outstanding platform check.
See [DEVELOPMENT.md](DEVELOPMENT.md) for validation commands and limitations.

To reproduce the high-quality previews and publish the gallery, video link,
theory page, and viewer together, see [site/README.md](site/README.md).

## Development

Curved scenes specify a physical curvature radius `R`; Euclidean scenes use one.
Hit distances and fog coefficients use physical world units. Spherical travel
distance includes complete circuits even when the ray returns to the same point.
The [geometry contract](GEOMETRY_CONTRACT.md) specifies units, transforms, tangent
frames, and precision limits; [ABOUT.md](ABOUT.md) describes the implementation.

The floorless spherical fog scene is available as `--scene sp-fog` and as the
standalone `sp-fog` binary. Its [scene construction](examples/src/bin/sp-fog/scene.rs)
uses two small emitters, opaque companions and a glass sphere, with extinction
`0.65` inverse world units and scattering albedo `[0.95; 3]`. Its [main function](examples/src/bin/sp-fog/main.rs) shows the
complete application setup. The simpler [vacuum studio](examples/src/bin/sp/scene.rs)
places two balls at opposite poles of the floor, with a small sun between them.

Fog samples a physical free-flight distance before resolving a surface miss;
that distance can span multiple spherical circuits. Independent transport tests
cover recurrence. See the [example guide](examples/README.md) and the
[renderer guide](renderer/README.md) for scene construction and custom components.

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy -p hypertrace-gallery --target wasm32-unknown-unknown --features web --bin viewer -- -D warnings
WGPU_BACKEND=vulkan cargo test --workspace -- --ignored --test-threads=1
```

GPU tests are opt-in and require a working compute adapter. See [ABOUT.md](ABOUT.md)
for the workspace structure and [TODO.md](TODO.md) for planned rendering features.
