# Working on Hypertrace

Read [DEVELOPMENT.md](DEVELOPMENT.md) for setup and checks, and
[GEOMETRY_CONTRACT.md](GEOMETRY_CONTRACT.md) before changing geometry or transport.
Keep this file focused on enduring engineering lessons; record implementation
history in commits, not new migration diaries.

## Boundaries and extensibility

- `objects/` owns composable Rust scene components and their shader behavior;
  `scene/` owns CPU scene compilation; `renderer/` owns GPU execution and
  presentation. `examples/` owns standalone demonstration applications and their
  shared application support, distinct from the `scene/` library.
- This project is a library, not an application framework. Dependencies point
  from applications to libraries: `examples` may depend on `renderer`, `objects`,
  and `scene`; those libraries must not depend on `examples`, including through
  development dependencies. Keep application-level integration tests with the
  applications; library tests use independent fixtures.
- Each scene example (`eu`, `hy`, `sp`, and fog scenes)
  is a self-contained normal binary. Its directory owns the scene construction,
  materials, camera setup, renderer initialization, and frame loop, using workspace
  crates directly. Opening an example must show how to build a user's own scene;
  a thin launcher around an example-library factory or shared application runner
  does not satisfy this requirement. Keep small local scaffolding explicit even
  when examples repeat it. Do not hide that scaffolding in generated includes or
  a host macro. Adding an example requires no renderer change or registration.
- Gallery, headless, and benchmark tools may import example-owned scene source
  files to display the same content. Dependencies point from those tools toward
  the examples, never from a standalone example toward the gallery, another
  example, a shared scene factory, or a shared application runner. The optional
  gallery catalogue belongs to `examples/gallery`, a separate Cargo package.
  Keep `hypertrace-examples` binary-only: Cargo builds a same-package library for
  its binaries even when their source never imports that library.
- Viewer, headless, and benchmark entry points are normal binaries in the
  applications package, not Cargo examples attached to the renderer library.
  Window/event-loop, CLI, and browser UI dependencies belong there as well.
  Gallery host utilities stay with the gallery; standalone examples show their
  own direct use of the rendering and window libraries.
- Use generic project names such as `renderer` and `shader`, and trait methods
  such as `shader`, `encode`, and `definition`. Avoid `wgpu`/`wgsl` prefixes or
  suffixes in project APIs and paths. Retain actual dependency/API identifiers,
  environment variables, and `.wgsl` file extensions where required.
- Do not preserve compatibility paths, aliases, old formats, or duplicate APIs
  when replacing a design. Update all callers and fixtures to the current API.
  Keep one canonical quaternion-pair isometry representation for every curvature;
  camera and scene transforms must not expose alternative storage formats.
- Construction is statically typed. Preserve geometry `G` through components,
  maps, cameras, scene definitions, compiled scenes, and renderers. Use trait
  implementations to express supported geometry/component combinations and
  typed shader roles to separate shapes, materials, and libraries. Do not erase
  these types with enums, `Any`, type IDs, or trait objects. Test forbidden
  combinations with compile-fail examples; validate numerical values at runtime.
- Use tuples for heterogeneous composition and homogeneous vectors for variable
  counts. Gallery selection dispatches once into a concrete typed application;
  browser selection reloads that application. Keep catalogues as metadata, without
  erased scene factories. GPU dispatch IDs and flattened shader dependencies are
  compiled data, not a host-side runtime type system.
- WGPU and WGSL are the sole rendering implementation. Do not add speculative
  backend abstractions. Keep CPU construction and validation usable without an
  adapter.
- Built-in shapes and materials must use the same extension path as downstream
  components. Avoid central lists of concrete component types in the renderer
  or compiler. A new shape/material should not require editing those crates.
- `ShaderModule` source uses `{{self}}` for entry points/private helper prefixes
  and `{{dep0}}` for declared dependencies. Keep module keys and source independent
  of parameter values; specialize structural keys with child modules. Component
  trait bounds express geometry support; validators own radius and payload checks,
  including child payload validation in wrappers. A shape needs at least one word
  for a distinct leaf identity even when it has no parameters.
- Deduplicate emitted shader source, not validation. Validate every dependency
  instance even when its shader key was already linked; host validation callbacks
  are not part of shader equality. Enforce shape identity allocation recursively.
- Compose material payloads through checked offset tables, including an end
  sentinel. Variable child payload lengths are data, not shader specializations;
  zero-length material payloads are valid. Reuse shared shader helpers through
  dependencies instead of copying primitive implementations.
- Use [the downstream extension test](renderer/tests/extensions.rs) as the
  acceptance model: custom shape and material types compose and render in every
  curvature without core edits. Primitive shader reuse requires explicit module
  dependencies, not renderer-provided functions named after built-in shapes.
- Separate shader structure from parameter values. Every tuple child and empty
  vector element type contributes its shader dependencies; changing values or
  vector lengths must not accidentally rebuild pipelines.
- Preserve composition order and material coordinate frames. Shape mapping and
  object mapping have different material-frame semantics. Nested mixtures retain
  their own random draws; emission and color wrappers run in their nesting order.

- Tiling compatibility follows the surface's intrinsic domain, not ambient
  curvature. Keep chart extraction, pattern selection and child materials
  separate. Map a tiled object as a whole to preserve its material frame.
  Flat pattern dimensions are physical, hyperbolic pentagon widths are
  curvature-normalized, and spherical border widths are angles; document these
  units explicitly. For hexagonal borders, convert edge-form thresholds by the
  gradient magnitude rather than treating chart coefficients as distances.

## Geometry and transport invariants

- Positions and tangents are scalar-first `(w,x,y,z)`. Curvature signs are
  `-1,0,+1`; curved scenes use physical radius `R`, and Euclidean scenes use one.
  Camera forward is local negative z. Isometry composition is
  `outer.chain(inner)(p) = outer(inner(p))`.
- Hit distance is forward physical travel, not endpoint distance. Surface
  intervals are `[minimum,maximum)`. Spherical phase reduction must never discard
  complete circuits from event distance, fog sampling or accumulated travel.
- A surface miss leaves the medium interval unbounded. Homogeneous free-flight
  sampling already accounts for survival; do not apply exponential attenuation
  a second time. Surface and volume events both consume the interaction budget.
- Canonical maps stay in CPU f64. Compose camera-relative object maps before
  checked f32 upload; do not mutate canonical data during camera movement.
  Check the finite point action at construction and composition: finite pair
  components alone do not guarantee that their sandwich product stays finite.
- Invalid numerical states are distinct from misses. Propagate failure through
  wrappers and stop with prior emission, without adding environmental light.
- The documented f32 limits matter. Hyperbolic step bounds are emergency guards,
  not accuracy promises. Distance accumulation range is not a coordinate range.
  Relaxed shader arithmetic can remove compensated-summation terms: preserve the
  explicit 1024-unit block/remainder travel representation.
- Tangent section classification needs a coefficient-scaled backward-error band.
  GPU drivers have differed by one ULP at a spherical tangent. Keep resolved near
  misses as misses rather than loosening all geometric test tolerances.
- Isolate leaf-identity tests from root rounding. Independently solved surfaces
  can disagree at nominally coincident contacts across GPU drivers; use exact
  shared-coordinate fixtures for suppression checks and separate analytic tests
  for nearest-hit geometry.
- Keep chart conversions explicit and local to calculations that need them.
  Half-space coordinates used by tilings are coordinates, not an alternate ray
  or transform representation. Parabolic translations along a horosphere differ
  from geodesic translations; preserve that distinction when constructing scenes.
- Camera motion uses the scene's physical curvature radius explicitly. Keep the
  translation-then-rotation composition order and reject invalid motion without
  changing the camera.

## Validation and GPU behavior

- Ordinary tests skip GPU checks. Run affected CPU tests first, then the opt-in
  GPU suite sequentially when changing shader generation, geometry or transport.
  Record which adapter ran; sandboxed runs may select software Vulkan while a
  desktop run selects hardware.
- Preserve independent analytic and invariant tests when removing old code.
  Port useful reset, resizing, tiling, accumulation and presentation tests rather
  than deleting them because their fixtures used a retired renderer path.
- In the examples web asset, use Trunk `data-bin="viewer"` to select the Cargo
  binary. `data-target-name` alone selects an artifact after building and does
  not prevent native-only tools from being compiled for WASM.
- Check both native viewer and WASM builds when touching shared viewer code.
  A successful web bundle does not prove browser WebGPU execution.
- WGPU queue writes execute before the next submission. Submit encoded work
  before changing parameters or resetting accumulation. Rebind presentation after
  replacing pixel buffers on resize.
- Native readback waits for producer work before submitting its copy. Intel MTL
  with Mesa 23.2.1 otherwise returned partially updated first snapshots. Interactive
  presentation should remain on the GPU without that CPU wait.
- Buffer upload chunking cannot bypass a storage-binding limit. Viewer hosts fit
  render resolution to supported bindings and scale presentation; headless tools
  preserve requested dimensions and report an unsupported size.
- Use the shared example catalogue for CLI and browser choices. Keep demonstrations
  focused on observable geometry, lighting and transport behavior. Keep numerical
  comparison and recurrence fixtures in tests rather than standalone demos.
  Place touching objects using intrinsic surface normals and physical distances;
  a coordinate offset need not produce tangency on a curved surface.
  Check default-camera renders to assess visibility, but preserve requested radii
  and placement constraints rather than changing them to improve framing.
  Pattern showcases should document viewing order and `{P,Q}` symbols, and use
  comparable surface sizes, palettes and border widths to make differences clear.
- Verify fog with an off-axis emitter: scattering must reveal its light, pure
  absorption must not, and switching off all emission with a black environment
  must yield black. In spherical scenes long free flights and repeated paths can
  spread illumination throughout the space; do not mistake that for fog emission.
  Smaller emitters at unchanged radiance reduce total light and sharpen shadows.
  Do not add example-specific scene/layout tests or pin visual tuning values.
  Keep numerical tuning in the scene source and geometry/transport invariants in
  independent library fixtures. Test host input and renderer behavior separately.
- Mouse capture belongs to application hosts. Native hosts use locked capture
  with confined fallback, restore the cursor on focus loss and exit, and consume
  raw relative motion once. Browser capture requests must run synchronously in a
  user gesture; use actual `pointerLockElement` state, handle denial and Escape,
  and discard accumulated look motion when capture changes.

## Repository workflow

- Companion checkouts are `../vecmat-rs`, `../ccgeom`, and `../wgame`. Cargo resolves
  the gallery's optional Wgame path even for headless builds. Keep compatible revision pins
  in `.travis.yml` and `DEVELOPMENT.md` synchronized when changing dependencies.
- Check worktree status before editing and avoid overwriting concurrent work.
  Keep mechanical renames separate from semantic changes and commit meaningful
  milestones when requested. Do not push or publish solely to complete local work.
- Keep generated renders, build output and temporary validation logs outside
  source control. The repository currently ignores `Cargo.lock`.
- Published previews retain their own render revision, dimensions, settings and
  image hash. Updating the viewer must not relabel older images with new source
  revisions; preserve an approved PNG verbatim and never upscale its derivatives.
