# Architecture

Hypertrace separates Rust scene construction from GPU rendering:

- `objects`: composable cameras, shapes, materials, backgrounds, and object trees.
  Shapes, materials and tilings own their WGSL implementations, parameter layouts,
  dependencies and validation. Traits and macros compose their scene descriptions.
- `examples`: scene factories, standalone scene binaries, shared native/browser
  viewer host, and optional gallery, headless, and benchmark applications.
  Applications depend on the libraries; the renderer has no example dependency.
- `scene`: CPU scene descriptions, a WGSL module linker and GPU data packing.
  Modules describe structure separately from parameter values so ordinary scene
  edits can reuse compiled pipelines. The linker has no built-in shape or material
  catalogue.
- `renderer`: compute pipelines, progressive accumulation, explicit CPU
  snapshots on native platforms, and direct GPU presentation. It accepts scene
  definitions and caller-owned WGPU devices; window hosting belongs to examples.
  The examples' Wgame viewer runs natively and through WebAssembly/WebGPU;
  Trunk bundles it with the HTML controls in `examples/web`.

The three spaces use scalar-first embedded points `(w,x,y,z)`. Hyperbolic
points lie on the positive hyperboloid, spherical points on the unit 3-sphere,
and Euclidean points have `w=1`. A shared algebra stores isometries as two
quaternions `a + e*b`, with `e*e` equal to the curvature sign. Ray advancement,
transform action, tangent frames, analytic sections, and event ordering share
one implementation with the necessary zero-curvature and periodic-root cases.

Sibling libraries own the CPU mathematics: `vecmat-rs` supplies the quaternion
pair algebra; `ccgeom` supplies checked isometries, points/tangents, physical
radius contexts, advancement, and ball/half-space conversions. Legacy
`Euclidean3` and `Hyperbolic3` keep their original coordinate meaning. Their
scene transforms convert explicitly through `RenderGeometry` and `RenderMap`
traits. Hyperbolic tilings use half-space coordinates through explicit adapters;
shader entry points receive embedded geometry in every curvature.

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
shared geometry, unwrapped travel distance, legacy chart adapters, surface types,
path tracing, and numerical limitations. Its source is
[site/theory.html](site/theory.html), expanding on the
[original article](https://agerasev.github.io/2020/03/12/hypertrace.html).

Custom shapes and materials implement the corresponding `objects` trait, returning
the same `ShaderModule` descriptions and encoded values as built-ins. Modules own
their WGSL source, parameter layouts, validators and explicit dependencies. The
linker assigns namespaces and dispatch functions; no global registry or central
list of implementations needs editing. Type dependencies include inactive choices
and empty vector elements, preserving program structure during ordinary updates.
See the [renderer guide](renderer/README.md#component-owned-shader-modules)
for signatures and the [downstream extension test](renderer/tests/extensions.rs)
for a complete composed shape/material implementation.

All scene builders and tools use the shared embedded renderer. WGPU and WGSL
are the rendering implementation; CPU construction and compilation remain
separate from GPU resource management and execution.
