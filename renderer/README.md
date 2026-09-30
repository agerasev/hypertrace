# Renderer

The renderer compiles generic Rust scene builders in all three curvatures to
WGSL compute shaders. This library owns GPU execution and presentation; the
`hypertrace-examples` package owns standalone scene applications, each with its
own Wgame setup. The separate `hypertrace-gallery` package in `examples/gallery`
owns optional gallery, headless, and benchmark tools. See the repository [web viewer instructions](../README.md#web-viewer) for Trunk setup and controls.

## Run

Use current stable Rust, a compute-capable native WGPU adapter (software Vulkan
is sufficient for correctness tests), sibling `../../vecmat-rs` and
`../../ccgeom` sources, and the sibling `../../wgame` checkout
with configurable `WindowConfig::required_limits` and `use_adapter_buffer_limits`.
WGPU is pinned to the same major version as that checkout, 30. Wgame belongs to
the application packages; Cargo still resolves the gallery's optional path
dependency for headless workspace builds.

From the Hypertrace repository root:

```sh
cargo run --release -p hypertrace-gallery --bin viewer -- --scene sp
cargo run --release -p hypertrace-gallery --bin headless -- --list-scenes
cargo run --release -p hypertrace-gallery --bin headless -- \
  --scene hy --width 320 --height 240 --samples 64 --seed 3735928559 --output /tmp/hy
```

`viewer`, `headless`, and `benchmark` share the `hypertrace_gallery::EXAMPLES` catalog and
support `--list-scenes`. The four choices are `eu`, `hy`, `sp`, and `eu-fog`.
The browser presents the same grouped catalog and accepts `?scene=NAME` URLs.
See the [example guide](../examples/README.md) for all IDs, default event budgets,
and the geometric effects to look for. Start with 256 samples and increase the count for indirect lighting and fog.

The viewer supports WASD/arrows, Space/C, Q/E, left-drag to look, scroll to zoom,
Tab to lock/unlock the mouse, R to restore the initial camera, and Escape to close.
Capture releases on focus loss; in the browser Escape releases capture before
acting as the pause shortcut. Movement integrates actual
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
rows, with the curvature sign specialized in generated WGSL. `Transform<G>` is a
canonical isometry of geometry `G`, constructed from `ccgeom::EmbeddedIsometry`
through `Transform::from_isometry`. `Camera<G>` wraps that transform and its
`move_local(translation, rotation, radius)` uses the same mathematics.
`SceneDefinition<G>`, `CompiledScene<G>`, and `Renderer<G>` retain the same type;
the compiler rejects attempts to mix geometries. Geometry is selected at compile
time, while radius and other numerical parameters remain checked scene values.
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

Each demonstration folder under [`examples/src/bin`](../examples/src/bin) contains a complete
application: `scene.rs` owns the concrete layout and materials, and `main.rs`
constructs the scene, `Renderer`, `Presenter`, and Wgame event loop. A standalone
example does not import another example, the `hypertrace_gallery` library, or a
common application runner. Its package has no library target or gallery
dependency. The separate [`examples/gallery`](../examples/gallery) package imports
these scene files for its metadata, factories, and application integration tests.

Applications construct and lower typed builders using the libraries directly:

```rust,ignore
use ccgeom::{Flat3, Geometry3};
use objects::{
    Mapped, Scene as _, SceneImpl, background::ConstBg,
    material::Lambertian, object::Covered, shape::Sphere, view::PointView,
};
let source = SceneImpl::<Flat3, _, _, _, 4>::new(
    Mapped::new(PointView::new(1.0), Flat3::shift_z(3.0)),
    Covered::new(Sphere, Lambertian),
    ConstBg::new([0.1, 0.2, 0.3].into()),
);
let definition = source.definition()?;
let scene = hypertrace_renderer::Scene::from_definition(&definition)?;
let renderer = hypertrace_renderer::Renderer::new(&device, &queue, (640, 480), scene, 1)?;
```

The [spherical studio](../examples/src/bin/sp/scene.rs) demonstrates shadows
with an off-center sun, diffuse and refractive balls, a glowing landmark, and
a mostly diffuse plane. The `eu`, `hy`,
`sp`, and `eu-fog` scenes expose local `scene::<H>()` constructors where `H`
sets the interaction budget. Numerical curvature and recurrence comparisons
live in independent tests.

`hypertrace-scene` is a CPU-only intermediate representation and WGSL compiler.
It has no graphics runtime dependency. The `objects` traits require shader
module descriptions and instance encoding. The renderer also accepts this
intermediate representation directly.

Supported compositions include `SceneImpl`, point and mapped views, constant and
Euclidean gradient backgrounds, covered and mapped objects, object tuples and
vectors, shape tuples and vectors, mapped shapes, nested mixtures, `Colored`,
and `Emissive`. Primitive shapes and tilings share the same geometry contracts.
`Flat3`, `Hyperboloid3`, and `Spherical3` builders all use checked
`EmbeddedIsometry<f64,K>` maps. `scene::Geometry` and
`objects::shader::RenderMap<G>` preserve these canonical geometry and map types
when encoding the scene description. Component trait implementations specify
supported geometries: for example, `Cube` supports only `Flat3`, and `Horosphere`
supports only `Hyperboloid3`.

Shader modules describe composition independently of values. Every tuple child
and empty vector element type contributes dependencies before generation, so
changing values or resizing vectors preserves the program. Tuples support
heterogeneous composition; vectors hold one component type. Object records and material descriptors
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

### Intrinsic surface tilings

`objects::object::Tiled` combines a shape, a pattern, a fixed array of tile
materials, and a border material. Shapes implement `TileSurface<G>` to supply
an intrinsic domain and chart; patterns implement `Tiling<Domain>`. Unsupported
surface/pattern combinations fail to compile. Downstream shapes and patterns use
these same traits without renderer or compiler registration.

| Surface | Intrinsic domain | Patterns |
| --- | --- | --- |
| Euclidean plane | Euclidean | `Square`, `Hexagonal` |
| Hyperbolic horosphere | Euclidean | `Square`, `Hexagonal` |
| Hyperbolic plane | Hyperbolic | `Pentagonal`, `Pentastar` |
| Spherical plane | Spherical | `RegularSpherical<P,Q>` |
| `Sphere` or `GeodesicSphere` in any geometry | Spherical | `RegularSpherical<P,Q>` |

`Uniform` works in every domain. Flat cell sizes and border half-widths use
physical units, including the curvature-radius scaling of horosphere charts.
Hyperbolic pentagon widths retain curvature-normalized units. Spherical widths
are angular half-widths in radians, independent of the sphere's radius.

For `RegularSpherical<P,Q>`, `P` is the number of edges per face and `Q` is the
number of faces meeting at each vertex. Supported families are tetrahedral
`<3,3>`, cubic `<4,3>`, octahedral `<3,4>`, dodecahedral `<5,3>`, icosahedral
`<3,5>`, lunes `<2,Q>`, and dihedra `<P,2>`. A dihedron has two hemispherical
faces; subdividing its equator does not create additional material boundaries.
The Platonic patterns use spherical Voronoi faces and great-circle borders,
so their charts have no longitude seam or pole singularity.

```rust,ignore
use objects::{material::{Colored, Lambertian}, object::{Tiled, tiling}, shape::Plane};
let floor = Tiled::new(
    Plane,
    tiling::Square::new(0.5, 0.01),
    [[0.8, 0.8, 0.8], [0.3, 0.4, 0.5]]
        .map(|rgb| Colored::new(Lambertian, rgb.into())),
    Colored::new(Lambertian, [0.05; 3].into()),
);
```

Place this object in a `Flat3` scene, or change the shape to `Horosphere` in a
`Hyperboloid3` scene. Use `GeodesicSphere::new(radius)` and
`RegularSpherical::<5,3>::new(0.02)` for a pentagonal sphere in any geometry.
Wrap the whole `Tiled` object in `Mapped` to carry its material coordinates with
its shape. Tile and border material types may differ; all child payloads retain
checked offsets and validation. Sizes, widths and material values remain buffer
data, so editing them does not rebuild the pipeline. Flat cells must remain
normal positive f32 values; samples with unresolved lattice coordinates at or
beyond `2^24` terminate rather than convert overflowing indices. Lune counts
must be below `2^20` so their sectors remain resolvable in f32 longitude.

### Homogeneous media

`Medium::vacuum()` is the default. `Medium { extinction, albedo }`
uses scalar extinction per physical world unit and RGB scattering albedo in
`[0,1]`. Zero extinction is vacuum; zero albedo is pure absorption. Scattering
is isotropic. Both surface and volume interactions consume the bounce budget.

A definition's medium can be changed before compiling it for the renderer:

```rust,ignore
use objects::shader::{Geometry, Medium, Result, SceneDefinition};

fn with_fog<G: Geometry>(mut definition: SceneDefinition<G>)
    -> Result<hypertrace_renderer::Scene<G>>
{
    definition.medium = Medium::homogeneous(0.65, [0.95; 3]);
    hypertrace_renderer::Scene::from_definition(&definition)
}
```

The [eu-fog scene](../examples/src/bin/eu-fog/scene.rs) sets its own medium
values alongside one bright source and red diffuse, green reflective, and blue
refractive spheres in Euclidean space, without a floor. Mean free flight is the
reciprocal of extinction in physical world units. Run the `eu-fog` binary or
select `--scene eu-fog` in a gallery tool.
The [sp scene](../examples/src/bin/sp/scene.rs) selects vacuum.
Both use a configurable black miss background, with different object layouts.
Independent transport tests cover unbounded misses and multiple spherical circuits;
rendered color images do not report individual travelled distances or cycle counts.

The integrator samples `-log(1-u)/extinction` and compares that physical distance
with the nearest surface. With no surface, the interval stays unbounded even
on a sphere: a medium event can occur after many complete circuits. At an event,
throughput is multiplied by scattering albedo. The sampled survival already
accounts for exponential attenuation; applying it again would bias the result.
The diagnostic `geo_transmittance` helper evaluates the analytic segment value
without adding a second attenuation factor to the analog integrator.

### Component-owned shader modules

Built-in and downstream shapes and materials use the same public traits. A
`Shape<G>` supplies `shader() -> Result<ShapeModule<G>>` and
`encode(&self) -> Result<ShapeValue<G>>`; `Material<G>` supplies the corresponding
`shader` and `encode` methods. Modules describe structure;
values encode the current instance into `u32` words. No global registry or
renderer dispatch list needs editing.

A `ShaderModule<G, Role>` owns its stable key, WGSL source, parameter word
count, dependencies, and CPU validation callbacks. `ShapeModule<G>`,
`MaterialModule<G>`, and `LibraryModule<G>` select the calling convention through
their role types. `.dependency()` clones a module into kindless source link data;
`.into_source()` moves it. Geometry remains typed in both forms. A fixed count is
`Some(n)`; `None` permits a component-defined variable layout. `validate_context`
checks physical radius requirements, and `validate_words` checks values during scene
compilation. A wrapper's validator must validate its child payloads as well.

Use `{{self}}` for the module's entry-point name and `{{self}}_helper` for its
private helpers. `{{dep0}}`, `{{dep1}}`, and so on name declared dependencies.
The linker assigns deterministic namespaces, deduplicates matching keys, and
rejects conflicting implementations, dependency cycles and unresolved imports.
If shader structure depends on children, use `ShapeModule::<G>::specialized_key`
(or the corresponding material/library method)
with those dependencies to distinguish its implementations. Parameter values
must not enter keys or source. There is no source rewriting for chart conventions:
all shape and material entry points use embedded geometry.

For example, a downstream glowing material can implement:

```rust,ignore
use objects::{Material, shader::{Geometry, MaterialModule, MaterialValue, Result}};

struct Glow([f32; 3]);
impl<G: Geometry> Material<G> for Glow {
    fn shader() -> Result<MaterialModule<G>> {
        let mut module = MaterialModule::new("my_app.glow", r#"
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
    fn encode(&self) -> Result<MaterialValue<G>> {
        MaterialValue::new(<Self as Material<G>>::shader()?,
            self.0.map(f32::to_bits).into())
    }
}
```

Use `Glow` in `Covered`, `Colored`, `Emissive`, and `mixture!` just like a
built-in material. The compiler collects type dependencies from every tuple child
and empty vector. Low-level construction helpers live with their
components: `objects::shape::{plane, geodesic_sphere, mapped, vector, tuple}`
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
`module.dependencies = vec![objects::shape::plane_schema::<G>().into_source()]` and call
`{{dep0}}(base,ray,previous_identity)` before applying its own clipping rule.
Primitive-specific shader functions are supplied by their components, not baked
into the renderer. Wrappers must preserve numerical failure and physical distance.

The complete downstream [extension integration test](tests/extensions.rs) defines
both an aperture shape and an emitting material outside the core crates. It
composes them with mappings, vectors, tuples and a mixture, then checks all three
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
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-gallery --bin viewer -- --scene hy --smoke
```

Coverage includes shared isometry composition, inverse and distance checks,
small and scaled distances, vertical and nearly vertical rays, hit/miss cases,
reset, resize, scene uploads, deterministic batching, captured tile-selection
fixtures, and presentation transfer and orientation. Generic scene tests cover
nested materials, downstream modules, empty vectors, tuples, repeated-hit identities,
and recovery after shader compilation or resource-limit errors. Embedded checks
compare map actions with independent matrices, verify parabolic placements and
material frames, test physical radii and interval boundaries, and force spherical medium
events beyond several circuits. Compile-fail tests cover mixed geometries,
incorrect shader roles, and unsupported shape/background combinations.
Keep these invariant and behavior checks when
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
and twelve for `eu-fog`.
Surface and volume interactions both consume this budget. Use `--list-scenes`
to find a workload and consult the [example guide](../examples/README.md) for its
layout; each spherical preset has a black miss background.

```sh
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-gallery --bin benchmark -- \
  --scene hy --width 1280 --height 720 --samples 16 --warmup 64 --trials 10 \
  --batch 1 --seed 3735928559 --output /tmp/batch1.json
WGPU_BACKEND=vulkan cargo run --release -p hypertrace-gallery --bin benchmark -- \
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
