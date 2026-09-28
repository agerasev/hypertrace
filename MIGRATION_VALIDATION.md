# Constant-curvature migration validation

Implemented locally on 2026-09-28. The default generated renderer now shares
Euclidean, hyperboloid and spherical geometry, physical-distance intersections,
tangent-space materials, and homogeneous medium transport. The roadmap's
browser runtime gate remains open because this session has no available browser.
No libraries or site have been published.

## Milestones and companion revisions

| Repository | Commit | Milestone |
| --- | --- | --- |
| Hypertrace | `356cf2f` | Migration roadmap |
| Hypertrace | `69f93ee` | Captured baseline, conventions and local dependencies |
| vecmat-rs | `2e43fd9dce251658a217a8556ba2695f83b36fdd` | Shared quaternion-pair algebra |
| ccgeom | `d901b0b` | Local dependency compatibility |
| ccgeom | `14e37f4` | Embedded geometry and legacy chart/map adapters |
| ccgeom | `902af7e33d9eb0aeae7bf916ab838d364b4ec65b` | Formatting and legacy test lint cleanup |
| Hypertrace | `eb9c41c` | Generated renderer, spherical demo, relative uploads and media |
| Hypertrace | `8002a22` | Numerical guards, cross-driver tangent fix, builder compatibility and circuit tests |
| wgame | `42858dc961e1ac27a56a5cc84572e7489d036abc` | Unchanged viewer dependency |

The subsequent precision, adapter and documentation commits follow the renderer
milestone in this repository's history. CI checks out the exact companion
revisions in `.travis.yml`. Those local library commits must be available on
their remotes before remote CI can fetch them; publishing them was not part of
this task. Cargo's existing untracked-lockfile policy remains unchanged.

## Acceptance evidence

The library checks cover independent matrix actions, inverse and composition,
metric preservation, physical radii, half-space/ball derivatives, antipodes,
repeated circuits, invalid inputs, and f32 range. vecmat's no-default-features
check and focused tests passed. ccgeom's 18 tests and all-target Clippy passed.

Workspace checks cover scene lowering, schema/payload validation, physical sphere
radii, custom shader versions, parameter-only updates, transactional camera
uploads, and failed-update recovery. GPU checks execute the production WGSL
against independent analytic references. They include:

- Shared transforms and primitives for every curvature sign, interval endpoints,
  tangent contacts, and spherical hits after several full circuits.
- Forced fog samples beyond three circuits, distinct optical depths at repeated
  endpoints, surface/medium competition, and analytic absorption probability.
- Retaining small distance increments after `2^24` world units, plus block carry
  through the `2^34` accumulation boundary.
- Transparent spherical surfaces encountered repeatedly, v2 material frames,
  v1 custom adapters, and numerical-failure propagation without background light.
- GPU uniform/storage layout, rendering and presentation, reset/resize behavior,
  camera-relative uploads, and pipeline reuse.

Final checks passed:

| Check | Result |
| --- | --- |
| `cargo test --workspace --all-targets` | 38 CPU tests; 35 GPU tests ignored in this command |
| `cargo test --workspace --doc` | 3 doctests |
| Opt-in GPU suite, llvmpipe Vulkan | 35 passed |
| Opt-in GPU suite, Intel Arc Vulkan | 35 passed |
| Workspace/native viewer Clippy with `-D warnings` | Passed |
| WASM viewer Clippy with `-D warnings` | Passed |
| `cargo fmt --all -- --check` | Passed |
| Python comparison-tool tests | 4 passed |
| `trunk build --release` | Passed |

GPU command: `WGPU_BACKEND=vulkan cargo test -p hypertrace-wgpu -- --ignored --test-threads=1`.
Software and hardware suites ran separately. The existing vecmat unused glob
re-export produces a dependency warning; workspace Clippy reports no project
warnings. Logs are copied into the artifact directory's `logs/` folder.

Native viewer smoke tests completed twelve frames for `eu`, `hy`, and `sp` on
Intel Arc Vulkan, including camera updates, a discarded frame, resize across a
2 MiB storage limit, recovery, and GPU presentation. The spherical example's
lighting was visually inspected and reduced to keep surfaces readable.

WASM Clippy and `trunk build --release` pass. Browser automation reports no
available browsers and cannot create an in-app browser, so browser scene
switching and WebGPU execution were **not verified** in this session. This is a
remaining platform acceptance check, not evidence of a browser failure.

## Artifacts and comparisons

The immutable reference lives at
`/tmp/hypertrace-curvature-baseline-20260928`; see
[MIGRATION_BASELINE.md](MIGRATION_BASELINE.md). New previews, comparison reports,
and benchmarks live at `/tmp/hypertrace-curvature-migration-20260928`.
Temporary artifacts must be preserved elsewhere before `/tmp` is cleared.

Frame comparisons use the original scenes/cameras, 160×120 pixels, 32 samples,
seed `3735928559`, and four Euclidean / three hyperbolic bounces on llvmpipe
Vulkan. These are reported measurements, not retrospectively chosen acceptance
tolerances. Independent primitive/material tests provide the correctness gates;
previews provide an additional visual check. Representation and rounding changes
can change later stochastic paths, so migration images need not be bit-identical.

| Scene | RGB MAE | RGB RMSE | RGB max error | Identical channels |
| --- | ---: | ---: | ---: | ---: |
| eu | 0.00009825 | 0.00207088 | 0.11760329 | 54,209/57,600 |
| hy | 0.00423811 | 0.01787324 | 0.44102597 | 50,404/57,600 |

Alpha error is zero for both. `final-comparison.png` shows the saved baseline and
final preview side by side. Numerical guards account for some additional
hyperbolic path truncation; their contribution compared with the pre-guard
embedded preview was measured separately at RGB MAE `0.00099706`.
`final-sp.png` is the inspected 320×240 spherical preview at 256 samples and six
bounces. `FINAL_SHA256SUMS.json` records final artifact hashes.

## Isolated performance comparison

The saved baseline binary and final migrated binary were rerun sequentially,
after builds and correctness tests completed, on the same llvmpipe adapter.
Each uses 320×240 pixels, 16 samples, 16 warmup samples, five trials, the original
seed and bounce count. These are synchronized software-renderer latency results
on a shared CPU host, not hardware GPU throughput or viewer FPS.

| Scene | Batch | Baseline median ms | Migrated median ms | Change |
| --- | ---: | ---: | ---: | ---: |
| eu | 1 | 198.940 | 216.839 | +9.0% |
| eu | 16 | 143.990 | 167.169 | +16.1% |
| hy | 1 | 214.030 | 223.522 | +4.4% |
| hy | 16 | 157.904 | 159.767 | +1.2% |

Measured median render latency increased by 1.2–16.1% across these configurations.
Shared embedded state, numerical guards, and event handling remain enabled;
this migration prioritizes a common validated path. No speculative optimization
or retrospective performance threshold was used. JSON reports preserve setup,
warmup, each render/readback trial, device identity and checksums. The original
phase-0 measurements remain unchanged in the baseline record.

Object records remain 80 bytes and each map remains eight f32 scalars. The
uniform block grows from 128 to 144 bytes for the medium row. Camera updates now
prepare relative object maps and upload them, while reusing the shader pipeline.
Their functional rollback/reset behavior is tested; a dedicated large-scene
camera-update microbenchmark remains future performance work.


## Numerical and compatibility decisions

See [GEOMETRY_CONTRACT.md](GEOMETRY_CONTRACT.md) for the exact conventions.
Canonical transforms remain f64; object maps are prepared relative to the camera
before checked f32 conversion. Curved sphere radii whose section equations cannot
be resolved in f32 are rejected with diagnostics, including tiny angular radii
and spherical radii extremely close to `pi*R`.

Intel and llvmpipe differed by one ULP in a spherical tangent section's cosine.
The solver now uses a coefficient-scaled backward-error band for repeated roots;
resolved near misses remain misses. Existing analytic hit tolerances were not
relaxed to accommodate the driver difference.

The GPU optimized away a Kahan compensation term under relaxed arithmetic.
Travel accounting therefore uses explicit 1024-unit blocks and a remainder.
This preserves later small segments within its documented range without wrapping
physical distance. Coordinate evaluation has separate, tighter f32 limits.

Nonfinite or invalid manifold/tangent states terminate the path while preserving
prior emission and adding no background. Hyperbolic steps above 40 normalized
units are rejected as an emergency exponential bound, not an accuracy promise.
These numerical terminations and the existing finite bounce budget introduce
truncation bias. Later per-path recentering remains follow-up work. Spherical
periodicity itself never terminates medium transport or truncates travel distance.

The old fixed-record renderer remains an explicit comparison fixture; all default
builders and tools use the shared embedded renderer. Legacy half-space APIs and
v1 custom leaves retain chart semantics through adapters. Spherical custom leaves
use v2 explicitly. Heterogeneous media, anisotropic scattering, spherical tilings,
triangles, acceleration structures, and new importance sampling remain outside
this migration.
