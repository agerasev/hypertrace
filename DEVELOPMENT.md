# Development

The workspace uses current stable Rust, local geometry libraries, and WGPU/WGSL.
Its CPU scene compiler does not require a graphics adapter. Native GPU tests need
a compute-capable adapter; software Vulkan is sufficient for correctness checks.

## Companion repositories

Keep these repositories next to `hypertrace/`. The checked revisions are also
pinned in [.travis.yml](.travis.yml); update both places when changing them.

| Directory | Repository | Compatible revision |
| --- | --- | --- |
| `../vecmat-rs` | [vecmat-rs](https://github.com/agerasev/vecmat-rs) | `2e43fd9dce251658a217a8556ba2695f83b36fdd` |
| `../ccgeom` | [ccgeom](https://github.com/agerasev/ccgeom) | `902af7e33d9eb0aeae7bf916ab838d364b4ec65b` |
| `../wgame` | [wgame](https://github.com/agerasev/wgame) | `42858dc961e1ac27a56a5cc84572e7489d036abc` |

The workspace patches crates.io `ccgeom` and `vecmat` to these checkouts, and
`ccgeom` also uses local `vecmat`. Standalone examples depend on Wgame directly;
the gallery enables it through its optional viewer feature. Cargo resolves that
path even for headless workspace builds. Wgame must expose `WindowConfig::required_limits` and
`use_adapter_buffer_limits` and use the same WGPU major version as Hypertrace.
Companion commits must be available on their remotes before remote CI can fetch
them. `Cargo.lock` is currently ignored by this repository.

## Example ownership

Each demonstration under `examples/src/bin/<name>/` owns its scene construction
in `scene.rs` and its application setup in `main.rs`. The entry point directly
creates the scene, `Renderer`, `Presenter`, and Wgame event loop; there is no
common example runner. Keep each folder usable as an independent tutorial.

The `hypertrace-examples` package has no library target. The separate
`hypertrace-gallery` package in `examples/gallery/` owns the optional viewer,
headless, and benchmark tools. Its `hypertrace_gallery` library imports the
example-owned scene files. Add metadata to
[`examples/gallery/src/catalog.rs`](examples/gallery/src/catalog.rs) only when
an example should appear in those tools. Standalone binaries must not depend on
the gallery or another example. Keep application-level integration tests in
`examples/gallery/tests`; avoid tests that pin demonstration layouts or tuning.

## Checks

Run from the Hypertrace root:

```sh
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo clippy --workspace --all-targets -- -D warnings
python3 -m unittest discover -s tools -p 'test_*.py'
node examples/gallery/web/controls.test.cjs
```

GPU tests are ignored by ordinary Cargo test runs. They fail if no suitable
adapter is available, so a passing CPU run is not GPU validation. Run GPU suites
sequentially, recording the adapter and driver in each run:

```sh
WGPU_BACKEND=vulkan cargo test --workspace -- --ignored --test-threads=1
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-gallery --bin viewer -- --scene spherical --smoke
```

The viewer smoke path exercises camera motion, accumulation resets, resizing
across storage-binding limits, presentation, and cursor lock/release. Exercise
the Euclidean and hyperbolic examples as well when changing their construction
or geometry.
The [example guide](examples/README.md) describes the lighting and material demonstrations.

For the web viewer, install `wasm32-unknown-unknown` and Trunk 0.21 or newer:

```sh
cargo clippy -p hypertrace-gallery --target wasm32-unknown-unknown --features web --bin viewer -- -D warnings
NO_COLOR=true trunk build --release
```

The HTML asset in `examples/gallery/web` points to the gallery package and selects
`data-bin="viewer"`, so Trunk builds only the web gallery binary. Native headless
and benchmark tools are separate binaries.

Build success does not establish browser rendering correctness. A headed browser
check should exercise scene selection, camera input, resizing, accumulation, and
error reporting with WebGPU enabled. See [README.md](README.md#web-viewer) for
serving the viewer and [site/README.md](site/README.md) for publishing the gallery.

## Numerical and rendering checks

Preserve the conventions and explicit precision limits in
[GEOMETRY_CONTRACT.md](GEOMETRY_CONTRACT.md). Independent analytic references,
CPU/GPU comparisons, and physical-distance tests are the correctness gates.
Image comparisons complement them: stochastic paths and driver rounding can
change later samples without indicating a geometry regression.

Use `tools/compare_frames.py` for matching linear captures and
`tools/compare_benchmarks.py` for comparable timing reports. The
[renderer guide](renderer/README.md#compare-rendered-frames) documents their
inputs. Run timing workloads sequentially on the same adapter. Software Vulkan
measures CPU-renderer performance; do not report it as hardware GPU throughput.
Temporary captures are disposable; durable evidence should include source
revisions, options, adapter/driver, and hashes alongside the files.

## Outstanding limitations

- Native validation has covered llvmpipe Vulkan and Intel Arc Vulkan. A headed
  browser run of the shared embedded renderer and additional GPU drivers remain
  platform checks to complete.
- Camera-relative f64 preparation improves nearby precision, but GPU tracing is
  f32. Later path-segment recentering and a wider hyperbolic numerical range remain
  open. Numerical termination preserves prior emission and adds no background;
  it and the finite interaction budget introduce truncation bias.
- The physical-distance accumulator's range does not certify coordinate accuracy
  over that range. Curved sphere radii must yield a resolvable f32 section equation.
- Large-scene camera-update cost needs a dedicated benchmark. Workgroup sizes and
  sample batching remain workload/device choices.
- The scene-wide homogeneous medium also occupies refractive objects. Medium
  boundaries are needed to exclude fog from glass. Small emitters and volumetric
  caustics converge slowly with the current camera-path sampler.
- Heterogeneous media, anisotropic scattering, triangle
  geometry, acceleration structures, and importance sampling remain extensions.
