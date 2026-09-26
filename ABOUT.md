# Architecture

Hypertrace separates Rust scene construction from GPU rendering:

- `objects`: composable cameras, shapes, materials, backgrounds, and object trees.
  Traits and choice/mixture macros lower these values to the scene representation.
- `scenes`: the shared Euclidean (`eu`) and hyperbolic (`hy`) example factories.
- `scene`: a device-independent scene representation and WGSL compiler. Shader
  schemas describe structure separately from parameter values so ordinary scene
  edits can reuse compiled pipelines.
- `renderer-wgpu`: compute pipelines, progressive accumulation, explicit CPU
  snapshots, and direct GPU presentation through the optional Wgame viewer.

Host-side camera transforms use f64. GPU storage and tracing use f32.
Hyperbolic space uses the upper half-space model; stored isometries are complex
2×2 Möbius matrices. Quaternion values are temporary intermediates for applying
those maps and transporting directions.

Custom shapes and materials implement the corresponding `objects` trait and
provide WGSL lowering hooks. A custom shader leaf carries its source, entry
point, and parameter layout; no central renderer dispatch enum needs editing.
See the [renderer guide](renderer-wgpu/README.md#custom-shader-leaves) for signatures.
