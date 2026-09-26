# Native WGPU backend

This migration stage renders the existing generic Rust `eu` and `hy` scene
builders with generated WGSL compute shaders. It includes a headless renderer
and a `wgame` viewer with direct GPU presentation. `hearth` is outside the
migration scope.

## Run

Use current stable Rust, a compute-capable native WGPU adapter (software Vulkan
is sufficient for correctness tests), and the sibling `../../wgame` checkout
with configurable `WindowConfig::required_limits`. WGPU is pinned to the same
major version as that checkout, 30. No OpenCL or SDL libraries are needed for
this package. The optional path dependency still needs to resolve even in a
headless Cargo build.

From the Hypertrace repository root:

```sh
cargo run -p hypertrace-wgpu --features viewer --example viewer -- --scene hy
cargo run -p hypertrace-wgpu --example headless -- \
  --scene hy --width 320 --height 240 --samples 64 --seed 3735928559 --output /tmp/hy-wgpu
```

The viewer supports WASD/arrows, Space/C, Q/E, left-drag to look, scroll to zoom,
R to restore the initial camera, and Escape to close. Movement integrates actual
elapsed time. Camera/scene changes and resizing reset progressive accumulation.
`--smoke` runs twelve frames with a resize and camera update, then exits.

Headless output is `PREFIX.rgba32f` (linear, normalized little-endian RGBA,
top row first), `PREFIX.ppm` (gamma 1/2.2), and `PREFIX.json` (settings).
`--bounces` overrides the scene default. Adapter details are printed; timings on
software Vulkan are not hardware performance measurements.

## Renderer contract

`Renderer::new` accepts an existing device and queue. Request
`wgpu::Limits::default()` or equivalent compute/storage limits; `wgame`'s default
WebGL2 limits disable compute. No optional WGPU features are required.

- `encode` adds compute work to a caller-owned command encoder.
- `Presenter::draw` adds a fullscreen pass after compute. Normal frames perform
  no CPU pixel readback or upload. Both linear and sRGB target formats apply the
  display transform once.
- `update_scene` uploads values and recompiles only when shader structure changes;
  scene buffers are reused while their capacity is sufficient. A shader validation
  failure leaves the old renderer usable. `shader_source()` exposes the flattened
  program, and `pipeline_revision()` reports successful program replacements.
- `resize` replaces pixel buffers. Rebind an existing `Presenter` afterwards.
  Zero-sized frames should be skipped by the window host.
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
It contains no WGPU or OpenCL dependency. The `objects` traits provide fallible
lowering hooks with unsupported defaults, so existing OpenCL-only custom types
still compile. The renderer also accepts this intermediate representation directly.

Supported compositions include `SceneImpl`, point and mapped views, constant and
Euclidean gradient backgrounds, covered and mapped objects, object choices and
vectors, shape choices and vectors, mapped shapes, nested mixtures, `Colored`,
and `Emissive`. Primitive shapes and tilings retain the preceding backend stage's
coverage. Maps support Euclidean shifts, rotations and homogeneous rigid maps,
and hyperbolic complex Möbius maps; other legacy maps report an unsupported error.

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
WGPU_BACKEND=vulkan cargo run -p hypertrace-wgpu --features viewer --example viewer -- --scene hy --smoke
```

Coverage includes Möbius matrix ordering, inverse/distance/derivative checks,
small and scaled distances, vertical and nearly vertical rays, hit/miss cases,
reset, resize, scene uploads, deterministic batching, OpenCL tile-selection
fixtures, and presentation transfer and orientation. The OpenCL baseline also
has independent analytic regressions:

```sh
POCL_KERNEL_CACHE=0 cargo test -p hypertrace-kernel --test hyperbolic_opencl -- --ignored
```

The generic path additionally checks nested material evaluation, custom leaves,
choice and vector updates (including empty vectors), repeated-hit identities,
and recovery after shader compilation or resource-limit errors. Run the CPU
compiler and builder checks with `cargo test -p hypertrace-scene -p hypertrace-objects`.
The legacy crates retain existing lint warnings, hence `--no-deps` above.

For image comparisons, follow the [reference runner guide](../main/examples/REFERENCE.md)
and use matching scene, resolution, sample count, seed, and bounce count. Compare
linear floats before display conversion. GPU compiler rounding can change a
silhouette or tile decision, which then changes later random paths; exact image
identity is not a general acceptance criterion.

The generated path was checked on software Vulkan and Intel Arc MTL (Mesa
23.2.1). At 256×192, 16 samples and seed 3735928559, the native generated images
had mean absolute RGB errors of 0.000181 (`eu`, four bounces) and 0.000451 (`hy`,
three bounces) against the PoCL reference. Maximum channel errors were 0.211
and 0.442 respectively; small global averages do not imply pixelwise equality.
These are correctness comparisons at the initial camera poses, not performance
benchmarks or exhaustive camera-path validation.

## Remaining migration work

The current typed storage records and shader dispatch cover the built-in scenes:
mapped Euclidean plane/sphere/cube, hyperbolic plane/horosphere, local materials,
and square/hexagonal/pentagonal/pentastar tilings. Record sizes and matrix order
are explicit and do not reuse OpenCL's C layout.

Remaining gates before making WGPU the default:

1. Extend lowering to additional geometry, view, background and map implementations
   as needed; unsupported implementations currently produce explicit errors.
2. Expand analytic/property tests and image comparisons across camera paths,
   grazing rays, near-boundary positions, and additional scene compositions.
3. Measure dispatch sizes, sample batching, and memory use on discrete/integrated
   GPUs before choosing performance defaults.
4. Finish manual input, minimize/restore, focus, and platform lifecycle checks.
5. Enable a separate high-level WebGPU path for browsers; `wgame`'s current web
   feature uses WebGL2 and cannot run this renderer.

Keep OpenCL available until those gates are met. A change of hyperbolic model or
relative-coordinate representation is a separate numerical experiment.
