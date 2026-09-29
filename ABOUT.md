# Architecture

Hypertrace separates Rust scene construction from GPU rendering:

- `objects`: composable cameras, shapes, materials, backgrounds, and object trees.
  Shapes, materials and tilings own their WGSL implementations, parameter layouts,
  dependencies and validation. Traits and macros compose their scene descriptions.
- `examples`: the `hypertrace-examples` package contains independent demonstration
  binaries and has no library target. Each `examples/src/bin/<name>/` owns
  `scene.rs` and a `main.rs` that constructs its renderer, presenter, and Wgame
  event loop. Standalone examples never depend on the gallery or a common runner.
- `examples/gallery`: the separate `hypertrace-gallery` package owns the optional
  viewer, headless, and benchmark applications and their integration tests. Its
  `hypertrace_gallery` library imports example-owned scene files for its catalogue.
  Applications depend on the libraries; the renderer has no example dependency.
- `scene`: CPU scene descriptions, a WGSL module linker and GPU data packing.
  Modules describe structure separately from parameter values so ordinary scene
  edits can reuse compiled pipelines. The linker has no built-in shape or material
  catalogue.
- `renderer`: compute pipelines, progressive accumulation, explicit CPU
  snapshots on native platforms, and direct GPU presentation. It accepts scene
  definitions and caller-owned WGPU devices; window hosting belongs to examples.
  The optional Wgame gallery viewer runs natively and through WebAssembly/WebGPU;
  Trunk bundles it with the HTML controls in `examples/gallery/web`.

The three spaces use scalar-first embedded points `(w,x,y,z)`. Hyperbolic
points lie on the positive hyperboloid, spherical points on the unit 3-sphere,
and Euclidean points have `w=1`. A shared algebra stores isometries as two
quaternions `a + e*b`, with `e*e` equal to the curvature sign. Ray advancement,
transform action, tangent frames, analytic sections, and event ordering share
one implementation with the necessary zero-curvature and periodic-root cases.

Sibling libraries own the CPU mathematics: `vecmat-rs` supplies the quaternion
pair algebra; `ccgeom` supplies checked isometries, points/tangents, physical
radius contexts, advancement, and ball/half-space conversions. Scene builders
use `Embedded3<f64, K>` and `EmbeddedIsometry<f64, K>` throughout. The scene
library's `Transform<G>` stores the same isometry and retains its geometry type
through `Camera<G>`, `SceneDefinition<G>`, `CompiledScene<G>`, and `Renderer<G>`.
Curvature is selected at compile time; physical radius remains a scene value.
All shader entry points receive
embedded geometry. Components can explicitly derive chart coordinates for
surface patterns, and the CPU geometry library can convert chart points and
their differentials when constructing embedded data.

Canonical scene and camera transforms remain in CPU f64. Before GPU upload,
object maps are composed with the inverse camera in f64, then checked and
converted to f32. The camera-relative GPU frame improves nearby precision
without changing canonical scene data. It does not remove cancellation at
large hyperbolic distances; recentering later segments remains future work.

Materials use three-component directions and normals in orthonormal tangent
frames. Positions stay embedded internally; the spherical frame is defined
even at antipodes. Reflection, refraction, diffuse sampling and surface emission
therefore retain common material implementations in all three spaces.

Every hit records forward physical travel distance independently of its
position. Periodic spherical intersections are lifted into a half-open query
interval; repeated-hit suppression removes only the immediate numerical
self-hit. The shared integrator selects the nearer surface or medium event.
Homogeneous media use scalar extinction and RGB scattering albedo; a surface
miss does not discard later medium events. Vacuum misses sample the configured
background, which starts black in `sp`.

The [theory guide](https://agerasev.github.io/hypertrace/theory.html) develops the
shared geometry, unwrapped travel distance, coordinate charts, surface types,
path tracing, and numerical limitations. Its source is
[site/theory.html](site/theory.html), expanding on the
[original article](https://agerasev.github.io/2020/03/12/hypertrace.html).

Custom shapes and materials implement the corresponding `objects` trait, returning
the same geometry- and role-typed shader modules and encoded values as built-ins. Modules own
their WGSL source, parameter layouts, validators and explicit dependencies. The
linker assigns namespaces and dispatch functions; no global registry or central
list of implementations needs editing. Heterogeneous components compose through
tuples, while homogeneous vectors allow variable counts. Every tuple child and
empty vector element type contributes its shader dependencies, preserving program
structure during ordinary updates. Unsupported geometry/component combinations
are rejected by trait bounds during compilation. Gallery applications select a
concrete typed entry point at startup instead of storing erased scene values.
See the [renderer guide](renderer/README.md#component-owned-shader-modules)
for signatures and the [downstream extension test](renderer/tests/extensions.rs)
for a complete composed shape/material implementation.

All scene builders and tools use the shared embedded renderer. WGPU and WGSL
are the rendering implementation; CPU construction and compilation remain
separate from GPU resource management and execution.
