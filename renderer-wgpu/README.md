# WGPU renderer

The renderer compiles generic Rust scene builders in all three curvatures to
WGSL compute shaders. It includes headless tools and a `wgame` viewer with direct GPU
presentation on native platforms and the web. See the repository
[web viewer instructions](../README.md#web-viewer) for Trunk setup and controls.

## Run

Use current stable Rust, a compute-capable native WGPU adapter (software Vulkan
is sufficient for correctness tests), sibling `../../vecmat-rs` and
`../../ccgeom` sources, and the sibling `../../wgame` checkout
with configurable `WindowConfig::required_limits` and `use_adapter_buffer_limits`.
WGPU is pinned to the same
major version as that checkout, 30. The optional path dependency still needs to
resolve even in a headless Cargo build.

From the Hypertrace repository root:

```sh
cargo run --release -p hypertrace-wgpu --features viewer --example viewer -- --scene sp
cargo run --release -p hypertrace-wgpu --example headless -- --list-scenes
cargo run --release -p hypertrace-wgpu --example headless -- \
  --scene hy --width 320 --height 240 --samples 64 --seed 3735928559 --output /tmp/hy-wgpu
```

`viewer`, `headless`, and `benchmark` share the `examples::EXAMPLES` catalog and
support `--list-scenes`. Besides `eu`, `hy`, and `sp`, it includes the
`compare-*` physical-layout comparisons, `sp-fog`, `sp-loop`, and `sp-loop-fog`.
The browser presents the same grouped catalog and accepts `?scene=NAME` URLs.
See the [example guide](../examples/README.md) for all IDs, default event budgets,
and the geometric effects to look for. Start with 64 samples for comparison
markers and 256 or more for fog.

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
JSON records the curvature sign and radius plus medium extinction and albedo,
so captures distinguish the physical configuration as well as the example ID.
`--bounces` overrides the scene default. Adapter details are printed; timings on
software Vulkan are not hardware performance measurements.
Headless rendering also requests supported buffer sizes, but preserves the exact
requested image dimensions: an unsupported size returns a diagnostic error.

## Renderer contract

`Renderer::new_async` accepts an existing device and queue. Await pipeline
validation in browser code; `Renderer::new` is a blocking native convenience
wrapper. Likewise, use `update_scene_async` on the web; `update_scene` is native
only. Blocking `snapshot` and `read_buffer` are native-only tools. Normal browser
frames encode compute and presentation on the GPU with no CPU pixel readback.

Request
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

### Geometry, units, and numerical limits

Generated scenes use a shared embedded kernel. Points and tangents have four
scalar-first components `(w,x,y,z)`: positive hyperboloid for `hy`, unit 3-sphere
for `sp`, and `w=1` points / `w=0` tangents for `eu`. Isometries use two quaternion
rows, with the curvature sign specialized in generated WGSL. The original
half-space model remains an explicit adapter for existing scenes, tilings and
custom leaves. CPU `ccgeom::Space3` also provides Poincaré-ball conversions.

`SceneDefinition.radius` is the physical curvature radius `R`; curved sectional
curvature is `K/R²`, where `K` is −1 or +1. Euclidean radius is fixed to one.
The kernel advances normalized coordinates by `distance/R` in curved spaces.
`GeodesicSphere::new(r)` and `ShapeValue::geodesic_sphere(r)` take physical
radii; spherical spheres require `0 < r < pi*R`. The compiler also rejects
curved radii whose f32 sine/cosine or hyperbolic equivalents collapse the section
equation or overflow; mathematically valid radii can exceed this solver's
precision. `Sphere` remains a unit physical
radius convenience. Map parameters describe normalized geometry: use a matching
`ccgeom::Space3::new(R)` context when constructing physical displacements.

Every `GeoHit.distance` is forward physical distance from the query origin;
isometries preserve it. Queries use `[minimum, maximum)`. Spherical roots can
occur beyond the antipode and after several circuits. Immediate repeated-leaf
suppression uses `8*EPS*R` physical distance (`EPS=1e-6`); it allows later hits
on the same surface. Coplanar rays have no isolated plane hit; tangent sphere
contacts are accepted. Directions and normals are ambient tangents at the hit,
then converted to orthonormal three-component material frames.

Canonical scene transforms and camera composition stay in CPU f64. Before
upload, object maps are composed with the inverse camera in f64 and converted
to checked f32 pairs. Moving the camera updates relative maps and resets
accumulation without changing the generated program. Object-local tilings and
materials retain their original coordinates. GPU tracing remains f32; a
camera-relative origin does not make distant objects or later bounces exact.

Path length uses a high multiple of 1024 world units plus a separate remainder,
so small later segments are not discarded merely because total travel is large.
Block increments remain exact below `2^34` total world units. The optional
`geo_path_distance` accessor rounds the two parts back to f32. This accounting
range is not a coordinate-accuracy guarantee: each segment and spherical phase
evaluation still have f32 precision, and hyperbolic coordinates can lose metric
accuracy far earlier. Numerical guards reject nonfinite states, invalid
manifold/tangent residuals, and hyperbolic normalized steps above the emergency
exponential bound of 40. A numerical termination retains already accumulated
emission and adds no background; it introduces explicit truncation bias. It is
not a surface miss or distance clamp. The 40-step bound is not a certified
accuracy range. See [the geometry contract](../GEOMETRY_CONTRACT.md).

## Generic scene builders

`hypertrace-examples` contains the shared factories and catalog used by native and
browser viewers. The WGPU examples lower those builders as follows:

```rust,ignore
use objects::Scene as _;
let definition = examples::hy::scene::<3>().wgsl_scene()?;
let scene = hypertrace_wgpu::Scene::from_definition(&definition)?;
let renderer = hypertrace_wgpu::Renderer::new(&device, &queue, (640, 480), scene, 1)?;
```

The generic `examples::comparison::scene::<K,H>(radius)?` builder preserves the
physical marker layout across curvature signs and radii. For example,
`scene::<1,1>(3.0)?` uses spherical curvature +1/9 with one surface event.
`examples::recurrence::scene::<12>(true)` builds the floorless spherical long-route
scene with fog. Its `false` variant selects vacuum. These geometric examples
use emissive absorbing spheres for clear silhouettes; the original studios
retain diffuse, reflective, and refractive materials.

`hypertrace-scene` is a CPU-only intermediate representation and WGSL compiler.
It has no graphics runtime dependency. The `objects` traits provide fallible
lowering hooks; types without an implementation report an unsupported error.
The renderer also accepts this intermediate representation directly.

Supported compositions include `SceneImpl`, point and mapped views, constant and
Euclidean gradient backgrounds, covered and mapped objects, object choices and
vectors, shape choices and vectors, mapped shapes, nested mixtures, `Colored`,
and `Emissive`. Primitive shapes and tilings share the same geometry contracts. Maps support Euclidean shifts, rotations and homogeneous rigid maps,
and hyperbolic complex Möbius maps through explicit adapters. Embedded
`Flat3`, `Hyperboloid3`, and `Spherical3` builders use their checked
`EmbeddedIsometry<f64,K>` maps directly. Other maps report an unsupported error.

Shader schemas describe composition independently of values. Choice variants and
empty vector element types are registered before generation, so switching a choice
or resizing a vector preserves the program. Object records and material descriptors
index a separate `u32` parameter arena. Each shape leaf has a distinct identity
for repeated-hit suppression, including leaves in a shape vector. Shape mapping
and object mapping retain their different material coordinate frames.

Update generated objects and materials by lowering a new definition and passing
it to `update_scene`. Direct edits to the legacy raw-record fields of a generated
`Scene` are rejected, keeping shader IDs and parameter offsets consistent. Camera
pose, field of view, background, radius, medium and bounce count are scene values;
changes reset accumulation. Change radius through the source definition and
lower it again so primitive validation uses the new physical scale.

Materials execute in the original nesting order: a mixture draws and subtracts
weights in order, `Colored` changes throughput before calling its child, and
`Emissive` adds light before calling its child. Nested mixtures keep their own
random draws.

### Homogeneous media

`Medium::Vacuum` is the default. `Medium::Homogeneous { extinction, albedo }`
uses scalar extinction per physical world unit and RGB scattering albedo in
`[0,1]`. Zero extinction is vacuum; zero albedo is pure absorption. Scattering
is isotropic. Both surface and volume interactions consume the bounce budget.

```rust,ignore
use objects::Scene as _;
let mut source = examples::sp::fog_scene::<12>();
source.medium = objects::wgsl::Medium::Homogeneous {
    extinction: 0.08,
    albedo: [0.85, 0.9, 0.95],
};
let definition = source.wgsl_scene()?;
let scene = hypertrace_wgpu::Scene::from_definition(&definition)?;
```

`fog_scene` supplies those medium values with the emissive spherical studio;
its mean free flight is 12.5 world units at radius one. The CLI selects it with
`--scene sp-fog`; `sp::scene` and `--scene sp` select vacuum. Both use a
configurable black miss background. The separate `sp-loop-fog` preset has no
floor and uses extinction 0.1 with albedo 0.9. Rays missing its beacons can
complete several circuits before scattering. Rendered color images do not
report individual travelled distances or cycle counts.

The integrator samples `-log(1-u)/extinction` and compares that physical distance
with the nearest surface. With no surface, the interval stays unbounded even
on a sphere: a medium event can occur after many complete circuits. At an event,
throughput is multiplied by scattering albedo. The sampled survival already
accounts for exponential attenuation; applying it again would bias the result.
The diagnostic `geo_transmittance` helper evaluates the analytic segment value
without adding a second attenuation factor to the analog integrator.

### Custom shader leaves

Return `MaterialValue::embedded_custom` or `ShapeValue::embedded_custom` for the
embedded v2 contract. The original `custom` constructors explicitly select the
v1 chart contract. `ShaderLeaf` contains a unique implementation key,
WGSL source, entry point, and fixed parameter word count. No central dispatch
enum needs editing. Helpers `load_u32`, `load_f32`, `load_vec3` and `load_vec4`
read relative word offsets in the parameter arena.

Embedded v2 material entry points use this signature:

```wgsl
fn my_material(base: u32, ctx: GeoMaterialContext,
    sample: ptr<function, MaterialSample>, rng: ptr<function, u32>) {
    (*sample).emission += (*sample).attenuation * load_vec3(base);
    (*sample).alive = 0u;
}
```

`GeoMaterialContext.position` is object-local `vec4<f32>` embedded position;
`normal` and `MaterialSample.direction` are `vec3<f32>` values in its local
orthonormal tangent frame. `MaterialSample` also supplies path throughput
(`attenuation`), accumulated emission and an `alive` flag.
`uniform_random(rng)` advances the same generator as built-ins.

V2 shape entry points take `(base: u32, ray: GeoRay, previous_identity: u32)`
and return `GeoTaggedHit { hit: GeoHit, identity: u32 }`. For example, a custom
physical-radius sphere with one f32 parameter word can call the shared solver:

```wgsl
fn my_sphere(base: u32, ray: GeoRay, previous_identity: u32) -> GeoTaggedHit {
    let minimum = select(0.0, 8.0*EPS*params.misc.y, base == previous_identity);
    let hit = geo_sphere(ray, minimum, geo_infinity(), params.misc.y, load_f32(base));
    return GeoTaggedHit(hit, base);
}
```

`params.misc.y` is the scene curvature radius. `GeoRay` contains `position` and
`tangent`, both `vec4<f32>`. `GeoHit` contains `valid: u32` (0 miss, 1 hit,
2 numerical failure), physical
`distance: f32`, and ambient `position`, `tangent`, `normal` vectors. A custom
leaf owns its interval and immediate-self-hit behavior; generated wrappers
assign the leaf identity. The `geo_plane`, `geo_sphere`, `geo_cube` (Euclidean)
and `geo_horosphere` (hyperbolic) helpers all take physical interval endpoints.

V1 shapes keep `(base: u32, ray: Ray, previous_identity: u32) -> TaggedHit`, with
three-component `Ray` / `Hit` values. V1 materials keep `MaterialContext` with
three-component positions and chart normals/directions. Euclidean coordinates
remain Cartesian; hyperbolic values are half-space coordinates with normalized
Euclidean directions. The adapter converts positions and direction derivatives
and scales legacy hyperbolic hit distances by `R`. V1 leaves are rejected for
spherical scenes with an explicit compatibility diagnostic; use v2 there. The
compiler does not rewrite source text to guess a leaf's coordinate convention.

Use unique WGSL helper names as well as a unique leaf key. Invalid WGSL is reported
when a renderer creates or updates its pipeline, with the flattened source
available from the compiler output for diagnostics.

## Validation

Ordinary tests do not require an adapter. GPU tests are ignored by default and
must be explicitly requested; missing adapters fail rather than silently skip.

```sh
cargo test -p hypertrace-wgpu
WGPU_BACKEND=vulkan cargo test -p hypertrace-wgpu -- --ignored --test-threads=1
cargo clippy --no-deps -p hypertrace-wgpu --all-targets --features viewer -- -D warnings
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-wgpu --features viewer --example viewer -- --scene hy --smoke
```

Coverage includes Möbius matrix ordering, inverse/distance/derivative checks,
small and scaled distances, vertical and nearly vertical rays, hit/miss cases,
reset, resize, scene uploads, deterministic batching, captured tile-selection
fixtures, and presentation transfer and orientation. Generic scene tests cover
nested materials, custom leaves, empty vectors, choices, repeated-hit identities,
and recovery after shader compilation or resource-limit errors. Embedded checks
compare map actions with independent matrices, verify chart derivatives and
frames, test physical radii and interval boundaries, and force spherical medium
events beyond several circuits. Keep these invariant and behavior checks when
changing internal representation; old implementation snapshots are not required.

Native numerical tests have run on software Vulkan. Viewer smoke tests passed
for `eu`, `hy` and `sp` on Intel Arc Vulkan, including camera motion, resizing
across binding limits, restoration and presentation. The WASM viewer build check
passed; a headed browser run of the shared renderer remains unverified.
See [DEVELOPMENT.md](../DEVELOPMENT.md) for the workspace validation workflow.

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
bounce limit. Defaults are four events for `eu`, three for `hy`, six for `sp`,
one for geometric comparisons and `sp-loop`, and twelve for both fog presets.
Surface and volume interactions both consume this budget. Use `--list-scenes`
to find a workload and consult the [example guide](../examples/README.md) for its
layout; each spherical preset has a black miss background.

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
2. GPU f32 arithmetic still loses precision near the ideal boundary. Camera-relative
   preparation is implemented; recentering later path segments remains future work.
3. Workgroup sizes and sample batching remain workload/device choices; the viewer
   uses a single sample per frame for responsive camera movement.
4. WebGPU is enabled alongside Wgame's web runtime. A fresh headed browser
   validation of the shared-kernel changes remains outstanding; build success
   alone does not establish browser rendering correctness.
5. Finite surface/volume bounce limits introduce truncation bias. Heterogeneous
   media, anisotropic phase functions and new importance sampling remain separate
   extensions.
