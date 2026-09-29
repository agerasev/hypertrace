# Renderer

The renderer compiles generic Rust scene builders in all three curvatures to
WGSL compute shaders. This library owns GPU execution and presentation; the
`hypertrace-examples` package owns standalone scenes, headless tools, and the
Wgame application host for native platforms and the web. See the repository
[web viewer instructions](../README.md#web-viewer) for Trunk setup and controls.

## Run

Use current stable Rust, a compute-capable native WGPU adapter (software Vulkan
is sufficient for correctness tests), sibling `../../vecmat-rs` and
`../../ccgeom` sources, and the sibling `../../wgame` checkout
with configurable `WindowConfig::required_limits` and `use_adapter_buffer_limits`.
WGPU is pinned to the same major version as that checkout, 30. Wgame belongs to
the examples package; Cargo still resolves its optional path dependency for
workspace builds.

From the Hypertrace repository root:

```sh
cargo run --release -p hypertrace-examples --bin viewer -- --scene sp
cargo run --release -p hypertrace-examples --bin headless -- --list-scenes
cargo run --release -p hypertrace-examples --bin headless -- \
  --scene hy --width 320 --height 240 --samples 64 --seed 3735928559 --output /tmp/hy
```

`viewer`, `headless`, and `benchmark` share the `hypertrace_examples::EXAMPLES` catalog and
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
rows, with the curvature sign specialized in generated WGSL. `Transform` is a
single canonical isometry type, constructed from `ccgeom::EmbeddedIsometry`
through `Transform::from_isometry`. `Camera` wraps that transform and its
`move_local(translation, rotation, radius)` uses the same mathematics.
Half-space coordinates are used explicitly for hyperbolic tiling classification;
CPU `ccgeom::Space3` also provides point and tangent conversions for chart-based
construction.

`SceneDefinition.radius` is the physical curvature radius `R`; curved sectional
curvature is `K/R²`, where `K` is −1 or +1. Euclidean radius is fixed to one.
The kernel advances normalized coordinates by `distance/R` in curved spaces.
`GeodesicSphere::new(r)` and `objects::shape::geodesic_sphere(r)` take physical
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

`hypertrace-examples` contains independent scene binaries, the optional gallery,
and shared factories. Applications lower typed builders as follows:

```rust,ignore
use objects::Scene as _;
let definition = hypertrace_examples::hy::scene::<3>().definition()?;
let scene = hypertrace_renderer::Scene::from_definition(&definition)?;
let renderer = hypertrace_renderer::Renderer::new(&device, &queue, (640, 480), scene, 1)?;
```

The generic `hypertrace_examples::comparison::scene::<K,H>(radius)?` builder preserves the
physical marker layout across curvature signs and radii. For example,
`scene::<1,1>(3.0)?` uses spherical curvature +1/9 with one surface event.
`hypertrace_examples::recurrence::scene::<12>(true)` builds the floorless spherical long-route
scene with fog. Its `false` variant selects vacuum. These geometric examples
use emissive absorbing spheres for clear silhouettes; the original studios
retain diffuse, reflective, and refractive materials.

`hypertrace-scene` is a CPU-only intermediate representation and WGSL compiler.
It has no graphics runtime dependency. The `objects` traits require shader
module descriptions and instance encoding. The renderer also accepts this
intermediate representation directly.

Supported compositions include `SceneImpl`, point and mapped views, constant and
Euclidean gradient backgrounds, covered and mapped objects, object choices and
vectors, shape choices and vectors, mapped shapes, nested mixtures, `Colored`,
and `Emissive`. Primitive shapes and tilings share the same geometry contracts.
`Flat3`, `Hyperboloid3`, and `Spherical3` builders all use checked
`EmbeddedIsometry<f64,K>` maps. `objects::shader::RenderGeometry` and
`RenderMap<G>` encode these canonical builder types into the scene description;
no alternate map formats or runtime type whitelist are involved.

Shader modules describe composition independently of values. Choice variants and
empty vector element types contribute dependencies before generation, so switching
a choice or resizing a vector preserves the program. Object records and material descriptors
index a separate `u32` parameter arena. Each shape leaf has a distinct identity
for repeated-hit suppression, including leaves in a shape vector. Shape mapping
and object mapping retain their different material coordinate frames.

Update generated objects and materials by lowering a new definition and passing
it to `update_scene`. Compiled storage records are read-only through the renderer
scene API, keeping shader IDs and parameter offsets consistent. Camera
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
let mut source = hypertrace_examples::sp::fog_scene::<12>();
source.medium = objects::shader::Medium::Homogeneous {
    extinction: 0.08,
    albedo: [0.85, 0.9, 0.95],
};
let definition = source.definition()?;
let scene = hypertrace_renderer::Scene::from_definition(&definition)?;
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

### Component-owned shader modules

Built-in and downstream shapes and materials use the same public traits. A
`Shape<G>` supplies `shader() -> Result<ShaderModule>` and
`encode(&self) -> Result<ShapeValue>`; `Material` supplies the corresponding
`shader` and `encode` methods. Modules describe structure;
values encode the current instance into `u32` words. No global registry or
renderer dispatch list needs editing.

A `ShaderModule` owns its stable key, `ShaderKind`, WGSL source, parameter word
count, dependencies, and CPU validation callbacks. A fixed count is `Some(n)`;
`None` permits a component-defined variable layout. `validate_context` checks
geometry/radius requirements, and `validate_words` checks values during scene
compilation. A wrapper's validator must validate its child payloads as well.

Use `{{self}}` for the module's entry-point name and `{{self}}_helper` for its
private helpers. `{{dep0}}`, `{{dep1}}`, and so on name declared dependencies.
The linker assigns deterministic namespaces, deduplicates matching keys, and
rejects conflicting implementations, dependency cycles and unresolved imports.
If shader structure depends on children, use `ShaderModule::specialized_key`
with those dependencies to distinguish its implementations. Parameter values
must not enter keys or source. There is no source rewriting for chart conventions:
all shape and material entry points use embedded geometry.

For example, a downstream glowing material can implement:

```rust,ignore
use objects::{Material, shader::{MaterialValue, Result, ShaderKind, ShaderModule}};

struct Glow([f32; 3]);
impl Material for Glow {
    fn shader() -> Result<ShaderModule> {
        let mut module = ShaderModule::new("my_app.glow", ShaderKind::Material, r#"
fn {{self}}(base:u32,ctx:GeoMaterialContext,
            sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    (*sample).emission += (*sample).attenuation*load_vec3(base);
    (*sample).alive = 0u;
}
"#, Some(3));
        module.validate_words = |_, _, words| {
            anyhow::ensure!(words.iter().all(|&word| {
                let value = f32::from_bits(word);
                value.is_finite() && value >= 0.0
            }), "glow must be finite and nonnegative");
            Ok(())
        };
        Ok(module)
    }
    fn encode(&self) -> Result<MaterialValue> {
        MaterialValue::new(Self::shader()?,
            self.0.map(f32::to_bits).into())
    }
}
```

Use `Glow` in `Covered`, `Colored`, `Emissive`, and `mixture!` just like a
built-in material. The compiler collects type dependencies even from inactive
choices and empty vectors. Low-level construction helpers live with their
components: `objects::shape::{plane, geodesic_sphere, mapped, vector, choice}`
and `objects::material::{absorbing, colored, emissive, mixture}`, for example.
`ShapeValueExt` and `MaterialValueExt` provide fluent value combinators.

Material positions are object-local embedded `vec4<f32>` values, scalar first.
`GeoMaterialContext.normal` and `MaterialSample.direction` are `vec3<f32>`
values in a local orthonormal tangent frame. `MaterialSample` also contains path
throughput (`attenuation`), accumulated emission and an `alive` flag.
`uniform_random(rng)` advances the common random generator. Parameter helpers
`load_u32`, `load_f32`, `load_vec3`, and `load_vec4` read the word arena.

Shape entry points have this signature:

```wgsl
fn {{self}}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit
```

They return a `GeoTaggedHit` containing `hit: GeoHit` and `identity: u32`.
`GeoRay` has ambient `position` and `tangent` vectors. `GeoHit` contains
`valid: u32` (0 miss, 1 hit, 2 numerical failure), physical `distance: f32`, and
ambient `position`, `tangent`, and `normal` vectors. A leaf normally uses `base`
as its identity and suppresses only immediate repeated hits; later spherical
intersections with that leaf remain eligible. A shape value always contains at
least one word so its base is distinct; a parameterless shape reserves a zero.

A new primitive can use shared geometry operations such as `geo_section_root`
and `geo_advance`. To reuse another component, declare its module dependency
explicitly. For instance, a custom plane wrapper can set
`module.dependencies = vec![objects::shape::plane_schema()]` and call
`{{dep0}}(base,ray,previous_identity)` before applying its own clipping rule.
Primitive-specific shader functions are supplied by their components, not baked
into the renderer. Wrappers must preserve numerical failure and physical distance.

The complete downstream [extension integration test](tests/extensions.rs) defines
both an aperture shape and an emitting material outside the core crates. It
composes them with mappings, vectors, choices and a mixture, then checks all three
curvatures, custom validation, and pipeline reuse across value/structure-preserving
updates. Invalid WGSL is reported when a renderer creates or updates its pipeline;
`shader_source()` exposes the flattened source for diagnostics.

## Validation

Ordinary tests do not require an adapter. GPU tests are ignored by default and
must be explicitly requested; missing adapters fail rather than silently skip.

```sh
cargo test -p hypertrace-renderer
WGPU_BACKEND=vulkan cargo test --workspace -- --ignored --test-threads=1
cargo clippy --no-deps --workspace --all-targets -- -D warnings
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-examples --bin viewer -- --scene hy --smoke
```

Coverage includes shared isometry composition, inverse and distance checks,
small and scaled distances, vertical and nearly vertical rays, hit/miss cases,
reset, resize, scene uploads, deterministic batching, captured tile-selection
fixtures, and presentation transfer and orientation. Generic scene tests cover
nested materials, downstream modules, empty vectors, choices, repeated-hit identities,
and recovery after shader compilation or resource-limit errors. Embedded checks
compare map actions with independent matrices, verify parabolic placements and
material frames, test physical radii and interval boundaries, and force spherical medium
events beyond several circuits. Keep these invariant and behavior checks when
changing internal representation; old implementation snapshots are not required.

Native numerical tests have run on software Vulkan. Viewer smoke tests passed
for `eu`, `hy` and `sp` on Intel Arc Vulkan, including camera motion, resizing
across binding limits, restoration and presentation. The WASM viewer build check
passed; a headed browser run of the shared renderer remains unverified.
See [DEVELOPMENT.md](../DEVELOPMENT.md) for the workspace validation workflow.

### Compare rendered frames

Render two frames with matching scene, dimensions, sample count, seed, bounce
count, curvature sign/radius, and medium using the `headless` binary. Compare
their linear outputs before display conversion:

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

Use the release-mode `benchmark` binary for completed-render timings. Run
configurations sequentially with identical scene, dimensions, samples, seed, and
bounce limit. Defaults are four events for `eu`, three for `hy`, six for `sp`,
one for geometric comparisons and `sp-loop`, and twelve for both fog presets.
Surface and volume interactions both consume this budget. Use `--list-scenes`
to find a workload and consult the [example guide](../examples/README.md) for its
layout; each spherical preset has a black miss background.

```sh
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-examples --bin benchmark -- \
  --scene hy --width 1280 --height 720 --samples 16 --warmup 64 --trials 10 \
  --batch 1 --seed 3735928559 --output /tmp/batch1.json
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-examples --bin benchmark -- \
  --scene hy --width 1280 --height 720 --samples 16 --warmup 64 --trials 10 \
  --batch 16 --seed 3735928559 --output /tmp/batch16.json
python3 tools/compare_benchmarks.py /tmp/batch1.json /tmp/batch16.json \
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

1. Geometry supports the three constant-curvature signs through one embedded
   representation. Other geometries require new mathematical support.
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
