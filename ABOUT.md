# Architecture

Hypertrace separates Rust scene construction from GPU rendering:

- `objects`: composable cameras, shapes, materials, backgrounds, and object trees.
  Traits and choice/mixture macros lower these values to the scene representation.
- `examples`: Euclidean (`eu`), hyperbolic (`hy`), and spherical (`sp`) example
  factories, including `sp::fog_scene` with homogeneous isotropic scattering.
- `scene`: a device-independent scene representation and WGSL compiler. Shader
  schemas describe structure separately from parameter values so ordinary scene
  edits can reuse compiled pipelines.
- `renderer-wgpu`: compute pipelines, progressive accumulation, explicit CPU
  snapshots on native platforms, and direct GPU presentation through the optional
  Wgame viewer. The same viewer runs in the browser using WebAssembly and WebGPU;
  Trunk bundles it with the HTML controls in `renderer-wgpu/web`.

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
scene transforms convert explicitly during lowering. Existing hyperbolic
tilings and v1 custom leaves receive half-space coordinates through adapters.

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

Custom shapes and materials implement the corresponding `objects` trait and
provide WGSL lowering hooks. A custom shader leaf carries its source, entry
point, and parameter layout; no central renderer dispatch enum needs editing.
See the [renderer guide](renderer-wgpu/README.md#custom-shader-leaves) for signatures.

All scene builders and tools use the shared embedded renderer. WGPU and WGSL
are the rendering implementation; CPU construction and compilation remain
separate from GPU resource management and execution.
