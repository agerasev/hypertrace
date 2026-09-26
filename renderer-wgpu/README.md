# Native WGPU backend

The renderer compiles the generic Rust `eu` and `hy` scene builders to WGSL
compute shaders. It includes headless tools and a `wgame` viewer with direct GPU
presentation.

## Run

Use current stable Rust, a compute-capable native WGPU adapter (software Vulkan
is sufficient for correctness tests), and the sibling `../../wgame` checkout
with configurable `WindowConfig::required_limits` and `use_adapter_buffer_limits`.
WGPU is pinned to the same
major version as that checkout, 30. The optional path dependency still needs to
resolve even in a headless Cargo build.

From the Hypertrace repository root:

```sh
cargo run --release -p hypertrace-wgpu --features viewer --example viewer -- --scene hy
cargo run --release -p hypertrace-wgpu --example headless -- \
  --scene hy --width 320 --height 240 --samples 64 --seed 3735928559 --output /tmp/hy-wgpu
```

The viewer supports WASD/arrows, Space/C, Q/E, left-drag to look, scroll to zoom,
R to restore the initial camera, and Escape to close. Movement integrates actual
elapsed time. Camera/scene changes and resizing reset progressive accumulation.
The viewer requests the adapter's supported buffer sizes. If a window exceeds
those limits, it reduces render resolution proportionally and scales the image
to the window. Returning to a supported size restores full resolution. The
console reports when this fallback is entered or left. Zero-sized frames are
skipped. `--smoke` uses a small binding limit and runs twelve frames, resizing
across that limit and back with camera updates, then exits.

Headless output is `PREFIX.rgba32f` (linear, normalized little-endian RGBA,
top row first), `PREFIX.ppm` (gamma 1/2.2), and `PREFIX.json` (settings).
`--bounces` overrides the scene default. Adapter details are printed; timings on
software Vulkan are not hardware performance measurements.
Headless rendering also requests supported buffer sizes, but preserves the exact
requested image dimensions: an unsupported size returns a diagnostic error.

## Renderer contract

`Renderer::new` accepts an existing device and queue. Request
`wgpu::Limits::default()` or equivalent compute/storage limits; `wgame`'s default
WebGL2 limits disable compute. No optional WGPU features are required.
The portable storage-binding default is 128 MiB, enough for 8,388,608 pixels at
16 bytes per accumulated pixel; a 3840×2400 window needs 140.625 MiB. In `wgame`,
use `.required_limits(wgpu::Limits::default()).use_adapter_buffer_limits(true)`
to request the selected adapter's buffer capacities. This does not allocate those
capacities in advance. Device limits cannot be raised after device creation.

Dividing a CPU upload into chunks does not bypass a storage-binding limit. The
shader binds the accumulation buffer as one resource. Rendering exact images
beyond the adapter's limit would require tiled storage/dispatch/presentation or
a texture-based accumulation design. For a window, `fit_render_size` provides a
bounded render resolution and `Presenter` scales it to the attachment.

- `encode` adds compute work to a caller-owned command encoder.
- `Presenter::draw` adds a fullscreen pass after compute. Normal frames perform
  no CPU pixel readback or upload. Both linear and sRGB target formats apply the
  display transform once. If attachment and render sizes differ, presentation
  scales the image with nearest-neighbor sampling.
- `update_scene` uploads values and recompiles only when shader structure changes;
  scene buffers are reused while their capacity is sufficient. A shader validation
  failure leaves the old renderer usable. `shader_source()` exposes the flattened
  program, and `pipeline_revision()` reports successful program replacements.
- `resize` replaces pixel buffers. Rebind an existing `Presenter` afterwards.
  It requires an exact supported size; a window host can call `fit_render_size`
  first. Zero-sized frames should be skipped by the window host.
- `reset` queues accumulation/seed writes separately from frame submission, so a
  discarded window frame cannot accidentally discard a needed reset.
- Submit encoded work before changing parameters/resetting: WGPU queue writes
  take effect before the next submission, not at the point where `encode` ran.
- `snapshot` explicitly waits and reads normalized linear pixels for tools/tests.
  It waits for producer work before submitting its copy: native Intel MTL with
  Mesa 23.2.1 returned partially updated first snapshots without that boundary.
  Interactive presentation has no such CPU wait.

Host camera composition remains f64; upload converts to f32. Hyperbolic geometry
remains the upper half-space model, with complex 2×2 Möbius matrices. Quaternions
are temporary values for the action and tangent transport, not stored isometries.
Nearby distance evaluation, vertical horosphere intersections, and plane hits
near the ideal boundary are stabilized in both backends. Upload rejects transforms
that become singular after f32 conversion (for example, a horizontal boost of
18 units). This does not remove all f32 limitations near the ideal boundary;
camera-relative coordinate frames remain a separate implementation step.

## Generic scene builders

`hypertrace-scenes` contains the shared `eu` and `hy` factories used by both
backends. The WGPU examples lower those builders as follows:

```rust,ignore
use objects::Scene as _;
let definition = scenes::hy::scene::<3>().wgsl_scene()?;
let scene = hypertrace_wgpu::Scene::from_definition(&definition)?;
let renderer = hypertrace_wgpu::Renderer::new(&device, &queue, (640, 480), scene, 1)?;
```

`hypertrace-scene` is a CPU-only intermediate representation and WGSL compiler.
It has no graphics runtime dependency. The `objects` traits provide fallible
lowering hooks; types without an implementation report an unsupported error.
The renderer also accepts this intermediate representation directly.

Supported compositions include `SceneImpl`, point and mapped views, constant and
Euclidean gradient backgrounds, covered and mapped objects, object choices and
vectors, shape choices and vectors, mapped shapes, nested mixtures, `Colored`,
and `Emissive`. Primitive shapes and tilings retain the preceding backend stage's
coverage. Maps support Euclidean shifts, rotations and homogeneous rigid maps,
and hyperbolic complex Möbius maps; other maps report an unsupported error.

Shader schemas describe composition independently of values. Choice variants and
empty vector element types are registered before generation, so switching a choice
or resizing a vector preserves the program. Object records and material descriptors
index a separate `u32` parameter arena. Each shape leaf has a distinct identity
for repeated-hit suppression, including leaves in a shape vector. Shape mapping
and object mapping retain their different material coordinate frames.

Update generated objects and materials by lowering a new definition and passing
it to `update_scene`. Direct edits to the legacy raw-record fields of a generated
`Scene` are rejected, keeping shader IDs and parameter offsets consistent. Camera
pose, field of view, background and bounce count remain ordinary scene values.

Materials execute in the original nesting order: a mixture draws and subtracts
weights in order, `Colored` changes throughput before calling its child, and
`Emissive` adds light before calling its child. Nested mixtures keep their own
random draws. The previous fixed-record renderer remains available through
`Scene::eu()` / `Scene::hy()` as a comparison fixture.

### Custom shader leaves

Return `MaterialValue::custom` or `ShapeValue::custom` from a lowering hook (or
construct them directly). `ShaderLeaf` contains a unique implementation key,
WGSL source, entry point, and fixed parameter word count. No central dispatch
enum needs editing. Helpers `load_u32`, `load_f32`, `load_vec3` and `load_vec4`
read relative word offsets in the parameter arena.

Material entry points use this signature:

```wgsl
fn my_material(base: u32, ctx: MaterialContext,
    sample: ptr<function, MaterialSample>, rng: ptr<function, u32>) {
    (*sample).emission += (*sample).attenuation * load_vec3(base);
    (*sample).alive = 0u;
}
```

`MaterialContext` supplies object-local position and normal. `MaterialSample`
supplies direction, path throughput (`attenuation`), accumulated emission and
an `alive` flag. `uniform_random(rng)` advances the same generator as built-ins.
Shape entry points take `(base: u32, ray: Ray, previous_identity: u32)` and return
`TaggedHit { hit: Hit, identity: u32 }`; leaf wrappers assign their unique identity.
Use unique WGSL helper names as well as a unique leaf key. Invalid WGSL is reported
when a renderer creates or updates its pipeline, with the flattened source
available from the compiler output for diagnostics.

## Validation

Ordinary tests do not require an adapter. GPU tests are ignored by default and
must be explicitly requested; missing adapters fail rather than silently skip.

```sh
cargo test -p hypertrace-wgpu
WGPU_BACKEND=vulkan cargo test -p hypertrace-wgpu -- --ignored
cargo clippy --no-deps -p hypertrace-wgpu --all-targets --features viewer -- -D warnings
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-wgpu --features viewer --example viewer -- --scene hy --smoke
```

Coverage includes Möbius matrix ordering, inverse/distance/derivative checks,
small and scaled distances, vertical and nearly vertical rays, hit/miss cases,
reset, resize, scene uploads, deterministic batching, captured tile-selection
fixtures, and presentation transfer and orientation. Generic scene tests cover
nested materials, custom leaves, empty vectors, choices, repeated-hit identities,
and recovery after shader compilation or resource-limit errors.

### Compare rendered frames

Render two frames with matching scene, dimensions, sample count, seed, and bounce
count using the `headless` example. Compare their linear outputs before display
conversion:

```sh
python3 tools/compare_frames.py /tmp/hy-a /tmp/hy-b \
  --diff /tmp/hy-difference.ppm --diff-scale 4 --report /tmp/hy-comparison.json
```

Output is normalized little-endian f32 RGBA, top row first, with alpha one after
rendering. Matching JSON settings files are required. The comparison tool checks
metadata, file lengths, and finite values and reports RGB mean absolute error,
RMSE, and maximum error. `--max-error 0` checks deterministic reruns on the same
adapter. Other optional limits are `--max-mae` and `--max-rmse`; choose tolerances
for the workload. Driver rounding can change silhouettes or tile decisions and
later random paths, so cross-device comparisons need not be pixelwise identical.

## Performance measurements

Use the release-mode `benchmark` example for completed-render timings. Run
configurations sequentially with identical scene, dimensions, samples, seed, and
bounce limit. Defaults are four bounces for `eu` and three for `hy`.

```sh
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-wgpu --example benchmark -- \
  --scene hy --width 1280 --height 720 --samples 16 --warmup 64 --trials 10 \
  --batch 1 --seed 3735928559 --output /tmp/wgpu-batch1.json
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-wgpu --example benchmark -- \
  --scene hy --width 1280 --height 720 --samples 16 --warmup 64 --trials 10 \
  --batch 16 --seed 3735928559 --output /tmp/wgpu-batch16.json
python3 tools/compare_benchmarks.py /tmp/wgpu-batch1.json /tmp/wgpu-batch16.json \
  --output /tmp/benchmark-comparison.md --json /tmp/benchmark-comparison.json
```

Each trial resets accumulation and seeds and finishes those uploads before the
timer starts. Render time includes submission and a completion wait after each
batch; setup, warmup, reset, and readback are excluded. This is synchronized
render latency, not GPU timestamp profiling or viewer FPS. `--warmup` counts
samples. Larger batches can improve throughput while delaying input response.

Readback includes staging allocation, transfer, host copying, normalization, and
validation. The viewer presents directly on the GPU and avoids this cost.
JSON reports record the adapter, driver, all trial times, setup/warmup times,
and RGB sums. Sums are diagnostics, not image-parity tests. Startup timings
depend on driver caching and deferred compilation. Always check device identity:
software Vulkan results measure CPU performance, and timings across different
GPUs do not isolate renderer changes.

## Current limitations

1. Additional geometry, view, background, and map types need lowering hooks;
   unsupported types produce explicit errors.
2. GPU f32 arithmetic still loses precision near the ideal boundary. Additional
   camera-path tests and relative-coordinate rendering remain useful work.
3. Workgroup sizes and sample batching remain workload/device choices; the viewer
   uses a single sample per frame for responsive camera movement.
4. Browser execution needs a high-level WebGPU path. Wgame's current web feature
   uses WebGL2 and cannot run this compute renderer.
