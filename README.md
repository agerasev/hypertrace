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
cargo run --release -p hypertrace-examples --bin headless -- --list-scenes
cargo run --release -p hypertrace-examples --bin headless -- \
  --scene eu --width 640 --height 480 --samples 64 --output /tmp/eu
```

Each scene has its own binary: `eu`, `hy`, `sp`, `sp-fog`, all `compare-*`
variants, `sp-loop`, and `sp-loop-fog`. The optional `viewer`, `headless`, and
`benchmark` applications provide a shared gallery and capture tools. These
applications live in `examples/` and depend on the rendering libraries.

The shared example catalog includes the original `eu`, `hy`, and `sp` scenes,
equal-layout curvature comparisons (`compare-eu`, `compare-hy`, `compare-sp`),
gentler-curvature variants, and spherical fog and long-route examples. Use
`--list-scenes` in any native tool to see all choices. Start with `compare-sp`
to see distant spheres grow again or `sp-loop` to see light arriving by the long
route around spherical space. The [example guide](examples/README.md) explains
what to observe and how the builders work.

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

The same viewer and scene builders run in WebAssembly, using Wgame's web runtime
and WebGPU compute shaders. From the repository root:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked  # if not already installed; use Trunk 0.21 or newer
NO_COLOR=true trunk serve --release
```

Open <http://127.0.0.1:8080>. Use the grouped Example menu to select a scene,
or start with any catalog ID such as `?scene=compare-sp` or
`?scene=sp-loop-fog`. Selection reloads the page into the chosen typed application.
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

The spherical studio's fog variant is available as `--scene sp-fog` and as a
Rust builder:

```rust,ignore
use objects::Scene as _;
let mut source = hypertrace_examples::sp::fog_scene::<12>();
source.medium = objects::shader::Medium {
    extinction: 0.08, // inverse world units
    albedo: [0.85, 0.9, 0.95],
};
let definition = source.definition()?;
let scene = hypertrace_renderer::Scene::from_definition(&definition)?;
```

The CLI's `sp` selection uses the vacuum version. The floorless `sp-loop-fog`
example allows rays missing every object to scatter after multiple spherical
circuits. Fog samples a physical free-flight distance before resolving a surface
miss. See the [example guide](examples/README.md) for shared curvature and
recurrence builders, and the [renderer guide](renderer/README.md) for
custom shapes, materials and their shader modules.

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy -p hypertrace-examples --target wasm32-unknown-unknown --features web --bin viewer -- -D warnings
WGPU_BACKEND=vulkan cargo test --workspace -- --ignored --test-threads=1
```

GPU tests are opt-in and require a working compute adapter. See [ABOUT.md](ABOUT.md)
for the workspace structure and [TODO.md](TODO.md) for planned rendering features.
