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
- A native WGPU adapter with compute support. Software Vulkan can run the tests.

Cargo fetches `vecmat`, `ccgeom`, and `wgame` from crates.io; no sibling checkouts
are required. See [DEVELOPMENT.md](DEVELOPMENT.md) for dependency versions and checks.

## Run

From the repository root:

```sh
cargo run --release -p hypertrace-examples --bin spherical
cargo run --release -p hypertrace-gallery --bin headless -- --list-scenes
cargo run --release -p hypertrace-gallery --bin headless -- \
  --scene euclidean --width 640 --height 480 --samples 64 --output /tmp/euclidean
```

Each scene has its own binary: `euclidean`, `hyperbolic`, `spherical`, `fog`, and `ball-tilings`.
Each scene folder under
[`examples/src/bin`](examples/src/bin) owns its `scene.rs` and `main.rs`: scene
construction, renderer and presenter setup, and the Wgame event loop are visible
in that example. Start with [the spherical studio](examples/src/bin/spherical) to copy
or adapt a complete application.

The separate [`hypertrace-gallery` package](examples/gallery) owns the optional
`viewer`, `headless`, and `benchmark` applications. Its `hypertrace_gallery`
library imports the example-owned scene files and supplies metadata and factories.
The `hypertrace-examples` package has no library target or gallery dependency;
standalone examples use `objects`, `ccgeom`, and `renderer` directly.

The gallery catalog includes Euclidean glass, hyperbolic tilings, a spherical
shadow studio, one floorless Euclidean fog scene, and a showcase of seven ball
tilings. Run `cargo run --release -p hypertrace-examples --bin ball-tilings` to
see the five Platonic patterns, eight lunes and two hemispheres together.
Use `--list-scenes` in a gallery tool to see the choices. Start with `spherical` to explore an off-center sun and
diffuse, refractive, and glowing balls on a slightly transparent, mostly diffuse plane. The
[example guide](examples/README.md) explains what to observe and how the builders work.

Spherical scenes use emissive objects and a configurable black miss background.
Headless output includes linear RGBA floats, a PPM preview, and JSON settings.
The [renderer guide](renderer/README.md) covers the rendering API, scene
extensions, validation, and benchmarking.

### Controls

- Left-drag to look, or press Tab to lock/unlock the mouse; scroll to zoom.
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
or start with any catalog ID such as `?scene=spherical` or
`?scene=fog`. Selection reloads the page into the chosen typed application.
Each example includes a short description. Click the canvas to use
the camera controls above. Tab locks/unlocks the mouse while the canvas is focused.
Escape releases browser mouse lock; when unlocked it toggles pause. Quality caps the
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

The floorless Euclidean fog scene is available as `--scene fog` and as the
standalone `fog` binary. Its [scene construction](examples/src/bin/fog/scene.rs)
uses one bright emitter surrounded by red diffuse, green reflective, and blue
refractive spheres. Light intensity, extinction and scattering albedo are adjustable
directly in that source. Its [main function](examples/src/bin/fog/main.rs) shows
the complete application setup. The [vacuum studio](examples/src/bin/spherical/scene.rs)
keeps two balls at opposite floor poles, with asymmetric sunlight, a nearby blue
ball, and a glowing sphere intersecting the floor.

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
